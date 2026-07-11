use ecow::EcoString;
use num_bigint::BigInt;
use thiserror::Error;

use super::{FunctionValue, Value};
use crate::plan::{FunctionType, ValueType};

use crate::plan::execution::{
    BoolListLocalId, ExecutionPlan, FloatListLocalId, FunctionListLocalId, IntListLocalId,
    ListListLocalId, ListLocal, NilListLocalId, StringListLocalId, TupleListLocalId,
};

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

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum ListLocalValue {
    Int {
        local: IntListLocalId,
        value: Vec<BigInt>,
    },
    String {
        local: StringListLocalId,
        value: Vec<EcoString>,
    },
    Float {
        local: FloatListLocalId,
        value: Vec<f64>,
    },
    Bool {
        local: BoolListLocalId,
        value: Vec<bool>,
    },
    Nil {
        local: NilListLocalId,
        len: usize,
    },
    Tuple {
        local: TupleListLocalId,
        item_type: Vec<ValueType>,
        value: Vec<Vec<Value>>,
    },
    List {
        local: ListListLocalId,
        item_type: Box<ValueType>,
        value: Vec<ListValue>,
    },
    Function {
        local: FunctionListLocalId,
        item_type: FunctionType,
        value: Vec<FunctionValue>,
    },
}

impl ListValue {
    pub(crate) fn into_kind(self) -> ListValueKind {
        self.kind
    }

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

    pub(crate) fn into_int_values(self) -> Option<Vec<BigInt>> {
        match self.kind {
            ListValueKind::Int(values) => Some(values),
            _ => None,
        }
    }

    pub(crate) fn into_string_values(self) -> Option<Vec<EcoString>> {
        match self.kind {
            ListValueKind::String(values) => Some(values),
            _ => None,
        }
    }

    pub(crate) fn into_float_values(self) -> Option<Vec<f64>> {
        match self.kind {
            ListValueKind::Float(values) => Some(values),
            _ => None,
        }
    }

    pub(crate) fn into_bool_values(self) -> Option<Vec<bool>> {
        match self.kind {
            ListValueKind::Bool(values) => Some(values),
            _ => None,
        }
    }

    pub(crate) fn into_nil_len(self) -> Option<usize> {
        match self.kind {
            ListValueKind::Nil(len) => Some(len),
            _ => None,
        }
    }

    pub(crate) fn into_tuple_values(self, item_type: &[ValueType]) -> Option<Vec<Vec<Value>>> {
        match self.kind {
            ListValueKind::Tuple {
                item_type: actual,
                values,
            } if actual == item_type => Some(values),
            _ => None,
        }
    }

    pub(crate) fn into_list_values(self, item_type: &ValueType) -> Option<Vec<ListValue>> {
        match self.kind {
            ListValueKind::List {
                item_type: actual,
                values,
            } if actual.as_ref() == item_type => Some(values),
            _ => None,
        }
    }

    pub(crate) fn into_function_values(
        self,
        item_type: &FunctionType,
    ) -> Option<Vec<FunctionValue>> {
        match self.kind {
            ListValueKind::Function {
                item_type: actual,
                values,
            } if actual == *item_type => Some(values),
            _ => None,
        }
    }

