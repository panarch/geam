use crate::plan::{
    ExecutionPlan, FloatFunctionExpr, FloatFunctionExprKind, FloatFunctionValue, FunctionValueKind,
    Value, ValueType,
};
use crate::runtime::ExecutionError;
use crate::runtime::expression::{
    eval_bool_expr, eval_float_expr, eval_int_expr, eval_string_expr, project_tuple_expr,
};
use crate::runtime::frame::Frame;
use crate::runtime::function;

pub(in crate::runtime) fn eval_float_function_expr(
    plan: &ExecutionPlan,
    frame: &mut Frame,
    expression: &FloatFunctionExpr,
) -> Result<FloatFunctionValue, ExecutionError> {
    match expression.kind() {
        FloatFunctionExprKind::Value(value) => Ok(value.clone()),
        FloatFunctionExprKind::Closure {
            runtime_id,
            params,
            captures,
        } => Ok(FloatFunctionValue::new_with_captures(
            *runtime_id,
            params.clone(),
            function::eval_capture_args(plan, frame, captures)?,
        )),
        FloatFunctionExprKind::LocalGet { local, .. } => Ok(frame.get_float_function(*local)),
        FloatFunctionExprKind::Call { function, args, .. } => {
            function::run_float_function_returning_function_call(plan, *function, args, frame)
        }
        FloatFunctionExprKind::FunctionCall {
            function: callee,
            args,
            ..
        } => function::run_float_function_function_call(plan, callee, args, frame),
        FloatFunctionExprKind::TupleIndex {
            tuple,
            index,
            type_,
        } => {
            let expected = ValueType::Function(Box::new(type_.clone()));
            let value = project_tuple_expr(plan, frame, tuple, *index, expected.clone())?;
            let actual = value.value_type();
            match value {
                Value::Function(function) => match function.kind() {
                    FunctionValueKind::Float(value) => Ok(value.clone()),
                    _ => Err(ExecutionError::tuple_index_family_mismatch(
                        expected, actual,
                    )),
                },
                _ => Err(ExecutionError::tuple_index_family_mismatch(
                    expected, actual,
                )),
            }
        }
        FloatFunctionExprKind::BoolCase {
            subject,
            true_,
            false_,
        } => {
            if eval_bool_expr(plan, frame, subject)? {
                eval_float_function_expr(plan, frame, true_)
            } else {
                eval_float_function_expr(plan, frame, false_)
            }
        }
        FloatFunctionExprKind::IntCase {
            subject,
            clauses,
            fallback,
        } => {
            let subject = eval_int_expr(plan, frame, subject)?;
            for (pattern, branch) in clauses {
                if pattern == &subject {
                    return eval_float_function_expr(plan, frame, branch);
                }
            }
            eval_float_function_expr(plan, frame, fallback)
        }
        FloatFunctionExprKind::StringCase {
            subject,
            clauses,
            fallback,
        } => {
            let subject = eval_string_expr(plan, frame, subject)?;
            for (pattern, branch) in clauses {
                if pattern == &subject {
                    return eval_float_function_expr(plan, frame, branch);
                }
            }
            eval_float_function_expr(plan, frame, fallback)
        }
        FloatFunctionExprKind::FloatCase {
            subject,
            clauses,
            fallback,
        } => {
            let subject = eval_float_expr(plan, frame, subject)?;
            for (pattern, branch) in clauses {
                if pattern == &subject {
                    return eval_float_function_expr(plan, frame, branch);
                }
            }
            eval_float_function_expr(plan, frame, fallback)
        }
        FloatFunctionExprKind::Block { steps, return_ } => {
            function::execute_steps(plan, steps, frame)?;
            eval_float_function_expr(plan, frame, return_)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::eval_float_function_expr;
    use crate::plan::{
        BoolExpr, BoolFunctionExpr, CaptureArg, ExecutionPlan, Expr, FloatExpr, FloatFunctionExpr,
        FloatFunctionFunctionId, FloatFunctionId, FloatFunctionLocalId, FloatFunctionValue,
        FloatLocalId, FunctionExpr, FunctionFunctionExpr, FunctionFunctionId,
        FunctionFunctionValue, FunctionId, FunctionPlan, FunctionReturnFamily, FunctionType,
        IntExpr, IntFunctionExpr, IntFunctionId, ParamLocal, ReturnExpr, Step, StringExpr,
        StringFunctionExpr, StringFunctionFunctionId, TupleExpr, ValueType,
    };
    use crate::runtime::ExecutionError;
    use crate::runtime::frame::Frame;

    #[test]
    fn eval_float_function_value_local_function_call_and_block() {
        let plan = plan();
        let mut frame = Frame::default();
        frame.set_float_function(FloatFunctionLocalId(0), function_runtime_value());

        let function = eval_float_function_expr(
            &plan,
            &mut frame,
            &FloatFunctionExpr::local_get(FloatFunctionLocalId(0), "value".into(), type_()),
        )
        .expect("expression should evaluate");
        assert_eq!(function.runtime_id(), FloatFunctionId(0));

        let function = eval_float_function_expr(
            &plan,
            &mut frame,
            &FloatFunctionExpr::closure(
                FloatFunctionId(0),
                vec![ParamLocal::float(FloatLocalId(0))],
                Vec::new(),
                type_(),
            ),
        )
        .expect("expression should evaluate");
        assert_eq!(function.runtime_id(), FloatFunctionId(0));

        let function = eval_float_function_expr(
            &plan,
            &mut frame,
            &FloatFunctionExpr::function_call(
                FunctionFunctionExpr::value(FunctionFunctionValue::new(
                    FunctionFunctionId::Float(FloatFunctionFunctionId(0)),
                    Vec::new(),
                    type_(),
                )),
                Vec::new(),
                type_(),
            ),
        )
        .expect("expression should evaluate");
        assert_eq!(function.runtime_id(), FloatFunctionId(0));

        let function = eval_float_function_expr(
            &plan,
            &mut frame,
            &FloatFunctionExpr::block(
                vec![Step::evaluate(Expr::float(FloatExpr::value(1.0)))],
                function_value(),
            ),
        )
        .expect("expression should evaluate");
        assert_eq!(function.runtime_id(), FloatFunctionId(0));
    }

    #[test]
    fn eval_float_function_case_branches() {
        let plan = plan();
        let mut frame = Frame::default();

        let function = eval_float_function_expr(
            &plan,
            &mut frame,
            &FloatFunctionExpr::bool_case(BoolExpr::value(true), function_value(), other_value()),
        )
        .expect("expression should evaluate");
        assert_eq!(function.runtime_id(), FloatFunctionId(0));

        let function = eval_float_function_expr(
            &plan,
            &mut frame,
            &FloatFunctionExpr::bool_case(BoolExpr::value(false), other_value(), function_value()),
        )
        .expect("expression should evaluate");
        assert_eq!(function.runtime_id(), FloatFunctionId(0));

        let function = eval_float_function_expr(
            &plan,
            &mut frame,
            &FloatFunctionExpr::int_case(
                IntExpr::value(1.into()),
                vec![(1.into(), function_value())],
                other_value(),
            ),
        )
        .expect("expression should evaluate");
        assert_eq!(function.runtime_id(), FloatFunctionId(0));

        let function = eval_float_function_expr(
            &plan,
            &mut frame,
            &FloatFunctionExpr::int_case(
                IntExpr::value(2.into()),
                vec![(1.into(), other_value())],
                function_value(),
            ),
        )
        .expect("expression should evaluate");
        assert_eq!(function.runtime_id(), FloatFunctionId(0));

        let function = eval_float_function_expr(
            &plan,
            &mut frame,
            &FloatFunctionExpr::string_case(
                StringExpr::value("hit".into()),
                vec![("hit".into(), function_value())],
                other_value(),
            ),
        )
        .expect("expression should evaluate");
        assert_eq!(function.runtime_id(), FloatFunctionId(0));

        let function = eval_float_function_expr(
            &plan,
            &mut frame,
            &FloatFunctionExpr::string_case(
                StringExpr::value("miss".into()),
                vec![("hit".into(), other_value())],
                function_value(),
            ),
        )
        .expect("expression should evaluate");
        assert_eq!(function.runtime_id(), FloatFunctionId(0));

        let function = eval_float_function_expr(
            &plan,
            &mut frame,
            &FloatFunctionExpr::float_case(
                FloatExpr::value(1.0),
                vec![(1.0, function_value())],
                other_value(),
            ),
        )
        .expect("expression should evaluate");
        assert_eq!(function.runtime_id(), FloatFunctionId(0));

        let function = eval_float_function_expr(
            &plan,
            &mut frame,
            &FloatFunctionExpr::float_case(
                FloatExpr::value(2.0),
                vec![(1.0, other_value())],
                function_value(),
            ),
        )
        .expect("expression should evaluate");
        assert_eq!(function.runtime_id(), FloatFunctionId(0));
    }

    #[test]
    fn eval_float_function_expr_propagates_operand_errors() {
        let execution_plan = plan();
        let mut frame = Frame::default();

        assert_float_error(FloatFunctionExpr::closure(
            FloatFunctionId(0),
            vec![ParamLocal::float(FloatLocalId(0))],
            vec![CaptureArg::float(FloatLocalId(0), error_float_expr())],
            type_(),
        ));
        assert_eq!(
            eval_float_function_expr(
                &execution_plan,
                &mut frame,
                &FloatFunctionExpr::bool_case(error_bool_expr(), function_value(), other_value(),),
            ),
            Err(function_return_family_error_value(
                FunctionReturnFamily::Bool,
                FunctionReturnFamily::String,
            )),
        );
        assert_eq!(
            eval_float_function_expr(
                &execution_plan,
                &mut frame,
                &FloatFunctionExpr::int_case(
                    error_int_expr(),
                    vec![(1.into(), function_value())],
                    other_value(),
                ),
            ),
            Err(function_return_family_error_value(
                FunctionReturnFamily::Int,
                FunctionReturnFamily::String,
            )),
        );
        assert_eq!(
            eval_float_function_expr(
                &execution_plan,
                &mut frame,
                &FloatFunctionExpr::string_case(
                    error_string_expr(),
                    vec![("hit".into(), function_value())],
                    other_value(),
                ),
            ),
            Err(function_return_family_error_value(
                FunctionReturnFamily::String,
                FunctionReturnFamily::Int,
            )),
        );
        assert_float_error(FloatFunctionExpr::float_case(
            error_float_expr(),
            vec![(1.0, function_value())],
            other_value(),
        ));
        assert_float_error(FloatFunctionExpr::block(
            vec![Step::evaluate(Expr::float(error_float_expr()))],
            function_value(),
        ));

        fn assert_float_error(expression: FloatFunctionExpr) {
            let plan = plan();
            let mut frame = Frame::default();

            assert_eq!(
                eval_float_function_expr(&plan, &mut frame, &expression),
                Err(function_return_family_error_value(
                    FunctionReturnFamily::Float,
                    FunctionReturnFamily::String,
                )),
            );
        }
    }

    #[test]
    fn eval_float_function_tuple_index() {
        let plan = plan();
        let mut frame = Frame::default();
        let tuple = TupleExpr::value(
            vec![Expr::function(FunctionExpr::float(function_value()))],
            vec![ValueType::Function(Box::new(type_()))],
        );

        assert_eq!(
            eval_float_function_expr(
                &plan,
                &mut frame,
                &FloatFunctionExpr::tuple_index(tuple, 0, type_()),
            )
            .expect("expression should evaluate")
            .runtime_id(),
            FloatFunctionId(0),
        );

        let mismatch_type = FunctionType::new(Vec::new(), ValueType::String);
        let tuple = TupleExpr::value(
            vec![Expr::function(FunctionExpr::string(
                StringFunctionExpr::value(crate::plan::StringFunctionValue::new(
                    crate::plan::StringFunctionId(0),
                    Vec::new(),
                )),
            ))],
            vec![ValueType::Function(Box::new(mismatch_type.clone()))],
        );

        assert_eq!(
            eval_float_function_expr(
                &plan,
                &mut frame,
                &FloatFunctionExpr::tuple_index(tuple, 0, type_()),
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
            eval_float_function_expr(
                &plan,
                &mut frame,
                &FloatFunctionExpr::tuple_index(tuple, 0, type_()),
            ),
            Err(ExecutionError::tuple_index_family_mismatch(
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
            vec![
                FunctionPlan::new(
                    FunctionId::new(1),
                    "float_value".into(),
                    Vec::new(),
                    Vec::new(),
                    ReturnExpr::float(FloatFunctionId(0), FloatExpr::value(1.0)),
                ),
                FunctionPlan::new(
                    FunctionId::new(2),
                    "get_float_value".into(),
                    Vec::new(),
                    Vec::new(),
                    ReturnExpr::float_function(FloatFunctionFunctionId(0), function_value()),
                ),
            ],
        )
    }

    fn function_return_family_error_value(
        expected: FunctionReturnFamily,
        actual: FunctionReturnFamily,
    ) -> ExecutionError {
        ExecutionError::function_return_family_mismatch(expected, actual)
    }

    fn error_bool_expr() -> BoolExpr {
        BoolExpr::function_call(
            BoolFunctionExpr::function_call(
                function_function_expr(),
                Vec::new(),
                FunctionType::new(Vec::new(), ValueType::Bool),
            ),
            Vec::new(),
        )
    }

    fn error_int_expr() -> IntExpr {
        IntExpr::function_call(
            IntFunctionExpr::function_call(
                function_function_expr(),
                Vec::new(),
                FunctionType::new(Vec::new(), ValueType::Int),
            ),
            Vec::new(),
        )
    }

    fn error_string_expr() -> StringExpr {
        StringExpr::function_call(
            StringFunctionExpr::function_call(
                FunctionFunctionExpr::value(FunctionFunctionValue::new(
                    FunctionFunctionId::Int(crate::plan::IntFunctionFunctionId(0)),
                    Vec::new(),
                    FunctionType::new(Vec::new(), ValueType::Int),
                )),
                Vec::new(),
                FunctionType::new(Vec::new(), ValueType::String),
            ),
            Vec::new(),
        )
    }

    fn error_float_expr() -> FloatExpr {
        FloatExpr::function_call(
            FloatFunctionExpr::function_call(
                function_function_expr(),
                Vec::new(),
                FunctionType::new(Vec::new(), ValueType::Float),
            ),
            Vec::new(),
        )
    }

    fn function_function_expr() -> FunctionFunctionExpr {
        FunctionFunctionExpr::value(FunctionFunctionValue::new(
            FunctionFunctionId::String(StringFunctionFunctionId(0)),
            Vec::new(),
            FunctionType::new(Vec::new(), ValueType::String),
        ))
    }

    fn function_value() -> FloatFunctionExpr {
        FloatFunctionExpr::value(function_runtime_value())
    }

    fn other_value() -> FloatFunctionExpr {
        FloatFunctionExpr::value(FloatFunctionValue::new(
            FloatFunctionId(1),
            vec![ParamLocal::float(crate::plan::FloatLocalId(0))],
        ))
    }

    fn function_runtime_value() -> FloatFunctionValue {
        FloatFunctionValue::new(
            FloatFunctionId(0),
            vec![ParamLocal::float(crate::plan::FloatLocalId(0))],
        )
    }

    fn type_() -> FunctionType {
        FunctionType::new(vec![ValueType::Float], ValueType::Float)
    }
}
