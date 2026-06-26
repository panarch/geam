use super::{eval_bool_expr, eval_int_expr};
use crate::plan::{
    BoolFunctionExpr, BoolFunctionExprKind, BoolFunctionValue, ExecutionPlan, FunctionExpr,
    FunctionExprKind, FunctionValue, IntFunctionExpr, IntFunctionExprKind, IntFunctionValue,
    NilFunctionExpr, NilFunctionExprKind, NilFunctionValue, StringFunctionExpr,
    StringFunctionExprKind, StringFunctionValue,
};
use crate::runtime::frame::Frame;
use crate::runtime::function;

pub(in crate::runtime) fn eval_function_expr(
    plan: &ExecutionPlan,
    frame: &mut Frame,
    expression: &FunctionExpr,
) -> FunctionValue {
    match expression.kind() {
        FunctionExprKind::Int(expression) => eval_int_function_expr(plan, frame, expression).into(),
        FunctionExprKind::String(expression) => {
            eval_string_function_expr(plan, frame, expression).into()
        }
        FunctionExprKind::Bool(expression) => {
            eval_bool_function_expr(plan, frame, expression).into()
        }
        FunctionExprKind::Nil(expression) => eval_nil_function_expr(plan, frame, expression).into(),
    }
}

pub(in crate::runtime) fn eval_int_function_expr(
    plan: &ExecutionPlan,
    frame: &mut Frame,
    expression: &IntFunctionExpr,
) -> IntFunctionValue {
    match expression.kind() {
        IntFunctionExprKind::Value(value) => value.clone(),
        IntFunctionExprKind::LocalGet { local, .. } => frame.get_int_function(*local),
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

pub(in crate::runtime) fn eval_bool_function_expr(
    plan: &ExecutionPlan,
    frame: &mut Frame,
    expression: &BoolFunctionExpr,
) -> BoolFunctionValue {
    match expression.kind() {
        BoolFunctionExprKind::Value(value) => value.clone(),
        BoolFunctionExprKind::LocalGet { local, .. } => frame.get_bool_function(*local),
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

pub(in crate::runtime) fn eval_nil_function_expr(
    plan: &ExecutionPlan,
    frame: &mut Frame,
    expression: &NilFunctionExpr,
) -> NilFunctionValue {
    match expression.kind() {
        NilFunctionExprKind::Value(value) => value.clone(),
        NilFunctionExprKind::LocalGet { local, .. } => frame.get_nil_function(*local),
        NilFunctionExprKind::BoolCase {
            subject,
            true_,
            false_,
        } => {
            if eval_bool_expr(plan, frame, subject) {
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
            let subject = eval_int_expr(plan, frame, subject);
            for (pattern, branch) in clauses {
                if pattern == &subject {
                    return eval_nil_function_expr(plan, frame, branch);
                }
            }
            eval_nil_function_expr(plan, frame, fallback)
        }
        NilFunctionExprKind::Block { steps, return_ } => {
            function::execute_steps(plan, steps, frame);
            eval_nil_function_expr(plan, frame, return_)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        eval_bool_function_expr, eval_function_expr, eval_int_function_expr,
        eval_nil_function_expr, eval_string_function_expr,
    };
    use crate::plan::{
        BoolExpr, BoolFunctionExpr, BoolFunctionId, BoolFunctionValue, BoolLocalId, ExecutionPlan,
        Expr, FunctionArgumentType, FunctionExpr, FunctionId, FunctionPlan, FunctionType,
        FunctionValue, IntExpr, IntFunctionExpr, IntFunctionId, IntFunctionValue, IntLocalId,
        LocalId, NilFunctionExpr, NilFunctionId, NilFunctionValue, NilLocalId, RuntimeFunctionId,
        Step, StringFunctionExpr, StringFunctionId, StringFunctionValue, StringLocalId, ValueType,
    };
    use crate::runtime::frame::Frame;
    use num_bigint::BigInt;

    #[test]
    fn eval_function_value() {
        let plan = plan_with_int_main(Vec::new());
        let mut frame = Frame::default();
        let function =
            eval_function_expr(&plan, &mut frame, &FunctionExpr::value(function_value()));

        assert_int_function(function);
    }

    #[test]
    fn eval_function_value_return_families() {
        let plan = plan_with_int_main(Vec::new());
        let mut frame = Frame::default();

        assert_eq!(
            eval_function_expr(
                &plan,
                &mut frame,
                &FunctionExpr::value(string_function_value())
            )
            .type_()
            .return_(),
            &ValueType::String,
        );
        assert_eq!(
            eval_function_expr(
                &plan,
                &mut frame,
                &FunctionExpr::value(bool_function_value())
            )
            .type_()
            .return_(),
            &ValueType::Bool,
        );
        assert_eq!(
            eval_function_expr(
                &plan,
                &mut frame,
                &FunctionExpr::value(nil_function_value())
            )
            .type_()
            .return_(),
            &ValueType::Nil,
        );
    }

    #[test]
    fn eval_int_function_bool_case_branches() {
        let plan = plan_with_int_main(Vec::new());
        let mut frame = Frame::default();
        let function = eval_int_function_expr(
            &plan,
            &mut frame,
            &IntFunctionExpr::bool_case(
                BoolExpr::value(true),
                int_function_value(),
                other_int_function_value(),
            ),
        );

        assert_eq!(function.runtime_id(), IntFunctionId(0));

        let function = eval_int_function_expr(
            &plan,
            &mut frame,
            &IntFunctionExpr::bool_case(
                BoolExpr::value(false),
                other_int_function_value(),
                int_function_value(),
            ),
        );

        assert_eq!(function.runtime_id(), IntFunctionId(0));
    }

    #[test]
    fn eval_int_function_int_case_branches() {
        let plan = plan_with_int_main(Vec::new());
        let mut frame = Frame::default();
        let function = eval_int_function_expr(
            &plan,
            &mut frame,
            &IntFunctionExpr::int_case(
                IntExpr::value(BigInt::from(1)),
                vec![(BigInt::from(1), int_function_value())],
                other_int_function_value(),
            ),
        );

        assert_eq!(function.runtime_id(), IntFunctionId(0));

        let function = eval_int_function_expr(
            &plan,
            &mut frame,
            &IntFunctionExpr::int_case(
                IntExpr::value(BigInt::from(2)),
                vec![(BigInt::from(1), other_int_function_value())],
                int_function_value(),
            ),
        );

        assert_eq!(function.runtime_id(), IntFunctionId(0));
    }

    #[test]
    fn eval_int_function_block() {
        let plan = plan_with_int_main(Vec::new());
        let mut frame = Frame::default();
        let function = eval_int_function_expr(
            &plan,
            &mut frame,
            &IntFunctionExpr::block(
                vec![Step::evaluate(Expr::int(IntExpr::value(BigInt::from(1))))],
                int_function_value(),
            ),
        );

        assert_eq!(function.runtime_id(), IntFunctionId(0));
    }

    #[test]
    fn eval_string_bool_nil_function_branches() {
        let plan = plan_with_int_main(Vec::new());
        let mut frame = Frame::default();

        let string = eval_string_function_expr(
            &plan,
            &mut frame,
            &StringFunctionExpr::bool_case(
                BoolExpr::value(true),
                string_function_expr(),
                other_string_function_expr(),
            ),
        );
        assert_eq!(string.runtime_id(), StringFunctionId(0));

        let string = eval_string_function_expr(
            &plan,
            &mut frame,
            &StringFunctionExpr::bool_case(
                BoolExpr::value(false),
                other_string_function_expr(),
                string_function_expr(),
            ),
        );
        assert_eq!(string.runtime_id(), StringFunctionId(0));

        let string = eval_string_function_expr(
            &plan,
            &mut frame,
            &StringFunctionExpr::int_case(
                IntExpr::value(BigInt::from(1)),
                vec![(BigInt::from(1), string_function_expr())],
                other_string_function_expr(),
            ),
        );
        assert_eq!(string.runtime_id(), StringFunctionId(0));

        let string = eval_string_function_expr(
            &plan,
            &mut frame,
            &StringFunctionExpr::int_case(
                IntExpr::value(BigInt::from(2)),
                vec![(BigInt::from(1), other_string_function_expr())],
                string_function_expr(),
            ),
        );
        assert_eq!(string.runtime_id(), StringFunctionId(0));

        let string = eval_string_function_expr(
            &plan,
            &mut frame,
            &StringFunctionExpr::block(
                vec![Step::evaluate(Expr::int(IntExpr::value(BigInt::from(1))))],
                string_function_expr(),
            ),
        );
        assert_eq!(string.runtime_id(), StringFunctionId(0));

        let bool_ = eval_bool_function_expr(
            &plan,
            &mut frame,
            &BoolFunctionExpr::bool_case(
                BoolExpr::value(true),
                bool_function_expr(),
                other_bool_function_expr(),
            ),
        );
        assert_eq!(bool_.runtime_id(), BoolFunctionId(0));

        let bool_ = eval_bool_function_expr(
            &plan,
            &mut frame,
            &BoolFunctionExpr::bool_case(
                BoolExpr::value(false),
                other_bool_function_expr(),
                bool_function_expr(),
            ),
        );
        assert_eq!(bool_.runtime_id(), BoolFunctionId(0));

        let bool_ = eval_bool_function_expr(
            &plan,
            &mut frame,
            &BoolFunctionExpr::int_case(
                IntExpr::value(BigInt::from(1)),
                vec![(BigInt::from(1), bool_function_expr())],
                other_bool_function_expr(),
            ),
        );
        assert_eq!(bool_.runtime_id(), BoolFunctionId(0));

        let bool_ = eval_bool_function_expr(
            &plan,
            &mut frame,
            &BoolFunctionExpr::int_case(
                IntExpr::value(BigInt::from(2)),
                vec![(BigInt::from(1), other_bool_function_expr())],
                bool_function_expr(),
            ),
        );
        assert_eq!(bool_.runtime_id(), BoolFunctionId(0));

        let bool_ = eval_bool_function_expr(
            &plan,
            &mut frame,
            &BoolFunctionExpr::block(
                vec![Step::evaluate(Expr::int(IntExpr::value(BigInt::from(1))))],
                bool_function_expr(),
            ),
        );
        assert_eq!(bool_.runtime_id(), BoolFunctionId(0));

        let nil = eval_nil_function_expr(
            &plan,
            &mut frame,
            &NilFunctionExpr::bool_case(
                BoolExpr::value(true),
                nil_function_expr(),
                other_nil_function_expr(),
            ),
        );
        assert_eq!(nil.runtime_id(), NilFunctionId(0));

        let nil = eval_nil_function_expr(
            &plan,
            &mut frame,
            &NilFunctionExpr::bool_case(
                BoolExpr::value(false),
                other_nil_function_expr(),
                nil_function_expr(),
            ),
        );
        assert_eq!(nil.runtime_id(), NilFunctionId(0));

        let nil = eval_nil_function_expr(
            &plan,
            &mut frame,
            &NilFunctionExpr::int_case(
                IntExpr::value(BigInt::from(1)),
                vec![(BigInt::from(1), nil_function_expr())],
                other_nil_function_expr(),
            ),
        );
        assert_eq!(nil.runtime_id(), NilFunctionId(0));

        let nil = eval_nil_function_expr(
            &plan,
            &mut frame,
            &NilFunctionExpr::int_case(
                IntExpr::value(BigInt::from(2)),
                vec![(BigInt::from(1), other_nil_function_expr())],
                nil_function_expr(),
            ),
        );
        assert_eq!(nil.runtime_id(), NilFunctionId(0));

        let nil = eval_nil_function_expr(
            &plan,
            &mut frame,
            &NilFunctionExpr::block(
                vec![Step::evaluate(Expr::int(IntExpr::value(BigInt::from(1))))],
                nil_function_expr(),
            ),
        );
        assert_eq!(nil.runtime_id(), NilFunctionId(0));
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

    fn function_value() -> FunctionValue {
        FunctionValue::new(
            RuntimeFunctionId::Int(IntFunctionId(0)),
            vec![LocalId::Int(IntLocalId(0))],
        )
    }

    fn string_function_value() -> FunctionValue {
        FunctionValue::new(
            RuntimeFunctionId::String(StringFunctionId(0)),
            vec![LocalId::String(StringLocalId(0))],
        )
    }

    fn bool_function_value() -> FunctionValue {
        FunctionValue::new(
            RuntimeFunctionId::Bool(BoolFunctionId(0)),
            vec![LocalId::Bool(BoolLocalId(0))],
        )
    }

    fn nil_function_value() -> FunctionValue {
        FunctionValue::new(
            RuntimeFunctionId::Nil(NilFunctionId(0)),
            vec![LocalId::Nil(NilLocalId(0))],
        )
    }

    fn int_function_value() -> IntFunctionExpr {
        IntFunctionExpr::value(IntFunctionValue::new(
            IntFunctionId(0),
            vec![LocalId::Int(IntLocalId(0))],
        ))
    }

    fn other_int_function_value() -> IntFunctionExpr {
        IntFunctionExpr::value(IntFunctionValue::new(
            IntFunctionId(1),
            vec![LocalId::Int(IntLocalId(0))],
        ))
    }

    fn string_function_expr() -> StringFunctionExpr {
        StringFunctionExpr::value(StringFunctionValue::new(
            StringFunctionId(0),
            vec![LocalId::String(StringLocalId(0))],
        ))
    }

    fn other_string_function_expr() -> StringFunctionExpr {
        StringFunctionExpr::value(StringFunctionValue::new(
            StringFunctionId(1),
            vec![LocalId::String(StringLocalId(0))],
        ))
    }

    fn bool_function_expr() -> BoolFunctionExpr {
        BoolFunctionExpr::value(BoolFunctionValue::new(
            BoolFunctionId(0),
            vec![LocalId::Bool(BoolLocalId(0))],
        ))
    }

    fn other_bool_function_expr() -> BoolFunctionExpr {
        BoolFunctionExpr::value(BoolFunctionValue::new(
            BoolFunctionId(1),
            vec![LocalId::Bool(BoolLocalId(0))],
        ))
    }

    fn nil_function_expr() -> NilFunctionExpr {
        NilFunctionExpr::value(NilFunctionValue::new(
            NilFunctionId(0),
            vec![LocalId::Nil(NilLocalId(0))],
        ))
    }

    fn other_nil_function_expr() -> NilFunctionExpr {
        NilFunctionExpr::value(NilFunctionValue::new(
            NilFunctionId(1),
            vec![LocalId::Nil(NilLocalId(0))],
        ))
    }
}
