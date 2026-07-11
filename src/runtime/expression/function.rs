mod bool;
mod float;
mod int;
mod list;
mod nil;
mod returning_function;
mod string;
mod tuple;

use crate::plan::execution::ExecutionPlan;
use crate::plan::execution::{FunctionExpr, FunctionExprKind};
use crate::runtime::frame::Frame;
use crate::runtime::state::RuntimeState;
use crate::runtime::{EvaluatedFunctionValue, ExecutionError};

pub(in crate::runtime) use self::{
    bool::eval_bool_function_expr, float::eval_float_function_expr, int::eval_int_function_expr,
    list::eval_list_function_expr, nil::eval_nil_function_expr,
    returning_function::eval_function_function_expr, string::eval_string_function_expr,
    tuple::eval_tuple_function_expr,
};

pub(in crate::runtime) fn eval_function_expr(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
    frame: &mut Frame,
    expression: &FunctionExpr,
) -> Result<EvaluatedFunctionValue, ExecutionError> {
    match expression.kind() {
        FunctionExprKind::Int(expression) => {
            Ok(eval_int_function_expr(plan, state, frame, expression)?.into())
        }
        FunctionExprKind::String(expression) => {
            Ok(eval_string_function_expr(plan, state, frame, expression)?.into())
        }
        FunctionExprKind::Float(expression) => {
            Ok(eval_float_function_expr(plan, state, frame, expression)?.into())
        }
        FunctionExprKind::Bool(expression) => {
            Ok(eval_bool_function_expr(plan, state, frame, expression)?.into())
        }
        FunctionExprKind::Nil(expression) => {
            Ok(eval_nil_function_expr(plan, state, frame, expression)?.into())
        }
        FunctionExprKind::Tuple(expression) => {
            Ok(eval_tuple_function_expr(plan, state, frame, expression)?.into())
        }
        FunctionExprKind::List(expression) => {
            Ok(eval_list_function_expr(plan, state, frame, expression)?.into())
        }
        FunctionExprKind::Function(expression) => {
            Ok(eval_function_function_expr(plan, state, frame, expression)?.into())
        }
    }
}

#[cfg(test)]
fn expect_function_list(expression: crate::plan::ListExpr) -> crate::plan::FunctionListExpr {
    match expression {
        crate::plan::ListExpr::Function(expression) => expression,
        _ => panic!("expected a function-valued list expression"),
    }
}

#[cfg(test)]
mod tests {
    use crate::runtime::run_main;

    #[test]
    fn compound_function_tuple_projections_propagate_tuple_errors() {
        let sources = [
            "fn provider() -> #(fn() -> List(Int)) { panic } pub fn main() { provider().0 }",
            "fn provider() -> #(fn() -> #(Int)) { panic } pub fn main() { provider().0 }",
        ];

        for source in sources {
            let plan = crate::runtime::plan_src(source);
            let error = run_main(&plan).expect_err("tuple provider panic should propagate");

            assert_eq!(error.to_string(), "panic: `panic` expression evaluated.");
        }
    }

    #[test]
    #[should_panic(expected = "expected a function-valued list expression")]
    fn function_list_shape_guard_rejects_int_lists() {
        let expression = crate::plan::ListExpr::panic(
            crate::plan::PanicExpr::panic_at(None, crate::plan::PanicSite::unknown()),
            crate::plan::ValueType::Int,
        );

        let _ = super::expect_function_list(expression);
    }
}
