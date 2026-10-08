use crate::StringValue;
use crate::runtime::borrowed::SharedIntegerReads;
use crate::runtime::integer::IntegerValue;
use bitvec::order::Msb0;
use bitvec::vec::BitVec;

mod capture;
mod external;
mod function;
mod read;
mod source;

pub(in crate::runtime) use capture::{
    EvaluatedCapture, EvaluatedCaptureKind, EvaluatedListCapture,
};
pub(crate) use external::EvaluatedExternalValue;
pub(in crate::runtime) use function::{
    EvaluatedBitArrayFunction, EvaluatedBoolFunction, EvaluatedCoreFunctionFunction,
    EvaluatedCustomFunction, EvaluatedExternalFunction, EvaluatedExternalFunctionFunction,
    EvaluatedExternalListFunction, EvaluatedFloatFunction, EvaluatedFunction,
    EvaluatedFunctionFunction, EvaluatedFunctionValue, EvaluatedFunctionValueKind,
    EvaluatedGenericFunction, EvaluatedIntFunction, EvaluatedListFunction, EvaluatedNeverFunction,
    EvaluatedNilFunction, EvaluatedStringFunction, EvaluatedTupleFunction,
    EvaluatedUtfCodepointFunction, FunctionCreation, FunctionReferenceId,
};
pub(in crate::runtime) use read::{EvaluatedFunctionRef, EvaluatedListRef, EvaluatedValueRef};
pub(in crate::runtime) use source::{value_refs_equal, value_source_hash, values_equal};

use super::state::list::{ListValueId, ParameterListValueId, StoredListValueId};
use crate::plan::ValueType;
use crate::plan::execution::runtime::RuntimeValueMetadata;
use crate::plan::execution::type_::{CustomConstructorId, CustomTypeId};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::runtime) struct EvaluatedBitArray {
    value: crate::BitArrayValue,
}

#[derive(Clone)]
pub(in crate::runtime) struct EvaluatedCustomValue {
    constructor: CustomConstructorId,
    fields: std::sync::Arc<CustomFields>,
}

#[derive(Default)]
struct CustomFields {
    values: Box<[EvaluatedValue]>,
    integer_reads: SharedIntegerReads,
}

impl Clone for CustomFields {
    fn clone(&self) -> Self {
        // A copied field allocation has new scalar identities. Shared custom
        // handles clone the Arc instead and keep this original read owner.
        Self {
            values: self.values.clone(),
            integer_reads: Default::default(),
        }
    }
}

impl std::fmt::Debug for EvaluatedCustomValue {
    fn fmt(&self, output: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        output
            .debug_struct("EvaluatedCustomValue")
            .field("constructor", &self.constructor)
            .field("fields", &self.fields())
            .finish()
    }
}

impl PartialEq for EvaluatedCustomValue {
    fn eq(&self, other: &Self) -> bool {
        self.constructor == other.constructor && self.fields() == other.fields()
    }
}

impl EvaluatedBitArray {
    pub(in crate::runtime) fn new(bits: BitVec<u8, Msb0>) -> Self {
        Self {
            value: crate::BitArrayValue::from_evaluated(bits),
        }
    }

    pub(in crate::runtime) fn bits(&self) -> &bitvec::slice::BitSlice<u8, Msb0> {
        self.value.bits()
    }

    pub(in crate::runtime) fn value(&self) -> crate::BitArrayValue {
        self.value.clone()
    }

    pub(in crate::runtime) fn as_value(&self) -> &crate::BitArrayValue {
        &self.value
    }

    pub(in crate::runtime) fn into_value(self) -> crate::BitArrayValue {
        self.value
    }

    pub(in crate::runtime) fn from_value(value: crate::BitArrayValue) -> Self {
        Self { value }
    }
}

impl EvaluatedCustomValue {
    pub(in crate::runtime) fn from_fields(
        constructor: CustomConstructorId,
        fields: Box<[EvaluatedValue]>,
    ) -> Self {
        Self {
            constructor,
            fields: std::sync::Arc::new(CustomFields {
                values: fields,
                integer_reads: Default::default(),
            }),
        }
    }

    pub(in crate::runtime) fn type_id(&self) -> CustomTypeId {
        self.constructor.type_id()
    }

    pub(in crate::runtime) fn constructor(&self) -> CustomConstructorId {
        self.constructor
    }

    pub(in crate::runtime) fn fields(&self) -> &[EvaluatedValue] {
        &self.fields.values
    }

    pub(in crate::runtime) fn integer_reads(&self) -> &SharedIntegerReads {
        &self.fields.integer_reads
    }

