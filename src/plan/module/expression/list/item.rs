use super::{ListElementTypeMismatch, ListElements, ListExpr, TypedListExpr};
use crate::plan::{
    BitArrayExpr, BitArrayListFunctionId, BitArrayListLocalId, BoolExpr, BoolListFunctionId,
    BoolListLocalId, CustomExpr, CustomListFunctionId, CustomListLocalId, CustomType, Expr,
    ExprKind, FloatExpr, FloatListFunctionId, FloatListLocalId, FunctionExpr,
    FunctionListFunctionId, FunctionListLocalId, FunctionType, IntExpr, IntListFunctionId,
    IntListLocalId, ListListFunctionId, ListListLocalId, ListLocal, NilExpr, NilListFunctionId,
    NilListLocalId, StringExpr, StringListFunctionId, StringListLocalId, TupleExpr,
    TupleListFunctionId, TupleListLocalId, UtfCodepointExpr, UtfCodepointListFunctionId,
    UtfCodepointListLocalId, ValueType,
};
use std::fmt::Debug;

pub(crate) trait ListItem: Debug + Clone + PartialEq {
    type ElementExpr: Debug + Clone + PartialEq;
    type Local: Debug + Clone + PartialEq;
    type Function: Debug + Clone + PartialEq;

    fn value_type(&self) -> ValueType;

    fn local_to_facade(&self, local: Self::Local) -> ListLocal;

    fn elements_from_exprs(
        item: &Self,
        values: Vec<Expr>,
    ) -> Result<Vec<Self::ElementExpr>, ListElementTypeMismatch>;

    fn elements_to_facade(item: Self, values: Vec<Self::ElementExpr>) -> ListElements;

    fn expr_to_facade(expression: TypedListExpr<Self>) -> ListExpr
    where
        Self: Sized;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct IntListItem;

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
    pub(super) item_type: Box<ValueType>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct FunctionListItem {
    pub(super) item_type: FunctionType,
}

impl TupleListItem {
    #[cfg(test)]
    pub(crate) fn new(item_type: Vec<ValueType>) -> Self {
        Self { item_type }
    }

    pub(crate) fn item_type(&self) -> Vec<ValueType> {
        self.item_type.clone()
    }

    pub(crate) fn into_item_type(self) -> Vec<ValueType> {
        self.item_type
    }
}

impl ListListItem {
    #[cfg(test)]
    pub(crate) fn new(item_type: Box<ValueType>) -> Self {
        Self { item_type }
    }

    pub(crate) fn item_type(&self) -> Box<ValueType> {
        self.item_type.clone()
    }

    pub(crate) fn into_item_type(self) -> Box<ValueType> {
        self.item_type
    }
}

impl FunctionListItem {
    #[cfg(test)]
    pub(crate) fn new(item_type: FunctionType) -> Self {
        Self { item_type }
    }

    pub(crate) fn item_type(&self) -> FunctionType {
        self.item_type.clone()
    }

    pub(crate) fn into_item_type(self) -> FunctionType {
        self.item_type
    }
}

impl CustomListItem {
    pub(crate) fn item_type(&self) -> CustomType {
        self.item_type.clone()
    }

