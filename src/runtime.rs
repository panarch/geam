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
    BitArrayFunctionBody, BoolFunctionBody, ExecutionHostTarget, FloatFunctionBody,
    IntFunctionBody, NilFunctionBody, RuntimeFunctionId, StringFunctionBody,
    UtfCodepointFunctionBody,
};
use crate::plan::execution::graph::ParamLocal;
use crate::plan::execution::runtime::RuntimeExecutionPlan;
use crate::runtime::error::{ExecutionResult, HostCallOrigin};
use crate::runtime::graph::RetainedValues;
use crate::runtime::state::{RuntimeState, RuntimeStateFor};
use ecow::EcoString;
use num_bigint::BigInt;

pub(in crate::runtime) trait ExecutableRuntimePlan:
    RuntimeExecutionPlan
{
    fn call_host_int(
        &self,
        state: &mut RuntimeStateFor<'_, Self>,
        origin: HostCallOrigin,
        target: &ExecutionHostTarget<Self::Profile, IntFunctionBody>,
        inputs: &RetainedValues,
    ) -> ExecutionResult<BigInt>;

    fn host_int_parameters(
        &self,
        target: &ExecutionHostTarget<Self::Profile, IntFunctionBody>,
    ) -> &[ParamLocal];

    fn call_host_float(
        &self,
        state: &mut RuntimeStateFor<'_, Self>,
        origin: HostCallOrigin,
        target: &ExecutionHostTarget<Self::Profile, FloatFunctionBody>,
        inputs: &RetainedValues,
    ) -> ExecutionResult<f64>;

    fn host_float_parameters(
        &self,
        target: &ExecutionHostTarget<Self::Profile, FloatFunctionBody>,
    ) -> &[ParamLocal];

    fn call_host_string(
        &self,
        state: &mut RuntimeStateFor<'_, Self>,
        origin: HostCallOrigin,
        target: &ExecutionHostTarget<Self::Profile, StringFunctionBody>,
        inputs: &RetainedValues,
    ) -> ExecutionResult<EcoString>;

    fn host_string_parameters(
        &self,
        target: &ExecutionHostTarget<Self::Profile, StringFunctionBody>,
    ) -> &[ParamLocal];

    fn call_host_bit_array(
        &self,
        state: &mut RuntimeStateFor<'_, Self>,
        origin: HostCallOrigin,
        target: &ExecutionHostTarget<Self::Profile, BitArrayFunctionBody>,
        inputs: &RetainedValues,
    ) -> ExecutionResult<BitArrayValue>;

    fn host_bit_array_parameters(
        &self,
        target: &ExecutionHostTarget<Self::Profile, BitArrayFunctionBody>,
    ) -> &[ParamLocal];

    fn call_host_utf_codepoint(
        &self,
        state: &mut RuntimeStateFor<'_, Self>,
        origin: HostCallOrigin,
        target: &ExecutionHostTarget<Self::Profile, UtfCodepointFunctionBody>,
        inputs: &RetainedValues,
    ) -> ExecutionResult<char>;

    fn host_utf_codepoint_parameters(
        &self,
        target: &ExecutionHostTarget<Self::Profile, UtfCodepointFunctionBody>,
    ) -> &[ParamLocal];

    fn call_host_bool(
        &self,
        state: &mut RuntimeStateFor<'_, Self>,
        origin: HostCallOrigin,
        target: &ExecutionHostTarget<Self::Profile, BoolFunctionBody>,
        inputs: &RetainedValues,
    ) -> ExecutionResult<bool>;

    fn host_bool_parameters(
        &self,
        target: &ExecutionHostTarget<Self::Profile, BoolFunctionBody>,
    ) -> &[ParamLocal];

    fn call_host_nil(
        &self,
        state: &mut RuntimeStateFor<'_, Self>,
        origin: HostCallOrigin,
        target: &ExecutionHostTarget<Self::Profile, NilFunctionBody>,
        inputs: &RetainedValues,
    ) -> ExecutionResult<()>;

    fn host_nil_parameters(
        &self,
        target: &ExecutionHostTarget<Self::Profile, NilFunctionBody>,
    ) -> &[ParamLocal];
}

impl ExecutableRuntimePlan for ExecutionPlan {
    fn call_host_int(
        &self,
        _state: &mut RuntimeStateFor<'_, Self>,
        _origin: HostCallOrigin,
        target: &ExecutionHostTarget<Self::Profile, IntFunctionBody>,
        _inputs: &RetainedValues,
    ) -> ExecutionResult<BigInt> {
        match *target {}
    }

    fn host_int_parameters(
        &self,
        target: &ExecutionHostTarget<Self::Profile, IntFunctionBody>,
    ) -> &[ParamLocal] {
        match *target {}
    }

    fn call_host_float(
        &self,
        _state: &mut RuntimeStateFor<'_, Self>,
        _origin: HostCallOrigin,
        target: &ExecutionHostTarget<Self::Profile, FloatFunctionBody>,
        _inputs: &RetainedValues,
    ) -> ExecutionResult<f64> {
        match *target {}
    }

