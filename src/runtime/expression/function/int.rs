use crate::plan::{
    ExecutionPlan, FunctionValueKind, IntFunctionExpr, IntFunctionExprKind, IntFunctionValue,
    Value, ValueType,
};
use crate::runtime::ExecutionError;
use crate::runtime::expression::{
    eval_bool_expr, eval_float_expr, eval_int_expr, eval_panic_expr, eval_string_expr,
    project_function_list_expr, project_tuple_expr,
};
use crate::runtime::frame::Frame;
use crate::runtime::function;

pub(in crate::runtime) fn eval_int_function_expr(
    plan: &ExecutionPlan,
    frame: &mut Frame,
    expression: &IntFunctionExpr,
) -> Result<IntFunctionValue, ExecutionError> {
    match expression.kind() {
        IntFunctionExprKind::Value(value) => Ok(value.clone()),
        IntFunctionExprKind::Closure {
            runtime_id,
            params,
            captures,
        } => Ok(IntFunctionValue::new_with_captures(
            *runtime_id,
            params.clone(),
            function::eval_capture_args(plan, frame, captures)?,
        )),
        IntFunctionExprKind::LocalGet { local, .. } => Ok(frame.get_int_function(*local)),
        IntFunctionExprKind::Call { function, args, .. } => {
            function::run_int_function_returning_function_call(plan, *function, args, frame)
        }
        IntFunctionExprKind::FunctionCall {
            function: callee,
            args,
            ..
        } => function::run_int_function_function_call(plan, callee, args, frame),
        IntFunctionExprKind::TupleIndex {
            tuple,
            index,
            type_,
        } => {
            let expected = ValueType::Function(Box::new(type_.clone()));
            let value = project_tuple_expr(plan, frame, tuple, *index, expected.clone())?;
            let actual = value.value_type();
            match value {
                Value::Function(function) => match function.kind() {
                    FunctionValueKind::Int(value) => Ok(value.clone()),
                    _ => Err(ExecutionError::tuple_index_family_mismatch(
                        expected, actual,
                    )),
                },
                _ => Err(ExecutionError::tuple_index_family_mismatch(
                    expected, actual,
                )),
            }
        }
        IntFunctionExprKind::ListIndex { list, index, type_ } => {
            let expected = ValueType::Function(Box::new(type_.clone()));
            let function = project_function_list_expr(plan, frame, list, *index, type_)?;
            match function.kind() {
                FunctionValueKind::Int(value) => Ok(value.clone()),
                _ => Err(ExecutionError::list_item_type_mismatch(
                    expected,
                    Value::Function(function).value_type(),
                )),
            }
        }
        IntFunctionExprKind::Panic(panic) => eval_panic_expr(plan, frame, panic),
        IntFunctionExprKind::BoolCase {
            subject,
            true_,
            false_,
        } => {
            if eval_bool_expr(plan, frame, subject)? {
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
            let subject = eval_int_expr(plan, frame, subject)?;
            for (pattern, branch) in clauses {
                if pattern == &subject {
                    return eval_int_function_expr(plan, frame, branch);
                }
            }
            eval_int_function_expr(plan, frame, fallback)
        }
        IntFunctionExprKind::StringCase {
            subject,
            clauses,
            fallback,
        } => {
            let subject = eval_string_expr(plan, frame, subject)?;
            for (pattern, branch) in clauses {
                if pattern == &subject {
                    return eval_int_function_expr(plan, frame, branch);
                }
            }
            eval_int_function_expr(plan, frame, fallback)
        }
        IntFunctionExprKind::FloatCase {
            subject,
            clauses,
            fallback,
        } => {
            let subject = eval_float_expr(plan, frame, subject)?;
            for (pattern, branch) in clauses {
                if pattern == &subject {
                    return eval_int_function_expr(plan, frame, branch);
                }
            }
            eval_int_function_expr(plan, frame, fallback)
        }
        IntFunctionExprKind::Block { steps, return_ } => {
            function::execute_steps(plan, steps, frame)?;
            eval_int_function_expr(plan, frame, return_)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::eval_int_function_expr;
    use crate::plan::FrameLayout;
    use crate::plan::{
        BoolExpr, BoolFunctionExpr, BoolFunctionId, BoolFunctionValue, CaptureArg, ExecutionPlan,
        Expr, FloatExpr, FunctionExpr, FunctionFunctionExpr, FunctionFunctionId,
        FunctionFunctionValue, FunctionId, FunctionPlan, FunctionType, IntExpr, IntFunctionExpr,
        IntFunctionFunctionId, IntFunctionId, IntFunctionLocalId, IntFunctionValue, IntLocalId,
        ListExpr, ListLocalId, ListValue, PanicExpr, PanicSite, ParamLocal, ReturnExpr, Step,
        StringExpr, TupleExpr, ValueType,
    };
    use crate::runtime::frame::Frame;
    use crate::runtime::{ExecutionError, PanicKind};

    #[test]
    fn eval_int_function_local_closure_call_function_call_and_block() {
        let plan = plan();
        let mut frame = Frame::default();
        frame.set_int_function(IntFunctionLocalId(0), function_runtime_value());

        assert_eq!(
            eval_int_function_expr(
                &plan,
                &mut frame,
                &IntFunctionExpr::local_get(IntFunctionLocalId(0), "value".into(), type_()),
            )
            .expect("expression should evaluate")
            .runtime_id(),
            IntFunctionId(0),
        );
        assert_eq!(
            eval_int_function_expr(
                &plan,
                &mut frame,
                &IntFunctionExpr::closure(
                    IntFunctionId(0),
                    vec![ParamLocal::int(IntLocalId(0))],
                    Vec::new(),
                    type_(),
                ),
            )
            .expect("expression should evaluate")
            .runtime_id(),
            IntFunctionId(0),
        );
        assert_eq!(
            eval_int_function_expr(
                &plan,
                &mut frame,
                &IntFunctionExpr::call(IntFunctionFunctionId(0), Vec::new(), type_()),
            )
            .expect("expression should evaluate")
            .runtime_id(),
            IntFunctionId(0),
        );
        assert_eq!(
            eval_int_function_expr(
                &plan,
                &mut frame,
                &IntFunctionExpr::function_call(
                    FunctionFunctionExpr::value(FunctionFunctionValue::new(
                        FunctionFunctionId::Int(IntFunctionFunctionId(0)),
                        Vec::new(),
                        type_(),
                    )),
                    Vec::new(),
                    type_(),
                ),
            )
            .expect("expression should evaluate")
            .runtime_id(),
            IntFunctionId(0),
        );
        assert_eq!(
            eval_int_function_expr(
                &plan,
                &mut frame,
                &IntFunctionExpr::block(
                    vec![Step::evaluate(Expr::int(IntExpr::value(1.into())))],
                    function_value(),
                ),
            )
            .expect("expression should evaluate")
            .runtime_id(),
            IntFunctionId(0),
        );
    }

    #[test]
    fn eval_int_function_panic_returns_error() {
        let plan = plan();
        let mut frame = Frame::default();

        assert_eq!(
            eval_int_function_expr(
                &plan,
                &mut frame,
                &IntFunctionExpr::panic(PanicExpr::panic_at(None, PanicSite::unknown()), type_()),
            ),
            Err(ExecutionError::source_panic(
                None,
                PanicKind::Panic,
                None,
                PanicSite::unknown()
            )),
        );
    }

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
        )
        .expect("expression should evaluate");

        assert_eq!(function.runtime_id(), IntFunctionId(0));

        let function = eval_int_function_expr(
            &plan,
            &mut frame,
            &IntFunctionExpr::bool_case(
                BoolExpr::value(false),
                other_function_value(),
                function_value(),
            ),
        )
        .expect("expression should evaluate");

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
        )
        .expect("expression should evaluate");

        assert_eq!(function.runtime_id(), IntFunctionId(0));

        let function = eval_int_function_expr(
            &plan,
            &mut frame,
            &IntFunctionExpr::int_case(
                IntExpr::value(2.into()),
                vec![(1.into(), other_function_value())],
                function_value(),
            ),
        )
        .expect("expression should evaluate");

        assert_eq!(function.runtime_id(), IntFunctionId(0));
    }

    #[test]
    fn eval_int_function_string_case_branches() {
        let plan = plan();
        let mut frame = Frame::default();
        let function = eval_int_function_expr(
            &plan,
            &mut frame,
            &IntFunctionExpr::string_case(
                StringExpr::value("hit".into()),
                vec![("hit".into(), function_value())],
                other_function_value(),
            ),
        )
        .expect("expression should evaluate");

        assert_eq!(function.runtime_id(), IntFunctionId(0));

        let function = eval_int_function_expr(
            &plan,
            &mut frame,
            &IntFunctionExpr::string_case(
                StringExpr::value("miss".into()),
                vec![("hit".into(), other_function_value())],
                function_value(),
            ),
        )
        .expect("expression should evaluate");

        assert_eq!(function.runtime_id(), IntFunctionId(0));
    }

    #[test]
    fn eval_int_function_float_case_branches() {
        let plan = plan();
        let mut frame = Frame::default();
        let function = eval_int_function_expr(
            &plan,
            &mut frame,
            &IntFunctionExpr::float_case(
                FloatExpr::value(1.0),
                vec![(1.0, function_value())],
                other_function_value(),
            ),
        )
        .expect("expression should evaluate");

        assert_eq!(function.runtime_id(), IntFunctionId(0));

        let function = eval_int_function_expr(
            &plan,
            &mut frame,
            &IntFunctionExpr::float_case(
                FloatExpr::value(2.0),
                vec![(1.0, other_function_value())],
                function_value(),
            ),
        )
        .expect("expression should evaluate");

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
        )
        .expect("expression should evaluate");

        assert_eq!(function.runtime_id(), IntFunctionId(0));
    }

    #[test]
    fn eval_int_function_expr_propagates_operand_errors() {
        assert_tuple_index_error(
            ValueType::Int,
            IntFunctionExpr::closure(
                IntFunctionId(0),
                vec![ParamLocal::int(IntLocalId(0))],
                vec![CaptureArg::int(IntLocalId(0), error_int_expr())],
                type_(),
            ),
        );
        assert_function_tuple_index_error(IntFunctionExpr::tuple_index(empty_tuple(), 0, type_()));
        assert_tuple_index_error(
            ValueType::Bool,
            IntFunctionExpr::bool_case(error_bool_expr(), function_value(), other_function_value()),
        );
        assert_tuple_index_error(
            ValueType::Int,
            IntFunctionExpr::int_case(
                error_int_expr(),
                vec![(1.into(), function_value())],
                other_function_value(),
            ),
        );
        assert_tuple_index_error(
            ValueType::String,
            IntFunctionExpr::string_case(
                error_string_expr(),
                vec![("hit".into(), function_value())],
                other_function_value(),
            ),
        );
        assert_tuple_index_error(
            ValueType::Float,
            IntFunctionExpr::float_case(
                error_float_expr(),
                vec![(1.0, function_value())],
                other_function_value(),
            ),
        );
        assert_tuple_index_error(
            ValueType::Int,
            IntFunctionExpr::block(
                vec![Step::evaluate(Expr::int(error_int_expr()))],
                function_value(),
            ),
        );
    }

    #[test]
    fn eval_int_function_tuple_index() {
        let plan = plan();
        let mut frame = Frame::default();
        let tuple = TupleExpr::value(
            vec![Expr::function(FunctionExpr::int(function_value()))],
            vec![ValueType::Function(Box::new(type_()))],
        );

        let function = eval_int_function_expr(
            &plan,
            &mut frame,
            &IntFunctionExpr::tuple_index(tuple, 0, type_()),
        )
        .expect("expression should evaluate");
        assert_eq!(function.runtime_id(), IntFunctionId(0));

        let mismatch_type = FunctionType::new(Vec::new(), ValueType::Bool);
        let tuple = TupleExpr::value(
            vec![Expr::function(FunctionExpr::bool(BoolFunctionExpr::value(
                BoolFunctionValue::new(BoolFunctionId(0), Vec::new()),
            )))],
            vec![ValueType::Function(Box::new(mismatch_type.clone()))],
        );

        assert_eq!(
            eval_int_function_expr(
                &plan,
                &mut frame,
                &IntFunctionExpr::tuple_index(tuple, 0, type_())
            ),
            Err(ExecutionError::tuple_index_family_mismatch(
                ValueType::Function(Box::new(type_())),
                ValueType::Function(Box::new(mismatch_type)),
            )),
        );

        let tuple = TupleExpr::value(
            vec![Expr::int(IntExpr::value(1.into()))],
            vec![ValueType::Int],
        );
        assert_eq!(
            eval_int_function_expr(
                &plan,
                &mut frame,
                &IntFunctionExpr::tuple_index(tuple, 0, type_()),
            ),
            Err(ExecutionError::tuple_index_family_mismatch(
                ValueType::Function(Box::new(type_())),
                ValueType::Int,
            )),
        );
    }

    #[test]
    fn eval_int_function_list_index() {
        let plan = plan();
        let mut frame = Frame::default();
        let list = ListExpr::value(
            vec![Expr::function(FunctionExpr::int(function_value()))],
            ValueType::Function(Box::new(type_())),
        );
        assert_eq!(
            eval_int_function_expr(
                &plan,
                &mut frame,
                &IntFunctionExpr::list_index(list, 0, type_()),
            )
            .expect("expression should evaluate")
            .runtime_id(),
            IntFunctionId(0),
        );

        let mismatch_type = FunctionType::new(Vec::new(), ValueType::Bool);
        let list = ListExpr::value(
            vec![Expr::function(FunctionExpr::bool(BoolFunctionExpr::value(
                BoolFunctionValue::new(BoolFunctionId(0), Vec::new()),
            )))],
            ValueType::Function(Box::new(mismatch_type.clone())),
        );

        assert_eq!(
            eval_int_function_expr(
                &plan,
                &mut frame,
                &IntFunctionExpr::list_index(list, 0, type_()),
            ),
            Err(ExecutionError::list_item_type_mismatch(
                ValueType::Function(Box::new(type_())),
                ValueType::Function(Box::new(mismatch_type)),
            )),
        );

        let mut layout = FrameLayout::default();
        layout.include_list(ListLocalId(0), ValueType::Int);
        let mut frame = Frame::new(layout);
        let mismatch_type = FunctionType::new(Vec::new(), ValueType::Bool);
        frame.set_list(
            ListLocalId(0),
            ListValue::function(
                type_(),
                vec![BoolFunctionValue::new(BoolFunctionId(0), Vec::new()).into()],
            ),
        );
        let list = ListExpr::local_get(
            ListLocalId(0),
            "functions".into(),
            ValueType::Function(Box::new(type_())),
        );
        assert_eq!(
            eval_int_function_expr(
                &plan,
                &mut frame,
                &IntFunctionExpr::list_index(list, 0, type_()),
            ),
            Err(ExecutionError::list_item_type_mismatch(
                ValueType::Function(Box::new(type_())),
                ValueType::Function(Box::new(mismatch_type)),
            )),
        );

        let mut frame = Frame::default();
        let list = ListExpr::value(
            vec![Expr::function(FunctionExpr::int(function_value()))],
            ValueType::Function(Box::new(type_())),
        );
        assert_eq!(
            eval_int_function_expr(
                &plan,
                &mut frame,
                &IntFunctionExpr::list_index(list, 1, type_()),
            ),
            Err(ExecutionError::list_index_out_of_bounds(
                ValueType::Function(Box::new(type_())),
                1,
                1,
            )),
        );

        let list = ListExpr::tuple_index(empty_tuple(), 0, ValueType::Function(Box::new(type_())));
        assert_eq!(
            eval_int_function_expr(
                &plan,
                &mut frame,
                &IntFunctionExpr::list_index(list, 0, type_()),
            ),
            Err(ExecutionError::tuple_index_family_mismatch(
                ValueType::List(Box::new(ValueType::Function(Box::new(type_())))),
                ValueType::Tuple(Vec::new()),
            )),
        );

        let list = ListExpr::value(vec![Expr::int(IntExpr::value(1.into()))], ValueType::Int);
        assert_eq!(
            eval_int_function_expr(
                &plan,
                &mut frame,
                &IntFunctionExpr::list_index(list, 0, type_()),
            ),
            Err(ExecutionError::list_item_type_mismatch(
                ValueType::Function(Box::new(type_())),
                ValueType::Int,
            )),
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
            vec![FunctionPlan::new(
                FunctionId::new(1),
                "get_int_value".into(),
                Vec::new(),
                Vec::new(),
                ReturnExpr::int_function(IntFunctionFunctionId(0), function_value()),
            )],
        )
    }

    fn function_value() -> IntFunctionExpr {
        IntFunctionExpr::value(function_runtime_value())
    }

    fn function_runtime_value() -> IntFunctionValue {
        IntFunctionValue::new(IntFunctionId(0), vec![ParamLocal::int(IntLocalId(0))])
    }

    fn other_function_value() -> IntFunctionExpr {
        IntFunctionExpr::value(IntFunctionValue::new(
            IntFunctionId(1),
            vec![ParamLocal::int(IntLocalId(0))],
        ))
    }

    fn type_() -> FunctionType {
        FunctionType::new(vec![ValueType::Int], ValueType::Int)
    }

    fn assert_tuple_index_error(expected: ValueType, expression: IntFunctionExpr) {
        let plan = plan();
        let mut frame = Frame::default();

        assert_eq!(
            eval_int_function_expr(&plan, &mut frame, &expression),
            Err(tuple_index_error(expected)),
        );
    }

    fn assert_function_tuple_index_error(expression: IntFunctionExpr) {
        let plan = plan();
        let mut frame = Frame::default();

        assert_eq!(
            eval_int_function_expr(&plan, &mut frame, &expression),
            Err(tuple_index_error(ValueType::Function(Box::new(type_())))),
        );
    }

    fn tuple_index_error(expected: ValueType) -> ExecutionError {
        ExecutionError::tuple_index_family_mismatch(expected, ValueType::Tuple(Vec::new()))
    }

    fn empty_tuple() -> TupleExpr {
        TupleExpr::value(Vec::new(), Vec::new())
    }

    fn error_bool_expr() -> BoolExpr {
        BoolExpr::tuple_index(empty_tuple(), 0)
    }

    fn error_int_expr() -> IntExpr {
        IntExpr::tuple_index(empty_tuple(), 0)
    }

    fn error_string_expr() -> StringExpr {
        StringExpr::tuple_index(empty_tuple(), 0)
    }

    fn error_float_expr() -> FloatExpr {
        FloatExpr::tuple_index(empty_tuple(), 0)
    }
}
