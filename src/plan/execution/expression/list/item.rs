use super::ListExpr;
use crate::plan::execution::{
    BoolExpr, BoolListFunctionId, BoolListLocalId, FloatExpr, FloatListFunctionId,
    FloatListLocalId, FunctionExpr, FunctionListFunctionId, FunctionListLocalId, IntExpr,
    IntListFunctionId, IntListLocalId, ListListFunctionId, ListListLocalId, NilExpr,
    NilListFunctionId, NilListLocalId, StringExpr, StringListFunctionId, StringListLocalId,
    TupleExpr, TupleListFunctionId, TupleListLocalId,
};
use crate::plan::{FunctionType, ValueType};
pub(crate) trait ListItem {
    type ElementExpr;
    type Local: Clone;
    type Function: Clone;

    fn value_type(&self) -> ValueType;
}

pub(crate) struct IntListItem;

pub(crate) struct StringListItem;

pub(crate) struct FloatListItem;

pub(crate) struct BoolListItem;

pub(crate) struct NilListItem;

pub(crate) struct TupleListItem {
    pub(super) item_type: Vec<ValueType>,
}

pub(crate) struct ListListItem {
    pub(super) item_type: Box<ValueType>,
}

pub(crate) struct FunctionListItem {
    pub(super) item_type: FunctionType,
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

macro_rules! primitive_list_item {
    (
        $item:ident,
        $expr:ty,
        $local:ty,
        $function:ty,
        $value_type:expr
    ) => {
        impl ListItem for $item {
            type ElementExpr = $expr;
            type Local = $local;
            type Function = $function;

            fn value_type(&self) -> ValueType {
                $value_type
            }
        }
    };
}

primitive_list_item!(
    IntListItem,
    IntExpr,
    IntListLocalId,
    IntListFunctionId,
    ValueType::Int
);

primitive_list_item!(
    StringListItem,
    StringExpr,
    StringListLocalId,
    StringListFunctionId,
    ValueType::String
);

primitive_list_item!(
    FloatListItem,
    FloatExpr,
    FloatListLocalId,
    FloatListFunctionId,
    ValueType::Float
);

primitive_list_item!(
    BoolListItem,
    BoolExpr,
    BoolListLocalId,
    BoolListFunctionId,
    ValueType::Bool
);

primitive_list_item!(
    NilListItem,
    NilExpr,
    NilListLocalId,
    NilListFunctionId,
    ValueType::Nil
);

impl ListItem for TupleListItem {
    type ElementExpr = TupleExpr;
    type Local = TupleListLocalId;
    type Function = TupleListFunctionId;

    fn value_type(&self) -> ValueType {
        ValueType::Tuple(self.item_type.clone())
    }
}

impl ListItem for ListListItem {
    type ElementExpr = ListExpr;
    type Local = ListListLocalId;
    type Function = ListListFunctionId;

    fn value_type(&self) -> ValueType {
        ValueType::List(self.item_type.clone())
    }
}

impl ListItem for FunctionListItem {
    type ElementExpr = FunctionExpr;
    type Local = FunctionListLocalId;
    type Function = FunctionListFunctionId;

    fn value_type(&self) -> ValueType {
        ValueType::Function(Box::new(self.item_type.clone()))
    }
}
