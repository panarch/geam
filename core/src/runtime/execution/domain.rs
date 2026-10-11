use super::{ExecutionContext, Request, Units};
use crate::execution::{
    DriverError, ExecutionClock, ExecutionHost, ExecutionOutcome, HostExecutionState, HostTask,
    TaskExit, UnitFinished, UnitOwner, Worker,
};
use crate::host::HostProfile;
use crate::plan::execution::HostedProgram;
use crate::plan::execution::runtime::RuntimeExecutionPlan;
use crate::runtime::error::ExecutionResult;
use crate::runtime::function::{EntryTarget, EntryValue, Execution};
use crate::runtime::state::{RuntimeHost, RuntimeState};
use crate::runtime::work::Cancelled;
use crate::runtime::work::execution::ExecutionWork;
use crate::runtime::work::request::{Requests, Sender};
use crate::runtime::{EchoSink, HostCallOrigin, RetainedValues, RuntimeListStorage};
use futures_util::StreamExt;
use futures_util::stream::FuturesUnordered;
use std::future::{Future, poll_fn};
use std::num::NonZeroUsize;
use std::pin::{Pin, pin};
use std::sync::Arc;
use std::task::{Context, Poll};

pub(crate) struct Domain<'host, Profile: HostProfile> {
    plan: Arc<HostedProgram<Profile>>,
    host: &'host dyn ExecutionHost,
    state: &'host mut Profile::RunState,
    stores: &'host mut Profile::ExternalStores,
    echo: &'host mut (dyn EchoSink + Send),
    lists: RuntimeListStorage,
    work: ExecutionWork<Profile>,
    entries: Requests<Entry<Profile>>,
    tasks: FuturesUnordered<Box<dyn HostTask>>,
    units: Units<Profile>,
    closed: bool,
    budget: NonZeroUsize,
}

pub(crate) struct EntryContext<Profile: HostProfile> {
    plan: Arc<HostedProgram<Profile>>,
    execution: ExecutionContext<Profile>,
    entries: Sender<Entry<Profile>>,
    completion: futures_channel::mpsc::UnboundedSender<UnitFinished>,
    budget: NonZeroUsize,
}

struct Entry<Profile: HostProfile> {
    owner: UnitOwner,
    start: Box<dyn FnOnce(ExecutionContext<Profile>, super::unit::Root) -> Worker + Send>,
}

impl<'host, Profile: HostProfile> Domain<'host, Profile> {
    pub(crate) const DEFAULT_BUDGET: NonZeroUsize = NonZeroUsize::MIN.saturating_add(1023);

    pub(crate) fn new(
        plan: Arc<HostedProgram<Profile>>,
        host: &'host dyn ExecutionHost,
        state: &'host mut Profile::RunState,
        stores: &'host mut Profile::ExternalStores,
        echo: &'host mut (dyn EchoSink + Send),
        captures: crate::runtime::CaptureStorage,
        budget: NonZeroUsize,
    ) -> Self {
        let mut services = Profile::initialize_execution(state);
        services.initialize(crate::execution::ExecutionMetadata(plan.value_metadata()));
        let units = Units::new(services);
        Self {
            plan,
            host,
            state,
            stores,
            echo,
            budget,
            lists: RuntimeListStorage::default(),
            work: ExecutionWork::new(captures.for_execution()),
            entries: Requests::new(),
            tasks: FuturesUnordered::new(),
            units,
            closed: false,
        }
    }

    pub(crate) fn context(&self) -> EntryContext<Profile> {
        EntryContext {
            plan: Arc::clone(&self.plan),
            execution: self.work.execution(),
            entries: self.entries.sender(),
            completion: self.units.completion(),
            budget: self.budget,
        }
    }

    pub(crate) async fn drive<Output>(
        mut self,
        body: impl Future<Output = Output>,
    ) -> Result<ExecutionOutcome<Output>, DriverError> {
        let mut output = {
            let mut body = pin!(body);
            poll_fn(|cx| self.poll_body(body.as_mut(), cx)).await
        };
        self.close();
        poll_fn(|cx| {
            if let Some(error) = self.reap(cx)
                && !matches!(output, Err(DriverError::Failed(_)))
            {
                output = Err(error);
            }
            if self.tasks.is_empty() {
                Poll::Ready(())
            } else {
                Poll::Pending
            }
        })
        .await;
        output
    }

    fn poll_body<Output>(
        &mut self,
        mut body: Pin<&mut impl Future<Output = Output>>,
        cx: &mut Context<'_>,
    ) -> Poll<Result<ExecutionOutcome<Output>, DriverError>> {
        self.finish_units(cx);
        let output = body.as_mut().poll(cx);
        if output.is_pending() {
            self.service(cx);
        }
        if let Some(error) = self.reap(cx) {
            return Poll::Ready(Err(error));
        }
        match self.work.exit_status() {
            Some(status) => Poll::Ready(Ok(ExecutionOutcome::Exited(status))),
            None => output.map(|value| Ok(ExecutionOutcome::Returned(value))),
        }
    }

    fn close(&mut self) {
        if self.closed {
            return;
        }
        self.closed = true;
        self.entries.close();
        self.work.close();
        self.units.close();
        for task in self.tasks.iter() {
            task.cancel();
        }
    }

    fn service(&mut self, cx: &mut Context<'_>) {
        // Bound each service turn, including while the Rust body waits.
        let mut remaining = self.budget.get();
        while remaining > 0 {
            let finished = self.units.finish_next(cx);
            let progress = self
                .units
                .state()
                .poll(cx, ExecutionClock::new(self.host))
                .is_ready();
            let entry = self.entries.next(cx);
            let spawned = self.units.next_spawn();
            let request = self.work.next(cx);
            let empty =
                !finished && !progress && entry.is_none() && spawned.is_none() && request.is_none();
            if let Some(entry) = entry
                && entry.owner.handle().is_active()
            {
                let (context, root) = self.begin(entry.owner);
                self.tasks
                    .push(self.host.spawn((entry.start)(context, root)));
            }
            if let Some(spawned) = spawned {
                let worker =
                    spawned.into_worker(Arc::clone(&self.plan), self.work.execution(), self.budget);
                self.tasks.push(self.host.spawn(worker));
            }
            if let Some(request) = request {
                let charged = self.dispatch(request, remaining);
                remaining = remaining.saturating_sub(charged);
            } else {
                remaining -= 1;
            }
            if self.closed {
                return;
            }
            if empty {
                return;
            }
        }
        cx.waker().wake_by_ref();
    }

    fn dispatch(&mut self, request: Request<Profile>, available: usize) -> usize {
        match request {
            Request::Service(request) => {
                let allowance = request.allowance(available);
                let context = self.work.execution().with_unit(request.unit().cloned());
                let delivery = {
                    let captures = context.services().captures().clone();
                    let mut runtime = RuntimeState::with_host_storage(
                        &mut *self.echo,
                        RuntimeHost::<Profile>::new(
                            &mut *self.state,
                            &*self.stores,
                            &self.work,
                            &mut self.units,
                            context,
                            ExecutionClock::new(self.host),
                        ),
                        self.lists.clone(),
                        captures,
                    );
                    request.service(&self.plan, &mut runtime, allowance)
                };
                if self.work.exit_status().is_some() {
                    self.close();
                }
                if let Some(delivery) = delivery {
                    delivery.deliver();
                }
                allowance
            }
            Request::Callback(request) => {
                let (context, root) = match request.unit() {
                    Some(unit) if !unit.is_active() => return 1,
                    Some(unit) => (self.work.execution().with_unit(Some(unit.clone())), None),
                    None => {
                        let (context, root) = self.begin(UnitOwner::new(self.units.completion()));
                        (context, Some(root))
                    }
                };
                let worker =
                    request.into_worker(Arc::clone(&self.plan), context, self.budget, root);
                self.tasks.push(self.host.spawn(worker));
                1
            }
        }
    }

    fn begin(&mut self, owner: UnitOwner) -> (ExecutionContext<Profile>, super::unit::Root) {
        let unit = owner.handle();
        let root = self.units.begin(owner);
        (self.work.execution().with_unit(Some(unit)), root)
    }

    fn finish_units(&mut self, cx: &mut Context<'_>) {
        for _ in 0..self.budget.get() {
            if !self.units.finish_next(cx) {
                return;
            }
        }
        cx.waker().wake_by_ref();
    }

    fn reap(&mut self, cx: &mut Context<'_>) -> Option<DriverError> {
        let mut failure = None;
        for _ in 0..self.budget.get() {
            match self.tasks.poll_next_unpin(cx) {
                Poll::Pending | Poll::Ready(None) => return failure,
                Poll::Ready(Some(TaskExit::Cancelled)) if !self.closed => {
                    failure.get_or_insert(DriverError::Cancelled);
                }
                Poll::Ready(Some(TaskExit::Failed(error))) => {
                    if !matches!(failure, Some(DriverError::Failed(_))) {
                        failure = Some(DriverError::Failed(error));
                    }
                }
                Poll::Ready(Some(TaskExit::Completed | TaskExit::Cancelled)) => {}
            }
        }
        if !self.tasks.is_empty() {
            cx.waker().wake_by_ref();
        }
        failure
    }
}

impl<Profile: HostProfile> Drop for Domain<'_, Profile> {
    fn drop(&mut self) {
        self.close();
    }
}

impl<Profile: HostProfile> EntryContext<Profile> {
    pub(crate) fn captures(&self) -> &crate::runtime::CaptureStorage {
        self.execution.services().captures()
    }

    pub(in crate::runtime) async fn run_main(
        &self,
    ) -> Result<crate::Value, crate::execution::RunError> {
        let plan = Arc::clone(&self.plan);
        let budget = self.budget;
        let owner = UnitOwner::new(self.completion.clone());
        let _cancel = super::unit::CancelOnDrop(owner.handle());
        self.entries
            .submit(|reply| Entry {
                owner,
                start: Box::new(move |context, root| {
                    super::worker::completing(
                        reply,
                        root.run(async move {
                            let returned = crate::runtime::function::prepare_main(&plan)
                                .submit(context.services(), budget)
                                .await?;
                            Ok(returned.map(|value| {
                                crate::runtime::materialize::value(
                                    plan.value_metadata(),
                                    &RuntimeListStorage::default(),
                                    value,
                                )
                            }))
                        }),
                    )
                }),
            })
            .await
            .map_err(|_| crate::execution::RunError::Cancelled)?
            .map_err(Into::into)
    }

    pub(in crate::runtime) fn invoke(
        &self,
        callable: crate::runtime::RetainedCallable,
        origin: HostCallOrigin,
        inputs: crate::runtime::CallbackInputs,
    ) -> impl Future<Output = Result<ExecutionResult<crate::runtime::EvaluatedValue>, Cancelled>>
    + Send
    + use<Profile> {
        let plan = Arc::clone(&self.plan);
        let budget = self.budget;
        let entries = self.entries.clone();
        let completion = self.completion.clone();
        async move {
            let owner = UnitOwner::new(completion);
            let _cancel = super::unit::CancelOnDrop(owner.handle());
            entries
                .submit(|reply| Entry {
                    owner,
                    start: Box::new(move |context, root| {
                        super::worker::completing(
                            reply,
                            root.run(async move {
                                let invocation = callable.with_value(|function| {
                                    crate::runtime::function::prepare_callable(
                                        plan.as_ref(),
                                        function,
                                        origin,
                                        inputs.into_arguments(),
                                    )
                                });
                                invocation.submit(context.services(), budget).await
                            }),
                        )
                    }),
                })
                .await
        }
    }

    pub(crate) async fn retain_outputs<Output: Send + 'static>(
        &self,
        retain: impl FnOnce(&Profile::ExternalStores) -> Output + Send + 'static,
    ) -> Result<Output, Cancelled> {
        self.execution
            .with_runtime(move |_, state| retain(state.host().stores()))
            .await
    }

    pub(in crate::runtime) fn call<Id>(
        &self,
        function: Id,
        origin: HostCallOrigin,
        inputs: RetainedValues,
    ) -> impl Future<
        Output = Result<ExecutionResult<EntryValue<HostedProgram<Profile>, Id>>, Cancelled>,
    > + Send
    + use<Profile, Id>
    where
        Id: EntryTarget<HostedProgram<Profile>> + 'static,
    {
        let plan = Arc::clone(&self.plan);
        let budget = self.budget;
        let entries = self.entries.clone();
        let completion = self.completion.clone();
        async move {
            let owner = UnitOwner::new(completion);
            let _cancel = super::unit::CancelOnDrop(owner.handle());
            entries
                .submit(|reply| Entry {
                    owner,
                    start: Box::new(move |context, root| {
                        super::worker::completing(
                            reply,
                            root.run(async move {
                                Execution::new(function, origin, inputs)
                                    .drive(&plan, context.services(), budget)
                                    .await
                            }),
                        )
                    }),
                })
                .await
        }
    }
}

#[cfg(test)]
mod tests {
    mod factory;

    use super::{Domain, Request};
    use crate::execution::{ExecutionHost, HostTask, TaskExit, Worker};
    use crate::host::{HostProfile, HostProviderSet};
    use crate::plan::execution::{HostedProgram, LibraryFunctionEntries};
    use crate::plan::{LibraryEntry, LibraryValueType};
    use crate::runtime::work::Cancelled;
    use crate::runtime::{EchoOutput, EchoSink, HostCallOrigin, RetainedValues};
    use futures_channel::oneshot;
    use futures_util::future::{AbortHandle, Abortable};
    use parking_lot::Mutex;
    use std::cell::Cell;
    use std::future::{Future, poll_fn};
    use std::num::NonZeroUsize;
    use std::pin::Pin;
    use std::sync::Arc;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::task::{Context, Poll, Waker};
    use std::time::Instant;

    #[test]
    fn native_loop_requests_resume_in_the_domain_and_leave_computed_producers_canonical() {
        use crate::execution_fixture::TestHost;
        use crate::plan::execution::compiled::CompiledImplementation;
        use crate::plan::execution::compiled::{
            NativeLoopContract, NativeLoopImplementation, NativeLoopTarget,
        };
        use crate::plan::execution::function::TupleFunctionId;
        use crate::plan::execution::function::{
            ExecutionFunctionEntry, ExecutionFunctionRef, IntFunctionId,
        };
        use crate::plan::execution::graph::IntLocalId;
        use crate::plan::execution::runtime::RuntimeExecutionPlan;
        use crate::runtime::RuntimeListStorage;
        use crate::runtime::compiled::native_loop;
        use crate::runtime::execution::Domain;
        use crate::runtime::graph::{
            GraphExecution as Execution, GraphProgress as Progress, GraphStorage as Storage,
        };
        use crate::runtime::state::RuntimeState;
        use crate::{
            HostCall, HostCallCompletion, HostCallError, HostProvider, HostProviderModule,
            HostProviderSet, StatelessHostProfile,
        };
        use num_bigint::BigInt;
        use std::num::NonZeroUsize;

        struct Provider;
        impl HostProvider<StatelessHostProfile> for Provider {
            type State = ();
            fn project(state: &mut ()) -> &mut () {
                state
            }
        }
        fn observe<'call>(
            mut call: HostCall<'call, StatelessHostProfile, Provider, BigInt>,
            value: BigInt,
        ) -> Result<HostCallCompletion<'call, BigInt>, HostCallError> {
            let _ = call.state();
            Ok(call.return_value(value + 1))
        }
        let source = r#"
