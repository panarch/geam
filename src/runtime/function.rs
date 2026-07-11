mod bind;
pub(in crate::runtime) mod return_body;
mod steps;

pub(in crate::runtime) use bind::eval_capture_args;
pub(in crate::runtime) use steps::execute_steps;

use crate::plan::execution::ExecutionPlan;
use crate::plan::execution::{
    BoolFunctionFunctionId, BoolFunctionId, CallArg, FloatFunctionFunctionId, FloatFunctionId,
    FunctionFunctionFunctionId, FunctionReturnFamily, IntFunctionFunctionId, IntFunctionId,
    ListFunctionFunctionId, ListFunctionId, NilFunctionFunctionId, NilFunctionId,
    RuntimeFunctionId, StringFunctionFunctionId, StringFunctionId, TupleFunctionFunctionId,
    TupleFunctionId,
};
use crate::runtime::error::ExecutionResult;
use crate::runtime::expression::{
    eval_bool_function_expr, eval_float_function_expr, eval_function_function_expr,
    eval_int_function_expr, eval_list_function_expr, eval_nil_function_expr,
    eval_string_function_expr, eval_tuple_function_expr,
};
use crate::runtime::frame::Frame;
use crate::runtime::{ExecutionError, FunctionFunctionValue, FunctionValue, Value};
use bind::{bind_arguments, bind_function_value_arguments};
use ecow::EcoString;
use num_bigint::BigInt;
use return_body::{
    run_bool_function_loop, run_bool_list_loop, run_bool_loop, run_float_function_loop,
    run_float_list_loop, run_float_loop, run_function_function_loop, run_function_list_loop,
    run_int_function_loop, run_int_list_loop, run_int_loop, run_list_function_loop,
    run_list_list_loop, run_list_loop, run_nil_function_loop, run_nil_list_loop, run_nil_loop,
    run_string_function_loop, run_string_list_loop, run_string_loop, run_tuple_function_loop,
    run_tuple_list_loop, run_tuple_loop,
};

pub(super) fn run_main(plan: &ExecutionPlan) -> ExecutionResult<Value> {
    let mut caller_frame = Frame::default();
    match plan.main_runtime() {
        RuntimeFunctionId::Int(function) => {
            run_int_call(plan, function, &[], &mut caller_frame).map(Value::Int)
        }
        RuntimeFunctionId::Float(function) => {
            run_float_call(plan, function, &[], &mut caller_frame).map(Value::Float)
        }
        RuntimeFunctionId::String(function) => {
            run_string_call(plan, function, &[], &mut caller_frame).map(Value::String)
        }
        RuntimeFunctionId::Bool(function) => {
            run_bool_call(plan, function, &[], &mut caller_frame).map(Value::Bool)
        }
        RuntimeFunctionId::Nil(function) => {
            run_nil_call(plan, function, &[], &mut caller_frame).map(|_| Value::Nil)
        }
        RuntimeFunctionId::Tuple { id, .. } => {
            run_tuple_call(plan, id, &[], &mut caller_frame).map(Value::Tuple)
        }
        RuntimeFunctionId::List(id) => {
            run_list_call(plan, id, &[], &mut caller_frame).map(Value::List)
        }
        RuntimeFunctionId::Function { id, .. } => {
            run_function_returning_function_call(plan, id, &[], &mut caller_frame)
                .map(Value::Function)
        }
    }
}

pub(super) fn run_int_call(
    plan: &ExecutionPlan,
    function: IntFunctionId,
    args: &[CallArg],
    caller_frame: &mut Frame,
) -> ExecutionResult<BigInt> {
    let frame = bind_arguments(
        plan,
        args,
        caller_frame,
        plan.int_function(function).frame_layout(),
    )?;
    run_int_loop(plan, function, frame)
}

pub(super) fn run_float_call(
    plan: &ExecutionPlan,
    function: FloatFunctionId,
    args: &[CallArg],
    caller_frame: &mut Frame,
) -> ExecutionResult<f64> {
    let frame = bind_arguments(
        plan,
        args,
        caller_frame,
        plan.float_function(function).frame_layout(),
    )?;
    run_float_loop(plan, function, frame)
}

pub(super) fn run_string_call(
    plan: &ExecutionPlan,
    function: StringFunctionId,
    args: &[CallArg],
    caller_frame: &mut Frame,
) -> ExecutionResult<EcoString> {
    let frame = bind_arguments(
        plan,
        args,
        caller_frame,
        plan.string_function(function).frame_layout(),
    )?;
    run_string_loop(plan, function, frame)
}

pub(super) fn run_bool_call(
    plan: &ExecutionPlan,
    function: BoolFunctionId,
    args: &[CallArg],
    caller_frame: &mut Frame,
) -> ExecutionResult<bool> {
    let frame = bind_arguments(
        plan,
        args,
        caller_frame,
        plan.bool_function(function).frame_layout(),
    )?;
    run_bool_loop(plan, function, frame)
}

pub(super) fn run_nil_call(
    plan: &ExecutionPlan,
    function: NilFunctionId,
    args: &[CallArg],
    caller_frame: &mut Frame,
) -> ExecutionResult<()> {
    let frame = bind_arguments(
        plan,
        args,
        caller_frame,
        plan.nil_function(function).frame_layout(),
    )?;
    run_nil_loop(plan, function, frame)
}

pub(super) fn run_tuple_call(
    plan: &ExecutionPlan,
    function: TupleFunctionId,
    args: &[CallArg],
    caller_frame: &mut Frame,
) -> ExecutionResult<Vec<Value>> {
    let frame = bind_arguments(
        plan,
        args,
        caller_frame,
        plan.tuple_function(function).frame_layout(),
    )?;
    run_tuple_loop(plan, function, frame)
}

pub(super) fn run_list_call(
    plan: &ExecutionPlan,
    function: ListFunctionId,
    args: &[CallArg],
    caller_frame: &mut Frame,
) -> ExecutionResult<crate::runtime::ListValue> {
    let frame = bind_arguments(
        plan,
        args,
        caller_frame,
        list_function_frame_layout(plan, &function),
    )?;
    run_list_loop(plan, function, frame)
}

pub(in crate::runtime) fn run_int_list_call(
    plan: &ExecutionPlan,
    function: crate::plan::execution::IntListFunctionId,
    args: &[CallArg],
    caller_frame: &mut Frame,
) -> ExecutionResult<Vec<BigInt>> {
    let frame = bind_arguments(
        plan,
        args,
        caller_frame,
        plan.int_list_function(function).frame_layout(),
    )?;
    run_int_list_loop(plan, function, frame)
}

