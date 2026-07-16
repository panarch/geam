mod case;
mod elements;
mod item;
mod local;
mod typed;

pub(crate) use self::{
    case::{BoolListCaseBranches, ListCaseBranches},
    elements::{ListElementTypeMismatch, ListElements, ListSpreadElements},
    item::{
        BitArrayListItem, BoolListItem, CustomListItem, FloatListItem, FunctionListItem,
        IntListItem, ListItem, ListListItem, NilListItem, StringListItem, TupleListItem,
        UtfCodepointListItem,
    },
    local::ListLocalExpr,
    typed::{ListIndexSource, TypedListExpr, TypedListExprKind, TypedListReturnKind},
};
use super::{
    BoolExpr, CallArg, CustomFieldAccess, Expr, FloatExpr, IntExpr, ListFunctionExpr, PanicExpr,
    StringExpr, TupleExpr,
};
use crate::plan::{ListFunctionId, ListLocal, Step, ValueType};
use ecow::EcoString;
use num_bigint::BigInt;

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum ListExpr {
    Int(IntListExpr),
    String(StringListExpr),
    BitArray(BitArrayListExpr),
    UtfCodepoint(UtfCodepointListExpr),
    Custom(CustomListExpr),
    Float(FloatListExpr),
    Bool(BoolListExpr),
    Nil(NilListExpr),
    Tuple(TupleListExpr),
    List(ListListExpr),
    Function(FunctionListExpr),
}

pub(crate) type IntListExpr = TypedListExpr<IntListItem>;
pub(crate) type StringListExpr = TypedListExpr<StringListItem>;
pub(crate) type BitArrayListExpr = TypedListExpr<BitArrayListItem>;
pub(crate) type UtfCodepointListExpr = TypedListExpr<UtfCodepointListItem>;
pub(crate) type CustomListExpr = TypedListExpr<CustomListItem>;
pub(crate) type FloatListExpr = TypedListExpr<FloatListItem>;
pub(crate) type BoolListExpr = TypedListExpr<BoolListItem>;
pub(crate) type NilListExpr = TypedListExpr<NilListItem>;
pub(crate) type TupleListExpr = TypedListExpr<TupleListItem>;
pub(crate) type ListListExpr = TypedListExpr<ListListItem>;
pub(crate) type FunctionListExpr = TypedListExpr<FunctionListItem>;

impl ListExpr {
    pub(crate) fn try_value(
        elements: Vec<Expr>,
        element_type: ValueType,
    ) -> Result<Self, ListElementTypeMismatch> {
        let item_shape = list_item_shape(&elements, &element_type)?;
        ListElements::from_exprs(element_type, elements)
            .map(Self::from_elements)
            .map(|value| value.with_item_shape(item_shape))
    }

    pub(crate) fn from_elements(elements: ListElements) -> Self {
        match elements {
            ListElements::Int(values) => Self::Int(IntListExpr::value(IntListItem, values)),
            ListElements::String(values) => {
                Self::String(StringListExpr::value(StringListItem, values))
            }
            ListElements::BitArray(values) => {
                Self::BitArray(BitArrayListExpr::value(BitArrayListItem, values))
            }
            ListElements::UtfCodepoint(values) => {
                Self::UtfCodepoint(UtfCodepointListExpr::value(UtfCodepointListItem, values))
            }
            ListElements::Custom { item_type, values } => {
                Self::Custom(CustomListExpr::value(CustomListItem { item_type }, values))
            }
            ListElements::Float(values) => Self::Float(FloatListExpr::value(FloatListItem, values)),
            ListElements::Bool(values) => Self::Bool(BoolListExpr::value(BoolListItem, values)),
            ListElements::Nil(values) => Self::Nil(NilListExpr::value(NilListItem, values)),
            ListElements::Tuple { item_type, values } => {
                let item = TupleListItem { item_type };
                Self::Tuple(TupleListExpr::value(item, values))
            }
            ListElements::List { item_type, values } => {
                let item = ListListItem { item_type };
                Self::List(ListListExpr::value(item, values))
            }
            ListElements::Function { item_type, values } => {
                let item = FunctionListItem { item_type };
                Self::Function(FunctionListExpr::value(item, values))
            }
        }
    }

    pub(crate) fn from_spread_elements(elements: ListSpreadElements) -> Self {
        match elements {
            ListSpreadElements::Int { values, tail } => {
                Self::Int(IntListExpr::spread(values, tail))
            }
            ListSpreadElements::String { values, tail } => {
                Self::String(StringListExpr::spread(values, tail))
            }
            ListSpreadElements::BitArray { values, tail } => {
                Self::BitArray(BitArrayListExpr::spread(values, tail))
            }
            ListSpreadElements::UtfCodepoint { values, tail } => {
                Self::UtfCodepoint(UtfCodepointListExpr::spread(values, tail))
            }
            ListSpreadElements::Custom { values, tail } => {
                Self::Custom(CustomListExpr::spread(values, tail))
            }
            ListSpreadElements::Float { values, tail } => {
                Self::Float(FloatListExpr::spread(values, tail))
            }
            ListSpreadElements::Bool { values, tail } => {
                Self::Bool(BoolListExpr::spread(values, tail))
            }
            ListSpreadElements::Nil { values, tail } => {
                Self::Nil(NilListExpr::spread(values, tail))
            }
            ListSpreadElements::Tuple { values, tail } => {
                Self::Tuple(TupleListExpr::spread(values, tail))
            }
            ListSpreadElements::List { values, tail } => {
                Self::List(ListListExpr::spread(values, tail))
            }
            ListSpreadElements::Function { values, tail } => {
                Self::Function(FunctionListExpr::spread(values, tail))
            }
        }
    }

    pub(crate) fn local_get(local: ListLocal, name: EcoString) -> Self {
        match local {
            ListLocal::Int(local) => Self::Int(IntListExpr::local_get(IntListItem, local, name)),
            ListLocal::String(local) => {
                Self::String(StringListExpr::local_get(StringListItem, local, name))
            }
            ListLocal::BitArray(local) => {
                Self::BitArray(BitArrayListExpr::local_get(BitArrayListItem, local, name))
            }
            ListLocal::UtfCodepoint(local) => Self::UtfCodepoint(UtfCodepointListExpr::local_get(
                UtfCodepointListItem,
                local,
                name,
            )),
            ListLocal::Custom { local, item_type } => Self::Custom(CustomListExpr::local_get(
                CustomListItem { item_type },
                local,
                name,
            )),
            ListLocal::Float(local) => {
                Self::Float(FloatListExpr::local_get(FloatListItem, local, name))
            }
            ListLocal::Bool(local) => {
                Self::Bool(BoolListExpr::local_get(BoolListItem, local, name))
            }
            ListLocal::Nil(local) => Self::Nil(NilListExpr::local_get(NilListItem, local, name)),
            ListLocal::Tuple { local, item_type } => {
                let item = TupleListItem { item_type };
                Self::Tuple(TupleListExpr::local_get(item, local, name))
            }
            ListLocal::List { local, item_type } => {
                let item = ListListItem { item_type };
                Self::List(ListListExpr::local_get(item, local, name))
            }
            ListLocal::Function { local, item_type } => {
                let item = FunctionListItem { item_type };
                Self::Function(FunctionListExpr::local_get(item, local, name))
            }
        }
    }

