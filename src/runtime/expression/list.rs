use super::{
    eval_bool_expr, eval_float_expr, eval_function_expr, eval_int_expr, eval_nil_expr,
    eval_panic_expr, eval_string_expr, eval_tuple_expr, project_tuple_expr,
};
use crate::plan::{
    BoolListExpr, ExecutionPlan, FloatListExpr, FunctionListExpr, FunctionType, FunctionValue,
    IntListExpr, ListElements, ListExpr, ListItem, ListListExpr, ListValue, NilListExpr,
    StringListExpr, TupleListExpr, TypedListExpr, TypedListExprKind, Value, ValueType,
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
    match expression {
        ListExpr::Int(expression) => eval_typed_list_expr(plan, frame, expression),
        ListExpr::String(expression) => eval_typed_list_expr(plan, frame, expression),
        ListExpr::Float(expression) => eval_typed_list_expr(plan, frame, expression),
        ListExpr::Bool(expression) => eval_typed_list_expr(plan, frame, expression),
        ListExpr::Nil(expression) => eval_typed_list_expr(plan, frame, expression),
        ListExpr::Tuple(expression) => eval_typed_list_expr(plan, frame, expression),
        ListExpr::List(expression) => eval_typed_list_expr(plan, frame, expression),
        ListExpr::Function(expression) => eval_typed_list_expr(plan, frame, expression),
    }
}

pub(in crate::runtime) fn eval_typed_list_expr<Item: ListItem>(
    plan: &ExecutionPlan,
    frame: &mut Frame,
    expression: &TypedListExpr<Item>,
) -> Result<ListValue, ExecutionError> {
    match expression.kind() {
        TypedListExprKind::Value(elements) => {
            eval_typed_list_elements(plan, frame, expression.item(), elements)
        }
        TypedListExprKind::Spread { elements, tail } => {
            let mut values = eval_typed_list_elements(plan, frame, expression.item(), elements)?;
            let tail = eval_typed_list_expr(plan, frame, tail)?;
            values.append(tail).map_err(|(expected, actual)| {
                ExecutionError::list_item_type_mismatch(expected, actual)
            })?;
            Ok(values)
        }
        TypedListExprKind::LocalGet { local, .. } => {
            frame.get_list(&expression.item().local_to_facade(local.clone()))
        }
        TypedListExprKind::Call { function, args } => function::run_list_call(
            plan,
            expression.item().function_to_facade(function.clone()),
            args,
            frame,
        ),
        TypedListExprKind::FunctionCall { function, args } => {
            function::run_list_function_call(plan, function, args, frame)
        }
        TypedListExprKind::TupleIndex { tuple, index } => {
            let expected = ValueType::List(Box::new(expression.element_type().clone()));
            match project_tuple_expr(plan, frame, tuple, *index, expected.clone())? {
                Value::List(value) => Ok(value),
                other => Err(ExecutionError::tuple_index_family_mismatch(
                    expected,
                    other.value_type(),
                )),
            }
        }
        TypedListExprKind::ListIndex { list, index } => {
            project_list_list_expr(plan, frame, list, *index, &expression.element_type())
        }
        TypedListExprKind::DropFirst { list, count } => {
            let list = eval_typed_list_expr(plan, frame, list)?;
            Ok(list.drop_first(*count))
        }
        TypedListExprKind::Panic(panic) => eval_panic_expr(plan, frame, panic),
        TypedListExprKind::BoolCase {
            subject,
            true_,
            false_,
        } => {
            if eval_bool_expr(plan, frame, subject)? {
                eval_typed_list_expr(plan, frame, true_)
            } else {
                eval_typed_list_expr(plan, frame, false_)
            }
        }
        TypedListExprKind::IntCase {
            subject,
            clauses,
            fallback,
        } => {
            let subject = eval_int_expr(plan, frame, subject)?;
            for (pattern, branch) in clauses {
                if pattern == &subject {
                    return eval_typed_list_expr(plan, frame, branch);
                }
            }
            eval_typed_list_expr(plan, frame, fallback)
        }
        TypedListExprKind::StringCase {
            subject,
            clauses,
            fallback,
        } => {
            let subject = eval_string_expr(plan, frame, subject)?;
            for (pattern, branch) in clauses {
                if pattern == &subject {
                    return eval_typed_list_expr(plan, frame, branch);
                }
            }
            eval_typed_list_expr(plan, frame, fallback)
        }
        TypedListExprKind::FloatCase {
            subject,
            clauses,
            fallback,
        } => {
            let subject = eval_float_expr(plan, frame, subject)?;
            for (pattern, branch) in clauses {
                if pattern == &subject {
                    return eval_typed_list_expr(plan, frame, branch);
                }
            }
            eval_typed_list_expr(plan, frame, fallback)
        }
        TypedListExprKind::Block { steps, return_ } => {
            function::execute_steps(plan, steps, frame)?;
            eval_typed_list_expr(plan, frame, return_)
        }
    }
}