    pub(crate) fn drop_first(&self, count: usize) -> Self {
        let start = count.min(self.len());
        match &self.kind {
            ListValueKind::Int(values) => Self::int(values[start..].to_vec()),
            ListValueKind::String(values) => Self::string(values[start..].to_vec()),
            ListValueKind::Float(values) => Self::float(values[start..].to_vec()),
            ListValueKind::Bool(values) => Self::bool(values[start..].to_vec()),
            ListValueKind::Nil(len) => Self::nil(len.saturating_sub(start)),
            ListValueKind::Tuple { item_type, values } => Self {
                kind: ListValueKind::Tuple {
                    item_type: item_type.clone(),
                    values: values[start..].to_vec(),
                },
            },
            ListValueKind::List { item_type, values } => Self {
                kind: ListValueKind::List {
                    item_type: item_type.clone(),
                    values: values[start..].to_vec(),
                },
            },
            ListValueKind::Function { item_type, values } => Self {
                kind: ListValueKind::Function {
                    item_type: item_type.clone(),
                    values: values[start..].to_vec(),
                },
            },
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

impl ListLocalValue {
    pub(crate) fn try_new(
        plan: &ExecutionPlan,
        local: ListLocal,
        value: ListValue,
    ) -> Option<Self> {
        match (local, value.into_kind()) {
            (ListLocal::Int { local, .. }, ListValueKind::Int(value)) => {
                Some(Self::Int { local, value })
            }
            (ListLocal::String { local, .. }, ListValueKind::String(value)) => {
                Some(Self::String { local, value })
            }
            (ListLocal::Float { local, .. }, ListValueKind::Float(value)) => {
                Some(Self::Float { local, value })
            }
            (ListLocal::Bool { local, .. }, ListValueKind::Bool(value)) => {
                Some(Self::Bool { local, value })
            }
            (ListLocal::Nil { local, .. }, ListValueKind::Nil(len)) => {
                Some(Self::Nil { local, len })
            }
            (
                ListLocal::Tuple { local, type_id },
                ListValueKind::Tuple {
                    item_type: actual,
                    values,
                },
            ) if plan.tuple_list_item_type(type_id) == actual => Some(Self::Tuple {
                local,
                item_type: actual,
                value: values,
            }),
            (
                ListLocal::List { local, type_id },
                ListValueKind::List {
                    item_type: actual,
                    values,
                },
            ) if Box::new(plan.nested_list_item_type(type_id)) == actual => Some(Self::List {
                local,
                item_type: actual,
                value: values,
            }),
            (
                ListLocal::Function { local, type_id },
                ListValueKind::Function {
                    item_type: actual,
                    values,
                },
            ) if plan.function_list_item_type(type_id) == actual => Some(Self::Function {
                local,
                item_type: actual,
                value: values,
            }),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{ListValue, ListValueItemTypeMismatch};
    use crate::plan::{FunctionType, ValueType};
    use crate::runtime::{FunctionValue, Value};

    #[test]
    fn list_value_operations_preserve_every_storage_family() {
        let function = sample_function();
        let function_type = function.type_();
        let values = [
            ListValue::int(vec![1.into(), 2.into()]),
            ListValue::string(vec!["one".into(), "two".into()]),
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
            assert_eq!(value.drop_first(1).len(), 1);
            assert_eq!(value.to_values().len(), 2);
        }

        for item_type in [
            ValueType::Int,
            ValueType::String,
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
    fn list_value_owned_accessors_preserve_and_reject_exact_families() {
        let function = sample_function();
        let function_type = function.type_();

        assert_eq!(
            ListValue::int(vec![1.into()]).into_int_values(),
            Some(vec![1.into()])
        );
        assert_eq!(
            ListValue::string(vec!["one".into()]).into_string_values(),
            Some(vec!["one".into()]),
        );
        assert_eq!(
            ListValue::float(vec![1.5]).into_float_values(),
            Some(vec![1.5])
        );
        assert_eq!(
            ListValue::bool(vec![true]).into_bool_values(),
            Some(vec![true])
        );
        assert_eq!(ListValue::nil(1).into_nil_len(), Some(1));
        assert_eq!(
            ListValue::from_evaluated_tuple(
                vec![ValueType::Int],
                vec![vec![Value::Int(1.into())]],
            )
            .into_tuple_values(&[ValueType::Int]),
            Some(vec![vec![Value::Int(1.into())]]),
        );
        assert_eq!(
            ListValue::from_evaluated_list(ValueType::Int, vec![ListValue::int(vec![1.into()])])
                .into_list_values(&ValueType::Int),
            Some(vec![ListValue::int(vec![1.into()])]),
        );
        assert_eq!(
            ListValue::from_evaluated_function(function_type.clone(), vec![function.clone()])
                .into_function_values(&function_type),
            Some(vec![function]),
        );

        assert_eq!(ListValue::string(Vec::new()).into_int_values(), None);
        assert_eq!(ListValue::int(Vec::new()).into_string_values(), None);
        assert_eq!(ListValue::int(Vec::new()).into_float_values(), None);
        assert_eq!(ListValue::int(Vec::new()).into_bool_values(), None);
        assert_eq!(ListValue::int(Vec::new()).into_nil_len(), None);
        assert_eq!(
            ListValue::from_evaluated_tuple(vec![ValueType::String], Vec::new())
                .into_tuple_values(&[ValueType::Int]),
            None,
        );
        assert_eq!(
            ListValue::from_evaluated_list(ValueType::String, Vec::new())
                .into_list_values(&ValueType::Int),
            None,
        );
        assert_eq!(
            ListValue::from_evaluated_function(
                FunctionType::new(Vec::new(), ValueType::String),
                Vec::new(),
            )
            .into_function_values(&FunctionType::new(Vec::new(), ValueType::Int)),
            None,
        );
    }

    #[test]
    fn checked_list_value_constructors_report_exact_item_mismatches() {
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

    fn sample_function() -> FunctionValue {
        FunctionValue::new(
            crate::plan::execution::RuntimeFunctionId::Int(crate::plan::execution::IntFunctionId(
                0,
            )),
            Vec::new(),
            FunctionType::new(Vec::new(), ValueType::Int),
        )
    }
}