    pub(crate) fn call(function: ListFunctionId, args: Vec<CallArg>) -> Self {
        match function {
            ListFunctionId::Int(function) => {
                Self::Int(IntListExpr::call(IntListItem, function, args))
            }
            ListFunctionId::String(function) => {
                Self::String(StringListExpr::call(StringListItem, function, args))
            }
            ListFunctionId::BitArray(function) => {
                Self::BitArray(BitArrayListExpr::call(BitArrayListItem, function, args))
            }
            ListFunctionId::UtfCodepoint(function) => Self::UtfCodepoint(
                UtfCodepointListExpr::call(UtfCodepointListItem, function, args),
            ),
            ListFunctionId::Custom { id, item_type } => {
                Self::Custom(CustomListExpr::call(CustomListItem { item_type }, id, args))
            }
            ListFunctionId::Float(function) => {
                Self::Float(FloatListExpr::call(FloatListItem, function, args))
            }
            ListFunctionId::Bool(function) => {
                Self::Bool(BoolListExpr::call(BoolListItem, function, args))
            }
            ListFunctionId::Nil(function) => {
                Self::Nil(NilListExpr::call(NilListItem, function, args))
            }
            ListFunctionId::Tuple { id, item_type } => {
                let item = TupleListItem { item_type };
                Self::Tuple(TupleListExpr::call(item, id, args))
            }
            ListFunctionId::List { id, item_type } => {
                let item = ListListItem { item_type };
                Self::List(ListListExpr::call(item, id, args))
            }
            ListFunctionId::Function { id, item_type } => {
                let item = FunctionListItem { item_type };
                Self::Function(FunctionListExpr::call(item, id, args))
            }
        }
    }

    pub(crate) fn function_call(function: ListFunctionExpr, args: Vec<CallArg>) -> Self {
        match function.return_item_type() {
            ValueType::Int => Self::Int(IntListExpr::function_call(IntListItem, function, args)),
            ValueType::String => Self::String(StringListExpr::function_call(
                StringListItem,
                function,
                args,
            )),
            ValueType::BitArray => Self::BitArray(BitArrayListExpr::function_call(
                BitArrayListItem,
                function,
                args,
            )),
            ValueType::UtfCodepoint => Self::UtfCodepoint(UtfCodepointListExpr::function_call(
                UtfCodepointListItem,
                function,
                args,
            )),
            ValueType::Custom(item_type) => Self::Custom(CustomListExpr::function_call(
                CustomListItem { item_type },
                function,
                args,
            )),
            ValueType::Float => {
                Self::Float(FloatListExpr::function_call(FloatListItem, function, args))
            }
            ValueType::Bool => {
                Self::Bool(BoolListExpr::function_call(BoolListItem, function, args))
            }
            ValueType::Nil => Self::Nil(NilListExpr::function_call(NilListItem, function, args)),
            ValueType::Tuple(item_type) => {
                let item = TupleListItem { item_type };
                Self::Tuple(TupleListExpr::function_call(item, function, args))
            }
            ValueType::List(item_type) => {
                let item = ListListItem { item_type };
                Self::List(ListListExpr::function_call(item, function, args))
            }
            ValueType::Function(item_type) => {
                let item = FunctionListItem {
                    item_type: *item_type,
                };
                Self::Function(FunctionListExpr::function_call(item, function, args))
            }
        }
    }

    pub(crate) fn tuple_index(tuple: TupleExpr, index: usize, element_type: ValueType) -> Self {
        match element_type {
            ValueType::Int => Self::Int(IntListExpr::tuple_index(IntListItem, tuple, index)),
            ValueType::String => {
                Self::String(StringListExpr::tuple_index(StringListItem, tuple, index))
            }
            ValueType::BitArray => Self::BitArray(BitArrayListExpr::tuple_index(
                BitArrayListItem,
                tuple,
                index,
            )),
            ValueType::UtfCodepoint => Self::UtfCodepoint(UtfCodepointListExpr::tuple_index(
                UtfCodepointListItem,
                tuple,
                index,
            )),
            ValueType::Custom(item_type) => Self::Custom(CustomListExpr::tuple_index(
                CustomListItem { item_type },
                tuple,
                index,
            )),
            ValueType::Float => {
                Self::Float(FloatListExpr::tuple_index(FloatListItem, tuple, index))
            }
            ValueType::Bool => Self::Bool(BoolListExpr::tuple_index(BoolListItem, tuple, index)),
            ValueType::Nil => Self::Nil(NilListExpr::tuple_index(NilListItem, tuple, index)),
            ValueType::Tuple(item_type) => {
                let item = TupleListItem { item_type };
                Self::Tuple(TupleListExpr::tuple_index(item, tuple, index))
            }
            ValueType::List(item_type) => {
                let item = ListListItem { item_type };
                Self::List(ListListExpr::tuple_index(item, tuple, index))
            }
            ValueType::Function(item_type) => {
                let item = FunctionListItem {
                    item_type: *item_type,
                };
                Self::Function(FunctionListExpr::tuple_index(item, tuple, index))
            }
        }
    }

    pub(crate) fn custom_field(access: CustomFieldAccess, element_type: ValueType) -> Self {
        match element_type {
            ValueType::Int => Self::Int(IntListExpr::custom_field(IntListItem, access)),
            ValueType::String => Self::String(StringListExpr::custom_field(StringListItem, access)),
            ValueType::BitArray => {
                Self::BitArray(BitArrayListExpr::custom_field(BitArrayListItem, access))
            }
            ValueType::UtfCodepoint => Self::UtfCodepoint(UtfCodepointListExpr::custom_field(
                UtfCodepointListItem,
                access,
            )),
            ValueType::Custom(item_type) => Self::Custom(CustomListExpr::custom_field(
                CustomListItem { item_type },
                access,
            )),
            ValueType::Float => Self::Float(FloatListExpr::custom_field(FloatListItem, access)),
            ValueType::Bool => Self::Bool(BoolListExpr::custom_field(BoolListItem, access)),
            ValueType::Nil => Self::Nil(NilListExpr::custom_field(NilListItem, access)),
            ValueType::Tuple(item_type) => Self::Tuple(TupleListExpr::custom_field(
                TupleListItem { item_type },
                access,
            )),
            ValueType::List(item_type) => Self::List(ListListExpr::custom_field(
                ListListItem { item_type },
                access,
            )),
            ValueType::Function(item_type) => Self::Function(FunctionListExpr::custom_field(
                FunctionListItem {
                    item_type: *item_type,
                },
                access,
            )),
        }
    }