@external(erlang, "native", "observe")
fn observe(value: Int) -> Int
fn cycle(counter: Int, producer: fn() -> Int) -> Int {
  let result = observe(producer())
  case counter { 1 -> result _ -> cycle(counter - 1, producer) }
}
fn captured(value: Int) { fn() { value } }
fn computed(value: Int) { fn() { value + 1 } }
pub fn main() { #(cycle, captured(7), computed(7)) }
"#;
        let typed = crate::compile_typed_host_program(
            "example",
            "example",
            [crate::PackageSource::new(
                "example",
                Vec::<String>::new(),
                [crate::ModuleSource::new(
                    "example",
                    "src/example.gleam",
                    source,
                )],
            )],
            HostProviderSet::from_providers([HostProviderModule::new("example", "example")
                .unwrap()
                .with_scoped_retained_function::<Provider, BigInt, BigInt, _, _>(
                    "observe",
                    observe,
                    |value: BigInt| Ok(value + 1),
                )
                .unwrap()])
            .unwrap(),
        )
        .unwrap();
        let mut hosted =
            crate::HostedExecution::try_from_module_plan(crate::plan_host_program(typed).unwrap())
                .unwrap();
        let (plan, stores, captures) = hosted.parts_mut();
        // The source has three Int graph bodies and one native Int entry.
        let graphs = (0..4)
            .filter_map(
                |index| match plan.int_function(IntFunctionId(index)).as_ref() {
                    ExecutionFunctionRef::Graph(entry) => {
                        Some((IntFunctionId(index), entry.body()))
                    }
                    ExecutionFunctionRef::Host(_) => None,
                },
            )
            .collect::<Vec<_>>();
        assert_eq!(graphs.len(), 3);
        let (id, body) = graphs[0];
        let contract = NativeLoopContract::inspect(body.block_graph()).unwrap();
        assert_eq!(contract.site.function(), "cycle");
        let finished = contract.exit;
        let implementation = CompiledImplementation::NativeLoop(
            Box::new(NativeLoopImplementation {
                function: NativeLoopTarget::Int(id),
                entry: 0,
                checkpoints: vec![contract.checkpoint()].into(),
                contract,
                run: native_loop::run,
            })
            .into(),
        );
        let host = TestHost::default();
        let mut run_state = ();
        let mut echo = Vec::new();
        let domain = Domain::new(
            Arc::clone(plan),
            &host,
            &mut run_state,
            stores,
            &mut echo,
            captures.clone(),
            Domain::<StatelessHostProfile>::DEFAULT_BUDGET,
        );
        let context = domain.context();
        host.block_on(domain.drive(async {
            let values = context
                .call(
                    TupleFunctionId(0),
                    HostCallOrigin::Entry,
                    RetainedValues::empty(),
                )
                .await
                .unwrap()
                .unwrap();
            assert_eq!(values.len(), 3);
            for (producer, expected, native) in [
                (values[1].clone(), BigInt::from(8), true),
                (values[2].clone(), BigInt::from(9), false),
            ] {
                let mut worker_echo = Vec::new();
                let mut state = RuntimeState::with_host_storage(
                    &mut worker_echo,
                    (),
                    RuntimeListStorage::default(),
                    context.captures().clone(),
                );
                let mut inputs = RetainedValues::empty();
                inputs.push_int(3.into());
                inputs.push_evaluated(producer);
                let mut execution =
                    Execution::new(body.block_graph().as_view(), inputs, Some(&implementation));
                let mut storage = Storage::new();
                let mut requests = 0;
                let result = loop {
                    match execution
                        .advance(&**plan, &mut state, &mut storage, &mut 0)
                        .unwrap()
                    {
                        Progress::Host(invocation) => {
                            requests += 1;
                            execution = invocation
                                .submit(context.execution.services(), NonZeroUsize::new(1).unwrap())
                                .await
                                .unwrap()
                                .unwrap();
                        }
                        progress => {
                            match crate::runtime::graph::tests::canonical_progress(progress) {
                                crate::runtime::graph::tests::CanonicalProgress::Continue(next) => {
                                    execution = next
                                }
                                crate::runtime::graph::tests::CanonicalProgress::Complete(
                                    completed,
                                ) => {
                                    assert_eq!(completed.exit(), finished);
                                    break completed.into_value(&IntLocalId(0)).into_bigint();
                                }
                            }
                        }
                    }
                };
                assert_eq!(result, expected);
                assert_eq!(requests, if native { 22 } else { 3 });
                assert!(worker_echo.is_empty());
            }
        }))
        .unwrap();
        assert!(echo.is_empty());
    }

    struct Profile;
    impl HostProfile for Profile {
        type RunState = Cell<usize>;
        type ExternalStores = Cell<()>;
        type ExecutionState = ();
    }

    #[derive(Default)]
    struct ManualHost {
        workers: Mutex<Vec<Worker>>,
        next_exit: Mutex<Option<TaskExit>>,
        acknowledgement_delay: AtomicUsize,
        released: Arc<AtomicUsize>,
        started: AtomicUsize,
        turns: AtomicUsize,
    }

    struct ManualTask {
        abort: AbortHandle,
        exit: oneshot::Receiver<TaskExit>,
        acknowledgement_delay: usize,
        completed: Option<TaskExit>,
    }

    struct Release(Arc<AtomicUsize>);
    impl Drop for Release {
        fn drop(&mut self) {
            self.0.fetch_add(1, Ordering::SeqCst);
        }
    }

    impl ExecutionHost for ManualHost {
        fn spawn(&self, worker: Worker) -> Box<dyn HostTask> {
            self.started.fetch_add(1, Ordering::SeqCst);
            let (abort, registration) = AbortHandle::new_pair();
            let (reply, exit) = oneshot::channel();
            let release = Release(Arc::clone(&self.released));
            let forced_exit = self.next_exit.lock().take();
            self.workers.lock().push(Box::pin(async move {
                let exit = {
                    let _release = release;
                    match forced_exit {
                        Some(exit) => {
                            drop(worker);
                            exit
                        }
                        None => match Abortable::new(worker, registration).await {
                            Ok(()) => TaskExit::Completed,
                            Err(_) => TaskExit::Cancelled,
                        },
                    }
                };
                let _ = reply.send(exit);
            }));
            Box::new(ManualTask {
                abort,
                exit,
                acknowledgement_delay: self.acknowledgement_delay.load(Ordering::SeqCst),
                completed: None,
            })
        }

        fn now(&self) -> Instant {
            Instant::now()
        }
        fn sleep_until(&self, _deadline: Instant) -> Worker {
            Box::pin(std::future::pending())
        }
    }

    impl Future for ManualTask {
        type Output = TaskExit;
        fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
            let exit = match self.completed.take() {
                Some(exit) => exit,
                None => std::task::ready!(Pin::new(&mut self.exit).poll(cx))
                    .unwrap_or(TaskExit::Cancelled),
            };
            if self.acknowledgement_delay == 0 {
                Poll::Ready(exit)
            } else {
                self.acknowledgement_delay -= 1;
                self.completed = Some(exit);
                cx.waker().wake_by_ref();
                Poll::Pending
            }
        }
    }

    impl HostTask for ManualTask {
        fn cancel(&self) {
            self.abort.abort();
        }
    }

    impl Drop for ManualTask {
        fn drop(&mut self) {
            self.cancel();
        }
    }

    impl ManualHost {
        fn turn(&self) {
            self.turns.fetch_add(1, Ordering::SeqCst);
            let workers = std::mem::take(&mut *self.workers.lock());
            for mut worker in workers {
                let pending = std::thread::scope(|threads| {
                    threads
                        .spawn(move || {
                            let progress = worker
                                .as_mut()
                                .poll(&mut Context::from_waker(Waker::noop()));
                            progress.is_pending().then_some(worker)
                        })
                        .join()
                        .expect("an owned worker can move between host threads")
                });
                if let Some(worker) = pending {
                    self.workers.lock().push(worker);
                }
            }
        }

        fn finish<Output>(&self, running: impl Future<Output = Output>) -> Output {
            let mut running = std::pin::pin!(running);
            for _ in 0..10_000 {
                let progress = running
                    .as_mut()
                    .poll(&mut Context::from_waker(Waker::noop()));
                if let Poll::Ready(output) = progress {
                    return output;
                }
                self.turn();
            }
            panic!("the bounded host did not finish");
        }
    }

    fn program(
        source: &str,
        result: LibraryValueType,
    ) -> (Arc<HostedProgram<Profile>>, LibraryFunctionEntries) {
        let typed = crate::compile_typed_host_program(
            "application",
            "library",
            [crate::PackageSource::new(
                "application",
                Vec::<String>::new(),
                [crate::ModuleSource::new(
                    "library",
                    "src/library.gleam",
                    source,
                )],
            )],
            HostProviderSet::from_providers([]).unwrap(),
        )
        .unwrap();
        let library = crate::planner::plan_host_library_program(typed).unwrap();
        let function = library
            .functions()
            .iter()
            .find(|function| function.name() == "main")
            .unwrap();
        let entry = LibraryEntry::new(function.signature().id(), result, Vec::new(), Vec::new());
        let (plan, entries, _) =
            HostedProgram::from_library_plan(library, entry, Vec::new()).unwrap();
        (Arc::new(plan), entries)
    }

    struct ProgressEcho(Arc<[AtomicUsize; 2]>);
    impl EchoSink for ProgressEcho {
        fn emit(&mut self, output: EchoOutput) {
            let index: usize = output.value().inspect().to_string().parse().unwrap();
            self.0[index].fetch_add(1, Ordering::SeqCst);
        }
    }

    #[test]
    fn bounded_native_loop_requests_leave_other_requests_for_the_next_service_turn() {
        let (plan, _) = program("pub fn main() { Nil }", LibraryValueType::Nil);
        for budget in [1, 2, 7, 1024] {
            let host = ManualHost::default();
            let mut state = Cell::new(7);
            let mut stores = Cell::new(());
            let mut echo = Vec::new();
            let mut domain = Domain::new(
                Arc::clone(&plan),
                &host,
                &mut state,
                &mut stores,
                &mut echo,
                Default::default(),
                NonZeroUsize::new(budget).unwrap(),
            );
            let context = domain.context().execution;
            let mut bounded = Box::pin(context.services().submit_bounded(
                usize::MAX,
                |_, state, granted| {
                    assert_eq!(state.host_state().get(), 7);
                    state.host_state().set(8);
                    granted
                },
            ));
            let mut ordinary = Box::pin(context.with_state(|state| {
                assert_eq!(state.get(), 8);
                state.set(9);
                42
            }));
            let mut cx = Context::from_waker(Waker::noop());
            assert_eq!(bounded.as_mut().poll(&mut cx), Poll::Pending);
            assert_eq!(ordinary.as_mut().poll(&mut cx), Poll::Pending);
            domain.service(&mut cx);
            assert_eq!(bounded.as_mut().poll(&mut cx), Poll::Ready(Ok(budget)));
            assert_eq!(ordinary.as_mut().poll(&mut cx), Poll::Pending);
            domain.service(&mut cx);
            assert_eq!(ordinary.as_mut().poll(&mut cx), Poll::Ready(Ok(42)));
            drop(domain);
            assert_eq!(state.get(), 9);
        }
    }

    #[test]
    #[should_panic(expected = "the bounded host did not finish")]
    fn manual_host_accepts_completion_and_rejects_an_unfinished_test_clock() {
        let host = ManualHost::default();
        for operation in [
            Box::pin(std::future::ready(())) as Worker,
            host.sleep_until(host.now()),
        ] {
            host.finish(operation);
        }
    }

    #[test]
    fn executor_failure_ends_the_domain_after_worker_destruction() {
        for (exit, message) in [
            (
                TaskExit::Cancelled,
                "the host executor cancelled an active worker",
            ),
            (
                TaskExit::Failed(std::io::Error::other("worker unavailable").into()),
                "the host executor failed: worker unavailable",
            ),
        ] {
            let (plan, functions) = program("pub fn main() { echo 42 42 }", LibraryValueType::Int);
            let host = ManualHost::default();
            *host.next_exit.lock() = Some(exit);
            let mut state = Cell::new(7);
            let mut stores = Cell::new(());
            let mut echo = Vec::new();
            let domain = Domain::new(
                plan,
                &host,
                &mut state,
                &mut stores,
                &mut echo,
                Default::default(),
                Domain::<Profile>::DEFAULT_BUDGET,
            );
            let context = domain.context();
            let error = host
                .finish(domain.drive(context.call(
                    *functions.ints[0].function(),
                    HostCallOrigin::Entry,
                    RetainedValues::empty(),
                )))
                .unwrap_err();
            assert_eq!(error.to_string(), message);
            assert_eq!(host.started.load(Ordering::SeqCst), 1);
            assert_eq!(host.released.load(Ordering::SeqCst), 1);
            assert_eq!(state.get(), 7);
            assert!(echo.is_empty());
            assert!(host.workers.lock().is_empty());
        }
    }

    #[test]
    fn intentional_exit_closes_admission_and_waits_for_worker_acknowledgement() {
        use crate::execution::{ExecutionOutcome, ExitStatus};

        let (plan, _) = program("pub fn main() { Nil }", LibraryValueType::Nil);
        let host = ManualHost::default();
        host.acknowledgement_delay.store(2, Ordering::SeqCst);
        let mut state = Cell::new(7);
        let mut stores = Cell::new(());
        let mut echo = Vec::new();
        let domain = Domain::new(
            Arc::clone(&plan),
            &host,
            &mut state,
            &mut stores,
            &mut echo,
            Default::default(),
            NonZeroUsize::MIN,
        );
        domain
            .tasks
            .push(host.spawn(Box::pin(std::future::pending())));
        let context = domain.context().execution;
        let exit_context = context.clone();
        let request = context.with_runtime(move |_, _| {
            exit_context.services().request_exit(ExitStatus::new(7));
            exit_context.services().request_exit(ExitStatus::new(0));
        });
        let mutate = |state: &mut Cell<usize>| state.set(42);
        let mut denied = Box::pin(context.with_state(mutate));
        let mut driving = Box::pin(domain.drive(request));
        let mut cx = Context::from_waker(Waker::noop());
        assert!(driving.as_mut().poll(&mut cx).is_pending());
        assert_eq!(host.released.load(Ordering::SeqCst), 0);
        assert_eq!(denied.as_mut().poll(&mut cx), Poll::Ready(Err(Cancelled)));
        host.turn();
        assert_eq!(host.released.load(Ordering::SeqCst), 1);
        assert!(driving.as_mut().poll(&mut cx).is_pending());
        assert_eq!(
            host.finish(driving).unwrap(),
            ExecutionOutcome::Exited(ExitStatus::new(7))
        );
        assert!(host.workers.lock().is_empty());
        assert_eq!(state.get(), 7);
        assert_eq!(host.finish(context.with_state(mutate)), Err(Cancelled));
        let domain = Domain::new(
            plan,
            &host,
            &mut state,
            &mut stores,
            &mut echo,
            Default::default(),
            NonZeroUsize::MIN,
        );
        let normal = domain.context().execution.with_state(mutate);
        assert_eq!(
            host.finish(domain.drive(normal)).unwrap(),
            ExecutionOutcome::Returned(Ok(()))
        );
        assert_eq!(state.get(), 42);
        assert!(echo.is_empty());
    }

    #[test]
    fn a_real_executor_failure_during_exit_cleanup_overrides_the_status() {
        use crate::execution::ExitStatus;

        let (plan, _) = program("pub fn main() { Nil }", LibraryValueType::Nil);
        let host = ManualHost::default();
        host.acknowledgement_delay.store(2, Ordering::SeqCst);
        *host.next_exit.lock() = Some(TaskExit::Failed(
            std::io::Error::other("exit cleanup failed").into(),
        ));
        let mut state = Cell::new(7);
        let mut stores = Cell::new(());
        let mut echo = Vec::new();
        let domain = Domain::new(
            plan,
            &host,
            &mut state,
            &mut stores,
            &mut echo,
            Default::default(),
            NonZeroUsize::MIN,
        );
        domain
            .tasks
            .push(host.spawn(Box::pin(std::future::pending())));
        let context = domain.context().execution;
        let exit_context = context.clone();
        let request = context
            .with_runtime(move |_, _| exit_context.services().request_exit(ExitStatus::new(0)));
        let result = host.finish(domain.drive(request));
        assert_eq!(
            result.unwrap_err().to_string(),
            "the host executor failed: exit cleanup failed"
        );
        assert_eq!(host.released.load(Ordering::SeqCst), 1);
        assert!(host.workers.lock().is_empty());
        assert_eq!(state.get(), 7);
        assert!(echo.is_empty());
    }

    #[test]
    fn late_executor_failure_survives_entry_cancellation_and_complete_shutdown() {
        let (plan, functions) = program("pub fn main() { echo 42 42 }", LibraryValueType::Int);
        let host = ManualHost::default();
        host.acknowledgement_delay.store(2, Ordering::SeqCst);
        *host.next_exit.lock() = Some(TaskExit::Failed(
            std::io::Error::other("worker failed before acknowledgement").into(),
        ));
        let mut state = Cell::new(7);
        let mut stores = Cell::new(());
        let mut echo = Vec::new();
        let domain = Domain::new(
            plan,
            &host,
            &mut state,
            &mut stores,
            &mut echo,
            Default::default(),
            NonZeroUsize::MIN,
        );
        let context = domain.context();
        let result = host.finish(domain.drive(context.call(
            *functions.ints[0].function(),
            HostCallOrigin::Entry,
            RetainedValues::empty(),
        )));
        assert_eq!(
            result.err().map(|error| error.to_string()).as_deref(),
            Some("the host executor failed: worker failed before acknowledgement")
        );
        assert_eq!(host.started.load(Ordering::SeqCst), 1);
        assert_eq!(host.released.load(Ordering::SeqCst), 1);
        assert!(host.workers.lock().is_empty());
        assert_eq!(state.get(), 7);
        assert!(echo.is_empty());
    }

    #[test]
    fn failed_tasks_take_precedence_over_cancellation_in_driving_and_shutdown() {
        let (plan, _) = program("pub fn main() { 42 }", LibraryValueType::Int);
        for budget in [NonZeroUsize::MIN, Domain::<Profile>::DEFAULT_BUDGET] {
            for body_ready in [false, true] {
                for cancelled_first in [false, true] {
                    for delay in [0, 2] {
                        let host = ManualHost::default();
                        host.acknowledgement_delay.store(delay, Ordering::SeqCst);
                        let mut state = Cell::new(7);
                        let mut stores = Cell::new(());
                        let mut echo = Vec::new();
                        let domain = Domain::new(
                            Arc::clone(&plan),
                            &host,
                            &mut state,
                            &mut stores,
                            &mut echo,
                            Default::default(),
                            budget,
                        );
                        for cancelled in [cancelled_first, !cancelled_first] {
                            *host.next_exit.lock() = Some(if cancelled {
                                TaskExit::Cancelled
                            } else {
                                TaskExit::Failed(std::io::Error::other("original failure").into())
                            });
                            domain
                                .tasks
                                .push(host.spawn(Box::pin(std::future::pending())));
                        }
                        host.turn();
                        let result = host.finish(domain.drive(poll_fn(|_| {
                            if body_ready {
                                Poll::Ready(42)
                            } else {
                                Poll::Pending
                            }
                        })));
                        let error = result.unwrap_err();
                        assert_eq!(
                            error.to_string(),
                            "the host executor failed: original failure"
                        );
                        assert_eq!(
                            std::error::Error::source(&error).unwrap().to_string(),
                            "original failure"
                        );
                        assert_eq!(host.started.load(Ordering::SeqCst), 2);
                        assert_eq!(host.released.load(Ordering::SeqCst), 2);
                        assert!(host.workers.lock().is_empty());
                        assert_eq!(state.get(), 7);
                        assert!(echo.is_empty());
                    }
                }
            }
        }
    }

    #[test]
    fn a_selected_host_failure_survives_later_failures_and_requested_cancellation() {
        let (plan, _) = program("pub fn main() { Nil }", LibraryValueType::Nil);
        for budget in [NonZeroUsize::MIN, Domain::<Profile>::DEFAULT_BUDGET] {
            // Equal simultaneous causes do not impose an order on independent failures.
            for (delay, later) in [(0, "first"), (2, "later")] {
                let host = ManualHost::default();
                let mut state = Cell::new(0);
                let mut stores = Cell::new(());
                let mut echo = Vec::new();
                let domain = Domain::new(
                    Arc::clone(&plan),
                    &host,
                    &mut state,
                    &mut stores,
                    &mut echo,
                    Default::default(),
                    budget,
                );
                *host.next_exit.lock() =
                    Some(TaskExit::Failed(std::io::Error::other("first").into()));
                domain
                    .tasks
                    .push(host.spawn(Box::pin(std::future::pending())));
                host.acknowledgement_delay.store(delay, Ordering::SeqCst);
                *host.next_exit.lock() =
                    Some(TaskExit::Failed(std::io::Error::other(later).into()));
                domain
                    .tasks
                    .push(host.spawn(Box::pin(std::future::pending())));
                domain
                    .tasks
                    .push(host.spawn(Box::pin(std::future::pending())));
                let error = host
                    .finish(domain.drive(std::future::pending::<()>()))
                    .unwrap_err();
                assert_eq!(error.to_string(), "the host executor failed: first");
                assert_eq!(host.started.load(Ordering::SeqCst), 3);
                assert_eq!(host.released.load(Ordering::SeqCst), 3);
                assert!(host.workers.lock().is_empty());
            }
        }
    }

    #[test]
    fn requested_shutdown_cancellation_preserves_source_results_and_errors() {
        for (source, expected) in [
            ("pub fn main() { echo 41 42 }", Ok("42")),
            (
                "pub fn main() -> Int { echo 41 panic as \"source failure\" }",
                Err("panic: source failure"),
            ),
        ] {
            let (plan, functions) = program(source, LibraryValueType::Int);
            let host = ManualHost::default();
            host.acknowledgement_delay.store(2, Ordering::SeqCst);
            let mut state = Cell::new(7);
            let mut stores = Cell::new(());
            let mut echo = Vec::new();
            let domain = Domain::new(
                plan,
                &host,
                &mut state,
                &mut stores,
                &mut echo,
                Default::default(),
                NonZeroUsize::MIN,
            );
            domain
                .tasks
                .push(host.spawn(Box::pin(std::future::pending())));
            let context = domain.context();
            let result = host
                .finish(domain.drive(context.call(
                    *functions.ints[0].function(),
                    HostCallOrigin::Entry,
                    RetainedValues::empty(),
                )))
                .unwrap()
                .try_into_value()
                .unwrap()
                .unwrap();
            assert_eq!(
                result
                    .map(|value| value.to_string())
                    .map_err(|error| error.to_string()),
                expected.map(str::to_owned).map_err(str::to_owned),
            );
            assert_eq!(host.started.load(Ordering::SeqCst), 2);
            assert_eq!(host.released.load(Ordering::SeqCst), 2);
            assert!(host.workers.lock().is_empty());
            assert_eq!(echo.len(), 1);
            assert_eq!(echo[0].value(), &crate::Value::Int(41.into()));
            assert_eq!(state.get(), 7);
        }
    }

    #[test]
    fn independent_cpu_entries_yield_cancel_and_leave_borrowed_state_with_the_driver() {
        for budget in [1, 17] {
            let (plan, functions) = program(
                "pub fn main(identity: Int) -> Int { echo identity main(identity) }",
                LibraryValueType::Int,
            );
            let host = ManualHost::default();
            let mut state = Cell::new(0);
            let mut stores = Cell::new(());
            let progress = Arc::new([AtomicUsize::new(0), AtomicUsize::new(0)]);
            let mut echo = ProgressEcho(Arc::clone(&progress));
            let domain = Domain::new(
                plan,
                &host,
                &mut state,
                &mut stores,
                &mut echo,
                Default::default(),
                NonZeroUsize::new(budget).unwrap(),
            );
            let context = domain.context();
            let id = *functions.ints[0].function();
            let mut left = RetainedValues::empty();
            left.push_int(0.into());
            let mut right = RetainedValues::empty();
            right.push_int(1.into());
            let result = host.finish(domain.drive(async {
                let mut left = Box::pin(context.call(id, HostCallOrigin::Entry, left));
                let mut right = Box::pin(context.call(id, HostCallOrigin::Entry, right));
                poll_fn(|cx| {
                    assert!(left.as_mut().poll(cx).is_pending());
                    assert!(right.as_mut().poll(cx).is_pending());
                    if progress
                        .iter()
                        .all(|value| value.load(Ordering::SeqCst) >= 10)
                    {
                        Poll::Ready(())
                    } else {
                        Poll::Pending
                    }
                })
                .await;
                drop(left);
                context
                    .execution
                    .with_state(|state| state.set(42))
                    .await
                    .unwrap();
                let previous = progress[1].load(Ordering::SeqCst);
                poll_fn(|cx| {
                    assert!(right.as_mut().poll(cx).is_pending());
                    if progress[1].load(Ordering::SeqCst) > previous + 10 {
                        Poll::Ready(())
                    } else {
                        Poll::Pending
                    }
                })
                .await;
                drop(right);
            }));
            result.unwrap();
            assert_eq!(state.get(), 42);
            assert_eq!(host.started.load(Ordering::SeqCst), 2);
            assert_eq!(host.released.load(Ordering::SeqCst), 2);
            assert!(host.workers.lock().is_empty());
            assert!(progress[0].load(Ordering::SeqCst) < progress[1].load(Ordering::SeqCst));
        }
    }

    #[test]
    fn driven_source_uses_short_host_requests_and_moves_without_host_borrows() {
        use crate::{
            HostCall, HostCallCompletion, HostCallError, HostProvider, HostProviderModule,
            ModuleSource, PackageSource,
        };
        use num_bigint::BigInt;
        struct Provider;
        impl HostProvider<Profile> for Provider {
            type State = Cell<usize>;
            fn project(state: &mut Self::State) -> &mut Self::State {
                state
            }
        }
        fn increment<'call>(
            mut call: HostCall<'call, Profile, Provider, BigInt>,
            value: BigInt,
        ) -> Result<HostCallCompletion<'call, BigInt>, HostCallError> {
            let state = call.state();
            state.set(state.get() + 1);
            Ok(call.return_value(value + 1))
        }
        let provider = HostProviderModule::new("application", "library")
            .unwrap()
            .with_scoped_function::<Provider, (BigInt,), BigInt, _>("increment", increment)
            .unwrap();
        let typed = crate::compile_typed_host_program(
            "application",
            "library",
            [PackageSource::new(
                "application",
                Vec::<String>::new(),
                [ModuleSource::new(
                    "library",
                    "src/library.gleam",
                    r#"
@external(erlang, "native", "increment")
fn increment(value: Int) -> Int
fn sum(n: Int, total: Int) -> Int {
  case n { 0 -> total _ -> sum(n - 1, total + 1) }
}
pub fn main() { echo 41 increment(sum(2_000, 0) - 1959) }
"#,
                )],
            )],
            HostProviderSet::from_providers([provider]).unwrap(),
        )
        .unwrap();
        let library = crate::planner::plan_host_library_program(typed).unwrap();
        let function = library
            .functions()
            .iter()
            .find(|function| function.name() == "main")
            .unwrap();
        let entry = LibraryEntry::new(
            function.signature().id(),
            LibraryValueType::Int,
            Vec::new(),
            Vec::new(),
        );
        let (plan, entries, _) =
            HostedProgram::from_library_plan(library, entry, Vec::new()).unwrap();
        let host = ManualHost::default();
        let mut state = Cell::new(0);
        let mut stores = Cell::new(());
        let mut echo = Vec::new();
        let domain = Domain::new(
            Arc::new(plan),
            &host,
            &mut state,
            &mut stores,
            &mut echo,
            Default::default(),
            NonZeroUsize::new(37).unwrap(),
        );
        let entry = domain.context();
        let output = host
            .finish(domain.drive(entry.call(
                *entries.ints[0].function(),
                HostCallOrigin::Entry,
                RetainedValues::empty(),
            )))
            .unwrap()
            .try_into_value()
            .unwrap()
            .unwrap();
        assert_eq!(output.map(Into::into), Ok(BigInt::from(42)));
        assert!(
            host.turns.load(Ordering::SeqCst) > 100,
            "the source yields during its CPU loop"
        );
        assert_eq!(host.started.load(Ordering::SeqCst), 1);
        assert_eq!(host.released.load(Ordering::SeqCst), 1);
        assert_eq!(state.get(), 1);
        assert_eq!(
            echo.iter().map(ToString::to_string).collect::<Vec<_>>(),
            ["src/library.gleam:7\n41"]
        );
    }

    #[test]
    fn native_waits_and_reentrant_callbacks_preserve_effects_across_budget_boundaries() {
        use crate::{
            HostCall, HostCallCompletion, HostCallContinuation, HostCallError, HostCallable,
            HostConstructions, HostFailure, HostFunctionType, HostOwnedCompletion, HostProvider,
            HostProviderModule, HostTypeList, HostTypeListEnd, ModuleSource, PackageSource,
        };
        use num_bigint::BigInt;

        struct Provider;
        impl HostProvider<Profile> for Provider {
            type State = Cell<usize>;
            fn project(state: &mut Self::State) -> &mut Self::State {
                state
            }
        }
        fn bump<'call>(
            mut call: HostCall<'call, Profile, Provider, BigInt>,
            fail: bool,
        ) -> Result<HostCallCompletion<'call, BigInt>, HostCallError> {
            let state = call.state();
            state.set(state.get() + 1);
            if fail {
                return Err(HostFailure::new("inside callback").into());
            }
            Ok(call.return_value(1.into()))
        }
        fn invoke<'call>(
            mut call: HostCall<'call, Profile, Provider, BigInt>,
            constructions: HostConstructions<'call, HostTypeListEnd>,
            callback: HostCallable<'call, HostTypeList<BigInt, HostTypeListEnd>, BigInt>,
            fail: bool,
        ) -> Result<HostCallContinuation<'call, BigInt>, HostCallError> {
            let state = call.state();
            state.set(state.get() + 10);
            let callback = call.owned_callable(callback, &constructions);
            Ok(call.resume(constructions, move |context| {
                Box::pin(async move {
                    crate::runtime::execution::Yield::new().await;
                    let value = callback
                        .invoke(
                            &context,
                            |_, _| (39.into(), ()),
                            |mut call, _, value| {
                                let state = call.state();
                                state.set(state.get() + 100);
                                Ok(value)
                            },
                        )
                        .await?;
                    if fail {
                        return Err(HostFailure::new("after callback").into());
                    }
                    Ok(HostOwnedCompletion::new(move |call, _| {
                        Ok(call.return_value(value))
                    }))
                })
            }))
        }

        for (fail_inside, fail_after) in [(false, false), (true, false), (false, true)] {
            let provider = HostProviderModule::new("application", "library")
                .unwrap()
                .with_scoped_function::<Provider, (bool,), BigInt, _>("bump", bump)
                .unwrap()
                .with_resumable_function::<Provider, (
                    HostFunctionType<HostTypeList<BigInt, HostTypeListEnd>, BigInt>,
                    bool,
                ), BigInt, HostTypeListEnd, _>("invoke", invoke)
                .unwrap();
            let source = format!(
                r#"
@external(erlang, "native", "bump")
fn bump(fail: Bool) -> Int
@external(erlang, "native", "invoke")
fn invoke(callback: fn(Int) -> Int, fail: Bool) -> Int
pub fn main() {{
  echo "before"
  let captured = 2
  let answer = invoke(fn(value) {{
    echo value
    value + bump({}) + captured + 1 - 1
  }}, {})
  echo answer
  answer
}}
"#,
                if fail_inside { "True" } else { "False" },
                if fail_after { "True" } else { "False" },
            );
            let typed = crate::compile_typed_host_program(
                "application",
                "library",
                [PackageSource::new(
                    "application",
                    Vec::<String>::new(),
                    [ModuleSource::new("library", "src/library.gleam", source)],
                )],
                HostProviderSet::from_providers([provider]).unwrap(),
            )
            .unwrap();
            let library = crate::planner::plan_host_library_program(typed).unwrap();
            let function = library
                .functions()
                .iter()
                .find(|function| function.name() == "main")
                .unwrap();
            let entry = LibraryEntry::new(
                function.signature().id(),
                LibraryValueType::Int,
                Vec::new(),
                Vec::new(),
            );
            let (plan, entries, _) =
                HostedProgram::from_library_plan(library, entry, Vec::new()).unwrap();
            let plan = Arc::new(plan);
            for budget in (1..=16).chain([1024]) {
                let host = ManualHost::default();
                let mut state = Cell::new(0);
                let mut stores = Cell::new(());
                let mut echo = Vec::new();
                let domain = Domain::new(
                    Arc::clone(&plan),
                    &host,
                    &mut state,
                    &mut stores,
                    &mut echo,
                    Default::default(),
                    NonZeroUsize::new(budget).unwrap(),
                );
                let context = domain.context();
                let result = host
                    .finish(domain.drive(context.call(
                        *entries.ints[0].function(),
                        HostCallOrigin::Entry,
                        RetainedValues::empty(),
                    )))
                    .unwrap()
                    .try_into_value()
                    .unwrap()
                    .unwrap();
                let (expected, expected_echo) = if fail_inside {
                    (
                        Err("host function application::library.bump failed: inside callback"),
                        vec!["\"before\"", "39"],
                    )
                } else if fail_after {
                    (
                        Err("host function application::library.invoke failed: after callback"),
                        vec!["\"before\"", "39"],
                    )
                } else {
                    (Ok(BigInt::from(42)), vec!["\"before\"", "39", "42"])
                };
                assert_eq!(
                    result.map_err(|error| error.to_string()),
                    expected.map(Into::into).map_err(str::to_owned)
                );
                assert_eq!(
                    state.get(),
                    if fail_inside { 11 } else { 111 },
                    "budget {budget}, inside {fail_inside}, after {fail_after}",
                );
                assert_eq!(
                    echo.iter()
                        .map(|output| output.value().inspect().to_string())
                        .collect::<Vec<_>>(),
                    expected_echo,
                );
                assert_eq!(
                    host.started.load(Ordering::SeqCst),
                    host.released.load(Ordering::SeqCst)
                );
                assert!(host.workers.lock().is_empty());
            }
        }
    }

    #[test]
    fn bounded_cleanup_wakes_for_unpolled_tasks_even_after_completed_tasks_are_removed() {
        struct WakeCount(AtomicUsize);
        impl std::task::Wake for WakeCount {
            fn wake(self: Arc<Self>) {
                self.0.fetch_add(1, Ordering::SeqCst);
            }
        }
        let (plan, _) = program("pub fn main() { Nil }", LibraryValueType::Nil);
        let host = ManualHost::default();
        let mut state = Cell::new(0);
        let mut stores = Cell::new(());
        let mut echo = Vec::new();
        let mut domain = Domain::new(
            plan,
            &host,
            &mut state,
            &mut stores,
            &mut echo,
            Default::default(),
            NonZeroUsize::MIN,
        );
        // Both are actual host tasks; their completion is already available when
        // bounded cleanup first observes them.
        domain.tasks.push(host.spawn(Box::pin(async {})));
        domain.tasks.push(host.spawn(Box::pin(async {})));
        host.turn();
        let wakes = Arc::new(WakeCount(AtomicUsize::new(0)));
        let waker = Waker::from(Arc::clone(&wakes));
        let mut cx = Context::from_waker(&waker);
        assert!(domain.reap(&mut cx).is_none());
        assert_eq!(domain.tasks.len(), 1);
        assert_eq!(wakes.0.load(Ordering::SeqCst), 1);
        assert!(domain.reap(&mut cx).is_none());
        assert!(domain.tasks.is_empty());
        assert_eq!(wakes.0.load(Ordering::SeqCst), 1);
        assert_eq!(host.released.load(Ordering::SeqCst), 2);
    }

    #[test]
    fn waiting_tasks_above_the_completion_budget_do_not_self_wake() {
        struct WakeCount(AtomicUsize);
        impl std::task::Wake for WakeCount {
            fn wake(self: Arc<Self>) {
                self.0.fetch_add(1, Ordering::SeqCst);
            }
        }
        let (plan, _) = program("pub fn main() { Nil }", LibraryValueType::Nil);
        let host = ManualHost::default();
        let mut state = Cell::new(0);
        let mut stores = Cell::new(());
        let mut echo = Vec::new();
        let mut domain = Domain::new(
            plan,
            &host,
            &mut state,
            &mut stores,
            &mut echo,
            Default::default(),
            NonZeroUsize::MIN,
        );
        for _ in 0..3 {
            domain
                .tasks
                .push(host.spawn(Box::pin(std::future::pending())));
        }
        let wakes = Arc::new(WakeCount(AtomicUsize::new(0)));
        let waker = Waker::from(Arc::clone(&wakes));
        let mut cx = Context::from_waker(&waker);
        assert!(domain.reap(&mut cx).is_none());
        assert!(domain.reap(&mut cx).is_none());
        // Initial queue registration may request a turn; settled waits must not.
        let registered = wakes.0.load(Ordering::SeqCst);
        for _ in 0..4 {
            assert!(domain.reap(&mut cx).is_none());
        }
        assert_eq!(wakes.0.load(Ordering::SeqCst), registered);
        assert_eq!(domain.tasks.len(), 3);
        host.finish(domain.drive(std::future::ready(())))
            .unwrap()
            .try_into_value()
            .unwrap();
        assert_eq!(host.released.load(Ordering::SeqCst), 3);
    }

    #[test]
    fn body_completion_closes_queued_state_access_before_any_more_effects() {
        let (plan, _) = program("pub fn main() { 0 }", LibraryValueType::Int);
        for complete_before_service in [false, true] {
            let host = crate::execution_fixture::TestHost::default();
            let mut state = Cell::new(7);
            let mut stores = Cell::new(());
            let mut echo = Vec::new();
            let domain = Domain::new(
                Arc::clone(&plan),
                &host,
                &mut state,
                &mut stores,
                &mut echo,
                Default::default(),
                NonZeroUsize::MIN,
            );
            let context = domain.context().execution;
            let mut queued = std::pin::pin!(context.with_state(|state| state.set(99)));
            let mut cx = Context::from_waker(Waker::noop());
            assert!(queued.as_mut().poll(&mut cx).is_pending());
            if complete_before_service {
                host.block_on(domain.drive(std::future::ready(())))
                    .unwrap()
                    .try_into_value()
                    .unwrap();
                assert_eq!(queued.as_mut().poll(&mut cx), Poll::Ready(Err(Cancelled)));
                assert_eq!(state.get(), 7);
            } else {
                assert_eq!(
                    host.block_on(domain.drive(queued.as_mut()))
                        .unwrap()
                        .try_into_value()
                        .unwrap(),
                    Ok(())
                );
                assert_eq!(state.get(), 99);
            }
            assert!(echo.is_empty());
        }
    }

    #[test]
    fn a_requested_callback_runs_on_an_owned_worker_while_the_rust_body_waits() {
        use crate::plan::ValueType;
        use crate::runtime::CallbackInputs;
        use crate::runtime::evaluated::EvaluatedValue;
        let (plan, functions) = program(
            r#"
fn count(n: Int, total: Int) -> Int {
  case n { 0 -> total _ -> count(n - 1, total + 1) }
}
pub fn main() { #(fn() { echo 41 count(2_000, 0) - 1958 }) }
"#,
            LibraryValueType::Tuple(vec![ValueType::Function(Box::new(
                crate::plan::FunctionType::new(vec![], ValueType::Int),
            ))]),
        );
        let host = ManualHost::default();
        let mut state = Cell::new(0);
        let mut stores = Cell::new(());
        let mut echo = Vec::new();
        let domain = Domain::new(
            plan,
            &host,
            &mut state,
            &mut stores,
            &mut echo,
            Default::default(),
            NonZeroUsize::new(37).unwrap(),
        );
        let context = domain.context();
        let result = host
            .finish(domain.drive(async {
                let tuple = context
                    .call(
                        *functions.tuples[0].function(),
                        HostCallOrigin::Entry,
                        RetainedValues::empty(),
                    )
                    .await
                    .unwrap()
                    .unwrap();
                let callable = int_callback(&tuple.as_slice()[0]);
                let result = context
                    .execution
                    .invoke(callable, HostCallOrigin::Entry, CallbackInputs::new())
                    .await
                    .unwrap()
                    .unwrap();
                context
                    .execution
                    .with_state(|state| state.set(1))
                    .await
                    .unwrap();
                result
            }))
            .unwrap()
            .try_into_value()
            .unwrap();
        assert_eq!(result.value(), &EvaluatedValue::Int(42.into()));
        assert_eq!(state.get(), 1);
        assert_eq!(
            echo.iter()
                .map(|output| output.value().inspect().to_string())
                .collect::<Vec<_>>(),
            ["41"]
        );
        assert_eq!(host.started.load(Ordering::SeqCst), 2);
        assert_eq!(host.released.load(Ordering::SeqCst), 2);
    }

    #[test]
    fn cancellation_rejects_already_queued_state_access_and_callbacks() {
        use crate::execution::UnitOwner;
        use crate::plan::{FunctionType, ValueType};
        use crate::runtime::CallbackInputs;

        let (plan, functions) = program(
            "pub fn main() { #(fn() { echo 42 42 }) }",
            LibraryValueType::Tuple(vec![ValueType::Function(Box::new(FunctionType::new(
                Vec::new(),
                ValueType::Int,
            )))]),
        );
        let host = ManualHost::default();
        let mut state = Cell::new(7);
        let mut stores = Cell::new(());
        let mut echo = Vec::new();
        let mut domain = Domain::new(
            plan,
            &host,
            &mut state,
            &mut stores,
            &mut echo,
            Default::default(),
            NonZeroUsize::MIN,
        );
        let context = domain.context();
        let mut entry = std::pin::pin!(context.call(
            *functions.tuples[0].function(),
            HostCallOrigin::Entry,
            RetainedValues::empty(),
        ));
        let tuple = host
            .finish(poll_fn(|cx| domain.poll_body(entry.as_mut(), cx)))
            .unwrap()
            .try_into_value()
            .unwrap()
            .unwrap()
            .unwrap();
        let callable = int_callback(&tuple.as_slice()[0]);
        let mut successful = std::pin::pin!(context.execution.with_state(set_state));
        host.finish(poll_fn(|cx| domain.poll_body(successful.as_mut(), cx)))
            .unwrap()
            .try_into_value()
            .unwrap()
            .unwrap();
        assert_eq!(domain.state.get(), 99);
        domain.state.set(7);
        let (context, root) = domain.begin(UnitOwner::new(domain.units.completion()));
        let unit = context.unit().unwrap().clone();
        let mut request = std::pin::pin!(context.with_state(set_state));
        let mut callback =
            std::pin::pin!(context.invoke(callable, HostCallOrigin::Entry, CallbackInputs::new(),));
        let mut cx = Context::from_waker(Waker::noop());
        assert!(request.as_mut().poll(&mut cx).is_pending());
        assert!(callback.as_mut().poll(&mut cx).is_pending());
        assert!(unit.cancel());
        domain.service(&mut cx);
        domain.service(&mut cx);
        assert_eq!(request.as_mut().poll(&mut cx), Poll::Ready(Err(Cancelled)));
        assert_eq!(
            callback.as_mut().poll(&mut cx).map(|result| result.err()),
            Poll::Ready(Some(Cancelled))
        );
        assert_eq!(host.started.load(Ordering::SeqCst), 1);
        drop(root);
        host.finish(domain.drive(std::future::ready(())))
            .unwrap()
            .try_into_value()
            .unwrap();
        assert_eq!(state.get(), 7);
        assert!(echo.is_empty());
    }

    #[test]
    fn native_view_continuations_preserve_failures_and_cancellation_at_each_codec_boundary() {
        use crate::execution::UnitOwner;
        use crate::host::native::{NativeCall, NativeRules};
        use crate::host::{
            HostCallCompletion, HostCallError, HostProvider, HostProviderModule, HostTypeIndex0,
            HostTypeList, HostTypeListEnd, HostTypeParameter, HostValue,
        };
        type Input = HostTypeParameter<1>;
        type Output = HostTypeParameter<0>;
        type One<Type> = HostTypeList<Type, HostTypeListEnd>;
        struct Views;
        impl HostProvider<Profile> for Views {
            type State = Cell<usize>;
            fn project(state: &mut Cell<usize>) -> &mut Cell<usize> {
                state
            }
        }
        let provider = HostProviderModule::new("application", "library")
            .unwrap()
            .with_native_function::<Views, (Input,), Output, One<Output>, _>(
                "coerce",
                NativeRules::default().retained_views::<One<Input>>(),
                |mut call: NativeCall<'_, Profile, Views, Output, One<Output>>,
                 value: HostValue<'_, Input>| {
                    assert_eq!(call.call().state().get(), 0);
                    let source = call.source::<Input>(value);
                    let value = call.convert::<HostTypeIndex0>(&source).unwrap();
                    Ok::<HostCallCompletion<'_, Output>, HostCallError>(call.finish(value))
                },
            )
            .unwrap();
        let source = r#"
@external(erlang, "gleam@function", "identity") fn coerce(value: a) -> b
pub fn main() {
  let view: fn(String) -> BitArray = coerce(fn(input: BitArray) {
    let assert <<42>> = input
    echo 42
    "*"
  })
  view("*")
}
pub fn invalid_input() {
  let view: fn(String) -> BitArray = coerce(fn(_input: Int) {
    echo 42
    "*"
  })
  view("*")
}
pub fn invalid_result() {
  let view: fn(String) -> BitArray = coerce(fn(input: BitArray) {
    let assert <<42>> = input
    echo 42
    42
  })
  view("*")
}
pub fn source_failure() {
  let view: fn(String) -> BitArray = coerce(fn(input: BitArray) -> String {
    let assert <<42>> = input
    echo 42
    panic as "source rejected"
  })
  view("*")
}
"#;
        let typed = crate::compile_typed_host_program(
            "application",
            "library",
            [crate::PackageSource::new(
                "application",
                Vec::<&str>::new(),
                [crate::ModuleSource::new(
                    "library",
                    "src/library.gleam",
                    source,
                )],
            )],
            HostProviderSet::from_providers([provider]).unwrap(),
        )
        .unwrap();
        let library = crate::planner::plan_host_library_program(typed).unwrap();
        let entry = |name: &str| {
            let function = library
                .functions()
                .iter()
                .find(|function| function.name() == name)
                .unwrap();
            LibraryEntry::new(
                function.signature().id(),
                LibraryValueType::BitArray,
                Vec::new(),
                Vec::new(),
            )
        };
        let first = entry("main");
        let remaining = ["invalid_input", "invalid_result", "source_failure"]
            .map(entry)
            .to_vec();
        let (plan, functions, _) =
            HostedProgram::from_library_plan(library, first, remaining).unwrap();
        let plan = Arc::new(plan);
        // Run the ordinary source entry through its real execution unit. The
        // Coerce and the view each start through a host service before the input
        // codec, source Echo, result codec and final host-result restoration.
        for (entry, failure) in functions.bit_arrays.iter().zip([
            None,
            Some("input does not match its source signature"),
            Some("result does not match its target signature"),
            Some("source rejected"),
        ]) {
            let boundaries = if failure.is_some() {
                &[(None, None)][..]
            } else {
                &[
                    (Some(2usize), None),
                    (Some(3), None),
                    (Some(4), None),
                    (None, Some(2)),
                    (None, Some(3)),
                    (None, Some(4)),
                    (None, None),
                ][..]
            };
            for &(cancel_at, close_at) in boundaries {
                let host = ManualHost::default();
                let mut state = Cell::new(0);
                let mut stores = Cell::new(());
                let mut echo = Vec::new();
                let mut domain = Domain::new(
                    Arc::clone(&plan),
                    &host,
                    &mut state,
                    &mut stores,
                    &mut echo,
                    Default::default(),
                    Domain::<Profile>::DEFAULT_BUDGET,
                );
                let (execution, root) = domain.begin(UnitOwner::new(domain.units.completion()));
                let unit = execution.unit().unwrap().clone();
                let mut completion = Box::pin(
                    crate::runtime::function::Execution::new(
                        *entry.function(),
                        HostCallOrigin::Entry,
                        RetainedValues::empty(),
                    )
                    .drive(
                        plan.as_ref(),
                        execution.services(),
                        Domain::<Profile>::DEFAULT_BUDGET,
                    ),
                );
                let mut services = 0usize;
                let mut closed_request = false;
                let result = host.finish(poll_fn(|cx| {
                    if let Poll::Ready(result) = completion.as_mut().poll(cx) {
                        return Poll::Ready(result);
                    }
                    while let Some(request) = domain.work.next(cx) {
                        let parent_service = match &request {
                            Request::Service(service) => {
                                service.unit().is_some_and(|owner| owner.id() == unit.id())
                            }
                            Request::Callback(_) => {
                                // Close the source reply before its first effect,
                                // while the enclosing execution unit stays active.
                                if close_at == Some(3) && services == 3 {
                                    assert!(!closed_request);
                                    closed_request = true;
                                    drop(request);
                                    continue;
                                }
                                false
                            }
                        };
                        if parent_service {
                            if cancel_at == Some(services) {
                                assert!(unit.cancel());
                            }
                            let close = close_at == Some(services);
                            services += 1;
                            if close {
                                assert!(!closed_request);
                                closed_request = true;
                                drop(request);
                                continue;
                            }
                        }
                        domain.dispatch(request, domain.budget.get());
                    }
                    domain.finish_units(cx);
                    Poll::Pending
                }));
                assert_eq!(
                    result.as_ref().err(),
                    cancel_at.or(close_at).as_ref().map(|_| &Cancelled)
                );
                if let Some(message) = failure {
                    let error = result.unwrap().unwrap_err();
                    assert!(error.to_string().contains(message), "{error}");
                } else if cancel_at.or(close_at).is_none() {
                    assert_eq!(
                        result.unwrap().unwrap().value(),
                        crate::BitArrayValue::from_bytes(vec![42])
                    );
                }
                if failure.is_none() {
                    assert_eq!(
                        services,
                        cancel_at.map_or_else(
                            || close_at.map_or(6, |index| if index == 3 { 3 } else { index + 1 }),
                            |index| index + 1,
                        )
                    );
                }
                assert_eq!(closed_request, close_at.is_some());
                if closed_request {
                    assert!(unit.is_active());
                }
                drop(root);
                host.finish(domain.drive(std::future::ready(())))
                    .unwrap()
                    .try_into_value()
                    .unwrap();
                assert_eq!(
                    echo.iter()
                        .map(|output| output.value().inspect().to_string())
                        .collect::<Vec<_>>(),
                    if matches!(cancel_at.or(close_at), Some(2 | 3))
                        || failure == Some("input does not match its source signature")
                    {
                        Vec::<String>::new()
                    } else {
                        vec!["42".to_owned()]
                    }
                );
            }
        }
    }

    fn set_state(state: &mut Cell<usize>) {
        state.set(99);
    }

    #[test]
    fn spawned_and_nested_callbacks_share_service_turns_without_sharing_lifetimes() {
        use crate::execution::UnitOwner;
        use crate::plan::{FunctionType, ValueType};
        use crate::runtime::{CallbackInputs, EvaluatedValue};

        let (plan, functions) = program(
            "pub fn main() { #(fn() { echo 42 42 }) }",
            LibraryValueType::Tuple(vec![ValueType::Function(Box::new(FunctionType::new(
                Vec::new(),
                ValueType::Int,
            )))]),
        );
        let host = ManualHost::default();
        let mut state = Cell::new(7);
        let mut stores = Cell::new(());
        let mut echo = Vec::new();
        let mut domain = Domain::new(
            plan,
            &host,
            &mut state,
            &mut stores,
            &mut echo,
            Default::default(),
            NonZeroUsize::MIN,
        );
        let context = domain.context();
        let mut entry = std::pin::pin!(context.call(
            *functions.tuples[0].function(),
            HostCallOrigin::Entry,
            RetainedValues::empty(),
        ));
        let tuple = host
            .finish(poll_fn(|cx| domain.poll_body(entry.as_mut(), cx)))
            .unwrap()
            .try_into_value()
            .unwrap()
            .unwrap()
            .unwrap();
        let callable = int_callback(&tuple.as_slice()[0]);
        let spawned = domain.units.spawn(callable.clone(), HostCallOrigin::Entry);
        let (context, root) = domain.begin(UnitOwner::new(domain.units.completion()));
        let caller = context.unit().unwrap().clone();
        let mut callback =
            std::pin::pin!(context.invoke(callable, HostCallOrigin::Entry, CallbackInputs::new(),));
        let value = host
            .finish(poll_fn(|cx| domain.poll_body(callback.as_mut(), cx)))
            .unwrap()
            .try_into_value()
            .unwrap()
            .unwrap()
            .unwrap();
        assert_eq!(value.value(), &EvaluatedValue::Int(42.into()));
        assert!(caller.is_active());
        assert!(!spawned.is_active());
        assert_ne!(caller.id(), spawned.id());
        host.finish(root.run(std::future::ready(Ok(Ok(())))))
            .unwrap()
            .unwrap();
        assert!(!caller.is_active());
        host.finish(domain.drive(std::future::ready(())))
            .unwrap()
            .try_into_value()
            .unwrap();
        assert_eq!(host.started.load(Ordering::SeqCst), 3);
        assert_eq!(host.released.load(Ordering::SeqCst), 3);
        assert_eq!(
            echo.iter()
                .map(|output| output.value().inspect().to_string())
                .collect::<Vec<_>>(),
            ["42", "42"]
        );
    }

    fn int_callback(value: &crate::runtime::EvaluatedValue) -> crate::runtime::RetainedCallable {
        use crate::runtime::evaluated::{EvaluatedFunctionValueKind, EvaluatedValue};
        use crate::runtime::function::InvocableFunctionValue;
        let EvaluatedValue::Function(function) = value else {
            panic!("fixture returns a function");
        };
        let EvaluatedFunctionValueKind::Int(function) = function.kind() else {
            panic!("fixture function returns Int");
        };
        crate::runtime::RetainedCallable::new(InvocableFunctionValue::Int(function.clone()))
    }

    #[test]
    fn int_callback_rejects_source_values_outside_its_fixture_shape() {
        use crate::plan::{FunctionType, ValueType};
        for (source, type_) in [
            ("pub fn main() { #(42) }", ValueType::Int),
            (
                "pub fn main() { #(fn() { 1.0 }) }",
                ValueType::Function(Box::new(FunctionType::new(Vec::new(), ValueType::Float))),
            ),
        ] {
            let (plan, functions) = program(source, LibraryValueType::Tuple(vec![type_]));
            let host = ManualHost::default();
            let mut state = Cell::new(0);
            let mut stores = Cell::new(());
            let mut echo = Vec::new();
            let domain = Domain::new(
                plan,
                &host,
                &mut state,
                &mut stores,
                &mut echo,
                Default::default(),
                NonZeroUsize::MIN,
            );
            let context = domain.context();
            let tuple = host
                .finish(domain.drive(context.call(
                    *functions.tuples[0].function(),
                    HostCallOrigin::Entry,
                    RetainedValues::empty(),
                )))
                .unwrap()
                .try_into_value()
                .unwrap()
                .unwrap()
                .unwrap();
            assert!(
                std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    int_callback(&tuple.as_slice()[0])
                }))
                .is_err()
            );
        }
    }
}

