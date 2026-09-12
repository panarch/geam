use super::ExecutionContext;
use crate::execution::{
    ExecutionUnit, ExecutionUnitId, HostExecutionState, UnitExit, UnitFinished, UnitOwner,
    UnitRecord, Worker,
};
use crate::host::HostProfile;
use crate::plan::execution::HostedProgram;
use crate::runtime::error::ExecutionResult;
use crate::runtime::work::Cancelled;
use crate::runtime::{CallbackInputs, HostCallOrigin, RetainedCallable};
use futures_channel::mpsc::{UnboundedReceiver, UnboundedSender, unbounded};
use futures_util::StreamExt;
use std::collections::{BTreeMap, VecDeque};
use std::future::{Future, poll_fn};
use std::num::NonZeroUsize;
use std::pin::pin;
use std::sync::Arc;
use std::task::{Context, Poll};

pub(crate) struct Units<Profile: HostProfile> {
    state: Profile::ExecutionState,
    live: BTreeMap<ExecutionUnitId, UnitRecord>,
    spawned: VecDeque<Spawned>,
    finished: UnboundedReceiver<UnitFinished>,
    completion: UnboundedSender<UnitFinished>,
}

pub(super) struct Spawned {
    root: Root,
    unit: ExecutionUnit,
    callable: RetainedCallable,
    origin: HostCallOrigin,
}

pub(super) struct Root {
    owner: UnitOwner,
}

pub(super) struct CancelOnDrop(pub(super) ExecutionUnit);

impl<Profile: HostProfile> Units<Profile> {
    pub(crate) fn new(state: Profile::ExecutionState) -> Self {
        let (completion, finished) = unbounded();
        Self {
            state,
            live: BTreeMap::new(),
            spawned: VecDeque::new(),
            finished,
            completion,
        }
    }

    pub(crate) fn state(&mut self) -> &mut Profile::ExecutionState {
        &mut self.state
    }

    pub(super) fn completion(&self) -> UnboundedSender<UnitFinished> {
        self.completion.clone()
    }

    pub(super) fn begin(&mut self, owner: UnitOwner) -> Root {
        let unit = owner.handle();
        self.live.insert(unit.id(), owner.record());
        self.state.started(unit);
        Root { owner }
    }

    pub(crate) fn spawn(
        &mut self,
        callable: RetainedCallable,
        origin: HostCallOrigin,
    ) -> ExecutionUnit {
        let owner = UnitOwner::new(self.completion());
        let unit = owner.handle();
        let root = self.begin(owner);
        self.spawned.push_back(Spawned {
            root,
            unit: unit.clone(),
            callable,
            origin,
        });
        unit
    }

    pub(super) fn next_spawn(&mut self) -> Option<Spawned> {
        self.spawned.pop_front()
    }

    pub(super) fn finish_next(&mut self, cx: &mut Context<'_>) -> bool {
        match self.finished.poll_next_unpin(cx) {
            Poll::Ready(Some(finished)) => {
                self.finish(finished);
                true
            }
            Poll::Pending | Poll::Ready(None) => false,
        }
    }

    pub(super) fn close(&mut self) {
        while let Ok(finished) = self.finished.try_recv() {
            self.finish(finished);
        }
        for (id, record) in std::mem::take(&mut self.live) {
            self.state.finished(id, &record.close());
        }
        self.spawned.clear();
        self.state.close();
    }

    fn finish(&mut self, finished: UnitFinished) {
        if self.live.remove(&finished.unit).is_some() {
            self.state.finished(finished.unit, &finished.exit);
        }
    }
}

impl Spawned {
    pub(super) fn into_worker<Profile: HostProfile>(
        self,
        plan: Arc<HostedProgram<Profile>>,
        context: ExecutionContext<Profile>,
        budget: NonZeroUsize,
    ) -> Worker {
        let Self {
            root,
            unit,
            callable,
            origin,
        } = self;
        let context = context.with_unit(Some(unit));
        Box::pin(async move {
            let _ = root
                .run(async move {
                    let invocation = callable.with_value(|function| {
                        crate::runtime::function::prepare_callable(
                            plan.as_ref(),
                            function,
                            origin,
                            CallbackInputs::new().into_arguments(),
                        )
                    });
                    invocation
                        .submit(context.services(), budget)
                        .await
                        .map(|result| result.map(|_| ()))
                })
                .await;
        })
    }
}

