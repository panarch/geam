#![allow(
    dead_code,
    reason = "each consumer selects only the fixture driving operations it needs"
)]

use geam_core::execution::{ExecutionHost, HostTask, RunError, TaskExit, Worker};
use geam_core::{EchoSink, ExecutionError, HostProfile, HostedExecution, Value};
use std::collections::VecDeque;
use std::future::Future;
use std::pin::{Pin, pin};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Wake, Waker};
use std::time::{Duration, Instant};

/// A manually driven test host. Workers are polled outside all scheduler locks.
#[derive(Clone)]
pub struct TestHost {
    workers: Arc<Mutex<VecDeque<Scheduled>>>,
    clock: Arc<Mutex<Clock>>,
}

struct Scheduled {
    worker: Worker,
    completion: Arc<Mutex<Completion>>,
}

struct Completion {
    cancelled: bool,
    exit: Option<TaskExit>,
    waiter: Option<Waker>,
}

struct Task(Arc<Mutex<Completion>>);

struct Progress(AtomicBool);
impl Wake for Progress {
    fn wake(self: Arc<Self>) {
        self.0.store(true, Ordering::Release);
    }
    fn wake_by_ref(self: &Arc<Self>) {
        self.0.store(true, Ordering::Release);
    }
}

struct Clock {
    now: Instant,
    waiters: Vec<Waker>,
}

impl Default for TestHost {
    fn default() -> Self {
        Self {
            workers: Arc::default(),
            clock: Arc::new(Mutex::new(Clock {
                now: Instant::now(),
                waiters: Vec::new(),
            })),
        }
    }
}

impl TestHost {
    pub fn block_on<Output>(&self, future: impl Future<Output = Output>) -> Output {
        let mut future = pin!(future);
        let mut cx = Context::from_waker(Waker::noop());
        loop {
            if let Poll::Ready(output) = future.as_mut().poll(&mut cx) {
                return output;
            }
            self.step();
        }
    }

    pub fn step(&self) {
        self.poll_workers(&mut Context::from_waker(Waker::noop()));
    }

    pub fn poll<Output>(&self, mut future: Pin<&mut impl Future<Output = Output>>) -> Poll<Output> {
        let progress = Arc::new(Progress(AtomicBool::new(false)));
        let waker = Waker::from(Arc::clone(&progress));
        let mut cx = Context::from_waker(&waker);
        loop {
            let output = future.as_mut().poll(&mut cx);
            if output.is_ready() {
                return output;
            }
            self.poll_workers(&mut cx);
            if !progress.0.swap(false, Ordering::AcqRel) {
                return Poll::Pending;
            }
        }
    }

    fn poll_workers(&self, cx: &mut Context<'_>) {
        let workers = std::mem::take(&mut *self.workers.lock().unwrap());
        for mut task in workers {
            let cancelled = task.completion.lock().unwrap().cancelled;
            let exit = if cancelled {
                Some(TaskExit::Cancelled)
            } else {
                match task.worker.as_mut().poll(cx) {
                    Poll::Ready(()) => Some(TaskExit::Completed),
                    Poll::Pending => None,
                }
            };
            if let Some(exit) = exit {
                drop(task.worker);
                let waiter = {
                    let mut completion = task.completion.lock().unwrap();
                    completion.exit = Some(exit);
                    completion.waiter.take()
                };
                if let Some(waiter) = waiter {
                    waiter.wake();
                }
            } else {
                self.workers.lock().unwrap().push_back(task);
            }
        }
    }

    pub fn advance(&self, duration: Duration) {
        let waiters = {
            let mut clock = self.clock.lock().unwrap();
            clock.now += duration;
            std::mem::take(&mut clock.waiters)
        };
        for waiter in waiters {
            waiter.wake();
        }
    }
}

