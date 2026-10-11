use super::{CallBitArray, CallNullary};
use crate::StringValue;
use crate::plan::execution::type_::{CustomConstructorId, ValueType};
use crate::runtime::evaluated::{EvaluatedCustomValue, EvaluatedValue};

/// The original immutable custom payload, retained across generated calls.
#[derive(Clone)]
pub struct CallCustom(pub(in crate::runtime) EvaluatedCustomValue);

/// An owned tuple crosses a continuation without materializing public values.
#[derive(Clone)]
pub struct CallTuple(pub(in crate::runtime) Vec<EvaluatedValue>);

/// A field borrow ends within one generated match. It never enters a pending
/// native request or a generated continuation.
pub struct CompoundField<'value>(&'value EvaluatedValue);

impl CallCustom {
    pub fn matches_constructor(&self, constructor: CustomConstructorId) -> bool {
        self.0.constructor() == constructor
    }

    pub fn nullary(&self, constructors: &[CustomConstructorId]) -> Option<CallNullary> {
        (self.0.fields().is_empty() && constructors.contains(&self.0.constructor()))
            .then(|| CallNullary::new(self.0.constructor()))
    }

    pub fn field(&self, index: usize) -> Option<CompoundField<'_>> {
        self.0.fields().get(index).map(CompoundField)
    }
}

impl CallTuple {
    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn field(&self, index: usize) -> Option<CompoundField<'_>> {
        self.0.get(index).map(CompoundField)
    }
}

impl CompoundField<'_> {
    /// Preserve the approved exact custom-field invariant without constructing
    /// public tuple type vectors on every successful source match.
    pub fn matches_type(&self, expected: &ValueType) -> bool {
        match (self.0, expected) {
            (EvaluatedValue::Int(_), ValueType::Int)
            | (EvaluatedValue::Float(_), ValueType::Float)
            | (EvaluatedValue::Bool(_), ValueType::Bool)
            | (EvaluatedValue::Nil, ValueType::Nil)
            | (EvaluatedValue::UtfCodepoint(_), ValueType::UtfCodepoint)
            | (EvaluatedValue::String(_), ValueType::String)
            | (EvaluatedValue::BitArray(_), ValueType::BitArray) => true,
            (EvaluatedValue::Custom(value), ValueType::Custom(expected)) => {
                value.constructor().type_id == *expected
            }
            (EvaluatedValue::Tuple(values), ValueType::Tuple(expected)) => {
                values.len() == expected.len()
                    && values
                        .iter()
                        .zip(expected.iter())
                        .all(|(value, expected)| CompoundField(value).matches_type(expected))
            }
            _ => false,
        }
    }

    pub fn matches_constructor(&self, constructor: CustomConstructorId) -> bool {
        matches!(self.0, EvaluatedValue::Custom(value) if value.constructor() == constructor)
    }

    pub fn tuple_len(&self) -> Option<usize> {
        match self.0 {
            EvaluatedValue::Tuple(values) => Some(values.len()),
            _ => None,
        }
    }

    pub fn field(&self, index: usize) -> Option<CompoundField<'_>> {
        match self.0 {
            EvaluatedValue::Custom(value) => value.fields().get(index).map(CompoundField),
            EvaluatedValue::Tuple(values) => values.get(index).map(CompoundField),
            _ => None,
        }
    }

    pub fn integer(&self) -> Option<i128> {
        match self.0 {
            EvaluatedValue::Int(value) => value.small().map(i128::from),
            _ => None,
        }
    }

    pub fn boolean(&self) -> Option<bool> {
        match self.0 {
            EvaluatedValue::Bool(value) => Some(*value),
            _ => None,
        }
    }

    pub fn float(&self) -> Option<f64> {
        match self.0 {
            EvaluatedValue::Float(value) => Some(*value),
            _ => None,
        }
    }

    pub fn string(&self) -> Option<StringValue> {
        match self.0 {
            EvaluatedValue::String(value) => Some(value.clone()),
            _ => None,
        }
    }

    pub fn bit_array(&self) -> Option<CallBitArray> {
        match self.0 {
            EvaluatedValue::BitArray(value) => Some(CallBitArray(value.clone())),
            _ => None,
        }
    }

    pub fn utf_codepoint(&self) -> Option<char> {
        match self.0 {
            EvaluatedValue::UtfCodepoint(value) => Some(*value),
            _ => None,
        }
    }

    pub fn nil(&self) -> Option<()> {
        matches!(self.0, EvaluatedValue::Nil).then_some(())
    }

    pub fn nullary(&self, constructors: &[CustomConstructorId]) -> Option<CallNullary> {
        match self.0 {
            EvaluatedValue::Custom(value)
                if value.fields().is_empty() && constructors.contains(&value.constructor()) =>
            {
                Some(CallNullary::new(value.constructor()))
            }
            _ => None,
        }
    }

    pub fn custom(&self) -> Option<CallCustom> {
        match self.0 {
            EvaluatedValue::Custom(value) => Some(CallCustom(value.clone())),
            _ => None,
        }
    }

    pub fn tuple(&self) -> Option<CallTuple> {
        match self.0 {
            EvaluatedValue::Tuple(values) => Some(CallTuple(values.clone())),
            _ => None,
        }
    }
}

