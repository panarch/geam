use super::{evaluate_entry, run_tail};
use crate::plan::execution::function::{
    BitArrayFunctionFunctionId, BoolFunctionFunctionId, CustomFunctionFunctionId,
    ExternalFunctionFunctionId, ExternalListFunctionFunctionId, FloatFunctionFunctionId,
    FunctionFunctionFunctionId, GenericFunctionFunctionId, IntFunctionFunctionId,
    NeverFunctionFunctionId, NilFunctionFunctionId, ProfiledFunctionFunctionId,
    ProfiledListFunctionFunctionId, StringFunctionFunctionId, TupleFunctionFunctionId,
    UtfCodepointFunctionFunctionId,
};
use crate::plan::execution::graph::ExternalFunctionCallTarget;
use crate::runtime::ExecutableRuntimePlan;
use crate::runtime::error::{ExecutionResult, HostCallOrigin};
use crate::runtime::evaluated::{
    EvaluatedBitArrayFunction, EvaluatedBoolFunction, EvaluatedCustomFunction,
    EvaluatedExternalFunction, EvaluatedFloatFunction, EvaluatedFunctionFunction,
    EvaluatedFunctionValue, EvaluatedGenericFunction, EvaluatedIntFunction, EvaluatedListFunction,
    EvaluatedNeverFunction, EvaluatedNilFunction, EvaluatedStringFunction, EvaluatedTupleFunction,
    EvaluatedUtfCodepointFunction,
};
use crate::runtime::graph::ProfiledRetainedValues;
use crate::runtime::state::RuntimeStateFor;
use std::convert::Infallible;