    pub(crate) fn into_item_type(self) -> CustomType {
        self.item_type
    }
}

macro_rules! primitive_list_item {
    (
        $item:ident,
        $expr:ty,
        $local:ty,
        $function:ty,
        $value_type:expr,
        $expr_pattern:pat => $expr_value:expr,
        $elements_variant:ident,
        $facade_variant:path,
        $local_ctor:path
    ) => {
        impl ListItem for $item {
            type ElementExpr = $expr;
            type Local = $local;
            type Function = $function;

            fn value_type(&self) -> ValueType {
                $value_type
            }

            fn local_to_facade(&self, local: Self::Local) -> ListLocal {
                $local_ctor(local)
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

            fn expr_to_facade(expression: TypedListExpr<Self>) -> ListExpr {
                $facade_variant(expression)
            }
        }
    };
}

primitive_list_item!(
    IntListItem,
    IntExpr,
    IntListLocalId,
    IntListFunctionId,
    ValueType::Int,
    ExprKind::Int(value) => value,
    Int,
    ListExpr::Int,
    ListLocal::Int
);

primitive_list_item!(
    StringListItem,
    StringExpr,
    StringListLocalId,
    StringListFunctionId,
    ValueType::String,
    ExprKind::String(value) => value,
    String,
    ListExpr::String,
    ListLocal::String
);

primitive_list_item!(
    BitArrayListItem,
    BitArrayExpr,
    BitArrayListLocalId,
    BitArrayListFunctionId,
    ValueType::BitArray,
    ExprKind::BitArray(value) => value,
    BitArray,
    ListExpr::BitArray,
    ListLocal::BitArray
);

primitive_list_item!(
    UtfCodepointListItem,
    UtfCodepointExpr,
    UtfCodepointListLocalId,
    UtfCodepointListFunctionId,
    ValueType::UtfCodepoint,
    ExprKind::UtfCodepoint(value) => value,
    UtfCodepoint,
    ListExpr::UtfCodepoint,
    ListLocal::UtfCodepoint
);

primitive_list_item!(
    FloatListItem,
    FloatExpr,
    FloatListLocalId,
    FloatListFunctionId,
    ValueType::Float,
    ExprKind::Float(value) => value,
    Float,
    ListExpr::Float,
    ListLocal::Float
);

primitive_list_item!(
    BoolListItem,
    BoolExpr,
    BoolListLocalId,
    BoolListFunctionId,
    ValueType::Bool,
    ExprKind::Bool(value) => value,
    Bool,
    ListExpr::Bool,
    ListLocal::Bool
);

primitive_list_item!(
    NilListItem,
    NilExpr,
    NilListLocalId,
    NilListFunctionId,
    ValueType::Nil,
    ExprKind::Nil(value) => value,
    Nil,
    ListExpr::Nil,
    ListLocal::Nil
);

impl ListItem for TupleListItem {
    type ElementExpr = TupleExpr;
    type Local = TupleListLocalId;
    type Function = TupleListFunctionId;

    fn value_type(&self) -> ValueType {
        ValueType::Tuple(self.item_type.clone())
    }

    fn local_to_facade(&self, local: Self::Local) -> ListLocal {
        ListLocal::tuple(local, self.item_type.clone())
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

    fn expr_to_facade(expression: TypedListExpr<Self>) -> ListExpr {
        ListExpr::Tuple(expression)
    }
}

impl ListItem for CustomListItem {
    type ElementExpr = CustomExpr;
    type Local = CustomListLocalId;
    type Function = CustomListFunctionId;

    fn value_type(&self) -> ValueType {
        ValueType::Custom(self.item_type.clone())
    }

    fn local_to_facade(&self, local: Self::Local) -> ListLocal {
        ListLocal::custom(local, self.item_type.clone())
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

    fn expr_to_facade(expression: TypedListExpr<Self>) -> ListExpr {
        ListExpr::Custom(expression)
    }
}

impl ListItem for ListListItem {
    type ElementExpr = ListExpr;
    type Local = ListListLocalId;
    type Function = ListListFunctionId;

    fn value_type(&self) -> ValueType {
        ValueType::List(self.item_type.clone())
    }

    fn local_to_facade(&self, local: Self::Local) -> ListLocal {
        ListLocal::list(local, self.item_type.as_ref().clone())
    }

    fn elements_from_exprs(
        item: &Self,
        values: Vec<Expr>,
    ) -> Result<Vec<Self::ElementExpr>, ListElementTypeMismatch> {
        values
            .into_iter()
            .map(|value| match value {
                Expr {
                    kind: ExprKind::List(value),
                    ..
                } if value.element_type() == item.item_type.as_ref().clone() => Ok(value),
                value => Err(ListElementTypeMismatch {
                    expected: item.value_type(),
                    actual: value.value_type(),
                }),
            })
            .collect()
    }

    fn elements_to_facade(item: Self, values: Vec<Self::ElementExpr>) -> ListElements {
        ListElements::List {
            item_type: item.item_type,
            values,
        }
    }

    fn expr_to_facade(expression: TypedListExpr<Self>) -> ListExpr {
        ListExpr::List(expression)
    }
}

impl ListItem for FunctionListItem {
    type ElementExpr = FunctionExpr;
    type Local = FunctionListLocalId;
    type Function = FunctionListFunctionId;

    fn value_type(&self) -> ValueType {
        ValueType::Function(Box::new(self.item_type.clone()))
    }

    fn local_to_facade(&self, local: Self::Local) -> ListLocal {
        ListLocal::function(local, self.item_type.clone())
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

    fn expr_to_facade(expression: TypedListExpr<Self>) -> ListExpr {
        ListExpr::Function(expression)
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

        let list_item = ListListItem::new(Box::new(ValueType::Bool));
        assert_eq!(list_item.item_type(), Box::new(ValueType::Bool));

        let function_type = FunctionType::new(vec![ValueType::Int], ValueType::String);
        let function_item = FunctionListItem::new(function_type.clone());
        assert_eq!(function_item.item_type(), function_type);
    }
}
