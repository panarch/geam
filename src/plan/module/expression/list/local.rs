use super::{
    BitArrayListExpr, BoolListExpr, CustomListExpr, FloatListExpr, FunctionListExpr, IntListExpr,
    ListListExpr, NilListExpr, StringListExpr, TupleListExpr, UtfCodepointListExpr,
};
use crate::plan::{
    BitArrayListLocalId, BoolListLocalId, CustomListLocalId, CustomType, FloatListLocalId,
    FunctionListLocalId, FunctionType, IntListLocalId, ListListLocalId, NilListLocalId,
    StringListLocalId, TupleListLocalId, UtfCodepointListLocalId, ValueType,
};

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
    BitArray {
        local: BitArrayListLocalId,
        value: BitArrayListExpr,
    },
    UtfCodepoint {
        local: UtfCodepointListLocalId,
        value: UtfCodepointListExpr,
    },
    Custom {
        local: CustomListLocalId,
        item_type: CustomType,
        value: CustomListExpr,
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
