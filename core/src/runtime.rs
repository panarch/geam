mod constant;
mod echo;
mod error;
mod evaluated;
mod function;
mod graph;
mod host;
mod materialize;
mod profile;
mod state;
mod value;

pub(crate) use host::{
    StoredRuntimeList, StoredRuntimeListCustomFields, StoredRuntimeListItem,
    StoredRuntimeListTupleItems, StoredRuntimeValue,
};

pub use echo::{EchoLocation, EchoOutput, EchoSink};
pub use error::{
    BitArraySegmentPanicReason, ExecutionError, HostError, HostLocation, HostOrigin,
    InvariantError, Panic, PanicDetails, PanicKind, PanicMessage,
};
pub(in crate::runtime) use evaluated::{
    EvaluatedBitArray, EvaluatedBitArrayFunction, EvaluatedBoolFunction, EvaluatedCapture,
    EvaluatedCustomFunction, EvaluatedCustomValue, EvaluatedExternalFunction,
    EvaluatedExternalValue, EvaluatedFloatFunction, EvaluatedFunctionFunction,
    EvaluatedFunctionValueKind, EvaluatedGenericFunction, EvaluatedIntFunction,
    EvaluatedListFunction, EvaluatedNeverFunction, EvaluatedNilFunction, EvaluatedStringFunction,
    EvaluatedTupleFunction, EvaluatedUtfCodepointFunction, EvaluatedValue,
};
#[cfg(test)]
pub(in crate::runtime) use evaluated::{EvaluatedFunctionValue, EvaluatedListCapture};
pub(crate) use value::{
    BitArrayFunctionValue, BoolFunctionValue, CaptureListValue, CaptureValue, CustomFunctionValue,
    CustomFunctionValueTarget, ExternalFunctionValue, FloatFunctionValue, FunctionFunctionValue,
    FunctionValueKind, GenericFunctionValue, IntFunctionValue, ListFunctionValue,
    NeverFunctionValue, NilFunctionValue, StringFunctionValue, TupleFunctionValue,
    UtfCodepointFunctionValue,
};
pub use value::{
    BitArrayValue, BitArrayValueLengthError, CustomFieldValue, CustomValue, ExternalValue,
    ExternalValueIdentity, FunctionValue, ListValue, ListValueItemTypeMismatch, Value,
    ValueInspection,
};

pub(in crate::runtime) use profile::{ExecutableRuntimePlan, RuntimeGraph};

use crate::plan::execution::ExecutionPlan;
use crate::plan::execution::function::{
    ExecutionGraphProfile, ProfiledCoreRuntimeFunctionId, ProfiledRuntimeFunctionId,
};
use crate::plan::execution::runtime::RuntimeExecutionPlan;
use crate::runtime::error::ExecutionResult;
use crate::runtime::graph::RetainedValues;
use crate::runtime::state::{RuntimeState, RuntimeStateFor};

pub fn run_main(plan: &ExecutionPlan, echo: &mut dyn EchoSink) -> Result<Value, ExecutionError> {
    let mut state = RuntimeState::new(echo);
    let function = match RuntimeExecutionPlan::main_runtime(plan) {
        ProfiledRuntimeFunctionId::Core(function) => function,
        ProfiledRuntimeFunctionId::External(function) => match function {},
    };
    let value = run_core_program(plan, &mut state, function, RetainedValues::empty())?;
    finish_program(plan, &mut state, value)
}

pub(crate) fn run_hosted_main<Profile: crate::HostProfile>(
    plan: &crate::plan::execution::HostedExecution<Profile>,
    host: &mut Profile::RunState,
    echo: &mut dyn EchoSink,
) -> Result<Value, ExecutionError> {
    let mut state = RuntimeState::with_host(echo, host);
    run_hosted_program_inner(plan, &mut state)
}

#[cfg(test)]
fn run_hosted_program<Profile: crate::HostProfile>(
    plan: &crate::plan::execution::HostedExecution<Profile>,
    state: &mut RuntimeStateFor<'_, crate::plan::execution::HostedExecution<Profile>>,
) -> Result<Value, ExecutionError> {
    run_hosted_program_inner(plan, state)
}

