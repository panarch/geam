use super::{Cancelled, Shared, Work, WorkFactory, WorkScope};
use crate::host::HostProfile;
use crate::plan::execution::HostedProgram;
use crate::runtime::error::HostCallOrigin;
pub(in crate::runtime) use crate::runtime::execution::Request;
use crate::runtime::execution::{ExecutionContext, ExecutionServices};
use crate::runtime::state::RuntimeStateFor;
use crate::runtime::{CallbackInputs, RetainedCallable, StoredRuntimeValue};
use std::future::Future;
use std::sync::OnceLock;
use std::task::Context;

pub(in crate::runtime) struct ExecutionWork<Profile: HostProfile> {
    initialized: OnceLock<WorkScope<Completion>>,
    execution: ExecutionServices<Profile>,
}

pub(crate) struct WorkContext<Profile: HostProfile> {
    work: WorkFactory<Completion>,
    pub(super) execution: ExecutionContext<Profile>,
}

pub(crate) type Completion = Result<Shared<StoredRuntimeValue>, Shared<crate::ExecutionError>>;
pub(crate) type SourceWork = Work<Completion>;

impl<Profile: HostProfile> ExecutionWork<Profile> {
    pub(in crate::runtime) fn new() -> Self {
        Self {
            initialized: OnceLock::new(),
            execution: ExecutionServices::new(),
        }
    }

    pub(in crate::runtime) fn context(&self) -> WorkContext<Profile> {
        WorkContext {
            work: self.initialized.get_or_init(WorkScope::new).factory(),
            execution: self.execution.context(),
        }
    }

    pub(in crate::runtime) fn next(&self, cx: &mut Context<'_>) -> Option<Request<Profile>> {
        self.execution.next(cx)
    }

    pub(in crate::runtime) fn execution(&self) -> ExecutionContext<Profile> {
        self.execution.context()
    }

    pub(in crate::runtime) fn close(&self) {
        self.execution.close();
        if let Some(work) = self.initialized.get() {
            work.close();
        }
    }
}

impl<Profile: HostProfile> WorkContext<Profile> {
    pub(crate) fn execution(&self) -> &ExecutionContext<Profile> {
        &self.execution
    }

    #[cfg(test)]
    pub(in crate::runtime) fn create(
        &self,
        native: impl Future<Output = crate::runtime::execution::NativeCompletion> + Send + 'static,
    ) -> SourceWork {
        self.compose(|_| async move {
            Ok(Shared::new(
                native.await.map(Shared::new).map_err(Shared::new),
            ))
        })
    }

    pub(crate) fn ready(&self, value: StoredRuntimeValue) -> SourceWork {
        self.work.ready(Ok(Shared::new(value)))
    }

    pub(crate) fn map(
        &self,
        input: SourceWork,
        callable: RetainedCallable,
        origin: HostCallOrigin,
    ) -> SourceWork {
        let context = self.execution.clone();
        self.work.compose(|dependencies| async move {
            let completed = dependencies.observe(&input).await?;
            let value = completed.read(Clone::clone);
            match value {
                Ok(value) => {
                    let mut inputs = CallbackInputs::new();
                    value.read(|value| inputs.push_value(value.value().clone()));
                    let result = context.invoke(callable, origin, inputs).await?;
                    Ok(Shared::new(result.map(Shared::new).map_err(Shared::new)))
                }
                Err(error) => Ok(Shared::new(Err(error))),
            }
        })
    }

    pub(super) fn compose<Native>(
        &self,
        build: impl FnOnce(super::Dependencies<Completion>) -> Native,
    ) -> SourceWork
    where
        Native: Future<Output = Result<Shared<Completion>, Cancelled>> + Send + 'static,
    {
        self.work.compose(build)
    }