pub(in crate::runtime) fn run_string_list_call(
    plan: &ExecutionPlan,
    function: crate::plan::execution::StringListFunctionId,
    args: &[CallArg],
    caller_frame: &mut Frame,
) -> ExecutionResult<Vec<EcoString>> {
    let frame = bind_arguments(
        plan,
        args,
        caller_frame,
        plan.string_list_function(function).frame_layout(),
    )?;
    run_string_list_loop(plan, function, frame)
}

pub(in crate::runtime) fn run_float_list_call(
    plan: &ExecutionPlan,
    function: crate::plan::execution::FloatListFunctionId,
    args: &[CallArg],
    caller_frame: &mut Frame,
) -> ExecutionResult<Vec<f64>> {
    let frame = bind_arguments(
        plan,
        args,
        caller_frame,
        plan.float_list_function(function).frame_layout(),
    )?;
    run_float_list_loop(plan, function, frame)
}

pub(in crate::runtime) fn run_bool_list_call(
    plan: &ExecutionPlan,
    function: crate::plan::execution::BoolListFunctionId,
    args: &[CallArg],
    caller_frame: &mut Frame,
) -> ExecutionResult<Vec<bool>> {
    let frame = bind_arguments(
        plan,
        args,
        caller_frame,
        plan.bool_list_function(function).frame_layout(),
    )?;
    run_bool_list_loop(plan, function, frame)
}

pub(in crate::runtime) fn run_nil_list_call(
    plan: &ExecutionPlan,
    function: crate::plan::execution::NilListFunctionId,
    args: &[CallArg],
    caller_frame: &mut Frame,
) -> ExecutionResult<usize> {
    let frame = bind_arguments(
        plan,
        args,
        caller_frame,
        plan.nil_list_function(function).frame_layout(),
    )?;
    run_nil_list_loop(plan, function, frame)
}

pub(in crate::runtime) fn run_tuple_list_call(
    plan: &ExecutionPlan,
    function: crate::plan::execution::TupleListFunctionId,
    args: &[CallArg],
    caller_frame: &mut Frame,
) -> ExecutionResult<Vec<Vec<Value>>> {
    let frame = bind_arguments(
        plan,
        args,
        caller_frame,
        plan.tuple_list_function(function).frame_layout(),
    )?;
    run_tuple_list_loop(plan, function, frame)
}

pub(in crate::runtime) fn run_list_list_call(
    plan: &ExecutionPlan,
    function: crate::plan::execution::ListListFunctionId,
    args: &[CallArg],
    caller_frame: &mut Frame,
) -> ExecutionResult<Vec<crate::runtime::ListValue>> {
    let frame = bind_arguments(
        plan,
        args,
        caller_frame,
        plan.list_list_function(function).frame_layout(),
    )?;
    run_list_list_loop(plan, function, frame)
}

pub(in crate::runtime) fn run_function_list_call(
    plan: &ExecutionPlan,
    function: crate::plan::execution::FunctionListFunctionId,
    args: &[CallArg],
    caller_frame: &mut Frame,
) -> ExecutionResult<Vec<FunctionValue>> {
    let frame = bind_arguments(
        plan,
        args,
        caller_frame,
        plan.function_list_function(function).frame_layout(),
    )?;
    run_function_list_loop(plan, function, frame)
}

pub(in crate::runtime) fn run_int_function_call(
    plan: &ExecutionPlan,
    function: &crate::plan::execution::IntFunctionExpr,
    args: &[CallArg],
    caller_frame: &mut Frame,
) -> ExecutionResult<BigInt> {
    let function = eval_int_function_expr(plan, caller_frame, function)?;
    let runtime_function = plan.int_function(function.runtime_id());
    let frame_layout = runtime_function.frame_layout();
    let frame =
        bind_function_value_arguments(plan, args, caller_frame, frame_layout, function.captures())?;
    run_int_loop(plan, function.runtime_id(), frame)
}

pub(in crate::runtime) fn run_string_function_call(
    plan: &ExecutionPlan,
    function: &crate::plan::execution::StringFunctionExpr,
    args: &[CallArg],
    caller_frame: &mut Frame,
) -> ExecutionResult<EcoString> {
    let function = eval_string_function_expr(plan, caller_frame, function)?;
    let runtime_function = plan.string_function(function.runtime_id());
    let frame_layout = runtime_function.frame_layout();
    let frame =
        bind_function_value_arguments(plan, args, caller_frame, frame_layout, function.captures())?;
    run_string_loop(plan, function.runtime_id(), frame)
}

pub(in crate::runtime) fn run_float_function_call(
    plan: &ExecutionPlan,
    function: &crate::plan::execution::FloatFunctionExpr,
    args: &[CallArg],
    caller_frame: &mut Frame,
) -> ExecutionResult<f64> {
    let function = eval_float_function_expr(plan, caller_frame, function)?;
    let runtime_function = plan.float_function(function.runtime_id());
    let frame_layout = runtime_function.frame_layout();
    let frame =
        bind_function_value_arguments(plan, args, caller_frame, frame_layout, function.captures())?;
    run_float_loop(plan, function.runtime_id(), frame)
}

pub(in crate::runtime) fn run_bool_function_call(
    plan: &ExecutionPlan,
    function: &crate::plan::execution::BoolFunctionExpr,
    args: &[CallArg],
    caller_frame: &mut Frame,
) -> ExecutionResult<bool> {
    let function = eval_bool_function_expr(plan, caller_frame, function)?;
    let runtime_function = plan.bool_function(function.runtime_id());
    let frame_layout = runtime_function.frame_layout();
    let frame =
        bind_function_value_arguments(plan, args, caller_frame, frame_layout, function.captures())?;
    run_bool_loop(plan, function.runtime_id(), frame)
}

pub(in crate::runtime) fn run_nil_function_call(
    plan: &ExecutionPlan,
    function: &crate::plan::execution::NilFunctionExpr,
    args: &[CallArg],
    caller_frame: &mut Frame,
) -> ExecutionResult<()> {
    let function = eval_nil_function_expr(plan, caller_frame, function)?;
    let runtime_function = plan.nil_function(function.runtime_id());
    let frame_layout = runtime_function.frame_layout();
    let frame =
        bind_function_value_arguments(plan, args, caller_frame, frame_layout, function.captures())?;
    run_nil_loop(plan, function.runtime_id(), frame)
}

pub(in crate::runtime) fn run_tuple_function_call(
    plan: &ExecutionPlan,
    function: &crate::plan::execution::TupleFunctionExpr,
    args: &[CallArg],
    caller_frame: &mut Frame,
) -> ExecutionResult<Vec<Value>> {
    let function = eval_tuple_function_expr(plan, caller_frame, function)?;
    let runtime_function = plan.tuple_function(function.runtime_id());
    let frame_layout = runtime_function.frame_layout();
    let frame =
        bind_function_value_arguments(plan, args, caller_frame, frame_layout, function.captures())?;
    run_tuple_loop(plan, function.runtime_id(), frame)
}