impl Root {
    pub(super) async fn run<Output>(
        self,
        operation: impl Future<Output = Result<ExecutionResult<Output>, Cancelled>>,
    ) -> Result<ExecutionResult<Output>, Cancelled> {
        let result = {
            let mut cancellation = pin!(self.owner.cancelled());
            let mut operation = pin!(operation);
            poll_fn(|cx| {
                if cancellation.as_mut().poll(cx).is_ready() {
                    Poll::Ready(Err(Cancelled))
                } else {
                    operation.as_mut().poll(cx)
                }
            })
            .await
        };
        let exit = match &result {
            Ok(Ok(_)) => UnitExit::Completed,
            Ok(Err(error)) => UnitExit::Failed(error.clone()),
            Err(Cancelled) => UnitExit::Cancelled,
        };
        if self.owner.finish(exit) {
            result
        } else {
            Err(Cancelled)
        }
    }
}

impl Drop for CancelOnDrop {
    fn drop(&mut self) {
        self.0.cancel();
    }
}

#[cfg(test)]
mod tests {
    use crate::embedding::{BigInt, CallError, FunctionDeclaration, HostedModuleBuilder};
    use crate::execution::{ExecutionUnit, ExecutionUnitId, HostExecutionState, UnitExit};
    use crate::execution_fixture::TestHost;
    use crate::host::{
        HostCall, HostCallCompletion, HostCallContinuation, HostCallError, HostCallable,
        HostConstructions, HostFunctionType, HostOwnedCompletion, HostProfile, HostProvider,
        HostProviderModule, HostProviderSet, HostTypeListEnd,
    };
    use crate::{ModuleSource, PackageSource};
    use futures_channel::{mpsc, oneshot};
    use futures_util::{StreamExt, future};
    use parking_lot::Mutex;
    use std::sync::Arc;

    #[derive(Clone, Debug, PartialEq, Eq)]
    enum Event {
        Started(ExecutionUnitId),
        Completed(ExecutionUnitId),
        Cancelled(ExecutionUnitId),
        Failed(ExecutionUnitId, String),
        Closed,
    }

    #[derive(Clone)]
    struct Trace {
        records: Arc<Mutex<Vec<Event>>>,
        sender: mpsc::UnboundedSender<Event>,
    }

    impl Trace {
        fn new() -> (Self, mpsc::UnboundedReceiver<Event>) {
            let (sender, receiver) = mpsc::unbounded();
            (
                Self {
                    records: Arc::default(),
                    sender,
                },
                receiver,
            )
        }

        fn emit(&self, event: Event) {
            self.records.lock().push(event.clone());
            let _ = self.sender.unbounded_send(event);
        }
    }

    impl Default for Trace {
        fn default() -> Self {
            Self::new().0
        }
    }

    #[derive(Default)]
    struct Lifecycle {
        trace: Trace,
        units: Vec<ExecutionUnit>,
    }

    impl HostExecutionState for Lifecycle {
        fn started(&mut self, unit: ExecutionUnit) {
            self.trace.emit(Event::Started(unit.id()));
            self.units.push(unit);
        }

        fn finished(&mut self, unit: ExecutionUnitId, exit: &UnitExit) {
            self.trace.emit(match exit {
                UnitExit::Completed => Event::Completed(unit),
                UnitExit::Cancelled => Event::Cancelled(unit),
                UnitExit::Failed(error) => Event::Failed(unit, error.to_string()),
            });
        }

        fn close(&mut self) {
            assert!(self.units.iter().all(|unit| !unit.is_active()));
            self.trace.emit(Event::Closed);
        }
    }

    struct Profile;
    struct Provider;

    struct State {
        trace: Trace,
        gate: Option<oneshot::Receiver<()>>,
        ready: Option<oneshot::Sender<()>>,
    }

    impl HostProfile for Profile {
        type RunState = State;
        type ExternalStores = ();
        type ExecutionState = Lifecycle;

        fn initialize_execution(state: &mut State) -> Lifecycle {
            Lifecycle {
                trace: state.trace.clone(),
                units: Vec::new(),
            }
        }
    }