    pub(in crate::runtime) fn with_runtime<Output: Send + 'static, Operation>(
        &self,
        operation: Operation,
    ) -> impl Future<Output = Result<Output, Cancelled>> + Send + use<Profile, Output, Operation>
    where
        Operation: FnOnce(
                &HostedProgram<Profile>,
                &mut RuntimeStateFor<'_, HostedProgram<Profile>>,
            ) -> Output
            + Send
            + 'static,
    {
        self.execution.with_runtime(operation)
    }
}

impl<Profile: HostProfile> Clone for WorkContext<Profile> {
    fn clone(&self) -> Self {
        Self {
            work: self.work.clone(),
            execution: self.execution.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ExecutionWork;
    use crate::host::HostProfile;
    use crate::runtime::shared::Shared;
    use crate::runtime::work::Cancelled;
    use crate::runtime::{EvaluatedValue, StoredRuntimeValue};
    use std::cell::Cell;
    use std::future::Future;
    use std::pin::pin;
    use std::sync::Arc;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::task::{Context, Poll, Waker};

    struct Profile;
    impl HostProfile for Profile {
        type RunState = Cell<usize>;
        type ExternalStores = ();
        type ExecutionState = ();
    }

    #[test]
    fn ordinary_execution_does_not_initialize_work_and_work_contexts_share_closure() {
        let execution = ExecutionWork::<Profile>::new();
        let mut cx = Context::from_waker(Waker::noop());
        assert!(execution.next(&mut cx).is_none());
        assert!(execution.initialized.get().is_none());
        let ordinary = execution.execution();
        let mut request = pin!(ordinary.with_state(std::mem::take));
        assert!(request.as_mut().poll(&mut cx).is_pending());
        assert!(execution.initialized.get().is_none());
        let first = execution.context();
        let second = execution.context();
        assert!(execution.initialized.get().is_some());
        let left = first.create(std::future::pending());
        let right = second.create(std::future::pending());
        execution.close();
        assert_eq!(request.as_mut().poll(&mut cx), Poll::Ready(Err(Cancelled)));
        for work in [left, right] {
            assert_eq!(
                pin!(work.observe()).as_mut().poll(&mut cx).map(Result::err),
                Poll::Ready(Some(Cancelled))
            );
        }
    }

    #[test]
    fn source_work_forwards_shared_completion_without_replaying_native_work() {
        let execution = ExecutionWork::<Profile>::new();
        let context = execution.context();
        let polls = Arc::new(AtomicUsize::new(0));
        let work = context.create({
            let polls = Arc::clone(&polls);
            async move {
                polls.fetch_add(1, Ordering::SeqCst);
                Ok(StoredRuntimeValue::test_int(42.into()))
            }
        });
        let forward =
            context.compose(|dependencies| async move { dependencies.observe(&work).await });
        let mut cx = Context::from_waker(Waker::noop());
        let mut first = pin!(forward.observe());
        let observed = first.as_mut().poll(&mut cx);
        assert_eq!(
            observed.map(|value| {
                let copied: Shared<super::Completion> = value.expect("live work").clone();
                copied.read(|value| {
                    value
                        .as_ref()
                        .map(|value| value.read(|value| value.value().clone()))
                        .map_err(|_| "native failure")
                })
            }),
            Poll::Ready(Ok(EvaluatedValue::Int(42.into())))
        );
        assert_eq!(polls.load(Ordering::SeqCst), 1);
        let ready = context.ready(StoredRuntimeValue::test_int(7.into()));
        drop(execution);
        assert_eq!(
            pin!(ready.observe()).as_mut().poll(&mut cx).map(|value| {
                value.expect("ready work").read(|value| {
                    value
                        .as_ref()
                        .map(|value| value.read(|value| value.value().clone()))
                        .map_err(|_| "ready failure")
                })
            }),
            Poll::Ready(Ok(EvaluatedValue::Int(7.into())))
        );
        assert!(pin!(forward.observe()).as_mut().poll(&mut cx).is_ready());
        assert_eq!(polls.load(Ordering::SeqCst), 1);
    }
}
