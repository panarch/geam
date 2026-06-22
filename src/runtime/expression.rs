mod bool;
mod int;
mod nil;
mod string;

use crate::plan::{ExecutionPlan, Expr, ExprKind, Value};
use crate::runtime::frame::Frame;

pub(super) use self::{
    bool::eval_bool_expr, int::eval_int_expr, nil::eval_nil_expr, string::eval_string_expr,
};

pub(super) fn eval_expr(plan: &ExecutionPlan, frame: &mut Frame, expression: &Expr) -> Value {
    match expression.kind() {
        ExprKind::Int(expression) => Value::Int(eval_int_expr(plan, frame, expression)),
        ExprKind::String(expression) => Value::String(eval_string_expr(plan, frame, expression)),
        ExprKind::Bool(expression) => Value::Bool(eval_bool_expr(plan, frame, expression)),
        ExprKind::Nil(expression) => {
            eval_nil_expr(plan, frame, expression);
            Value::Nil
        }
    }
}
