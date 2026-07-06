mod bool;
mod float;
mod function;
mod int;
mod list;
mod nil;
mod string;
mod tuple;

use crate::plan::{ExecutionPlan, Expr, ExprKind, PanicExpr, PanicExprKind, Value};
use crate::runtime::frame::Frame;
use crate::runtime::{ExecutionError, PanicKind};

pub(super) use self::{
    bool::eval_bool_expr,
    float::eval_float_expr,
    function::{
        eval_bool_function_expr, eval_float_function_expr, eval_function_expr,
        eval_function_function_expr, eval_int_function_expr, eval_list_function_expr,
        eval_nil_function_expr, eval_string_function_expr, eval_tuple_function_expr,
    },
    int::eval_int_expr,
    list::eval_list_expr,
    nil::eval_nil_expr,
    string::eval_string_expr,
    tuple::{eval_tuple_expr, project_tuple_expr},
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
        ExprKind::Tuple(expression) => Ok(Value::Tuple(eval_tuple_expr(plan, frame, expression)?)),
        ExprKind::List(expression) => Ok(Value::List(eval_list_expr(plan, frame, expression)?)),
        ExprKind::Function(expression) => {
            let value = eval_function_expr(plan, frame, expression)?;
            Ok(Value::Function(value))
        }
    }
}

pub(super) fn eval_panic_expr<T>(
    plan: &ExecutionPlan,
    frame: &mut Frame,
    expression: &PanicExpr,
) -> Result<T, ExecutionError> {
    let kind = match expression.kind() {
        PanicExprKind::Panic { message } => PanicKind::Panic {
            message: eval_panic_message(plan, frame, message.as_deref())?,
        },
        PanicExprKind::Todo { message } => PanicKind::Todo {
            message: eval_panic_message(plan, frame, message.as_deref())?,
        },
        PanicExprKind::EmptyFunction => PanicKind::EmptyFunction,
        PanicExprKind::EmptyBlock => PanicKind::EmptyBlock,
        PanicExprKind::IncompleteUse => PanicKind::IncompleteUse,
    };

    Err(ExecutionError::panic(kind))
}

fn eval_panic_message(
    plan: &ExecutionPlan,
    frame: &mut Frame,
    message: Option<&crate::plan::StringExpr>,
) -> Result<Option<ecow::EcoString>, ExecutionError> {
    match message {
        Some(message) => Ok(Some(eval_string_expr(plan, frame, message)?)),
        None => Ok(None),
    }
}

#[cfg(test)]
mod tests {
    use super::eval_panic_expr;
    use crate::plan::{
        ExecutionPlan, FunctionId, FunctionPlan, IntExpr, IntFunctionId, PanicExpr, ReturnExpr,
        StringExpr,
    };
    use crate::runtime::frame::Frame;
    use crate::runtime::{ExecutionError, PanicKind};

    #[test]
    fn eval_panic_expr_returns_exact_panic_error() {
        assert_eq!(
            eval_panic(PanicExpr::panic(None)),
            Err(ExecutionError::panic(PanicKind::Panic { message: None })),
        );
        assert_eq!(
            eval_panic(PanicExpr::todo(Some(StringExpr::value("later".into())))),
            Err(ExecutionError::panic(PanicKind::Todo {
                message: Some("later".into()),
            })),
        );
    }

    #[test]
    fn eval_generated_todo_kinds_return_distinct_panic_errors() {
        for (expression, expected) in [
            (PanicExpr::empty_function(), PanicKind::EmptyFunction),
            (PanicExpr::empty_block(), PanicKind::EmptyBlock),
            (PanicExpr::incomplete_use(), PanicKind::IncompleteUse),
        ] {
            assert_eq!(eval_panic(expression), Err(ExecutionError::panic(expected)),);
        }
    }

    #[test]
    fn eval_panic_expr_propagates_message_error_first() {
        let message = StringExpr::panic(PanicExpr::todo(None));

        assert_eq!(
            eval_panic(PanicExpr::panic(Some(message))),
            Err(ExecutionError::panic(PanicKind::Todo { message: None })),
        );
    }

    #[test]
    fn eval_todo_expr_propagates_message_error_first() {
        let message = StringExpr::panic(PanicExpr::panic(None));

        assert_eq!(
            eval_panic(PanicExpr::todo(Some(message))),
            Err(ExecutionError::panic(PanicKind::Panic { message: None })),
        );
    }

    fn eval_panic(expression: PanicExpr) -> Result<(), ExecutionError> {
        let plan = plan();
        let mut frame = Frame::default();

        eval_panic_expr(&plan, &mut frame, &expression)
    }

    fn plan() -> ExecutionPlan {
        ExecutionPlan::new(
            "main".into(),
            FunctionPlan::new(
                FunctionId::new(0),
                "main".into(),
                Vec::new(),
                Vec::new(),
                ReturnExpr::int(IntFunctionId(0), IntExpr::value(0.into())),
            ),
            Vec::new(),
        )
    }
}
