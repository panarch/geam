use crate::plan::execution::type_::CustomConstructorId;
use crate::runtime::evaluated::{EvaluatedBitArray, EvaluatedCustomValue, EvaluatedValue};
use crate::runtime::graph::RetainedValues;

pub(crate) struct EmbeddingInput(EvaluatedValue);

impl EmbeddingInput {
    pub(crate) fn int(value: num_bigint::BigInt) -> Self {
        Self(EvaluatedValue::Int(value))
    }

    pub(crate) fn float(value: f64) -> Self {
        Self(EvaluatedValue::Float(value))
    }

    pub(crate) fn string(value: ecow::EcoString) -> Self {
        Self(EvaluatedValue::String(value))
    }

    pub(crate) fn bit_array(value: crate::BitArrayValue) -> Self {
        Self(EvaluatedValue::BitArray(EvaluatedBitArray::from_value(
            value,
        )))
    }

    pub(crate) fn utf_codepoint(value: char) -> Self {
        Self(EvaluatedValue::UtfCodepoint(value))
    }

    pub(crate) fn custom(
        constructor: CustomConstructorId,
        fields: impl IntoIterator<Item = Self>,
    ) -> Self {
        Self(EvaluatedValue::Custom(EvaluatedCustomValue::from_fields(
            constructor,
            fields
                .into_iter()
                .map(|field| field.0)
                .collect::<Vec<_>>()
                .into_boxed_slice(),
        )))
    }

    pub(crate) fn bool(value: bool) -> Self {
        Self(EvaluatedValue::Bool(value))
    }

    pub(crate) fn nil() -> Self {
        Self(EvaluatedValue::Nil)
    }

    pub(crate) fn tuple(fields: impl IntoIterator<Item = Self>) -> Self {
        Self(EvaluatedValue::Tuple(
            fields.into_iter().map(|field| field.0).collect(),
        ))
    }

    pub(crate) fn retain(self, values: &mut RetainedValues) {
        values.push_evaluated(self.0);
    }
}
