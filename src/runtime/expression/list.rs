use super::{eval_bool_expr, eval_float_expr, eval_int_expr, eval_string_expr, project_tuple_expr};
use crate::plan::{ExecutionPlan, ListExpr, ListExprKind, ListValue, Value, ValueType};
use crate::runtime::ExecutionError;
use crate::runtime::frame::Frame;
use crate::runtime::function;

pub(in crate::runtime) fn eval_list_expr(
    plan: &ExecutionPlan,
    frame: &mut Frame,
    expression: &ListExpr,
) -> Result<ListValue, ExecutionError> {
    match expression.kind() {
        ListExprKind::Value(elements) => {
            let mut values = Vec::with_capacity(elements.len());
            for element in elements {
                values.push(super::eval_expr(plan, frame, element)?);
            }
            Ok(ListValue::new(expression.element_type().clone(), values))
        }
        ListExprKind::LocalGet { local, .. } => Ok(frame.get_list(*local)),
        ListExprKind::Call { function, args } => {
            function::run_list_call(plan, *function, args, frame)
        }
        ListExprKind::FunctionCall { function, args } => {
            function::run_list_function_call(plan, function, args, frame)
        }
        ListExprKind::TupleIndex { tuple, index } => {
            let expected = ValueType::List(Box::new(expression.element_type().clone()));
            match project_tuple_expr(plan, frame, tuple, *index, expected.clone())? {
                Value::List(value) => Ok(value),
                other => Err(ExecutionError::tuple_index_family_mismatch(
                    expected,
                    other.value_type(),
                )),
            }
        }
        ListExprKind::BoolCase {
            subject,
            true_,
            false_,
        } => {
            if eval_bool_expr(plan, frame, subject)? {
                eval_list_expr(plan, frame, true_)
            } else {
                eval_list_expr(plan, frame, false_)
            }
        }
        ListExprKind::IntCase {
            subject,
            clauses,
            fallback,
        } => {
            let subject = eval_int_expr(plan, frame, subject)?;
            for (pattern, branch) in clauses {
                if pattern == &subject {
                    return eval_list_expr(plan, frame, branch);
                }
            }
            eval_list_expr(plan, frame, fallback)
        }
        ListExprKind::StringCase {
            subject,
            clauses,
            fallback,
        } => {
            let subject = eval_string_expr(plan, frame, subject)?;
            for (pattern, branch) in clauses {
                if pattern == &subject {
                    return eval_list_expr(plan, frame, branch);
                }
            }
            eval_list_expr(plan, frame, fallback)
        }
        ListExprKind::FloatCase {
            subject,
            clauses,
            fallback,
        } => {
            let subject = eval_float_expr(plan, frame, subject)?;
            for (pattern, branch) in clauses {
                if pattern == &subject {
                    return eval_list_expr(plan, frame, branch);
                }
            }
            eval_list_expr(plan, frame, fallback)
        }
        ListExprKind::Block { steps, return_ } => {
            function::execute_steps(plan, steps, frame)?;
            eval_list_expr(plan, frame, return_)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::eval_list_expr;
    use crate::plan::{
        BoolExpr, ExecutionPlan, Expr, FloatExpr, FrameLayout, FunctionId, FunctionPlan, IntExpr,
        IntFunctionId, ListExpr, ListFunctionId, ListLocalId, ListValue, ReturnBody, ReturnExpr,
        Step, StringExpr, TupleExpr, Value, ValueType,
    };
    use crate::runtime::ExecutionError;
    use crate::runtime::frame::Frame;
    use num_bigint::BigInt;

    #[test]
    fn eval_list_expr_direct_case_and_block_paths() {
        let plan = plan();
        let mut frame = frame();
        frame.set_list(ListLocalId(0), list_value(1));

        assert_eq!(
            eval_list_expr(&plan, &mut frame, &list_expr(1)),
            Ok(list_value(1)),
        );
        assert_eq!(
            eval_list_expr(
                &plan,
                &mut frame,
                &ListExpr::local_get(ListLocalId(0), "values".into(), element_type()),
            ),
            Ok(list_value(1)),
        );
        assert_eq!(
            eval_list_expr(
                &plan,
                &mut frame,
                &ListExpr::call(ListFunctionId(0), Vec::new(), element_type()),
            ),
            Ok(list_value(1)),
        );
        assert_eq!(
            eval_list_expr(
                &plan,
                &mut frame,
                &ListExpr::bool_case(BoolExpr::value(true), list_expr(1), list_expr(2),),
            ),
            Ok(list_value(1)),
        );
        assert_eq!(
            eval_list_expr(
                &plan,
                &mut frame,
                &ListExpr::bool_case(BoolExpr::value(false), list_expr(2), list_expr(1),),
            ),
            Ok(list_value(1)),
        );
        assert_eq!(
            eval_list_expr(
                &plan,
                &mut frame,
                &ListExpr::int_case(
                    IntExpr::value(1.into()),
                    vec![(1.into(), list_expr(1))],
                    list_expr(2),
                ),
            ),
            Ok(list_value(1)),
        );
        assert_eq!(
            eval_list_expr(
                &plan,
                &mut frame,
                &ListExpr::int_case(
                    IntExpr::value(2.into()),
                    vec![(1.into(), list_expr(2))],
                    list_expr(1),
                ),
            ),
            Ok(list_value(1)),
        );
        assert_eq!(
            eval_list_expr(
                &plan,
                &mut frame,
                &ListExpr::string_case(
                    StringExpr::value("hit".into()),
                    vec![("hit".into(), list_expr(1))],
                    list_expr(2),
                ),
            ),
            Ok(list_value(1)),
        );
        assert_eq!(
            eval_list_expr(
                &plan,
                &mut frame,
                &ListExpr::string_case(
                    StringExpr::value("miss".into()),
                    vec![("hit".into(), list_expr(2))],
                    list_expr(1),
                ),
            ),
            Ok(list_value(1)),
        );
        assert_eq!(
            eval_list_expr(
                &plan,
                &mut frame,
                &ListExpr::float_case(
                    FloatExpr::value(1.0),
                    vec![(1.0, list_expr(1))],
                    list_expr(2),
                ),
            ),
            Ok(list_value(1)),
        );
        assert_eq!(
            eval_list_expr(
                &plan,
                &mut frame,
                &ListExpr::float_case(
                    FloatExpr::value(2.0),
                    vec![(1.0, list_expr(2))],
                    list_expr(1),
                ),
            ),
            Ok(list_value(1)),
        );
        assert_eq!(
            eval_list_expr(
                &plan,
                &mut frame,
                &ListExpr::block(
                    vec![Step::evaluate(Expr::int(IntExpr::value(0.into())))],
                    list_expr(1),
                ),
            ),
            Ok(list_value(1)),
        );
    }

    #[test]
    fn eval_list_tuple_index_paths() {
        let plan = plan();
        let mut frame = Frame::default();
        let tuple = TupleExpr::value(
            vec![Expr::list(list_expr(1))],
            vec![ValueType::List(Box::new(element_type()))],
        );

        assert_eq!(
            eval_list_expr(
                &plan,
                &mut frame,
                &ListExpr::tuple_index(tuple, 0, element_type()),
            ),
            Ok(list_value(1)),
        );

        let tuple = TupleExpr::value(
            vec![Expr::int(IntExpr::value(1.into()))],
            vec![ValueType::Int],
        );
        assert_eq!(
            eval_list_expr(
                &plan,
                &mut frame,
                &ListExpr::tuple_index(tuple, 0, element_type()),
            ),
            Err(ExecutionError::tuple_index_family_mismatch(
                ValueType::List(Box::new(element_type())),
                ValueType::Int,
            )),
        );
    }

    #[test]
    fn eval_list_expr_propagates_operand_errors() {
        let plan = plan();
        let mut frame = Frame::default();

        for (expression, expected) in [
            (
                ListExpr::value(vec![Expr::int(error_int_expr())], element_type()),
                ValueType::Int,
            ),
            (
                ListExpr::bool_case(error_bool_expr(), list_expr(1), list_expr(2)),
                ValueType::Bool,
            ),
            (
                ListExpr::int_case(
                    error_int_expr(),
                    vec![(1.into(), list_expr(1))],
                    list_expr(2),
                ),
                ValueType::Int,
            ),
            (
                ListExpr::string_case(
                    error_string_expr(),
                    vec![("hit".into(), list_expr(1))],
                    list_expr(2),
                ),
                ValueType::String,
            ),
            (
                ListExpr::float_case(error_float_expr(), vec![(1.0, list_expr(1))], list_expr(2)),
                ValueType::Float,
            ),
            (
                ListExpr::block(
                    vec![Step::evaluate(Expr::bool(error_bool_expr()))],
                    list_expr(1),
                ),
                ValueType::Bool,
            ),
        ] {
            assert_eq!(
                eval_list_expr(&plan, &mut frame, &expression),
                Err(tuple_index_error(expected)),
            );
        }

        assert_eq!(
            eval_list_expr(&plan, &mut frame, &error_list_expr()),
            Err(tuple_index_error(ValueType::List(Box::new(element_type())))),
        );
    }

    fn error_int_expr() -> IntExpr {
        IntExpr::tuple_index(empty_tuple(), 0)
    }

    fn error_bool_expr() -> BoolExpr {
        BoolExpr::tuple_index(empty_tuple(), 0)
    }

    fn error_string_expr() -> StringExpr {
        StringExpr::tuple_index(empty_tuple(), 0)
    }

    fn error_float_expr() -> FloatExpr {
        FloatExpr::tuple_index(empty_tuple(), 0)
    }

    fn error_list_expr() -> ListExpr {
        ListExpr::tuple_index(empty_tuple(), 0, element_type())
    }

    fn empty_tuple() -> TupleExpr {
        TupleExpr::value(Vec::new(), Vec::new())
    }

    fn tuple_index_error(expected: ValueType) -> ExecutionError {
        ExecutionError::tuple_index_family_mismatch(expected, ValueType::Tuple(Vec::new()))
    }

    fn list_expr(value: i64) -> ListExpr {
        ListExpr::value(
            vec![Expr::int(IntExpr::value(BigInt::from(value)))],
            element_type(),
        )
    }

    fn list_value(value: i64) -> ListValue {
        ListValue::new(element_type(), vec![Value::Int(BigInt::from(value))])
    }

    fn element_type() -> ValueType {
        ValueType::Int
    }

    fn frame() -> Frame {
        let mut layout = FrameLayout::default();
        layout.include_list(ListLocalId(0));
        Frame::new(layout)
    }

    fn plan() -> ExecutionPlan {
        ExecutionPlan::new(
            "main".into(),
            FunctionPlan::new(
                FunctionId::new(0),
                "main".into(),
                Vec::new(),
                Vec::new(),
                ReturnExpr::int(IntFunctionId(0), IntExpr::value(0.into())),
            ),
            vec![FunctionPlan::new(
                FunctionId::new(1),
                "list_value".into(),
                Vec::new(),
                Vec::new(),
                ReturnExpr::list_body(
                    ListFunctionId(0),
                    element_type(),
                    ReturnBody::expr(list_expr(1)),
                ),
            )],
        )
    }
}