pub(in crate::runtime) fn run_int_list_function_call(
    plan: &ExecutionPlan,
    function: &crate::plan::execution::ListFunctionExpr,
    args: &[CallArg],
    caller_frame: &mut Frame,
) -> ExecutionResult<Vec<BigInt>> {
    let function = eval_list_function_expr(plan, caller_frame, function)?;
    let ListFunctionId::Int(runtime_id) = function.runtime_id() else {
        return Err(ExecutionError::function_return_family_mismatch(
            FunctionReturnFamily::List,
            FunctionReturnFamily::List,
        ));
    };
    let frame = bind_function_value_arguments(
        plan,
        args,
        caller_frame,
        plan.int_list_function(runtime_id).frame_layout(),
        function.captures(),
    )?;
    run_int_list_loop(plan, runtime_id, frame)
}

pub(in crate::runtime) fn run_string_list_function_call(
    plan: &ExecutionPlan,
    function: &crate::plan::execution::ListFunctionExpr,
    args: &[CallArg],
    caller_frame: &mut Frame,
) -> ExecutionResult<Vec<EcoString>> {
    let function = eval_list_function_expr(plan, caller_frame, function)?;
    let ListFunctionId::String(runtime_id) = function.runtime_id() else {
        return Err(ExecutionError::function_return_family_mismatch(
            FunctionReturnFamily::List,
            FunctionReturnFamily::List,
        ));
    };
    let frame = bind_function_value_arguments(
        plan,
        args,
        caller_frame,
        plan.string_list_function(runtime_id).frame_layout(),
        function.captures(),
    )?;
    run_string_list_loop(plan, runtime_id, frame)
}

pub(in crate::runtime) fn run_float_list_function_call(
    plan: &ExecutionPlan,
    function: &crate::plan::execution::ListFunctionExpr,
    args: &[CallArg],
    caller_frame: &mut Frame,
) -> ExecutionResult<Vec<f64>> {
    let function = eval_list_function_expr(plan, caller_frame, function)?;
    let ListFunctionId::Float(runtime_id) = function.runtime_id() else {
        return Err(ExecutionError::function_return_family_mismatch(
            FunctionReturnFamily::List,
            FunctionReturnFamily::List,
        ));
    };
    let frame = bind_function_value_arguments(
        plan,
        args,
        caller_frame,
        plan.float_list_function(runtime_id).frame_layout(),
        function.captures(),
    )?;
    run_float_list_loop(plan, runtime_id, frame)
}

pub(in crate::runtime) fn run_bool_list_function_call(
    plan: &ExecutionPlan,
    function: &crate::plan::execution::ListFunctionExpr,
    args: &[CallArg],
    caller_frame: &mut Frame,
) -> ExecutionResult<Vec<bool>> {
    let function = eval_list_function_expr(plan, caller_frame, function)?;
    let ListFunctionId::Bool(runtime_id) = function.runtime_id() else {
        return Err(ExecutionError::function_return_family_mismatch(
            FunctionReturnFamily::List,
            FunctionReturnFamily::List,
        ));
    };
    let frame = bind_function_value_arguments(
        plan,
        args,
        caller_frame,
        plan.bool_list_function(runtime_id).frame_layout(),
        function.captures(),
    )?;
    run_bool_list_loop(plan, runtime_id, frame)
}

pub(in crate::runtime) fn run_nil_list_function_call(
    plan: &ExecutionPlan,
    function: &crate::plan::execution::ListFunctionExpr,
    args: &[CallArg],
    caller_frame: &mut Frame,
) -> ExecutionResult<usize> {
    let function = eval_list_function_expr(plan, caller_frame, function)?;
    let ListFunctionId::Nil(runtime_id) = function.runtime_id() else {
        return Err(ExecutionError::function_return_family_mismatch(
            FunctionReturnFamily::List,
            FunctionReturnFamily::List,
        ));
    };
    let frame = bind_function_value_arguments(
        plan,
        args,
        caller_frame,
        plan.nil_list_function(runtime_id).frame_layout(),
        function.captures(),
    )?;
    run_nil_list_loop(plan, runtime_id, frame)
}

pub(in crate::runtime) fn run_tuple_list_function_call(
    plan: &ExecutionPlan,
    function: &crate::plan::execution::ListFunctionExpr,
    args: &[CallArg],
    caller_frame: &mut Frame,
) -> ExecutionResult<Vec<Vec<Value>>> {
    let function = eval_list_function_expr(plan, caller_frame, function)?;
    let ListFunctionId::Tuple(runtime_id) = function.runtime_id() else {
        return Err(ExecutionError::function_return_family_mismatch(
            FunctionReturnFamily::List,
            FunctionReturnFamily::List,
        ));
    };
    let frame = bind_function_value_arguments(
        plan,
        args,
        caller_frame,
        plan.tuple_list_function(runtime_id).frame_layout(),
        function.captures(),
    )?;
    run_tuple_list_loop(plan, runtime_id, frame)
}

pub(in crate::runtime) fn run_list_list_function_call(
    plan: &ExecutionPlan,
    function: &crate::plan::execution::ListFunctionExpr,
    args: &[CallArg],
    caller_frame: &mut Frame,
) -> ExecutionResult<Vec<crate::runtime::ListValue>> {
    let function = eval_list_function_expr(plan, caller_frame, function)?;
    let ListFunctionId::List(runtime_id) = function.runtime_id() else {
        return Err(ExecutionError::function_return_family_mismatch(
            FunctionReturnFamily::List,
            FunctionReturnFamily::List,
        ));
    };
    let frame = bind_function_value_arguments(
        plan,
        args,
        caller_frame,
        plan.list_list_function(runtime_id).frame_layout(),
        function.captures(),
    )?;
    run_list_list_loop(plan, runtime_id, frame)
}

pub(in crate::runtime) fn run_function_list_function_call(
    plan: &ExecutionPlan,
    function: &crate::plan::execution::ListFunctionExpr,
    args: &[CallArg],
    caller_frame: &mut Frame,
) -> ExecutionResult<Vec<FunctionValue>> {
    let function = eval_list_function_expr(plan, caller_frame, function)?;
    let ListFunctionId::Function(runtime_id) = function.runtime_id() else {
        return Err(ExecutionError::function_return_family_mismatch(
            FunctionReturnFamily::List,
            FunctionReturnFamily::List,
        ));
    };
    let frame = bind_function_value_arguments(
        plan,
        args,
        caller_frame,
        plan.function_list_function(runtime_id).frame_layout(),
        function.captures(),
    )?;
    run_function_list_loop(plan, runtime_id, frame)
}