    pub(crate) fn list_index(list: ListListExpr, index: usize) -> Self {
        let element_type = list.item().item_type().as_ref().clone();
        match element_type {
            ValueType::Int => Self::Int(IntListExpr::from_list_index(
                IntListItem,
                ListIndexSource::new(list, index),
            )),
            ValueType::String => Self::String(StringListExpr::from_list_index(
                StringListItem,
                ListIndexSource::new(list, index),
            )),
            ValueType::BitArray => Self::BitArray(BitArrayListExpr::from_list_index(
                BitArrayListItem,
                ListIndexSource::new(list, index),
            )),
            ValueType::UtfCodepoint => Self::UtfCodepoint(UtfCodepointListExpr::from_list_index(
                UtfCodepointListItem,
                ListIndexSource::new(list, index),
            )),
            ValueType::Custom(item_type) => {
                let item = CustomListItem { item_type };
                Self::Custom(CustomListExpr::from_list_index(
                    item,
                    ListIndexSource::new(list, index),
                ))
            }
            ValueType::Float => Self::Float(FloatListExpr::from_list_index(
                FloatListItem,
                ListIndexSource::new(list, index),
            )),
            ValueType::Bool => Self::Bool(BoolListExpr::from_list_index(
                BoolListItem,
                ListIndexSource::new(list, index),
            )),
            ValueType::Nil => Self::Nil(NilListExpr::from_list_index(
                NilListItem,
                ListIndexSource::new(list, index),
            )),
            ValueType::Tuple(item_type) => {
                let item = TupleListItem { item_type };
                Self::Tuple(TupleListExpr::from_list_index(
                    item,
                    ListIndexSource::new(list, index),
                ))
            }
            ValueType::List(item_type) => {
                let item = ListListItem { item_type };
                Self::List(ListListExpr::from_list_index(
                    item,
                    ListIndexSource::new(list, index),
                ))
            }
            ValueType::Function(item_type) => {
                let item = FunctionListItem {
                    item_type: *item_type,
                };
                Self::Function(FunctionListExpr::from_list_index(
                    item,
                    ListIndexSource::new(list, index),
                ))
            }
        }
    }

    pub(crate) fn drop_first(list: ListExpr, count: usize) -> Self {
        match list {
            Self::Int(list) => Self::Int(IntListExpr::drop_first(list, count)),
            Self::String(list) => Self::String(StringListExpr::drop_first(list, count)),
            Self::BitArray(list) => Self::BitArray(BitArrayListExpr::drop_first(list, count)),
            Self::UtfCodepoint(list) => {
                Self::UtfCodepoint(UtfCodepointListExpr::drop_first(list, count))
            }
            Self::Custom(list) => Self::Custom(CustomListExpr::drop_first(list, count)),
            Self::Float(list) => Self::Float(FloatListExpr::drop_first(list, count)),
            Self::Bool(list) => Self::Bool(BoolListExpr::drop_first(list, count)),
            Self::Nil(list) => Self::Nil(NilListExpr::drop_first(list, count)),
            Self::Tuple(list) => Self::Tuple(TupleListExpr::drop_first(list, count)),
            Self::List(list) => Self::List(ListListExpr::drop_first(list, count)),
            Self::Function(list) => Self::Function(FunctionListExpr::drop_first(list, count)),
        }
    }

    pub(crate) fn panic(panic: PanicExpr, element_type: ValueType) -> Self {
        match element_type {
            ValueType::Int => Self::Int(IntListExpr::panic(IntListItem, panic)),
            ValueType::String => Self::String(StringListExpr::panic(StringListItem, panic)),
            ValueType::BitArray => Self::BitArray(BitArrayListExpr::panic(BitArrayListItem, panic)),
            ValueType::UtfCodepoint => {
                Self::UtfCodepoint(UtfCodepointListExpr::panic(UtfCodepointListItem, panic))
            }
            ValueType::Custom(item_type) => {
                Self::Custom(CustomListExpr::panic(CustomListItem { item_type }, panic))
            }
            ValueType::Float => Self::Float(FloatListExpr::panic(FloatListItem, panic)),
            ValueType::Bool => Self::Bool(BoolListExpr::panic(BoolListItem, panic)),
            ValueType::Nil => Self::Nil(NilListExpr::panic(NilListItem, panic)),
            ValueType::Tuple(item_type) => {
                Self::Tuple(TupleListExpr::panic(TupleListItem { item_type }, panic))
            }
            ValueType::List(item_type) => {
                Self::List(ListListExpr::panic(ListListItem { item_type }, panic))
            }
            ValueType::Function(item_type) => Self::Function(FunctionListExpr::panic(
                FunctionListItem {
                    item_type: *item_type,
                },
                panic,
            )),
        }
    }

    pub(crate) fn bool_case(subject: BoolExpr, branches: BoolListCaseBranches) -> Self {
        match branches {
            BoolListCaseBranches::Int { true_, false_ } => {
                Self::Int(IntListExpr::bool_case(subject, true_, false_))
            }
            BoolListCaseBranches::String { true_, false_ } => {
                Self::String(StringListExpr::bool_case(subject, true_, false_))
            }
            BoolListCaseBranches::BitArray { true_, false_ } => {
                Self::BitArray(BitArrayListExpr::bool_case(subject, true_, false_))
            }
            BoolListCaseBranches::UtfCodepoint { true_, false_ } => {
                Self::UtfCodepoint(UtfCodepointListExpr::bool_case(subject, true_, false_))
            }
            BoolListCaseBranches::Custom { true_, false_ } => {
                Self::Custom(CustomListExpr::bool_case(subject, true_, false_))
            }
            BoolListCaseBranches::Float { true_, false_ } => {
                Self::Float(FloatListExpr::bool_case(subject, true_, false_))
            }
            BoolListCaseBranches::Bool { true_, false_ } => {
                Self::Bool(BoolListExpr::bool_case(subject, true_, false_))
            }
            BoolListCaseBranches::Nil { true_, false_ } => {
                Self::Nil(NilListExpr::bool_case(subject, true_, false_))
            }
            BoolListCaseBranches::Tuple { true_, false_ } => {
                Self::Tuple(TupleListExpr::bool_case(subject, true_, false_))
            }
            BoolListCaseBranches::List { true_, false_ } => {
                Self::List(ListListExpr::bool_case(subject, true_, false_))
            }
            BoolListCaseBranches::Function { true_, false_ } => {
                Self::Function(FunctionListExpr::bool_case(subject, true_, false_))
            }
        }
    }

    pub(crate) fn int_case(subject: IntExpr, branches: ListCaseBranches<BigInt>) -> Self {
        match branches {
            ListCaseBranches::Int { clauses, fallback } => {
                Self::Int(IntListExpr::int_case(subject, clauses, fallback))
            }
            ListCaseBranches::String { clauses, fallback } => {
                Self::String(StringListExpr::int_case(subject, clauses, fallback))
            }
            ListCaseBranches::BitArray { clauses, fallback } => {
                Self::BitArray(BitArrayListExpr::int_case(subject, clauses, fallback))
            }
            ListCaseBranches::UtfCodepoint { clauses, fallback } => {
                Self::UtfCodepoint(UtfCodepointListExpr::int_case(subject, clauses, fallback))
            }
            ListCaseBranches::Custom { clauses, fallback } => {
                Self::Custom(CustomListExpr::int_case(subject, clauses, fallback))
            }
            ListCaseBranches::Float { clauses, fallback } => {
                Self::Float(FloatListExpr::int_case(subject, clauses, fallback))
            }
            ListCaseBranches::Bool { clauses, fallback } => {
                Self::Bool(BoolListExpr::int_case(subject, clauses, fallback))
            }
            ListCaseBranches::Nil { clauses, fallback } => {
                Self::Nil(NilListExpr::int_case(subject, clauses, fallback))
            }
            ListCaseBranches::Tuple { clauses, fallback } => {
                Self::Tuple(TupleListExpr::int_case(subject, clauses, fallback))
            }
            ListCaseBranches::List { clauses, fallback } => {
                Self::List(ListListExpr::int_case(subject, clauses, fallback))
            }
            ListCaseBranches::Function { clauses, fallback } => {
                Self::Function(FunctionListExpr::int_case(subject, clauses, fallback))
            }
        }
    }

