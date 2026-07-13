use ecow::EcoString;
use num_bigint::BigInt;

use super::{
    eval_bit_array_expr, eval_bool_expr, eval_float_expr, eval_function_expr, eval_int_expr,
    eval_nil_expr, eval_panic_expr, eval_string_expr, eval_tuple_expr, project_tuple_expr,
};
use crate::plan::execution::{
    BitArrayListExpr, BitArrayListItem, BoolListExpr, BoolListItem, ExecutionPlan, FloatListExpr,
    FloatListItem, FunctionListExpr, FunctionListItem, IntListExpr, IntListItem, ListExpr,
    ListItem, ListListExpr, ListListItem, NilListExpr, NilListItem, StringListExpr, StringListItem,
    TupleListExpr, TupleListItem, TypedListExpr, TypedListExprKind,
};
use crate::plan::{FunctionType, ValueType};
use crate::runtime::ExecutionError;
use crate::runtime::evaluated::{EvaluatedBitArray, EvaluatedFunctionValue, EvaluatedValue};
use crate::runtime::frame::Frame;
use crate::runtime::function;
use crate::runtime::state::{
    BitArrayListValueId, BoolListValueId, FloatListValueId, FunctionListValueId, IntListValueId,
    ListHandleCore, ListListValueId, ListValueId, NilListValueId, RuntimeState, StringListValueId,
    TupleListValueId,
};

pub(in crate::runtime) fn eval_list_expr(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
    frame: &mut Frame,
    expression: &ListExpr,
) -> Result<ListValueId, ExecutionError> {
    match expression {
        ListExpr::Int(expression) => {
            eval_int_list_expr(plan, state, frame, expression).map(Into::into)
        }
        ListExpr::String(expression) => {
            eval_string_list_expr(plan, state, frame, expression).map(Into::into)
        }
        ListExpr::BitArray(expression) => {
            eval_bit_array_list_expr(plan, state, frame, expression).map(Into::into)
        }
        ListExpr::Float(expression) => {
            eval_float_list_expr(plan, state, frame, expression).map(Into::into)
        }
        ListExpr::Bool(expression) => {
            eval_bool_list_expr(plan, state, frame, expression).map(Into::into)
        }
        ListExpr::Nil(expression) => {
            eval_nil_list_expr(plan, state, frame, expression).map(Into::into)
        }
        ListExpr::Tuple(expression) => {
            eval_tuple_list_expr(plan, state, frame, expression).map(Into::into)
        }
        ListExpr::List(expression) => {
            eval_list_list_expr(plan, state, frame, expression).map(Into::into)
        }
        ListExpr::Function(expression) => {
            eval_function_list_expr(plan, state, frame, expression).map(Into::into)
        }
    }
}

pub(in crate::runtime) fn eval_int_list_expr(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
    frame: &mut Frame,
    expression: &IntListExpr,
) -> Result<IntListValueId, ExecutionError> {
    eval_typed_list_expr(plan, state, frame, expression)
}

pub(in crate::runtime) fn eval_string_list_expr(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
    frame: &mut Frame,
    expression: &StringListExpr,
) -> Result<StringListValueId, ExecutionError> {
    eval_typed_list_expr(plan, state, frame, expression)
}

pub(in crate::runtime) fn eval_bit_array_list_expr(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
    frame: &mut Frame,
    expression: &BitArrayListExpr,
) -> Result<BitArrayListValueId, ExecutionError> {
    eval_typed_list_expr(plan, state, frame, expression)
}

pub(in crate::runtime) fn eval_float_list_expr(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
    frame: &mut Frame,
    expression: &FloatListExpr,
) -> Result<FloatListValueId, ExecutionError> {
    eval_typed_list_expr(plan, state, frame, expression)
}

pub(in crate::runtime) fn eval_bool_list_expr(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
    frame: &mut Frame,
    expression: &BoolListExpr,
) -> Result<BoolListValueId, ExecutionError> {
    eval_typed_list_expr(plan, state, frame, expression)
}

pub(in crate::runtime) fn eval_nil_list_expr(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
    frame: &mut Frame,
    expression: &NilListExpr,
) -> Result<NilListValueId, ExecutionError> {
    eval_typed_list_expr(plan, state, frame, expression)
}

pub(in crate::runtime) fn eval_tuple_list_expr(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
    frame: &mut Frame,
    expression: &TupleListExpr,
) -> Result<TupleListValueId, ExecutionError> {
    eval_typed_list_expr(plan, state, frame, expression)
}

pub(in crate::runtime) fn eval_list_list_expr(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
    frame: &mut Frame,
    expression: &ListListExpr,
) -> Result<ListListValueId, ExecutionError> {
    eval_typed_list_expr(plan, state, frame, expression)
}

pub(in crate::runtime) fn eval_function_list_expr(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
    frame: &mut Frame,
    expression: &FunctionListExpr,
) -> Result<FunctionListValueId, ExecutionError> {
    eval_typed_list_expr(plan, state, frame, expression)
}

fn eval_typed_list_expr<Item: RuntimeListItem>(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
    frame: &mut Frame,
    expression: &TypedListExpr<Item>,
) -> Result<Item::Handle, ExecutionError> {
    eval_typed_list_expr_kind(plan, state, frame, expression.item(), expression.kind())
}

