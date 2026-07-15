mod bind;
pub(in crate::runtime) mod return_body;
mod steps;

pub(in crate::runtime) use bind::eval_capture_args;
pub(in crate::runtime) use steps::{execute_steps, match_and_apply_assert_pattern};

use crate::plan::execution::ExecutionPlan;
use crate::plan::execution::{
    BitArrayFunctionFunctionId, BitArrayFunctionId, BoolFunctionFunctionId, BoolFunctionId,
    CallArg, CustomFunctionFunctionId, CustomFunctionId, FloatFunctionFunctionId, FloatFunctionId,
    FunctionFunctionFunctionId, FunctionReturnFamily, IntFunctionFunctionId, IntFunctionId,
    ListFunctionFunctionId, ListFunctionId, NilFunctionFunctionId, NilFunctionId,
    RuntimeFunctionId, StringFunctionFunctionId, StringFunctionId, TupleFunctionFunctionId,
    TupleFunctionId, UtfCodepointFunctionFunctionId, UtfCodepointFunctionId,
};
use crate::runtime::error::ExecutionResult;
use crate::runtime::expression::{
    eval_bit_array_function_expr, eval_bool_function_expr, eval_custom_function_expr,
    eval_float_function_expr, eval_function_function_expr, eval_int_function_expr,
    eval_list_function_expr, eval_nil_function_expr, eval_string_function_expr,
    eval_tuple_function_expr, eval_utf_codepoint_function_expr,
};
use crate::runtime::frame::Frame;
use crate::runtime::state::{
    BitArrayListValueId, BoolListValueId, CustomListValueId, FloatListValueId, FunctionListValueId,
    IntListValueId, ListListValueId, ListValueId, NilListValueId, RuntimeState, StringListValueId,
    TupleListValueId, UtfCodepointListValueId,
};
use crate::runtime::{
    EvaluatedBitArray, EvaluatedBitArrayFunction, EvaluatedBoolFunction, EvaluatedCustomFunction,
    EvaluatedCustomValue, EvaluatedFloatFunction, EvaluatedFunctionFunction,
    EvaluatedFunctionValue, EvaluatedIntFunction, EvaluatedListFunction, EvaluatedNilFunction,
    EvaluatedStringFunction, EvaluatedTupleFunction, EvaluatedUtfCodepointFunction, EvaluatedValue,
    ExecutionError, Value,
};
use bind::{bind_arguments, bind_function_value_arguments, eval_call_argument_values};
use ecow::EcoString;
use num_bigint::BigInt;
use return_body::{
    run_bit_array_function_loop, run_bit_array_list_loop, run_bit_array_loop,
    run_bool_function_loop, run_bool_list_loop, run_bool_loop, run_custom_function_loop,
    run_custom_list_loop, run_custom_loop, run_float_function_loop, run_float_list_loop,
    run_float_loop, run_function_function_loop, run_function_list_loop, run_int_function_loop,
    run_int_list_loop, run_int_loop, run_list_function_loop, run_list_list_loop, run_list_loop,
    run_nil_function_loop, run_nil_list_loop, run_nil_loop, run_string_function_loop,
    run_string_list_loop, run_string_loop, run_tuple_function_loop, run_tuple_list_loop,
    run_tuple_loop, run_utf_codepoint_function_loop, run_utf_codepoint_list_loop,
    run_utf_codepoint_loop,
};

pub(super) fn run_main(plan: &ExecutionPlan) -> ExecutionResult<Value> {
    let mut state = RuntimeState::new();
    let empty_layout = crate::plan::execution::FrameLayout::default();
    let mut caller_frame = Frame::new(&empty_layout, &mut state);
    let value = match plan.main_runtime() {
        RuntimeFunctionId::Int(function) => {
            run_int_call(plan, &mut state, function, &[], &mut caller_frame)
                .map(EvaluatedValue::Int)
        }
        RuntimeFunctionId::Float(function) => {
            run_float_call(plan, &mut state, function, &[], &mut caller_frame)
                .map(EvaluatedValue::Float)
        }
        RuntimeFunctionId::String(function) => {
            run_string_call(plan, &mut state, function, &[], &mut caller_frame)
                .map(EvaluatedValue::String)
        }
        RuntimeFunctionId::BitArray(function) => {
            run_bit_array_call(plan, &mut state, function, &[], &mut caller_frame)
                .map(EvaluatedValue::BitArray)
        }
        RuntimeFunctionId::UtfCodepoint(function) => {
            run_utf_codepoint_call(plan, &mut state, function, &[], &mut caller_frame)
                .map(EvaluatedValue::UtfCodepoint)
        }
        RuntimeFunctionId::Custom(function) => {
            run_custom_call(plan, &mut state, function, &[], &mut caller_frame)
                .map(EvaluatedValue::Custom)
        }
        RuntimeFunctionId::Bool(function) => {
            run_bool_call(plan, &mut state, function, &[], &mut caller_frame)
                .map(EvaluatedValue::Bool)
        }
        RuntimeFunctionId::Nil(function) => {
            run_nil_call(plan, &mut state, function, &[], &mut caller_frame)
                .map(|_| EvaluatedValue::Nil)
        }
        RuntimeFunctionId::Tuple { id, .. } => {
            run_tuple_call(plan, &mut state, id, &[], &mut caller_frame).map(EvaluatedValue::Tuple)
        }
        RuntimeFunctionId::List(id) => {
            run_list_call(plan, &mut state, id, &[], &mut caller_frame).map(EvaluatedValue::List)
        }
        RuntimeFunctionId::Function { id, .. } => {
            run_function_returning_function_call(plan, &mut state, id, &[], &mut caller_frame)
                .map(EvaluatedValue::Function)
        }
    }?;
    state.drain_releases();
    Ok(crate::runtime::materialize::value(plan, &state, value))
}

pub(super) fn run_int_call(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
    function: IntFunctionId,
    args: &[CallArg],
    caller_frame: &mut Frame,
) -> ExecutionResult<BigInt> {
    let frame = bind_arguments(
        plan,
        state,
        args,
        caller_frame,
        plan.int_function(function).frame_layout(),
    )?;
    run_int_loop(plan, state, function, frame)
}

pub(super) fn run_float_call(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
    function: FloatFunctionId,
    args: &[CallArg],
    caller_frame: &mut Frame,
) -> ExecutionResult<f64> {
    let frame = bind_arguments(
        plan,
        state,
        args,
        caller_frame,
        plan.float_function(function).frame_layout(),
    )?;
    run_float_loop(plan, state, function, frame)
}

pub(super) fn run_string_call(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
    function: StringFunctionId,
    args: &[CallArg],
    caller_frame: &mut Frame,
) -> ExecutionResult<EcoString> {
    let frame = bind_arguments(
        plan,
        state,
        args,
        caller_frame,
        plan.string_function(function).frame_layout(),
    )?;
    run_string_loop(plan, state, function, frame)
}

pub(super) fn run_bit_array_call(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
    function: BitArrayFunctionId,
    args: &[CallArg],
    caller_frame: &mut Frame,
) -> ExecutionResult<EvaluatedBitArray> {
    let frame = bind_arguments(
        plan,
        state,
        args,
        caller_frame,
        plan.bit_array_function(function).frame_layout(),
    )?;
    run_bit_array_loop(plan, state, function, frame)
}

pub(super) fn run_utf_codepoint_call(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
    function: UtfCodepointFunctionId,
    args: &[CallArg],
    caller_frame: &mut Frame,
) -> ExecutionResult<char> {
    let frame = bind_arguments(
        plan,
        state,
        args,
        caller_frame,
        plan.utf_codepoint_function(function).frame_layout(),
    )?;
    run_utf_codepoint_loop(plan, state, function, frame)
}

pub(super) fn run_custom_call(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
    function: CustomFunctionId,
    args: &[CallArg],
    caller_frame: &mut Frame,
) -> ExecutionResult<EvaluatedCustomValue> {
    let frame = bind_arguments(
        plan,
        state,
        args,
        caller_frame,
        plan.custom_function(function).frame_layout(),
    )?;
    run_custom_loop(plan, state, function, frame)
}

pub(super) fn run_bool_call(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
    function: BoolFunctionId,
    args: &[CallArg],
    caller_frame: &mut Frame,
) -> ExecutionResult<bool> {
    let frame = bind_arguments(
        plan,
        state,
        args,
        caller_frame,
        plan.bool_function(function).frame_layout(),
    )?;
    run_bool_loop(plan, state, function, frame)
}

pub(super) fn run_nil_call(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
    function: NilFunctionId,
    args: &[CallArg],
    caller_frame: &mut Frame,
) -> ExecutionResult<()> {
    let frame = bind_arguments(
        plan,
        state,
        args,
        caller_frame,
        plan.nil_function(function).frame_layout(),
    )?;
    run_nil_loop(plan, state, function, frame)
}

pub(super) fn run_tuple_call(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
    function: TupleFunctionId,
    args: &[CallArg],
    caller_frame: &mut Frame,
) -> ExecutionResult<Vec<EvaluatedValue>> {
    let frame = bind_arguments(
        plan,
        state,
        args,
        caller_frame,
        plan.tuple_function(function).frame_layout(),
    )?;
    run_tuple_loop(plan, state, function, frame)
}

pub(super) fn run_list_call(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
    function: ListFunctionId,
    args: &[CallArg],
    caller_frame: &mut Frame,
) -> ExecutionResult<ListValueId> {
    let frame = bind_arguments(
        plan,
        state,
        args,
        caller_frame,
        list_function_frame_layout(plan, &function),
    )?;
    run_list_loop(plan, state, function, frame)
}

pub(in crate::runtime) fn run_int_list_call(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
    function: crate::plan::execution::IntListFunctionId,
    args: &[CallArg],
    caller_frame: &mut Frame,
) -> ExecutionResult<IntListValueId> {
    let frame = bind_arguments(
        plan,
        state,
        args,
        caller_frame,
        plan.int_list_function(function).frame_layout(),
    )?;
    run_int_list_loop(plan, state, function, frame)
}

pub(in crate::runtime) fn run_string_list_call(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
    function: crate::plan::execution::StringListFunctionId,
    args: &[CallArg],
    caller_frame: &mut Frame,
) -> ExecutionResult<StringListValueId> {
    let frame = bind_arguments(
        plan,
        state,
        args,
        caller_frame,
        plan.string_list_function(function).frame_layout(),
    )?;
    run_string_list_loop(plan, state, function, frame)
}

pub(in crate::runtime) fn run_bit_array_list_call(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
    function: crate::plan::execution::BitArrayListFunctionId,
    args: &[CallArg],
    caller_frame: &mut Frame,
) -> ExecutionResult<BitArrayListValueId> {
    let frame = bind_arguments(
        plan,
        state,
        args,
        caller_frame,
        plan.bit_array_list_function(function).frame_layout(),
    )?;
    run_bit_array_list_loop(plan, state, function, frame)
}

pub(in crate::runtime) fn run_utf_codepoint_list_call(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
    function: crate::plan::execution::UtfCodepointListFunctionId,
    args: &[CallArg],
    caller_frame: &mut Frame,
) -> ExecutionResult<UtfCodepointListValueId> {
    let frame = bind_arguments(
        plan,
        state,
        args,
        caller_frame,
        plan.utf_codepoint_list_function(function).frame_layout(),
    )?;
    run_utf_codepoint_list_loop(plan, state, function, frame)
}

pub(in crate::runtime) fn run_custom_list_call(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
    function: crate::plan::execution::CustomListFunctionId,
    args: &[CallArg],
    caller_frame: &mut Frame,
) -> ExecutionResult<CustomListValueId> {
    let frame = bind_arguments(
        plan,
        state,
        args,
        caller_frame,
        plan.custom_list_function(function).frame_layout(),
    )?;
    run_custom_list_loop(plan, state, function, frame)
}

