use ecow::EcoString;
use num_bigint::BigInt;
use thiserror::Error;

use super::{BitArrayValue, CustomValue, FunctionValue, Value};
use crate::plan::{CustomType, FunctionType, ValueType};

#[derive(Debug, Clone, PartialEq)]
pub struct ListValue {
    kind: ListValueKind,
}

#[derive(Debug, Error, Clone, PartialEq, Eq)]
#[error("list value item type mismatch at index {index} (expected {expected:?}, got {actual:?})")]
pub struct ListValueItemTypeMismatch {
    pub index: usize,
    pub expected: ValueType,
    pub actual: ValueType,
}

// Empty lists still need item type metadata; variants that cannot infer it from
// their values carry the metadata explicitly.
#[derive(Debug, Clone, PartialEq)]
pub(crate) enum ListValueKind {
    Int(Vec<BigInt>),
    String(Vec<EcoString>),
    BitArray(Vec<BitArrayValue>),
    UtfCodepoint(Vec<char>),
    Custom {
        item_type: CustomType,
        values: Vec<CustomValue>,
    },
    Float(Vec<f64>),
    Bool(Vec<bool>),
    Nil(usize),
    Tuple {
        item_type: Vec<ValueType>,
        values: Vec<Vec<Value>>,
    },
    List {
        item_type: Box<ValueType>,
        values: Vec<ListValue>,
    },
    Function {
        item_type: FunctionType,
        values: Vec<FunctionValue>,
    },
}

impl ListValue {
    pub fn int(values: Vec<BigInt>) -> Self {
        Self {
            kind: ListValueKind::Int(values),
        }
    }

    pub fn string(values: Vec<EcoString>) -> Self {
        Self {
            kind: ListValueKind::String(values),
        }
    }

    pub fn bit_array(values: Vec<BitArrayValue>) -> Self {
        Self {
            kind: ListValueKind::BitArray(values),
        }
    }

    pub fn utf_codepoint(values: Vec<char>) -> Self {
        Self {
            kind: ListValueKind::UtfCodepoint(values),
        }
    }

    pub fn try_custom(
        item_type: CustomType,
        values: Vec<CustomValue>,
    ) -> Result<Self, ListValueItemTypeMismatch> {
        let expected = ValueType::Custom(item_type.clone());
        ensure_item_types(
            &expected,
            values
                .iter()
                .map(|value| ValueType::Custom(value.type_().clone())),
        )?;
        Ok(Self {
            kind: ListValueKind::Custom { item_type, values },
        })
    }

    pub(crate) fn from_evaluated_custom(item_type: CustomType, values: Vec<CustomValue>) -> Self {
        Self {
            kind: ListValueKind::Custom { item_type, values },
        }
    }

    pub fn float(values: Vec<f64>) -> Self {
        Self {
            kind: ListValueKind::Float(values),
        }
    }

    pub fn bool(values: Vec<bool>) -> Self {
        Self {
            kind: ListValueKind::Bool(values),
        }
    }

    pub fn nil(len: usize) -> Self {
        Self {
            kind: ListValueKind::Nil(len),
        }
    }

    pub fn try_tuple(
        item_type: Vec<ValueType>,
        values: Vec<Vec<Value>>,
    ) -> Result<Self, ListValueItemTypeMismatch> {
        let expected = ValueType::Tuple(item_type.clone());
        ensure_item_types(
            &expected,
            values
                .iter()
                .map(|values| ValueType::Tuple(values.iter().map(Value::value_type).collect())),
        )?;
        Ok(Self {
            kind: ListValueKind::Tuple { item_type, values },
        })
    }

    pub(crate) fn from_evaluated_tuple(item_type: Vec<ValueType>, values: Vec<Vec<Value>>) -> Self {
        Self {
            kind: ListValueKind::Tuple { item_type, values },
        }
    }

    pub fn try_list(
        item_type: ValueType,
        values: Vec<ListValue>,
    ) -> Result<Self, ListValueItemTypeMismatch> {
        let expected = ValueType::List(Box::new(item_type.clone()));
        ensure_item_types(
            &expected,
            values
                .iter()
                .map(|value| ValueType::List(Box::new(value.item_type()))),
        )?;
        Ok(Self {
            kind: ListValueKind::List {
                item_type: Box::new(item_type),
                values,
            },
        })
    }