fn run_hosted_program_inner<Profile: crate::HostProfile>(
    plan: &crate::plan::execution::HostedExecution<Profile>,
    state: &mut RuntimeStateFor<'_, crate::plan::execution::HostedExecution<Profile>>,
) -> Result<Value, ExecutionError> {
    let inputs = RetainedValues::empty();
    let value = match plan.main_runtime() {
        ProfiledRuntimeFunctionId::Core(function) => {
            run_core_program(plan, state, function, inputs)
        }
        ProfiledRuntimeFunctionId::External(function) => {
            function::run_external(plan, state, function, error::HostCallOrigin::Entry, inputs)
                .map(EvaluatedValue::External)
        }
    }?;
    finish_program(plan, state, value)
}

fn run_core_program<Plan>(
    plan: &Plan,
    state: &mut RuntimeStateFor<'_, Plan>,
    function: ProfiledCoreRuntimeFunctionId<RuntimeGraph<Plan>>,
    inputs: RetainedValues,
) -> ExecutionResult<EvaluatedValue>
where
    Plan: ExecutableRuntimePlan,
{
    match function {
        ProfiledCoreRuntimeFunctionId::Never(function) => {
            function::run_never(plan, state, function, error::HostCallOrigin::Entry, inputs)
                .map(|never| match never {})
        }
        ProfiledCoreRuntimeFunctionId::Int(function) => {
            function::run_int(plan, state, function, error::HostCallOrigin::Entry, inputs)
                .map(EvaluatedValue::Int)
        }
        ProfiledCoreRuntimeFunctionId::Float(function) => {
            function::run_float(plan, state, function, error::HostCallOrigin::Entry, inputs)
                .map(EvaluatedValue::Float)
        }
        ProfiledCoreRuntimeFunctionId::String(function) => {
            function::run_string(plan, state, function, error::HostCallOrigin::Entry, inputs)
                .map(EvaluatedValue::String)
        }
        ProfiledCoreRuntimeFunctionId::BitArray(function) => {
            function::run_bit_array(plan, state, function, error::HostCallOrigin::Entry, inputs)
                .map(EvaluatedValue::BitArray)
        }
        ProfiledCoreRuntimeFunctionId::UtfCodepoint(function) => {
            function::run_utf_codepoint(plan, state, function, error::HostCallOrigin::Entry, inputs)
                .map(EvaluatedValue::UtfCodepoint)
        }
        ProfiledCoreRuntimeFunctionId::Custom(function) => {
            function::run_custom(plan, state, function, error::HostCallOrigin::Entry, inputs)
                .map(EvaluatedValue::Custom)
        }
        ProfiledCoreRuntimeFunctionId::Bool(function) => {
            function::run_bool(plan, state, function, error::HostCallOrigin::Entry, inputs)
                .map(EvaluatedValue::Bool)
        }
        ProfiledCoreRuntimeFunctionId::Nil(function) => {
            function::run_nil(plan, state, function, error::HostCallOrigin::Entry, inputs)
                .map(|()| EvaluatedValue::Nil)
        }
        ProfiledCoreRuntimeFunctionId::Tuple { id, .. } => {
            function::run_tuple(plan, state, id, error::HostCallOrigin::Entry, inputs)
                .map(EvaluatedValue::Tuple)
        }
        ProfiledCoreRuntimeFunctionId::List(function) => {
            let function = <RuntimeGraph<Plan> as ExecutionGraphProfile>::list_function(&function);
            function::run_list(plan, state, function, error::HostCallOrigin::Entry, inputs)
                .map(EvaluatedValue::from)
        }
        ProfiledCoreRuntimeFunctionId::Function { id, .. } => plan
            .run_function_return(state, id, error::HostCallOrigin::Entry, inputs)
            .map(EvaluatedValue::Function),
    }
}

fn finish_program<Plan>(
    plan: &Plan,
    state: &mut RuntimeStateFor<'_, Plan>,
    value: EvaluatedValue,
) -> Result<Value, ExecutionError>
where
    Plan: ExecutableRuntimePlan,
{
    state.lists_mut().drain_releases();
    Ok(materialize::value(
        plan.value_metadata(),
        state.lists(),
        value,
    ))
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
