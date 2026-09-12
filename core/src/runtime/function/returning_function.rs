use super::run;
use crate::plan::execution::ExecutionPlan;
use crate::plan::execution::function::{
    BitArrayFunctionFunctionId, BoolFunctionFunctionId, CustomFunctionFunctionId,
    FloatFunctionFunctionId, FunctionFunctionFunctionId, GenericFunctionFunctionId,
    IntFunctionFunctionId, NeverFunctionFunctionId, NilFunctionFunctionId,
    ProfiledFunctionFunctionId, ProfiledListFunctionFunctionId, StringFunctionFunctionId,
    TupleFunctionFunctionId, UtfCodepointFunctionFunctionId,
};
use crate::runtime::error::{ExecutionResult, HostCallOrigin};
use crate::runtime::evaluated::{
    EvaluatedBitArrayFunction, EvaluatedBoolFunction, EvaluatedCustomFunction,
    EvaluatedFloatFunction, EvaluatedFunctionFunction, EvaluatedFunctionValue,
    EvaluatedGenericFunction, EvaluatedIntFunction, EvaluatedListFunction, EvaluatedNeverFunction,
    EvaluatedNilFunction, EvaluatedStringFunction, EvaluatedTupleFunction,
    EvaluatedUtfCodepointFunction,
};
use crate::runtime::graph::RetainedValues;
use crate::runtime::state::RuntimeState;
use std::convert::Infallible;

pub(in crate::runtime) fn run_core_function(
    plan: &ExecutionPlan,
    state: &mut RuntimeState<'_>,
    function: ProfiledFunctionFunctionId<Infallible>,
    origin: HostCallOrigin,
    inputs: RetainedValues,
) -> ExecutionResult<EvaluatedFunctionValue> {
    use ProfiledFunctionFunctionId as F;

    match function {
        F::Generic(function) => {
            run_generic_function(plan, state, function, origin, inputs).map(Into::into)
        }
        F::Never(function) => {
            run_never_function(plan, state, function, origin, inputs).map(Into::into)
        }
        F::Int(function) => run_int_function(plan, state, function, origin, inputs).map(Into::into),
        F::Float(function) => {
            run_float_function(plan, state, function, origin, inputs).map(Into::into)
        }
        F::String(function) => {
            run_string_function(plan, state, function, origin, inputs).map(Into::into)
        }
        F::BitArray(function) => {
            run_bit_array_function(plan, state, function, origin, inputs).map(Into::into)
        }
        F::UtfCodepoint(function) => {
            run_utf_codepoint_function(plan, state, function, origin, inputs).map(Into::into)
        }
        F::Custom(function) => {
            run_custom_function(plan, state, function, origin, inputs).map(Into::into)
        }
        F::External(function) => match function {},
        F::Bool(function) => {
            run_bool_function(plan, state, function, origin, inputs).map(Into::into)
        }
        F::Nil(function) => run_nil_function(plan, state, function, origin, inputs).map(Into::into),
        F::Tuple(function) => {
            run_tuple_function(plan, state, function, origin, inputs).map(Into::into)
        }
        F::List(function) => {
            run_core_list_function(plan, state, function, origin, inputs).map(Into::into)
        }
        F::Function(function) => {
            run_function_function(plan, state, function, origin, inputs).map(Into::into)
        }
    }
}

pub(in crate::runtime) fn run_generic_function(
    plan: &ExecutionPlan,
    state: &mut RuntimeState<'_>,
    function: GenericFunctionFunctionId,
    origin: HostCallOrigin,
    inputs: RetainedValues,
) -> ExecutionResult<EvaluatedGenericFunction> {
    run(plan, state, function, origin, inputs)
}

pub(in crate::runtime) fn run_never_function(
    plan: &ExecutionPlan,
    state: &mut RuntimeState<'_>,
    function: NeverFunctionFunctionId,
    origin: HostCallOrigin,
    inputs: RetainedValues,
) -> ExecutionResult<EvaluatedNeverFunction> {
    run(plan, state, function, origin, inputs)
}

