use super::{
    BitArrayListExpr, BitArrayListItem, BoolListExpr, BoolListItem, CustomListExpr, CustomListItem,
    ExternalListExpr, ExternalListItem, FloatListExpr, FloatListItem, FunctionListExpr,
    FunctionListItem, GenericListExpr, GenericListItem, IntListExpr, IntListItem, ListExpr,
    ListItem, ListListExpr, ListListItem, NilListExpr, NilListItem, ParameterListListExpr,
    ParameterListListItem, StoredListExpr, StringListExpr, StringListItem, TupleListExpr,
    TupleListItem, UtfCodepointListExpr, UtfCodepointListItem,
};
use crate::plan::{
    BitArrayExpr, BoolExpr, CustomExpr, CustomType, Expr, ExternalExpr, ExternalType, FloatExpr,
    FunctionExpr, FunctionType, GenericExpr, IntExpr, NilExpr, StringExpr, TupleExpr,
    TypeParameterId, UtfCodepointExpr, ValueRepresentation, ValueShape, ValueStorageShape,
    ValueType,
};
use vec1::Vec1;

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum ListElements {
    Generic {
        parameter: TypeParameterId,
        values: Vec<GenericExpr>,
    },
    ParameterList {
        parameter: TypeParameterId,
        values: Vec<GenericListExpr>,
    },
    Int(Vec<IntExpr>),
    String(Vec<StringExpr>),
    BitArray(Vec<BitArrayExpr>),
    UtfCodepoint(Vec<UtfCodepointExpr>),
    Custom {
        item_type: CustomType,
        values: Vec<CustomExpr>,
    },
    External {
        item_type: ExternalType,
        values: Vec<ExternalExpr>,
    },
    Float(Vec<FloatExpr>),
    Bool(Vec<BoolExpr>),
    Nil(Vec<NilExpr>),
    Tuple {
        item_type: Vec<ValueType>,
        values: Vec<TupleExpr>,
    },
    List {
        item_shape: ValueStorageShape,
        values: Vec<StoredListExpr>,
    },
    Function {
        item_type: FunctionType,
        values: Vec<FunctionExpr>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum ListSpreadElements {
    Generic {
        values: Vec1<GenericExpr>,
        tail: GenericListExpr,
    },
    ParameterList {
        values: Vec1<GenericListExpr>,
        tail: ParameterListListExpr,
    },
    Int {
        values: Vec1<IntExpr>,
        tail: IntListExpr,
    },
    String {
        values: Vec1<StringExpr>,
        tail: StringListExpr,
    },
    BitArray {
        values: Vec1<BitArrayExpr>,
        tail: BitArrayListExpr,
    },
    UtfCodepoint {
        values: Vec1<UtfCodepointExpr>,
        tail: UtfCodepointListExpr,
    },
    Custom {
        values: Vec1<CustomExpr>,
        tail: CustomListExpr,
    },
    External {
        values: Vec1<ExternalExpr>,
        tail: ExternalListExpr,
    },
    Float {
        values: Vec1<FloatExpr>,
        tail: FloatListExpr,
    },
    Bool {
        values: Vec1<BoolExpr>,
        tail: BoolListExpr,
    },
    Nil {
        values: Vec1<NilExpr>,
        tail: NilListExpr,
    },
    Tuple {
        values: Vec1<TupleExpr>,
        tail: TupleListExpr,
    },
    List {
        values: Vec1<StoredListExpr>,
        tail: ListListExpr,
    },
    Function {
        values: Vec1<FunctionExpr>,
        tail: FunctionListExpr,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ListElementTypeMismatch {
    pub(crate) expected: ValueType,
    pub(crate) actual: ValueType,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum ListSpreadConstructionError {
    EmptyPrefix,
    ElementTypeMismatch(ListElementTypeMismatch),
}

impl ListElements {
    pub(crate) fn from_exprs(
        item_type: ValueType,
        values: Vec<Expr>,
    ) -> Result<Self, ListElementTypeMismatch> {
        match item_type {
            ValueType::Parameter(parameter) => {
                list_elements_from_exprs(GenericListItem::new(parameter), values)
            }
            ValueType::Int => list_elements_from_exprs(IntListItem, values),
            ValueType::String => list_elements_from_exprs(StringListItem, values),
            ValueType::BitArray => list_elements_from_exprs(BitArrayListItem, values),
            ValueType::UtfCodepoint => list_elements_from_exprs(UtfCodepointListItem, values),
            ValueType::Custom(item_type) => {
                list_elements_from_exprs(CustomListItem { item_type }, values)
            }
            ValueType::External(item_type) => {
                list_elements_from_exprs(ExternalListItem { item_type }, values)
            }
            ValueType::Float => list_elements_from_exprs(FloatListItem, values),
            ValueType::Bool => list_elements_from_exprs(BoolListItem, values),
            ValueType::Nil => list_elements_from_exprs(NilListItem, values),
            ValueType::Tuple(item_type) => {
                list_elements_from_exprs(TupleListItem { item_type }, values)
            }
            ValueType::List(item_type) => {
                match ValueShape::from_value_type(*item_type).representation() {
                    ValueRepresentation::Uninhabited(parameter) => {
                        list_elements_from_exprs(ParameterListListItem::new(parameter), values)
                    }
                    ValueRepresentation::Stored(item_shape) => {
                        list_elements_from_exprs(ListListItem::new(item_shape), values)
                    }
                }
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
            Self::Generic { parameter, .. } => ValueType::Parameter(*parameter),
            Self::ParameterList { parameter, .. } => {
                ValueType::List(Box::new(ValueType::Parameter(*parameter)))
            }
            Self::Int(_) => ValueType::Int,
            Self::String(_) => ValueType::String,
            Self::BitArray(_) => ValueType::BitArray,
            Self::UtfCodepoint(_) => ValueType::UtfCodepoint,
            Self::Custom { item_type, .. } => ValueType::Custom(item_type.clone()),
            Self::External { item_type, .. } => ValueType::External(item_type.clone()),
            Self::Float(_) => ValueType::Float,
            Self::Bool(_) => ValueType::Bool,
            Self::Nil(_) => ValueType::Nil,
            Self::Tuple { item_type, .. } => ValueType::Tuple(item_type.clone()),
            Self::List { item_shape, .. } => ValueType::List(Box::new(item_shape.value_type())),
            Self::Function { item_type, .. } => ValueType::Function(Box::new(item_type.clone())),
        }
    }
}

impl ListSpreadElements {
    pub(crate) fn from_parts(
        elements: ListElements,
        tail: ListExpr,
    ) -> Result<Self, ListSpreadConstructionError> {
        let expected = elements.item_type();
        let actual = tail.element_type();

        match elements {
            ListElements::Generic {
                parameter: _,
                values,
            } => {
                let Some(tail) = tail.into_generic() else {
                    return Err(spread_element_type_mismatch(expected, actual));
                };
                if tail.element_type() != expected {
                    return Err(spread_element_type_mismatch(expected, tail.element_type()));
                }
                let values = non_empty_spread_values(values)?;
                Ok(Self::Generic { values, tail })
            }
            ListElements::ParameterList {
                parameter: _,
                values,
            } => {
                let Some(tail) = tail.into_parameter_list() else {
                    return Err(spread_element_type_mismatch(expected, actual));
                };
                if tail.element_type() != expected {
                    return Err(spread_element_type_mismatch(expected, tail.element_type()));
                }
                let values = non_empty_spread_values(values)?;
                Ok(Self::ParameterList { values, tail })
            }
            ListElements::Int(values) => {
                let Some(tail) = tail.into_int() else {
                    return Err(spread_element_type_mismatch(expected, actual));
                };
                let values = non_empty_spread_values(values)?;
                Ok(Self::Int { values, tail })
            }
            ListElements::String(values) => {
                let Some(tail) = tail.into_string() else {
                    return Err(spread_element_type_mismatch(expected, actual));
                };
                let values = non_empty_spread_values(values)?;
                Ok(Self::String { values, tail })
            }
            ListElements::BitArray(values) => {
                let Some(tail) = tail.into_bit_array() else {
                    return Err(spread_element_type_mismatch(expected, actual));
                };
                let values = non_empty_spread_values(values)?;
                Ok(Self::BitArray { values, tail })
            }
            ListElements::UtfCodepoint(values) => {
                let Some(tail) = tail.into_utf_codepoint() else {
                    return Err(spread_element_type_mismatch(expected, actual));
                };
                let values = non_empty_spread_values(values)?;
                Ok(Self::UtfCodepoint { values, tail })
            }
            ListElements::Custom {
                item_type: _,
                values,
            } => {
                let Some(tail) = tail.into_custom() else {
                    return Err(spread_element_type_mismatch(expected, actual));
                };
                if tail.element_type() != expected {
                    return Err(spread_element_type_mismatch(expected, tail.element_type()));
                }
                let values = non_empty_spread_values(values)?;
                Ok(Self::Custom { values, tail })
            }
            ListElements::External {
                item_type: _,
                values,
            } => {
                let Some(tail) = tail.into_external() else {
                    return Err(spread_element_type_mismatch(expected, actual));
                };
                if tail.element_type() != expected {
                    return Err(spread_element_type_mismatch(expected, tail.element_type()));
                }
                let values = non_empty_spread_values(values)?;
                Ok(Self::External { values, tail })
            }
            ListElements::Float(values) => {
                let Some(tail) = tail.into_float() else {
                    return Err(spread_element_type_mismatch(expected, actual));
                };
                let values = non_empty_spread_values(values)?;
                Ok(Self::Float { values, tail })
            }
            ListElements::Bool(values) => {
                let Some(tail) = tail.into_bool() else {
                    return Err(spread_element_type_mismatch(expected, actual));
                };
                let values = non_empty_spread_values(values)?;
                Ok(Self::Bool { values, tail })
            }
            ListElements::Nil(values) => {
                let Some(tail) = tail.into_nil() else {
                    return Err(spread_element_type_mismatch(expected, actual));
                };
                let values = non_empty_spread_values(values)?;
                Ok(Self::Nil { values, tail })
            }
            ListElements::Tuple {
                item_type: _,
                values,
            } => {
                let Some(tail) = tail.into_tuple() else {
                    return Err(spread_element_type_mismatch(expected, actual));
                };
                if tail.element_type() != expected {
                    return Err(spread_element_type_mismatch(expected, tail.element_type()));
                }
                let values = non_empty_spread_values(values)?;
                Ok(Self::Tuple { values, tail })
            }
            ListElements::List {
                item_shape: _,
                values,
            } => {
                let Some(tail) = tail.into_list() else {
                    return Err(spread_element_type_mismatch(expected, actual));
                };
                if tail.element_type() != expected {
                    return Err(spread_element_type_mismatch(expected, tail.element_type()));
                }
                let values = non_empty_spread_values(values)?;
                Ok(Self::List { values, tail })
            }
            ListElements::Function {
                item_type: _,
                values,
            } => {
                let Some(tail) = tail.into_function() else {
                    return Err(spread_element_type_mismatch(expected, actual));
                };
                if tail.element_type() != expected {
                    return Err(spread_element_type_mismatch(expected, tail.element_type()));
                }
                let values = non_empty_spread_values(values)?;
                Ok(Self::Function { values, tail })
            }
        }
    }
}

fn non_empty_spread_values<Value>(
    values: Vec<Value>,
) -> Result<Vec1<Value>, ListSpreadConstructionError> {
    Vec1::try_from_vec(values).map_err(|_| ListSpreadConstructionError::EmptyPrefix)
}

fn spread_element_type_mismatch(
    expected: ValueType,
    actual: ValueType,
) -> ListSpreadConstructionError {
    ListSpreadConstructionError::ElementTypeMismatch(ListElementTypeMismatch { expected, actual })
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
    use super::{
        ListElementTypeMismatch, ListElements, ListSpreadConstructionError, ListSpreadElements,
    };
    use crate::plan::module::{GenericListExpr, GenericListItem};
    use crate::plan::{
        BitArrayExpr, BoolExpr, CustomExpr, CustomLocal, CustomLocalId, CustomType, CustomTypeName,
        Expr, ExternalExpr, ExternalLocal, ExternalLocalId, ExternalType, ExternalTypeName,
        ExternalValueShape, FloatExpr, FunctionExpr, FunctionReference, FunctionShape,
        FunctionType, GenericExpr, GenericLocal, GenericLocalId, IntExpr, IntListExpr, IntListItem,
        ListExpr, NilExpr, StoredListExpr, StringExpr, StringListExpr, StringListItem, TupleExpr,
        TypeParameterId, UtfCodepointExpr, UtfCodepointLocalId, ValueStorageShape, ValueType,
        monomorphic_function_instantiation,
    };

    fn custom_type(name: &str) -> CustomType {
        CustomType::new(
            CustomTypeName::new("geam".into(), "main".into(), name.into()),
            Vec::new(),
        )
    }

    fn external_type(name: &str) -> ExternalType {
        ExternalType::new(
            ExternalTypeName::new("geam".into(), "main".into(), name.into()),
            Vec::new(),
        )
    }

    fn external_value(name: &str, local: usize) -> ExternalExpr {
        ExternalExpr::local_get(
            ExternalLocal::from_shape(
                ExternalLocalId(local),
                ExternalValueShape::any(external_type(name)),
            ),
            "external".into(),
        )
    }

    #[test]
    fn from_exprs_preserves_every_item_family() {
        let parameter = TypeParameterId(0);
        let generic = GenericExpr::local_get(
            GenericLocal::new(GenericLocalId(0), parameter),
            "generic".into(),
        );
        assert_eq!(
            ListElements::from_exprs(
                ValueType::Parameter(parameter),
                vec![Expr::generic(generic.clone())],
            ),
            Ok(ListElements::Generic {
                parameter,
                values: vec![generic],
            }),
        );

        let int = IntExpr::value(1.into());
        assert_eq!(
            ListElements::from_exprs(ValueType::Int, vec![Expr::int(int.clone())]),
            Ok(ListElements::Int(vec![int])),
        );
        let string = StringExpr::value("one".into());
        assert_eq!(
            ListElements::from_exprs(ValueType::String, vec![Expr::string(string.clone())]),
            Ok(ListElements::String(vec![string])),
        );
        let bit_array = BitArrayExpr::value(Vec::new());
        assert_eq!(
            ListElements::from_exprs(
                ValueType::BitArray,
                vec![Expr::bit_array(bit_array.clone())],
            ),
            Ok(ListElements::BitArray(vec![bit_array])),
        );
        let utf_codepoint = UtfCodepointExpr::local_get(UtfCodepointLocalId(0), "codepoint".into());
        assert_eq!(
            ListElements::from_exprs(
                ValueType::UtfCodepoint,
                vec![Expr::utf_codepoint(utf_codepoint.clone())],
            ),
            Ok(ListElements::UtfCodepoint(vec![utf_codepoint])),
        );

        let custom_type = custom_type("Token");
        let custom = CustomExpr::local_get(
            crate::plan::CustomLocal::new(CustomLocalId(0), custom_type.clone()),
            "token".into(),
        );
        assert_eq!(
            ListElements::from_exprs(
                ValueType::Custom(custom_type.clone()),
                vec![Expr::custom(custom.clone())],
            ),
            Ok(ListElements::Custom {
                item_type: custom_type,
                values: vec![custom],
            }),
        );

        let external_type = external_type("Token");
        let external = external_value("Token", 0);
        assert_eq!(
            ListElements::from_exprs(
                ValueType::External(external_type.clone()),
                vec![Expr::external(external.clone())],
            ),
            Ok(ListElements::External {
                item_type: external_type,
                values: vec![external],
            }),
        );

        let float = FloatExpr::value(1.5);
        assert_eq!(
            ListElements::from_exprs(ValueType::Float, vec![Expr::float(float.clone())]),
            Ok(ListElements::Float(vec![float])),
        );
        let bool_ = BoolExpr::value(true);
        assert_eq!(
            ListElements::from_exprs(ValueType::Bool, vec![Expr::bool(bool_.clone())]),
            Ok(ListElements::Bool(vec![bool_])),
        );
        let nil = NilExpr::value();
        assert_eq!(
            ListElements::from_exprs(ValueType::Nil, vec![Expr::nil(nil.clone())]),
            Ok(ListElements::Nil(vec![nil])),
        );

        let tuple = TupleExpr::value(
            vec![Expr::int(IntExpr::value(2.into()))],
            vec![ValueType::Int],
        );
        assert_eq!(
            ListElements::from_exprs(
                ValueType::Tuple(vec![ValueType::Int]),
                vec![Expr::tuple(tuple.clone())],
            ),
            Ok(ListElements::Tuple {
                item_type: vec![ValueType::Int],
                values: vec![tuple],
            }),
        );

        let nested = ListExpr::value(Vec::new(), ValueType::String);
        assert_eq!(
            ListElements::from_exprs(
                ValueType::List(Box::new(ValueType::String)),
                vec![Expr::list(nested.clone())],
            ),
            Ok(ListElements::List {
                item_shape: ValueStorageShape::String,
                values: vec![StoredListExpr::String(StringListExpr::value(
                    StringListItem,
                    Vec::new()
                ),)],
            }),
        );

        let function_type = FunctionType::new(vec![ValueType::Int], ValueType::Int);
        let function =
            FunctionExpr::reference(FunctionReference::new(monomorphic_function_instantiation(
                0,
                FunctionShape::from_function_type(function_type.clone()),
            )));
        assert_eq!(
            ListElements::from_exprs(
                ValueType::Function(Box::new(function_type.clone())),
                vec![Expr::function(function.clone())],
            ),
            Ok(ListElements::Function {
                item_type: function_type,
                values: vec![function],
            }),
        );
    }

    #[test]
    fn from_exprs_rejects_wrong_item_family_and_nested_metadata() {
        let first_custom = custom_type("First");
        let second_custom = custom_type("Second");
        let first_external = external_type("First");
        let second_external = external_type("Second");
        let first_parameter = TypeParameterId(0);
        let second_parameter = TypeParameterId(1);

        assert_eq!(
            ListElements::from_exprs(
                ValueType::Parameter(first_parameter),
                vec![Expr::generic(GenericExpr::local_get(
                    GenericLocal::new(GenericLocalId(0), second_parameter),
                    "value".into(),
                ))],
            ),
            Err(ListElementTypeMismatch {
                expected: ValueType::Parameter(first_parameter),
                actual: ValueType::Parameter(second_parameter),
            }),
        );

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
        assert_eq!(
            ListElements::from_exprs(
                ValueType::Custom(first_custom.clone()),
                vec![Expr::custom(CustomExpr::local_get(
                    crate::plan::CustomLocal::new(CustomLocalId(0), second_custom.clone()),
                    "value".into(),
                ))],
            ),
            Err(ListElementTypeMismatch {
                expected: ValueType::Custom(first_custom),
                actual: ValueType::Custom(second_custom),
            }),
        );
        assert_eq!(
            ListElements::from_exprs(
                ValueType::External(first_external.clone()),
                vec![Expr::external(external_value("Second", 0))],
            ),
            Err(ListElementTypeMismatch {
                expected: ValueType::External(first_external),
                actual: ValueType::External(second_external),
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

        assert_eq!(
            ListElements::from_exprs(
                ValueType::List(Box::new(ValueType::Parameter(first_parameter))),
                vec![Expr::list(ListExpr::value(Vec::new(), ValueType::Int))],
            ),
            Err(ListElementTypeMismatch {
                expected: ValueType::List(Box::new(ValueType::Parameter(first_parameter))),
                actual: ValueType::List(Box::new(ValueType::Int)),
            }),
        );

        let function =
            FunctionExpr::reference(FunctionReference::new(monomorphic_function_instantiation(
                0,
                FunctionShape::from_function_type(FunctionType::new(Vec::new(), ValueType::Int)),
            )));
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
            ListElements::Generic {
                parameter: TypeParameterId(0),
                values: Vec::new(),
            }
            .item_type(),
            ValueType::Parameter(TypeParameterId(0)),
        );
        assert_eq!(
            ListElements::Int(vec![IntExpr::value(1.into())]).item_type(),
            ValueType::Int,
        );
        assert_eq!(
            ListElements::BitArray(vec![BitArrayExpr::value(Vec::new())]).item_type(),
            ValueType::BitArray,
        );
        assert_eq!(
            ListElements::UtfCodepoint(vec![UtfCodepointExpr::local_get(
                UtfCodepointLocalId(0),
                "codepoint".into(),
            )])
            .item_type(),
            ValueType::UtfCodepoint,
        );
        assert_eq!(
            ListElements::Custom {
                item_type: custom_type("Boxed"),
                values: Vec::new(),
            }
            .item_type(),
            ValueType::Custom(custom_type("Boxed")),
        );
        assert_eq!(
            ListElements::External {
                item_type: external_type("Token"),
                values: Vec::new(),
            }
            .item_type(),
            ValueType::External(external_type("Token")),
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
                item_shape: ValueStorageShape::Bool,
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
    fn spread_parts_reject_empty_prefix_for_every_item_family() {
        let parameter = TypeParameterId(0);
        let custom = custom_type("Token");
        let external = external_type("Token");
        let function = FunctionType::new(vec![ValueType::Int], ValueType::Int);
        let cases = vec![
            (
                ListElements::Generic {
                    parameter,
                    values: Vec::new(),
                },
                ValueType::Parameter(parameter),
            ),
            (
                ListElements::ParameterList {
                    parameter,
                    values: Vec::new(),
                },
                ValueType::List(Box::new(ValueType::Parameter(parameter))),
            ),
            (ListElements::Int(Vec::new()), ValueType::Int),
            (ListElements::String(Vec::new()), ValueType::String),
            (ListElements::BitArray(Vec::new()), ValueType::BitArray),
            (
                ListElements::UtfCodepoint(Vec::new()),
                ValueType::UtfCodepoint,
            ),
            (
                ListElements::Custom {
                    item_type: custom.clone(),
                    values: Vec::new(),
                },
                ValueType::Custom(custom),
            ),
            (
                ListElements::External {
                    item_type: external.clone(),
                    values: Vec::new(),
                },
                ValueType::External(external),
            ),
            (ListElements::Float(Vec::new()), ValueType::Float),
            (ListElements::Bool(Vec::new()), ValueType::Bool),
            (ListElements::Nil(Vec::new()), ValueType::Nil),
            (
                ListElements::Tuple {
                    item_type: vec![ValueType::Int],
                    values: Vec::new(),
                },
                ValueType::Tuple(vec![ValueType::Int]),
            ),
            (
                ListElements::List {
                    item_shape: ValueStorageShape::Int,
                    values: Vec::new(),
                },
                ValueType::List(Box::new(ValueType::Int)),
            ),
            (
                ListElements::Function {
                    item_type: function.clone(),
                    values: Vec::new(),
                },
                ValueType::Function(Box::new(function)),
            ),
        ];

        for (elements, item_type) in cases {
            assert_eq!(
                ListSpreadElements::from_parts(elements, ListExpr::value(Vec::new(), item_type),),
                Err(ListSpreadConstructionError::EmptyPrefix),
            );
        }
    }

    #[test]
    fn spread_parts_preserve_external_values_and_nominal_type() {
        let item_type = external_type("Token");
        let head = external_value("Token", 0);
        let tail = crate::plan::module::ExternalListExpr::value(
            crate::plan::module::ExternalListItem::new(item_type.clone()),
            Vec::new(),
        );

        assert_eq!(
            ListSpreadElements::from_parts(
                ListElements::External {
                    item_type: item_type.clone(),
                    values: vec![head.clone()],
                },
                ListExpr::External(tail.clone()),
            ),
            Ok(ListSpreadElements::External {
                values: vec1::vec1![head],
                tail,
            }),
        );
    }

    #[test]
    fn spread_parts_reject_wrong_tail_family_and_nested_metadata() {
        let first_custom = custom_type("First");
        let second_custom = custom_type("Second");
        let first_external = external_type("First");
        let second_external = external_type("Second");
        let first_parameter = TypeParameterId(0);
        let second_parameter = TypeParameterId(1);
        let generic_value = GenericExpr::local_get(
            GenericLocal::new(GenericLocalId(0), first_parameter),
            "value".into(),
        );
        let parameter_list_value =
            GenericListExpr::value(GenericListItem::new(first_parameter), Vec::new());
        let custom_value = CustomExpr::local_get(
            CustomLocal::new(CustomLocalId(0), first_custom.clone()),
            "custom".into(),
        );
        let external_value = external_value("First", 0);
        let tuple_value = TupleExpr::value(
            vec![Expr::int(IntExpr::value(1.into()))],
            vec![ValueType::Int],
        );
        let nested_value = StoredListExpr::Int(IntListExpr::value(IntListItem, Vec::new()));
        let int_to_int = FunctionType::new(vec![ValueType::Int], ValueType::Int);
        let function_value =
            FunctionExpr::reference(FunctionReference::new(monomorphic_function_instantiation(
                0,
                FunctionShape::from_function_type(int_to_int.clone()),
            )));

        assert_eq!(
            ListSpreadElements::from_parts(
                ListElements::Generic {
                    parameter: first_parameter,
                    values: vec![generic_value.clone()],
                },
                ListExpr::value(Vec::new(), ValueType::Int),
            ),
            Err(ListSpreadConstructionError::ElementTypeMismatch(
                ListElementTypeMismatch {
                    expected: ValueType::Parameter(first_parameter),
                    actual: ValueType::Int,
                },
            )),
        );

        assert_eq!(
            ListSpreadElements::from_parts(
                ListElements::ParameterList {
                    parameter: first_parameter,
                    values: vec![parameter_list_value.clone()],
                },
                ListExpr::value(Vec::new(), ValueType::Int),
            ),
            Err(ListSpreadConstructionError::ElementTypeMismatch(
                ListElementTypeMismatch {
                    expected: ValueType::List(Box::new(ValueType::Parameter(first_parameter))),
                    actual: ValueType::Int,
                },
            )),
        );
        assert_eq!(
            ListSpreadElements::from_parts(
                ListElements::ParameterList {
                    parameter: first_parameter,
                    values: vec![parameter_list_value],
                },
                ListExpr::try_value(
                    Vec::new(),
                    ValueType::List(Box::new(ValueType::Parameter(second_parameter))),
                )
                .expect("empty nested parameter list"),
            ),
            Err(ListSpreadConstructionError::ElementTypeMismatch(
                ListElementTypeMismatch {
                    expected: ValueType::List(Box::new(ValueType::Parameter(first_parameter))),
                    actual: ValueType::List(Box::new(ValueType::Parameter(second_parameter))),
                },
            )),
        );
        assert_eq!(
            ListSpreadElements::from_parts(
                ListElements::Generic {
                    parameter: first_parameter,
                    values: vec![generic_value],
                },
                ListExpr::value(Vec::new(), ValueType::Parameter(second_parameter)),
            ),
            Err(ListSpreadConstructionError::ElementTypeMismatch(
                ListElementTypeMismatch {
                    expected: ValueType::Parameter(first_parameter),
                    actual: ValueType::Parameter(second_parameter),
                },
            )),
        );

        assert_eq!(
            ListSpreadElements::from_parts(
                ListElements::String(vec![StringExpr::value("head".into())]),
                ListExpr::value(Vec::new(), ValueType::Int),
            ),
            Err(ListSpreadConstructionError::ElementTypeMismatch(
                ListElementTypeMismatch {
                    expected: ValueType::String,
                    actual: ValueType::Int,
                },
            )),
        );
        assert_eq!(
            ListSpreadElements::from_parts(
                ListElements::Custom {
                    item_type: first_custom.clone(),
                    values: vec![custom_value.clone()],
                },
                ListExpr::value(Vec::new(), ValueType::Int),
            ),
            Err(ListSpreadConstructionError::ElementTypeMismatch(
                ListElementTypeMismatch {
                    expected: ValueType::Custom(first_custom.clone()),
                    actual: ValueType::Int,
                },
            )),
        );
        assert_eq!(
            ListSpreadElements::from_parts(
                ListElements::External {
                    item_type: first_external.clone(),
                    values: vec![external_value.clone()],
                },
                ListExpr::value(Vec::new(), ValueType::Int),
            ),
            Err(ListSpreadConstructionError::ElementTypeMismatch(
                ListElementTypeMismatch {
                    expected: ValueType::External(first_external.clone()),
                    actual: ValueType::Int,
                },
            )),
        );
        assert_eq!(
            ListSpreadElements::from_parts(
                ListElements::External {
                    item_type: first_external.clone(),
                    values: vec![external_value],
                },
                ListExpr::value(Vec::new(), ValueType::External(second_external.clone()),),
            ),
            Err(ListSpreadConstructionError::ElementTypeMismatch(
                ListElementTypeMismatch {
                    expected: ValueType::External(first_external),
                    actual: ValueType::External(second_external),
                },
            )),
        );
        assert_eq!(
            ListSpreadElements::from_parts(
                ListElements::Custom {
                    item_type: first_custom.clone(),
                    values: vec![custom_value],
                },
                ListExpr::value(Vec::new(), ValueType::Custom(second_custom.clone())),
            ),
            Err(ListSpreadConstructionError::ElementTypeMismatch(
                ListElementTypeMismatch {
                    expected: ValueType::Custom(first_custom),
                    actual: ValueType::Custom(second_custom),
                },
            )),
        );
        assert_eq!(
            ListSpreadElements::from_parts(
                ListElements::BitArray(vec![BitArrayExpr::value(Vec::new())]),
                ListExpr::value(Vec::new(), ValueType::Int),
            ),
            Err(ListSpreadConstructionError::ElementTypeMismatch(
                ListElementTypeMismatch {
                    expected: ValueType::BitArray,
                    actual: ValueType::Int,
                },
            )),
        );
        assert_eq!(
            ListSpreadElements::from_parts(
                ListElements::UtfCodepoint(vec![UtfCodepointExpr::local_get(
                    UtfCodepointLocalId(0),
                    "codepoint".into(),
                )]),
                ListExpr::value(Vec::new(), ValueType::Int),
            ),
            Err(ListSpreadConstructionError::ElementTypeMismatch(
                ListElementTypeMismatch {
                    expected: ValueType::UtfCodepoint,
                    actual: ValueType::Int,
                },
            )),
        );
        assert_eq!(
            ListSpreadElements::from_parts(
                ListElements::Float(vec![FloatExpr::value(1.5)]),
                ListExpr::value(Vec::new(), ValueType::Int),
            ),
            Err(ListSpreadConstructionError::ElementTypeMismatch(
                ListElementTypeMismatch {
                    expected: ValueType::Float,
                    actual: ValueType::Int,
                },
            )),
        );
        assert_eq!(
            ListSpreadElements::from_parts(
                ListElements::Bool(vec![BoolExpr::value(true)]),
                ListExpr::value(Vec::new(), ValueType::Int),
            ),
            Err(ListSpreadConstructionError::ElementTypeMismatch(
                ListElementTypeMismatch {
                    expected: ValueType::Bool,
                    actual: ValueType::Int,
                },
            )),
        );
        assert_eq!(
            ListSpreadElements::from_parts(
                ListElements::Nil(vec![NilExpr::value()]),
                ListExpr::value(Vec::new(), ValueType::Int),
            ),
            Err(ListSpreadConstructionError::ElementTypeMismatch(
                ListElementTypeMismatch {
                    expected: ValueType::Nil,
                    actual: ValueType::Int,
                },
            )),
        );
        assert_eq!(
            ListSpreadElements::from_parts(
                ListElements::Tuple {
                    item_type: vec![ValueType::Int],
                    values: vec![tuple_value.clone()],
                },
                ListExpr::value(Vec::new(), ValueType::String),
            ),
            Err(ListSpreadConstructionError::ElementTypeMismatch(
                ListElementTypeMismatch {
                    expected: ValueType::Tuple(vec![ValueType::Int]),
                    actual: ValueType::String,
                },
            )),
        );
        assert_eq!(
            ListSpreadElements::from_parts(
                ListElements::Tuple {
                    item_type: vec![ValueType::Int],
                    values: vec![tuple_value],
                },
                ListExpr::value(Vec::new(), ValueType::Tuple(vec![ValueType::String])),
            ),
            Err(ListSpreadConstructionError::ElementTypeMismatch(
                ListElementTypeMismatch {
                    expected: ValueType::Tuple(vec![ValueType::Int]),
                    actual: ValueType::Tuple(vec![ValueType::String]),
                },
            )),
        );
        assert_eq!(
            ListSpreadElements::from_parts(
                ListElements::List {
                    item_shape: ValueStorageShape::Int,
                    values: vec![nested_value.clone()],
                },
                ListExpr::value(Vec::new(), ValueType::String),
            ),
            Err(ListSpreadConstructionError::ElementTypeMismatch(
                ListElementTypeMismatch {
                    expected: ValueType::List(Box::new(ValueType::Int)),
                    actual: ValueType::String,
                },
            )),
        );
        assert_eq!(
            ListSpreadElements::from_parts(
                ListElements::List {
                    item_shape: ValueStorageShape::Int,
                    values: vec![nested_value],
                },
                ListExpr::value(Vec::new(), ValueType::List(Box::new(ValueType::String))),
            ),
            Err(ListSpreadConstructionError::ElementTypeMismatch(
                ListElementTypeMismatch {
                    expected: ValueType::List(Box::new(ValueType::Int)),
                    actual: ValueType::List(Box::new(ValueType::String)),
                },
            )),
        );

        let int_to_string = FunctionType::new(vec![ValueType::Int], ValueType::String);
        assert_eq!(
            ListSpreadElements::from_parts(
                ListElements::Function {
                    item_type: int_to_int.clone(),
                    values: vec![function_value.clone()],
                },
                ListExpr::value(Vec::new(), ValueType::String),
            ),
            Err(ListSpreadConstructionError::ElementTypeMismatch(
                ListElementTypeMismatch {
                    expected: ValueType::Function(Box::new(int_to_int.clone())),
                    actual: ValueType::String,
                },
            )),
        );
        assert_eq!(
            ListSpreadElements::from_parts(
                ListElements::Function {
                    item_type: int_to_int.clone(),
                    values: vec![function_value],
                },
                ListExpr::value(
                    Vec::new(),
                    ValueType::Function(Box::new(int_to_string.clone()))
                ),
            ),
            Err(ListSpreadConstructionError::ElementTypeMismatch(
                ListElementTypeMismatch {
                    expected: ValueType::Function(Box::new(int_to_int)),
                    actual: ValueType::Function(Box::new(int_to_string)),
                },
            )),
        );
    }
}