    pub(in crate::runtime) fn take_fields(&mut self) -> Box<[EvaluatedValue]> {
        std::sync::Arc::unwrap_or_clone(std::mem::take(&mut self.fields)).values
    }

    pub(in crate::runtime) fn into_fields(self) -> (CustomConstructorId, Box<[EvaluatedValue]>) {
        (
            self.constructor,
            std::sync::Arc::unwrap_or_clone(self.fields).values,
        )
    }
}

#[derive(Debug, Clone, PartialEq)]
pub(in crate::runtime) enum EvaluatedValue {
    Int(IntegerValue),
    Float(f64),
    String(StringValue),
    BitArray(EvaluatedBitArray),
    UtfCodepoint(char),
    Custom(EvaluatedCustomValue),
    External(EvaluatedExternalValue),
    Bool(bool),
    Nil,
    Tuple(Vec<EvaluatedValue>),
    ParameterList(ParameterListValueId),
    List(StoredListValueId),
    Function(EvaluatedFunctionValue),
}

impl From<ListValueId> for EvaluatedValue {
    fn from(value: ListValueId) -> Self {
        match value {
            ListValueId::Parameter(value) => Self::ParameterList(value),
            ListValueId::Int(value) => Self::List(StoredListValueId::Int(value)),
            ListValueId::String(value) => Self::List(StoredListValueId::String(value)),
            ListValueId::BitArray(value) => Self::List(StoredListValueId::BitArray(value)),
            ListValueId::UtfCodepoint(value) => Self::List(StoredListValueId::UtfCodepoint(value)),
            ListValueId::Custom(value) => Self::List(StoredListValueId::Custom(value)),
            ListValueId::External(value) => Self::List(StoredListValueId::External(value)),
            ListValueId::Float(value) => Self::List(StoredListValueId::Float(value)),
            ListValueId::Bool(value) => Self::List(StoredListValueId::Bool(value)),
            ListValueId::Nil(value) => Self::List(StoredListValueId::Nil(value)),
            ListValueId::Tuple(value) => Self::List(StoredListValueId::Tuple(value)),
            ListValueId::ParameterList(value) => {
                Self::List(StoredListValueId::ParameterList(value))
            }
            ListValueId::List(value) => Self::List(StoredListValueId::List(value)),
            ListValueId::Function(value) => Self::List(StoredListValueId::Function(value)),
        }
    }
}

impl From<StoredListValueId> for EvaluatedValue {
    fn from(value: StoredListValueId) -> Self {
        Self::List(value)
    }
}

impl EvaluatedValue {
    pub(in crate::runtime) fn requires_execution(
        &self,
        metadata: RuntimeValueMetadata<'_>,
    ) -> bool {
        match self {
            Self::Custom(value) => metadata
                .custom_lifetime(value.type_id())
                .requires_execution(),
            Self::External(value) => metadata
                .external_lifetime(value.type_id())
                .requires_execution(),
            Self::List(value) => metadata
                .list_lifetime(value.list_type())
                .requires_execution(),
            Self::ParameterList(value) => metadata
                .list_lifetime(value.type_id().list_type())
                .requires_execution(),
            Self::Tuple(values) => values
                .iter()
                .any(|value| value.requires_execution(metadata)),
            Self::Function(_) => true,
            Self::Int(_)
            | Self::Float(_)
            | Self::String(_)
            | Self::BitArray(_)
            | Self::UtfCodepoint(_)
            | Self::Bool(_)
            | Self::Nil => false,
        }
    }

