use super::{
    BitArrayListExpr, BoolListExpr, CustomListExpr, FloatListExpr, FunctionListExpr,
    GenericListExpr, IntListExpr, ListListExpr, NilListExpr, ParameterListListExpr, StringListExpr,
    TupleListExpr, UtfCodepointListExpr,
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
            Self::ParameterList { value, .. } => value.item_shape(),
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

#[cfg(test)]
mod tests {
    use super::ListLocalExpr;
    use crate::plan::{ListExpr, ListListLocalId, TypeParameterId, ValueShape, ValueType};

    #[test]
    fn parameter_list_local_preserves_recursive_item_shape() {
        let parameter = TypeParameterId(3);
        let value = ListExpr::try_value(
            Vec::new(),
            ValueType::List(Box::new(ValueType::Parameter(parameter))),
        )
        .expect("empty nested parameter list")
        .into_parameter_list()
        .expect("parameter-list item family");

        assert_eq!(
            ListLocalExpr::ParameterList {
                local: ListListLocalId(2),
                parameter,
                value,
            }
            .item_shape(),
            &ValueShape::List(Box::new(ValueShape::Parameter(parameter))),
        );
    }
}
