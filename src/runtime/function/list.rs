use super::{evaluate, run_tail};
use crate::plan::execution::{
    BitArrayListFunctionId, BoolListFunctionId, CustomListFunctionId, ExecutionPlan,
    FloatListFunctionId, FunctionListFunctionId, IntListFunctionId, ListFunctionId,
    ListListFunctionId, NilListFunctionId, ParameterListFunctionId, ParameterListListFunctionId,
    StringListFunctionId, TupleListFunctionId, UtfCodepointListFunctionId,
};
use crate::runtime::error::ExecutionResult;
use crate::runtime::graph::RetainedValues;
use crate::runtime::state::{
    BitArrayListValueId, BoolListValueId, CustomListValueId, FloatListValueId, FunctionListValueId,
    IntListValueId, ListListValueId, ListValueId, NilListValueId, ParameterListListValueId,
    ParameterListValueId, RuntimeState, StringListValueId, TupleListValueId,
    UtfCodepointListValueId,
};

pub(in crate::runtime) fn run_parameter_list(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
    function: ParameterListFunctionId,
    inputs: RetainedValues,
) -> ExecutionResult<ParameterListValueId> {
    run_tail(
        plan,
        state,
        function,
        inputs,
        |plan, state, function, inputs| {
            evaluate(
                plan,
                state,
                plan.parameter_list_function(*function).graph(),
                inputs,
            )
        },
        |_, _, target| target,
    )
}

pub(in crate::runtime) fn run_parameter_list_list(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
    function: ParameterListListFunctionId,
    inputs: RetainedValues,
) -> ExecutionResult<ParameterListListValueId> {
    run_tail(
        plan,
        state,
        function,
        inputs,
        |plan, state, function, inputs| {
            evaluate(
                plan,
                state,
                plan.parameter_list_list_function(*function).graph(),
                inputs,
            )
        },
        |_, _, target| target,
    )
}

pub(in crate::runtime) fn run_int_list(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
    function: IntListFunctionId,
    inputs: RetainedValues,
) -> ExecutionResult<IntListValueId> {
    run_tail(
        plan,
        state,
        function,
        inputs,
        |plan, state, function, inputs| {
            evaluate(
                plan,
                state,
                plan.int_list_function(*function).graph(),
                inputs,
            )
        },
        |_, _, target| target,
    )
}

pub(in crate::runtime) fn run_string_list(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
    function: StringListFunctionId,
    inputs: RetainedValues,
) -> ExecutionResult<StringListValueId> {
    run_tail(
        plan,
        state,
        function,
        inputs,
        |plan, state, function, inputs| {
            evaluate(
                plan,
                state,
                plan.string_list_function(*function).graph(),
                inputs,
            )
        },
        |_, _, target| target,
    )
}

pub(in crate::runtime) fn run_bit_array_list(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
    function: BitArrayListFunctionId,
    inputs: RetainedValues,
) -> ExecutionResult<BitArrayListValueId> {
    run_tail(
        plan,
        state,
        function,
        inputs,
        |plan, state, function, inputs| {
            evaluate(
                plan,
                state,
                plan.bit_array_list_function(*function).graph(),
                inputs,
            )
        },
        |_, _, target| target,
    )
}

pub(in crate::runtime) fn run_utf_codepoint_list(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
    function: UtfCodepointListFunctionId,
    inputs: RetainedValues,
) -> ExecutionResult<UtfCodepointListValueId> {
    run_tail(
        plan,
        state,
        function,
        inputs,
        |plan, state, function, inputs| {
            evaluate(
                plan,
                state,
                plan.utf_codepoint_list_function(*function).graph(),
                inputs,
            )
        },
        |_, _, target| target,
    )
}

pub(in crate::runtime) fn run_custom_list(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
    function: CustomListFunctionId,
    inputs: RetainedValues,
) -> ExecutionResult<CustomListValueId> {
    run_tail(
        plan,
        state,
        function,
        inputs,
        |plan, state, function, inputs| {
            evaluate(
                plan,
                state,
                plan.custom_list_function(*function).graph(),
                inputs,
            )
        },
        |_, _, target| target,
    )
}

pub(in crate::runtime) fn run_float_list(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
    function: FloatListFunctionId,
    inputs: RetainedValues,
) -> ExecutionResult<FloatListValueId> {
    run_tail(
        plan,
        state,
        function,
        inputs,
        |plan, state, function, inputs| {
            evaluate(
                plan,
                state,
                plan.float_list_function(*function).graph(),
                inputs,
            )
        },
        |_, _, target| target,
    )
}

