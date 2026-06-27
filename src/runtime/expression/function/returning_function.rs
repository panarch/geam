use crate::plan::{
    ExecutionPlan, FunctionFunctionExpr, FunctionFunctionExprKind, FunctionFunctionValue,
};
use crate::runtime::expression::{eval_bool_expr, eval_int_expr};
use crate::runtime::frame::Frame;
use crate::runtime::function;

pub(in crate::runtime) fn eval_function_function_expr(
    plan: &ExecutionPlan,
    frame: &mut Frame,
    expression: &FunctionFunctionExpr,
) -> FunctionFunctionValue {
    match expression.kind() {
        FunctionFunctionExprKind::Value(value) => value.clone(),
        FunctionFunctionExprKind::LocalGet { local, .. } => frame.get_function_function(*local),
        FunctionFunctionExprKind::Call { function, args, .. } => {
            function::run_function_function_returning_function_call(plan, *function, args, frame)
        }
        FunctionFunctionExprKind::FunctionCall {
            function: callee,
            args,
            ..
        } => function::run_function_function_function_call(plan, callee, args, frame),
        FunctionFunctionExprKind::BoolCase {
            subject,
            true_,
            false_,
        } => {
            if eval_bool_expr(plan, frame, subject) {
                eval_function_function_expr(plan, frame, true_)
            } else {
                eval_function_function_expr(plan, frame, false_)
            }
        }
        FunctionFunctionExprKind::IntCase {
            subject,
            clauses,
            fallback,
        } => {
            let subject = eval_int_expr(plan, frame, subject);
            for (pattern, branch) in clauses {
                if pattern == &subject {
                    return eval_function_function_expr(plan, frame, branch);
                }
            }
            eval_function_function_expr(plan, frame, fallback)
        }
        FunctionFunctionExprKind::Block { steps, return_ } => {
            function::execute_steps(plan, steps, frame);
            eval_function_function_expr(plan, frame, return_)
        }
    }
}
