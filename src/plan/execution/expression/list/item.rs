use super::ListExpr;
use crate::plan::execution::{
    BitArrayExpr, BitArrayListFunctionId, BitArrayListLocalId, BitArrayListTypeId, BoolExpr,
    BoolListFunctionId, BoolListLocalId, BoolListTypeId, FloatExpr, FloatListFunctionId,
    FloatListLocalId, FloatListTypeId, FunctionExpr, FunctionListFunctionId, FunctionListLocalId,
    FunctionListTypeId, IntExpr, IntListFunctionId, IntListLocalId, IntListTypeId,
    ListListFunctionId, ListListLocalId, ListListTypeId, ListTypeId, NilExpr, NilListFunctionId,
    NilListLocalId, NilListTypeId, StringExpr, StringListFunctionId, StringListLocalId,
    StringListTypeId, TupleExpr, TupleListFunctionId, TupleListLocalId, TupleListTypeId,
    UtfCodepointExpr, UtfCodepointListFunctionId, UtfCodepointListLocalId, UtfCodepointListTypeId,
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

pub(crate) struct BitArrayListItem {
    type_id: BitArrayListTypeId,
}

pub(crate) struct UtfCodepointListItem {
    type_id: UtfCodepointListTypeId,
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
list_item!(BitArrayListItem, BitArrayListTypeId);
list_item!(UtfCodepointListItem, UtfCodepointListTypeId);
list_item!(FloatListItem, FloatListTypeId);
list_item!(BoolListItem, BoolListTypeId);
list_item!(NilListItem, NilListTypeId);
list_item!(TupleListItem, TupleListTypeId);
list_item!(ListListItem, ListListTypeId);
list_item!(FunctionListItem, FunctionListTypeId);

impl ListItem for IntListItem {
    type ElementExpr = IntExpr;
    type Local = IntListLocalId;
    type Function = IntListFunctionId;

    fn list_type(&self) -> ListTypeId {
        self.type_id.list_type()
    }
}

impl ListItem for StringListItem {
    type ElementExpr = StringExpr;
    type Local = StringListLocalId;
    type Function = StringListFunctionId;

    fn list_type(&self) -> ListTypeId {
        self.type_id.list_type()
    }
}

impl ListItem for BitArrayListItem {
    type ElementExpr = BitArrayExpr;
    type Local = BitArrayListLocalId;
    type Function = BitArrayListFunctionId;

    fn list_type(&self) -> ListTypeId {
        self.type_id.list_type()
    }
}

impl ListItem for UtfCodepointListItem {
    type ElementExpr = UtfCodepointExpr;
    type Local = UtfCodepointListLocalId;
    type Function = UtfCodepointListFunctionId;

    fn list_type(&self) -> ListTypeId {
        self.type_id.list_type()
    }
}

impl ListItem for FloatListItem {
    type ElementExpr = FloatExpr;
    type Local = FloatListLocalId;
    type Function = FloatListFunctionId;

    fn list_type(&self) -> ListTypeId {
        self.type_id.list_type()
    }
}

impl ListItem for BoolListItem {
    type ElementExpr = BoolExpr;
    type Local = BoolListLocalId;
    type Function = BoolListFunctionId;

    fn list_type(&self) -> ListTypeId {
        self.type_id.list_type()
    }
}

impl ListItem for NilListItem {
    type ElementExpr = NilExpr;
    type Local = NilListLocalId;
    type Function = NilListFunctionId;

    fn list_type(&self) -> ListTypeId {
        self.type_id.list_type()
    }
}

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