    pub(crate) fn from_evaluated_list(item_type: ValueType, values: Vec<ListValue>) -> Self {
        Self {
            kind: ListValueKind::List {
                item_type: Box::new(item_type),
                values,
            },
        }
    }

    pub fn try_function(
        item_type: FunctionType,
        values: Vec<FunctionValue>,
    ) -> Result<Self, ListValueItemTypeMismatch> {
        let expected = ValueType::Function(Box::new(item_type.clone()));
        ensure_item_types(
            &expected,
            values
                .iter()
                .map(|value| ValueType::Function(Box::new(value.type_()))),
        )?;
        Ok(Self {
            kind: ListValueKind::Function { item_type, values },
        })
    }

    pub(crate) fn from_evaluated_function(
        item_type: FunctionType,
        values: Vec<FunctionValue>,
    ) -> Self {
        Self {
            kind: ListValueKind::Function { item_type, values },
        }
    }

    pub fn empty(item_type: ValueType) -> Self {
        match item_type {
            ValueType::Int => Self::int(Vec::new()),
            ValueType::String => Self::string(Vec::new()),
            ValueType::BitArray => Self::bit_array(Vec::new()),
            ValueType::UtfCodepoint => Self::utf_codepoint(Vec::new()),
            ValueType::Custom(item_type) => Self {
                kind: ListValueKind::Custom {
                    item_type,
                    values: Vec::new(),
                },
            },
            ValueType::Float => Self::float(Vec::new()),
            ValueType::Bool => Self::bool(Vec::new()),
            ValueType::Nil => Self::nil(0),
            ValueType::Tuple(item_type) => Self {
                kind: ListValueKind::Tuple {
                    item_type,
                    values: Vec::new(),
                },
            },
            ValueType::List(item_type) => Self {
                kind: ListValueKind::List {
                    item_type,
                    values: Vec::new(),
                },
            },
            ValueType::Function(item_type) => Self {
                kind: ListValueKind::Function {
                    item_type: *item_type,
                    values: Vec::new(),
                },
            },
        }
    }

    pub fn item_type(&self) -> ValueType {
        match &self.kind {
            ListValueKind::Int(_) => ValueType::Int,
            ListValueKind::String(_) => ValueType::String,
            ListValueKind::BitArray(_) => ValueType::BitArray,
            ListValueKind::UtfCodepoint(_) => ValueType::UtfCodepoint,
            ListValueKind::Custom { item_type, .. } => ValueType::Custom(item_type.clone()),
            ListValueKind::Float(_) => ValueType::Float,
            ListValueKind::Bool(_) => ValueType::Bool,
            ListValueKind::Nil(_) => ValueType::Nil,
            ListValueKind::Tuple { item_type, .. } => ValueType::Tuple(item_type.clone()),
            ListValueKind::List { item_type, .. } => ValueType::List(item_type.clone()),
            ListValueKind::Function { item_type, .. } => {
                ValueType::Function(Box::new(item_type.clone()))
            }
        }
    }

