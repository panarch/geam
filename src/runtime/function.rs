mod bind;
mod return_body;
mod steps;

pub(in crate::runtime) use bind::eval_capture_args;
pub(in crate::runtime) use steps::execute_steps;

use crate::plan::{
    BoolFunctionFunctionId, BoolFunctionId, CallArg, ExecutionPlan, FloatFunctionFunctionId,
    FloatFunctionId, FunctionFunctionFunctionId, FunctionFunctionValue, FunctionReturnFamily,
    FunctionValue, IntFunctionFunctionId, IntFunctionId, ListFunctionFunctionId, ListFunctionId,
    NilFunctionFunctionId, NilFunctionId, RuntimeFunctionId, StringFunctionFunctionId,
    StringFunctionId, TupleFunctionFunctionId, TupleFunctionId, Value,
};
use crate::runtime::ExecutionError;
use crate::runtime::error::ExecutionResult;
use crate::runtime::expression::{
    eval_bool_function_expr, eval_float_function_expr, eval_function_function_expr,
    eval_int_function_expr, eval_list_function_expr, eval_nil_function_expr,
    eval_string_function_expr, eval_tuple_function_expr,
};
use crate::runtime::frame::Frame;
use bind::{bind_arguments, bind_function_value_arguments};
use ecow::EcoString;
use num_bigint::BigInt;
use return_body::{
    run_bool_function_loop, run_bool_loop, run_float_function_loop, run_float_loop,
    run_function_function_loop, run_int_function_loop, run_int_loop, run_list_function_loop,
    run_list_loop, run_nil_function_loop, run_nil_loop, run_string_function_loop, run_string_loop,
    run_tuple_function_loop, run_tuple_loop,
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
        RuntimeFunctionId::List { id, .. } => {
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
) -> ExecutionResult<crate::plan::ListValue> {
    let frame = bind_arguments(
        plan,
        args,
        caller_frame,
        plan.list_function(function).frame_layout(),
    )?;
    run_list_loop(plan, function, frame)
}

pub(in crate::runtime) fn run_int_function_call(
    plan: &ExecutionPlan,
    function: &crate::plan::IntFunctionExpr,
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
    function: &crate::plan::StringFunctionExpr,
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
    function: &crate::plan::FloatFunctionExpr,
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
    function: &crate::plan::BoolFunctionExpr,
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
    function: &crate::plan::NilFunctionExpr,
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
    function: &crate::plan::TupleFunctionExpr,
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

pub(in crate::runtime) fn run_list_function_call(
    plan: &ExecutionPlan,
    function: &crate::plan::ListFunctionExpr,
    args: &[CallArg],
    caller_frame: &mut Frame,
) -> ExecutionResult<crate::plan::ListValue> {
    let function = eval_list_function_expr(plan, caller_frame, function)?;
    let runtime_function = plan.list_function(function.runtime_id());
    let frame_layout = runtime_function.frame_layout();
    let frame =
        bind_function_value_arguments(plan, args, caller_frame, frame_layout, function.captures())?;
    run_list_loop(plan, function.runtime_id(), frame)
}

pub(in crate::runtime) fn run_int_function_returning_function_call(
    plan: &ExecutionPlan,
    function: IntFunctionFunctionId,
    args: &[CallArg],
    caller_frame: &mut Frame,
) -> ExecutionResult<crate::plan::IntFunctionValue> {
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
) -> ExecutionResult<crate::plan::FloatFunctionValue> {
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
) -> ExecutionResult<crate::plan::StringFunctionValue> {
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
) -> ExecutionResult<crate::plan::BoolFunctionValue> {
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
) -> ExecutionResult<crate::plan::NilFunctionValue> {
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
) -> ExecutionResult<crate::plan::TupleFunctionValue> {
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
) -> ExecutionResult<crate::plan::ListFunctionValue> {
    let frame = bind_arguments(
        plan,
        args,
        caller_frame,
        plan.list_function_function(function).frame_layout(),
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
    function: &crate::plan::FunctionFunctionExpr,
    args: &[CallArg],
    caller_frame: &mut Frame,
) -> ExecutionResult<crate::plan::IntFunctionValue> {
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
    function: &crate::plan::FunctionFunctionExpr,
    args: &[CallArg],
    caller_frame: &mut Frame,
) -> ExecutionResult<crate::plan::FloatFunctionValue> {
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
    function: &crate::plan::FunctionFunctionExpr,
    args: &[CallArg],
    caller_frame: &mut Frame,
) -> ExecutionResult<crate::plan::StringFunctionValue> {
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
    function: &crate::plan::FunctionFunctionExpr,
    args: &[CallArg],
    caller_frame: &mut Frame,
) -> ExecutionResult<crate::plan::BoolFunctionValue> {
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
    function: &crate::plan::FunctionFunctionExpr,
    args: &[CallArg],
    caller_frame: &mut Frame,
) -> ExecutionResult<crate::plan::NilFunctionValue> {
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
    function: &crate::plan::FunctionFunctionExpr,
    args: &[CallArg],
    caller_frame: &mut Frame,
) -> ExecutionResult<crate::plan::TupleFunctionValue> {
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
    function: &crate::plan::FunctionFunctionExpr,
    args: &[CallArg],
    caller_frame: &mut Frame,
) -> ExecutionResult<crate::plan::ListFunctionValue> {
    let function = eval_function_function_expr(plan, caller_frame, function)?;
    let runtime_id = function.runtime_id();
    let function_id = runtime_id
        .list()
        .ok_or(ExecutionError::function_return_family_mismatch(
            FunctionReturnFamily::List,
            runtime_id.family(),
        ))?;
    let runtime_function = plan.list_function_function(function_id);
    let frame_layout = runtime_function.frame_layout();
    let frame =
        bind_function_value_arguments(plan, args, caller_frame, frame_layout, function.captures())?;
    run_list_function_loop(plan, function_id, frame)
}

pub(in crate::runtime) fn run_function_function_function_call(
    plan: &ExecutionPlan,
    function: &crate::plan::FunctionFunctionExpr,
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
    function: crate::plan::FunctionFunctionId,
    args: &[CallArg],
    caller_frame: &mut Frame,
) -> ExecutionResult<FunctionValue> {
    match function {
        crate::plan::FunctionFunctionId::Int(function) => {
            run_int_function_returning_function_call(plan, function, args, caller_frame)
                .map(Into::into)
        }
        crate::plan::FunctionFunctionId::Float(function) => {
            run_float_function_returning_function_call(plan, function, args, caller_frame)
                .map(Into::into)
        }
        crate::plan::FunctionFunctionId::String(function) => {
            run_string_function_returning_function_call(plan, function, args, caller_frame)
                .map(Into::into)
        }
        crate::plan::FunctionFunctionId::Bool(function) => {
            run_bool_function_returning_function_call(plan, function, args, caller_frame)
                .map(Into::into)
        }
        crate::plan::FunctionFunctionId::Nil(function) => {
            run_nil_function_returning_function_call(plan, function, args, caller_frame)
                .map(Into::into)
        }
        crate::plan::FunctionFunctionId::Tuple(function) => {
            run_tuple_function_returning_function_call(plan, function, args, caller_frame)
                .map(Into::into)
        }
        crate::plan::FunctionFunctionId::List(function) => {
            run_list_function_returning_function_call(plan, function, args, caller_frame)
                .map(Into::into)
        }
        crate::plan::FunctionFunctionId::Function(function) => {
            run_function_function_returning_function_call(plan, function, args, caller_frame)
                .map(Into::into)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        run_bool_call, run_bool_function_call, run_bool_function_function_call,
        run_bool_function_returning_function_call, run_float_call, run_float_function_call,
        run_float_function_function_call, run_float_function_returning_function_call,
        run_function_function_function_call, run_function_function_returning_function_call,
        run_function_returning_function_call, run_int_call, run_int_function_call,
        run_int_function_function_call, run_int_function_returning_function_call, run_list_call,
        run_list_function_call, run_list_function_function_call,
        run_list_function_returning_function_call, run_main, run_nil_call, run_nil_function_call,
        run_nil_function_function_call, run_nil_function_returning_function_call, run_string_call,
        run_string_function_call, run_string_function_function_call,
        run_string_function_returning_function_call, run_tuple_call, run_tuple_function_call,
        run_tuple_function_function_call, run_tuple_function_returning_function_call,
    };
    use crate::plan::{
        BoolExpr, BoolFunctionExpr, BoolFunctionFunctionId, BoolFunctionId, BoolFunctionValue,
        BoolLocalId, CallArg, ExecutionPlan, Expr, FloatExpr, FloatFunctionExpr,
        FloatFunctionFunctionId, FloatFunctionId, FloatFunctionValue, FunctionExpr,
        FunctionExprKind, FunctionFunctionExpr, FunctionFunctionFunctionId, FunctionFunctionId,
        FunctionFunctionLocalId, FunctionFunctionValue, FunctionId, FunctionPlan,
        FunctionReturnFamily, FunctionType, FunctionValue, IntExpr, IntFunctionExpr,
        IntFunctionFunctionId, IntFunctionId, IntFunctionValue, IntLocalId, ListExpr,
        ListFunctionExpr, ListFunctionFunctionId, ListFunctionId, ListFunctionValue, ListValue,
        NilExpr, NilFunctionExpr, NilFunctionFunctionId, NilFunctionId, NilFunctionValue,
        NilLocalId, ParamLocal, ReturnBody, ReturnExpr, RuntimeFunctionId, Step, StringExpr,
        StringFunctionExpr, StringFunctionFunctionId, StringFunctionId, StringFunctionValue,
        StringLocalId, TupleExpr, TupleFunctionExpr, TupleFunctionFunctionId, TupleFunctionId,
        TupleFunctionValue, ValueType,
    };
    use crate::runtime::frame::Frame;
    use crate::runtime::{ExecutionError, Value, run_src};

    #[test]
    fn function_function_call_returns_execution_error_on_return_family_mismatch() {
        let plan = plan();

        assert_function_return_family_mismatch(
            run_int_function_function_call(
                &plan,
                &function_function_expr(FunctionFunctionId::String(StringFunctionFunctionId(0))),
                &[],
                &mut Frame::default(),
            ),
            FunctionReturnFamily::Int,
            FunctionReturnFamily::String,
        );
        assert_function_return_family_mismatch(
            run_string_function_function_call(
                &plan,
                &function_function_expr(FunctionFunctionId::Int(IntFunctionFunctionId(0))),
                &[],
                &mut Frame::default(),
            ),
            FunctionReturnFamily::String,
            FunctionReturnFamily::Int,
        );
        assert_function_return_family_mismatch(
            run_float_function_function_call(
                &plan,
                &function_function_expr(FunctionFunctionId::Int(IntFunctionFunctionId(0))),
                &[],
                &mut Frame::default(),
            ),
            FunctionReturnFamily::Float,
            FunctionReturnFamily::Int,
        );
        assert_function_return_family_mismatch(
            run_bool_function_function_call(
                &plan,
                &function_function_expr(FunctionFunctionId::Int(IntFunctionFunctionId(0))),
                &[],
                &mut Frame::default(),
            ),
            FunctionReturnFamily::Bool,
            FunctionReturnFamily::Int,
        );
        assert_function_return_family_mismatch(
            run_nil_function_function_call(
                &plan,
                &function_function_expr(FunctionFunctionId::Int(IntFunctionFunctionId(0))),
                &[],
                &mut Frame::default(),
            ),
            FunctionReturnFamily::Nil,
            FunctionReturnFamily::Int,
        );
        assert_function_return_family_mismatch(
            run_tuple_function_function_call(
                &plan,
                &function_function_expr(FunctionFunctionId::Int(IntFunctionFunctionId(0))),
                &[],
                &mut Frame::default(),
            ),
            FunctionReturnFamily::Tuple,
            FunctionReturnFamily::Int,
        );
        assert_function_return_family_mismatch(
            run_list_function_function_call(
                &plan,
                &function_function_expr(FunctionFunctionId::Int(IntFunctionFunctionId(0))),
                &[],
                &mut Frame::default(),
            ),
            FunctionReturnFamily::List,
            FunctionReturnFamily::Int,
        );
        assert_expected_function_got_int(run_function_function_function_call(
            &plan,
            &function_function_expr(FunctionFunctionId::Int(IntFunctionFunctionId(0))),
            &[],
            &mut Frame::default(),
        ));
    }

    #[test]
    fn function_function_call_returns_function_values() {
        assert_eq!(
            run_src(
                r#"
fn int_identity(value: Int) {
  value
}

fn string_identity(value: String) {
  value
}

fn float_identity(value: Float) {
  value
}

fn bool_identity(value: Bool) {
  value
}

fn nil_identity(value: Nil) {
  value
}

fn get_int() {
  int_identity
}

fn get_string() {
  string_identity
}

fn get_float() {
  float_identity
}

fn get_bool() {
  bool_identity
}

fn get_nil() {
  nil_identity
}

fn get_list() {
  list_identity
}

fn list_identity(value: List(Int)) {
  value
}

fn get_get_int() {
  get_int
}

fn get_get_get_int() {
  get_get_int
}

pub fn main() {
  let int_ok = get_int()(1) + get_get_get_int()()()(2) == 3
  let float_ok = get_float()(1.5) == 1.5
  let bool_ok = get_bool()(True)
  let list_ok = get_list()([1]) == [1]

  get_nil()(Nil)

  case int_ok && float_ok && bool_ok && list_ok {
    True -> get_string()("ge") <> get_string()("am")
    False -> "bad"
  }
}
"#,
            ),
            Value::String("geam".into()),
        );
    }

    #[test]
    fn run_src_returns_function_value_shapes() {
        assert_eq!(
            run_src(
                r#"
fn identity(value: Int) {
  value
}

pub fn main() {
  identity
}
"#,
            ),
            Value::Function(FunctionValue::new(
                RuntimeFunctionId::Int(IntFunctionId(0)),
                vec![ParamLocal::int(IntLocalId(0))],
            )),
        );
        assert_eq!(
            run_src(
                r#"
fn identity(value: String) {
  value
}

pub fn main() {
  identity
}
"#,
            ),
            Value::Function(FunctionValue::new(
                RuntimeFunctionId::String(StringFunctionId(0)),
                vec![ParamLocal::string(StringLocalId(0))],
            )),
        );
        assert_eq!(
            run_src(
                r#"
fn identity(value: Float) {
  value
}

pub fn main() {
  identity
}
"#,
            ),
            Value::Function(FunctionValue::new(
                RuntimeFunctionId::Float(FloatFunctionId(0)),
                vec![ParamLocal::float(crate::plan::FloatLocalId(0))],
            )),
        );
        assert_eq!(
            run_src(
                r#"
fn identity(value: Bool) {
  value
}

pub fn main() {
  identity
}
"#,
            ),
            Value::Function(FunctionValue::new(
                RuntimeFunctionId::Bool(BoolFunctionId(0)),
                vec![ParamLocal::bool(BoolLocalId(0))],
            )),
        );
        assert_eq!(
            run_src(
                r#"
fn identity(value: Nil) {
  value
}

pub fn main() {
  identity
}
"#,
            ),
            Value::Function(FunctionValue::new(
                RuntimeFunctionId::Nil(NilFunctionId(0)),
                vec![ParamLocal::nil(NilLocalId(0))],
            )),
        );
        assert_eq!(
            run_src(
                r#"
fn identity(value: List(Int)) {
  value
}

pub fn main() {
  identity
}
"#,
            ),
            Value::Function(FunctionValue::new(
                RuntimeFunctionId::List {
                    id: ListFunctionId(0),
                    return_type: Box::new(ValueType::Int),
                },
                vec![ParamLocal::list(
                    crate::plan::ListLocalId(0),
                    ValueType::Int
                )],
            )),
        );
        assert_eq!(
            run_src(
                r#"
fn add_one(value: Int) {
  value + 1
}

fn get() {
  add_one
}

pub fn main() {
  get
}
"#,
            ),
            Value::Function(FunctionValue::new(
                RuntimeFunctionId::Function {
                    id: FunctionFunctionId::Int(IntFunctionFunctionId(0)),
                    return_type: FunctionType::new(vec![ValueType::Int], ValueType::Int),
                },
                Vec::new(),
            )),
        );
    }

    #[test]
    fn function_function_projection_returns_typed_function_values() {
        let plan = plan_with_function_function_steps(Vec::new());

        assert_eq!(
            run_int_function_function_call(
                &plan,
                &function_function_expr(FunctionFunctionId::Int(IntFunctionFunctionId(0))),
                &[],
                &mut Frame::default(),
            )
            .map(|value| value.type_().return_().clone()),
            Ok(ValueType::Int),
        );
        assert_eq!(
            run_string_function_function_call(
                &plan,
                &function_function_expr(FunctionFunctionId::String(StringFunctionFunctionId(0))),
                &[],
                &mut Frame::default(),
            )
            .map(|value| value.type_().return_().clone()),
            Ok(ValueType::String),
        );
        assert_eq!(
            run_float_function_function_call(
                &plan,
                &function_function_expr(FunctionFunctionId::Float(FloatFunctionFunctionId(0))),
                &[],
                &mut Frame::default(),
            )
            .map(|value| value.type_().return_().clone()),
            Ok(ValueType::Float),
        );
        assert_eq!(
            run_bool_function_function_call(
                &plan,
                &function_function_expr(FunctionFunctionId::Bool(BoolFunctionFunctionId(0))),
                &[],
                &mut Frame::default(),
            )
            .map(|value| value.type_().return_().clone()),
            Ok(ValueType::Bool),
        );
        assert_eq!(
            run_nil_function_function_call(
                &plan,
                &function_function_expr(FunctionFunctionId::Nil(NilFunctionFunctionId(0))),
                &[],
                &mut Frame::default(),
            )
            .map(|value| value.type_().return_().clone()),
            Ok(ValueType::Nil),
        );
        assert_eq!(
            run_tuple_function_function_call(
                &plan,
                &function_function_expr(FunctionFunctionId::Tuple(TupleFunctionFunctionId(0))),
                &[],
                &mut Frame::default(),
            )
            .map(|value| value.type_().return_().clone()),
            Ok(ValueType::Tuple(vec![ValueType::Int])),
        );
        assert_eq!(
            run_list_function_function_call(
                &plan,
                &function_function_expr(FunctionFunctionId::List(ListFunctionFunctionId(0))),
                &[],
                &mut Frame::default(),
            )
            .map(|value| value.type_().return_().clone()),
            Ok(ValueType::List(Box::new(ValueType::Int))),
        );
        assert_eq!(
            run_function_function_function_call(
                &plan,
                &function_function_expr(FunctionFunctionId::Function(FunctionFunctionFunctionId(
                    0
                ))),
                &[],
                &mut Frame::default(),
            )
            .map(|value| value.type_().return_().clone()),
            Ok(ValueType::Function(Box::new(FunctionType::new(
                Vec::new(),
                ValueType::Int,
            )))),
        );
    }

    #[test]
    fn primitive_function_call_returns_values() {
        let plan = primitive_function_plan();

        assert_eq!(
            run_int_call(&plan, IntFunctionId(0), &[], &mut Frame::default()),
            Ok(1.into()),
        );
        assert_eq!(
            run_string_call(&plan, StringFunctionId(0), &[], &mut Frame::default()),
            Ok("geam".into()),
        );
        assert_eq!(
            run_float_call(&plan, FloatFunctionId(0), &[], &mut Frame::default()),
            Ok(1.5),
        );
        assert_eq!(
            run_bool_call(&plan, BoolFunctionId(0), &[], &mut Frame::default()),
            Ok(true),
        );
        assert_eq!(
            run_nil_call(&plan, NilFunctionId(0), &[], &mut Frame::default()),
            Ok(()),
        );
        assert_eq!(
            run_tuple_call(&plan, TupleFunctionId(0), &[], &mut Frame::default()),
            Ok(vec![Value::Int(1.into())]),
        );
        assert_eq!(
            run_list_call(&plan, ListFunctionId(0), &[], &mut Frame::default()),
            Ok(ListValue::new(ValueType::Int, vec![Value::Int(1.into())])),
        );
    }

    #[test]
    fn direct_function_returning_function_call_returns_function_values() {
        let plan = plan_with_function_function_steps(Vec::new());

        assert_eq!(
            run_int_function_returning_function_call(
                &plan,
                IntFunctionFunctionId(0),
                &[],
                &mut Frame::default(),
            )
            .map(|value| value.type_().return_().clone()),
            Ok(ValueType::Int),
        );
        assert_eq!(
            run_string_function_returning_function_call(
                &plan,
                StringFunctionFunctionId(0),
                &[],
                &mut Frame::default(),
            )
            .map(|value| value.type_().return_().clone()),
            Ok(ValueType::String),
        );
        assert_eq!(
            run_float_function_returning_function_call(
                &plan,
                FloatFunctionFunctionId(0),
                &[],
                &mut Frame::default(),
            )
            .map(|value| value.type_().return_().clone()),
            Ok(ValueType::Float),
        );
        assert_eq!(
            run_function_returning_function_call(
                &plan,
                FunctionFunctionId::Float(FloatFunctionFunctionId(0)),
                &[],
                &mut Frame::default(),
            )
            .map(|value| value.type_().return_().clone()),
            Ok(ValueType::Float),
        );
        assert_eq!(
            run_bool_function_returning_function_call(
                &plan,
                BoolFunctionFunctionId(0),
                &[],
                &mut Frame::default(),
            )
            .map(|value| value.type_().return_().clone()),
            Ok(ValueType::Bool),
        );
        assert_eq!(
            run_nil_function_returning_function_call(
                &plan,
                NilFunctionFunctionId(0),
                &[],
                &mut Frame::default(),
            )
            .map(|value| value.type_().return_().clone()),
            Ok(ValueType::Nil),
        );
        assert_eq!(
            run_tuple_function_returning_function_call(
                &plan,
                TupleFunctionFunctionId(0),
                &[],
                &mut Frame::default(),
            )
            .map(|value| value.type_().return_().clone()),
            Ok(ValueType::Tuple(vec![ValueType::Int])),
        );
        assert_eq!(
            run_list_function_returning_function_call(
                &plan,
                ListFunctionFunctionId(0),
                &[],
                &mut Frame::default(),
            )
            .map(|value| value.type_().return_().clone()),
            Ok(ValueType::List(Box::new(ValueType::Int))),
        );
        assert_eq!(
            run_function_returning_function_call(
                &plan,
                FunctionFunctionId::List(ListFunctionFunctionId(0)),
                &[],
                &mut Frame::default(),
            )
            .map(|value| value.type_().return_().clone()),
            Ok(ValueType::List(Box::new(ValueType::Int))),
        );
        assert_eq!(
            run_function_returning_function_call(
                &plan,
                FunctionFunctionId::Tuple(TupleFunctionFunctionId(0)),
                &[],
                &mut Frame::default(),
            )
            .map(|value| value.type_().return_().clone()),
            Ok(ValueType::Tuple(vec![ValueType::Int])),
        );
        assert_eq!(
            run_function_function_returning_function_call(
                &plan,
                FunctionFunctionFunctionId(0),
                &[],
                &mut Frame::default(),
            )
            .map(|value| value.type_().return_().clone()),
            Ok(ValueType::Function(Box::new(FunctionType::new(
                Vec::new(),
                ValueType::Int,
            )))),
        );
    }

    #[test]
    fn primitive_function_calls_propagate_frame_binding_errors() {
        let plan = primitive_function_plan();

        assert_expected_function_got_int(run_int_call(
            &plan,
            IntFunctionId(0),
            &[failing_function_function_arg()],
            &mut Frame::default(),
        ));
        assert_expected_function_got_int(run_string_call(
            &plan,
            StringFunctionId(0),
            &[failing_function_function_arg()],
            &mut Frame::default(),
        ));
        assert_expected_function_got_int(run_float_call(
            &plan,
            FloatFunctionId(0),
            &[failing_function_function_arg()],
            &mut Frame::default(),
        ));
        assert_expected_function_got_int(run_bool_call(
            &plan,
            BoolFunctionId(0),
            &[failing_function_function_arg()],
            &mut Frame::default(),
        ));
        assert_expected_function_got_int(run_nil_call(
            &plan,
            NilFunctionId(0),
            &[failing_function_function_arg()],
            &mut Frame::default(),
        ));
        assert_expected_function_got_int(run_tuple_call(
            &plan,
            TupleFunctionId(0),
            &[failing_function_function_arg()],
            &mut Frame::default(),
        ));
        assert_expected_function_got_int(run_list_call(
            &plan,
            ListFunctionId(0),
            &[failing_function_function_arg()],
            &mut Frame::default(),
        ));
    }

    #[test]
    fn function_returning_function_calls_propagate_frame_binding_errors() {
        let plan = plan_with_function_function_steps(Vec::new());

        assert_expected_function_got_int(run_int_function_returning_function_call(
            &plan,
            IntFunctionFunctionId(0),
            &[failing_function_function_arg()],
            &mut Frame::default(),
        ));
        assert_expected_function_got_int(run_string_function_returning_function_call(
            &plan,
            StringFunctionFunctionId(0),
            &[failing_function_function_arg()],
            &mut Frame::default(),
        ));
        assert_expected_function_got_int(run_float_function_returning_function_call(
            &plan,
            FloatFunctionFunctionId(0),
            &[failing_function_function_arg()],
            &mut Frame::default(),
        ));
        assert_expected_function_got_int(run_bool_function_returning_function_call(
            &plan,
            BoolFunctionFunctionId(0),
            &[failing_function_function_arg()],
            &mut Frame::default(),
        ));
        assert_expected_function_got_int(run_nil_function_returning_function_call(
            &plan,
            NilFunctionFunctionId(0),
            &[failing_function_function_arg()],
            &mut Frame::default(),
        ));
        assert_expected_function_got_int(run_tuple_function_returning_function_call(
            &plan,
            TupleFunctionFunctionId(0),
            &[failing_function_function_arg()],
            &mut Frame::default(),
        ));
        assert_expected_function_got_int(run_list_function_returning_function_call(
            &plan,
            ListFunctionFunctionId(0),
            &[failing_function_function_arg()],
            &mut Frame::default(),
        ));
        assert_expected_function_got_int(run_function_function_returning_function_call(
            &plan,
            FunctionFunctionFunctionId(0),
            &[failing_function_function_arg()],
            &mut Frame::default(),
        ));
    }

    #[test]
    fn function_function_call_propagates_callee_evaluation_error() {
        let plan = plan();
        let expression = failing_function_function_expr();

        assert_expected_function_got_int(run_int_function_function_call(
            &plan,
            &expression,
            &[],
            &mut Frame::default(),
        ));
        assert_expected_function_got_int(run_string_function_function_call(
            &plan,
            &expression,
            &[],
            &mut Frame::default(),
        ));
        assert_expected_function_got_int(run_bool_function_function_call(
            &plan,
            &expression,
            &[],
            &mut Frame::default(),
        ));
        assert_expected_function_got_int(run_nil_function_function_call(
            &plan,
            &expression,
            &[],
            &mut Frame::default(),
        ));
        assert_expected_function_got_int(run_tuple_function_function_call(
            &plan,
            &expression,
            &[],
            &mut Frame::default(),
        ));
        assert_expected_function_got_int(run_list_function_function_call(
            &plan,
            &expression,
            &[],
            &mut Frame::default(),
        ));
        assert_expected_function_got_int(run_function_function_function_call(
            &plan,
            &expression,
            &[],
            &mut Frame::default(),
        ));
    }

    #[test]
    fn run_main_propagates_function_body_error_by_return_family() {
        let steps = vec![failing_step()];

        assert_expected_function_got_int(run_main(&plan_with_main(
            steps.clone(),
            ReturnExpr::int(IntFunctionId(0), IntExpr::value(1.into())),
        )));
        assert_expected_function_got_int(run_main(&plan_with_main(
            steps.clone(),
            ReturnExpr::string(StringFunctionId(0), StringExpr::value("geam".into())),
        )));
        assert_expected_function_got_int(run_main(&plan_with_main(
            steps.clone(),
            ReturnExpr::bool(BoolFunctionId(0), BoolExpr::value(true)),
        )));
        assert_expected_function_got_int(run_main(&plan_with_main(
            steps.clone(),
            ReturnExpr::nil(NilFunctionId(0), NilExpr::value()),
        )));
        assert_expected_function_got_int(run_main(&plan_with_main(
            steps.clone(),
            ReturnExpr::tuple(
                TupleFunctionId(0),
                TupleExpr::value(
                    vec![Expr::int(IntExpr::value(1.into()))],
                    vec![ValueType::Int],
                ),
            ),
        )));
        assert_expected_function_got_int(run_main(&plan_with_main(
            steps.clone(),
            ReturnExpr::list_body(
                ListFunctionId(0),
                ValueType::Int,
                ReturnBody::expr(ListExpr::value(
                    vec![Expr::int(IntExpr::value(1.into()))],
                    ValueType::Int,
                )),
            ),
        )));
        assert_expected_function_got_int(run_main(&plan_with_main(
            steps,
            function_return_expr(function_function_expr_value()),
        )));
    }

    #[test]
    fn primitive_function_value_call_propagates_callee_evaluation_error() {
        let plan = primitive_function_plan();

        assert_expected_function_got_int(run_int_function_call(
            &plan,
            &IntFunctionExpr::function_call(
                failing_function_function_expr(),
                Vec::new(),
                FunctionType::new(Vec::new(), ValueType::Int),
            ),
            &[],
            &mut Frame::default(),
        ));
        assert_expected_function_got_int(run_string_function_call(
            &plan,
            &StringFunctionExpr::function_call(
                failing_function_function_expr(),
                Vec::new(),
                FunctionType::new(Vec::new(), ValueType::String),
            ),
            &[],
            &mut Frame::default(),
        ));
        assert_expected_function_got_int(run_float_function_call(
            &plan,
            &FloatFunctionExpr::function_call(
                failing_function_function_expr(),
                Vec::new(),
                FunctionType::new(Vec::new(), ValueType::Float),
            ),
            &[],
            &mut Frame::default(),
        ));
        assert_expected_function_got_int(run_bool_function_call(
            &plan,
            &BoolFunctionExpr::function_call(
                failing_function_function_expr(),
                Vec::new(),
                FunctionType::new(Vec::new(), ValueType::Bool),
            ),
            &[],
            &mut Frame::default(),
        ));
        assert_expected_function_got_int(run_nil_function_call(
            &plan,
            &NilFunctionExpr::function_call(
                failing_function_function_expr(),
                Vec::new(),
                FunctionType::new(Vec::new(), ValueType::Nil),
            ),
            &[],
            &mut Frame::default(),
        ));
        assert_expected_function_got_int(run_tuple_function_call(
            &plan,
            &TupleFunctionExpr::function_call(
                failing_function_function_expr(),
                Vec::new(),
                FunctionType::new(Vec::new(), ValueType::Tuple(vec![ValueType::Int])),
            ),
            &[],
            &mut Frame::default(),
        ));
        assert_expected_function_got_int(run_list_function_call(
            &plan,
            &ListFunctionExpr::function_call(
                failing_function_function_expr(),
                Vec::new(),
                FunctionType::new(Vec::new(), ValueType::List(Box::new(ValueType::Int))),
            ),
            &[],
            &mut Frame::default(),
        ));
    }

    fn assert_expected_function_got_int<T>(actual: Result<T, ExecutionError>) {
        assert_function_return_family_mismatch(
            actual,
            FunctionReturnFamily::Function,
            FunctionReturnFamily::Int,
        );
    }

    fn assert_function_return_family_mismatch<T>(
        actual: Result<T, ExecutionError>,
        expected: FunctionReturnFamily,
        actual_family: FunctionReturnFamily,
    ) {
        let error = actual.err().expect("call should fail");

        assert_eq!(
            error,
            ExecutionError::function_return_family_mismatch(expected, actual_family),
        );
    }

    fn function_function_expr(runtime_id: FunctionFunctionId) -> FunctionFunctionExpr {
        FunctionFunctionExpr::value(FunctionFunctionValue::new(
            runtime_id,
            Vec::new(),
            FunctionType::new(Vec::new(), ValueType::Int),
        ))
    }

    fn failing_function_function_expr() -> FunctionFunctionExpr {
        FunctionFunctionExpr::function_call(
            function_function_expr(FunctionFunctionId::Int(IntFunctionFunctionId(0))),
            Vec::new(),
            FunctionType::new(
                Vec::new(),
                ValueType::Function(Box::new(FunctionType::new(Vec::new(), ValueType::Int))),
            ),
        )
    }

    fn failing_function_function_arg() -> CallArg {
        CallArg::function_function(FunctionFunctionLocalId(0), failing_function_function_expr())
    }

    fn failing_step() -> Step {
        Step::evaluate(Expr::function(FunctionExpr::function(
            failing_function_function_expr(),
        )))
    }

    fn plan_with_function_function_steps(steps: Vec<Step>) -> ExecutionPlan {
        ExecutionPlan::new(
            "main".into(),
            FunctionPlan::new(
                FunctionId::new(0),
                "main".into(),
                Vec::new(),
                Vec::new(),
                ReturnExpr::int(IntFunctionId(0), IntExpr::value(1.into())),
            ),
            vec![
                function_plan(1, "int_function", steps.clone(), int_function_expr()),
                function_plan(2, "string_function", steps.clone(), string_function_expr()),
                function_plan(3, "float_function", steps.clone(), float_function_expr()),
                function_plan(4, "bool_function", steps.clone(), bool_function_expr()),
                function_plan(5, "nil_function", steps.clone(), nil_function_expr()),
                function_plan(6, "tuple_function", steps.clone(), tuple_function_expr()),
                function_plan(7, "list_function", steps.clone(), list_function_expr()),
                function_plan(
                    8,
                    "function_function",
                    steps,
                    function_function_expr_value(),
                ),
            ],
        )
    }

    fn plan_with_main(steps: Vec<Step>, return_: ReturnExpr) -> ExecutionPlan {
        ExecutionPlan::new(
            "main".into(),
            FunctionPlan::new(
                FunctionId::new(0),
                "main".into(),
                Vec::new(),
                steps,
                return_,
            ),
            vec![function_plan(
                1,
                "int_function",
                Vec::new(),
                int_function_expr(),
            )],
        )
    }

    fn primitive_function_plan() -> ExecutionPlan {
        primitive_function_plan_with_steps(Vec::new())
    }

    fn primitive_function_plan_with_steps(steps: Vec<Step>) -> ExecutionPlan {
        ExecutionPlan::new(
            "main".into(),
            FunctionPlan::new(
                FunctionId::new(0),
                "main".into(),
                Vec::new(),
                steps.clone(),
                ReturnExpr::int(IntFunctionId(0), IntExpr::value(1.into())),
            ),
            vec![
                FunctionPlan::new(
                    FunctionId::new(1),
                    "string".into(),
                    Vec::new(),
                    steps.clone(),
                    ReturnExpr::string(StringFunctionId(0), StringExpr::value("geam".into())),
                ),
                FunctionPlan::new(
                    FunctionId::new(2),
                    "float".into(),
                    Vec::new(),
                    steps.clone(),
                    ReturnExpr::float(FloatFunctionId(0), FloatExpr::value(1.5)),
                ),
                FunctionPlan::new(
                    FunctionId::new(3),
                    "bool".into(),
                    Vec::new(),
                    steps.clone(),
                    ReturnExpr::bool(BoolFunctionId(0), BoolExpr::value(true)),
                ),
                FunctionPlan::new(
                    FunctionId::new(4),
                    "nil".into(),
                    Vec::new(),
                    steps.clone(),
                    ReturnExpr::nil(NilFunctionId(0), NilExpr::value()),
                ),
                FunctionPlan::new(
                    FunctionId::new(5),
                    "tuple".into(),
                    Vec::new(),
                    steps.clone(),
                    ReturnExpr::tuple(
                        TupleFunctionId(0),
                        TupleExpr::value(
                            vec![Expr::int(IntExpr::value(1.into()))],
                            vec![ValueType::Int],
                        ),
                    ),
                ),
                FunctionPlan::new(
                    FunctionId::new(6),
                    "list".into(),
                    Vec::new(),
                    steps,
                    ReturnExpr::list_body(
                        ListFunctionId(0),
                        ValueType::Int,
                        ReturnBody::expr(ListExpr::value(
                            vec![Expr::int(IntExpr::value(1.into()))],
                            ValueType::Int,
                        )),
                    ),
                ),
            ],
        )
    }

    fn function_plan(
        id: usize,
        name: &str,
        steps: Vec<Step>,
        return_: FunctionExpr,
    ) -> FunctionPlan {
        FunctionPlan::new(
            FunctionId::new(id),
            name.into(),
            Vec::new(),
            steps,
            function_return_expr(return_),
        )
    }

    fn function_return_expr(return_: FunctionExpr) -> ReturnExpr {
        match return_.into_kind() {
            FunctionExprKind::Int(return_) => {
                ReturnExpr::int_function(IntFunctionFunctionId(0), return_)
            }
            FunctionExprKind::String(return_) => {
                ReturnExpr::string_function(StringFunctionFunctionId(0), return_)
            }
            FunctionExprKind::Float(return_) => {
                ReturnExpr::float_function(FloatFunctionFunctionId(0), return_)
            }
            FunctionExprKind::Bool(return_) => {
                ReturnExpr::bool_function(BoolFunctionFunctionId(0), return_)
            }
            FunctionExprKind::Nil(return_) => {
                ReturnExpr::nil_function(NilFunctionFunctionId(0), return_)
            }
            FunctionExprKind::Tuple(return_) => {
                ReturnExpr::tuple_function(TupleFunctionFunctionId(0), return_)
            }
            FunctionExprKind::List(return_) => {
                ReturnExpr::list_function(crate::plan::ListFunctionFunctionId(0), return_)
            }
            FunctionExprKind::Function(return_) => {
                ReturnExpr::function_function(FunctionFunctionFunctionId(0), return_)
            }
        }
    }

    fn int_function_expr() -> FunctionExpr {
        FunctionExpr::int(IntFunctionExpr::value(IntFunctionValue::new(
            IntFunctionId(0),
            Vec::new(),
        )))
    }

    fn string_function_expr() -> FunctionExpr {
        FunctionExpr::string(StringFunctionExpr::value(StringFunctionValue::new(
            StringFunctionId(0),
            Vec::new(),
        )))
    }

    fn float_function_expr() -> FunctionExpr {
        FunctionExpr::float(FloatFunctionExpr::value(FloatFunctionValue::new(
            FloatFunctionId(0),
            Vec::new(),
        )))
    }

    fn bool_function_expr() -> FunctionExpr {
        FunctionExpr::bool(BoolFunctionExpr::value(BoolFunctionValue::new(
            BoolFunctionId(0),
            Vec::new(),
        )))
    }

    fn nil_function_expr() -> FunctionExpr {
        FunctionExpr::nil(NilFunctionExpr::value(NilFunctionValue::new(
            NilFunctionId(0),
            Vec::new(),
        )))
    }

    fn tuple_function_expr() -> FunctionExpr {
        FunctionExpr::tuple(TupleFunctionExpr::value(TupleFunctionValue::new(
            TupleFunctionId(0),
            Vec::new(),
            vec![ValueType::Int],
        )))
    }

    fn list_function_expr() -> FunctionExpr {
        FunctionExpr::list(ListFunctionExpr::value(ListFunctionValue::new(
            ListFunctionId(0),
            Vec::new(),
            ValueType::Int,
        )))
    }

    fn function_function_expr_value() -> FunctionExpr {
        FunctionExpr::function(function_function_expr(FunctionFunctionId::Int(
            IntFunctionFunctionId(0),
        )))
    }

    fn plan() -> ExecutionPlan {
        ExecutionPlan::new(
            "main".into(),
            FunctionPlan::new(
                FunctionId::new(0),
                "main".into(),
                Vec::new(),
                Vec::new(),
                ReturnExpr::int(IntFunctionId(0), IntExpr::value(1.into())),
            ),
            Vec::new(),
        )
    }
}