#[cfg(test)]
fn poll_domain<Profile: HostProfile, Output>(
    domain: &mut Domain<'_, Profile>,
    host: &crate::execution_fixture::TestHost,
    mut future: Pin<&mut impl Future<Output = Output>>,
) -> Poll<Output> {
    let mut driving = pin!(poll_fn(|cx| domain.poll_body(future.as_mut(), cx)));
    host.poll(driving.as_mut()).map(|output| {
        output
            .expect("controlled host turn")
            .try_into_value()
            .unwrap()
    })
}

#[cfg(test)]
fn complete_domain<Profile: HostProfile, Output>(
    domain: &mut Domain<'_, Profile>,
    host: &crate::execution_fixture::TestHost,
    future: impl Future<Output = Output>,
) -> Output {
    let mut future = pin!(future);
    host.block_on(poll_fn(|cx| domain.poll_body(future.as_mut(), cx)))
        .expect("controlled host turn")
        .try_into_value()
        .unwrap()
}

#[cfg(test)]
mod source_work {
    use super::{Domain, complete_domain, poll_domain};
    use crate::execution_fixture::TestHost;
    use crate::frontend::compile_typed_host_program;
    use crate::host::{HostProfile, HostProviderSet};
    use crate::plan::execution::HostedProgram;
    use crate::plan::{LibraryEntry, LibraryValueType, ValueType};
    use crate::runtime::evaluated::{EvaluatedFunctionValueKind, EvaluatedValue};
    use crate::runtime::function::InvocableFunctionValue;
    use crate::runtime::shared::Shared;
    use crate::runtime::work::Cancelled;
    use crate::runtime::work::execution::Completion;
    use crate::runtime::{CallbackInputs, HostCallOrigin, RetainedCallable, RetainedInputs};
    use crate::{ModuleSource, PackageSource};
    use std::cell::Cell;
    use std::future::Future;
    use std::num::NonZeroUsize;
    use std::sync::Arc;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::task::{Context, Poll, Wake, Waker};

    struct Profile;

    #[derive(Default)]
    struct Echo {
        output: Vec<String>,
        exclusive: Cell<usize>,
    }

    impl crate::EchoSink for Echo {
        fn emit(&mut self, value: crate::EchoOutput) {
            self.exclusive.set(self.exclusive.get() + 1);
            self.output.push(value.to_string());
        }
    }

    impl HostProfile for Profile {
        type RunState = Cell<usize>;
        type ExternalStores = Cell<()>;
        type ExecutionState = ();
    }

    #[derive(Default)]
    struct WakeCount(AtomicUsize);

    impl Wake for WakeCount {
        fn wake(self: Arc<Self>) {
            self.0.fetch_add(1, Ordering::SeqCst);
        }
    }

    #[test]
    fn claimed_requests_skip_abandoned_receivers_and_allow_reentrant_wakes() {
        type Request = futures_util::future::BoxFuture<'static, Result<usize, Cancelled>>;
        struct CheckWake {
            context: crate::runtime::execution::ExecutionContext<Profile>,
            requests: std::sync::Mutex<Vec<Request>>,
        }
        impl Wake for CheckWake {
            fn wake(self: Arc<Self>) {
                let mut request = Box::pin(self.context.with_state(|state| {
                    state.set(state.get() + 1);
                    state.get()
                }));
                assert!(
                    request
                        .as_mut()
                        .poll(&mut Context::from_waker(Waker::noop()))
                        .is_pending()
                );
                self.requests.lock().unwrap().push(request);
            }
        }
        let source = "pub fn idle() { Nil }";
        let program = compile_typed_host_program(
            "application",
            "library",
            [PackageSource::new(
                "application",
                Vec::<String>::new(),
                [ModuleSource::new("library", "src/library.gleam", source)],
            )],
            HostProviderSet::<Profile>::from_providers(Vec::new()).expect("no providers"),
        )
        .expect("source");
        let library = crate::planner::plan_host_library_program(program).expect("plan");
        let id = library
            .functions()
            .iter()
            .find(|function| function.name() == "idle")
            .expect("idle")
            .signature()
            .id();
        let (plan, _, _) = HostedProgram::from_library_plan(
            library,
            LibraryEntry::new(id, LibraryValueType::Nil, Vec::new(), Vec::new()),
            Vec::new(),
        )
        .expect("sealed library");
        let mut state = Cell::new(0);
        let mut stores = Cell::new(());
        let mut echo = Echo::default();
        let host = TestHost::default();
        let mut driver = Domain::new(
            Arc::new(plan),
            &host,
            &mut state,
            &mut stores,
            &mut echo,
            Default::default(),
            NonZeroUsize::MIN,
        );
        assert!(
            driver
                .work
                .next(&mut Context::from_waker(Waker::noop()))
                .is_none()
        );
        for cancelled in [false, true] {
            let wake = Arc::new(CheckWake {
                context: driver.work.execution(),
                requests: Default::default(),
            });
            let waker = Waker::from(Arc::clone(&wake));
            let context = driver.work.execution();
            let mut receiver = Box::pin(context.with_state(|state| {
                state.set(state.get() + 1);
                state.get()
            }));
            let mut cx = Context::from_waker(if cancelled { Waker::noop() } else { &waker });
            assert!(receiver.as_mut().poll(&mut cx).is_pending());
            let request = driver
                .work
                .next(&mut Context::from_waker(Waker::noop()))
                .expect("claimed state request");
            let receiver = if cancelled {
                drop(receiver);
                None
            } else {
                Some(receiver)
            };
            if let Some(mut receiver) = receiver {
                driver.dispatch(request, driver.budget.get());
                assert_eq!(receiver.as_mut().poll(&mut cx), Poll::Ready(Ok(1)));
                let mut reentrant = wake
                    .requests
                    .lock()
                    .unwrap()
                    .pop()
                    .expect("completion wake submitted another request");
                driver.service(&mut Context::from_waker(Waker::noop()));
                assert_eq!(reentrant.as_mut().poll(&mut cx), Poll::Ready(Ok(2)));
            } else {
                driver.dispatch(request, driver.budget.get());
                assert!(wake.requests.lock().unwrap().is_empty());
            }
        }
        drop(driver);
        assert_eq!(state.get(), 2);
        assert!(echo.output.is_empty());
    }

    #[test]
    fn competing_observers_queue_state_access_and_receive_a_completion_wake() {
        use std::sync::Barrier;

        let typed = compile_typed_host_program(
            "application",
            "library",
            [PackageSource::new(
                "application",
                Vec::<String>::new(),
                [ModuleSource::new(
                    "library",
                    "src/library.gleam",
                    "pub fn run() { Nil }",
                )],
            )],
            HostProviderSet::<Profile>::from_providers(Vec::new()).expect("provider set"),
        )
        .expect("typed source");
        let plan = crate::planner::plan_host_library_program(typed).expect("library plan");
        let function = plan
            .functions()
            .iter()
            .find(|function| function.name() == "run")
            .expect("run function")
            .gleam_body()
            .expect("source body");
        let entry = LibraryEntry::new(function.id(), LibraryValueType::Nil, Vec::new(), Vec::new());
        let (plan, _, _) =
            HostedProgram::from_library_plan(plan, entry, Vec::new()).expect("sealed execution");
        let mut host = Cell::new(0);
        let mut stores = Cell::new(());
        let mut echo = Echo::default();
        let executor = TestHost::default();
        let mut driver = Domain::new(
            Arc::new(plan),
            &executor,
            &mut host,
            &mut stores,
            &mut echo,
            Default::default(),
            NonZeroUsize::MIN,
        );
        let entered = Arc::new(Barrier::new(2));
        let release = Arc::new(Barrier::new(2));
        let [mut first, mut second, mut abandoned] = [true, false, false].map(|pause| {
            let context = driver.work.execution();
            let entered = Arc::clone(&entered);
            let release = Arc::clone(&release);
            Box::pin(context.with_state(move |state| {
                if pause {
                    entered.wait();
                    release.wait();
                }
                state.set(state.get() + 1);
                state.get()
            }))
        });
        let wake = Arc::new(WakeCount::default());
        let waker = Waker::from(Arc::clone(&wake));
        assert!(
            first
                .as_mut()
                .poll(&mut Context::from_waker(&waker))
                .is_pending()
        );
        std::thread::scope(|threads| {
            let worker = threads.spawn(|| driver.service(&mut Context::from_waker(Waker::noop())));
            entered.wait();
            let deferred = second.as_mut().poll(&mut Context::from_waker(&waker));
            release.wait();
            worker.join().expect("state worker");
            assert!(deferred.is_pending());
        });
        assert!(
            first
                .as_mut()
                .poll(&mut Context::from_waker(&waker))
                .is_ready()
        );
        assert!(wake.0.load(Ordering::SeqCst) > 0);
        let before_second = wake.0.load(Ordering::SeqCst);
        driver.service(&mut Context::from_waker(Waker::noop()));
        assert!(wake.0.load(Ordering::SeqCst) > before_second);
        let second_completion = second.as_mut().poll(&mut Context::from_waker(&waker));
        assert!(second_completion.is_ready());
        drop(first);
        drop(second);
        let mut cx = Context::from_waker(Waker::noop());
        assert!(abandoned.as_mut().poll(&mut cx).is_pending());
        drop(driver.work.next(&mut cx).expect("unserviced state request"));
        assert_eq!(
            abandoned.as_mut().poll(&mut cx).map(Result::err),
            Poll::Ready(Some(Cancelled))
        );
        drop(driver);
        assert_eq!(host.get(), 2);
        assert!(echo.output.is_empty());
    }

    #[test]
    fn a_pending_driver_moves_between_workers_and_reenters_the_original_source_and_state() {
        let source = "pub fn make() {\n  let captured = 40\n  #(fn(value: Int) {\n    echo value\n    captured + value\n  })\n}\n";
        let typed = compile_typed_host_program(
            "application",
            "library",
            [PackageSource::new(
                "application",
                Vec::<String>::new(),
                [ModuleSource::new("library", "src/library.gleam", source)],
            )],
            HostProviderSet::<Profile>::from_providers(Vec::new()).expect("provider set"),
        )
        .expect("typed source");
        let plan = crate::planner::plan_host_library_program(typed).expect("library plan");
        let function = plan
            .functions()
            .iter()
            .find(|function| function.name() == "make")
            .expect("make function")
            .gleam_body()
            .expect("source body");
        let types = tuple_return(function.signature().shape().type_().return_());
        let entry = LibraryEntry::new(
            function.id(),
            LibraryValueType::Tuple(types),
            Vec::new(),
            Vec::new(),
        );
        let (plan, entries, _) =
            HostedProgram::from_library_plan(plan, entry, Vec::new()).expect("sealed execution");
        let entry = *entries.tuples[0].function();
        let mut host = Cell::new(2);
        let mut stores = Cell::new(());
        let mut echo = Echo::default();
        let (ready, wait) = futures_channel::oneshot::channel::<()>();
        let plan = Arc::new(plan);
        let executor = TestHost::default();
        let driver = Domain::<Profile>::new(
            Arc::clone(&plan),
            &executor,
            &mut host,
            &mut stores,
            &mut echo,
            Default::default(),
            NonZeroUsize::MIN,
        );
        let entries = driver.context();
        let context = driver.work.execution();
        let mut observation = Box::pin(driver.drive(async move {
            let values = entries
                .call(
                    entry,
                    HostCallOrigin::Entry,
                    RetainedInputs::empty().into_retained(),
                )
                .await
                .expect("active entry")
                .expect("source closure");
            let callback = int_callback(&values);
            resume_callback_after_gate(context, wait, callback)
                .await
                .expect("completed callback")
        }));
        std::thread::scope(|threads| {
            let executor = &executor;
            observation = threads
                .spawn(move || {
                    assert!(executor.poll(observation.as_mut()).is_pending());
                    observation
                })
                .join()
                .expect("first worker");
            ready.send(()).expect("active waiter");
            let completion = threads
                .spawn(move || {
                    executor
                        .block_on(observation)
                        .expect("domain cleanup")
                        .try_into_value()
                        .unwrap()
                })
                .join()
                .expect("second worker");
            threads
                .spawn(move || {
                    completion.read(|result| {
                        let value = result.as_ref().ok().expect("completed callback");
                        value.read(|value| {
                            assert_eq!(value.value(), &EvaluatedValue::Int(42.into()))
                        });
                    });
                })
                .join()
                .expect("completion worker");
        });
        assert_eq!(host.get(), 3);
        assert_eq!(echo.output, ["src/library.gleam:4\n2"]);
        assert_eq!(echo.exclusive.get(), 1);

        for serviced in 0..3 {
            let mut state = Cell::new(2);
            let mut stores = Cell::new(());
            let mut echo = Echo::default();
            let driver = Domain::new(
                Arc::clone(&plan),
                &executor,
                &mut state,
                &mut stores,
                &mut echo,
                Default::default(),
                NonZeroUsize::MIN,
            );
            let entries = driver.context();
            let context = driver.work.execution();
            let (created, callback) = futures_channel::oneshot::channel();
            let mut running = Box::pin(driver.drive(async move {
                let values = entries
                    .call(
                        entry,
                        HostCallOrigin::Entry,
                        RetainedInputs::empty().into_retained(),
                    )
                    .await
                    .expect("active entry")
                    .expect("source callback");
                assert!(created.send(int_callback(&values)).is_ok());
                match std::future::pending::<std::convert::Infallible>().await {}
            }));
            assert!(executor.poll(running.as_mut()).is_pending());
            let callback = executor.block_on(callback).expect("constructed callback");
            let (ready, wait) = futures_channel::oneshot::channel();
            let mut resumed = Box::pin(resume_callback_after_gate(context, wait, callback));
            let mut cx = Context::from_waker(Waker::noop());
            if serviced == 0 {
                drop(ready);
            } else {
                ready.send(()).expect("live native gate");
                assert!(resumed.as_mut().poll(&mut cx).is_pending());
                if serviced == 2 {
                    assert!(executor.poll(running.as_mut()).is_pending());
                    assert!(resumed.as_mut().poll(&mut cx).is_pending());
                }
            }
            drop(running);
            executor.step();
            assert_eq!(
                resumed.as_mut().poll(&mut cx).map(Result::err),
                Poll::Ready(Some(Cancelled))
            );
            assert_eq!(state.get(), if serviced == 2 { 3 } else { 2 });
            assert!(echo.output.is_empty());
        }
    }

    async fn resume_callback_after_gate(
        context: crate::runtime::execution::ExecutionContext<Profile>,
        wait: futures_channel::oneshot::Receiver<()>,
        callback: crate::runtime::RetainedCallable,
    ) -> Result<Shared<Completion>, Cancelled> {
        wait.await.map_err(|_| Cancelled)?;
        let previous = context
            .with_state(|state| {
                let previous = state.get();
                state.set(previous + 1);
                previous
            })
            .await?;
        let mut inputs = CallbackInputs::new();
        inputs.push_value(EvaluatedValue::Int(previous.into()));
        let result = context
            .invoke(callback, HostCallOrigin::Entry, inputs)
            .await?;
        Ok(Shared::new(result.map(Shared::new).map_err(Shared::new)))
    }

    struct FutureProfile;

    const FUTURE_SOURCE: &str = crate::work_fixture::WorkComponent::SOURCE;

    struct FutureState {
        unit: (),
    }

    struct NativeProfile;
    struct NativeProvider;

    #[derive(Default)]
    struct NativeState {
        unit: (),
        gates: std::collections::VecDeque<
            futures_channel::oneshot::Receiver<Result<i32, crate::HostFailure>>,
        >,
        polls: Arc<AtomicUsize>,
        starts: Arc<AtomicUsize>,
        generation: usize,
        fail_touch: bool,
    }

    impl HostProfile for NativeProfile {
        type RunState = NativeState;
        type ExternalStores = crate::host::HostFutureStore;
        type ExecutionState = ();
    }

    impl crate::HostProvider<NativeProfile> for NativeProvider {
        type State = NativeState;

        fn project(state: &mut NativeState) -> &mut NativeState {
            state
        }
    }

    impl crate::host::HostWorkProfile for NativeProfile {
        type Work = crate::work_fixture::WorkComponent;
    }
    impl crate::host::HostComponentProfile<crate::work_fixture::WorkComponent> for NativeProfile {
        fn component_stores(stores: &Self::ExternalStores) -> &crate::host::HostFutureStore {
            stores
        }

        fn component_state(state: &mut Self::RunState) -> &mut () {
            &mut state.unit
        }
    }