    pub(crate) fn string_case(subject: StringExpr, branches: ListCaseBranches<EcoString>) -> Self {
        match branches {
            ListCaseBranches::Int { clauses, fallback } => {
                Self::Int(IntListExpr::string_case(subject, clauses, fallback))
            }
            ListCaseBranches::String { clauses, fallback } => {
                Self::String(StringListExpr::string_case(subject, clauses, fallback))
            }
            ListCaseBranches::BitArray { clauses, fallback } => {
                Self::BitArray(BitArrayListExpr::string_case(subject, clauses, fallback))
            }
            ListCaseBranches::UtfCodepoint { clauses, fallback } => Self::UtfCodepoint(
                UtfCodepointListExpr::string_case(subject, clauses, fallback),
            ),
            ListCaseBranches::Custom { clauses, fallback } => {
                Self::Custom(CustomListExpr::string_case(subject, clauses, fallback))
            }
            ListCaseBranches::Float { clauses, fallback } => {
                Self::Float(FloatListExpr::string_case(subject, clauses, fallback))
            }
            ListCaseBranches::Bool { clauses, fallback } => {
                Self::Bool(BoolListExpr::string_case(subject, clauses, fallback))
            }
            ListCaseBranches::Nil { clauses, fallback } => {
                Self::Nil(NilListExpr::string_case(subject, clauses, fallback))
            }
            ListCaseBranches::Tuple { clauses, fallback } => {
                Self::Tuple(TupleListExpr::string_case(subject, clauses, fallback))
            }
            ListCaseBranches::List { clauses, fallback } => {
                Self::List(ListListExpr::string_case(subject, clauses, fallback))
            }
            ListCaseBranches::Function { clauses, fallback } => {
                Self::Function(FunctionListExpr::string_case(subject, clauses, fallback))
            }
        }
    }

    pub(crate) fn float_case(subject: FloatExpr, branches: ListCaseBranches<f64>) -> Self {
        match branches {
            ListCaseBranches::Int { clauses, fallback } => {
                Self::Int(IntListExpr::float_case(subject, clauses, fallback))
            }
            ListCaseBranches::String { clauses, fallback } => {
                Self::String(StringListExpr::float_case(subject, clauses, fallback))
            }
            ListCaseBranches::BitArray { clauses, fallback } => {
                Self::BitArray(BitArrayListExpr::float_case(subject, clauses, fallback))
            }
            ListCaseBranches::UtfCodepoint { clauses, fallback } => {
                Self::UtfCodepoint(UtfCodepointListExpr::float_case(subject, clauses, fallback))
            }
            ListCaseBranches::Custom { clauses, fallback } => {
                Self::Custom(CustomListExpr::float_case(subject, clauses, fallback))
            }
            ListCaseBranches::Float { clauses, fallback } => {
                Self::Float(FloatListExpr::float_case(subject, clauses, fallback))
            }
            ListCaseBranches::Bool { clauses, fallback } => {
                Self::Bool(BoolListExpr::float_case(subject, clauses, fallback))
            }
            ListCaseBranches::Nil { clauses, fallback } => {
                Self::Nil(NilListExpr::float_case(subject, clauses, fallback))
            }
            ListCaseBranches::Tuple { clauses, fallback } => {
                Self::Tuple(TupleListExpr::float_case(subject, clauses, fallback))
            }
            ListCaseBranches::List { clauses, fallback } => {
                Self::List(ListListExpr::float_case(subject, clauses, fallback))
            }
            ListCaseBranches::Function { clauses, fallback } => {
                Self::Function(FunctionListExpr::float_case(subject, clauses, fallback))
            }
        }
    }

    pub(crate) fn block(steps: Vec<Step>, return_: ListExpr) -> Self {
        match return_ {
            Self::Int(return_) => Self::Int(IntListExpr::block(steps, return_)),
            Self::String(return_) => Self::String(StringListExpr::block(steps, return_)),
            Self::BitArray(return_) => Self::BitArray(BitArrayListExpr::block(steps, return_)),
            Self::UtfCodepoint(return_) => {
                Self::UtfCodepoint(UtfCodepointListExpr::block(steps, return_))
            }
            Self::Custom(return_) => Self::Custom(CustomListExpr::block(steps, return_)),
            Self::Float(return_) => Self::Float(FloatListExpr::block(steps, return_)),
            Self::Bool(return_) => Self::Bool(BoolListExpr::block(steps, return_)),
            Self::Nil(return_) => Self::Nil(NilListExpr::block(steps, return_)),
            Self::Tuple(return_) => Self::Tuple(TupleListExpr::block(steps, return_)),
            Self::List(return_) => Self::List(ListListExpr::block(steps, return_)),
            Self::Function(return_) => Self::Function(FunctionListExpr::block(steps, return_)),
        }
    }

    pub fn element_type(&self) -> ValueType {
        match self {
            Self::Int(expression) => expression.element_type(),
            Self::String(expression) => expression.element_type(),
            Self::BitArray(expression) => expression.element_type(),
            Self::UtfCodepoint(expression) => expression.element_type(),
            Self::Custom(expression) => expression.element_type(),
            Self::Float(expression) => expression.element_type(),
            Self::Bool(expression) => expression.element_type(),
            Self::Nil(expression) => expression.element_type(),
            Self::Tuple(expression) => expression.element_type(),
            Self::List(expression) => expression.element_type(),
            Self::Function(expression) => expression.element_type(),
        }
    }

    pub(crate) fn item_shape(&self) -> &crate::plan::ValueShape {
        match self {
            Self::Int(expression) => expression.item_shape(),
            Self::String(expression) => expression.item_shape(),
            Self::BitArray(expression) => expression.item_shape(),
            Self::UtfCodepoint(expression) => expression.item_shape(),
            Self::Custom(expression) => expression.item_shape(),
            Self::Float(expression) => expression.item_shape(),
            Self::Bool(expression) => expression.item_shape(),
            Self::Nil(expression) => expression.item_shape(),
            Self::Tuple(expression) => expression.item_shape(),
            Self::List(expression) => expression.item_shape(),
            Self::Function(expression) => expression.item_shape(),
        }
    }

    pub(crate) fn with_item_shape(self, item_shape: crate::plan::ValueShape) -> Self {
        match self {
            Self::Int(expression) => Self::Int(expression.with_item_shape(item_shape)),
            Self::String(expression) => Self::String(expression.with_item_shape(item_shape)),
            Self::BitArray(expression) => Self::BitArray(expression.with_item_shape(item_shape)),
            Self::UtfCodepoint(expression) => {
                Self::UtfCodepoint(expression.with_item_shape(item_shape))
            }
            Self::Custom(expression) => Self::Custom(expression.with_item_shape(item_shape)),
            Self::Float(expression) => Self::Float(expression.with_item_shape(item_shape)),
            Self::Bool(expression) => Self::Bool(expression.with_item_shape(item_shape)),
            Self::Nil(expression) => Self::Nil(expression.with_item_shape(item_shape)),
            Self::Tuple(expression) => Self::Tuple(expression.with_item_shape(item_shape)),
            Self::List(expression) => Self::List(expression.with_item_shape(item_shape)),
            Self::Function(expression) => Self::Function(expression.with_item_shape(item_shape)),
        }
    }

    pub(crate) fn into_int(self) -> Option<IntListExpr> {
        match self {
            Self::Int(expression) => Some(expression),
            _ => None,
        }
    }

