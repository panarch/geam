use super::ListExpr;
use crate::plan::execution::{
    BoolExpr, BoolListFunctionId, BoolListLocalId, BoolListTypeId, FloatExpr, FloatListFunctionId,
    FloatListLocalId, FloatListTypeId, FunctionExpr, FunctionListFunctionId, FunctionListLocalId,
    FunctionListTypeId, IntExpr, IntListFunctionId, IntListLocalId, IntListTypeId,
    ListListFunctionId, ListListLocalId, ListListTypeId, ListTypeId, NilExpr, NilListFunctionId,
    NilListLocalId, NilListTypeId, StringExpr, StringListFunctionId, StringListLocalId,
    StringListTypeId, TupleExpr, TupleListFunctionId, TupleListLocalId, TupleListTypeId,
};
pub(crate) trait ListItem {
    type ElementExpr;
    type Local: Clone;
    type Function: Clone;

    fn list_type(&self) -> ListTypeId;
}

pub(crate) struct IntListItem {
    type_id: IntListTypeId,
}

pub(crate) struct StringListItem {
    type_id: StringListTypeId,
}

pub(crate) struct FloatListItem {
    type_id: FloatListTypeId,
}

pub(crate) struct BoolListItem {
    type_id: BoolListTypeId,
}

pub(crate) struct NilListItem {
    type_id: NilListTypeId,
}

pub(crate) struct TupleListItem {
    type_id: TupleListTypeId,
}

pub(crate) struct ListListItem {
    type_id: ListListTypeId,
}

pub(crate) struct FunctionListItem {
    type_id: FunctionListTypeId,
}

macro_rules! list_item {
    ($item:ident, $type_id:ty) => {
        impl $item {
            pub(in crate::plan::execution) fn new(type_id: $type_id) -> Self {
                Self { type_id }
            }

            pub(crate) fn type_id(&self) -> $type_id {
                self.type_id
            }
        }
    };
}

list_item!(IntListItem, IntListTypeId);
list_item!(StringListItem, StringListTypeId);
list_item!(FloatListItem, FloatListTypeId);
list_item!(BoolListItem, BoolListTypeId);
list_item!(NilListItem, NilListTypeId);
list_item!(TupleListItem, TupleListTypeId);
list_item!(ListListItem, ListListTypeId);
list_item!(FunctionListItem, FunctionListTypeId);

macro_rules! primitive_list_item {
    (
        $item:ident,
        $expr:ty,
        $local:ty,
        $function:ty,
        $type_id:ty
    ) => {
        impl ListItem for $item {
            type ElementExpr = $expr;
            type Local = $local;
            type Function = $function;

            fn list_type(&self) -> ListTypeId {
                self.type_id.list_type()
            }
        }
    };
}

primitive_list_item!(
    IntListItem,
    IntExpr,
    IntListLocalId,
    IntListFunctionId,
    IntListTypeId
);

primitive_list_item!(
    StringListItem,
    StringExpr,
    StringListLocalId,
    StringListFunctionId,
    StringListTypeId
);

primitive_list_item!(
    FloatListItem,
    FloatExpr,
    FloatListLocalId,
    FloatListFunctionId,
    FloatListTypeId
);

primitive_list_item!(
    BoolListItem,
    BoolExpr,
    BoolListLocalId,
    BoolListFunctionId,
    BoolListTypeId
);

primitive_list_item!(
    NilListItem,
    NilExpr,
    NilListLocalId,
    NilListFunctionId,
    NilListTypeId
);

impl ListItem for TupleListItem {
    type ElementExpr = TupleExpr;
    type Local = TupleListLocalId;
    type Function = TupleListFunctionId;

    fn list_type(&self) -> ListTypeId {
        self.type_id.list_type()
    }
}

impl ListItem for ListListItem {
    type ElementExpr = ListExpr;
    type Local = ListListLocalId;
    type Function = ListListFunctionId;

    fn list_type(&self) -> ListTypeId {
        self.type_id.list_type()
    }
}

impl ListItem for FunctionListItem {
    type ElementExpr = FunctionExpr;
    type Local = FunctionListLocalId;
    type Function = FunctionListFunctionId;

    fn list_type(&self) -> ListTypeId {
        self.type_id.list_type()
    }
}