fn eval_typed_list_elements<Item: ListItem>(
    plan: &ExecutionPlan,
    frame: &mut Frame,
    item: &Item,
    elements: &[Item::ElementExpr],
) -> Result<ListValue, ExecutionError> {
    let elements = Item::elements_to_facade(item.clone(), elements.to_vec());
    eval_list_elements(plan, frame, &elements)
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

fn eval_projected_list<Item: ListItem>(
    plan: &ExecutionPlan,
    frame: &mut Frame,
    list: &TypedListExpr<Item>,
) -> Result<ListValue, ExecutionError> {
    eval_typed_list_expr(plan, frame, list)
}

pub(in crate::runtime) fn project_int_list_expr(
    plan: &ExecutionPlan,
    frame: &mut Frame,
    list: &IntListExpr,
    index: usize,
) -> Result<BigInt, ExecutionError> {
    let list = eval_typed_list_expr(plan, frame, list)?;
    let actual = list.item_type();
    let values = list
        .into_int_values()
        .ok_or_else(|| ExecutionError::list_item_type_mismatch(ValueType::Int, actual))?;
    values.get(index).cloned().ok_or_else(|| {
        ExecutionError::list_index_out_of_bounds(ValueType::Int, index, values.len())
    })
}

pub(in crate::runtime) fn project_string_list_expr(
    plan: &ExecutionPlan,
    frame: &mut Frame,
    list: &StringListExpr,
    index: usize,
) -> Result<EcoString, ExecutionError> {
    let list = eval_projected_list(plan, frame, list)?;
    let actual = list.item_type();
    let values = list
        .into_string_values()
        .ok_or_else(|| ExecutionError::list_item_type_mismatch(ValueType::String, actual))?;
    values.get(index).cloned().ok_or_else(|| {
        ExecutionError::list_index_out_of_bounds(ValueType::String, index, values.len())
    })
}

pub(in crate::runtime) fn project_float_list_expr(
    plan: &ExecutionPlan,
    frame: &mut Frame,
    list: &FloatListExpr,
    index: usize,
) -> Result<f64, ExecutionError> {
    let list = eval_projected_list(plan, frame, list)?;
    let actual = list.item_type();
    let values = list
        .into_float_values()
        .ok_or_else(|| ExecutionError::list_item_type_mismatch(ValueType::Float, actual))?;
    values.get(index).copied().ok_or_else(|| {
        ExecutionError::list_index_out_of_bounds(ValueType::Float, index, values.len())
    })
}

pub(in crate::runtime) fn project_bool_list_expr(
    plan: &ExecutionPlan,
    frame: &mut Frame,
    list: &BoolListExpr,
    index: usize,
) -> Result<bool, ExecutionError> {
    let list = eval_projected_list(plan, frame, list)?;
    let actual = list.item_type();
    let values = list
        .into_bool_values()
        .ok_or_else(|| ExecutionError::list_item_type_mismatch(ValueType::Bool, actual))?;
    values.get(index).copied().ok_or_else(|| {
        ExecutionError::list_index_out_of_bounds(ValueType::Bool, index, values.len())
    })
}

pub(in crate::runtime) fn project_nil_list_expr(
    plan: &ExecutionPlan,
    frame: &mut Frame,
    list: &NilListExpr,
    index: usize,
) -> Result<(), ExecutionError> {
    let list = eval_projected_list(plan, frame, list)?;
    let actual = list.item_type();
    let len = list
        .into_nil_len()
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
    list: &TupleListExpr,
    index: usize,
    item_type: &[ValueType],
) -> Result<Vec<Value>, ExecutionError> {
    let list = eval_projected_list(plan, frame, list)?;
    let expected = ValueType::Tuple(item_type.to_vec());
    let actual = list.item_type();
    let values = list
        .into_tuple_values(item_type)
        .ok_or_else(|| ExecutionError::list_item_type_mismatch(expected.clone(), actual))?;
    values
        .get(index)
        .cloned()
        .ok_or_else(|| ExecutionError::list_index_out_of_bounds(expected, index, values.len()))
}

pub(in crate::runtime) fn project_list_list_expr(
    plan: &ExecutionPlan,
    frame: &mut Frame,
    list: &ListListExpr,
    index: usize,
    item_type: &ValueType,
) -> Result<ListValue, ExecutionError> {
    let list = eval_projected_list(plan, frame, list)?;
    let expected = ValueType::List(Box::new(item_type.clone()));
    let actual = list.item_type();
    let values = list
        .into_list_values(item_type)
        .ok_or_else(|| ExecutionError::list_item_type_mismatch(expected.clone(), actual))?;
    values
        .get(index)
        .cloned()
        .ok_or_else(|| ExecutionError::list_index_out_of_bounds(expected, index, values.len()))
}

pub(in crate::runtime) fn project_function_list_expr(
    plan: &ExecutionPlan,
    frame: &mut Frame,
    list: &FunctionListExpr,
    index: usize,
    item_type: &FunctionType,
) -> Result<FunctionValue, ExecutionError> {
    let list = eval_projected_list(plan, frame, list)?;
    let expected = ValueType::Function(Box::new(item_type.clone()));
    let actual = list.item_type();
    let values = list
        .into_function_values(item_type)
        .ok_or_else(|| ExecutionError::list_item_type_mismatch(expected.clone(), actual))?;
    values
        .get(index)
        .cloned()
        .ok_or_else(|| ExecutionError::list_index_out_of_bounds(expected, index, values.len()))
}

#[cfg(test)]
mod tests {
    use super::{
        eval_list_expr, project_bool_list_expr, project_float_list_expr,
        project_function_list_expr, project_int_list_expr, project_list_list_expr,
        project_nil_list_expr, project_string_list_expr, project_tuple_list_expr,
    };
    use crate::plan::ListElements;
    use crate::plan::{
        BoolExpr, BoolListCaseBranches, ExecutionPlan, Expr, FloatExpr, FrameLayout, FunctionExpr,
        FunctionId, FunctionPlan, FunctionType, IntExpr, IntFunctionExpr, IntFunctionId,
        IntFunctionValue, IntListLocalId, ListExpr, ListFunctionId, ListLocal, ListReturn,
        ListValue, NilExpr, PanicExpr, PanicSite, ReturnExpr, Step, StringExpr, TupleExpr,
        ValueType,
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
        assert_eq!(
            frame.set_list(&ListLocal::int(IntListLocalId(0)), list_value(1)),
            Ok(())
        );

        assert_eq!(
            eval_list_expr(&plan, &mut frame, &list_expr(1)),
            Ok(list_value(1)),
        );
        assert_eq!(
            eval_list_expr(
                &plan,
                &mut frame,
                &ListExpr::local_get(ListLocal::int(IntListLocalId(0)), "values".into()),
            ),
            Ok(list_value(1)),
        );
        assert_eq!(
            eval_list_expr(
                &plan,
                &mut frame,
                &ListExpr::call(
                    ListFunctionId::from_item_type(0, crate::plan::ValueType::Int),
                    Vec::new()
                ),
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
        assert_eq!(
            frame.set_list(
                &ListLocal::int(IntListLocalId(0)),
                ListValue::int(vec![1.into()])
            ),
            Ok(())
        );
        assert_eq!(
            eval_list_expr(
                &plan,
                &mut frame,
                &ListExpr::spread(
                    vec![Expr::int(IntExpr::value(0.into()))],
                    ListExpr::local_get(ListLocal::int(IntListLocalId(0)), "tail".into()),
                    element_type(),
                ),
            ),
            Ok(ListValue::int(vec![0.into(), 1.into()])),
        );

        let plan = plan_with_string_list_function();
        let mut frame = Frame::default();
        let spread = ListExpr::from_spread_elements(
            ListElements::Int(vec![IntExpr::value(0.into())]),
            ListExpr::call(
                ListFunctionId::from_item_type(0, crate::plan::ValueType::Int),
                Vec::new(),
            ),
        );

        assert_eq!(
            eval_list_expr(&plan, &mut frame, &spread),
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
                &ListExpr::bool_case(
                    BoolExpr::value(true),
                    BoolListCaseBranches::Int {
                        true_: list_expr(1)
                            .into_int()
                            .expect("true branch should be List(Int)"),
                        false_: list_expr(2)
                            .into_int()
                            .expect("false branch should be List(Int)"),
                    },
                ),
            ),
            Ok(list_value(1)),
        );
        assert_eq!(
            eval_list_expr(
                &plan,
                &mut frame,
                &ListExpr::bool_case(
                    BoolExpr::value(false),
                    BoolListCaseBranches::Int {
                        true_: list_expr(2)
                            .into_int()
                            .expect("true branch should be List(Int)"),
                        false_: list_expr(1)
                            .into_int()
                            .expect("false branch should be List(Int)"),
                    },
                ),
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
                        ListLocal::int(IntListLocalId(0)),
                        "values".into(),
                        list_expr(1)
                    )],
                    ListExpr::local_get(ListLocal::int(IntListLocalId(0)), "values".into()),
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
                &ListExpr::list_index(
                    nested_list
                        .clone()
                        .into_list()
                        .expect("nested list should build a ListListExpr"),
                    1,
                ),
            ),
            Ok(list_value(2)),
        );
        assert_eq!(
            eval_list_expr(
                &plan,
                &mut frame,
                &ListExpr::list_index(
                    nested_list
                        .clone()
                        .into_list()
                        .expect("nested list should build a ListListExpr"),
                    2,
                ),
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
                &ListExpr::list_index(
                    ListExpr::tuple_index(
                        empty_tuple(),
                        0,
                        ValueType::List(Box::new(element_type())),
                    )
                    .into_list()
                    .expect("error nested list should build a ListListExpr"),
                    0,
                ),
            ),
            Err(tuple_index_error(ValueType::List(Box::new(
                ValueType::List(Box::new(element_type())),
            )))),
        );
        assert_eq!(
            project_list_list_expr(
                &plan,
                &mut frame,
                &nested_list
                    .clone()
                    .into_list()
                    .expect("nested list should build a ListListExpr"),
                2,
                &element_type(),
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
                )
                .into_string()
                .expect("string list should build a StringListExpr"),
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
                &ListExpr::value(vec![Expr::float(FloatExpr::value(1.5))], ValueType::Float)
                    .into_float()
                    .expect("float list should build a FloatListExpr"),
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
                &ListExpr::value(vec![Expr::bool(BoolExpr::value(true))], ValueType::Bool)
                    .into_bool()
                    .expect("bool list should build a BoolListExpr"),
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
                &ListExpr::value(vec![Expr::nil(NilExpr::value())], ValueType::Nil)
                    .into_nil()
                    .expect("nil list should build a NilListExpr"),
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
                )
                .into_tuple()
                .expect("tuple list should build a TupleListExpr"),
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
                )
                .into_list()
                .expect("nested list should build a ListListExpr"),
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
                )
                .into_function()
                .expect("function list should build a FunctionListExpr"),
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
    fn project_list_expr_rejects_mismatched_function_result_family() {
        let mut frame = Frame::default();
        let string_list = ListExpr::value(
            vec![Expr::string(StringExpr::value("wrong".into()))],
            ValueType::String,
        );
        let int_list = ListExpr::value(vec![Expr::int(IntExpr::value(1.into()))], ValueType::Int);
        let function_type = FunctionType::new(Vec::new(), ValueType::Int);

        let plan = plan_with_mismatched_list_function(ValueType::Int, string_list.clone());
        assert_eq!(
            project_int_list_expr(
                &plan,
                &mut frame,
                &ListExpr::call(
                    ListFunctionId::from_item_type(0, ValueType::Int),
                    Vec::new()
                )
                .into_int()
                .expect("int list call should build an IntListExpr"),
                0,
            ),
            Err(ExecutionError::list_item_type_mismatch(
                ValueType::Int,
                ValueType::String,
            )),
        );

        let plan = plan_with_mismatched_list_function(ValueType::String, int_list);
        assert_eq!(
            project_string_list_expr(
                &plan,
                &mut frame,
                &ListExpr::call(
                    ListFunctionId::from_item_type(0, ValueType::String),
                    Vec::new()
                )
                .into_string()
                .expect("string list call should build a StringListExpr"),
                0,
            ),
            Err(ExecutionError::list_item_type_mismatch(
                ValueType::String,
                ValueType::Int,
            )),
        );

        let plan = plan_with_mismatched_list_function(ValueType::Float, string_list.clone());
        assert_eq!(
            project_float_list_expr(
                &plan,
                &mut frame,
                &ListExpr::call(
                    ListFunctionId::from_item_type(0, ValueType::Float),
                    Vec::new()
                )
                .into_float()
                .expect("float list call should build a FloatListExpr"),
                0,
            ),
            Err(ExecutionError::list_item_type_mismatch(
                ValueType::Float,
                ValueType::String,
            )),
        );

        let plan = plan_with_mismatched_list_function(ValueType::Bool, string_list.clone());
        assert_eq!(
            project_bool_list_expr(
                &plan,
                &mut frame,
                &ListExpr::call(
                    ListFunctionId::from_item_type(0, ValueType::Bool),
                    Vec::new()
                )
                .into_bool()
                .expect("bool list call should build a BoolListExpr"),
                0,
            ),
            Err(ExecutionError::list_item_type_mismatch(
                ValueType::Bool,
                ValueType::String,
            )),
        );

        let plan = plan_with_mismatched_list_function(ValueType::Nil, string_list.clone());
        assert_eq!(
            project_nil_list_expr(
                &plan,
                &mut frame,
                &ListExpr::call(
                    ListFunctionId::from_item_type(0, ValueType::Nil),
                    Vec::new()
                )
                .into_nil()
                .expect("nil list call should build a NilListExpr"),
                0,
            ),
            Err(ExecutionError::list_item_type_mismatch(
                ValueType::Nil,
                ValueType::String,
            )),
        );

        let tuple_type = vec![ValueType::Int];
        let plan = plan_with_mismatched_list_function(
            ValueType::Tuple(tuple_type.clone()),
            string_list.clone(),
        );
        assert_eq!(
            project_tuple_list_expr(
                &plan,
                &mut frame,
                &ListExpr::call(
                    ListFunctionId::from_item_type(0, ValueType::Tuple(tuple_type.clone())),
                    Vec::new(),
                )
                .into_tuple()
                .expect("tuple list call should build a TupleListExpr"),
                0,
                &tuple_type,
            ),
            Err(ExecutionError::list_item_type_mismatch(
                ValueType::Tuple(tuple_type),
                ValueType::String,
            )),
        );

        let nested_type = ValueType::Int;
        let plan = plan_with_mismatched_list_function(
            ValueType::List(Box::new(nested_type.clone())),
            string_list.clone(),
        );
        assert_eq!(
            project_list_list_expr(
                &plan,
                &mut frame,
                &ListExpr::call(
                    ListFunctionId::from_item_type(
                        0,
                        ValueType::List(Box::new(nested_type.clone())),
                    ),
                    Vec::new(),
                )
                .into_list()
                .expect("nested list call should build a ListListExpr"),
                0,
                &nested_type,
            ),
            Err(ExecutionError::list_item_type_mismatch(
                ValueType::List(Box::new(nested_type)),
                ValueType::String,
            )),
        );

        let plan = plan_with_mismatched_list_function(
            ValueType::Function(Box::new(function_type.clone())),
            string_list,
        );
        assert_eq!(
            project_function_list_expr(
                &plan,
                &mut frame,
                &ListExpr::call(
                    ListFunctionId::from_item_type(
                        0,
                        ValueType::Function(Box::new(function_type.clone())),
                    ),
                    Vec::new(),
                )
                .into_function()
                .expect("function list call should build a FunctionListExpr"),
                0,
                &function_type,
            ),
            Err(ExecutionError::list_item_type_mismatch(
                ValueType::Function(Box::new(function_type)),
                ValueType::String,
            )),
        );
    }

    #[test]
    fn project_list_expr_propagates_list_evaluation_errors() {
        let plan = plan();
        let mut frame = Frame::default();
        let function_type = FunctionType::new(Vec::new(), ValueType::Int);

        assert_eq!(
            project_int_list_expr(
                &plan,
                &mut frame,
                &error_list_expr()
                    .into_int()
                    .expect("int error list should build an IntListExpr"),
                0,
            ),
            Err(tuple_index_error(ValueType::List(Box::new(element_type())))),
        );
        assert_eq!(
            project_string_list_expr(
                &plan,
                &mut frame,
                &ListExpr::tuple_index(empty_tuple(), 0, ValueType::String)
                    .into_string()
                    .expect("string error list should build a StringListExpr"),
                0,
            ),
            Err(tuple_index_error(ValueType::List(Box::new(
                ValueType::String
            )))),
        );
        assert_eq!(
            project_float_list_expr(
                &plan,
                &mut frame,
                &ListExpr::tuple_index(empty_tuple(), 0, ValueType::Float)
                    .into_float()
                    .expect("float error list should build a FloatListExpr"),
                0,
            ),
            Err(tuple_index_error(ValueType::List(Box::new(
                ValueType::Float
            )))),
        );
        assert_eq!(
            project_bool_list_expr(
                &plan,
                &mut frame,
                &ListExpr::tuple_index(empty_tuple(), 0, ValueType::Bool)
                    .into_bool()
                    .expect("bool error list should build a BoolListExpr"),
                0,
            ),
            Err(tuple_index_error(ValueType::List(Box::new(
                ValueType::Bool
            )))),
        );
        assert_eq!(
            project_nil_list_expr(
                &plan,
                &mut frame,
                &ListExpr::tuple_index(empty_tuple(), 0, ValueType::Nil)
                    .into_nil()
                    .expect("nil error list should build a NilListExpr"),
                0,
            ),
            Err(tuple_index_error(ValueType::List(Box::new(ValueType::Nil)))),
        );
        assert_eq!(
            project_tuple_list_expr(
                &plan,
                &mut frame,
                &ListExpr::tuple_index(empty_tuple(), 0, ValueType::Tuple(vec![ValueType::Int]))
                    .into_tuple()
                    .expect("tuple error list should build a TupleListExpr"),
                0,
                &[ValueType::Int],
            ),
            Err(tuple_index_error(ValueType::List(Box::new(
                ValueType::Tuple(vec![ValueType::Int])
            )))),
        );
        assert_eq!(
            project_list_list_expr(
                &plan,
                &mut frame,
                &ListExpr::tuple_index(
                    empty_tuple(),
                    0,
                    ValueType::List(Box::new(element_type())),
                )
                .into_list()
                .expect("nested error list should build a ListListExpr"),
                0,
                &element_type(),
            ),
            Err(tuple_index_error(ValueType::List(Box::new(
                ValueType::List(Box::new(element_type())),
            )))),
        );
        assert_eq!(
            project_function_list_expr(
                &plan,
                &mut frame,
                &ListExpr::tuple_index(
                    empty_tuple(),
                    0,
                    ValueType::Function(Box::new(function_type.clone())),
                )
                .into_function()
                .expect("function error list should build a FunctionListExpr"),
                0,
                &function_type,
            ),
            Err(tuple_index_error(ValueType::List(Box::new(
                ValueType::Function(Box::new(function_type))
            )))),
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
                ListExpr::bool_case(
                    error_bool_expr(),
                    BoolListCaseBranches::Int {
                        true_: list_expr(1)
                            .into_int()
                            .expect("true branch should be List(Int)"),
                        false_: list_expr(2)
                            .into_int()
                            .expect("false branch should be List(Int)"),
                    },
                ),
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
        layout.include_list(ListLocal::int(IntListLocalId(0)));
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
                    ListFunctionId::from_item_type(0, element_type()),
                    ListReturn::expr(list_expr(1)),
                ),
            )],
        )
    }

    fn plan_with_string_list_function() -> ExecutionPlan {
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
                "string_list".into(),
                Vec::new(),
                Vec::new(),
                ReturnExpr::list_body(
                    ListFunctionId::from_item_type(0, ValueType::Int),
                    ListReturn::expr(ListExpr::value(
                        vec![Expr::string(StringExpr::value("tail".into()))],
                        ValueType::String,
                    )),
                ),
            )],
        )
    }

    fn plan_with_mismatched_list_function(
        expected_item_type: ValueType,
        body: ListExpr,
    ) -> ExecutionPlan {
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
                "mismatched_list".into(),
                Vec::new(),
                Vec::new(),
                ReturnExpr::list_body(
                    ListFunctionId::from_item_type(0, expected_item_type),
                    ListReturn::expr(body),
                ),
            )],
        )
    }
}
