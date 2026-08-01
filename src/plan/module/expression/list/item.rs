use super::{
    GenericListExpr, ListElementTypeMismatch, ListElements, ListExpr, ListListExpr,
    ParameterListListExpr, StoredListExpr,
};
use crate::plan::{
    BitArrayExpr, BitArrayListLocalId, BoolExpr, BoolListLocalId,
    ConstantBitArrayListInstantiation, ConstantBoolListInstantiation,
    ConstantCustomListInstantiation, ConstantExternalListInstantiation,
    ConstantFloatListInstantiation, ConstantFunctionListInstantiation,
    ConstantGenericListInstantiation, ConstantIntListInstantiation, ConstantListListInstantiation,
    ConstantNilListInstantiation, ConstantParameterListListInstantiation,
    ConstantStringListInstantiation, ConstantTupleListInstantiation,
    ConstantUtfCodepointListInstantiation, CustomExpr, CustomListLocalId, CustomType, Expr,
    ExprKind, ExternalExpr, ExternalListLocalId, ExternalType, FloatExpr, FloatListLocalId,
    FunctionExpr, FunctionInstantiation, FunctionListLocalId, FunctionType, GenericExpr,
    GenericListLocalId, IntExpr, IntListLocalId, ListListLocalId, NilExpr, NilListLocalId,
    StringExpr, StringListLocalId, TupleExpr, TupleListLocalId, TypeParameterId, UtfCodepointExpr,
    UtfCodepointListLocalId, ValueStorageShape, ValueType,
};
use std::fmt::Debug;

pub(crate) trait ListItem: Debug + Clone + PartialEq {
    type ElementExpr: Debug + Clone + PartialEq;
    type IndexSource: Debug + Clone + PartialEq;
    type Local: Debug + Clone + PartialEq;
    type Function: Debug + Clone + PartialEq;
    type Constant: Debug + Clone + PartialEq;

    fn value_type(&self) -> ValueType;

    fn elements_from_exprs(
        item: &Self,
        values: Vec<Expr>,
    ) -> Result<Vec<Self::ElementExpr>, ListElementTypeMismatch>;