pub(in crate::runtime) fn run_float_list_call(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
    function: crate::plan::execution::FloatListFunctionId,
    args: &[CallArg],
    caller_frame: &mut Frame,
) -> ExecutionResult<FloatListValueId> {
    let frame = bind_arguments(
        plan,
        state,
        args,
        caller_frame,
        plan.float_list_function(function).frame_layout(),
    )?;
    run_float_list_loop(plan, state, function, frame)
}

pub(in crate::runtime) fn run_bool_list_call(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
    function: crate::plan::execution::BoolListFunctionId,
    args: &[CallArg],
    caller_frame: &mut Frame,
) -> ExecutionResult<BoolListValueId> {
    let frame = bind_arguments(
        plan,
        state,
        args,
        caller_frame,
        plan.bool_list_function(function).frame_layout(),
    )?;
    run_bool_list_loop(plan, state, function, frame)
}

pub(in crate::runtime) fn run_nil_list_call(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
    function: crate::plan::execution::NilListFunctionId,
    args: &[CallArg],
    caller_frame: &mut Frame,
) -> ExecutionResult<NilListValueId> {
    let frame = bind_arguments(
        plan,
        state,
        args,
        caller_frame,
        plan.nil_list_function(function).frame_layout(),
    )?;
    run_nil_list_loop(plan, state, function, frame)
}

pub(in crate::runtime) fn run_tuple_list_call(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
    function: crate::plan::execution::TupleListFunctionId,
    args: &[CallArg],
    caller_frame: &mut Frame,
) -> ExecutionResult<TupleListValueId> {
    let frame = bind_arguments(
        plan,
        state,
        args,
        caller_frame,
        plan.tuple_list_function(function).frame_layout(),
    )?;
    run_tuple_list_loop(plan, state, function, frame)
}

pub(in crate::runtime) fn run_list_list_call(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
    function: crate::plan::execution::ListListFunctionId,
    args: &[CallArg],
    caller_frame: &mut Frame,
) -> ExecutionResult<ListListValueId> {
    let frame = bind_arguments(
        plan,
        state,
        args,
        caller_frame,
        plan.list_list_function(function).frame_layout(),
    )?;
    run_list_list_loop(plan, state, function, frame)
}

pub(in crate::runtime) fn run_function_list_call(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
    function: crate::plan::execution::FunctionListFunctionId,
    args: &[CallArg],
    caller_frame: &mut Frame,
) -> ExecutionResult<FunctionListValueId> {
    let frame = bind_arguments(
        plan,
        state,
        args,
        caller_frame,
        plan.function_list_function(function).frame_layout(),
    )?;
    run_function_list_loop(plan, state, function, frame)
}

pub(in crate::runtime) fn run_int_function_call(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
    function: &crate::plan::execution::IntFunctionExpr,
    args: &[CallArg],
    caller_frame: &mut Frame,
) -> ExecutionResult<BigInt> {
    let function = eval_int_function_expr(plan, state, caller_frame, function)?;
    let runtime_function = plan.int_function(function.runtime_id());
    let frame_layout = runtime_function.frame_layout();
    let frame = bind_function_value_arguments(
        plan,
        state,
        args,
        caller_frame,
        frame_layout,
        function.captures(),
    )?;
    run_int_loop(plan, state, function.runtime_id(), frame)
}

pub(in crate::runtime) fn run_string_function_call(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
    function: &crate::plan::execution::StringFunctionExpr,
    args: &[CallArg],
    caller_frame: &mut Frame,
) -> ExecutionResult<EcoString> {
    let function = eval_string_function_expr(plan, state, caller_frame, function)?;
    let runtime_function = plan.string_function(function.runtime_id());
    let frame_layout = runtime_function.frame_layout();
    let frame = bind_function_value_arguments(
        plan,
        state,
        args,
        caller_frame,
        frame_layout,
        function.captures(),
    )?;
    run_string_loop(plan, state, function.runtime_id(), frame)
}

pub(in crate::runtime) fn run_bit_array_function_call(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
    function: &crate::plan::execution::BitArrayFunctionExpr,
    args: &[CallArg],
    caller_frame: &mut Frame,
) -> ExecutionResult<EvaluatedBitArray> {
    let function = eval_bit_array_function_expr(plan, state, caller_frame, function)?;
    let runtime_function = plan.bit_array_function(function.runtime_id());
    let frame = bind_function_value_arguments(
        plan,
        state,
        args,
        caller_frame,
        runtime_function.frame_layout(),
        function.captures(),
    )?;
    run_bit_array_loop(plan, state, function.runtime_id(), frame)
}

pub(in crate::runtime) fn run_utf_codepoint_function_call(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
    function: &crate::plan::execution::UtfCodepointFunctionExpr,
    args: &[CallArg],
    caller_frame: &mut Frame,
) -> ExecutionResult<char> {
    let function = eval_utf_codepoint_function_expr(plan, state, caller_frame, function)?;
    let runtime_function = plan.utf_codepoint_function(function.runtime_id());
    let frame = bind_function_value_arguments(
        plan,
        state,
        args,
        caller_frame,
        runtime_function.frame_layout(),
        function.captures(),
    )?;
    run_utf_codepoint_loop(plan, state, function.runtime_id(), frame)
}

pub(in crate::runtime) fn run_custom_function_call(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
    call: &crate::plan::execution::CustomFunctionCall,
    caller_frame: &mut Frame,
) -> ExecutionResult<EvaluatedCustomValue> {
    let function = eval_custom_function_expr(plan, state, caller_frame, call.function())?;
    match function {
        EvaluatedCustomFunction::Function(function) => {
            let runtime_id = function.runtime_id();
            let runtime_function = plan.custom_function(runtime_id);
            let frame = bind_function_value_arguments(
                plan,
                state,
                call.arguments(),
                caller_frame,
                runtime_function.frame_layout(),
                function.captures(),
            )?;
            run_custom_loop(plan, state, runtime_id, frame)
        }
        EvaluatedCustomFunction::Constructor(function) => {
            let fields = eval_call_argument_values(plan, state, call.arguments(), caller_frame)?;
            Ok(EvaluatedCustomValue::from_fields(
                function.runtime_id(),
                fields.into_boxed_slice(),
            ))
        }
    }
}

pub(in crate::runtime) fn run_float_function_call(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
    function: &crate::plan::execution::FloatFunctionExpr,
    args: &[CallArg],
    caller_frame: &mut Frame,
) -> ExecutionResult<f64> {
    let function = eval_float_function_expr(plan, state, caller_frame, function)?;
    let runtime_function = plan.float_function(function.runtime_id());
    let frame_layout = runtime_function.frame_layout();
    let frame = bind_function_value_arguments(
        plan,
        state,
        args,
        caller_frame,
        frame_layout,
        function.captures(),
    )?;
    run_float_loop(plan, state, function.runtime_id(), frame)
}

pub(in crate::runtime) fn run_bool_function_call(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
    function: &crate::plan::execution::BoolFunctionExpr,
    args: &[CallArg],
    caller_frame: &mut Frame,
) -> ExecutionResult<bool> {
    let function = eval_bool_function_expr(plan, state, caller_frame, function)?;
    let runtime_function = plan.bool_function(function.runtime_id());
    let frame_layout = runtime_function.frame_layout();
    let frame = bind_function_value_arguments(
        plan,
        state,
        args,
        caller_frame,
        frame_layout,
        function.captures(),
    )?;
    run_bool_loop(plan, state, function.runtime_id(), frame)
}

pub(in crate::runtime) fn run_nil_function_call(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
    function: &crate::plan::execution::NilFunctionExpr,
    args: &[CallArg],
    caller_frame: &mut Frame,
) -> ExecutionResult<()> {
    let function = eval_nil_function_expr(plan, state, caller_frame, function)?;
    let runtime_function = plan.nil_function(function.runtime_id());
    let frame_layout = runtime_function.frame_layout();
    let frame = bind_function_value_arguments(
        plan,
        state,
        args,
        caller_frame,
        frame_layout,
        function.captures(),
    )?;
    run_nil_loop(plan, state, function.runtime_id(), frame)
}

pub(in crate::runtime) fn run_tuple_function_call(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
    function: &crate::plan::execution::TupleFunctionExpr,
    args: &[CallArg],
    caller_frame: &mut Frame,
) -> ExecutionResult<Vec<EvaluatedValue>> {
    let function = eval_tuple_function_expr(plan, state, caller_frame, function)?;
    let runtime_function = plan.tuple_function(function.runtime_id());
    let frame_layout = runtime_function.frame_layout();
    let frame = bind_function_value_arguments(
        plan,
        state,
        args,
        caller_frame,
        frame_layout,
        function.captures(),
    )?;
    run_tuple_loop(plan, state, function.runtime_id(), frame)
}

pub(in crate::runtime) fn run_int_list_function_call(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
    function: &crate::plan::execution::ListFunctionExpr,
    args: &[CallArg],
    caller_frame: &mut Frame,
) -> ExecutionResult<IntListValueId> {
    let function = eval_list_function_expr(plan, state, caller_frame, function)?;
    let ListFunctionId::Int(runtime_id) = function.runtime_id() else {
        return Err(ExecutionError::FunctionReturnFamilyMismatch {
            expected: FunctionReturnFamily::List,
            actual: FunctionReturnFamily::List,
        });
    };
    let frame = bind_function_value_arguments(
        plan,
        state,
        args,
        caller_frame,
        plan.int_list_function(runtime_id).frame_layout(),
        function.captures(),
    )?;
    run_int_list_loop(plan, state, runtime_id, frame)
}

pub(in crate::runtime) fn run_string_list_function_call(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
    function: &crate::plan::execution::ListFunctionExpr,
    args: &[CallArg],
    caller_frame: &mut Frame,
) -> ExecutionResult<StringListValueId> {
    let function = eval_list_function_expr(plan, state, caller_frame, function)?;
    let ListFunctionId::String(runtime_id) = function.runtime_id() else {
        return Err(ExecutionError::FunctionReturnFamilyMismatch {
            expected: FunctionReturnFamily::List,
            actual: FunctionReturnFamily::List,
        });
    };
    let frame = bind_function_value_arguments(
        plan,
        state,
        args,
        caller_frame,
        plan.string_list_function(runtime_id).frame_layout(),
        function.captures(),
    )?;
    run_string_list_loop(plan, state, runtime_id, frame)
}

pub(in crate::runtime) fn run_bit_array_list_function_call(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
    function: &crate::plan::execution::ListFunctionExpr,
    args: &[CallArg],
    caller_frame: &mut Frame,
) -> ExecutionResult<BitArrayListValueId> {
    let function = eval_list_function_expr(plan, state, caller_frame, function)?;
    let ListFunctionId::BitArray(runtime_id) = function.runtime_id() else {
        return Err(ExecutionError::FunctionReturnFamilyMismatch {
            expected: FunctionReturnFamily::List,
            actual: FunctionReturnFamily::List,
        });
    };
    let frame = bind_function_value_arguments(
        plan,
        state,
        args,
        caller_frame,
        plan.bit_array_list_function(runtime_id).frame_layout(),
        function.captures(),
    )?;
    run_bit_array_list_loop(plan, state, runtime_id, frame)
}

pub(in crate::runtime) fn run_utf_codepoint_list_function_call(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
    function: &crate::plan::execution::ListFunctionExpr,
    args: &[CallArg],
    caller_frame: &mut Frame,
) -> ExecutionResult<UtfCodepointListValueId> {
    let function = eval_list_function_expr(plan, state, caller_frame, function)?;
    let ListFunctionId::UtfCodepoint(runtime_id) = function.runtime_id() else {
        return Err(ExecutionError::FunctionReturnFamilyMismatch {
            expected: FunctionReturnFamily::List,
            actual: FunctionReturnFamily::List,
        });
    };
    let frame = bind_function_value_arguments(
        plan,
        state,
        args,
        caller_frame,
        plan.utf_codepoint_list_function(runtime_id).frame_layout(),
        function.captures(),
    )?;
    run_utf_codepoint_list_loop(plan, state, runtime_id, frame)
}

