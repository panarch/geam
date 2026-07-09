use super::{
    BoolListExpr, BoolListItem, FloatListExpr, FloatListItem, FunctionListExpr, FunctionListItem,
    IntListExpr, IntListItem, ListExpr, ListItem, ListListExpr, ListListItem, NilListExpr,
    NilListItem, StringListExpr, StringListItem, TupleListExpr, TupleListItem,
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

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum ListSpreadElements {
    Int {
        values: Vec<IntExpr>,
        tail: IntListExpr,
    },
    String {
        values: Vec<StringExpr>,
        tail: StringListExpr,
    },
    Float {
        values: Vec<FloatExpr>,
        tail: FloatListExpr,
    },
    Bool {
        values: Vec<BoolExpr>,
        tail: BoolListExpr,
    },
    Nil {
        values: Vec<NilExpr>,
        tail: NilListExpr,
    },
    Tuple {
        item_type: Vec<ValueType>,
        values: Vec<TupleExpr>,
        tail: TupleListExpr,
    },
    List {
        item_type: Box<ValueType>,
        values: Vec<ListExpr>,
        tail: ListListExpr,
    },
    Function {
        item_type: FunctionType,
        values: Vec<FunctionExpr>,
        tail: FunctionListExpr,
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

impl ListSpreadElements {
    pub(crate) fn from_parts(
        elements: ListElements,
        tail: ListExpr,
    ) -> Result<Self, ListElementTypeMismatch> {
        let expected = elements.item_type();
        let actual = tail.element_type();

        match elements {
            ListElements::Int(values) => {
                let Some(tail) = tail.into_int() else {
                    return Err(ListElementTypeMismatch { expected, actual });
                };
                Ok(Self::Int { values, tail })
            }
            ListElements::String(values) => {
                let Some(tail) = tail.into_string() else {
                    return Err(ListElementTypeMismatch { expected, actual });
                };
                Ok(Self::String { values, tail })
            }
            ListElements::Float(values) => {
                let Some(tail) = tail.into_float() else {
                    return Err(ListElementTypeMismatch { expected, actual });
                };
                Ok(Self::Float { values, tail })
            }
            ListElements::Bool(values) => {
                let Some(tail) = tail.into_bool() else {
                    return Err(ListElementTypeMismatch { expected, actual });
                };
                Ok(Self::Bool { values, tail })
            }
            ListElements::Nil(values) => {
                let Some(tail) = tail.into_nil() else {
                    return Err(ListElementTypeMismatch { expected, actual });
                };
                Ok(Self::Nil { values, tail })
            }
            ListElements::Tuple { item_type, values } => {
                let Some(tail) = tail.into_tuple() else {
                    return Err(ListElementTypeMismatch { expected, actual });
                };
                if tail.element_type() != expected {
                    return Err(ListElementTypeMismatch {
                        expected,
                        actual: tail.element_type(),
                    });
                }
                Ok(Self::Tuple {
                    item_type,
                    values,
                    tail,
                })
            }
            ListElements::List { item_type, values } => {
                let Some(tail) = tail.into_list() else {
                    return Err(ListElementTypeMismatch { expected, actual });
                };
                if tail.element_type() != expected {
                    return Err(ListElementTypeMismatch {
                        expected,
                        actual: tail.element_type(),
                    });
                }
                Ok(Self::List {
                    item_type,
                    values,
                    tail,
                })
            }
            ListElements::Function { item_type, values } => {
                let Some(tail) = tail.into_function() else {
                    return Err(ListElementTypeMismatch { expected, actual });
                };
                if tail.element_type() != expected {
                    return Err(ListElementTypeMismatch {
                        expected,
                        actual: tail.element_type(),
                    });
                }
                Ok(Self::Function {
                    item_type,
                    values,
                    tail,
                })
            }
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
    use super::{ListElementTypeMismatch, ListElements, ListSpreadElements};
    use crate::plan::{
        BoolExpr, Expr, FloatExpr, FunctionExpr, FunctionType, FunctionValue, IntExpr, ListExpr,
        NilExpr, RuntimeFunctionId, StringExpr, TupleExpr, ValueType,
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

    #[test]
    fn spread_parts_reject_wrong_tail_family_and_nested_metadata() {
        assert_eq!(
            ListSpreadElements::from_parts(
                ListElements::String(vec![StringExpr::value("head".into())]),
                ListExpr::value(Vec::new(), ValueType::Int),
            ),
            Err(ListElementTypeMismatch {
                expected: ValueType::String,
                actual: ValueType::Int,
            }),
        );
        assert_eq!(
            ListSpreadElements::from_parts(
                ListElements::Float(vec![FloatExpr::value(1.5)]),
                ListExpr::value(Vec::new(), ValueType::Int),
            ),
            Err(ListElementTypeMismatch {
                expected: ValueType::Float,
                actual: ValueType::Int,
            }),
        );
        assert_eq!(
            ListSpreadElements::from_parts(
                ListElements::Bool(vec![BoolExpr::value(true)]),
                ListExpr::value(Vec::new(), ValueType::Int),
            ),
            Err(ListElementTypeMismatch {
                expected: ValueType::Bool,
                actual: ValueType::Int,
            }),
        );
        assert_eq!(
            ListSpreadElements::from_parts(
                ListElements::Nil(vec![NilExpr::value()]),
                ListExpr::value(Vec::new(), ValueType::Int),
            ),
            Err(ListElementTypeMismatch {
                expected: ValueType::Nil,
                actual: ValueType::Int,
            }),
        );
        assert_eq!(
            ListSpreadElements::from_parts(
                ListElements::Tuple {
                    item_type: vec![ValueType::Int],
                    values: Vec::new(),
                },
                ListExpr::value(Vec::new(), ValueType::String),
            ),
            Err(ListElementTypeMismatch {
                expected: ValueType::Tuple(vec![ValueType::Int]),
                actual: ValueType::String,
            }),
        );
        assert_eq!(
            ListSpreadElements::from_parts(
                ListElements::Tuple {
                    item_type: vec![ValueType::Int],
                    values: Vec::new(),
                },
                ListExpr::value(Vec::new(), ValueType::Tuple(vec![ValueType::String])),
            ),
            Err(ListElementTypeMismatch {
                expected: ValueType::Tuple(vec![ValueType::Int]),
                actual: ValueType::Tuple(vec![ValueType::String]),
            }),
        );
        assert_eq!(
            ListSpreadElements::from_parts(
                ListElements::List {
                    item_type: Box::new(ValueType::Int),
                    values: Vec::new(),
                },
                ListExpr::value(Vec::new(), ValueType::String),
            ),
            Err(ListElementTypeMismatch {
                expected: ValueType::List(Box::new(ValueType::Int)),
                actual: ValueType::String,
            }),
        );
        assert_eq!(
            ListSpreadElements::from_parts(
                ListElements::List {
                    item_type: Box::new(ValueType::Int),
                    values: Vec::new(),
                },
                ListExpr::value(Vec::new(), ValueType::List(Box::new(ValueType::String))),
            ),
            Err(ListElementTypeMismatch {
                expected: ValueType::List(Box::new(ValueType::Int)),
                actual: ValueType::List(Box::new(ValueType::String)),
            }),
        );

        let int_to_int = FunctionType::new(vec![ValueType::Int], ValueType::Int);
        let int_to_string = FunctionType::new(vec![ValueType::Int], ValueType::String);
        assert_eq!(
            ListSpreadElements::from_parts(
                ListElements::Function {
                    item_type: int_to_int.clone(),
                    values: Vec::new(),
                },
                ListExpr::value(Vec::new(), ValueType::String),
            ),
            Err(ListElementTypeMismatch {
                expected: ValueType::Function(Box::new(int_to_int.clone())),
                actual: ValueType::String,
            }),
        );
        assert_eq!(
            ListSpreadElements::from_parts(
                ListElements::Function {
                    item_type: int_to_int.clone(),
                    values: Vec::new(),
                },
                ListExpr::value(
                    Vec::new(),
                    ValueType::Function(Box::new(int_to_string.clone()))
                ),
            ),
            Err(ListElementTypeMismatch {
                expected: ValueType::Function(Box::new(int_to_int)),
                actual: ValueType::Function(Box::new(int_to_string)),
            }),
        );
    }
}
