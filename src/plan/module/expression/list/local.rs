use super::{
    BitArrayListExpr, BoolListExpr, CustomListExpr, FloatListExpr, FunctionListExpr,
    GenericListExpr, IntListExpr, ListListExpr, NilListExpr, StringListExpr, TupleListExpr,
    UtfCodepointListExpr,
};
use crate::plan::{
    BitArrayListLocalId, BoolListLocalId, CustomListLocalId, CustomType, FloatListLocalId,
    FunctionListLocalId, FunctionType, GenericListLocalId, IntListLocalId, ListListLocalId,
    NilListLocalId, StringListLocalId, TupleListLocalId, TypeParameterId, UtfCodepointListLocalId,
    ValueType,
};

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum ListLocalExpr {
    Generic {
        local: GenericListLocalId,
        parameter: TypeParameterId,
        value: GenericListExpr,
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

impl ListLocalExpr {
    pub(crate) fn item_shape(&self) -> &crate::plan::ValueShape {
        match self {
            Self::Generic { value, .. } => value.item_shape(),
            Self::Int { value, .. } => value.item_shape(),
            Self::String { value, .. } => value.item_shape(),
            Self::BitArray { value, .. } => value.item_shape(),
            Self::UtfCodepoint { value, .. } => value.item_shape(),
            Self::Custom { value, .. } => value.item_shape(),
            Self::Float { value, .. } => value.item_shape(),
            Self::Bool { value, .. } => value.item_shape(),
            Self::Nil { value, .. } => value.item_shape(),
            Self::Tuple { value, .. } => value.item_shape(),
            Self::List { value, .. } => value.item_shape(),
            Self::Function { value, .. } => value.item_shape(),
        }
    }
}
