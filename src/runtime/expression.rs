mod bool;
mod float;
mod function;
mod int;
mod nil;
mod string;

use crate::plan::{ExecutionPlan, Expr, ExprKind, Value};
use crate::runtime::ExecutionError;
use crate::runtime::frame::Frame;

pub(super) use self::{
    bool::eval_bool_expr,
    float::eval_float_expr,
    function::{
        eval_bool_function_expr, eval_float_function_expr, eval_function_expr,
        eval_function_function_expr, eval_int_function_expr, eval_nil_function_expr,
        eval_string_function_expr,
    },
    int::eval_int_expr,
    nil::eval_nil_expr,
    string::eval_string_expr,
};

pub(super) fn eval_expr(
    plan: &ExecutionPlan,
    frame: &mut Frame,
    expression: &Expr,
) -> Result<Value, ExecutionError> {
    match expression.kind() {
        ExprKind::Int(expression) => Ok(Value::Int(eval_int_expr(plan, frame, expression)?)),
        ExprKind::String(expression) => {
            Ok(Value::String(eval_string_expr(plan, frame, expression)?))
        }
        ExprKind::Float(expression) => Ok(Value::Float(eval_float_expr(plan, frame, expression)?)),
        ExprKind::Bool(expression) => Ok(Value::Bool(eval_bool_expr(plan, frame, expression)?)),
        ExprKind::Nil(expression) => {
            eval_nil_expr(plan, frame, expression)?;
            Ok(Value::Nil)
        }
        ExprKind::Function(expression) => {
            let value = eval_function_expr(plan, frame, expression)?;
            Ok(Value::Function(value))
        }
    }
}
