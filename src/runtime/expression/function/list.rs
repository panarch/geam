use crate::plan::{
    ExecutionPlan, FunctionReturnFamily, FunctionValueKind, ListFunctionExpr, ListFunctionExprKind,
    ListFunctionValue, Value, ValueType,
};
use crate::runtime::ExecutionError;
use crate::runtime::expression::{
    eval_bool_expr, eval_float_expr, eval_int_expr, eval_panic_expr, eval_string_expr,
    project_function_list_expr, project_tuple_expr,
};
use crate::runtime::frame::Frame;
use crate::runtime::function;

pub(in crate::runtime) fn eval_list_function_expr(
    plan: &ExecutionPlan,
    frame: &mut Frame,
    expression: &ListFunctionExpr,
) -> Result<ListFunctionValue, ExecutionError> {
    match expression.kind() {
        ListFunctionExprKind::Value(value) => Ok(value.clone()),
        ListFunctionExprKind::Closure {
            runtime_id,
            params,
            captures,
        } => Ok(ListFunctionValue::new_with_captures(
            runtime_id.clone(),
            params.clone(),
            function::eval_capture_args(plan, frame, captures)?,
        )),
        ListFunctionExprKind::LocalGet { local, .. } => Ok(frame.get_list_function(local)),
        ListFunctionExprKind::Call { function, args, .. } => {
            function::run_list_function_returning_function_call(plan, function.clone(), args, frame)
        }
        ListFunctionExprKind::FunctionCall {
            function: callee,
            args,
            ..
        } => function::run_list_function_function_call(plan, callee, args, frame),
        ListFunctionExprKind::TupleIndex {
            tuple,
            index,
            type_,
        } => {
            match project_tuple_expr(
                plan,
                frame,
                tuple,
                *index,
                ValueType::Function(Box::new(type_.clone())),
            )? {
                Value::Function(function) => match function.kind() {
                    FunctionValueKind::List(value) => Ok(value.clone()),
                    _ => Err(ExecutionError::tuple_index_family_mismatch(
                        ValueType::Function(Box::new(type_.clone())),
                        Value::Function(function).value_type(),
                    )),
                },
                other => Err(ExecutionError::tuple_index_family_mismatch(
                    ValueType::Function(Box::new(type_.clone())),
                    other.value_type(),
                )),
            }
        }
        ListFunctionExprKind::ListIndex { list, index, type_ } => {
            let function = project_function_list_expr(plan, frame, list, *index, type_)?;
            match function.kind() {
                FunctionValueKind::List(value) => Ok(value.clone()),
                _ => Err(ExecutionError::function_return_family_mismatch(
                    FunctionReturnFamily::List,
                    function.kind().family(),
                )),
            }
        }
        ListFunctionExprKind::Panic(panic) => eval_panic_expr(plan, frame, panic),
        ListFunctionExprKind::BoolCase {
            subject,
            true_,
            false_,
        } => {
            if eval_bool_expr(plan, frame, subject)? {
                eval_list_function_expr(plan, frame, true_)
            } else {
                eval_list_function_expr(plan, frame, false_)
            }
        }
        ListFunctionExprKind::IntCase {
            subject,
            clauses,
            fallback,
        } => {
            let subject = eval_int_expr(plan, frame, subject)?;
            for (pattern, branch) in clauses {
                if pattern == &subject {
                    return eval_list_function_expr(plan, frame, branch);
                }
            }
            eval_list_function_expr(plan, frame, fallback)
        }
        ListFunctionExprKind::StringCase {
            subject,
            clauses,
            fallback,
        } => {
            let subject = eval_string_expr(plan, frame, subject)?;
            for (pattern, branch) in clauses {
                if pattern == &subject {
                    return eval_list_function_expr(plan, frame, branch);
                }
            }
            eval_list_function_expr(plan, frame, fallback)
        }
        ListFunctionExprKind::FloatCase {
            subject,
            clauses,
            fallback,
        } => {
            let subject = eval_float_expr(plan, frame, subject)?;
            for (pattern, branch) in clauses {
                if pattern == &subject {
                    return eval_list_function_expr(plan, frame, branch);
                }
            }
            eval_list_function_expr(plan, frame, fallback)
        }
        ListFunctionExprKind::Block { steps, return_ } => {
            function::execute_steps(plan, steps, frame)?;
            eval_list_function_expr(plan, frame, return_)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::eval_list_function_expr;
    use crate::plan::{
        BoolExpr, CaptureArg, ExecutionPlan, Expr, FloatExpr, FunctionExpr, FunctionFunctionExpr,
        FunctionFunctionId, FunctionFunctionValue, FunctionId, FunctionPlan, FunctionReturnFamily,
        FunctionType, IntExpr, IntFunctionExpr, IntFunctionId, IntFunctionValue, IntListLocalId,
        ListElements, ListExpr, ListFunctionExpr, ListFunctionFunctionId, ListFunctionId,
        ListFunctionValue, ListLocal, ListReturn, PanicExpr, PanicSite, ParamLocal, ReturnBody,
        ReturnExpr, Step, StringExpr, TupleExpr, ValueType,
    };
    use crate::runtime::frame::Frame;
    use crate::runtime::{ExecutionError, PanicKind};
    use num_bigint::BigInt;

    #[test]
    fn eval_list_function_direct_expression_paths() {
        let plan = plan();
        let mut frame = Frame::default();
        frame.set_list_function(
            crate::plan::ListFunctionLocal::from_item_type(
                0,
                list_function_type(),
                crate::plan::ValueType::Int,
            ),
            list_function_value(),
        );

        assert_eq!(
            eval_list_function_expr(
                &plan,
                &mut frame,
                &ListFunctionExpr::closure(
                    ListFunctionId::from_item_type(0, crate::plan::ValueType::Int),
                    vec![ParamLocal::int(crate::plan::IntLocalId(0))],
                    Vec::new()
                ),
            )
            .expect("expression should evaluate")
            .runtime_id(),
            ListFunctionId::from_item_type(0, crate::plan::ValueType::Int),
        );
        assert_eq!(
            eval_list_function_expr(
                &plan,
                &mut frame,
                &ListFunctionExpr::panic(
                    PanicExpr::panic_at(None, PanicSite::unknown()),
                    list_function_type(),
                    ValueType::Int,
                ),
            ),
            Err(ExecutionError::source_panic(
                None,
                PanicKind::Panic,
                None,
                PanicSite::unknown()
            )),
        );
        assert_eq!(
            eval_list_function_expr(
                &plan,
                &mut frame,
                &ListFunctionExpr::local_get(
                    crate::plan::ListFunctionLocal::from_item_type(
                        0,
                        list_function_type(),
                        crate::plan::ValueType::Int
                    ),
                    "make".into(),
                ),
            )
            .expect("expression should evaluate")
            .runtime_id(),
            ListFunctionId::from_item_type(0, crate::plan::ValueType::Int),
        );
        assert_eq!(
            eval_list_function_expr(
                &plan,
                &mut frame,
                &ListFunctionExpr::call(
                    ListFunctionFunctionId::from_item_type(
                        0,
                        list_function_type(),
                        crate::plan::ValueType::Int
                    ),
                    Vec::new(),
                ),
            )
            .expect("expression should evaluate")
            .runtime_id(),
            ListFunctionId::from_item_type(0, crate::plan::ValueType::Int),
        );
        assert_eq!(
            eval_list_function_expr(
                &plan,
                &mut frame,
                &ListFunctionExpr::function_call(
                    FunctionFunctionExpr::value(FunctionFunctionValue::new(
                        FunctionFunctionId::List(ListFunctionFunctionId::from_item_type(
                            0,
                            list_function_type(),
                            crate::plan::ValueType::Int,
                        )),
                        Vec::new(),
                        list_function_type(),
                    )),
                    Vec::new(),
                    list_function_type(),
                    ValueType::Int,
                ),
            )
            .expect("expression should evaluate")
            .runtime_id(),
            ListFunctionId::from_item_type(0, crate::plan::ValueType::Int),
        );
        assert_eq!(
            eval_list_function_expr(
                &plan,
                &mut frame,
                &ListFunctionExpr::tuple_index(
                    TupleExpr::value(
                        vec![Expr::function(FunctionExpr::list(list_function_expr()))],
                        vec![ValueType::Function(Box::new(list_function_type()))],
                    ),
                    0,
                    list_function_type(),
                    ValueType::Int,
                ),
            )
            .expect("expression should evaluate")
            .runtime_id(),
            ListFunctionId::from_item_type(0, crate::plan::ValueType::Int),
        );
        assert_eq!(
            eval_list_function_expr(
                &plan,
                &mut frame,
                &ListFunctionExpr::bool_case(
                    BoolExpr::value(true),
                    list_function_expr(),
                    other_list_function_expr(),
                ),
            )
            .expect("expression should evaluate")
            .runtime_id(),
            ListFunctionId::from_item_type(0, crate::plan::ValueType::Int),
        );
        assert_eq!(
            eval_list_function_expr(
                &plan,
                &mut frame,
                &ListFunctionExpr::bool_case(
                    BoolExpr::value(false),
                    other_list_function_expr(),
                    list_function_expr(),
                ),
            )
            .expect("expression should evaluate")
            .runtime_id(),
            ListFunctionId::from_item_type(0, crate::plan::ValueType::Int),
        );
        assert_eq!(
            eval_list_function_expr(
                &plan,
                &mut frame,
                &ListFunctionExpr::int_case(
                    IntExpr::value(1.into()),
                    vec![(1.into(), list_function_expr())],
                    other_list_function_expr(),
                ),
            )
            .expect("expression should evaluate")
            .runtime_id(),
            ListFunctionId::from_item_type(0, crate::plan::ValueType::Int),
        );
        assert_eq!(
            eval_list_function_expr(
                &plan,
                &mut frame,
                &ListFunctionExpr::int_case(
                    IntExpr::value(2.into()),
                    vec![(1.into(), other_list_function_expr())],
                    list_function_expr(),
                ),
            )
            .expect("expression should evaluate")
            .runtime_id(),
            ListFunctionId::from_item_type(0, crate::plan::ValueType::Int),
        );
        assert_eq!(
            eval_list_function_expr(
                &plan,
                &mut frame,
                &ListFunctionExpr::string_case(
                    StringExpr::value("hit".into()),
                    vec![("hit".into(), list_function_expr())],
                    other_list_function_expr(),
                ),
            )
            .expect("expression should evaluate")
            .runtime_id(),
            ListFunctionId::from_item_type(0, crate::plan::ValueType::Int),
        );
        assert_eq!(
            eval_list_function_expr(
                &plan,
                &mut frame,
                &ListFunctionExpr::string_case(
                    StringExpr::value("miss".into()),
                    vec![("hit".into(), other_list_function_expr())],
                    list_function_expr(),
                ),
            )
            .expect("expression should evaluate")
            .runtime_id(),
            ListFunctionId::from_item_type(0, crate::plan::ValueType::Int),
        );
        assert_eq!(
            eval_list_function_expr(
                &plan,
                &mut frame,
                &ListFunctionExpr::float_case(
                    FloatExpr::value(1.0),
                    vec![(1.0, list_function_expr())],
                    other_list_function_expr(),
                ),
            )
            .expect("expression should evaluate")
            .runtime_id(),
            ListFunctionId::from_item_type(0, crate::plan::ValueType::Int),
        );
        assert_eq!(
            eval_list_function_expr(
                &plan,
                &mut frame,
                &ListFunctionExpr::float_case(
                    FloatExpr::value(2.0),
                    vec![(1.0, other_list_function_expr())],
                    list_function_expr(),
                ),
            )
            .expect("expression should evaluate")
            .runtime_id(),
            ListFunctionId::from_item_type(0, crate::plan::ValueType::Int),
        );
        assert_eq!(
            eval_list_function_expr(
                &plan,
                &mut frame,
                &ListFunctionExpr::block(
                    vec![Step::evaluate(Expr::int(IntExpr::value(1.into())))],
                    list_function_expr(),
                ),
            )
            .expect("expression should evaluate")
            .runtime_id(),
            ListFunctionId::from_item_type(0, crate::plan::ValueType::Int),
        );
    }

    #[test]
    fn eval_list_function_expr_propagates_operand_errors() {
        assert_tuple_index_error(
            ValueType::List(Box::new(element_type())),
            ListFunctionExpr::closure(
                ListFunctionId::from_item_type(0, crate::plan::ValueType::Int),
                vec![ParamLocal::list(ListLocal::int(IntListLocalId(0)))],
                vec![CaptureArg::list(
                    crate::plan::ListLocalExpr::try_new(
                        ListLocal::int(IntListLocalId(0)),
                        error_list_expr(),
                    )
                    .expect("list capture should match local item type"),
                )],
            ),
        );
        assert_function_tuple_index_error(ListFunctionExpr::tuple_index(
            empty_tuple(),
            0,
            list_function_type(),
            ValueType::Int,
        ));
        assert_tuple_index_error(
            ValueType::Bool,
            ListFunctionExpr::bool_case(
                error_bool_expr(),
                list_function_expr(),
                other_list_function_expr(),
            ),
        );
        assert_tuple_index_error(
            ValueType::Int,
            ListFunctionExpr::int_case(
                error_int_expr(),
                vec![(1.into(), list_function_expr())],
                other_list_function_expr(),
            ),
        );
        assert_tuple_index_error(
            ValueType::String,
            ListFunctionExpr::string_case(
                error_string_expr(),
                vec![("hit".into(), list_function_expr())],
                other_list_function_expr(),
            ),
        );
        assert_tuple_index_error(
            ValueType::Float,
            ListFunctionExpr::float_case(
                error_float_expr(),
                vec![(1.0, list_function_expr())],
                other_list_function_expr(),
            ),
        );
        assert_tuple_index_error(
            ValueType::List(Box::new(element_type())),
            ListFunctionExpr::block(
                vec![Step::evaluate(Expr::list(error_list_expr()))],
                list_function_expr(),
            ),
        );
    }

    #[test]
    fn list_function_projection_invariant_error() {
        let plan = plan();
        let mut frame = Frame::default();
        let int_function_type = FunctionType::new(Vec::new(), ValueType::Int);
        let list_function_type = list_function_expr().type_().clone();
        let tuple = TupleExpr::value(
            vec![Expr::function(FunctionExpr::int(
                crate::plan::IntFunctionExpr::value(IntFunctionValue::new(
                    crate::plan::IntFunctionId(0),
                    Vec::new(),
                )),
            ))],
            vec![ValueType::Function(Box::new(int_function_type.clone()))],
        );

        assert_eq!(
            eval_list_function_expr(
                &plan,
                &mut frame,
                &ListFunctionExpr::tuple_index(
                    tuple,
                    0,
                    list_function_type.clone(),
                    ValueType::Int
                ),
            ),
            Err(ExecutionError::tuple_index_family_mismatch(
                ValueType::Function(Box::new(list_function_type.clone())),
                ValueType::Function(Box::new(int_function_type)),
            )),
        );

        let tuple = TupleExpr::value(
            vec![Expr::int(IntExpr::value(1.into()))],
            vec![ValueType::Int],
        );
        assert_eq!(
            eval_list_function_expr(
                &plan,
                &mut frame,
                &ListFunctionExpr::tuple_index(
                    tuple,
                    0,
                    list_function_type.clone(),
                    ValueType::Int
                ),
            ),
            Err(ExecutionError::tuple_index_family_mismatch(
                ValueType::Function(Box::new(list_function_type.clone())),
                ValueType::Int,
            )),
        );

        let tuple = TupleExpr::value(
            vec![Expr::int(IntExpr::value(1.into()))],
            vec![ValueType::Int],
        );
        assert_eq!(
            eval_list_function_expr(
                &plan,
                &mut frame,
                &ListFunctionExpr::tuple_index(
                    tuple,
                    1,
                    list_function_type.clone(),
                    ValueType::Int
                ),
            ),
            Err(ExecutionError::tuple_index_family_mismatch(
                ValueType::Function(Box::new(list_function_type)),
                ValueType::Tuple(vec![ValueType::Int]),
            )),
        );
    }

    #[test]
    fn list_function_list_projection() {
        let plan = plan();
        let mut frame = Frame::default();
        let list_function_type = list_function_expr().type_().clone();
        let list = ListExpr::value(
            vec![Expr::function(FunctionExpr::list(list_function_expr()))],
            ValueType::Function(Box::new(list_function_type.clone())),
        );
        assert_eq!(
            eval_list_function_expr(
                &plan,
                &mut frame,
                &ListFunctionExpr::list_index(list, 0, list_function_type.clone(), ValueType::Int),
            )
            .expect("expression should evaluate")
            .runtime_id(),
            ListFunctionId::from_item_type(0, crate::plan::ValueType::Int),
        );

        let list = ListExpr::from_elements(ListElements::Function {
            item_type: list_function_type.clone(),
            values: vec![FunctionExpr::int(IntFunctionExpr::value(
                IntFunctionValue::new(IntFunctionId(0), Vec::new()),
            ))],
        });
        assert_eq!(
            eval_list_function_expr(
                &plan,
                &mut frame,
                &ListFunctionExpr::list_index(list, 0, list_function_type.clone(), ValueType::Int),
            ),
            Err(ExecutionError::function_return_family_mismatch(
                FunctionReturnFamily::List,
                FunctionReturnFamily::Int,
            )),
        );

        let mut frame = Frame::default();
        let list = ListExpr::value(
            vec![Expr::function(FunctionExpr::list(list_function_expr()))],
            ValueType::Function(Box::new(list_function_type.clone())),
        );
        assert_eq!(
            eval_list_function_expr(
                &plan,
                &mut frame,
                &ListFunctionExpr::list_index(list, 1, list_function_type.clone(), ValueType::Int),
            ),
            Err(ExecutionError::list_index_out_of_bounds(
                ValueType::Function(Box::new(list_function_type.clone())),
                1,
                1,
            )),
        );

        let list = ListExpr::tuple_index(
            empty_tuple(),
            0,
            ValueType::Function(Box::new(list_function_type.clone())),
        );
        assert_eq!(
            eval_list_function_expr(
                &plan,
                &mut frame,
                &ListFunctionExpr::list_index(list, 0, list_function_type.clone(), ValueType::Int),
            ),
            Err(ExecutionError::tuple_index_family_mismatch(
                ValueType::List(Box::new(ValueType::Function(Box::new(
                    list_function_type.clone(),
                )))),
                ValueType::Tuple(Vec::new()),
            )),
        );
    }

    fn assert_tuple_index_error(expected: ValueType, expression: ListFunctionExpr) {
        let plan = plan();
        let mut frame = Frame::default();

        assert_eq!(
            eval_list_function_expr(&plan, &mut frame, &expression),
            Err(tuple_index_error(expected)),
        );
    }

    fn assert_function_tuple_index_error(expression: ListFunctionExpr) {
        let plan = plan();
        let mut frame = Frame::default();

        assert_eq!(
            eval_list_function_expr(&plan, &mut frame, &expression),
            Err(tuple_index_error(ValueType::Function(Box::new(
                list_function_type()
            )))),
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

    fn list_function_expr() -> ListFunctionExpr {
        ListFunctionExpr::value(list_function_value())
    }

    fn other_list_function_expr() -> ListFunctionExpr {
        ListFunctionExpr::value(ListFunctionValue::new(
            ListFunctionId::from_item_type(1, crate::plan::ValueType::Int),
            vec![ParamLocal::int(crate::plan::IntLocalId(0))],
        ))
    }

    fn list_function_value() -> ListFunctionValue {
        ListFunctionValue::new(
            ListFunctionId::from_item_type(0, crate::plan::ValueType::Int),
            vec![ParamLocal::int(crate::plan::IntLocalId(0))],
        )
    }

    fn list_function_type() -> FunctionType {
        FunctionType::new(
            vec![ValueType::Int],
            ValueType::List(Box::new(element_type())),
        )
    }

    fn list_expr(value: i64) -> ListExpr {
        ListExpr::value(
            vec![Expr::int(IntExpr::value(BigInt::from(value)))],
            element_type(),
        )
    }

    fn element_type() -> ValueType {
        ValueType::Int
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
            vec![
                FunctionPlan::new(
                    FunctionId::new(1),
                    "make".into(),
                    vec![crate::plan::Param::named(
                        ParamLocal::int(crate::plan::IntLocalId(0)),
                        "value".into(),
                    )],
                    Vec::new(),
                    ReturnExpr::list_body(
                        ListFunctionId::from_item_type(0, element_type()),
                        ListReturn::expr(list_expr(1)),
                    ),
                ),
                FunctionPlan::new(
                    FunctionId::new(2),
                    "get".into(),
                    Vec::new(),
                    Vec::new(),
                    ReturnExpr::list_function_body(
                        ListFunctionFunctionId::from_item_type(
                            0,
                            list_function_type(),
                            crate::plan::ValueType::Int,
                        ),
                        ReturnBody::expr(list_function_expr()),
                    ),
                ),
            ],
        )
    }
}