    pub fn len(&self) -> usize {
        match &self.kind {
            ListValueKind::Int(values) => values.len(),
            ListValueKind::String(values) => values.len(),
            ListValueKind::BitArray(values) => values.len(),
            ListValueKind::UtfCodepoint(values) => values.len(),
            ListValueKind::Custom { values, .. } => values.len(),
            ListValueKind::Float(values) => values.len(),
            ListValueKind::Bool(values) => values.len(),
            ListValueKind::Nil(len) => *len,
            ListValueKind::Tuple { values, .. } => values.len(),
            ListValueKind::List { values, .. } => values.len(),
            ListValueKind::Function { values, .. } => values.len(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn to_values(&self) -> Vec<Value> {
        match &self.kind {
            ListValueKind::Int(values) => values.iter().cloned().map(Value::Int).collect(),
            ListValueKind::String(values) => values.iter().cloned().map(Value::String).collect(),
            ListValueKind::BitArray(values) => {
                values.iter().cloned().map(Value::BitArray).collect()
            }
            ListValueKind::UtfCodepoint(values) => {
                values.iter().copied().map(Value::UtfCodepoint).collect()
            }
            ListValueKind::Custom { values, .. } => {
                values.iter().cloned().map(Value::Custom).collect()
            }
            ListValueKind::Float(values) => values.iter().copied().map(Value::Float).collect(),
            ListValueKind::Bool(values) => values.iter().copied().map(Value::Bool).collect(),
            ListValueKind::Nil(len) => vec![Value::Nil; *len],
            ListValueKind::Tuple { values, .. } => {
                values.iter().cloned().map(Value::Tuple).collect()
            }
            ListValueKind::List { values, .. } => values.iter().cloned().map(Value::List).collect(),
            ListValueKind::Function { values, .. } => {
                values.iter().cloned().map(Value::Function).collect()
            }
        }
    }
}

fn ensure_item_types(
    expected: &ValueType,
    actual: impl IntoIterator<Item = ValueType>,
) -> Result<(), ListValueItemTypeMismatch> {
    for (index, actual) in actual.into_iter().enumerate() {
        if actual != *expected {
            return Err(ListValueItemTypeMismatch {
                index,
                expected: expected.clone(),
                actual,
            });
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{ListValue, ListValueItemTypeMismatch};
    use crate::plan::{CustomType, CustomTypeName, FunctionType, ValueType};
    use crate::runtime::{BitArrayValue, CustomValue, FunctionValue, Value};

    #[test]
    fn list_value_operations_preserve_every_storage_family() {
        let function = sample_function();
        let function_type = function.type_();
        let custom_type = sample_custom_type("Boxed");
        let custom = sample_custom_value(custom_type.clone(), "Boxed");
        let values = [
            ListValue::int(vec![1.into(), 2.into()]),
            ListValue::string(vec!["one".into(), "two".into()]),
            ListValue::bit_array(vec![
                BitArrayValue::from_bytes(vec![1]),
                BitArrayValue::from_bytes(vec![2]),
            ]),
            ListValue::utf_codepoint(vec!['a', '\u{10ffff}']),
            ListValue::from_evaluated_custom(
                custom_type.clone(),
                vec![custom.clone(), custom.clone()],
            ),
            ListValue::float(vec![1.5, 2.5]),
            ListValue::bool(vec![true, false]),
            ListValue::nil(2),
            ListValue::from_evaluated_tuple(
                vec![ValueType::Int],
                vec![vec![Value::Int(1.into())], vec![Value::Int(2.into())]],
            ),
            ListValue::from_evaluated_list(
                ValueType::Int,
                vec![
                    ListValue::int(vec![1.into()]),
                    ListValue::int(vec![2.into()]),
                ],
            ),
            ListValue::from_evaluated_function(
                function_type.clone(),
                vec![function.clone(), function.clone()],
            ),
        ];
        let item_types = [
            ValueType::Int,
            ValueType::String,
            ValueType::BitArray,
            ValueType::UtfCodepoint,
            ValueType::Custom(custom_type.clone()),
            ValueType::Float,
            ValueType::Bool,
            ValueType::Nil,
            ValueType::Tuple(vec![ValueType::Int]),
            ValueType::List(Box::new(ValueType::Int)),
            ValueType::Function(Box::new(function_type.clone())),
        ];

        for (value, item_type) in values.iter().zip(item_types) {
            assert_eq!(value.item_type(), item_type);
            assert_eq!(value.len(), 2);
            assert!(!value.is_empty());
            assert_eq!(value.to_values().len(), 2);
        }

        for item_type in [
            ValueType::Int,
            ValueType::String,
            ValueType::BitArray,
            ValueType::UtfCodepoint,
            ValueType::Custom(custom_type),
            ValueType::Float,
            ValueType::Bool,
            ValueType::Nil,
            ValueType::Tuple(vec![ValueType::Int]),
            ValueType::List(Box::new(ValueType::Int)),
            ValueType::Function(Box::new(function_type)),
        ] {
            let value = ListValue::empty(item_type.clone());
            assert_eq!(value.item_type(), item_type);
            assert_eq!(value.len(), 0);
            assert!(value.is_empty());
            assert_eq!(value.to_values(), Vec::<Value>::new());
        }
    }

    #[test]
    fn checked_list_value_constructors_report_exact_item_mismatches() {
        let expected_custom_type = sample_custom_type("Expected");
        let actual_custom_type = sample_custom_type("Actual");
        let expected_custom_value = sample_custom_value(expected_custom_type.clone(), "Expected");
        let actual_custom_value = sample_custom_value(actual_custom_type.clone(), "Actual");

        assert_eq!(
            ListValue::try_custom(
                expected_custom_type.clone(),
                vec![expected_custom_value.clone(), actual_custom_value],
            ),
            Err(ListValueItemTypeMismatch {
                index: 1,
                expected: ValueType::Custom(expected_custom_type.clone()),
                actual: ValueType::Custom(actual_custom_type),
            }),
        );
        assert_eq!(
            ListValue::try_tuple(
                vec![ValueType::Int],
                vec![
                    vec![Value::Int(1.into())],
                    vec![Value::String("wrong".into())],
                ],
            ),
            Err(ListValueItemTypeMismatch {
                index: 1,
                expected: ValueType::Tuple(vec![ValueType::Int]),
                actual: ValueType::Tuple(vec![ValueType::String]),
            }),
        );
        assert_eq!(
            ListValue::try_list(
                ValueType::Int,
                vec![ListValue::int(Vec::new()), ListValue::string(Vec::new())],
            ),
            Err(ListValueItemTypeMismatch {
                index: 1,
                expected: ValueType::List(Box::new(ValueType::Int)),
                actual: ValueType::List(Box::new(ValueType::String)),
            }),
        );

        let function = sample_function();
        assert_eq!(
            ListValue::try_function(
                FunctionType::new(vec![ValueType::Int], ValueType::Int),
                vec![function],
            ),
            Err(ListValueItemTypeMismatch {
                index: 0,
                expected: ValueType::Function(Box::new(FunctionType::new(
                    vec![ValueType::Int],
                    ValueType::Int,
                ))),
                actual: ValueType::Function(Box::new(FunctionType::new(
                    Vec::new(),
                    ValueType::Int,
                ))),
            }),
        );

        assert_eq!(
            ListValue::try_custom(
                expected_custom_type.clone(),
                vec![expected_custom_value.clone()],
            ),
            Ok(ListValue::from_evaluated_custom(
                expected_custom_type.clone(),
                vec![expected_custom_value],
            )),
        );
        assert_eq!(
            ListValue::try_custom(expected_custom_type.clone(), Vec::new()),
            Ok(ListValue::from_evaluated_custom(
                expected_custom_type,
                Vec::new(),
            )),
        );
        assert_eq!(
            ListValue::try_tuple(vec![ValueType::Int], Vec::new()),
            Ok(ListValue::from_evaluated_tuple(
                vec![ValueType::Int],
                Vec::new()
            )),
        );
        assert_eq!(
            ListValue::try_list(ValueType::Int, Vec::new()),
            Ok(ListValue::from_evaluated_list(ValueType::Int, Vec::new())),
        );
        assert_eq!(
            ListValue::try_function(FunctionType::new(Vec::new(), ValueType::Int), Vec::new()),
            Ok(ListValue::from_evaluated_function(
                FunctionType::new(Vec::new(), ValueType::Int),
                Vec::new(),
            )),
        );
    }

    #[test]
    fn checked_function_list_value_constructor_accepts_matching_non_empty_values() {
        let function = sample_function();
        let function_type = function.type_();

        assert_eq!(
            ListValue::try_function(function_type.clone(), vec![function.clone()]),
            Ok(ListValue::from_evaluated_function(
                function_type,
                vec![function],
            )),
        );
    }

    fn sample_function() -> FunctionValue {
        FunctionValue::new(
            crate::plan::execution::RuntimeFunctionId::Int(crate::plan::execution::IntFunctionId(
                0,
            )),
            Vec::new(),
            FunctionType::new(Vec::new(), ValueType::Int),
        )
    }

    fn sample_custom_type(name: &str) -> CustomType {
        CustomType::new(
            CustomTypeName::new("geam".into(), "main".into(), name.into()),
            Vec::new(),
        )
    }

    fn sample_custom_value(type_: CustomType, constructor_name: &str) -> CustomValue {
        CustomValue::from_evaluated(type_, constructor_name.into(), 0, Vec::new())
    }
}
