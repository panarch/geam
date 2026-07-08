use ecow::EcoString;
use num_bigint::BigInt;

use super::{FunctionType, FunctionValue, Value, ValueType};

#[derive(Debug, Clone, PartialEq)]
pub struct ListValue {
    kind: ListValueKind,
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ListItemTypeMismatch {
    pub(crate) expected: ValueType,
    pub(crate) actual: ValueType,
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

    pub fn tuple(item_type: Vec<ValueType>, values: Vec<Vec<Value>>) -> Self {
        Self {
            kind: ListValueKind::Tuple { item_type, values },
        }
    }

    pub fn list(item_type: ValueType, values: Vec<ListValue>) -> Self {
        Self {
            kind: ListValueKind::List {
                item_type: Box::new(item_type),
                values,
            },
        }
    }

    pub fn function(item_type: FunctionType, values: Vec<FunctionValue>) -> Self {
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
            ValueType::Tuple(item_type) => Self::tuple(item_type, Vec::new()),
            ValueType::List(item_type) => Self::list(*item_type, Vec::new()),
            ValueType::Function(item_type) => Self::function(*item_type, Vec::new()),
        }
    }

    pub(crate) fn empty_list(item_type: ValueType) -> Self {
        Self::list(item_type, Vec::new())
    }

    pub(crate) fn empty_function(item_type: FunctionType) -> Self {
        Self::function(item_type, Vec::new())
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

    pub(crate) fn int_values(&self) -> Option<&[BigInt]> {
        match &self.kind {
            ListValueKind::Int(values) => Some(values),
            _ => None,
        }
    }

    pub(crate) fn string_values(&self) -> Option<&[EcoString]> {
        match &self.kind {
            ListValueKind::String(values) => Some(values),
            _ => None,
        }
    }

    pub(crate) fn float_values(&self) -> Option<&[f64]> {
        match &self.kind {
            ListValueKind::Float(values) => Some(values),
            _ => None,
        }
    }

    pub(crate) fn bool_values(&self) -> Option<&[bool]> {
        match &self.kind {
            ListValueKind::Bool(values) => Some(values),
            _ => None,
        }
    }

    pub(crate) fn nil_len(&self) -> Option<usize> {
        match &self.kind {
            ListValueKind::Nil(len) => Some(*len),
            _ => None,
        }
    }

    pub(crate) fn tuple_values(&self, item_type: &[ValueType]) -> Option<&[Vec<Value>]> {
        match &self.kind {
            ListValueKind::Tuple {
                item_type: actual,
                values,
            } if actual == item_type => Some(values),
            _ => None,
        }
    }

    pub(crate) fn list_values(&self, item_type: &ValueType) -> Option<&[ListValue]> {
        match &self.kind {
            ListValueKind::List {
                item_type: actual,
                values,
            } if actual.as_ref() == item_type => Some(values),
            _ => None,
        }
    }

    pub(crate) fn function_values(&self, item_type: &FunctionType) -> Option<&[FunctionValue]> {
        match &self.kind {
            ListValueKind::Function {
                item_type: actual,
                values,
            } if actual == item_type => Some(values),
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
            ListValueKind::Tuple { item_type, values } => {
                Self::tuple(item_type.clone(), values[start..].to_vec())
            }
            ListValueKind::List { item_type, values } => {
                Self::list(item_type.as_ref().clone(), values[start..].to_vec())
            }
            ListValueKind::Function { item_type, values } => {
                Self::function(item_type.clone(), values[start..].to_vec())
            }
        }
    }

    pub(crate) fn append(&mut self, tail: &Self) -> Result<(), ListItemTypeMismatch> {
        let expected = self.item_type();
        let actual = tail.item_type();

        match (&mut self.kind, &tail.kind) {
            (ListValueKind::Int(values), ListValueKind::Int(tail)) => values.extend(tail.clone()),
            (ListValueKind::String(values), ListValueKind::String(tail)) => {
                values.extend(tail.clone())
            }
            (ListValueKind::Float(values), ListValueKind::Float(tail)) => {
                values.extend(tail.iter().copied())
            }
            (ListValueKind::Bool(values), ListValueKind::Bool(tail)) => {
                values.extend(tail.iter().copied())
            }
            (ListValueKind::Nil(len), ListValueKind::Nil(tail)) => *len += tail,
            (
                ListValueKind::Tuple {
                    item_type, values, ..
                },
                ListValueKind::Tuple {
                    item_type: tail_type,
                    values: tail,
                },
            ) if item_type == tail_type => values.extend(tail.clone()),
            (
                ListValueKind::List {
                    item_type, values, ..
                },
                ListValueKind::List {
                    item_type: tail_type,
                    values: tail,
                },
            ) if item_type == tail_type => values.extend(tail.clone()),
            (
                ListValueKind::Function {
                    item_type, values, ..
                },
                ListValueKind::Function {
                    item_type: tail_type,
                    values: tail,
                },
            ) if item_type == tail_type => values.extend(tail.clone()),
            _ => return Err(ListItemTypeMismatch { expected, actual }),
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::{ListItemTypeMismatch, ListValue};
    use crate::plan::{
        FunctionType, FunctionValue, IntFunctionId, ParamLocal, RuntimeFunctionId, Value, ValueType,
    };

    #[test]
    fn empty_preserves_item_type_for_every_family() {
        let function_type = FunctionType::new(vec![ValueType::Int], ValueType::String);
        let types = [
            ValueType::Int,
            ValueType::String,
            ValueType::Float,
            ValueType::Bool,
            ValueType::Nil,
            ValueType::Tuple(vec![ValueType::Int, ValueType::String]),
            ValueType::List(Box::new(ValueType::Int)),
            ValueType::Function(Box::new(function_type)),
        ];

        for item_type in types {
            let value = ListValue::empty(item_type.clone());
            assert_eq!(value.item_type(), item_type);
            assert_eq!(value.len(), 0);
            assert!(value.is_empty());
            assert_eq!(value.to_values(), Vec::<Value>::new());
        }
    }

    #[test]
    fn to_values_preserves_family_specific_storage() {
        let function_type = FunctionType::new(vec![ValueType::Int], ValueType::Int);
        let function_value = FunctionValue::new(
            RuntimeFunctionId::Int(IntFunctionId(0)),
            vec![ParamLocal::int(crate::plan::IntLocalId(0))],
        );

        assert_eq!(
            ListValue::int(vec![1.into(), 2.into()]).to_values(),
            vec![Value::Int(1.into()), Value::Int(2.into())],
        );
        assert_eq!(
            ListValue::string(vec!["one".into(), "two".into()]).to_values(),
            vec![Value::String("one".into()), Value::String("two".into())],
        );
        assert_eq!(
            ListValue::float(vec![1.5, 2.5]).to_values(),
            vec![Value::Float(1.5), Value::Float(2.5)],
        );
        assert_eq!(
            ListValue::bool(vec![true, false]).to_values(),
            vec![Value::Bool(true), Value::Bool(false)],
        );
        assert_eq!(ListValue::nil(2).to_values(), vec![Value::Nil, Value::Nil],);
        assert_eq!(
            ListValue::tuple(
                vec![ValueType::Int, ValueType::String],
                vec![vec![Value::Int(1.into()), Value::String("one".into())]],
            )
            .to_values(),
            vec![Value::Tuple(vec![
                Value::Int(1.into()),
                Value::String("one".into())
            ])],
        );
        assert_eq!(
            ListValue::list(ValueType::Int, vec![ListValue::int(vec![1.into()])]).to_values(),
            vec![Value::List(ListValue::int(vec![1.into()]))],
        );
        assert_eq!(
            ListValue::function(function_type, vec![function_value.clone()]).to_values(),
            vec![Value::Function(function_value)],
        );
    }

    #[test]
    fn drop_first_preserves_family_specific_storage() {
        let function_type = FunctionType::new(vec![ValueType::Int], ValueType::Int);
        let function_value = FunctionValue::new(
            RuntimeFunctionId::Int(IntFunctionId(0)),
            vec![ParamLocal::int(crate::plan::IntLocalId(0))],
        );

        assert_eq!(
            ListValue::int(vec![1.into(), 2.into()]).drop_first(1),
            ListValue::int(vec![2.into()]),
        );
        assert_eq!(
            ListValue::string(vec!["one".into(), "two".into()]).drop_first(1),
            ListValue::string(vec!["two".into()]),
        );
        assert_eq!(
            ListValue::float(vec![1.5, 2.5]).drop_first(1),
            ListValue::float(vec![2.5]),
        );
        assert_eq!(
            ListValue::bool(vec![true, false]).drop_first(1),
            ListValue::bool(vec![false]),
        );
        assert_eq!(ListValue::nil(2).drop_first(1), ListValue::nil(1));
        assert_eq!(
            ListValue::tuple(
                vec![ValueType::Int],
                vec![vec![Value::Int(1.into())], vec![Value::Int(2.into())]],
            )
            .drop_first(1),
            ListValue::tuple(vec![ValueType::Int], vec![vec![Value::Int(2.into())]]),
        );
        assert_eq!(
            ListValue::list(
                ValueType::Int,
                vec![
                    ListValue::int(vec![1.into()]),
                    ListValue::int(vec![2.into()])
                ],
            )
            .drop_first(1),
            ListValue::list(ValueType::Int, vec![ListValue::int(vec![2.into()])]),
        );
        assert_eq!(
            ListValue::function(
                function_type.clone(),
                vec![function_value.clone(), function_value.clone()],
            )
            .drop_first(1),
            ListValue::function(function_type, vec![function_value]),
        );
    }

    #[test]
    fn append_preserves_family_specific_storage() {
        let function_type = FunctionType::new(vec![ValueType::Int], ValueType::Int);
        let function_value = FunctionValue::new(
            RuntimeFunctionId::Int(IntFunctionId(0)),
            vec![ParamLocal::int(crate::plan::IntLocalId(0))],
        );

        let mut ints = ListValue::int(vec![1.into()]);
        assert_eq!(ints.append(&ListValue::int(vec![2.into()])), Ok(()));
        assert_eq!(ints, ListValue::int(vec![1.into(), 2.into()]));

        let mut strings = ListValue::string(vec!["one".into()]);
        assert_eq!(
            strings.append(&ListValue::string(vec!["two".into()])),
            Ok(())
        );
        assert_eq!(strings, ListValue::string(vec!["one".into(), "two".into()]));

        let mut floats = ListValue::float(vec![1.5]);
        assert_eq!(floats.append(&ListValue::float(vec![2.5])), Ok(()));
        assert_eq!(floats, ListValue::float(vec![1.5, 2.5]));

        let mut bools = ListValue::bool(vec![true]);
        assert_eq!(bools.append(&ListValue::bool(vec![false])), Ok(()));
        assert_eq!(bools, ListValue::bool(vec![true, false]));

        let mut nils = ListValue::nil(1);
        assert_eq!(nils.append(&ListValue::nil(2)), Ok(()));
        assert_eq!(nils, ListValue::nil(3));

        let mut tuples = ListValue::tuple(vec![ValueType::Int], vec![vec![Value::Int(1.into())]]);
        assert_eq!(
            tuples.append(&ListValue::tuple(
                vec![ValueType::Int],
                vec![vec![Value::Int(2.into())]],
            )),
            Ok(()),
        );
        assert_eq!(
            tuples,
            ListValue::tuple(
                vec![ValueType::Int],
                vec![vec![Value::Int(1.into())], vec![Value::Int(2.into())]],
            ),
        );

        let mut lists = ListValue::list(ValueType::Int, vec![ListValue::int(vec![1.into()])]);
        assert_eq!(
            lists.append(&ListValue::list(
                ValueType::Int,
                vec![ListValue::int(vec![2.into()])],
            )),
            Ok(()),
        );
        assert_eq!(
            lists,
            ListValue::list(
                ValueType::Int,
                vec![
                    ListValue::int(vec![1.into()]),
                    ListValue::int(vec![2.into()])
                ],
            ),
        );

        let mut functions =
            ListValue::function(function_type.clone(), vec![function_value.clone()]);
        assert_eq!(
            functions.append(&ListValue::function(
                function_type.clone(),
                vec![function_value.clone()],
            )),
            Ok(()),
        );
        assert_eq!(
            functions,
            ListValue::function(function_type, vec![function_value.clone(), function_value]),
        );

        let mut values = ListValue::int(vec![1.into()]);
        assert_eq!(
            values.append(&ListValue::string(vec!["two".into()])),
            Err(ListItemTypeMismatch {
                expected: ValueType::Int,
                actual: ValueType::String,
            }),
        );
    }

    #[test]
    fn append_rejects_nested_item_metadata_mismatch() {
        let mut tuples = ListValue::tuple(vec![ValueType::Int], vec![vec![Value::Int(1.into())]]);
        assert_eq!(
            tuples.append(&ListValue::tuple(
                vec![ValueType::String],
                vec![vec![Value::String("one".into())]],
            )),
            Err(ListItemTypeMismatch {
                expected: ValueType::Tuple(vec![ValueType::Int]),
                actual: ValueType::Tuple(vec![ValueType::String]),
            }),
        );

        let mut lists = ListValue::list(ValueType::Int, vec![ListValue::int(vec![1.into()])]);
        assert_eq!(
            lists.append(&ListValue::list(
                ValueType::String,
                vec![ListValue::string(vec!["one".into()])],
            )),
            Err(ListItemTypeMismatch {
                expected: ValueType::List(Box::new(ValueType::Int)),
                actual: ValueType::List(Box::new(ValueType::String)),
            }),
        );

        let int_function_type = FunctionType::new(Vec::new(), ValueType::Int);
        let string_function_type = FunctionType::new(Vec::new(), ValueType::String);
        let int_function = FunctionValue::new(RuntimeFunctionId::Int(IntFunctionId(0)), Vec::new());
        let string_function = FunctionValue::new(
            RuntimeFunctionId::String(crate::plan::StringFunctionId(0)),
            Vec::new(),
        );
        let mut functions = ListValue::function(int_function_type.clone(), vec![int_function]);
        assert_eq!(
            functions.append(&ListValue::function(
                string_function_type.clone(),
                vec![string_function],
            )),
            Err(ListItemTypeMismatch {
                expected: ValueType::Function(Box::new(int_function_type)),
                actual: ValueType::Function(Box::new(string_function_type)),
            }),
        );
    }
}
