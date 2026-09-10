use super::EntryTarget;
use crate::plan::execution::ExecutionPlan;
use crate::plan::execution::function::{ExecutionFunctionRef, FunctionBodyOwner, FunctionExit};
use crate::runtime::ExecutableRuntimePlan;
use crate::runtime::error::{ExecutionResult, HostCallOrigin};
use crate::runtime::execution::{Evaluation, ServiceContext, Yield};
use crate::runtime::graph::RuntimeGraphState;
use crate::runtime::graph::{GraphExecution, GraphProgress, GraphValue, RetainedValues, Returns};
use crate::runtime::state::RuntimeState;
use std::num::NonZeroUsize;

pub(in crate::runtime) struct Execution<'plan, Plan: ExecutableRuntimePlan, Id: EntryTarget<Plan>> {
    function: Id,
    position: Position<'plan, Plan, Id>,
    returns: Box<Returns<'plan, Plan>>,
}

pub(in crate::runtime) enum Progress<
    'plan,
    Plan: ExecutableRuntimePlan + 'plan,
    Id: EntryTarget<Plan> + 'plan,
> {
    Continue(Execution<'plan, Plan, Id>),
    Host(Plan::HostInvocation<'plan, Progress<'plan, Plan, Id>>),
    Complete(EntryValue<Plan, Id>),
}

pub(in crate::runtime) type EntryValue<Plan, Id> =
    <<<Id as EntryTarget<Plan>>::Body as FunctionBodyOwner>::Return as GraphValue>::Evaluated;

enum Position<'plan, Plan: ExecutableRuntimePlan, Id: EntryTarget<Plan>> {
    Entry {
        origin: HostCallOrigin,
        inputs: RetainedValues,
    },
    Graph {
        body: &'plan Id::Body,
        execution: GraphExecution<'plan, Plan>,
    },
}

pub(super) fn run<'plan, Id>(
    plan: &'plan ExecutionPlan,
    state: &mut RuntimeState<'_>,
    function: Id,
    origin: HostCallOrigin,
    inputs: RetainedValues,
) -> ExecutionResult<EntryValue<ExecutionPlan, Id>>
where
    Id: EntryTarget<ExecutionPlan> + 'plan,
    Id::Body: 'plan,
    Id::HostTarget: 'plan,
{
    let mut progress = Progress::Continue(Execution::new(function, origin, inputs));
    loop {
        progress = match progress {
            Progress::Continue(execution) => execution.advance(plan, state, NonZeroUsize::MAX)?,
            Progress::Host(invoke) => match invoke {},
            Progress::Complete(value) => return Ok(value),
        };
    }
}

