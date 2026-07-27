mod constant;
mod echo;
mod error;
mod evaluated;
mod function;
mod graph;
mod materialize;
mod state;
mod value;

pub use echo::{EchoLocation, EchoOutput, EchoSink};
pub use error::{
    BitArraySegmentPanicReason, ExecutionError, InvariantError, Panic, PanicDetails, PanicKind,
    PanicMessage,
};
pub(in crate::runtime) use evaluated::{
    EvaluatedBitArray, EvaluatedBitArrayFunction, EvaluatedBoolFunction, EvaluatedCapture,
    EvaluatedCustomFunction, EvaluatedCustomValue, EvaluatedFloatFunction,
    EvaluatedFunctionFunction, EvaluatedFunctionValueKind, EvaluatedGenericFunction,
    EvaluatedIntFunction, EvaluatedListFunction, EvaluatedNeverFunction, EvaluatedNilFunction,
    EvaluatedStringFunction, EvaluatedTupleFunction, EvaluatedUtfCodepointFunction, EvaluatedValue,
};
#[cfg(test)]
pub(in crate::runtime) use evaluated::{EvaluatedFunctionValue, EvaluatedListCapture};
pub(crate) use value::{
    BitArrayFunctionValue, BoolFunctionValue, CaptureListValue, CaptureValue, CustomFunctionValue,
    CustomFunctionValueTarget, FloatFunctionValue, FunctionFunctionValue, FunctionValueKind,
    GenericFunctionValue, IntFunctionValue, ListFunctionValue, NeverFunctionValue,
    NilFunctionValue, StringFunctionValue, TupleFunctionValue, UtfCodepointFunctionValue,
};
pub use value::{
    BitArrayValue, BitArrayValueLengthError, CustomFieldValue, CustomValue, FunctionValue,
    ListValue, ListValueItemTypeMismatch, Value, ValueInspection,
};

use crate::plan::execution::ExecutionPlan;
use crate::plan::execution::function::RuntimeFunctionId;
use crate::plan::execution::runtime::RuntimeExecutionPlan;
use crate::runtime::graph::RetainedValues;
use crate::runtime::state::RuntimeState;

pub(in crate::runtime) trait ExecutableRuntimePlan:
    RuntimeExecutionPlan<
        IntFunction: function::RuntimeIntFunction<Self>,
        BoolFunction: function::RuntimeBoolFunction<Self>,
    >
{
}

impl<Plan> ExecutableRuntimePlan for Plan
where
    Plan: RuntimeExecutionPlan,
    Plan::IntFunction: function::RuntimeIntFunction<Plan>,
    Plan::BoolFunction: function::RuntimeBoolFunction<Plan>,
{
}

pub fn run_main(plan: &ExecutionPlan, echo: &mut dyn EchoSink) -> Result<Value, ExecutionError> {
    run_program(plan, echo)
}

pub(crate) fn run_hosted_main(
    plan: &crate::plan::execution::HostedExecution,
    echo: &mut dyn EchoSink,
) -> Result<Value, ExecutionError> {
    run_program(plan, echo)
}

fn run_program(
    plan: &impl ExecutableRuntimePlan,
    echo: &mut dyn EchoSink,
) -> Result<Value, ExecutionError> {
    let mut state = RuntimeState::new(echo);
    let inputs = RetainedValues::empty();
    let value = match plan.main_runtime() {
        RuntimeFunctionId::Never(function) => {
            return function::run_never(plan, &mut state, function, inputs)
                .map(|never| match never {});
        }
        RuntimeFunctionId::Int(function) => {
            function::run_int(plan, &mut state, function, inputs).map(EvaluatedValue::Int)
        }
        RuntimeFunctionId::Float(function) => {
            function::run_float(plan, &mut state, function, inputs).map(EvaluatedValue::Float)
        }
        RuntimeFunctionId::String(function) => {
            function::run_string(plan, &mut state, function, inputs).map(EvaluatedValue::String)
        }
        RuntimeFunctionId::BitArray(function) => {
            function::run_bit_array(plan, &mut state, function, inputs)
                .map(EvaluatedValue::BitArray)
        }
        RuntimeFunctionId::UtfCodepoint(function) => {
            function::run_utf_codepoint(plan, &mut state, function, inputs)
                .map(EvaluatedValue::UtfCodepoint)
        }
        RuntimeFunctionId::Custom(function) => {
            function::run_custom(plan, &mut state, function, inputs).map(EvaluatedValue::Custom)
        }
        RuntimeFunctionId::Bool(function) => {
            function::run_bool(plan, &mut state, function, inputs).map(EvaluatedValue::Bool)
        }
        RuntimeFunctionId::Nil(function) => {
            function::run_nil(plan, &mut state, function, inputs).map(|()| EvaluatedValue::Nil)
        }
        RuntimeFunctionId::Tuple { id, .. } => {
            function::run_tuple(plan, &mut state, id, inputs).map(EvaluatedValue::Tuple)
        }
        RuntimeFunctionId::List(function) => {
            function::run_list(plan, &mut state, function, inputs).map(EvaluatedValue::List)
        }
        RuntimeFunctionId::Function { id, .. } => {
            function::run_function(plan, &mut state, id, inputs).map(EvaluatedValue::Function)
        }
    }?;
    state.drain_releases();
    Ok(materialize::value(plan, &state, value))
}

