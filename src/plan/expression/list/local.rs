use super::{
    BoolListExpr, FloatListExpr, FunctionListExpr, IntListExpr, ListListExpr, NilListExpr,
    StringListExpr, TupleListExpr,
};
use crate::plan::{
    BoolListLocalId, FloatListLocalId, FunctionListLocalId, FunctionType, IntListLocalId,
    ListListLocalId, NilListLocalId, StringListLocalId, TupleListLocalId, ValueType,
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
