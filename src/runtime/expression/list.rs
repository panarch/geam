use ecow::EcoString;
use num_bigint::BigInt;

use super::{
    eval_bit_array_expr, eval_bool_expr, eval_custom_expr, eval_custom_field, eval_float_expr,
    eval_function_expr, eval_int_expr, eval_never_expr, eval_nil_expr, eval_panic_expr,
    eval_string_expr, eval_tuple_expr, eval_utf_codepoint_expr, project_tuple_expr,
};
use crate::plan::execution::{
    BitArrayListExpr, BitArrayListItem, BoolListExpr, BoolListItem, CustomListExpr, CustomListItem,
    CustomTypeId, ExecutionPlan, FloatListExpr, FloatListItem, FunctionListExpr, FunctionListItem,
    IntListExpr, IntListItem, ListExpr, ListItem, ListListExpr, ListListItem, NilListExpr,
    NilListItem, ParameterListExpr, ParameterListExprKind, ParameterListItem,
    ParameterListListExpr, ParameterListListItem, StoredListExpr, StringListExpr, StringListItem,
    TupleListExpr, TupleListItem, TypedListExpr, TypedListExprKind, UtfCodepointListExpr,
    UtfCodepointListItem,
};
use crate::plan::{FunctionType, ValueType};
use crate::runtime::evaluated::{
    EvaluatedBitArray, EvaluatedCustomValue, EvaluatedFunctionValue, EvaluatedValue,
};
use crate::runtime::frame::Frame;
use crate::runtime::function;
use crate::runtime::state::{
    BitArrayListValueId, BoolListValueId, CustomListAllocation, CustomListValueId,
    FloatListValueId, FunctionListValueId, IntListValueId, ListHandleCore, ListListValueId,
    ListValueId, NilListValueId, ParameterListListValueId, ParameterListValueId, RuntimeState,
    StoredListValueId, StringListValueId, TupleListValueId, UtfCodepointListValueId,
};
use crate::runtime::{ExecutionError, InvariantError};

