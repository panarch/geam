mod activation;
mod bit_array;
mod environment;
mod instruction;
mod pattern;
mod terminator;
mod value;

pub(crate) use environment::RetainedValues;
pub(super) use value::GraphValue;

pub(in crate::runtime) use self::environment::BlockEnvironment;
pub(in crate::runtime) use self::terminator::RuntimeGraphState;
use crate::plan::execution::graph::{BlockGraphExitId, BlockId, ParamLocal};
use crate::runtime::ExecutableRuntimePlan;
use crate::runtime::error::ExecutionResult;
pub(in crate::runtime) use activation::{Activation, Frame, Returns};
pub(in crate::runtime) use activation::{Execution as GraphExecution, Progress as GraphProgress};

struct GraphPosition {
    block: BlockId,
    instruction: usize,
    environment: BlockEnvironment,
}

impl GraphPosition {
    fn new(entry: BlockId, inputs: RetainedValues) -> Self {
        Self {
            block: entry,
            instruction: 0,
            environment: BlockEnvironment::from_retained(inputs),
        }
    }
}

pub(in crate::runtime) struct CompletedGraph {
    exit: BlockGraphExitId,
    environment: BlockEnvironment,
}

impl CompletedGraph {
    pub(in crate::runtime) fn exit(&self) -> BlockGraphExitId {
        self.exit
    }

    pub(in crate::runtime) fn into_value<Value>(self, value: &Value) -> Value::Evaluated
    where
        Value: GraphValue,
    {
        let value = value.read(&self.environment);
        drop(self.environment);
        value
    }

    pub(in crate::runtime) fn into_retained(self, values: &[ParamLocal]) -> RetainedValues {
        let retained = self.environment.retain(values);
        drop(self.environment);
        retained
    }
}

pub(in crate::runtime) fn advance_external_list_instruction<'plan, Plan>(
    plan: &'plan Plan,
    state: &mut impl RuntimeGraphState<Error = crate::ExecutionError>,
    frame: Frame<'plan, Plan>,
    returns: &mut Returns<'plan, Plan>,
    instruction: &crate::plan::execution::graph::ExternalListInstruction,
    expected: &crate::plan::ValueType,
) -> ExecutionResult<Activation<'plan, Plan>>
where
    Plan: ExecutableRuntimePlan<Profile = crate::plan::execution::host::HostedExecutionProfile>,
{
    instruction::advance_external_list(plan, state, frame, returns, instruction, expected)
}

pub(in crate::runtime) fn advance_external_function_instruction<'plan, Plan>(
    plan: &'plan Plan,
    frame: Frame<'plan, Plan>,
    returns: &mut Returns<'plan, Plan>,
    instruction: &crate::plan::execution::graph::ExternalFunctionInstruction,
) -> Activation<'plan, Plan>
where
    Plan: ExecutableRuntimePlan<Profile = crate::plan::execution::host::HostedExecutionProfile>,
{
    instruction::advance_external_function(plan, frame, returns, instruction)
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

        assert_eq!(
            crate::run_main(&plan, &mut Vec::new()),
            Ok(Value::Int(1.into())),
        );
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
        assert_eq!(
            crate::run_main(&plan, &mut Vec::new()),
            Ok(Value::Int(1.into())),
        );

        let constructor = plan.custom_constructor_id(0, 0);
        let descriptor = plan.custom_constructor(constructor);
        let malformed = EvaluatedCustomValue::from_fields(
            constructor,
            vec![EvaluatedValue::String("wrong".into())].into_boxed_slice(),
        );
        let mut inputs = RetainedValues::empty();
        inputs.push_evaluated(EvaluatedValue::Custom(malformed));

        assert_eq!(
            crate::runtime::function::run_int(
                &plan,
                &mut RuntimeState::new(&mut Vec::new()),
                IntFunctionId(1),
                crate::runtime::HostCallOrigin::Entry,
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
