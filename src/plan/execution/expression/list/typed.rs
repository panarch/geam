use super::{ListItem, ListListExpr};
use crate::plan::execution::{
    BoolExpr, CallArg, CustomFieldAccess, FloatExpr, IntExpr, ListFunctionExpr, PanicExpr, Step,
    StringExpr, TupleExpr,
};
use ecow::EcoString;
use num_bigint::BigInt;
use std::marker::PhantomData;

pub(crate) struct TypedListExpr<Item: ListItem> {
    pub(super) item: Item,
    pub(super) kind: TypedListExprKind<Item>,
}

pub(crate) struct ListIndexSource<Item: ListItem> {
    list: Box<ListListExpr>,
    index: usize,
    result_item: PhantomData<fn() -> Item>,
}

pub(crate) enum TypedListExprKind<Item: ListItem> {
    Value(Vec<Item::ElementExpr>),
    Spread {
        elements: Vec<Item::ElementExpr>,
        tail: Box<TypedListExprKind<Item>>,
    },
    LocalGet {
        local: Item::Local,
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
    CustomField(CustomFieldAccess),
    ListIndex(ListIndexSource<Item>),
    DropFirst {
        list: Box<TypedListExprKind<Item>>,
        count: usize,
    },
    Panic(PanicExpr),
    BoolCase {
        subject: Box<BoolExpr>,
        true_: Box<TypedListExprKind<Item>>,
        false_: Box<TypedListExprKind<Item>>,
    },
    IntCase {
        subject: Box<IntExpr>,
        clauses: Vec<(BigInt, TypedListExprKind<Item>)>,
        fallback: Box<TypedListExprKind<Item>>,
    },
    StringCase {
        subject: Box<StringExpr>,
        clauses: Vec<(EcoString, TypedListExprKind<Item>)>,
        fallback: Box<TypedListExprKind<Item>>,
    },
    FloatCase {
        subject: Box<FloatExpr>,
        clauses: Vec<(f64, TypedListExprKind<Item>)>,
        fallback: Box<TypedListExprKind<Item>>,
    },
    Block {
        steps: Vec<Step>,
        return_: Box<TypedListExprKind<Item>>,
    },
}

impl<Item: ListItem> ListIndexSource<Item> {
    pub(in crate::plan::execution) fn from_parts(list: ListListExpr, index: usize) -> Self {
        Self::new(list, index)
    }

    pub(super) fn new(list: ListListExpr, index: usize) -> Self {
        Self {
            list: Box::new(list),
            index,
            result_item: PhantomData,
        }
    }

    pub(crate) fn list(&self) -> &ListListExpr {
        &self.list
    }

    pub(crate) fn index(&self) -> usize {
        self.index
    }
}

impl<Item: ListItem> TypedListExpr<Item> {
    pub(crate) fn item(&self) -> &Item {
        &self.item
    }

    pub(crate) fn kind(&self) -> &TypedListExprKind<Item> {
        &self.kind
    }

    pub(in crate::plan::execution) fn from_item_and_kind(
        item: Item,
        kind: TypedListExprKind<Item>,
    ) -> Self {
        Self { item, kind }
    }
}