    fn fetch<'call>(
        mut call: crate::host::HostCall<
            'call,
            NativeProfile,
            NativeProvider,
            crate::work_fixture::WorkHostType<num_bigint::BigInt>,
        >,
        constructions: crate::HostConstructions<'call, crate::HostTypeListEnd>,
        value: num_bigint::BigInt,
    ) -> Result<
        crate::HostCallCompletion<'call, crate::work_fixture::WorkHostType<num_bigint::BigInt>>,
        crate::HostCallError,
    > {
        let mut gate = call
            .state()
            .gates
            .pop_front()
            .expect("native construction gate");
        let polls = Arc::clone(&call.state().polls);
        let starts = Arc::clone(&call.state().starts);
        Ok(call.return_future(constructions, move |_| {
            Box::pin(async move {
                starts.fetch_add(1, Ordering::SeqCst);
                let increment = std::future::poll_fn(move |cx| {
                    polls.fetch_add(1, Ordering::SeqCst);
                    std::pin::Pin::new(&mut gate).poll(cx)
                })
                .await
                .map_err(|_| crate::host::HostExecutionError::Cancelled)??;
                Ok(crate::host::HostOwnedCompletion::new(move |call, _| {
                    Ok(call.return_value(value + increment))
                }))
            })
        }))
    }

    fn delayed<'call>(
        mut call: crate::host::HostCall<
            'call,
            NativeProfile,
            NativeProvider,
            crate::work_fixture::WorkHostType<num_bigint::BigInt>,
        >,
        constructions: crate::HostConstructions<'call, crate::HostTypeListEnd>,
        callback: crate::HostCallable<
            'call,
            crate::HostTypeList<num_bigint::BigInt, crate::HostTypeListEnd>,
            num_bigint::BigInt,
        >,
    ) -> Result<
        crate::HostCallCompletion<'call, crate::work_fixture::WorkHostType<num_bigint::BigInt>>,
        crate::HostCallError,
    > {
        let callback = call.owned_callable(callback, &constructions);
        let gate = call
            .state()
            .gates
            .pop_front()
            .expect("delayed operation gate");
        Ok(call.return_future(constructions, move |context| {
            Box::pin(async move {
                let before = context.with_state(|state| state.generation).await?;
                gate.await.expect("delayed native gate")?;
                let after = context
                    .with_state(|state| {
                        let before = state.generation;
                        state.generation += 1;
                        before
                    })
                    .await?;
                let first = callback
                    .invoke(
                        context.execution(),
                        move |_, _| (before.into(), ()),
                        |_, _, value| Ok(value),
                    )
                    .await?;
                let second = callback
                    .invoke(
                        context.execution(),
                        move |_, _| (after.into(), ()),
                        |_, _, value| Ok(value),
                    )
                    .await?;
                Ok(crate::host::HostOwnedCompletion::new(move |call, _| {
                    Ok(call.return_value(first + second))
                }))
            })
        }))
    }

    fn touch<'call>(
        mut call: crate::host::HostCall<'call, NativeProfile, NativeProvider, num_bigint::BigInt>,
    ) -> Result<crate::HostCallCompletion<'call, num_bigint::BigInt>, crate::HostCallError> {
        let state = call.state();
        let value = state.generation;
        if state.fail_touch && value == 4 {
            return Err(crate::HostFailure::new("touch failed").into());
        }
        state.generation += 1;
        Ok(call.return_value(value.into()))
    }

    #[test]
    fn native_context_reenters_the_original_state_and_repeats_a_captured_callback_after_pending() {
        for (source_failure, native_failure, first_failure) in [
            (false, false, false),
            (true, false, false),
            (false, true, false),
            (true, false, true),
        ] {
            let assertion = if first_failure {
                "let assert True = value < 1"
            } else if source_failure {
                "let assert True = value < 2"
            } else {
                ""
            };
            let source = format!(
                r#"import fixture/work as future
@external(erlang, "native", "delayed")
fn delayed(callback: fn(Int) -> Int) -> future.Work(Int)
@external(erlang, "native", "touch")
fn touch() -> Int
pub fn make() {{
  let captured = #(40, [1, 2])
  #(delayed(fn(value) {{
    echo value
    {assertion}
    let assert #(saved, [_, _]) = captured
    saved + value + touch()
  }}))
}}
"#
            );
            let mut providers = crate::work_fixture::WorkComponent::providers::<NativeProfile>()
                .expect("Future provider");
            providers.push(
                crate::host::HostProviderModule::new("application", "library")
                    .expect("native callback module")
                    .with_scoped_function_and_constructions::<NativeProvider, (
                        crate::HostFunctionType<
                            crate::HostTypeList<num_bigint::BigInt, crate::HostTypeListEnd>,
                            num_bigint::BigInt,
                        >,
                    ), crate::work_fixture::WorkHostType<num_bigint::BigInt>, crate::HostTypeListEnd, _>(
                        "delayed", delayed,
                    )
                    .expect("owned callback registration")
                    .with_scoped_function::<NativeProvider, (), num_bigint::BigInt, _>(
                        "touch", touch,
                    )
                    .expect("same-component state registration"),
            );
            let typed = compile_typed_host_program(
                "application",
                "library",
                [
                    PackageSource::new(
                        "work_fixture",
                        Vec::<String>::new(),
                        [ModuleSource::new(
                            "fixture/work",
                            "src/fixture/work.gleam",
                            FUTURE_SOURCE,
                        )],
                    ),
                    PackageSource::new(
                        "application",
                        ["work_fixture"],
                        [ModuleSource::new("library", "src/library.gleam", source)],
                    ),
                ],
                HostProviderSet::<NativeProfile>::from_providers(providers)
                    .expect("callback provider set"),
            )
            .expect("Future callback source");
            let plan = crate::planner::plan_host_library_program(typed).expect("callback library");
            let function = plan
                .functions()
                .iter()
                .find(|function| function.name() == "make")
                .expect("make callback entry")
                .gleam_body()
                .expect("source callback entry");
            let types = tuple_return(function.signature().shape().type_().return_());
            let entry = LibraryEntry::new(
                function.id(),
                LibraryValueType::Tuple(types),
                Vec::new(),
                Vec::new(),
            );
            let (plan, entries, _) = HostedProgram::from_library_plan(plan, entry, Vec::new())
                .expect("retained callback specialization");
            let plan = Arc::new(plan);
            let executor = TestHost::default();
            let entry = *entries.tuples[0].function();
            let (send, gate) = futures_channel::oneshot::channel();
            let mut state = NativeState {
                gates: [gate].into(),
                generation: 1,
                fail_touch: native_failure,
                ..NativeState::default()
            };
            let mut stores = crate::host::HostFutureStore::default();
            let mut echo = Echo::default();
            let mut driver = Domain::new(
                Arc::clone(&plan),
                &executor,
                &mut state,
                &mut stores,
                &mut echo,
                Default::default(),
                NonZeroUsize::MIN,
            );
            let work = {
                let source_entries = driver.context();
                let services = driver.work.execution();
                complete_domain(&mut driver, &executor, async move {
                    let value = source_entries
                        .call(
                            entry,
                            HostCallOrigin::Entry,
                            RetainedInputs::empty().into_retained(),
                        )
                        .await
                        .expect("active source entry")
                        .expect("construct owned callback work");
                    services
                        .with_runtime(move |_plan, runtime| {
                            let [value] = external_values(&value);
                            runtime.host().stores().work(value.lease())
                        })
                        .await
                        .expect("active runtime service")
                })
            };
            let _cx = Context::from_waker(Waker::noop());
            assert!(
                poll_domain(&mut driver, &executor, Box::pin(work.observe()).as_mut()).is_pending()
            );
            {
                let _source_entries = driver.context();
                let services = driver.work.execution();
                complete_domain(&mut driver, &executor, async move {
                    services
                        .with_runtime(move |_, runtime| {
                            assert_eq!(runtime.host_state().generation, 1);
                            runtime.host_state().generation = 2;
                        })
                        .await
                        .expect("active runtime service")
                })
            };
            send.send(Ok(0)).expect("native gate survives waiter drop");
            let completion = completed_observation(poll_domain(
                &mut driver,
                &executor,
                Box::pin(work.observe()).as_mut(),
            ));
            completion.read(|result| match result {
                Ok(value) => {
                    assert!(!source_failure && !native_failure);
                    value.read(|value| assert_eq!(value.value(), &EvaluatedValue::Int(90.into())));
                }
                Err(error) => error.read(|error| {
                    if source_failure {
                        let error = source_panic(error);
                        assert_eq!(error.kind(), crate::PanicKind::LetAssert);
                        assert_eq!(error.site().module(), "library");
                    } else {
                        assert!(native_failure);
                        let error = host_error(error);
                        assert_eq!(error.function(), "touch");
                        assert_eq!(error.location().line(), Some(12));
                    }
                }),
            });
            drop(driver);
            assert_eq!(
                state.generation,
                if first_failure {
                    3
                } else if source_failure || native_failure {
                    4
                } else {
                    5
                }
            );
            assert_eq!(echo.output.len(), if first_failure { 1 } else { 2 });
            for (index, output) in echo.output.iter().enumerate() {
                assert!(output.ends_with(&format!("\n{}", index + 1)));
            }
            if !source_failure && !native_failure {
                for discarded in 0..14 {
                    let (send, gate) = futures_channel::oneshot::channel();
                    let mut state = NativeState {
                        gates: [gate].into(),
                        generation: 1,
                        ..NativeState::default()
                    };
                    let mut stores = crate::host::HostFutureStore::default();
                    let mut echo = Echo::default();
                    let mut driver = Domain::new(
                        Arc::clone(&plan),
                        &executor,
                        &mut state,
                        &mut stores,
                        &mut echo,
                        Default::default(),
                        NonZeroUsize::MIN,
                    );
                    let work = {
                        let source_entries = driver.context();
                        let services = driver.work.execution();
                        complete_domain(&mut driver, &executor, async move {
                            let values = source_entries
                                .call(
                                    entry,
                                    HostCallOrigin::Entry,
                                    RetainedInputs::empty().into_retained(),
                                )
                                .await
                                .expect("active source entry")
                                .expect("owned callback work");
                            services
                                .with_runtime(move |_plan, runtime| {
                                    let [value] = external_values(&values);
                                    runtime.host().stores().work(value.lease())
                                })
                                .await
                                .expect("active runtime service")
                        })
                    };
                    send.send(if discarded == 13 {
                        Err(crate::HostFailure::new("native gate rejected"))
                    } else {
                        Ok(0)
                    })
                    .expect("unpolled native gate");
                    let mut cx = Context::from_waker(Waker::noop());
                    if discarded == 13 {
                        let result = completed_observation(poll_domain(
                            &mut driver,
                            &executor,
                            Box::pin(work.observe()).as_mut(),
                        ));
                        result.read(|result| {
                            result
                                .as_ref()
                                .err()
                                .expect("native gate failure")
                                .read(|error| {
                                    assert_eq!(
                                        host_error(error).failure().message(),
                                        "native gate rejected"
                                    );
                                })
                        });
                    } else {
                        let mut observer = std::pin::pin!(work.observe());
                        for index in 0..=discarded {
                            assert!(observer.as_mut().poll(&mut cx).is_pending());
                            let request = driver.work.next(&mut cx).expect("next native boundary");
                            if index == discarded {
                                drop(request);
                            } else {
                                driver.dispatch(request, driver.budget.get());
                            }
                            assert!(
                                executor
                                    .poll(std::pin::pin!(std::future::pending::<()>()).as_mut())
                                    .is_pending()
                            );
                        }
                        assert_eq!(
                            observer.as_mut().poll(&mut cx).map(Result::err),
                            Poll::Ready(Some(Cancelled))
                        );
                    }
                    drop(driver);
                    assert_eq!(
                        state.generation,
                        if discarded == 13 || discarded < 2 {
                            1
                        } else if discarded < 6 {
                            2
                        } else if discarded < 11 {
                            3
                        } else {
                            4
                        }
                    );
                    assert_eq!(
                        echo.output.len(),
                        if discarded == 13 {
                            0
                        } else {
                            usize::from(discarded >= 5) + usize::from(discarded >= 10)
                        }
                    );
                }
            }
        }
    }

    #[test]
    fn discarded_driver_requests_cancel_only_the_dependent_composition() {
        use crate::host::HostProviderModule;
        use crate::work_fixture::{WorkComponent, WorkHostType};
        let cases = [
            ("native", "fetch(40)", 1),
            (
                "map",
                "future.map(future.ready(41), fn(value) { value + 1 })",
                1,
            ),
            (
                "then",
                "future.then(future.ready(41), fn(value) { future.ready(value + 1) })",
                3,
            ),
            ("all", "future.all([future.ready(40), future.ready(2)])", 1),
        ];
        for (name, expression, request_count) in cases {
            let source = format!(
                r#"import fixture/work as future
@external(erlang, "native", "fetch")
fn fetch(value: Int) -> future.Work(Int)
pub fn make() {{ echo 7 #({expression}, future.ready(7)) }}
"#
            );
            let mut providers = WorkComponent::providers::<NativeProfile>().expect("Future module");
            providers.push(HostProviderModule::new("application", "library")
                .expect("native module")
                .with_scoped_function_and_constructions::<NativeProvider, (num_bigint::BigInt,), WorkHostType<num_bigint::BigInt>, crate::HostTypeListEnd, _>("fetch", fetch).expect("native constructor"));
            let typed = compile_typed_host_program(
                "application",
                "library",
                [
                    crate::PackageSource::new(
                        "work_fixture",
                        Vec::<String>::new(),
                        [crate::ModuleSource::new(
                            "fixture/work",
                            "src/fixture/work.gleam",
                            FUTURE_SOURCE,
                        )],
                    ),
                    crate::PackageSource::new(
                        "application",
                        ["work_fixture"],
                        [crate::ModuleSource::new(
                            "library",
                            "src/library.gleam",
                            source,
                        )],
                    ),
                ],
                HostProviderSet::from_providers(providers).expect("providers"),
            )
            .expect("source");
            let library = crate::planner::plan_host_library_program(typed).expect("plan");
            let make = library
                .functions()
                .iter()
                .find(|function| function.name() == "make")
                .expect("make")
                .gleam_body()
                .expect("source entry");
            let entry = LibraryEntry::new(
                make.id(),
                LibraryValueType::Tuple(tuple_return(make.signature().shape().type_().return_())),
                Vec::new(),
                Vec::new(),
            );
            let (plan, entries, _) = HostedProgram::from_library_plan(library, entry, Vec::new())
                .expect("sealed source");
            let plan = Arc::new(plan);
            let executor = TestHost::default();
            let entry = *entries.tuples[0].function();
            for discarded in 0..=request_count {
                let (sender, gate) = futures_channel::oneshot::channel();
                sender.send(Ok(2)).expect("ready native result");
                let mut state = NativeState::default();
                state.gates.push_back(gate);
                let mut stores = crate::host::HostFutureStore::default();
                let mut echo = Vec::new();
                let mut output = |value: crate::EchoOutput| echo.push(value.to_string());
                let mut driver = Domain::new(
                    Arc::clone(&plan),
                    &executor,
                    &mut state,
                    &mut stores,
                    &mut output,
                    Default::default(),
                    NonZeroUsize::MIN,
                );
                let (work, independent) = {
                    let source_entries = driver.context();
                    let services = driver.work.execution();
                    complete_domain(&mut driver, &executor, async move {
                        let values = source_entries
                            .call(
                                entry,
                                HostCallOrigin::Entry,
                                RetainedInputs::empty().into_retained(),
                            )
                            .await
                            .expect("active source entry")
                            .expect("source work construction");
                        services
                            .with_runtime(move |_plan, runtime| {
                                let [work, independent] = external_values::<2>(&values);
                                (
                                    runtime.host().stores().work(work.lease()),
                                    runtime.host().stores().work(independent.lease()),
                                )
                            })
                            .await
                            .expect("active runtime service")
                    })
                };
                let mut observer = std::pin::pin!(work.observe());
                let mut cx = Context::from_waker(Waker::noop());
                for index in 0..request_count {
                    assert!(
                        observer.as_mut().poll(&mut cx).is_pending(),
                        "{name} request {index}"
                    );
                    let request = driver
                        .work
                        .next(&mut cx)
                        .expect("composition requested its runtime");
                    if index == discarded {
                        drop(request);
                        assert!(
                            executor
                                .poll(std::pin::pin!(std::future::pending::<()>()).as_mut())
                                .is_pending()
                        );
                        break;
                    }
                    driver.dispatch(request, driver.budget.get());
                    assert!(
                        executor
                            .poll(std::pin::pin!(std::future::pending::<()>()).as_mut())
                            .is_pending()
                    );
                }
                let result = observer.as_mut().poll(&mut cx);
                if discarded < request_count {
                    assert_eq!(
                        result.map(Result::err),
                        Poll::Ready(Some(Cancelled)),
                        "{name} discarded at {discarded}"
                    );
                } else {
                    let completed = completed_observation(result);
                    completed.read(|value| assert!(value.is_ok(), "{name} completes"));
                }
                completed_observation(std::pin::Pin::new(&mut independent.observe()).poll(&mut cx))
                    .read(|value| {
                        value
                            .as_ref()
                            .ok()
                            .expect("independent success")
                            .read(|value| assert_eq!(value.value(), &EvaluatedValue::Int(7.into())))
                    });
                drop(driver);
                assert_eq!(echo, ["src/library.gleam:4\n7"]);
            }
        }
    }

    #[test]
    fn native_work_is_unpolled_at_construction_and_preserves_completion_and_error_origin() {
        fn invoke<'call>(
            call: crate::HostCall<
                'call,
                NativeProfile,
                NativeProvider,
                crate::work_fixture::WorkHostType<num_bigint::BigInt>,
            >,
            constructions: crate::HostConstructions<'call, crate::HostTypeListEnd>,
            callback: crate::HostCallable<
                'call,
                crate::HostTypeList<num_bigint::BigInt, crate::HostTypeListEnd>,
                crate::work_fixture::WorkHostType<num_bigint::BigInt>,
            >,
            value: num_bigint::BigInt,
        ) -> Result<
            crate::HostCallContinuation<
                'call,
                crate::work_fixture::WorkHostType<num_bigint::BigInt>,
            >,
            crate::HostCallError,
        > {
            type WorkValue = crate::work_fixture::WorkHostType<num_bigint::BigInt>;
            type OwnedWork =
                crate::provider::Value<WorkValue, crate::provider::ProviderValueContext<WorkValue>>;
            let callback = call.owned_callable(callback, &constructions);
            Ok(call.resume(constructions, move |context| {
                Box::pin(async move {
                    let work = callback
                        .invoke(
                            &context,
                            move |_, _| (value, ()),
                            |call, _, value| Ok(OwnedWork::from_host(&call, value)),
                        )
                        .await?;
                    Ok(crate::HostOwnedCompletion::new(move |mut call, _| {
                        let value = work
                            .into_host(&mut call)
                            .expect("work belongs to this execution");
                        Ok(call.return_value(value))
                    }))
                })
            }))
        }
        for (construction, construction_fails) in [
            ("fetch(40)", false),
            ("invoke(fetch, 40)", false),
            (
                "invoke(fn(_) { panic as \"construction failed\" }, 40)",
                true,
            ),
        ] {
            let source = format!(
                "import fixture/work as future\n@external(erlang, \"native\", \"fetch\")\nfn fetch(value: Int) -> future.Work(Int)\npub fn make() {{\n  let original = {construction}\n  #(future.map(original, fn(value) {{ echo value value + 2 }}), original)\n}}\n@external(erlang, \"native\", \"invoke\")\nfn invoke(callback: fn(Int) -> future.Work(Int), value: Int) -> future.Work(Int)\n"
            );
            let mut providers = crate::work_fixture::WorkComponent::providers::<NativeProfile>()
                .expect("Future provider");
            providers.push(crate::host::HostProviderModule::new("application", "library")
            .expect("native module")
            .with_scoped_function_and_constructions::<NativeProvider, (num_bigint::BigInt,), crate::work_fixture::WorkHostType<num_bigint::BigInt>, crate::HostTypeListEnd, _>("fetch", fetch)
            .expect("native Future function")
            .with_resumable_function::<NativeProvider, (crate::HostFunctionType<crate::HostTypeList<num_bigint::BigInt, crate::HostTypeListEnd>, crate::work_fixture::WorkHostType<num_bigint::BigInt>>, num_bigint::BigInt), crate::work_fixture::WorkHostType<num_bigint::BigInt>, crate::HostTypeListEnd, _>("invoke", invoke)
            .expect("native constructor callback"));
            let typed = compile_typed_host_program(
                "application",
                "library",
                [
                    PackageSource::new(
                        "work_fixture",
                        Vec::<String>::new(),
                        [ModuleSource::new(
                            "fixture/work",
                            "src/fixture/work.gleam",
                            FUTURE_SOURCE,
                        )],
                    ),
                    PackageSource::new(
                        "application",
                        ["work_fixture"],
                        [ModuleSource::new("library", "src/library.gleam", source)],
                    ),
                ],
                HostProviderSet::<NativeProfile>::from_providers(providers)
                    .expect("native provider set"),
            )
            .expect("ordinary Future-typed external");
            let plan = crate::planner::plan_host_library_program(typed).expect("native library");
            let function = plan
                .functions()
                .iter()
                .find(|function| function.name() == "make")
                .expect("make entry")
                .gleam_body()
                .expect("source entry");
            let types = tuple_return(function.signature().shape().type_().return_());
            let entry = LibraryEntry::new(
                function.id(),
                LibraryValueType::Tuple(types),
                Vec::new(),
                Vec::new(),
            );
            let (plan, entries, _) = HostedProgram::from_library_plan(plan, entry, Vec::new())
                .expect("native Future execution");
            let plan = Arc::new(plan);
            let executor = TestHost::default();
            let entry = *entries.tuples[0].function();

            if construction_fails {
                let mut state = NativeState::default();
                let mut stores = crate::host::HostFutureStore::default();
                let mut echo = Echo::default();
                let mut driver = Domain::new(
                    Arc::clone(&plan),
                    &executor,
                    &mut state,
                    &mut stores,
                    &mut echo,
                    Default::default(),
                    NonZeroUsize::MIN,
                );
                let entries = driver.context();
                let result = complete_domain(
                    &mut driver,
                    &executor,
                    entries.call(
                        entry,
                        HostCallOrigin::Entry,
                        RetainedInputs::empty().into_retained(),
                    ),
                )
                .expect("active source entry");
                assert_eq!(
                    result.unwrap_err().to_string(),
                    "panic: construction failed"
                );
                drop(driver);
                assert_eq!(state.polls.load(Ordering::SeqCst), 0);
                assert!(echo.output.is_empty());
                continue;
            }

            for outcome in [Ok(0), Err(crate::HostFailure::new("native unavailable"))] {
                let failed = outcome.is_err();
                let (send, gate) = futures_channel::oneshot::channel();
                let polls = Arc::new(AtomicUsize::new(0));
                let mut state = NativeState {
                    gates: [gate].into(),
                    polls: Arc::clone(&polls),
                    ..NativeState::default()
                };
                let mut stores = crate::host::HostFutureStore::default();
                let mut echo = Echo::default();
                let mut driver = Domain::new(
                    Arc::clone(&plan),
                    &executor,
                    &mut state,
                    &mut stores,
                    &mut echo,
                    Default::default(),
                    NonZeroUsize::MIN,
                );
                let (mapped, original) = {
                    let source_entries = driver.context();
                    let services = driver.work.execution();
                    complete_domain(&mut driver, &executor, async move {
                        let values = source_entries
                            .call(
                                entry,
                                HostCallOrigin::Entry,
                                RetainedInputs::empty().into_retained(),
                            )
                            .await
                            .expect("active source entry")
                            .expect("construct native Future");
                        services
                            .with_runtime(move |_plan, runtime| {
                                let [mapped, original] = external_values(&values);
                                (
                                    runtime.host().stores().work(mapped.lease()),
                                    runtime.host().stores().work(original.lease()),
                                )
                            })
                            .await
                            .expect("active runtime service")
                    })
                };
                assert_eq!(polls.load(Ordering::SeqCst), 0);
                let wake = Arc::new(WakeCount::default());
                let waker = Waker::from(Arc::clone(&wake));
                let mut cx = Context::from_waker(&waker);
                let mut observation = Box::pin(mapped.observe());
                assert!(poll_domain(&mut driver, &executor, observation.as_mut()).is_pending());
                assert_eq!(polls.load(Ordering::SeqCst), 1);
                drop(observation);
                send.send(outcome).expect("native operation still retained");
                assert_eq!(polls.load(Ordering::SeqCst), 1);
                let mut observation = Box::pin(mapped.observe());
                let completion = completed_observation(poll_domain(
                    &mut driver,
                    &executor,
                    observation.as_mut(),
                ));
                assert_eq!(polls.load(Ordering::SeqCst), 2);
                drop(observation);
                drop(driver);
                completion.read(|result| match result {
                    Ok(value) => {
                        assert!(!failed);
                        value.read(|value| {
                            assert_eq!(value.value(), &EvaluatedValue::Int(42.into()))
                        });
                        assert_eq!(echo.output.len(), 1);
                        assert!(echo.output[0].ends_with("\n40"));
                    }
                    Err(error) => {
                        assert!(failed);
                        assert!(echo.output.is_empty());
                        error.read(|error| {
                            let error = host_error(error);
                            assert_eq!(error.package(), "application");
                            assert_eq!(error.module(), "library");
                            assert_eq!(error.function(), "fetch");
                            if construction == "fetch(40)" {
                                assert_eq!(error.location().line(), Some(5));
                                assert_eq!(error.location().caller(), None);
                            } else {
                                assert_eq!(error.location().line(), None);
                                let caller = error.location().caller().expect("native caller");
                                assert_eq!(caller.package(), "application");
                                assert_eq!(caller.module(), "library");
                                assert_eq!(caller.function(), "invoke");
                            }
                            assert_eq!(error.failure().message(), "native unavailable");
                        });
                    }
                });
                assert!(Box::pin(mapped.observe()).as_mut().poll(&mut cx).is_ready());
                assert!(
                    Box::pin(original.observe())
                        .as_mut()
                        .poll(&mut cx)
                        .is_ready()
                );
                assert_eq!(polls.load(Ordering::SeqCst), 2);
            }
        }
    }

    impl HostProfile for FutureProfile {
        type RunState = FutureState;
        type ExternalStores = crate::host::HostFutureStore;
        type ExecutionState = ();
    }

    impl crate::host::HostWorkProfile for FutureProfile {
        type Work = crate::work_fixture::WorkComponent;
    }
    impl crate::host::HostComponentProfile<crate::work_fixture::WorkComponent> for FutureProfile {
        fn component_stores(stores: &Self::ExternalStores) -> &crate::host::HostFutureStore {
            stores
        }

        fn component_state(state: &mut Self::RunState) -> &mut () {
            &mut state.unit
        }
    }

    #[test]
    fn then_preserves_native_failure_or_cancellation_before_its_callback_runs() {
        let execution_host = crate::execution_fixture::TestHost::default();

        use crate::embedding::{FunctionDeclaration, HostedModuleBuilder};
        use crate::work_fixture::WorkType;
        use num_bigint::BigInt;
        let source = r#"import fixture/work as future
@external(erlang, "native", "fetch")
fn fetch(value: Int) -> future.Work(Int)
pub fn make() {
  future.then(fetch(40), fn(value) {
    echo value
    future.ready(value + 2)
  })
}
"#;
        let mut providers = crate::work_fixture::WorkComponent::providers::<NativeProfile>()
            .expect("Future module");
        providers.push(crate::host::HostProviderModule::new("application", "library")
            .expect("native module")
            .with_scoped_function_and_constructions::<NativeProvider, (BigInt,), crate::work_fixture::WorkHostType<BigInt>, crate::HostTypeListEnd, _>("fetch", fetch)
            .expect("native function"));
        let program = compile_typed_host_program(
            "application",
            "library",
            [
                PackageSource::new(
                    "work_fixture",
                    Vec::<String>::new(),
                    [ModuleSource::new(
                        "fixture/work",
                        "src/fixture/work.gleam",
                        FUTURE_SOURCE,
                    )],
                ),
                PackageSource::new(
                    "application",
                    ["work_fixture"],
                    [ModuleSource::new("library", "src/library.gleam", source)],
                ),
            ],
            HostProviderSet::from_providers(providers).expect("providers"),
        )
        .expect("typed source");
        let (bindings, make) = HostedModuleBuilder::new(program)
            .expect("plan")
            .function(FunctionDeclaration::<(), WorkType<BigInt>>::new("make"))
            .expect("binding");
        let mut module = bindings.seal().expect("sealing");
        for (outcome, expected) in [
            (Some(Ok(0)), None),
            (
                Some(Err(crate::HostFailure::new("upstream failed"))),
                Some("host function application::library.fetch failed: upstream failed"),
            ),
            (None, Some("the Future operation was cancelled")),
        ] {
            let (send, gate) = futures_channel::oneshot::channel();
            let mut state = NativeState {
                gates: [gate].into(),
                ..NativeState::default()
            };
            assert!(std::ptr::eq(
                <crate::work_fixture::WorkComponent as crate::HostProvider<NativeProfile>>::project(
                    &mut state
                ),
                &state.unit
            ));
            let polls = Arc::clone(&state.polls);
            let mut echo = Echo::default();
            execution_host
                .block_on(module.with_execution(
                    &execution_host,
                    &mut state,
                    &mut echo,
                    async |scope| {
                        let work = scope.call(&make, ()).await.expect("construct then");
                        assert_eq!(polls.load(Ordering::SeqCst), 0);
                        assert!(
                            Box::pin(scope.observe(&work))
                                .as_mut()
                                .poll(&mut Context::from_waker(Waker::noop()))
                                .is_pending()
                        );
                        if let Some(outcome) = outcome {
                            send.send(outcome).expect("retained native operation");
                        } else {
                            drop(send);
                        }
                        let result = scope.observe(&work).await;
                        assert_eq!(
                            result.as_ref().err().map(ToString::to_string).as_deref(),
                            expected
                        );
                        if let Ok(value) = result {
                            value.read(|value| assert_eq!(*value, BigInt::from(42)));
                        }
                        assert_eq!(
                            scope
                                .observe(&work)
                                .await
                                .as_ref()
                                .err()
                                .map(ToString::to_string)
                                .as_deref(),
                            expected
                        );
                    },
                ))
                .expect("host-controlled completion")
                .try_into_value()
                .unwrap();
            assert_eq!(polls.load(Ordering::SeqCst), 2);
            assert_eq!(echo.output.len(), usize::from(expected.is_none()));
        }
    }

    #[test]
    fn source_all_polls_independent_inputs_and_does_not_cancel_a_separately_retained_sibling() {
        let source = r#"import fixture/work as future
@external(erlang, "native", "fetch")
fn fetch(value: Int) -> future.Work(Int)
pub fn make() {
  let left = fetch(10)
  let right = fetch(20)
  #(future.all([left, left, right]), right)
}
"#;
        let mut providers = crate::work_fixture::WorkComponent::providers::<NativeProfile>()
            .expect("Future provider");
        providers.push(crate::host::HostProviderModule::new("application", "library")
            .expect("all native module")
            .with_scoped_function_and_constructions::<NativeProvider, (num_bigint::BigInt,), crate::work_fixture::WorkHostType<num_bigint::BigInt>, crate::HostTypeListEnd, _>("fetch", fetch)
            .expect("all native function"));
        let typed = compile_typed_host_program(
            "application",
            "library",
            [
                PackageSource::new(
                    "work_fixture",
                    Vec::<String>::new(),
                    [ModuleSource::new(
                        "fixture/work",
                        "src/fixture/work.gleam",
                        FUTURE_SOURCE,
                    )],
                ),
                PackageSource::new(
                    "application",
                    ["work_fixture"],
                    [ModuleSource::new("library", "src/library.gleam", source)],
                ),
            ],
            HostProviderSet::<NativeProfile>::from_providers(providers).expect("all providers"),
        )
        .expect("all source");
        let plan = crate::planner::plan_host_library_program(typed).expect("all library");
        let function = plan
            .functions()
            .iter()
            .find(|function| function.name() == "make")
            .expect("all entry")
            .gleam_body()
            .expect("all source body");
        let types = tuple_return(function.signature().shape().type_().return_());
        let entry = LibraryEntry::new(
            function.id(),
            LibraryValueType::Tuple(types),
            Vec::new(),
            Vec::new(),
        );
        let (plan, entries, _) =
            HostedProgram::from_library_plan(plan, entry, Vec::new()).expect("all execution");
        let plan = Arc::new(plan);
        let executor = TestHost::default();
        let entry = *entries.tuples[0].function();

        for (failure, cancelled) in [(false, false), (true, false), (false, true)] {
            let (left_send, left_wait) = futures_channel::oneshot::channel();
            let (right_send, right_wait) = futures_channel::oneshot::channel();
            let starts = Arc::new(AtomicUsize::new(0));
            let mut state = NativeState {
                gates: [left_wait, right_wait].into(),
                starts: Arc::clone(&starts),
                ..NativeState::default()
            };
            let mut stores = crate::host::HostFutureStore::default();
            let mut echo = Echo::default();
            let mut driver = Domain::new(
                Arc::clone(&plan),
                &executor,
                &mut state,
                &mut stores,
                &mut echo,
                Default::default(),
                NonZeroUsize::MIN,
            );
            let (all, sibling) = {
                let source_entries = driver.context();
                let services = driver.work.execution();
                complete_domain(&mut driver, &executor, async move {
                    let values = source_entries
                        .call(
                            entry,
                            HostCallOrigin::Entry,
                            RetainedInputs::empty().into_retained(),
                        )
                        .await
                        .expect("active source entry")
                        .expect("construct independent work");
                    services
                        .with_runtime(move |_plan, runtime| {
                            let [all, sibling] = external_values(&values);
                            (
                                runtime.host().stores().work(all.lease()),
                                runtime.host().stores().work(sibling.lease()),
                            )
                        })
                        .await
                        .expect("active runtime service")
                })
            };
            let _cx = Context::from_waker(Waker::noop());
            assert!(
                poll_domain(&mut driver, &executor, Box::pin(all.observe()).as_mut()).is_pending()
            );
            assert_eq!(starts.load(Ordering::SeqCst), 2);
            if failure || cancelled {
                if cancelled {
                    drop(left_send);
                } else {
                    left_send
                        .send(Err(crate::HostFailure::new("left failed")))
                        .expect("left waiter");
                }
                let completion =
                    poll_domain(&mut driver, &executor, Box::pin(all.observe()).as_mut());
                if cancelled {
                    assert_eq!(completion.map(Result::err), Poll::Ready(Some(Cancelled)));
                } else {
                    completed_observation(completion).read(|result| assert!(result.is_err()));
                }
                drop(all);
                right_send
                    .send(Ok(2))
                    .expect("retained sibling was not cancelled");
                let completion = completed_observation(poll_domain(
                    &mut driver,
                    &executor,
                    Box::pin(sibling.observe()).as_mut(),
                ));
                completion.read(|result| {
                    let value = result.as_ref().ok().expect("sibling success");
                    value.read(|value| assert_eq!(value.value(), &EvaluatedValue::Int(22.into())));
                });
            } else {
                right_send.send(Ok(2)).expect("right waiter");
                assert!(
                    poll_domain(&mut driver, &executor, Box::pin(all.observe()).as_mut())
                        .is_pending()
                );
                left_send.send(Ok(1)).expect("left still pending");
                let completion = completed_observation(poll_domain(
                    &mut driver,
                    &executor,
                    Box::pin(all.observe()).as_mut(),
                ));
                completion.read(|result| {
                    let value = result.as_ref().ok().expect("all success");
                    value.read(|value| {
                        let list = crate::runtime::StoredRuntimeList::new(
                            crate::runtime::BorrowedValue::from_stored(value).list(),
                            crate::runtime::ValueRetention::new(value.metadata()),
                        );
                        assert_eq!(list.decode_item(0, |item| item.into_int()), Some(11.into()));
                        assert_eq!(list.decode_item(1, |item| item.into_int()), Some(11.into()));
                        assert_eq!(list.decode_item(2, |item| item.into_int()), Some(22.into()));
                        assert_eq!(list.len(), 3);
                    });
                });
                assert!(
                    poll_domain(&mut driver, &executor, Box::pin(sibling.observe()).as_mut())
                        .is_ready()
                );
            }
            drop(driver);
            assert!(echo.output.is_empty());
        }
    }

    #[test]
    fn ordinary_source_constructs_and_maps_values_and_functions_without_implicit_awaiting() {
        for source in [
            r#"
import fixture/work as future
pub fn make() {
  let original = future.ready(40)
  let mapped = future.map(original, fn(value) { echo value value + 2 })
  echo mapped
  #(mapped, original == original, original == future.ready(40))
}
"#,
            r#"
import fixture/work as future
type Packet {
  Packet(String, List(Int), #(Int, Int), BitArray, fn(Int) -> Int)
}
pub fn make() {
  let saved = 40
  let add = fn(value) { saved + value }
  let original = future.ready(Ok(Packet("saved", [1, 2], #(3, 4), <<42>>, add)))
  let mapped = future.map(original, fn(packet) {
    let assert Ok(Packet(name, [one, two], #(three, four), bytes, callback)) = packet
    let assert True = name == "saved" && one + two + three + four == 10 && bytes == <<42>>
    echo 40
    callback(2)
  })
  echo mapped
  #(mapped, original == original, original == future.ready(Error(Nil)))
}
"#,
            r#"
import fixture/work as future
fn chain(remaining: Int, work: future.Work(Int)) -> future.Work(Int) {
  case remaining {
    0 -> work
    _ -> chain(remaining - 1, future.map(work, fn(value) { value }))
  }
}
pub fn make() {
  let original = future.ready(40)
  let mapped = future.map(chain(5000, original), fn(value) { echo value value + 2 })
  echo mapped
  #(mapped, original == original, original == future.ready(40))
}
"#,
            r#"
import fixture/work as future
pub fn make() {
  let captured = 40
  let original = future.ready(fn(value: Int) { captured + value })
  let mapped = future.map(original, fn(add) { echo 40 add(2) })
  echo mapped
  #(mapped, original == original, original == future.ready(fn(value: Int) { value }))
}
"#,
            r#"
import fixture/work as future
pub fn make() {
  let original = future.ready(40)
  let mapped = future.then(original, fn(value) { echo value future.ready(value + 2) })
  echo mapped
  #(mapped, original == original, original == future.ready(40))
}
"#,
            r#"
import fixture/work as future
pub fn make() {
  let original = future.ready(20)
  let mapped = future.map(future.all([original, original]), fn(values) {
    let assert [first, second] = values
    echo first + second
    first + second + 2
  })
  echo mapped
  #(mapped, original == original, original == future.ready(20))
}
"#,
            r#"
import fixture/work as future
pub fn make() {
  let original = future.ready(40)
  let mapped = future.map(future.all([]), fn(values: List(Int)) {
    let assert [] = values
    echo 40
    42
  })
  echo mapped
  #(mapped, original == original, original == future.ready(40))
}
"#,
        ] {
            let typed = compile_typed_host_program(
                "application",
                "library",
                [
                    PackageSource::new(
                        "work_fixture",
                        Vec::<String>::new(),
                        [ModuleSource::new(
                            "fixture/work",
                            "src/fixture/work.gleam",
                            FUTURE_SOURCE,
                        )],
                    ),
                    PackageSource::new(
                        "application",
                        ["work_fixture"],
                        [ModuleSource::new("library", "src/library.gleam", source)],
                    ),
                ],
                HostProviderSet::<FutureProfile>::from_providers(
                    crate::work_fixture::WorkComponent::providers().expect("Future registration"),
                )
                .expect("source providers"),
            )
            .expect("ordinary dependency source");
            let plan = crate::planner::plan_host_library_program(typed).expect("Future library");
            let function = plan
                .functions()
                .iter()
                .find(|function| function.name() == "make")
                .expect("make")
                .gleam_body()
                .expect("source entry");
            let types = tuple_return(function.signature().shape().type_().return_());
            let entry = LibraryEntry::new(
                function.id(),
                LibraryValueType::Tuple(types),
                Vec::new(),
                Vec::new(),
            );
            let (plan, entries, _) = HostedProgram::from_library_plan(plan, entry, Vec::new())
                .expect("specialized Future callbacks");
            let plan = Arc::new(plan);
            let executor = TestHost::default();
            let entry = *entries.tuples[0].function();
            let mut host = FutureState { unit: () };
            assert!(std::ptr::eq(
                <crate::work_fixture::WorkComponent as crate::HostProvider<FutureProfile>>::project(
                    &mut host
                ),
                &host.unit,
            ));
            let mut stores = crate::host::HostFutureStore::default();
            let mut echo = Echo::default();
            let mut driver = Domain::new(
                Arc::clone(&plan),
                &executor,
                &mut host,
                &mut stores,
                &mut echo,
                Default::default(),
                NonZeroUsize::MIN,
            );
            let work = {
                let source_entries = driver.context();
                let services = driver.work.execution();
                complete_domain(&mut driver, &executor, async move {
                    let values = source_entries
                        .call(
                            entry,
                            HostCallOrigin::Entry,
                            RetainedInputs::empty().into_retained(),
                        )
                        .await
                        .expect("active source entry")
                        .expect("construct Future graph");
                    services
                        .with_runtime(move |_plan, runtime| {
                            let [value] = external_values(&values[..1]);
                            assert_eq!(
                                &values[1..],
                                &[EvaluatedValue::Bool(true), EvaluatedValue::Bool(false)]
                            );
                            runtime.host().stores().work(value.lease())
                        })
                        .await
                        .expect("active runtime service")
                })
            };
            let mut observation = Box::pin(work.observe());
            let completion =
                completed_observation(poll_domain(&mut driver, &executor, observation.as_mut()));
            completion.read(|result| {
                let value = result.as_ref().ok().expect("map success");
                value.read(|value| assert_eq!(value.value(), &EvaluatedValue::Int(42.into())));
            });
            drop(observation);
            drop(driver);
            assert_eq!(echo.output.len(), 2);
            assert!(echo.output[0].ends_with("\nWork(...)"));
            assert!(echo.output[1].ends_with("\n40"));
        }
    }

    #[test]
    fn work_hidden_in_a_completed_external_payload_cannot_restart_in_another_execution() {
        use crate::host::{
            HostCall, HostCallCompletion, HostComponentProfile, HostConstructions, HostExternal,
            HostExternalBinding, HostExternalEquality, HostExternalHashing, HostExternalInspection,
            HostExternalSchema, HostExternalStorage, HostExternalStore, HostExternalType,
            HostFutureStore, HostProvider, HostProviderModule, HostTypeListEnd,
        };
        use crate::runtime::StoredRuntimeValue;
        use crate::work_fixture::{WorkComponent, WorkHostType};
        use num_bigint::BigInt;

        struct RetainedProfile;
        struct Provider;
        struct Envelope;
        struct EnvelopeStorage;
        #[derive(Default)]
        struct Stores {
            future: HostFutureStore,
            envelopes: HostExternalStore<StoredRuntimeValue>,
        }
        struct State {
            gate: Option<futures_channel::oneshot::Receiver<Result<(), crate::HostFailure>>>,
            polls: Arc<AtomicUsize>,
            unit: (),
        }
        impl HostProfile for RetainedProfile {
            type RunState = State;
            type ExternalStores = Stores;
            type ExecutionState = ();
        }
        impl HostProvider<RetainedProfile> for Provider {
            type State = State;
            fn project(state: &mut State) -> &mut State {
                state
            }
        }
        impl crate::host::HostWorkProfile for RetainedProfile {
            type Work = crate::work_fixture::WorkComponent;
        }
        impl HostComponentProfile<WorkComponent> for RetainedProfile {
            fn component_stores(stores: &Stores) -> &HostFutureStore {
                &stores.future
            }
            fn component_state(state: &mut State) -> &mut () {
                &mut state.unit
            }
        }
        impl HostExternalSchema for Envelope {
            const PACKAGE: &'static str = "application";
            const MODULE: &'static str = "library";
            const NAME: &'static str = "Envelope";
            const PARAMETER_COUNT: usize = 0;
        }
        impl HostExternalBinding<RetainedProfile, Envelope> for Provider {
            type Storage = EnvelopeStorage;
        }
        impl HostExternalStorage<RetainedProfile, Envelope> for EnvelopeStorage {
            type Payload = StoredRuntimeValue;
            fn store(stores: &Stores) -> &HostExternalStore<Self::Payload> {
                &stores.envelopes
            }
            fn source_equal(
                context: &HostExternalEquality<'_>,
                a: &Self::Payload,
                b: &Self::Payload,
            ) -> bool {
                context.provider_stored_values_equal(a, b)
            }
            fn source_hash(context: &HostExternalHashing<'_>, value: &Self::Payload) -> u64 {
                context.provider_stored_value_hash(value)
            }
            fn inspect(
                context: &HostExternalInspection<'_>,
                value: &Self::Payload,
            ) -> ecow::EcoString {
                format!("Envelope({})", context.provider_inspect_stored_value(value)).into()
            }
        }
        fn fetch<'call>(
            mut call: HostCall<'call, RetainedProfile, Provider, WorkHostType<BigInt>>,
            constructions: HostConstructions<'call, HostTypeListEnd>,
        ) -> Result<HostCallCompletion<'call, WorkHostType<BigInt>>, crate::HostCallError> {
            let mut gate = call.state().gate.take().expect("one original construction");
            let polls = Arc::clone(&call.state().polls);
            Ok(call.return_future(constructions, move |_| {
                Box::pin(async move {
                    std::future::poll_fn(move |cx| {
                        polls.fetch_add(1, Ordering::SeqCst);
                        std::pin::Pin::new(&mut gate).poll(cx)
                    })
                    .await
                    .map_err(|_| crate::HostExecutionError::Cancelled)??;
                    Ok(crate::host::HostOwnedCompletion::new(|call, _| {
                        Ok(call.return_value(42.into()))
                    }))
                })
            }))
        }
        fn retain<'call>(
            mut call: HostCall<'call, RetainedProfile, Provider, HostExternalType<Envelope>>,
            value: HostExternal<'call, WorkHostType<BigInt>>,
        ) -> Result<HostCallCompletion<'call, HostExternalType<Envelope>>, crate::HostCallError>
        {
            let stored = call.retain_value::<WorkHostType<BigInt>>(value);
            let value = call.create_external_with_binding::<Provider>(stored);
            Ok(call.return_value(value))
        }
        fn restore<'call>(
            mut call: HostCall<'call, RetainedProfile, Provider, WorkHostType<BigInt>>,
            value: HostExternal<'call, HostExternalType<Envelope>>,
        ) -> Result<HostCallCompletion<'call, WorkHostType<BigInt>>, crate::HostCallError> {
            let payload = call.external_payload(value);
            let work = call.restore_value::<WorkHostType<BigInt>>(&payload)?;
            drop(payload);
            Ok(call.return_value(work))
        }
        fn hash<'call>(
            call: HostCall<'call, RetainedProfile, Provider, BigInt>,
            value: HostExternal<'call, HostExternalType<Envelope>>,
        ) -> Result<HostCallCompletion<'call, BigInt>, crate::HostCallError> {
            let hash = call.source_hash::<HostExternalType<Envelope>>(value);
            Ok(call.return_value(hash.into()))
        }
        fn observe_native<'call>(
            call: HostCall<'call, RetainedProfile, Provider, WorkHostType<BigInt>>,
            constructions: HostConstructions<'call, HostTypeListEnd>,
            work: HostExternal<'call, WorkHostType<BigInt>>,
        ) -> Result<HostCallCompletion<'call, WorkHostType<BigInt>>, crate::HostCallError> {
            let work = call.future_value(work);
            Ok(call.return_future(constructions, move |context| {
                Box::pin(async move {
                    let value = work.observe(&context, |_, value| Ok(value)).await?;
                    if value < BigInt::from(0) {
                        return Err(crate::HostFailure::new("negative completion").into());
                    }
                    Ok(crate::host::HostOwnedCompletion::new(move |call, _| {
                        Ok(call.return_value(value))
                    }))
                })
            }))
        }
        let native = HostProviderModule::new("application", "library")
            .expect("native module")
            .with_external_type::<Provider, Envelope>().expect("Envelope schema")
            .with_scoped_function_and_constructions::<Provider, (), WorkHostType<BigInt>, HostTypeListEnd, _>("fetch", fetch).expect("fetch")
            .with_scoped_function::<Provider, (WorkHostType<BigInt>,), HostExternalType<Envelope>, _>("retain", retain).expect("retain")
            .with_scoped_function::<Provider, (HostExternalType<Envelope>,), WorkHostType<BigInt>, _>("restore", restore).expect("restore")
            .with_scoped_function_and_constructions::<Provider, (WorkHostType<BigInt>,), WorkHostType<BigInt>, HostTypeListEnd, _>("observe_native", observe_native).expect("explicit native observation")
            .with_scoped_function::<Provider, (HostExternalType<Envelope>,), BigInt, _>("hash", hash).expect("hash");
        let mut providers = WorkComponent::providers().expect("Future component");
        providers.push(native);
        let source = r#"
