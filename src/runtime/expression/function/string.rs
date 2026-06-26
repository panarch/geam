use crate::plan::{ExecutionPlan, StringFunctionExpr, StringFunctionExprKind, StringFunctionValue};
use crate::runtime::expression::{eval_bool_expr, eval_int_expr};
use crate::runtime::frame::Frame;
use crate::runtime::function;

pub(in crate::runtime) fn eval_string_function_expr(
    plan: &ExecutionPlan,
    frame: &mut Frame,
    expression: &StringFunctionExpr,
) -> StringFunctionValue {
    match expression.kind() {
        StringFunctionExprKind::Value(value) => value.clone(),
        StringFunctionExprKind::LocalGet { local, .. } => frame.get_string_function(*local),
        StringFunctionExprKind::BoolCase {
            subject,
            true_,
            false_,
        } => {
            if eval_bool_expr(plan, frame, subject) {
                eval_string_function_expr(plan, frame, true_)
            } else {
                eval_string_function_expr(plan, frame, false_)
            }
        }
        StringFunctionExprKind::IntCase {
            subject,
            clauses,
            fallback,
        } => {
            let subject = eval_int_expr(plan, frame, subject);
            for (pattern, branch) in clauses {
                if pattern == &subject {
                    return eval_string_function_expr(plan, frame, branch);
                }
            }
            eval_string_function_expr(plan, frame, fallback)
        }
        StringFunctionExprKind::Block { steps, return_ } => {
            function::execute_steps(plan, steps, frame);
            eval_string_function_expr(plan, frame, return_)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::eval_string_function_expr;
    use crate::plan::{
        BoolExpr, ExecutionPlan, Expr, FunctionId, FunctionPlan, IntExpr, ParamLocal, ReturnExpr,
        Step, StringFunctionExpr, StringFunctionId, StringFunctionValue, StringLocalId,
    };
    use crate::runtime::frame::Frame;

    #[test]
    fn eval_string_function_bool_case_branches() {
        let plan = plan();
        let mut frame = Frame::default();

        assert_eq!(
            eval_string_function_expr(
                &plan,
                &mut frame,
                &StringFunctionExpr::bool_case(
                    BoolExpr::value(true),
                    function_value(),
                    other_function_value(),
                ),
            )
            .runtime_id(),
            StringFunctionId(0),
        );
        assert_eq!(
            eval_string_function_expr(
                &plan,
                &mut frame,
                &StringFunctionExpr::bool_case(
                    BoolExpr::value(false),
                    other_function_value(),
                    function_value(),
                ),
            )
            .runtime_id(),
            StringFunctionId(0),
        );
    }

    #[test]
    fn eval_string_function_int_case_branches() {
        let plan = plan();
        let mut frame = Frame::default();

        assert_eq!(
            eval_string_function_expr(
                &plan,
                &mut frame,
                &StringFunctionExpr::int_case(
                    IntExpr::value(1.into()),
                    vec![(1.into(), function_value())],
                    other_function_value(),
                ),
            )
            .runtime_id(),
            StringFunctionId(0),
        );
        assert_eq!(
            eval_string_function_expr(
                &plan,
                &mut frame,
                &StringFunctionExpr::int_case(
                    IntExpr::value(2.into()),
                    vec![(1.into(), other_function_value())],
                    function_value(),
                ),
            )
            .runtime_id(),
            StringFunctionId(0),
        );
    }

    #[test]
    fn eval_string_function_block() {
        let plan = plan();
        let mut frame = Frame::default();

        assert_eq!(
            eval_string_function_expr(
                &plan,
                &mut frame,
                &StringFunctionExpr::block(
                    vec![Step::evaluate(Expr::int(IntExpr::value(1.into())))],
                    function_value(),
                ),
            )
            .runtime_id(),
            StringFunctionId(0),
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

    fn function_value() -> StringFunctionExpr {
        StringFunctionExpr::value(StringFunctionValue::new(
            StringFunctionId(0),
            vec![ParamLocal::string(StringLocalId(0))],
        ))
    }

    fn other_function_value() -> StringFunctionExpr {
        StringFunctionExpr::value(StringFunctionValue::new(
            StringFunctionId(1),
            vec![ParamLocal::string(StringLocalId(0))],
        ))
    }
}
