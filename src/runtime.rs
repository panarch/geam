mod error;
mod evaluated;
mod expression;
mod frame;
mod function;
mod materialize;
mod pattern;
mod state;
mod value;

pub use error::{
    BitArraySegmentPanicReason, ExecutionError, InvariantError, Panic, PanicDetails, PanicKind,
    PanicMessage,
};
pub(in crate::runtime) use evaluated::{
    EvaluatedBitArray, EvaluatedBitArrayFunction, EvaluatedBoolFunction, EvaluatedCapture,
    EvaluatedCaptureKind, EvaluatedCustomFunction, EvaluatedCustomValue, EvaluatedFloatFunction,
    EvaluatedFunction, EvaluatedFunctionFunction, EvaluatedFunctionValue,
    EvaluatedFunctionValueKind, EvaluatedGenericFunction, EvaluatedIntFunction,
    EvaluatedListCapture, EvaluatedListFunction, EvaluatedNeverFunction, EvaluatedNilFunction,
    EvaluatedStringFunction, EvaluatedTupleFunction, EvaluatedUtfCodepointFunction, EvaluatedValue,
};
#[cfg(test)]
pub(in crate::runtime) use state::RuntimeState;
pub(crate) use value::{
    BitArrayFunctionValue, BoolFunctionValue, CaptureListValue, CaptureValue, CustomFunctionValue,
    CustomFunctionValueTarget, FloatFunctionValue, FunctionFunctionValue, FunctionValueKind,
    GenericFunctionValue, IntFunctionValue, ListFunctionValue, NeverFunctionValue,
    NilFunctionValue, StringFunctionValue, TupleFunctionValue, UtfCodepointFunctionValue,
};
pub use value::{
    BitArrayValue, BitArrayValueLengthError, CustomFieldValue, CustomValue, FunctionValue,
    ListValue, ListValueItemTypeMismatch, Value,
};

use crate::plan::execution::ExecutionPlan;

pub fn run_main(plan: &ExecutionPlan) -> Result<Value, ExecutionError> {
    function::run_main(plan)
}

#[cfg(test)]
fn run_src(src: &str) -> Value {
    let module =
        crate::compile_typed_module("main", "main.gleam", src).expect("source should compile");
    let module_plan = crate::plan_module(module).expect("source should plan");
    let plan = crate::ExecutionPlan::from_module_plan(module_plan);
    run_main(&plan).expect("source should run")
}

#[cfg(test)]
fn run_src_error(src: &str) -> ExecutionError {
    let module =
        crate::compile_typed_module("main", "main.gleam", src).expect("source should compile");
    let module_plan = crate::plan_module(module).expect("source should plan");
    let plan = crate::ExecutionPlan::from_module_plan(module_plan);
    run_main(&plan).expect_err("source should fail at runtime")
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
