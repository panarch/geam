use super::{evaluate, run_tail};
use crate::plan::execution::function::{
    BitArrayFunctionFunctionId, BoolFunctionFunctionId, CustomFunctionFunctionId,
    FloatFunctionFunctionId, FunctionFunctionFunctionId, FunctionFunctionId,
    GenericFunctionFunctionId, IntFunctionFunctionId, ListFunctionFunctionId,
    NeverFunctionFunctionId, NilFunctionFunctionId, StringFunctionFunctionId,
    TupleFunctionFunctionId, UtfCodepointFunctionFunctionId,
};
use crate::runtime::ExecutableRuntimePlan;
use crate::runtime::error::ExecutionResult;
use crate::runtime::evaluated::{
    EvaluatedBitArrayFunction, EvaluatedBoolFunction, EvaluatedCustomFunction,
    EvaluatedFloatFunction, EvaluatedFunctionFunction, EvaluatedFunctionValue,
    EvaluatedGenericFunction, EvaluatedIntFunction, EvaluatedListFunction, EvaluatedNeverFunction,
    EvaluatedNilFunction, EvaluatedStringFunction, EvaluatedTupleFunction,
    EvaluatedUtfCodepointFunction,
};
use crate::runtime::graph::RetainedValues;
use crate::runtime::state::RuntimeStateFor;

pub(in crate::runtime) fn run_int_function<Plan: ExecutableRuntimePlan>(
    plan: &Plan,
    state: &mut RuntimeStateFor<'_, Plan>,
    function: IntFunctionFunctionId,
    inputs: RetainedValues,
) -> ExecutionResult<EvaluatedIntFunction> {
    run_tail(
        plan,
        state,
        function,
        inputs,
        |plan, state, function, inputs| {
            evaluate(
                plan,
                state,
                plan.int_function_function(*function).body().function_body(),
                inputs,
            )
        },
        |_, _, target| target,
    )
}

pub(in crate::runtime) fn run_float_function<Plan: ExecutableRuntimePlan>(
    plan: &Plan,
    state: &mut RuntimeStateFor<'_, Plan>,
    function: FloatFunctionFunctionId,
    inputs: RetainedValues,
) -> ExecutionResult<EvaluatedFloatFunction> {
    run_tail(
        plan,
        state,
        function,
        inputs,
        |plan, state, function, inputs| {
            evaluate(
                plan,
                state,
                plan.float_function_function(*function)
                    .body()
                    .function_body(),
                inputs,
            )
        },
        |_, _, target| target,
    )
}

pub(in crate::runtime) fn run_string_function<Plan: ExecutableRuntimePlan>(
    plan: &Plan,
    state: &mut RuntimeStateFor<'_, Plan>,
    function: StringFunctionFunctionId,
    inputs: RetainedValues,
) -> ExecutionResult<EvaluatedStringFunction> {
    run_tail(
        plan,
        state,
        function,
        inputs,
        |plan, state, function, inputs| {
            evaluate(
                plan,
                state,
                plan.string_function_function(*function)
                    .body()
                    .function_body(),
                inputs,
            )
        },
        |_, _, target| target,
    )
}

pub(in crate::runtime) fn run_bit_array_function<Plan: ExecutableRuntimePlan>(
    plan: &Plan,
    state: &mut RuntimeStateFor<'_, Plan>,
    function: BitArrayFunctionFunctionId,
    inputs: RetainedValues,
) -> ExecutionResult<EvaluatedBitArrayFunction> {
    run_tail(
        plan,
        state,
        function,
        inputs,
        |plan, state, function, inputs| {
            evaluate(
                plan,
                state,
                plan.bit_array_function_function(*function)
                    .body()
                    .function_body(),
                inputs,
            )
        },
        |_, _, target| target,
    )
}

pub(in crate::runtime) fn run_utf_codepoint_function<Plan: ExecutableRuntimePlan>(
    plan: &Plan,
    state: &mut RuntimeStateFor<'_, Plan>,
    function: UtfCodepointFunctionFunctionId,
    inputs: RetainedValues,
) -> ExecutionResult<EvaluatedUtfCodepointFunction> {
    run_tail(
        plan,
        state,
        function,
        inputs,
        |plan, state, function, inputs| {
            evaluate(
                plan,
                state,
                plan.utf_codepoint_function_function(*function)
                    .body()
                    .function_body(),
                inputs,
            )
        },
        |_, _, target| target,
    )
}

pub(in crate::runtime) fn run_generic_function<Plan: ExecutableRuntimePlan>(
    plan: &Plan,
    state: &mut RuntimeStateFor<'_, Plan>,
    function: GenericFunctionFunctionId,
    inputs: RetainedValues,
) -> ExecutionResult<EvaluatedGenericFunction> {
    run_tail(
        plan,
        state,
        function,
        inputs,
        |plan, state, function, inputs| {
            evaluate(
                plan,
                state,
                plan.generic_function_function(function)
                    .body()
                    .function_body(),
                inputs,
            )
        },
        |_, _, target| target,
    )
}

pub(in crate::runtime) fn run_never_function<Plan: ExecutableRuntimePlan>(
    plan: &Plan,
    state: &mut RuntimeStateFor<'_, Plan>,
    function: NeverFunctionFunctionId,
    inputs: RetainedValues,
) -> ExecutionResult<EvaluatedNeverFunction> {
    run_tail(
        plan,
        state,
        function,
        inputs,
        |plan, state, function, inputs| {
            evaluate(
                plan,
                state,
                plan.never_function_function(function)
                    .body()
                    .function_body(),
                inputs,
            )
        },
        |_, _, target| target,
    )
}