import fixture/work as future
pub type Envelope
@external(erlang, "native", "fetch")
fn fetch() -> future.Work(Int)
@external(erlang, "native", "retain")
fn retain(value: future.Work(Int)) -> Envelope
@external(erlang, "native", "restore")
fn restore(value: Envelope) -> future.Work(Int)
pub fn make() {
  let inner = future.map(fetch(), fn(value) { echo value value + 1 })
  let envelope = retain(inner)
  echo envelope
  let alias = retain(inner)
  let assert True = envelope == alias && hash(envelope) == hash(alias)
  #(future.ready(envelope), inner)
}
pub fn open(envelope: Envelope) {
  let work = restore(envelope)
  #(work, observe_native(work), future.all([work]))
}
@external(erlang, "native", "hash")
fn hash(envelope: Envelope) -> Int
@external(erlang, "native", "observe_native")
fn observe_native(work: future.Work(Int)) -> future.Work(Int)
pub fn observe_negative() { #(observe_native(future.ready(-1))) }
pub fn observe_ready() { #(observe_native(future.ready(43))) }
pub fn observe_pending() { #(observe_native(fetch())) }
pub fn observe_failed() {
  #(observe_native(future.map(future.ready(0), fn(_) { panic as "observed source failure" })))
}
"#;
        let typed = compile_typed_host_program(
            "application",
            "library",
            [
                PackageSource::new(
                    "work_fixture",
                    Vec::<String>::new(),
                    [ModuleSource::new(
                        "fixture/work",
                        "src/fixture/work.gleam",
                        FUTURE_SOURCE,
                    )],
                ),
                PackageSource::new(
                    "application",
                    ["work_fixture"],
                    [ModuleSource::new("library", "src/library.gleam", source)],
                ),
            ],
            HostProviderSet::<RetainedProfile>::from_providers(providers).expect("providers"),
        )
        .expect("source");
        let library = crate::planner::plan_host_library_program(typed).expect("library");
        let mut entries = Vec::new();
        for name in [
            "make",
            "open",
            "observe_negative",
            "observe_ready",
            "observe_pending",
            "observe_failed",
        ] {
            let function = library
                .functions()
                .iter()
                .find(|function| function.name() == name)
                .expect("source entry")
                .gleam_body()
                .expect("source body");
            let elements = tuple_return(function.signature().shape().type_().return_());
            entries.push(LibraryEntry::new(
                function.id(),
                LibraryValueType::Tuple(elements),
                Vec::new(),
                Vec::new(),
            ));
        }
        let entry = entries.remove(0);
        let (plan, entries, _) =
            HostedProgram::from_library_plan(library, entry, entries).expect("sealed entries");
        let plan = Arc::new(plan);
        let executor = TestHost::default();
        let make = *entries.tuples[0].function();
        let open = *entries.tuples[1].function();
        let source_failure = *entries.tuples[5].function();
        for (entry, cancelled, failed) in [
            (*entries.tuples[2].function(), false, true),
            (*entries.tuples[3].function(), true, false),
            (*entries.tuples[3].function(), false, false),
            (source_failure, false, true),
        ] {
            let polls = Arc::new(AtomicUsize::new(0));
            let mut state = State {
                gate: None,
                polls: Arc::clone(&polls),
                unit: (),
            };
            let mut stores = Stores::default();
            let mut echo = Echo::default();
            let mut driver = Domain::new(
                Arc::clone(&plan),
                &executor,
                &mut state,
                &mut stores,
                &mut echo,
                Default::default(),
                NonZeroUsize::MIN,
            );
            let work = {
                let source_entries = driver.context();
                let services = driver.work.execution();
                complete_domain(&mut driver, &executor, async move {
                    let values = source_entries
                        .call(
                            entry,
                            HostCallOrigin::Entry,
                            RetainedInputs::empty().into_retained(),
                        )
                        .await
                        .expect("active source entry")
                        .expect("native observation of a ready input");
                    services
                        .with_runtime(move |_plan, runtime| {
                            let [value] = external_values(&values);
                            runtime.host().stores().future.work(value.lease())
                        })
                        .await
                        .expect("active runtime service")
                })
            };
            let mut cx = Context::from_waker(Waker::noop());
            if cancelled {
                let mut observer = std::pin::pin!(work.observe());
                assert!(observer.as_mut().poll(&mut cx).is_pending());
                let request = driver.work.next(&mut cx).expect("decode the input");
                driver.dispatch(request, driver.budget.get());
                assert!(observer.as_mut().poll(&mut cx).is_pending());
                drop(driver.work.next(&mut cx).expect("encode the completion"));
                assert_eq!(
                    observer.as_mut().poll(&mut cx).map(Result::err),
                    Poll::Ready(Some(Cancelled))
                );
            } else {
                let result = completed_observation(poll_domain(
                    &mut driver,
                    &executor,
                    Box::pin(work.observe()).as_mut(),
                ));
                result.read(|result| {
                    if failed {
                        result
                            .as_ref()
                            .err()
                            .expect("native validation failed")
                            .read(|error| {
                                if entry == source_failure {
                                    assert_eq!(error.to_string(), "panic: observed source failure");
                                } else {
                                    assert_eq!(
                                        host_error(error).failure().message(),
                                        "negative completion"
                                    );
                                }
                            })
                    } else {
                        result
                            .as_ref()
                            .ok()
                            .expect("native observation completed")
                            .read(|value| {
                                assert_eq!(value.value(), &EvaluatedValue::Int(43.into()));
                            });
                    }
                });
            }
            drop(driver);
            assert_eq!(polls.load(Ordering::SeqCst), 0);
            assert!(echo.output.is_empty());
        }
        {
            let (sender, gate) = futures_channel::oneshot::channel();
            let mut state = State {
                gate: Some(gate),
                polls: Arc::new(AtomicUsize::new(0)),
                unit: (),
            };
            let mut stores = Stores::default();
            let mut echo = Echo::default();
            let mut driver = Domain::new(
                Arc::clone(&plan),
                &executor,
                &mut state,
                &mut stores,
                &mut echo,
                Default::default(),
                NonZeroUsize::MIN,
            );
            let source_entries = driver.context();
            let services = driver.work.execution();
            let entry = *entries.tuples[4].function();
            let work = complete_domain(&mut driver, &executor, async move {
                let values = source_entries
                    .call(
                        entry,
                        HostCallOrigin::Entry,
                        RetainedInputs::empty().into_retained(),
                    )
                    .await
                    .expect("original execution is live")
                    .expect("observe pending work");
                services
                    .with_runtime(move |_plan, runtime| {
                        let [value] = external_values(&values);
                        runtime.host().stores().future.work(value.lease())
                    })
                    .await
                    .expect("original runtime is live")
            });
            let mut observer = std::pin::pin!(work.observe());
            assert!(poll_domain(&mut driver, &executor, observer.as_mut()).is_pending());
            drop(sender);
            assert_eq!(
                poll_domain(&mut driver, &executor, observer.as_mut()).map(Result::err),
                Poll::Ready(Some(Cancelled)),
            );
        }
        for (poll_inner, complete_inner, failed) in [
            (false, false, false),
            (true, false, false),
            (true, true, false),
            (true, true, true),
        ] {
            let (send, receive) = futures_channel::oneshot::channel();
            let polls = Arc::new(AtomicUsize::new(0));
            let mut state = State {
                gate: Some(receive),
                polls: Arc::clone(&polls),
                unit: (),
            };
            assert!(std::ptr::eq(
                <WorkComponent as HostProvider<RetainedProfile>>::project(&mut state),
                &state.unit,
            ));
            let mut stores = Stores::default();
            let mut echo = Echo::default();
            let mut driver = Domain::new(
                Arc::clone(&plan),
                &executor,
                &mut state,
                &mut stores,
                &mut echo,
                Default::default(),
                NonZeroUsize::MIN,
            );
            let (outer, inner) = {
                let source_entries = driver.context();
                let services = driver.work.execution();
                complete_domain(&mut driver, &executor, async move {
                    let values = source_entries
                        .call(
                            make,
                            HostCallOrigin::Entry,
                            RetainedInputs::empty().into_retained(),
                        )
                        .await
                        .expect("active source entry")
                        .expect("construct retained graph");
                    services
                        .with_runtime(move |_plan, runtime| {
                            let [outer, inner] = external_values(&values);
                            let store = &runtime.host().stores().future;
                            (store.work(outer.lease()), store.work(inner.lease()))
                        })
                        .await
                        .expect("active runtime service")
                })
            };
            let completed = completed_observation(poll_domain(
                &mut driver,
                &executor,
                Box::pin(outer.observe()).as_mut(),
            ));
            if poll_inner {
                assert!(
                    poll_domain(&mut driver, &executor, Box::pin(inner.observe()).as_mut())
                        .is_pending()
                );
            }
            let send = if complete_inner {
                send.send(if failed {
                    Err(crate::HostFailure::new("original failed"))
                } else {
                    Ok(())
                })
                .expect("original operation remains live");
                let value = completed_observation(poll_domain(
                    &mut driver,
                    &executor,
                    Box::pin(inner.observe()).as_mut(),
                ));
                value.read(|value| match value {
                    Ok(value) => {
                        assert!(!failed);
                        value.read(|value| {
                            assert_eq!(value.value(), &EvaluatedValue::Int(43.into()))
                        });
                    }
                    Err(error) => {
                        assert!(failed);
                        error.read(|error| {
                            assert_eq!(host_error(error).failure().message(), "original failed")
                        });
                    }
                });
                None
            } else {
                Some(send)
            };
            let retained_envelope = completed.read(|result| {
                result
                    .as_ref()
                    .ok()
                    .expect("original envelope completed")
                    .clone()
            });
            let retained_envelope = retained_envelope.read(|value| value.value().clone());
            let restored_identity = {
                let source_entries = driver.context();
                let services = driver.work.execution();
                complete_domain(&mut driver, &executor, async move {
                    let mut input = RetainedInputs::empty();
                    input.push_value(retained_envelope);
                    let values = source_entries
                        .call(open, HostCallOrigin::Entry, input.into_retained())
                        .await
                        .expect("original execution is live")
                        .expect("restore keeps the original work");
                    services
                        .with_runtime(move |_plan, runtime| {
                            let [restored, _, _] = external_values(&values);
                            runtime
                                .host()
                                .stores()
                                .future
                                .work(restored.lease())
                                .identity()
                        })
                        .await
                        .expect("original runtime is live")
                })
            };
            assert_eq!(restored_identity, inner.identity());
            drop(inner);
            drop(driver);
            if let Some(send) = send {
                assert!(send.is_canceled());
                assert!(send.send(Ok(())).is_err());
            }
            let expected_polls = usize::from(poll_inner) + usize::from(complete_inner);
            assert_eq!(polls.load(Ordering::SeqCst), expected_polls);
            let completed =
                completed.read(|result| result.as_ref().ok().expect("completed outer").clone());
            let envelope = completed.read(|value| value.value().clone());
            let mut fresh = State {
                gate: None,
                polls: Arc::clone(&polls),
                unit: (),
            };
            let mut next = Domain::new(
                Arc::clone(&plan),
                &executor,
                &mut fresh,
                &mut stores,
                &mut echo,
                Default::default(),
                NonZeroUsize::MIN,
            );
            let error = {
                let source_entries = next.context();
                complete_domain(&mut next, &executor, async move {
                    let mut input = RetainedInputs::empty();
                    input.push_value(envelope);
                    source_entries
                        .call(open, HostCallOrigin::Entry, input.into_retained())
                        .await
                        .expect("active restoring entry")
                        .expect_err("a new execution cannot restore the retained work")
                })
            };
            let error = host_error(&error);
            assert_eq!(error.function(), "restore");
            assert_eq!(
                error.failure().message(),
                "retained value belongs to another execution"
            );
            assert!(matches!(
                error.location(),
                crate::HostLocation::Resolved { path, line: 19, .. }
                    if path.as_str() == "src/library.gleam"
            ));
            assert_eq!(
                poll_domain(&mut next, &executor, Box::pin(outer.observe()).as_mut())
                    .map(|result| result.is_ok()),
                Poll::Ready(true)
            );
            assert_eq!(polls.load(Ordering::SeqCst), expected_polls);
            drop(next);
            let mut expected_echo = vec!["src/library.gleam:13\nEnvelope(Work(...))"];
            if complete_inner && !failed {
                expected_echo.push("src/library.gleam:11\n42");
            }
            assert_eq!(echo.output, expected_echo);
        }
    }

    fn completed_observation<Value>(poll: Poll<Result<Value, Cancelled>>) -> Value {
        match poll {
            Poll::Ready(value) => value.expect("live observation"),
            Poll::Pending => panic!("fixture observation is still pending"),
        }
    }

    #[test]
    #[should_panic(expected = "fixture observation is still pending")]
    fn completed_observation_rejects_a_pending_fixture() {
        completed_observation::<()>(Poll::Pending);
    }

    #[test]
    #[should_panic(expected = "live observation")]
    fn completed_observation_rejects_a_cancelled_fixture() {
        completed_observation::<()>(Poll::Ready(Err(Cancelled)));
    }

    fn tuple_return(value: &ValueType) -> Vec<ValueType> {
        match value {
            ValueType::Tuple(elements) => elements.clone(),
            _ => panic!("fixture entry must return a tuple"),
        }
    }

    fn external_values<const N: usize>(
        values: &[EvaluatedValue],
    ) -> [&crate::runtime::EvaluatedExternalValue; N] {
        let values: &[_; N] = values.try_into().expect("fixture external tuple length");
        values.each_ref().map(|value| match value {
            EvaluatedValue::External(value) => value,
            _ => panic!("fixture tuple must contain external values"),
        })
    }

    #[test]
    #[should_panic(expected = "fixture entry must return a tuple")]
    fn tuple_return_rejects_a_scalar_fixture() {
        tuple_return(&ValueType::Int);
    }

    #[test]
    #[should_panic(expected = "fixture tuple must contain external values")]
    fn external_values_rejects_a_scalar_fixture() {
        external_values::<1>(&[EvaluatedValue::Nil]);
    }

    #[test]
    #[should_panic(expected = "fixture external tuple length")]
    fn external_values_rejects_another_tuple_length() {
        external_values::<1>(&[]);
    }
    fn host_error(error: &crate::ExecutionError) -> &crate::HostError {
        match error {
            crate::ExecutionError::Host(error) => error,
            _ => panic!("fixture must fail in a host provider"),
        }
    }

    fn source_panic(error: &crate::ExecutionError) -> &crate::Panic<crate::PanicValue> {
        match error {
            crate::ExecutionError::Panic(error) => error,
            _ => panic!("fixture must stop with a source panic"),
        }
    }

    #[test]
    #[should_panic(expected = "fixture must fail in a host provider")]
    fn host_error_rejects_an_invariant_fixture() {
        host_error(&crate::ExecutionError::Invariant(
            crate::InvariantError::ListIndexOutOfBounds {
                item_type: ValueType::Int,
                index: 0,
                length: 0,
            },
        ));
    }

    #[test]
    #[should_panic(expected = "fixture must stop with a source panic")]
    fn source_panic_rejects_an_invariant_fixture() {
        source_panic(&crate::ExecutionError::Invariant(
            crate::InvariantError::ListIndexOutOfBounds {
                item_type: ValueType::Int,
                index: 0,
                length: 0,
            },
        ));
    }
    fn int_callback(values: &[EvaluatedValue]) -> RetainedCallable {
        let [EvaluatedValue::Function(value)] = values else {
            panic!("fixture must return one callback");
        };
        let EvaluatedFunctionValueKind::Int(value) = value.kind() else {
            panic!("fixture callback must return Int");
        };
        RetainedCallable::new(InvocableFunctionValue::Int(value.clone()))
    }

    #[test]
    #[should_panic(expected = "fixture must return one callback")]
    fn int_callback_rejects_a_scalar_fixture() {
        int_callback(&[EvaluatedValue::Nil]);
    }

    #[test]
    #[should_panic(expected = "fixture callback must return Int")]
    fn int_callback_rejects_a_nil_callback_fixture() {
        let function = crate::runtime::evaluated::EvaluatedNilFunction::reference(
            crate::plan::execution::function::NilFunctionId(0),
            Default::default(),
            crate::plan::execution::type_::FunctionType::new(
                Vec::new(),
                crate::plan::execution::type_::ValueType::Nil,
            ),
        );
        int_callback(&[EvaluatedValue::Function(function.into())]);
    }
}

#[cfg(test)]
mod work_requests {
    use super::{Domain, complete_domain, poll_domain};
    use crate::execution_fixture::TestHost;
    use crate::frontend::compile_typed_host_program;
    use crate::host::{HostProfile, HostProviderModule, HostProviderSet};
    use crate::plan::execution::HostedProgram;
    use crate::plan::execution::function::TupleFunctionId;
    use crate::plan::{LibraryEntry, LibraryValueType, ValueType};
    use crate::runtime::evaluated::{EvaluatedFunctionValueKind, EvaluatedValue};
    use crate::runtime::execution::ExecutionContext;
    use crate::runtime::function::InvocableFunctionValue;
    use crate::runtime::shared::Shared;
    use crate::runtime::work::Cancelled;
    use crate::runtime::{CallbackInputs, HostCallOrigin, RetainedCallable, RetainedInputs};
    use crate::{ModuleSource, PackageSource};
    use num_bigint::BigInt;
    use std::cell::Cell;
    use std::future::Future;
    use std::num::NonZeroUsize;
    use std::pin::pin;
    use std::sync::Arc;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::task::{Context, Poll, Wake, Waker};

    struct Profile;

    impl HostProfile for Profile {
        type RunState = Cell<usize>;
        type ExternalStores = ();
        type ExecutionState = ();
    }

    #[derive(Default)]
    struct WakeCount(AtomicUsize);

    impl Wake for WakeCount {
        fn wake(self: Arc<Self>) {
            self.0.fetch_add(1, Ordering::SeqCst);
        }
    }

    fn callback_program(
        source: &str,
        providers: Vec<HostProviderModule<Profile>>,
    ) -> (Arc<HostedProgram<Profile>>, TupleFunctionId) {
        let providers = HostProviderSet::from_providers(providers).expect("provider set");
        let typed = compile_typed_host_program(
            "application",
            "library",
            [PackageSource::new(
                "application",
                Vec::<String>::new(),
                [ModuleSource::new("library", "src/library.gleam", source)],
            )],
            providers,
        )
        .expect("callback source");
        let plan = crate::planner::plan_host_library_program(typed).expect("callback plan");
        let entry = plan
            .functions()
            .iter()
            .find(|function| function.name() == "make")
            .expect("source entry")
            .gleam_body()
            .expect("Gleam body");
        let entry = LibraryEntry::new(
            entry.id(),
            tuple_entry_type(entry.signature().shape().type_().return_().clone()),
            Vec::new(),
            Vec::new(),
        );
        let (execution, entries, _) = HostedProgram::from_library_plan(plan, entry, Vec::new())
            .expect("sealed callback program");
        (Arc::new(execution), *entries.tuples[0].function())
    }

    fn tuple_entry_type(value: ValueType) -> LibraryValueType {
        let ValueType::Tuple(elements) = value else {
            panic!("fixture returns a tuple of callbacks");
        };
        LibraryValueType::Tuple(elements)
    }

    fn int_callback(
        domain: &mut Domain<'_, Profile>,
        host: &TestHost,
        entry: TupleFunctionId,
    ) -> RetainedCallable {
        let entries = domain.context();
        let values = complete_domain(
            domain,
            host,
            entries.call(
                entry,
                HostCallOrigin::Entry,
                RetainedInputs::empty().into_retained(),
            ),
        )
        .expect("active source entry")
        .expect("source constructs an owned closure");
        let [EvaluatedValue::Function(function)] = values.as_slice() else {
            panic!("fixture returns one function");
        };
        let EvaluatedFunctionValueKind::Int(function) = function.kind() else {
            panic!("fixture callback returns Int");
        };
        RetainedCallable::new(InvocableFunctionValue::Int(function.clone()))
    }

    async fn invoke_after_state_request(
        context: ExecutionContext<Profile>,
        callback: RetainedCallable,
    ) -> Result<crate::runtime::execution::NativeCompletion, Cancelled> {
        let previous = context
            .with_state(|state| {
                let previous = state.get();
                state.set(previous + 1);
                previous
            })
            .await?;
        let mut arguments = CallbackInputs::new();
        arguments.push_value(crate::runtime::EvaluatedValue::Int(previous.into()));
        context
            .invoke(callback, HostCallOrigin::Entry, arguments)
            .await
    }

    #[test]
    fn owned_callback_runs_after_its_creator_returns_using_the_original_state_and_echo() {
        let (plan, entry) = callback_program(
            r#"
    pub fn make() {
      let captured = 40
      #(fn(value: Int) {
        echo value
        captured + value
      })
    }
    "#,
            Vec::new(),
        );
        let mut host = Cell::new(2);
        let mut echo = Vec::new();
        let executor = TestHost::default();
        let mut stores = ();
        let mut execution = Domain::new(
            Arc::clone(&plan),
            &executor,
            &mut host,
            &mut stores,
            &mut echo,
            Default::default(),
            NonZeroUsize::MIN,
        );
        let callback = int_callback(&mut execution, &executor, entry);
        let context = execution.work.execution();
        let wake = Arc::new(WakeCount::default());
        let waker = Waker::from(wake.clone());
        let mut cx = Context::from_waker(&waker);
        let mut state_request = pin!(context.with_state(|state| {
            let before = state.get();
            state.set(before + 1);
            before
        }));
        assert!(state_request.as_mut().poll(&mut cx).is_pending());
        assert_eq!(execution.state.get(), 2);
        let request = execution.work.next(&mut cx).expect("state request");
        let wakes_before_delivery = wake.0.load(Ordering::SeqCst);
        execution.dispatch(request, execution.budget.get());
        assert_eq!(state_request.as_mut().poll(&mut cx), Poll::Ready(Ok(2)));
        assert_eq!(execution.state.get(), 3);
        assert!(wake.0.load(Ordering::SeqCst) > wakes_before_delivery);
        let work_context = execution.work.context();
        let work = work_context.map(
            work_context.ready(crate::runtime::StoredRuntimeValue::test_int(2.into())),
            callback,
            HostCallOrigin::Entry,
        );
        let mut observer = pin!(work.observe());
        assert!(observer.as_mut().poll(&mut cx).is_pending());
        let result =
            completed_observation(poll_domain(&mut execution, &executor, observer.as_mut()));
        result.read(|result| {
            let value = result.as_ref().ok().expect("successful callback");
            value.read(|value| {
                assert_eq!(value.value(), &EvaluatedValue::Int(BigInt::from(42).into()))
            });
        });

        drop(execution);
        let mut repeated = pin!(work.observe());
        let repeated = completed_observation(repeated.as_mut().poll(&mut cx));
        repeated.read(|result| {
            let value = result.as_ref().ok().expect("cached success");
            value.read(|value| {
                assert_eq!(value.value(), &EvaluatedValue::Int(BigInt::from(42).into()))
            });
        });
        assert_eq!(host.get(), 3);
        assert_eq!(echo.len(), 1);
        assert_eq!(echo[0].value(), &crate::Value::Int(BigInt::from(2)));
    }

    #[test]
    fn a_closed_execution_cancels_a_pending_callback_without_an_execution_error() {
        let (plan, entry) = callback_program(
            "pub fn make() { #(fn(value: Int) { value + 1 }) }",
            Vec::new(),
        );
        let mut host = Cell::new(0);
        let mut echo = Vec::new();
        let executor = TestHost::default();
        let mut stores = ();
        let mut execution = Domain::new(
            Arc::clone(&plan),
            &executor,
            &mut host,
            &mut stores,
            &mut echo,
            Default::default(),
            NonZeroUsize::MIN,
        );
        let callback = int_callback(&mut execution, &executor, entry);
        let context = execution.work.execution();
        let mut arguments = CallbackInputs::new();
        arguments.push_value(crate::runtime::EvaluatedValue::Int(41.into()));
        let mut request = pin!(context.invoke(callback, HostCallOrigin::Entry, arguments));
        let mut cx = Context::from_waker(Waker::noop());
        assert!(request.as_mut().poll(&mut cx).is_pending());
        drop(execution);
        assert_eq!(
            request.as_mut().poll(&mut cx).map(Result::err),
            Poll::Ready(Some(Cancelled))
        );
        assert!(echo.is_empty());
    }

    #[test]
    fn retained_native_context_accesses_state_only_while_its_execution_is_open() {
        let (plan, entry) = callback_program(
            "pub fn make() { #(fn(value: Int) { echo value value + 1 }) }",
            Vec::new(),
        );
        for close_before_service in [false, true] {
            let mut host = Cell::new(41);
            let mut echo = Vec::new();
            let executor = TestHost::default();
            let mut stores = ();
            let mut execution = Domain::new(
                Arc::clone(&plan),
                &executor,
                &mut host,
                &mut stores,
                &mut echo,
                Default::default(),
                NonZeroUsize::MIN,
            );
            let callback = int_callback(&mut execution, &executor, entry);
            let mut request = Box::pin(invoke_after_state_request(
                execution.work.execution(),
                callback,
            ));
            let mut cx = Context::from_waker(Waker::noop());
            assert!(request.as_mut().poll(&mut cx).is_pending());
            if close_before_service {
                drop(execution);
                assert_eq!(
                    request.as_mut().poll(&mut cx).map(Result::err),
                    Poll::Ready(Some(Cancelled))
                );
                assert_eq!(host.get(), 41);
                assert!(echo.is_empty());
            } else {
                let value = complete_domain(&mut execution, &executor, request.as_mut())
                    .expect("active execution")
                    .expect("successful callback");
                assert_eq!(value.value(), &EvaluatedValue::Int(42.into()));
                drop(execution);
                assert_eq!(host.get(), 42);
                assert_eq!(echo.len(), 1);
                assert_eq!(echo[0].value(), &crate::Value::Int(41.into()));
            }
        }
    }

    fn int_codec(
        plan: &HostedProgram<Profile>,
        entry: crate::plan::execution::function::IntFunctionId,
    ) -> Option<crate::host::HostCodecScope> {
        use crate::plan::execution::function::{ExecutionFunctionEntry, ExecutionFunctionRef};
        use crate::plan::execution::host::HostedFunctionTarget;
        use crate::plan::execution::runtime::RuntimeExecutionPlan;
        match plan.int_function(entry).as_ref() {
            ExecutionFunctionRef::Host(HostedFunctionTarget::Value(function)) => {
                Some(crate::host::HostCodecScope::new(Arc::clone(
                    plan.host_value_function(function).metadata_handle(),
                )))
            }
            _ => None,
        }
    }

    #[test]
    fn owned_callback_requests_terminate_at_each_conversion_boundary() {
        use crate::host::{HostCall, HostExecutionError, HostProvider};
        struct Provider;
        impl HostProvider<Profile> for Provider {
            type State = Cell<usize>;
            fn project(state: &mut Self::State) -> &mut Self::State {
                state
            }
        }
        fn increment<'call>(
            mut call: HostCall<'call, Profile, Provider, BigInt>,
            value: BigInt,
        ) -> Result<crate::HostCallCompletion<'call, BigInt>, crate::HostCallError> {
            let next = call.state().get() + 1;
            call.state().set(next);
            Ok(call.return_value(value + 1))
        }
        let provider = HostProviderModule::new("application", "library")
            .expect("module")
            .with_scoped_function::<Provider, (BigInt,), BigInt, _>("increment", increment)
            .expect("native function");
        let typed = compile_typed_host_program(
            "application",
            "library",
            [PackageSource::new(
                "application",
                Vec::<String>::new(),
                [ModuleSource::new(
                    "library",
                    "src/library.gleam",
                    r#"
@external(erlang, "native", "increment")
fn increment(value: Int) -> Int
pub fn plain(value: Int) { value }
pub fn make() { #(fn(value: Int) {
  echo value
  case value {
    -1 -> panic as "source rejected"
    _ -> increment(value)
  }
}) }
"#,
                )],
            )],
            HostProviderSet::from_providers([provider]).expect("providers"),
        )
        .expect("ordinary callback source");
        let library = crate::planner::plan_host_library_program(typed).expect("plan");
        let entry = |name: &str, return_| {
            let function = library
                .functions()
                .iter()
                .find(|function| function.name() == name)
                .expect("declared function");
            LibraryEntry::new(function.signature().id(), return_, Vec::new(), Vec::new())
        };
        let make = library
            .functions()
            .iter()
            .find(|function| function.name() == "make")
            .expect("make");
        let return_type = tuple_entry_type(
            make.gleam_body()
                .expect("source body")
                .signature()
                .shape()
                .type_()
                .return_()
                .clone(),
        );
        let first = entry("make", return_type);
        let remaining = vec![
            entry("increment", LibraryValueType::Int),
            entry("plain", LibraryValueType::Int),
        ];
        let (plan, entries, _) =
            HostedProgram::from_library_plan(library, first, remaining).expect("sealed callbacks");
        let plan = Arc::new(plan);
        let executor = TestHost::default();
        let codec =
            int_codec(&plan, *entries.ints[0].function()).expect("source-selected native codec");
        assert!(int_codec(&plan, *entries.ints[1].function()).is_none());

        for (serviced, input, decode_fails) in [
            (0, 41, false),
            (1, 41, false),
            (2, 41, false),
            (3, 41, false),
            (4, 41, false),
            (5, 41, false),
            (5, -1, false),
            (5, -2, false),
            (5, 41, true),
        ] {
            let mut state = Cell::new(0);
            let mut echo = Vec::new();
            let mut stores = ();
            let mut execution = Domain::new(
                Arc::clone(&plan),
                &executor,
                &mut state,
                &mut stores,
                &mut echo,
                Default::default(),
                NonZeroUsize::MIN,
            );
            let callback = int_callback(&mut execution, &executor, *entries.tuples[0].function());
            let context = execution.work.execution();
            let mut invocation = Box::pin(context.invoke_owned(
                callback,
                codec.clone(),
                HostCallOrigin::Entry,
                move |_| {
                    if input == -2 {
                        return Err(crate::HostFailure::new("encoder rejected").into());
                    }
                    let mut values = CallbackInputs::new();
                    values.push_value(EvaluatedValue::Int(input.into()));
                    Ok(values)
                },
                move |runtime, value| {
                    let value = runtime.int(value);
                    if decode_fails {
                        Err(crate::HostFailure::new("decoder rejected").into())
                    } else {
                        Ok(value)
                    }
                },
            ));
            let mut cx = Context::from_waker(Waker::noop());
            let mut result = invocation.as_mut().poll(&mut cx);
            for _ in 0..serviced {
                if result.is_ready() {
                    break;
                }
                let request = execution
                    .work
                    .next(&mut cx)
                    .expect("next conversion boundary");
                execution.dispatch(request, execution.budget.get());
                assert!(
                    executor
                        .poll(std::pin::pin!(std::future::pending::<()>()).as_mut())
                        .is_pending()
                );
                result = invocation.as_mut().poll(&mut cx);
            }
            if serviced < 5 {
                assert!(result.is_pending());
                drop(execution);
                assert!(
                    executor
                        .poll(std::pin::pin!(std::future::pending::<()>()).as_mut())
                        .is_pending()
                );
                result = invocation.as_mut().poll(&mut cx);
                assert_eq!(
                    result.map(|value| value.expect_err("closed endpoint").to_string()),
                    Poll::Ready(HostExecutionError::Cancelled.to_string())
                );
            } else {
                if input == -1 {
                    assert_eq!(
                        result.map(|result| result
                            .expect_err("source panic")
                            .to_string()
                            .contains("source rejected")),
                        Poll::Ready(true)
                    );
                } else if input == -2 {
                    assert_eq!(
                        result.map(|result| result.expect_err("encoder failure").to_string()),
                        Poll::Ready("encoder rejected".to_owned())
                    );
                } else if decode_fails {
                    assert_eq!(
                        result.map(|result| result.expect_err("decoder failure").to_string()),
                        Poll::Ready("decoder rejected".to_owned())
                    );
                } else {
                    assert_eq!(
                        result.map(|result| result.expect("decoded completion")),
                        Poll::Ready(BigInt::from(42))
                    );
                }
                drop(execution);
            }
            assert_eq!(state.get(), usize::from(serviced >= 4 && input >= 0));
            assert_eq!(echo.len(), usize::from(serviced >= 3 && input != -2));
        }
        for close in [false, true] {
            let mut state = Cell::new(0);
            let mut stores = ();
            let mut echo = Vec::new();
            let mut execution = Domain::new(
                Arc::clone(&plan),
                &executor,
                &mut state,
                &mut stores,
                &mut echo,
                Default::default(),
                NonZeroUsize::MIN,
            );
            let context = execution.work.execution();
            let value = Shared::new(crate::runtime::StoredRuntimeValue::test_int(42.into()));
            let mut decoded = Box::pin(context.decode_completion(
                value,
                codec.clone(),
                HostCallOrigin::Entry,
                |runtime, value| Ok(runtime.int(value)),
            ));
            let mut cx = Context::from_waker(Waker::noop());
            assert!(decoded.as_mut().poll(&mut cx).is_pending());
            if close {
                drop(execution);
            } else {
                let request = execution
                    .work
                    .next(&mut cx)
                    .expect("completion decoder request");
                execution.dispatch(request, execution.budget.get());
                drop(execution);
            }
            assert_eq!(
                decoded
                    .as_mut()
                    .poll(&mut cx)
                    .map(|value| value.map_err(|error| error.to_string())),
                Poll::Ready(if close {
                    Err(HostExecutionError::Cancelled.to_string())
                } else {
                    Ok(BigInt::from(42))
                })
            );
            assert_eq!(state.get(), 0);
            assert!(echo.is_empty());
        }
    }

    #[test]
    fn an_abandoned_state_request_is_not_applied_after_the_driver_claims_it() {
        let (plan, _) =
            callback_program("pub fn make() { #(fn(value: Int) { value }) }", Vec::new());
        for cancel in [false, true] {
            let executor = TestHost::default();
            let mut state = Cell::new(1);
            let mut echo = Vec::new();
            let mut stores = ();
            let mut execution = Domain::new(
                Arc::clone(&plan),
                &executor,
                &mut state,
                &mut stores,
                &mut echo,
                Default::default(),
                NonZeroUsize::MIN,
            );
            let context = execution.work.execution();
            let mut request = Box::pin(context.with_state(|state| state.set(99)));
            let mut cx = Context::from_waker(Waker::noop());
            assert!(request.as_mut().poll(&mut cx).is_pending());
            let operation = execution.work.next(&mut cx).expect("claimed request");
            let receiver = if cancel {
                drop(request);
                None
            } else {
                Some(request)
            };
            execution.dispatch(operation, execution.budget.get());
            if let Some(mut receiver) = receiver {
                assert_eq!(receiver.as_mut().poll(&mut cx), Poll::Ready(Ok(())));
            }
            drop(execution);
            assert_eq!(state.get(), if cancel { 1 } else { 99 });
        }
    }

    #[test]
    fn a_claimed_runtime_request_executes_only_while_its_receiver_is_alive() {
        let (plan, _) =
            callback_program("pub fn make() { #(fn(value: Int) { value }) }", Vec::new());
        for cancel in [false, true] {
            let executor = TestHost::default();
            let mut state = Cell::new(1);
            let mut echo = Vec::new();
            let mut stores = ();
            let mut execution = Domain::new(
                Arc::clone(&plan),
                &executor,
                &mut state,
                &mut stores,
                &mut echo,
                Default::default(),
                NonZeroUsize::MIN,
            );
            let context = execution.work.execution();
            let mut request = Box::pin(context.with_runtime(|_, state| state.host_state().set(99)));
            let mut cx = Context::from_waker(Waker::noop());
            assert!(request.as_mut().poll(&mut cx).is_pending());
            let operation = execution
                .work
                .next(&mut cx)
                .expect("claimed runtime request");
            let receiver = if cancel {
                drop(request);
                None
            } else {
                Some(request)
            };
            execution.dispatch(operation, execution.budget.get());
            if let Some(mut receiver) = receiver {
                assert_eq!(receiver.as_mut().poll(&mut cx), Poll::Ready(Ok(())));
            }
            drop(execution);
            assert_eq!(state.get(), if cancel { 1 } else { 99 });
            assert!(echo.is_empty());
        }
    }

    struct NativeProvider;

    impl crate::host::HostProvider<Profile> for NativeProvider {
        type State = Cell<usize>;

        fn project(state: &mut Cell<usize>) -> &mut Self::State {
            state
        }
    }

    fn fail_native<'call>(
        mut call: crate::host::HostCall<'call, Profile, NativeProvider, BigInt>,
        value: BigInt,
    ) -> Result<crate::host::HostCallCompletion<'call, BigInt>, crate::HostCallError> {
        let state = call.state();
        state.set(state.get() + 1);
        Err(crate::HostFailure::new(format!("native rejected {value}")).into())
    }

    #[test]
    fn delayed_native_failure_keeps_its_provider_and_source_location_and_is_shared() {
        let native = HostProviderModule::<Profile>::new("application", "library")
            .expect("native module")
            .with_scoped_function::<NativeProvider, (BigInt,), BigInt, _>("native", fail_native)
            .expect("native function");
        let (plan, entry) = callback_program(
            "@external(erlang, \"native\", \"fail\")\nfn native(value: Int) -> Int\n\npub fn make() {\n  let captured = 40\n  #(fn(value: Int) {\n    echo captured\n    native(captured + value)\n  })\n}\n",
            vec![native],
        );
        let mut host = Cell::new(0);
        let mut echo = Vec::new();
        let executor = TestHost::default();
        let mut stores = ();
        let mut execution = Domain::new(
            Arc::clone(&plan),
            &executor,
            &mut host,
            &mut stores,
            &mut echo,
            Default::default(),
            NonZeroUsize::MIN,
        );
        let callback = int_callback(&mut execution, &executor, entry);
        let context = execution.work.context();
        let work = context.map(
            context.ready(crate::runtime::StoredRuntimeValue::test_int(2.into())),
            callback,
            HostCallOrigin::Entry,
        );
        let mut cx = Context::from_waker(Waker::noop());
        let mut observer = pin!(work.observe());
        assert!(observer.as_mut().poll(&mut cx).is_pending());
        let completion =
            completed_observation(poll_domain(&mut execution, &executor, observer.as_mut()));
        completion.read(|result| {
            let error = result.as_ref().err().expect("native failure");
            error.read(|error| {
                let error = host_error(error);
                assert_eq!(error.package(), "application");
                assert_eq!(error.module(), "library");
                assert_eq!(error.function(), "native");
                assert_eq!(error.failure().message(), "native rejected 42");
                assert_eq!(
                    error.location().path().map(|path| path.as_str()),
                    Some("src/library.gleam")
                );
                assert_eq!(error.location().line(), Some(8));
            });
        });
        drop(execution);
        assert_eq!(host.get(), 1);
        assert_eq!(echo.len(), 1);
        assert_eq!(echo[0].value(), &crate::Value::Int(40.into()));
        let mut repeated = pin!(work.observe());
        assert_eq!(
            repeated.as_mut().poll(&mut cx).map(|result| result.is_ok()),
            Poll::Ready(true)
        );
        assert_eq!(host.get(), 1);
    }

    #[test]
    fn delayed_source_panic_is_not_relabelled_as_cancellation_or_native_failure() {
        let (plan, entry) = callback_program(
            "pub fn make() {\n  #(fn(value: Int) {\n    echo value\n    let assert True = value > 0\n    value\n  })\n}\n",
            Vec::new(),
        );
        let mut host = Cell::new(0);
        let mut echo = Vec::new();
        let executor = TestHost::default();
        let mut stores = ();
        let mut execution = Domain::new(
            Arc::clone(&plan),
            &executor,
            &mut host,
            &mut stores,
            &mut echo,
            Default::default(),
            NonZeroUsize::MIN,
        );
        let callback = int_callback(&mut execution, &executor, entry);
        let context = execution.work.context();
        let work = context.map(
            context.ready(crate::runtime::StoredRuntimeValue::test_int((-1).into())),
            callback,
            HostCallOrigin::Entry,
        );
        let mut cx = Context::from_waker(Waker::noop());
        let mut observer = pin!(work.observe());
        assert!(observer.as_mut().poll(&mut cx).is_pending());
        let completion =
            completed_observation(poll_domain(&mut execution, &executor, observer.as_mut()));
        completion.read(|result| {
            let error = result.as_ref().err().expect("failed assertion");
            error.read(|error| {
                let panic = source_panic(error);
                assert_eq!(panic.kind(), crate::PanicKind::LetAssert);
                assert_eq!(panic.site().module(), "library");
            });
        });
        drop(execution);
        assert_eq!(echo.len(), 1);
        assert_eq!(echo[0].value(), &crate::Value::Int((-1).into()));
        assert_eq!(host.get(), 0);
    }

    #[test]
    fn dropping_a_claimed_callback_request_does_not_execute_its_source() {
        let (plan, entry) = callback_program(
            "pub fn make() { #(fn(value: Int) { echo value value }) }",
            Vec::new(),
        );
        let mut host = Cell::new(0);
        let mut echo = Vec::new();
        let executor = TestHost::default();
        let mut stores = ();
        let mut execution = Domain::new(
            Arc::clone(&plan),
            &executor,
            &mut host,
            &mut stores,
            &mut echo,
            Default::default(),
            NonZeroUsize::MIN,
        );
        let callback = int_callback(&mut execution, &executor, entry);
        let context = execution.work.execution();
        let mut arguments = CallbackInputs::new();
        arguments.push_value(crate::runtime::EvaluatedValue::Int(42.into()));
        let mut request = Box::pin(context.invoke(callback, HostCallOrigin::Entry, arguments));
        let mut cx = Context::from_waker(Waker::noop());
        assert!(request.as_mut().poll(&mut cx).is_pending());
        let request_to_service = execution.work.next(&mut cx).expect("claimed request");
        drop(request);
        execution.dispatch(request_to_service, execution.budget.get());
        executor
            .block_on(execution.drive(std::future::ready(())))
            .expect("cancelled callback cleanup")
            .try_into_value()
            .unwrap();
        assert!(echo.is_empty());
    }

    fn completed_observation<Value>(poll: Poll<Result<Value, Cancelled>>) -> Value {
        match poll {
            Poll::Ready(value) => value.expect("live observation"),
            Poll::Pending => panic!("fixture observation is still pending"),
        }
    }

    #[test]
    #[should_panic(expected = "fixture observation is still pending")]
    fn completed_observation_rejects_a_pending_fixture() {
        completed_observation::<()>(Poll::Pending);
    }

    #[test]
    #[should_panic(expected = "live observation")]
    fn completed_observation_rejects_a_cancelled_fixture() {
        completed_observation::<()>(Poll::Ready(Err(Cancelled)));
    }

    #[test]
    #[should_panic(expected = "fixture returns a tuple of callbacks")]
    fn callback_fixture_rejects_a_non_tuple_entry() {
        callback_program("pub fn make() { 42 }", Vec::new());
    }

    #[test]
    #[should_panic(expected = "fixture returns one function")]
    fn callback_fixture_rejects_a_non_function_tuple() {
        let (plan, entry) = callback_program("pub fn make() { #(42) }", Vec::new());
        let host = TestHost::default();
        let mut state = Cell::new(0);
        let mut echo = Vec::new();
        let mut stores = ();
        let mut execution = Domain::new(
            plan,
            &host,
            &mut state,
            &mut stores,
            &mut echo,
            Default::default(),
            NonZeroUsize::MIN,
        );
        int_callback(&mut execution, &host, entry);
    }

    #[test]
    #[should_panic(expected = "fixture callback returns Int")]
    fn callback_fixture_rejects_another_return_family() {
        let (plan, entry) =
            callback_program("pub fn make() { #(fn(_value: Int) { Nil }) }", Vec::new());
        let host = TestHost::default();
        let mut state = Cell::new(0);
        let mut echo = Vec::new();
        let mut stores = ();
        let mut execution = Domain::new(
            plan,
            &host,
            &mut state,
            &mut stores,
            &mut echo,
            Default::default(),
            NonZeroUsize::MIN,
        );
        int_callback(&mut execution, &host, entry);
    }
    fn host_error(error: &crate::ExecutionError) -> &crate::HostError {
        match error {
            crate::ExecutionError::Host(error) => error,
            _ => panic!("fixture must fail in a host provider"),
        }
    }

    fn source_panic(error: &crate::ExecutionError) -> &crate::Panic<crate::PanicValue> {
        match error {
            crate::ExecutionError::Panic(error) => error,
            _ => panic!("fixture must stop with a source panic"),
        }
    }

    #[test]
    #[should_panic(expected = "fixture must fail in a host provider")]
    fn host_error_rejects_an_invariant_fixture() {
        host_error(&crate::ExecutionError::Invariant(
            crate::InvariantError::ListIndexOutOfBounds {
                item_type: crate::ValueType::Int,
                index: 0,
                length: 0,
            },
        ));
    }

    #[test]
    #[should_panic(expected = "fixture must stop with a source panic")]
    fn source_panic_rejects_an_invariant_fixture() {
        source_panic(&crate::ExecutionError::Invariant(
            crate::InvariantError::ListIndexOutOfBounds {
                item_type: crate::ValueType::Int,
                index: 0,
                length: 0,
            },
        ));
    }
}

