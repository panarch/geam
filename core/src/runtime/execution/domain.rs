use super::{ExecutionContext, Request, Units};
use crate::execution::{
    DriverError, ExecutionClock, ExecutionHost, HostExecutionState, HostTask, TaskExit,
    UnitFinished, UnitOwner, Worker,
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
            work: ExecutionWork::new(),
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
    ) -> Result<Output, DriverError> {
        let output = {
            let mut body = pin!(body);
            poll_fn(|cx| self.poll_body(body.as_mut(), cx)).await
        };
        self.close();
        poll_fn(|cx| {
            self.reap(cx);
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
    ) -> Poll<Result<Output, DriverError>> {
        self.finish_units(cx);
        let output = body.as_mut().poll(cx);
        if output.is_pending() {
            self.service(cx);
        }
        if let Some(error) = self.reap(cx) {
            return Poll::Ready(Err(error));
        }
        output.map(Ok)
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
        for _ in 0..self.budget.get() {
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
                self.dispatch(request);
            }
            if empty {
                return;
            }
        }
        cx.waker().wake_by_ref();
    }

    fn dispatch(&mut self, request: Request<Profile>) {
        match request {
            Request::Service(request) => {
                let context = self.work.execution().with_unit(request.unit().cloned());
                let delivery = {
                    let mut runtime = RuntimeState::with_host_and_lists(
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
                    );
                    request.service(&self.plan, &mut runtime)
                };
                if let Some(delivery) = delivery {
                    delivery.deliver();
                }
            }
            Request::Callback(request) => {
                let (context, root) = match request.unit() {
                    Some(unit) if !unit.is_active() => return,
                    Some(unit) => (self.work.execution().with_unit(Some(unit.clone())), None),
                    None => {
                        let (context, root) = self.begin(UnitOwner::new(self.units.completion()));
                        (context, Some(root))
                    }
                };
                let worker =
                    request.into_worker(Arc::clone(&self.plan), context, self.budget, root);
                self.tasks.push(self.host.spawn(worker));
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
                Poll::Ready(Some(TaskExit::Completed)) => {}
                Poll::Ready(Some(TaskExit::Cancelled)) => {
                    failure.get_or_insert(DriverError::Cancelled);
                }
                Poll::Ready(Some(TaskExit::Failed(error))) => {
                    failure.get_or_insert(DriverError::Failed(error));
                }
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
    use super::Domain;
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
        released: Arc<AtomicUsize>,
        started: AtomicUsize,
        turns: AtomicUsize,
    }

    struct ManualTask {
        abort: AbortHandle,
        exit: oneshot::Receiver<TaskExit>,
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
            Box::new(ManualTask { abort, exit })
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
            Pin::new(&mut self.exit)
                .poll(cx)
                .map(|exit| exit.unwrap_or(TaskExit::Cancelled))
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
        let (plan, entries) = HostedProgram::from_library_plan(library, entry, Vec::new()).unwrap();
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
    fn independent_cpu_entries_yield_cancel_and_leave_borrowed_state_with_the_driver() {
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
            NonZeroUsize::new(17).unwrap(),
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
        let (plan, entries) = HostedProgram::from_library_plan(library, entry, Vec::new()).unwrap();
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
            .unwrap();
        assert_eq!(output, Ok(BigInt::from(42)));
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
        host.finish(domain.drive(std::future::ready(()))).unwrap();
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
                NonZeroUsize::MIN,
            );
            let context = domain.context().execution;
            let mut queued = std::pin::pin!(context.with_state(|state| state.set(99)));
            let mut cx = Context::from_waker(Waker::noop());
            assert!(queued.as_mut().poll(&mut cx).is_pending());
            if complete_before_service {
                host.block_on(domain.drive(std::future::ready(()))).unwrap();
                assert_eq!(queued.as_mut().poll(&mut cx), Poll::Ready(Err(Cancelled)));
                assert_eq!(state.get(), 7);
            } else {
                assert_eq!(
                    host.block_on(domain.drive(queued.as_mut())).unwrap(),
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
            .unwrap()
            .unwrap();
        let callable = int_callback(&tuple.as_slice()[0]);
        let mut successful = std::pin::pin!(context.execution.with_state(set_state));
        host.finish(poll_fn(|cx| domain.poll_body(successful.as_mut(), cx)))
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
        host.finish(domain.drive(std::future::ready(()))).unwrap();
        assert_eq!(state.get(), 7);
        assert!(echo.is_empty());
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
        host.finish(domain.drive(std::future::ready(()))).unwrap();
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
    host.poll(driving.as_mut())
        .map(|output| output.expect("controlled host turn"))
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
        let (plan, _) = HostedProgram::from_library_plan(
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
                driver.dispatch(request);
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
                driver.dispatch(request);
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
        let (plan, _) =
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
        let (plan, entries) =
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
                .spawn(move || executor.block_on(observation).expect("domain cleanup"))
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
            let (plan, entries) = HostedProgram::from_library_plan(plan, entry, Vec::new())
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
                                driver.dispatch(request);
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
            let (plan, entries) = HostedProgram::from_library_plan(library, entry, Vec::new())
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
                    driver.dispatch(request);
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
                        let value = work.into_host(&mut call);
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
            let (plan, entries) = HostedProgram::from_library_plan(plan, entry, Vec::new())
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
            let (plan, entries) = HostedProgram::from_library_plan(plan, entry, Vec::new())
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
                    .expect("original host gate")?;
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
        let plan = Arc::new(plan);
        let executor = TestHost::default();
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
            let mut driver = Domain::new(
                Arc::clone(&plan),
                &executor,
                &mut state,
                &mut stores,
                &mut echo,
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
                driver.dispatch(request);
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
            let mut driver = Domain::new(
                Arc::clone(&plan),
                &executor,
                &mut state,
                &mut stores,
                &mut echo,
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
                NonZeroUsize::MIN,
            );
            let restored = {
                let source_entries = next.context();
                let services = next.work.execution();
                complete_domain(&mut next, &executor, async move {
                    let mut input = RetainedInputs::empty();
                    input.push_value(envelope);
                    let values = source_entries
                        .call(open, HostCallOrigin::Entry, input.into_retained())
                        .await
                        .expect("active restoring entry")
                        .expect("restore via real source call");
                    services
                        .with_runtime(move |_, runtime| {
                            let values = external_values::<3>(&values);
                            values.map(|work| runtime.host().stores().future.work(work.lease()))
                        })
                        .await
                        .expect("active runtime service")
                })
            };
            for restored in restored {
                let restored =
                    poll_domain(&mut next, &executor, Box::pin(restored.observe()).as_mut());
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
        let (execution, entries) = HostedProgram::from_library_plan(plan, entry, Vec::new())
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
        execution.dispatch(request);
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
            value.read(|value| assert_eq!(value.value(), &EvaluatedValue::Int(BigInt::from(42))));
        });

        drop(execution);
        let mut repeated = pin!(work.observe());
        let repeated = completed_observation(repeated.as_mut().poll(&mut cx));
        repeated.read(|result| {
            let value = result.as_ref().ok().expect("cached success");
            value.read(|value| assert_eq!(value.value(), &EvaluatedValue::Int(BigInt::from(42))));
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
        let (plan, entries) =
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
                NonZeroUsize::MIN,
            );
            let callback = int_callback(&mut execution, &executor, *entries.tuples[0].function());
            let context = execution.work.execution();
            let mut invocation = Box::pin(context.invoke_owned(
                callback,
                codec.clone(),
                HostCallOrigin::Entry,
                move |_| {
                    let mut values = CallbackInputs::new();
                    values.push_value(EvaluatedValue::Int(input.into()));
                    values
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
                execution.dispatch(request);
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
            assert_eq!(state.get(), usize::from(serviced >= 4 && input != -1));
            assert_eq!(echo.len(), usize::from(serviced >= 3));
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
                execution.dispatch(request);
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
            execution.dispatch(operation);
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
            execution.dispatch(operation);
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
        execution.dispatch(request_to_service);
        executor
            .block_on(execution.drive(std::future::ready(())))
            .expect("cancelled callback cleanup");
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
