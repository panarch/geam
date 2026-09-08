use crate::runtime::ExecutableRuntimePlan;
use crate::runtime::error::{ExecutionResult, HostCallOrigin};
use crate::runtime::evaluated::{
    EvaluatedBitArrayFunction, EvaluatedBoolFunction, EvaluatedCustomFunction,
    EvaluatedCustomValue, EvaluatedExternalFunction, EvaluatedFloatFunction,
    EvaluatedFunctionFunction, EvaluatedFunctionValue, EvaluatedIntFunction, EvaluatedListFunction,
    EvaluatedNeverFunction, EvaluatedNilFunction, EvaluatedStringFunction, EvaluatedTupleFunction,
    EvaluatedUtfCodepointFunction, EvaluatedValue,
};
use crate::runtime::graph::RetainedValues;
use crate::runtime::state::RuntimeStateFor;

#[derive(Debug, Clone, PartialEq)]
pub(in crate::runtime) enum InvocableFunctionValue {
    Never(EvaluatedNeverFunction),
    Int(EvaluatedIntFunction),
    Float(EvaluatedFloatFunction),
    String(EvaluatedStringFunction),
    BitArray(EvaluatedBitArrayFunction),
    UtfCodepoint(EvaluatedUtfCodepointFunction),
    Custom(EvaluatedCustomFunction),
    External(EvaluatedExternalFunction),
    Bool(EvaluatedBoolFunction),
    Nil(EvaluatedNilFunction),
    Tuple(EvaluatedTupleFunction),
    List(EvaluatedListFunction),
    Function(EvaluatedFunctionFunction),
}

impl InvocableFunctionValue {
    pub(in crate::runtime) fn into_evaluated(self) -> EvaluatedFunctionValue {
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
    function: &InvocableFunctionValue,
    origin: HostCallOrigin,
    arguments: Box<[EvaluatedValue]>,
) -> ExecutionResult<EvaluatedValue> {
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

pub(in crate::runtime) fn callable_inputs(
    arguments: Box<[EvaluatedValue]>,
    captures: &[crate::runtime::evaluated::EvaluatedCapture],
) -> RetainedValues {
    let mut inputs = RetainedValues::empty();
    for value in arguments {
        inputs.push_evaluated(value);
    }
    inputs.append_captures(captures);
    inputs
}