    pub(crate) fn into_string(self) -> Option<StringListExpr> {
        match self {
            Self::String(expression) => Some(expression),
            _ => None,
        }
    }

    pub(crate) fn into_bit_array(self) -> Option<BitArrayListExpr> {
        match self {
            Self::BitArray(expression) => Some(expression),
            _ => None,
        }
    }

    pub(crate) fn into_utf_codepoint(self) -> Option<UtfCodepointListExpr> {
        match self {
            Self::UtfCodepoint(expression) => Some(expression),
            _ => None,
        }
    }

    pub(crate) fn into_custom(self) -> Option<CustomListExpr> {
        match self {
            Self::Custom(expression) => Some(expression),
            _ => None,
        }
    }

    pub(crate) fn into_float(self) -> Option<FloatListExpr> {
        match self {
            Self::Float(expression) => Some(expression),
            _ => None,
        }
    }

    pub(crate) fn into_bool(self) -> Option<BoolListExpr> {
        match self {
            Self::Bool(expression) => Some(expression),
            _ => None,
        }
    }

    pub(crate) fn into_nil(self) -> Option<NilListExpr> {
        match self {
            Self::Nil(expression) => Some(expression),
            _ => None,
        }
    }

    pub(crate) fn into_tuple(self) -> Option<TupleListExpr> {
        match self {
            Self::Tuple(expression) => Some(expression),
            _ => None,
        }
    }

    pub(crate) fn into_list(self) -> Option<ListListExpr> {
        match self {
            Self::List(expression) => Some(expression),
            _ => None,
        }
    }

    pub(crate) fn into_function(self) -> Option<FunctionListExpr> {
        match self {
            Self::Function(expression) => Some(expression),
            _ => None,
        }
    }
}

fn list_item_shape(
    elements: &[Expr],
    element_type: &ValueType,
) -> Result<crate::plan::ValueShape, ListElementTypeMismatch> {
    let Some((first, rest)) = elements.split_first() else {
        return Ok(crate::plan::ValueShape::from_value_type(
            element_type.clone(),
        ));
    };
    if first.value_type() != *element_type {
        return Err(ListElementTypeMismatch {
            expected: element_type.clone(),
            actual: first.value_type(),
        });
    }

    let mut shape = first.value_shape().clone();
    for element in rest {
        if element.value_type() != *element_type {
            return Err(ListElementTypeMismatch {
                expected: element_type.clone(),
                actual: element.value_type(),
            });
        }
        let Some(merged) = shape.merge(element.value_shape()) else {
            return Err(ListElementTypeMismatch {
                expected: element_type.clone(),
                actual: element.value_type(),
            });
        };
        shape = merged;
    }
    Ok(shape)
}

#[cfg(test)]
impl ListExpr {
    pub(crate) fn value(elements: Vec<Expr>, element_type: ValueType) -> Self {
        Self::try_value(elements, element_type)
            .expect("list expression elements must match declared item type")
    }

    pub(crate) fn spread(elements: Vec<Expr>, tail: ListExpr, element_type: ValueType) -> Self {
        let elements = ListElements::from_exprs(element_type, elements)
            .expect("list spread elements must match declared item type");
        Self::try_spread(elements, tail).expect("list spread tail must match prefix item type")
    }

    pub(crate) fn try_spread(
        elements: ListElements,
        tail: ListExpr,
    ) -> Result<Self, ListElementTypeMismatch> {
        ListSpreadElements::from_parts(elements, tail).map(Self::from_spread_elements)
    }
}

#[cfg(test)]
mod tests {
    use super::{
        BitArrayListExpr, BitArrayListItem, BoolListCaseBranches, BoolListExpr, BoolListItem,
        CustomListExpr, CustomListItem, FloatListExpr, FloatListItem, FunctionListExpr,
        FunctionListItem, IntListExpr, IntListItem, ListCaseBranches, ListElementTypeMismatch,
        ListElements, ListExpr, ListIndexSource, ListListExpr, ListListItem, NilListExpr,
        NilListItem, StringListExpr, StringListItem, TupleListExpr, TupleListItem,
        UtfCodepointListExpr, UtfCodepointListItem,
    };
    use crate::plan::{
        BitArrayExpr, BoolExpr, CustomConstructorRefinement, CustomExpr, CustomFieldAccess,
        CustomLocal, CustomLocalId, CustomType, CustomTypeName, CustomValueShape, Expr, FloatExpr,
        FunctionExpr, FunctionReference, FunctionShape, FunctionType, IntExpr, IntFunctionId,
        IntListFunctionId, IntListLocalId, ListFunctionExpr, ListFunctionId, ListFunctionReference,
        ListLocal, NilExpr, PanicExpr, PanicSite, RuntimeFunctionId, Step, StringExpr, TupleExpr,
        UtfCodepointExpr, UtfCodepointLocalId, ValueShape, ValueType,
    };
    use num_bigint::BigInt;

    #[test]
    fn value_constructor_preserves_typed_item_family() {
        assert_eq!(
            ListExpr::value(vec![Expr::int(IntExpr::value(1.into()))], ValueType::Int),
            ListExpr::Int(IntListExpr::value(
                IntListItem,
                vec![IntExpr::value(1.into())],
            )),
        );
        assert_eq!(
            ListExpr::value(
                vec![Expr::string(StringExpr::value("one".into()))],
                ValueType::String,
            ),
            ListExpr::String(StringListExpr::value(
                StringListItem,
                vec![StringExpr::value("one".into())],
            )),
        );
        assert_eq!(
            ListExpr::value(vec![Expr::float(FloatExpr::value(1.5))], ValueType::Float),
            ListExpr::Float(FloatListExpr::value(
                FloatListItem,
                vec![FloatExpr::value(1.5)],
            )),
        );
        assert_eq!(
            ListExpr::value(vec![Expr::bool(BoolExpr::value(true))], ValueType::Bool),
            ListExpr::Bool(BoolListExpr::value(
                BoolListItem,
                vec![BoolExpr::value(true)],
            )),
        );
        let codepoint = UtfCodepointExpr::local_get(UtfCodepointLocalId(0), "codepoint".into());
        assert_eq!(
            ListExpr::value(
                vec![Expr::utf_codepoint(codepoint.clone())],
                ValueType::UtfCodepoint,
            ),
            ListExpr::UtfCodepoint(UtfCodepointListExpr::value(
                UtfCodepointListItem,
                vec![codepoint],
            )),
        );
        assert_eq!(
            ListExpr::value(vec![Expr::nil(NilExpr::value())], ValueType::Nil),
            ListExpr::Nil(NilListExpr::value(NilListItem, vec![NilExpr::value()])),
        );

        let tuple = TupleExpr::value(
            vec![Expr::int(IntExpr::value(2.into()))],
            vec![ValueType::Int],
        );
        assert_eq!(
            ListExpr::value(
                vec![Expr::tuple(tuple.clone())],
                ValueType::Tuple(vec![ValueType::Int]),
            ),
            ListExpr::Tuple(TupleListExpr::value(
                TupleListItem {
                    item_type: vec![ValueType::Int],
                },
                vec![tuple],
            )),
        );

        let nested = ListExpr::value(vec![Expr::int(IntExpr::value(3.into()))], ValueType::Int);
        assert_eq!(
            ListExpr::value(
                vec![Expr::list(nested.clone())],
                ValueType::List(Box::new(ValueType::Int)),
            ),
            ListExpr::List(ListListExpr::value(
                ListListItem {
                    item_type: Box::new(ValueType::Int),
                },
                vec![nested],
            )),
        );

        let function_type = FunctionType::new(Vec::new(), ValueType::Int);
        let function = FunctionExpr::reference(FunctionReference::new(
            RuntimeFunctionId::Int(IntFunctionId(0)),
            Vec::new(),
        ));
        assert_eq!(
            ListExpr::value(
                vec![Expr::function(function.clone())],
                ValueType::Function(Box::new(function_type.clone())),
            ),
            ListExpr::Function(FunctionListExpr::value(
                FunctionListItem {
                    item_type: function_type,
                },
                vec![function],
            )),
        );
    }