pub(in crate::runtime) fn run_bool_list(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
    function: BoolListFunctionId,
    inputs: RetainedValues,
) -> ExecutionResult<BoolListValueId> {
    run_tail(
        plan,
        state,
        function,
        inputs,
        |plan, state, function, inputs| {
            evaluate(
                plan,
                state,
                plan.bool_list_function(*function).graph(),
                inputs,
            )
        },
        |_, _, target| target,
    )
}

pub(in crate::runtime) fn run_nil_list(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
    function: NilListFunctionId,
    inputs: RetainedValues,
) -> ExecutionResult<NilListValueId> {
    run_tail(
        plan,
        state,
        function,
        inputs,
        |plan, state, function, inputs| {
            evaluate(
                plan,
                state,
                plan.nil_list_function(*function).graph(),
                inputs,
            )
        },
        |_, _, target| target,
    )
}

pub(in crate::runtime) fn run_tuple_list(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
    function: TupleListFunctionId,
    inputs: RetainedValues,
) -> ExecutionResult<TupleListValueId> {
    run_tail(
        plan,
        state,
        function,
        inputs,
        |plan, state, function, inputs| {
            evaluate(
                plan,
                state,
                plan.tuple_list_function(*function).graph(),
                inputs,
            )
        },
        |_, _, target| target,
    )
}

pub(in crate::runtime) fn run_list_list(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
    function: ListListFunctionId,
    inputs: RetainedValues,
) -> ExecutionResult<ListListValueId> {
    run_tail(
        plan,
        state,
        function,
        inputs,
        |plan, state, function, inputs| {
            evaluate(
                plan,
                state,
                plan.list_list_function(*function).graph(),
                inputs,
            )
        },
        |_, _, target| target,
    )
}

pub(in crate::runtime) fn run_function_list(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
    function: FunctionListFunctionId,
    inputs: RetainedValues,
) -> ExecutionResult<FunctionListValueId> {
    run_tail(
        plan,
        state,
        function,
        inputs,
        |plan, state, function, inputs| {
            evaluate(
                plan,
                state,
                plan.function_list_function(*function).graph(),
                inputs,
            )
        },
        |_, _, target| target,
    )
}

pub(in crate::runtime) fn run_list(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
    function: ListFunctionId,
    inputs: RetainedValues,
) -> ExecutionResult<ListValueId> {
    match function {
        ListFunctionId::Parameter(function) => {
            run_parameter_list(plan, state, function, inputs).map(ListValueId::Parameter)
        }
        ListFunctionId::ParameterList(function) => {
            run_parameter_list_list(plan, state, function, inputs).map(ListValueId::ParameterList)
        }
        ListFunctionId::Int(function) => {
            run_int_list(plan, state, function, inputs).map(ListValueId::Int)
        }
        ListFunctionId::String(function) => {
            run_string_list(plan, state, function, inputs).map(ListValueId::String)
        }
        ListFunctionId::BitArray(function) => {
            run_bit_array_list(plan, state, function, inputs).map(ListValueId::BitArray)
        }
        ListFunctionId::UtfCodepoint(function) => {
            run_utf_codepoint_list(plan, state, function, inputs).map(ListValueId::UtfCodepoint)
        }
        ListFunctionId::Custom(function) => {
            run_custom_list(plan, state, function, inputs).map(ListValueId::Custom)
        }
        ListFunctionId::Float(function) => {
            run_float_list(plan, state, function, inputs).map(ListValueId::Float)
        }
        ListFunctionId::Bool(function) => {
            run_bool_list(plan, state, function, inputs).map(ListValueId::Bool)
        }
        ListFunctionId::Nil(function) => {
            run_nil_list(plan, state, function, inputs).map(ListValueId::Nil)
        }
        ListFunctionId::Tuple(function) => {
            run_tuple_list(plan, state, function, inputs).map(ListValueId::Tuple)
        }
        ListFunctionId::List(function) => {
            run_list_list(plan, state, function, inputs).map(ListValueId::List)
        }
        ListFunctionId::Function(function) => {
            run_function_list(plan, state, function, inputs).map(ListValueId::Function)
        }
    }
}