fn list_function_frame_layout<'a>(
    plan: &'a ExecutionPlan,
    function: &ListFunctionId,
) -> &'a crate::plan::execution::FrameLayout {
    match function {
        ListFunctionId::Int(id) => plan.int_list_function(*id).frame_layout(),
        ListFunctionId::String(id) => plan.string_list_function(*id).frame_layout(),
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
    function: IntFunctionFunctionId,
    args: &[CallArg],
    caller_frame: &mut Frame,
) -> ExecutionResult<crate::runtime::IntFunctionValue> {
    let frame = bind_arguments(
        plan,
        args,
        caller_frame,
        plan.int_function_function(function).frame_layout(),
    )?;
    run_int_function_loop(plan, function, frame)
}

pub(in crate::runtime) fn run_float_function_returning_function_call(
    plan: &ExecutionPlan,
    function: FloatFunctionFunctionId,
    args: &[CallArg],
    caller_frame: &mut Frame,
) -> ExecutionResult<crate::runtime::FloatFunctionValue> {
    let frame = bind_arguments(
        plan,
        args,
        caller_frame,
        plan.float_function_function(function).frame_layout(),
    )?;
    run_float_function_loop(plan, function, frame)
}

pub(in crate::runtime) fn run_string_function_returning_function_call(
    plan: &ExecutionPlan,
    function: StringFunctionFunctionId,
    args: &[CallArg],
    caller_frame: &mut Frame,
) -> ExecutionResult<crate::runtime::StringFunctionValue> {
    let frame = bind_arguments(
        plan,
        args,
        caller_frame,
        plan.string_function_function(function).frame_layout(),
    )?;
    run_string_function_loop(plan, function, frame)
}

pub(in crate::runtime) fn run_bool_function_returning_function_call(
    plan: &ExecutionPlan,
    function: BoolFunctionFunctionId,
    args: &[CallArg],
    caller_frame: &mut Frame,
) -> ExecutionResult<crate::runtime::BoolFunctionValue> {
    let frame = bind_arguments(
        plan,
        args,
        caller_frame,
        plan.bool_function_function(function).frame_layout(),
    )?;
    run_bool_function_loop(plan, function, frame)
}

pub(in crate::runtime) fn run_nil_function_returning_function_call(
    plan: &ExecutionPlan,
    function: NilFunctionFunctionId,
    args: &[CallArg],
    caller_frame: &mut Frame,
) -> ExecutionResult<crate::runtime::NilFunctionValue> {
    let frame = bind_arguments(
        plan,
        args,
        caller_frame,
        plan.nil_function_function(function).frame_layout(),
    )?;
    run_nil_function_loop(plan, function, frame)
}

pub(in crate::runtime) fn run_tuple_function_returning_function_call(
    plan: &ExecutionPlan,
    function: TupleFunctionFunctionId,
    args: &[CallArg],
    caller_frame: &mut Frame,
) -> ExecutionResult<crate::runtime::TupleFunctionValue> {
    let frame = bind_arguments(
        plan,
        args,
        caller_frame,
        plan.tuple_function_function(function).frame_layout(),
    )?;
    run_tuple_function_loop(plan, function, frame)
}

pub(in crate::runtime) fn run_list_function_returning_function_call(
    plan: &ExecutionPlan,
    function: ListFunctionFunctionId,
    args: &[CallArg],
    caller_frame: &mut Frame,
) -> ExecutionResult<crate::runtime::ListFunctionValue> {
    let frame = bind_arguments(
        plan,
        args,
        caller_frame,
        plan.list_function_function(&function).frame_layout(),
    )?;
    run_list_function_loop(plan, function, frame)
}

pub(in crate::runtime) fn run_function_function_returning_function_call(
    plan: &ExecutionPlan,
    function: FunctionFunctionFunctionId,
    args: &[CallArg],
    caller_frame: &mut Frame,
) -> ExecutionResult<FunctionFunctionValue> {
    let frame = bind_arguments(
        plan,
        args,
        caller_frame,
        plan.function_function_function(function).frame_layout(),
    )?;
    run_function_function_loop(plan, function, frame)
}

pub(in crate::runtime) fn run_int_function_function_call(
    plan: &ExecutionPlan,
    function: &crate::plan::execution::FunctionFunctionExpr,
    args: &[CallArg],
    caller_frame: &mut Frame,
) -> ExecutionResult<crate::runtime::IntFunctionValue> {
    let function = eval_function_function_expr(plan, caller_frame, function)?;
    let runtime_id = function.runtime_id();
    let function_id = runtime_id
        .int()
        .ok_or(ExecutionError::function_return_family_mismatch(
            FunctionReturnFamily::Int,
            runtime_id.family(),
        ))?;
    let runtime_function = plan.int_function_function(function_id);
    let frame_layout = runtime_function.frame_layout();
    let frame =
        bind_function_value_arguments(plan, args, caller_frame, frame_layout, function.captures())?;
    run_int_function_loop(plan, function_id, frame)
}

pub(in crate::runtime) fn run_float_function_function_call(
    plan: &ExecutionPlan,
    function: &crate::plan::execution::FunctionFunctionExpr,
    args: &[CallArg],
    caller_frame: &mut Frame,
) -> ExecutionResult<crate::runtime::FloatFunctionValue> {
    let function = eval_function_function_expr(plan, caller_frame, function)?;
    let runtime_id = function.runtime_id();
    let function_id = runtime_id
        .float()
        .ok_or(ExecutionError::function_return_family_mismatch(
            FunctionReturnFamily::Float,
            runtime_id.family(),
        ))?;
    let runtime_function = plan.float_function_function(function_id);
    let frame_layout = runtime_function.frame_layout();
    let frame =
        bind_function_value_arguments(plan, args, caller_frame, frame_layout, function.captures())?;
    run_float_function_loop(plan, function_id, frame)
}

pub(in crate::runtime) fn run_string_function_function_call(
    plan: &ExecutionPlan,
    function: &crate::plan::execution::FunctionFunctionExpr,
    args: &[CallArg],
    caller_frame: &mut Frame,
) -> ExecutionResult<crate::runtime::StringFunctionValue> {
    let function = eval_function_function_expr(plan, caller_frame, function)?;
    let runtime_id = function.runtime_id();
    let function_id =
        runtime_id
            .string()
            .ok_or(ExecutionError::function_return_family_mismatch(
                FunctionReturnFamily::String,
                runtime_id.family(),
            ))?;
    let runtime_function = plan.string_function_function(function_id);
    let frame_layout = runtime_function.frame_layout();
    let frame =
        bind_function_value_arguments(plan, args, caller_frame, frame_layout, function.captures())?;
    run_string_function_loop(plan, function_id, frame)
}