fn eval_typed_list_expr_kind<Item: RuntimeListItem>(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
    frame: &mut Frame,
    item: &Item,
    kind: &TypedListExprKind<Item>,
) -> Result<Item::Handle, ExecutionError> {
    match kind {
        TypedListExprKind::Value(elements) => {
            let values = Item::eval_elements(plan, state, frame, elements)?;
            Ok(Item::allocate(state, item, values))
        }
        TypedListExprKind::Spread { elements, tail } => {
            let mut values = Item::eval_elements(plan, state, frame, elements)?;
            let tail = eval_typed_list_expr_kind(plan, state, frame, item, tail)?;
            Item::append(state, &mut values, &tail);
            Ok(Item::allocate(state, item, values))
        }
        TypedListExprKind::LocalGet { local } => Ok(Item::get_local(frame, local.clone())),
        TypedListExprKind::Call { function, args } => {
            Item::run_call(plan, state, function.clone(), args, frame)
        }
        TypedListExprKind::FunctionCall { function, args } => {
            Item::run_function_call(plan, state, function, args, frame)
        }
        TypedListExprKind::TupleIndex { tuple, index } => {
            let expected = plan.list_value_type(item.list_type());
            match project_tuple_expr(plan, state, frame, tuple, *index, expected.clone())? {
                EvaluatedValue::List(value) => {
                    let actual = plan.list_value_type(value.list_type());
                    Item::from_tuple_value(value).ok_or_else(|| {
                        ExecutionError::TupleIndexFamilyMismatch { expected, actual }
                    })
                }
                other => Err(ExecutionError::TupleIndexFamilyMismatch {
                    expected,
                    actual: other.value_type(plan),
                }),
            }
        }
        TypedListExprKind::ListIndex(source) => {
            let list = eval_list_list_expr(plan, state, frame, source.list())?;
            let values = state.list_values(&list);
            let Some(value) = values.get(source.index()).cloned() else {
                return Err(ExecutionError::ListIndexOutOfBounds {
                    item_type: plan.list_value_type(item.list_type()),
                    index: source.index(),
                    length: values.len(),
                });
            };
            Ok(Item::from_core(item, value))
        }
        TypedListExprKind::DropFirst { list, count } => {
            let list = eval_typed_list_expr_kind(plan, state, frame, item, list)?;
            let values = Item::drop_first(state, &list, *count);
            Ok(Item::allocate(state, item, values))
        }
        TypedListExprKind::Panic(panic) => {
            eval_panic_expr(plan, state, frame, panic).map(|never| match never {})
        }
        TypedListExprKind::BoolCase {
            subject,
            true_,
            false_,
        } => {
            if eval_bool_expr(plan, state, frame, subject)? {
                eval_typed_list_expr_kind(plan, state, frame, item, true_)
            } else {
                eval_typed_list_expr_kind(plan, state, frame, item, false_)
            }
        }
        TypedListExprKind::IntCase {
            subject,
            clauses,
            fallback,
        } => {
            let subject = eval_int_expr(plan, state, frame, subject)?;
            for (pattern, branch) in clauses {
                if pattern == &subject {
                    return eval_typed_list_expr_kind(plan, state, frame, item, branch);
                }
            }
            eval_typed_list_expr_kind(plan, state, frame, item, fallback)
        }
        TypedListExprKind::StringCase {
            subject,
            clauses,
            fallback,
        } => {
            let subject = eval_string_expr(plan, state, frame, subject)?;
            for (pattern, branch) in clauses {
                if pattern == &subject {
                    return eval_typed_list_expr_kind(plan, state, frame, item, branch);
                }
            }
            eval_typed_list_expr_kind(plan, state, frame, item, fallback)
        }
        TypedListExprKind::FloatCase {
            subject,
            clauses,
            fallback,
        } => {
            let subject = eval_float_expr(plan, state, frame, subject)?;
            for (pattern, branch) in clauses {
                if pattern == &subject {
                    return eval_typed_list_expr_kind(plan, state, frame, item, branch);
                }
            }
            eval_typed_list_expr_kind(plan, state, frame, item, fallback)
        }
        TypedListExprKind::Block { steps, return_ } => {
            function::execute_steps(plan, state, steps, frame)?;
            eval_typed_list_expr_kind(plan, state, frame, item, return_)
        }
    }
}

trait RuntimeListItem: ListItem {
    type Values;
    type Handle: Clone + Into<ListValueId>;

    fn eval_elements(
        plan: &ExecutionPlan,
        state: &mut RuntimeState,
        frame: &mut Frame,
        elements: &[Self::ElementExpr],
    ) -> Result<Self::Values, ExecutionError>;

    fn allocate(state: &mut RuntimeState, item: &Self, values: Self::Values) -> Self::Handle;

    fn append(state: &RuntimeState, values: &mut Self::Values, tail: &Self::Handle);

    fn drop_first(state: &RuntimeState, values: &Self::Handle, count: usize) -> Self::Values;

    fn from_tuple_value(value: ListValueId) -> Option<Self::Handle>;

    fn from_core(item: &Self, core: ListHandleCore) -> Self::Handle;

    fn get_local(frame: &Frame, local: Self::Local) -> Self::Handle;

    fn run_call(
        plan: &ExecutionPlan,
        state: &mut RuntimeState,
        function: Self::Function,
        args: &[crate::plan::execution::CallArg],
        frame: &mut Frame,
    ) -> Result<Self::Handle, ExecutionError>;

    fn run_function_call(
        plan: &ExecutionPlan,
        state: &mut RuntimeState,
        function: &crate::plan::execution::ListFunctionExpr,
        args: &[crate::plan::execution::CallArg],
        frame: &mut Frame,
    ) -> Result<Self::Handle, ExecutionError>;
}

macro_rules! primitive_runtime_list_item {
    (
        $item:ty,
        $value:ty,
        $handle:ty,
        $variant:ident,
        $element_eval:ident,
        $state_values:ident,
        $state_allocate:ident,
        $get_local:ident,
        $run_call:ident,
        $run_function_call:ident
    ) => {
        impl RuntimeListItem for $item {
            type Values = Vec<$value>;
            type Handle = $handle;

            fn eval_elements(
                plan: &ExecutionPlan,
                state: &mut RuntimeState,
                frame: &mut Frame,
                elements: &[Self::ElementExpr],
            ) -> Result<Self::Values, ExecutionError> {
                elements
                    .iter()
                    .map(|element| $element_eval(plan, state, frame, element))
                    .collect()
            }

            fn allocate(
                state: &mut RuntimeState,
                item: &Self,
                values: Self::Values,
            ) -> Self::Handle {
                state.$state_allocate(item.type_id(), values)
            }

            fn append(state: &RuntimeState, values: &mut Self::Values, tail: &Self::Handle) {
                values.extend(state.$state_values(tail).iter().cloned());
            }

            fn drop_first(
                state: &RuntimeState,
                values: &Self::Handle,
                count: usize,
            ) -> Self::Values {
                let values = state.$state_values(values);
                values[count.min(values.len())..].to_vec()
            }

            fn from_tuple_value(value: ListValueId) -> Option<Self::Handle> {
                match value {
                    ListValueId::$variant(value) => Some(value),
                    _ => None,
                }
            }

            fn from_core(item: &Self, core: ListHandleCore) -> Self::Handle {
                <$handle>::new(item.type_id(), core)
            }

            fn get_local(frame: &Frame, local: Self::Local) -> Self::Handle {
                frame.$get_local(local)
            }

            fn run_call(
                plan: &ExecutionPlan,
                state: &mut RuntimeState,
                function: Self::Function,
                args: &[crate::plan::execution::CallArg],
                frame: &mut Frame,
            ) -> Result<Self::Handle, ExecutionError> {
                function::$run_call(plan, state, function, args, frame)
            }

            fn run_function_call(
                plan: &ExecutionPlan,
                state: &mut RuntimeState,
                function: &crate::plan::execution::ListFunctionExpr,
                args: &[crate::plan::execution::CallArg],
                frame: &mut Frame,
            ) -> Result<Self::Handle, ExecutionError> {
                function::$run_function_call(plan, state, function, args, frame)
            }
        }
    };
}