    impl HostProvider<Profile> for Provider {
        type State = State;
        fn project(state: &mut State) -> &mut State {
            state
        }
    }

    type Callback = HostFunctionType<HostTypeListEnd, BigInt>;

    #[test]
    fn cancellation_notifies_services_before_an_unpolled_worker_is_released() {
        let (trace, _) = Trace::new();
        let mut units = super::Units::<Profile>::new(Lifecycle {
            trace: trace.clone(),
            units: Vec::new(),
        });
        let owner = crate::execution::UnitOwner::new(units.completion());
        let unit = owner.handle();
        let root = units.begin(owner);
        assert!(unit.cancel());
        let mut cx = std::task::Context::from_waker(std::task::Waker::noop());
        assert!(units.finish_next(&mut cx));
        assert_eq!(
            &*trace.records.lock(),
            &[Event::Started(unit.id()), Event::Cancelled(unit.id())],
        );
        drop(root);
        assert!(!units.finish_next(&mut cx));
        let owner = crate::execution::UnitOwner::new(units.completion());
        let shutdown_unit = owner.handle();
        let root = units.begin(owner);
        units.close();
        drop(root);
        assert!(units.finish_next(&mut cx));
        assert!(!units.finish_next(&mut cx));
        assert_eq!(
            &*trace.records.lock(),
            &[
                Event::Started(unit.id()),
                Event::Cancelled(unit.id()),
                Event::Started(shutdown_unit.id()),
                Event::Cancelled(shutdown_unit.id()),
                Event::Closed,
            ]
        );
    }

    #[test]
    fn shutdown_preserves_completion_that_wins_during_another_terminal_hook() {
        #[derive(Default)]
        struct Finishing {
            next: Option<crate::execution::UnitOwner>,
            events: Vec<(ExecutionUnitId, bool)>,
            closed: bool,
        }
        impl HostExecutionState for Finishing {
            fn started(&mut self, _: ExecutionUnit) {}
            fn finished(&mut self, unit: ExecutionUnitId, exit: &UnitExit) {
                self.events
                    .push((unit, matches!(exit, UnitExit::Completed)));
                if let Some(owner) = self.next.take() {
                    assert!(owner.finish(UnitExit::Completed));
                }
            }
            fn close(&mut self) {
                self.closed = true;
            }
        }
        struct Closing;
        impl HostProfile for Closing {
            type RunState = ();
            type ExternalStores = ();
            type ExecutionState = Finishing;
        }
        let mut units = super::Units::<Closing>::new(Finishing::default());
        let first = crate::execution::UnitOwner::new(units.completion());
        let first_id = first.handle().id();
        let first_root = units.begin(first);
        let second = crate::execution::UnitOwner::new(units.completion());
        let second_id = second.handle().id();
        let second_root = units.begin(second);
        units.state.next = Some(second_root.owner);
        units.close();
        assert_eq!(units.state.events, [(first_id, false), (second_id, true)]);
        assert!(units.state.closed);
        assert!(units.live.is_empty());
        drop(first_root);
        let mut cx = std::task::Context::from_waker(std::task::Waker::noop());
        while units.finish_next(&mut cx) {}
        assert_eq!(units.state.events, [(first_id, false), (second_id, true)]);
    }

