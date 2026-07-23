mod bit_array;
mod environment;
mod instruction;
mod pattern;
mod terminator;
mod value;

pub(super) use environment::RetainedValues;
pub(super) use value::GraphValue;

use self::environment::BlockEnvironment;
use self::terminator::{GraphAction, NeverCall, terminator_action};
use crate::plan::execution::ExecutionPlan;
use crate::plan::execution::graph::{BlockGraph, BlockGraphExitId, ParamLocal};
use crate::runtime::error::ExecutionResult;
use crate::runtime::state::RuntimeState;

pub(super) struct CompletedGraph {
    exit: BlockGraphExitId,
    environment: BlockEnvironment,
}

impl CompletedGraph {
    pub(super) fn exit(&self) -> BlockGraphExitId {
        self.exit
    }

    pub(super) fn into_value<Value>(
        self,
        state: &mut RuntimeState,
        value: &Value,
    ) -> Value::Evaluated
    where
        Value: GraphValue,
    {
        let value = value.read(&self);
        drop(self.environment);
        state.drain_releases();
        value
    }

    pub(super) fn into_retained(
        self,
        state: &mut RuntimeState,
        values: &[ParamLocal],
    ) -> RetainedValues {
        let retained = self.environment.retain(values);
        drop(self.environment);
        state.drain_releases();
        retained
    }
}

pub(super) fn execute(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
    graph: &BlockGraph,
    inputs: RetainedValues,
) -> ExecutionResult<CompletedGraph> {
    let mut block_id = graph.entry();
    let mut environment = BlockEnvironment::from_retained(inputs);

    loop {
        let block = graph.block(block_id);
        for instruction in block.instructions() {
            instruction::execute(plan, state, &mut environment, instruction)?;
        }

        match terminator_action(plan, state, &environment, block.terminator())? {
            GraphAction::Continue { block, inputs } => {
                drop(environment);
                state.drain_releases();
                block_id = block;
                environment = BlockEnvironment::from_retained(inputs);
            }
            GraphAction::Exit(exit) => return Ok(CompletedGraph { exit, environment }),
            GraphAction::NeverCall { function, inputs } => {
                drop(environment);
                state.drain_releases();
                return match function {
                    NeverCall::Direct(function) => {
                        crate::runtime::function::run_never(plan, state, function, inputs)
                            .map(|never| match never {})
                    }
                    NeverCall::Value(function) => {
                        crate::runtime::function::run_never_value(plan, state, function, inputs)
                            .map(|never| match never {})
                    }
                };
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::RetainedValues;
    use crate::ValueType;
    use crate::plan::execution::function::IntFunctionId;
    use crate::runtime::error::InvariantError;
    use crate::runtime::evaluated::{EvaluatedCustomValue, EvaluatedValue};
    use crate::runtime::state::RuntimeState;
    use crate::runtime::{ExecutionError, Value};

    #[test]
    fn deeply_nested_intra_function_control_flow_runs_iteratively() {
        let mut body = "1".to_string();
        for _ in 0..512 {
            body = format!("case flag {{ True -> {body} False -> 0 }}");
        }
        let source =
            format!("fn deep(flag: Bool) -> Int {{ {body} }} pub fn main() {{ deep(True) }}");
        let plan = std::thread::Builder::new()
            .stack_size(32 * 1024 * 1024)
            .spawn(move || {
                let module = crate::compile_typed_module("main", "main.gleam", source.as_str())
                    .expect("deep source should compile");
                let module_plan = crate::plan_module(module).expect("deep source should plan");
                crate::ExecutionPlan::from_module_plan(module_plan)
            })
            .expect("deep-plan lowering thread should start")
            .join()
            .expect("deep-plan lowering should complete");

        assert_eq!(crate::run_main(&plan), Ok(Value::Int(1.into())));
    }

    #[test]
    fn match_terminator_propagates_custom_field_family_corruption() {
        let plan = execution_plan(
            r#"
pub type Boxed {
  Boxed(Int)
  Empty
}

fn read(value: Boxed) {
  let assert Boxed(inner) = value
  inner
}

pub fn main() {
  read(Boxed(1))
}
"#,
        );
        assert_eq!(crate::run_main(&plan), Ok(Value::Int(1.into())));

        let constructor = plan.custom_constructor_id(0, 0);
        let descriptor = plan.custom_constructor(constructor);
        let malformed = EvaluatedCustomValue::from_fields(
            constructor,
            vec![EvaluatedValue::String("wrong".into())].into_boxed_slice(),
        );
        let mut inputs = RetainedValues::empty();
        inputs.push_evaluated(EvaluatedValue::Custom(malformed));

        assert_eq!(
            super::execute(
                &plan,
                &mut RuntimeState::new(),
                plan.int_function(IntFunctionId(1)).body().block_graph(),
                inputs,
            )
            .map(|_| ()),
            Err(ExecutionError::Invariant(
                InvariantError::CustomFieldFamilyMismatch {
                    custom_type: plan.custom_value_type(constructor.type_id()),
                    constructor: descriptor.name().clone(),
                    field_index: 0,
                    expected: ValueType::Int,
                    actual: ValueType::String,
                },
            )),
        );
    }

    fn execution_plan(source: &str) -> crate::ExecutionPlan {
        let module = crate::compile_typed_module("main", "main.gleam", source)
            .expect("source should compile");
        let module_plan = crate::plan_module(module).expect("source should plan");
        crate::ExecutionPlan::from_module_plan(module_plan)
    }
}
