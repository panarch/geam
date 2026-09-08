use crate::runtime::ExecutableRuntimePlan;
use crate::runtime::error::{ExecutionResult, HostCallOrigin};
use crate::runtime::evaluated::{
    EvaluatedBitArrayFunction, EvaluatedBoolFunction, EvaluatedCustomFunction,
    EvaluatedCustomValue, EvaluatedExternalFunction, EvaluatedFloatFunction,
    EvaluatedFunctionFunction, EvaluatedFunctionValue, EvaluatedIntFunction, EvaluatedListFunction,
    EvaluatedNeverFunction, EvaluatedNilFunction, EvaluatedStringFunction, EvaluatedTupleFunction,
    EvaluatedUtfCodepointFunction, EvaluatedValue,
};
use crate::runtime::graph::ProfiledRetainedValues;
use crate::runtime::state::RuntimeStateFor;
use crate::runtime::{LocalValues, RuntimeValueProfile};

#[derive(Debug, Clone, PartialEq)]
pub(in crate::runtime) enum InvocableFunctionValue<Values: RuntimeValueProfile = LocalValues> {
    Never(EvaluatedNeverFunction<Values>),
    Int(EvaluatedIntFunction<Values>),
    Float(EvaluatedFloatFunction<Values>),
    String(EvaluatedStringFunction<Values>),
    BitArray(EvaluatedBitArrayFunction<Values>),
    UtfCodepoint(EvaluatedUtfCodepointFunction<Values>),
    Custom(EvaluatedCustomFunction<Values>),
    External(EvaluatedExternalFunction<Values>),
    Bool(EvaluatedBoolFunction<Values>),
    Nil(EvaluatedNilFunction<Values>),
    Tuple(EvaluatedTupleFunction<Values>),
    List(EvaluatedListFunction<Values>),
    Function(EvaluatedFunctionFunction<Values>),
}

pub(in crate::runtime) trait StoredCallable<Values: RuntimeValueProfile>:
    Clone
{
    fn from_callable(value: InvocableFunctionValue<Values>) -> Self;
    fn into_evaluated(self) -> EvaluatedFunctionValue<Values>;
}

#[derive(Clone)]
pub(crate) struct LocalCallable(InvocableFunctionValue);

impl StoredCallable<LocalValues> for LocalCallable {
    fn from_callable(value: InvocableFunctionValue) -> Self {
        Self(value)
    }
    fn into_evaluated(self) -> EvaluatedFunctionValue {
        self.0.into_evaluated()
    }
}

impl LocalCallable {
    pub(in crate::runtime) fn with_value<Output>(
        &self,
        read: impl FnOnce(&InvocableFunctionValue) -> Output,
    ) -> Output {
        read(&self.0)
    }
}

impl<Values: RuntimeValueProfile> InvocableFunctionValue<Values> {
    pub(in crate::runtime) fn into_evaluated(self) -> EvaluatedFunctionValue<Values> {
        match self {
            Self::Never(function) => function.into(),
            Self::Int(function) => function.into(),
            Self::Float(function) => function.into(),
            Self::String(function) => function.into(),
            Self::BitArray(function) => function.into(),
            Self::UtfCodepoint(function) => function.into(),
            Self::Custom(function) => function.into(),
            Self::External(function) => function.into(),
            Self::Bool(function) => function.into(),
            Self::Nil(function) => function.into(),
            Self::Tuple(function) => function.into(),
            Self::List(function) => function.into(),
            Self::Function(function) => function.into(),
        }
    }
}