pub(in crate::runtime) fn run_custom_list_function_call(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
    function: &crate::plan::execution::ListFunctionExpr,
    args: &[CallArg],
    caller_frame: &mut Frame,
) -> ExecutionResult<CustomListValueId> {
    let function = eval_list_function_expr(plan, state, caller_frame, function)?;
    let ListFunctionId::Custom(runtime_id) = function.runtime_id() else {
        return Err(ExecutionError::FunctionReturnFamilyMismatch {
            expected: FunctionReturnFamily::List,
            actual: FunctionReturnFamily::List,
        });
    };
    let frame = bind_function_value_arguments(
        plan,
        state,
        args,
        caller_frame,
        plan.custom_list_function(runtime_id).frame_layout(),
        function.captures(),
    )?;
    run_custom_list_loop(plan, state, runtime_id, frame)
}

pub(in crate::runtime) fn run_float_list_function_call(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
    function: &crate::plan::execution::ListFunctionExpr,
    args: &[CallArg],
    caller_frame: &mut Frame,
) -> ExecutionResult<FloatListValueId> {
    let function = eval_list_function_expr(plan, state, caller_frame, function)?;
    let ListFunctionId::Float(runtime_id) = function.runtime_id() else {
        return Err(ExecutionError::FunctionReturnFamilyMismatch {
            expected: FunctionReturnFamily::List,
            actual: FunctionReturnFamily::List,
        });
    };
    let frame = bind_function_value_arguments(
        plan,
        state,
        args,
        caller_frame,
        plan.float_list_function(runtime_id).frame_layout(),
        function.captures(),
    )?;
    run_float_list_loop(plan, state, runtime_id, frame)
}

pub(in crate::runtime) fn run_bool_list_function_call(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
    function: &crate::plan::execution::ListFunctionExpr,
    args: &[CallArg],
    caller_frame: &mut Frame,
) -> ExecutionResult<BoolListValueId> {
    let function = eval_list_function_expr(plan, state, caller_frame, function)?;
    let ListFunctionId::Bool(runtime_id) = function.runtime_id() else {
        return Err(ExecutionError::FunctionReturnFamilyMismatch {
            expected: FunctionReturnFamily::List,
            actual: FunctionReturnFamily::List,
        });
    };
    let frame = bind_function_value_arguments(
        plan,
        state,
        args,
        caller_frame,
        plan.bool_list_function(runtime_id).frame_layout(),
        function.captures(),
    )?;
    run_bool_list_loop(plan, state, runtime_id, frame)
}

pub(in crate::runtime) fn run_nil_list_function_call(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
    function: &crate::plan::execution::ListFunctionExpr,
    args: &[CallArg],
    caller_frame: &mut Frame,
) -> ExecutionResult<NilListValueId> {
    let function = eval_list_function_expr(plan, state, caller_frame, function)?;
    let ListFunctionId::Nil(runtime_id) = function.runtime_id() else {
        return Err(ExecutionError::FunctionReturnFamilyMismatch {
            expected: FunctionReturnFamily::List,
            actual: FunctionReturnFamily::List,
        });
    };
    let frame = bind_function_value_arguments(
        plan,
        state,
        args,
        caller_frame,
        plan.nil_list_function(runtime_id).frame_layout(),
        function.captures(),
    )?;
    run_nil_list_loop(plan, state, runtime_id, frame)
}

pub(in crate::runtime) fn run_tuple_list_function_call(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
    function: &crate::plan::execution::ListFunctionExpr,
    args: &[CallArg],
    caller_frame: &mut Frame,
) -> ExecutionResult<TupleListValueId> {
    let function = eval_list_function_expr(plan, state, caller_frame, function)?;
    let ListFunctionId::Tuple(runtime_id) = function.runtime_id() else {
        return Err(ExecutionError::FunctionReturnFamilyMismatch {
            expected: FunctionReturnFamily::List,
            actual: FunctionReturnFamily::List,
        });
    };
    let frame = bind_function_value_arguments(
        plan,
        state,
        args,
        caller_frame,
        plan.tuple_list_function(runtime_id).frame_layout(),
        function.captures(),
    )?;
    run_tuple_list_loop(plan, state, runtime_id, frame)
}

pub(in crate::runtime) fn run_list_list_function_call(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
    function: &crate::plan::execution::ListFunctionExpr,
    args: &[CallArg],
    caller_frame: &mut Frame,
) -> ExecutionResult<ListListValueId> {
    let function = eval_list_function_expr(plan, state, caller_frame, function)?;
    let ListFunctionId::List(runtime_id) = function.runtime_id() else {
        return Err(ExecutionError::FunctionReturnFamilyMismatch {
            expected: FunctionReturnFamily::List,
            actual: FunctionReturnFamily::List,
        });
    };
    let frame = bind_function_value_arguments(
        plan,
        state,
        args,
        caller_frame,
        plan.list_list_function(runtime_id).frame_layout(),
        function.captures(),
    )?;
    run_list_list_loop(plan, state, runtime_id, frame)
}

pub(in crate::runtime) fn run_function_list_function_call(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
    function: &crate::plan::execution::ListFunctionExpr,
    args: &[CallArg],
    caller_frame: &mut Frame,
) -> ExecutionResult<FunctionListValueId> {
    let function = eval_list_function_expr(plan, state, caller_frame, function)?;
    let ListFunctionId::Function(runtime_id) = function.runtime_id() else {
        return Err(ExecutionError::FunctionReturnFamilyMismatch {
            expected: FunctionReturnFamily::List,
            actual: FunctionReturnFamily::List,
        });
    };
    let frame = bind_function_value_arguments(
        plan,
        state,
        args,
        caller_frame,
        plan.function_list_function(runtime_id).frame_layout(),
        function.captures(),
    )?;
    run_function_list_loop(plan, state, runtime_id, frame)
}

fn list_function_frame_layout<'a>(
    plan: &'a ExecutionPlan,
    function: &ListFunctionId,
) -> &'a crate::plan::execution::FrameLayout {
    match function {
        ListFunctionId::Int(id) => plan.int_list_function(*id).frame_layout(),
        ListFunctionId::String(id) => plan.string_list_function(*id).frame_layout(),
        ListFunctionId::BitArray(id) => plan.bit_array_list_function(*id).frame_layout(),
        ListFunctionId::UtfCodepoint(id) => plan.utf_codepoint_list_function(*id).frame_layout(),
        ListFunctionId::Custom(id) => plan.custom_list_function(*id).frame_layout(),
        ListFunctionId::Float(id) => plan.float_list_function(*id).frame_layout(),
        ListFunctionId::Bool(id) => plan.bool_list_function(*id).frame_layout(),
        ListFunctionId::Nil(id) => plan.nil_list_function(*id).frame_layout(),
        ListFunctionId::Tuple(id) => plan.tuple_list_function(*id).frame_layout(),
        ListFunctionId::List(id) => plan.list_list_function(*id).frame_layout(),
        ListFunctionId::Function(id) => plan.function_list_function(*id).frame_layout(),
    }
}

pub(in crate::runtime) fn run_int_function_returning_function_call(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
    function: IntFunctionFunctionId,
    args: &[CallArg],
    caller_frame: &mut Frame,
) -> ExecutionResult<EvaluatedIntFunction> {
    let frame = bind_arguments(
        plan,
        state,
        args,
        caller_frame,
        plan.int_function_function(function).frame_layout(),
    )?;
    run_int_function_loop(plan, state, function, frame)
}

pub(in crate::runtime) fn run_float_function_returning_function_call(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
    function: FloatFunctionFunctionId,
    args: &[CallArg],
    caller_frame: &mut Frame,
) -> ExecutionResult<EvaluatedFloatFunction> {
    let frame = bind_arguments(
        plan,
        state,
        args,
        caller_frame,
        plan.float_function_function(function).frame_layout(),
    )?;
    run_float_function_loop(plan, state, function, frame)
}

pub(in crate::runtime) fn run_string_function_returning_function_call(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
    function: StringFunctionFunctionId,
    args: &[CallArg],
    caller_frame: &mut Frame,
) -> ExecutionResult<EvaluatedStringFunction> {
    let frame = bind_arguments(
        plan,
        state,
        args,
        caller_frame,
        plan.string_function_function(function).frame_layout(),
    )?;
    run_string_function_loop(plan, state, function, frame)
}

pub(in crate::runtime) fn run_bit_array_function_returning_function_call(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
    function: BitArrayFunctionFunctionId,
    args: &[CallArg],
    caller_frame: &mut Frame,
) -> ExecutionResult<EvaluatedBitArrayFunction> {
    let frame = bind_arguments(
        plan,
        state,
        args,
        caller_frame,
        plan.bit_array_function_function(function).frame_layout(),
    )?;
    run_bit_array_function_loop(plan, state, function, frame)
}

pub(in crate::runtime) fn run_utf_codepoint_function_returning_function_call(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
    function: UtfCodepointFunctionFunctionId,
    args: &[CallArg],
    caller_frame: &mut Frame,
) -> ExecutionResult<EvaluatedUtfCodepointFunction> {
    let frame = bind_arguments(
        plan,
        state,
        args,
        caller_frame,
        plan.utf_codepoint_function_function(function)
            .frame_layout(),
    )?;
    run_utf_codepoint_function_loop(plan, state, function, frame)
}

pub(in crate::runtime) fn run_custom_function_returning_function_call(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
    function: CustomFunctionFunctionId,
    args: &[CallArg],
    caller_frame: &mut Frame,
) -> ExecutionResult<EvaluatedCustomFunction> {
    let frame = bind_arguments(
        plan,
        state,
        args,
        caller_frame,
        plan.custom_function_function(&function).frame_layout(),
    )?;
    run_custom_function_loop(plan, state, function, frame)
}

pub(in crate::runtime) fn run_bool_function_returning_function_call(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
    function: BoolFunctionFunctionId,
    args: &[CallArg],
    caller_frame: &mut Frame,
) -> ExecutionResult<EvaluatedBoolFunction> {
    let frame = bind_arguments(
        plan,
        state,
        args,
        caller_frame,
        plan.bool_function_function(function).frame_layout(),
    )?;
    run_bool_function_loop(plan, state, function, frame)
}

pub(in crate::runtime) fn run_nil_function_returning_function_call(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
    function: NilFunctionFunctionId,
    args: &[CallArg],
    caller_frame: &mut Frame,
) -> ExecutionResult<EvaluatedNilFunction> {
    let frame = bind_arguments(
        plan,
        state,
        args,
        caller_frame,
        plan.nil_function_function(function).frame_layout(),
    )?;
    run_nil_function_loop(plan, state, function, frame)
}

pub(in crate::runtime) fn run_tuple_function_returning_function_call(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
    function: TupleFunctionFunctionId,
    args: &[CallArg],
    caller_frame: &mut Frame,
) -> ExecutionResult<EvaluatedTupleFunction> {
    let frame = bind_arguments(
        plan,
        state,
        args,
        caller_frame,
        plan.tuple_function_function(function).frame_layout(),
    )?;
    run_tuple_function_loop(plan, state, function, frame)
}

pub(in crate::runtime) fn run_list_function_returning_function_call(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
    function: ListFunctionFunctionId,
    args: &[CallArg],
    caller_frame: &mut Frame,
) -> ExecutionResult<EvaluatedListFunction> {
    let frame = bind_arguments(
        plan,
        state,
        args,
        caller_frame,
        plan.list_function_function(&function).frame_layout(),
    )?;
    run_list_function_loop(plan, state, function, frame)
}