#[cfg(test)]
mod string_native_tests {
    use super::Domain;
    use crate::execution_fixture::TestHost;
    use crate::plan::execution::function::{
        ExecutionFunctionEntry, ExecutionFunctionRef, StringFunctionId,
    };
    use crate::plan::execution::graph::StringLocalId;
    use crate::plan::execution::runtime::RuntimeExecutionPlan;
    use crate::plan::{HostCallSite, SourceSpan};
    use crate::runtime::ExecutableRuntimePlan;
    use crate::runtime::compiled::calls::{
        CallExecution, CallInputs, CallOps, CallOutput, CallProgress, CallValues,
        GeneratedNativePhase, GeneratedNativeState, StringNativeExecution, StringNativeRequest,
    };
    use crate::runtime::error::HostCallOrigin;
    use crate::runtime::graph::{BlockEnvironment, GraphValue, RetainedValues};
    use crate::{
        HostCall, HostCallCompletion, HostCallError, HostProfile, HostProvider, HostProviderModule,
        HostProviderSet, ModuleSource, PackageSource, StringValue,
    };
    use std::num::NonZeroUsize;
    use std::sync::Arc;

    struct Profile;
    impl HostProfile for Profile {
        type RunState = Vec<StringValue>;
        type ExternalStores = ();
        type ExecutionState = ();
    }
    impl HostProvider<Profile> for Profile {
        type State = Vec<StringValue>;
        fn project(state: &mut Self::State) -> &mut Self::State {
            state
        }
    }
    fn append<'call>(
        mut call: HostCall<'call, Profile, Profile, StringValue>,
        value: StringValue,
    ) -> Result<HostCallCompletion<'call, StringValue>, HostCallError> {
        call.state().push(value.clone());
        Ok(call.return_value(format!("{}!", value.as_str().unwrap()).into()))
    }

    // The service protocol owns an already-published Native boundary. This
    // local engine records one subsequent call and its actual delivered value;
    // selection of compiler-generated engines belongs to public preparation tests.
    struct Delivery {
        function: StringFunctionId,
        site: HostCallSite,
        result: StringValue,
        next_call: bool,
    }
    impl CallExecution for Delivery {
        fn restart(
            &mut self,
            _: crate::plan::execution::compiled::CallTarget,
            _: usize,
            _: CallInputs<'_>,
        ) -> bool {
            false
        }
        fn retained_bytes(&self) -> usize {
            0
        }
        fn advance(mut self: Box<Self>, _: &mut CallOps<'_>, budget: &mut usize) -> CallProgress {
            if self.next_call {
                if *budget == 0 {
                    return CallProgress::Yield(self);
                }
                *budget -= 1;
                self.next_call = false;
                CallProgress::StringNative(StringNativeRequest {
                    function: self.function,
                    site: self.site.clone(),
                    root_tail: false,
                    arguments: Box::new(CallValues {
                        strings: vec![self.result.clone()],
                        ..Default::default()
                    }),
                    execution: self,
                })
            } else {
                CallProgress::Complete {
                    output: CallOutput::String(self.result.clone()),
                    execution: self,
                }
            }
        }
    }
    impl StringNativeExecution for Delivery {
        fn resume_native(mut self: Box<Self>, value: StringValue) -> Box<dyn CallExecution> {
            self.result = value;
            self
        }
    }

    #[test]
    fn native_service_retains_delivery_phase_and_workspace_across_single_unit_grants() {
        let source = r#"
@external(erlang, "native", "append")
fn append(value: String) -> String
pub fn main() { let first = append("input") append(first) }
"#;
        let typed = crate::compile_typed_host_program(
            "application",
            "example",
            [PackageSource::new(
                "application",
                Vec::<String>::new(),
                [ModuleSource::new("example", "src/example.gleam", source)],
            )],
            HostProviderSet::from_providers([HostProviderModule::new("application", "example")
                .unwrap()
                .with_scoped_function::<Profile, (StringValue,), StringValue, _>("append", append)
                .unwrap()])
            .unwrap(),
        )
        .unwrap();
        let mut hosted =
            crate::HostedExecution::try_from_module_plan(crate::plan_host_program(typed).unwrap())
                .unwrap();
        let (plan, stores, captures) = hosted.parts_mut();
        let function = (0..2)
            .map(StringFunctionId)
            .find(|id| {
                matches!(
                    plan.string_function(*id).as_ref(),
                    ExecutionFunctionRef::Host(_)
                )
            })
            .unwrap();
        let site = HostCallSite::from_static("example", "main", SourceSpan::new(0, source.len()));
        let mut delivery = Delivery {
            function,
            site: site.clone(),
            result: "".into(),
            next_call: true,
        };
        let empty = BlockEnvironment::from_retained(RetainedValues::empty());
        assert!(!delivery.restart(
            crate::plan::execution::compiled::CallTarget::String(function),
            0,
            CallInputs::new(&empty),
        ));
        assert_eq!(delivery.retained_bytes(), 0);
        let mut state = Box::new(GeneratedNativeState {
            phase: GeneratedNativePhase::Progress(CallProgress::StringNative(
                StringNativeRequest {
                    function,
                    site: site.clone(),
                    root_tail: false,
                    arguments: Box::new(CallValues {
                        strings: vec!["input".into()],
                        ..Default::default()
                    }),
                    execution: Box::new(delivery),
                },
            )),
            numeric: Default::default(),
            strings: None,
            bit_arrays: None,
            root_tail_entry: true,
            domain: captures.domain(),
            prepaid_completion: false,
        });
        let host = TestHost::default();
        let mut effects = Vec::new();
        let mut echo = Vec::new();
        let domain = Domain::new(
            Arc::clone(plan),
            &host,
            &mut effects,
            stores,
            &mut echo,
            captures.clone(),
            NonZeroUsize::MIN,
        );
        let context = domain.context();
        host.block_on(domain.drive(async {
            let original_owner = std::ptr::from_ref(&*state);
            state = plan
                .prepare_generated_native(state, 1)
                .ok()
                .unwrap()
                .submit(context.execution.services(), NonZeroUsize::MIN)
                .await
                .unwrap()
                .unwrap();

            assert_eq!(
                context
                    .execution
                    .with_state(|effects| effects.len())
                    .await
                    .unwrap(),
                0
            );
            let mut grants = 0;
            let output = loop {
                grants += 1;
                assert!(
                    grants < 20,
                    "each bounded phase must make progress without replay"
                );
                state = plan
                    .prepare_generated_native(state, 1)
                    .ok()
                    .unwrap()
                    .submit(context.execution.services(), NonZeroUsize::MIN)
                    .await
                    .unwrap()
                    .unwrap();
                assert_eq!(std::ptr::from_ref(&*state), original_owner);
                state.phase = match state.phase {
                    GeneratedNativePhase::Progress(CallProgress::Complete {
                        output,
                        execution,
                    }) => {
                        if state.prepaid_completion {
                            break output;
                        }
                        GeneratedNativePhase::Progress(CallProgress::Complete { output, execution })
                    }
                    phase => phase,
                };
            };
            assert_eq!(grants, 7);
            assert_eq!(
                StringLocalId::from_call_output(output).unwrap().as_str(),
                Ok("input!!")
            );
        }))
        .unwrap()
        .try_into_value()
        .unwrap();
        assert_eq!(
            effects,
            [StringValue::from("input"), StringValue::from("input!")]
        );
        assert!(echo.is_empty());
    }

    #[test]
    fn native_service_preserves_provider_failure_exit_and_terminal_checkpoints() {
        use crate::HostFailure;
        use crate::execution::{ExecutionOutcome, ExitStatus};
        use crate::plan::execution::compiled::{CallTarget, CompiledCheckpoint};
        use crate::plan::execution::graph::BlockId;
        use std::panic::{AssertUnwindSafe, catch_unwind};

        fn completed_string(phase: &GeneratedNativePhase) -> &StringValue {
            match phase {
                GeneratedNativePhase::Progress(CallProgress::Complete {
                    output: CallOutput::String(value),
                    ..
                }) => value,
                _ => panic!("a root tail Native must publish its delivered String completion"),
            }
        }

        fn fail<'call>(
            mut call: HostCall<'call, Profile, Profile, StringValue>,
            value: StringValue,
        ) -> Result<HostCallCompletion<'call, StringValue>, HostCallError> {
            call.state().push(value);
            Err(HostFailure::new("native refused").into())
        }
        fn stop<'call>(
            mut call: HostCall<'call, Profile, Profile, StringValue>,
            value: StringValue,
        ) -> Result<HostCallCompletion<'call, StringValue>, HostCallError> {
            call.state().push(value);
            call.exit(ExitStatus::new(7))
        }
        type TerminalNative =
            for<'call> fn(
                HostCall<'call, Profile, Profile, StringValue>,
                StringValue,
            )
                -> Result<HostCallCompletion<'call, StringValue>, HostCallError>;
        for (name, callback) in [
            ("fail", fail as TerminalNative),
            ("stop", stop as TerminalNative),
            ("append", append as TerminalNative),
        ] {
            let source = format!(
                "@external(erlang, \"native\", \"{name}\") fn native(value: String) -> String pub fn main() {{ native(\"input\") }}"
            );
            let typed = crate::compile_typed_host_program(
                "application",
                "example",
                [PackageSource::new(
                    "application",
                    Vec::<String>::new(),
                    [ModuleSource::new(
                        "example",
                        "src/example.gleam",
                        source.clone(),
                    )],
                )],
                HostProviderSet::from_providers([HostProviderModule::new(
                    "application",
                    "example",
                )
                .unwrap()
                .with_scoped_function::<Profile, (StringValue,), StringValue, _>("native", callback)
                .unwrap()])
                .unwrap(),
            )
            .unwrap();
            let mut hosted = crate::HostedExecution::try_from_module_plan(
                crate::plan_host_program(typed).unwrap(),
            )
            .unwrap();
            let (plan, stores, captures) = hosted.parts_mut();
            let function = StringFunctionId(
                plan.synchronous_strings()
                    .iter()
                    .position(|enabled| *enabled)
                    .unwrap(),
            );
            let host = TestHost::default();
            let mut effects = Vec::new();
            let mut echo = Vec::new();
            let state = Box::new(GeneratedNativeState {
                phase: GeneratedNativePhase::Progress(CallProgress::StringNative(
                    StringNativeRequest {
                        function,
                        site: HostCallSite::from_static(
                            "example",
                            "main",
                            SourceSpan::new(0, source.len()),
                        ),
                        root_tail: true,
                        arguments: Box::new(CallValues {
                            strings: vec!["input".into()],
                            ..Default::default()
                        }),
                        execution: Box::new(Delivery {
                            function,
                            site: HostCallSite::from_static(
                                "example",
                                "main",
                                SourceSpan::new(0, source.len()),
                            ),
                            result: "".into(),
                            next_call: false,
                        }),
                    },
                )),
                numeric: Default::default(),
                strings: None,
                bit_arrays: None,
                root_tail_entry: true,
                domain: captures.domain(),
                prepaid_completion: false,
            });
            assert!(catch_unwind(AssertUnwindSafe(|| completed_string(&state.phase))).is_err());
            let domain = Domain::new(
                Arc::clone(plan),
                &host,
                &mut effects,
                stores,
                &mut echo,
                captures.clone(),
                NonZeroUsize::new(8).unwrap(),
            );
            let context = domain.context();
            let result = host
                .block_on(domain.drive(async {
                    plan.prepare_generated_native(state, 8)
                        .ok()
                        .unwrap()
                        .submit(context.execution.services(), NonZeroUsize::new(8).unwrap())
                        .await
                }))
                .unwrap();
            if name == "stop" {
                assert!(
                    matches!(result, ExecutionOutcome::Exited(status) if status == ExitStatus::new(7))
                );
            } else if name == "append" {
                let state = result.try_into_value().unwrap().unwrap().unwrap();
                assert!(state.prepaid_completion);
                assert_eq!(completed_string(&state.phase).as_str(), Ok("input!"));
            } else {
                let error = result.try_into_value().unwrap().unwrap().err().unwrap();
                assert!(error.to_string().contains("native refused"));
            }
            assert_eq!(effects, [StringValue::from("input")]);
            assert!(echo.is_empty());
        }
        let source = "pub fn main() { \"kept\" }";
        let typed = crate::compile_typed_host_program(
            "application",
            "example",
            [PackageSource::new(
                "application",
                Vec::<String>::new(),
                [ModuleSource::new("example", "src/example.gleam", source)],
            )],
            HostProviderSet::<Profile>::new([]).unwrap(),
        )
        .unwrap();
        let mut hosted =
            crate::HostedExecution::try_from_module_plan(crate::plan_host_program(typed).unwrap())
                .unwrap();
        let (plan, stores, captures) = hosted.parts_mut();
        let point = CompiledCheckpoint {
            block: BlockId(0),
            instruction: 0,
            ints: 0,
            bools: 0,
            bit_arrays: 0,
            int_lists: 0,
            strings: 0,
            customs: 0,
            custom_lists: 0,
            int_functions: 0,
            bool_functions: 0,
        };
        let state = Box::new(GeneratedNativeState {
            phase: GeneratedNativePhase::Progress(CallProgress::Interpreted {
                target: CallTarget::String(StringFunctionId(0)),
                point,
                values: Box::default(),
            }),
            numeric: Default::default(),
            strings: None,
            bit_arrays: None,
            root_tail_entry: true,
            domain: captures.domain(),
            prepaid_completion: false,
        });
        let owner = std::ptr::from_ref(&*state);
        let host = TestHost::default();
        let mut effects = Vec::new();
        let mut echo = Vec::new();
        let domain = Domain::new(
            Arc::clone(plan),
            &host,
            &mut effects,
            stores,
            &mut echo,
            captures.clone(),
            NonZeroUsize::MIN,
        );
        let context = domain.context();
        let returned = host
            .block_on(domain.drive(async {
                plan.prepare_generated_native(state, 1)
                    .ok()
                    .unwrap()
                    .submit(context.execution.services(), NonZeroUsize::MIN)
                    .await
                    .unwrap()
                    .unwrap()
            }))
            .unwrap()
            .try_into_value()
            .unwrap();
        assert_eq!(std::ptr::from_ref(&*returned), owner);
        assert_eq!(
            std::mem::discriminant(&returned.phase),
            std::mem::discriminant(&GeneratedNativePhase::Progress(CallProgress::Interpreted {
                target: CallTarget::String(StringFunctionId(0)),
                point,
                values: Box::default(),
            }))
        );
        assert!(effects.is_empty());
        assert!(echo.is_empty());
    }

    #[test]
    fn native_completion_cancelled_during_advance_is_never_published() {
        use crate::execution::{ExecutionUnit, UnitOwner};
        struct CancelledCompletion(ExecutionUnit);
        impl CallExecution for CancelledCompletion {
            fn restart(
                &mut self,
                _: crate::plan::execution::compiled::CallTarget,
                _: usize,
                _: CallInputs<'_>,
            ) -> bool {
                false
            }
            fn retained_bytes(&self) -> usize {
                0
            }
            fn advance(self: Box<Self>, _: &mut CallOps<'_>, _: &mut usize) -> CallProgress {
                assert!(self.0.cancel());
                CallProgress::Complete {
                    output: CallOutput::String("unpublished".into()),
                    execution: self,
                }
            }
        }
        let typed = crate::compile_typed_host_program(
            "application",
            "example",
            [PackageSource::new(
                "application",
                Vec::<String>::new(),
                [ModuleSource::new(
                    "example",
                    "src/example.gleam",
                    "pub fn main() { \"kept\" }",
                )],
            )],
            HostProviderSet::<Profile>::new([]).unwrap(),
        )
        .unwrap();
        let mut hosted =
            crate::HostedExecution::try_from_module_plan(crate::plan_host_program(typed).unwrap())
                .unwrap();
        let (plan, stores, captures) = hosted.parts_mut();
        let host = TestHost::default();
        let mut effects = Vec::new();
        let mut echo = Vec::new();
        let mut domain = Domain::new(
            Arc::clone(plan),
            &host,
            &mut effects,
            stores,
            &mut echo,
            captures.clone(),
            NonZeroUsize::MIN,
        );
        let (context, root) = domain.begin(UnitOwner::new(domain.units.completion()));
        let mut completion = CancelledCompletion(context.unit().unwrap().clone());
        let empty = BlockEnvironment::from_retained(RetainedValues::empty());
        assert!(!completion.restart(
            crate::plan::execution::compiled::CallTarget::String(StringFunctionId(0)),
            0,
            CallInputs::new(&empty)
        ));
        assert_eq!(completion.retained_bytes(), 0);
        let state = Box::new(GeneratedNativeState {
            phase: GeneratedNativePhase::Progress(CallProgress::Yield(Box::new(completion))),
            numeric: Default::default(),
            strings: None,
            bit_arrays: None,
            root_tail_entry: true,
            domain: captures.domain(),
            prepaid_completion: false,
        });
        let result = host
            .block_on(domain.drive(async {
                let result = plan
                    .prepare_generated_native(state, 1)
                    .ok()
                    .unwrap()
                    .submit(context.services(), NonZeroUsize::MIN)
                    .await;
                drop(root);
                result
            }))
            .unwrap()
            .try_into_value()
            .unwrap();
        assert!(result.is_err());
        assert!(effects.is_empty());
        assert!(echo.is_empty());
    }

    #[test]
    fn a_later_unsupported_native_request_returns_its_owner_without_invoking_it() {
        use crate::{
            HostCallContinuation, HostConstructions, HostOwnedCompletion, HostTypeListEnd,
        };
        fn later<'call>(
            mut call: HostCall<'call, Profile, Profile, StringValue>,
            constructions: HostConstructions<'call, HostTypeListEnd>,
            value: StringValue,
        ) -> Result<HostCallContinuation<'call, StringValue>, HostCallError> {
            call.state().push(value.clone());
            Ok(call.resume(constructions, move |_| {
                Box::pin(async move {
                    Ok(HostOwnedCompletion::new(move |call, _| {
                        Ok(call.return_value(value))
                    }))
                })
            }))
        }
        let source = r#"