    fn current<'call>(
        mut call: HostCall<'call, Profile, Provider, BigInt>,
    ) -> Result<HostCallCompletion<'call, BigInt>, HostCallError> {
        let current = call.execution_unit().expect("source has an execution unit");
        let index = call
            .execution_state()
            .units
            .iter()
            .position(|unit| unit.id() == current.id())
            .unwrap();
        Ok(call.return_value(index.into()))
    }

    fn spawn<'call>(
        mut call: HostCall<'call, Profile, Provider, BigInt>,
        callback: HostCallable<'call, HostTypeListEnd, BigInt>,
    ) -> Result<HostCallCompletion<'call, BigInt>, HostCallError> {
        let spawned = call.spawn(callback);
        let index = call
            .execution_state()
            .units
            .iter()
            .position(|unit| unit.id() == spawned.id())
            .unwrap();
        Ok(call.return_value(index.into()))
    }

    fn cancel<'call>(
        call: HostCall<'call, Profile, Provider, ()>,
    ) -> Result<HostCallCompletion<'call, ()>, HostCallError> {
        assert!(call.execution_unit().unwrap().cancel());
        Ok(call.return_value(()))
    }

    fn hold<'call>(
        mut call: HostCall<'call, Profile, Provider, ()>,
        constructions: HostConstructions<'call, HostTypeListEnd>,
    ) -> Result<HostCallContinuation<'call, ()>, HostCallError> {
        let gate = call.state().gate.take().expect("one waiting source call");
        let ready = call.state().ready.take().unwrap();
        Ok(call.resume(constructions, move |_| {
            Box::pin(async move {
                ready.send(()).unwrap();
                gate.await.unwrap();
                Ok(HostOwnedCompletion::new(
                    |call, _| Ok(call.return_value(())),
                ))
            })
        }))
    }

    fn invoke<'call>(
        call: HostCall<'call, Profile, Provider, BigInt>,
        constructions: HostConstructions<'call, HostTypeListEnd>,
        callback: HostCallable<'call, HostTypeListEnd, BigInt>,
    ) -> Result<HostCallContinuation<'call, BigInt>, HostCallError> {
        let callback = call.owned_callable(callback, &constructions);
        Ok(call.resume(constructions, move |context| {
            Box::pin(async move {
                let value = callback
                    .invoke(&context, |_, _| (), |_, _, value| Ok(value))
                    .await?;
                Ok(HostOwnedCompletion::new(move |call, _| {
                    Ok(call.return_value(value))
                }))
            })
        }))
    }

    fn program(source: &str) -> HostedModuleBuilder<Profile> {
        let providers = HostProviderModule::new("application", "library")
            .unwrap()
            .with_scoped_function::<Provider, (), BigInt, _>("current", current)
            .unwrap()
            .with_scoped_function::<Provider, (Callback,), BigInt, _>("spawn", spawn)
            .unwrap()
            .with_scoped_function::<Provider, (), (), _>("cancel", cancel)
            .unwrap()
            .with_resumable_function::<Provider, (), (), HostTypeListEnd, _>("hold", hold)
            .unwrap()
            .with_resumable_function::<Provider, (Callback,), BigInt, HostTypeListEnd, _>(
                "invoke", invoke,
            )
            .unwrap();
        let source = format!(
            r#"
@external(erlang, "native", "current")
fn current() -> Int
@external(erlang, "native", "spawn")
fn spawn(callback: fn() -> Int) -> Int
@external(erlang, "native", "cancel")
fn cancel() -> Nil
@external(erlang, "native", "hold")
fn hold() -> Nil
@external(erlang, "native", "invoke")
fn invoke(callback: fn() -> Int) -> Int
{source}
"#
        );
        let typed = crate::compile_typed_host_program(
            "application",
            "library",
            [PackageSource::new(
                "application",
                Vec::<String>::new(),
                [ModuleSource::new("library", "library.gleam", source)],
            )],
            HostProviderSet::from_providers([providers]).unwrap(),
        )
        .unwrap();
        HostedModuleBuilder::new(typed).unwrap()
    }

    #[test]
    fn ordinary_callbacks_keep_the_callers_unit_across_a_native_wait() {
        let initial = Lifecycle::default();
        assert!(initial.trace.records.lock().is_empty());
        assert!(initial.units.is_empty());
        let (builder, run) =
            program("pub fn run() { #(current(), invoke(fn() { hold() current() }), current()) }")
                .function(FunctionDeclaration::<(), (BigInt, BigInt, BigInt)>::new(
                    "run",
                ))
                .unwrap();
        let mut module = builder.seal().unwrap();
        let host = TestHost::default();
        let (trace, _) = Trace::new();
        let (release, gate) = oneshot::channel();
        let (ready, waiting) = oneshot::channel();
        let mut state = State {
            trace: trace.clone(),
            gate: Some(gate),
            ready: Some(ready),
        };
        let mut echo = Vec::new();
        host.block_on(
            module.with_execution(&host, &mut state, &mut echo, async |scope| {
                let (value, ()) = future::join(scope.call(&run, ()), async move {
                    waiting.await.unwrap();
                    release.send(()).unwrap();
                })
                .await;
                assert_eq!(value, Ok((0.into(), 0.into(), 0.into())));
            }),
        )
        .unwrap();
        let records = trace.records.lock();
        let ids = records
            .iter()
            .filter_map(|event| match event {
                Event::Started(id) => Some(*id),
                _ => None,
            })
            .collect::<Vec<_>>();
        assert_eq!(ids.len(), 1);
        let id = ids[0];
        assert_eq!(
            &records[..],
            &[Event::Started(id), Event::Completed(id), Event::Closed]
        );
    }

    #[test]
    fn a_spawned_unit_outlives_its_source_creator_and_progresses_between_rust_entries() {
        let builder = program(
            "pub fn launch() { spawn(fn() { hold() current() }) } pub fn tag() { current() }",
        );
        let (mut builder, launch) = builder
            .function(FunctionDeclaration::<(), BigInt>::new("launch"))
            .unwrap();
        let tag = builder
            .function(FunctionDeclaration::<(), BigInt>::new("tag"))
            .unwrap();
        let mut module = builder.seal().unwrap();
        let host = TestHost::default();
        let (trace, mut events) = Trace::new();
        let (release, gate) = oneshot::channel();
        let (ready, waiting) = oneshot::channel();
        let mut state = State {
            trace: trace.clone(),
            gate: Some(gate),
            ready: Some(ready),
        };
        let mut echo = Vec::new();
        host.block_on(
            module.with_execution(&host, &mut state, &mut echo, async |scope| {
                assert_eq!(scope.call(&launch, ()).await, Ok(1.into()));
                waiting.await.unwrap();
                assert_eq!(scope.call(&tag, ()).await, Ok(2.into()));
                let ids = trace
                    .records
                    .lock()
                    .iter()
                    .filter_map(|event| match event {
                        Event::Started(id) => Some(*id),
                        _ => None,
                    })
                    .collect::<Vec<_>>();
                assert_eq!(ids.len(), 3);
                let child = ids[1];
                release.send(()).unwrap();
                while events.next().await.unwrap() != Event::Completed(child) {}
                assert_eq!(scope.call(&tag, ()).await, Ok(3.into()));
            }),
        )
        .unwrap();
        let records = trace.records.lock();
        assert_eq!(
            records
                .iter()
                .filter(|event| matches!(event, Event::Started(_)))
                .count(),
            4
        );
        assert_eq!(
            records
                .iter()
                .filter(|event| matches!(event, Event::Completed(_)))
                .count(),
            4
        );
        assert_eq!(records.last(), Some(&Event::Closed));
    }

    #[test]
    fn source_failure_and_cancellation_remain_distinct_and_emit_one_terminal_event() {
        for (source, failed) in [
            ("pub fn run() { panic as \"source failure\" 1 }", true),
            (
                "pub fn run() { invoke(fn() { panic as \"source failure\" 1 }) }",
                true,
            ),
            ("pub fn run() { cancel() echo 1 1 }", false),
        ] {
            let (builder, run) = program(source)
                .function(FunctionDeclaration::<(), BigInt>::new("run"))
                .unwrap();
            let mut module = builder.seal().unwrap();
            let host = TestHost::default();
            let (trace, _) = Trace::new();
            let mut state = State {
                trace: trace.clone(),
                gate: None,
                ready: None,
            };
            let mut echo = Vec::new();
            let result = host
                .block_on(
                    module.with_execution(&host, &mut state, &mut echo, async |scope| {
                        scope.call(&run, ()).await
                    }),
                )
                .unwrap();
            assert!(echo.is_empty());
            let records = trace.records.lock();
            let ids = records
                .iter()
                .filter_map(|event| match event {
                    Event::Started(id) => Some(*id),
                    _ => None,
                })
                .collect::<Vec<_>>();
            assert_eq!(ids.len(), 1);
            let id = ids[0];
            let terminal = if failed {
                let error = result.unwrap_err();
                let message = error.to_string();
                assert!(message.contains("source failure"));
                Event::Failed(id, message)
            } else {
                assert_eq!(result, Err(CallError::Cancelled));
                Event::Cancelled(id)
            };
            assert_eq!(&records[..], &[Event::Started(id), terminal, Event::Closed]);
        }
    }
}
