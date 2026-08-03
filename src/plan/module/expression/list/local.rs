use super::{
    BitArrayListExpr, BoolListExpr, CustomListExpr, ExternalListExpr, FloatListExpr,
    FunctionListExpr, GenericListExpr, IntListExpr, ListListExpr, NilListExpr,
    ParameterListListExpr, StringListExpr, TupleListExpr, UtfCodepointListExpr,
};
use crate::plan::{
    BitArrayListLocalId, BoolListLocalId, CustomListLocalId, CustomType, ExternalListLocalId,
    ExternalType, FloatListLocalId, FunctionListLocalId, FunctionType, GenericListLocalId,
    IntListLocalId, ListListLocalId, NilListLocalId, StringListLocalId, TupleListLocalId,
    TypeParameterId, UtfCodepointListLocalId, ValueType,
};

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum ListLocalExpr {
    Generic {
        local: GenericListLocalId,
        parameter: TypeParameterId,
        value: GenericListExpr,
    },
    ParameterList {
        local: ListListLocalId,
        parameter: TypeParameterId,
        value: ParameterListListExpr,
    },
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
    External {
        local: ExternalListLocalId,
        item_type: ExternalType,
        value: ExternalListExpr,
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