@external(erlang, "native", "append")
fn append(value: String) -> String
@external(erlang, "native", "later")
fn later(value: String) -> String
pub fn main() { let first = append("input") later(first) }
"#;
        let typed = crate::compile_typed_host_program("application", "example",
            [PackageSource::new("application", Vec::<String>::new(),
                [ModuleSource::new("example", "src/example.gleam", source)])],
            HostProviderSet::from_providers([HostProviderModule::new("application", "example").unwrap()
                .with_scoped_function::<Profile, (StringValue,), StringValue, _>("append", append).unwrap()
                .with_resumable_function::<Profile, (StringValue,), StringValue, HostTypeListEnd, _>("later", later).unwrap()]).unwrap()
        ).unwrap();
        let mut hosted =
            crate::HostedExecution::try_from_module_plan(crate::plan_host_program(typed).unwrap())
                .unwrap();
        let (plan, stores, captures) = hosted.parts_mut();
        let synchronous = plan.synchronous_strings();
        let first = StringFunctionId(synchronous.iter().position(|enabled| *enabled).unwrap());
        let next = (0..3)
            .map(StringFunctionId)
            .find(|id| {
                !synchronous[id.0]
                    && matches!(
                        plan.string_function(*id).as_ref(),
                        ExecutionFunctionRef::Host(_)
                    )
            })
            .unwrap();
        let site = HostCallSite::from_static("example", "main", SourceSpan::new(0, source.len()));
        let state = Box::new(GeneratedNativeState {
            phase: GeneratedNativePhase::Progress(CallProgress::StringNative(
                StringNativeRequest {
                    function: first,
                    site: site.clone(),
                    root_tail: false,
                    arguments: Box::new(CallValues {
                        strings: vec!["input".into()],
                        ..Default::default()
                    }),
                    execution: Box::new(Delivery {
                        function: next,
                        site,
                        result: "".into(),
                        next_call: true,
                    }),
                },
            )),
            numeric: Default::default(),
            strings: None,
            bit_arrays: None,
            root_tail_entry: true,
            domain: captures.domain(),
            prepaid_completion: false,
        });
        let owner = std::ptr::from_ref(&*state);
        let host = TestHost::default();
        let mut effects = Vec::new();
        let mut echo = Vec::new();
        let domain = Domain::new(
            Arc::clone(plan),
            &host,
            &mut effects,
            stores,
            &mut echo,
            captures.clone(),
            NonZeroUsize::MIN,
        );
        let context = domain.context();
        host.block_on(domain.drive(async {
            let mut state = state;
            let mut grants = 0;
            let state = loop {
                match plan.prepare_generated_native(state, 32) {
                    Ok(invoke) => {
                        grants += 1;
                        assert!(
                            grants < 20,
                            "bounded phases must reach the unsupported request"
                        );
                        state = invoke
                            .submit(context.execution.services(), NonZeroUsize::new(32).unwrap())
                            .await
                            .unwrap()
                            .unwrap();
                        assert_eq!(std::ptr::from_ref(&*state), owner);
                    }
                    Err(state) => break state,
                }
            };
            assert_eq!(std::ptr::from_ref(&*state), owner);
            drop(state);
            assert_eq!(
                context
                    .execution
                    .with_state(|effects| effects.clone())
                    .await
                    .unwrap(),
                [StringValue::from("input")]
            );
            let mut inputs = RetainedValues::empty();
            inputs.push_string("control".into());
            let returned = context
                .call(next, HostCallOrigin::Entry, inputs)
                .await
                .unwrap()
                .unwrap();
            assert_eq!(returned.as_str(), Ok("control"));
            assert_eq!(
                context
                    .execution
                    .with_state(|effects| effects.clone())
                    .await
                    .unwrap(),
                [StringValue::from("input"), StringValue::from("control")]
            );
        }))
        .unwrap();
        assert!(echo.is_empty());
    }
}