pub(in crate::runtime) fn invoke_callable<Plan: ExecutableRuntimePlan>(
    plan: &Plan,
    state: &mut RuntimeStateFor<'_, Plan>,
    function: &InvocableFunctionValue<Plan::Values>,
    origin: HostCallOrigin,
    arguments: Box<[EvaluatedValue<Plan::Values>]>,
) -> ExecutionResult<EvaluatedValue<Plan::Values>, Plan::Values> {
    match function {
        InvocableFunctionValue::Never(function) => {
            let inputs = callable_inputs(arguments, function.captures());
            super::run_never(plan, state, function.runtime_id(), origin, inputs)
                .map(|never| match never {})
        }
        InvocableFunctionValue::Int(function) => {
            let inputs = callable_inputs(arguments, function.captures());
            super::run_int(plan, state, function.runtime_id(), origin, inputs)
                .map(EvaluatedValue::Int)
        }
        InvocableFunctionValue::Float(function) => {
            let inputs = callable_inputs(arguments, function.captures());
            super::run_float(plan, state, function.runtime_id(), origin, inputs)
                .map(EvaluatedValue::Float)
        }
        InvocableFunctionValue::String(function) => {
            let inputs = callable_inputs(arguments, function.captures());
            super::run_string(plan, state, function.runtime_id(), origin, inputs)
                .map(EvaluatedValue::String)
        }
        InvocableFunctionValue::BitArray(function) => {
            let inputs = callable_inputs(arguments, function.captures());
            super::run_bit_array(plan, state, function.runtime_id(), origin, inputs)
                .map(EvaluatedValue::BitArray)
        }
        InvocableFunctionValue::UtfCodepoint(function) => {
            let inputs = callable_inputs(arguments, function.captures());
            super::run_utf_codepoint(plan, state, function.runtime_id(), origin, inputs)
                .map(EvaluatedValue::UtfCodepoint)
        }
        InvocableFunctionValue::Custom(function) => match function {
            EvaluatedCustomFunction::Function(function) => {
                let inputs = callable_inputs(arguments, function.captures());
                super::run_custom(plan, state, function.runtime_id(), origin, inputs)
                    .map(EvaluatedValue::Custom)
            }
            EvaluatedCustomFunction::Constructor(function) => Ok(EvaluatedValue::Custom(
                EvaluatedCustomValue::from_fields(function.runtime_id(), arguments),
            )),
        },
        InvocableFunctionValue::External(function) => {
            let inputs = callable_inputs(arguments, function.captures());
            super::run_external(plan, state, function.runtime_id(), origin, inputs)
                .map(EvaluatedValue::External)
        }
        InvocableFunctionValue::Bool(function) => {
            let inputs = callable_inputs(arguments, function.captures());
            super::run_bool(plan, state, function.runtime_id(), origin, inputs)
                .map(EvaluatedValue::Bool)
        }
        InvocableFunctionValue::Nil(function) => {
            let inputs = callable_inputs(arguments, function.captures());
            super::run_nil(plan, state, function.runtime_id(), origin, inputs)
                .map(|()| EvaluatedValue::Nil)
        }
        InvocableFunctionValue::Tuple(function) => {
            let inputs = callable_inputs(arguments, function.captures());
            super::run_tuple(plan, state, function.runtime_id(), origin, inputs)
                .map(EvaluatedValue::Tuple)
        }
        InvocableFunctionValue::List(function) => {
            let inputs = callable_inputs(arguments, function.captures());
            super::run_list(plan, state, function.runtime_id(), origin, inputs)
                .map(EvaluatedValue::from)
        }
        InvocableFunctionValue::Function(function) => match function {
            EvaluatedFunctionFunction::Core(function) => {
                let inputs = callable_inputs(arguments, function.captures());
                super::run_core_function(plan, state, function.runtime_id(), origin, inputs)
                    .map(EvaluatedValue::Function)
            }
            EvaluatedFunctionFunction::External(function) => {
                let inputs = callable_inputs(arguments, function.captures());
                super::run_external_function_function(
                    plan,
                    state,
                    function.runtime_id(),
                    origin,
                    inputs,
                )
                .map(EvaluatedValue::Function)
            }
        },
    }
}

pub(in crate::runtime) fn callable_inputs<Values: RuntimeValueProfile>(
    arguments: Box<[EvaluatedValue<Values>]>,
    captures: &[crate::runtime::evaluated::EvaluatedCapture<Values>],
) -> ProfiledRetainedValues<Values> {
    let mut inputs = ProfiledRetainedValues::empty();
    for value in arguments {
        inputs.push_evaluated(value);
    }
    inputs.append_captures(captures);
    inputs
}
