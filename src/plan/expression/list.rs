mod case;
mod elements;
mod item;
mod local;
mod typed;

pub(crate) use self::{
    case::{BoolListCaseBranches, ListCaseBranches},
    elements::{ListElementTypeMismatch, ListElements, ListSpreadElements},
    item::{
        BoolListItem, FloatListItem, FunctionListItem, IntListItem, ListItem, ListListItem,
        NilListItem, StringListItem, TupleListItem,
    },
    local::ListLocalExpr,
    typed::{TypedListExpr, TypedListExprKind},
};
use super::{
    BoolExpr, CallArg, Expr, FloatExpr, FunctionExpr, IntExpr, ListFunctionExpr, PanicExpr,
    StringExpr, TupleExpr,
};
use crate::plan::value::ListValueKind;
use crate::plan::{ListFunctionId, ListLocal, ListValue, Step, ValueType};
use ecow::EcoString;
use num_bigint::BigInt;

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum ListExpr {
    Int(IntListExpr),
    String(StringListExpr),
    Float(FloatListExpr),
    Bool(BoolListExpr),
    Nil(NilListExpr),
    Tuple(TupleListExpr),
    List(ListListExpr),
    Function(FunctionListExpr),
}

pub(crate) type IntListExpr = TypedListExpr<IntListItem>;
pub(crate) type StringListExpr = TypedListExpr<StringListItem>;
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
        ListElements::from_exprs(element_type, elements).map(Self::from_elements)
    }

    pub(crate) fn from_elements(elements: ListElements) -> Self {
        match elements {
            ListElements::Int(values) => Self::Int(IntListExpr::value(IntListItem, values)),
            ListElements::String(values) => {
                Self::String(StringListExpr::value(StringListItem, values))
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

    pub(crate) fn from_value(value: ListValue) -> Self {
        match value.into_kind() {
            ListValueKind::Int(values) => Self::Int(IntListExpr::value(
                IntListItem,
                values.into_iter().map(IntExpr::value).collect(),
            )),
            ListValueKind::String(values) => Self::String(StringListExpr::value(
                StringListItem,
                values.into_iter().map(StringExpr::value).collect(),
            )),
            ListValueKind::Float(values) => Self::Float(FloatListExpr::value(
                FloatListItem,
                values.into_iter().map(FloatExpr::value).collect(),
            )),
            ListValueKind::Bool(values) => Self::Bool(BoolListExpr::value(
                BoolListItem,
                values.into_iter().map(BoolExpr::value).collect(),
            )),
            ListValueKind::Nil(len) => Self::Nil(NilListExpr::value(
                NilListItem,
                vec![crate::plan::NilExpr::value(); len],
            )),
            ListValueKind::Tuple { item_type, values } => {
                let tuples = values
                    .into_iter()
                    .map(|values| {
                        TupleExpr::value(
                            values.into_iter().map(Expr::from).collect(),
                            item_type.clone(),
                        )
                    })
                    .collect();
                Self::Tuple(TupleListExpr::value(TupleListItem { item_type }, tuples))
            }
            ListValueKind::List { item_type, values } => {
                let values = values.into_iter().map(Self::from_value).collect();
                Self::List(ListListExpr::value(ListListItem { item_type }, values))
            }
            ListValueKind::Function { item_type, values } => {
                let values = values.into_iter().map(FunctionExpr::value).collect();
                Self::Function(FunctionListExpr::value(
                    FunctionListItem { item_type },
                    values,
                ))
            }
        }
    }

    pub(crate) fn from_spread_elements(elements: ListSpreadElements) -> Self {
        match elements {
            ListSpreadElements::Int { values, tail } => {
                Self::Int(IntListExpr::spread(IntListItem, values, tail))
            }
            ListSpreadElements::String { values, tail } => {
                Self::String(StringListExpr::spread(StringListItem, values, tail))
            }
            ListSpreadElements::Float { values, tail } => {
                Self::Float(FloatListExpr::spread(FloatListItem, values, tail))
            }
            ListSpreadElements::Bool { values, tail } => {
                Self::Bool(BoolListExpr::spread(BoolListItem, values, tail))
            }
            ListSpreadElements::Nil { values, tail } => {
                Self::Nil(NilListExpr::spread(NilListItem, values, tail))
            }
            ListSpreadElements::Tuple {
                item_type,
                values,
                tail,
            } => {
                let item = TupleListItem { item_type };
                Self::Tuple(TupleListExpr::spread(item, values, tail))
            }
            ListSpreadElements::List {
                item_type,
                values,
                tail,
            } => {
                let item = ListListItem { item_type };
                Self::List(ListListExpr::spread(item, values, tail))
            }
            ListSpreadElements::Function {
                item_type,
                values,
                tail,
            } => {
                let item = FunctionListItem { item_type };
                Self::Function(FunctionListExpr::spread(item, values, tail))
            }
        }
    }

    pub(crate) fn local_get(local: ListLocal, name: EcoString) -> Self {
        match local {
            ListLocal::Int(local) => Self::Int(IntListExpr::local_get(IntListItem, local, name)),
            ListLocal::String(local) => {
                Self::String(StringListExpr::local_get(StringListItem, local, name))
            }
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

    pub(crate) fn list_index(list: ListListExpr, index: usize) -> Self {
        let element_type = list.item().item_type().as_ref().clone();
        match element_type {
            ValueType::Int => Self::Int(IntListExpr::list_index(IntListItem, list, index)),
            ValueType::String => {
                Self::String(StringListExpr::list_index(StringListItem, list, index))
            }
            ValueType::Float => Self::Float(FloatListExpr::list_index(FloatListItem, list, index)),
            ValueType::Bool => Self::Bool(BoolListExpr::list_index(BoolListItem, list, index)),
            ValueType::Nil => Self::Nil(NilListExpr::list_index(NilListItem, list, index)),
            ValueType::Tuple(item_type) => {
                let item = TupleListItem { item_type };
                Self::Tuple(TupleListExpr::list_index(item, list, index))
            }
            ValueType::List(item_type) => {
                let item = ListListItem { item_type };
                Self::List(ListListExpr::list_index(item, list, index))
            }
            ValueType::Function(item_type) => {
                let item = FunctionListItem {
                    item_type: *item_type,
                };
                Self::Function(FunctionListExpr::list_index(item, list, index))
            }
        }
    }

    pub(crate) fn drop_first(list: ListExpr, count: usize) -> Self {
        match list {
            Self::Int(list) => Self::Int(IntListExpr::drop_first(IntListItem, list, count)),
            Self::String(list) => {
                Self::String(StringListExpr::drop_first(StringListItem, list, count))
            }
            Self::Float(list) => Self::Float(FloatListExpr::drop_first(FloatListItem, list, count)),
            Self::Bool(list) => Self::Bool(BoolListExpr::drop_first(BoolListItem, list, count)),
            Self::Nil(list) => Self::Nil(NilListExpr::drop_first(NilListItem, list, count)),
            Self::Tuple(list) => {
                let item = list.item.clone();
                Self::Tuple(TupleListExpr::drop_first(item, list, count))
            }
            Self::List(list) => {
                let item = list.item.clone();
                Self::List(ListListExpr::drop_first(item, list, count))
            }
            Self::Function(list) => {
                let item = list.item.clone();
                Self::Function(FunctionListExpr::drop_first(item, list, count))
            }
        }
    }

    pub(crate) fn panic(panic: PanicExpr, element_type: ValueType) -> Self {
        match element_type {
            ValueType::Int => Self::Int(IntListExpr::panic(IntListItem, panic)),
            ValueType::String => Self::String(StringListExpr::panic(StringListItem, panic)),
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
                Self::Int(IntListExpr::bool_case(IntListItem, subject, true_, false_))
            }
            BoolListCaseBranches::String { true_, false_ } => Self::String(
                StringListExpr::bool_case(StringListItem, subject, true_, false_),
            ),
            BoolListCaseBranches::Float { true_, false_ } => Self::Float(FloatListExpr::bool_case(
                FloatListItem,
                subject,
                true_,
                false_,
            )),
            BoolListCaseBranches::Bool { true_, false_ } => Self::Bool(BoolListExpr::bool_case(
                BoolListItem,
                subject,
                true_,
                false_,
            )),
            BoolListCaseBranches::Nil { true_, false_ } => {
                Self::Nil(NilListExpr::bool_case(NilListItem, subject, true_, false_))
            }
            BoolListCaseBranches::Tuple { true_, false_ } => {
                let item = true_.item.clone();
                Self::Tuple(TupleListExpr::bool_case(item, subject, true_, false_))
            }
            BoolListCaseBranches::List { true_, false_ } => {
                let item = true_.item.clone();
                Self::List(ListListExpr::bool_case(item, subject, true_, false_))
            }
            BoolListCaseBranches::Function { true_, false_ } => {
                let item = true_.item.clone();
                Self::Function(FunctionListExpr::bool_case(item, subject, true_, false_))
            }
        }
    }

    pub(crate) fn int_case(subject: IntExpr, branches: ListCaseBranches<BigInt>) -> Self {
        match branches {
            ListCaseBranches::Int { clauses, fallback } => Self::Int(IntListExpr::int_case(
                IntListItem,
                subject,
                clauses,
                fallback,
            )),
            ListCaseBranches::String { clauses, fallback } => Self::String(
                StringListExpr::int_case(StringListItem, subject, clauses, fallback),
            ),
            ListCaseBranches::Float { clauses, fallback } => Self::Float(FloatListExpr::int_case(
                FloatListItem,
                subject,
                clauses,
                fallback,
            )),
            ListCaseBranches::Bool { clauses, fallback } => Self::Bool(BoolListExpr::int_case(
                BoolListItem,
                subject,
                clauses,
                fallback,
            )),
            ListCaseBranches::Nil { clauses, fallback } => Self::Nil(NilListExpr::int_case(
                NilListItem,
                subject,
                clauses,
                fallback,
            )),
            ListCaseBranches::Tuple { clauses, fallback } => {
                let item = fallback.item.clone();
                Self::Tuple(TupleListExpr::int_case(item, subject, clauses, fallback))
            }
            ListCaseBranches::List { clauses, fallback } => {
                let item = fallback.item.clone();
                Self::List(ListListExpr::int_case(item, subject, clauses, fallback))
            }
            ListCaseBranches::Function { clauses, fallback } => {
                let item = fallback.item.clone();
                Self::Function(FunctionListExpr::int_case(item, subject, clauses, fallback))
            }
        }
    }

    pub(crate) fn string_case(subject: StringExpr, branches: ListCaseBranches<EcoString>) -> Self {
        match branches {
            ListCaseBranches::Int { clauses, fallback } => Self::Int(IntListExpr::string_case(
                IntListItem,
                subject,
                clauses,
                fallback,
            )),
            ListCaseBranches::String { clauses, fallback } => Self::String(
                StringListExpr::string_case(StringListItem, subject, clauses, fallback),
            ),
            ListCaseBranches::Float { clauses, fallback } => Self::Float(
                FloatListExpr::string_case(FloatListItem, subject, clauses, fallback),
            ),
            ListCaseBranches::Bool { clauses, fallback } => Self::Bool(BoolListExpr::string_case(
                BoolListItem,
                subject,
                clauses,
                fallback,
            )),
            ListCaseBranches::Nil { clauses, fallback } => Self::Nil(NilListExpr::string_case(
                NilListItem,
                subject,
                clauses,
                fallback,
            )),
            ListCaseBranches::Tuple { clauses, fallback } => {
                let item = fallback.item.clone();
                Self::Tuple(TupleListExpr::string_case(item, subject, clauses, fallback))
            }
            ListCaseBranches::List { clauses, fallback } => {
                let item = fallback.item.clone();
                Self::List(ListListExpr::string_case(item, subject, clauses, fallback))
            }
            ListCaseBranches::Function { clauses, fallback } => {
                let item = fallback.item.clone();
                Self::Function(FunctionListExpr::string_case(
                    item, subject, clauses, fallback,
                ))
            }
        }
    }

    pub(crate) fn float_case(subject: FloatExpr, branches: ListCaseBranches<f64>) -> Self {
        match branches {
            ListCaseBranches::Int { clauses, fallback } => Self::Int(IntListExpr::float_case(
                IntListItem,
                subject,
                clauses,
                fallback,
            )),
            ListCaseBranches::String { clauses, fallback } => Self::String(
                StringListExpr::float_case(StringListItem, subject, clauses, fallback),
            ),
            ListCaseBranches::Float { clauses, fallback } => Self::Float(
                FloatListExpr::float_case(FloatListItem, subject, clauses, fallback),
            ),
            ListCaseBranches::Bool { clauses, fallback } => Self::Bool(BoolListExpr::float_case(
                BoolListItem,
                subject,
                clauses,
                fallback,
            )),
            ListCaseBranches::Nil { clauses, fallback } => Self::Nil(NilListExpr::float_case(
                NilListItem,
                subject,
                clauses,
                fallback,
            )),
            ListCaseBranches::Tuple { clauses, fallback } => {
                let item = fallback.item.clone();
                Self::Tuple(TupleListExpr::float_case(item, subject, clauses, fallback))
            }
            ListCaseBranches::List { clauses, fallback } => {
                let item = fallback.item.clone();
                Self::List(ListListExpr::float_case(item, subject, clauses, fallback))
            }
            ListCaseBranches::Function { clauses, fallback } => {
                let item = fallback.item.clone();
                Self::Function(FunctionListExpr::float_case(
                    item, subject, clauses, fallback,
                ))
            }
        }
    }

    pub(crate) fn block(steps: Vec<Step>, return_: ListExpr) -> Self {
        match return_ {
            Self::Int(return_) => Self::Int(IntListExpr::block(IntListItem, steps, return_)),
            Self::String(return_) => {
                Self::String(StringListExpr::block(StringListItem, steps, return_))
            }
            Self::Float(return_) => {
                Self::Float(FloatListExpr::block(FloatListItem, steps, return_))
            }
            Self::Bool(return_) => Self::Bool(BoolListExpr::block(BoolListItem, steps, return_)),
            Self::Nil(return_) => Self::Nil(NilListExpr::block(NilListItem, steps, return_)),
            Self::Tuple(return_) => {
                let item = return_.item.clone();
                Self::Tuple(TupleListExpr::block(item, steps, return_))
            }
            Self::List(return_) => {
                let item = return_.item.clone();
                Self::List(ListListExpr::block(item, steps, return_))
            }
            Self::Function(return_) => {
                let item = return_.item.clone();
                Self::Function(FunctionListExpr::block(item, steps, return_))
            }
        }
    }

    pub fn element_type(&self) -> ValueType {
        match self {
            Self::Int(expression) => expression.element_type(),
            Self::String(expression) => expression.element_type(),
            Self::Float(expression) => expression.element_type(),
            Self::Bool(expression) => expression.element_type(),
            Self::Nil(expression) => expression.element_type(),
            Self::Tuple(expression) => expression.element_type(),
            Self::List(expression) => expression.element_type(),
            Self::Function(expression) => expression.element_type(),
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
        BoolListCaseBranches, BoolListExpr, BoolListItem, FloatListExpr, FloatListItem,
        FunctionListExpr, FunctionListItem, IntListExpr, IntListItem, ListCaseBranches,
        ListElementTypeMismatch, ListElements, ListExpr, ListListExpr, ListListItem, NilListExpr,
        NilListItem, StringListExpr, StringListItem, TupleListExpr, TupleListItem,
    };
    use crate::plan::{
        BoolExpr, Expr, FloatExpr, FunctionExpr, FunctionType, FunctionValue, IntExpr,
        IntFunctionId, IntListFunctionId, IntListLocalId, ListFunctionExpr, ListFunctionId,
        ListFunctionValue, ListLocal, ListValue, NilExpr, PanicExpr, PanicSite, RuntimeFunctionId,
        Step, StringExpr, TupleExpr, ValueType,
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
        let function = FunctionExpr::value(FunctionValue::new(
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
    fn from_value_preserves_typed_item_family() {
        assert_eq!(
            ListExpr::from_value(ListValue::int(vec![1.into()])),
            ListExpr::Int(IntListExpr::value(
                IntListItem,
                vec![IntExpr::value(1.into())]
            )),
        );
        assert_eq!(
            ListExpr::from_value(ListValue::string(vec!["one".into()])),
            ListExpr::String(StringListExpr::value(
                StringListItem,
                vec![StringExpr::value("one".into())],
            )),
        );
        assert_eq!(
            ListExpr::from_value(ListValue::float(vec![1.5])),
            ListExpr::Float(FloatListExpr::value(
                FloatListItem,
                vec![FloatExpr::value(1.5)]
            )),
        );
        assert_eq!(
            ListExpr::from_value(ListValue::bool(vec![true])),
            ListExpr::Bool(BoolListExpr::value(
                BoolListItem,
                vec![BoolExpr::value(true)]
            )),
        );
        assert_eq!(
            ListExpr::from_value(ListValue::nil(2)),
            ListExpr::Nil(NilListExpr::value(
                NilListItem,
                vec![NilExpr::value(), NilExpr::value()],
            )),
        );
        assert_eq!(
            ListExpr::from_value(ListValue::tuple(
                vec![ValueType::Int],
                vec![vec![crate::plan::Value::Int(1.into())]],
            )),
            ListExpr::Tuple(TupleListExpr::value(
                TupleListItem {
                    item_type: vec![ValueType::Int],
                },
                vec![TupleExpr::value(
                    vec![Expr::int(IntExpr::value(1.into()))],
                    vec![ValueType::Int],
                )],
            )),
        );
        assert_eq!(
            ListExpr::from_value(ListValue::list(
                ValueType::String,
                vec![ListValue::string(vec!["child".into()])],
            )),
            ListExpr::List(ListListExpr::value(
                ListListItem {
                    item_type: Box::new(ValueType::String),
                },
                vec![ListExpr::String(StringListExpr::value(
                    StringListItem,
                    vec![StringExpr::value("child".into())],
                ))],
            )),
        );

        let function_type = FunctionType::new(Vec::new(), ValueType::Int);
        let function_value =
            FunctionValue::new(RuntimeFunctionId::Int(IntFunctionId(0)), Vec::new());
        assert_eq!(
            ListExpr::from_value(ListValue::function(
                function_type.clone(),
                vec![function_value.clone()],
            )),
            ListExpr::Function(FunctionListExpr::value(
                FunctionListItem {
                    item_type: function_type,
                },
                vec![FunctionExpr::value(function_value)],
            )),
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
                IntListItem,
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
                TupleListItem {
                    item_type: vec![ValueType::Int],
                },
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

        let list_function = ListFunctionExpr::value(ListFunctionValue::new(
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
            ListExpr::String(StringListExpr::list_index(StringListItem, nested, 0)),
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
            ListExpr::Int(IntListExpr::bool_case(IntListItem, subject, true_, false_,)),
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
        let item = TupleListItem {
            item_type: vec![ValueType::Int],
        };
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
                item.clone(),
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
                StringListItem,
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
                    ListFunctionExpr::value(ListFunctionValue::new(
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
        let item_types = vec![
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

            assert_eq!(ListExpr::list_index(list, 0).element_type(), item_type);
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
                vec![Expr::function(FunctionExpr::value(FunctionValue::new(
                    RuntimeFunctionId::Int(IntFunctionId(0)),
                    Vec::new(),
                )))],
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
