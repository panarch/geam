use super::{GraphExit, evaluate};
use crate::plan::execution::{
    BitArrayFunctionFunctionId, BitArrayFunctionId, BitArrayListFunctionId, BoolFunctionFunctionId,
    BoolFunctionId, BoolListFunctionId, CustomFunctionFunctionId, CustomFunctionId,
    CustomListFunctionId, ExecutionPlan, FloatFunctionFunctionId, FloatFunctionId,
    FloatListFunctionId, FunctionFunctionFunctionId, FunctionFunctionId, FunctionListFunctionId,
    GenericFunctionFunctionId, IntFunctionFunctionId, IntFunctionId, IntListFunctionId,
    ListFunctionFunctionId, ListFunctionId, ListListFunctionId, NeverFunctionFunctionId,
    NeverFunctionId, NilFunctionFunctionId, NilFunctionId, NilListFunctionId,
    ParameterListFunctionId, ParameterListListFunctionId, StringFunctionFunctionId,
    StringFunctionId, StringListFunctionId, TupleFunctionFunctionId, TupleFunctionId,
    TupleListFunctionId, UtfCodepointFunctionFunctionId, UtfCodepointFunctionId,
    UtfCodepointListFunctionId,
};
use crate::runtime::environment::RetainedValues;
use crate::runtime::error::ExecutionResult;
use crate::runtime::evaluated::{
    EvaluatedBitArray, EvaluatedBitArrayFunction, EvaluatedBoolFunction, EvaluatedCustomFunction,
    EvaluatedCustomValue, EvaluatedFloatFunction, EvaluatedFunctionFunction,
    EvaluatedFunctionValue, EvaluatedGenericFunction, EvaluatedIntFunction, EvaluatedListFunction,
    EvaluatedNeverFunction, EvaluatedNilFunction, EvaluatedStringFunction, EvaluatedTupleFunction,
    EvaluatedUtfCodepointFunction, EvaluatedValue,
};
use crate::runtime::state::{
    BitArrayListValueId, BoolListValueId, CustomListValueId, FloatListValueId, FunctionListValueId,
    IntListValueId, ListListValueId, ListValueId, NilListValueId, ParameterListListValueId,
    ParameterListValueId, RuntimeState, StringListValueId, TupleListValueId,
    UtfCodepointListValueId,
};
use ecow::EcoString;
use num_bigint::BigInt;
use std::convert::Infallible;

fn run_tail<Id, Return, TailCall>(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
    mut function: Id,
    mut inputs: RetainedValues,
    execute: impl Fn(
        &ExecutionPlan,
        &mut RuntimeState,
        &Id,
        RetainedValues,
    ) -> ExecutionResult<GraphExit<Return, TailCall>>,
    next: impl Fn(&ExecutionPlan, &Id, TailCall) -> Id,
) -> ExecutionResult<Return> {
    loop {
        match execute(plan, state, &function, inputs)? {
            GraphExit::Return(value) => return Ok(value),
            GraphExit::TailCall {
                function: target,
                args,
            } => {
                function = next(plan, &function, target);
                inputs = args;
            }
        }
    }
}

pub(super) fn run_never(
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
            evaluate(plan, state, plan.never_function(*function).graph(), inputs)
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
            evaluate(plan, state, plan.int_function(*function).graph(), inputs)
        },
        |_, _, target| target,
    )
}

pub(super) fn run_float(
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
            evaluate(plan, state, plan.float_function(*function).graph(), inputs)
        },
        |_, _, target| target,
    )
}

pub(super) fn run_string(
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
            evaluate(plan, state, plan.string_function(*function).graph(), inputs)
        },
        |_, _, target| target,
    )
}

pub(super) fn run_bit_array(
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
                plan.bit_array_function(*function).graph(),
                inputs,
            )
        },
        |_, _, target| target,
    )
}

pub(super) fn run_utf_codepoint(
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
                plan.utf_codepoint_function(*function).graph(),
                inputs,
            )
        },
        |_, _, target| target,
    )
}

