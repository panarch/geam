use super::ServiceContext;
use crate::runtime::ExecutableRuntimePlan;
use crate::runtime::error::ExecutionResult;
use crate::runtime::function::{EntryTarget, EntryValue, Execution};
use crate::runtime::state::RuntimeStateFor;
use crate::runtime::work::Cancelled;
use crate::runtime::{HostCallOrigin, RetainedValues};
use std::future::Future;
use std::num::NonZeroUsize;
use std::pin::Pin;

pub(in crate::runtime) struct Invocation<'plan, Plan: ExecutableRuntimePlan, Output> {
    inner: Box<dyn Invoke<'plan, Plan, Output> + 'plan>,
}

pub(in crate::runtime) type Waiting<'plan, Output> =
    Pin<Box<dyn Future<Output = Result<ExecutionResult<Output>, Cancelled>> + Send + 'plan>>;

trait Invoke<'plan, Plan: ExecutableRuntimePlan, Output>: Send {
    fn submit(
        self: Box<Self>,
        context: &ServiceContext<Plan>,
        budget: NonZeroUsize,
    ) -> Waiting<'plan, Output>;
}

pub(in crate::runtime) enum NativeReturn<Output> {
    Immediate(Output),
    Continuing(Waiting<'static, Output>),
}

struct Operation<Function>(Function);
struct Ready<Output>(Output);
struct SourceExecution<'plan, Plan: ExecutableRuntimePlan, Id: EntryTarget<Plan>> {
    plan: &'plan Plan,
    execution: Execution<'plan, Plan, Id>,
}
struct Mapped<'plan, Plan: ExecutableRuntimePlan, Input, Map> {
    input: Invocation<'plan, Plan, Input>,
    map: Map,
}

impl<'plan, Plan: ExecutableRuntimePlan + 'plan, Output: Send + 'plan>
    Invocation<'plan, Plan, Output>
{
    pub(in crate::runtime) fn ready(value: Output) -> Self {
        Self {
            inner: Box::new(Ready(value)),
        }
    }
    pub(in crate::runtime) fn execution<Id>(
        plan: &'plan Plan,
        function: Id,
        origin: HostCallOrigin,
        inputs: RetainedValues,
    ) -> Self
    where
        Id: EntryTarget<Plan> + 'plan,
        <Id::Body as crate::plan::execution::function::FunctionBodyOwner>::Return:
            crate::runtime::graph::GraphValue<Evaluated = Output>,
    {
        Self {
            inner: Box::new(SourceExecution {
                plan,
                execution: Execution::new(function, origin, inputs),
            }),
        }
    }
    pub(in crate::runtime) fn new(
        function: impl FnOnce(
            &Plan,
            &mut RuntimeStateFor<'_, Plan>,
        ) -> ExecutionResult<NativeReturn<Output>>
        + Send
        + 'static,
    ) -> Self
    where
        Output: 'static,
    {
        Self {
            inner: Box::new(Operation(function)),
        }
    }

    pub(in crate::runtime) fn map<MappedOutput: Send + 'plan>(
        self,
        map: impl FnOnce(Output) -> ExecutionResult<MappedOutput> + Send + 'plan,
    ) -> Invocation<'plan, Plan, MappedOutput> {
        Invocation {
            inner: Box::new(Mapped { input: self, map }),
        }
    }

    pub(in crate::runtime) fn submit(
        self,
        context: &ServiceContext<Plan>,
        budget: NonZeroUsize,
    ) -> Waiting<'plan, Output> {
        self.inner.submit(context, budget)
    }
}

impl<'plan, Plan: ExecutableRuntimePlan, Output: Send + 'plan> Invoke<'plan, Plan, Output>
    for Ready<Output>
{
    fn submit(
        self: Box<Self>,
        _context: &ServiceContext<Plan>,
        _budget: NonZeroUsize,
    ) -> Waiting<'plan, Output> {
        Box::pin(std::future::ready(Ok(Ok(self.0))))
    }
}