    #[test]
    fn value_constructor_rejects_later_nominal_and_refinement_mismatches() {
        assert_eq!(
            ListExpr::try_value(
                vec![
                    Expr::int(IntExpr::value(1.into())),
                    Expr::string(StringExpr::value("wrong".into())),
                ],
                ValueType::Int,
            ),
            Err(ListElementTypeMismatch {
                expected: ValueType::Int,
                actual: ValueType::String,
            }),
        );

        let type_ = CustomType::new(
            CustomTypeName::new("geam".into(), "main".into(), "Choice".into()),
            Vec::new(),
        );
        let function_type =
            FunctionType::new(vec![ValueType::Custom(type_.clone())], ValueType::Int);
        let function = |id, constructor| {
            let custom_shape = CustomValueShape::new(
                type_.type_name().clone(),
                Vec::new(),
                CustomConstructorRefinement::Exact(constructor),
            );
            let shape = FunctionShape::new(
                vec![ValueShape::Custom(custom_shape.clone())],
                ValueShape::Int,
            );
            FunctionExpr::reference(FunctionReference::new(
                RuntimeFunctionId::Int(IntFunctionId(id)),
                vec![crate::plan::ParamLocal::Custom(CustomLocal::from_shape(
                    CustomLocalId(0),
                    custom_shape,
                ))],
            ))
            .with_resolved_shape(shape)
            .expect("function shape has the same nominal type")
        };

        assert_eq!(
            ListExpr::try_value(
                vec![
                    Expr::function(function(0, 0)),
                    Expr::function(function(1, 1)),
                ],
                ValueType::Function(Box::new(function_type.clone())),
            ),
            Err(ListElementTypeMismatch {
                expected: ValueType::Function(Box::new(function_type.clone())),
                actual: ValueType::Function(Box::new(function_type)),
            }),
        );
    }

    #[test]
    fn spread_constructor_preserves_typed_tail_family() {
        let int_tail = ListExpr::value(vec![Expr::int(IntExpr::value(2.into()))], ValueType::Int);
        assert_eq!(
            ListExpr::try_spread(
                ListElements::Int(vec![IntExpr::value(1.into())]),
                int_tail.clone(),
            ),
            Ok(ListExpr::Int(IntListExpr::spread(
                vec![IntExpr::value(1.into())],
                int_tail.into_int().expect("int list"),
            ))),
        );

        let tuple_tail = ListExpr::value(
            vec![Expr::tuple(TupleExpr::value(
                vec![Expr::int(IntExpr::value(2.into()))],
                vec![ValueType::Int],
            ))],
            ValueType::Tuple(vec![ValueType::Int]),
        );
        assert_eq!(
            ListExpr::try_spread(
                ListElements::Tuple {
                    item_type: vec![ValueType::Int],
                    values: vec![TupleExpr::value(
                        vec![Expr::int(IntExpr::value(1.into()))],
                        vec![ValueType::Int],
                    )],
                },
                tuple_tail.clone(),
            ),
            Ok(ListExpr::Tuple(TupleListExpr::spread(
                vec![TupleExpr::value(
                    vec![Expr::int(IntExpr::value(1.into()))],
                    vec![ValueType::Int],
                )],
                tuple_tail.into_tuple().expect("tuple list"),
            ))),
        );

        assert_eq!(
            ListExpr::try_spread(
                ListElements::Int(vec![IntExpr::value(1.into())]),
                ListExpr::value(
                    vec![Expr::string(StringExpr::value("wrong".into()))],
                    ValueType::String,
                ),
            ),
            Err(ListElementTypeMismatch {
                expected: ValueType::Int,
                actual: ValueType::String,
            }),
        );
    }

    #[test]
    fn facade_typed_projection_rejects_wrong_item_family() {
        assert_eq!(
            ListExpr::value(
                vec![Expr::string(StringExpr::value("one".into()))],
                ValueType::String,
            )
            .into_int(),
            None,
        );

        let int_list = ListExpr::value(vec![Expr::int(IntExpr::value(1.into()))], ValueType::Int);
        assert_eq!(int_list.clone().into_string(), None);
        assert_eq!(int_list.clone().into_bit_array(), None);
        assert_eq!(int_list.clone().into_utf_codepoint(), None);
        assert_eq!(int_list.clone().into_custom(), None);
        assert_eq!(int_list.clone().into_float(), None);
        assert_eq!(int_list.clone().into_bool(), None);
        assert_eq!(int_list.clone().into_nil(), None);
        assert_eq!(int_list.clone().into_tuple(), None);
        assert_eq!(int_list.clone().into_list(), None);
        assert_eq!(int_list.into_function(), None);
    }

    #[test]
    fn facade_constructors_dispatch_to_typed_shape() {
        assert_eq!(
            ListExpr::local_get(ListLocal::int(IntListLocalId(0)), "values".into()),
            ListExpr::Int(IntListExpr::local_get(
                IntListItem,
                IntListLocalId(0),
                "values".into(),
            )),
        );
        assert_eq!(
            ListExpr::call(ListFunctionId::Int(IntListFunctionId(0)), Vec::new()),
            ListExpr::Int(IntListExpr::call(
                IntListItem,
                IntListFunctionId(0),
                Vec::new(),
            )),
        );

        let list_function = ListFunctionExpr::reference(ListFunctionReference::new(
            ListFunctionId::from_item_type(0, ValueType::Int),
            Vec::new(),
        ));
        assert_eq!(
            ListExpr::function_call(list_function.clone(), Vec::new()),
            ListExpr::Int(IntListExpr::function_call(
                IntListItem,
                list_function,
                Vec::new(),
            )),
        );

        let tuple = TupleExpr::value(
            vec![Expr::list(ListExpr::value(
                vec![Expr::int(IntExpr::value(1.into()))],
                ValueType::Int,
            ))],
            vec![ValueType::List(Box::new(ValueType::Int))],
        );
        assert_eq!(
            ListExpr::tuple_index(tuple.clone(), 0, ValueType::List(Box::new(ValueType::Int))),
            ListExpr::List(ListListExpr::tuple_index(
                ListListItem {
                    item_type: Box::new(ValueType::Int),
                },
                tuple,
                0,
            )),
        );

        let nested = ListExpr::value(
            vec![Expr::list(ListExpr::value(
                vec![Expr::string(StringExpr::value("one".into()))],
                ValueType::String,
            ))],
            ValueType::List(Box::new(ValueType::String)),
        )
        .into_list()
        .expect("nested list");
        assert_eq!(
            ListExpr::list_index(nested.clone(), 0),
            ListExpr::String(StringListExpr::from_list_index(
                StringListItem,
                ListIndexSource::new(nested, 0),
            )),
        );
    }

