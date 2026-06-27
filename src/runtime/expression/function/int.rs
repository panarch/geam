use crate::plan::{ExecutionPlan, IntFunctionExpr, IntFunctionExprKind, IntFunctionValue};
use crate::runtime::expression::{eval_bool_expr, eval_int_expr};
use crate::runtime::frame::Frame;
use crate::runtime::function;

pub(in crate::runtime) fn eval_int_function_expr(
    plan: &ExecutionPlan,
    frame: &mut Frame,
    expression: &IntFunctionExpr,
) -> IntFunctionValue {
    match expression.kind() {
        IntFunctionExprKind::Value(value) => value.clone(),
        IntFunctionExprKind::LocalGet { local, .. } => frame.get_int_function(*local),
        IntFunctionExprKind::Call { function, args, .. } => {
            function::run_int_function_returning_function_call(plan, *function, args, frame)
        }
        IntFunctionExprKind::FunctionCall {
            function: callee,
            args,
            ..
        } => function::run_int_function_function_call(plan, callee, args, frame),
        IntFunctionExprKind::BoolCase {
            subject,
            true_,
            false_,
        } => {
            if eval_bool_expr(plan, frame, subject) {
                eval_int_function_expr(plan, frame, true_)
            } else {
                eval_int_function_expr(plan, frame, false_)
            }
        }
        IntFunctionExprKind::IntCase {
            subject,
            clauses,
            fallback,
        } => {
            let subject = eval_int_expr(plan, frame, subject);
            for (pattern, branch) in clauses {
                if pattern == &subject {
                    return eval_int_function_expr(plan, frame, branch);
                }
            }
            eval_int_function_expr(plan, frame, fallback)
        }
        IntFunctionExprKind::Block { steps, return_ } => {
            function::execute_steps(plan, steps, frame);
            eval_int_function_expr(plan, frame, return_)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::eval_int_function_expr;
    use crate::plan::{
        BoolExpr, ExecutionPlan, Expr, FunctionId, FunctionPlan, IntExpr, IntFunctionExpr,
        IntFunctionId, IntFunctionValue, IntLocalId, ParamLocal, ReturnExpr, Step,
    };
    use crate::runtime::frame::Frame;

    #[test]
    fn eval_int_function_bool_case_branches() {
        let plan = plan();
        let mut frame = Frame::default();
        let function = eval_int_function_expr(
            &plan,
            &mut frame,
            &IntFunctionExpr::bool_case(
                BoolExpr::value(true),
                function_value(),
                other_function_value(),
            ),
        );

        assert_eq!(function.runtime_id(), IntFunctionId(0));

        let function = eval_int_function_expr(
            &plan,
            &mut frame,
            &IntFunctionExpr::bool_case(
                BoolExpr::value(false),
                other_function_value(),
                function_value(),
            ),
        );

        assert_eq!(function.runtime_id(), IntFunctionId(0));
    }

    #[test]
    fn eval_int_function_int_case_branches() {
        let plan = plan();
        let mut frame = Frame::default();
        let function = eval_int_function_expr(
            &plan,
            &mut frame,
            &IntFunctionExpr::int_case(
                IntExpr::value(1.into()),
                vec![(1.into(), function_value())],
                other_function_value(),
            ),
        );

        assert_eq!(function.runtime_id(), IntFunctionId(0));

        let function = eval_int_function_expr(
            &plan,
            &mut frame,
            &IntFunctionExpr::int_case(
                IntExpr::value(2.into()),
                vec![(1.into(), other_function_value())],
                function_value(),
            ),
        );

        assert_eq!(function.runtime_id(), IntFunctionId(0));
    }

    #[test]
    fn eval_int_function_block() {
        let plan = plan();
        let mut frame = Frame::default();
        let function = eval_int_function_expr(
            &plan,
            &mut frame,
            &IntFunctionExpr::block(
                vec![Step::evaluate(Expr::int(IntExpr::value(1.into())))],
                function_value(),
            ),
        );

        assert_eq!(function.runtime_id(), IntFunctionId(0));
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

    fn function_value() -> IntFunctionExpr {
        IntFunctionExpr::value(IntFunctionValue::new(
            IntFunctionId(0),
            vec![ParamLocal::int(IntLocalId(0))],
        ))
    }

    fn other_function_value() -> IntFunctionExpr {
        IntFunctionExpr::value(IntFunctionValue::new(
            IntFunctionId(1),
            vec![ParamLocal::int(IntLocalId(0))],
        ))
    }
}
