use super::run;
use crate::plan::execution::ExecutionPlan;
use crate::plan::execution::function::{
    BitArrayListFunctionId, BoolListFunctionId, CustomListFunctionId, FloatListFunctionId,
    FunctionListFunctionId, IntListFunctionId, ListFunctionId, ListListFunctionId,
    NilListFunctionId, ParameterListFunctionId, ParameterListListFunctionId,
    ProfiledListFunctionId, StringListFunctionId, TupleListFunctionId, UtfCodepointListFunctionId,
};
use crate::runtime::error::{ExecutionResult, HostCallOrigin};
use crate::runtime::graph::RetainedValues;
use crate::runtime::state::RuntimeState;
use crate::runtime::state::list::{
    BitArrayListValueId, BoolListValueId, CustomListValueId, FloatListValueId, FunctionListValueId,
    IntListValueId, ListListValueId, ListValueId, NilListValueId, ParameterListListValueId,
    ParameterListValueId, StringListValueId, TupleListValueId, UtfCodepointListValueId,
};

pub(in crate::runtime) fn run_list(
    plan: &ExecutionPlan,
    state: &mut RuntimeState<'_>,
    function: ProfiledListFunctionId<std::convert::Infallible>,
    origin: HostCallOrigin,
    inputs: RetainedValues,
) -> ExecutionResult<ListValueId> {
    match function {
        ProfiledListFunctionId::Core(function) => {
            run_core_list(plan, state, function, origin, inputs)
        }
        ProfiledListFunctionId::External(never) => match never {},
    }
}

pub(in crate::runtime) fn run_core_list(
    plan: &ExecutionPlan,
    state: &mut RuntimeState<'_>,
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

pub(in crate::runtime) fn run_parameter_list(
    plan: &ExecutionPlan,
    state: &mut RuntimeState<'_>,
    function: ParameterListFunctionId,
    origin: HostCallOrigin,
    inputs: RetainedValues,
) -> ExecutionResult<ParameterListValueId> {
    run(plan, state, function, origin, inputs)
}

pub(in crate::runtime) fn run_parameter_list_list(
    plan: &ExecutionPlan,
    state: &mut RuntimeState<'_>,
    function: ParameterListListFunctionId,
    origin: HostCallOrigin,
    inputs: RetainedValues,
) -> ExecutionResult<ParameterListListValueId> {
    run(plan, state, function, origin, inputs)
}

pub(in crate::runtime) fn run_int_list(
    plan: &ExecutionPlan,
    state: &mut RuntimeState<'_>,
    function: IntListFunctionId,
    origin: HostCallOrigin,
    inputs: RetainedValues,
) -> ExecutionResult<IntListValueId> {
    run(plan, state, function, origin, inputs)
}

pub(in crate::runtime) fn run_string_list(
    plan: &ExecutionPlan,
    state: &mut RuntimeState<'_>,
    function: StringListFunctionId,
    origin: HostCallOrigin,
    inputs: RetainedValues,
) -> ExecutionResult<StringListValueId> {
    run(plan, state, function, origin, inputs)
}

pub(in crate::runtime) fn run_bit_array_list(
    plan: &ExecutionPlan,
    state: &mut RuntimeState<'_>,
    function: BitArrayListFunctionId,
    origin: HostCallOrigin,
    inputs: RetainedValues,
) -> ExecutionResult<BitArrayListValueId> {
    run(plan, state, function, origin, inputs)
}

pub(in crate::runtime) fn run_utf_codepoint_list(
    plan: &ExecutionPlan,
    state: &mut RuntimeState<'_>,
    function: UtfCodepointListFunctionId,
    origin: HostCallOrigin,
    inputs: RetainedValues,
) -> ExecutionResult<UtfCodepointListValueId> {
    run(plan, state, function, origin, inputs)
}

pub(in crate::runtime) fn run_custom_list(
    plan: &ExecutionPlan,
    state: &mut RuntimeState<'_>,
    function: CustomListFunctionId,
    origin: HostCallOrigin,
    inputs: RetainedValues,
) -> ExecutionResult<CustomListValueId> {
    run(plan, state, function, origin, inputs)
}

pub(in crate::runtime) fn run_float_list(
    plan: &ExecutionPlan,
    state: &mut RuntimeState<'_>,
    function: FloatListFunctionId,
    origin: HostCallOrigin,
    inputs: RetainedValues,
) -> ExecutionResult<FloatListValueId> {
    run(plan, state, function, origin, inputs)
}

pub(in crate::runtime) fn run_bool_list(
    plan: &ExecutionPlan,
    state: &mut RuntimeState<'_>,
    function: BoolListFunctionId,
    origin: HostCallOrigin,
    inputs: RetainedValues,
) -> ExecutionResult<BoolListValueId> {
    run(plan, state, function, origin, inputs)
}

pub(in crate::runtime) fn run_nil_list(
    plan: &ExecutionPlan,
    state: &mut RuntimeState<'_>,
    function: NilListFunctionId,
    origin: HostCallOrigin,
    inputs: RetainedValues,
) -> ExecutionResult<NilListValueId> {
    run(plan, state, function, origin, inputs)
}

pub(in crate::runtime) fn run_tuple_list(
    plan: &ExecutionPlan,
    state: &mut RuntimeState<'_>,
    function: TupleListFunctionId,
    origin: HostCallOrigin,
    inputs: RetainedValues,
) -> ExecutionResult<TupleListValueId> {
    run(plan, state, function, origin, inputs)
}

pub(in crate::runtime) fn run_list_list(
    plan: &ExecutionPlan,
    state: &mut RuntimeState<'_>,
    function: ListListFunctionId,
    origin: HostCallOrigin,
    inputs: RetainedValues,
) -> ExecutionResult<ListListValueId> {
    run(plan, state, function, origin, inputs)
}

pub(in crate::runtime) fn run_function_list(
    plan: &ExecutionPlan,
    state: &mut RuntimeState<'_>,
    function: FunctionListFunctionId,
    origin: HostCallOrigin,
    inputs: RetainedValues,
) -> ExecutionResult<FunctionListValueId> {
    run(plan, state, function, origin, inputs)
}