primitive_runtime_list_item!(
    IntListItem,
    BigInt,
    IntListValueId,
    Int,
    eval_int_expr,
    int_values,
    int,
    get_int_list,
    run_int_list_call,
    run_int_list_function_call
);
primitive_runtime_list_item!(
    StringListItem,
    EcoString,
    StringListValueId,
    String,
    eval_string_expr,
    string_values,
    string,
    get_string_list,
    run_string_list_call,
    run_string_list_function_call
);
primitive_runtime_list_item!(
    BitArrayListItem,
    EvaluatedBitArray,
    BitArrayListValueId,
    BitArray,
    eval_bit_array_expr,
    bit_array_values,
    bit_array,
    get_bit_array_list,
    run_bit_array_list_call,
    run_bit_array_list_function_call
);
primitive_runtime_list_item!(
    FloatListItem,
    f64,
    FloatListValueId,
    Float,
    eval_float_expr,
    float_values,
    float,
    get_float_list,
    run_float_list_call,
    run_float_list_function_call
);
primitive_runtime_list_item!(
    BoolListItem,
    bool,
    BoolListValueId,
    Bool,
    eval_bool_expr,
    bool_values,
    bool,
    get_bool_list,
    run_bool_list_call,
    run_bool_list_function_call
);

impl RuntimeListItem for NilListItem {
    type Values = usize;
    type Handle = NilListValueId;

    fn eval_elements(
        plan: &ExecutionPlan,
        state: &mut RuntimeState,
        frame: &mut Frame,
        elements: &[Self::ElementExpr],
    ) -> Result<Self::Values, ExecutionError> {
        for element in elements {
            eval_nil_expr(plan, state, frame, element)?;
        }
        Ok(elements.len())
    }

    fn allocate(state: &mut RuntimeState, item: &Self, values: Self::Values) -> Self::Handle {
        state.nil(item.type_id(), values)
    }

    fn append(state: &RuntimeState, values: &mut Self::Values, tail: &Self::Handle) {
        *values += state.nil_len(tail);
    }

    fn drop_first(state: &RuntimeState, values: &Self::Handle, count: usize) -> Self::Values {
        state.nil_len(values).saturating_sub(count)
    }

    fn from_tuple_value(value: ListValueId) -> Option<Self::Handle> {
        match value {
            ListValueId::Nil(value) => Some(value),
            _ => None,
        }
    }

    fn from_core(item: &Self, core: ListHandleCore) -> Self::Handle {
        NilListValueId::new(item.type_id(), core)
    }

    fn get_local(frame: &Frame, local: Self::Local) -> Self::Handle {
        frame.get_nil_list(local)
    }

    fn run_call(
        plan: &ExecutionPlan,
        state: &mut RuntimeState,
        function: Self::Function,
        args: &[crate::plan::execution::CallArg],
        frame: &mut Frame,
    ) -> Result<Self::Handle, ExecutionError> {
        function::run_nil_list_call(plan, state, function, args, frame)
    }

    fn run_function_call(
        plan: &ExecutionPlan,
        state: &mut RuntimeState,
        function: &crate::plan::execution::ListFunctionExpr,
        args: &[crate::plan::execution::CallArg],
        frame: &mut Frame,
    ) -> Result<Self::Handle, ExecutionError> {
        function::run_nil_list_function_call(plan, state, function, args, frame)
    }
}

impl RuntimeListItem for TupleListItem {
    type Values = Vec<Vec<EvaluatedValue>>;
    type Handle = TupleListValueId;

    fn eval_elements(
        plan: &ExecutionPlan,
        state: &mut RuntimeState,
        frame: &mut Frame,
        elements: &[Self::ElementExpr],
    ) -> Result<Self::Values, ExecutionError> {
        elements
            .iter()
            .map(|element| eval_tuple_expr(plan, state, frame, element))
            .collect()
    }

    fn allocate(state: &mut RuntimeState, item: &Self, values: Self::Values) -> Self::Handle {
        state.tuple(item.type_id(), values)
    }

    fn append(state: &RuntimeState, values: &mut Self::Values, tail: &Self::Handle) {
        values.extend(state.tuple_values(tail).iter().cloned());
    }

    fn drop_first(state: &RuntimeState, values: &Self::Handle, count: usize) -> Self::Values {
        let values = state.tuple_values(values);
        values[count.min(values.len())..].to_vec()
    }

    fn from_tuple_value(value: ListValueId) -> Option<Self::Handle> {
        match value {
            ListValueId::Tuple(value) => Some(value),
            _ => None,
        }
    }

    fn from_core(item: &Self, core: ListHandleCore) -> Self::Handle {
        TupleListValueId::new(item.type_id(), core)
    }

    fn get_local(frame: &Frame, local: Self::Local) -> Self::Handle {
        frame.get_tuple_list(local)
    }

    fn run_call(
        plan: &ExecutionPlan,
        state: &mut RuntimeState,
        function: Self::Function,
        args: &[crate::plan::execution::CallArg],
        frame: &mut Frame,
    ) -> Result<Self::Handle, ExecutionError> {
        function::run_tuple_list_call(plan, state, function, args, frame)
    }

    fn run_function_call(
        plan: &ExecutionPlan,
        state: &mut RuntimeState,
        function: &crate::plan::execution::ListFunctionExpr,
        args: &[crate::plan::execution::CallArg],
        frame: &mut Frame,
    ) -> Result<Self::Handle, ExecutionError> {
        function::run_tuple_list_function_call(plan, state, function, args, frame)
    }
}

impl RuntimeListItem for ListListItem {
    type Values = Vec<ListHandleCore>;
    type Handle = ListListValueId;

    fn eval_elements(
        plan: &ExecutionPlan,
        state: &mut RuntimeState,
        frame: &mut Frame,
        elements: &[Self::ElementExpr],
    ) -> Result<Self::Values, ExecutionError> {
        elements
            .iter()
            .map(|element| eval_list_expr(plan, state, frame, element).map(ListValueId::into_core))
            .collect()
    }

    fn allocate(state: &mut RuntimeState, item: &Self, values: Self::Values) -> Self::Handle {
        state.list(item.type_id(), values)
    }