#[cfg(test)]
mod compound_native_tests {
    use super::Domain;
    use crate::execution_fixture::TestHost;
    use crate::plan::HostCallSite;
    use crate::plan::execution::HostedProgram;
    use crate::plan::execution::compiled::CallTarget;
    use crate::plan::execution::function::{
        CoreRuntimeFunctionId, CustomFunctionId, ExecutionFunctionEntry, ExecutionFunctionRef,
        RuntimeFunctionId, StringFunctionId, TupleFunctionId,
    };
    use crate::plan::execution::graph::{
        CustomInstruction, ProfiledInstructionKind, StringInstruction, TupleInstruction,
    };
    use crate::plan::execution::runtime::RuntimeExecutionPlan;
    use crate::runtime::EvaluatedValue;
    use crate::runtime::ExecutableRuntimePlan;
    use crate::runtime::compiled::calls::{
        CallArguments, CallCustom, CallExecution, CallInputs, CallOps, CallOutput, CallProgress,
        CallTuple, CallValues, CustomNativeExecution, CustomNativeRequest, GeneratedNativePhase,
        GeneratedNativeState, StringNativeExecution, StringNativeRequest, TupleNativeExecution,
        TupleNativeRequest,
    };
    use crate::{
        HostCall, HostCallCompletion, HostCallError, HostCustomConstructorAt,
        HostCustomConstructorDefinition, HostCustomConstructorList, HostCustomConstructorListEnd,
        HostCustomFieldListEnd, HostCustomIndex0, HostCustomSchema, HostCustomType, HostProfile,
        HostProvider, HostProviderModule, HostProviderSet, HostTupleType, HostTypeList,
        HostTypeListEnd, ModuleSource, PackageSource, StringValue,
    };
    use std::num::NonZeroUsize;
    use std::sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    };

    struct Profile;
    impl HostProfile for Profile {
        type RunState = Vec<StringValue>;
        type ExternalStores = ();
        type ExecutionState = ();
    }
    impl HostProvider<Profile> for Profile {
        type State = Vec<StringValue>;
        fn project(state: &mut Self::State) -> &mut Self::State {
            state
        }
    }
    struct MarkerSchema;
    struct Found;
    impl HostCustomSchema for MarkerSchema {
        const PACKAGE: &'static str = "application";
        const MODULE: &'static str = "example";
        const NAME: &'static str = "Marker";
        const PARAMETER_COUNT: usize = 0;
        type Constructors = HostCustomConstructorList<Found, HostCustomConstructorListEnd>;
    }
    impl HostCustomConstructorDefinition for Found {
        const NAME: &'static str = "Found";
        type Fields = HostCustomFieldListEnd;
    }
    type Marker = HostCustomType<MarkerSchema, HostTypeListEnd>;
    type MarkerFound = HostCustomConstructorAt<Marker, HostCustomIndex0, Found>;
    type Pair =
        HostTupleType<HostTypeList<StringValue, HostTypeList<StringValue, HostTypeListEnd>>>;
    fn mark<'call>(
        mut call: HostCall<'call, Profile, Profile, Marker>,
    ) -> Result<HostCallCompletion<'call, Marker>, HostCallError> {
        call.state().push("mark".into());
        if call
            .state()
            .first()
            .is_some_and(|value| value.as_str() == Ok("exit"))
        {
            return call.exit(crate::execution::ExitStatus::new(7));
        }
        if call
            .state()
            .first()
            .is_some_and(|value| value.as_str() == Ok("cancel"))
        {
            assert!(call.execution_unit().unwrap().cancel());
        }
        Ok(call.return_custom::<MarkerFound>(()))
    }
    fn pair<'call>(
        mut call: HostCall<'call, Profile, Profile, Pair>,
        value: StringValue,
    ) -> Result<HostCallCompletion<'call, Pair>, HostCallError> {
        call.state().push(value.clone());
        if call
            .state()
            .first()
            .is_some_and(|value| value.as_str() == Ok("exit"))
        {
            return call.exit(crate::execution::ExitStatus::new(7));
        }
        if call
            .state()
            .first()
            .is_some_and(|value| value.as_str() == Ok("cancel"))
        {
            assert!(call.execution_unit().unwrap().cancel());
        }
        Ok(call.return_tuple((value.clone(), (value, ()))))
    }

    fn mirror<'call>(
        mut call: HostCall<'call, Profile, Profile, StringValue>,
        value: StringValue,
    ) -> Result<HostCallCompletion<'call, StringValue>, HostCallError> {
        call.state().push(value.clone());
        if call
            .state()
            .first()
            .is_some_and(|value| value.as_str() == Ok("exit"))
        {
            return call.exit(crate::execution::ExitStatus::new(7));
        }
        if call
            .state()
            .first()
            .is_some_and(|value| value.as_str() == Ok("cancel"))
        {
            assert!(call.execution_unit().unwrap().cancel());
        }
        Ok(call.return_value(value))
    }

    #[derive(Clone, Copy, PartialEq, Eq)]
    enum NativeFamily {
        Custom,
        Tuple,
        String,
    }

    fn native_fixture(
        source: &str,
        provider: HostProviderModule<Profile>,
    ) -> crate::HostedExecution<Profile> {
        let typed = crate::compile_typed_host_program(
            "application",
            "example",
            [PackageSource::new(
                "application",
                Vec::<String>::new(),
                [ModuleSource::new("example", "src/example.gleam", source)],
            )],
            HostProviderSet::from_providers([provider]).unwrap(),
        )
        .unwrap();
        crate::HostedExecution::try_from_module_plan(crate::plan_host_program(typed).unwrap())
            .unwrap()
    }

    // The source in each test contains exactly these three calls. This fixture
    // reads their real targets/sites; it does not synthesize or change the graph.
    struct NativeCalls {
        main: TupleFunctionId,
        custom: (CustomFunctionId, HostCallSite),
        tuple: (TupleFunctionId, HostCallSite),
        string: (StringFunctionId, HostCallSite),
    }
    impl NativeCalls {
        fn inspect(plan: &HostedProgram<Profile>) -> Self {
            let graphs = (0..plan.synchronous_tuples().len())
                .map(TupleFunctionId)
                .filter_map(|id| match plan.tuple_function(id).as_ref() {
                    ExecutionFunctionRef::Graph(function) => Some((id, function)),
                    ExecutionFunctionRef::Host(_) => None,
                })
                .collect::<Vec<_>>();
            assert_eq!(graphs.len(), 1);
            let (main, function) = graphs[0];
            assert!(
                matches!(plan.main_runtime(), RuntimeFunctionId::Core(CoreRuntimeFunctionId::Tuple { id, .. }) if id == main)
            );
            let graph = function.body().block_graph().as_view();
            let mut customs = Vec::new();
            let mut tuples = Vec::new();
            let mut strings = Vec::new();
            for instruction in graph
                .blocks()
                .flat_map(|block| block.instructions())
                .filter_map(|instruction| instruction.value())
            {
                match instruction.kind() {
                    ProfiledInstructionKind::Custom(CustomInstruction::Call {
                        function,
                        site,
                        ..
                    }) => customs.push((*function, site.clone())),
                    ProfiledInstructionKind::Tuple(TupleInstruction::Call {
                        function,
                        site,
                        ..
                    }) => tuples.push((*function, site.clone())),
                    ProfiledInstructionKind::String(StringInstruction::Call {
                        function,
                        site,
                        ..
                    }) => strings.push((*function, site.clone())),
                    _ => {}
                }
            }
            assert_eq!((customs.len(), tuples.len(), strings.len()), (1, 1, 1));
            Self {
                main,
                custom: customs.remove(0),
                tuple: tuples.remove(0),
                string: strings.remove(0),
            }
        }

        fn request(
            &self,
            family: NativeFamily,
            input: StringValue,
            root_tail: bool,
            owner: DeliveryOwner,
        ) -> CallProgress {
            let execution = Box::new(PendingDelivery { owner, root_tail });
            match family {
                NativeFamily::Custom => CallProgress::CustomNative(CustomNativeRequest {
                    function: self.custom.0,
                    site: self.custom.1.clone(),
                    arguments: Box::default(),
                    root_tail,
                    execution,
                }),
                NativeFamily::Tuple => CallProgress::TupleNative(TupleNativeRequest {
                    function: self.tuple.0,
                    site: self.tuple.1.clone(),
                    arguments: Box::new(CallValues {
                        strings: vec![input],
                        ..Default::default()
                    }),
                    root_tail,
                    execution,
                }),
                NativeFamily::String => CallProgress::StringNative(StringNativeRequest {
                    function: self.string.0,
                    site: self.string.1.clone(),
                    arguments: Box::new(CallValues {
                        strings: vec![input],
                        ..Default::default()
                    }),
                    root_tail,
                    execution,
                }),
            }
        }
    }

    // This owner observes one already-published typed Native request. Public
    // preparation tests prove the actual generated caller and match selection.
    struct DeliveryOwner(Arc<AtomicUsize>);
    impl Drop for DeliveryOwner {
        fn drop(&mut self) {
            self.0.fetch_add(1, Ordering::SeqCst);
        }
    }
    struct PendingDelivery {
        owner: DeliveryOwner,
        root_tail: bool,
    }
    struct Delivered {
        value: Option<CallOutput>,
        _owner: DeliveryOwner,
        root_tail: bool,
    }
    struct PublishedRequest(CallProgress);
    impl CallExecution for PublishedRequest {
        fn restart(&mut self, _: CallTarget, _: usize, _: CallInputs<'_>) -> bool {
            false
        }
        fn retained_bytes(&self) -> usize {
            0
        }
        fn advance(self: Box<Self>, _: &mut CallOps<'_>, _: &mut usize) -> CallProgress {
            self.0
        }
    }
    impl CallExecution for PendingDelivery {
        fn restart(&mut self, _: CallTarget, _: usize, _: CallInputs<'_>) -> bool {
            false
        }
        fn retained_bytes(&self) -> usize {
            0
        }
        fn advance(self: Box<Self>, _: &mut CallOps<'_>, _: &mut usize) -> CallProgress {
            CallProgress::Yield(self)
        }
    }
    impl StringNativeExecution for PendingDelivery {
        fn resume_native(self: Box<Self>, value: StringValue) -> Box<dyn CallExecution> {
            let Self { owner, root_tail } = *self;
            Box::new(Delivered {
                value: Some(CallOutput::String(value)),
                _owner: owner,
                root_tail,
            })
        }
    }
    impl CustomNativeExecution for PendingDelivery {
        fn resume_native(self: Box<Self>, value: CallCustom) -> Box<dyn CallExecution> {
            let Self { owner, root_tail } = *self;
            Box::new(Delivered {
                value: Some(CallOutput::Custom(value)),
                _owner: owner,
                root_tail,
            })
        }
    }
    impl TupleNativeExecution for PendingDelivery {
        fn resume_native(self: Box<Self>, value: CallTuple) -> Box<dyn CallExecution> {
            let Self { owner, root_tail } = *self;
            Box::new(Delivered {
                value: Some(CallOutput::Tuple(value)),
                _owner: owner,
                root_tail,
            })
        }
    }
    impl CallExecution for Delivered {
        fn restart(&mut self, _: CallTarget, _: usize, _: CallInputs<'_>) -> bool {
            false
        }
        fn retained_bytes(&self) -> usize {
            0
        }
        fn advance(mut self: Box<Self>, _: &mut CallOps<'_>, budget: &mut usize) -> CallProgress {
            if !self.root_tail && *budget == 0 {
                return CallProgress::Yield(self);
            }
            if !self.root_tail {
                *budget -= 1;
            }
            match self.value.take() {
                Some(output) => CallProgress::Complete {
                    output,
                    execution: self,
                },
                None => CallProgress::Yield(self),
            }
        }
    }

    #[test]
    fn pending_and_published_delivery_owners_refuse_restart_and_release_once() {
        use crate::plan::execution::function::FunctionReturnFamily;
        use crate::plan::execution::function::IntFunctionId;
        use crate::runtime::CaptureStorage;
        use crate::runtime::compiled::numeric::NumericValues;
        use crate::runtime::graph::{BlockEnvironment, RetainedValues};
        use crate::runtime::state::list::RuntimeListStorage;
        let drops = Arc::new(AtomicUsize::new(0));
        let scenarios: [(Box<dyn CallExecution>, usize, bool); 5] = [
            (
                Box::new(PendingDelivery {
                    owner: DeliveryOwner(Arc::clone(&drops)),
                    root_tail: false,
                }),
                0,
                false,
            ),
            (
                Box::new(PendingDelivery {
                    owner: DeliveryOwner(Arc::clone(&drops)),
                    root_tail: false,
                }),
                1,
                false,
            ),
            (
                Box::new(Delivered {
                    value: Some(CallOutput::Nil(())),
                    _owner: DeliveryOwner(Arc::clone(&drops)),
                    root_tail: false,
                }),
                0,
                false,
            ),
            (
                Box::new(Delivered {
                    value: Some(CallOutput::Nil(())),
                    _owner: DeliveryOwner(Arc::clone(&drops)),
                    root_tail: false,
                }),
                1,
                true,
            ),
            (
                Box::new(Delivered {
                    value: Some(CallOutput::Nil(())),
                    _owner: DeliveryOwner(Arc::clone(&drops)),
                    root_tail: true,
                }),
                0,
                true,
            ),
        ];
        let captures = CaptureStorage::default();
        let mut numeric = NumericValues::default();
        let lists = RuntimeListStorage::default();
        let mut strings = None;
        let mut bits = None;
        let environment = BlockEnvironment::from_retained(RetainedValues::empty());
        let mut ops = CallOps::new(&captures, &mut numeric, &lists, &mut strings, &mut bits);
        for (index, (mut execution, mut grant, completed)) in scenarios.into_iter().enumerate() {
            assert!(!execution.restart(
                CallTarget::Int(IntFunctionId(0)),
                0,
                CallInputs::new(&environment)
            ));
            assert_eq!(execution.retained_bytes(), 0);
            let progress = execution.advance(&mut ops, &mut grant);
            assert_eq!(
                matches!(&progress, CallProgress::Complete { .. }),
                completed
            );
            assert_eq!(matches!(&progress, CallProgress::Yield(_)), !completed);
            assert_eq!(drops.load(Ordering::SeqCst), index);
            if let CallProgress::Complete { output, execution } = progress {
                assert_eq!(output.family(), FunctionReturnFamily::Nil);
                // Completion consumes the value once, while its pooled owner
                // remains inert until it is dropped or successfully restarted.
                let next = execution.advance(&mut ops, &mut grant);
                drop(next);
            } else {
                drop(progress);
            }
            assert_eq!(drops.load(Ordering::SeqCst), index + 1);
        }
    }

    #[test]
    fn compound_native_exit_keeps_the_effect_prefix_and_drops_the_pending_owner() {
        let source = r#"
pub type Marker { Found }
@external(erlang, "native", "mark")
fn mark() -> Marker
@external(erlang, "native", "pair")
fn pair(value: String) -> #(String, String)
@external(erlang, "native", "mirror")
fn mirror(value: String) -> String
pub fn main() { #(mark(), pair("input"), mirror("input")) }
"#;
        for family in [
            NativeFamily::Custom,
            NativeFamily::Tuple,
            NativeFamily::String,
        ] {
            let custom = family == NativeFamily::Custom;
            let mut hosted = native_fixture(
                source,
                HostProviderModule::new("application", "example")
                    .unwrap()
                    .with_scoped_function::<Profile, (), Marker, _>("mark", mark)
                    .unwrap()
                    .with_scoped_function::<Profile, (StringValue,), Pair, _>("pair", pair)
                    .unwrap()
                    .with_scoped_function::<Profile, (StringValue,), StringValue, _>(
                        "mirror", mirror,
                    )
                    .unwrap(),
            );
            let (plan, stores, captures) = hosted.parts_mut();
            let calls = NativeCalls::inspect(plan);
            let drops = Arc::new(AtomicUsize::new(0));
            let request = calls.request(
                family,
                "input".into(),
                false,
                DeliveryOwner(Arc::clone(&drops)),
            );
            let state = Box::new(GeneratedNativeState {
                phase: GeneratedNativePhase::Progress(request),
                numeric: Default::default(),
                strings: None,
                bit_arrays: None,
                root_tail_entry: true,
                domain: captures.domain(),
                prepaid_completion: false,
            });
            let host = TestHost::default();
            let mut effects = vec![StringValue::from("exit")];
            let mut echo = Vec::new();
            let domain = Domain::new(
                Arc::clone(plan),
                &host,
                &mut effects,
                stores,
                &mut echo,
                captures.clone(),
                NonZeroUsize::new(8).unwrap(),
            );
            let context = domain.context();
            let result = host
                .block_on(
                    domain.drive(
                        plan.prepare_generated_native(state, 8)
                            .ok()
                            .unwrap()
                            .submit(context.execution.services(), NonZeroUsize::new(8).unwrap()),
                    ),
                )
                .unwrap();
            assert_eq!(
                result.try_into_value().err(),
                Some(crate::execution::ExitStatus::new(7))
            );
            assert_eq!(
                effects,
                ["exit", if custom { "mark" } else { "input" }].map(StringValue::from)
            );
            assert_eq!(drops.load(Ordering::SeqCst), 1);
            assert!(echo.is_empty());
        }
    }

    #[test]
    fn a_resumed_compound_request_declines_continuing_bindings_before_any_native_effect() {
        use crate::HostCallContinuation;
        use crate::HostConstructions;
        use crate::HostFailure;
        use crate::HostOwnedCompletion;
        use crate::runtime::HostCallOrigin;
        use crate::runtime::graph::{BlockEnvironment, RetainedValues};
        fn later_mark<'call>(
            mut call: HostCall<'call, Profile, Profile, Marker>,
            _: HostConstructions<'call, HostTypeListEnd>,
        ) -> Result<HostCallContinuation<'call, Marker>, HostCallError> {
            call.state().push("mark".into());
            Err(HostFailure::new("canonical marker refusal").into())
        }
        fn later_pair<'call>(
            mut call: HostCall<'call, Profile, Profile, Pair>,
            _: HostConstructions<'call, HostTypeListEnd>,
            value: StringValue,
        ) -> Result<HostCallContinuation<'call, Pair>, HostCallError> {
            call.state().push(value);
            Err(HostFailure::new("canonical tuple refusal").into())
        }
        fn later_mirror<'call>(
            mut call: HostCall<'call, Profile, Profile, StringValue>,
            constructions: HostConstructions<'call, HostTypeListEnd>,
            value: StringValue,
        ) -> Result<HostCallContinuation<'call, StringValue>, HostCallError> {
            call.state().push(value.clone());
            Ok(call.resume(constructions, move |_| {
                Box::pin(async move {
                    Ok(HostOwnedCompletion::new(move |call, _| {
                        Ok(call.return_value(value))
                    }))
                })
            }))
        }
        let source = r#"
pub type Marker { Found }
@external(erlang, "native", "mark")
fn mark() -> Marker
@external(erlang, "native", "pair")
fn pair(value: String) -> #(String, String)
@external(erlang, "native", "mirror")
fn mirror(value: String) -> String
pub fn main() { #(mark(), pair("input"), mirror("input")) }
"#;
        for (family, native) in [
            (NativeFamily::Custom, true),
            (NativeFamily::Tuple, true),
            (NativeFamily::String, true),
            (NativeFamily::String, false),
        ] {
            let custom = family == NativeFamily::Custom;
            let input = "input";
            let mut hosted = native_fixture(source, HostProviderModule::new(
                    "application",
                    "example",
                )
                .unwrap()
                .with_resumable_function::<Profile, (), Marker, HostTypeListEnd, _>(
                    "mark", later_mark,
                )
                .unwrap()
                .with_resumable_function::<Profile, (StringValue,), Pair, HostTypeListEnd, _>(
                    "pair", later_pair,
                )
                .unwrap()
                .with_resumable_function::<Profile, (StringValue,), StringValue, HostTypeListEnd, _>(
                    "mirror", later_mirror,
                )
                .unwrap());
            let (plan, stores, captures) = hosted.parts_mut();
            let calls = NativeCalls::inspect(plan);
            let main_id = calls.main;
            let (custom_function, custom_site) = calls.custom.clone();
            let (tuple_function, tuple_site) = calls.tuple.clone();
            let (string_function, string_site) = calls.string.clone();
            assert!(!plan.synchronous_customs()[custom_function.index]);
            assert!(!plan.synchronous_tuples()[tuple_function.0]);
            assert!(!plan.synchronous_strings()[string_function.0]);
            let drops = Arc::new(AtomicUsize::new(0));
            let owner = DeliveryOwner(Arc::clone(&drops));
            let request = if native {
                calls.request(family, input.into(), false, owner)
            } else {
                let execution = Box::new(PendingDelivery {
                    owner,
                    root_tail: false,
                });
                CallProgress::String {
                    function: string_function,
                    site: string_site.clone(),
                    arguments: CallArguments {
                        values: Box::new(CallValues {
                            strings: vec![input.into()],
                            ..Default::default()
                        }),
                        captures: None,
                    },
                    resume: Box::new(move |value| {
                        StringNativeExecution::resume_native(execution, value)
                    }),
                }
            };
            let mut published = PublishedRequest(request);
            let environment = BlockEnvironment::from_retained(RetainedValues::empty());
            assert!(!published.restart(
                CallTarget::Tuple(main_id),
                0,
                CallInputs::new(&environment)
            ));
            assert_eq!(published.retained_bytes(), 0);
            let state = Box::new(GeneratedNativeState {
                phase: GeneratedNativePhase::Progress(CallProgress::Yield(Box::new(published))),
                numeric: Default::default(),
                strings: None,
                bit_arrays: None,
                root_tail_entry: true,
                domain: captures.domain(),
                prepaid_completion: false,
            });
            let host = TestHost::default();
            let mut effects = Vec::new();
            let mut echo = Vec::new();
            let domain = Domain::new(
                Arc::clone(plan),
                &host,
                &mut effects,
                stores,
                &mut echo,
                captures.clone(),
                NonZeroUsize::new(8).unwrap(),
            );
            let context = domain.context();
            host.block_on(domain.drive(async {
                let state = plan.prepare_generated_native(state, 8).ok().unwrap().submit(context.execution.services(), NonZeroUsize::new(8).unwrap()).await.unwrap().unwrap();
                assert_eq!(drops.load(Ordering::SeqCst), 0);
                assert!(context.execution.with_state(|effects| effects.is_empty()).await.unwrap());
                if custom {
                    assert!(matches!(&state.phase, GeneratedNativePhase::Progress(CallProgress::CustomNative(request)) if request.function == custom_function && request.site == custom_site));
                } else if family == NativeFamily::Tuple {
                    assert!(matches!(&state.phase, GeneratedNativePhase::Progress(CallProgress::TupleNative(request)) if request.function == tuple_function && request.site == tuple_site));
                } else if native {
                    assert!(matches!(&state.phase, GeneratedNativePhase::Progress(CallProgress::StringNative(request)) if request.function == string_function && request.site == string_site));
                } else {
                    assert!(matches!(&state.phase, GeneratedNativePhase::Progress(CallProgress::String { function, site, arguments, .. }) if *function == string_function && *site == string_site && arguments.values.strings == [StringValue::from(input)] && arguments.captures.is_none()));
                }
                let declined = if native {
                    plan.prepare_generated_native(state, 8).err().unwrap()
                } else {
                    state
                };
                let resume = match declined.phase {
                    GeneratedNativePhase::Progress(CallProgress::String { arguments, resume, .. }) => Some((arguments, resume)),
                    phase => { drop(phase); None }
                };
                assert_eq!(resume.is_some(), !native);
                // The original canonical caller owns this invocation. Native
                // refusal has no effects; this call executes the body once.
                match family {
                    NativeFamily::Custom => assert!(context.call(custom_function, HostCallOrigin::source(custom_site), RetainedValues::empty()).await.unwrap().is_err()),
                    NativeFamily::Tuple => {
                        let mut inputs = RetainedValues::empty();
                        inputs.push_string(input.into());
                        assert!(context.call(tuple_function, HostCallOrigin::source(tuple_site), inputs).await.unwrap().is_err());
                    }
                    NativeFamily::String => {
                        if let Some((arguments, resume)) = resume {
                            let value = context.call(string_function, HostCallOrigin::source(string_site), arguments.values.into_retained()).await.unwrap().unwrap();
                            assert_eq!(value, StringValue::from(input));
                            assert_eq!(drops.load(Ordering::SeqCst), 0);
                            drop(resume(value));
                        } else {
                            let mut inputs = RetainedValues::empty();
                            inputs.push_string(input.into());
                            assert_eq!(context.call(string_function, HostCallOrigin::source(string_site), inputs).await.unwrap().unwrap(), StringValue::from(input));
                        }
                    }
                }
                assert_eq!(drops.load(Ordering::SeqCst), 1);
            })).unwrap().try_into_value().unwrap();
            assert_eq!(
                effects,
                [StringValue::from(if custom { "mark" } else { input })]
            );
            assert!(echo.is_empty());
        }
    }

    #[test]
    fn compound_native_grants_keep_typed_delivery_owned_until_publication_or_drop() {
        let source = r#"
pub type Marker { Found }
@external(erlang, "native", "mark")
fn mark() -> Marker
@external(erlang, "native", "pair")
fn pair(value: String) -> #(String, String)
@external(erlang, "native", "mirror")
fn mirror(value: String) -> String
pub fn main() { #(mark(), pair("input"), mirror("input")) }
"#;
        for family in [
            NativeFamily::Custom,
            NativeFamily::Tuple,
            NativeFamily::String,
        ] {
            let custom = family == NativeFamily::Custom;
            for root_tail in [false, true] {
                for grant in [0, 1, 2, 7, 1024] {
                    for drop_after_body in [false, true] {
                        let mut hosted = native_fixture(
                            source,
                            HostProviderModule::new("application", "example")
                                .unwrap()
                                .with_scoped_function::<Profile, (), Marker, _>("mark", mark)
                                .unwrap()
                                .with_scoped_function::<Profile, (StringValue,), Pair, _>(
                                    "pair", pair,
                                )
                                .unwrap()
                                .with_scoped_function::<Profile, (StringValue,), StringValue, _>(
                                    "mirror", mirror,
                                )
                                .unwrap(),
                        );
                        let (plan, stores, captures) = hosted.parts_mut();
                        let calls = NativeCalls::inspect(plan);
                        let drops = Arc::new(AtomicUsize::new(0));
                        let request = calls.request(
                            family,
                            "input".into(),
                            root_tail,
                            DeliveryOwner(Arc::clone(&drops)),
                        );
                        let mut state = Box::new(GeneratedNativeState {
                            phase: GeneratedNativePhase::Progress(request),
                            numeric: Default::default(),
                            strings: None,
                            bit_arrays: None,
                            root_tail_entry: true,
                            domain: captures.domain(),
                            prepaid_completion: false,
                        });
                        let pointer = std::ptr::from_ref(&*state);
                        let host = TestHost::default();
                        let mut effects = Vec::new();
                        let mut echo = Vec::new();
                        let domain = Domain::new(
                            Arc::clone(plan),
                            &host,
                            &mut effects,
                            stores,
                            &mut echo,
                            captures.clone(),
                            NonZeroUsize::new(1024).unwrap(),
                        );
                        let context = domain.context();
                        host.block_on(domain.drive(async {
                            let mut turns = 0;
                            loop {
                                let allowance = if turns == 0 {
                                    grant
                                } else if drop_after_body {
                                    1
                                } else {
                                    grant.max(1)
                                };
                                state = plan
                                    .prepare_generated_native(state, allowance)
                                    .ok()
                                    .unwrap()
                                    .submit(
                                        context.execution.services(),
                                        NonZeroUsize::new(allowance.max(1)).unwrap(),
                                    )
                                    .await
                                    .unwrap()
                                    .unwrap();
                                assert_eq!(std::ptr::from_ref(&*state), pointer);
                                assert_eq!(drops.load(Ordering::SeqCst), 0);
                                let count = context
                                    .execution
                                    .with_state(|values| values.len())
                                    .await
                                    .unwrap();
                                assert!(count <= 1, "a bounded grant cannot replay the body");
                                if turns == 0 && grant == 0 {
                                    assert_eq!(count, 0);
                                }
                                let completed = matches!(
                                    &state.phase,
                                    GeneratedNativePhase::Progress(CallProgress::Complete { .. })
                                ) && state.prepaid_completion;
                                if completed || (drop_after_body && count == 1) {
                                    break;
                                }
                                turns += 1;
                                assert!(turns < 10);
                            }
                            let body_charge: usize = if root_tail { 3 } else { 2 };
                            let expected_turns = if drop_after_body {
                                body_charge.saturating_sub(grant.max(1))
                            } else {
                                let publication_charge: usize = if root_tail { 3 } else { 5 };
                                publication_charge.div_ceil(grant.max(1)) - 1
                            };
                            assert_eq!(turns, expected_turns);
                            if drop_after_body && !root_tail && grant < 3 {
                                assert!(matches!(
                                    &state.phase,
                                    GeneratedNativePhase::Deliver { charge: true, .. }
                                        | GeneratedNativePhase::CustomDeliver { charge: true, .. }
                                        | GeneratedNativePhase::TupleDeliver { charge: true, .. }
                                ));
                            }
                            assert_eq!(
                                context
                                    .execution
                                    .with_state(|values| values.clone())
                                    .await
                                    .unwrap(),
                                vec![StringValue::from(if custom { "mark" } else { "input" })]
                            );
                            if !drop_after_body {
                                if custom {
                                    assert!(matches!(&state.phase, GeneratedNativePhase::Progress(CallProgress::Complete { output: CallOutput::Custom(value), .. }) if value.0.fields().is_empty() && value.0.constructor().index == 0));
                                } else if family == NativeFamily::Tuple {
                                    assert!(matches!(&state.phase, GeneratedNativePhase::Progress(CallProgress::Complete { output: CallOutput::Tuple(value), .. }) if value.0 == vec![EvaluatedValue::String("input".into()), EvaluatedValue::String("input".into())]));
                                } else {
                                    assert!(matches!(&state.phase, GeneratedNativePhase::Progress(CallProgress::Complete { output: CallOutput::String(value), .. }) if value.as_str() == Ok("input")));
                                }
                            }
                            drop(state);
                            assert_eq!(drops.load(Ordering::SeqCst), 1);
                        }))
                        .unwrap()
                        .try_into_value()
                        .unwrap();
                        assert!(echo.is_empty());
                    }
                }
            }
        }
    }

    #[test]
    fn cancellation_inside_native_preserves_the_effect_and_drops_undelivered_output() {
        use crate::execution::UnitOwner;

        let source = r#"
pub type Marker { Found }
@external(erlang, "native", "mark")
fn mark() -> Marker
@external(erlang, "native", "pair")
fn pair(value: String) -> #(String, String)
@external(erlang, "native", "mirror")
fn mirror(value: String) -> String
pub fn main() { #(mark(), pair("input"), mirror("input")) }
"#;
        for family in [
            NativeFamily::Custom,
            NativeFamily::Tuple,
            NativeFamily::String,
        ] {
            let mut hosted = native_fixture(
                source,
                HostProviderModule::new("application", "example")
                    .unwrap()
                    .with_scoped_function::<Profile, (), Marker, _>("mark", mark)
                    .unwrap()
                    .with_scoped_function::<Profile, (StringValue,), Pair, _>("pair", pair)
                    .unwrap()
                    .with_scoped_function::<Profile, (StringValue,), StringValue, _>(
                        "mirror", mirror,
                    )
                    .unwrap(),
            );
            let (plan, stores, captures) = hosted.parts_mut();
            let calls = NativeCalls::inspect(plan);
            let drops = Arc::new(AtomicUsize::new(0));
            let request = calls.request(
                family,
                "input".into(),
                false,
                DeliveryOwner(Arc::clone(&drops)),
            );
            let state = Box::new(GeneratedNativeState {
                phase: GeneratedNativePhase::Progress(request),
                numeric: Default::default(),
                strings: None,
                bit_arrays: None,
                root_tail_entry: true,
                domain: captures.domain(),
                prepaid_completion: false,
            });
            let host = TestHost::default();
            let mut effects = vec![StringValue::from("cancel")];
            let mut echo = Vec::new();
            let mut domain = Domain::new(
                Arc::clone(plan),
                &host,
                &mut effects,
                stores,
                &mut echo,
                captures.clone(),
                NonZeroUsize::new(8).unwrap(),
            );
            let (context, root) = domain.begin(UnitOwner::new(domain.units.completion()));
            host.block_on(domain.drive(async {
                let result = plan
                    .prepare_generated_native(state, 8)
                    .ok()
                    .unwrap()
                    .submit(context.services(), NonZeroUsize::new(8).unwrap())
                    .await;
                assert!(result.is_err());
                assert!(!context.unit().unwrap().is_active());
                assert_eq!(drops.load(Ordering::SeqCst), 1);
                drop(root);
            }))
            .unwrap()
            .try_into_value()
            .unwrap();
            assert_eq!(
                effects,
                [
                    StringValue::from("cancel"),
                    StringValue::from(if family == NativeFamily::Custom {
                        "mark"
                    } else {
                        "input"
                    })
                ]
            );
            assert!(echo.is_empty());
        }
    }

    #[test]
    fn compound_native_cancellation_releases_invoke_delivery_and_publication_owners() {
        use crate::execution::UnitOwner;

        let source = r#"
pub type Marker { Found }
@external(erlang, "native", "mark")
fn mark() -> Marker
@external(erlang, "native", "pair")
fn pair(value: String) -> #(String, String)
@external(erlang, "native", "mirror")
fn mirror(value: String) -> String
pub fn main() { #(mark(), pair("input"), mirror("input")) }
"#;
        for family in [
            NativeFamily::Custom,
            NativeFamily::Tuple,
            NativeFamily::String,
        ] {
            let custom = family == NativeFamily::Custom;
            for charged in 0..=4 {
                let mut hosted = native_fixture(
                    source,
                    HostProviderModule::new("application", "example")
                        .unwrap()
                        .with_scoped_function::<Profile, (), Marker, _>("mark", mark)
                        .unwrap()
                        .with_scoped_function::<Profile, (StringValue,), Pair, _>("pair", pair)
                        .unwrap()
                        .with_scoped_function::<Profile, (StringValue,), StringValue, _>(
                            "mirror", mirror,
                        )
                        .unwrap(),
                );
                let (plan, stores, captures) = hosted.parts_mut();
                let calls = NativeCalls::inspect(plan);
                let drops = Arc::new(AtomicUsize::new(0));
                let request = calls.request(
                    family,
                    "input".into(),
                    false,
                    DeliveryOwner(Arc::clone(&drops)),
                );
                let mut state = Box::new(GeneratedNativeState {
                    phase: GeneratedNativePhase::Progress(request),
                    numeric: Default::default(),
                    strings: None,
                    bit_arrays: None,
                    root_tail_entry: true,
                    domain: captures.domain(),
                    prepaid_completion: false,
                });
                let host = TestHost::default();
                let mut effects = Vec::new();
                let mut echo = Vec::new();
                let mut domain = Domain::new(
                    Arc::clone(plan),
                    &host,
                    &mut effects,
                    stores,
                    &mut echo,
                    captures.clone(),
                    NonZeroUsize::new(1024).unwrap(),
                );
                let (context, root) = domain.begin(UnitOwner::new(domain.units.completion()));
                host.block_on(domain.drive(async {
                    if charged > 0 {
                        state = plan
                            .prepare_generated_native(state, charged)
                            .ok()
                            .unwrap()
                            .submit(context.services(), NonZeroUsize::new(charged).unwrap())
                            .await
                            .unwrap()
                            .unwrap();
                    }
                    assert!(state.domain == captures.domain());
                    assert_eq!(drops.load(Ordering::SeqCst), 0);
                    assert_eq!(
                        matches!(
                            &state.phase,
                            GeneratedNativePhase::Progress(
                                CallProgress::CustomNative(_)
                                    | CallProgress::TupleNative(_)
                                    | CallProgress::StringNative(_)
                            )
                        ),
                        charged == 0,
                    );
                    assert_eq!(
                        matches!(
                            &state.phase,
                            GeneratedNativePhase::CustomInvoke { before: 0, .. }
                                | GeneratedNativePhase::TupleInvoke { before: 0, .. }
                                | GeneratedNativePhase::Invoke { before: 0, .. }
                        ),
                        charged == 1,
                    );
                    assert_eq!(
                        matches!(
                            &state.phase,
                            GeneratedNativePhase::CustomDeliver { charge: true, .. }
                                | GeneratedNativePhase::TupleDeliver { charge: true, .. }
                                | GeneratedNativePhase::Deliver { charge: true, .. }
                        ),
                        charged == 2,
                    );
                    assert_eq!(
                        matches!(
                            &state.phase,
                            GeneratedNativePhase::Progress(CallProgress::Yield(_))
                        ),
                        charged == 3,
                    );
                    assert_eq!(
                        matches!(
                            &state.phase,
                            GeneratedNativePhase::Progress(CallProgress::Complete { .. })
                        ),
                        charged == 4,
                    );
                    assert!(!state.prepaid_completion);
                    // A separate request runs after Native's mutable state borrow
                    // and before cancellation of the still-owned typed result.
                    assert_eq!(
                        context.with_state(|values| values.clone()).await.unwrap(),
                        if charged < 2 {
                            Vec::new()
                        } else {
                            vec![StringValue::from(if custom { "mark" } else { "input" })]
                        }
                    );
                    assert!(context.unit().unwrap().cancel());
                    let result = plan
                        .prepare_generated_native(state, 1)
                        .ok()
                        .unwrap()
                        .submit(context.services(), NonZeroUsize::MIN)
                        .await;
                    drop(root);
                    assert!(result.is_err());
                    assert_eq!(drops.load(Ordering::SeqCst), 1);
                }))
                .unwrap()
                .try_into_value()
                .unwrap();
                assert_eq!(effects.len(), usize::from(charged >= 2));
                assert!(echo.is_empty());
            }
        }
    }
}