impl ExecutionHost for TestHost {
    fn spawn(&self, worker: Worker) -> Box<dyn HostTask> {
        let completion = Arc::new(Mutex::new(Completion {
            cancelled: false,
            exit: None,
            waiter: None,
        }));
        self.workers.lock().unwrap().push_back(Scheduled {
            worker,
            completion: Arc::clone(&completion),
        });
        Box::new(Task(completion))
    }

    fn now(&self) -> Instant {
        self.clock.lock().unwrap().now
    }

    fn sleep_until(&self, deadline: Instant) -> Worker {
        let clock = Arc::clone(&self.clock);
        Box::pin(std::future::poll_fn(move |cx| {
            let mut clock = clock.lock().unwrap();
            if clock.now >= deadline {
                Poll::Ready(())
            } else {
                if !clock
                    .waiters
                    .iter()
                    .any(|waiter| waiter.will_wake(cx.waker()))
                {
                    clock.waiters.push(cx.waker().clone());
                }
                Poll::Pending
            }
        }))
    }
}

impl Future for Task {
    type Output = TaskExit;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<TaskExit> {
        let mut completion = self.0.lock().unwrap();
        match completion.exit.take() {
            Some(exit) => Poll::Ready(exit),
            None => {
                completion.waiter = Some(cx.waker().clone());
                Poll::Pending
            }
        }
    }
}

impl HostTask for Task {
    fn cancel(&self) {
        self.0.lock().unwrap().cancelled = true;
    }
}

impl Drop for Task {
    fn drop(&mut self) {
        self.cancel();
    }
}

pub fn run<Profile: HostProfile>(
    execution: &mut HostedExecution<Profile>,
    state: &mut Profile::RunState,
    echo: &mut (dyn EchoSink + Send),
) -> Result<Value, ExecutionError>
where
    Profile::RunState: Send,
    Profile::ExternalStores: Send,
{
    let host = TestHost::default();
    host.block_on(execution.run_main(&host, state, echo))
        .map_err(|error| match error {
            RunError::Execution(error) => error,
            other => panic!("the controlled test host failed: {other}"),
        })
}

#[cfg(test)]
mod tests {
    use geam_core::host::{
        HostCall, HostCallContinuation, HostCallError, HostConstructions, HostExecutionError,
        HostProfile, HostProvider, HostProviderModule, HostProviderSet, HostTypeListEnd,
    };
    use geam_core::{HostedExecution, ModuleSource, PackageSource};

    struct Profile;
    impl HostProfile for Profile {
        type RunState = ();
        type ExternalStores = ();
    }

    struct Provider;
    impl HostProvider<Profile> for Provider {
        type State = ();
        fn project(state: &mut ()) -> &mut () {
            state
        }
    }

    fn cancel<'call>(
        call: HostCall<'call, Profile, Provider, ()>,
        constructions: HostConstructions<'call, HostTypeListEnd>,
    ) -> Result<HostCallContinuation<'call, ()>, HostCallError> {
        Ok(call.resume(constructions, |_| {
            Box::pin(async { Err(HostExecutionError::Cancelled) })
        }))
    }

    #[test]
    #[should_panic(expected = "the controlled test host failed: the Gleam entry was cancelled")]
    fn execution_only_fixture_rejects_cancelled_entries() {
        let providers =
            HostProviderSet::from_providers([HostProviderModule::new("application", "main")
                .unwrap()
                .with_resumable_function::<Provider, (), (), HostTypeListEnd, _>("cancel", cancel)
                .unwrap()])
            .unwrap();
        let program = geam_core::compile_typed_host_program(
            "application",
            "main",
            [PackageSource::new(
                "application",
                Vec::<String>::new(),
                [ModuleSource::new(
                    "main",
                    "main.gleam",
                    r#"
@external(erlang, "native", "cancel")
fn cancel() -> Nil
pub fn main() { cancel() }
"#,
                )],
            )],
            providers,
        )
        .unwrap();
        let plan = geam_core::plan_host_program(program).unwrap();
        let mut execution = HostedExecution::try_from_module_plan(plan).unwrap();
        let _ = super::run(&mut execution, &mut (), &mut Vec::new());
    }
}