pub(in crate::runtime) fn run_function_function_returning_function_call(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
    function: FunctionFunctionFunctionId,
    args: &[CallArg],
    caller_frame: &mut Frame,
) -> ExecutionResult<EvaluatedFunctionFunction> {
    let frame = bind_arguments(
        plan,
        state,
        args,
        caller_frame,
        plan.function_function_function(&function).frame_layout(),
    )?;
    run_function_function_loop(plan, state, function, frame)
}

pub(in crate::runtime) fn run_int_function_function_call(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
    function: &crate::plan::execution::FunctionFunctionExpr,
    args: &[CallArg],
    caller_frame: &mut Frame,
) -> ExecutionResult<EvaluatedIntFunction> {
    let function = eval_function_function_expr(plan, state, caller_frame, function)?;
    let runtime_id = function.runtime_id();
    let function_id = runtime_id
        .int()
        .ok_or(ExecutionError::FunctionReturnFamilyMismatch {
            expected: FunctionReturnFamily::Int,
            actual: runtime_id.family(),
        })?;
    let runtime_function = plan.int_function_function(function_id);
    let frame_layout = runtime_function.frame_layout();
    let frame = bind_function_value_arguments(
        plan,
        state,
        args,
        caller_frame,
        frame_layout,
        function.captures(),
    )?;
    run_int_function_loop(plan, state, function_id, frame)
}

pub(in crate::runtime) fn run_float_function_function_call(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
    function: &crate::plan::execution::FunctionFunctionExpr,
    args: &[CallArg],
    caller_frame: &mut Frame,
) -> ExecutionResult<EvaluatedFloatFunction> {
    let function = eval_function_function_expr(plan, state, caller_frame, function)?;
    let runtime_id = function.runtime_id();
    let function_id = runtime_id
        .float()
        .ok_or(ExecutionError::FunctionReturnFamilyMismatch {
            expected: FunctionReturnFamily::Float,
            actual: runtime_id.family(),
        })?;
    let runtime_function = plan.float_function_function(function_id);
    let frame_layout = runtime_function.frame_layout();
    let frame = bind_function_value_arguments(
        plan,
        state,
        args,
        caller_frame,
        frame_layout,
        function.captures(),
    )?;
    run_float_function_loop(plan, state, function_id, frame)
}

pub(in crate::runtime) fn run_string_function_function_call(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
    function: &crate::plan::execution::FunctionFunctionExpr,
    args: &[CallArg],
    caller_frame: &mut Frame,
) -> ExecutionResult<EvaluatedStringFunction> {
    let function = eval_function_function_expr(plan, state, caller_frame, function)?;
    let runtime_id = function.runtime_id();
    let function_id = runtime_id
        .string()
        .ok_or(ExecutionError::FunctionReturnFamilyMismatch {
            expected: FunctionReturnFamily::String,
            actual: runtime_id.family(),
        })?;
    let runtime_function = plan.string_function_function(function_id);
    let frame_layout = runtime_function.frame_layout();
    let frame = bind_function_value_arguments(
        plan,
        state,
        args,
        caller_frame,
        frame_layout,
        function.captures(),
    )?;
    run_string_function_loop(plan, state, function_id, frame)
}

pub(in crate::runtime) fn run_bit_array_function_function_call(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
    function: &crate::plan::execution::FunctionFunctionExpr,
    args: &[CallArg],
    caller_frame: &mut Frame,
) -> ExecutionResult<EvaluatedBitArrayFunction> {
    let function = eval_function_function_expr(plan, state, caller_frame, function)?;
    let runtime_id = function.runtime_id();
    let function_id =
        runtime_id
            .bit_array()
            .ok_or(ExecutionError::FunctionReturnFamilyMismatch {
                expected: FunctionReturnFamily::BitArray,
                actual: runtime_id.family(),
            })?;
    let runtime_function = plan.bit_array_function_function(function_id);
    let frame = bind_function_value_arguments(
        plan,
        state,
        args,
        caller_frame,
        runtime_function.frame_layout(),
        function.captures(),
    )?;
    run_bit_array_function_loop(plan, state, function_id, frame)
}

pub(in crate::runtime) fn run_utf_codepoint_function_function_call(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
    function: &crate::plan::execution::FunctionFunctionExpr,
    args: &[CallArg],
    caller_frame: &mut Frame,
) -> ExecutionResult<EvaluatedUtfCodepointFunction> {
    let function = eval_function_function_expr(plan, state, caller_frame, function)?;
    let runtime_id = function.runtime_id();
    let function_id =
        runtime_id
            .utf_codepoint()
            .ok_or(ExecutionError::FunctionReturnFamilyMismatch {
                expected: FunctionReturnFamily::UtfCodepoint,
                actual: runtime_id.family(),
            })?;
    let runtime_function = plan.utf_codepoint_function_function(function_id);
    let frame = bind_function_value_arguments(
        plan,
        state,
        args,
        caller_frame,
        runtime_function.frame_layout(),
        function.captures(),
    )?;
    run_utf_codepoint_function_loop(plan, state, function_id, frame)
}

pub(in crate::runtime) fn run_custom_function_function_call(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
    function: &crate::plan::execution::FunctionFunctionExpr,
    args: &[CallArg],
    caller_frame: &mut Frame,
) -> ExecutionResult<EvaluatedCustomFunction> {
    let function = eval_function_function_expr(plan, state, caller_frame, function)?;
    let runtime_id = function.runtime_id();
    let function_id = runtime_id
        .custom()
        .ok_or(ExecutionError::FunctionReturnFamilyMismatch {
            expected: FunctionReturnFamily::Custom,
            actual: runtime_id.family(),
        })?;
    let runtime_function = plan.custom_function_function(&function_id);
    let frame = bind_function_value_arguments(
        plan,
        state,
        args,
        caller_frame,
        runtime_function.frame_layout(),
        function.captures(),
    )?;
    run_custom_function_loop(plan, state, function_id, frame)
}

pub(in crate::runtime) fn run_bool_function_function_call(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
    function: &crate::plan::execution::FunctionFunctionExpr,
    args: &[CallArg],
    caller_frame: &mut Frame,
) -> ExecutionResult<EvaluatedBoolFunction> {
    let function = eval_function_function_expr(plan, state, caller_frame, function)?;
    let runtime_id = function.runtime_id();
    let function_id = runtime_id
        .bool()
        .ok_or(ExecutionError::FunctionReturnFamilyMismatch {
            expected: FunctionReturnFamily::Bool,
            actual: runtime_id.family(),
        })?;
    let runtime_function = plan.bool_function_function(function_id);
    let frame_layout = runtime_function.frame_layout();
    let frame = bind_function_value_arguments(
        plan,
        state,
        args,
        caller_frame,
        frame_layout,
        function.captures(),
    )?;
    run_bool_function_loop(plan, state, function_id, frame)
}

pub(in crate::runtime) fn run_nil_function_function_call(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
    function: &crate::plan::execution::FunctionFunctionExpr,
    args: &[CallArg],
    caller_frame: &mut Frame,
) -> ExecutionResult<EvaluatedNilFunction> {
    let function = eval_function_function_expr(plan, state, caller_frame, function)?;
    let runtime_id = function.runtime_id();
    let function_id = runtime_id
        .nil()
        .ok_or(ExecutionError::FunctionReturnFamilyMismatch {
            expected: FunctionReturnFamily::Nil,
            actual: runtime_id.family(),
        })?;
    let runtime_function = plan.nil_function_function(function_id);
    let frame_layout = runtime_function.frame_layout();
    let frame = bind_function_value_arguments(
        plan,
        state,
        args,
        caller_frame,
        frame_layout,
        function.captures(),
    )?;
    run_nil_function_loop(plan, state, function_id, frame)
}

pub(in crate::runtime) fn run_tuple_function_function_call(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
    function: &crate::plan::execution::FunctionFunctionExpr,
    args: &[CallArg],
    caller_frame: &mut Frame,
) -> ExecutionResult<EvaluatedTupleFunction> {
    let function = eval_function_function_expr(plan, state, caller_frame, function)?;
    let runtime_id = function.runtime_id();
    let function_id = runtime_id
        .tuple()
        .ok_or(ExecutionError::FunctionReturnFamilyMismatch {
            expected: FunctionReturnFamily::Tuple,
            actual: runtime_id.family(),
        })?;
    let runtime_function = plan.tuple_function_function(function_id);
    let frame_layout = runtime_function.frame_layout();
    let frame = bind_function_value_arguments(
        plan,
        state,
        args,
        caller_frame,
        frame_layout,
        function.captures(),
    )?;
    run_tuple_function_loop(plan, state, function_id, frame)
}

pub(in crate::runtime) fn run_list_function_function_call(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
    function: &crate::plan::execution::FunctionFunctionExpr,
    args: &[CallArg],
    caller_frame: &mut Frame,
) -> ExecutionResult<EvaluatedListFunction> {
    let function = eval_function_function_expr(plan, state, caller_frame, function)?;
    let runtime_id = function.runtime_id();
    let function_id = runtime_id
        .list()
        .ok_or(ExecutionError::FunctionReturnFamilyMismatch {
            expected: FunctionReturnFamily::List,
            actual: runtime_id.family(),
        })?;
    let runtime_function = plan.list_function_function(&function_id);
    let frame_layout = runtime_function.frame_layout();
    let frame = bind_function_value_arguments(
        plan,
        state,
        args,
        caller_frame,
        frame_layout,
        function.captures(),
    )?;
    run_list_function_loop(plan, state, function_id, frame)
}

