use super::{
    eval_bool_expr, eval_float_expr, eval_function_expr, eval_int_expr, eval_nil_expr,
    eval_panic_expr, eval_string_expr, eval_tuple_expr, project_tuple_expr,
};
use crate::plan::execution::ExecutionPlan;
use crate::plan::execution::{
    BoolListExpr, BoolListItem, FloatListExpr, FloatListItem, FunctionListExpr, FunctionListItem,
    IntListExpr, IntListItem, ListExpr, ListItem, ListListExpr, ListListItem, NilListExpr,
    NilListItem, StringListExpr, StringListItem, TupleListExpr, TupleListItem, TypedListExpr,
    TypedListExprKind,
};
use crate::plan::{FunctionType, ValueType};
use crate::runtime::frame::Frame;
use crate::runtime::function;
use crate::runtime::{ExecutionError, FunctionValue, ListValue, Value};
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
            .map(|values| ListValue::from_evaluated_tuple(expression.item().item_type(), values)),
        ListExpr::List(expression) => eval_list_list_expr(plan, frame, expression).map(|values| {
            ListValue::from_evaluated_list(expression.item().item_type().as_ref().clone(), values)
        }),
        ListExpr::Function(expression) => {
            eval_function_list_expr(plan, frame, expression).map(|values| {
                ListValue::from_evaluated_function(expression.item().item_type(), values)
            })
        }
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
    eval_typed_list_expr_kind(plan, frame, expression.item(), expression.kind())
}