pub(in crate::runtime) fn run_bool_function_function_call(
    plan: &ExecutionPlan,
    function: &crate::plan::execution::FunctionFunctionExpr,
    args: &[CallArg],
    caller_frame: &mut Frame,
) -> ExecutionResult<crate::runtime::BoolFunctionValue> {
    let function = eval_function_function_expr(plan, caller_frame, function)?;
    let runtime_id = function.runtime_id();
    let function_id = runtime_id
        .bool()
        .ok_or(ExecutionError::function_return_family_mismatch(
            FunctionReturnFamily::Bool,
            runtime_id.family(),
        ))?;
    let runtime_function = plan.bool_function_function(function_id);
    let frame_layout = runtime_function.frame_layout();
    let frame =
        bind_function_value_arguments(plan, args, caller_frame, frame_layout, function.captures())?;
    run_bool_function_loop(plan, function_id, frame)
}

pub(in crate::runtime) fn run_nil_function_function_call(
    plan: &ExecutionPlan,
    function: &crate::plan::execution::FunctionFunctionExpr,
    args: &[CallArg],
    caller_frame: &mut Frame,
) -> ExecutionResult<crate::runtime::NilFunctionValue> {
    let function = eval_function_function_expr(plan, caller_frame, function)?;
    let runtime_id = function.runtime_id();
    let function_id = runtime_id
        .nil()
        .ok_or(ExecutionError::function_return_family_mismatch(
            FunctionReturnFamily::Nil,
            runtime_id.family(),
        ))?;
    let runtime_function = plan.nil_function_function(function_id);
    let frame_layout = runtime_function.frame_layout();
    let frame =
        bind_function_value_arguments(plan, args, caller_frame, frame_layout, function.captures())?;
    run_nil_function_loop(plan, function_id, frame)
}

pub(in crate::runtime) fn run_tuple_function_function_call(
    plan: &ExecutionPlan,
    function: &crate::plan::execution::FunctionFunctionExpr,
    args: &[CallArg],
    caller_frame: &mut Frame,
) -> ExecutionResult<crate::runtime::TupleFunctionValue> {
    let function = eval_function_function_expr(plan, caller_frame, function)?;
    let runtime_id = function.runtime_id();
    let function_id = runtime_id
        .tuple()
        .ok_or(ExecutionError::function_return_family_mismatch(
            FunctionReturnFamily::Tuple,
            runtime_id.family(),
        ))?;
    let runtime_function = plan.tuple_function_function(function_id);
    let frame_layout = runtime_function.frame_layout();
    let frame =
        bind_function_value_arguments(plan, args, caller_frame, frame_layout, function.captures())?;
    run_tuple_function_loop(plan, function_id, frame)
}

pub(in crate::runtime) fn run_list_function_function_call(
    plan: &ExecutionPlan,
    function: &crate::plan::execution::FunctionFunctionExpr,
    args: &[CallArg],
    caller_frame: &mut Frame,
) -> ExecutionResult<crate::runtime::ListFunctionValue> {
    let function = eval_function_function_expr(plan, caller_frame, function)?;
    let runtime_id = function.runtime_id();
    let function_id = runtime_id
        .list()
        .ok_or(ExecutionError::function_return_family_mismatch(
            FunctionReturnFamily::List,
            runtime_id.family(),
        ))?;
    let runtime_function = plan.list_function_function(&function_id);
    let frame_layout = runtime_function.frame_layout();
    let frame =
        bind_function_value_arguments(plan, args, caller_frame, frame_layout, function.captures())?;
    run_list_function_loop(plan, function_id, frame)
}