pub(in crate::runtime) fn run_function_function_function_call(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
    function: &crate::plan::execution::FunctionFunctionExpr,
    args: &[CallArg],
    caller_frame: &mut Frame,
) -> ExecutionResult<EvaluatedFunctionFunction> {
    let function = eval_function_function_expr(plan, state, caller_frame, function)?;
    let runtime_id = function.runtime_id();
    let function_id =
        runtime_id
            .function()
            .ok_or(ExecutionError::FunctionReturnFamilyMismatch {
                expected: FunctionReturnFamily::Function,
                actual: runtime_id.family(),
            })?;
    let runtime_function = plan.function_function_function(&function_id);
    let frame_layout = runtime_function.frame_layout();
    let frame = bind_function_value_arguments(
        plan,
        state,
        args,
        caller_frame,
        frame_layout,
        function.captures(),
    )?;
    run_function_function_loop(plan, state, function_id, frame)
}
fn run_function_returning_function_call(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
    function: crate::plan::execution::FunctionFunctionId,
    args: &[CallArg],
    caller_frame: &mut Frame,
) -> ExecutionResult<EvaluatedFunctionValue> {
    match function {
        crate::plan::execution::FunctionFunctionId::Int(function) => {
            run_int_function_returning_function_call(plan, state, function, args, caller_frame)
                .map(Into::into)
        }
        crate::plan::execution::FunctionFunctionId::Float(function) => {
            run_float_function_returning_function_call(plan, state, function, args, caller_frame)
                .map(Into::into)
        }
        crate::plan::execution::FunctionFunctionId::String(function) => {
            run_string_function_returning_function_call(plan, state, function, args, caller_frame)
                .map(Into::into)
        }
        crate::plan::execution::FunctionFunctionId::BitArray(function) => {
            run_bit_array_function_returning_function_call(
                plan,
                state,
                function,
                args,
                caller_frame,
            )
            .map(Into::into)
        }
        crate::plan::execution::FunctionFunctionId::UtfCodepoint(function) => {
            run_utf_codepoint_function_returning_function_call(
                plan,
                state,
                function,
                args,
                caller_frame,
            )
            .map(Into::into)
        }
        crate::plan::execution::FunctionFunctionId::Custom(function) => {
            run_custom_function_returning_function_call(plan, state, function, args, caller_frame)
                .map(Into::into)
        }
        crate::plan::execution::FunctionFunctionId::Bool(function) => {
            run_bool_function_returning_function_call(plan, state, function, args, caller_frame)
                .map(Into::into)
        }
        crate::plan::execution::FunctionFunctionId::Nil(function) => {
            run_nil_function_returning_function_call(plan, state, function, args, caller_frame)
                .map(Into::into)
        }
        crate::plan::execution::FunctionFunctionId::Tuple(function) => {
            run_tuple_function_returning_function_call(plan, state, function, args, caller_frame)
                .map(Into::into)
        }
        crate::plan::execution::FunctionFunctionId::List(function) => {
            run_list_function_returning_function_call(plan, state, function, args, caller_frame)
                .map(Into::into)
        }
        crate::plan::execution::FunctionFunctionId::Function(function) => {
            run_function_function_returning_function_call(plan, state, function, args, caller_frame)
                .map(Into::into)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        run_bit_array_function_loop, run_bit_array_list_loop, run_bool_function_loop,
        run_bool_list_loop, run_custom_function_loop, run_custom_list_loop,
        run_float_function_loop, run_float_list_loop, run_function_function_loop,
        run_function_list_loop, run_int_function_loop, run_int_list_loop, run_list_function_loop,
        run_list_list_loop, run_nil_function_loop, run_nil_list_loop, run_string_function_loop,
        run_string_list_loop, run_tuple_function_loop, run_tuple_list_loop,
        run_utf_codepoint_function_loop, run_utf_codepoint_list_loop,
    };
    use crate::plan::execution::{
        BoolFunctionFunctionId, CallArg, FloatFunctionFunctionId, FunctionFunctionId,
        FunctionListLocalId, FunctionReturnFamily, IntFunctionFunctionId, IntFunctionId,
        ListFunctionId, NilFunctionFunctionId, ReturnBody, ReturnBodyKind,
        StringFunctionFunctionId, StringFunctionId, TupleFunctionFunctionId,
        UtfCodepointFunctionFunctionId,
    };
    use crate::runtime::FunctionValueKind;
    use crate::runtime::frame::Frame;
    use crate::runtime::{ExecutionError, Value, run_main};

    #[test]
    fn source_utf_codepoint_list_function_calls_return_values_and_argument_errors() {
        assert_eq!(
            crate::runtime::run_src(include_str!(
                "../../tests/fixtures/execution/values/utf_codepoint_list_function_paths.gleam"
            )),
            Value::Tuple(vec![Value::UtfCodepoint('A'); 6]),
        );
        assert_eq!(
            crate::runtime::run_src_error(include_str!(
                "../../tests/fixtures/execution_errors/functions/utf_codepoint_list_call_argument.gleam"
            ))
            .to_string(),
            "panic: argument",
        );
    }

    #[test]
    fn primitive_function_value_calls_propagate_callee_and_argument_panics() {
        let callee_sources = [
            "pub fn main() -> Int { case True { True -> panic as \"callee\" False -> fn() { 0 } }() }",
            "pub fn main() -> String { case True { True -> panic as \"callee\" False -> fn() { \"\" } }() }",
            "pub fn main() -> BitArray { case True { True -> panic as \"callee\" False -> fn() { <<>> } }() }",
            "pub fn main() -> UtfCodepoint { case True { True -> panic as \"callee\" False -> fn() { panic } }() }",
            "pub fn main() -> Float { case True { True -> panic as \"callee\" False -> fn() { 0.0 } }() }",
            "pub fn main() -> Bool { case True { True -> panic as \"callee\" False -> fn() { False } }() }",
            "pub fn main() -> Nil { case True { True -> panic as \"callee\" False -> fn() { Nil } }() }",
            "pub fn main() -> #(Int) { case True { True -> panic as \"callee\" False -> fn() { #(0) } }() }",
        ];
        let argument_sources = [
            "fn callee(value: Int) -> Int { value } pub fn main() { let function = callee function(panic as \"argument\") }",
            "fn callee(value: Int) -> String { \"value\" } pub fn main() { let function = callee function(panic as \"argument\") }",
            "fn callee(value: Int) -> BitArray { <<value>> } pub fn main() { let function = callee function(panic as \"argument\") }",
            "fn callee(value: Int) -> UtfCodepoint { panic } pub fn main() { let function = callee function(panic as \"argument\") }",
            "fn callee(value: Int) -> Float { 1.0 } pub fn main() { let function = callee function(panic as \"argument\") }",
            "fn callee(value: Int) -> Bool { True } pub fn main() { let function = callee function(panic as \"argument\") }",
            "fn callee(value: Int) -> Nil { Nil } pub fn main() { let function = callee function(panic as \"argument\") }",
            "fn callee(value: Int) -> #(Int) { #(value) } pub fn main() { let function = callee function(panic as \"argument\") }",
        ];

        for source in callee_sources {
            assert_eq!(
                crate::runtime::run_src_error(source).to_string(),
                "panic: callee",
            );
        }
        for source in argument_sources {
            assert_eq!(
                crate::runtime::run_src_error(source).to_string(),
                "panic: argument",
            );
        }
        assert_eq!(
            crate::runtime::run_src_error(
                "pub type Boxed { Boxed(Int) } pub fn main() -> Boxed { case True { True -> panic as \"callee\" False -> fn() { Boxed(0) } }() }",
            )
            .to_string(),
            "panic: callee",
        );
        assert_eq!(
            crate::runtime::run_src_error(
                "pub type Boxed { Boxed(Int) } fn callee(value: Int) -> Boxed { Boxed(value) } pub fn main() { let function = callee function(panic as \"argument\") }",
            )
            .to_string(),
            "panic: argument",
        );
    }

    #[test]
    fn list_function_value_calls_propagate_callee_panics() {
        let sources = [
            "pub fn main() -> List(Int) { case True { True -> panic as \"callee\" False -> fn() { [] } }() }",
            "pub fn main() -> List(String) { case True { True -> panic as \"callee\" False -> fn() { [] } }() }",
            "pub fn main() -> List(BitArray) { case True { True -> panic as \"callee\" False -> fn() { [] } }() }",
            "pub fn main() -> List(UtfCodepoint) { case True { True -> panic as \"callee\" False -> fn() { [] } }() }",
            "pub fn main() -> List(Float) { case True { True -> panic as \"callee\" False -> fn() { [] } }() }",
            "pub fn main() -> List(Bool) { case True { True -> panic as \"callee\" False -> fn() { [] } }() }",
            "pub fn main() -> List(Nil) { case True { True -> panic as \"callee\" False -> fn() { [] } }() }",
            "pub fn main() -> List(#(Int)) { case True { True -> panic as \"callee\" False -> fn() { [] } }() }",
            "pub fn main() -> List(List(Int)) { case True { True -> panic as \"callee\" False -> fn() { [] } }() }",
            "pub fn main() -> List(fn() -> Int) { case True { True -> panic as \"callee\" False -> fn() { [] } }() }",
        ];

        for source in sources {
            assert_eq!(
                crate::runtime::run_src_error(source).to_string(),
                "panic: callee",
            );
        }
        assert_eq!(
            crate::runtime::run_src_error(
                "pub type Boxed { Boxed(Int) } pub fn main() -> List(Boxed) { case True { True -> panic as \"callee\" False -> fn() { [] } }() }",
            )
            .to_string(),
            "panic: callee",
        );
    }

    #[test]
    fn function_returning_function_value_calls_propagate_callee_and_argument_panics() {
        let callee_sources = [
            "pub fn main() -> fn() -> Int { case True { True -> panic as \"callee\" False -> fn() { fn() { 0 } } }() }",
            "pub fn main() -> fn() -> String { case True { True -> panic as \"callee\" False -> fn() { fn() { \"\" } } }() }",
            "pub fn main() -> fn() -> BitArray { case True { True -> panic as \"callee\" False -> fn() { fn() { <<>> } } }() }",
            "pub fn main() -> fn() -> UtfCodepoint { case True { True -> panic as \"callee\" False -> fn() { fn() { panic } } }() }",
            "pub fn main() -> fn() -> Float { case True { True -> panic as \"callee\" False -> fn() { fn() { 0.0 } } }() }",
            "pub fn main() -> fn() -> Bool { case True { True -> panic as \"callee\" False -> fn() { fn() { False } } }() }",
            "pub fn main() -> fn() -> Nil { case True { True -> panic as \"callee\" False -> fn() { fn() { Nil } } }() }",
            "pub fn main() -> fn() -> #(Int) { case True { True -> panic as \"callee\" False -> fn() { fn() { #(0) } } }() }",
            "pub fn main() -> fn() -> List(Int) { case True { True -> panic as \"callee\" False -> fn() { fn() { [] } } }() }",
            "pub fn main() -> fn() -> fn() -> Int { case True { True -> panic as \"callee\" False -> fn() { fn() { fn() { 0 } } } }() }",
        ];
        let argument_sources = [
            "fn callee(value: Int) -> fn() -> Int { fn() { value } } pub fn main() { let function = callee function(panic as \"argument\") }",
            "fn callee(value: Int) -> fn() -> String { fn() { \"value\" } } pub fn main() { let function = callee function(panic as \"argument\") }",
            "fn callee(value: Int) -> fn() -> BitArray { fn() { <<value>> } } pub fn main() { let function = callee function(panic as \"argument\") }",
            "fn callee(value: Int) -> fn() -> UtfCodepoint { fn() { panic } } pub fn main() { let function = callee function(panic as \"argument\") }",
            "fn callee(value: Int) -> fn() -> Float { fn() { 1.0 } } pub fn main() { let function = callee function(panic as \"argument\") }",
            "fn callee(value: Int) -> fn() -> Bool { fn() { True } } pub fn main() { let function = callee function(panic as \"argument\") }",
            "fn callee(value: Int) -> fn() -> Nil { fn() { Nil } } pub fn main() { let function = callee function(panic as \"argument\") }",
            "fn callee(value: Int) -> fn() -> #(Int) { fn() { #(value) } } pub fn main() { let function = callee function(panic as \"argument\") }",
            "fn callee(value: Int) -> fn() -> List(Int) { fn() { [value] } } pub fn main() { let function = callee function(panic as \"argument\") }",
            "fn callee(value: Int) -> fn() -> fn() -> Int { fn() { fn() { value } } } pub fn main() { let function = callee function(panic as \"argument\") }",
        ];

        for source in callee_sources {
            assert_eq!(
                crate::runtime::run_src_error(source).to_string(),
                "panic: callee",
            );
        }
        for source in argument_sources {
            assert_eq!(
                crate::runtime::run_src_error(source).to_string(),
                "panic: argument",
            );
        }
        assert_eq!(
            crate::runtime::run_src_error(
                "pub type Boxed { Boxed(Int) } pub fn main() -> fn() -> Boxed { case True { True -> panic as \"callee\" False -> fn() { fn() { Boxed(0) } } }() }",
            )
            .to_string(),
            "panic: callee",
        );
        assert_eq!(
            crate::runtime::run_src_error(
                "pub type Boxed { Boxed(Int) } fn callee(value: Int) -> fn() -> Boxed { fn() { Boxed(value) } } pub fn main() { let function = callee function(panic as \"argument\") }",
            )
            .to_string(),
            "panic: argument",
        );
    }

    #[test]
    fn direct_calls_propagate_argument_panics_for_every_return_family() {
        let sources = [
            "fn callee(value: Int) -> Int { value } pub fn main() { let _ = callee(panic) 0 }",
            "fn callee(value: Int) -> String { \"value\" } pub fn main() { let _ = callee(panic) 0 }",
            "fn callee(value: Int) -> BitArray { <<value>> } pub fn main() { let _ = callee(panic) 0 }",
            "fn callee(value: Int) -> UtfCodepoint { panic } pub fn main() { let _ = callee(panic) 0 }",
            "fn callee(value: Int) -> Float { 1.0 } pub fn main() { let _ = callee(panic) 0 }",
            "fn callee(value: Int) -> Bool { True } pub fn main() { let _ = callee(panic) 0 }",
            "fn callee(value: Int) -> Nil { Nil } pub fn main() { let _ = callee(panic) 0 }",
            "fn callee(value: Int) -> #(Int) { #(value) } pub fn main() { let _ = callee(panic) 0 }",
            "fn callee(value: Int) -> List(Int) { [] } pub fn main() { let _ = callee(panic) 0 }",
            "fn callee(value: Int) -> List(String) { [] } pub fn main() { let _ = callee(panic) 0 }",
            "fn callee(value: Int) -> List(BitArray) { [] } pub fn main() { let _ = callee(panic) 0 }",
            "fn callee(value: Int) -> List(UtfCodepoint) { [] } pub fn main() { let _ = callee(panic) 0 }",
            "fn callee(value: Int) -> List(Float) { [] } pub fn main() { let _ = callee(panic) 0 }",
            "fn callee(value: Int) -> List(Bool) { [] } pub fn main() { let _ = callee(panic) 0 }",
            "fn callee(value: Int) -> List(Nil) { [] } pub fn main() { let _ = callee(panic) 0 }",
            "fn callee(value: Int) -> List(#(Int)) { [] } pub fn main() { let _ = callee(panic) 0 }",
            "fn callee(value: Int) -> List(List(Int)) { [] } pub fn main() { let _ = callee(panic) 0 }",
            "fn callee(value: Int) -> List(fn() -> Int) { [] } pub fn main() { let _ = callee(panic) 0 }",
        ];

        for source in sources {
            let plan = crate::runtime::plan_src(source);
            let error = run_main(&plan).expect_err("panic argument should fail execution");

            assert_eq!(error.to_string(), "panic: `panic` expression evaluated.");
        }
        for source in [
            "pub type Boxed { Boxed(Int) } fn callee(value: Int) -> Boxed { Boxed(value) } pub fn main() { let _ = callee(panic) 0 }",
            "pub type Boxed { Boxed(Int) } fn callee(value: Int) -> List(Boxed) { [] } pub fn main() { let _ = callee(panic) 0 }",
        ] {
            assert_eq!(
                crate::runtime::run_src_error(source).to_string(),
                "panic: `panic` expression evaluated.",
            );
        }
    }

    #[test]
    fn generic_list_call_propagates_tail_call_argument_panics() {
        let plan = crate::runtime::plan_src(
            "fn callee(value: Int) -> List(Int) { [] } pub fn main() { callee(panic) }",
        );
        let main = plan.int_list_function(plan.int_list_function_id(0));
        let args = expect_tail_call_args(main.return_());
        let mut state = crate::runtime::RuntimeState::new();
        let layout = crate::plan::execution::FrameLayout::default();
        let mut caller_frame = Frame::new(&layout, &mut state);

        let error = super::run_list_call(
            &plan,
            &mut state,
            ListFunctionId::Int(plan.int_list_function_id(1)),
            args,
            &mut caller_frame,
        )
        .expect_err("panic argument should fail execution");

        assert_eq!(error.to_string(), "panic: `panic` expression evaluated.");
    }

    #[test]
    #[should_panic(expected = "expected a tail-call return body")]
    fn tail_call_shape_guard_rejects_expression_returns() {
        let plan = crate::runtime::plan_src("pub fn main() -> List(Int) { [] }");
        let main = plan.int_list_function(plan.int_list_function_id(0));

        let _ = expect_tail_call_args(main.return_());
    }

    #[test]
    fn list_function_value_calls_propagate_argument_panics() {
        let sources = [
            "fn callee(value: Int) -> List(Int) { [] } pub fn main() { let function = callee function(panic) }",
            "fn callee(value: Int) -> List(String) { [] } pub fn main() { let function = callee function(panic) }",
            "fn callee(value: Int) -> List(BitArray) { [] } pub fn main() { let function = callee function(panic) }",
            "fn callee(value: Int) -> List(UtfCodepoint) { [] } pub fn main() { let function = callee function(panic) }",
            "fn callee(value: Int) -> List(Float) { [] } pub fn main() { let function = callee function(panic) }",
            "fn callee(value: Int) -> List(Bool) { [] } pub fn main() { let function = callee function(panic) }",
            "fn callee(value: Int) -> List(Nil) { [] } pub fn main() { let function = callee function(panic) }",
            "fn callee(value: Int) -> List(#(Int)) { [] } pub fn main() { let function = callee function(panic) }",
            "fn callee(value: Int) -> List(List(Int)) { [] } pub fn main() { let function = callee function(panic) }",
            "fn callee(value: Int) -> List(fn() -> Int) { [] } pub fn main() { let function = callee function(panic) }",
        ];

        for source in sources {
            let plan = crate::runtime::plan_src(source);
            let error = run_main(&plan).expect_err("panic argument should fail execution");

            assert_eq!(error.to_string(), "panic: `panic` expression evaluated.");
        }
        assert_eq!(
            crate::runtime::run_src_error(
                "pub type Boxed { Boxed(Int) } fn callee(value: Int) -> List(Boxed) { [] } pub fn main() { let function = callee function(panic) }",
            )
            .to_string(),
            "panic: `panic` expression evaluated.",
        );
    }

    #[test]
    fn list_function_calls_report_direct_mutated_item_return_families() {
        let plan = crate::runtime::plan_src(
            r#"
fn ints(function: fn() -> List(Int)) { function() }
fn strings(function: fn() -> List(String)) { function() }
fn bit_arrays(function: fn() -> List(BitArray)) { function() }
fn utf_codepoints(function: fn() -> List(UtfCodepoint)) { function() }
pub type Boxed { Boxed(Int) }
fn customs(function: fn() -> List(Boxed)) { function() }
fn floats(function: fn() -> List(Float)) { function() }
fn bools(function: fn() -> List(Bool)) { function() }
fn nils(function: fn() -> List(Nil)) { function() }
fn tuples(function: fn() -> List(#(Int))) { function() }
fn lists(function: fn() -> List(List(Int))) { function() }
fn functions(function: fn() -> List(fn() -> Int)) { function() }
pub fn main() { Nil }
"#,
        );
        let wrong_int = crate::runtime::EvaluatedListFunction::new(
            ListFunctionId::Int(plan.int_list_function_id(0)),
            Vec::new(),
            Vec::new(),
            crate::plan::execution::FunctionType::new(
                Vec::new(),
                crate::plan::execution::ValueType::List(
                    plan.int_list_function_id(0).type_id().list_type(),
                ),
            ),
        );
        let wrong_string = crate::runtime::EvaluatedListFunction::new(
            ListFunctionId::String(plan.string_list_function_id(0)),
            Vec::new(),
            Vec::new(),
            crate::plan::execution::FunctionType::new(
                Vec::new(),
                crate::plan::execution::ValueType::List(
                    plan.string_list_function_id(0).type_id().list_type(),
                ),
            ),
        );
        let expected = ExecutionError::FunctionReturnFamilyMismatch {
            expected: FunctionReturnFamily::List,
            actual: FunctionReturnFamily::List,
        };

        let function = plan.int_list_function(plan.int_list_function_id(0));
        let mut state = crate::runtime::RuntimeState::new();
        let mut frame = Frame::new(function.frame_layout(), &mut state);
        frame.set_list_function(
            function.frame_layout().list_functions()[0].clone(),
            wrong_string,
        );
        assert_eq!(
            run_int_list_loop(&plan, &mut state, plan.int_list_function_id(0), frame)
                .expect_err("direct-mutated list function family must fail"),
            expected,
        );

        let function = plan.string_list_function(plan.string_list_function_id(0));
        let mut state = crate::runtime::RuntimeState::new();
        let mut frame = Frame::new(function.frame_layout(), &mut state);
        frame.set_list_function(
            function.frame_layout().list_functions()[0].clone(),
            wrong_int.clone(),
        );
        assert_eq!(
            run_string_list_loop(&plan, &mut state, plan.string_list_function_id(0), frame,)
                .expect_err("direct-mutated list function family must fail"),
            expected,
        );

        let function = plan.bit_array_list_function(plan.bit_array_list_function_id(0));
        let mut state = crate::runtime::RuntimeState::new();
        let mut frame = Frame::new(function.frame_layout(), &mut state);
        frame.set_list_function(
            function.frame_layout().list_functions()[0].clone(),
            wrong_int.clone(),
        );
        assert_eq!(
            run_bit_array_list_loop(&plan, &mut state, plan.bit_array_list_function_id(0), frame,)
                .expect_err("direct-mutated list function family must fail"),
            expected,
        );

        let function = plan.utf_codepoint_list_function(plan.utf_codepoint_list_function_id(0));
        let mut state = crate::runtime::RuntimeState::new();
        let mut frame = Frame::new(function.frame_layout(), &mut state);
        frame.set_list_function(
            function.frame_layout().list_functions()[0].clone(),
            wrong_int.clone(),
        );
        assert_eq!(
            run_utf_codepoint_list_loop(
                &plan,
                &mut state,
                plan.utf_codepoint_list_function_id(0),
                frame,
            )
            .expect_err("direct-mutated list function family must fail"),
            expected,
        );

        let function = plan.custom_list_function(plan.custom_list_function_id(0));
        let mut state = crate::runtime::RuntimeState::new();
        let mut frame = Frame::new(function.frame_layout(), &mut state);
        frame.set_list_function(
            function.frame_layout().list_functions()[0].clone(),
            wrong_int.clone(),
        );
        assert_eq!(
            run_custom_list_loop(&plan, &mut state, plan.custom_list_function_id(0), frame,)
                .expect_err("direct-mutated list function family must fail"),
            expected,
        );

        let function = plan.float_list_function(plan.float_list_function_id(0));
        let mut state = crate::runtime::RuntimeState::new();
        let mut frame = Frame::new(function.frame_layout(), &mut state);
        frame.set_list_function(
            function.frame_layout().list_functions()[0].clone(),
            wrong_int.clone(),
        );
        assert_eq!(
            run_float_list_loop(&plan, &mut state, plan.float_list_function_id(0), frame,)
                .expect_err("direct-mutated list function family must fail"),
            expected,
        );

        let function = plan.bool_list_function(plan.bool_list_function_id(0));
        let mut state = crate::runtime::RuntimeState::new();
        let mut frame = Frame::new(function.frame_layout(), &mut state);
        frame.set_list_function(
            function.frame_layout().list_functions()[0].clone(),
            wrong_int.clone(),
        );
        assert_eq!(
            run_bool_list_loop(&plan, &mut state, plan.bool_list_function_id(0), frame,)
                .expect_err("direct-mutated list function family must fail"),
            expected,
        );

        let function = plan.nil_list_function(plan.nil_list_function_id(0));
        let mut state = crate::runtime::RuntimeState::new();
        let mut frame = Frame::new(function.frame_layout(), &mut state);
        frame.set_list_function(
            function.frame_layout().list_functions()[0].clone(),
            wrong_int.clone(),
        );
        assert_eq!(
            run_nil_list_loop(&plan, &mut state, plan.nil_list_function_id(0), frame)
                .expect_err("direct-mutated list function family must fail"),
            expected,
        );

        let function = plan.tuple_list_function(plan.tuple_list_function_id(0));
        let mut state = crate::runtime::RuntimeState::new();
        let mut frame = Frame::new(function.frame_layout(), &mut state);
        frame.set_list_function(
            function.frame_layout().list_functions()[0].clone(),
            wrong_int.clone(),
        );
        assert_eq!(
            run_tuple_list_loop(&plan, &mut state, plan.tuple_list_function_id(0), frame,)
                .expect_err("direct-mutated list function family must fail"),
            expected,
        );

        let function = plan.list_list_function(plan.list_list_function_id(0));
        let mut state = crate::runtime::RuntimeState::new();
        let mut frame = Frame::new(function.frame_layout(), &mut state);
        frame.set_list_function(
            function.frame_layout().list_functions()[0].clone(),
            wrong_int.clone(),
        );
        assert_eq!(
            run_list_list_loop(&plan, &mut state, plan.list_list_function_id(0), frame,)
                .expect_err("direct-mutated list function family must fail"),
            expected,
        );

        let function = plan.function_list_function(plan.function_list_function_id(0));
        let mut state = crate::runtime::RuntimeState::new();
        let mut frame = Frame::new(function.frame_layout(), &mut state);
        frame.set_list_function(
            function.frame_layout().list_functions()[0].clone(),
            wrong_int,
        );
        assert_eq!(
            run_function_list_loop(&plan, &mut state, plan.function_list_function_id(0), frame,)
                .expect_err("direct-mutated list function family must fail"),
            expected,
        );
    }

    #[test]
    fn non_tail_function_returning_calls_propagate_argument_panics() {
        let sources = [
            "fn callee(value: Int) -> fn() -> Int { panic } pub fn main() { let _ = callee(panic) 0 }",
            "fn callee(value: Int) -> fn() -> String { panic } pub fn main() { let _ = callee(panic) 0 }",
            "fn callee(value: Int) -> fn() -> BitArray { panic } pub fn main() { let _ = callee(panic) 0 }",
            "fn callee(value: Int) -> fn() -> UtfCodepoint { panic } pub fn main() { let _ = callee(panic) 0 }",
            "fn callee(value: Int) -> fn() -> Float { panic } pub fn main() { let _ = callee(panic) 0 }",
            "fn callee(value: Int) -> fn() -> Bool { panic } pub fn main() { let _ = callee(panic) 0 }",
            "fn callee(value: Int) -> fn() -> Nil { panic } pub fn main() { let _ = callee(panic) 0 }",
            "fn callee(value: Int) -> fn() -> #(Int) { panic } pub fn main() { let _ = callee(panic) 0 }",
            "fn callee(value: Int) -> fn() -> List(Int) { panic } pub fn main() { let _ = callee(panic) 0 }",
            "fn callee(value: Int) -> fn() -> fn() -> Int { panic } pub fn main() { let _ = callee(panic) 0 }",
            "pub type Boxed { Boxed(Int) } fn callee(value: Int) -> fn() -> Boxed { fn() { Boxed(value) } } pub fn main() { let _ = callee(panic) 0 }",
        ];

        for source in sources {
            let plan = crate::runtime::plan_src(source);
            let error = run_main(&plan).expect_err("panic argument should fail execution");

            assert_eq!(error.to_string(), "panic: `panic` expression evaluated.");
        }
    }

    #[test]
    fn function_function_calls_reject_wrong_return_families() {
        let plan = crate::runtime::plan_src(
            r#"
fn int_function(provider: fn() -> fn() -> Int) { provider() }
fn string_function(provider: fn() -> fn() -> String) { provider() }
fn bit_array_function(provider: fn() -> fn() -> BitArray) { provider() }
fn utf_codepoint_function(provider: fn() -> fn() -> UtfCodepoint) { provider() }
pub type Boxed { Boxed(Int) }
fn custom_function(provider: fn() -> fn() -> Boxed) { provider() }
fn float_function(provider: fn() -> fn() -> Float) { provider() }
fn bool_function(provider: fn() -> fn() -> Bool) { provider() }
fn nil_function(provider: fn() -> fn() -> Nil) { provider() }
fn tuple_function(provider: fn() -> fn() -> #(Int)) { provider() }
fn list_function(provider: fn() -> fn() -> List(Int)) { provider() }
fn function_function(provider: fn() -> fn() -> fn() -> Int) { provider() }

pub fn main() { #(list_function, custom_function, function_function) }
"#,
        );
        let mut functions = expect_tuple_values(
            run_main(&plan).expect("main should return the selected functions"),
        );
        let function_function_id = expect_function_function_id(
            functions
                .pop()
                .expect("function tuple should contain function_function"),
        )
        .function()
        .expect("function_function should return a function function");
        let custom_function_id = expect_function_function_id(
            functions
                .pop()
                .expect("function tuple should contain custom_function"),
        )
        .custom()
        .expect("custom_function should return a custom function");
        let list_function_id = expect_function_function_id(
            functions
                .pop()
                .expect("function tuple should contain list_function"),
        )
        .list()
        .expect("list_function should return a list function");
        assert_eq!(functions, Vec::new());
        let wrong_string = crate::runtime::EvaluatedFunctionFunction::new(
            FunctionFunctionId::String(StringFunctionFunctionId(0)),
            Vec::new(),
            Vec::new(),
            crate::plan::execution::FunctionType::new(
                Vec::new(),
                crate::plan::execution::ValueType::String,
            ),
        );
        let wrong_int = crate::runtime::EvaluatedFunctionFunction::new(
            FunctionFunctionId::Int(IntFunctionFunctionId(0)),
            Vec::new(),
            Vec::new(),
            crate::plan::execution::FunctionType::new(
                Vec::new(),
                crate::plan::execution::ValueType::Int,
            ),
        );

        let function = plan.int_function_function(IntFunctionFunctionId(0));
        let mut state = crate::runtime::RuntimeState::new();
        let mut frame = Frame::new(function.frame_layout(), &mut state);
        let local = function.frame_layout().function_functions()[0].clone();
        frame.set_function_function(&local, wrong_string);
        assert_eq!(
            run_int_function_loop(&plan, &mut state, IntFunctionFunctionId(0), frame),
            Err(ExecutionError::FunctionReturnFamilyMismatch {
                expected: FunctionReturnFamily::Int,
                actual: FunctionReturnFamily::String,
            }),
        );

        let function = plan.string_function_function(StringFunctionFunctionId(0));
        let mut state = crate::runtime::RuntimeState::new();
        let mut frame = Frame::new(function.frame_layout(), &mut state);
        let local = function.frame_layout().function_functions()[0].clone();
        frame.set_function_function(&local, wrong_int.clone());
        assert_eq!(
            run_string_function_loop(&plan, &mut state, StringFunctionFunctionId(0), frame),
            Err(ExecutionError::FunctionReturnFamilyMismatch {
                expected: FunctionReturnFamily::String,
                actual: FunctionReturnFamily::Int,
            }),
        );

        let function =
            plan.bit_array_function_function(crate::plan::execution::BitArrayFunctionFunctionId(0));
        let mut state = crate::runtime::RuntimeState::new();
        let mut frame = Frame::new(function.frame_layout(), &mut state);
        let local = function.frame_layout().function_functions()[0].clone();
        frame.set_function_function(&local, wrong_int.clone());
        assert_eq!(
            run_bit_array_function_loop(
                &plan,
                &mut state,
                crate::plan::execution::BitArrayFunctionFunctionId(0),
                frame,
            ),
            Err(ExecutionError::FunctionReturnFamilyMismatch {
                expected: FunctionReturnFamily::BitArray,
                actual: FunctionReturnFamily::Int,
            }),
        );

        let function = plan.utf_codepoint_function_function(UtfCodepointFunctionFunctionId(0));
        let mut state = crate::runtime::RuntimeState::new();
        let mut frame = Frame::new(function.frame_layout(), &mut state);
        let local = function.frame_layout().function_functions()[0].clone();
        frame.set_function_function(&local, wrong_int.clone());
        assert_eq!(
            run_utf_codepoint_function_loop(
                &plan,
                &mut state,
                UtfCodepointFunctionFunctionId(0),
                frame,
            ),
            Err(ExecutionError::FunctionReturnFamilyMismatch {
                expected: FunctionReturnFamily::UtfCodepoint,
                actual: FunctionReturnFamily::Int,
            }),
        );

        let function = plan.custom_function_function(&custom_function_id);
        let mut state = crate::runtime::RuntimeState::new();
        let mut frame = Frame::new(function.frame_layout(), &mut state);
        let local = function.frame_layout().function_functions()[0].clone();
        frame.set_function_function(&local, wrong_int.clone());
        assert_eq!(
            run_custom_function_loop(&plan, &mut state, custom_function_id.clone(), frame),
            Err(ExecutionError::FunctionReturnFamilyMismatch {
                expected: FunctionReturnFamily::Custom,
                actual: FunctionReturnFamily::Int,
            }),
        );

        let function = plan.float_function_function(FloatFunctionFunctionId(0));
        let mut state = crate::runtime::RuntimeState::new();
        let mut frame = Frame::new(function.frame_layout(), &mut state);
        let local = function.frame_layout().function_functions()[0].clone();
        frame.set_function_function(&local, wrong_int.clone());
        assert_eq!(
            run_float_function_loop(&plan, &mut state, FloatFunctionFunctionId(0), frame),
            Err(ExecutionError::FunctionReturnFamilyMismatch {
                expected: FunctionReturnFamily::Float,
                actual: FunctionReturnFamily::Int,
            }),
        );

        let function = plan.bool_function_function(BoolFunctionFunctionId(0));
        let mut state = crate::runtime::RuntimeState::new();
        let mut frame = Frame::new(function.frame_layout(), &mut state);
        let local = function.frame_layout().function_functions()[0].clone();
        frame.set_function_function(&local, wrong_int.clone());
        assert_eq!(
            run_bool_function_loop(&plan, &mut state, BoolFunctionFunctionId(0), frame),
            Err(ExecutionError::FunctionReturnFamilyMismatch {
                expected: FunctionReturnFamily::Bool,
                actual: FunctionReturnFamily::Int,
            }),
        );

        let function = plan.nil_function_function(NilFunctionFunctionId(0));
        let mut state = crate::runtime::RuntimeState::new();
        let mut frame = Frame::new(function.frame_layout(), &mut state);
        let local = function.frame_layout().function_functions()[0].clone();
        frame.set_function_function(&local, wrong_int.clone());
        assert_eq!(
            run_nil_function_loop(&plan, &mut state, NilFunctionFunctionId(0), frame),
            Err(ExecutionError::FunctionReturnFamilyMismatch {
                expected: FunctionReturnFamily::Nil,
                actual: FunctionReturnFamily::Int,
            }),
        );

        let function = plan.tuple_function_function(TupleFunctionFunctionId(0));
        let mut state = crate::runtime::RuntimeState::new();
        let mut frame = Frame::new(function.frame_layout(), &mut state);
        let local = function.frame_layout().function_functions()[0].clone();
        frame.set_function_function(&local, wrong_int.clone());
        assert_eq!(
            run_tuple_function_loop(&plan, &mut state, TupleFunctionFunctionId(0), frame),
            Err(ExecutionError::FunctionReturnFamilyMismatch {
                expected: FunctionReturnFamily::Tuple,
                actual: FunctionReturnFamily::Int,
            }),
        );

        let function = plan.list_function_function(&list_function_id);
        let mut state = crate::runtime::RuntimeState::new();
        let mut frame = Frame::new(function.frame_layout(), &mut state);
        let local = function.frame_layout().function_functions()[0].clone();
        frame.set_function_function(&local, wrong_int.clone());
        assert_eq!(
            run_list_function_loop(&plan, &mut state, list_function_id, frame),
            Err(ExecutionError::FunctionReturnFamilyMismatch {
                expected: FunctionReturnFamily::List,
                actual: FunctionReturnFamily::Int,
            }),
        );

        let function = plan.function_function_function(&function_function_id);
        let mut state = crate::runtime::RuntimeState::new();
        let mut frame = Frame::new(function.frame_layout(), &mut state);
        let local = function.frame_layout().function_functions()[0].clone();
        frame.set_function_function(&local, wrong_int);
        assert_eq!(
            run_function_function_loop(&plan, &mut state, function_function_id.clone(), frame),
            Err(ExecutionError::FunctionReturnFamilyMismatch {
                expected: FunctionReturnFamily::Function,
                actual: FunctionReturnFamily::Int,
            }),
        );
    }

    #[test]
    fn function_list_projections_reject_wrong_return_families() {
        let plan = crate::runtime::plan_src(
            r#"
fn int_function(values: List(fn() -> Int)) {
  case values { [value, ..] -> value _ -> panic }
}
fn string_function(values: List(fn() -> String)) {
  case values { [value, ..] -> value _ -> panic }
}
fn bit_array_function(values: List(fn() -> BitArray)) {
  case values { [value, ..] -> value _ -> panic }
}
fn utf_codepoint_function(values: List(fn() -> UtfCodepoint)) {
  case values { [value, ..] -> value _ -> panic }
}
pub type Boxed { Boxed(Int) }
fn custom_function(values: List(fn() -> Boxed)) {
  case values { [value, ..] -> value _ -> panic }
}
fn float_function(values: List(fn() -> Float)) {
  case values { [value, ..] -> value _ -> panic }
}
fn bool_function(values: List(fn() -> Bool)) {
  case values { [value, ..] -> value _ -> panic }
}
fn nil_function(values: List(fn() -> Nil)) {
  case values { [value, ..] -> value _ -> panic }
}
fn tuple_function(values: List(fn() -> #(Int))) {
  case values { [value, ..] -> value _ -> panic }
}
fn list_function(values: List(fn() -> List(Int))) {
  case values { [value, ..] -> value _ -> panic }
}
fn function_function(values: List(fn() -> fn() -> Int)) {
  case values { [value, ..] -> value _ -> panic }
}

pub fn main() { #(list_function, custom_function, function_function) }
"#,
        );
        let mut functions = expect_tuple_values(
            run_main(&plan).expect("main should return the selected functions"),
        );
        let function_function_id = expect_function_function_id(
            functions
                .pop()
                .expect("function tuple should contain function_function"),
        )
        .function()
        .expect("function_function should return a function function");
        let custom_function_id = expect_function_function_id(
            functions
                .pop()
                .expect("function tuple should contain custom_function"),
        )
        .custom()
        .expect("custom_function should return a custom function");
        let list_function_id = expect_function_function_id(
            functions
                .pop()
                .expect("function tuple should contain list_function"),
        )
        .list()
        .expect("list_function should return a list function");
        assert_eq!(functions, Vec::new());
        let wrong_string = crate::runtime::EvaluatedStringFunction::new(
            StringFunctionId(0),
            Vec::new(),
            Vec::new(),
            crate::plan::execution::FunctionType::new(
                Vec::new(),
                crate::plan::execution::ValueType::String,
            ),
        );
        let wrong_int = crate::runtime::EvaluatedIntFunction::new(
            IntFunctionId(0),
            Vec::new(),
            Vec::new(),
            crate::plan::execution::FunctionType::new(
                Vec::new(),
                crate::plan::execution::ValueType::Int,
            ),
        );

        let function = plan.int_function_function(IntFunctionFunctionId(0));
        let mut state = crate::runtime::RuntimeState::new();
        let mut frame = Frame::new(function.frame_layout(), &mut state);
        let value = state.function(
            function.frame_layout().function_lists()[0],
            vec![wrong_string.into()],
        );
        frame.set_function_list(FunctionListLocalId(0), value);
        assert_eq!(
            run_int_function_loop(&plan, &mut state, IntFunctionFunctionId(0), frame),
            Err(ExecutionError::FunctionReturnFamilyMismatch {
                expected: FunctionReturnFamily::Int,
                actual: FunctionReturnFamily::String,
            }),
        );

        let function = plan.string_function_function(StringFunctionFunctionId(0));
        let mut state = crate::runtime::RuntimeState::new();
        let mut frame = Frame::new(function.frame_layout(), &mut state);
        let value = state.function(
            function.frame_layout().function_lists()[0],
            vec![wrong_int.clone().into()],
        );
        frame.set_function_list(FunctionListLocalId(0), value);
        assert_eq!(
            run_string_function_loop(&plan, &mut state, StringFunctionFunctionId(0), frame),
            Err(ExecutionError::FunctionReturnFamilyMismatch {
                expected: FunctionReturnFamily::String,
                actual: FunctionReturnFamily::Int,
            }),
        );

        let function =
            plan.bit_array_function_function(crate::plan::execution::BitArrayFunctionFunctionId(0));
        let mut state = crate::runtime::RuntimeState::new();
        let mut frame = Frame::new(function.frame_layout(), &mut state);
        let value = state.function(
            function.frame_layout().function_lists()[0],
            vec![wrong_int.clone().into()],
        );
        frame.set_function_list(FunctionListLocalId(0), value);
        assert_eq!(
            run_bit_array_function_loop(
                &plan,
                &mut state,
                crate::plan::execution::BitArrayFunctionFunctionId(0),
                frame,
            ),
            Err(ExecutionError::FunctionReturnFamilyMismatch {
                expected: FunctionReturnFamily::BitArray,
                actual: FunctionReturnFamily::Int,
            }),
        );

        let function = plan.utf_codepoint_function_function(UtfCodepointFunctionFunctionId(0));
        let mut state = crate::runtime::RuntimeState::new();
        let mut frame = Frame::new(function.frame_layout(), &mut state);
        let value = state.function(
            function.frame_layout().function_lists()[0],
            vec![wrong_int.clone().into()],
        );
        frame.set_function_list(FunctionListLocalId(0), value);
        assert_eq!(
            run_utf_codepoint_function_loop(
                &plan,
                &mut state,
                UtfCodepointFunctionFunctionId(0),
                frame,
            ),
            Err(ExecutionError::FunctionReturnFamilyMismatch {
                expected: FunctionReturnFamily::UtfCodepoint,
                actual: FunctionReturnFamily::Int,
            }),
        );

        let function = plan.custom_function_function(&custom_function_id);
        let mut state = crate::runtime::RuntimeState::new();
        let mut frame = Frame::new(function.frame_layout(), &mut state);
        let value = state.function(
            function.frame_layout().function_lists()[0],
            vec![wrong_int.clone().into()],
        );
        frame.set_function_list(FunctionListLocalId(0), value);
        assert_eq!(
            run_custom_function_loop(&plan, &mut state, custom_function_id.clone(), frame),
            Err(ExecutionError::FunctionReturnFamilyMismatch {
                expected: FunctionReturnFamily::Custom,
                actual: FunctionReturnFamily::Int,
            }),
        );

        let function = plan.float_function_function(FloatFunctionFunctionId(0));
        let mut state = crate::runtime::RuntimeState::new();
        let mut frame = Frame::new(function.frame_layout(), &mut state);
        let value = state.function(
            function.frame_layout().function_lists()[0],
            vec![wrong_int.clone().into()],
        );
        frame.set_function_list(FunctionListLocalId(0), value);
        assert_eq!(
            run_float_function_loop(&plan, &mut state, FloatFunctionFunctionId(0), frame),
            Err(ExecutionError::FunctionReturnFamilyMismatch {
                expected: FunctionReturnFamily::Float,
                actual: FunctionReturnFamily::Int,
            }),
        );

        let function = plan.bool_function_function(BoolFunctionFunctionId(0));
        let mut state = crate::runtime::RuntimeState::new();
        let mut frame = Frame::new(function.frame_layout(), &mut state);
        let value = state.function(
            function.frame_layout().function_lists()[0],
            vec![wrong_int.clone().into()],
        );
        frame.set_function_list(FunctionListLocalId(0), value);
        assert_eq!(
            run_bool_function_loop(&plan, &mut state, BoolFunctionFunctionId(0), frame),
            Err(ExecutionError::FunctionReturnFamilyMismatch {
                expected: FunctionReturnFamily::Bool,
                actual: FunctionReturnFamily::Int,
            }),
        );

        let function = plan.nil_function_function(NilFunctionFunctionId(0));
        let mut state = crate::runtime::RuntimeState::new();
        let mut frame = Frame::new(function.frame_layout(), &mut state);
        let value = state.function(
            function.frame_layout().function_lists()[0],
            vec![wrong_int.clone().into()],
        );
        frame.set_function_list(FunctionListLocalId(0), value);
        assert_eq!(
            run_nil_function_loop(&plan, &mut state, NilFunctionFunctionId(0), frame),
            Err(ExecutionError::FunctionReturnFamilyMismatch {
                expected: FunctionReturnFamily::Nil,
                actual: FunctionReturnFamily::Int,
            }),
        );

        let function = plan.tuple_function_function(TupleFunctionFunctionId(0));
        let mut state = crate::runtime::RuntimeState::new();
        let mut frame = Frame::new(function.frame_layout(), &mut state);
        let value = state.function(
            function.frame_layout().function_lists()[0],
            vec![wrong_int.clone().into()],
        );
        frame.set_function_list(FunctionListLocalId(0), value);
        assert_eq!(
            run_tuple_function_loop(&plan, &mut state, TupleFunctionFunctionId(0), frame),
            Err(ExecutionError::FunctionReturnFamilyMismatch {
                expected: FunctionReturnFamily::Tuple,
                actual: FunctionReturnFamily::Int,
            }),
        );

        let function = plan.list_function_function(&list_function_id);
        let mut state = crate::runtime::RuntimeState::new();
        let mut frame = Frame::new(function.frame_layout(), &mut state);
        let value = state.function(
            function.frame_layout().function_lists()[0],
            vec![wrong_int.clone().into()],
        );
        frame.set_function_list(FunctionListLocalId(0), value);
        assert_eq!(
            run_list_function_loop(&plan, &mut state, list_function_id, frame),
            Err(ExecutionError::FunctionReturnFamilyMismatch {
                expected: FunctionReturnFamily::List,
                actual: FunctionReturnFamily::Int,
            }),
        );

        let function = plan.function_function_function(&function_function_id);
        let mut state = crate::runtime::RuntimeState::new();
        let mut frame = Frame::new(function.frame_layout(), &mut state);
        let value = state.function(
            function.frame_layout().function_lists()[0],
            vec![wrong_int.into()],
        );
        frame.set_function_list(FunctionListLocalId(0), value);
        assert_eq!(
            run_function_function_loop(&plan, &mut state, function_function_id.clone(), frame),
            Err(ExecutionError::FunctionReturnFamilyMismatch {
                expected: FunctionReturnFamily::Function,
                actual: FunctionReturnFamily::Int,
            }),
        );
    }

    #[test]
    #[should_panic(expected = "expected a function returning a function")]
    fn list_function_function_fixture_guard_rejects_non_function_value() {
        let _ = expect_function_function_id(Value::Int(0.into()));
    }

    #[test]
    #[should_panic(expected = "expected a function returning a function")]
    fn list_function_function_fixture_guard_rejects_primitive_function() {
        let value = crate::runtime::run_src("pub fn main() { fn() { 1 } }");
        let _ = expect_function_function_id(value);
    }

    #[test]
    #[should_panic(expected = "expected a function returning a list function")]
    fn list_function_function_fixture_guard_rejects_int_function_function() {
        let value = crate::runtime::run_src("pub fn main() { fn() { fn() { 1 } } }");
        let _ = expect_function_function_id(value)
            .list()
            .expect("expected a function returning a list function");
    }

    #[test]
    #[should_panic(expected = "expected a tuple of functions")]
    fn function_tuple_fixture_guard_rejects_non_tuple_value() {
        let _ = expect_tuple_values(Value::Int(0.into()));
    }

    fn expect_tail_call_args<Expression, Function>(
        body: &ReturnBody<Expression, Function>,
    ) -> &[CallArg] {
        match body.kind() {
            ReturnBodyKind::TailCall { args, .. } => args,
            _ => panic!("expected a tail-call return body"),
        }
    }

    fn expect_function_function_id(value: Value) -> FunctionFunctionId {
        match value {
            Value::Function(function) => match function.kind() {
                FunctionValueKind::Function(function) => function.runtime_id().clone(),
                _ => panic!("expected a function returning a function"),
            },
            _ => panic!("expected a function returning a function"),
        }
    }

    fn expect_tuple_values(value: Value) -> Vec<Value> {
        match value {
            Value::Tuple(values) => values,
            _ => panic!("expected a tuple of functions"),
        }
    }
}