pub(super) fn run_custom(
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
                plan.custom_function(*function).graph().body(),
                inputs,
            )
        },
        |plan, function, target| plan.custom_function(*function).graph().function_id(target),
    )
}

pub(super) fn run_bool(
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
            evaluate(plan, state, plan.bool_function(*function).graph(), inputs)
        },
        |_, _, target| target,
    )
}

pub(super) fn run_nil(
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
            evaluate(plan, state, plan.nil_function(*function).graph(), inputs)
        },
        |_, _, target| target,
    )
}

pub(super) fn run_tuple(
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
            evaluate(plan, state, plan.tuple_function(*function).graph(), inputs)
        },
        |_, _, target| target,
    )
}

pub(super) fn run_parameter_list(
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

pub(super) fn run_parameter_list_list(
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

pub(super) fn run_string_list(
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

pub(super) fn run_bit_array_list(
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

pub(super) fn run_utf_codepoint_list(
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

pub(super) fn run_custom_list(
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

pub(super) fn run_float_list(
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

pub(super) fn run_bool_list(
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

pub(super) fn run_nil_list(
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

pub(super) fn run_tuple_list(
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

pub(super) fn run_list_list(
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

pub(super) fn run_function_list(
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

pub(super) fn run_list(
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

pub(super) fn run_int_function(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
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
                plan.int_function_function(*function).graph().body(),
                inputs,
            )
        },
        |_, _, target| target,
    )
}

pub(super) fn run_float_function(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
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
                plan.float_function_function(*function).graph().body(),
                inputs,
            )
        },
        |_, _, target| target,
    )
}

pub(super) fn run_string_function(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
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
                plan.string_function_function(*function).graph().body(),
                inputs,
            )
        },
        |_, _, target| target,
    )
}

pub(super) fn run_bit_array_function(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
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
                plan.bit_array_function_function(*function).graph().body(),
                inputs,
            )
        },
        |_, _, target| target,
    )
}

pub(super) fn run_utf_codepoint_function(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
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
                    .graph()
                    .body(),
                inputs,
            )
        },
        |_, _, target| target,
    )
}

pub(super) fn run_generic_function(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
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
                plan.generic_function_function(function).graph().body(),
                inputs,
            )
        },
        |_, _, target| target,
    )
}

pub(super) fn run_never_function(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
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
                plan.never_function_function(function).graph().body(),
                inputs,
            )
        },
        |_, _, target| target,
    )
}

pub(super) fn run_custom_function(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
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
                plan.custom_function_function(function).graph().body(),
                inputs,
            )
        },
        |plan, function, target| {
            plan.custom_function_function(function)
                .graph()
                .function_id(target)
        },
    )
}

pub(super) fn run_bool_function(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
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
                plan.bool_function_function(*function).graph().body(),
                inputs,
            )
        },
        |_, _, target| target,
    )
}

pub(super) fn run_nil_function(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
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
                plan.nil_function_function(*function).graph().body(),
                inputs,
            )
        },
        |_, _, target| target,
    )
}

pub(super) fn run_tuple_function(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
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
                plan.tuple_function_function(*function).graph().body(),
                inputs,
            )
        },
        |_, _, target| target,
    )
}

pub(super) fn run_list_function(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
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
                plan.list_function_function(function).graph().body(),
                inputs,
            )
        },
        |_, _, target| target,
    )
}

pub(super) fn run_function_function(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
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
                plan.function_function_function(function).graph().body(),
                inputs,
            )
        },
        |plan, function, target| {
            plan.function_function_function(function)
                .graph()
                .function_id(target)
        },
    )
}

pub(super) fn run_function(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
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

pub(super) fn run_never_value(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
    function: EvaluatedNeverFunction,
    mut inputs: RetainedValues,
) -> ExecutionResult<Infallible> {
    inputs.append_captures(function.captures());
    run_never(plan, state, function.runtime_id(), inputs)
}