    #[test]
    fn custom_field_constructor_dispatches_every_item_family() {
        let custom_type = CustomType::new(
            CustomTypeName::new("geam".into(), "main".into(), "Boxed".into()),
            Vec::new(),
        );
        let access = CustomFieldAccess::new(
            CustomExpr::local_get(
                crate::plan::CustomLocal::new(CustomLocalId(0), custom_type.clone()),
                "boxed".into(),
            ),
            0,
            Some("value".into()),
        );
        let function_type = FunctionType::new(Vec::new(), ValueType::Int);

        assert_eq!(
            ListExpr::custom_field(access.clone(), ValueType::Int),
            ListExpr::Int(IntListExpr::custom_field(IntListItem, access.clone())),
        );
        assert_eq!(
            ListExpr::custom_field(access.clone(), ValueType::String),
            ListExpr::String(StringListExpr::custom_field(StringListItem, access.clone(),)),
        );
        assert_eq!(
            ListExpr::custom_field(access.clone(), ValueType::BitArray),
            ListExpr::BitArray(BitArrayListExpr::custom_field(
                BitArrayListItem,
                access.clone(),
            )),
        );
        assert_eq!(
            ListExpr::custom_field(access.clone(), ValueType::UtfCodepoint),
            ListExpr::UtfCodepoint(UtfCodepointListExpr::custom_field(
                UtfCodepointListItem,
                access.clone(),
            )),
        );
        assert_eq!(
            ListExpr::custom_field(access.clone(), ValueType::Custom(custom_type.clone())),
            ListExpr::Custom(CustomListExpr::custom_field(
                CustomListItem {
                    item_type: custom_type,
                },
                access.clone(),
            )),
        );
        assert_eq!(
            ListExpr::custom_field(access.clone(), ValueType::Float),
            ListExpr::Float(FloatListExpr::custom_field(FloatListItem, access.clone(),)),
        );
        assert_eq!(
            ListExpr::custom_field(access.clone(), ValueType::Bool),
            ListExpr::Bool(BoolListExpr::custom_field(BoolListItem, access.clone())),
        );
        assert_eq!(
            ListExpr::custom_field(access.clone(), ValueType::Nil),
            ListExpr::Nil(NilListExpr::custom_field(NilListItem, access.clone())),
        );
        assert_eq!(
            ListExpr::custom_field(access.clone(), ValueType::Tuple(vec![ValueType::Int]),),
            ListExpr::Tuple(TupleListExpr::custom_field(
                TupleListItem {
                    item_type: vec![ValueType::Int],
                },
                access.clone(),
            )),
        );
        assert_eq!(
            ListExpr::custom_field(access.clone(), ValueType::List(Box::new(ValueType::String)),),
            ListExpr::List(ListListExpr::custom_field(
                ListListItem {
                    item_type: Box::new(ValueType::String),
                },
                access.clone(),
            )),
        );
        assert_eq!(
            ListExpr::custom_field(
                access.clone(),
                ValueType::Function(Box::new(function_type.clone())),
            ),
            ListExpr::Function(FunctionListExpr::custom_field(
                FunctionListItem {
                    item_type: function_type,
                },
                access,
            )),
        );
    }

    #[test]
    fn case_and_block_constructors_preserve_typed_family() {
        let subject = BoolExpr::value(true);
        let true_ = ListExpr::value(vec![Expr::int(IntExpr::value(1.into()))], ValueType::Int)
            .into_int()
            .expect("int list");
        let false_ = ListExpr::value(vec![Expr::int(IntExpr::value(2.into()))], ValueType::Int)
            .into_int()
            .expect("int list");

        assert_eq!(
            ListExpr::bool_case(
                subject.clone(),
                BoolListCaseBranches::Int {
                    true_: true_.clone(),
                    false_: false_.clone(),
                },
            ),
            ListExpr::Int(IntListExpr::bool_case(subject, true_, false_)),
        );

        let fallback = ListExpr::value(
            vec![Expr::tuple(TupleExpr::value(
                vec![Expr::int(IntExpr::value(3.into()))],
                vec![ValueType::Int],
            ))],
            ValueType::Tuple(vec![ValueType::Int]),
        );
        let branch = ListExpr::value(
            vec![Expr::tuple(TupleExpr::value(
                vec![Expr::int(IntExpr::value(4.into()))],
                vec![ValueType::Int],
            ))],
            ValueType::Tuple(vec![ValueType::Int]),
        );
        assert_eq!(
            ListExpr::int_case(
                IntExpr::value(1.into()),
                ListCaseBranches::from_exprs(
                    vec![(BigInt::from(1), branch.clone())],
                    fallback.clone()
                )
                .expect("list case branches"),
            ),
            ListExpr::Tuple(TupleListExpr::int_case(
                IntExpr::value(1.into()),
                vec![(BigInt::from(1), branch.into_tuple().expect("tuple list"))],
                fallback.into_tuple().expect("tuple list"),
            )),
        );

        let string_list = ListExpr::value(
            vec![Expr::string(StringExpr::value("done".into()))],
            ValueType::String,
        );
        assert_eq!(
            ListExpr::block(Vec::<Step>::new(), string_list.clone()),
            ListExpr::String(StringListExpr::block(
                Vec::<Step>::new(),
                string_list.into_string().expect("string list"),
            )),
        );
    }

    #[test]
    fn facade_dispatch_preserves_every_item_type() {
        let function_type = FunctionType::new(Vec::new(), ValueType::Int);
        let item_types = vec![
            ValueType::Int,
            ValueType::String,
            ValueType::BitArray,
            ValueType::UtfCodepoint,
            ValueType::Custom(crate::plan::CustomType::new(
                crate::plan::CustomTypeName::new("geam".into(), "main".into(), "Boxed".into()),
                Vec::new(),
            )),
            ValueType::Float,
            ValueType::Bool,
            ValueType::Nil,
            ValueType::Tuple(vec![ValueType::Int]),
            ValueType::List(Box::new(ValueType::String)),
            ValueType::Function(Box::new(function_type.clone())),
        ];

        for item_type in item_types {
            assert_eq!(
                ListExpr::call(
                    ListFunctionId::from_item_type(0, item_type.clone()),
                    Vec::new()
                )
                .element_type(),
                item_type,
            );
            assert_eq!(
                ListExpr::function_call(
                    ListFunctionExpr::reference(ListFunctionReference::new(
                        ListFunctionId::from_item_type(0, item_type.clone()),
                        Vec::new(),
                    )),
                    Vec::new(),
                )
                .element_type(),
                item_type,
            );
            assert_eq!(
                ListExpr::tuple_index(
                    TupleExpr::value(
                        vec![Expr::list(ListExpr::value(Vec::new(), item_type.clone()))],
                        vec![ValueType::List(Box::new(item_type.clone()))],
                    ),
                    0,
                    item_type.clone(),
                )
                .element_type(),
                item_type,
            );
            assert_eq!(
                ListExpr::drop_first(ListExpr::value(Vec::new(), item_type.clone()), 1)
                    .element_type(),
                item_type,
            );
            assert_eq!(
                ListExpr::panic(
                    PanicExpr::todo_at(None, PanicSite::unknown()),
                    item_type.clone()
                )
                .element_type(),
                item_type,
            );
            assert_eq!(
                ListExpr::int_case(
                    IntExpr::value(1.into()),
                    ListCaseBranches::from_exprs(
                        vec![(
                            BigInt::from(1),
                            ListExpr::value(Vec::new(), item_type.clone()),
                        )],
                        ListExpr::value(Vec::new(), item_type.clone()),
                    )
                    .expect("list case branches"),
                )
                .element_type(),
                item_type,
            );
            assert_eq!(
                ListExpr::string_case(
                    StringExpr::value("one".into()),
                    ListCaseBranches::from_exprs(
                        vec![("one".into(), ListExpr::value(Vec::new(), item_type.clone()))],
                        ListExpr::value(Vec::new(), item_type.clone()),
                    )
                    .expect("list case branches"),
                )
                .element_type(),
                item_type,
            );
            assert_eq!(
                ListExpr::float_case(
                    FloatExpr::value(1.5),
                    ListCaseBranches::from_exprs(
                        vec![(1.5, ListExpr::value(Vec::new(), item_type.clone()))],
                        ListExpr::value(Vec::new(), item_type.clone()),
                    )
                    .expect("list case branches"),
                )
                .element_type(),
                item_type,
            );
            assert_eq!(
                ListExpr::block(
                    Vec::<Step>::new(),
                    ListExpr::value(Vec::new(), item_type.clone())
                )
                .element_type(),
                item_type,
            );
        }
    }

