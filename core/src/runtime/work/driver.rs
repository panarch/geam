#[cfg(test)]
use super::execution::WorkContext;
use super::execution::{Completion, ExecutionWork, Request, SourceWork};
use super::{Cancelled, Observer, Observers, Shared};
use crate::host::HostProfile;
use crate::plan::execution::HostedProgram;
use crate::runtime::state::{RuntimeHost, RuntimeState, RuntimeStateFor};
use crate::runtime::{EchoSink, RuntimeListStorage};
use parking_lot::Mutex;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::sync::atomic::Ordering;
use std::task::{Context, Poll, Waker};

pub(crate) struct Driver<'host, Profile: HostProfile> {
    work: ExecutionWork<Profile>,
    runtime: Mutex<AttachedRuntime<'host, Profile>>,
    wake: Arc<Observers>,
}

struct AttachedRuntime<'host, Profile: HostProfile> {
    plan: &'host HostedProgram<Profile>,
    host: &'host mut Profile::RunState,
    stores: &'host mut Profile::ExternalStores,
    echo: &'host mut (dyn EchoSink + Send),
    lists: RuntimeListStorage,
}

pub(crate) struct DrivenObservation<'driver, 'host, Profile: HostProfile> {
    driver: &'driver Driver<'host, Profile>,
    observer: Observer<Completion>,
    ticket: u64,
}

impl<'host, Profile: HostProfile> Driver<'host, Profile> {
    pub(crate) fn new(
        plan: &'host HostedProgram<Profile>,
        host: &'host mut Profile::RunState,
        stores: &'host mut Profile::ExternalStores,
        echo: &'host mut (dyn EchoSink + Send),
    ) -> Self {
        Self {
            work: ExecutionWork::new(),
            runtime: Mutex::new(AttachedRuntime {
                plan,
                host,
                stores,
                echo,
                lists: RuntimeListStorage::default(),
            }),
            wake: Arc::new(Observers::default()),
        }
    }

    #[cfg(test)]
    pub(in crate::runtime) fn context(&self) -> WorkContext<Profile> {
        self.work.context()
    }

    pub(in crate::runtime) fn call<Output>(
        &mut self,
        call: impl FnOnce(
            &HostedProgram<Profile>,
            &mut RuntimeStateFor<'_, HostedProgram<Profile>>,
        ) -> Output,
    ) -> Output {
        self.runtime.get_mut().call(&self.work, call)
    }

    pub(crate) fn observe(&self, work: &SourceWork) -> DrivenObservation<'_, 'host, Profile> {
        DrivenObservation {
            driver: self,
            observer: work.observe(),
            ticket: self.wake.next.fetch_add(1, Ordering::Relaxed),
        }
    }

    fn service_requests(&self) -> bool {
        let mut serviced = false;
        let waker = Waker::from(Arc::clone(&self.wake));
        let mut cx = Context::from_waker(&waker);
        loop {
            let Some(runtime) = self.runtime.try_lock() else {
                return serviced;
            };
            let Some(request) = self.work.next(&mut cx) else {
                return serviced;
            };
            self.dispatch(request, runtime);
            serviced = true;
        }
    }

    fn dispatch(
        &self,
        request: Request<Profile>,
        mut runtime: parking_lot::MutexGuard<'_, AttachedRuntime<'host, Profile>>,
    ) {
        let delivery = runtime.call(&self.work, |plan, state| request.service(plan, state));
        drop(runtime);
        if let Some(delivery) = delivery {
            delivery.deliver();
        }
        self.wake.notify();
    }
}

impl<Profile: HostProfile> AttachedRuntime<'_, Profile> {
    fn call<Output>(
        &mut self,
        work: &ExecutionWork<Profile>,
        call: impl FnOnce(
            &HostedProgram<Profile>,
            &mut RuntimeStateFor<'_, HostedProgram<Profile>>,
        ) -> Output,
    ) -> Output {
        let mut state = RuntimeState::with_host_and_lists(
            &mut *self.echo,
            RuntimeHost::<Profile>::new(&mut *self.host, &*self.stores, work),
            self.lists.clone(),
        );
        call(self.plan, &mut state)
    }
}