    fn append(state: &RuntimeState, values: &mut Self::Values, tail: &Self::Handle) {
        values.extend(state.list_values(tail).iter().cloned());
    }

    fn drop_first(state: &RuntimeState, values: &Self::Handle, count: usize) -> Self::Values {
        let values = state.list_values(values);
        values[count.min(values.len())..].to_vec()
    }

    fn from_tuple_value(value: ListValueId) -> Option<Self::Handle> {
        match value {
            ListValueId::List(value) => Some(value),
            _ => None,
        }
    }

    fn from_core(item: &Self, core: ListHandleCore) -> Self::Handle {
        ListListValueId::new(item.type_id(), core)
    }

    fn get_local(frame: &Frame, local: Self::Local) -> Self::Handle {
        frame.get_list_list(local)
    }

    fn run_call(
        plan: &ExecutionPlan,
        state: &mut RuntimeState,
        function: Self::Function,
        args: &[crate::plan::execution::CallArg],
        frame: &mut Frame,
    ) -> Result<Self::Handle, ExecutionError> {
        function::run_list_list_call(plan, state, function, args, frame)
    }

    fn run_function_call(
        plan: &ExecutionPlan,
        state: &mut RuntimeState,
        function: &crate::plan::execution::ListFunctionExpr,
        args: &[crate::plan::execution::CallArg],
        frame: &mut Frame,
    ) -> Result<Self::Handle, ExecutionError> {
        function::run_list_list_function_call(plan, state, function, args, frame)
    }
}

impl RuntimeListItem for FunctionListItem {
    type Values = Vec<EvaluatedFunctionValue>;
    type Handle = FunctionListValueId;

    fn eval_elements(
        plan: &ExecutionPlan,
        state: &mut RuntimeState,
        frame: &mut Frame,
        elements: &[Self::ElementExpr],
    ) -> Result<Self::Values, ExecutionError> {
        elements
            .iter()
            .map(|element| eval_function_expr(plan, state, frame, element))
            .collect()
    }

    fn allocate(state: &mut RuntimeState, item: &Self, values: Self::Values) -> Self::Handle {
        state.function(item.type_id(), values)
    }

    fn append(state: &RuntimeState, values: &mut Self::Values, tail: &Self::Handle) {
        values.extend(state.function_values(tail).iter().cloned());
    }

    fn drop_first(state: &RuntimeState, values: &Self::Handle, count: usize) -> Self::Values {
        let values = state.function_values(values);
        values[count.min(values.len())..].to_vec()
    }

    fn from_tuple_value(value: ListValueId) -> Option<Self::Handle> {
        match value {
            ListValueId::Function(value) => Some(value),
            _ => None,
        }
    }

    fn from_core(item: &Self, core: ListHandleCore) -> Self::Handle {
        FunctionListValueId::new(item.type_id(), core)
    }

    fn get_local(frame: &Frame, local: Self::Local) -> Self::Handle {
        frame.get_function_list(local)
    }

    fn run_call(
        plan: &ExecutionPlan,
        state: &mut RuntimeState,
        function: Self::Function,
        args: &[crate::plan::execution::CallArg],
        frame: &mut Frame,
    ) -> Result<Self::Handle, ExecutionError> {
        function::run_function_list_call(plan, state, function, args, frame)
    }

    fn run_function_call(
        plan: &ExecutionPlan,
        state: &mut RuntimeState,
        function: &crate::plan::execution::ListFunctionExpr,
        args: &[crate::plan::execution::CallArg],
        frame: &mut Frame,
    ) -> Result<Self::Handle, ExecutionError> {
        function::run_function_list_function_call(plan, state, function, args, frame)
    }
}

pub(in crate::runtime) fn get_list_value(
    frame: &Frame,
    local: &crate::plan::execution::ListLocal,
) -> ListValueId {
    match local {
        crate::plan::execution::ListLocal::Int { local, .. } => frame.get_int_list(*local).into(),
        crate::plan::execution::ListLocal::String { local, .. } => {
            frame.get_string_list(*local).into()
        }
        crate::plan::execution::ListLocal::BitArray { local, .. } => {
            frame.get_bit_array_list(*local).into()
        }
        crate::plan::execution::ListLocal::Float { local, .. } => {
            frame.get_float_list(*local).into()
        }
        crate::plan::execution::ListLocal::Bool { local, .. } => frame.get_bool_list(*local).into(),
        crate::plan::execution::ListLocal::Nil { local, .. } => frame.get_nil_list(*local).into(),
        crate::plan::execution::ListLocal::Tuple { local, .. } => {
            frame.get_tuple_list(*local).into()
        }
        crate::plan::execution::ListLocal::List { local, .. } => frame.get_list_list(*local).into(),
        crate::plan::execution::ListLocal::Function { local, .. } => {
            frame.get_function_list(*local).into()
        }
    }
}

macro_rules! project_primitive_list {
    ($name:ident, $expr:ty, $value:ty, $eval:ident, $values:ident, $expected:expr, $get:ident) => {
        pub(in crate::runtime) fn $name(
            plan: &ExecutionPlan,
            state: &mut RuntimeState,
            frame: &mut Frame,
            list: &$expr,
            index: usize,
        ) -> Result<$value, ExecutionError> {
            let list = $eval(plan, state, frame, list)?;
            let values = state.$values(&list);
            values
                .get(index)
                .$get()
                .ok_or_else(|| ExecutionError::ListIndexOutOfBounds {
                    item_type: $expected,
                    index,
                    length: values.len(),
                })
        }
    };
}

project_primitive_list!(
    project_int_list_expr,
    IntListExpr,
    BigInt,
    eval_int_list_expr,
    int_values,
    ValueType::Int,
    cloned
);
project_primitive_list!(
    project_string_list_expr,
    StringListExpr,
    EcoString,
    eval_string_list_expr,
    string_values,
    ValueType::String,
    cloned
);
project_primitive_list!(
    project_bit_array_list_expr,
    BitArrayListExpr,
    EvaluatedBitArray,
    eval_bit_array_list_expr,
    bit_array_values,
    ValueType::BitArray,
    cloned
);
project_primitive_list!(
    project_float_list_expr,
    FloatListExpr,
    f64,
    eval_float_list_expr,
    float_values,
    ValueType::Float,
    copied
);
project_primitive_list!(
    project_bool_list_expr,
    BoolListExpr,
    bool,
    eval_bool_list_expr,
    bool_values,
    ValueType::Bool,
    copied
);

