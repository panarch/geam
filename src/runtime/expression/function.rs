use super::{eval_bool_expr, eval_int_expr};
use crate::plan::{ExecutionPlan, FunctionExpr, FunctionExprKind, FunctionValue};
use crate::runtime::frame::Frame;
use crate::runtime::function;

pub(in crate::runtime) fn eval_function_expr(
    plan: &ExecutionPlan,
    frame: &mut Frame,
    expression: &FunctionExpr,
) -> FunctionValue {
    match expression.kind() {
        FunctionExprKind::Value(value) => value.clone(),
        FunctionExprKind::BoolCase {
            subject,
            true_,
            false_,
        } => {
            if eval_bool_expr(plan, frame, subject) {
                eval_function_expr(plan, frame, true_)
            } else {
                eval_function_expr(plan, frame, false_)
            }
        }
        FunctionExprKind::IntCase {
            subject,
            clauses,
            fallback,
        } => {
            let subject = eval_int_expr(plan, frame, subject);
            for (pattern, branch) in clauses {
                if pattern == &subject {
                    return eval_function_expr(plan, frame, branch);
                }
            }
            eval_function_expr(plan, frame, fallback)
        }
        FunctionExprKind::Block { steps, return_ } => {
            function::execute_steps(plan, steps, frame);
            eval_function_expr(plan, frame, return_)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::eval_function_expr;
    use crate::plan::{
        BoolExpr, ExecutionPlan, Expr, FunctionArgumentType, FunctionExpr, FunctionId,
        FunctionPlan, FunctionType, FunctionValue, IntExpr, IntFunctionId, RuntimeFunctionId, Step,
        ValueType,
    };
    use crate::runtime::frame::Frame;
    use num_bigint::BigInt;

    #[test]
    fn eval_function_value() {
        let plan = plan_with_int_main(Vec::new());
        let mut frame = Frame::default();
        let function = eval_function_expr(
            &plan,
            &mut frame,
            &FunctionExpr::value(int_function_value()),
        );

        assert_int_function(function);
    }

    #[test]
    fn eval_function_bool_case_branches() {
        let plan = plan_with_int_main(Vec::new());
        let mut frame = Frame::default();
        let function = eval_function_expr(
            &plan,
            &mut frame,
            &FunctionExpr::bool_case(
                BoolExpr::value(true),
                FunctionExpr::value(int_function_value()),
                FunctionExpr::value(other_int_function_value()),
            ),
        );

        assert_int_function(function);

        let function = eval_function_expr(
            &plan,
            &mut frame,
            &FunctionExpr::bool_case(
                BoolExpr::value(false),
                FunctionExpr::value(other_int_function_value()),
                FunctionExpr::value(int_function_value()),
            ),
        );

        assert_int_function(function);
    }

    #[test]
    fn eval_function_int_case_branches() {
        let plan = plan_with_int_main(Vec::new());
        let mut frame = Frame::default();
        let function = eval_function_expr(
            &plan,
            &mut frame,
            &FunctionExpr::int_case(
                IntExpr::value(BigInt::from(1)),
                vec![(BigInt::from(1), FunctionExpr::value(int_function_value()))],
                FunctionExpr::value(other_int_function_value()),
            ),
        );

        assert_int_function(function);

        let function = eval_function_expr(
            &plan,
            &mut frame,
            &FunctionExpr::int_case(
                IntExpr::value(BigInt::from(2)),
                vec![(
                    BigInt::from(1),
                    FunctionExpr::value(other_int_function_value()),
                )],
                FunctionExpr::value(int_function_value()),
            ),
        );

        assert_int_function(function);
    }

    #[test]
    fn eval_function_block() {
        let plan = plan_with_int_main(Vec::new());
        let mut frame = Frame::default();
        let function = eval_function_expr(
            &plan,
            &mut frame,
            &FunctionExpr::block(
                vec![Step::evaluate(Expr::int(IntExpr::value(BigInt::from(1))))],
                FunctionExpr::value(int_function_value()),
            ),
        );

        assert_int_function(function);
    }

    fn plan_with_int_main(functions: Vec<FunctionPlan>) -> ExecutionPlan {
        ExecutionPlan::new(
            "main".into(),
            FunctionPlan::new(
                FunctionId::new(0),
                "main".into(),
                Vec::new(),
                Vec::new(),
                crate::plan::ReturnExpr::int(IntExpr::value(BigInt::from(1))),
            ),
            functions,
        )
    }

    fn assert_int_function(function: FunctionValue) {
        let type_ = function.type_();

        assert_eq!(
            type_,
            FunctionType::new(vec![FunctionArgumentType::Int], ValueType::Int),
        );
        assert_eq!(type_.return_(), &ValueType::Int);
    }

    fn int_function_value() -> FunctionValue {
        FunctionValue::new(
            RuntimeFunctionId::Int(IntFunctionId(0)),
            vec![crate::plan::LocalId::Int(crate::plan::IntLocalId(0))],
        )
    }

    fn other_int_function_value() -> FunctionValue {
        FunctionValue::new(
            RuntimeFunctionId::Int(IntFunctionId(1)),
            vec![crate::plan::LocalId::Int(crate::plan::IntLocalId(0))],
        )
    }
}
