mod error;
mod evaluated;
mod expression;
mod frame;
mod function;
mod materialize;
mod state;
mod value;

pub use error::{ExecutionError, Panic, PanicDetails, PanicKind, PanicMessage};
pub(in crate::runtime) use evaluated::{
    EvaluatedBitArray, EvaluatedBitArrayFunction, EvaluatedBoolFunction, EvaluatedCapture,
    EvaluatedCaptureKind, EvaluatedFloatFunction, EvaluatedFunctionFunction,
    EvaluatedFunctionValue, EvaluatedFunctionValueKind, EvaluatedIntFunction, EvaluatedListCapture,
    EvaluatedListFunction, EvaluatedNilFunction, EvaluatedStringFunction, EvaluatedTupleFunction,
    EvaluatedValue,
};
#[cfg(test)]
pub(in crate::runtime) use state::RuntimeState;
pub(crate) use value::{
    BitArrayFunctionValue, BoolFunctionValue, CaptureListValue, CaptureValue, FloatFunctionValue,
    FunctionFunctionValue, FunctionValueKind, IntFunctionValue, ListFunctionValue,
    NilFunctionValue, StringFunctionValue, TupleFunctionValue,
};
pub use value::{
    BitArrayValue, BitArrayValueLengthError, FunctionValue, ListValue, ListValueItemTypeMismatch,
    Value,
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
    use super::{int, run_src};

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
}
