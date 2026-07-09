use super::{
    eval_bool_expr, eval_float_expr, eval_function_expr, eval_int_expr, eval_nil_expr,
    eval_panic_expr, eval_string_expr, eval_tuple_expr, project_tuple_expr,
};
use crate::plan::{
    BoolListExpr, BoolListItem, ExecutionPlan, FloatListExpr, FloatListItem, FunctionListExpr,
    FunctionListItem, FunctionType, FunctionValue, IntListExpr, IntListItem, ListExpr, ListItem,
    ListListExpr, ListListItem, ListValue, NilListExpr, NilListItem, StringListExpr,
    StringListItem, TupleListExpr, TupleListItem, TypedListExpr, TypedListExprKind, Value,
    ValueType,
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
        ListExpr::Int(expression) => {
            eval_int_list_expr(plan, frame, expression).map(ListValue::int)
        }
        ListExpr::String(expression) => {
            eval_string_list_expr(plan, frame, expression).map(ListValue::string)
        }
        ListExpr::Float(expression) => {
            eval_float_list_expr(plan, frame, expression).map(ListValue::float)
        }
        ListExpr::Bool(expression) => {
            eval_bool_list_expr(plan, frame, expression).map(ListValue::bool)
        }
        ListExpr::Nil(expression) => {
            eval_nil_list_expr(plan, frame, expression).map(ListValue::nil)
        }
        ListExpr::Tuple(expression) => eval_tuple_list_expr(plan, frame, expression)
            .map(|values| ListValue::tuple(expression.item().item_type(), values)),
        ListExpr::List(expression) => eval_list_list_expr(plan, frame, expression)
            .map(|values| ListValue::list(expression.item().item_type().as_ref().clone(), values)),
        ListExpr::Function(expression) => eval_function_list_expr(plan, frame, expression)
            .map(|values| ListValue::function(expression.item().item_type(), values)),
    }
}

pub(in crate::runtime) fn eval_int_list_expr(
    plan: &ExecutionPlan,
    frame: &mut Frame,
    expression: &IntListExpr,
) -> Result<Vec<BigInt>, ExecutionError> {
    eval_typed_list_expr(plan, frame, expression)
}

pub(in crate::runtime) fn eval_string_list_expr(
    plan: &ExecutionPlan,
    frame: &mut Frame,
    expression: &StringListExpr,
) -> Result<Vec<EcoString>, ExecutionError> {
    eval_typed_list_expr(plan, frame, expression)
}

pub(in crate::runtime) fn eval_float_list_expr(
    plan: &ExecutionPlan,
    frame: &mut Frame,
    expression: &FloatListExpr,
) -> Result<Vec<f64>, ExecutionError> {
    eval_typed_list_expr(plan, frame, expression)
}

pub(in crate::runtime) fn eval_bool_list_expr(
    plan: &ExecutionPlan,
    frame: &mut Frame,
    expression: &BoolListExpr,
) -> Result<Vec<bool>, ExecutionError> {
    eval_typed_list_expr(plan, frame, expression)
}

pub(in crate::runtime) fn eval_nil_list_expr(
    plan: &ExecutionPlan,
    frame: &mut Frame,
    expression: &NilListExpr,
) -> Result<usize, ExecutionError> {
    eval_typed_list_expr(plan, frame, expression)
}

pub(in crate::runtime) fn eval_tuple_list_expr(
    plan: &ExecutionPlan,
    frame: &mut Frame,
    expression: &TupleListExpr,
) -> Result<Vec<Vec<Value>>, ExecutionError> {
    eval_typed_list_expr(plan, frame, expression)
}

pub(in crate::runtime) fn eval_list_list_expr(
    plan: &ExecutionPlan,
    frame: &mut Frame,
    expression: &ListListExpr,
) -> Result<Vec<ListValue>, ExecutionError> {
    eval_typed_list_expr(plan, frame, expression)
}

pub(in crate::runtime) fn eval_function_list_expr(
    plan: &ExecutionPlan,
    frame: &mut Frame,
    expression: &FunctionListExpr,
) -> Result<Vec<FunctionValue>, ExecutionError> {
    eval_typed_list_expr(plan, frame, expression)
}

