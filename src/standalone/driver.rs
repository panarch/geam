use std::future::Future;
use tokio::runtime::Runtime;
use tokio::task::{JoinError, JoinHandle};

pub fn run_driver<T: Send + 'static>(
    runtime: &Runtime,
    body: impl Future<Output = T> + Send + 'static,
) -> Result<T, JoinError> {
    Driver(runtime.spawn(body)).join(runtime)
}

struct Driver<T>(JoinHandle<T>);

impl<T> Driver<T> {
    fn join(mut self, runtime: &Runtime) -> Result<T, JoinError> {
        match runtime.block_on(&mut self.0) {
            Ok(output) => Ok(output),
            Err(error) => match error.try_into_panic() {
                Ok(payload) => std::panic::resume_unwind(payload),
                Err(error) => Err(error),
            },
        }
    }
}

impl<T> Drop for Driver<T> {
    fn drop(&mut self) {
        self.0.abort();
    }
}

#[cfg(test)]
mod tests {
    use super::{Driver, run_driver};
    use futures_channel::oneshot;
    use std::cell::Cell;
    use std::panic::{AssertUnwindSafe, catch_unwind, panic_any};
    use tokio::runtime::Builder;

    #[test]
    fn joins_owned_send_only_state_on_the_existing_worker_pool() {
        let runtime = Builder::new_multi_thread()
            .worker_threads(1)
            .build()
            .unwrap();
        let caller = std::thread::current().id();
        let state = Cell::new(41);
        let (state, worker) = run_driver(&runtime, async move {
            state.set(state.get() + 1);
            (state, std::thread::current().id())
        })
        .unwrap();
        assert_ne!(caller, worker);
        assert_eq!(state.get(), 42);
    }

    #[test]
    fn resumes_the_original_non_string_panic_after_releasing_task_inputs() {
        #[derive(Debug, PartialEq, Eq)]
        struct Cause(u32);

        let runtime = Builder::new_multi_thread()
            .worker_threads(1)
            .build()
            .unwrap();
        let (released, wait) = oneshot::channel::<()>();
        let panic = catch_unwind(AssertUnwindSafe(|| {
            run_driver::<i32>(&runtime, async move {
                let _released = released;
                panic_any(Cause(42));
            })
        }))
        .unwrap_err();
        assert_eq!(*panic.downcast::<Cause>().unwrap(), Cause(42));
        assert!(runtime.block_on(wait).is_err());
    }

    #[test]
    fn completion_cancellation_drop_and_unwinding_release_the_owned_task() {
        enum Ending {
            Complete,
            Cancel,
            Drop,
            Unwind,
        }

        for ending in [
            Ending::Complete,
            Ending::Cancel,
            Ending::Drop,
            Ending::Unwind,
        ] {
            let runtime = Builder::new_multi_thread()
                .worker_threads(1)
                .build()
                .unwrap();
            let (started, start) = oneshot::channel();
            let (released, wait) = oneshot::channel::<()>();
            let (complete, completion) = oneshot::channel();
            let driver = Driver(runtime.spawn(async move {
                let _released = released;
                started.send(()).unwrap();
                completion.await.unwrap()
            }));
            runtime.block_on(start).unwrap();
            match ending {
                Ending::Complete => {
                    complete.send(42).unwrap();
                    assert_eq!(driver.join(&runtime).unwrap(), 42);
                }
                Ending::Cancel => {
                    driver.0.abort();
                    assert!(driver.join(&runtime).unwrap_err().is_cancelled());
                }
                Ending::Drop => drop(driver),
                Ending::Unwind => {
                    let panic = catch_unwind(AssertUnwindSafe(|| {
                        runtime.block_on(async { driver.join(&runtime) })
                    }));
                    assert!(panic.is_err());
                }
            }
            assert!(runtime.block_on(wait).is_err());
        }
    }
}
