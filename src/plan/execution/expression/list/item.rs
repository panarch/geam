use super::StoredListExpr;
use crate::plan::execution::{
    BitArrayExpr, BitArrayListFunctionId, BitArrayListLocalId, BitArrayListTypeId, BoolExpr,
    BoolListFunctionId, BoolListLocalId, BoolListTypeId, ConstantId, CustomExpr,
    CustomListFunctionId, CustomListLocalId, CustomListTypeId, FloatExpr, FloatListFunctionId,
    FloatListLocalId, FloatListTypeId, FunctionExpr, FunctionListFunctionId, FunctionListLocalId,
    FunctionListTypeId, IntExpr, IntListFunctionId, IntListLocalId, IntListTypeId,
    ListListFunctionId, ListListLocalId, ListListTypeId, ListTypeId, NilExpr, NilListFunctionId,
    NilListLocalId, NilListTypeId, ParameterListListFunctionId, ParameterListListLocalId,
    ParameterListListTypeId, ParameterListTypeId, StringExpr, StringListFunctionId,
    StringListLocalId, StringListTypeId, TupleExpr, TupleListFunctionId, TupleListLocalId,
    TupleListTypeId, UtfCodepointExpr, UtfCodepointListFunctionId, UtfCodepointListLocalId,
    UtfCodepointListTypeId,
};
pub(crate) trait ListItem {
    type ElementExpr;
    type Local: Clone;
    type Function: Clone;
    type IndexSource;
    type Constant: Copy;

    fn list_type(&self) -> ListTypeId;
}

pub(crate) struct IntListItem {
    type_id: IntListTypeId,
}

pub(crate) struct ParameterListItem {
    type_id: ParameterListTypeId,
}

pub(crate) struct ParameterListListItem {
    type_id: ParameterListListTypeId,
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

pub(crate) struct CustomListItem {
    type_id: CustomListTypeId,
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
list_item!(ParameterListItem, ParameterListTypeId);
list_item!(ParameterListListItem, ParameterListListTypeId);
list_item!(StringListItem, StringListTypeId);
list_item!(BitArrayListItem, BitArrayListTypeId);
list_item!(UtfCodepointListItem, UtfCodepointListTypeId);
list_item!(CustomListItem, CustomListTypeId);
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
    type IndexSource = super::ListListExpr;
    type Constant = ConstantId<super::IntListExpr>;

    fn list_type(&self) -> ListTypeId {
        self.type_id.list_type()
    }
}

impl ListItem for ParameterListListItem {
    type ElementExpr = super::ParameterListExpr;
    type Local = ParameterListListLocalId;
    type Function = ParameterListListFunctionId;
    type IndexSource = super::ListListExpr;
    type Constant = ConstantId<super::ParameterListListExpr>;

    fn list_type(&self) -> ListTypeId {
        self.type_id.list_type()
    }
}

impl ListItem for StringListItem {
    type ElementExpr = StringExpr;
    type Local = StringListLocalId;
    type Function = StringListFunctionId;
    type IndexSource = super::ListListExpr;
    type Constant = ConstantId<super::StringListExpr>;

    fn list_type(&self) -> ListTypeId {
        self.type_id.list_type()
    }
}

impl ListItem for BitArrayListItem {
    type ElementExpr = BitArrayExpr;
    type Local = BitArrayListLocalId;
    type Function = BitArrayListFunctionId;
    type IndexSource = super::ListListExpr;
    type Constant = ConstantId<super::BitArrayListExpr>;

    fn list_type(&self) -> ListTypeId {
        self.type_id.list_type()
    }
}

impl ListItem for UtfCodepointListItem {
    type ElementExpr = UtfCodepointExpr;
    type Local = UtfCodepointListLocalId;
    type Function = UtfCodepointListFunctionId;
    type IndexSource = super::ListListExpr;
    type Constant = ConstantId<super::UtfCodepointListExpr>;

    fn list_type(&self) -> ListTypeId {
        self.type_id.list_type()
    }
}

impl ListItem for CustomListItem {
    type ElementExpr = CustomExpr;
    type Local = CustomListLocalId;
    type Function = CustomListFunctionId;
    type IndexSource = super::ListListExpr;
    type Constant = ConstantId<super::CustomListExpr>;

    fn list_type(&self) -> ListTypeId {
        self.type_id.list_type()
    }
}

impl ListItem for FloatListItem {
    type ElementExpr = FloatExpr;
    type Local = FloatListLocalId;
    type Function = FloatListFunctionId;
    type IndexSource = super::ListListExpr;
    type Constant = ConstantId<super::FloatListExpr>;

    fn list_type(&self) -> ListTypeId {
        self.type_id.list_type()
    }
}

impl ListItem for BoolListItem {
    type ElementExpr = BoolExpr;
    type Local = BoolListLocalId;
    type Function = BoolListFunctionId;
    type IndexSource = super::ListListExpr;
    type Constant = ConstantId<super::BoolListExpr>;

    fn list_type(&self) -> ListTypeId {
        self.type_id.list_type()
    }
}

impl ListItem for NilListItem {
    type ElementExpr = NilExpr;
    type Local = NilListLocalId;
    type Function = NilListFunctionId;
    type IndexSource = super::ListListExpr;
    type Constant = ConstantId<super::NilListExpr>;

    fn list_type(&self) -> ListTypeId {
        self.type_id.list_type()
    }
}

impl ListItem for TupleListItem {
    type ElementExpr = TupleExpr;
    type Local = TupleListLocalId;
    type Function = TupleListFunctionId;
    type IndexSource = super::ListListExpr;
    type Constant = ConstantId<super::TupleListExpr>;

    fn list_type(&self) -> ListTypeId {
        self.type_id.list_type()
    }
}

impl ListItem for ListListItem {
    type ElementExpr = StoredListExpr;
    type Local = ListListLocalId;
    type Function = ListListFunctionId;
    type IndexSource = super::ListListExpr;
    type Constant = ConstantId<super::ListListExpr>;

    fn list_type(&self) -> ListTypeId {
        self.type_id.list_type()
    }
}

impl ListItem for FunctionListItem {
    type ElementExpr = FunctionExpr;
    type Local = FunctionListLocalId;
    type Function = FunctionListFunctionId;
    type IndexSource = super::ListListExpr;
    type Constant = ConstantId<super::FunctionListExpr>;

    fn list_type(&self) -> ListTypeId {
        self.type_id.list_type()
    }
}