impl<Profile: HostProfile> Future for DrivenObservation<'_, '_, Profile> {
    type Output = Result<Shared<Completion>, Cancelled>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.driver.wake.register(self.ticket, cx.waker().clone());
        let driver = self.driver;
        let result = self.observer.poll_with(cx, || driver.service_requests());
        if result.is_ready() {
            self.driver.wake.remove(self.ticket);
        }
        result
    }
}

impl<Profile: HostProfile> Drop for DrivenObservation<'_, '_, Profile> {
    fn drop(&mut self) {
        self.driver.wake.remove(self.ticket);
    }
}

#[cfg(test)]
mod tests {
    use super::Driver;
    use crate::frontend::compile_typed_host_program;
    use crate::host::{HostProfile, HostProviderSet};
    use crate::plan::execution::HostedProgram;
    use crate::plan::{LibraryEntry, LibraryValueType, ValueType};
    use crate::runtime::evaluated::{EvaluatedFunctionValueKind, EvaluatedValue};
    use crate::runtime::function::{InvocableFunctionValue, run_tuple};
    use crate::runtime::work::Cancelled;
    use crate::runtime::{CallbackInputs, HostCallOrigin, RetainedCallable, RetainedInputs};
    use crate::{ModuleSource, PackageSource};
    use std::cell::Cell;
    use std::future::Future;
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
    }

    #[derive(Default)]
    struct WakeCount(AtomicUsize);

    impl Wake for WakeCount {
        fn wake(self: Arc<Self>) {
            self.0.fetch_add(1, Ordering::SeqCst);
        }
    }

    #[test]
    fn claimed_requests_skip_abandoned_receivers_and_deliver_after_unlocking() {
        struct CheckWake {
            entered: Arc<std::sync::Barrier>,
            release: Arc<std::sync::Barrier>,
        }
        impl Wake for CheckWake {
            fn wake(self: Arc<Self>) {
                self.entered.wait();
                self.release.wait();
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
        let (plan, _) = HostedProgram::from_library_plan(
            library,
            LibraryEntry::new(id, LibraryValueType::Nil, Vec::new(), Vec::new()),
            Vec::new(),
        )
        .expect("sealed library");
        let mut state = Cell::new(0);
        let mut stores = Cell::new(());
        let mut echo = Echo::default();
        let driver = Driver::new(&plan, &mut state, &mut stores, &mut echo);
        for cancelled in [false, true] {
            let entered = Arc::new(std::sync::Barrier::new(2));
            let release = Arc::new(std::sync::Barrier::new(2));
            let waker = Waker::from(Arc::new(CheckWake {
                entered: Arc::clone(&entered),
                release: Arc::clone(&release),
            }));
            let context = driver.context();
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
                std::thread::scope(|threads| {
                    let checking = threads.spawn(|| {
                        entered.wait();
                        let unlocked = driver.runtime.try_lock().is_some();
                        release.wait();
                        assert!(
                            unlocked,
                            "a completion wake must not run under the host state lock"
                        );
                    });
                    driver.dispatch(request, driver.runtime.lock());
                    checking.join().expect("wake ownership check");
                });
                assert_eq!(receiver.as_mut().poll(&mut cx), Poll::Ready(Ok(1)));
            } else {
                driver.dispatch(request, driver.runtime.lock());
            }
        }
        drop(driver);
        assert_eq!(state.get(), 1);
        assert!(echo.output.is_empty());
    }

    #[test]
    fn competing_observers_defer_busy_state_access_and_receive_a_completion_wake() {
        use crate::runtime::StoredRuntimeValue;
        use crate::runtime::shared::Shared;
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
        let (plan, _) =
            HostedProgram::from_library_plan(plan, entry, Vec::new()).expect("sealed execution");
        let mut host = Cell::new(0);
        let mut stores = Cell::new(());
        let mut echo = Echo::default();
        let driver = Driver::new(&plan, &mut host, &mut stores, &mut echo);
        let entered = Arc::new(Barrier::new(2));
        let release = Arc::new(Barrier::new(2));
        let works = [true, false, false].map(|pause| {
            let context = driver.context();
            let state_context = context.clone();
            let entered = Arc::clone(&entered);
            let release = Arc::clone(&release);
            context.compose(|_| async move {
                let value = state_context
                    .with_state(move |state| {
                        if pause {
                            entered.wait();
                            release.wait();
                        }
                        state.set(state.get() + 1);
                        state.get()
                    })
                    .await?;
                Ok(Shared::new(Ok(Shared::new(StoredRuntimeValue::new(
                    EvaluatedValue::Int(value.into()),
                    ValueType::Int,
                )))))
            })
        });
        let wake = Arc::new(WakeCount::default());
        let waker = Waker::from(Arc::clone(&wake));
        let mut first = Box::pin(driver.observe(&works[0]));
        let mut second = Box::pin(driver.observe(&works[1]));
        let completed = std::thread::scope(|threads| {
            let worker =
                threads.spawn(|| first.as_mut().poll(&mut Context::from_waker(Waker::noop())));
            entered.wait();
            let deferred = second.as_mut().poll(&mut Context::from_waker(&waker));
            release.wait();
            let completed = worker.join().expect("state worker");
            assert!(deferred.is_pending());
            completed
        });
        assert!(completed.is_ready());
        assert!(wake.0.load(Ordering::SeqCst) > 0);
        let second_completion = second.as_mut().poll(&mut Context::from_waker(&waker));
        assert!(second_completion.is_ready());
        drop(first);
        drop(second);
        let mut abandoned = std::pin::pin!(works[2].observe());
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
        let (plan, entries) =
            HostedProgram::from_library_plan(plan, entry, Vec::new()).expect("sealed execution");
        let entry = *entries.tuples[0].function();
        let mut host = Cell::new(2);
        let mut stores = Cell::new(());
        let mut echo = Echo::default();
        let (ready, wait) = futures_channel::oneshot::channel::<()>();
        let wake = Arc::new(WakeCount::default());
        let waker = Waker::from(wake.clone());

        let mut driver = Driver::<Profile>::new(&plan, &mut host, &mut stores, &mut echo);
        let callback = driver.call(|plan, runtime| {
            let values = run_tuple(
                plan,
                runtime,
                entry,
                HostCallOrigin::Entry,
                RetainedInputs::empty().into_retained(),
            )
            .expect("source closure");
            int_callback(&values)
        });
        let context = driver.context();
        let work = context.compose(|_| resume_callback_after_gate(context.clone(), wait, callback));
        let mut observation = Box::pin(driver.observe(&work));
        std::thread::scope(|threads| {
            let first_waker = waker.clone();
            observation = threads
                .spawn(move || {
                    assert!(
                        observation
                            .as_mut()
                            .poll(&mut Context::from_waker(&first_waker))
                            .is_pending()
                    );
                    observation
                })
                .join()
                .expect("first worker");
            ready.send(()).expect("active waiter");
            assert!(wake.0.load(Ordering::SeqCst) > 0);
            let completion = threads
                .spawn(move || {
                    completed_observation(
                        observation.as_mut().poll(&mut Context::from_waker(&waker)),
                    )
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
        drop(driver);
        assert_eq!(host.get(), 3);
        assert_eq!(echo.output, ["src/library.gleam:4\n2"]);
        assert_eq!(echo.exclusive.get(), 1);

        for serviced in 0..3 {
            let mut state = Cell::new(2);
            let mut stores = Cell::new(());
            let mut echo = Echo::default();
            let mut driver = Driver::new(&plan, &mut state, &mut stores, &mut echo);
            let callback = driver.call(|plan, runtime| {
                let values = run_tuple(
                    plan,
                    runtime,
                    entry,
                    HostCallOrigin::Entry,
                    RetainedInputs::empty().into_retained(),
                )
                .expect("source callback");
                int_callback(&values)
            });
            let (ready, wait) = futures_channel::oneshot::channel();
            let mut resumed =
                Box::pin(resume_callback_after_gate(driver.context(), wait, callback));
            let mut cx = Context::from_waker(Waker::noop());
            if serviced == 0 {
                drop(ready);
            } else {
                ready.send(()).expect("live native gate");
                assert!(resumed.as_mut().poll(&mut cx).is_pending());
                if serviced == 2 {
                    assert!(driver.service_requests());
                    assert!(resumed.as_mut().poll(&mut cx).is_pending());
                }
            }
            drop(driver);
            assert_eq!(
                resumed.as_mut().poll(&mut cx).map(Result::err),
                Poll::Ready(Some(Cancelled))
            );
            assert_eq!(state.get(), if serviced == 2 { 3 } else { 2 });
            assert!(echo.output.is_empty());
        }
    }

    async fn resume_callback_after_gate(
        context: super::WorkContext<Profile>,
        wait: futures_channel::oneshot::Receiver<()>,
        callback: crate::runtime::RetainedCallable,
    ) -> Result<super::Shared<super::Completion>, Cancelled> {
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
        Ok(super::Shared::new(
            result.map(super::Shared::new).map_err(super::Shared::new),
        ))
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
        generation: usize,
        fail_touch: bool,
    }

    impl HostProfile for NativeProfile {
        type RunState = NativeState;
        type ExternalStores = crate::host::HostFutureStore;
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
        Ok(call.return_future(constructions, move |_| {
            Box::pin(async move {
                let increment = std::future::poll_fn(move |cx| {
                    polls.fetch_add(1, Ordering::SeqCst);
                    std::pin::Pin::new(&mut gate).poll(cx)
                })
                .await
                .map_err(|_| crate::host::HostFutureError::Cancelled)??;
                Ok(crate::host::HostFutureCompletion::new(move |call, _| {
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
        let retained_callback = call.retain_value::<crate::HostFunctionType<
            crate::HostTypeList<num_bigint::BigInt, crate::HostTypeListEnd>,
            num_bigint::BigInt,
        >>(callback);
        let callback = call.future_callable(callback, &constructions);
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
                        &context,
                        move |_, _| (before.into(), ()),
                        |_, value| Ok(value),
                    )
                    .await?;
                Ok(crate::host::HostFutureCompletion::new(
                    move |mut call, _| {
                        let callback = call.restore_value::<crate::HostFunctionType<
                            crate::HostTypeList<num_bigint::BigInt, crate::HostTypeListEnd>,
                            num_bigint::BigInt,
                        >>(&retained_callback);
                        let second = call.invoke(callback, (after.into(), ()))?;
                        Ok(call.return_value(first + second))
                    },
                ))
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
            let (plan, entries) = HostedProgram::from_library_plan(plan, entry, Vec::new())
                .expect("retained callback specialization");
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
            let mut driver =
                Driver::<NativeProfile>::new(&plan, &mut state, &mut stores, &mut echo);
            let work = driver.call(|plan, runtime| {
                let value = run_tuple(
                    plan,
                    runtime,
                    entry,
                    HostCallOrigin::Entry,
                    RetainedInputs::empty().into_retained(),
                )
                .expect("construct owned callback work");
                let [value] = external_values(&value);
                runtime.host().stores().work(value.lease())
            });
            let mut cx = Context::from_waker(Waker::noop());
            assert!(
                Box::pin(driver.observe(&work))
                    .as_mut()
                    .poll(&mut cx)
                    .is_pending()
            );
            driver.call(|_, runtime| {
                assert_eq!(runtime.host_state().generation, 1);
                runtime.host_state().generation = 2;
            });
            send.send(Ok(0)).expect("native gate survives waiter drop");
            let completion =
                completed_observation(Box::pin(driver.observe(&work)).as_mut().poll(&mut cx));
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
                for discarded in 0..7 {
                    let (send, gate) = futures_channel::oneshot::channel();
                    let mut state = NativeState {
                        gates: [gate].into(),
                        generation: 1,
                        ..NativeState::default()
                    };
                    let mut stores = crate::host::HostFutureStore::default();
                    let mut echo = Echo::default();
                    let mut driver = Driver::new(&plan, &mut state, &mut stores, &mut echo);
                    let work = driver.call(|plan, runtime| {
                        let values = run_tuple(
                            plan,
                            runtime,
                            entry,
                            HostCallOrigin::Entry,
                            RetainedInputs::empty().into_retained(),
                        )
                        .expect("owned callback work");
                        let [value] = external_values(&values);
                        runtime.host().stores().work(value.lease())
                    });
                    send.send(if discarded == 6 {
                        Err(crate::HostFailure::new("native gate rejected"))
                    } else {
                        Ok(0)
                    })
                    .expect("unpolled native gate");
                    let mut cx = Context::from_waker(Waker::noop());
                    if discarded == 6 {
                        let result = completed_observation(
                            Box::pin(driver.observe(&work)).as_mut().poll(&mut cx),
                        );
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
                                driver
                                    .call(|plan, runtime| {
                                        request.service(plan, runtime).expect("live conversion")
                                    })
                                    .deliver();
                            }
                        }
                        assert_eq!(
                            observer.as_mut().poll(&mut cx).map(Result::err),
                            Poll::Ready(Some(Cancelled))
                        );
                    }
                    drop(driver);
                    assert_eq!(
                        state.generation,
                        if discarded == 6 || discarded < 2 {
                            1
                        } else if discarded < 4 {
                            2
                        } else {
                            3
                        }
                    );
                    assert_eq!(
                        echo.output.len(),
                        usize::from(discarded == 4 || discarded == 5)
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
                2,
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
            let (plan, entries) = HostedProgram::from_library_plan(library, entry, Vec::new())
                .expect("sealed source");
            for discarded in 0..=request_count {
                let (sender, gate) = futures_channel::oneshot::channel();
                sender.send(Ok(2)).expect("ready native result");
                let mut state = NativeState::default();
                state.gates.push_back(gate);
                let mut stores = crate::host::HostFutureStore::default();
                let mut echo = Vec::new();
                let mut output = |value: crate::EchoOutput| echo.push(value.to_string());
                let mut driver = Driver::new(&plan, &mut state, &mut stores, &mut output);
                let (work, independent) = driver.call(|plan, runtime| {
                    let values = run_tuple(
                        plan,
                        runtime,
                        *entries.tuples[0].function(),
                        HostCallOrigin::Entry,
                        RetainedInputs::empty().into_retained(),
                    )
                    .expect("source work construction");
                    let [work, independent] = external_values::<2>(&values);
                    (
                        runtime.host().stores().work(work.lease()),
                        runtime.host().stores().work(independent.lease()),
                    )
                });
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
                        break;
                    }
                    driver
                        .call(|plan, state| request.service(plan, state).expect("live request"))
                        .deliver();
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
            mut call: crate::HostCall<
                'call,
                NativeProfile,
                NativeProvider,
                crate::work_fixture::WorkHostType<num_bigint::BigInt>,
            >,
            callback: crate::HostCallable<
                'call,
                crate::HostTypeList<num_bigint::BigInt, crate::HostTypeListEnd>,
                crate::work_fixture::WorkHostType<num_bigint::BigInt>,
            >,
            value: num_bigint::BigInt,
        ) -> Result<
            crate::HostCallCompletion<'call, crate::work_fixture::WorkHostType<num_bigint::BigInt>>,
            crate::HostCallError,
        > {
            call.invoke(callback, (value, ()))
                .map(|value| call.return_value(value))
        }
        for construction in ["fetch(40)", "invoke(fetch, 40)"] {
            let source = format!(
                "import fixture/work as future\n@external(erlang, \"native\", \"fetch\")\nfn fetch(value: Int) -> future.Work(Int)\npub fn make() {{\n  let original = {construction}\n  #(future.map(original, fn(value) {{ echo value value + 2 }}), original)\n}}\n@external(erlang, \"native\", \"invoke\")\nfn invoke(callback: fn(Int) -> future.Work(Int), value: Int) -> future.Work(Int)\n"
            );
            let mut providers = crate::work_fixture::WorkComponent::providers::<NativeProfile>()
                .expect("Future provider");
            providers.push(crate::host::HostProviderModule::new("application", "library")
            .expect("native module")
            .with_scoped_function_and_constructions::<NativeProvider, (num_bigint::BigInt,), crate::work_fixture::WorkHostType<num_bigint::BigInt>, crate::HostTypeListEnd, _>("fetch", fetch)
            .expect("native Future function")
            .with_scoped_function::<NativeProvider, (crate::HostFunctionType<crate::HostTypeList<num_bigint::BigInt, crate::HostTypeListEnd>, crate::work_fixture::WorkHostType<num_bigint::BigInt>>, num_bigint::BigInt), crate::work_fixture::WorkHostType<num_bigint::BigInt>, _>("invoke", invoke)
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
            let (plan, entries) = HostedProgram::from_library_plan(plan, entry, Vec::new())
                .expect("native Future execution");
            let entry = *entries.tuples[0].function();

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
                let mut driver =
                    Driver::<NativeProfile>::new(&plan, &mut state, &mut stores, &mut echo);
                let (mapped, original) = driver.call(|plan, runtime| {
                    let values = run_tuple(
                        plan,
                        runtime,
                        entry,
                        HostCallOrigin::Entry,
                        RetainedInputs::empty().into_retained(),
                    )
                    .expect("construct native Future");
                    let [mapped, original] = external_values(&values);
                    (
                        runtime.host().stores().work(mapped.lease()),
                        runtime.host().stores().work(original.lease()),
                    )
                });
                assert_eq!(polls.load(Ordering::SeqCst), 0);
                let wake = Arc::new(WakeCount::default());
                let waker = Waker::from(Arc::clone(&wake));
                let mut cx = Context::from_waker(&waker);
                let mut observation = Box::pin(driver.observe(&mapped));
                assert!(observation.as_mut().poll(&mut cx).is_pending());
                assert_eq!(polls.load(Ordering::SeqCst), 1);
                drop(observation);
                send.send(outcome).expect("native operation still retained");
                assert_eq!(polls.load(Ordering::SeqCst), 1);
                let mut observation = Box::pin(driver.observe(&mapped));
                let completion = completed_observation(observation.as_mut().poll(&mut cx));
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
        use crate::embedding::{FunctionDeclaration, HostedModuleBuilder, with_execution_scope};
        use crate::work_fixture::WorkType;
        use futures_util::FutureExt;
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
            with_execution_scope(async |guard| {
                let mut scope = module.attach(guard, &mut state, &mut echo);
                let work = scope.call(&make, ()).expect("construct then");
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
            })
            .now_or_never()
            .expect("host-controlled completion");
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
        let (plan, entries) =
            HostedProgram::from_library_plan(plan, entry, Vec::new()).expect("all execution");
        let entry = *entries.tuples[0].function();

        for (failure, cancelled) in [(false, false), (true, false), (false, true)] {
            let (left_send, left_wait) = futures_channel::oneshot::channel();
            let (right_send, right_wait) = futures_channel::oneshot::channel();
            let polls = Arc::new(AtomicUsize::new(0));
            let mut state = NativeState {
                gates: [left_wait, right_wait].into(),
                polls: Arc::clone(&polls),
                ..NativeState::default()
            };
            let mut stores = crate::host::HostFutureStore::default();
            let mut echo = Echo::default();
            let mut driver =
                Driver::<NativeProfile>::new(&plan, &mut state, &mut stores, &mut echo);
            let (all, sibling) = driver.call(|plan, runtime| {
                let values = run_tuple(
                    plan,
                    runtime,
                    entry,
                    HostCallOrigin::Entry,
                    RetainedInputs::empty().into_retained(),
                )
                .expect("construct independent work");
                let [all, sibling] = external_values(&values);
                (
                    runtime.host().stores().work(all.lease()),
                    runtime.host().stores().work(sibling.lease()),
                )
            });
            let mut cx = Context::from_waker(Waker::noop());
            assert!(
                Box::pin(driver.observe(&all))
                    .as_mut()
                    .poll(&mut cx)
                    .is_pending()
            );
            assert_eq!(polls.load(Ordering::SeqCst), 2);
            if failure || cancelled {
                if cancelled {
                    drop(left_send);
                } else {
                    left_send
                        .send(Err(crate::HostFailure::new("left failed")))
                        .expect("left waiter");
                }
                let completion = Box::pin(driver.observe(&all)).as_mut().poll(&mut cx);
                if cancelled {
                    assert_eq!(completion.map(Result::err), Poll::Ready(Some(Cancelled)));
                } else {
                    completed_observation(completion).read(|result| assert!(result.is_err()));
                }
                drop(all);
                right_send
                    .send(Ok(2))
                    .expect("retained sibling was not cancelled");
                let completion = completed_observation(
                    Box::pin(driver.observe(&sibling)).as_mut().poll(&mut cx),
                );
                completion.read(|result| {
                    let value = result.as_ref().ok().expect("sibling success");
                    value.read(|value| assert_eq!(value.value(), &EvaluatedValue::Int(22.into())));
                });
            } else {
                right_send.send(Ok(2)).expect("right waiter");
                assert!(
                    Box::pin(driver.observe(&all))
                        .as_mut()
                        .poll(&mut cx)
                        .is_pending()
                );
                left_send.send(Ok(1)).expect("left still pending");
                let completion =
                    completed_observation(Box::pin(driver.observe(&all)).as_mut().poll(&mut cx));
                completion.read(|result| {
                    let value = result.as_ref().ok().expect("all success");
                    value.read(|value| {
                        let list = crate::runtime::StoredRuntimeList::new(
                            crate::runtime::BorrowedValue::from_stored(value).list(),
                        );
                        assert_eq!(list.decode_item(0, |item| item.into_int()), Some(11.into()));
                        assert_eq!(list.decode_item(1, |item| item.into_int()), Some(11.into()));
                        assert_eq!(list.decode_item(2, |item| item.into_int()), Some(22.into()));
                        assert_eq!(list.len(), 3);
                    });
                });
                assert!(
                    Box::pin(driver.observe(&sibling))
                        .as_mut()
                        .poll(&mut cx)
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
            let (plan, entries) = HostedProgram::from_library_plan(plan, entry, Vec::new())
                .expect("specialized Future callbacks");
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
            let mut driver = Driver::<FutureProfile>::new(&plan, &mut host, &mut stores, &mut echo);
            let work = driver.call(|plan, runtime| {
                let values = run_tuple(
                    plan,
                    runtime,
                    entry,
                    HostCallOrigin::Entry,
                    RetainedInputs::empty().into_retained(),
                )
                .expect("construct Future graph");
                let [value] = external_values(&values[..1]);
                assert_eq!(
                    &values[1..],
                    &[EvaluatedValue::Bool(true), EvaluatedValue::Bool(false)]
                );
                runtime.host().stores().work(value.lease())
            });
            let mut observation = Box::pin(driver.observe(&work));
            let completion = completed_observation(
                observation
                    .as_mut()
                    .poll(&mut Context::from_waker(Waker::noop())),
            );
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
                    .expect("original host gate")?;
                    Ok(crate::host::HostFutureCompletion::new(|call, _| {
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
            let work = call.restore_value::<WorkHostType<BigInt>>(&payload);
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
                    Ok(crate::host::HostFutureCompletion::new(move |call, _| {
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
        for name in ["make", "open", "observe_negative", "observe_ready"] {
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
        let (plan, entries) =
            HostedProgram::from_library_plan(library, entry, entries).expect("sealed entries");
        let make = *entries.tuples[0].function();
        let open = *entries.tuples[1].function();
        for (entry, cancelled) in [
            (*entries.tuples[2].function(), false),
            (*entries.tuples[3].function(), true),
        ] {
            let polls = Arc::new(AtomicUsize::new(0));
            let mut state = State {
                gate: None,
                polls: Arc::clone(&polls),
                unit: (),
            };
            let mut stores = Stores::default();
            let mut echo = Echo::default();
            let mut driver = Driver::new(&plan, &mut state, &mut stores, &mut echo);
            let work = driver.call(|plan, runtime| {
                let values = run_tuple(
                    plan,
                    runtime,
                    entry,
                    HostCallOrigin::Entry,
                    RetainedInputs::empty().into_retained(),
                )
                .expect("native observation of a ready input");
                let [value] = external_values(&values);
                runtime.host().stores().future.work(value.lease())
            });
            let mut cx = Context::from_waker(Waker::noop());
            if cancelled {
                let mut observer = std::pin::pin!(work.observe());
                assert!(observer.as_mut().poll(&mut cx).is_pending());
                let request = driver.work.next(&mut cx).expect("decode the input");
                driver
                    .call(|plan, runtime| request.service(plan, runtime).expect("live decode"))
                    .deliver();
                assert!(observer.as_mut().poll(&mut cx).is_pending());
                drop(driver.work.next(&mut cx).expect("encode the completion"));
                assert_eq!(
                    observer.as_mut().poll(&mut cx).map(Result::err),
                    Poll::Ready(Some(Cancelled))
                );
            } else {
                let result =
                    completed_observation(Box::pin(driver.observe(&work)).as_mut().poll(&mut cx));
                result.read(|result| {
                    result
                        .as_ref()
                        .err()
                        .expect("native validation failed")
                        .read(|error| {
                            assert_eq!(
                                host_error(error).failure().message(),
                                "negative completion"
                            );
                        })
                });
            }
            drop(driver);
            assert_eq!(polls.load(Ordering::SeqCst), 0);
            assert!(echo.output.is_empty());
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
            let mut driver = Driver::new(&plan, &mut state, &mut stores, &mut echo);
            let (outer, inner) = driver.call(|plan, runtime| {
                let values = run_tuple(
                    plan,
                    runtime,
                    make,
                    HostCallOrigin::Entry,
                    RetainedInputs::empty().into_retained(),
                )
                .expect("construct retained graph");
                let [outer, inner] = external_values(&values);
                let store = &runtime.host().stores().future;
                (store.work(outer.lease()), store.work(inner.lease()))
            });
            let completed = completed_observation(
                Box::pin(driver.observe(&outer))
                    .as_mut()
                    .poll(&mut Context::from_waker(Waker::noop())),
            );
            if poll_inner {
                assert!(
                    Box::pin(driver.observe(&inner))
                        .as_mut()
                        .poll(&mut Context::from_waker(Waker::noop()))
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
                let value = completed_observation(
                    Box::pin(driver.observe(&inner))
                        .as_mut()
                        .poll(&mut Context::from_waker(Waker::noop())),
                );
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
            let mut next = Driver::new(&plan, &mut fresh, &mut stores, &mut echo);
            let restored = next.call(|plan, runtime| {
                let mut input = RetainedInputs::empty();
                input.push_value(envelope);
                let values = run_tuple(
                    plan,
                    runtime,
                    open,
                    HostCallOrigin::Entry,
                    input.into_retained(),
                )
                .expect("restore via real source call");
                let values = external_values::<3>(&values);
                values.map(|work| runtime.host().stores().future.work(work.lease()))
            });
            for restored in restored {
                let restored = Box::pin(next.observe(&restored))
                    .as_mut()
                    .poll(&mut Context::from_waker(Waker::noop()));
                assert_eq!(
                    restored.map(|value| value.map(|completion| completion.read(Result::is_err))),
                    if complete_inner {
                        Poll::Ready(Ok(failed))
                    } else {
                        Poll::Ready(Err(Cancelled))
                    },
                );
            }
            assert_eq!(
                Box::pin(next.observe(&outer))
                    .as_mut()
                    .poll(&mut Context::from_waker(Waker::noop()))
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
            Vec::new(),
            Vec::new(),
            crate::plan::execution::type_::FunctionType::new(
                Vec::new(),
                crate::plan::execution::type_::ValueType::Nil,
            ),
        );
        int_callback(&[EvaluatedValue::Function(function.into())]);
    }
}