impl<'plan, Plan, Id> Execution<'plan, Plan, Id>
where
    Plan: ExecutableRuntimePlan,
    Id: EntryTarget<Plan> + 'plan,
    Id::Body: 'plan,
    Id::HostTarget: 'plan,
{
    pub(in crate::runtime) fn new(
        function: Id,
        origin: HostCallOrigin,
        inputs: RetainedValues,
    ) -> Self {
        Self {
            function,
            position: Position::Entry { origin, inputs },
            returns: Box::new(Returns::new()),
        }
    }

    pub(in crate::runtime) fn step(
        self,
        plan: &'plan Plan,
        state: &mut impl RuntimeGraphState<Error = crate::ExecutionError>,
    ) -> ExecutionResult<Progress<'plan, Plan, Id>> {
        let Self {
            function,
            position,
            mut returns,
        } = self;
        match position {
            Position::Entry { origin, inputs } => match function.entry(plan) {
                ExecutionFunctionRef::Graph(entry) => Ok(Progress::Continue(Self {
                    function,
                    returns,
                    position: Position::Graph {
                        body: entry.body(),
                        execution: GraphExecution::new(
                            entry.body().function_body().block_graph(),
                            inputs,
                        ),
                    },
                })),
                ExecutionFunctionRef::Host(target) => Ok(Progress::Host(Plan::map_host(
                    Id::prepare_host(plan, origin, target, inputs),
                    |value| Ok(Progress::Complete(value)),
                ))),
            },
            Position::Graph { body, execution } => {
                match execution.step(plan, state, &mut returns)? {
                    GraphProgress::Host(invoke) => {
                        Ok(Progress::Host(Plan::map_host(invoke, move |execution| {
                            Ok(Progress::Continue(Self {
                                function,
                                position: Position::Graph { body, execution },
                                returns,
                            }))
                        })))
                    }
                    GraphProgress::Continue(next) => Ok(Progress::Continue(Self {
                        function,
                        returns,
                        position: Position::Graph {
                            body,
                            execution: next,
                        },
                    })),
                    GraphProgress::Complete(completed) => {
                        Ok(match body.function_body().exit(completed.exit()) {
                            FunctionExit::Return(value) => {
                                Progress::Complete(completed.into_value(value))
                            }
                            FunctionExit::TailCall {
                                function: target,
                                args,
                            } => {
                                let (function, origin) = function.next(target);
                                Progress::Continue(Self {
                                    function,
                                    position: Position::Entry {
                                        origin,
                                        inputs: completed.into_retained(args),
                                    },
                                    returns,
                                })
                            }
                        })
                    }
                }
            }
        }
    }

    pub(in crate::runtime) fn advance(
        mut self,
        plan: &'plan Plan,
        state: &mut impl RuntimeGraphState<Error = crate::ExecutionError>,
        budget: NonZeroUsize,
    ) -> ExecutionResult<Progress<'plan, Plan, Id>> {
        for _ in 0..budget.get() {
            match self.step(plan, state)? {
                Progress::Continue(next) => self = next,
                host @ Progress::Host(_) => return Ok(host),
                complete @ Progress::Complete(_) => return Ok(complete),
            }
        }
        Ok(Progress::Continue(self))
    }

    pub(in crate::runtime) async fn drive(
        self,
        plan: &'plan Plan,
        context: &ServiceContext<Plan>,
        budget: NonZeroUsize,
    ) -> Result<ExecutionResult<EntryValue<Plan, Id>>, crate::runtime::work::Cancelled> {
        let mut evaluation = Evaluation::default();
        let mut progress = Ok(Progress::Continue(self));
        loop {
            progress = match progress {
                Ok(Progress::Continue(execution)) => {
                    let result = evaluation.access(|state| execution.advance(plan, state, budget));
                    let echo = evaluation.take_echo();
                    if !echo.is_empty() {
                        context
                            .submit(move |_, state| {
                                for output in echo {
                                    state.emit_echo(output);
                                }
                            })
                            .await?;
                    }
                    if matches!(result, Ok(Progress::Continue(_))) {
                        Yield::new().await;
                    }
                    result
                }
                Ok(Progress::Host(invoke)) => Plan::submit_host(invoke, context, budget).await?,
                Ok(Progress::Complete(value)) => return Ok(Ok(value)),
                Err(error) => return Ok(Err(error)),
            };
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Execution, Progress};
    use crate::ExecutionPlan;
    use crate::plan::execution::function::IntFunctionId;
    use crate::runtime::error::HostCallOrigin;
    use crate::runtime::graph::RetainedValues;
    use crate::runtime::state::RuntimeState;
    use std::num::NonZeroUsize;

    #[test]
    fn finite_entry_completes_with_unused_budget() {
        let plan = crate::runtime::plan_src("pub fn main() { 42 }");
        let execution = Execution::new(
            IntFunctionId(0),
            HostCallOrigin::Entry,
            RetainedValues::empty(),
        );
        let mut echo = Vec::new();
        let mut state = RuntimeState::new(&mut echo);
        let result = execution.advance(&plan, &mut state, NonZeroUsize::new(100).unwrap());
        assert!(matches!(result, Ok(Progress::Complete(value)) if value == 42.into()));
    }

    #[test]
    fn tail_recursive_entries_share_bounded_turns_and_release_independently() {
        let plan = crate::runtime::plan_src(
            r#"
fn count(value: Int) -> Int {
  echo value
  count(value + 1)
}
pub fn main() { count(0) }
"#,
        );
        let mut left = Execution::new(
            IntFunctionId(0),
            HostCallOrigin::Entry,
            RetainedValues::empty(),
        );
        let mut right = Execution::new(
            IntFunctionId(0),
            HostCallOrigin::Entry,
            RetainedValues::empty(),
        );
        let left_storage = std::ptr::from_ref(left.returns.as_ref());
        let right_storage = std::ptr::from_ref(right.returns.as_ref());
        let budget = NonZeroUsize::new(29).unwrap();
        let mut left_echo = Vec::new();
        let mut right_echo = Vec::new();
        for _ in 0..100 {
            let mut state = RuntimeState::new(&mut left_echo);
            left = continuing(left.advance(&plan, &mut state, budget).expect("left turn"));
            let mut state = RuntimeState::new(&mut right_echo);
            right = continuing(
                right
                    .advance(&plan, &mut state, budget)
                    .expect("right turn"),
            );
        }
        assert_eq!(std::ptr::from_ref(left.returns.as_ref()), left_storage);
        assert_eq!(std::ptr::from_ref(right.returns.as_ref()), right_storage);
        assert!(left_echo.len() > 100);
        assert_eq!(left_echo.len(), right_echo.len());
        for (index, output) in left_echo.iter().enumerate() {
            assert_eq!(output.value().inspect().to_string(), index.to_string());
        }
        drop(left);
        let previous = right_echo.len();
        let mut state = RuntimeState::new(&mut right_echo);
        let progress = right
            .advance(&plan, &mut state, budget)
            .expect("surviving turn");
        drop(continuing(progress));
        assert!(right_echo.len() > previous);
    }

    #[test]
    #[should_panic(expected = "fixture entry must still be running")]
    fn continuing_guard_rejects_a_completed_source_entry() {
        let plan = crate::runtime::plan_src("pub fn main() { 42 }");
        let execution = Execution::new(
            IntFunctionId(0),
            HostCallOrigin::Entry,
            RetainedValues::empty(),
        );
        let mut echo = Vec::new();
        let mut state = RuntimeState::new(&mut echo);
        continuing(
            execution
                .advance(&plan, &mut state, NonZeroUsize::MAX)
                .unwrap(),
        );
    }

    fn continuing(
        progress: Progress<'_, ExecutionPlan, IntFunctionId>,
    ) -> Execution<'_, ExecutionPlan, IntFunctionId> {
        match progress {
            Progress::Continue(next) => next,
            Progress::Complete(_) => panic!("fixture entry must still be running"),
            Progress::Host(invoke) => match invoke {},
        }
    }
}