    fn host_float_parameters(
        &self,
        target: &ExecutionHostTarget<Self::Profile, FloatFunctionBody>,
    ) -> &[ParamLocal] {
        match *target {}
    }

    fn call_host_string(
        &self,
        _state: &mut RuntimeStateFor<'_, Self>,
        _origin: HostCallOrigin,
        target: &ExecutionHostTarget<Self::Profile, StringFunctionBody>,
        _inputs: &RetainedValues,
    ) -> ExecutionResult<EcoString> {
        match *target {}
    }

    fn host_string_parameters(
        &self,
        target: &ExecutionHostTarget<Self::Profile, StringFunctionBody>,
    ) -> &[ParamLocal] {
        match *target {}
    }

    fn call_host_bit_array(
        &self,
        _state: &mut RuntimeStateFor<'_, Self>,
        _origin: HostCallOrigin,
        target: &ExecutionHostTarget<Self::Profile, BitArrayFunctionBody>,
        _inputs: &RetainedValues,
    ) -> ExecutionResult<BitArrayValue> {
        match *target {}
    }

    fn host_bit_array_parameters(
        &self,
        target: &ExecutionHostTarget<Self::Profile, BitArrayFunctionBody>,
    ) -> &[ParamLocal] {
        match *target {}
    }

    fn call_host_utf_codepoint(
        &self,
        _state: &mut RuntimeStateFor<'_, Self>,
        _origin: HostCallOrigin,
        target: &ExecutionHostTarget<Self::Profile, UtfCodepointFunctionBody>,
        _inputs: &RetainedValues,
    ) -> ExecutionResult<char> {
        match *target {}
    }

    fn host_utf_codepoint_parameters(
        &self,
        target: &ExecutionHostTarget<Self::Profile, UtfCodepointFunctionBody>,
    ) -> &[ParamLocal] {
        match *target {}
    }

    fn call_host_bool(
        &self,
        _state: &mut RuntimeStateFor<'_, Self>,
        _origin: HostCallOrigin,
        target: &ExecutionHostTarget<Self::Profile, BoolFunctionBody>,
        _inputs: &RetainedValues,
    ) -> ExecutionResult<bool> {
        match *target {}
    }

    fn host_bool_parameters(
        &self,
        target: &ExecutionHostTarget<Self::Profile, BoolFunctionBody>,
    ) -> &[ParamLocal] {
        match *target {}
    }

    fn call_host_nil(
        &self,
        _state: &mut RuntimeStateFor<'_, Self>,
        _origin: HostCallOrigin,
        target: &ExecutionHostTarget<Self::Profile, NilFunctionBody>,
        _inputs: &RetainedValues,
    ) -> ExecutionResult<()> {
        match *target {}
    }

    fn host_nil_parameters(
        &self,
        target: &ExecutionHostTarget<Self::Profile, NilFunctionBody>,
    ) -> &[ParamLocal] {
        match *target {}
    }
}

