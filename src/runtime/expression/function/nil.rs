use crate::plan::{ExecutionPlan, NilFunctionExpr, NilFunctionExprKind, NilFunctionValue};
use crate::runtime::ExecutionError;
use crate::runtime::expression::{eval_bool_expr, eval_int_expr};
use crate::runtime::frame::Frame;
use crate::runtime::function;

pub(in crate::runtime) fn eval_nil_function_expr(
    plan: &ExecutionPlan,
    frame: &mut Frame,
    expression: &NilFunctionExpr,
) -> Result<NilFunctionValue, ExecutionError> {
    match expression.kind() {
        NilFunctionExprKind::Value(value) => Ok(value.clone()),
        NilFunctionExprKind::Closure {
            runtime_id,
            params,
            captures,
        } => Ok(NilFunctionValue::new_with_captures(
            *runtime_id,
            params.clone(),
            function::eval_capture_args(plan, frame, captures)?,
        )),
        NilFunctionExprKind::LocalGet { local, .. } => Ok(frame.get_nil_function(*local)),
        NilFunctionExprKind::Call { function, args, .. } => {
            function::run_nil_function_returning_function_call(plan, *function, args, frame)
        }
        NilFunctionExprKind::FunctionCall {
            function: callee,
            args,
            ..
        } => function::run_nil_function_function_call(plan, callee, args, frame),
        NilFunctionExprKind::BoolCase {
            subject,
            true_,
            false_,
        } => {
            if eval_bool_expr(plan, frame, subject)? {
                eval_nil_function_expr(plan, frame, true_)
            } else {
                eval_nil_function_expr(plan, frame, false_)
            }
        }
        NilFunctionExprKind::IntCase {
            subject,
            clauses,
            fallback,
        } => {
            let subject = eval_int_expr(plan, frame, subject)?;
            for (pattern, branch) in clauses {
                if pattern == &subject {
                    return eval_nil_function_expr(plan, frame, branch);
                }
            }
            eval_nil_function_expr(plan, frame, fallback)
        }
        NilFunctionExprKind::Block { steps, return_ } => {
            function::execute_steps(plan, steps, frame)?;
            eval_nil_function_expr(plan, frame, return_)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::eval_nil_function_expr;
    use crate::plan::{
        BoolExpr, ExecutionPlan, Expr, FunctionId, FunctionPlan, IntExpr, IntFunctionId,
        NilFunctionExpr, NilFunctionId, NilFunctionValue, NilLocalId, ParamLocal, ReturnExpr, Step,
    };
    use crate::runtime::frame::Frame;

    #[test]
    fn eval_nil_function_bool_case_branches() {
        let plan = plan();
        let mut frame = Frame::default();

        assert_eq!(
            eval_nil_function_expr(
                &plan,
                &mut frame,
                &NilFunctionExpr::bool_case(
                    BoolExpr::value(true),
                    function_value(),
                    other_function_value(),
                ),
            )
            .expect("expression should evaluate")
            .runtime_id(),
            NilFunctionId(0),
        );
        assert_eq!(
            eval_nil_function_expr(
                &plan,
                &mut frame,
                &NilFunctionExpr::bool_case(
                    BoolExpr::value(false),
                    other_function_value(),
                    function_value(),
                ),
            )
            .expect("expression should evaluate")
            .runtime_id(),
            NilFunctionId(0),
        );
    }

    #[test]
    fn eval_nil_function_int_case_branches() {
        let plan = plan();
        let mut frame = Frame::default();

        assert_eq!(
            eval_nil_function_expr(
                &plan,
                &mut frame,
                &NilFunctionExpr::int_case(
                    IntExpr::value(1.into()),
                    vec![(1.into(), function_value())],
                    other_function_value(),
                ),
            )
            .expect("expression should evaluate")
            .runtime_id(),
            NilFunctionId(0),
        );
        assert_eq!(
            eval_nil_function_expr(
                &plan,
                &mut frame,
                &NilFunctionExpr::int_case(
                    IntExpr::value(2.into()),
                    vec![(1.into(), other_function_value())],
                    function_value(),
                ),
            )
            .expect("expression should evaluate")
            .runtime_id(),
            NilFunctionId(0),
        );
    }

    #[test]
    fn eval_nil_function_block() {
        let plan = plan();
        let mut frame = Frame::default();

        assert_eq!(
            eval_nil_function_expr(
                &plan,
                &mut frame,
                &NilFunctionExpr::block(
                    vec![Step::evaluate(Expr::int(IntExpr::value(1.into())))],
                    function_value(),
                ),
            )
            .expect("expression should evaluate")
            .runtime_id(),
            NilFunctionId(0),
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
                ReturnExpr::int(IntFunctionId(0), IntExpr::value(1.into())),
            ),
            Vec::new(),
        )
    }

    fn function_value() -> NilFunctionExpr {
        NilFunctionExpr::value(NilFunctionValue::new(
            NilFunctionId(0),
            vec![ParamLocal::nil(NilLocalId(0))],
        ))
    }

    fn other_function_value() -> NilFunctionExpr {
        NilFunctionExpr::value(NilFunctionValue::new(
            NilFunctionId(1),
            vec![ParamLocal::nil(NilLocalId(0))],
        ))
    }
}
