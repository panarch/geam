use super::{
    BoolListItem, FloatListItem, FunctionListItem, IntListItem, ListExpr, ListItem, ListListItem,
    NilListItem, StringListItem, TupleListItem,
};
use crate::plan::{
    BoolExpr, Expr, FloatExpr, FunctionExpr, FunctionType, IntExpr, NilExpr, StringExpr, TupleExpr,
    ValueType,
};

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum ListElements {
    Int(Vec<IntExpr>),
    String(Vec<StringExpr>),
    Float(Vec<FloatExpr>),
    Bool(Vec<BoolExpr>),
    Nil(Vec<NilExpr>),
    Tuple {
        item_type: Vec<ValueType>,
        values: Vec<TupleExpr>,
    },
    List {
        item_type: Box<ValueType>,
        values: Vec<ListExpr>,
    },
    Function {
        item_type: FunctionType,
        values: Vec<FunctionExpr>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ListElementTypeMismatch {
    pub(crate) expected: ValueType,
    pub(crate) actual: ValueType,
}

impl ListElements {
    pub(crate) fn from_exprs(
        item_type: ValueType,
        values: Vec<Expr>,
    ) -> Result<Self, ListElementTypeMismatch> {
        match item_type {
            ValueType::Int => list_elements_from_exprs(IntListItem, values),
            ValueType::String => list_elements_from_exprs(StringListItem, values),
            ValueType::Float => list_elements_from_exprs(FloatListItem, values),
            ValueType::Bool => list_elements_from_exprs(BoolListItem, values),
            ValueType::Nil => list_elements_from_exprs(NilListItem, values),
            ValueType::Tuple(item_type) => {
                list_elements_from_exprs(TupleListItem { item_type }, values)
            }
            ValueType::List(item_type) => {
                list_elements_from_exprs(ListListItem { item_type }, values)
            }
            ValueType::Function(item_type) => list_elements_from_exprs(
                FunctionListItem {
                    item_type: *item_type,
                },
                values,
            ),
        }
    }

    #[cfg(test)]
    pub(crate) fn item_type(&self) -> ValueType {
        match self {
            Self::Int(_) => ValueType::Int,
            Self::String(_) => ValueType::String,
            Self::Float(_) => ValueType::Float,
            Self::Bool(_) => ValueType::Bool,
            Self::Nil(_) => ValueType::Nil,
            Self::Tuple { item_type, .. } => ValueType::Tuple(item_type.clone()),
            Self::List { item_type, .. } => ValueType::List(item_type.clone()),
            Self::Function { item_type, .. } => ValueType::Function(Box::new(item_type.clone())),
        }
    }
}

fn list_elements_from_exprs<Item: ListItem>(
    item: Item,
    values: Vec<Expr>,
) -> Result<ListElements, ListElementTypeMismatch> {
    let elements = Item::elements_from_exprs(&item, values)?;
    Ok(Item::elements_to_facade(item, elements))
}

#[cfg(test)]
mod tests {
    use super::{ListElementTypeMismatch, ListElements};
    use crate::plan::{
        Expr, FunctionExpr, FunctionType, FunctionValue, IntExpr, ListExpr, RuntimeFunctionId,
        StringExpr, TupleExpr, ValueType,
    };

    #[test]
    fn from_exprs_rejects_wrong_item_family_and_nested_metadata() {
        assert_eq!(
            ListElements::from_exprs(
                ValueType::Int,
                vec![Expr::string(StringExpr::value("wrong".into()))],
            ),
            Err(ListElementTypeMismatch {
                expected: ValueType::Int,
                actual: ValueType::String,
            }),
        );

        let tuple = TupleExpr::value(
            vec![Expr::string(StringExpr::value("wrong".into()))],
            vec![ValueType::String],
        );
        assert_eq!(
            ListElements::from_exprs(
                ValueType::Tuple(vec![ValueType::Int]),
                vec![Expr::tuple(tuple)],
            ),
            Err(ListElementTypeMismatch {
                expected: ValueType::Tuple(vec![ValueType::Int]),
                actual: ValueType::Tuple(vec![ValueType::String]),
            }),
        );

        let nested = ListExpr::value(
            vec![Expr::string(StringExpr::value("wrong".into()))],
            ValueType::String,
        );
        assert_eq!(
            ListElements::from_exprs(
                ValueType::List(Box::new(ValueType::Int)),
                vec![Expr::list(nested)],
            ),
            Err(ListElementTypeMismatch {
                expected: ValueType::List(Box::new(ValueType::Int)),
                actual: ValueType::List(Box::new(ValueType::String)),
            }),
        );

        let function = FunctionExpr::value(FunctionValue::new(
            RuntimeFunctionId::Int(crate::plan::IntFunctionId(0)),
            Vec::new(),
        ));
        assert_eq!(
            ListElements::from_exprs(
                ValueType::Function(Box::new(FunctionType::new(Vec::new(), ValueType::String))),
                vec![Expr::function(function)],
            ),
            Err(ListElementTypeMismatch {
                expected: ValueType::Function(Box::new(FunctionType::new(
                    Vec::new(),
                    ValueType::String,
                ))),
                actual: ValueType::Function(Box::new(FunctionType::new(
                    Vec::new(),
                    ValueType::Int,
                ))),
            }),
        );
    }

    #[test]
    fn item_type_reports_typed_element_family() {
        assert_eq!(
            ListElements::Int(vec![IntExpr::value(1.into())]).item_type(),
            ValueType::Int,
        );
        assert_eq!(
            ListElements::Tuple {
                item_type: vec![ValueType::String],
                values: Vec::new(),
            }
            .item_type(),
            ValueType::Tuple(vec![ValueType::String]),
        );
        assert_eq!(
            ListElements::List {
                item_type: Box::new(ValueType::Bool),
                values: Vec::new(),
            }
            .item_type(),
            ValueType::List(Box::new(ValueType::Bool)),
        );
        assert_eq!(
            ListElements::Function {
                item_type: FunctionType::new(Vec::new(), ValueType::Nil),
                values: Vec::new(),
            }
            .item_type(),
            ValueType::Function(Box::new(FunctionType::new(Vec::new(), ValueType::Nil))),
        );
    }
}
