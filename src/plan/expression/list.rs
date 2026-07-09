use super::{
    BoolExpr, CallArg, Expr, ExprKind, FloatExpr, FunctionExpr, IntExpr, ListFunctionExpr, NilExpr,
    PanicExpr, StringExpr, TupleExpr,
};
use crate::plan::{
    BoolListFunctionId, BoolListLocalId, FloatListFunctionId, FloatListLocalId,
    FunctionListFunctionId, FunctionListLocalId, FunctionType, IntListFunctionId, IntListLocalId,
    ListFunctionId, ListListFunctionId, ListListLocalId, ListLocal, NilListFunctionId,
    NilListLocalId, Step, StringListFunctionId, StringListLocalId, TupleListFunctionId,
    TupleListLocalId, ValueType,
};
use ecow::EcoString;
use num_bigint::BigInt;
use std::fmt::Debug;

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

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum BoolListCaseBranches {
    Int {
        true_: IntListExpr,
        false_: IntListExpr,
    },
    String {
        true_: StringListExpr,
        false_: StringListExpr,
    },
    Float {
        true_: FloatListExpr,
        false_: FloatListExpr,
    },
    Bool {
        true_: BoolListExpr,
        false_: BoolListExpr,
    },
    Nil {
        true_: NilListExpr,
        false_: NilListExpr,
    },
    Tuple {
        true_: TupleListExpr,
        false_: TupleListExpr,
    },
    List {
        true_: ListListExpr,
        false_: ListListExpr,
    },
    Function {
        true_: FunctionListExpr,
        false_: FunctionListExpr,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct TypedListExpr<Item: ListItem> {
    item: Item,
    kind: TypedListExprKind<Item>,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum TypedListExprKind<Item: ListItem> {
    Value(Vec<Item::ElementExpr>),
    Spread {
        elements: Vec<Item::ElementExpr>,
        tail: Box<TypedListExpr<Item>>,
    },
    LocalGet {
        local: Item::Local,
        name: EcoString,
    },
    Call {
        function: Item::Function,
        args: Vec<CallArg>,
    },
    FunctionCall {
        function: Box<ListFunctionExpr>,
        args: Vec<CallArg>,
    },
    TupleIndex {
        tuple: Box<TupleExpr>,
        index: usize,
    },
    ListIndex {
        list: Box<ListListExpr>,
        index: usize,
    },
    DropFirst {
        list: Box<TypedListExpr<Item>>,
        count: usize,
    },
    Panic(PanicExpr),
    BoolCase {
        subject: Box<BoolExpr>,
        true_: Box<TypedListExpr<Item>>,
        false_: Box<TypedListExpr<Item>>,
    },
    IntCase {
        subject: Box<IntExpr>,
        clauses: Vec<(BigInt, TypedListExpr<Item>)>,
        fallback: Box<TypedListExpr<Item>>,
    },
    StringCase {
        subject: Box<StringExpr>,
        clauses: Vec<(EcoString, TypedListExpr<Item>)>,
        fallback: Box<TypedListExpr<Item>>,
    },
    FloatCase {
        subject: Box<FloatExpr>,
        clauses: Vec<(f64, TypedListExpr<Item>)>,
        fallback: Box<TypedListExpr<Item>>,
    },
    Block {
        steps: Vec<Step>,
        return_: Box<TypedListExpr<Item>>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum ListElements {
    Int(Vec<IntExpr>),
    String(Vec<StringExpr>),
    Float(Vec<FloatExpr>),
    Bool(Vec<BoolExpr>),
    Nil(Vec<NilExpr>),
    Tuple {
        item_type: Vec<ValueType>,
        values: Vec<TupleExpr>,
    },
    List {
        item_type: Box<ValueType>,
        values: Vec<ListExpr>,
    },
    Function {
        item_type: FunctionType,
        values: Vec<FunctionExpr>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum ListLocalExpr {
    Int {
        local: IntListLocalId,
        value: IntListExpr,
    },
    String {
        local: StringListLocalId,
        value: StringListExpr,
    },
    Float {
        local: FloatListLocalId,
        value: FloatListExpr,
    },
    Bool {
        local: BoolListLocalId,
        value: BoolListExpr,
    },
    Nil {
        local: NilListLocalId,
        value: NilListExpr,
    },
    Tuple {
        local: TupleListLocalId,
        item_type: Vec<ValueType>,
        value: TupleListExpr,
    },
    List {
        local: ListListLocalId,
        item_type: Box<ValueType>,
        value: ListListExpr,
    },
    Function {
        local: FunctionListLocalId,
        item_type: FunctionType,
        value: FunctionListExpr,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ListElementTypeMismatch {
    pub(crate) expected: ValueType,
    pub(crate) actual: ValueType,
}

pub(crate) trait ListItem: Debug + Clone + PartialEq {
    type ElementExpr: Debug + Clone + PartialEq;
    type Local: Debug + Clone + PartialEq;
    type Function: Debug + Clone + PartialEq;

    fn value_type(&self) -> ValueType;

    fn local_to_facade(&self, local: Self::Local) -> ListLocal;

    fn elements_from_exprs(
        item: &Self,
        values: Vec<Expr>,
    ) -> Result<Vec<Self::ElementExpr>, ListElementTypeMismatch>;

    fn elements_to_facade(item: Self, values: Vec<Self::ElementExpr>) -> ListElements;

    fn expr_to_facade(expression: TypedListExpr<Self>) -> ListExpr
    where
        Self: Sized;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct IntListItem;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct StringListItem;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct FloatListItem;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct BoolListItem;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct NilListItem;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct TupleListItem {
    item_type: Vec<ValueType>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ListListItem {
    item_type: Box<ValueType>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct FunctionListItem {
    item_type: FunctionType,
}

impl TupleListItem {
    pub(crate) fn new(item_type: Vec<ValueType>) -> Self {
        Self { item_type }
    }

    pub(crate) fn item_type(&self) -> Vec<ValueType> {
        self.item_type.clone()
    }
}

impl ListListItem {
    pub(crate) fn new(item_type: Box<ValueType>) -> Self {
        Self { item_type }
    }

    pub(crate) fn item_type(&self) -> Box<ValueType> {
        self.item_type.clone()
    }
}

impl FunctionListItem {
    pub(crate) fn new(item_type: FunctionType) -> Self {
        Self { item_type }
    }

    pub(crate) fn item_type(&self) -> FunctionType {
        self.item_type.clone()
    }
}

impl<Item: ListItem> TypedListExpr<Item> {
    fn new(item: Item, kind: TypedListExprKind<Item>) -> Self {
        Self { item, kind }
    }

    pub(crate) fn item(&self) -> &Item {
        &self.item
    }

    pub(crate) fn element_type(&self) -> ValueType {
        self.item.value_type()
    }

    pub(crate) fn kind(&self) -> &TypedListExprKind<Item> {
        &self.kind
    }

    fn value(item: Item, elements: Vec<Item::ElementExpr>) -> Self {
        Self::new(item, TypedListExprKind::Value(elements))
    }

    fn spread(item: Item, elements: Vec<Item::ElementExpr>, tail: TypedListExpr<Item>) -> Self {
        Self::new(
            item,
            TypedListExprKind::Spread {
                elements,
                tail: Box::new(tail),
            },
        )
    }

    pub(crate) fn local_get(item: Item, local: Item::Local, name: EcoString) -> Self {
        Self::new(item, TypedListExprKind::LocalGet { local, name })
    }

    fn call(item: Item, function: Item::Function, args: Vec<CallArg>) -> Self {
        Self::new(item, TypedListExprKind::Call { function, args })
    }

    fn function_call(item: Item, function: ListFunctionExpr, args: Vec<CallArg>) -> Self {
        Self::new(
            item,
            TypedListExprKind::FunctionCall {
                function: Box::new(function),
                args,
            },
        )
    }

    fn tuple_index(item: Item, tuple: TupleExpr, index: usize) -> Self {
        Self::new(
            item,
            TypedListExprKind::TupleIndex {
                tuple: Box::new(tuple),
                index,
            },
        )
    }

    fn list_index(item: Item, list: ListListExpr, index: usize) -> Self {
        Self::new(
            item,
            TypedListExprKind::ListIndex {
                list: Box::new(list),
                index,
            },
        )
    }

    fn drop_first(item: Item, list: TypedListExpr<Item>, count: usize) -> Self {
        Self::new(
            item,
            TypedListExprKind::DropFirst {
                list: Box::new(list),
                count,
            },
        )
    }

    fn panic(item: Item, panic: PanicExpr) -> Self {
        Self::new(item, TypedListExprKind::Panic(panic))
    }

    fn bool_case(
        item: Item,
        subject: BoolExpr,
        true_: TypedListExpr<Item>,
        false_: TypedListExpr<Item>,
    ) -> Self {
        Self::new(
            item,
            TypedListExprKind::BoolCase {
                subject: Box::new(subject),
                true_: Box::new(true_),
                false_: Box::new(false_),
            },
        )
    }

    fn int_case(
        item: Item,
        subject: IntExpr,
        clauses: Vec<(BigInt, TypedListExpr<Item>)>,
        fallback: TypedListExpr<Item>,
    ) -> Self {
        Self::new(
            item,
            TypedListExprKind::IntCase {
                subject: Box::new(subject),
                clauses,
                fallback: Box::new(fallback),
            },
        )
    }

    fn string_case(
        item: Item,
        subject: StringExpr,
        clauses: Vec<(EcoString, TypedListExpr<Item>)>,
        fallback: TypedListExpr<Item>,
    ) -> Self {
        Self::new(
            item,
            TypedListExprKind::StringCase {
                subject: Box::new(subject),
                clauses,
                fallback: Box::new(fallback),
            },
        )
    }

    fn float_case(
        item: Item,
        subject: FloatExpr,
        clauses: Vec<(f64, TypedListExpr<Item>)>,
        fallback: TypedListExpr<Item>,
    ) -> Self {
        Self::new(
            item,
            TypedListExprKind::FloatCase {
                subject: Box::new(subject),
                clauses,
                fallback: Box::new(fallback),
            },
        )
    }

    fn block(item: Item, steps: Vec<Step>, return_: TypedListExpr<Item>) -> Self {
        Self::new(
            item,
            TypedListExprKind::Block {
                steps,
                return_: Box::new(return_),
            },
        )
    }
}

impl ListExpr {
    pub(crate) fn value(elements: Vec<Expr>, element_type: ValueType) -> Self {
        Self::try_value(elements, element_type)
            .expect("list expression elements must match declared item type")
    }

    pub(crate) fn try_value(
        elements: Vec<Expr>,
        element_type: ValueType,
    ) -> Result<Self, ListElementTypeMismatch> {
        let item = ListItemTag::from_value_type(element_type.clone());
        item.expr_from_values(elements)
    }

    #[cfg(test)]
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

    #[cfg(test)]
    pub(crate) fn spread(elements: Vec<Expr>, tail: ListExpr, element_type: ValueType) -> Self {
        let elements = ListElements::from_exprs(element_type, elements)
            .expect("list spread elements must match declared item type");
        Self::try_spread(elements, tail).expect("list spread tail must match prefix item type")
    }

    #[cfg(test)]
    pub(crate) fn try_spread(
        elements: ListElements,
        tail: ListExpr,
    ) -> Result<Self, ListElementTypeMismatch> {
        let expected = elements.item_type();
        let actual = tail.element_type();
        if expected != actual {
            return Err(ListElementTypeMismatch { expected, actual });
        }

        Ok(Self::from_spread_elements(elements, tail))
    }

    pub(crate) fn from_spread_elements(elements: ListElements, tail: ListExpr) -> Self {
        match elements {
            ListElements::Int(values) => {
                let tail = tail.into_int().expect("list spread tail must be List(Int)");
                Self::Int(IntListExpr::spread(IntListItem, values, tail))
            }
            ListElements::String(values) => {
                let tail = tail
                    .into_string()
                    .expect("list spread tail must be List(String)");
                Self::String(StringListExpr::spread(StringListItem, values, tail))
            }
            ListElements::Float(values) => {
                let tail = tail
                    .into_float()
                    .expect("list spread tail must be List(Float)");
                Self::Float(FloatListExpr::spread(FloatListItem, values, tail))
            }
            ListElements::Bool(values) => {
                let tail = tail
                    .into_bool()
                    .expect("list spread tail must be List(Bool)");
                Self::Bool(BoolListExpr::spread(BoolListItem, values, tail))
            }
            ListElements::Nil(values) => {
                let tail = tail.into_nil().expect("list spread tail must be List(Nil)");
                Self::Nil(NilListExpr::spread(NilListItem, values, tail))
            }
            ListElements::Tuple { item_type, values } => {
                let item = TupleListItem { item_type };
                let tail = tail
                    .into_tuple()
                    .filter(|tail| tail.item == item)
                    .expect("list spread tail must be List(tuple item type)");
                Self::Tuple(TupleListExpr::spread(item, values, tail))
            }
            ListElements::List { item_type, values } => {
                let item = ListListItem { item_type };
                let tail = tail
                    .into_list()
                    .filter(|tail| tail.item == item)
                    .expect("list spread tail must be List(list item type)");
                Self::List(ListListExpr::spread(item, values, tail))
            }
            ListElements::Function { item_type, values } => {
                let item = FunctionListItem { item_type };
                let tail = tail
                    .into_function()
                    .filter(|tail| tail.item == item)
                    .expect("list spread tail must be List(function item type)");
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

    pub(crate) fn int_case(
        subject: IntExpr,
        clauses: Vec<(BigInt, ListExpr)>,
        fallback: ListExpr,
    ) -> Self {
        match fallback {
            Self::Int(fallback) => Self::Int(IntListExpr::int_case(
                IntListItem,
                subject,
                into_int_clauses(clauses),
                fallback,
            )),
            Self::String(fallback) => Self::String(StringListExpr::int_case(
                StringListItem,
                subject,
                into_string_clauses(clauses),
                fallback,
            )),
            Self::Float(fallback) => Self::Float(FloatListExpr::int_case(
                FloatListItem,
                subject,
                into_float_clauses(clauses),
                fallback,
            )),
            Self::Bool(fallback) => Self::Bool(BoolListExpr::int_case(
                BoolListItem,
                subject,
                into_bool_clauses(clauses),
                fallback,
            )),
            Self::Nil(fallback) => Self::Nil(NilListExpr::int_case(
                NilListItem,
                subject,
                into_nil_clauses(clauses),
                fallback,
            )),
            Self::Tuple(fallback) => {
                let item = fallback.item.clone();
                Self::Tuple(TupleListExpr::int_case(
                    item.clone(),
                    subject,
                    into_tuple_clauses(clauses, &item),
                    fallback,
                ))
            }
            Self::List(fallback) => {
                let item = fallback.item.clone();
                Self::List(ListListExpr::int_case(
                    item.clone(),
                    subject,
                    into_list_clauses(clauses, &item),
                    fallback,
                ))
            }
            Self::Function(fallback) => {
                let item = fallback.item.clone();
                Self::Function(FunctionListExpr::int_case(
                    item.clone(),
                    subject,
                    into_function_clauses(clauses, &item),
                    fallback,
                ))
            }
        }
    }

    pub(crate) fn string_case(
        subject: StringExpr,
        clauses: Vec<(EcoString, ListExpr)>,
        fallback: ListExpr,
    ) -> Self {
        match fallback {
            Self::Int(fallback) => Self::Int(IntListExpr::string_case(
                IntListItem,
                subject,
                into_int_clauses(clauses),
                fallback,
            )),
            Self::String(fallback) => Self::String(StringListExpr::string_case(
                StringListItem,
                subject,
                into_string_clauses(clauses),
                fallback,
            )),
            Self::Float(fallback) => Self::Float(FloatListExpr::string_case(
                FloatListItem,
                subject,
                into_float_clauses(clauses),
                fallback,
            )),
            Self::Bool(fallback) => Self::Bool(BoolListExpr::string_case(
                BoolListItem,
                subject,
                into_bool_clauses(clauses),
                fallback,
            )),
            Self::Nil(fallback) => Self::Nil(NilListExpr::string_case(
                NilListItem,
                subject,
                into_nil_clauses(clauses),
                fallback,
            )),
            Self::Tuple(fallback) => {
                let item = fallback.item.clone();
                Self::Tuple(TupleListExpr::string_case(
                    item.clone(),
                    subject,
                    into_tuple_clauses(clauses, &item),
                    fallback,
                ))
            }
            Self::List(fallback) => {
                let item = fallback.item.clone();
                Self::List(ListListExpr::string_case(
                    item.clone(),
                    subject,
                    into_list_clauses(clauses, &item),
                    fallback,
                ))
            }
            Self::Function(fallback) => {
                let item = fallback.item.clone();
                Self::Function(FunctionListExpr::string_case(
                    item.clone(),
                    subject,
                    into_function_clauses(clauses, &item),
                    fallback,
                ))
            }
        }
    }

    pub(crate) fn float_case(
        subject: FloatExpr,
        clauses: Vec<(f64, ListExpr)>,
        fallback: ListExpr,
    ) -> Self {
        match fallback {
            Self::Int(fallback) => Self::Int(IntListExpr::float_case(
                IntListItem,
                subject,
                into_int_clauses(clauses),
                fallback,
            )),
            Self::String(fallback) => Self::String(StringListExpr::float_case(
                StringListItem,
                subject,
                into_string_clauses(clauses),
                fallback,
            )),
            Self::Float(fallback) => Self::Float(FloatListExpr::float_case(
                FloatListItem,
                subject,
                into_float_clauses(clauses),
                fallback,
            )),
            Self::Bool(fallback) => Self::Bool(BoolListExpr::float_case(
                BoolListItem,
                subject,
                into_bool_clauses(clauses),
                fallback,
            )),
            Self::Nil(fallback) => Self::Nil(NilListExpr::float_case(
                NilListItem,
                subject,
                into_nil_clauses(clauses),
                fallback,
            )),
            Self::Tuple(fallback) => {
                let item = fallback.item.clone();
                Self::Tuple(TupleListExpr::float_case(
                    item.clone(),
                    subject,
                    into_tuple_clauses(clauses, &item),
                    fallback,
                ))
            }
            Self::List(fallback) => {
                let item = fallback.item.clone();
                Self::List(ListListExpr::float_case(
                    item.clone(),
                    subject,
                    into_list_clauses(clauses, &item),
                    fallback,
                ))
            }
            Self::Function(fallback) => {
                let item = fallback.item.clone();
                Self::Function(FunctionListExpr::float_case(
                    item.clone(),
                    subject,
                    into_function_clauses(clauses, &item),
                    fallback,
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
macro_rules! impl_typed_list_expr_from_facade {
    ($type:ty, $method:ident, $name:literal) => {
        impl From<ListExpr> for $type {
            fn from(value: ListExpr) -> Self {
                value
                    .$method()
                    .expect(concat!("expected ", $name, " list expression"))
            }
        }
    };
}

#[cfg(test)]
impl_typed_list_expr_from_facade!(IntListExpr, into_int, "int");
#[cfg(test)]
impl_typed_list_expr_from_facade!(StringListExpr, into_string, "string");
#[cfg(test)]
impl_typed_list_expr_from_facade!(FloatListExpr, into_float, "float");
#[cfg(test)]
impl_typed_list_expr_from_facade!(BoolListExpr, into_bool, "bool");
#[cfg(test)]
impl_typed_list_expr_from_facade!(NilListExpr, into_nil, "nil");
#[cfg(test)]
impl_typed_list_expr_from_facade!(TupleListExpr, into_tuple, "tuple");
#[cfg(test)]
impl_typed_list_expr_from_facade!(ListListExpr, into_list, "list");
#[cfg(test)]
impl_typed_list_expr_from_facade!(FunctionListExpr, into_function, "function");

impl ListElements {
    pub(crate) fn from_exprs(
        item_type: ValueType,
        values: Vec<Expr>,
    ) -> Result<Self, ListElementTypeMismatch> {
        match item_type {
            ValueType::Int => list_elements_from_exprs(IntListItem, values),
            ValueType::String => list_elements_from_exprs(StringListItem, values),
            ValueType::Float => list_elements_from_exprs(FloatListItem, values),
            ValueType::Bool => list_elements_from_exprs(BoolListItem, values),
            ValueType::Nil => list_elements_from_exprs(NilListItem, values),
            ValueType::Tuple(item_type) => {
                list_elements_from_exprs(TupleListItem { item_type }, values)
            }
            ValueType::List(item_type) => {
                list_elements_from_exprs(ListListItem { item_type }, values)
            }
            ValueType::Function(item_type) => list_elements_from_exprs(
                FunctionListItem {
                    item_type: *item_type,
                },
                values,
            ),
        }
    }

    #[cfg(test)]
    pub(crate) fn item_type(&self) -> ValueType {
        match self {
            Self::Int(_) => ValueType::Int,
            Self::String(_) => ValueType::String,
            Self::Float(_) => ValueType::Float,
            Self::Bool(_) => ValueType::Bool,
            Self::Nil(_) => ValueType::Nil,
            Self::Tuple { item_type, .. } => ValueType::Tuple(item_type.clone()),
            Self::List { item_type, .. } => ValueType::List(item_type.clone()),
            Self::Function { item_type, .. } => ValueType::Function(Box::new(item_type.clone())),
        }
    }
}

fn list_elements_from_exprs<Item: ListItem>(
    item: Item,
    values: Vec<Expr>,
) -> Result<ListElements, ListElementTypeMismatch> {
    let elements = Item::elements_from_exprs(&item, values)?;
    Ok(Item::elements_to_facade(item, elements))
}

#[derive(Debug, Clone, PartialEq)]
enum ListItemTag {
    Int(IntListItem),
    String(StringListItem),
    Float(FloatListItem),
    Bool(BoolListItem),
    Nil(NilListItem),
    Tuple(TupleListItem),
    List(ListListItem),
    Function(FunctionListItem),
}

impl ListItemTag {
    fn from_value_type(value: ValueType) -> Self {
        match value {
            ValueType::Int => Self::Int(IntListItem),
            ValueType::String => Self::String(StringListItem),
            ValueType::Float => Self::Float(FloatListItem),
            ValueType::Bool => Self::Bool(BoolListItem),
            ValueType::Nil => Self::Nil(NilListItem),
            ValueType::Tuple(item_type) => Self::Tuple(TupleListItem { item_type }),
            ValueType::List(item_type) => Self::List(ListListItem { item_type }),
            ValueType::Function(item_type) => Self::Function(FunctionListItem {
                item_type: *item_type,
            }),
        }
    }

    fn expr_from_values(self, values: Vec<Expr>) -> Result<ListExpr, ListElementTypeMismatch> {
        match self {
            Self::Int(item) => ItemExprBuilder(item).build(values).map(ListExpr::Int),
            Self::String(item) => ItemExprBuilder(item).build(values).map(ListExpr::String),
            Self::Float(item) => ItemExprBuilder(item).build(values).map(ListExpr::Float),
            Self::Bool(item) => ItemExprBuilder(item).build(values).map(ListExpr::Bool),
            Self::Nil(item) => ItemExprBuilder(item).build(values).map(ListExpr::Nil),
            Self::Tuple(item) => ItemExprBuilder(item).build(values).map(ListExpr::Tuple),
            Self::List(item) => ItemExprBuilder(item).build(values).map(ListExpr::List),
            Self::Function(item) => ItemExprBuilder(item).build(values).map(ListExpr::Function),
        }
    }
}

struct ItemExprBuilder<Item>(Item);

impl<Item: ListItem> ItemExprBuilder<Item> {
    fn build(self, values: Vec<Expr>) -> Result<TypedListExpr<Item>, ListElementTypeMismatch> {
        let elements = Item::elements_from_exprs(&self.0, values)?;
        Ok(TypedListExpr::value(self.0, elements))
    }
}

macro_rules! primitive_list_item {
    (
        $item:ident,
        $expr:ty,
        $local:ty,
        $function:ty,
        $value_type:expr,
        $expr_pattern:pat => $expr_value:expr,
        $elements_variant:ident,
        $facade_variant:path,
        $local_ctor:path
    ) => {
        impl ListItem for $item {
            type ElementExpr = $expr;
            type Local = $local;
            type Function = $function;

            fn value_type(&self) -> ValueType {
                $value_type
            }

            fn local_to_facade(&self, local: Self::Local) -> ListLocal {
                $local_ctor(local)
            }

            fn elements_from_exprs(
                _item: &Self,
                values: Vec<Expr>,
            ) -> Result<Vec<Self::ElementExpr>, ListElementTypeMismatch> {
                values
                    .into_iter()
                    .map(|value| match value {
                        Expr {
                            kind: $expr_pattern,
                        } => Ok($expr_value),
                        value => Err(ListElementTypeMismatch {
                            expected: $value_type,
                            actual: value.value_type(),
                        }),
                    })
                    .collect()
            }

            fn elements_to_facade(_item: Self, values: Vec<Self::ElementExpr>) -> ListElements {
                ListElements::$elements_variant(values)
            }

            fn expr_to_facade(expression: TypedListExpr<Self>) -> ListExpr {
                $facade_variant(expression)
            }
        }
    };
}

primitive_list_item!(
    IntListItem,
    IntExpr,
    IntListLocalId,
    IntListFunctionId,
    ValueType::Int,
    ExprKind::Int(value) => value,
    Int,
    ListExpr::Int,
    ListLocal::Int
);

primitive_list_item!(
    StringListItem,
    StringExpr,
    StringListLocalId,
    StringListFunctionId,
    ValueType::String,
    ExprKind::String(value) => value,
    String,
    ListExpr::String,
    ListLocal::String
);

primitive_list_item!(
    FloatListItem,
    FloatExpr,
    FloatListLocalId,
    FloatListFunctionId,
    ValueType::Float,
    ExprKind::Float(value) => value,
    Float,
    ListExpr::Float,
    ListLocal::Float
);

primitive_list_item!(
    BoolListItem,
    BoolExpr,
    BoolListLocalId,
    BoolListFunctionId,
    ValueType::Bool,
    ExprKind::Bool(value) => value,
    Bool,
    ListExpr::Bool,
    ListLocal::Bool
);

primitive_list_item!(
    NilListItem,
    NilExpr,
    NilListLocalId,
    NilListFunctionId,
    ValueType::Nil,
    ExprKind::Nil(value) => value,
    Nil,
    ListExpr::Nil,
    ListLocal::Nil
);

impl ListItem for TupleListItem {
    type ElementExpr = TupleExpr;
    type Local = TupleListLocalId;
    type Function = TupleListFunctionId;

    fn value_type(&self) -> ValueType {
        ValueType::Tuple(self.item_type.clone())
    }

    fn local_to_facade(&self, local: Self::Local) -> ListLocal {
        ListLocal::tuple(local, self.item_type.clone())
    }

    fn elements_from_exprs(
        item: &Self,
        values: Vec<Expr>,
    ) -> Result<Vec<Self::ElementExpr>, ListElementTypeMismatch> {
        values
            .into_iter()
            .map(|value| match value {
                Expr {
                    kind: ExprKind::Tuple(value),
                } if value.type_() == item.item_type.as_slice() => Ok(value),
                value => Err(ListElementTypeMismatch {
                    expected: item.value_type(),
                    actual: value.value_type(),
                }),
            })
            .collect()
    }

    fn elements_to_facade(item: Self, values: Vec<Self::ElementExpr>) -> ListElements {
        ListElements::Tuple {
            item_type: item.item_type,
            values,
        }
    }

    fn expr_to_facade(expression: TypedListExpr<Self>) -> ListExpr {
        ListExpr::Tuple(expression)
    }
}

impl ListItem for ListListItem {
    type ElementExpr = ListExpr;
    type Local = ListListLocalId;
    type Function = ListListFunctionId;

    fn value_type(&self) -> ValueType {
        ValueType::List(self.item_type.clone())
    }

    fn local_to_facade(&self, local: Self::Local) -> ListLocal {
        ListLocal::list(local, self.item_type.as_ref().clone())
    }

    fn elements_from_exprs(
        item: &Self,
        values: Vec<Expr>,
    ) -> Result<Vec<Self::ElementExpr>, ListElementTypeMismatch> {
        values
            .into_iter()
            .map(|value| match value {
                Expr {
                    kind: ExprKind::List(value),
                } if value.element_type() == item.item_type.as_ref().clone() => Ok(value),
                value => Err(ListElementTypeMismatch {
                    expected: item.value_type(),
                    actual: value.value_type(),
                }),
            })
            .collect()
    }

    fn elements_to_facade(item: Self, values: Vec<Self::ElementExpr>) -> ListElements {
        ListElements::List {
            item_type: item.item_type,
            values,
        }
    }

    fn expr_to_facade(expression: TypedListExpr<Self>) -> ListExpr {
        ListExpr::List(expression)
    }
}

impl ListItem for FunctionListItem {
    type ElementExpr = FunctionExpr;
    type Local = FunctionListLocalId;
    type Function = FunctionListFunctionId;

    fn value_type(&self) -> ValueType {
        ValueType::Function(Box::new(self.item_type.clone()))
    }

    fn local_to_facade(&self, local: Self::Local) -> ListLocal {
        ListLocal::function(local, self.item_type.clone())
    }

    fn elements_from_exprs(
        item: &Self,
        values: Vec<Expr>,
    ) -> Result<Vec<Self::ElementExpr>, ListElementTypeMismatch> {
        values
            .into_iter()
            .map(|value| match value {
                Expr {
                    kind: ExprKind::Function(value),
                } if value.type_() == &item.item_type => Ok(value),
                value => Err(ListElementTypeMismatch {
                    expected: item.value_type(),
                    actual: value.value_type(),
                }),
            })
            .collect()
    }

    fn elements_to_facade(item: Self, values: Vec<Self::ElementExpr>) -> ListElements {
        ListElements::Function {
            item_type: item.item_type,
            values,
        }
    }

    fn expr_to_facade(expression: TypedListExpr<Self>) -> ListExpr {
        ListExpr::Function(expression)
    }
}

fn into_int_clauses<Pattern>(clauses: Vec<(Pattern, ListExpr)>) -> Vec<(Pattern, IntListExpr)> {
    clauses
        .into_iter()
        .map(|(pattern, branch)| {
            (
                pattern,
                branch
                    .into_int()
                    .expect("list case branches must be List(Int)"),
            )
        })
        .collect()
}

fn into_string_clauses<Pattern>(
    clauses: Vec<(Pattern, ListExpr)>,
) -> Vec<(Pattern, StringListExpr)> {
    clauses
        .into_iter()
        .map(|(pattern, branch)| {
            (
                pattern,
                branch
                    .into_string()
                    .expect("list case branches must be List(String)"),
            )
        })
        .collect()
}

fn into_float_clauses<Pattern>(clauses: Vec<(Pattern, ListExpr)>) -> Vec<(Pattern, FloatListExpr)> {
    clauses
        .into_iter()
        .map(|(pattern, branch)| {
            (
                pattern,
                branch
                    .into_float()
                    .expect("list case branches must be List(Float)"),
            )
        })
        .collect()
}

fn into_bool_clauses<Pattern>(clauses: Vec<(Pattern, ListExpr)>) -> Vec<(Pattern, BoolListExpr)> {
    clauses
        .into_iter()
        .map(|(pattern, branch)| {
            (
                pattern,
                branch
                    .into_bool()
                    .expect("list case branches must be List(Bool)"),
            )
        })
        .collect()
}

fn into_nil_clauses<Pattern>(clauses: Vec<(Pattern, ListExpr)>) -> Vec<(Pattern, NilListExpr)> {
    clauses
        .into_iter()
        .map(|(pattern, branch)| {
            (
                pattern,
                branch
                    .into_nil()
                    .expect("list case branches must be List(Nil)"),
            )
        })
        .collect()
}

fn into_tuple_clauses<Pattern>(
    clauses: Vec<(Pattern, ListExpr)>,
    item: &TupleListItem,
) -> Vec<(Pattern, TupleListExpr)> {
    clauses
        .into_iter()
        .map(|(pattern, branch)| {
            let branch = branch
                .into_tuple()
                .expect("list case branches must be List(tuple)");
            assert_eq!(
                &branch.item, item,
                "tuple list branch item types must match"
            );
            (pattern, branch)
        })
        .collect()
}

fn into_list_clauses<Pattern>(
    clauses: Vec<(Pattern, ListExpr)>,
    item: &ListListItem,
) -> Vec<(Pattern, ListListExpr)> {
    clauses
        .into_iter()
        .map(|(pattern, branch)| {
            let branch = branch
                .into_list()
                .expect("list case branches must be List(list)");
            assert_eq!(
                &branch.item, item,
                "nested list branch item types must match"
            );
            (pattern, branch)
        })
        .collect()
}

fn into_function_clauses<Pattern>(
    clauses: Vec<(Pattern, ListExpr)>,
    item: &FunctionListItem,
) -> Vec<(Pattern, FunctionListExpr)> {
    clauses
        .into_iter()
        .map(|(pattern, branch)| {
            let branch = branch
                .into_function()
                .expect("list case branches must be List(function)");
            assert_eq!(
                &branch.item, item,
                "function list branch item types must match"
            );
            (pattern, branch)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{
        BoolListCaseBranches, BoolListExpr, BoolListItem, FloatListExpr, FloatListItem,
        FunctionListExpr, FunctionListItem, IntListExpr, IntListItem, ListElementTypeMismatch,
        ListElements, ListExpr, ListListExpr, ListListItem, NilListExpr, NilListItem,
        StringListExpr, StringListItem, TupleListExpr, TupleListItem, TypedListExprKind,
        into_function_clauses, into_list_clauses, into_nil_clauses, into_tuple_clauses,
    };
    use crate::plan::{
        BoolExpr, Expr, FloatExpr, FunctionExpr, FunctionType, FunctionValue, IntExpr,
        IntFunctionId, IntListFunctionId, IntListLocalId, ListFunctionExpr, ListFunctionId,
        ListFunctionValue, ListLocal, NilExpr, PanicExpr, PanicSite, RuntimeFunctionId, Step,
        StringExpr, TupleExpr, ValueType,
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
    fn from_exprs_rejects_wrong_item_family_and_nested_metadata() {
        assert_eq!(
            ListElements::from_exprs(
                ValueType::Int,
                vec![Expr::string(StringExpr::value("wrong".into()))]
            ),
            Err(ListElementTypeMismatch {
                expected: ValueType::Int,
                actual: ValueType::String,
            }),
        );

        let tuple = TupleExpr::value(
            vec![Expr::string(StringExpr::value("wrong".into()))],
            vec![ValueType::String],
        );
        assert_eq!(
            ListElements::from_exprs(
                ValueType::Tuple(vec![ValueType::Int]),
                vec![Expr::tuple(tuple)],
            ),
            Err(ListElementTypeMismatch {
                expected: ValueType::Tuple(vec![ValueType::Int]),
                actual: ValueType::Tuple(vec![ValueType::String]),
            }),
        );

        let nested = ListExpr::value(
            vec![Expr::string(StringExpr::value("wrong".into()))],
            ValueType::String,
        );
        assert_eq!(
            ListElements::from_exprs(
                ValueType::List(Box::new(ValueType::Int)),
                vec![Expr::list(nested)],
            ),
            Err(ListElementTypeMismatch {
                expected: ValueType::List(Box::new(ValueType::Int)),
                actual: ValueType::List(Box::new(ValueType::String)),
            }),
        );

        let function = FunctionExpr::value(FunctionValue::new(
            RuntimeFunctionId::Int(IntFunctionId(0)),
            Vec::new(),
        ));
        assert_eq!(
            ListElements::from_exprs(
                ValueType::Function(Box::new(FunctionType::new(Vec::new(), ValueType::String))),
                vec![Expr::function(function)],
            ),
            Err(ListElementTypeMismatch {
                expected: ValueType::Function(Box::new(FunctionType::new(
                    Vec::new(),
                    ValueType::String,
                ))),
                actual: ValueType::Function(Box::new(FunctionType::new(
                    Vec::new(),
                    ValueType::Int,
                ))),
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
                vec![(BigInt::from(1), branch.clone())],
                fallback.clone(),
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
    fn typed_expression_accessors_report_item_type_and_kind() {
        let expression =
            ListExpr::value(vec![Expr::float(FloatExpr::value(1.5))], ValueType::Float)
                .into_float()
                .expect("float list");

        assert_eq!(expression.item(), &FloatListItem);
        assert_eq!(expression.element_type(), ValueType::Float);
        assert_eq!(
            expression.kind(),
            &TypedListExprKind::Value(vec![FloatExpr::value(1.5)]),
        );
        assert_eq!(
            ListExpr::Float(expression.clone()).element_type(),
            ValueType::Float,
        );
        assert_eq!(
            ListExpr::Float(expression.clone()).into_float(),
            Some(expression)
        );
        assert_eq!(
            ListExpr::value(vec![Expr::bool(BoolExpr::value(false))], ValueType::Bool).into_float(),
            None,
        );
        assert_eq!(
            ListExpr::value(
                vec![Expr::string(StringExpr::value("one".into()))],
                ValueType::String,
            )
            .into_int(),
            None,
        );
        assert_eq!(
            ListExpr::value(vec![Expr::int(IntExpr::value(1.into()))], ValueType::Int)
                .into_string(),
            None,
        );
        assert_eq!(
            ListExpr::value(vec![Expr::int(IntExpr::value(1.into()))], ValueType::Int).into_bool(),
            None,
        );
        assert_eq!(
            ListExpr::value(vec![Expr::int(IntExpr::value(1.into()))], ValueType::Int).into_nil(),
            None,
        );
        assert_eq!(
            ListExpr::value(vec![Expr::int(IntExpr::value(1.into()))], ValueType::Int).into_tuple(),
            None,
        );
        assert_eq!(
            ListExpr::value(vec![Expr::int(IntExpr::value(1.into()))], ValueType::Int).into_list(),
            None,
        );
        assert_eq!(
            ListExpr::value(vec![Expr::int(IntExpr::value(1.into()))], ValueType::Int)
                .into_function(),
            None,
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
                    vec![(
                        BigInt::from(1),
                        ListExpr::value(Vec::new(), item_type.clone())
                    )],
                    ListExpr::value(Vec::new(), item_type.clone()),
                )
                .element_type(),
                item_type,
            );
            assert_eq!(
                ListExpr::string_case(
                    StringExpr::value("one".into()),
                    vec![("one".into(), ListExpr::value(Vec::new(), item_type.clone()))],
                    ListExpr::value(Vec::new(), item_type.clone()),
                )
                .element_type(),
                item_type,
            );
            assert_eq!(
                ListExpr::float_case(
                    FloatExpr::value(1.5),
                    vec![(1.5, ListExpr::value(Vec::new(), item_type.clone()))],
                    ListExpr::value(Vec::new(), item_type.clone()),
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
    fn bool_case_dispatch_preserves_every_item_type() {
        let function_type = FunctionType::new(Vec::new(), ValueType::Int);

        assert_eq!(
            ListExpr::bool_case(
                BoolExpr::value(true),
                BoolListCaseBranches::String {
                    true_: ListExpr::value(Vec::new(), ValueType::String)
                        .into_string()
                        .expect("string list"),
                    false_: ListExpr::value(Vec::new(), ValueType::String)
                        .into_string()
                        .expect("string list"),
                },
            )
            .element_type(),
            ValueType::String,
        );
        assert_eq!(
            ListExpr::bool_case(
                BoolExpr::value(true),
                BoolListCaseBranches::Float {
                    true_: ListExpr::value(Vec::new(), ValueType::Float)
                        .into_float()
                        .expect("float list"),
                    false_: ListExpr::value(Vec::new(), ValueType::Float)
                        .into_float()
                        .expect("float list"),
                },
            )
            .element_type(),
            ValueType::Float,
        );
        assert_eq!(
            ListExpr::bool_case(
                BoolExpr::value(true),
                BoolListCaseBranches::Bool {
                    true_: ListExpr::value(Vec::new(), ValueType::Bool)
                        .into_bool()
                        .expect("bool list"),
                    false_: ListExpr::value(Vec::new(), ValueType::Bool)
                        .into_bool()
                        .expect("bool list"),
                },
            )
            .element_type(),
            ValueType::Bool,
        );
        assert_eq!(
            ListExpr::bool_case(
                BoolExpr::value(true),
                BoolListCaseBranches::Nil {
                    true_: ListExpr::value(Vec::new(), ValueType::Nil)
                        .into_nil()
                        .expect("nil list"),
                    false_: ListExpr::value(Vec::new(), ValueType::Nil)
                        .into_nil()
                        .expect("nil list"),
                },
            )
            .element_type(),
            ValueType::Nil,
        );
        assert_eq!(
            ListExpr::bool_case(
                BoolExpr::value(true),
                BoolListCaseBranches::Tuple {
                    true_: ListExpr::value(Vec::new(), ValueType::Tuple(vec![ValueType::Int]))
                        .into_tuple()
                        .expect("tuple list"),
                    false_: ListExpr::value(Vec::new(), ValueType::Tuple(vec![ValueType::Int]))
                        .into_tuple()
                        .expect("tuple list"),
                },
            )
            .element_type(),
            ValueType::Tuple(vec![ValueType::Int]),
        );
        assert_eq!(
            ListExpr::bool_case(
                BoolExpr::value(true),
                BoolListCaseBranches::List {
                    true_: ListExpr::value(
                        Vec::new(),
                        ValueType::List(Box::new(ValueType::String)),
                    )
                    .into_list()
                    .expect("list list"),
                    false_: ListExpr::value(
                        Vec::new(),
                        ValueType::List(Box::new(ValueType::String)),
                    )
                    .into_list()
                    .expect("list list"),
                },
            )
            .element_type(),
            ValueType::List(Box::new(ValueType::String)),
        );
        assert_eq!(
            ListExpr::bool_case(
                BoolExpr::value(true),
                BoolListCaseBranches::Function {
                    true_: ListExpr::value(
                        Vec::new(),
                        ValueType::Function(Box::new(function_type.clone())),
                    )
                    .into_function()
                    .expect("function list"),
                    false_: ListExpr::value(
                        Vec::new(),
                        ValueType::Function(Box::new(function_type.clone())),
                    )
                    .into_function()
                    .expect("function list"),
                },
            )
            .element_type(),
            ValueType::Function(Box::new(function_type)),
        );
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

    #[test]
    fn metadata_helpers_preserve_tuple_list_and_function_item_types() {
        let tuple_item = TupleListItem {
            item_type: vec![ValueType::Int],
        };
        let list_item = ListListItem {
            item_type: Box::new(ValueType::String),
        };
        let function_type = FunctionType::new(Vec::new(), ValueType::Bool);
        let function_item = FunctionListItem {
            item_type: function_type.clone(),
        };

        assert_eq!(
            into_nil_clauses(vec![(
                BigInt::from(1),
                ListExpr::value(vec![Expr::nil(NilExpr::value())], ValueType::Nil),
            )]),
            vec![(
                BigInt::from(1),
                ListExpr::value(vec![Expr::nil(NilExpr::value())], ValueType::Nil)
                    .into_nil()
                    .expect("nil list"),
            )],
        );
        assert_eq!(
            into_tuple_clauses(
                vec![(
                    "tuple",
                    ListExpr::value(
                        vec![Expr::tuple(TupleExpr::value(
                            vec![Expr::int(IntExpr::value(1.into()))],
                            vec![ValueType::Int],
                        ))],
                        ValueType::Tuple(vec![ValueType::Int]),
                    ),
                )],
                &tuple_item,
            ),
            vec![(
                "tuple",
                ListExpr::value(
                    vec![Expr::tuple(TupleExpr::value(
                        vec![Expr::int(IntExpr::value(1.into()))],
                        vec![ValueType::Int],
                    ))],
                    ValueType::Tuple(vec![ValueType::Int]),
                )
                .into_tuple()
                .expect("tuple list"),
            )],
        );
        assert_eq!(
            into_list_clauses(
                vec![(
                    "list",
                    ListExpr::value(
                        vec![Expr::list(ListExpr::value(
                            vec![Expr::string(StringExpr::value("one".into()))],
                            ValueType::String,
                        ))],
                        ValueType::List(Box::new(ValueType::String)),
                    ),
                )],
                &list_item,
            ),
            vec![(
                "list",
                ListExpr::value(
                    vec![Expr::list(ListExpr::value(
                        vec![Expr::string(StringExpr::value("one".into()))],
                        ValueType::String,
                    ))],
                    ValueType::List(Box::new(ValueType::String)),
                )
                .into_list()
                .expect("list list"),
            )],
        );
        assert_eq!(
            into_function_clauses(
                vec![(
                    "function",
                    ListExpr::value(
                        vec![Expr::function(FunctionExpr::value(FunctionValue::new(
                            RuntimeFunctionId::Bool(crate::plan::BoolFunctionId(0)),
                            Vec::new(),
                        )))],
                        ValueType::Function(Box::new(FunctionType::new(
                            Vec::new(),
                            ValueType::Bool
                        ))),
                    ),
                )],
                &function_item,
            ),
            vec![(
                "function",
                ListExpr::value(
                    vec![Expr::function(FunctionExpr::value(FunctionValue::new(
                        RuntimeFunctionId::Bool(crate::plan::BoolFunctionId(0)),
                        Vec::new(),
                    )))],
                    ValueType::Function(Box::new(FunctionType::new(Vec::new(), ValueType::Bool))),
                )
                .into_function()
                .expect("function list"),
            )],
        );
    }
}