    fn elements_to_facade(item: Self, values: Vec<Self::ElementExpr>) -> ListElements;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct IntListItem;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct GenericListItem {
    parameter: TypeParameterId,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ParameterListListItem {
    parameter: TypeParameterId,
}

impl ParameterListListItem {
    pub(crate) fn new(parameter: TypeParameterId) -> Self {
        Self { parameter }
    }

    pub(crate) fn parameter(&self) -> TypeParameterId {
        self.parameter
    }
}

impl GenericListItem {
    pub(crate) fn new(parameter: TypeParameterId) -> Self {
        Self { parameter }
    }

    pub(crate) fn parameter(&self) -> TypeParameterId {
        self.parameter
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct StringListItem;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct BitArrayListItem;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct UtfCodepointListItem;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct CustomListItem {
    pub(super) item_type: CustomType,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ExternalListItem {
    pub(super) item_type: ExternalType,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct FloatListItem;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct BoolListItem;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct NilListItem;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct TupleListItem {
    pub(super) item_type: Vec<ValueType>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ListListItem {
    item_shape: ValueStorageShape,
}

#[derive(Debug, Clone, PartialEq, Eq)]
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
    pub(crate) fn new(item_shape: ValueStorageShape) -> Self {
        Self { item_shape }
    }

    pub(crate) fn item_type(&self) -> Box<ValueType> {
        Box::new(self.item_shape.value_type())
    }

    pub(crate) fn item_shape(&self) -> &ValueStorageShape {
        &self.item_shape
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

impl CustomListItem {
    pub(in crate::plan::module) fn new(item_type: CustomType) -> Self {
        Self { item_type }
    }

    pub(crate) fn item_type(&self) -> CustomType {
        self.item_type.clone()
    }
}

impl ExternalListItem {
    pub(in crate::plan::module) fn new(item_type: ExternalType) -> Self {
        Self { item_type }
    }

    pub(crate) fn item_type(&self) -> ExternalType {
        self.item_type.clone()
    }
}

impl ListItem for GenericListItem {
    type ElementExpr = GenericExpr;
    type IndexSource = ParameterListListExpr;
    type Local = GenericListLocalId;
    type Function = FunctionInstantiation;
    type Constant = ConstantGenericListInstantiation;

    fn value_type(&self) -> ValueType {
        ValueType::Parameter(self.parameter)
    }

    fn elements_from_exprs(
        item: &Self,
        values: Vec<Expr>,
    ) -> Result<Vec<Self::ElementExpr>, ListElementTypeMismatch> {
        values
            .into_iter()
            .map(|value| match value {
                Expr {
                    kind: ExprKind::Generic(value),
                    ..
                } if value.parameter() == item.parameter => Ok(value),
                value => Err(ListElementTypeMismatch {
                    expected: item.value_type(),
                    actual: value.value_type(),
                }),
            })
            .collect()
    }

    fn elements_to_facade(item: Self, values: Vec<Self::ElementExpr>) -> ListElements {
        ListElements::Generic {
            parameter: item.parameter,
            values,
        }
    }
}

impl ListItem for ParameterListListItem {
    type ElementExpr = GenericListExpr;
    type IndexSource = ListListExpr;
    type Local = ListListLocalId;
    type Function = FunctionInstantiation;
    type Constant = ConstantParameterListListInstantiation;

    fn value_type(&self) -> ValueType {
        ValueType::List(Box::new(ValueType::Parameter(self.parameter)))
    }

    fn elements_from_exprs(
        item: &Self,
        values: Vec<Expr>,
    ) -> Result<Vec<Self::ElementExpr>, ListElementTypeMismatch> {
        values
            .into_iter()
            .map(|value| match value {
                Expr {
                    kind: ExprKind::List(ListExpr::Generic(value)),
                    ..
                } if value.item().parameter() == item.parameter => Ok(value),
                value => Err(ListElementTypeMismatch {
                    expected: item.value_type(),
                    actual: value.value_type(),
                }),
            })
            .collect()
    }

    fn elements_to_facade(item: Self, values: Vec<Self::ElementExpr>) -> ListElements {
        ListElements::ParameterList {
            parameter: item.parameter,
            values,
        }
    }
}

macro_rules! primitive_list_item {
    (
        $item:ident,
        $expr:ty,
        $local:ty,
        $constant:ty,
        $value_type:expr,
        $expr_pattern:pat => $expr_value:expr,
        $elements_variant:ident
    ) => {
        impl ListItem for $item {
            type ElementExpr = $expr;
            type IndexSource = ListListExpr;
            type Local = $local;
            type Function = FunctionInstantiation;
            type Constant = $constant;

            fn value_type(&self) -> ValueType {
                $value_type
            }

            fn elements_from_exprs(
                _item: &Self,
                values: Vec<Expr>,
            ) -> Result<Vec<Self::ElementExpr>, ListElementTypeMismatch> {
                values
                    .into_iter()
                    .map(|value| match value {
                        Expr {
                            kind: $expr_pattern,
                            ..
                        } => Ok($expr_value),
                        value => Err(ListElementTypeMismatch {
                            expected: $value_type,
                            actual: value.value_type(),
                        }),
                    })
                    .collect()
            }

            fn elements_to_facade(_item: Self, values: Vec<Self::ElementExpr>) -> ListElements {
                ListElements::$elements_variant(values)
            }
        }
    };
}

primitive_list_item!(
    IntListItem,
    IntExpr,
    IntListLocalId,
    ConstantIntListInstantiation,
    ValueType::Int,
    ExprKind::Int(value) => value,
    Int
);

primitive_list_item!(
    StringListItem,
    StringExpr,
    StringListLocalId,
    ConstantStringListInstantiation,
    ValueType::String,
    ExprKind::String(value) => value,
    String
);

primitive_list_item!(
    BitArrayListItem,
    BitArrayExpr,
    BitArrayListLocalId,
    ConstantBitArrayListInstantiation,
    ValueType::BitArray,
    ExprKind::BitArray(value) => value,
    BitArray
);

primitive_list_item!(
    UtfCodepointListItem,
    UtfCodepointExpr,
    UtfCodepointListLocalId,
    ConstantUtfCodepointListInstantiation,
    ValueType::UtfCodepoint,
    ExprKind::UtfCodepoint(value) => value,
    UtfCodepoint
);

primitive_list_item!(
    FloatListItem,
    FloatExpr,
    FloatListLocalId,
    ConstantFloatListInstantiation,
    ValueType::Float,
    ExprKind::Float(value) => value,
    Float
);

primitive_list_item!(
    BoolListItem,
    BoolExpr,
    BoolListLocalId,
    ConstantBoolListInstantiation,
    ValueType::Bool,
    ExprKind::Bool(value) => value,
    Bool
);

primitive_list_item!(
    NilListItem,
    NilExpr,
    NilListLocalId,
    ConstantNilListInstantiation,
    ValueType::Nil,
    ExprKind::Nil(value) => value,
    Nil
);

impl ListItem for TupleListItem {
    type ElementExpr = TupleExpr;
    type IndexSource = ListListExpr;
    type Local = TupleListLocalId;
    type Function = FunctionInstantiation;
    type Constant = ConstantTupleListInstantiation;

    fn value_type(&self) -> ValueType {
        ValueType::Tuple(self.item_type.clone())
    }

    fn elements_from_exprs(
        item: &Self,
        values: Vec<Expr>,
    ) -> Result<Vec<Self::ElementExpr>, ListElementTypeMismatch> {
        values
            .into_iter()
            .map(|value| match value {
                Expr {
                    kind: ExprKind::Tuple(value),
                    ..
                } if value.type_() == item.item_type.as_slice() => Ok(value),
                value => Err(ListElementTypeMismatch {
                    expected: item.value_type(),
                    actual: value.value_type(),
                }),
            })
            .collect()
    }

    fn elements_to_facade(item: Self, values: Vec<Self::ElementExpr>) -> ListElements {
        ListElements::Tuple {
            item_type: item.item_type,
            values,
        }
    }
}

impl ListItem for CustomListItem {
    type ElementExpr = CustomExpr;
    type IndexSource = ListListExpr;
    type Local = CustomListLocalId;
    type Function = FunctionInstantiation;
    type Constant = ConstantCustomListInstantiation;

    fn value_type(&self) -> ValueType {
        ValueType::Custom(self.item_type.clone())
    }

    fn elements_from_exprs(
        item: &Self,
        values: Vec<Expr>,
    ) -> Result<Vec<Self::ElementExpr>, ListElementTypeMismatch> {
        values
            .into_iter()
            .map(|value| match value {
                Expr {
                    kind: ExprKind::Custom(value),
                    ..
                } if value.type_() == &item.item_type => Ok(value),
                value => Err(ListElementTypeMismatch {
                    expected: item.value_type(),
                    actual: value.value_type(),
                }),
            })
            .collect()
    }

    fn elements_to_facade(item: Self, values: Vec<Self::ElementExpr>) -> ListElements {
        ListElements::Custom {
            item_type: item.item_type,
            values,
        }
    }
}

impl ListItem for ExternalListItem {
    type ElementExpr = ExternalExpr;
    type IndexSource = ListListExpr;
    type Local = ExternalListLocalId;
    type Function = FunctionInstantiation;
    type Constant = ConstantExternalListInstantiation;

    fn value_type(&self) -> ValueType {
        ValueType::External(self.item_type.clone())
    }

    fn elements_from_exprs(
        item: &Self,
        values: Vec<Expr>,
    ) -> Result<Vec<Self::ElementExpr>, ListElementTypeMismatch> {
        values
            .into_iter()
            .map(|value| match value {
                Expr {
                    kind: ExprKind::External(value),
                    ..
                } if value.type_() == &item.item_type => Ok(value),
                value => Err(ListElementTypeMismatch {
                    expected: item.value_type(),
                    actual: value.value_type(),
                }),
            })
            .collect()
    }

    fn elements_to_facade(item: Self, values: Vec<Self::ElementExpr>) -> ListElements {
        ListElements::External {
            item_type: item.item_type,
            values,
        }
    }
}

impl ListItem for ListListItem {
    type ElementExpr = StoredListExpr;
    type IndexSource = ListListExpr;
    type Local = ListListLocalId;
    type Function = FunctionInstantiation;
    type Constant = ConstantListListInstantiation;

    fn value_type(&self) -> ValueType {
        ValueType::List(self.item_type())
    }

    fn elements_from_exprs(
        item: &Self,
        values: Vec<Expr>,
    ) -> Result<Vec<Self::ElementExpr>, ListElementTypeMismatch> {
        values
            .into_iter()
            .map(|value| {
                let actual = value.value_type();
                match value {
                    Expr {
                        kind: ExprKind::List(value),
                        ..
                    } if value.element_type() == item.item_shape.value_type() => {
                        StoredListExpr::try_from_facade(value).ok_or(ListElementTypeMismatch {
                            expected: item.value_type(),
                            actual,
                        })
                    }
                    _ => Err(ListElementTypeMismatch {
                        expected: item.value_type(),
                        actual,
                    }),
                }
            })
            .collect()
    }

    fn elements_to_facade(item: Self, values: Vec<Self::ElementExpr>) -> ListElements {
        ListElements::List {
            item_shape: item.item_shape,
            values,
        }
    }
}

impl ListItem for FunctionListItem {
    type ElementExpr = FunctionExpr;
    type IndexSource = ListListExpr;
    type Local = FunctionListLocalId;
    type Function = FunctionInstantiation;
    type Constant = ConstantFunctionListInstantiation;

    fn value_type(&self) -> ValueType {
        ValueType::Function(Box::new(self.item_type.clone()))
    }

    fn elements_from_exprs(
        item: &Self,
        values: Vec<Expr>,
    ) -> Result<Vec<Self::ElementExpr>, ListElementTypeMismatch> {
        values
            .into_iter()
            .map(|value| match value {
                Expr {
                    kind: ExprKind::Function(value),
                    ..
                } if value.type_() == item.item_type => Ok(value),
                value => Err(ListElementTypeMismatch {
                    expected: item.value_type(),
                    actual: value.value_type(),
                }),
            })
            .collect()
    }

    fn elements_to_facade(item: Self, values: Vec<Self::ElementExpr>) -> ListElements {
        ListElements::Function {
            item_type: item.item_type,
            values,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{FunctionListItem, ListListItem, TupleListItem};
    use crate::plan::{FunctionType, ValueType};

    #[test]
    fn metadata_items_preserve_nested_item_types() {
        let tuple_item = TupleListItem::new(vec![ValueType::Int, ValueType::String]);
        assert_eq!(
            tuple_item.item_type(),
            vec![ValueType::Int, ValueType::String],
        );

        let list_item = ListListItem::new(crate::plan::ValueStorageShape::Bool);
        assert_eq!(list_item.item_type(), Box::new(ValueType::Bool));

        let function_type = FunctionType::new(vec![ValueType::Int], ValueType::String);
        let function_item = FunctionListItem::new(function_type.clone());
        assert_eq!(function_item.item_type(), function_type);
    }
}
