use super::EntryTarget;
use crate::plan::execution::ExecutionPlan;
use crate::plan::execution::function::{ExecutionFunctionRef, FunctionBodyOwner, FunctionExit};
use crate::runtime::ExecutableRuntimePlan;
use crate::runtime::error::{ExecutionResult, HostCallOrigin};
use crate::runtime::execution::{Evaluation, ServiceContext, Yield};
use crate::runtime::graph::RuntimeGraphState;
use crate::runtime::graph::{
    GraphExecution, GraphProgress, GraphStorage, GraphValue, RetainedValues,
};
use crate::runtime::state::RuntimeState;
use std::num::NonZeroUsize;

pub(in crate::runtime) struct Execution<'plan, Plan: ExecutableRuntimePlan, Id: EntryTarget<Plan>> {
    function: Id,
    position: Position<'plan, Plan, Id>,
    storage: Box<GraphStorage<'plan, Plan>>,
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
            storage: Box::new(GraphStorage::new()),
        }
    }

    pub(in crate::runtime) async fn drive(
        self,
        plan: &'plan Plan,
        context: &ServiceContext<Plan>,
        budget: NonZeroUsize,
    ) -> Result<ExecutionResult<EntryValue<Plan, Id>>, crate::runtime::work::Cancelled> {
        let mut evaluation = Evaluation::new(context.captures().clone());
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

    pub(in crate::runtime) fn advance(
        mut self,
        plan: &'plan Plan,
        state: &mut impl RuntimeGraphState<Error = crate::ExecutionError>,
        budget: NonZeroUsize,
    ) -> ExecutionResult<Progress<'plan, Plan, Id>> {
        let mut remaining = budget.get();
        while remaining > 0 {
            remaining -= 1;
            match self.advance_position(plan, state, &mut remaining)? {
                Progress::Continue(next) => self = next,
                host @ Progress::Host(_) => return Ok(host),
                complete @ Progress::Complete(_) => return Ok(complete),
            }
        }
        Ok(Progress::Continue(self))
    }

    fn advance_position(
        self,
        plan: &'plan Plan,
        state: &mut impl RuntimeGraphState<Error = crate::ExecutionError>,
        remaining: &mut usize,
    ) -> ExecutionResult<Progress<'plan, Plan, Id>> {
        let Self {
            function,
            position,
            mut storage,
        } = self;
        match position {
            Position::Entry { origin, inputs } => {
                if let Some(cancelled) =
                    plan.reject_foreign_callable(&inputs, Some(state.captures().domain()))
                {
                    return Ok(Progress::Host(cancelled));
                }
                match function.entry(plan) {
                    ExecutionFunctionRef::Graph(entry) => Ok(Progress::Continue(Self {
                        function,
                        storage,
                        position: Position::Graph {
                            body: entry.body(),
                            execution: GraphExecution::new(
                                entry.body().function_body().block_graph().as_view(),
                                inputs,
                            ),
                        },
                    })),
                    ExecutionFunctionRef::Host(target) => Ok(Progress::Host(Plan::map_host(
                        Id::prepare_host(plan, origin, target, inputs),
                        |value| Ok(Progress::Complete(value)),
                    ))),
                }
            }
            Position::Graph { body, execution } => {
                match execution.advance(plan, state, &mut storage, remaining)? {
                    GraphProgress::Host(invoke) => {
                        Ok(Progress::Host(Plan::map_host(invoke, move |execution| {
                            Ok(Progress::Continue(Self {
                                function,
                                position: Position::Graph { body, execution },
                                storage,
                            }))
                        })))
                    }
                    GraphProgress::Continue(next) => Ok(Progress::Continue(Self {
                        function,
                        storage,
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
                                transfer,
                                ..
                            } => {
                                let (function, origin) = function.next(target);
                                Progress::Continue(Self {
                                    function,
                                    position: Position::Entry {
                                        origin,
                                        inputs: completed.into_retained(transfer),
                                    },
                                    storage,
                                })
                            }
                        })
                    }
                }
            }
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
    fn pure_self_loops_yield_at_root_and_nested_entries_without_effects() {
        for source in [
            "pub fn main() -> Int { main() }",
            "fn spin() -> Int { spin() } pub fn main() { spin() + 1 }",
            "pub fn main() -> Int { case 1 < 2 { True -> main() False -> 0 } }",
            "fn spin(value: Int) { case value >= 0 { True -> spin(value) False -> value } } pub fn main() { spin(1) + 1 }",
        ] {
            let plan = crate::runtime::plan_src(source);
            for budget in [1, 2, 7, 1024] {
                let mut execution = Execution::new(
                    IntFunctionId(0),
                    HostCallOrigin::Entry,
                    RetainedValues::empty(),
                );
                let mut echo = Vec::new();
                for _ in 0..8 {
                    execution = continuing(
                        execution
                            .advance(
                                &plan,
                                &mut RuntimeState::new(&mut echo),
                                NonZeroUsize::new(budget).unwrap(),
                            )
                            .unwrap(),
                    );
                }
                drop(execution);
                assert!(echo.is_empty());
            }
        }
    }

    #[test]
    fn bounded_calls_preserve_exact_echo_and_completion_turns() {
        let plan = crate::runtime::plan_src(
            r#"
const adjustment = 2

fn apply(value, callback) { callback(value) + adjustment }

fn walk(remaining, total, callback) {
  case remaining {
    0 -> total
    n -> {
      let value = apply(n, callback)
      walk(n - 1, total + value, callback)
    }
  }
}

pub fn main() {
  let offset = 5
  walk(12, 0, fn(value) {
    echo value
    value + offset
  })
}
"#,
        );
        // Twelve self tails use entry jumps instead of two additional root
        // exit/entry steps each: 179 steps, first echo at 12, then fourteen apart.
        for (budget, turns, echo_turns) in [
            (
                1,
                179,
                [12, 26, 40, 54, 68, 82, 96, 110, 124, 138, 152, 166],
            ),
            (2, 90, [6, 13, 20, 27, 34, 41, 48, 55, 62, 69, 76, 83]),
            (3, 60, [4, 9, 14, 18, 23, 28, 32, 37, 42, 46, 51, 56]),
            (7, 26, [2, 4, 6, 8, 10, 12, 14, 16, 18, 20, 22, 24]),
            (29, 7, [1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6]),
            (128, 2, [1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 2, 2]),
            (179, 1, [1; 12]),
            (1024, 1, [1; 12]),
            (usize::MAX, 1, [1; 12]),
        ] {
            let trace = trace_int_turns(&plan, NonZeroUsize::new(budget).unwrap());
            assert_eq!(trace.turns, turns, "budget {budget}");
            assert_eq!(trace.result, Ok(162.into()), "budget {budget}");
            assert_eq!(
                trace.echo,
                echo_turns
                    .into_iter()
                    .zip([
                        "12", "11", "10", "9", "8", "7", "6", "5", "4", "3", "2", "1"
                    ])
                    .map(|(turn, value)| (turn, value.to_owned()))
                    .collect::<Vec<_>>(),
                "budget {budget}",
            );
        }
    }

    #[test]
    fn a_source_panic_preserves_its_exact_turn_echo_and_origin() {
        let plan = crate::runtime::plan_src(
            r#"
fn walk(remaining) {
  echo remaining
  case remaining {
    0 -> panic as "probe stop"
    n -> walk(n - 1)
  }
}

pub fn main() { walk(5) + 1 }
"#,
        );
        // Five literal loads disappear before the panic. Echo steps are
        // 4, 8, 12, 16, 20, 24 and the source stop is reached at step 27.
        for (budget, turns, echo_turns) in [
            (1, 27, [4, 8, 12, 16, 20, 24]),
            (2, 14, [2, 4, 6, 8, 10, 12]),
            (3, 9, [2, 3, 4, 6, 7, 8]),
            (7, 4, [1, 2, 2, 3, 3, 4]),
            (27, 1, [1, 1, 1, 1, 1, 1]),
            (29, 1, [1, 1, 1, 1, 1, 1]),
            (128, 1, [1, 1, 1, 1, 1, 1]),
            (1024, 1, [1, 1, 1, 1, 1, 1]),
            (usize::MAX, 1, [1; 6]),
        ] {
            let trace = trace_int_turns(&plan, NonZeroUsize::new(budget).unwrap());
            assert_eq!(trace.turns, turns, "budget {budget}");
            assert_eq!(
                trace.echo,
                echo_turns
                    .into_iter()
                    .zip(["5", "4", "3", "2", "1", "0"])
                    .map(|(turn, value)| (turn, value.to_owned()))
                    .collect::<Vec<_>>(),
                "budget {budget}",
            );
            let error = trace.result.unwrap_err().into_materialized();
            assert_eq!(
                format!("{error:?}"),
                r#"Panic(Panic { kind: Panic, message: Explicit("probe stop"), site: PanicSite { module: "main", function: "walk", span: SourceSpan { start: 67, end: 88 } }, source: None, details: None })"#,
            );
        }
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
        let left_storage = std::ptr::from_ref(left.storage.as_ref());
        let right_storage = std::ptr::from_ref(right.storage.as_ref());
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
        assert_eq!(std::ptr::from_ref(left.storage.as_ref()), left_storage);
        assert_eq!(std::ptr::from_ref(right.storage.as_ref()), right_storage);
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
    fn graph_storage_survives_root_tail_calls_nested_calls_and_independent_yields() {
        let plan = crate::runtime::plan_src(
            r#"
fn ordinary(value, depth) {
  let assert <<first, rest:bits>> = value
  case depth {
    0 -> first
    _ -> ordinary(value, depth - 1) + first
  }
}
fn left(values: List(Int), count: Int) -> Int {
  let assert [head, ..tail] = values
  echo ordinary(<<head, 2>>, 8)
  right([head, ..tail], count + 1)
}
fn right(values: List(Int), count: Int) -> Int {
  let assert [head, ..tail] = values
  echo count
  left([head, ..tail], count)
}
pub fn main() { left([1, 2, 3], 0) }
"#,
        );
        let mut first = Execution::new(
            IntFunctionId(0),
            HostCallOrigin::Entry,
            RetainedValues::empty(),
        );
        let mut second = Execution::new(
            IntFunctionId(0),
            HostCallOrigin::Entry,
            RetainedValues::empty(),
        );
        let mut first_echo = Vec::new();
        let mut second_echo = Vec::new();
        // Enter both source executions with a fixed number of one-step turns.
        for _ in 0..32 {
            first = continuing(
                first
                    .advance(
                        &plan,
                        &mut RuntimeState::new(&mut first_echo),
                        NonZeroUsize::MIN,
                    )
                    .unwrap(),
            );
            second = continuing(
                second
                    .advance(
                        &plan,
                        &mut RuntimeState::new(&mut second_echo),
                        NonZeroUsize::MIN,
                    )
                    .unwrap(),
            );
        }
        let first_buffer = &*first.storage as *const _;
        let second_buffer = &*second.storage as *const _;
        assert_ne!(first_buffer, second_buffer);
        for _ in 0..4_000 {
            first = continuing(
                first
                    .advance(
                        &plan,
                        &mut RuntimeState::new(&mut first_echo),
                        NonZeroUsize::MIN,
                    )
                    .unwrap(),
            );
            second = continuing(
                second
                    .advance(
                        &plan,
                        &mut RuntimeState::new(&mut second_echo),
                        NonZeroUsize::MIN,
                    )
                    .unwrap(),
            );

            assert_eq!(&*first.storage as *const _, first_buffer);
            assert_eq!(&*second.storage as *const _, second_buffer);
        }
        assert!(first_echo.len() > 10);
        assert_eq!(first_echo.len(), second_echo.len());
        for (index, pair) in first_echo.chunks_exact(2).enumerate() {
            assert_eq!(pair[0].value().inspect().to_string(), "9");
            assert_eq!(
                pair[1].value().inspect().to_string(),
                (index + 1).to_string()
            );
        }
        drop(first);
        second = continuing(
            second
                .advance(
                    &plan,
                    &mut RuntimeState::new(&mut second_echo),
                    NonZeroUsize::MIN,
                )
                .unwrap(),
        );
        assert_eq!(&*second.storage as *const _, second_buffer);
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

    struct TurnTrace {
        turns: usize,
        echo: Vec<(usize, String)>,
        result: crate::runtime::error::ExecutionResult<num_bigint::BigInt>,
    }

    fn trace_int_turns(plan: &ExecutionPlan, budget: NonZeroUsize) -> TurnTrace {
        let mut execution = Execution::new(
            IntFunctionId(0),
            HostCallOrigin::Entry,
            RetainedValues::empty(),
        );
        let mut echo = Vec::new();
        let mut turns = 0;
        let mut observed = Vec::new();
        loop {
            turns += 1;
            let result = execution.advance(plan, &mut RuntimeState::new(&mut echo), budget);
            observed.extend(
                echo.drain(..)
                    .map(|output| (turns, output.value().inspect().to_string())),
            );
            let result = match result {
                Ok(Progress::Continue(next)) => {
                    execution = next;
                    continue;
                }
                Ok(Progress::Host(invoke)) => match invoke {},
                Ok(Progress::Complete(value)) => Ok(value),
                Err(error) => Err(error),
            };
            return TurnTrace {
                turns,
                echo: observed,
                result,
            };
        }
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
