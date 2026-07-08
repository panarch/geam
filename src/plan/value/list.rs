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

    pub(crate) fn append(&mut self, tail: Self) -> Result<(), (ValueType, ValueType)> {
        let expected = self.item_type();
        let actual = tail.item_type();
        match (&mut self.kind, tail.kind) {
            (ListValueKind::Int(values), ListValueKind::Int(tail)) => values.extend(tail),
            (ListValueKind::String(values), ListValueKind::String(tail)) => values.extend(tail),
            (ListValueKind::Float(values), ListValueKind::Float(tail)) => values.extend(tail),
            (ListValueKind::Bool(values), ListValueKind::Bool(tail)) => values.extend(tail),
            (ListValueKind::Nil(len), ListValueKind::Nil(tail)) => *len += tail,
            (
                ListValueKind::Tuple { item_type, values },
                ListValueKind::Tuple {
                    item_type: tail_type,
                    values: tail,
                },
            ) if *item_type == tail_type => values.extend(tail),
            (
                ListValueKind::List { item_type, values },
                ListValueKind::List {
                    item_type: tail_type,
                    values: tail,
                },
            ) if *item_type == tail_type => values.extend(tail),
            (
                ListValueKind::Function { item_type, values },
                ListValueKind::Function {
                    item_type: tail_type,
                    values: tail,
                },
            ) if *item_type == tail_type => values.extend(tail),
            _ => return Err((expected, actual)),
        }
        Ok(())
    }

    pub(crate) fn item_value_mismatch(&self) -> Option<ValueType> {
        match &self.kind {
            ListValueKind::Tuple { item_type, values } => values
                .iter()
                .map(|value| ValueType::Tuple(value.iter().map(Value::value_type).collect()))
                .find(|actual| actual != &ValueType::Tuple(item_type.clone())),
            ListValueKind::List { item_type, values } => values.iter().find_map(|value| {
                value.item_value_mismatch().or_else(|| {
                    let actual = ValueType::List(Box::new(value.item_type()));
                    (actual != ValueType::List(item_type.clone())).then_some(actual)
                })
            }),
            ListValueKind::Function { item_type, values } => values
                .iter()
                .map(|value| ValueType::Function(Box::new(value.type_())))
                .find(|actual| actual != &ValueType::Function(Box::new(item_type.clone()))),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ListValue;
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
    fn owned_accessors_preserve_family_specific_storage() {
        let function_type = FunctionType::new(vec![ValueType::Int], ValueType::Int);
        let function_value = FunctionValue::new(
            RuntimeFunctionId::Int(IntFunctionId(0)),
            vec![ParamLocal::int(crate::plan::IntLocalId(0))],
        );

        assert_eq!(
            ListValue::int(vec![1.into(), 2.into()]).into_int_values(),
            Some(vec![1.into(), 2.into()]),
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
        assert_eq!(ListValue::nil(2).into_nil_len(), Some(2));
        assert_eq!(
            ListValue::tuple(vec![ValueType::Int], vec![vec![Value::Int(1.into())]],)
                .into_tuple_values(&[ValueType::Int]),
            Some(vec![vec![Value::Int(1.into())]]),
        );
        assert_eq!(
            ListValue::list(ValueType::Int, vec![ListValue::int(vec![1.into()])])
                .into_list_values(&ValueType::Int),
            Some(vec![ListValue::int(vec![1.into()])]),
        );
        assert_eq!(
            ListValue::function(function_type.clone(), vec![function_value.clone()])
                .into_function_values(&function_type),
            Some(vec![function_value.clone()]),
        );

        assert_eq!(
            ListValue::string(vec!["wrong".into()]).into_int_values(),
            None,
        );
        assert_eq!(ListValue::int(vec![1.into()]).into_string_values(), None);
        assert_eq!(ListValue::int(vec![1.into()]).into_float_values(), None);
        assert_eq!(ListValue::int(vec![1.into()]).into_bool_values(), None);
        assert_eq!(ListValue::int(vec![1.into()]).into_nil_len(), None);
        assert_eq!(
            ListValue::tuple(
                vec![ValueType::String],
                vec![vec![Value::String("one".into())]],
            )
            .into_tuple_values(&[ValueType::Int]),
            None,
        );
        assert_eq!(
            ListValue::list(
                ValueType::String,
                vec![ListValue::string(vec!["one".into()])]
            )
            .into_list_values(&ValueType::Int),
            None,
        );
        assert_eq!(
            ListValue::function(function_type.clone(), vec![function_value])
                .into_function_values(&FunctionType::new(Vec::new(), ValueType::Nil)),
            None,
        );
    }

    #[test]
    fn append_preserves_same_family_storage_and_rejects_mismatch() {
        let function_type = FunctionType::new(vec![ValueType::Int], ValueType::Int);
        let function_value = FunctionValue::new(
            RuntimeFunctionId::Int(IntFunctionId(0)),
            vec![ParamLocal::int(crate::plan::IntLocalId(0))],
        );

        let mut int = ListValue::int(vec![1.into()]);
        assert_eq!(int.append(ListValue::int(vec![2.into()])), Ok(()));
        assert_eq!(int, ListValue::int(vec![1.into(), 2.into()]));

        let mut string = ListValue::string(vec!["one".into()]);
        assert_eq!(string.append(ListValue::string(vec!["two".into()])), Ok(()),);
        assert_eq!(string, ListValue::string(vec!["one".into(), "two".into()]),);

        let mut float = ListValue::float(vec![1.5]);
        assert_eq!(float.append(ListValue::float(vec![2.5])), Ok(()));
        assert_eq!(float, ListValue::float(vec![1.5, 2.5]));

        let mut bool_ = ListValue::bool(vec![true]);
        assert_eq!(bool_.append(ListValue::bool(vec![false])), Ok(()));
        assert_eq!(bool_, ListValue::bool(vec![true, false]));

        let mut nil = ListValue::nil(1);
        assert_eq!(nil.append(ListValue::nil(2)), Ok(()));
        assert_eq!(nil, ListValue::nil(3));

        let mut tuple = ListValue::tuple(vec![ValueType::Int], vec![vec![Value::Int(1.into())]]);
        assert_eq!(
            tuple.append(ListValue::tuple(
                vec![ValueType::Int],
                vec![vec![Value::Int(2.into())]],
            )),
            Ok(()),
        );
        assert_eq!(
            tuple,
            ListValue::tuple(
                vec![ValueType::Int],
                vec![vec![Value::Int(1.into())], vec![Value::Int(2.into())]],
            ),
        );

        let mut list = ListValue::list(ValueType::Int, vec![ListValue::int(vec![1.into()])]);
        assert_eq!(
            list.append(ListValue::list(
                ValueType::Int,
                vec![ListValue::int(vec![2.into()])],
            )),
            Ok(()),
        );
        assert_eq!(
            list,
            ListValue::list(
                ValueType::Int,
                vec![
                    ListValue::int(vec![1.into()]),
                    ListValue::int(vec![2.into()])
                ],
            ),
        );

        let mut function = ListValue::function(function_type.clone(), vec![function_value.clone()]);
        assert_eq!(
            function.append(ListValue::function(
                function_type.clone(),
                vec![function_value.clone()]
            )),
            Ok(()),
        );
        assert_eq!(
            function,
            ListValue::function(function_type, vec![function_value.clone(), function_value]),
        );

        let mut int = ListValue::int(vec![1.into()]);
        assert_eq!(
            int.append(ListValue::string(vec!["wrong".into()])),
            Err((ValueType::Int, ValueType::String)),
        );
    }
}
