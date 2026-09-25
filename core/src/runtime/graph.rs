mod activation;
mod bit_array;
mod environment;
mod instruction;
mod pattern;
mod terminator;

pub(super) use environment::GraphValue;
pub(crate) use environment::RetainedValues;

pub(in crate::runtime) use self::environment::BlockEnvironment;
pub(in crate::runtime) use self::terminator::RuntimeGraphState;
use crate::ExecutionError;
use crate::plan::execution::graph::{
    BlockGraphExitId, BlockId, ExternalFunctionInstruction, ExternalListInstruction, Transfer,
};
use crate::plan::execution::host::HostedExecutionProfile;
use crate::plan::execution::type_::ValueType;
use crate::runtime::error::ExecutionResult;
use crate::runtime::{CaptureStorage, ExecutableRuntimePlan};
pub(in crate::runtime) use activation::Returns;
pub(in crate::runtime) use activation::{Execution as GraphExecution, Progress as GraphProgress};
pub(in crate::runtime) use instruction::{
    ExternalFunctionInstructionValue, ExternalListInstructionValue,
};

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
        value.take(self.environment)
    }

    pub(in crate::runtime) fn into_retained(self, transfer: &Transfer) -> RetainedValues {
        self.environment.into_retained(transfer)
    }
}

pub(in crate::runtime) fn evaluate_external_list_instruction<Plan>(
    plan: &Plan,
    state: &mut impl RuntimeGraphState<Error = ExecutionError>,
    environment: &BlockEnvironment,
    instruction: &ExternalListInstruction,
    expected: &ValueType,
) -> ExecutionResult<ExternalListInstructionValue>
where
    Plan: ExecutableRuntimePlan<Profile = HostedExecutionProfile>,
{
    instruction::evaluate_external_list(plan, state, environment, instruction, expected)
}

pub(in crate::runtime) fn evaluate_external_function_instruction<Plan>(
    plan: &Plan,
    captures: &CaptureStorage,
    environment: &BlockEnvironment,
    instruction: &ExternalFunctionInstruction,
) -> ExternalFunctionInstructionValue
where
    Plan: ExecutableRuntimePlan<Profile = HostedExecutionProfile>,
{
    instruction::evaluate_external_function(plan, captures, environment, instruction)
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
    fn mutual_tail_handoffs_preserve_duplicates_and_an_ordinary_callers_live_values() {
        let source = r#"
fn left(n, first, second) {
  case n {
    0 -> #(second, first, second)
    _ -> right(n - 1, second, first)
  }
}

fn right(n, first, second) {
  case n {
    0 -> #(first, second, first)
    _ -> left(n - 1, second, first)
  }
}

pub fn main() {
  let first = #(10, "first")
  let second = #(20, "second")
  let result = left(1001, first, second)
  #(first, second, result)
}
"#;
        let first = Value::Tuple(vec![Value::Int(10.into()), Value::String("first".into())]);
        let second = Value::Tuple(vec![Value::Int(20.into()), Value::String("second".into())]);
        assert_eq!(
            crate::runtime::run_src(source),
            Value::Tuple(vec![
                first.clone(),
                second.clone(),
                Value::Tuple(vec![second.clone(), first, second])
            ]),
        );
    }

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
                    constructor: descriptor.name().into(),
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