pub(in crate::runtime) fn eval_list_expr(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
    frame: &mut Frame,
    expression: &ListExpr,
) -> Result<ListValueId, ExecutionError> {
    match expression {
        ListExpr::Parameter(expression) => {
            eval_parameter_list_expr(plan, state, frame, expression).map(Into::into)
        }
        ListExpr::ParameterList(expression) => {
            eval_parameter_list_list_expr(plan, state, frame, expression).map(Into::into)
        }
        ListExpr::Int(expression) => {
            eval_int_list_expr(plan, state, frame, expression).map(Into::into)
        }
        ListExpr::String(expression) => {
            eval_string_list_expr(plan, state, frame, expression).map(Into::into)
        }
        ListExpr::BitArray(expression) => {
            eval_bit_array_list_expr(plan, state, frame, expression).map(Into::into)
        }
        ListExpr::UtfCodepoint(expression) => {
            eval_utf_codepoint_list_expr(plan, state, frame, expression).map(Into::into)
        }
        ListExpr::Custom(expression) => {
            eval_custom_list_expr(plan, state, frame, expression).map(Into::into)
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

fn eval_stored_list_expr(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
    frame: &mut Frame,
    expression: &StoredListExpr,
) -> Result<StoredListValueId, ExecutionError> {
    match expression {
        StoredListExpr::ParameterList(expression) => {
            eval_parameter_list_list_expr(plan, state, frame, expression).map(Into::into)
        }
        StoredListExpr::Int(expression) => {
            eval_int_list_expr(plan, state, frame, expression).map(Into::into)
        }
        StoredListExpr::String(expression) => {
            eval_string_list_expr(plan, state, frame, expression).map(Into::into)
        }
        StoredListExpr::BitArray(expression) => {
            eval_bit_array_list_expr(plan, state, frame, expression).map(Into::into)
        }
        StoredListExpr::UtfCodepoint(expression) => {
            eval_utf_codepoint_list_expr(plan, state, frame, expression).map(Into::into)
        }
        StoredListExpr::Custom(expression) => {
            eval_custom_list_expr(plan, state, frame, expression).map(Into::into)
        }
        StoredListExpr::Float(expression) => {
            eval_float_list_expr(plan, state, frame, expression).map(Into::into)
        }
        StoredListExpr::Bool(expression) => {
            eval_bool_list_expr(plan, state, frame, expression).map(Into::into)
        }
        StoredListExpr::Nil(expression) => {
            eval_nil_list_expr(plan, state, frame, expression).map(Into::into)
        }
        StoredListExpr::Tuple(expression) => {
            eval_tuple_list_expr(plan, state, frame, expression).map(Into::into)
        }
        StoredListExpr::List(expression) => {
            eval_list_list_expr(plan, state, frame, expression).map(Into::into)
        }
        StoredListExpr::Function(expression) => {
            eval_function_list_expr(plan, state, frame, expression).map(Into::into)
        }
    }
}

pub(in crate::runtime) fn eval_parameter_list_expr(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
    frame: &mut Frame,
    expression: &ParameterListExpr,
) -> Result<ParameterListValueId, ExecutionError> {
    eval_parameter_list_expr_kind(plan, state, frame, expression.item(), expression.kind())
}

fn eval_parameter_list_expr_kind(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
    frame: &mut Frame,
    item: &ParameterListItem,
    kind: &ParameterListExprKind,
) -> Result<ParameterListValueId, ExecutionError> {
    match kind {
        ParameterListExprKind::Value => Ok(ParameterListValueId::new(item.type_id())),
        ParameterListExprKind::Never(expression) => {
            eval_never_expr(plan, state, frame, expression).map(|value| match value {})
        }
        ParameterListExprKind::Constant(constant) => {
            eval_parameter_list_expr(plan, state, frame, plan.constant(*constant))
        }
        ParameterListExprKind::LocalGet { local } => Ok(frame.get_parameter_list(*local)),
        ParameterListExprKind::Call(call) => super::eval_direct_call(
            plan,
            state,
            frame,
            call,
            |plan, state, function, args, frame| {
                function::run_parameter_list_call(plan, state, *function, args, frame)
            },
        ),
        ParameterListExprKind::FunctionCall(call) => super::eval_function_call(
            plan,
            state,
            frame,
            call,
            function::run_parameter_list_function_call,
        ),
        ParameterListExprKind::TupleIndex { tuple, index } => {
            let expected = plan.list_value_type(item.type_id().list_type());
            match project_tuple_expr(plan, state, frame, tuple, *index, expected.clone())? {
                EvaluatedValue::List(ListValueId::Parameter(value)) => Ok(value),
                EvaluatedValue::List(value) => Err(ExecutionError::Invariant(
                    InvariantError::TupleIndexFamilyMismatch {
                        expected,
                        actual: plan.list_value_type(value.list_type()),
                    },
                )),
                other => Err(ExecutionError::Invariant(
                    InvariantError::TupleIndexFamilyMismatch {
                        expected,
                        actual: other.value_type(plan),
                    },
                )),
            }
        }
        ParameterListExprKind::CustomField(access) => {
            let expected = plan.list_value_type(item.type_id().list_type());
            let (constructor, value) = eval_custom_field(plan, state, frame, access)?;
            match value {
                EvaluatedValue::List(ListValueId::Parameter(value)) => Ok(value),
                value => {
                    let descriptor = plan.custom_constructor(constructor);
                    Err(ExecutionError::Invariant(
                        InvariantError::CustomFieldFamilyMismatch {
                            custom_type: plan.custom_value_type(constructor.type_id()),
                            constructor: descriptor.name().clone(),
                            field_index: access.index(),
                            expected,
                            actual: value.value_type(plan),
                        },
                    ))
                }
            }
        }
        ParameterListExprKind::ListIndex(source) => {
            let list = eval_parameter_list_list_expr(plan, state, frame, source.list())?;
            let length = state.parameter_list_list_len(&list);
            if source.index() < length {
                Ok(ParameterListValueId::new(item.type_id()))
            } else {
                Err(ExecutionError::Invariant(
                    InvariantError::ListIndexOutOfBounds {
                        item_type: plan.list_value_type(item.type_id().list_type()),
                        index: source.index(),
                        length,
                    },
                ))
            }
        }
        ParameterListExprKind::Panic(panic) => {
            eval_panic_expr(plan, state, frame, panic).map(|never| match never {})
        }
        ParameterListExprKind::BoolCase {
            subject,
            true_,
            false_,
        } => {
            if eval_bool_expr(plan, state, frame, subject)? {
                eval_parameter_list_expr_kind(plan, state, frame, item, true_)
            } else {
                eval_parameter_list_expr_kind(plan, state, frame, item, false_)
            }
        }
        ParameterListExprKind::IntCase {
            subject,
            clauses,
            fallback,
        } => {
            let subject = eval_int_expr(plan, state, frame, subject)?;
            for (pattern, branch) in clauses {
                if pattern == &subject {
                    return eval_parameter_list_expr_kind(plan, state, frame, item, branch);
                }
            }
            eval_parameter_list_expr_kind(plan, state, frame, item, fallback)
        }
        ParameterListExprKind::StringCase {
            subject,
            clauses,
            fallback,
        } => {
            let subject = eval_string_expr(plan, state, frame, subject)?;
            for (pattern, branch) in clauses {
                if pattern == &subject {
                    return eval_parameter_list_expr_kind(plan, state, frame, item, branch);
                }
            }
            eval_parameter_list_expr_kind(plan, state, frame, item, fallback)
        }
        ParameterListExprKind::FloatCase {
            subject,
            clauses,
            fallback,
        } => {
            let subject = eval_float_expr(plan, state, frame, subject)?;
            for (pattern, branch) in clauses {
                if pattern == &subject {
                    return eval_parameter_list_expr_kind(plan, state, frame, item, branch);
                }
            }
            eval_parameter_list_expr_kind(plan, state, frame, item, fallback)
        }
        ParameterListExprKind::Block { steps, return_ } => {
            function::execute_steps(plan, state, steps, frame)?;
            eval_parameter_list_expr_kind(plan, state, frame, item, return_)
        }
    }
}

pub(in crate::runtime) fn eval_parameter_list_list_expr(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
    frame: &mut Frame,
    expression: &ParameterListListExpr,
) -> Result<ParameterListListValueId, ExecutionError> {
    eval_typed_list_expr(plan, state, frame, expression)
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

pub(in crate::runtime) fn eval_utf_codepoint_list_expr(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
    frame: &mut Frame,
    expression: &UtfCodepointListExpr,
) -> Result<UtfCodepointListValueId, ExecutionError> {
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

pub(in crate::runtime) fn eval_custom_list_expr(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
    frame: &mut Frame,
    expression: &CustomListExpr,
) -> Result<CustomListValueId, ExecutionError> {
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
        TypedListExprKind::Constant(constant) => Item::eval_constant(plan, state, frame, *constant),
        TypedListExprKind::Spread { elements, tail } => {
            let mut values = Item::eval_elements(plan, state, frame, elements)?;
            let tail = eval_typed_list_expr_kind(plan, state, frame, item, tail)?;
            Item::append(state, &mut values, &tail);
            Ok(Item::allocate(state, item, values))
        }
        TypedListExprKind::LocalGet { local } => Ok(Item::get_local(frame, local.clone())),
        TypedListExprKind::Call(call) => super::eval_direct_call(
            plan,
            state,
            frame,
            call,
            |plan, state, function, args, frame| {
                Item::run_call(plan, state, function.clone(), args, frame)
            },
        ),
        TypedListExprKind::FunctionCall(call) => {
            super::eval_function_call(plan, state, frame, call, Item::run_function_call)
        }
        TypedListExprKind::TupleIndex { tuple, index } => {
            let expected = plan.list_value_type(item.list_type());
            match project_tuple_expr(plan, state, frame, tuple, *index, expected.clone())? {
                EvaluatedValue::List(value) => {
                    let actual = plan.list_value_type(value.list_type());
                    Item::from_tuple_value(value).ok_or_else(|| {
                        ExecutionError::Invariant(InvariantError::TupleIndexFamilyMismatch {
                            expected,
                            actual,
                        })
                    })
                }
                other => Err(ExecutionError::Invariant(
                    InvariantError::TupleIndexFamilyMismatch {
                        expected,
                        actual: other.value_type(plan),
                    },
                )),
            }
        }
        TypedListExprKind::CustomField(access) => {
            let expected = plan.list_value_type(item.list_type());
            let (constructor, value) = eval_custom_field(plan, state, frame, access)?;
            match value {
                EvaluatedValue::List(value) => {
                    let actual = plan.list_value_type(value.list_type());
                    Item::from_tuple_value(value).ok_or_else(|| {
                        let descriptor = plan.custom_constructor(constructor);
                        ExecutionError::Invariant(InvariantError::CustomFieldFamilyMismatch {
                            custom_type: plan.custom_value_type(constructor.type_id()),
                            constructor: descriptor.name().clone(),
                            field_index: access.index(),
                            expected,
                            actual,
                        })
                    })
                }
                other => {
                    let descriptor = plan.custom_constructor(constructor);
                    Err(ExecutionError::Invariant(
                        InvariantError::CustomFieldFamilyMismatch {
                            custom_type: plan.custom_value_type(constructor.type_id()),
                            constructor: descriptor.name().clone(),
                            field_index: access.index(),
                            expected,
                            actual: other.value_type(plan),
                        },
                    ))
                }
            }
        }
        TypedListExprKind::ListIndex(source) => {
            let list = eval_list_list_expr(plan, state, frame, source.list())?;
            let values = state.list_values(&list);
            values
                .get(source.index())
                .cloned()
                .map(|value| Item::from_core(item, value.into_core()))
                .ok_or_else(|| {
                    ExecutionError::Invariant(InvariantError::ListIndexOutOfBounds {
                        item_type: plan.list_value_type(item.list_type()),
                        index: source.index(),
                        length: values.len(),
                    })
                })
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

trait RuntimeListItem: ListItem<IndexSource = ListListExpr> + Sized {
    type Values;
    type Handle: Clone + Into<ListValueId>;

    fn eval_elements(
        plan: &ExecutionPlan,
        state: &mut RuntimeState,
        frame: &mut Frame,
        elements: &[Self::ElementExpr],
    ) -> Result<Self::Values, ExecutionError>;

    fn allocate(state: &mut RuntimeState, item: &Self, values: Self::Values) -> Self::Handle;

    fn eval_constant(
        plan: &ExecutionPlan,
        state: &mut RuntimeState,
        frame: &mut Frame,
        constant: Self::Constant,
    ) -> Result<Self::Handle, ExecutionError>;

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

impl RuntimeListItem for ParameterListListItem {
    type Values = usize;
    type Handle = ParameterListListValueId;

    fn eval_elements(
        plan: &ExecutionPlan,
        state: &mut RuntimeState,
        frame: &mut Frame,
        elements: &[Self::ElementExpr],
    ) -> Result<Self::Values, ExecutionError> {
        elements
            .iter()
            .try_for_each(|element| {
                eval_parameter_list_expr(plan, state, frame, element).map(|_| ())
            })
            .map(|()| elements.len())
    }

    fn allocate(state: &mut RuntimeState, item: &Self, values: Self::Values) -> Self::Handle {
        state.parameter_list_list(item.type_id(), values)
    }

    fn eval_constant(
        plan: &ExecutionPlan,
        state: &mut RuntimeState,
        frame: &mut Frame,
        constant: Self::Constant,
    ) -> Result<Self::Handle, ExecutionError> {
        eval_parameter_list_list_expr(plan, state, frame, plan.constant(constant))
    }

    fn append(state: &RuntimeState, values: &mut Self::Values, tail: &Self::Handle) {
        *values += state.parameter_list_list_len(tail);
    }

    fn drop_first(state: &RuntimeState, values: &Self::Handle, count: usize) -> Self::Values {
        state.parameter_list_list_len(values).saturating_sub(count)
    }

    fn from_tuple_value(value: ListValueId) -> Option<Self::Handle> {
        match value {
            ListValueId::ParameterList(value) => Some(value),
            _ => None,
        }
    }

    fn from_core(item: &Self, core: ListHandleCore) -> Self::Handle {
        ParameterListListValueId::new(item.type_id(), core)
    }

    fn get_local(frame: &Frame, local: Self::Local) -> Self::Handle {
        frame.get_parameter_list_list(local)
    }

    fn run_call(
        plan: &ExecutionPlan,
        state: &mut RuntimeState,
        function: Self::Function,
        args: &[crate::plan::execution::CallArg],
        frame: &mut Frame,
    ) -> Result<Self::Handle, ExecutionError> {
        function::run_parameter_list_list_call(plan, state, function, args, frame)
    }

    fn run_function_call(
        plan: &ExecutionPlan,
        state: &mut RuntimeState,
        function: &crate::plan::execution::ListFunctionExpr,
        args: &[crate::plan::execution::CallArg],
        frame: &mut Frame,
    ) -> Result<Self::Handle, ExecutionError> {
        function::run_parameter_list_list_function_call(plan, state, function, args, frame)
    }
}

macro_rules! primitive_runtime_list_item {
    (
        $item:ty,
        $value:ty,
        $handle:ty,
        $variant:ident,
        $element_eval:ident,
        $list_eval:ident,
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

            fn eval_constant(
                plan: &ExecutionPlan,
                state: &mut RuntimeState,
                frame: &mut Frame,
                constant: Self::Constant,
            ) -> Result<Self::Handle, ExecutionError> {
                $list_eval(plan, state, frame, plan.constant(constant))
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
    eval_int_list_expr,
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
    eval_string_list_expr,
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
    eval_bit_array_list_expr,
    bit_array_values,
    bit_array,
    get_bit_array_list,
    run_bit_array_list_call,
    run_bit_array_list_function_call
);
primitive_runtime_list_item!(
    UtfCodepointListItem,
    char,
    UtfCodepointListValueId,
    UtfCodepoint,
    eval_utf_codepoint_expr,
    eval_utf_codepoint_list_expr,
    utf_codepoint_values,
    utf_codepoint,
    get_utf_codepoint_list,
    run_utf_codepoint_list_call,
    run_utf_codepoint_list_function_call
);
impl RuntimeListItem for CustomListItem {
    type Values = Vec<EvaluatedCustomValue>;
    type Handle = CustomListValueId;

    fn eval_elements(
        plan: &ExecutionPlan,
        state: &mut RuntimeState,
        frame: &mut Frame,
        elements: &[Self::ElementExpr],
    ) -> Result<Self::Values, ExecutionError> {
        elements
            .iter()
            .map(|element| eval_custom_expr(plan, state, frame, element))
            .collect()
    }

    fn allocate(state: &mut RuntimeState, item: &Self, values: Self::Values) -> Self::Handle {
        state.custom(CustomListAllocation::from_item(item, values))
    }

    fn eval_constant(
        plan: &ExecutionPlan,
        state: &mut RuntimeState,
        frame: &mut Frame,
        constant: Self::Constant,
    ) -> Result<Self::Handle, ExecutionError> {
        eval_custom_list_expr(plan, state, frame, plan.constant(constant))
    }

    fn append(state: &RuntimeState, values: &mut Self::Values, tail: &Self::Handle) {
        values.extend(state.custom_values(tail).iter().cloned());
    }

    fn drop_first(state: &RuntimeState, values: &Self::Handle, count: usize) -> Self::Values {
        let values = state.custom_values(values);
        values[count.min(values.len())..].to_vec()
    }

    fn from_tuple_value(value: ListValueId) -> Option<Self::Handle> {
        match value {
            ListValueId::Custom(value) => Some(value),
            _ => None,
        }
    }

    fn from_core(item: &Self, core: ListHandleCore) -> Self::Handle {
        CustomListValueId::new(item.type_id(), core)
    }

    fn get_local(frame: &Frame, local: Self::Local) -> Self::Handle {
        frame.get_custom_list(local)
    }

    fn run_call(
        plan: &ExecutionPlan,
        state: &mut RuntimeState,
        function: Self::Function,
        args: &[crate::plan::execution::CallArg],
        frame: &mut Frame,
    ) -> Result<Self::Handle, ExecutionError> {
        function::run_custom_list_call(plan, state, function, args, frame)
    }

    fn run_function_call(
        plan: &ExecutionPlan,
        state: &mut RuntimeState,
        function: &crate::plan::execution::ListFunctionExpr,
        args: &[crate::plan::execution::CallArg],
        frame: &mut Frame,
    ) -> Result<Self::Handle, ExecutionError> {
        function::run_custom_list_function_call(plan, state, function, args, frame)
    }
}
primitive_runtime_list_item!(
    FloatListItem,
    f64,
    FloatListValueId,
    Float,
    eval_float_expr,
    eval_float_list_expr,
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
    eval_bool_list_expr,
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

    fn eval_constant(
        plan: &ExecutionPlan,
        state: &mut RuntimeState,
        frame: &mut Frame,
        constant: Self::Constant,
    ) -> Result<Self::Handle, ExecutionError> {
        eval_nil_list_expr(plan, state, frame, plan.constant(constant))
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

    fn eval_constant(
        plan: &ExecutionPlan,
        state: &mut RuntimeState,
        frame: &mut Frame,
        constant: Self::Constant,
    ) -> Result<Self::Handle, ExecutionError> {
        eval_tuple_list_expr(plan, state, frame, plan.constant(constant))
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
    type Values = Vec<StoredListValueId>;
    type Handle = ListListValueId;

    fn eval_elements(
        plan: &ExecutionPlan,
        state: &mut RuntimeState,
        frame: &mut Frame,
        elements: &[Self::ElementExpr],
    ) -> Result<Self::Values, ExecutionError> {
        elements
            .iter()
            .map(|element| eval_stored_list_expr(plan, state, frame, element))
            .collect()
    }

    fn allocate(state: &mut RuntimeState, item: &Self, values: Self::Values) -> Self::Handle {
        state.list(item.type_id(), values)
    }

    fn eval_constant(
        plan: &ExecutionPlan,
        state: &mut RuntimeState,
        frame: &mut Frame,
        constant: Self::Constant,
    ) -> Result<Self::Handle, ExecutionError> {
        eval_list_list_expr(plan, state, frame, plan.constant(constant))
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

    fn eval_constant(
        plan: &ExecutionPlan,
        state: &mut RuntimeState,
        frame: &mut Frame,
        constant: Self::Constant,
    ) -> Result<Self::Handle, ExecutionError> {
        eval_function_list_expr(plan, state, frame, plan.constant(constant))
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
        crate::plan::execution::ListLocal::Parameter { local, .. } => {
            frame.get_parameter_list(*local).into()
        }
        crate::plan::execution::ListLocal::ParameterList { local, .. } => {
            frame.get_parameter_list_list(*local).into()
        }
        crate::plan::execution::ListLocal::Int { local, .. } => frame.get_int_list(*local).into(),
        crate::plan::execution::ListLocal::String { local, .. } => {
            frame.get_string_list(*local).into()
        }
        crate::plan::execution::ListLocal::BitArray { local, .. } => {
            frame.get_bit_array_list(*local).into()
        }
        crate::plan::execution::ListLocal::UtfCodepoint { local, .. } => {
            frame.get_utf_codepoint_list(*local).into()
        }
        crate::plan::execution::ListLocal::Custom { local, .. } => {
            frame.get_custom_list(*local).into()
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
            values.get(index).$get().ok_or_else(|| {
                ExecutionError::Invariant(InvariantError::ListIndexOutOfBounds {
                    item_type: $expected,
                    index,
                    length: values.len(),
                })
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
    project_utf_codepoint_list_expr,
    UtfCodepointListExpr,
    char,
    eval_utf_codepoint_list_expr,
    utf_codepoint_values,
    ValueType::UtfCodepoint,
    copied
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

pub(in crate::runtime) fn project_custom_list_expr(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
    frame: &mut Frame,
    list: &CustomListExpr,
    index: usize,
    item_type: CustomTypeId,
) -> Result<EvaluatedCustomValue, ExecutionError> {
    let list = eval_custom_list_expr(plan, state, frame, list)?;
    let values = state.custom_values(&list);
    values.get(index).cloned().ok_or_else(|| {
        ExecutionError::Invariant(InvariantError::ListIndexOutOfBounds {
            item_type: ValueType::Custom(plan.custom_value_type(item_type)),
            index,
            length: values.len(),
        })
    })
}

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
        Err(ExecutionError::Invariant(
            InvariantError::ListIndexOutOfBounds {
                item_type: ValueType::Nil,
                index,
                length: len,
            },
        ))
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
    values.get(index).cloned().ok_or_else(|| {
        ExecutionError::Invariant(InvariantError::ListIndexOutOfBounds {
            item_type: ValueType::Tuple(item_type.to_vec()),
            index,
            length: values.len(),
        })
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
    values.get(index).cloned().ok_or_else(|| {
        ExecutionError::Invariant(InvariantError::ListIndexOutOfBounds {
            item_type: ValueType::Function(Box::new(item_type.clone())),
            index,
            length: values.len(),
        })
    })
}

#[cfg(test)]
mod tests {
    use super::{
        project_bit_array_list_expr, project_bool_list_expr, project_custom_list_expr,
        project_float_list_expr, project_function_list_expr, project_int_list_expr,
        project_nil_list_expr, project_string_list_expr, project_tuple_list_expr,
        project_utf_codepoint_list_expr,
    };
    use crate::plan::execution::{
        BitArrayListItem, BoolListItem, CallArg, CallArgKind, CustomConstruction, CustomExpr,
        CustomExprKind, CustomFieldAccess, CustomLocalExpr, FloatListItem, FunctionListItem,
        IntListItem, ListFunctionId, ListListItem, NilListItem, ParameterListExpr,
        ParameterListExprKind, ParameterListFunctionId, ParameterListListItem, RuntimeFunctionId,
        StringListItem, TupleListItem, UtfCodepointListItem,
    };
    use crate::plan::execution::{ReturnBlock, ReturnGraph};
    use crate::plan::module::{GenericListExpr, GenericListReturn};
    use crate::plan::{
        BoolExpr, BoolListCaseBranches, Expr, FloatExpr, FunctionShape, FunctionTemplate,
        FunctionTemplateId, FunctionTemplateSignature, FunctionType, IntExpr, IntListExpr,
        ListCaseBranches, ListExpr as ModuleListExpr, ModulePlan, NilExpr, NilListExpr, PanicExpr,
        PanicSite, ReturnExpr, Step, StringExpr, TupleExpr, TypeParameterId, TypeScheme,
        ValueShape, ValueStorageShape, ValueType,
    };
    use crate::runtime::frame::Frame;
    use crate::runtime::{EvaluatedCustomValue, EvaluatedValue};
    use crate::runtime::{ExecutionError, InvariantError, ListValue, RuntimeState, Value};

    #[test]
    fn source_bool_cases_select_false_branches_for_every_list_storage_family() {
        assert_eq!(
            crate::runtime::run_src(
                r#"
pub type Boxed { Boxed(Int) }

fn selected() { False }
fn keep(values: List(value)) { values }
fn int_identity(value: Int) { value }
fn codepoint() -> UtfCodepoint {
  let assert <<value:utf8_codepoint>> = <<65>>
  value
}

pub fn main() {
  let selector = selected()
  #(
    keep(case selector { True -> [] False -> [] }) == [],
    keep(case selector { True -> [[]] False -> [[]] }) == [[]],
    keep(case selector { True -> [1] False -> [2] }) == [2],
    keep(case selector { True -> ["one"] False -> ["two"] }) == ["two"],
    keep(case selector { True -> [<<1>>] False -> [<<2>>] }) == [<<2>>],
    keep(case selector { True -> [codepoint()] False -> [codepoint()] }) == [codepoint()],
    keep(case selector { True -> [Boxed(1)] False -> [Boxed(2)] }) == [Boxed(2)],
    keep(case selector { True -> [1.0] False -> [2.0] }) == [2.0],
    keep(case selector { True -> [True] False -> [False] }) == [False],
    keep(case selector { True -> [Nil] False -> [Nil] }) == [Nil],
    keep(case selector { True -> [#(1)] False -> [#(2)] }) == [#(2)],
    keep(case selector { True -> [[1]] False -> [[2]] }) == [[2]],
    keep(case selector { True -> [int_identity] False -> [int_identity] }) == [int_identity],
  )
}
"#,
            ),
            Value::Tuple(vec![Value::Bool(true); 13]),
        );
    }

    #[test]
    fn generic_bit_array_and_utf_codepoint_lists_preserve_storage_operations() {
        assert_eq!(
            crate::runtime::run_src(include_str!(
                "../../../tests/fixtures/execution/functions/generic_runtime_list_storage_paths.gleam"
            )),
            Value::Tuple(vec![Value::Bool(true); 6]),
        );
    }

    #[test]
    fn generic_nested_bit_array_and_utf_codepoint_lists_project_stored_values() {
        assert_eq!(
            crate::runtime::run_src(
                r#"
fn codepoint() -> UtfCodepoint {
  let assert <<value:utf8_codepoint>> = <<65>>
  value
}

fn first(values: List(List(value))) -> List(value) {
  case values {
    [first, ..] -> first
    _ -> []
  }
}

pub fn main() {
  #(
    first([[<<1>>]]) == [<<1>>],
    first([[codepoint()]]) == [codepoint()],
  )
}
"#,
            ),
            Value::Tuple(vec![Value::Bool(true); 2]),
        );
    }

    #[test]
    fn parameter_list_tuple_projection_reports_direct_mutated_family_mismatch() {
        let parameter = TypeParameterId(0);
        let expected = ValueType::List(Box::new(ValueType::Parameter(parameter)));
        let expression = ModuleListExpr::tuple_index(
            TupleExpr::value(
                vec![Expr::list(ModuleListExpr::value(
                    Vec::new(),
                    ValueType::Int,
                ))],
                vec![expected.clone()],
            ),
            0,
            ValueType::Parameter(parameter),
        )
        .into_generic()
        .expect("parameter list tuple projection should retain its generic item");

        assert_eq!(
            crate::run_main(&generic_list_plan(expression)),
            Err(ExecutionError::Invariant(
                InvariantError::TupleIndexFamilyMismatch {
                    expected: expected.clone(),
                    actual: ValueType::List(Box::new(ValueType::Int)),
                }
            )),
        );

        let expression = ModuleListExpr::tuple_index(
            TupleExpr::value(
                vec![Expr::int(IntExpr::value(1.into()))],
                vec![expected.clone()],
            ),
            0,
            ValueType::Parameter(parameter),
        )
        .into_generic()
        .expect("parameter list tuple projection should retain its generic item");
        assert_eq!(
            crate::run_main(&generic_list_plan(expression)),
            Err(ExecutionError::Invariant(
                InvariantError::TupleIndexFamilyMismatch {
                    expected,
                    actual: ValueType::Int,
                }
            )),
        );
    }

    #[test]
    fn parameter_list_projection_reports_only_missing_nested_index() {
        let parameter = TypeParameterId(0);
        let nested = ModuleListExpr::value(
            Vec::new(),
            ValueType::List(Box::new(ValueType::Parameter(parameter))),
        )
        .into_parameter_list()
        .expect("empty nested parameter list should retain its recursive item");
        let expression = ModuleListExpr::parameter_list_index(nested, 0)
            .into_generic()
            .expect("nested parameter list projection should retain its generic item");

        assert_eq!(
            crate::run_main(&generic_list_plan(expression)),
            Err(ExecutionError::Invariant(
                InvariantError::ListIndexOutOfBounds {
                    item_type: ValueType::List(Box::new(ValueType::Parameter(parameter))),
                    index: 0,
                    length: 0,
                }
            )),
        );
    }

    #[test]
    fn parameter_list_drop_first_preserves_empty_parameter_metadata() {
        let parameter = TypeParameterId(0);
        let expression = ModuleListExpr::drop_first(
            ModuleListExpr::value(Vec::new(), ValueType::Parameter(parameter)),
            1,
        )
        .into_generic()
        .expect("empty parameter list drop should retain its generic item");

        assert_eq!(
            crate::run_main(&generic_list_plan(expression)),
            Ok(Value::List(ListValue::empty(ValueType::Parameter(
                parameter
            )))),
        );
    }

    #[test]
    fn parameter_list_custom_field_reports_direct_mutated_family_mismatch() {
        let plan = parameter_list_custom_field_execution_plan();
        let main_id = main_parameter_list_id(&plan);
        let main = plan.parameter_list_function(main_id);
        let (target_id, args) = parameter_list_tail_call(main.return_());
        let binding = parameter_list_custom_argument(&args[0]);
        let construction = parameter_list_custom_construction(binding.value());
        let target = plan.parameter_list_function(target_id);
        let expression = parameter_list_expression_return(target.return_());
        let access = parameter_list_custom_field(expression);
        let int_type = plan.int_list_function_id(0).type_id();
        let mut state = RuntimeState::new();
        let wrong = state.int(int_type, Vec::new());
        let value = EvaluatedCustomValue::from_fields(
            construction.constructor(),
            vec![EvaluatedValue::List(wrong.into())].into_boxed_slice(),
        );
        let mut frame = Frame::new(target.frame_layout(), &mut state);
        frame.set_custom(binding.local(), value);
        let descriptor = plan.custom_constructor(construction.constructor());

        assert_eq!(
            super::eval_parameter_list_expr(&plan, &mut state, &mut frame, expression),
            Err(ExecutionError::Invariant(
                InvariantError::CustomFieldFamilyMismatch {
                    custom_type: plan.custom_value_type(construction.constructor().type_id()),
                    constructor: descriptor.name().clone(),
                    field_index: access.index(),
                    expected: plan.list_value_type(expression.item().type_id().list_type()),
                    actual: ValueType::List(Box::new(ValueType::Int)),
                }
            )),
        );
    }

    #[test]
    #[should_panic(expected = "expected a parameter-list main")]
    fn parameter_list_main_fixture_guard_rejects_int_main() {
        let plan = crate::runtime::plan_src("pub fn main() { 1 }");
        let _ = main_parameter_list_id(&plan);
    }

    #[test]
    #[should_panic(expected = "expected a parameter-list tail call")]
    fn parameter_list_tail_call_fixture_guard_rejects_expression_return() {
        let plan = parameter_list_expression_execution_plan();
        let main = plan.parameter_list_function(main_parameter_list_id(&plan));
        let _ = parameter_list_tail_call(main.return_());
    }

    #[test]
    #[should_panic(expected = "expected a custom call argument")]
    fn parameter_list_custom_argument_fixture_guard_rejects_int_argument() {
        let plan = crate::runtime::plan_src(
            "fn target(_value: Int) -> List(value) { [] }\npub fn main() { target(1) }",
        );
        let main = plan.parameter_list_function(main_parameter_list_id(&plan));
        let (_, args) = parameter_list_tail_call(main.return_());
        let _ = parameter_list_custom_argument(&args[0]);
    }

    #[test]
    #[should_panic(expected = "expected a custom construction")]
    fn parameter_list_custom_construction_fixture_guard_rejects_local_get() {
        let plan = parameter_list_custom_field_execution_plan();
        let main = plan.parameter_list_function(main_parameter_list_id(&plan));
        let (target_id, _) = parameter_list_tail_call(main.return_());
        let target = plan.parameter_list_function(target_id);
        let access =
            parameter_list_custom_field(parameter_list_expression_return(target.return_()));
        let _ = parameter_list_custom_construction(access.source());
    }

    #[test]
    #[should_panic(expected = "expected a parameter-list expression return")]
    fn parameter_list_expression_return_fixture_guard_rejects_tail_call() {
        let plan = parameter_list_custom_field_execution_plan();
        let main = plan.parameter_list_function(main_parameter_list_id(&plan));
        let _ = parameter_list_expression_return(main.return_());
    }

    #[test]
    #[should_panic(expected = "expected a parameter-list custom-field projection")]
    fn parameter_list_custom_field_fixture_guard_rejects_value() {
        let plan = parameter_list_expression_execution_plan();
        let main = plan.parameter_list_function(main_parameter_list_id(&plan));
        let expression = parameter_list_expression_return(main.return_());
        let _ = parameter_list_custom_field(expression);
    }

    #[test]
    fn source_custom_list_expression_variants_evaluate_exact_values() {
        let source = r#"
pub type Boxed {
  Boxed(Int)
}

fn boxed(value: Int) -> List(Boxed) {
  [Boxed(value)]
}

fn make_boxed(offset: Int) -> fn(Int) -> List(Boxed) {
  fn(value) { [Boxed(value + offset)] }
}

fn unbox(values: List(Boxed)) -> Int {
  case values {
    [Boxed(value)] -> value
    _ -> -1
  }
}

pub fn main() {
  let local = [Boxed(2)]
  let maker = make_boxed
  #(
    unbox([Boxed(0)]),
    unbox([Boxed(1), ..[]]),
    unbox(local),
    unbox(boxed(3)),
    unbox(make_boxed(1)(3)),
    unbox(maker(1)(4)),
    unbox(#([Boxed(6)]).0),
    case [[Boxed(7)]] { [values] -> unbox(values) _ -> -1 },
    unbox(case True { True -> [Boxed(8)] False -> [] }),
    unbox(case False { True -> [] False -> [Boxed(9)] }),
    unbox(case 1 { 1 -> [Boxed(10)] _ -> [] }),
    unbox(case 0 { 1 -> [] _ -> [Boxed(11)] }),
    unbox(case "hit" { "hit" -> [Boxed(12)] _ -> [] }),
    unbox(case "miss" { "hit" -> [] _ -> [Boxed(13)] }),
    unbox(case 1.0 { 1.0 -> [Boxed(14)] _ -> [] }),
    unbox(case 0.0 { 1.0 -> [] _ -> [Boxed(15)] }),
    case [Boxed(0), Boxed(16)] { [_, ..rest] -> unbox(rest) _ -> -1 },
    unbox({ let _ = 0 [Boxed(17)] }),
  )
}
"#;

        assert_eq!(
            crate::runtime::run_src(source),
            crate::runtime::Value::Tuple(
                (0_i64..=17)
                    .map(|value| crate::runtime::Value::Int(value.into()))
                    .collect(),
            ),
        );
    }

    #[test]
    fn list_projection_reports_out_of_bounds_for_every_item_family() {
        let plan = crate::runtime::plan_src(
            r#"
fn int_values(values: List(Int)) { values }
fn string_values(values: List(String)) { values }
fn bit_array_values(values: List(BitArray)) { values }
fn utf_codepoint_values(values: List(UtfCodepoint)) { values }
pub type Boxed { Boxed(Int) }
fn custom_values(values: List(Boxed)) { values }
fn float_values(values: List(Float)) { values }
fn bool_values(values: List(Bool)) { values }
fn nil_values(values: List(Nil)) { values }
fn tuple_values(values: List(#(Int))) { values }
fn nested_values(values: List(List(Int))) { values }
fn function_values(values: List(fn() -> Int)) { values }
pub fn main() {
  let _ = int_values
  let _ = string_values
  let _ = bit_array_values
  let _ = utf_codepoint_values
  let _ = custom_values
  let _ = float_values
  let _ = bool_values
  let _ = nil_values
  let _ = tuple_values
  let _ = nested_values
  let _ = function_values
  Nil
}
"#,
        );
        let mut state = RuntimeState::new();

        let function = plan.int_list_function(plan.int_list_function_id(0));
        let expression = expect_expression_return(function.return_());
        let mut frame = Frame::new(function.frame_layout(), &mut state);
        assert_eq!(
            project_int_list_expr(&plan, &mut state, &mut frame, expression, 0),
            Err(ExecutionError::Invariant(
                InvariantError::ListIndexOutOfBounds {
                    item_type: ValueType::Int,
                    index: 0,
                    length: 0,
                }
            )),
        );

        let function = plan.string_list_function(plan.string_list_function_id(0));
        let expression = expect_expression_return(function.return_());
        let mut frame = Frame::new(function.frame_layout(), &mut state);
        assert_eq!(
            project_string_list_expr(&plan, &mut state, &mut frame, expression, 0),
            Err(ExecutionError::Invariant(
                InvariantError::ListIndexOutOfBounds {
                    item_type: ValueType::String,
                    index: 0,
                    length: 0,
                }
            )),
        );

        let function = plan.bit_array_list_function(plan.bit_array_list_function_id(0));
        let expression = expect_expression_return(function.return_());
        let mut frame = Frame::new(function.frame_layout(), &mut state);
        assert_eq!(
            project_bit_array_list_expr(&plan, &mut state, &mut frame, expression, 0),
            Err(ExecutionError::Invariant(
                InvariantError::ListIndexOutOfBounds {
                    item_type: ValueType::BitArray,
                    index: 0,
                    length: 0,
                }
            )),
        );

        let function = plan.utf_codepoint_list_function(plan.utf_codepoint_list_function_id(0));
        let expression = expect_expression_return(function.return_());
        let mut frame = Frame::new(function.frame_layout(), &mut state);
        assert_eq!(
            project_utf_codepoint_list_expr(&plan, &mut state, &mut frame, expression, 0),
            Err(ExecutionError::Invariant(
                InvariantError::ListIndexOutOfBounds {
                    item_type: ValueType::UtfCodepoint,
                    index: 0,
                    length: 0,
                }
            )),
        );

        let function = plan.custom_list_function(plan.custom_list_function_id(0));
        let expression = expect_expression_return(function.return_());
        let item_type = expression.item().type_id().item_type();
        let mut frame = Frame::new(function.frame_layout(), &mut state);
        assert_eq!(
            project_custom_list_expr(&plan, &mut state, &mut frame, expression, 0, item_type,),
            Err(ExecutionError::Invariant(
                InvariantError::ListIndexOutOfBounds {
                    item_type: ValueType::Custom(plan.custom_value_type(item_type)),
                    index: 0,
                    length: 0,
                }
            )),
        );

        let function = plan.float_list_function(plan.float_list_function_id(0));
        let expression = expect_expression_return(function.return_());
        let mut frame = Frame::new(function.frame_layout(), &mut state);
        assert_eq!(
            project_float_list_expr(&plan, &mut state, &mut frame, expression, 0),
            Err(ExecutionError::Invariant(
                InvariantError::ListIndexOutOfBounds {
                    item_type: ValueType::Float,
                    index: 0,
                    length: 0,
                }
            )),
        );

        let function = plan.bool_list_function(plan.bool_list_function_id(0));
        let expression = expect_expression_return(function.return_());
        let mut frame = Frame::new(function.frame_layout(), &mut state);
        assert_eq!(
            project_bool_list_expr(&plan, &mut state, &mut frame, expression, 0),
            Err(ExecutionError::Invariant(
                InvariantError::ListIndexOutOfBounds {
                    item_type: ValueType::Bool,
                    index: 0,
                    length: 0,
                }
            )),
        );

        let function = plan.nil_list_function(plan.nil_list_function_id(0));
        let expression = expect_expression_return(function.return_());
        let mut frame = Frame::new(function.frame_layout(), &mut state);
        assert_eq!(
            project_nil_list_expr(&plan, &mut state, &mut frame, expression, 0),
            Err(ExecutionError::Invariant(
                InvariantError::ListIndexOutOfBounds {
                    item_type: ValueType::Nil,
                    index: 0,
                    length: 0,
                }
            )),
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
            Err(ExecutionError::Invariant(
                InvariantError::ListIndexOutOfBounds {
                    item_type: ValueType::Tuple(vec![ValueType::Int]),
                    index: 0,
                    length: 0,
                }
            )),
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
            Err(ExecutionError::Invariant(
                InvariantError::ListIndexOutOfBounds {
                    item_type: ValueType::Function(Box::new(function_type)),
                    index: 0,
                    length: 0,
                }
            )),
        );
    }

    #[test]
    fn nested_list_projection_reports_only_the_missing_index_for_every_item_family() {
        let plan = crate::runtime::plan_src(
            r#"
fn first_int(values: List(List(Int))) -> List(Int) { case values { [first, ..] -> first _ -> [] } }
fn first_string(values: List(List(String))) -> List(String) { case values { [first, ..] -> first _ -> [] } }
fn first_bit_array(values: List(List(BitArray))) -> List(BitArray) { case values { [first, ..] -> first _ -> [] } }
fn first_utf_codepoint(values: List(List(UtfCodepoint))) -> List(UtfCodepoint) { case values { [first, ..] -> first _ -> [] } }
pub type Boxed { Boxed(Int) }
fn first_custom(values: List(List(Boxed))) -> List(Boxed) { case values { [first, ..] -> first _ -> [] } }
fn first_float(values: List(List(Float))) -> List(Float) { case values { [first, ..] -> first _ -> [] } }
fn first_bool(values: List(List(Bool))) -> List(Bool) { case values { [first, ..] -> first _ -> [] } }
fn first_nil(values: List(List(Nil))) -> List(Nil) { case values { [first, ..] -> first _ -> [] } }
fn first_tuple(values: List(List(#(Int)))) -> List(#(Int)) { case values { [first, ..] -> first _ -> [] } }
fn first_list(values: List(List(List(Int)))) -> List(List(Int)) { case values { [first, ..] -> first _ -> [] } }
fn first_function(values: List(List(fn() -> Int))) -> List(fn() -> Int) { case values { [first, ..] -> first _ -> [] } }
fn first_parameter(values: List(List(value))) -> List(value) { case values { [first, ..] -> first _ -> [] } }
fn first_parameter_list(values: List(List(List(value)))) -> List(List(value)) { case values { [first, ..] -> first _ -> [] } }
pub fn main() {
  let _ = first_int
  let _ = first_string
  let _ = first_bit_array
  let _ = first_utf_codepoint
  let _ = first_custom
  let _ = first_float
  let _ = first_bool
  let _ = first_nil
  let _ = first_tuple
  let _ = first_list
  let _ = first_function
  let _ = first_parameter([[]])
  let _ = first_parameter_list([[[]]])
  Nil
}
"#,
        );
        let mut state = RuntimeState::new();

        let function = plan.parameter_list_function(plan.parameter_list_function_id(0));
        let mut frame = Frame::new(function.frame_layout(), &mut state);
        assert_nested_list_out_of_bounds(
            &plan,
            &mut state,
            &mut frame,
            expect_nested_list_binding(function.return_()),
        );

        let function = plan.parameter_list_list_function(plan.parameter_list_list_function_id(0));
        let mut frame = Frame::new(function.frame_layout(), &mut state);
        assert_nested_list_out_of_bounds(
            &plan,
            &mut state,
            &mut frame,
            expect_nested_list_binding(function.return_()),
        );

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

        let function = plan.custom_list_function(plan.custom_list_function_id(0));
        let mut frame = Frame::new(function.frame_layout(), &mut state);
        assert_nested_list_out_of_bounds(
            &plan,
            &mut state,
            &mut frame,
            expect_nested_list_binding(function.return_()),
        );

        let function = plan.utf_codepoint_list_function(plan.utf_codepoint_list_function_id(0));
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
pub fn main() {
  let _ = int_from_tuple
  let _ = strings
  Nil
}
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
            Err(ExecutionError::Invariant(
                InvariantError::TupleIndexFamilyMismatch {
                    expected: ValueType::List(Box::new(ValueType::Int)),
                    actual: ValueType::List(Box::new(ValueType::String)),
                }
            )),
        );
    }

    #[test]
    fn tuple_list_family_conversion_rejects_every_wrong_handle_family() {
        let plan = crate::runtime::plan_src(
            r#"
fn ints() -> List(Int) { [] }
fn strings() -> List(String) { [] }
fn bit_arrays() -> List(BitArray) { [] }
fn utf_codepoints() -> List(UtfCodepoint) { [] }
fn floats() -> List(Float) { [] }
fn bools() -> List(Bool) { [] }
fn nils() -> List(Nil) { [] }
fn tuples() -> List(#(Int)) { [] }
fn lists() -> List(List(Int)) { [] }
fn functions() -> List(fn() -> Int) { [] }
pub fn main() {
  let _ = ints
  let _ = strings
  let _ = bit_arrays
  let _ = utf_codepoints
  let _ = floats
  let _ = bools
  let _ = nils
  let _ = tuples
  let _ = lists
  let _ = functions
  Nil
}
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
            <BitArrayListItem as super::RuntimeListItem>::from_tuple_value(int.clone().into()),
            None,
        );
        assert_eq!(
            <UtfCodepointListItem as super::RuntimeListItem>::from_tuple_value(int.clone().into()),
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
        assert_eq!(
            <ParameterListListItem as super::RuntimeListItem>::from_tuple_value(string.into()),
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
    fn module_dependency_errors_propagate_through_parameter_list_wrappers() {
        let parameter = TypeParameterId(0);
        let panic = || PanicExpr::panic_at(None, PanicSite::unknown());
        let empty = || {
            ModuleListExpr::try_value(Vec::new(), ValueType::Parameter(parameter))
                .expect("empty parameter list should retain its item metadata")
                .into_generic()
                .expect("parameter item should create a generic list expression")
        };
        let panic_list = || {
            ModuleListExpr::panic(panic(), ValueType::Parameter(parameter))
                .into_generic()
                .expect("panic parameter list should retain its item metadata")
        };
        let nested_source = ModuleListExpr::panic(
            panic(),
            ValueType::List(Box::new(ValueType::Parameter(parameter))),
        )
        .into_parameter_list()
        .expect("nested parameter list should retain its recursive item metadata");
        let expressions = vec![
            ModuleListExpr::tuple_index(
                TupleExpr::panic(
                    panic(),
                    vec![ValueType::List(Box::new(ValueType::Parameter(parameter)))],
                ),
                0,
                ValueType::Parameter(parameter),
            )
            .into_generic()
            .expect("tuple projection should retain its parameter item"),
            ModuleListExpr::parameter_list_index(nested_source, 0)
                .into_generic()
                .expect("nested projection should retain its parameter item"),
            ModuleListExpr::drop_first(ModuleListExpr::Generic(panic_list()), 1)
                .into_generic()
                .expect("drop-first should retain its parameter item"),
            ModuleListExpr::bool_case(
                BoolExpr::panic(panic()),
                BoolListCaseBranches::Generic {
                    true_: empty(),
                    false_: empty(),
                },
            )
            .into_generic()
            .expect("Bool case should retain its parameter item"),
            ModuleListExpr::int_case(
                IntExpr::panic(panic()),
                ListCaseBranches::Generic {
                    clauses: Vec::new(),
                    fallback: empty(),
                },
            )
            .into_generic()
            .expect("Int case should retain its parameter item"),
            ModuleListExpr::string_case(
                StringExpr::panic(panic()),
                ListCaseBranches::Generic {
                    clauses: Vec::new(),
                    fallback: empty(),
                },
            )
            .into_generic()
            .expect("String case should retain its parameter item"),
            ModuleListExpr::float_case(
                FloatExpr::panic(panic()),
                ListCaseBranches::Generic {
                    clauses: Vec::new(),
                    fallback: empty(),
                },
            )
            .into_generic()
            .expect("Float case should retain its parameter item"),
            ModuleListExpr::block(
                vec![Step::evaluate(Expr::int(IntExpr::panic(panic())))],
                ModuleListExpr::Generic(empty()),
            )
            .into_generic()
            .expect("block should retain its parameter item"),
        ];

        for expression in expressions {
            assert_eq!(
                crate::runtime::run_main(&generic_list_plan(expression)),
                Err(ExecutionError::source_panic(
                    None,
                    crate::runtime::PanicKind::Panic,
                    None,
                    PanicSite::unknown(),
                )),
            );
        }
    }

    #[test]
    fn parameter_list_custom_field_and_nested_elements_propagate_source_errors() {
        let custom_field = r#"
pub type Box(value) {
  Box(values: List(value))
}

fn fail() -> Box(value) {
  panic as "custom field"
}

pub fn main() {
  fail().values
}
"#;
        let nested_element = r#"
fn fail() -> List(value) {
  panic as "element"
}

pub fn main() {
  [fail()]
}
"#;

        assert_eq!(
            crate::runtime::run_src_error(custom_field).to_string(),
            "panic: custom field",
        );
        assert_eq!(
            crate::runtime::run_src_error(nested_element).to_string(),
            "panic: element",
        );
    }

    #[test]
    fn parameter_list_never_element_propagates_its_source_error() {
        assert_eq!(
            crate::runtime::run_src_error(include_str!(
                "../../../tests/fixtures/execution_errors/expressions/panic_nested_generic_list_item.gleam"
            ))
            .to_string(),
            "panic: nested generic list item failed",
        );
    }

    #[test]
    fn list_projection_propagates_source_errors_for_primitive_nil_tuple_and_custom_lists() {
        let plan = crate::runtime::plan_src(
            r#"
fn ints() -> List(Int) { panic }
fn strings() -> List(String) { panic }
fn bit_arrays() -> List(BitArray) { panic }
fn utf_codepoints() -> List(UtfCodepoint) { panic }
fn floats() -> List(Float) { panic }
fn bools() -> List(Bool) { panic }
fn nils() -> List(Nil) { panic }
fn tuples() -> List(#(Int)) { panic }
pub type Boxed { Boxed(Int) }
fn customs() -> List(Boxed) { panic }
pub fn main() {
  let _ = ints
  let _ = strings
  let _ = bit_arrays
  let _ = utf_codepoints
  let _ = floats
  let _ = bools
  let _ = nils
  let _ = tuples
  let _ = customs
  Nil
}
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

        let function = plan.string_list_function(plan.string_list_function_id(0));
        let expression = expect_expression_return(function.return_());
        let mut frame = Frame::new(function.frame_layout(), &mut state);
        assert_eq!(
            project_string_list_expr(&plan, &mut state, &mut frame, expression, 0)
                .expect_err("String list source should fail")
                .to_string(),
            "panic: `panic` expression evaluated.",
        );

        let function = plan.bit_array_list_function(plan.bit_array_list_function_id(0));
        let expression = expect_expression_return(function.return_());
        let mut frame = Frame::new(function.frame_layout(), &mut state);
        assert_eq!(
            project_bit_array_list_expr(&plan, &mut state, &mut frame, expression, 0)
                .expect_err("BitArray list source should fail")
                .to_string(),
            "panic: `panic` expression evaluated.",
        );

        let function = plan.utf_codepoint_list_function(plan.utf_codepoint_list_function_id(0));
        let expression = expect_expression_return(function.return_());
        let mut frame = Frame::new(function.frame_layout(), &mut state);
        assert_eq!(
            project_utf_codepoint_list_expr(&plan, &mut state, &mut frame, expression, 0)
                .expect_err("UtfCodepoint list source should fail")
                .to_string(),
            "panic: `panic` expression evaluated.",
        );

        let function = plan.float_list_function(plan.float_list_function_id(0));
        let expression = expect_expression_return(function.return_());
        let mut frame = Frame::new(function.frame_layout(), &mut state);
        assert_eq!(
            project_float_list_expr(&plan, &mut state, &mut frame, expression, 0)
                .expect_err("Float list source should fail")
                .to_string(),
            "panic: `panic` expression evaluated.",
        );

        let function = plan.bool_list_function(plan.bool_list_function_id(0));
        let expression = expect_expression_return(function.return_());
        let mut frame = Frame::new(function.frame_layout(), &mut state);
        assert_eq!(
            project_bool_list_expr(&plan, &mut state, &mut frame, expression, 0)
                .expect_err("Bool list source should fail")
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

        let function = plan.custom_list_function(plan.custom_list_function_id(0));
        let expression = expect_expression_return(function.return_());
        let item_type = expression.item().type_id().item_type();
        let mut frame = Frame::new(function.frame_layout(), &mut state);
        assert_eq!(
            project_custom_list_expr(&plan, &mut state, &mut frame, expression, 0, item_type,)
                .expect_err("custom list source should fail")
                .to_string(),
            "panic: `panic` expression evaluated.",
        );
    }

    #[test]
    #[should_panic(expected = "expected a list expression return body")]
    fn expression_return_fixture_guard_rejects_tail_calls() {
        let plan = crate::runtime::plan_src(
            "fn recurse() -> List(Int) { recurse() } pub fn main() { let _ = recurse Nil }",
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
            "fn empty(values: List(Int)) -> List(Int) { case values { [] -> [] _ -> [] } } pub fn main() { let _ = empty Nil }",
        );
        let function = plan.int_list_function(plan.int_list_function_id(0));
        let _ = expect_nested_list_binding(function.return_());
    }

    #[test]
    #[should_panic(expected = "expected a list binding step")]
    fn nested_projection_fixture_guard_rejects_int_bindings() {
        let plan = crate::runtime::plan_src(
            "fn first(values: List(Int)) -> List(Int) { case values { [first, ..] -> [] _ -> [] } } pub fn main() { let _ = first Nil }",
        );
        let function = plan.int_list_function(plan.int_list_function_id(0));
        let _ = expect_nested_list_binding(function.return_());
    }

    fn expect_expression_return<Expression, Function>(
        graph: &ReturnGraph<Expression, Function>,
    ) -> &Expression {
        match graph.block(graph.entry()) {
            ReturnBlock::Return { expression } => graph.expression(*expression),
            _ => panic!("expected a list expression return body"),
        }
    }

    fn expect_nested_list_binding<Expression, Function>(
        graph: &ReturnGraph<Expression, Function>,
    ) -> &crate::plan::execution::ListLocalExpr {
        let ReturnBlock::Steps { next, .. } = graph.block(graph.entry()) else {
            panic!("expected a block return body");
        };
        let ReturnBlock::BoolBranch { true_, .. } = graph.block(*next) else {
            panic!("expected a Bool case return body");
        };
        let ReturnBlock::Steps { steps, .. } = graph.block(*true_) else {
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
            crate::plan::execution::ListLocalExpr::Parameter { value, .. } => assert_eq!(
                super::eval_parameter_list_expr(plan, state, frame, value),
                Err(ExecutionError::Invariant(
                    InvariantError::ListIndexOutOfBounds {
                        item_type: ValueType::List(Box::new(ValueType::Parameter(
                            value.item().type_id().item(),
                        ))),
                        index: 0,
                        length: 0,
                    }
                )),
            ),
            crate::plan::execution::ListLocalExpr::Int { value, .. } => assert_eq!(
                super::eval_int_list_expr(plan, state, frame, value),
                Err(ExecutionError::Invariant(
                    InvariantError::ListIndexOutOfBounds {
                        item_type: ValueType::List(Box::new(ValueType::Int)),
                        index: 0,
                        length: 0,
                    }
                )),
            ),
            crate::plan::execution::ListLocalExpr::String { value, .. } => assert_eq!(
                super::eval_string_list_expr(plan, state, frame, value),
                Err(ExecutionError::Invariant(
                    InvariantError::ListIndexOutOfBounds {
                        item_type: ValueType::List(Box::new(ValueType::String)),
                        index: 0,
                        length: 0,
                    }
                )),
            ),
            crate::plan::execution::ListLocalExpr::BitArray { value, .. } => assert_eq!(
                super::eval_bit_array_list_expr(plan, state, frame, value),
                Err(ExecutionError::Invariant(
                    InvariantError::ListIndexOutOfBounds {
                        item_type: ValueType::List(Box::new(ValueType::BitArray)),
                        index: 0,
                        length: 0,
                    }
                )),
            ),
            crate::plan::execution::ListLocalExpr::UtfCodepoint { value, .. } => assert_eq!(
                super::eval_utf_codepoint_list_expr(plan, state, frame, value),
                Err(ExecutionError::Invariant(
                    InvariantError::ListIndexOutOfBounds {
                        item_type: ValueType::List(Box::new(ValueType::UtfCodepoint)),
                        index: 0,
                        length: 0,
                    }
                )),
            ),
            crate::plan::execution::ListLocalExpr::Custom { value, .. } => assert_eq!(
                super::eval_custom_list_expr(plan, state, frame, value),
                Err(ExecutionError::Invariant(
                    InvariantError::ListIndexOutOfBounds {
                        item_type: ValueType::List(Box::new(ValueType::Custom(
                            plan.custom_value_type(value.item().type_id().item_type()),
                        ))),
                        index: 0,
                        length: 0,
                    }
                )),
            ),
            crate::plan::execution::ListLocalExpr::Float { value, .. } => assert_eq!(
                super::eval_float_list_expr(plan, state, frame, value),
                Err(ExecutionError::Invariant(
                    InvariantError::ListIndexOutOfBounds {
                        item_type: ValueType::List(Box::new(ValueType::Float)),
                        index: 0,
                        length: 0,
                    }
                )),
            ),
            crate::plan::execution::ListLocalExpr::Bool { value, .. } => assert_eq!(
                super::eval_bool_list_expr(plan, state, frame, value),
                Err(ExecutionError::Invariant(
                    InvariantError::ListIndexOutOfBounds {
                        item_type: ValueType::List(Box::new(ValueType::Bool)),
                        index: 0,
                        length: 0,
                    }
                )),
            ),
            crate::plan::execution::ListLocalExpr::Nil { value, .. } => assert_eq!(
                super::eval_nil_list_expr(plan, state, frame, value),
                Err(ExecutionError::Invariant(
                    InvariantError::ListIndexOutOfBounds {
                        item_type: ValueType::List(Box::new(ValueType::Nil)),
                        index: 0,
                        length: 0,
                    }
                )),
            ),
            crate::plan::execution::ListLocalExpr::Tuple { value, .. } => assert_eq!(
                super::eval_tuple_list_expr(plan, state, frame, value),
                Err(ExecutionError::Invariant(
                    InvariantError::ListIndexOutOfBounds {
                        item_type: ValueType::List(Box::new(ValueType::Tuple(vec![
                            ValueType::Int
                        ]))),
                        index: 0,
                        length: 0,
                    }
                )),
            ),
            crate::plan::execution::ListLocalExpr::ParameterList { value, .. } => assert_eq!(
                super::eval_parameter_list_list_expr(plan, state, frame, value),
                Err(ExecutionError::Invariant(
                    InvariantError::ListIndexOutOfBounds {
                        item_type: ValueType::List(Box::new(ValueType::List(Box::new(
                            ValueType::Parameter(value.item().type_id().item_type().item()),
                        )))),
                        index: 0,
                        length: 0,
                    }
                )),
            ),
            crate::plan::execution::ListLocalExpr::List { value, .. } => assert_eq!(
                super::eval_list_list_expr(plan, state, frame, value),
                Err(ExecutionError::Invariant(
                    InvariantError::ListIndexOutOfBounds {
                        item_type: ValueType::List(Box::new(ValueType::List(Box::new(
                            ValueType::Int
                        )))),
                        index: 0,
                        length: 0,
                    }
                )),
            ),
            crate::plan::execution::ListLocalExpr::Function { value, .. } => assert_eq!(
                super::eval_function_list_expr(plan, state, frame, value),
                Err(ExecutionError::Invariant(
                    InvariantError::ListIndexOutOfBounds {
                        item_type: ValueType::List(Box::new(ValueType::Function(Box::new(
                            FunctionType::new(Vec::new(), ValueType::Int),
                        )))),
                        index: 0,
                        length: 0,
                    }
                )),
            ),
        }
    }

    fn parameter_list_custom_field_execution_plan() -> crate::ExecutionPlan {
        crate::runtime::plan_src(
            r#"
pub type Box(value) {
  Box(values: List(value))
}

fn ints() -> List(Int) {
  []
}

fn get(box: Box(value)) {
  box.values
}

pub fn main() {
  let _ = ints
  get(Box([]))
}
"#,
        )
    }

    fn parameter_list_expression_execution_plan() -> crate::ExecutionPlan {
        crate::runtime::plan_src("pub fn main() { [] }")
    }

    fn main_parameter_list_id(plan: &crate::ExecutionPlan) -> ParameterListFunctionId {
        match plan.main_runtime() {
            RuntimeFunctionId::List(ListFunctionId::Parameter(id)) => id,
            _ => panic!("expected a parameter-list main"),
        }
    }

    fn parameter_list_tail_call(
        graph: &ReturnGraph<ParameterListExpr, ParameterListFunctionId>,
    ) -> (ParameterListFunctionId, &[CallArg]) {
        match graph.block(graph.entry()) {
            ReturnBlock::TailCall { call } => {
                let call = graph.tail_call(*call);
                (*call.function(), call.args())
            }
            _ => panic!("expected a parameter-list tail call"),
        }
    }

    fn parameter_list_custom_argument(argument: &CallArg) -> &CustomLocalExpr {
        match argument.kind() {
            CallArgKind::Custom(binding) => binding,
            _ => panic!("expected a custom call argument"),
        }
    }

    fn parameter_list_custom_construction(expression: &CustomExpr) -> &CustomConstruction {
        match expression.kind() {
            CustomExprKind::Constructor(construction) => construction,
            _ => panic!("expected a custom construction"),
        }
    }

    fn parameter_list_expression_return(
        graph: &ReturnGraph<ParameterListExpr, ParameterListFunctionId>,
    ) -> &ParameterListExpr {
        match graph.block(graph.entry()) {
            ReturnBlock::Return { expression } => graph.expression(*expression),
            _ => panic!("expected a parameter-list expression return"),
        }
    }

    fn parameter_list_custom_field(expression: &ParameterListExpr) -> &CustomFieldAccess {
        match expression.kind() {
            ParameterListExprKind::CustomField(access) => access,
            _ => panic!("expected a parameter-list custom-field projection"),
        }
    }

    fn run_module_int_list_expression(expression: IntListExpr) -> ExecutionError {
        let main = FunctionTemplate::new(
            FunctionTemplateId::new(0),
            "main".into(),
            Vec::new(),
            Vec::new(),
            ReturnExpr::int_list_body(crate::plan::IntListReturn::expr(expression)),
        );
        let module = ModulePlan::new("main".into(), main, Vec::new());
        let plan = crate::ExecutionPlan::from_module_plan(module);

        crate::runtime::run_main(&plan)
            .expect_err("module Int list expression should fail at runtime")
    }

    fn run_module_nil_list_expression(expression: NilListExpr) -> ExecutionError {
        let main = FunctionTemplate::new(
            FunctionTemplateId::new(0),
            "main".into(),
            Vec::new(),
            Vec::new(),
            ReturnExpr::nil_list_body(crate::plan::NilListReturn::expr(expression)),
        );
        let module = ModulePlan::new("main".into(), main, Vec::new());
        let plan = crate::ExecutionPlan::from_module_plan(module);

        crate::runtime::run_main(&plan)
            .expect_err("module Nil list expression should fail at runtime")
    }

    fn generic_list_plan(expression: GenericListExpr) -> crate::ExecutionPlan {
        let parameter = expression.item().parameter();
        let return_shape = ValueStorageShape::List(Box::new(ValueShape::Parameter(parameter)));
        let signature = FunctionTemplateSignature::new(
            FunctionTemplateId::new(0),
            TypeScheme::new(1),
            FunctionShape::new(Vec::new(), return_shape.to_value_shape()),
        );
        let main = FunctionTemplate::from_signature(
            signature,
            "main".into(),
            Vec::new(),
            Vec::new(),
            ReturnExpr::generic_list_body(parameter, GenericListReturn::expr(expression)),
        );
        let module = ModulePlan::new("main".into(), main, Vec::new());

        crate::ExecutionPlan::from_module_plan(module)
    }
}
