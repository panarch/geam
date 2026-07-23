use super::{evaluate, run_tail};
use crate::plan::execution::ExecutionPlan;
use crate::plan::execution::function::{
    BitArrayFunctionId, BoolFunctionId, CustomFunctionId, FloatFunctionId, IntFunctionId,
    NeverFunctionId, NilFunctionId, StringFunctionId, TupleFunctionId, UtfCodepointFunctionId,
};
use crate::runtime::error::ExecutionResult;
use crate::runtime::evaluated::{
    EvaluatedBitArray, EvaluatedCustomValue, EvaluatedNeverFunction, EvaluatedValue,
};
use crate::runtime::graph::RetainedValues;
use crate::runtime::state::RuntimeState;
use ecow::EcoString;
use num_bigint::BigInt;
use std::convert::Infallible;

pub(in crate::runtime) fn run_never(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
    function: NeverFunctionId,
    inputs: RetainedValues,
) -> ExecutionResult<Infallible> {
    run_tail(
        plan,
        state,
        function,
        inputs,
        |plan, state, function, inputs| {
            evaluate(plan, state, plan.never_function(*function).body(), inputs)
        },
        |_, _, target| target,
    )
}

pub(in crate::runtime) fn run_int(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
    function: IntFunctionId,
    inputs: RetainedValues,
) -> ExecutionResult<BigInt> {
    run_tail(
        plan,
        state,
        function,
        inputs,
        |plan, state, function, inputs| {
            evaluate(plan, state, plan.int_function(*function).body(), inputs)
        },
        |_, _, target| target,
    )
}

pub(in crate::runtime) fn run_float(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
    function: FloatFunctionId,
    inputs: RetainedValues,
) -> ExecutionResult<f64> {
    run_tail(
        plan,
        state,
        function,
        inputs,
        |plan, state, function, inputs| {
            evaluate(plan, state, plan.float_function(*function).body(), inputs)
        },
        |_, _, target| target,
    )
}

pub(in crate::runtime) fn run_string(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
    function: StringFunctionId,
    inputs: RetainedValues,
) -> ExecutionResult<EcoString> {
    run_tail(
        plan,
        state,
        function,
        inputs,
        |plan, state, function, inputs| {
            evaluate(plan, state, plan.string_function(*function).body(), inputs)
        },
        |_, _, target| target,
    )
}

pub(in crate::runtime) fn run_bit_array(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
    function: BitArrayFunctionId,
    inputs: RetainedValues,
) -> ExecutionResult<EvaluatedBitArray> {
    run_tail(
        plan,
        state,
        function,
        inputs,
        |plan, state, function, inputs| {
            evaluate(
                plan,
                state,
                plan.bit_array_function(*function).body(),
                inputs,
            )
        },
        |_, _, target| target,
    )
}

pub(in crate::runtime) fn run_utf_codepoint(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
    function: UtfCodepointFunctionId,
    inputs: RetainedValues,
) -> ExecutionResult<char> {
    run_tail(
        plan,
        state,
        function,
        inputs,
        |plan, state, function, inputs| {
            evaluate(
                plan,
                state,
                plan.utf_codepoint_function(*function).body(),
                inputs,
            )
        },
        |_, _, target| target,
    )
}

pub(in crate::runtime) fn run_custom(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
    function: CustomFunctionId,
    inputs: RetainedValues,
) -> ExecutionResult<EvaluatedCustomValue> {
    run_tail(
        plan,
        state,
        function,
        inputs,
        |plan, state, function, inputs| {
            evaluate(
                plan,
                state,
                plan.custom_function(*function).body().function_body(),
                inputs,
            )
        },
        |plan, function, target| plan.custom_function(*function).body().function_id(target),
    )
}

pub(in crate::runtime) fn run_bool(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
    function: BoolFunctionId,
    inputs: RetainedValues,
) -> ExecutionResult<bool> {
    run_tail(
        plan,
        state,
        function,
        inputs,
        |plan, state, function, inputs| {
            evaluate(plan, state, plan.bool_function(*function).body(), inputs)
        },
        |_, _, target| target,
    )
}

pub(in crate::runtime) fn run_nil(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
    function: NilFunctionId,
    inputs: RetainedValues,
) -> ExecutionResult<()> {
    run_tail(
        plan,
        state,
        function,
        inputs,
        |plan, state, function, inputs| {
            evaluate(plan, state, plan.nil_function(*function).body(), inputs)
        },
        |_, _, target| target,
    )
}

pub(in crate::runtime) fn run_tuple(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
    function: TupleFunctionId,
    inputs: RetainedValues,
) -> ExecutionResult<Vec<EvaluatedValue>> {
    run_tail(
        plan,
        state,
        function,
        inputs,
        |plan, state, function, inputs| {
            evaluate(plan, state, plan.tuple_function(*function).body(), inputs)
        },
        |_, _, target| target,
    )
}

pub(in crate::runtime) fn run_never_value(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
    function: EvaluatedNeverFunction,
    mut inputs: RetainedValues,
) -> ExecutionResult<Infallible> {
    inputs.append_captures(function.captures());
    run_never(plan, state, function.runtime_id(), inputs)
}
