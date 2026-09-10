use super::{ExecutionHost, HostTask, TaskExit, Worker};
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::Instant;
use tokio::runtime::Handle;
use tokio::task::JoinHandle;

/// Scheduling and timers on an existing Tokio runtime.
#[derive(Clone)]
pub struct TokioHost {
    runtime: Handle,
}

struct Task {
    join: JoinHandle<()>,
}

impl TokioHost {
    pub fn new(runtime: Handle) -> Self {
        Self { runtime }
    }
}

impl ExecutionHost for TokioHost {
    fn spawn(&self, worker: Worker) -> Box<dyn HostTask> {
        Box::new(Task {
            join: self.runtime.spawn(worker),
        })
    }

    fn now(&self) -> Instant {
        let _entered = self.runtime.enter();
        tokio::time::Instant::now().into_std()
    }

    fn sleep_until(&self, deadline: Instant) -> Pin<Box<dyn Future<Output = ()> + Send + 'static>> {
        let _entered = self.runtime.enter();
        Box::pin(tokio::time::sleep_until(deadline.into()))
    }
}

impl Future for Task {
    type Output = TaskExit;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<TaskExit> {
        Pin::new(&mut self.join).poll(cx).map(|exit| match exit {
            Ok(()) => TaskExit::Completed,
            Err(error) if error.is_cancelled() => TaskExit::Cancelled,
            Err(error) => TaskExit::Failed(Box::new(error)),
        })
    }
}

impl HostTask for Task {
    fn cancel(&self) {
        self.join.abort();
    }
}

impl Drop for Task {
    fn drop(&mut self) {
        self.cancel();
    }
}

#[cfg(test)]
mod tests {
    use super::TokioHost;
    use crate::execution::{ExecutionHost, TaskExit};
    use std::sync::Arc;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::time::Duration;

    struct Release(Arc<AtomicUsize>);
    impl Drop for Release {
        fn drop(&mut self) {
            self.0.fetch_add(1, Ordering::SeqCst);
        }
    }

    #[test]
    fn uses_the_selected_runtime_clock_and_acknowledges_completion() {
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_time()
            .start_paused(true)
            .build()
            .unwrap();
        let host = TokioHost::new(runtime.handle().clone());
        let start = host.now();
        let sleep = host.clone().sleep_until(start + Duration::from_secs(5));
        let count = Arc::new(AtomicUsize::new(0));
        let released = Arc::clone(&count);
        let task = host.spawn(Box::pin(async move {
            let _release = Release(released);
            sleep.await;
        }));
        assert_eq!(count.load(Ordering::SeqCst), 0);
        runtime.block_on(async {
            tokio::time::advance(Duration::from_secs(5)).await;
            assert_eq!(task_outcome(task.await), Ok("completed"));
        });
        assert_eq!(host.now().duration_since(start), Duration::from_secs(5));
        assert_eq!(count.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn completion_and_cancellation_acknowledge_destruction_before_and_after_first_poll() {
        let runtime = tokio::runtime::Builder::new_current_thread()
            .build()
            .unwrap();
        let host = TokioHost::new(runtime.handle().clone());
        for (started, cancel) in [(false, true), (true, true), (false, false), (true, false)] {
            let count = Arc::new(AtomicUsize::new(0));
            let release = Release(Arc::clone(&count));
            let (send, receive) = futures_channel::oneshot::channel();
            let (finish, wait_finish) = futures_channel::oneshot::channel();
            let task = host.spawn(Box::pin(async move {
                let _release = release;
                send.send(()).unwrap();
                wait_finish.await.unwrap();
            }));
            runtime.block_on(async {
                if started {
                    receive.await.unwrap();
                }
                assert_eq!(count.load(Ordering::SeqCst), 0);
                if cancel {
                    task.cancel();
                    assert_eq!(task_outcome(task.await), Ok("cancelled"));
                } else {
                    finish.send(()).unwrap();
                    assert_eq!(task_outcome(task.await), Ok("completed"));
                }
                assert_eq!(count.load(Ordering::SeqCst), 1);
            });
        }
    }

    #[test]
    fn dropping_the_handle_requests_cancellation_without_detaching_work() {
        let runtime = tokio::runtime::Builder::new_current_thread()
            .build()
            .unwrap();
        let host = TokioHost::new(runtime.handle().clone());
        for (started, complete) in [(false, false), (true, false), (false, true), (true, true)] {
            let (released, release) = futures_channel::oneshot::channel::<()>();
            let (ready, wait) = futures_channel::oneshot::channel();
            let (finish, wait_finish) = futures_channel::oneshot::channel();
            let task = host.spawn(Box::pin(async move {
                let _retained = released;
                ready.send(()).unwrap();
                wait_finish.await.unwrap();
            }));
            runtime.block_on(async {
                if started {
                    wait.await.unwrap();
                }
                if complete {
                    finish.send(()).unwrap();
                    assert_eq!(task_outcome(task.await), Ok("completed"));
                } else {
                    drop(task);
                }
                assert!(release.await.is_err());
            });
        }
    }

    #[test]
    fn a_worker_panic_remains_an_executor_failure() {
        let runtime = tokio::runtime::Builder::new_current_thread()
            .build()
            .unwrap();
        let host = TokioHost::new(runtime.handle().clone());
        let task = host.spawn(Box::pin(async {
            panic!("native worker panic");
        }));
        let error = task_outcome(runtime.block_on(task)).unwrap_err();
        assert!(error.contains("native worker panic"));
    }

    fn task_outcome(exit: TaskExit) -> Result<&'static str, String> {
        match exit {
            TaskExit::Completed => Ok("completed"),
            TaskExit::Cancelled => Ok("cancelled"),
            TaskExit::Failed(error) => Err(error.to_string()),
        }
    }
}
