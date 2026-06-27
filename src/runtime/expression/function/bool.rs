use crate::plan::{BoolFunctionExpr, BoolFunctionExprKind, BoolFunctionValue, ExecutionPlan};
use crate::runtime::expression::{eval_bool_expr, eval_int_expr};
use crate::runtime::frame::Frame;
use crate::runtime::function;

pub(in crate::runtime) fn eval_bool_function_expr(
    plan: &ExecutionPlan,
    frame: &mut Frame,
    expression: &BoolFunctionExpr,
) -> BoolFunctionValue {
    match expression.kind() {
        BoolFunctionExprKind::Value(value) => value.clone(),
        BoolFunctionExprKind::LocalGet { local, .. } => frame.get_bool_function(*local),
        BoolFunctionExprKind::Call { function, args, .. } => {
            function::run_bool_function_returning_function_call(plan, *function, args, frame)
        }
        BoolFunctionExprKind::FunctionCall {
            function: callee,
            args,
            ..
        } => function::run_bool_function_function_call(plan, callee, args, frame),
        BoolFunctionExprKind::BoolCase {
            subject,
            true_,
            false_,
        } => {
            if eval_bool_expr(plan, frame, subject) {
                eval_bool_function_expr(plan, frame, true_)
            } else {
                eval_bool_function_expr(plan, frame, false_)
            }
        }
        BoolFunctionExprKind::IntCase {
            subject,
            clauses,
            fallback,
        } => {
            let subject = eval_int_expr(plan, frame, subject);
            for (pattern, branch) in clauses {
                if pattern == &subject {
                    return eval_bool_function_expr(plan, frame, branch);
                }
            }
            eval_bool_function_expr(plan, frame, fallback)
        }
        BoolFunctionExprKind::Block { steps, return_ } => {
            function::execute_steps(plan, steps, frame);
            eval_bool_function_expr(plan, frame, return_)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::eval_bool_function_expr;
    use crate::plan::{
        BoolExpr, BoolFunctionExpr, BoolFunctionId, BoolFunctionValue, BoolLocalId, ExecutionPlan,
        Expr, FunctionId, FunctionPlan, IntExpr, ParamLocal, ReturnExpr, Step,
    };
    use crate::runtime::frame::Frame;

    #[test]
    fn eval_bool_function_bool_case_branches() {
        let plan = plan();
        let mut frame = Frame::default();

        assert_eq!(
            eval_bool_function_expr(
                &plan,
                &mut frame,
                &BoolFunctionExpr::bool_case(
                    BoolExpr::value(true),
                    function_value(),
                    other_function_value(),
                ),
            )
            .runtime_id(),
            BoolFunctionId(0),
        );
        assert_eq!(
            eval_bool_function_expr(
                &plan,
                &mut frame,
                &BoolFunctionExpr::bool_case(
                    BoolExpr::value(false),
                    other_function_value(),
                    function_value(),
                ),
            )
            .runtime_id(),
            BoolFunctionId(0),
        );
    }

    #[test]
    fn eval_bool_function_int_case_branches() {
        let plan = plan();
        let mut frame = Frame::default();

        assert_eq!(
            eval_bool_function_expr(
                &plan,
                &mut frame,
                &BoolFunctionExpr::int_case(
                    IntExpr::value(1.into()),
                    vec![(1.into(), function_value())],
                    other_function_value(),
                ),
            )
            .runtime_id(),
            BoolFunctionId(0),
        );
        assert_eq!(
            eval_bool_function_expr(
                &plan,
                &mut frame,
                &BoolFunctionExpr::int_case(
                    IntExpr::value(2.into()),
                    vec![(1.into(), other_function_value())],
                    function_value(),
                ),
            )
            .runtime_id(),
            BoolFunctionId(0),
        );
    }

    #[test]
    fn eval_bool_function_block() {
        let plan = plan();
        let mut frame = Frame::default();

        assert_eq!(
            eval_bool_function_expr(
                &plan,
                &mut frame,
                &BoolFunctionExpr::block(
                    vec![Step::evaluate(Expr::int(IntExpr::value(1.into())))],
                    function_value(),
                ),
            )
            .runtime_id(),
            BoolFunctionId(0),
        );
    }

    fn plan() -> ExecutionPlan {
        ExecutionPlan::new(
            "main".into(),
            FunctionPlan::new(
                FunctionId::new(0),
                "main".into(),
                Vec::new(),
                Vec::new(),
                ReturnExpr::int(IntExpr::value(1.into())),
            ),
            Vec::new(),
        )
    }

    fn function_value() -> BoolFunctionExpr {
        BoolFunctionExpr::value(BoolFunctionValue::new(
            BoolFunctionId(0),
            vec![ParamLocal::bool(BoolLocalId(0))],
        ))
    }

    fn other_function_value() -> BoolFunctionExpr {
        BoolFunctionExpr::value(BoolFunctionValue::new(
            BoolFunctionId(1),
            vec![ParamLocal::bool(BoolLocalId(0))],
        ))
    }
}