impl<'plan, Plan, Id> Invoke<'plan, Plan, EntryValue<Plan, Id>> for SourceExecution<'plan, Plan, Id>
where
    Plan: ExecutableRuntimePlan + 'plan,
    Id: EntryTarget<Plan> + 'plan,
{
    fn submit(
        self: Box<Self>,
        context: &ServiceContext<Plan>,
        budget: NonZeroUsize,
    ) -> Waiting<'plan, EntryValue<Plan, Id>> {
        let context = context.clone();
        Box::pin(async move { self.execution.drive(self.plan, &context, budget).await })
    }
}

impl<'plan, Plan, Output, Function> Invoke<'plan, Plan, Output> for Operation<Function>
where
    Plan: ExecutableRuntimePlan + 'plan,
    Output: Send + 'static,
    Function: FnOnce(&Plan, &mut RuntimeStateFor<'_, Plan>) -> ExecutionResult<NativeReturn<Output>>
        + Send
        + 'static,
{
    fn submit(
        self: Box<Self>,
        context: &ServiceContext<Plan>,
        _budget: NonZeroUsize,
    ) -> Waiting<'plan, Output> {
        let request = context.submit(self.0);
        Box::pin(async move {
            match request.await? {
                Ok(NativeReturn::Immediate(value)) => Ok(Ok(value)),
                Ok(NativeReturn::Continuing(operation)) => operation.await,
                Err(error) => Ok(Err(error)),
            }
        })
    }
}

impl<'plan, Plan, Input, Output, Map> Invoke<'plan, Plan, Output>
    for Mapped<'plan, Plan, Input, Map>
where
    Plan: ExecutableRuntimePlan + 'plan,
    Input: Send + 'plan,
    Output: Send + 'plan,
    Map: FnOnce(Input) -> ExecutionResult<Output> + Send + 'plan,
{
    fn submit(
        self: Box<Self>,
        context: &ServiceContext<Plan>,
        budget: NonZeroUsize,
    ) -> Waiting<'plan, Output> {
        let Self { input, map } = *self;
        let waiting = input.submit(context, budget);
        Box::pin(async move { Ok(waiting.await?.and_then(map)) })
    }
}

#[cfg(test)]
mod tests {
    use super::{Invocation, NativeReturn};
    use crate::runtime::execution::Services;
    use crate::runtime::state::RuntimeState;
    use crate::runtime::work::Cancelled;
    use std::num::NonZeroUsize;
    use std::task::{Context, Poll, Waker};

    #[test]
    fn native_operations_preserve_completion_failure_and_request_cancellation() {
        enum Completion {
            Immediate,
            Continuing,
            Failure,
        }

        let plan = crate::runtime::plan_src("pub fn main() { 42 }");
        let source_error =
            crate::runtime::run_src_error("pub fn main() { panic as \"callback stopped\" }");
        for completion in [
            Completion::Immediate,
            Completion::Continuing,
            Completion::Failure,
        ] {
            for cancelled in [false, true] {
                let services = Services::new();
                let context = services.context();
                let error = source_error.clone();
                let outcome = match completion {
                    Completion::Immediate => Ok(NativeReturn::Immediate(42)),
                    Completion::Continuing => Ok(NativeReturn::Continuing(Box::pin(
                        std::future::ready(Ok(Ok(42))),
                    ))),
                    Completion::Failure => Err(error.clone()),
                };
                let expected = match completion {
                    Completion::Immediate | Completion::Continuing => Ok(42),
                    Completion::Failure => Err(error),
                };
                let invocation = Invocation::new(move |_, _| outcome);
                let mut waiting = invocation.submit(&context, NonZeroUsize::MIN);
                let mut cx = Context::from_waker(Waker::noop());
                assert!(waiting.as_mut().poll(&mut cx).is_pending());
                let mut echo = Vec::new();
                let mut state = RuntimeState::new(&mut echo);
                if cancelled {
                    services.close();
                    assert_eq!(waiting.as_mut().poll(&mut cx), Poll::Ready(Err(Cancelled)));
                } else {
                    services
                        .next(&mut cx)
                        .expect("queued native operation")
                        .service(&plan, &mut state)
                        .expect("live observer")
                        .deliver();
                    assert_eq!(waiting.as_mut().poll(&mut cx), Poll::Ready(Ok(expected)));
                }
                assert!(services.next(&mut cx).is_none());
                assert!(echo.is_empty());
            }
        }
    }
}