impl From<CallNullary> for CallCustom {
    fn from(value: CallNullary) -> Self {
        Self(value.into_evaluated())
    }
}

#[cfg(test)]
mod tests {
    use super::super::CallOutput;
    use super::{CallCustom, CallNullary, CallTuple, CompoundField};
    use crate::host::HostExternalStore;
    use crate::plan::execution::function::FunctionReturnFamily;
    use crate::plan::execution::type_::{
        CustomConstructorId, CustomTypeId, ExternalTypeId, ValueType,
    };
    use crate::runtime::evaluated::{
        EvaluatedBitArray, EvaluatedCustomValue, EvaluatedExternalValue, EvaluatedValue,
    };
    use crate::{BitArrayValue, StringValue};
    use ecow::EcoString;
    use num_bigint::BigInt;
    use std::sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    };

    #[test]
    fn field_reads_keep_exact_leaf_types_and_decline_an_unrepresentable_integer() {
        let text: StringValue = "kept".into();
        let bytes = BitArrayValue::try_from_parts(vec![0xa0], 3).unwrap();
        let fields = [
            (EvaluatedValue::Int(17.into()), ValueType::Int),
            (EvaluatedValue::Float(-0.0), ValueType::Float),
            (EvaluatedValue::Bool(false), ValueType::Bool),
            (EvaluatedValue::Nil, ValueType::Nil),
            (EvaluatedValue::UtfCodepoint('한'), ValueType::UtfCodepoint),
            (EvaluatedValue::String(text.clone()), ValueType::String),
            (
                EvaluatedValue::BitArray(EvaluatedBitArray::from_value(bytes.clone())),
                ValueType::BitArray,
            ),
        ];
        for (index, (value, expected)) in fields.iter().enumerate() {
            let field = CompoundField(value);
            for (other, (_, type_)) in fields.iter().enumerate() {
                assert_eq!(field.matches_type(type_), index == other);
            }
            assert!(field.matches_type(expected));
            assert_eq!(field.integer(), (index == 0).then_some(17));
            assert_eq!(
                field.float().map(f64::to_bits),
                (index == 1).then_some((-0.0_f64).to_bits())
            );
            assert_eq!(field.boolean(), (index == 2).then_some(false));
            assert_eq!(field.nil(), (index == 3).then_some(()));
            assert_eq!(field.utf_codepoint(), (index == 4).then_some('한'));
            assert_eq!(field.string(), (index == 5).then(|| text.clone()));
            assert_eq!(
                field.bit_array().map(|value| value.0.into_value()),
                (index == 6).then(|| bytes.clone())
            );
            assert!(field.custom().is_none());
            assert!(field.tuple().is_none());
            assert_eq!(field.tuple_len(), None);
            assert!(field.field(0).is_none());
        }
        let wide: BigInt = BigInt::from(1_u8) << 130_usize;
        let value = EvaluatedValue::Int(wide.into());
        assert!(CompoundField(&value).matches_type(&ValueType::Int));
        assert_eq!(CompoundField(&value).integer(), None);
    }

    #[test]
    fn custom_and_nested_tuple_checks_borrow_the_original_owner() {
        let constructor = CustomConstructorId {
            type_id: CustomTypeId(2),
            index: 1,
        };
        let other = CustomConstructorId {
            type_id: CustomTypeId(2),
            index: 0,
        };
        let tuple = EvaluatedValue::Tuple(vec![
            EvaluatedValue::Int(7.into()),
            EvaluatedValue::String("a long immutable tail whose backing is shared".into()),
        ]);
        let owner = CallCustom(EvaluatedCustomValue::from_fields(
            constructor,
            vec![tuple].into_boxed_slice(),
        ));
        assert_eq!(
            CallOutput::Custom(owner.clone()).family(),
            FunctionReturnFamily::Custom,
        );
        assert!(owner.matches_constructor(constructor));
        assert!(!owner.matches_constructor(other));
        assert!(owner.field(1).is_none());
        assert!(owner.nullary(&[constructor]).is_none());
        let field = owner.field(0).unwrap();
        let expected = ValueType::Tuple(vec![ValueType::Int, ValueType::String].into());
        assert!(field.matches_type(&expected));
        assert!(!field.matches_type(&ValueType::Tuple(vec![ValueType::Int].into())));
        assert!(!field.matches_type(&ValueType::Tuple(
            vec![ValueType::Int, ValueType::Bool].into()
        )));
        assert_eq!(field.tuple_len(), Some(2));
        assert_eq!(field.field(0).unwrap().integer(), Some(7));
        assert!(field.field(2).is_none());
        let text = field.field(1).unwrap().string().unwrap();
        let escaped = field.tuple().unwrap();
        assert_eq!(
            CallOutput::Tuple(escaped.clone()).family(),
            FunctionReturnFamily::Tuple,
        );
        assert_eq!(escaped.len(), 2);
        assert!(!escaped.is_empty());
        assert!(escaped.field(2).is_none());
        assert_eq!(
            escaped
                .field(1)
                .unwrap()
                .string()
                .unwrap()
                .as_bytes()
                .as_ptr(),
            text.as_bytes().as_ptr()
        );
        let value = EvaluatedValue::Custom(owner.0.clone());
        let custom = CompoundField(&value);
        assert!(custom.matches_type(&ValueType::Custom(CustomTypeId(2))));
        assert!(!custom.matches_type(&ValueType::Custom(CustomTypeId(3))));
        assert!(custom.matches_constructor(constructor));
        assert!(!custom.matches_constructor(other));
        assert_eq!(
            custom.custom().unwrap().0.fields().as_ptr(),
            owner.0.fields().as_ptr()
        );
        assert_eq!(custom.field(0).unwrap().tuple_len(), Some(2));
        assert!(custom.field(1).is_none());
        assert!(custom.nullary(&[constructor]).is_none());
        let nil = CallCustom::from(CallNullary::new(other));
        assert!(nil.nullary(&[other]).unwrap().matches_constructor(other));
        assert!(
            !nil.nullary(&[other])
                .unwrap()
                .matches_constructor(constructor)
        );
        assert!(nil.nullary(&[constructor]).is_none());
        let value = EvaluatedValue::Custom(nil.0);
        assert!(
            CompoundField(&value)
                .nullary(&[other])
                .unwrap()
                .matches_constructor(other)
        );
        assert!(CompoundField(&value).nullary(&[constructor]).is_none());
        assert!(
            CompoundField(&EvaluatedValue::Nil)
                .nullary(&[other])
                .is_none()
        );
        assert!(!CompoundField(&EvaluatedValue::Nil).matches_constructor(other));
        let empty = CallTuple(Vec::new());
        assert!(empty.is_empty());
        assert_eq!(empty.len(), 0);
    }

    #[test]
    fn escaping_a_tuple_field_does_not_keep_an_unrelated_custom_field_alive() {
        struct Resource(Arc<AtomicUsize>);
        impl Drop for Resource {
            fn drop(&mut self) {
                self.0.fetch_add(1, Ordering::SeqCst);
            }
        }
        const INSPECTION: EcoString = EcoString::new();
        let drops = Arc::new(AtomicUsize::new(0));
        let store = HostExternalStore::default();
        let lease = store.insert(
            Resource(Arc::clone(&drops)),
            |_, _, _| false,
            |_, _| 0,
            |_, _| INSPECTION,
            |_| None,
        );
        let resource =
            EvaluatedValue::External(EvaluatedExternalValue::new(ExternalTypeId::new(0), lease));
        let constructor = CustomConstructorId {
            type_id: CustomTypeId(0),
            index: 0,
        };
        let owner = CallCustom(EvaluatedCustomValue::from_fields(
            constructor,
            vec![
                EvaluatedValue::Tuple(vec![EvaluatedValue::String("kept".into())]),
                resource,
            ]
            .into_boxed_slice(),
        ));
        let escaped = owner.field(0).unwrap().tuple().unwrap();
        assert_eq!(drops.load(Ordering::SeqCst), 0);
        drop(owner);
        assert_eq!(drops.load(Ordering::SeqCst), 1);
        assert_eq!(
            escaped
                .field(0)
                .unwrap()
                .string()
                .unwrap()
                .as_str()
                .unwrap(),
            "kept"
        );
        drop(escaped);
        assert_eq!(drops.load(Ordering::SeqCst), 1);
    }
}