#[cfg(test)]
fn run_src(src: &str) -> Value {
    let module =
        crate::compile_typed_module("main", "main.gleam", src).expect("source should compile");
    let module_plan = crate::plan_module(module).expect("source should plan");
    let plan = crate::ExecutionPlan::from_module_plan(module_plan);
    run_main(&plan, &mut Vec::new()).expect("source should run")
}

#[cfg(test)]
fn run_src_error(src: &str) -> ExecutionError {
    let module =
        crate::compile_typed_module("main", "main.gleam", src).expect("source should compile");
    let module_plan = crate::plan_module(module).expect("source should plan");
    let plan = crate::ExecutionPlan::from_module_plan(module_plan);
    run_main(&plan, &mut Vec::new()).expect_err("source should fail at runtime")
}

#[cfg(test)]
fn plan_src(src: &str) -> crate::ExecutionPlan {
    let module =
        crate::compile_typed_module("main", "main.gleam", src).expect("source should compile");
    let module_plan = crate::plan_module(module).expect("source should plan");
    crate::ExecutionPlan::from_module_plan(module_plan)
}

#[cfg(test)]
fn int(value: i64) -> Value {
    Value::Int(num_bigint::BigInt::from(value))
}

#[cfg(test)]
mod tests {
    use super::{BitArrayValue, ListValue, Value, int, run_src};

    #[test]
    fn run_main() {
        assert_eq!(
            run_src(
                r#"
pub fn main() {
  1
}
"#,
            ),
            int(1),
        );
    }

    #[test]
    fn run_main_materializes_utf_codepoint_and_nil_returns() {
        assert_eq!(
            run_src("pub fn main() { let assert <<value:utf8_codepoint>> = <<65>> value }"),
            Value::UtfCodepoint('A'),
        );
        assert_eq!(run_src("pub fn main() { Nil }"), Value::Nil);
    }

    #[test]
    fn source_constants_preserve_runtime_values_and_function_identity() {
        let source = r#"
pub type Boxed(value) { Boxed(value) }

const int = 1
const float = 1.5
const string = "geam"
const bit_array = <<1>>
const bool = True
const nil = Nil
const tuple = #(1, "one")
const list = [1, 2]
const empty = []
const other_empty = []
const nested = [[]]
const boxed = Boxed(1)
const function = identity
const other_function = identity

fn identity(value) { value }

pub fn main() {
  #(
    int,
    float,
    string,
    bit_array,
    bool,
    nil,
    tuple,
    list,
    empty == [],
    empty == other_empty,
    nested == [[]],
    boxed == Boxed(1),
    function == function,
    function == other_function,
  )
}
"#;

        assert_eq!(
            run_src(source),
            Value::Tuple(vec![
                Value::Int(1.into()),
                Value::Float(1.5),
                Value::String("geam".into()),
                Value::BitArray(BitArrayValue::from_bytes(vec![1])),
                Value::Bool(true),
                Value::Nil,
                Value::Tuple(vec![Value::Int(1.into()), Value::String("one".into())]),
                Value::List(ListValue::int(vec![1.into(), 2.into()])),
                Value::Bool(true),
                Value::Bool(true),
                Value::Bool(true),
                Value::Bool(true),
                Value::Bool(true),
                Value::Bool(true),
            ]),
        );
    }

    #[test]
    fn constants_referenced_only_by_unreachable_functions_are_not_evaluated() {
        let source = r#"
const failing = <<<<1>>:bits-size(16)>>

fn unused() {
  failing
}

pub fn main() {
  1
}
"#;

        assert_eq!(run_src(source), Value::Int(1.into()));
    }

    #[test]
    fn constants_are_evaluated_only_when_their_reference_is_evaluated() {
        let source = r#"
const failing = <<<<1>>:bits-size(16)>>

pub fn main() {
  case False {
    True -> failing
    False -> <<>>
  }
}
"#;

        assert_eq!(
            run_src(source),
            Value::BitArray(BitArrayValue::from_bytes(Vec::new())),
        );
    }

    #[test]
    fn function_constants_preserve_reference_and_instance_identity() {
        let source = r#"
pub type Boxed(value) { Boxed(value) }

const constructor = Boxed
const function = identity

fn identity(value) { value }

pub fn main() {
  #(
    constructor == constructor,
    Boxed == Boxed,
    function == function,
    identity == identity,
  )
}
"#;

        assert_eq!(
            run_src(source),
            Value::Tuple(vec![
                Value::Bool(false),
                Value::Bool(false),
                Value::Bool(true),
                Value::Bool(true),
            ]),
        );
    }
}