impl<Profile: crate::HostProfile> ExecutableRuntimePlan
    for crate::plan::execution::HostedExecution<Profile>
{
    fn call_host_int(
        &self,
        state: &mut RuntimeStateFor<'_, Self>,
        origin: HostCallOrigin,
        target: &ExecutionHostTarget<Self::Profile, IntFunctionBody>,
        inputs: &RetainedValues,
    ) -> ExecutionResult<BigInt> {
        let function = self.host_int_function(*target);
        function.call(state.host_state(), inputs).map_err(|error| {
            let site = origin.into_site(function.site());
            ExecutionError::from_host_call(
                function.metadata(),
                site.clone(),
                self.source_context_for(site.module()),
                error,
            )
        })
    }

    fn host_int_parameters(
        &self,
        target: &ExecutionHostTarget<Self::Profile, IntFunctionBody>,
    ) -> &[ParamLocal] {
        self.host_int_function(*target).parameters()
    }

    fn call_host_float(
        &self,
        state: &mut RuntimeStateFor<'_, Self>,
        origin: HostCallOrigin,
        target: &ExecutionHostTarget<Self::Profile, FloatFunctionBody>,
        inputs: &RetainedValues,
    ) -> ExecutionResult<f64> {
        let function = self.host_float_function(*target);
        function.call(state.host_state(), inputs).map_err(|error| {
            let site = origin.into_site(function.site());
            ExecutionError::from_host_call(
                function.metadata(),
                site.clone(),
                self.source_context_for(site.module()),
                error,
            )
        })
    }

    fn host_float_parameters(
        &self,
        target: &ExecutionHostTarget<Self::Profile, FloatFunctionBody>,
    ) -> &[ParamLocal] {
        self.host_float_function(*target).parameters()
    }

    fn call_host_string(
        &self,
        state: &mut RuntimeStateFor<'_, Self>,
        origin: HostCallOrigin,
        target: &ExecutionHostTarget<Self::Profile, StringFunctionBody>,
        inputs: &RetainedValues,
    ) -> ExecutionResult<EcoString> {
        let function = self.host_string_function(*target);
        function.call(state.host_state(), inputs).map_err(|error| {
            let site = origin.into_site(function.site());
            ExecutionError::from_host_call(
                function.metadata(),
                site.clone(),
                self.source_context_for(site.module()),
                error,
            )
        })
    }

    fn host_string_parameters(
        &self,
        target: &ExecutionHostTarget<Self::Profile, StringFunctionBody>,
    ) -> &[ParamLocal] {
        self.host_string_function(*target).parameters()
    }

    fn call_host_bit_array(
        &self,
        state: &mut RuntimeStateFor<'_, Self>,
        origin: HostCallOrigin,
        target: &ExecutionHostTarget<Self::Profile, BitArrayFunctionBody>,
        inputs: &RetainedValues,
    ) -> ExecutionResult<BitArrayValue> {
        let function = self.host_bit_array_function(*target);
        function.call(state.host_state(), inputs).map_err(|error| {
            let site = origin.into_site(function.site());
            ExecutionError::from_host_call(
                function.metadata(),
                site.clone(),
                self.source_context_for(site.module()),
                error,
            )
        })
    }

    fn host_bit_array_parameters(
        &self,
        target: &ExecutionHostTarget<Self::Profile, BitArrayFunctionBody>,
    ) -> &[ParamLocal] {
        self.host_bit_array_function(*target).parameters()
    }

    fn call_host_utf_codepoint(
        &self,
        state: &mut RuntimeStateFor<'_, Self>,
        origin: HostCallOrigin,
        target: &ExecutionHostTarget<Self::Profile, UtfCodepointFunctionBody>,
        inputs: &RetainedValues,
    ) -> ExecutionResult<char> {
        let function = self.host_utf_codepoint_function(*target);
        function.call(state.host_state(), inputs).map_err(|error| {
            let site = origin.into_site(function.site());
            ExecutionError::from_host_call(
                function.metadata(),
                site.clone(),
                self.source_context_for(site.module()),
                error,
            )
        })
    }

    fn host_utf_codepoint_parameters(
        &self,
        target: &ExecutionHostTarget<Self::Profile, UtfCodepointFunctionBody>,
    ) -> &[ParamLocal] {
        self.host_utf_codepoint_function(*target).parameters()
    }

    fn call_host_bool(
        &self,
        state: &mut RuntimeStateFor<'_, Self>,
        origin: HostCallOrigin,
        target: &ExecutionHostTarget<Self::Profile, BoolFunctionBody>,
        inputs: &RetainedValues,
    ) -> ExecutionResult<bool> {
        let function = self.host_bool_function(*target);
        function.call(state.host_state(), inputs).map_err(|error| {
            let site = origin.into_site(function.site());
            ExecutionError::from_host_call(
                function.metadata(),
                site.clone(),
                self.source_context_for(site.module()),
                error,
            )
        })
    }

    fn host_bool_parameters(
        &self,
        target: &ExecutionHostTarget<Self::Profile, BoolFunctionBody>,
    ) -> &[ParamLocal] {
        self.host_bool_function(*target).parameters()
    }

    fn call_host_nil(
        &self,
        state: &mut RuntimeStateFor<'_, Self>,
        origin: HostCallOrigin,
        target: &ExecutionHostTarget<Self::Profile, NilFunctionBody>,
        inputs: &RetainedValues,
    ) -> ExecutionResult<()> {
        let function = self.host_nil_function(*target);
        function.call(state.host_state(), inputs).map_err(|error| {
            let site = origin.into_site(function.site());
            ExecutionError::from_host_call(
                function.metadata(),
                site.clone(),
                self.source_context_for(site.module()),
                error,
            )
        })
    }

    fn host_nil_parameters(
        &self,
        target: &ExecutionHostTarget<Self::Profile, NilFunctionBody>,
    ) -> &[ParamLocal] {
        self.host_nil_function(*target).parameters()
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
    state: &mut RuntimeState<'_, Plan::RunState>,
) -> Result<Value, ExecutionError>
where
    Plan: ExecutableRuntimePlan,
{
    let inputs = RetainedValues::empty();
    let value = match plan.main_runtime() {
        RuntimeFunctionId::Never(function) => {
            return function::run_never(plan, state, function, inputs).map(|never| match never {});
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
            function::run_custom(plan, state, function, inputs).map(EvaluatedValue::Custom)
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
            function::run_tuple(plan, state, id, inputs).map(EvaluatedValue::Tuple)
        }
        RuntimeFunctionId::List(function) => {
            function::run_list(plan, state, function, inputs).map(EvaluatedValue::List)
        }
        RuntimeFunctionId::Function { id, .. } => {
            function::run_function(plan, state, id, inputs).map(EvaluatedValue::Function)
        }
    }?;
    state.drain_releases();
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