fn eval_typed_list_expr_kind<Item: RuntimeListItem>(
    plan: &ExecutionPlan,
    frame: &mut Frame,
    item: &Item,
    kind: &TypedListExprKind<Item>,
) -> Result<Item::RuntimeValue, ExecutionError> {
    match kind {
        TypedListExprKind::Value(elements) => Item::eval_elements(plan, frame, item, elements),
        TypedListExprKind::Spread { elements, tail } => {
            let mut values = Item::eval_elements(plan, frame, item, elements)?;
            let tail_values = eval_typed_list_expr_kind(plan, frame, item, tail)?;
            Item::append(&mut values, tail_values);
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
            let expected = ValueType::List(Box::new(item.value_type()));
            match project_tuple_expr(plan, frame, tuple, *index, expected.clone())? {
                Value::List(value) => {
                    let actual = value.item_type();
                    Item::from_facade(item, value).ok_or_else(|| {
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
        TypedListExprKind::ListIndex(source) => {
            Item::project_nested_list(plan, frame, item, source.list(), source.index())
        }
        TypedListExprKind::DropFirst { list, count } => {
            let list = eval_typed_list_expr_kind(plan, frame, item, list)?;
            Ok(Item::drop_first(&list, *count))
        }
        TypedListExprKind::Panic(panic) => {
            eval_panic_expr(plan, frame, panic).map(|never| match never {})
        }
        TypedListExprKind::BoolCase {
            subject,
            true_,
            false_,
        } => {
            if eval_bool_expr(plan, frame, subject)? {
                eval_typed_list_expr_kind(plan, frame, item, true_)
            } else {
                eval_typed_list_expr_kind(plan, frame, item, false_)
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
                    return eval_typed_list_expr_kind(plan, frame, item, branch);
                }
            }
            eval_typed_list_expr_kind(plan, frame, item, fallback)
        }
        TypedListExprKind::StringCase {
            subject,
            clauses,
            fallback,
        } => {
            let subject = eval_string_expr(plan, frame, subject)?;
            for (pattern, branch) in clauses {
                if pattern == &subject {
                    return eval_typed_list_expr_kind(plan, frame, item, branch);
                }
            }
            eval_typed_list_expr_kind(plan, frame, item, fallback)
        }
        TypedListExprKind::FloatCase {
            subject,
            clauses,
            fallback,
        } => {
            let subject = eval_float_expr(plan, frame, subject)?;
            for (pattern, branch) in clauses {
                if pattern == &subject {
                    return eval_typed_list_expr_kind(plan, frame, item, branch);
                }
            }
            eval_typed_list_expr_kind(plan, frame, item, fallback)
        }
        TypedListExprKind::Block { steps, return_ } => {
            function::execute_steps(plan, steps, frame)?;
            eval_typed_list_expr_kind(plan, frame, item, return_)
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
        args: &[crate::plan::execution::CallArg],
        frame: &mut Frame,
    ) -> Result<Self::RuntimeValue, ExecutionError>;

    fn run_function_call(
        plan: &ExecutionPlan,
        function: &crate::plan::execution::ListFunctionExpr,
        args: &[crate::plan::execution::CallArg],
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
        Self::from_facade(item, value).ok_or_else(|| ExecutionError::ListIndexFamilyMismatch {
            expected: ValueType::List(Box::new(item.value_type())),
            actual: ValueType::List(Box::new(actual)),
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
                args: &[crate::plan::execution::CallArg],
                frame: &mut Frame,
            ) -> Result<Self::RuntimeValue, ExecutionError> {
                function::$run_call(plan, function, args, frame)
            }

            fn run_function_call(
                plan: &ExecutionPlan,
                function: &crate::plan::execution::ListFunctionExpr,
                args: &[crate::plan::execution::CallArg],
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
        args: &[crate::plan::execution::CallArg],
        frame: &mut Frame,
    ) -> Result<Self::RuntimeValue, ExecutionError> {
        function::run_nil_list_call(plan, function, args, frame)
    }

    fn run_function_call(
        plan: &ExecutionPlan,
        function: &crate::plan::execution::ListFunctionExpr,
        args: &[crate::plan::execution::CallArg],
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
        args: &[crate::plan::execution::CallArg],
        frame: &mut Frame,
    ) -> Result<Self::RuntimeValue, ExecutionError> {
        function::run_tuple_list_call(plan, function, args, frame)
    }

    fn run_function_call(
        plan: &ExecutionPlan,
        function: &crate::plan::execution::ListFunctionExpr,
        args: &[crate::plan::execution::CallArg],
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
        args: &[crate::plan::execution::CallArg],
        frame: &mut Frame,
    ) -> Result<Self::RuntimeValue, ExecutionError> {
        function::run_list_list_call(plan, function, args, frame)
    }

    fn run_function_call(
        plan: &ExecutionPlan,
        function: &crate::plan::execution::ListFunctionExpr,
        args: &[crate::plan::execution::CallArg],
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
        args: &[crate::plan::execution::CallArg],
        frame: &mut Frame,
    ) -> Result<Self::RuntimeValue, ExecutionError> {
        function::run_function_list_call(plan, function, args, frame)
    }

    fn run_function_call(
        plan: &ExecutionPlan,
        function: &crate::plan::execution::ListFunctionExpr,
        args: &[crate::plan::execution::CallArg],
        frame: &mut Frame,
    ) -> Result<Self::RuntimeValue, ExecutionError> {
        function::run_function_list_function_call(plan, function, args, frame)
    }
}

pub(in crate::runtime) fn get_list_value(
    frame: &Frame,
    local: &crate::plan::execution::ListLocal,
) -> ListValue {
    match local {
        crate::plan::execution::ListLocal::Int(local) => ListValue::int(frame.get_int_list(*local)),
        crate::plan::execution::ListLocal::String(local) => {
            ListValue::string(frame.get_string_list(*local))
        }
        crate::plan::execution::ListLocal::Float(local) => {
            ListValue::float(frame.get_float_list(*local))
        }
        crate::plan::execution::ListLocal::Bool(local) => {
            ListValue::bool(frame.get_bool_list(*local))
        }
        crate::plan::execution::ListLocal::Nil(local) => ListValue::nil(frame.get_nil_list(*local)),
        crate::plan::execution::ListLocal::Tuple { local, item_type } => {
            ListValue::from_evaluated_tuple(item_type.clone(), frame.get_tuple_list(*local))
        }
        crate::plan::execution::ListLocal::List { local, item_type } => {
            ListValue::from_evaluated_list(item_type.as_ref().clone(), frame.get_list_list(*local))
        }
        crate::plan::execution::ListLocal::Function { local, item_type } => {
            ListValue::from_evaluated_function(item_type.clone(), frame.get_function_list(*local))
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
        RuntimeListItem, project_bool_list_expr, project_float_list_expr,
        project_function_list_expr, project_int_list_expr, project_nil_list_expr,
        project_string_list_expr, project_tuple_list_expr,
    };
    use crate::plan::execution::{
        BoolListFunctionId, FloatListFunctionId, FunctionListFunctionId, IntListFunctionId,
        IntListItem, ListListFunctionId, ListListLocalId, NilListFunctionId, ReturnBody,
        ReturnBodyKind, StringListFunctionId, TupleListFunctionId,
    };
    use crate::plan::{
        BoolExpr as ModuleBoolExpr, BoolListCaseBranches, Expr as ModuleExpr,
        FloatExpr as ModuleFloatExpr, FunctionId as ModuleFunctionId,
        FunctionPlan as ModuleFunctionPlan, FunctionType, IntExpr as ModuleIntExpr,
        IntListExpr as ModuleIntListExpr, IntListFunctionId as ModuleIntListFunctionId,
        ListCaseBranches, ListExpr as ModuleListExpr, ListListExpr as ModuleListListExpr,
        ModulePlan, NilExpr as ModuleNilExpr, NilListFunctionId as ModuleNilListFunctionId,
        PanicExpr as ModulePanicExpr, PanicSite, ReturnBody as ModuleReturnBody,
        ReturnExpr as ModuleReturnExpr, Step as ModuleStep, StringExpr as ModuleStringExpr,
        TupleExpr as ModuleTupleExpr, ValueType,
    };
    use crate::runtime::frame::Frame;
    use crate::runtime::{ExecutionError, ListValue};
    use num_bigint::BigInt;

    #[test]
    fn source_list_expression_item_families_evaluate_exact_values() {
        let source = include_str!(
            "../../../tests/fixtures/execution/values/list_expression_item_families.gleam"
        );

        assert_eq!(
            crate::runtime::run_src(source),
            crate::runtime::Value::Int(BigInt::from(42)),
        );
    }

    #[test]
    fn list_projection_reports_out_of_bounds_for_every_item_family() {
        let plan = crate::runtime::plan_src(
            r#"
fn int_values(values: List(Int)) { values }
fn string_values(values: List(String)) { values }
fn float_values(values: List(Float)) { values }
fn bool_values(values: List(Bool)) { values }
fn nil_values(values: List(Nil)) { values }
fn tuple_values(values: List(#(Int))) { values }
fn nested_values(values: List(List(Int))) { values }
fn function_values(values: List(fn() -> Int)) { values }

pub fn main() { Nil }
"#,
        );

        let function = plan.int_list_function(IntListFunctionId(0));
        let expression = expect_expression_return(function.return_());
        let mut frame = Frame::new(function.frame_layout());
        assert_eq!(
            project_int_list_expr(&plan, &mut frame, expression, 0),
            Err(ExecutionError::list_index_out_of_bounds(
                ValueType::Int,
                0,
                0,
            )),
        );

        let function = plan.string_list_function(StringListFunctionId(0));
        let expression = expect_expression_return(function.return_());
        let mut frame = Frame::new(function.frame_layout());
        assert_eq!(
            project_string_list_expr(&plan, &mut frame, expression, 0),
            Err(ExecutionError::list_index_out_of_bounds(
                ValueType::String,
                0,
                0,
            )),
        );

        let function = plan.float_list_function(FloatListFunctionId(0));
        let expression = expect_expression_return(function.return_());
        let mut frame = Frame::new(function.frame_layout());
        assert_eq!(
            project_float_list_expr(&plan, &mut frame, expression, 0),
            Err(ExecutionError::list_index_out_of_bounds(
                ValueType::Float,
                0,
                0,
            )),
        );

        let function = plan.bool_list_function(BoolListFunctionId(0));
        let expression = expect_expression_return(function.return_());
        let mut frame = Frame::new(function.frame_layout());
        assert_eq!(
            project_bool_list_expr(&plan, &mut frame, expression, 0),
            Err(ExecutionError::list_index_out_of_bounds(
                ValueType::Bool,
                0,
                0,
            )),
        );

        let function = plan.nil_list_function(NilListFunctionId(0));
        let expression = expect_expression_return(function.return_());
        let mut frame = Frame::new(function.frame_layout());
        assert_eq!(
            project_nil_list_expr(&plan, &mut frame, expression, 0),
            Err(ExecutionError::list_index_out_of_bounds(
                ValueType::Nil,
                0,
                0,
            )),
        );

        let function = plan.tuple_list_function(TupleListFunctionId(0));
        let expression = expect_expression_return(function.return_());
        let mut frame = Frame::new(function.frame_layout());
        assert_eq!(
            project_tuple_list_expr(&plan, &mut frame, expression, 0, &[ValueType::Int]),
            Err(ExecutionError::list_index_out_of_bounds(
                ValueType::Tuple(vec![ValueType::Int]),
                0,
                0,
            )),
        );

        let function = plan.function_list_function(FunctionListFunctionId(0));
        let expression = expect_expression_return(function.return_());
        let mut frame = Frame::new(function.frame_layout());
        let function_type = FunctionType::new(Vec::new(), ValueType::Int);
        assert_eq!(
            project_function_list_expr(&plan, &mut frame, expression, 0, &function_type),
            Err(ExecutionError::list_index_out_of_bounds(
                ValueType::Function(Box::new(function_type)),
                0,
                0,
            )),
        );
    }

    #[test]
    fn nested_list_projection_has_only_index_and_family_invariants() {
        let plan = crate::runtime::plan_src(
            r#"
fn nested_values(values: List(List(Int))) { values }
pub fn main() { Nil }
"#,
        );
        let function = plan.list_list_function(ListListFunctionId(0));
        let expression = expect_expression_return(function.return_());
        let mut frame = Frame::new(function.frame_layout());

        assert_eq!(
            IntListItem::project_nested_list(&plan, &mut frame, &IntListItem, expression, 0),
            Err(ExecutionError::list_index_out_of_bounds(
                ValueType::List(Box::new(ValueType::Int)),
                0,
                0,
            )),
        );

        frame.set_list_list(
            ListListLocalId(0),
            vec![ListValue::string(vec!["wrong".into()])],
        );
        assert_eq!(
            IntListItem::project_nested_list(&plan, &mut frame, &IntListItem, expression, 0),
            Err(ExecutionError::ListIndexFamilyMismatch {
                expected: ValueType::List(Box::new(ValueType::Int)),
                actual: ValueType::List(Box::new(ValueType::String)),
            }),
        );
    }

    #[test]
    fn module_expression_errors_propagate_through_list_wrappers() {
        let panic = |message: &str| {
            ModulePanicExpr::panic_at(
                Some(ModuleStringExpr::value(message.into())),
                PanicSite::unknown(),
            )
        };
        let int_panic =
            |message| expect_module_int_list(ModuleListExpr::panic(panic(message), ValueType::Int));
        let fallback = || int_panic("fallback");
        let expressions = [
            (
                expect_module_int_list(ModuleListExpr::from_spread_elements(
                    crate::plan::ListSpreadElements::Int {
                        values: vec![ModuleIntExpr::panic(panic("prefix"))],
                        tail: int_panic("tail fallback"),
                    },
                )),
                "prefix",
            ),
            (
                expect_module_int_list(ModuleListExpr::from_spread_elements(
                    crate::plan::ListSpreadElements::Int {
                        values: Vec::new(),
                        tail: int_panic("tail"),
                    },
                )),
                "tail",
            ),
            (
                expect_module_int_list(ModuleListExpr::tuple_index(
                    ModuleTupleExpr::panic(
                        panic("tuple"),
                        vec![ValueType::List(Box::new(ValueType::Int))],
                    ),
                    0,
                    ValueType::Int,
                )),
                "tuple",
            ),
            (
                expect_module_int_list(ModuleListExpr::drop_first(
                    ModuleListExpr::panic(panic("drop"), ValueType::Int),
                    1,
                )),
                "drop",
            ),
            (
                expect_module_int_list(ModuleListExpr::bool_case(
                    ModuleBoolExpr::panic(panic("bool subject")),
                    BoolListCaseBranches::Int {
                        true_: fallback(),
                        false_: fallback(),
                    },
                )),
                "bool subject",
            ),
            (
                expect_module_int_list(ModuleListExpr::int_case(
                    ModuleIntExpr::panic(panic("int subject")),
                    ListCaseBranches::Int {
                        clauses: Vec::new(),
                        fallback: fallback(),
                    },
                )),
                "int subject",
            ),
            (
                expect_module_int_list(ModuleListExpr::string_case(
                    ModuleStringExpr::panic(panic("string subject")),
                    ListCaseBranches::Int {
                        clauses: Vec::new(),
                        fallback: fallback(),
                    },
                )),
                "string subject",
            ),
            (
                expect_module_int_list(ModuleListExpr::float_case(
                    ModuleFloatExpr::panic(panic("float subject")),
                    ListCaseBranches::Int {
                        clauses: Vec::new(),
                        fallback: fallback(),
                    },
                )),
                "float subject",
            ),
            (
                expect_module_int_list(ModuleListExpr::block(
                    vec![ModuleStep::evaluate(ModuleExpr::bool(
                        ModuleBoolExpr::panic(panic("step")),
                    ))],
                    ModuleListExpr::panic(panic("fallback"), ValueType::Int),
                )),
                "step",
            ),
            (
                expect_module_int_list(ModuleListExpr::list_index(
                    expect_module_list_list(ModuleListExpr::panic(
                        panic("nested list"),
                        ValueType::List(Box::new(ValueType::Int)),
                    )),
                    0,
                )),
                "nested list",
            ),
        ];

        for (expression, message) in expressions {
            assert_eq!(
                run_module_int_list_expression(expression).to_string(),
                format!("panic: {message}"),
            );
        }
    }

    #[test]
    fn nil_list_element_errors_propagate() {
        let panic = ModulePanicExpr::panic_at(
            Some(ModuleStringExpr::value("nil element".into())),
            PanicSite::unknown(),
        );
        let expression = expect_module_nil_list(ModuleListExpr::from_elements(
            crate::plan::ListElements::Nil(vec![ModuleNilExpr::panic(panic)]),
        ));

        assert_eq!(
            run_module_nil_list_expression(expression).to_string(),
            "panic: nil element",
        );
    }

    #[test]
    fn list_projection_propagates_source_errors_for_every_item_family() {
        let plans = [
            crate::runtime::plan_src("pub fn main() -> List(Int) { panic as \"int\" }"),
            crate::runtime::plan_src("pub fn main() -> List(String) { panic as \"string\" }"),
            crate::runtime::plan_src("pub fn main() -> List(Float) { panic as \"float\" }"),
            crate::runtime::plan_src("pub fn main() -> List(Bool) { panic as \"bool\" }"),
            crate::runtime::plan_src("pub fn main() -> List(Nil) { panic as \"nil\" }"),
            crate::runtime::plan_src("pub fn main() -> List(#(Int)) { panic as \"tuple\" }"),
            crate::runtime::plan_src(
                "pub fn main() -> List(fn() -> Int) { panic as \"function\" }",
            ),
        ];

        let function = plans[0].int_list_function(IntListFunctionId(0));
        let mut frame = Frame::new(function.frame_layout());
        assert_eq!(
            project_int_list_expr(
                &plans[0],
                &mut frame,
                expect_expression_return(function.return_()),
                0,
            )
            .expect_err("list source should fail")
            .to_string(),
            "panic: int",
        );

        let function = plans[1].string_list_function(StringListFunctionId(0));
        let mut frame = Frame::new(function.frame_layout());
        assert_eq!(
            project_string_list_expr(
                &plans[1],
                &mut frame,
                expect_expression_return(function.return_()),
                0,
            )
            .expect_err("list source should fail")
            .to_string(),
            "panic: string",
        );

        let function = plans[2].float_list_function(FloatListFunctionId(0));
        let mut frame = Frame::new(function.frame_layout());
        assert_eq!(
            project_float_list_expr(
                &plans[2],
                &mut frame,
                expect_expression_return(function.return_()),
                0,
            )
            .expect_err("list source should fail")
            .to_string(),
            "panic: float",
        );

        let function = plans[3].bool_list_function(BoolListFunctionId(0));
        let mut frame = Frame::new(function.frame_layout());
        assert_eq!(
            project_bool_list_expr(
                &plans[3],
                &mut frame,
                expect_expression_return(function.return_()),
                0,
            )
            .expect_err("list source should fail")
            .to_string(),
            "panic: bool",
        );

        let function = plans[4].nil_list_function(NilListFunctionId(0));
        let mut frame = Frame::new(function.frame_layout());
        assert_eq!(
            project_nil_list_expr(
                &plans[4],
                &mut frame,
                expect_expression_return(function.return_()),
                0,
            )
            .expect_err("list source should fail")
            .to_string(),
            "panic: nil",
        );

        let function = plans[5].tuple_list_function(TupleListFunctionId(0));
        let mut frame = Frame::new(function.frame_layout());
        assert_eq!(
            project_tuple_list_expr(
                &plans[5],
                &mut frame,
                expect_expression_return(function.return_()),
                0,
                &[ValueType::Int],
            )
            .expect_err("list source should fail")
            .to_string(),
            "panic: tuple",
        );

        let function = plans[6].function_list_function(FunctionListFunctionId(0));
        let mut frame = Frame::new(function.frame_layout());
        assert_eq!(
            project_function_list_expr(
                &plans[6],
                &mut frame,
                expect_expression_return(function.return_()),
                0,
                &FunctionType::new(Vec::new(), ValueType::Int),
            )
            .expect_err("list source should fail")
            .to_string(),
            "panic: function",
        );
    }

    #[test]
    #[should_panic(expected = "expected a list expression return body")]
    fn list_expression_return_shape_guard_rejects_tail_calls() {
        let plan = crate::runtime::plan_src(
            r#"
fn recurse() -> List(Int) { recurse() }
pub fn main() { Nil }
"#,
        );
        let function = plan.int_list_function(IntListFunctionId(0));

        let _ = expect_expression_return(function.return_());
    }

    #[test]
    #[should_panic(expected = "expected an Int list expression")]
    fn int_list_shape_guard_rejects_string_lists() {
        let expression = ModuleListExpr::panic(
            ModulePanicExpr::panic_at(None, PanicSite::unknown()),
            ValueType::String,
        );

        let _ = expect_module_int_list(expression);
    }

    #[test]
    #[should_panic(expected = "expected a nested list expression")]
    fn nested_list_shape_guard_rejects_int_lists() {
        let expression = ModuleListExpr::panic(
            ModulePanicExpr::panic_at(None, PanicSite::unknown()),
            ValueType::Int,
        );

        let _ = expect_module_list_list(expression);
    }

    #[test]
    #[should_panic(expected = "expected a Nil list expression")]
    fn nil_list_shape_guard_rejects_int_lists() {
        let expression = ModuleListExpr::panic(
            ModulePanicExpr::panic_at(None, PanicSite::unknown()),
            ValueType::Int,
        );

        let _ = expect_module_nil_list(expression);
    }

    fn run_module_int_list_expression(expression: ModuleIntListExpr) -> ExecutionError {
        let main = ModuleFunctionPlan::new(
            ModuleFunctionId::new(0),
            "main".into(),
            Vec::new(),
            Vec::new(),
            ModuleReturnExpr::int_list_body(
                ModuleIntListFunctionId(0),
                ModuleReturnBody::expr(expression),
            ),
        );
        let module = ModulePlan::new("main".into(), main, Vec::new());
        let plan = crate::ExecutionPlan::from_module_plan(module);

        crate::runtime::run_main(&plan).expect_err("module expression should fail at runtime")
    }

    fn run_module_nil_list_expression(expression: crate::plan::NilListExpr) -> ExecutionError {
        let main = ModuleFunctionPlan::new(
            ModuleFunctionId::new(0),
            "main".into(),
            Vec::new(),
            Vec::new(),
            ModuleReturnExpr::nil_list_body(
                ModuleNilListFunctionId(0),
                ModuleReturnBody::expr(expression),
            ),
        );
        let module = ModulePlan::new("main".into(), main, Vec::new());
        let plan = crate::ExecutionPlan::from_module_plan(module);

        crate::runtime::run_main(&plan).expect_err("module expression should fail at runtime")
    }

    fn expect_module_int_list(expression: ModuleListExpr) -> ModuleIntListExpr {
        match expression {
            ModuleListExpr::Int(expression) => expression,
            _ => panic!("expected an Int list expression"),
        }
    }

    fn expect_module_list_list(expression: ModuleListExpr) -> ModuleListListExpr {
        match expression {
            ModuleListExpr::List(expression) => expression,
            _ => panic!("expected a nested list expression"),
        }
    }

    fn expect_module_nil_list(expression: ModuleListExpr) -> crate::plan::NilListExpr {
        match expression {
            ModuleListExpr::Nil(expression) => expression,
            _ => panic!("expected a Nil list expression"),
        }
    }

    fn expect_expression_return<Expression, Function>(
        body: &ReturnBody<Expression, Function>,
    ) -> &Expression {
        match body.kind() {
            ReturnBodyKind::Expr(expression) => expression,
            _ => panic!("expected a list expression return body"),
        }
    }
}
