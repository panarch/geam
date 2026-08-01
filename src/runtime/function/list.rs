use super::{evaluate_entry, run_tail};
use crate::plan::execution::function::{
    BitArrayListFunctionId, BoolListFunctionId, CustomListFunctionId, ExternalListFunctionId,
    FloatListFunctionId, FunctionListFunctionId, IntListFunctionId, ListFunctionId,
    ListListFunctionId, NilListFunctionId, ParameterListFunctionId, ParameterListListFunctionId,
    RuntimeListFunctionId, StringListFunctionId, TupleListFunctionId, UtfCodepointListFunctionId,
};
use crate::runtime::ExecutableRuntimePlan;
use crate::runtime::error::{ExecutionResult, HostCallOrigin};
use crate::runtime::graph::RetainedValues;
use crate::runtime::state::{
    BitArrayListValueId, BoolListValueId, CustomListValueId, ExternalListValueId, FloatListValueId,
    FunctionListValueId, IntListValueId, ListListValueId, ListValueId, NilListValueId,
    ParameterListListValueId, ParameterListValueId, RuntimeStateFor, StringListValueId,
    TupleListValueId, UtfCodepointListValueId,
};

pub(in crate::runtime) fn run_parameter_list<Plan: ExecutableRuntimePlan>(
    plan: &Plan,
    state: &mut RuntimeStateFor<'_, Plan>,
    function: ParameterListFunctionId,
    origin: HostCallOrigin,
    inputs: RetainedValues,
) -> ExecutionResult<ParameterListValueId> {
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
                plan.parameter_list_function(*function),
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

pub(in crate::runtime) fn run_parameter_list_list<Plan: ExecutableRuntimePlan>(
    plan: &Plan,
    state: &mut RuntimeStateFor<'_, Plan>,
    function: ParameterListListFunctionId,
    origin: HostCallOrigin,
    inputs: RetainedValues,
) -> ExecutionResult<ParameterListListValueId> {
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
                plan.parameter_list_list_function(*function),
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

pub(in crate::runtime) fn run_int_list<Plan: ExecutableRuntimePlan>(
    plan: &Plan,
    state: &mut RuntimeStateFor<'_, Plan>,
    function: IntListFunctionId,
    origin: HostCallOrigin,
    inputs: RetainedValues,
) -> ExecutionResult<IntListValueId> {
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
                plan.int_list_function(*function),
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

pub(in crate::runtime) fn run_string_list<Plan: ExecutableRuntimePlan>(
    plan: &Plan,
    state: &mut RuntimeStateFor<'_, Plan>,
    function: StringListFunctionId,
    origin: HostCallOrigin,
    inputs: RetainedValues,
) -> ExecutionResult<StringListValueId> {
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
                plan.string_list_function(*function),
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

pub(in crate::runtime) fn run_bit_array_list<Plan: ExecutableRuntimePlan>(
    plan: &Plan,
    state: &mut RuntimeStateFor<'_, Plan>,
    function: BitArrayListFunctionId,
    origin: HostCallOrigin,
    inputs: RetainedValues,
) -> ExecutionResult<BitArrayListValueId> {
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
                plan.bit_array_list_function(*function),
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

pub(in crate::runtime) fn run_utf_codepoint_list<Plan: ExecutableRuntimePlan>(
    plan: &Plan,
    state: &mut RuntimeStateFor<'_, Plan>,
    function: UtfCodepointListFunctionId,
    origin: HostCallOrigin,
    inputs: RetainedValues,
) -> ExecutionResult<UtfCodepointListValueId> {
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
                plan.utf_codepoint_list_function(*function),
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

pub(in crate::runtime) fn run_custom_list<Plan: ExecutableRuntimePlan>(
    plan: &Plan,
    state: &mut RuntimeStateFor<'_, Plan>,
    function: CustomListFunctionId,
    origin: HostCallOrigin,
    inputs: RetainedValues,
) -> ExecutionResult<CustomListValueId> {
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
                plan.custom_list_function(*function),
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

pub(in crate::runtime) fn run_external_list<Plan: ExecutableRuntimePlan>(
    plan: &Plan,
    state: &mut RuntimeStateFor<'_, Plan>,
    function: ExternalListFunctionId,
    origin: HostCallOrigin,
    inputs: RetainedValues,
) -> ExecutionResult<ExternalListValueId> {
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
                plan.external_list_function(*function),
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

pub(in crate::runtime) fn run_float_list<Plan: ExecutableRuntimePlan>(
    plan: &Plan,
    state: &mut RuntimeStateFor<'_, Plan>,
    function: FloatListFunctionId,
    origin: HostCallOrigin,
    inputs: RetainedValues,
) -> ExecutionResult<FloatListValueId> {
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
                plan.float_list_function(*function),
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

pub(in crate::runtime) fn run_bool_list<Plan: ExecutableRuntimePlan>(
    plan: &Plan,
    state: &mut RuntimeStateFor<'_, Plan>,
    function: BoolListFunctionId,
    origin: HostCallOrigin,
    inputs: RetainedValues,
) -> ExecutionResult<BoolListValueId> {
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
                plan.bool_list_function(*function),
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

pub(in crate::runtime) fn run_nil_list<Plan: ExecutableRuntimePlan>(
    plan: &Plan,
    state: &mut RuntimeStateFor<'_, Plan>,
    function: NilListFunctionId,
    origin: HostCallOrigin,
    inputs: RetainedValues,
) -> ExecutionResult<NilListValueId> {
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
                plan.nil_list_function(*function),
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

pub(in crate::runtime) fn run_tuple_list<Plan: ExecutableRuntimePlan>(
    plan: &Plan,
    state: &mut RuntimeStateFor<'_, Plan>,
    function: TupleListFunctionId,
    origin: HostCallOrigin,
    inputs: RetainedValues,
) -> ExecutionResult<TupleListValueId> {
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
                plan.tuple_list_function(*function),
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

pub(in crate::runtime) fn run_list_list<Plan: ExecutableRuntimePlan>(
    plan: &Plan,
    state: &mut RuntimeStateFor<'_, Plan>,
    function: ListListFunctionId,
    origin: HostCallOrigin,
    inputs: RetainedValues,
) -> ExecutionResult<ListListValueId> {
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
                plan.list_list_function(*function),
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

pub(in crate::runtime) fn run_function_list<Plan: ExecutableRuntimePlan>(
    plan: &Plan,
    state: &mut RuntimeStateFor<'_, Plan>,
    function: FunctionListFunctionId,
    origin: HostCallOrigin,
    inputs: RetainedValues,
) -> ExecutionResult<FunctionListValueId> {
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
                plan.function_list_function(*function),
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

pub(in crate::runtime) fn run_list<Plan: ExecutableRuntimePlan>(
    plan: &Plan,
    state: &mut RuntimeStateFor<'_, Plan>,
    function: RuntimeListFunctionId,
    origin: HostCallOrigin,
    inputs: RetainedValues,
) -> ExecutionResult<ListValueId> {
    match function {
        RuntimeListFunctionId::Core(function) => {
            run_core_list(plan, state, function, origin, inputs)
        }
        RuntimeListFunctionId::External(function) => {
            run_external_list(plan, state, function, origin, inputs).map(ListValueId::External)
        }
    }
}

pub(in crate::runtime) fn run_core_list<Plan: ExecutableRuntimePlan>(
    plan: &Plan,
    state: &mut RuntimeStateFor<'_, Plan>,
    function: ListFunctionId,
    origin: HostCallOrigin,
    inputs: RetainedValues,
) -> ExecutionResult<ListValueId> {
    match function {
        ListFunctionId::Parameter(function) => {
            run_parameter_list(plan, state, function, origin, inputs).map(ListValueId::Parameter)
        }
        ListFunctionId::ParameterList(function) => {
            run_parameter_list_list(plan, state, function, origin, inputs)
                .map(ListValueId::ParameterList)
        }
        ListFunctionId::Int(function) => {
            run_int_list(plan, state, function, origin, inputs).map(ListValueId::Int)
        }
        ListFunctionId::String(function) => {
            run_string_list(plan, state, function, origin, inputs).map(ListValueId::String)
        }
        ListFunctionId::BitArray(function) => {
            run_bit_array_list(plan, state, function, origin, inputs).map(ListValueId::BitArray)
        }
        ListFunctionId::UtfCodepoint(function) => {
            run_utf_codepoint_list(plan, state, function, origin, inputs)
                .map(ListValueId::UtfCodepoint)
        }
        ListFunctionId::Custom(function) => {
            run_custom_list(plan, state, function, origin, inputs).map(ListValueId::Custom)
        }
        ListFunctionId::Float(function) => {
            run_float_list(plan, state, function, origin, inputs).map(ListValueId::Float)
        }
        ListFunctionId::Bool(function) => {
            run_bool_list(plan, state, function, origin, inputs).map(ListValueId::Bool)
        }
        ListFunctionId::Nil(function) => {
            run_nil_list(plan, state, function, origin, inputs).map(ListValueId::Nil)
        }
        ListFunctionId::Tuple(function) => {
            run_tuple_list(plan, state, function, origin, inputs).map(ListValueId::Tuple)
        }
        ListFunctionId::List(function) => {
            run_list_list(plan, state, function, origin, inputs).map(ListValueId::List)
        }
        ListFunctionId::Function(function) => {
            run_function_list(plan, state, function, origin, inputs).map(ListValueId::Function)
        }
    }
}