pub(in crate::runtime) fn project_nil_list_expr(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
    frame: &mut Frame,
    list: &NilListExpr,
    index: usize,
) -> Result<(), ExecutionError> {
    let list = eval_nil_list_expr(plan, state, frame, list)?;
    let len = state.nil_len(&list);
    if index < len {
        Ok(())
    } else {
        Err(ExecutionError::ListIndexOutOfBounds {
            item_type: ValueType::Nil,
            index,
            length: len,
        })
    }
}

pub(in crate::runtime) fn project_tuple_list_expr(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
    frame: &mut Frame,
    list: &TupleListExpr,
    index: usize,
    item_type: &[ValueType],
) -> Result<Vec<EvaluatedValue>, ExecutionError> {
    let list = eval_tuple_list_expr(plan, state, frame, list)?;
    let values = state.tuple_values(&list);
    values
        .get(index)
        .cloned()
        .ok_or_else(|| ExecutionError::ListIndexOutOfBounds {
            item_type: ValueType::Tuple(item_type.to_vec()),
            index,
            length: values.len(),
        })
}

pub(in crate::runtime) fn project_function_list_expr(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
    frame: &mut Frame,
    list: &FunctionListExpr,
    index: usize,
    item_type: &FunctionType,
) -> Result<EvaluatedFunctionValue, ExecutionError> {
    let list = eval_function_list_expr(plan, state, frame, list)?;
    let values = state.function_values(&list);
    values
        .get(index)
        .cloned()
        .ok_or_else(|| ExecutionError::ListIndexOutOfBounds {
            item_type: ValueType::Function(Box::new(item_type.clone())),
            index,
            length: values.len(),
        })
}

#[cfg(test)]
mod tests {
    use super::{
        project_bool_list_expr, project_float_list_expr, project_function_list_expr,
        project_int_list_expr, project_nil_list_expr, project_string_list_expr,
        project_tuple_list_expr,
    };
    use crate::plan::execution::{
        BoolListItem, FloatListItem, FunctionListItem, IntListItem, ListListItem, NilListItem,
        StringListItem, TupleListItem,
    };
    use crate::plan::execution::{ReturnBody, ReturnBodyKind};
    use crate::plan::{
        BoolExpr, BoolListCaseBranches, Expr, FloatExpr, FunctionId, FunctionPlan, FunctionType,
        IntExpr, IntListExpr, IntListFunctionId, ListCaseBranches, ListExpr as ModuleListExpr,
        ModulePlan, NilExpr, NilListExpr, NilListFunctionId, PanicExpr, PanicSite, ReturnExpr,
        Step, StringExpr, TupleExpr, ValueType,
    };
    use crate::runtime::EvaluatedValue;
    use crate::runtime::frame::Frame;
    use crate::runtime::{ExecutionError, RuntimeState};

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
        let mut state = RuntimeState::new();

        let function = plan.int_list_function(plan.int_list_function_id(0));
        let expression = expect_expression_return(function.return_());
        let mut frame = Frame::new(function.frame_layout(), &mut state);
        assert_eq!(
            project_int_list_expr(&plan, &mut state, &mut frame, expression, 0),
            Err(ExecutionError::ListIndexOutOfBounds {
                item_type: ValueType::Int,
                index: 0,
                length: 0,
            }),
        );

        let function = plan.string_list_function(plan.string_list_function_id(0));
        let expression = expect_expression_return(function.return_());
        let mut frame = Frame::new(function.frame_layout(), &mut state);
        assert_eq!(
            project_string_list_expr(&plan, &mut state, &mut frame, expression, 0),
            Err(ExecutionError::ListIndexOutOfBounds {
                item_type: ValueType::String,
                index: 0,
                length: 0,
            }),
        );

        let function = plan.float_list_function(plan.float_list_function_id(0));
        let expression = expect_expression_return(function.return_());
        let mut frame = Frame::new(function.frame_layout(), &mut state);
        assert_eq!(
            project_float_list_expr(&plan, &mut state, &mut frame, expression, 0),
            Err(ExecutionError::ListIndexOutOfBounds {
                item_type: ValueType::Float,
                index: 0,
                length: 0,
            }),
        );

        let function = plan.bool_list_function(plan.bool_list_function_id(0));
        let expression = expect_expression_return(function.return_());
        let mut frame = Frame::new(function.frame_layout(), &mut state);
        assert_eq!(
            project_bool_list_expr(&plan, &mut state, &mut frame, expression, 0),
            Err(ExecutionError::ListIndexOutOfBounds {
                item_type: ValueType::Bool,
                index: 0,
                length: 0,
            }),
        );

        let function = plan.nil_list_function(plan.nil_list_function_id(0));
        let expression = expect_expression_return(function.return_());
        let mut frame = Frame::new(function.frame_layout(), &mut state);
        assert_eq!(
            project_nil_list_expr(&plan, &mut state, &mut frame, expression, 0),
            Err(ExecutionError::ListIndexOutOfBounds {
                item_type: ValueType::Nil,
                index: 0,
                length: 0,
            }),
        );

        let function = plan.tuple_list_function(plan.tuple_list_function_id(0));
        let expression = expect_expression_return(function.return_());
        let mut frame = Frame::new(function.frame_layout(), &mut state);
        assert_eq!(
            project_tuple_list_expr(
                &plan,
                &mut state,
                &mut frame,
                expression,
                0,
                &[ValueType::Int],
            ),
            Err(ExecutionError::ListIndexOutOfBounds {
                item_type: ValueType::Tuple(vec![ValueType::Int]),
                index: 0,
                length: 0,
            }),
        );

