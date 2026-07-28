mod constant;
mod echo;
mod error;
mod evaluated;
mod function;
mod graph;
mod host;
mod materialize;
mod state;
mod value;

pub use echo::{EchoLocation, EchoOutput, EchoSink};
pub use error::{
    BitArraySegmentPanicReason, ExecutionError, HostError, HostLocation, InvariantError, Panic,
    PanicDetails, PanicKind, PanicMessage,
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
use crate::plan::execution::function::{
    ExecutionFunctionBody, ExecutionHostTarget, FunctionBodyOwner, RuntimeFunctionId,
};
use crate::plan::execution::graph::ParamLocal;
use crate::plan::execution::runtime::RuntimeExecutionPlan;
use crate::runtime::error::{ExecutionResult, HostCallOrigin};
use crate::runtime::graph::RetainedValues;
use crate::runtime::state::{RuntimeState, RuntimeStateFor};

pub(in crate::runtime) trait ExecutableRuntimePlan:
    RuntimeExecutionPlan
{
    type RuntimeHost<'run>: state::RuntimeHostState<State = Self::RunState>
    where
        Self: 'run;

    fn call_host<Body>(
        &self,
        state: &mut RuntimeStateFor<'_, Self>,
        origin: HostCallOrigin,
        target: &ExecutionHostTarget<Self::Profile, Body>,
        inputs: RetainedValues,
    ) -> ExecutionResult<<<Body as FunctionBodyOwner>::Return as graph::GraphValue>::Evaluated>
    where
        Body: ExecutionFunctionBody,
        Body::Return: graph::GraphValue;

    fn host_parameters<Body>(
        &self,
        target: &ExecutionHostTarget<Self::Profile, Body>,
    ) -> &[ParamLocal]
    where
        Body: ExecutionFunctionBody;
}

impl ExecutableRuntimePlan for ExecutionPlan {
    type RuntimeHost<'run> = ();

    fn call_host<Body>(
        &self,
        _state: &mut RuntimeStateFor<'_, Self>,
        _origin: HostCallOrigin,
        target: &ExecutionHostTarget<Self::Profile, Body>,
        _inputs: RetainedValues,
    ) -> ExecutionResult<<<Body as FunctionBodyOwner>::Return as graph::GraphValue>::Evaluated>
    where
        Body: ExecutionFunctionBody,
        Body::Return: graph::GraphValue,
    {
        match *target {}
    }

    fn host_parameters<Body>(
        &self,
        target: &ExecutionHostTarget<Self::Profile, Body>,
    ) -> &[ParamLocal]
    where
        Body: ExecutionFunctionBody,
    {
        match *target {}
    }
}

impl<Profile: crate::HostProfile> ExecutableRuntimePlan
    for crate::plan::execution::HostedExecution<Profile>
{
    type RuntimeHost<'run>
        = &'run mut Profile::RunState
    where
        Self: 'run;

    fn call_host<Body>(
        &self,
        state: &mut RuntimeStateFor<'_, Self>,
        origin: HostCallOrigin,
        target: &ExecutionHostTarget<Self::Profile, Body>,
        inputs: RetainedValues,
    ) -> ExecutionResult<<<Body as FunctionBodyOwner>::Return as graph::GraphValue>::Evaluated>
    where
        Body: ExecutionFunctionBody,
        Body::Return: graph::GraphValue,
    {
        match target {
            crate::plan::execution::host::HostedFunctionTarget::Value(target) => {
                let function = self.host_value_function(target);
                let mut call = host::RuntimeHostCall::new(self, state, function, inputs);
                match function.implementation().call(&mut call) {
                    Ok(returned) => Ok(call.finish(returned, target.return_())),
                    Err(error) => {
                        drop(call);
                        state.values_mut().drain_releases();
                        let site = origin.into_site(function.site());
                        Err(ExecutionError::from_host_call(
                            function.metadata(),
                            site.clone(),
                            self.source_context_for(site.module()),
                            error,
                        ))
                    }
                }
            }
            crate::plan::execution::host::HostedFunctionTarget::Never(target) => {
                let function = self.host_never_function(*target);
                let mut call = host::RuntimeHostCall::new(self, state, function, inputs);
                match function.implementation().call(&mut call) {
                    Ok(never) => match never {},
                    Err(error) => {
                        drop(call);
                        state.values_mut().drain_releases();
                        let site = origin.into_site(function.site());
                        Err(ExecutionError::from_host_call(
                            function.metadata(),
                            site.clone(),
                            self.source_context_for(site.module()),
                            error,
                        ))
                    }
                }
            }
        }
    }

    fn host_parameters<Body>(
        &self,
        target: &ExecutionHostTarget<Self::Profile, Body>,
    ) -> &[ParamLocal]
    where
        Body: ExecutionFunctionBody,
    {
        match target {
            crate::plan::execution::host::HostedFunctionTarget::Value(target) => {
                self.host_value_function(target).parameters()
            }
            crate::plan::execution::host::HostedFunctionTarget::Never(target) => {
                self.host_never_function(*target).parameters()
            }
        }
    }
}

pub fn run_main(plan: &ExecutionPlan, echo: &mut dyn EchoSink) -> Result<Value, ExecutionError> {
    let mut state = RuntimeState::new(echo);
    run_program(plan, &mut state)
}

pub(crate) fn run_hosted_main<Profile: crate::HostProfile>(
    plan: &crate::plan::execution::HostedExecution<Profile>,
    host: &mut Profile::RunState,
    echo: &mut dyn EchoSink,
) -> Result<Value, ExecutionError> {
    let mut state = RuntimeState::with_host(echo, host);
    run_program(plan, &mut state)
}

fn run_program<Plan>(
    plan: &Plan,
    state: &mut RuntimeStateFor<'_, Plan>,
) -> Result<Value, ExecutionError>
where
    Plan: ExecutableRuntimePlan,
{
    let inputs = RetainedValues::empty();
    let value = match plan.main_runtime() {
        RuntimeFunctionId::Never(function) => {
            return function::run_never(
                plan,
                state,
                function,
                error::HostCallOrigin::Entry,
                inputs,
            )
            .map(|never| match never {});
        }
        RuntimeFunctionId::Int(function) => {
            function::run_int(plan, state, function, error::HostCallOrigin::Entry, inputs)
                .map(EvaluatedValue::Int)
        }
        RuntimeFunctionId::Float(function) => {
            function::run_float(plan, state, function, error::HostCallOrigin::Entry, inputs)
                .map(EvaluatedValue::Float)
        }
        RuntimeFunctionId::String(function) => {
            function::run_string(plan, state, function, error::HostCallOrigin::Entry, inputs)
                .map(EvaluatedValue::String)
        }
        RuntimeFunctionId::BitArray(function) => {
            function::run_bit_array(plan, state, function, error::HostCallOrigin::Entry, inputs)
                .map(EvaluatedValue::BitArray)
        }
        RuntimeFunctionId::UtfCodepoint(function) => {
            function::run_utf_codepoint(plan, state, function, error::HostCallOrigin::Entry, inputs)
                .map(EvaluatedValue::UtfCodepoint)
        }
        RuntimeFunctionId::Custom(function) => {
            function::run_custom(plan, state, function, error::HostCallOrigin::Entry, inputs)
                .map(EvaluatedValue::Custom)
        }
        RuntimeFunctionId::Bool(function) => {
            function::run_bool(plan, state, function, error::HostCallOrigin::Entry, inputs)
                .map(EvaluatedValue::Bool)
        }
        RuntimeFunctionId::Nil(function) => {
            function::run_nil(plan, state, function, error::HostCallOrigin::Entry, inputs)
                .map(|()| EvaluatedValue::Nil)
        }
        RuntimeFunctionId::Tuple { id, .. } => {
            function::run_tuple(plan, state, id, error::HostCallOrigin::Entry, inputs)
                .map(EvaluatedValue::Tuple)
        }
        RuntimeFunctionId::List(function) => {
            function::run_list(plan, state, function, error::HostCallOrigin::Entry, inputs)
                .map(EvaluatedValue::from)
        }
        RuntimeFunctionId::Function { id, .. } => {
            function::run_function(plan, state, id, error::HostCallOrigin::Entry, inputs)
                .map(EvaluatedValue::Function)
        }
    }?;
    state.values_mut().drain_releases();
    Ok(materialize::value(plan, state, value))
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