    #[test]
    fn nested_list_index_dispatch_preserves_every_item_type() {
        let function_type = FunctionType::new(Vec::new(), ValueType::Int);
        let custom_type = crate::plan::CustomType::new(
            crate::plan::CustomTypeName::new("geam".into(), "main".into(), "Boxed".into()),
            Vec::new(),
        );
        let item_types = vec![
            ValueType::Int,
            ValueType::String,
            ValueType::BitArray,
            ValueType::UtfCodepoint,
            ValueType::Custom(custom_type),
            ValueType::Float,
            ValueType::Bool,
            ValueType::Nil,
            ValueType::Tuple(vec![ValueType::Int]),
            ValueType::List(Box::new(ValueType::String)),
            ValueType::Function(Box::new(function_type.clone())),
        ];

        for item_type in item_types {
            let list = ListExpr::value(
                vec![Expr::list(ListExpr::value(Vec::new(), item_type.clone()))],
                ValueType::List(Box::new(item_type.clone())),
            )
            .into_list()
            .expect("list item list");
            let expected = match item_type.clone() {
                ValueType::Int => ListExpr::Int(IntListExpr::from_list_index(
                    IntListItem,
                    ListIndexSource::new(list.clone(), 3),
                )),
                ValueType::String => ListExpr::String(StringListExpr::from_list_index(
                    StringListItem,
                    ListIndexSource::new(list.clone(), 3),
                )),
                ValueType::BitArray => ListExpr::BitArray(BitArrayListExpr::from_list_index(
                    BitArrayListItem,
                    ListIndexSource::new(list.clone(), 3),
                )),
                ValueType::UtfCodepoint => {
                    ListExpr::UtfCodepoint(UtfCodepointListExpr::from_list_index(
                        UtfCodepointListItem,
                        ListIndexSource::new(list.clone(), 3),
                    ))
                }
                ValueType::Custom(item_type) => ListExpr::Custom(CustomListExpr::from_list_index(
                    CustomListItem { item_type },
                    ListIndexSource::new(list.clone(), 3),
                )),
                ValueType::Float => ListExpr::Float(FloatListExpr::from_list_index(
                    FloatListItem,
                    ListIndexSource::new(list.clone(), 3),
                )),
                ValueType::Bool => ListExpr::Bool(BoolListExpr::from_list_index(
                    BoolListItem,
                    ListIndexSource::new(list.clone(), 3),
                )),
                ValueType::Nil => ListExpr::Nil(NilListExpr::from_list_index(
                    NilListItem,
                    ListIndexSource::new(list.clone(), 3),
                )),
                ValueType::Tuple(item_type) => ListExpr::Tuple(TupleListExpr::from_list_index(
                    TupleListItem { item_type },
                    ListIndexSource::new(list.clone(), 3),
                )),
                ValueType::List(item_type) => ListExpr::List(ListListExpr::from_list_index(
                    ListListItem { item_type },
                    ListIndexSource::new(list.clone(), 3),
                )),
                ValueType::Function(item_type) => {
                    ListExpr::Function(FunctionListExpr::from_list_index(
                        FunctionListItem {
                            item_type: *item_type,
                        },
                        ListIndexSource::new(list.clone(), 3),
                    ))
                }
            };

            assert_eq!(ListExpr::list_index(list, 3), expected);
            assert_eq!(expected.element_type(), item_type);
        }
    }

    #[test]
    fn spread_dispatch_preserves_remaining_item_families() {
        assert_eq!(
            ListExpr::spread(
                vec![Expr::string(StringExpr::value("head".into()))],
                ListExpr::value(Vec::new(), ValueType::String),
                ValueType::String,
            )
            .element_type(),
            ValueType::String,
        );
        assert_eq!(
            ListExpr::spread(
                vec![Expr::bit_array(BitArrayExpr::value(Vec::new()))],
                ListExpr::value(Vec::new(), ValueType::BitArray),
                ValueType::BitArray,
            )
            .element_type(),
            ValueType::BitArray,
        );
        assert_eq!(
            ListExpr::spread(
                vec![Expr::utf_codepoint(UtfCodepointExpr::local_get(
                    UtfCodepointLocalId(0),
                    "codepoint".into(),
                ))],
                ListExpr::value(Vec::new(), ValueType::UtfCodepoint),
                ValueType::UtfCodepoint,
            )
            .element_type(),
            ValueType::UtfCodepoint,
        );
        assert_eq!(
            ListExpr::spread(
                vec![Expr::float(FloatExpr::value(1.5))],
                ListExpr::value(Vec::new(), ValueType::Float),
                ValueType::Float,
            )
            .element_type(),
            ValueType::Float,
        );
        assert_eq!(
            ListExpr::spread(
                vec![Expr::bool(BoolExpr::value(true))],
                ListExpr::value(Vec::new(), ValueType::Bool),
                ValueType::Bool,
            )
            .element_type(),
            ValueType::Bool,
        );
        assert_eq!(
            ListExpr::spread(
                vec![Expr::nil(NilExpr::value())],
                ListExpr::value(Vec::new(), ValueType::Nil),
                ValueType::Nil,
            )
            .element_type(),
            ValueType::Nil,
        );

        let function_type = FunctionType::new(Vec::new(), ValueType::Int);
        assert_eq!(
            ListExpr::spread(
                vec![Expr::list(ListExpr::value(Vec::new(), ValueType::String))],
                ListExpr::value(Vec::new(), ValueType::List(Box::new(ValueType::String))),
                ValueType::List(Box::new(ValueType::String)),
            )
            .element_type(),
            ValueType::List(Box::new(ValueType::String)),
        );
        assert_eq!(
            ListExpr::spread(
                vec![Expr::function(FunctionExpr::reference(
                    FunctionReference::new(RuntimeFunctionId::Int(IntFunctionId(0)), Vec::new(),)
                ))],
                ListExpr::value(
                    Vec::new(),
                    ValueType::Function(Box::new(function_type.clone()))
                ),
                ValueType::Function(Box::new(function_type.clone())),
            )
            .element_type(),
            ValueType::Function(Box::new(function_type)),
        );
    }
}