pub(in crate::runtime) fn run_custom_function<Plan: ExecutableRuntimePlan>(
    plan: &Plan,
    state: &mut RuntimeStateFor<'_, Plan>,
    function: CustomFunctionFunctionId,
    inputs: RetainedValues,
) -> ExecutionResult<EvaluatedCustomFunction> {
    run_tail(
        plan,
        state,
        function,
        inputs,
        |plan, state, function, inputs| {
            evaluate(
                plan,
                state,
                plan.custom_function_function(function)
                    .body()
                    .function_body(),
                inputs,
            )
        },
        |plan, function, target| {
            plan.custom_function_function(function)
                .body()
                .function_id(target)
        },
    )
}

pub(in crate::runtime) fn run_bool_function<Plan: ExecutableRuntimePlan>(
    plan: &Plan,
    state: &mut RuntimeStateFor<'_, Plan>,
    function: BoolFunctionFunctionId,
    inputs: RetainedValues,
) -> ExecutionResult<EvaluatedBoolFunction> {
    run_tail(
        plan,
        state,
        function,
        inputs,
        |plan, state, function, inputs| {
            evaluate(
                plan,
                state,
                plan.bool_function_function(*function)
                    .body()
                    .function_body(),
                inputs,
            )
        },
        |_, _, target| target,
    )
}

pub(in crate::runtime) fn run_nil_function<Plan: ExecutableRuntimePlan>(
    plan: &Plan,
    state: &mut RuntimeStateFor<'_, Plan>,
    function: NilFunctionFunctionId,
    inputs: RetainedValues,
) -> ExecutionResult<EvaluatedNilFunction> {
    run_tail(
        plan,
        state,
        function,
        inputs,
        |plan, state, function, inputs| {
            evaluate(
                plan,
                state,
                plan.nil_function_function(*function).body().function_body(),
                inputs,
            )
        },
        |_, _, target| target,
    )
}

pub(in crate::runtime) fn run_tuple_function<Plan: ExecutableRuntimePlan>(
    plan: &Plan,
    state: &mut RuntimeStateFor<'_, Plan>,
    function: TupleFunctionFunctionId,
    inputs: RetainedValues,
) -> ExecutionResult<EvaluatedTupleFunction> {
    run_tail(
        plan,
        state,
        function,
        inputs,
        |plan, state, function, inputs| {
            evaluate(
                plan,
                state,
                plan.tuple_function_function(*function)
                    .body()
                    .function_body(),
                inputs,
            )
        },
        |_, _, target| target,
    )
}

pub(in crate::runtime) fn run_list_function<Plan: ExecutableRuntimePlan>(
    plan: &Plan,
    state: &mut RuntimeStateFor<'_, Plan>,
    function: ListFunctionFunctionId,
    inputs: RetainedValues,
) -> ExecutionResult<EvaluatedListFunction> {
    run_tail(
        plan,
        state,
        function,
        inputs,
        |plan, state, function, inputs| {
            evaluate(
                plan,
                state,
                plan.list_function_function(function).body().function_body(),
                inputs,
            )
        },
        |_, _, target| target,
    )
}

pub(in crate::runtime) fn run_function_function<Plan: ExecutableRuntimePlan>(
    plan: &Plan,
    state: &mut RuntimeStateFor<'_, Plan>,
    function: FunctionFunctionFunctionId,
    inputs: RetainedValues,
) -> ExecutionResult<EvaluatedFunctionFunction> {
    run_tail(
        plan,
        state,
        function,
        inputs,
        |plan, state, function, inputs| {
            evaluate(
                plan,
                state,
                plan.function_function_function(function)
                    .body()
                    .function_body(),
                inputs,
            )
        },
        |plan, function, target| {
            plan.function_function_function(function)
                .body()
                .function_id(target)
        },
    )
}

pub(in crate::runtime) fn run_function<Plan: ExecutableRuntimePlan>(
    plan: &Plan,
    state: &mut RuntimeStateFor<'_, Plan>,
    function: FunctionFunctionId,
    inputs: RetainedValues,
) -> ExecutionResult<EvaluatedFunctionValue> {
    match function {
        FunctionFunctionId::Generic(function) => {
            run_generic_function(plan, state, function, inputs).map(Into::into)
        }
        FunctionFunctionId::Never(function) => {
            run_never_function(plan, state, function, inputs).map(Into::into)
        }
        FunctionFunctionId::Int(function) => {
            run_int_function(plan, state, function, inputs).map(Into::into)
        }
        FunctionFunctionId::Float(function) => {
            run_float_function(plan, state, function, inputs).map(Into::into)
        }
        FunctionFunctionId::String(function) => {
            run_string_function(plan, state, function, inputs).map(Into::into)
        }
        FunctionFunctionId::BitArray(function) => {
            run_bit_array_function(plan, state, function, inputs).map(Into::into)
        }
        FunctionFunctionId::UtfCodepoint(function) => {
            run_utf_codepoint_function(plan, state, function, inputs).map(Into::into)
        }
        FunctionFunctionId::Custom(function) => {
            run_custom_function(plan, state, function, inputs).map(Into::into)
        }
        FunctionFunctionId::Bool(function) => {
            run_bool_function(plan, state, function, inputs).map(Into::into)
        }
        FunctionFunctionId::Nil(function) => {
            run_nil_function(plan, state, function, inputs).map(Into::into)
        }
        FunctionFunctionId::Tuple(function) => {
            run_tuple_function(plan, state, function, inputs).map(Into::into)
        }
        FunctionFunctionId::List(function) => {
            run_list_function(plan, state, function, inputs).map(Into::into)
        }
        FunctionFunctionId::Function(function) => {
            run_function_function(plan, state, function, inputs).map(Into::into)
        }
    }
}