pub(in crate::runtime) fn run_function_function_function_call(
    plan: &ExecutionPlan,
    function: &crate::plan::execution::FunctionFunctionExpr,
    args: &[CallArg],
    caller_frame: &mut Frame,
) -> ExecutionResult<FunctionFunctionValue> {
    let function = eval_function_function_expr(plan, caller_frame, function)?;
    let runtime_id = function.runtime_id();
    let function_id =
        runtime_id
            .function()
            .ok_or(ExecutionError::function_return_family_mismatch(
                FunctionReturnFamily::Function,
                runtime_id.family(),
            ))?;
    let runtime_function = plan.function_function_function(function_id);
    let frame_layout = runtime_function.frame_layout();
    let frame =
        bind_function_value_arguments(plan, args, caller_frame, frame_layout, function.captures())?;
    run_function_function_loop(plan, function_id, frame)
}
fn run_function_returning_function_call(
    plan: &ExecutionPlan,
    function: crate::plan::execution::FunctionFunctionId,
    args: &[CallArg],
    caller_frame: &mut Frame,
) -> ExecutionResult<FunctionValue> {
    match function {
        crate::plan::execution::FunctionFunctionId::Int(function) => {
            run_int_function_returning_function_call(plan, function, args, caller_frame)
                .map(Into::into)
        }
        crate::plan::execution::FunctionFunctionId::Float(function) => {
            run_float_function_returning_function_call(plan, function, args, caller_frame)
                .map(Into::into)
        }
        crate::plan::execution::FunctionFunctionId::String(function) => {
            run_string_function_returning_function_call(plan, function, args, caller_frame)
                .map(Into::into)
        }
        crate::plan::execution::FunctionFunctionId::Bool(function) => {
            run_bool_function_returning_function_call(plan, function, args, caller_frame)
                .map(Into::into)
        }
        crate::plan::execution::FunctionFunctionId::Nil(function) => {
            run_nil_function_returning_function_call(plan, function, args, caller_frame)
                .map(Into::into)
        }
        crate::plan::execution::FunctionFunctionId::Tuple(function) => {
            run_tuple_function_returning_function_call(plan, function, args, caller_frame)
                .map(Into::into)
        }
        crate::plan::execution::FunctionFunctionId::List(function) => {
            run_list_function_returning_function_call(plan, function, args, caller_frame)
                .map(Into::into)
        }
        crate::plan::execution::FunctionFunctionId::Function(function) => {
            run_function_function_returning_function_call(plan, function, args, caller_frame)
                .map(Into::into)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        run_bool_function_loop, run_bool_list_loop, run_float_function_loop, run_float_list_loop,
        run_function_function_loop, run_function_list_loop, run_int_function_loop,
        run_int_list_loop, run_list_function_loop, run_list_list_loop, run_nil_function_loop,
        run_nil_list_loop, run_string_function_loop, run_string_list_loop, run_tuple_function_loop,
        run_tuple_list_loop,
    };
    use crate::plan::execution::{
        BoolFunctionFunctionId, CallArg, FloatFunctionFunctionId, FunctionFunctionFunctionId,
        FunctionFunctionId, FunctionFunctionLocalId, FunctionListLocalId, FunctionReturnFamily,
        IntFunctionFunctionId, IntFunctionId, ListFunctionFunctionId, ListFunctionId,
        NilFunctionFunctionId, ReturnBody, ReturnBodyKind, RuntimeFunctionId,
        StringFunctionFunctionId, StringFunctionId, TupleFunctionFunctionId,
    };
    use crate::plan::{FunctionType, ValueType};
    use crate::runtime::frame::Frame;
    use crate::runtime::{ExecutionError, Value, run_main};
    use crate::runtime::{
        FunctionFunctionValue, FunctionValue, FunctionValueKind, ListFunctionValue,
    };

    #[test]
    fn primitive_function_value_calls_propagate_callee_and_argument_panics() {
        let callee_sources = [
            "pub fn main() -> Int { case True { True -> panic as \"callee\" False -> fn() { 0 } }() }",
            "pub fn main() -> String { case True { True -> panic as \"callee\" False -> fn() { \"\" } }() }",
            "pub fn main() -> Float { case True { True -> panic as \"callee\" False -> fn() { 0.0 } }() }",
            "pub fn main() -> Bool { case True { True -> panic as \"callee\" False -> fn() { False } }() }",
            "pub fn main() -> Nil { case True { True -> panic as \"callee\" False -> fn() { Nil } }() }",
            "pub fn main() -> #(Int) { case True { True -> panic as \"callee\" False -> fn() { #(0) } }() }",
        ];
        let argument_sources = [
            "fn callee(value: Int) -> Int { value } pub fn main() { let function = callee function(panic as \"argument\") }",
            "fn callee(value: Int) -> String { \"value\" } pub fn main() { let function = callee function(panic as \"argument\") }",
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
    }

    #[test]
    fn list_function_value_calls_propagate_callee_panics() {
        let sources = [
            "pub fn main() -> List(Int) { case True { True -> panic as \"callee\" False -> fn() { [] } }() }",
            "pub fn main() -> List(String) { case True { True -> panic as \"callee\" False -> fn() { [] } }() }",
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
    }

    #[test]
    fn list_function_calls_reject_wrong_item_families_at_the_transitional_runtime_boundary() {
        let plan = crate::runtime::plan_src(
            r#"
fn int_values(provider: fn() -> List(Int)) { provider() }
fn string_values(provider: fn() -> List(String)) { provider() }
fn float_values(provider: fn() -> List(Float)) { provider() }
fn bool_values(provider: fn() -> List(Bool)) { provider() }
fn nil_values(provider: fn() -> List(Nil)) { provider() }
fn tuple_values(provider: fn() -> List(#(Int))) { provider() }
fn list_values(provider: fn() -> List(List(Int))) { provider() }
fn function_values(provider: fn() -> List(fn() -> Int)) { provider() }
fn wrong_int() -> List(Int) { [] }
fn wrong_string() -> List(String) { [] }

pub fn main() { Nil }
"#,
        );
        let wrong_int = ListFunctionValue::new_with_captures(
            ListFunctionId::Int(plan.int_list_function_id(1)),
            Vec::new(),
            Vec::new(),
            FunctionType::new(Vec::new(), ValueType::List(Box::new(ValueType::Int))),
        );
        let wrong_string = ListFunctionValue::new_with_captures(
            ListFunctionId::String(plan.string_list_function_id(1)),
            Vec::new(),
            Vec::new(),
            FunctionType::new(Vec::new(), ValueType::List(Box::new(ValueType::String))),
        );
        let function = plan.int_list_function(plan.int_list_function_id(0));
        let mut frame = Frame::new(function.frame_layout());
        frame.set_list_function(
            function.frame_layout().list_functions()[0].clone(),
            wrong_string,
        );
        assert_eq!(
            run_int_list_loop(&plan, plan.int_list_function_id(0), frame),
            Err(list_item_family_mismatch()),
        );

        let function = plan.string_list_function(plan.string_list_function_id(0));
        let mut frame = Frame::new(function.frame_layout());
        frame.set_list_function(
            function.frame_layout().list_functions()[0].clone(),
            wrong_int.clone(),
        );
        assert_eq!(
            run_string_list_loop(&plan, plan.string_list_function_id(0), frame),
            Err(list_item_family_mismatch()),
        );

        let function = plan.float_list_function(plan.float_list_function_id(0));
        let mut frame = Frame::new(function.frame_layout());
        frame.set_list_function(
            function.frame_layout().list_functions()[0].clone(),
            wrong_int.clone(),
        );
        assert_eq!(
            run_float_list_loop(&plan, plan.float_list_function_id(0), frame),
            Err(list_item_family_mismatch()),
        );

        let function = plan.bool_list_function(plan.bool_list_function_id(0));
        let mut frame = Frame::new(function.frame_layout());
        frame.set_list_function(
            function.frame_layout().list_functions()[0].clone(),
            wrong_int.clone(),
        );
        assert_eq!(
            run_bool_list_loop(&plan, plan.bool_list_function_id(0), frame),
            Err(list_item_family_mismatch()),
        );

        let function = plan.nil_list_function(plan.nil_list_function_id(0));
        let mut frame = Frame::new(function.frame_layout());
        frame.set_list_function(
            function.frame_layout().list_functions()[0].clone(),
            wrong_int.clone(),
        );
        assert_eq!(
            run_nil_list_loop(&plan, plan.nil_list_function_id(0), frame),
            Err(list_item_family_mismatch()),
        );

        let function = plan.tuple_list_function(plan.tuple_list_function_id(0));
        let mut frame = Frame::new(function.frame_layout());
        frame.set_list_function(
            function.frame_layout().list_functions()[0].clone(),
            wrong_int.clone(),
        );
        assert_eq!(
            run_tuple_list_loop(&plan, plan.tuple_list_function_id(0), frame),
            Err(list_item_family_mismatch()),
        );

        let function = plan.list_list_function(plan.list_list_function_id(0));
        let mut frame = Frame::new(function.frame_layout());
        frame.set_list_function(
            function.frame_layout().list_functions()[0].clone(),
            wrong_int.clone(),
        );
        assert_eq!(
            run_list_list_loop(&plan, plan.list_list_function_id(0), frame),
            Err(list_item_family_mismatch()),
        );

        let function = plan.function_list_function(plan.function_list_function_id(0));
        let mut frame = Frame::new(function.frame_layout());
        frame.set_list_function(
            function.frame_layout().list_functions()[0].clone(),
            wrong_int,
        );
        assert_eq!(
            run_function_list_loop(&plan, plan.function_list_function_id(0), frame),
            Err(list_item_family_mismatch()),
        );
    }

    #[test]
    fn function_returning_function_value_calls_propagate_callee_and_argument_panics() {
        let callee_sources = [
            "pub fn main() -> fn() -> Int { case True { True -> panic as \"callee\" False -> fn() { fn() { 0 } } }() }",
            "pub fn main() -> fn() -> String { case True { True -> panic as \"callee\" False -> fn() { fn() { \"\" } } }() }",
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
    }

    #[test]
    fn direct_calls_propagate_argument_panics_for_every_return_family() {
        let sources = [
            "fn callee(value: Int) -> Int { value } pub fn main() { let _ = callee(panic) 0 }",
            "fn callee(value: Int) -> String { \"value\" } pub fn main() { let _ = callee(panic) 0 }",
            "fn callee(value: Int) -> Float { 1.0 } pub fn main() { let _ = callee(panic) 0 }",
            "fn callee(value: Int) -> Bool { True } pub fn main() { let _ = callee(panic) 0 }",
            "fn callee(value: Int) -> Nil { Nil } pub fn main() { let _ = callee(panic) 0 }",
            "fn callee(value: Int) -> #(Int) { #(value) } pub fn main() { let _ = callee(panic) 0 }",
            "fn callee(value: Int) -> List(Int) { [] } pub fn main() { let _ = callee(panic) 0 }",
            "fn callee(value: Int) -> List(String) { [] } pub fn main() { let _ = callee(panic) 0 }",
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
    }

    #[test]
    fn generic_list_call_propagates_tail_call_argument_panics() {
        let plan = crate::runtime::plan_src(
            "fn callee(value: Int) -> List(Int) { [] } pub fn main() { callee(panic) }",
        );
        let main = plan.int_list_function(plan.int_list_function_id(0));
        let args = expect_tail_call_args(main.return_());
        let mut caller_frame = Frame::default();

        let error = super::run_list_call(
            &plan,
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
    }

    #[test]
    fn non_tail_function_returning_calls_propagate_argument_panics() {
        let sources = [
            "fn callee(value: Int) -> fn() -> Int { panic } pub fn main() { let _ = callee(panic) 0 }",
            "fn callee(value: Int) -> fn() -> String { panic } pub fn main() { let _ = callee(panic) 0 }",
            "fn callee(value: Int) -> fn() -> Float { panic } pub fn main() { let _ = callee(panic) 0 }",
            "fn callee(value: Int) -> fn() -> Bool { panic } pub fn main() { let _ = callee(panic) 0 }",
            "fn callee(value: Int) -> fn() -> Nil { panic } pub fn main() { let _ = callee(panic) 0 }",
            "fn callee(value: Int) -> fn() -> #(Int) { panic } pub fn main() { let _ = callee(panic) 0 }",
            "fn callee(value: Int) -> fn() -> List(Int) { panic } pub fn main() { let _ = callee(panic) 0 }",
            "fn callee(value: Int) -> fn() -> fn() -> Int { panic } pub fn main() { let _ = callee(panic) 0 }",
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
fn float_function(provider: fn() -> fn() -> Float) { provider() }
fn bool_function(provider: fn() -> fn() -> Bool) { provider() }
fn nil_function(provider: fn() -> fn() -> Nil) { provider() }
fn tuple_function(provider: fn() -> fn() -> #(Int)) { provider() }
fn list_function(provider: fn() -> fn() -> List(Int)) { provider() }
fn function_function(provider: fn() -> fn() -> fn() -> Int) { provider() }

pub fn main() { list_function }
"#,
        );
        let list_function_id = expect_list_function_function_id(
            run_main(&plan).expect("main should return list_function"),
        );
        let int_return = FunctionType::new(Vec::new(), ValueType::Int);
        let wrong_string = FunctionFunctionValue::new(
            FunctionFunctionId::String(StringFunctionFunctionId(0)),
            Vec::new(),
            FunctionType::new(Vec::new(), ValueType::String),
        );
        let wrong_int = FunctionFunctionValue::new(
            FunctionFunctionId::Int(IntFunctionFunctionId(0)),
            Vec::new(),
            int_return.clone(),
        );

        let function = plan.int_function_function(IntFunctionFunctionId(0));
        let mut frame = Frame::new(function.frame_layout());
        frame.set_function_function(FunctionFunctionLocalId(0), wrong_string);
        assert_eq!(
            run_int_function_loop(&plan, IntFunctionFunctionId(0), frame),
            Err(ExecutionError::function_return_family_mismatch(
                FunctionReturnFamily::Int,
                FunctionReturnFamily::String,
            )),
        );

        let function = plan.string_function_function(StringFunctionFunctionId(0));
        let mut frame = Frame::new(function.frame_layout());
        frame.set_function_function(FunctionFunctionLocalId(0), wrong_int.clone());
        assert_eq!(
            run_string_function_loop(&plan, StringFunctionFunctionId(0), frame),
            Err(ExecutionError::function_return_family_mismatch(
                FunctionReturnFamily::String,
                FunctionReturnFamily::Int,
            )),
        );

        let function = plan.float_function_function(FloatFunctionFunctionId(0));
        let mut frame = Frame::new(function.frame_layout());
        frame.set_function_function(FunctionFunctionLocalId(0), wrong_int.clone());
        assert_eq!(
            run_float_function_loop(&plan, FloatFunctionFunctionId(0), frame),
            Err(ExecutionError::function_return_family_mismatch(
                FunctionReturnFamily::Float,
                FunctionReturnFamily::Int,
            )),
        );

        let function = plan.bool_function_function(BoolFunctionFunctionId(0));
        let mut frame = Frame::new(function.frame_layout());
        frame.set_function_function(FunctionFunctionLocalId(0), wrong_int.clone());
        assert_eq!(
            run_bool_function_loop(&plan, BoolFunctionFunctionId(0), frame),
            Err(ExecutionError::function_return_family_mismatch(
                FunctionReturnFamily::Bool,
                FunctionReturnFamily::Int,
            )),
        );

        let function = plan.nil_function_function(NilFunctionFunctionId(0));
        let mut frame = Frame::new(function.frame_layout());
        frame.set_function_function(FunctionFunctionLocalId(0), wrong_int.clone());
        assert_eq!(
            run_nil_function_loop(&plan, NilFunctionFunctionId(0), frame),
            Err(ExecutionError::function_return_family_mismatch(
                FunctionReturnFamily::Nil,
                FunctionReturnFamily::Int,
            )),
        );

        let function = plan.tuple_function_function(TupleFunctionFunctionId(0));
        let mut frame = Frame::new(function.frame_layout());
        frame.set_function_function(FunctionFunctionLocalId(0), wrong_int.clone());
        assert_eq!(
            run_tuple_function_loop(&plan, TupleFunctionFunctionId(0), frame),
            Err(ExecutionError::function_return_family_mismatch(
                FunctionReturnFamily::Tuple,
                FunctionReturnFamily::Int,
            )),
        );

        let function = plan.list_function_function(&list_function_id);
        let mut frame = Frame::new(function.frame_layout());
        frame.set_function_function(FunctionFunctionLocalId(0), wrong_int.clone());
        assert_eq!(
            run_list_function_loop(&plan, list_function_id, frame),
            Err(ExecutionError::function_return_family_mismatch(
                FunctionReturnFamily::List,
                FunctionReturnFamily::Int,
            )),
        );

        let function = plan.function_function_function(FunctionFunctionFunctionId(1));
        let mut frame = Frame::new(function.frame_layout());
        frame.set_function_function(FunctionFunctionLocalId(0), wrong_int);
        assert_eq!(
            run_function_function_loop(&plan, FunctionFunctionFunctionId(1), frame),
            Err(ExecutionError::function_return_family_mismatch(
                FunctionReturnFamily::Function,
                FunctionReturnFamily::Int,
            )),
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

pub fn main() { list_function }
"#,
        );
        let list_function_id = expect_list_function_function_id(
            run_main(&plan).expect("main should return list_function"),
        );
        let wrong_string = FunctionValue::new(
            RuntimeFunctionId::String(StringFunctionId(0)),
            Vec::new(),
            FunctionType::new(Vec::new(), ValueType::String),
        );
        let wrong_int = FunctionValue::new(
            RuntimeFunctionId::Int(IntFunctionId(0)),
            Vec::new(),
            FunctionType::new(Vec::new(), ValueType::Int),
        );

        let function = plan.int_function_function(IntFunctionFunctionId(0));
        let mut frame = Frame::new(function.frame_layout());
        frame.set_function_list(FunctionListLocalId(0), vec![wrong_string]);
        assert_eq!(
            run_int_function_loop(&plan, IntFunctionFunctionId(0), frame),
            Err(ExecutionError::function_return_family_mismatch(
                FunctionReturnFamily::Int,
                FunctionReturnFamily::String,
            )),
        );

        let function = plan.string_function_function(StringFunctionFunctionId(0));
        let mut frame = Frame::new(function.frame_layout());
        frame.set_function_list(FunctionListLocalId(0), vec![wrong_int.clone()]);
        assert_eq!(
            run_string_function_loop(&plan, StringFunctionFunctionId(0), frame),
            Err(ExecutionError::function_return_family_mismatch(
                FunctionReturnFamily::String,
                FunctionReturnFamily::Int,
            )),
        );

        let function = plan.float_function_function(FloatFunctionFunctionId(0));
        let mut frame = Frame::new(function.frame_layout());
        frame.set_function_list(FunctionListLocalId(0), vec![wrong_int.clone()]);
        assert_eq!(
            run_float_function_loop(&plan, FloatFunctionFunctionId(0), frame),
            Err(ExecutionError::function_return_family_mismatch(
                FunctionReturnFamily::Float,
                FunctionReturnFamily::Int,
            )),
        );

        let function = plan.bool_function_function(BoolFunctionFunctionId(0));
        let mut frame = Frame::new(function.frame_layout());
        frame.set_function_list(FunctionListLocalId(0), vec![wrong_int.clone()]);
        assert_eq!(
            run_bool_function_loop(&plan, BoolFunctionFunctionId(0), frame),
            Err(ExecutionError::function_return_family_mismatch(
                FunctionReturnFamily::Bool,
                FunctionReturnFamily::Int,
            )),
        );

        let function = plan.nil_function_function(NilFunctionFunctionId(0));
        let mut frame = Frame::new(function.frame_layout());
        frame.set_function_list(FunctionListLocalId(0), vec![wrong_int.clone()]);
        assert_eq!(
            run_nil_function_loop(&plan, NilFunctionFunctionId(0), frame),
            Err(ExecutionError::function_return_family_mismatch(
                FunctionReturnFamily::Nil,
                FunctionReturnFamily::Int,
            )),
        );

        let function = plan.tuple_function_function(TupleFunctionFunctionId(0));
        let mut frame = Frame::new(function.frame_layout());
        frame.set_function_list(FunctionListLocalId(0), vec![wrong_int.clone()]);
        assert_eq!(
            run_tuple_function_loop(&plan, TupleFunctionFunctionId(0), frame),
            Err(ExecutionError::function_return_family_mismatch(
                FunctionReturnFamily::Tuple,
                FunctionReturnFamily::Int,
            )),
        );

        let function = plan.list_function_function(&list_function_id);
        let mut frame = Frame::new(function.frame_layout());
        frame.set_function_list(FunctionListLocalId(0), vec![wrong_int.clone()]);
        assert_eq!(
            run_list_function_loop(&plan, list_function_id, frame),
            Err(ExecutionError::function_return_family_mismatch(
                FunctionReturnFamily::List,
                FunctionReturnFamily::Int,
            )),
        );

        let function = plan.function_function_function(FunctionFunctionFunctionId(1));
        let mut frame = Frame::new(function.frame_layout());
        frame.set_function_list(FunctionListLocalId(0), vec![wrong_int]);
        assert_eq!(
            run_function_function_loop(&plan, FunctionFunctionFunctionId(1), frame),
            Err(ExecutionError::function_return_family_mismatch(
                FunctionReturnFamily::Function,
                FunctionReturnFamily::Int,
            )),
        );
    }

    #[test]
    #[should_panic(expected = "expected a function returning a list function")]
    fn list_function_function_fixture_guard_rejects_non_function_value() {
        let _ = expect_list_function_function_id(Value::Int(0.into()));
    }

    #[test]
    #[should_panic(expected = "expected a function returning a list function")]
    fn list_function_function_fixture_guard_rejects_primitive_function() {
        let value = crate::runtime::run_src("pub fn main() { fn() { 1 } }");
        let _ = expect_list_function_function_id(value);
    }

    #[test]
    #[should_panic(expected = "expected a function returning a list function")]
    fn list_function_function_fixture_guard_rejects_int_function_function() {
        let value = crate::runtime::run_src("pub fn main() { fn() { fn() { 1 } } }");
        let _ = expect_list_function_function_id(value);
    }

    fn expect_tail_call_args<Expression, Function>(
        body: &ReturnBody<Expression, Function>,
    ) -> &[CallArg] {
        match body.kind() {
            ReturnBodyKind::TailCall { args, .. } => args,
            _ => panic!("expected a tail-call return body"),
        }
    }

    fn expect_list_function_function_id(value: Value) -> ListFunctionFunctionId {
        match value {
            Value::Function(function) => match function.kind() {
                FunctionValueKind::Function(function) => match function.runtime_id() {
                    FunctionFunctionId::List(function) => function,
                    _ => panic!("expected a function returning a list function"),
                },
                _ => panic!("expected a function returning a list function"),
            },
            _ => panic!("expected a function returning a list function"),
        }
    }

    fn list_item_family_mismatch() -> ExecutionError {
        ExecutionError::function_return_family_mismatch(
            FunctionReturnFamily::List,
            FunctionReturnFamily::List,
        )
    }
}
