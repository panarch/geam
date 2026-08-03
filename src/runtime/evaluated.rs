use bitvec::order::Msb0;
use bitvec::vec::BitVec;
use ecow::EcoString;
use num_bigint::BigInt;

mod capture;
mod external;
mod function;
mod source;

pub(in crate::runtime) use capture::{
    EvaluatedCapture, EvaluatedCaptureKind, EvaluatedListCapture,
};
pub(in crate::runtime) use external::EvaluatedExternalValue;
pub(in crate::runtime) use function::{
    EvaluatedBitArrayFunction, EvaluatedBoolFunction, EvaluatedCoreFunctionFunction,
    EvaluatedCustomFunction, EvaluatedExternalFunction, EvaluatedExternalFunctionFunction,
    EvaluatedExternalListFunction, EvaluatedFloatFunction, EvaluatedFunction,
    EvaluatedFunctionFunction, EvaluatedFunctionValue, EvaluatedFunctionValueKind,
    EvaluatedGenericFunction, EvaluatedIntFunction, EvaluatedListFunction, EvaluatedNeverFunction,
    EvaluatedNilFunction, EvaluatedStringFunction, EvaluatedTupleFunction,
    EvaluatedUtfCodepointFunction, FunctionReferenceId,
};
pub(in crate::runtime) use source::{value_source_hash, values_equal};

use super::state::list::{ListValueId, ParameterListValueId, StoredListValueId};
use crate::plan::ValueType;
use crate::plan::execution::runtime::RuntimeValueMetadata;
use crate::plan::execution::type_::{CustomConstructorId, CustomTypeId};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::runtime) struct EvaluatedBitArray {
    value: crate::BitArrayValue,
}

#[derive(Debug, Clone, PartialEq)]
pub(in crate::runtime) struct EvaluatedCustomValue {
    constructor: CustomConstructorId,
    fields: Box<[EvaluatedValue]>,
}

impl EvaluatedCustomValue {
    pub(in crate::runtime) fn from_fields(
        constructor: CustomConstructorId,
        fields: Box<[EvaluatedValue]>,
    ) -> Self {
        Self {
            constructor,
            fields,
        }
    }

    pub(in crate::runtime) fn type_id(&self) -> CustomTypeId {
        self.constructor.type_id()
    }

    pub(in crate::runtime) fn constructor(&self) -> CustomConstructorId {
        self.constructor
    }

    pub(in crate::runtime) fn fields(&self) -> &[EvaluatedValue] {
        &self.fields
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

    pub(in crate::runtime) fn from_value(value: crate::BitArrayValue) -> Self {
        Self { value }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub(in crate::runtime) enum EvaluatedValue {
    Int(BigInt),
    Float(f64),
    String(EcoString),
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
            Vec::new(),
            Vec::new(),
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