        let function = plan.function_list_function(plan.function_list_function_id(0));
        let expression = expect_expression_return(function.return_());
        let mut frame = Frame::new(function.frame_layout(), &mut state);
        let function_type = FunctionType::new(Vec::new(), ValueType::Int);
        assert_eq!(
            project_function_list_expr(
                &plan,
                &mut state,
                &mut frame,
                expression,
                0,
                &function_type,
            ),
            Err(ExecutionError::ListIndexOutOfBounds {
                item_type: ValueType::Function(Box::new(function_type)),
                index: 0,
                length: 0,
            }),
        );
    }

    #[test]
    fn nested_list_projection_reports_only_the_missing_index_for_every_item_family() {
        let plan = crate::runtime::plan_src(
            r#"
fn first_int(values: List(List(Int))) -> List(Int) { case values { [first, ..] -> first _ -> [] } }
fn first_string(values: List(List(String))) -> List(String) { case values { [first, ..] -> first _ -> [] } }
fn first_bit_array(values: List(List(BitArray))) -> List(BitArray) { case values { [first, ..] -> first _ -> [] } }
fn first_float(values: List(List(Float))) -> List(Float) { case values { [first, ..] -> first _ -> [] } }
fn first_bool(values: List(List(Bool))) -> List(Bool) { case values { [first, ..] -> first _ -> [] } }
fn first_nil(values: List(List(Nil))) -> List(Nil) { case values { [first, ..] -> first _ -> [] } }
fn first_tuple(values: List(List(#(Int)))) -> List(#(Int)) { case values { [first, ..] -> first _ -> [] } }
fn first_list(values: List(List(List(Int)))) -> List(List(Int)) { case values { [first, ..] -> first _ -> [] } }
fn first_function(values: List(List(fn() -> Int))) -> List(fn() -> Int) { case values { [first, ..] -> first _ -> [] } }
pub fn main() { Nil }
"#,
        );
        let mut state = RuntimeState::new();

        let function = plan.int_list_function(plan.int_list_function_id(0));
        let mut frame = Frame::new(function.frame_layout(), &mut state);
        assert_nested_list_out_of_bounds(
            &plan,
            &mut state,
            &mut frame,
            expect_nested_list_binding(function.return_()),
        );

        let function = plan.bit_array_list_function(plan.bit_array_list_function_id(0));
        let mut frame = Frame::new(function.frame_layout(), &mut state);
        assert_nested_list_out_of_bounds(
            &plan,
            &mut state,
            &mut frame,
            expect_nested_list_binding(function.return_()),
        );

        let function = plan.string_list_function(plan.string_list_function_id(0));
        let mut frame = Frame::new(function.frame_layout(), &mut state);
        assert_nested_list_out_of_bounds(
            &plan,
            &mut state,
            &mut frame,
            expect_nested_list_binding(function.return_()),
        );

        let function = plan.float_list_function(plan.float_list_function_id(0));
        let mut frame = Frame::new(function.frame_layout(), &mut state);
        assert_nested_list_out_of_bounds(
            &plan,
            &mut state,
            &mut frame,
            expect_nested_list_binding(function.return_()),
        );

        let function = plan.bool_list_function(plan.bool_list_function_id(0));
        let mut frame = Frame::new(function.frame_layout(), &mut state);
        assert_nested_list_out_of_bounds(
            &plan,
            &mut state,
            &mut frame,
            expect_nested_list_binding(function.return_()),
        );

        let function = plan.nil_list_function(plan.nil_list_function_id(0));
        let mut frame = Frame::new(function.frame_layout(), &mut state);
        assert_nested_list_out_of_bounds(
            &plan,
            &mut state,
            &mut frame,
            expect_nested_list_binding(function.return_()),
        );

        let function = plan.tuple_list_function(plan.tuple_list_function_id(0));
        let mut frame = Frame::new(function.frame_layout(), &mut state);
        assert_nested_list_out_of_bounds(
            &plan,
            &mut state,
            &mut frame,
            expect_nested_list_binding(function.return_()),
        );

        let function = plan.list_list_function(plan.list_list_function_id(0));
        let mut frame = Frame::new(function.frame_layout(), &mut state);
        assert_nested_list_out_of_bounds(
            &plan,
            &mut state,
            &mut frame,
            expect_nested_list_binding(function.return_()),
        );

        let function = plan.function_list_function(plan.function_list_function_id(0));
        let mut frame = Frame::new(function.frame_layout(), &mut state);
        assert_nested_list_out_of_bounds(
            &plan,
            &mut state,
            &mut frame,
            expect_nested_list_binding(function.return_()),
        );
    }

    #[test]
    fn tuple_list_projection_reports_the_tuple_family_mismatch() {
        let plan = crate::runtime::plan_src(
            r#"
fn int_from_tuple(value: #(List(Int))) { value.0 }
fn strings() -> List(String) { [] }
pub fn main() { Nil }
"#,
        );
        let function = plan.int_list_function(plan.int_list_function_id(0));
        let expression = expect_expression_return(function.return_());
        let mut state = RuntimeState::new();
        let wrong = state.string(plan.string_list_function_id(0).type_id(), Vec::new());
        let mut frame = Frame::new(function.frame_layout(), &mut state);
        frame.set_tuple(
            crate::plan::execution::TupleLocalId(0),
            vec![EvaluatedValue::List(wrong.into())],
        );

        assert_eq!(
            super::eval_int_list_expr(&plan, &mut state, &mut frame, expression),
            Err(ExecutionError::TupleIndexFamilyMismatch {
                expected: ValueType::List(Box::new(ValueType::Int)),
                actual: ValueType::List(Box::new(ValueType::String)),
            }),
        );
    }

    #[test]
    fn tuple_list_family_conversion_rejects_every_wrong_handle_family() {
        let plan = crate::runtime::plan_src(
            r#"
fn ints() -> List(Int) { [] }
fn strings() -> List(String) { [] }
fn floats() -> List(Float) { [] }
fn bools() -> List(Bool) { [] }
fn nils() -> List(Nil) { [] }
fn tuples() -> List(#(Int)) { [] }
fn lists() -> List(List(Int)) { [] }
fn functions() -> List(fn() -> Int) { [] }
pub fn main() { Nil }
"#,
        );
        let mut state = RuntimeState::new();
        let int = state.int(plan.int_list_function_id(0).type_id(), Vec::new());
        let string = state.string(plan.string_list_function_id(0).type_id(), Vec::new());

        assert_eq!(
            <IntListItem as super::RuntimeListItem>::from_tuple_value(string.clone().into()),
            None,
        );
        assert_eq!(
            <StringListItem as super::RuntimeListItem>::from_tuple_value(int.clone().into()),
            None,
        );
        assert_eq!(
            <FloatListItem as super::RuntimeListItem>::from_tuple_value(int.clone().into()),
            None,
        );
        assert_eq!(
            <BoolListItem as super::RuntimeListItem>::from_tuple_value(int.clone().into()),
            None,
        );
        assert_eq!(
            <NilListItem as super::RuntimeListItem>::from_tuple_value(int.clone().into()),
            None,
        );
        assert_eq!(
            <TupleListItem as super::RuntimeListItem>::from_tuple_value(int.clone().into()),
            None,
        );
        assert_eq!(
            <ListListItem as super::RuntimeListItem>::from_tuple_value(int.clone().into()),
            None,
        );
        assert_eq!(
            <FunctionListItem as super::RuntimeListItem>::from_tuple_value(int.into()),
            None,
        );
    }

    #[test]
    fn module_dependency_errors_propagate_through_every_list_wrapper() {
        let panic = || PanicExpr::panic_at(None, PanicSite::unknown());
        let empty = || {
            ModuleListExpr::value(Vec::new(), ValueType::Int)
                .into_int()
                .expect("empty Int list must retain its item family")
        };
        let panic_int_list = || {
            ModuleListExpr::panic(panic(), ValueType::Int)
                .into_int()
                .expect("panic Int list must retain its item family")
        };

        let nested_source =
            ModuleListExpr::panic(panic(), ValueType::List(Box::new(ValueType::Int)))
                .into_list()
                .expect("nested panic list must retain its item family");
        let expressions = vec![
            ModuleListExpr::value(vec![Expr::int(IntExpr::panic(panic()))], ValueType::Int)
                .into_int()
                .expect("Int list elements must retain their item family"),
            ModuleListExpr::spread(
                vec![Expr::int(IntExpr::panic(panic()))],
                ModuleListExpr::Int(empty()),
                ValueType::Int,
            )
            .into_int()
            .expect("Int spread prefix must retain its item family"),
            ModuleListExpr::spread(
                vec![Expr::int(IntExpr::value(1.into()))],
                ModuleListExpr::Int(panic_int_list()),
                ValueType::Int,
            )
            .into_int()
            .expect("Int spread tail must retain its item family"),
            ModuleListExpr::tuple_index(
                TupleExpr::panic(panic(), vec![ValueType::List(Box::new(ValueType::Int))]),
                0,
                ValueType::Int,
            )
            .into_int()
            .expect("tuple list projection must retain its item family"),
            ModuleListExpr::list_index(nested_source, 0)
                .into_int()
                .expect("nested list projection must retain its item family"),
            ModuleListExpr::drop_first(ModuleListExpr::Int(panic_int_list()), 1)
                .into_int()
                .expect("dropped Int list must retain its item family"),
            ModuleListExpr::bool_case(
                BoolExpr::panic(panic()),
                BoolListCaseBranches::Int {
                    true_: empty(),
                    false_: empty(),
                },
            )
            .into_int()
            .expect("Bool case branches must retain their Int item family"),
            ModuleListExpr::int_case(
                IntExpr::panic(panic()),
                ListCaseBranches::Int {
                    clauses: Vec::new(),
                    fallback: empty(),
                },
            )
            .into_int()
            .expect("Int case branches must retain their item family"),
            ModuleListExpr::string_case(
                StringExpr::panic(panic()),
                ListCaseBranches::Int {
                    clauses: Vec::new(),
                    fallback: empty(),
                },
            )
            .into_int()
            .expect("String case branches must retain their Int item family"),
            ModuleListExpr::float_case(
                FloatExpr::panic(panic()),
                ListCaseBranches::Int {
                    clauses: Vec::new(),
                    fallback: empty(),
                },
            )
            .into_int()
            .expect("Float case branches must retain their Int item family"),
            ModuleListExpr::block(
                vec![Step::evaluate(Expr::bool(BoolExpr::panic(panic())))],
                ModuleListExpr::Int(empty()),
            )
            .into_int()
            .expect("block return must retain its Int item family"),
        ];

        for expression in expressions {
            assert_eq!(
                run_module_int_list_expression(expression).to_string(),
                "panic: `panic` expression evaluated.",
            );
        }

        let nil_expression =
            ModuleListExpr::value(vec![Expr::nil(NilExpr::panic(panic()))], ValueType::Nil)
                .into_nil()
                .expect("Nil list elements must retain their item family");
        assert_eq!(
            run_module_nil_list_expression(nil_expression).to_string(),
            "panic: `panic` expression evaluated.",
        );
    }

    #[test]
    fn list_projection_propagates_source_errors_for_primitive_nil_and_tuple_lists() {
        let plan = crate::runtime::plan_src(
            r#"
fn ints() -> List(Int) { panic }
fn nils() -> List(Nil) { panic }
fn tuples() -> List(#(Int)) { panic }
pub fn main() { Nil }
"#,
        );
        let mut state = RuntimeState::new();

        let function = plan.int_list_function(plan.int_list_function_id(0));
        let expression = expect_expression_return(function.return_());
        let mut frame = Frame::new(function.frame_layout(), &mut state);
        assert_eq!(
            project_int_list_expr(&plan, &mut state, &mut frame, expression, 0)
                .expect_err("Int list source should fail")
                .to_string(),
            "panic: `panic` expression evaluated.",
        );

        let function = plan.nil_list_function(plan.nil_list_function_id(0));
        let expression = expect_expression_return(function.return_());
        let mut frame = Frame::new(function.frame_layout(), &mut state);
        assert_eq!(
            project_nil_list_expr(&plan, &mut state, &mut frame, expression, 0)
                .expect_err("Nil list source should fail")
                .to_string(),
            "panic: `panic` expression evaluated.",
        );

        let function = plan.tuple_list_function(plan.tuple_list_function_id(0));
        let expression = expect_expression_return(function.return_());
        let mut frame = Frame::new(function.frame_layout(), &mut state);
        assert_eq!(
            project_tuple_list_expr(
                &plan,
                &mut state,
                &mut frame,
                expression,
                0,
                &[ValueType::Int],
            )
            .expect_err("tuple list source should fail")
            .to_string(),
            "panic: `panic` expression evaluated.",
        );
    }

    #[test]
    #[should_panic(expected = "expected a list expression return body")]
    fn expression_return_fixture_guard_rejects_tail_calls() {
        let plan = crate::runtime::plan_src(
            "fn recurse() -> List(Int) { recurse() } pub fn main() { Nil }",
        );
        let function = plan.int_list_function(plan.int_list_function_id(0));
        let _ = expect_expression_return(function.return_());
    }

    #[test]
    #[should_panic(expected = "expected a block return body")]
    fn nested_projection_fixture_guard_rejects_expression_returns() {
        let plan = crate::runtime::plan_src("pub fn main() -> List(Int) { [] }");
        let function = plan.int_list_function(plan.int_list_function_id(0));
        let _ = expect_nested_list_binding(function.return_());
    }

    #[test]
    #[should_panic(expected = "expected a Bool case return body")]
    fn nested_projection_fixture_guard_rejects_plain_blocks() {
        let plan = crate::runtime::plan_src("pub fn main() -> List(Int) { { let value = 1 [] } }");
        let function = plan.int_list_function(plan.int_list_function_id(0));
        let _ = expect_nested_list_binding(function.return_());
    }

    #[test]
    #[should_panic(expected = "expected a binding block return body")]
    fn nested_projection_fixture_guard_rejects_unbound_case_branches() {
        let plan = crate::runtime::plan_src(
            "fn empty(values: List(Int)) -> List(Int) { case values { [] -> [] _ -> [] } } pub fn main() { Nil }",
        );
        let function = plan.int_list_function(plan.int_list_function_id(0));
        let _ = expect_nested_list_binding(function.return_());
    }

    #[test]
    #[should_panic(expected = "expected a list binding step")]
    fn nested_projection_fixture_guard_rejects_int_bindings() {
        let plan = crate::runtime::plan_src(
            "fn first(values: List(Int)) -> List(Int) { case values { [first, ..] -> [] _ -> [] } } pub fn main() { Nil }",
        );
        let function = plan.int_list_function(plan.int_list_function_id(0));
        let _ = expect_nested_list_binding(function.return_());
    }

    fn expect_expression_return<Expression, Function>(
        body: &ReturnBody<Expression, Function>,
    ) -> &Expression {
        match body.kind() {
            ReturnBodyKind::Expr(expression) => expression,
            _ => panic!("expected a list expression return body"),
        }
    }

    fn expect_nested_list_binding<Expression, Function>(
        body: &ReturnBody<Expression, Function>,
    ) -> &crate::plan::execution::ListLocalExpr {
        let ReturnBodyKind::Block { return_, .. } = body.kind() else {
            panic!("expected a block return body");
        };
        let ReturnBodyKind::BoolCase { true_, .. } = return_.kind() else {
            panic!("expected a Bool case return body");
        };
        let ReturnBodyKind::Block { steps, .. } = true_.kind() else {
            panic!("expected a binding block return body");
        };
        let crate::plan::execution::StepKind::LetList { value } = steps[0].kind() else {
            panic!("expected a list binding step");
        };
        value
    }

    fn assert_nested_list_out_of_bounds(
        plan: &crate::ExecutionPlan,
        state: &mut RuntimeState,
        frame: &mut Frame,
        expression: &crate::plan::execution::ListLocalExpr,
    ) {
        match expression {
            crate::plan::execution::ListLocalExpr::Int { value, .. } => assert_eq!(
                super::eval_int_list_expr(plan, state, frame, value),
                Err(ExecutionError::ListIndexOutOfBounds {
                    item_type: ValueType::List(Box::new(ValueType::Int)),
                    index: 0,
                    length: 0,
                }),
            ),
            crate::plan::execution::ListLocalExpr::String { value, .. } => assert_eq!(
                super::eval_string_list_expr(plan, state, frame, value),
                Err(ExecutionError::ListIndexOutOfBounds {
                    item_type: ValueType::List(Box::new(ValueType::String)),
                    index: 0,
                    length: 0,
                }),
            ),
            crate::plan::execution::ListLocalExpr::BitArray { value, .. } => assert_eq!(
                super::eval_bit_array_list_expr(plan, state, frame, value),
                Err(ExecutionError::ListIndexOutOfBounds {
                    item_type: ValueType::List(Box::new(ValueType::BitArray)),
                    index: 0,
                    length: 0,
                }),
            ),
            crate::plan::execution::ListLocalExpr::Float { value, .. } => assert_eq!(
                super::eval_float_list_expr(plan, state, frame, value),
                Err(ExecutionError::ListIndexOutOfBounds {
                    item_type: ValueType::List(Box::new(ValueType::Float)),
                    index: 0,
                    length: 0,
                }),
            ),
            crate::plan::execution::ListLocalExpr::Bool { value, .. } => assert_eq!(
                super::eval_bool_list_expr(plan, state, frame, value),
                Err(ExecutionError::ListIndexOutOfBounds {
                    item_type: ValueType::List(Box::new(ValueType::Bool)),
                    index: 0,
                    length: 0,
                }),
            ),
            crate::plan::execution::ListLocalExpr::Nil { value, .. } => assert_eq!(
                super::eval_nil_list_expr(plan, state, frame, value),
                Err(ExecutionError::ListIndexOutOfBounds {
                    item_type: ValueType::List(Box::new(ValueType::Nil)),
                    index: 0,
                    length: 0,
                }),
            ),
            crate::plan::execution::ListLocalExpr::Tuple { value, .. } => assert_eq!(
                super::eval_tuple_list_expr(plan, state, frame, value),
                Err(ExecutionError::ListIndexOutOfBounds {
                    item_type: ValueType::List(Box::new(ValueType::Tuple(vec![ValueType::Int]))),
                    index: 0,
                    length: 0,
                }),
            ),
            crate::plan::execution::ListLocalExpr::List { value, .. } => assert_eq!(
                super::eval_list_list_expr(plan, state, frame, value),
                Err(ExecutionError::ListIndexOutOfBounds {
                    item_type: ValueType::List(Box::new(ValueType::List(Box::new(ValueType::Int)))),
                    index: 0,
                    length: 0,
                }),
            ),
            crate::plan::execution::ListLocalExpr::Function { value, .. } => assert_eq!(
                super::eval_function_list_expr(plan, state, frame, value),
                Err(ExecutionError::ListIndexOutOfBounds {
                    item_type: ValueType::List(Box::new(ValueType::Function(Box::new(
                        FunctionType::new(Vec::new(), ValueType::Int),
                    )))),
                    index: 0,
                    length: 0,
                }),
            ),
        }
    }

    fn run_module_int_list_expression(expression: IntListExpr) -> ExecutionError {
        let main = FunctionPlan::new(
            FunctionId::new(0),
            "main".into(),
            Vec::new(),
            Vec::new(),
            ReturnExpr::int_list_body(
                IntListFunctionId(0),
                crate::plan::IntListReturn::expr(expression),
            ),
        );
        let module = ModulePlan::new("main".into(), main, Vec::new());
        let plan = crate::ExecutionPlan::from_module_plan(module);

        crate::runtime::run_main(&plan)
            .expect_err("module Int list expression should fail at runtime")
    }

    fn run_module_nil_list_expression(expression: NilListExpr) -> ExecutionError {
        let main = FunctionPlan::new(
            FunctionId::new(0),
            "main".into(),
            Vec::new(),
            Vec::new(),
            ReturnExpr::nil_list_body(
                NilListFunctionId(0),
                crate::plan::NilListReturn::expr(expression),
            ),
        );
        let module = ModulePlan::new("main".into(), main, Vec::new());
        let plan = crate::ExecutionPlan::from_module_plan(module);

        crate::runtime::run_main(&plan)
            .expect_err("module Nil list expression should fail at runtime")
    }
}
