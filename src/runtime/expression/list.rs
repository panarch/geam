use super::{
    eval_bool_expr, eval_float_expr, eval_function_expr, eval_int_expr, eval_nil_expr,
    eval_panic_expr, eval_string_expr, eval_tuple_expr, project_tuple_expr,
};
use crate::plan::{
    ExecutionPlan, FunctionType, FunctionValue, ListElements, ListExpr, ListExprKind, ListValue,
    Value, ValueType,
};
use crate::runtime::ExecutionError;
use crate::runtime::frame::Frame;
use crate::runtime::function;
use ecow::EcoString;
use num_bigint::BigInt;

pub(in crate::runtime) fn eval_list_expr(
    plan: &ExecutionPlan,
    frame: &mut Frame,
    expression: &ListExpr,
) -> Result<ListValue, ExecutionError> {
    match expression.kind() {
        ListExprKind::Value(elements) => eval_list_elements(plan, frame, elements),
        ListExprKind::Spread { elements, tail } => {
            let mut values = eval_list_elements(plan, frame, elements)?;
            let tail = eval_list_expr(plan, frame, tail)?;
            values.append(&tail).map_err(|error| {
                ExecutionError::list_item_type_mismatch(error.expected, error.actual)
            })?;
            Ok(values)
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
        ListExprKind::ListIndex { list, index } => {
            project_list_list_expr(plan, frame, list, *index, expression.element_type())
        }
        ListExprKind::DropFirst { list, count } => {
            let list = eval_list_expr(plan, frame, list)?;
            Ok(list.drop_first(*count))
        }
        ListExprKind::Panic(panic) => eval_panic_expr(plan, frame, panic),
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

fn eval_list_elements(
    plan: &ExecutionPlan,
    frame: &mut Frame,
    elements: &ListElements,
) -> Result<ListValue, ExecutionError> {
    match elements {
        ListElements::Int(elements) => elements
            .iter()
            .map(|element| eval_int_expr(plan, frame, element))
            .collect::<Result<Vec<_>, _>>()
            .map(ListValue::int),
        ListElements::String(elements) => elements
            .iter()
            .map(|element| eval_string_expr(plan, frame, element))
            .collect::<Result<Vec<_>, _>>()
            .map(ListValue::string),
        ListElements::Float(elements) => elements
            .iter()
            .map(|element| eval_float_expr(plan, frame, element))
            .collect::<Result<Vec<_>, _>>()
            .map(ListValue::float),
        ListElements::Bool(elements) => elements
            .iter()
            .map(|element| eval_bool_expr(plan, frame, element))
            .collect::<Result<Vec<_>, _>>()
            .map(ListValue::bool),
        ListElements::Nil(elements) => {
            for element in elements {
                eval_nil_expr(plan, frame, element)?;
            }
            Ok(ListValue::nil(elements.len()))
        }
        ListElements::Tuple { item_type, values } => values
            .iter()
            .map(|element| eval_tuple_expr(plan, frame, element))
            .collect::<Result<Vec<_>, _>>()
            .map(|values| ListValue::tuple(item_type.clone(), values)),
        ListElements::List { item_type, values } => values
            .iter()
            .map(|element| eval_list_expr(plan, frame, element))
            .collect::<Result<Vec<_>, _>>()
            .map(|values| ListValue::list(item_type.as_ref().clone(), values)),
        ListElements::Function { item_type, values } => values
            .iter()
            .map(|element| eval_function_expr(plan, frame, element))
            .collect::<Result<Vec<_>, _>>()
            .map(|values| ListValue::function(item_type.clone(), values)),
    }
}

fn eval_projected_list(
    plan: &ExecutionPlan,
    frame: &mut Frame,
    list: &ListExpr,
) -> Result<ListValue, ExecutionError> {
    eval_list_expr(plan, frame, list)
}

pub(in crate::runtime) fn project_int_list_expr(
    plan: &ExecutionPlan,
    frame: &mut Frame,
    list: &ListExpr,
    index: usize,
) -> Result<BigInt, ExecutionError> {
    let list = eval_list_expr(plan, frame, list)?;
    let actual = list.item_type();
    let values = list
        .int_values()
        .ok_or_else(|| ExecutionError::list_item_type_mismatch(ValueType::Int, actual))?;
    values.get(index).cloned().ok_or_else(|| {
        ExecutionError::list_index_out_of_bounds(ValueType::Int, index, values.len())
    })
}

pub(in crate::runtime) fn project_string_list_expr(
    plan: &ExecutionPlan,
    frame: &mut Frame,
    list: &ListExpr,
    index: usize,
) -> Result<EcoString, ExecutionError> {
    let list = eval_projected_list(plan, frame, list)?;
    let actual = list.item_type();
    let values = list
        .string_values()
        .ok_or_else(|| ExecutionError::list_item_type_mismatch(ValueType::String, actual))?;
    values.get(index).cloned().ok_or_else(|| {
        ExecutionError::list_index_out_of_bounds(ValueType::String, index, values.len())
    })
}

pub(in crate::runtime) fn project_float_list_expr(
    plan: &ExecutionPlan,
    frame: &mut Frame,
    list: &ListExpr,
    index: usize,
) -> Result<f64, ExecutionError> {
    let list = eval_projected_list(plan, frame, list)?;
    let actual = list.item_type();
    let values = list
        .float_values()
        .ok_or_else(|| ExecutionError::list_item_type_mismatch(ValueType::Float, actual))?;
    values.get(index).copied().ok_or_else(|| {
        ExecutionError::list_index_out_of_bounds(ValueType::Float, index, values.len())
    })
}

pub(in crate::runtime) fn project_bool_list_expr(
    plan: &ExecutionPlan,
    frame: &mut Frame,
    list: &ListExpr,
    index: usize,
) -> Result<bool, ExecutionError> {
    let list = eval_projected_list(plan, frame, list)?;
    let actual = list.item_type();
    let values = list
        .bool_values()
        .ok_or_else(|| ExecutionError::list_item_type_mismatch(ValueType::Bool, actual))?;
    values.get(index).copied().ok_or_else(|| {
        ExecutionError::list_index_out_of_bounds(ValueType::Bool, index, values.len())
    })
}

pub(in crate::runtime) fn project_nil_list_expr(
    plan: &ExecutionPlan,
    frame: &mut Frame,
    list: &ListExpr,
    index: usize,
) -> Result<(), ExecutionError> {
    let list = eval_projected_list(plan, frame, list)?;
    let actual = list.item_type();
    let len = list
        .nil_len()
        .ok_or_else(|| ExecutionError::list_item_type_mismatch(ValueType::Nil, actual))?;
    if index < len {
        Ok(())
    } else {
        Err(ExecutionError::list_index_out_of_bounds(
            ValueType::Nil,
            index,
            len,
        ))
    }
}

pub(in crate::runtime) fn project_tuple_list_expr(
    plan: &ExecutionPlan,
    frame: &mut Frame,
    list: &ListExpr,
    index: usize,
    item_type: &[ValueType],
) -> Result<Vec<Value>, ExecutionError> {
    let list = eval_projected_list(plan, frame, list)?;
    let actual = list.item_type();
    let values = list.tuple_values(item_type).ok_or_else(|| {
        ExecutionError::list_item_type_mismatch(ValueType::Tuple(item_type.to_vec()), actual)
    })?;
    values.get(index).cloned().ok_or_else(|| {
        ExecutionError::list_index_out_of_bounds(
            ValueType::Tuple(item_type.to_vec()),
            index,
            values.len(),
        )
    })
}

pub(in crate::runtime) fn project_list_list_expr(
    plan: &ExecutionPlan,
    frame: &mut Frame,
    list: &ListExpr,
    index: usize,
    item_type: &ValueType,
) -> Result<ListValue, ExecutionError> {
    let list = eval_projected_list(plan, frame, list)?;
    let actual = list.item_type();
    let values = list.list_values(item_type).ok_or_else(|| {
        ExecutionError::list_item_type_mismatch(
            ValueType::List(Box::new(item_type.clone())),
            actual,
        )
    })?;
    values.get(index).cloned().ok_or_else(|| {
        ExecutionError::list_index_out_of_bounds(
            ValueType::List(Box::new(item_type.clone())),
            index,
            values.len(),
        )
    })
}

pub(in crate::runtime) fn project_function_list_expr(
    plan: &ExecutionPlan,
    frame: &mut Frame,
    list: &ListExpr,
    index: usize,
    item_type: &FunctionType,
) -> Result<FunctionValue, ExecutionError> {
    let list = eval_projected_list(plan, frame, list)?;
    let actual = list.item_type();
    let values = list.function_values(item_type).ok_or_else(|| {
        ExecutionError::list_item_type_mismatch(
            ValueType::Function(Box::new(item_type.clone())),
            actual,
        )
    })?;
    values.get(index).cloned().ok_or_else(|| {
        ExecutionError::list_index_out_of_bounds(
            ValueType::Function(Box::new(item_type.clone())),
            index,
            values.len(),
        )
    })
}

#[cfg(test)]
mod tests {
    use super::{
        eval_list_expr, project_bool_list_expr, project_float_list_expr,
        project_function_list_expr, project_int_list_expr, project_list_list_expr,
        project_nil_list_expr, project_string_list_expr, project_tuple_list_expr,
    };
    use crate::plan::{
        BoolExpr, ExecutionPlan, Expr, FloatExpr, FrameLayout, FunctionExpr, FunctionId,
        FunctionPlan, FunctionType, IntExpr, IntFunctionExpr, IntFunctionId, IntFunctionValue,
        ListExpr, ListFunctionId, ListLocalId, ListValue, NilExpr, PanicExpr, PanicSite,
        ReturnBody, ReturnExpr, Step, StringExpr, TupleExpr, ValueType,
    };
    use crate::runtime::frame::Frame;
    use crate::runtime::{ExecutionError, PanicKind};
    use num_bigint::BigInt;

    #[test]
    fn eval_list_panic_returns_error() {
        let plan = plan();
        let mut frame = Frame::default();

        assert_eq!(
            eval_list_expr(
                &plan,
                &mut frame,
                &ListExpr::panic(
                    PanicExpr::panic_at(None, PanicSite::unknown()),
                    element_type()
                ),
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
    fn eval_list_expr_evaluates_value_local_and_direct_call() {
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
    }

    #[test]
    fn eval_list_expr_evaluates_spread_prefix_before_tail() {
        let plan = plan();
        let mut default_frame = Frame::default();

        assert_eq!(
            eval_list_expr(
                &plan,
                &mut default_frame,
                &ListExpr::spread(
                    vec![Expr::int(IntExpr::value(0.into()))],
                    list_expr(1),
                    element_type(),
                ),
            ),
            Ok(ListValue::int(vec![0.into(), 1.into()])),
        );
        assert_eq!(
            eval_list_expr(
                &plan,
                &mut default_frame,
                &ListExpr::spread(
                    vec![Expr::int(error_int_expr())],
                    error_list_expr(),
                    element_type(),
                ),
            ),
            Err(tuple_index_error(ValueType::Int)),
        );

        let mut frame = frame();
        frame.set_list(ListLocalId(0), ListValue::string(vec!["tail".into()]));
        assert_eq!(
            eval_list_expr(
                &plan,
                &mut frame,
                &ListExpr::spread(
                    vec![Expr::int(IntExpr::value(0.into()))],
                    ListExpr::local_get(ListLocalId(0), "tail".into(), element_type()),
                    element_type(),
                ),
            ),
            Err(ExecutionError::list_item_type_mismatch(
                ValueType::Int,
                ValueType::String,
            )),
        );
    }

    #[test]
    fn eval_list_expr_selects_case_branches() {
        let plan = plan();
        let mut frame = Frame::default();

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
    }

    #[test]
    fn eval_list_expr_executes_block_steps_before_return() {
        let plan = plan();
        let mut frame = frame();

        assert_eq!(
            eval_list_expr(
                &plan,
                &mut frame,
                &ListExpr::block(
                    vec![Step::let_list(
                        ListLocalId(0),
                        "values".into(),
                        list_expr(1)
                    )],
                    ListExpr::local_get(ListLocalId(0), "values".into(), element_type()),
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
    fn eval_list_index_and_drop_first_paths() {
        let plan = plan();
        let mut frame = Frame::default();
        let nested_list = ListExpr::value(
            vec![Expr::list(list_expr(1)), Expr::list(list_expr(2))],
            ValueType::List(Box::new(element_type())),
        );

        assert_eq!(
            eval_list_expr(
                &plan,
                &mut frame,
                &ListExpr::list_index(nested_list.clone(), 1, element_type()),
            ),
            Ok(list_value(2)),
        );
        assert_eq!(
            eval_list_expr(
                &plan,
                &mut frame,
                &ListExpr::list_index(nested_list.clone(), 2, element_type()),
            ),
            Err(ExecutionError::list_index_out_of_bounds(
                ValueType::List(Box::new(element_type())),
                2,
                2,
            )),
        );
        assert_eq!(
            eval_list_expr(
                &plan,
                &mut frame,
                &ListExpr::list_index(error_list_expr(), 0, element_type()),
            ),
            Err(tuple_index_error(ValueType::List(Box::new(element_type())))),
        );
        assert_eq!(
            eval_list_expr(
                &plan,
                &mut frame,
                &ListExpr::list_index(
                    ListExpr::value(vec![Expr::int(IntExpr::value(1.into()))], element_type()),
                    0,
                    element_type(),
                ),
            ),
            Err(ExecutionError::list_item_type_mismatch(
                ValueType::List(Box::new(element_type())),
                ValueType::Int,
            )),
        );
        assert_eq!(
            project_list_list_expr(&plan, &mut frame, &nested_list, 2, &element_type(),),
            Err(ExecutionError::list_index_out_of_bounds(
                ValueType::List(Box::new(element_type())),
                2,
                2,
            )),
        );
        assert_eq!(
            eval_list_expr(
                &plan,
                &mut frame,
                &ListExpr::drop_first(
                    ListExpr::value(
                        vec![
                            Expr::int(IntExpr::value(1.into())),
                            Expr::int(IntExpr::value(2.into())),
                        ],
                        element_type(),
                    ),
                    1,
                ),
            ),
            Ok(list_value(2)),
        );
        assert_eq!(
            eval_list_expr(
                &plan,
                &mut frame,
                &ListExpr::drop_first(
                    ListExpr::value(vec![Expr::int(IntExpr::value(1.into()))], element_type()),
                    2,
                ),
            ),
            Ok(ListValue::empty(element_type())),
        );
        assert_eq!(
            eval_list_expr(
                &plan,
                &mut frame,
                &ListExpr::drop_first(error_list_expr(), 1),
            ),
            Err(tuple_index_error(ValueType::List(Box::new(element_type())))),
        );
    }

    #[test]
    fn project_list_expr_out_of_bounds_for_each_item_family() {
        let plan = plan();
        let mut frame = Frame::default();
        let function_type = FunctionType::new(Vec::new(), ValueType::Int);

        assert_eq!(
            project_string_list_expr(
                &plan,
                &mut frame,
                &ListExpr::value(
                    vec![Expr::string(StringExpr::value("one".into()))],
                    ValueType::String,
                ),
                1,
            ),
            Err(ExecutionError::list_index_out_of_bounds(
                ValueType::String,
                1,
                1,
            )),
        );
        assert_eq!(
            project_float_list_expr(
                &plan,
                &mut frame,
                &ListExpr::value(vec![Expr::float(FloatExpr::value(1.5))], ValueType::Float),
                1,
            ),
            Err(ExecutionError::list_index_out_of_bounds(
                ValueType::Float,
                1,
                1,
            )),
        );
        assert_eq!(
            project_bool_list_expr(
                &plan,
                &mut frame,
                &ListExpr::value(vec![Expr::bool(BoolExpr::value(true))], ValueType::Bool),
                1,
            ),
            Err(ExecutionError::list_index_out_of_bounds(
                ValueType::Bool,
                1,
                1,
            )),
        );
        assert_eq!(
            project_nil_list_expr(
                &plan,
                &mut frame,
                &ListExpr::value(vec![Expr::nil(NilExpr::value())], ValueType::Nil),
                1,
            ),
            Err(ExecutionError::list_index_out_of_bounds(
                ValueType::Nil,
                1,
                1,
            )),
        );
        assert_eq!(
            project_tuple_list_expr(
                &plan,
                &mut frame,
                &ListExpr::value(
                    vec![Expr::tuple(TupleExpr::value(
                        vec![Expr::int(IntExpr::value(1.into()))],
                        vec![ValueType::Int],
                    ))],
                    ValueType::Tuple(vec![ValueType::Int]),
                ),
                1,
                &[ValueType::Int],
            ),
            Err(ExecutionError::list_index_out_of_bounds(
                ValueType::Tuple(vec![ValueType::Int]),
                1,
                1,
            )),
        );
        assert_eq!(
            project_list_list_expr(
                &plan,
                &mut frame,
                &ListExpr::value(
                    vec![Expr::list(list_expr(1))],
                    ValueType::List(Box::new(element_type())),
                ),
                1,
                &element_type(),
            ),
            Err(ExecutionError::list_index_out_of_bounds(
                ValueType::List(Box::new(element_type())),
                1,
                1,
            )),
        );
        assert_eq!(
            project_function_list_expr(
                &plan,
                &mut frame,
                &ListExpr::value(
                    vec![Expr::function(FunctionExpr::int(IntFunctionExpr::value(
                        IntFunctionValue::new(IntFunctionId(0), Vec::new()),
                    )))],
                    ValueType::Function(Box::new(function_type.clone())),
                ),
                1,
                &function_type,
            ),
            Err(ExecutionError::list_index_out_of_bounds(
                ValueType::Function(Box::new(function_type)),
                1,
                1,
            )),
        );
    }

    #[test]
    fn project_list_expr_propagates_list_evaluation_errors() {
        let plan = plan();
        let mut frame = Frame::default();
        let function_type = FunctionType::new(Vec::new(), ValueType::Int);

        assert_eq!(
            project_int_list_expr(&plan, &mut frame, &error_list_expr(), 0),
            Err(tuple_index_error(ValueType::List(Box::new(element_type())))),
        );
        assert_eq!(
            project_string_list_expr(&plan, &mut frame, &error_list_expr(), 0),
            Err(tuple_index_error(ValueType::List(Box::new(element_type())))),
        );
        assert_eq!(
            project_float_list_expr(&plan, &mut frame, &error_list_expr(), 0),
            Err(tuple_index_error(ValueType::List(Box::new(element_type())))),
        );
        assert_eq!(
            project_bool_list_expr(&plan, &mut frame, &error_list_expr(), 0),
            Err(tuple_index_error(ValueType::List(Box::new(element_type())))),
        );
        assert_eq!(
            project_nil_list_expr(&plan, &mut frame, &error_list_expr(), 0),
            Err(tuple_index_error(ValueType::List(Box::new(element_type())))),
        );
        assert_eq!(
            project_tuple_list_expr(&plan, &mut frame, &error_list_expr(), 0, &[ValueType::Int],),
            Err(tuple_index_error(ValueType::List(Box::new(element_type())))),
        );
        assert_eq!(
            project_list_list_expr(&plan, &mut frame, &error_list_expr(), 0, &element_type(),),
            Err(tuple_index_error(ValueType::List(Box::new(element_type())))),
        );
        assert_eq!(
            project_function_list_expr(&plan, &mut frame, &error_list_expr(), 0, &function_type,),
            Err(tuple_index_error(ValueType::List(Box::new(element_type())))),
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
                ListExpr::value(
                    vec![Expr::nil(NilExpr::tuple_index(empty_tuple(), 0))],
                    ValueType::Nil,
                ),
                ValueType::Nil,
            ),
            (
                ListExpr::spread(
                    vec![Expr::int(error_int_expr())],
                    list_expr(1),
                    element_type(),
                ),
                ValueType::Int,
            ),
            (
                ListExpr::spread(Vec::new(), error_list_expr(), element_type()),
                ValueType::List(Box::new(element_type())),
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
        ListValue::int(vec![BigInt::from(value)])
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