    pub(in crate::runtime) fn value_type(&self, metadata: RuntimeValueMetadata<'_>) -> ValueType {
        match self {
            Self::Int(_) => ValueType::Int,
            Self::Float(_) => ValueType::Float,
            Self::String(_) => ValueType::String,
            Self::BitArray(_) => ValueType::BitArray,
            Self::UtfCodepoint(_) => ValueType::UtfCodepoint,
            Self::Custom(value) => ValueType::Custom(metadata.custom_value_type(value.type_id())),
            Self::External(value) => {
                ValueType::External(metadata.external_value_type(value.type_id()))
            }
            Self::Bool(_) => ValueType::Bool,
            Self::Nil => ValueType::Nil,
            Self::Tuple(values) => ValueType::Tuple(
                values
                    .iter()
                    .map(|value| value.value_type(metadata))
                    .collect(),
            ),
            Self::ParameterList(value) => metadata.list_value_type(value.type_id().list_type()),
            Self::List(value) => metadata.list_value_type(value.list_type()),
            Self::Function(value) => {
                ValueType::Function(Box::new(metadata.function_type(value.type_())))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{EvaluatedBitArray, EvaluatedFunctionValue, EvaluatedIntFunction, EvaluatedValue};
    use crate::plan::ValueType;
    use crate::plan::execution::function::IntFunctionId;
    use crate::plan::execution::runtime::RuntimeExecutionPlan;
    use crate::runtime::state::RuntimeState;
    use crate::runtime::state::list::ListValueId;
    use bitvec::order::Msb0;
    use bitvec::view::BitView;

    #[test]
    fn custom_reads_are_shared_metadata_and_do_not_change_value_identity() {
        use super::EvaluatedCustomValue;
        use crate::plan::execution::type_::{CustomConstructorId, CustomTypeId};
        let constructor = CustomConstructorId {
            type_id: CustomTypeId(0),
            index: 0,
        };
        let value = EvaluatedCustomValue::from_fields(
            constructor,
            vec![EvaluatedValue::Int(42.into())].into_boxed_slice(),
        );
        let alias = value.clone();
        let same = EvaluatedCustomValue::from_fields(
            constructor,
            vec![EvaluatedValue::Int(42.into())].into_boxed_slice(),
        );
        value.integer_reads().get_or_init(Default::default);
        assert!(std::ptr::eq(value.integer_reads(), alias.integer_reads()));
        assert!(same.integer_reads().get().is_none());
        assert_eq!(value, same);
        let expected = "EvaluatedCustomValue { constructor: CustomConstructorId { type_id: CustomTypeId(0), index: 0 }, fields: [Int(42)] }";
        assert_eq!(format!("{value:?}"), expected);
        assert_eq!(format!("{same:?}"), expected);
        let different = EvaluatedCustomValue::from_fields(
            constructor,
            vec![EvaluatedValue::Int(43.into())].into_boxed_slice(),
        );
        assert_ne!(value, different);
        let other = EvaluatedCustomValue::from_fields(
            CustomConstructorId {
                index: 1,
                ..constructor
            },
            value.fields().into(),
        );
        assert_ne!(value, other);
        let copied = value.fields.as_ref().clone();
        assert!(copied.integer_reads.get().is_none());
        assert_eq!(copied.values, value.fields.values);
        assert!(!std::ptr::eq(
            copied.values.as_ptr(),
            value.fields().as_ptr()
        ));
    }

    #[test]
    fn evaluated_bit_array_aligns_owned_slices() {
        let source = [0x77u8];
        let value = EvaluatedBitArray::new(source.view_bits::<Msb0>()[4..6].to_bitvec());

        assert_eq!(value.bits(), &[0b0100_0000u8].view_bits::<Msb0>()[..2],);
        assert_eq!(value.bits().len(), 2);
    }

    #[test]
    fn evaluated_value_type_preserves_every_runtime_family() {
        let plan = crate::runtime::plan_src(
            r#"
fn ints() -> List(Int) { [] }

pub fn main() {
  let _ = ints
  0
}
"#,
        );
        let mut echo = Vec::new();
        let mut state = RuntimeState::new(&mut echo);
        let list = state
            .lists_mut()
            .int(plan.int_list_function_id(0).type_id(), vec![1.into()]);
        let function = EvaluatedIntFunction::reference(
            IntFunctionId(0),
            Default::default(),
            crate::plan::execution::type_::FunctionType::new(
                Vec::new(),
                crate::plan::execution::type_::ValueType::Int,
            ),
        );
        let values = [
            EvaluatedValue::Int(1.into()),
            EvaluatedValue::Float(1.5),
            EvaluatedValue::String("one".into()),
            EvaluatedValue::BitArray(EvaluatedBitArray::new(bitvec::vec::BitVec::new())),
            EvaluatedValue::UtfCodepoint('\u{10ffff}'),
            EvaluatedValue::Bool(true),
            EvaluatedValue::Nil,
            EvaluatedValue::Tuple(vec![EvaluatedValue::Int(1.into())]),
            EvaluatedValue::from(ListValueId::Int(list)),
            EvaluatedValue::Function(EvaluatedFunctionValue::from(function)),
        ];
        let expected = [
            ValueType::Int,
            ValueType::Float,
            ValueType::String,
            ValueType::BitArray,
            ValueType::UtfCodepoint,
            ValueType::Bool,
            ValueType::Nil,
            ValueType::Tuple(vec![ValueType::Int]),
            ValueType::List(Box::new(ValueType::Int)),
            ValueType::Function(Box::new(crate::plan::FunctionType::new(
                Vec::new(),
                ValueType::Int,
            ))),
        ];

        for (value, expected) in values.iter().zip(expected) {
            assert_eq!(value.value_type(plan.value_metadata()), expected);
        }
    }
}