fn eval_typed_list_expr<Item: RuntimeListItem>(
    plan: &ExecutionPlan,
    frame: &mut Frame,
    expression: &TypedListExpr<Item>,
) -> Result<Item::RuntimeValue, ExecutionError> {
    match expression.kind() {
        TypedListExprKind::Value(elements) => {
            Item::eval_elements(plan, frame, expression.item(), elements)
        }
        TypedListExprKind::Spread { elements, tail } => {
            let mut values = Item::eval_elements(plan, frame, expression.item(), elements)?;
            let tail = eval_typed_list_expr(plan, frame, tail)?;
            Item::append(&mut values, tail);
            Ok(values)
        }
        TypedListExprKind::LocalGet { local, .. } => Ok(Item::get_local(frame, local.clone())),
        TypedListExprKind::Call { function, args } => {
            Item::run_call(plan, function.clone(), args, frame)
        }
        TypedListExprKind::FunctionCall { function, args } => {
            Item::run_function_call(plan, function, args, frame)
        }
        TypedListExprKind::TupleIndex { tuple, index } => {
            let expected = ValueType::List(Box::new(expression.element_type().clone()));
            match project_tuple_expr(plan, frame, tuple, *index, expected.clone())? {
                Value::List(value) => {
                    let actual = value.item_type();
                    Item::from_facade(expression.item(), value).ok_or_else(|| {
                        ExecutionError::tuple_index_family_mismatch(
                            expected,
                            ValueType::List(Box::new(actual)),
                        )
                    })
                }
                other => Err(ExecutionError::tuple_index_family_mismatch(
                    expected,
                    other.value_type(),
                )),
            }
        }
        TypedListExprKind::ListIndex { list, index } => {
            Item::project_nested_list(plan, frame, expression.item(), list, *index)
        }
        TypedListExprKind::DropFirst { list, count } => {
            let list = eval_typed_list_expr(plan, frame, list)?;
            Ok(Item::drop_first(&list, *count))
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

trait RuntimeListItem: ListItem {
    type RuntimeValue: Clone;

    fn eval_elements(
        plan: &ExecutionPlan,
        frame: &mut Frame,
        item: &Self,
        elements: &[Self::ElementExpr],
    ) -> Result<Self::RuntimeValue, ExecutionError>;

    fn append(values: &mut Self::RuntimeValue, tail: Self::RuntimeValue);

    fn drop_first(values: &Self::RuntimeValue, count: usize) -> Self::RuntimeValue;

    fn from_facade(item: &Self, value: ListValue) -> Option<Self::RuntimeValue>;

    fn get_local(frame: &Frame, local: Self::Local) -> Self::RuntimeValue;

    fn run_call(
        plan: &ExecutionPlan,
        function: Self::Function,
        args: &[crate::plan::CallArg],
        frame: &mut Frame,
    ) -> Result<Self::RuntimeValue, ExecutionError>;

    fn run_function_call(
        plan: &ExecutionPlan,
        function: &crate::plan::ListFunctionExpr,
        args: &[crate::plan::CallArg],
        frame: &mut Frame,
    ) -> Result<Self::RuntimeValue, ExecutionError>;

    fn project_nested_list(
        plan: &ExecutionPlan,
        frame: &mut Frame,
        item: &Self,
        list: &ListListExpr,
        index: usize,
    ) -> Result<Self::RuntimeValue, ExecutionError> {
        let values = eval_list_list_expr(plan, frame, list)?;
        let expected = ValueType::List(Box::new(item.value_type()));
        let Some(value) = values.get(index).cloned() else {
            return Err(ExecutionError::list_index_out_of_bounds(
                expected,
                index,
                values.len(),
            ));
        };
        let actual = value.item_type();
        Self::from_facade(item, value).ok_or_else(|| {
            ExecutionError::tuple_index_family_mismatch(
                ValueType::List(Box::new(item.value_type())),
                ValueType::List(Box::new(actual)),
            )
        })
    }
}

macro_rules! primitive_runtime_list_item {
    (
        $item:ty,
        $value:ty,
        $element_eval:ident,
        $from_facade:ident,
        $get_local:ident,
        $run_call:ident,
        $run_function_call:ident
    ) => {
        impl RuntimeListItem for $item {
            type RuntimeValue = Vec<$value>;

            fn eval_elements(
                plan: &ExecutionPlan,
                frame: &mut Frame,
                _item: &Self,
                elements: &[Self::ElementExpr],
            ) -> Result<Self::RuntimeValue, ExecutionError> {
                elements
                    .iter()
                    .map(|element| $element_eval(plan, frame, element))
                    .collect()
            }

            fn append(values: &mut Self::RuntimeValue, tail: Self::RuntimeValue) {
                values.extend(tail);
            }

            fn drop_first(values: &Self::RuntimeValue, count: usize) -> Self::RuntimeValue {
                values[count.min(values.len())..].to_vec()
            }

            fn from_facade(_item: &Self, value: ListValue) -> Option<Self::RuntimeValue> {
                value.$from_facade()
            }

            fn get_local(frame: &Frame, local: Self::Local) -> Self::RuntimeValue {
                frame.$get_local(local)
            }

            fn run_call(
                plan: &ExecutionPlan,
                function: Self::Function,
                args: &[crate::plan::CallArg],
                frame: &mut Frame,
            ) -> Result<Self::RuntimeValue, ExecutionError> {
                function::$run_call(plan, function, args, frame)
            }

            fn run_function_call(
                plan: &ExecutionPlan,
                function: &crate::plan::ListFunctionExpr,
                args: &[crate::plan::CallArg],
                frame: &mut Frame,
            ) -> Result<Self::RuntimeValue, ExecutionError> {
                function::$run_function_call(plan, function, args, frame)
            }
        }
    };
}

primitive_runtime_list_item!(
    IntListItem,
    BigInt,
    eval_int_expr,
    into_int_values,
    get_int_list,
    run_int_list_call,
    run_int_list_function_call
);

primitive_runtime_list_item!(
    StringListItem,
    EcoString,
    eval_string_expr,
    into_string_values,
    get_string_list,
    run_string_list_call,
    run_string_list_function_call
);

primitive_runtime_list_item!(
    FloatListItem,
    f64,
    eval_float_expr,
    into_float_values,
    get_float_list,
    run_float_list_call,
    run_float_list_function_call
);

primitive_runtime_list_item!(
    BoolListItem,
    bool,
    eval_bool_expr,
    into_bool_values,
    get_bool_list,
    run_bool_list_call,
    run_bool_list_function_call
);

impl RuntimeListItem for NilListItem {
    type RuntimeValue = usize;

    fn eval_elements(
        plan: &ExecutionPlan,
        frame: &mut Frame,
        _item: &Self,
        elements: &[Self::ElementExpr],
    ) -> Result<Self::RuntimeValue, ExecutionError> {
        for element in elements {
            eval_nil_expr(plan, frame, element)?;
        }
        Ok(elements.len())
    }

    fn append(values: &mut Self::RuntimeValue, tail: Self::RuntimeValue) {
        *values += tail;
    }

    fn drop_first(values: &Self::RuntimeValue, count: usize) -> Self::RuntimeValue {
        values.saturating_sub(count)
    }

    fn from_facade(_item: &Self, value: ListValue) -> Option<Self::RuntimeValue> {
        value.into_nil_len()
    }

    fn get_local(frame: &Frame, local: Self::Local) -> Self::RuntimeValue {
        frame.get_nil_list(local)
    }

    fn run_call(
        plan: &ExecutionPlan,
        function: Self::Function,
        args: &[crate::plan::CallArg],
        frame: &mut Frame,
    ) -> Result<Self::RuntimeValue, ExecutionError> {
        function::run_nil_list_call(plan, function, args, frame)
    }

    fn run_function_call(
        plan: &ExecutionPlan,
        function: &crate::plan::ListFunctionExpr,
        args: &[crate::plan::CallArg],
        frame: &mut Frame,
    ) -> Result<Self::RuntimeValue, ExecutionError> {
        function::run_nil_list_function_call(plan, function, args, frame)
    }
}

impl RuntimeListItem for TupleListItem {
    type RuntimeValue = Vec<Vec<Value>>;

    fn eval_elements(
        plan: &ExecutionPlan,
        frame: &mut Frame,
        _item: &Self,
        elements: &[Self::ElementExpr],
    ) -> Result<Self::RuntimeValue, ExecutionError> {
        elements
            .iter()
            .map(|element| eval_tuple_expr(plan, frame, element))
            .collect()
    }

    fn append(values: &mut Self::RuntimeValue, tail: Self::RuntimeValue) {
        values.extend(tail);
    }

    fn drop_first(values: &Self::RuntimeValue, count: usize) -> Self::RuntimeValue {
        values[count.min(values.len())..].to_vec()
    }

    fn from_facade(item: &Self, value: ListValue) -> Option<Self::RuntimeValue> {
        value.into_tuple_values(&item.item_type())
    }

    fn get_local(frame: &Frame, local: Self::Local) -> Self::RuntimeValue {
        frame.get_tuple_list(local)
    }

    fn run_call(
        plan: &ExecutionPlan,
        function: Self::Function,
        args: &[crate::plan::CallArg],
        frame: &mut Frame,
    ) -> Result<Self::RuntimeValue, ExecutionError> {
        function::run_tuple_list_call(plan, function, args, frame)
    }

    fn run_function_call(
        plan: &ExecutionPlan,
        function: &crate::plan::ListFunctionExpr,
        args: &[crate::plan::CallArg],
        frame: &mut Frame,
    ) -> Result<Self::RuntimeValue, ExecutionError> {
        function::run_tuple_list_function_call(plan, function, args, frame)
    }
}

impl RuntimeListItem for ListListItem {
    type RuntimeValue = Vec<ListValue>;

    fn eval_elements(
        plan: &ExecutionPlan,
        frame: &mut Frame,
        _item: &Self,
        elements: &[Self::ElementExpr],
    ) -> Result<Self::RuntimeValue, ExecutionError> {
        elements
            .iter()
            .map(|element| eval_list_expr(plan, frame, element))
            .collect()
    }

    fn append(values: &mut Self::RuntimeValue, tail: Self::RuntimeValue) {
        values.extend(tail);
    }

    fn drop_first(values: &Self::RuntimeValue, count: usize) -> Self::RuntimeValue {
        values[count.min(values.len())..].to_vec()
    }

    fn from_facade(item: &Self, value: ListValue) -> Option<Self::RuntimeValue> {
        value.into_list_values(&item.item_type())
    }

    fn get_local(frame: &Frame, local: Self::Local) -> Self::RuntimeValue {
        frame.get_list_list(local)
    }

    fn run_call(
        plan: &ExecutionPlan,
        function: Self::Function,
        args: &[crate::plan::CallArg],
        frame: &mut Frame,
    ) -> Result<Self::RuntimeValue, ExecutionError> {
        function::run_list_list_call(plan, function, args, frame)
    }

    fn run_function_call(
        plan: &ExecutionPlan,
        function: &crate::plan::ListFunctionExpr,
        args: &[crate::plan::CallArg],
        frame: &mut Frame,
    ) -> Result<Self::RuntimeValue, ExecutionError> {
        function::run_list_list_function_call(plan, function, args, frame)
    }
}

impl RuntimeListItem for FunctionListItem {
    type RuntimeValue = Vec<FunctionValue>;

    fn eval_elements(
        plan: &ExecutionPlan,
        frame: &mut Frame,
        _item: &Self,
        elements: &[Self::ElementExpr],
    ) -> Result<Self::RuntimeValue, ExecutionError> {
        elements
            .iter()
            .map(|element| eval_function_expr(plan, frame, element))
            .collect()
    }

    fn append(values: &mut Self::RuntimeValue, tail: Self::RuntimeValue) {
        values.extend(tail);
    }

    fn drop_first(values: &Self::RuntimeValue, count: usize) -> Self::RuntimeValue {
        values[count.min(values.len())..].to_vec()
    }

    fn from_facade(item: &Self, value: ListValue) -> Option<Self::RuntimeValue> {
        value.into_function_values(&item.item_type())
    }

    fn get_local(frame: &Frame, local: Self::Local) -> Self::RuntimeValue {
        frame.get_function_list(local)
    }

    fn run_call(
        plan: &ExecutionPlan,
        function: Self::Function,
        args: &[crate::plan::CallArg],
        frame: &mut Frame,
    ) -> Result<Self::RuntimeValue, ExecutionError> {
        function::run_function_list_call(plan, function, args, frame)
    }

    fn run_function_call(
        plan: &ExecutionPlan,
        function: &crate::plan::ListFunctionExpr,
        args: &[crate::plan::CallArg],
        frame: &mut Frame,
    ) -> Result<Self::RuntimeValue, ExecutionError> {
        function::run_function_list_function_call(plan, function, args, frame)
    }
}

pub(in crate::runtime) fn get_list_value(
    frame: &Frame,
    local: &crate::plan::ListLocal,
) -> ListValue {
    match local {
        crate::plan::ListLocal::Int(local) => ListValue::int(frame.get_int_list(*local)),
        crate::plan::ListLocal::String(local) => ListValue::string(frame.get_string_list(*local)),
        crate::plan::ListLocal::Float(local) => ListValue::float(frame.get_float_list(*local)),
        crate::plan::ListLocal::Bool(local) => ListValue::bool(frame.get_bool_list(*local)),
        crate::plan::ListLocal::Nil(local) => ListValue::nil(frame.get_nil_list(*local)),
        crate::plan::ListLocal::Tuple { local, item_type } => {
            ListValue::tuple(item_type.clone(), frame.get_tuple_list(*local))
        }
        crate::plan::ListLocal::List { local, item_type } => {
            ListValue::list(item_type.as_ref().clone(), frame.get_list_list(*local))
        }
        crate::plan::ListLocal::Function { local, item_type } => {
            ListValue::function(item_type.clone(), frame.get_function_list(*local))
        }
    }
}

pub(in crate::runtime) fn project_int_list_expr(
    plan: &ExecutionPlan,
    frame: &mut Frame,
    list: &IntListExpr,
    index: usize,
) -> Result<BigInt, ExecutionError> {
    let values = eval_int_list_expr(plan, frame, list)?;
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
    let values = eval_string_list_expr(plan, frame, list)?;
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
    let values = eval_float_list_expr(plan, frame, list)?;
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
    let values = eval_bool_list_expr(plan, frame, list)?;
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
    let len = eval_nil_list_expr(plan, frame, list)?;
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
    let expected = ValueType::Tuple(item_type.to_vec());
    let values = eval_tuple_list_expr(plan, frame, list)?;
    values
        .get(index)
        .cloned()
        .ok_or_else(|| ExecutionError::list_index_out_of_bounds(expected, index, values.len()))
}

#[cfg(test)]
fn project_list_list_expr(
    plan: &ExecutionPlan,
    frame: &mut Frame,
    list: &ListListExpr,
    index: usize,
    item_type: &ValueType,
) -> Result<ListValue, ExecutionError> {
    let expected = ValueType::List(Box::new(item_type.clone()));
    let values = eval_list_list_expr(plan, frame, list)?;
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
    let expected = ValueType::Function(Box::new(item_type.clone()));
    let values = eval_function_list_expr(plan, frame, list)?;
    values
        .get(index)
        .cloned()
        .ok_or_else(|| ExecutionError::list_index_out_of_bounds(expected, index, values.len()))
}

#[cfg(test)]
mod tests {
    use super::{
        RuntimeListItem, eval_list_expr, get_list_value, project_bool_list_expr,
        project_float_list_expr, project_function_list_expr, project_int_list_expr,
        project_list_list_expr, project_nil_list_expr, project_string_list_expr,
        project_tuple_list_expr,
    };
    use crate::plan::{
        BoolExpr, BoolListCaseBranches, BoolListLocalId, CallArg, ExecutionPlan, Expr, FloatExpr,
        FloatListLocalId, FrameLayout, FunctionExpr, FunctionId, FunctionListLocalId, FunctionPlan,
        FunctionReturnFamily, FunctionType, IntExpr, IntFunctionExpr, IntFunctionId,
        IntFunctionValue, IntListLocalId, IntLocalId, ListCaseBranches, ListExpr, ListFunctionExpr,
        ListFunctionId, ListFunctionValue, ListListLocalId, ListLocal, ListReturn, ListValue,
        NilExpr, NilListLocalId, PanicExpr, PanicSite, ReturnExpr, Step, StringExpr,
        StringListItem, StringListLocalId, TupleExpr, TupleListLocalId, ValueType,
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
        frame.set_int_list(IntListLocalId(0), vec![1.into()]);

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
    fn eval_list_expr_evaluates_direct_and_function_value_calls_for_every_item_family() {
        let plan = all_list_family_plan();
        let mut frame = Frame::default();
        let function_type = FunctionType::new(Vec::new(), ValueType::Int);
        let cases = [
            (
                ListFunctionId::from_item_type(0, ValueType::Int),
                ListValue::int(vec![1.into()]),
            ),
            (
                ListFunctionId::from_item_type(0, ValueType::String),
                ListValue::string(vec!["one".into()]),
            ),
            (
                ListFunctionId::from_item_type(0, ValueType::Float),
                ListValue::float(vec![1.5]),
            ),
            (
                ListFunctionId::from_item_type(0, ValueType::Bool),
                ListValue::bool(vec![true]),
            ),
            (
                ListFunctionId::from_item_type(0, ValueType::Nil),
                ListValue::nil(1),
            ),
            (
                ListFunctionId::from_item_type(0, ValueType::Tuple(vec![ValueType::Int])),
                ListValue::tuple(
                    vec![ValueType::Int],
                    vec![vec![crate::plan::Value::Int(2.into())]],
                ),
            ),
            (
                ListFunctionId::from_item_type(0, ValueType::List(Box::new(ValueType::Int))),
                ListValue::list(ValueType::Int, vec![ListValue::int(vec![3.into()])]),
            ),
            (
                ListFunctionId::from_item_type(
                    0,
                    ValueType::Function(Box::new(function_type.clone())),
                ),
                ListValue::function(
                    function_type,
                    vec![crate::plan::FunctionValue::new(
                        crate::plan::RuntimeFunctionId::Int(IntFunctionId(0)),
                        Vec::new(),
                    )],
                ),
            ),
        ];

        for (function, expected) in cases {
            assert_eq!(
                eval_list_expr(
                    &plan,
                    &mut frame,
                    &ListExpr::call(function.clone(), Vec::new())
                ),
                Ok(expected.clone()),
            );
            assert_eq!(
                eval_list_expr(
                    &plan,
                    &mut frame,
                    &ListExpr::function_call(
                        ListFunctionExpr::value(ListFunctionValue::new(function, Vec::new())),
                        Vec::new(),
                    ),
                ),
                Ok(expected),
            );
        }
    }

    #[test]
    fn eval_list_expr_direct_calls_propagate_argument_errors_for_every_item_family() {
        let plan = all_list_family_plan();
        let mut frame = Frame::default();
        let function_type = FunctionType::new(Vec::new(), ValueType::Int);

        for function in [
            ListFunctionId::from_item_type(0, ValueType::Int),
            ListFunctionId::from_item_type(0, ValueType::String),
            ListFunctionId::from_item_type(0, ValueType::Float),
            ListFunctionId::from_item_type(0, ValueType::Bool),
            ListFunctionId::from_item_type(0, ValueType::Nil),
            ListFunctionId::from_item_type(0, ValueType::Tuple(vec![ValueType::Int])),
            ListFunctionId::from_item_type(0, ValueType::List(Box::new(ValueType::Int))),
            ListFunctionId::from_item_type(0, ValueType::Function(Box::new(function_type.clone()))),
        ] {
            assert_eq!(
                eval_list_expr(
                    &plan,
                    &mut frame,
                    &ListExpr::call(
                        function,
                        vec![CallArg::int(IntLocalId(0), error_int_expr())],
                    ),
                ),
                Err(tuple_index_error(ValueType::Int)),
            );
        }
    }

    #[test]
    fn eval_list_expr_function_value_calls_reject_wrong_list_return_family() {
        let plan = all_list_family_plan();
        let mut frame = Frame::default();

        assert_wrong_list_function_family(crate::runtime::function::run_int_list_function_call(
            &plan,
            &wrong_list_function_value(ValueType::String),
            &[],
            &mut frame,
        ));
        assert_wrong_list_function_family(crate::runtime::function::run_string_list_function_call(
            &plan,
            &wrong_list_function_value(ValueType::Int),
            &[],
            &mut frame,
        ));
        assert_wrong_list_function_family(crate::runtime::function::run_float_list_function_call(
            &plan,
            &wrong_list_function_value(ValueType::Int),
            &[],
            &mut frame,
        ));
        assert_wrong_list_function_family(crate::runtime::function::run_bool_list_function_call(
            &plan,
            &wrong_list_function_value(ValueType::Int),
            &[],
            &mut frame,
        ));
        assert_wrong_list_function_family(crate::runtime::function::run_nil_list_function_call(
            &plan,
            &wrong_list_function_value(ValueType::Int),
            &[],
            &mut frame,
        ));
        assert_wrong_list_function_family(crate::runtime::function::run_tuple_list_function_call(
            &plan,
            &wrong_list_function_value(ValueType::Int),
            &[],
            &mut frame,
        ));
        assert_wrong_list_function_family(crate::runtime::function::run_list_list_function_call(
            &plan,
            &wrong_list_function_value(ValueType::Int),
            &[],
            &mut frame,
        ));
        assert_wrong_list_function_family(
            crate::runtime::function::run_function_list_function_call(
                &plan,
                &wrong_list_function_value(ValueType::Int),
                &[],
                &mut frame,
            ),
        );
    }

    #[test]
    fn generic_list_function_value_call_dispatches_every_item_family() {
        let plan = all_list_family_plan();
        let mut frame = Frame::default();
        let function_type = FunctionType::new(Vec::new(), ValueType::Int);

        for (function, expected) in [
            (
                ListFunctionId::from_item_type(0, ValueType::Int),
                ListValue::int(vec![1.into()]),
            ),
            (
                ListFunctionId::from_item_type(0, ValueType::String),
                ListValue::string(vec!["one".into()]),
            ),
            (
                ListFunctionId::from_item_type(0, ValueType::Float),
                ListValue::float(vec![1.5]),
            ),
            (
                ListFunctionId::from_item_type(0, ValueType::Bool),
                ListValue::bool(vec![true]),
            ),
            (
                ListFunctionId::from_item_type(0, ValueType::Nil),
                ListValue::nil(1),
            ),
            (
                ListFunctionId::from_item_type(0, ValueType::Tuple(vec![ValueType::Int])),
                ListValue::tuple(
                    vec![ValueType::Int],
                    vec![vec![crate::plan::Value::Int(2.into())]],
                ),
            ),
            (
                ListFunctionId::from_item_type(0, ValueType::List(Box::new(ValueType::Int))),
                ListValue::list(ValueType::Int, vec![ListValue::int(vec![3.into()])]),
            ),
            (
                ListFunctionId::from_item_type(
                    0,
                    ValueType::Function(Box::new(function_type.clone())),
                ),
                ListValue::function(
                    function_type,
                    vec![crate::plan::FunctionValue::new(
                        crate::plan::RuntimeFunctionId::Int(IntFunctionId(0)),
                        Vec::new(),
                    )],
                ),
            ),
        ] {
            assert_eq!(
                crate::runtime::function::run_list_function_call(
                    &plan,
                    &list_function_value(function),
                    &[],
                    &mut frame,
                ),
                Ok(expected),
            );
        }
    }

    #[test]
    fn list_function_value_calls_propagate_argument_errors_for_every_item_family() {
        let plan = all_list_family_plan();
        let mut frame = Frame::default();
        let function_type = FunctionType::new(Vec::new(), ValueType::Int);

        assert_int_argument_error(crate::runtime::function::run_int_list_function_call(
            &plan,
            &list_function_value(ListFunctionId::from_item_type(0, ValueType::Int)),
            &[CallArg::int(IntLocalId(0), error_int_expr())],
            &mut frame,
        ));
        assert_int_argument_error(crate::runtime::function::run_string_list_function_call(
            &plan,
            &list_function_value(ListFunctionId::from_item_type(0, ValueType::String)),
            &[CallArg::int(IntLocalId(0), error_int_expr())],
            &mut frame,
        ));
        assert_int_argument_error(crate::runtime::function::run_float_list_function_call(
            &plan,
            &list_function_value(ListFunctionId::from_item_type(0, ValueType::Float)),
            &[CallArg::int(IntLocalId(0), error_int_expr())],
            &mut frame,
        ));
        assert_int_argument_error(crate::runtime::function::run_bool_list_function_call(
            &plan,
            &list_function_value(ListFunctionId::from_item_type(0, ValueType::Bool)),
            &[CallArg::int(IntLocalId(0), error_int_expr())],
            &mut frame,
        ));
        assert_int_argument_error(crate::runtime::function::run_nil_list_function_call(
            &plan,
            &list_function_value(ListFunctionId::from_item_type(0, ValueType::Nil)),
            &[CallArg::int(IntLocalId(0), error_int_expr())],
            &mut frame,
        ));
        assert_int_argument_error(crate::runtime::function::run_tuple_list_function_call(
            &plan,
            &list_function_value(ListFunctionId::from_item_type(
                0,
                ValueType::Tuple(vec![ValueType::Int]),
            )),
            &[CallArg::int(IntLocalId(0), error_int_expr())],
            &mut frame,
        ));
        assert_int_argument_error(crate::runtime::function::run_list_list_function_call(
            &plan,
            &list_function_value(ListFunctionId::from_item_type(
                0,
                ValueType::List(Box::new(ValueType::Int)),
            )),
            &[CallArg::int(IntLocalId(0), error_int_expr())],
            &mut frame,
        ));
        assert_int_argument_error(crate::runtime::function::run_function_list_function_call(
            &plan,
            &list_function_value(ListFunctionId::from_item_type(
                0,
                ValueType::Function(Box::new(function_type)),
            )),
            &[CallArg::int(IntLocalId(0), error_int_expr())],
            &mut frame,
        ));
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
        frame.set_int_list(IntListLocalId(0), vec![1.into()]);
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
    }

    #[test]
    fn eval_list_expr_preserves_every_item_family_through_local_spread_and_drop() {
        let plan = plan();
        let mut frame = all_list_family_frame();

        assert_eq!(
            eval_list_expr(
                &plan,
                &mut frame,
                &ListExpr::local_get(ListLocal::string(StringListLocalId(1)), "values".into()),
            ),
            Ok(ListValue::string(vec!["one".into()])),
        );
        assert_eq!(
            eval_list_expr(
                &plan,
                &mut frame,
                &ListExpr::local_get(ListLocal::float(FloatListLocalId(2)), "values".into()),
            ),
            Ok(ListValue::float(vec![1.5])),
        );
        assert_eq!(
            eval_list_expr(
                &plan,
                &mut frame,
                &ListExpr::local_get(ListLocal::bool(BoolListLocalId(3)), "values".into()),
            ),
            Ok(ListValue::bool(vec![true])),
        );
        assert_eq!(
            eval_list_expr(
                &plan,
                &mut frame,
                &ListExpr::local_get(ListLocal::nil(NilListLocalId(4)), "values".into()),
            ),
            Ok(ListValue::nil(1)),
        );
        assert_eq!(
            eval_list_expr(
                &plan,
                &mut frame,
                &ListExpr::local_get(
                    ListLocal::tuple(TupleListLocalId(5), vec![ValueType::Int]),
                    "values".into(),
                ),
            ),
            Ok(ListValue::tuple(
                vec![ValueType::Int],
                vec![vec![crate::plan::Value::Int(2.into())]],
            )),
        );
        assert_eq!(
            eval_list_expr(
                &plan,
                &mut frame,
                &ListExpr::local_get(
                    ListLocal::list(ListListLocalId(6), ValueType::Int),
                    "values".into(),
                ),
            ),
            Ok(ListValue::list(
                ValueType::Int,
                vec![ListValue::int(vec![3.into()])],
            )),
        );
        assert_eq!(
            eval_list_expr(
                &plan,
                &mut frame,
                &ListExpr::local_get(
                    ListLocal::function(
                        FunctionListLocalId(7),
                        FunctionType::new(Vec::new(), ValueType::Int),
                    ),
                    "values".into(),
                ),
            ),
            Ok(ListValue::function(
                FunctionType::new(Vec::new(), ValueType::Int),
                vec![crate::plan::FunctionValue::new(
                    crate::plan::RuntimeFunctionId::Int(IntFunctionId(0)),
                    Vec::new(),
                )],
            )),
        );

        assert_eq!(
            eval_list_expr(
                &plan,
                &mut frame,
                &ListExpr::spread(
                    vec![Expr::nil(NilExpr::value())],
                    ListExpr::local_get(ListLocal::nil(NilListLocalId(4)), "tail".into()),
                    ValueType::Nil,
                ),
            ),
            Ok(ListValue::nil(2)),
        );
        assert_eq!(
            eval_list_expr(
                &plan,
                &mut frame,
                &ListExpr::spread(
                    vec![Expr::tuple(TupleExpr::value(
                        vec![Expr::int(IntExpr::value(1.into()))],
                        vec![ValueType::Int],
                    ))],
                    ListExpr::local_get(
                        ListLocal::tuple(TupleListLocalId(5), vec![ValueType::Int]),
                        "tail".into(),
                    ),
                    ValueType::Tuple(vec![ValueType::Int]),
                ),
            ),
            Ok(ListValue::tuple(
                vec![ValueType::Int],
                vec![
                    vec![crate::plan::Value::Int(1.into())],
                    vec![crate::plan::Value::Int(2.into())],
                ],
            )),
        );
        assert_eq!(
            eval_list_expr(
                &plan,
                &mut frame,
                &ListExpr::drop_first(
                    ListExpr::spread(
                        vec![Expr::nil(NilExpr::value())],
                        ListExpr::local_get(ListLocal::nil(NilListLocalId(4)), "tail".into()),
                        ValueType::Nil,
                    ),
                    1,
                ),
            ),
            Ok(ListValue::nil(1)),
        );
        assert_eq!(
            eval_list_expr(
                &plan,
                &mut frame,
                &ListExpr::drop_first(
                    ListExpr::spread(
                        vec![Expr::tuple(TupleExpr::value(
                            vec![Expr::int(IntExpr::value(1.into()))],
                            vec![ValueType::Int],
                        ))],
                        ListExpr::local_get(
                            ListLocal::tuple(TupleListLocalId(5), vec![ValueType::Int]),
                            "tail".into(),
                        ),
                        ValueType::Tuple(vec![ValueType::Int]),
                    ),
                    1,
                ),
            ),
            Ok(ListValue::tuple(
                vec![ValueType::Int],
                vec![vec![crate::plan::Value::Int(2.into())]],
            )),
        );
        assert_eq!(
            eval_list_expr(
                &plan,
                &mut frame,
                &ListExpr::drop_first(
                    ListExpr::spread(
                        vec![Expr::list(ListExpr::value(
                            vec![Expr::int(IntExpr::value(1.into()))],
                            ValueType::Int,
                        ))],
                        ListExpr::local_get(
                            ListLocal::list(ListListLocalId(6), ValueType::Int),
                            "tail".into(),
                        ),
                        ValueType::List(Box::new(ValueType::Int)),
                    ),
                    1,
                ),
            ),
            Ok(ListValue::list(
                ValueType::Int,
                vec![ListValue::int(vec![3.into()])],
            )),
        );
        assert_eq!(
            eval_list_expr(
                &plan,
                &mut frame,
                &ListExpr::drop_first(
                    ListExpr::spread(
                        vec![Expr::function(FunctionExpr::value(
                            crate::plan::FunctionValue::new(
                                crate::plan::RuntimeFunctionId::Int(IntFunctionId(0)),
                                Vec::new(),
                            ),
                        ))],
                        ListExpr::local_get(
                            ListLocal::function(
                                FunctionListLocalId(7),
                                FunctionType::new(Vec::new(), ValueType::Int),
                            ),
                            "tail".into(),
                        ),
                        ValueType::Function(Box::new(FunctionType::new(
                            Vec::new(),
                            ValueType::Int,
                        ))),
                    ),
                    1,
                ),
            ),
            Ok(ListValue::function(
                FunctionType::new(Vec::new(), ValueType::Int),
                vec![crate::plan::FunctionValue::new(
                    crate::plan::RuntimeFunctionId::Int(IntFunctionId(0)),
                    Vec::new(),
                )],
            )),
        );
    }

    #[test]
    fn get_list_value_preserves_every_item_family_from_frame() {
        let frame = all_list_family_frame();

        assert_eq!(
            get_list_value(&frame, &ListLocal::string(StringListLocalId(1))),
            ListValue::string(vec!["one".into()]),
        );
        assert_eq!(
            get_list_value(&frame, &ListLocal::float(FloatListLocalId(2))),
            ListValue::float(vec![1.5]),
        );
        assert_eq!(
            get_list_value(&frame, &ListLocal::bool(BoolListLocalId(3))),
            ListValue::bool(vec![true]),
        );
        assert_eq!(
            get_list_value(&frame, &ListLocal::nil(NilListLocalId(4))),
            ListValue::nil(1),
        );
        assert_eq!(
            get_list_value(
                &frame,
                &ListLocal::tuple(TupleListLocalId(5), vec![ValueType::Int]),
            ),
            ListValue::tuple(
                vec![ValueType::Int],
                vec![vec![crate::plan::Value::Int(2.into())]],
            ),
        );
        assert_eq!(
            get_list_value(
                &frame,
                &ListLocal::function(
                    FunctionListLocalId(7),
                    FunctionType::new(Vec::new(), ValueType::Int),
                ),
            ),
            ListValue::function(
                FunctionType::new(Vec::new(), ValueType::Int),
                vec![crate::plan::FunctionValue::new(
                    crate::plan::RuntimeFunctionId::Int(IntFunctionId(0)),
                    Vec::new(),
                )],
            ),
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
                    ListCaseBranches::from_exprs(vec![(1.into(), list_expr(1))], list_expr(2))
                        .expect("list case branches"),
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
                    ListCaseBranches::from_exprs(vec![(1.into(), list_expr(2))], list_expr(1))
                        .expect("list case branches"),
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
                    ListCaseBranches::from_exprs(vec![("hit".into(), list_expr(1))], list_expr(2))
                        .expect("list case branches"),
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
                    ListCaseBranches::from_exprs(vec![("hit".into(), list_expr(2))], list_expr(1))
                        .expect("list case branches"),
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
                    ListCaseBranches::from_exprs(vec![(1.0, list_expr(1))], list_expr(2))
                        .expect("list case branches"),
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
                    ListCaseBranches::from_exprs(vec![(1.0, list_expr(2))], list_expr(1))
                        .expect("list case branches"),
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
                    vec![Step::let_list_expr(
                        "values".into(),
                        crate::plan::ListLocalExpr::Int {
                            local: IntListLocalId(0),
                            value: list_expr(1).into_int().expect("expected int list"),
                        },
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

        let tuple = TupleExpr::value(
            vec![Expr::list(list_expr(1))],
            vec![ValueType::List(Box::new(element_type()))],
        );
        assert_eq!(
            eval_list_expr(
                &plan,
                &mut frame,
                &ListExpr::tuple_index(tuple, 0, ValueType::String),
            ),
            Err(ExecutionError::tuple_index_family_mismatch(
                ValueType::List(Box::new(ValueType::String)),
                ValueType::List(Box::new(ValueType::Int)),
            )),
        );

        let tuple = TupleExpr::value(
            vec![Expr::list(ListExpr::value(
                vec![Expr::nil(NilExpr::value())],
                ValueType::Nil,
            ))],
            vec![ValueType::List(Box::new(ValueType::Nil))],
        );
        assert_eq!(
            eval_list_expr(
                &plan,
                &mut frame,
                &ListExpr::tuple_index(tuple, 0, ValueType::Nil),
            ),
            Ok(ListValue::nil(1)),
        );

        let tuple = TupleExpr::value(
            vec![Expr::list(ListExpr::value(
                vec![Expr::tuple(TupleExpr::value(
                    vec![Expr::int(IntExpr::value(2.into()))],
                    vec![ValueType::Int],
                ))],
                ValueType::Tuple(vec![ValueType::Int]),
            ))],
            vec![ValueType::List(Box::new(ValueType::Tuple(vec![
                ValueType::Int,
            ])))],
        );
        assert_eq!(
            eval_list_expr(
                &plan,
                &mut frame,
                &ListExpr::tuple_index(tuple, 0, ValueType::Tuple(vec![ValueType::Int])),
            ),
            Ok(ListValue::tuple(
                vec![ValueType::Int],
                vec![vec![crate::plan::Value::Int(2.into())]],
            )),
        );

        let tuple = TupleExpr::value(
            vec![Expr::list(ListExpr::value(
                vec![Expr::list(list_expr(3))],
                ValueType::List(Box::new(ValueType::Int)),
            ))],
            vec![ValueType::List(Box::new(ValueType::List(Box::new(
                ValueType::Int,
            ))))],
        );
        assert_eq!(
            eval_list_expr(
                &plan,
                &mut frame,
                &ListExpr::tuple_index(tuple, 0, ValueType::List(Box::new(ValueType::Int))),
            ),
            Ok(ListValue::list(
                ValueType::Int,
                vec![ListValue::int(vec![3.into()])],
            )),
        );

        let function_type = FunctionType::new(Vec::new(), ValueType::Int);
        let tuple = TupleExpr::value(
            vec![Expr::list(ListExpr::value(
                vec![Expr::function(FunctionExpr::value(
                    crate::plan::FunctionValue::new(
                        crate::plan::RuntimeFunctionId::Int(IntFunctionId(0)),
                        Vec::new(),
                    ),
                ))],
                ValueType::Function(Box::new(function_type.clone())),
            ))],
            vec![ValueType::List(Box::new(ValueType::Function(Box::new(
                function_type.clone(),
            ))))],
        );
        assert_eq!(
            eval_list_expr(
                &plan,
                &mut frame,
                &ListExpr::tuple_index(
                    tuple,
                    0,
                    ValueType::Function(Box::new(function_type.clone())),
                ),
            ),
            Ok(ListValue::function(
                function_type,
                vec![crate::plan::FunctionValue::new(
                    crate::plan::RuntimeFunctionId::Int(IntFunctionId(0)),
                    Vec::new(),
                )],
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
    fn runtime_list_item_project_nested_list_rejects_item_family_mismatch() {
        let plan = plan();
        let mut frame = Frame::default();
        let nested = ListExpr::value(
            vec![Expr::list(list_expr(1))],
            ValueType::List(Box::new(ValueType::Int)),
        )
        .into_list()
        .expect("nested list should build");

        assert_eq!(
            <StringListItem as RuntimeListItem>::project_nested_list(
                &plan,
                &mut frame,
                &StringListItem,
                &nested,
                0,
            ),
            Err(ExecutionError::tuple_index_family_mismatch(
                ValueType::List(Box::new(ValueType::String)),
                ValueType::List(Box::new(ValueType::Int)),
            )),
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
                    ListCaseBranches::from_exprs(vec![(1.into(), list_expr(1))], list_expr(2))
                        .expect("list case branches"),
                ),
                ValueType::Int,
            ),
            (
                ListExpr::string_case(
                    error_string_expr(),
                    ListCaseBranches::from_exprs(vec![("hit".into(), list_expr(1))], list_expr(2))
                        .expect("list case branches"),
                ),
                ValueType::String,
            ),
            (
                ListExpr::float_case(
                    error_float_expr(),
                    ListCaseBranches::from_exprs(vec![(1.0, list_expr(1))], list_expr(2))
                        .expect("list case branches"),
                ),
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

    fn all_list_family_frame() -> Frame {
        let mut layout = FrameLayout::default();
        layout.include_list(ListLocal::int(IntListLocalId(0)));
        layout.include_list(ListLocal::string(StringListLocalId(1)));
        layout.include_list(ListLocal::float(FloatListLocalId(2)));
        layout.include_list(ListLocal::bool(BoolListLocalId(3)));
        layout.include_list(ListLocal::nil(NilListLocalId(4)));
        layout.include_list(ListLocal::tuple(TupleListLocalId(5), vec![ValueType::Int]));
        layout.include_list(ListLocal::list(ListListLocalId(6), ValueType::Int));
        layout.include_list(ListLocal::function(
            FunctionListLocalId(7),
            FunctionType::new(Vec::new(), ValueType::Int),
        ));
        let mut frame = Frame::new(layout);
        frame.set_int_list(IntListLocalId(0), vec![1.into()]);
        frame.set_string_list(StringListLocalId(1), vec!["one".into()]);
        frame.set_float_list(FloatListLocalId(2), vec![1.5]);
        frame.set_bool_list(BoolListLocalId(3), vec![true]);
        frame.set_nil_list(NilListLocalId(4), 1);
        frame.set_tuple_list(
            TupleListLocalId(5),
            vec![vec![crate::plan::Value::Int(2.into())]],
        );
        frame.set_list_list(ListListLocalId(6), vec![ListValue::int(vec![3.into()])]);
        frame.set_function_list(
            FunctionListLocalId(7),
            vec![crate::plan::FunctionValue::new(
                crate::plan::RuntimeFunctionId::Int(IntFunctionId(0)),
                Vec::new(),
            )],
        );
        frame
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

    fn all_list_family_plan() -> ExecutionPlan {
        let function_type = FunctionType::new(Vec::new(), ValueType::Int);
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
                list_function_plan(
                    1,
                    "int_list",
                    ListFunctionId::from_item_type(0, ValueType::Int),
                    ListExpr::value(vec![Expr::int(IntExpr::value(1.into()))], ValueType::Int),
                ),
                list_function_plan(
                    2,
                    "string_list",
                    ListFunctionId::from_item_type(0, ValueType::String),
                    ListExpr::value(
                        vec![Expr::string(StringExpr::value("one".into()))],
                        ValueType::String,
                    ),
                ),
                list_function_plan(
                    3,
                    "float_list",
                    ListFunctionId::from_item_type(0, ValueType::Float),
                    ListExpr::value(vec![Expr::float(FloatExpr::value(1.5))], ValueType::Float),
                ),
                list_function_plan(
                    4,
                    "bool_list",
                    ListFunctionId::from_item_type(0, ValueType::Bool),
                    ListExpr::value(vec![Expr::bool(BoolExpr::value(true))], ValueType::Bool),
                ),
                list_function_plan(
                    5,
                    "nil_list",
                    ListFunctionId::from_item_type(0, ValueType::Nil),
                    ListExpr::value(vec![Expr::nil(NilExpr::value())], ValueType::Nil),
                ),
                list_function_plan(
                    6,
                    "tuple_list",
                    ListFunctionId::from_item_type(0, ValueType::Tuple(vec![ValueType::Int])),
                    ListExpr::value(
                        vec![Expr::tuple(TupleExpr::value(
                            vec![Expr::int(IntExpr::value(2.into()))],
                            vec![ValueType::Int],
                        ))],
                        ValueType::Tuple(vec![ValueType::Int]),
                    ),
                ),
                list_function_plan(
                    7,
                    "list_list",
                    ListFunctionId::from_item_type(0, ValueType::List(Box::new(ValueType::Int))),
                    ListExpr::value(
                        vec![Expr::list(ListExpr::value(
                            vec![Expr::int(IntExpr::value(3.into()))],
                            ValueType::Int,
                        ))],
                        ValueType::List(Box::new(ValueType::Int)),
                    ),
                ),
                list_function_plan(
                    8,
                    "function_list",
                    ListFunctionId::from_item_type(
                        0,
                        ValueType::Function(Box::new(function_type.clone())),
                    ),
                    ListExpr::value(
                        vec![Expr::function(FunctionExpr::value(
                            crate::plan::FunctionValue::new(
                                crate::plan::RuntimeFunctionId::Int(IntFunctionId(0)),
                                Vec::new(),
                            ),
                        ))],
                        ValueType::Function(Box::new(function_type)),
                    ),
                ),
            ],
        )
    }

    fn list_function_plan(
        function_index: usize,
        name: &str,
        runtime_id: ListFunctionId,
        expression: ListExpr,
    ) -> FunctionPlan {
        FunctionPlan::new(
            FunctionId::new(function_index),
            name.into(),
            Vec::new(),
            Vec::new(),
            ReturnExpr::list_body(runtime_id, ListReturn::expr(expression)),
        )
    }

    fn wrong_list_function_value(item_type: ValueType) -> ListFunctionExpr {
        list_function_value(ListFunctionId::from_item_type(0, item_type))
    }

    fn list_function_value(function: ListFunctionId) -> ListFunctionExpr {
        ListFunctionExpr::value(ListFunctionValue::new(function, Vec::new()))
    }

    fn assert_wrong_list_function_family<T>(actual: Result<T, ExecutionError>) {
        assert_eq!(
            actual.err().expect("call should fail"),
            ExecutionError::function_return_family_mismatch(
                FunctionReturnFamily::List,
                FunctionReturnFamily::List,
            ),
        );
    }

    fn assert_int_argument_error<T>(actual: Result<T, ExecutionError>) {
        assert_eq!(
            actual.err().expect("call should fail"),
            tuple_index_error(ValueType::Int),
        );
    }
}