pub(in crate::runtime) fn run_int_function(
    plan: &ExecutionPlan,
    state: &mut RuntimeState<'_>,
    function: IntFunctionFunctionId,
    origin: HostCallOrigin,
    inputs: RetainedValues,
) -> ExecutionResult<EvaluatedIntFunction> {
    run(plan, state, function, origin, inputs)
}

pub(in crate::runtime) fn run_float_function(
    plan: &ExecutionPlan,
    state: &mut RuntimeState<'_>,
    function: FloatFunctionFunctionId,
    origin: HostCallOrigin,
    inputs: RetainedValues,
) -> ExecutionResult<EvaluatedFloatFunction> {
    run(plan, state, function, origin, inputs)
}

pub(in crate::runtime) fn run_string_function(
    plan: &ExecutionPlan,
    state: &mut RuntimeState<'_>,
    function: StringFunctionFunctionId,
    origin: HostCallOrigin,
    inputs: RetainedValues,
) -> ExecutionResult<EvaluatedStringFunction> {
    run(plan, state, function, origin, inputs)
}

pub(in crate::runtime) fn run_bit_array_function(
    plan: &ExecutionPlan,
    state: &mut RuntimeState<'_>,
    function: BitArrayFunctionFunctionId,
    origin: HostCallOrigin,
    inputs: RetainedValues,
) -> ExecutionResult<EvaluatedBitArrayFunction> {
    run(plan, state, function, origin, inputs)
}

pub(in crate::runtime) fn run_utf_codepoint_function(
    plan: &ExecutionPlan,
    state: &mut RuntimeState<'_>,
    function: UtfCodepointFunctionFunctionId,
    origin: HostCallOrigin,
    inputs: RetainedValues,
) -> ExecutionResult<EvaluatedUtfCodepointFunction> {
    run(plan, state, function, origin, inputs)
}

pub(in crate::runtime) fn run_custom_function(
    plan: &ExecutionPlan,
    state: &mut RuntimeState<'_>,
    function: CustomFunctionFunctionId,
    origin: HostCallOrigin,
    inputs: RetainedValues,
) -> ExecutionResult<EvaluatedCustomFunction> {
    run(plan, state, function, origin, inputs)
}

pub(in crate::runtime) fn run_bool_function(
    plan: &ExecutionPlan,
    state: &mut RuntimeState<'_>,
    function: BoolFunctionFunctionId,
    origin: HostCallOrigin,
    inputs: RetainedValues,
) -> ExecutionResult<EvaluatedBoolFunction> {
    run(plan, state, function, origin, inputs)
}

pub(in crate::runtime) fn run_nil_function(
    plan: &ExecutionPlan,
    state: &mut RuntimeState<'_>,
    function: NilFunctionFunctionId,
    origin: HostCallOrigin,
    inputs: RetainedValues,
) -> ExecutionResult<EvaluatedNilFunction> {
    run(plan, state, function, origin, inputs)
}

pub(in crate::runtime) fn run_tuple_function(
    plan: &ExecutionPlan,
    state: &mut RuntimeState<'_>,
    function: TupleFunctionFunctionId,
    origin: HostCallOrigin,
    inputs: RetainedValues,
) -> ExecutionResult<EvaluatedTupleFunction> {
    run(plan, state, function, origin, inputs)
}

fn run_core_list_function(
    plan: &ExecutionPlan,
    state: &mut RuntimeState<'_>,
    function: ProfiledListFunctionFunctionId<Infallible>,
    origin: HostCallOrigin,
    inputs: RetainedValues,
) -> ExecutionResult<EvaluatedListFunction> {
    run(plan, state, function, origin, inputs)
}

pub(in crate::runtime) fn run_function_function(
    plan: &ExecutionPlan,
    state: &mut RuntimeState<'_>,
    function: FunctionFunctionFunctionId,
    origin: HostCallOrigin,
    inputs: RetainedValues,
) -> ExecutionResult<EvaluatedFunctionFunction> {
    run(plan, state, function, origin, inputs)
}
