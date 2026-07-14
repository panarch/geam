use super::{
    BitArrayListExpr, BoolListExpr, FloatListExpr, FunctionListExpr, IntListExpr, ListListExpr,
    NilListExpr, StringListExpr, TupleListExpr, UtfCodepointListExpr,
};
use crate::plan::execution::{
    BitArrayListLocalId, BoolListLocalId, FloatListLocalId, FunctionListLocalId, IntListLocalId,
    ListListLocalId, NilListLocalId, StringListLocalId, TupleListLocalId, UtfCodepointListLocalId,
};

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
        value: TupleListExpr,
    },
    List {
        local: ListListLocalId,
        value: ListListExpr,
    },
    Function {
        local: FunctionListLocalId,
        value: FunctionListExpr,
    },
}