pub(in crate::runtime) fn run_core_function<Plan: ExecutableRuntimePlan>(
    plan: &Plan,
    state: &mut RuntimeStateFor<'_, Plan>,
    function: ProfiledFunctionFunctionId<Infallible>,
    origin: HostCallOrigin,
    inputs: ProfiledRetainedValues<Plan::Values>,
) -> ExecutionResult<EvaluatedFunctionValue<Plan::Values>, Plan::Values> {
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

pub(in crate::runtime) fn run_external_function_function<Plan: ExecutableRuntimePlan>(
    plan: &Plan,
    state: &mut RuntimeStateFor<'_, Plan>,
    function: ExternalFunctionCallTarget,
    origin: HostCallOrigin,
    inputs: ProfiledRetainedValues<Plan::Values>,
) -> ExecutionResult<EvaluatedFunctionValue<Plan::Values>, Plan::Values> {
    match function {
        ExternalFunctionCallTarget::Function(function) => {
            run_external_function(plan, state, function, origin, inputs).map(Into::into)
        }
        ExternalFunctionCallTarget::ListFunction { id, .. } => {
            run_external_list_function(plan, state, id, origin, inputs).map(Into::into)
        }
    }
}

pub(in crate::runtime) fn run_generic_function<Plan: ExecutableRuntimePlan>(
    plan: &Plan,
    state: &mut RuntimeStateFor<'_, Plan>,
    function: GenericFunctionFunctionId,
    origin: HostCallOrigin,
    inputs: ProfiledRetainedValues<Plan::Values>,
) -> ExecutionResult<EvaluatedGenericFunction<Plan::Values>, Plan::Values> {
    run_tail(
        plan,
        state,
        function,
        origin,
        inputs,
        |plan, state, function, origin, inputs| {
            evaluate_entry(
                plan,
                state,
                plan.generic_function_function(function),
                origin,
                inputs,
            )
        },
        |_, _, target| {
            (
                target.function().clone(),
                HostCallOrigin::source(target.site().clone()),
            )
        },
    )
}

pub(in crate::runtime) fn run_never_function<Plan: ExecutableRuntimePlan>(
    plan: &Plan,
    state: &mut RuntimeStateFor<'_, Plan>,
    function: NeverFunctionFunctionId,
    origin: HostCallOrigin,
    inputs: ProfiledRetainedValues<Plan::Values>,
) -> ExecutionResult<EvaluatedNeverFunction<Plan::Values>, Plan::Values> {
    run_tail(
        plan,
        state,
        function,
        origin,
        inputs,
        |plan, state, function, origin, inputs| {
            evaluate_entry(
                plan,
                state,
                plan.never_function_function(function),
                origin,
                inputs,
            )
        },
        |_, _, target| {
            (
                target.function().clone(),
                HostCallOrigin::source(target.site().clone()),
            )
        },
    )
}

pub(in crate::runtime) fn run_int_function<Plan: ExecutableRuntimePlan>(
    plan: &Plan,
    state: &mut RuntimeStateFor<'_, Plan>,
    function: IntFunctionFunctionId,
    origin: HostCallOrigin,
    inputs: ProfiledRetainedValues<Plan::Values>,
) -> ExecutionResult<EvaluatedIntFunction<Plan::Values>, Plan::Values> {
    run_tail(
        plan,
        state,
        function,
        origin,
        inputs,
        |plan, state, function, origin, inputs| {
            evaluate_entry(
                plan,
                state,
                plan.int_function_function(*function),
                origin,
                inputs,
            )
        },
        |_, _, target| {
            (
                *target.function(),
                HostCallOrigin::source(target.site().clone()),
            )
        },
    )
}

pub(in crate::runtime) fn run_float_function<Plan: ExecutableRuntimePlan>(
    plan: &Plan,
    state: &mut RuntimeStateFor<'_, Plan>,
    function: FloatFunctionFunctionId,
    origin: HostCallOrigin,
    inputs: ProfiledRetainedValues<Plan::Values>,
) -> ExecutionResult<EvaluatedFloatFunction<Plan::Values>, Plan::Values> {
    run_tail(
        plan,
        state,
        function,
        origin,
        inputs,
        |plan, state, function, origin, inputs| {
            evaluate_entry(
                plan,
                state,
                plan.float_function_function(*function),
                origin,
                inputs,
            )
        },
        |_, _, target| {
            (
                *target.function(),
                HostCallOrigin::source(target.site().clone()),
            )
        },
    )
}

pub(in crate::runtime) fn run_string_function<Plan: ExecutableRuntimePlan>(
    plan: &Plan,
    state: &mut RuntimeStateFor<'_, Plan>,
    function: StringFunctionFunctionId,
    origin: HostCallOrigin,
    inputs: ProfiledRetainedValues<Plan::Values>,
) -> ExecutionResult<EvaluatedStringFunction<Plan::Values>, Plan::Values> {
    run_tail(
        plan,
        state,
        function,
        origin,
        inputs,
        |plan, state, function, origin, inputs| {
            evaluate_entry(
                plan,
                state,
                plan.string_function_function(*function),
                origin,
                inputs,
            )
        },
        |_, _, target| {
            (
                *target.function(),
                HostCallOrigin::source(target.site().clone()),
            )
        },
    )
}

pub(in crate::runtime) fn run_bit_array_function<Plan: ExecutableRuntimePlan>(
    plan: &Plan,
    state: &mut RuntimeStateFor<'_, Plan>,
    function: BitArrayFunctionFunctionId,
    origin: HostCallOrigin,
    inputs: ProfiledRetainedValues<Plan::Values>,
) -> ExecutionResult<EvaluatedBitArrayFunction<Plan::Values>, Plan::Values> {
    run_tail(
        plan,
        state,
        function,
        origin,
        inputs,
        |plan, state, function, origin, inputs| {
            evaluate_entry(
                plan,
                state,
                plan.bit_array_function_function(*function),
                origin,
                inputs,
            )
        },
        |_, _, target| {
            (
                *target.function(),
                HostCallOrigin::source(target.site().clone()),
            )
        },
    )
}

pub(in crate::runtime) fn run_utf_codepoint_function<Plan: ExecutableRuntimePlan>(
    plan: &Plan,
    state: &mut RuntimeStateFor<'_, Plan>,
    function: UtfCodepointFunctionFunctionId,
    origin: HostCallOrigin,
    inputs: ProfiledRetainedValues<Plan::Values>,
) -> ExecutionResult<EvaluatedUtfCodepointFunction<Plan::Values>, Plan::Values> {
    run_tail(
        plan,
        state,
        function,
        origin,
        inputs,
        |plan, state, function, origin, inputs| {
            evaluate_entry(
                plan,
                state,
                plan.utf_codepoint_function_function(*function),
                origin,
                inputs,
            )
        },
        |_, _, target| {
            (
                *target.function(),
                HostCallOrigin::source(target.site().clone()),
            )
        },
    )
}

pub(in crate::runtime) fn run_custom_function<Plan: ExecutableRuntimePlan>(
    plan: &Plan,
    state: &mut RuntimeStateFor<'_, Plan>,
    function: CustomFunctionFunctionId,
    origin: HostCallOrigin,
    inputs: ProfiledRetainedValues<Plan::Values>,
) -> ExecutionResult<EvaluatedCustomFunction<Plan::Values>, Plan::Values> {
    run_tail(
        plan,
        state,
        function,
        origin,
        inputs,
        |plan, state, function, origin, inputs| {
            evaluate_entry(
                plan,
                state,
                plan.custom_function_function(function),
                origin,
                inputs,
            )
        },
        |_, function, target| {
            (
                function.with_index(*target.function()),
                HostCallOrigin::source(target.site().clone()),
            )
        },
    )
}

pub(in crate::runtime) fn run_external_function<Plan: ExecutableRuntimePlan>(
    plan: &Plan,
    state: &mut RuntimeStateFor<'_, Plan>,
    function: ExternalFunctionFunctionId,
    origin: HostCallOrigin,
    inputs: ProfiledRetainedValues<Plan::Values>,
) -> ExecutionResult<EvaluatedExternalFunction<Plan::Values>, Plan::Values> {
    run_tail(
        plan,
        state,
        function,
        origin,
        inputs,
        |plan, state, function, origin, inputs| {
            evaluate_entry(
                plan,
                state,
                plan.external_function_function(function),
                origin,
                inputs,
            )
        },
        |_, function, target| {
            (
                function.with_index(*target.function()),
                HostCallOrigin::source(target.site().clone()),
            )
        },
    )
}

pub(in crate::runtime) fn run_bool_function<Plan: ExecutableRuntimePlan>(
    plan: &Plan,
    state: &mut RuntimeStateFor<'_, Plan>,
    function: BoolFunctionFunctionId,
    origin: HostCallOrigin,
    inputs: ProfiledRetainedValues<Plan::Values>,
) -> ExecutionResult<EvaluatedBoolFunction<Plan::Values>, Plan::Values> {
    run_tail(
        plan,
        state,
        function,
        origin,
        inputs,
        |plan, state, function, origin, inputs| {
            evaluate_entry(
                plan,
                state,
                plan.bool_function_function(*function),
                origin,
                inputs,
            )
        },
        |_, _, target| {
            (
                *target.function(),
                HostCallOrigin::source(target.site().clone()),
            )
        },
    )
}

pub(in crate::runtime) fn run_nil_function<Plan: ExecutableRuntimePlan>(
    plan: &Plan,
    state: &mut RuntimeStateFor<'_, Plan>,
    function: NilFunctionFunctionId,
    origin: HostCallOrigin,
    inputs: ProfiledRetainedValues<Plan::Values>,
) -> ExecutionResult<EvaluatedNilFunction<Plan::Values>, Plan::Values> {
    run_tail(
        plan,
        state,
        function,
        origin,
        inputs,
        |plan, state, function, origin, inputs| {
            evaluate_entry(
                plan,
                state,
                plan.nil_function_function(*function),
                origin,
                inputs,
            )
        },
        |_, _, target| {
            (
                *target.function(),
                HostCallOrigin::source(target.site().clone()),
            )
        },
    )
}

pub(in crate::runtime) fn run_tuple_function<Plan: ExecutableRuntimePlan>(
    plan: &Plan,
    state: &mut RuntimeStateFor<'_, Plan>,
    function: TupleFunctionFunctionId,
    origin: HostCallOrigin,
    inputs: ProfiledRetainedValues<Plan::Values>,
) -> ExecutionResult<EvaluatedTupleFunction<Plan::Values>, Plan::Values> {
    run_tail(
        plan,
        state,
        function,
        origin,
        inputs,
        |plan, state, function, origin, inputs| {
            evaluate_entry(
                plan,
                state,
                plan.tuple_function_function(*function),
                origin,
                inputs,
            )
        },
        |_, _, target| {
            (
                *target.function(),
                HostCallOrigin::source(target.site().clone()),
            )
        },
    )
}

fn run_core_list_function<Plan: ExecutableRuntimePlan>(
    plan: &Plan,
    state: &mut RuntimeStateFor<'_, Plan>,
    function: ProfiledListFunctionFunctionId<Infallible>,
    origin: HostCallOrigin,
    inputs: ProfiledRetainedValues<Plan::Values>,
) -> ExecutionResult<EvaluatedListFunction<Plan::Values>, Plan::Values> {
    run_tail(
        plan,
        state,
        function,
        origin,
        inputs,
        |plan, state, function, origin, inputs| {
            evaluate_entry(
                plan,
                state,
                plan.core_list_function_function(function),
                origin,
                inputs,
            )
        },
        |_, _, target| {
            (
                target.function().clone(),
                HostCallOrigin::source(target.site().clone()),
            )
        },
    )
}

fn run_external_list_function<Plan: ExecutableRuntimePlan>(
    plan: &Plan,
    state: &mut RuntimeStateFor<'_, Plan>,
    function: ExternalListFunctionFunctionId,
    origin: HostCallOrigin,
    inputs: ProfiledRetainedValues<Plan::Values>,
) -> ExecutionResult<EvaluatedListFunction<Plan::Values>, Plan::Values> {
    run_tail(
        plan,
        state,
        function,
        origin,
        inputs,
        |plan, state, function, origin, inputs| {
            evaluate_entry(
                plan,
                state,
                plan.external_list_function_function(*function),
                origin,
                inputs,
            )
        },
        |_, _, target| {
            (
                *target.function(),
                HostCallOrigin::source(target.site().clone()),
            )
        },
    )
}

pub(in crate::runtime) fn run_function_function<Plan: ExecutableRuntimePlan>(
    plan: &Plan,
    state: &mut RuntimeStateFor<'_, Plan>,
    function: FunctionFunctionFunctionId,
    origin: HostCallOrigin,
    inputs: ProfiledRetainedValues<Plan::Values>,
) -> ExecutionResult<EvaluatedFunctionFunction<Plan::Values>, Plan::Values> {
    run_tail(
        plan,
        state,
        function,
        origin,
        inputs,
        |plan, state, function, origin, inputs| {
            evaluate_entry(
                plan,
                state,
                plan.function_function_function(function),
                origin,
                inputs,
            )
        },
        |_, function, target| {
            (
                function.with_index(*target.function()),
                HostCallOrigin::source(target.site().clone()),
            )
        },
    )
}
