use crate::plan::execution::type_::{
    BitArrayListTypeId, BoolListTypeId, CustomConstructorId, CustomListTypeId, FloatListTypeId,
    IntListTypeId, ListListTypeId, NilListTypeId, StringListTypeId, TupleListTypeId,
    UtfCodepointListTypeId,
};
use crate::runtime::evaluated::{EvaluatedBitArray, EvaluatedCustomValue, EvaluatedValue};
use crate::runtime::graph::RetainedValues;
use crate::runtime::state::list::{CustomListAllocation, RuntimeListStorage, StoredListValueId};

pub(crate) struct EmbeddingInput(EvaluatedValue);

pub(crate) trait EmbeddingInputValue: Sized {
    type ListType: Copy;

    fn into_input(self) -> EmbeddingInput;

    fn into_list(
        type_: Self::ListType,
        values: impl ExactSizeIterator<Item = Self>,
        storage: &EmbeddingInputStorage,
    ) -> EmbeddingListInput;
}

#[derive(Default)]
pub(crate) struct EmbeddingInputStorage(std::cell::RefCell<Option<RuntimeListStorage>>);

pub(crate) struct EmbeddingTupleInput(Vec<EvaluatedValue>);
pub(crate) struct EmbeddingCustomInput(EvaluatedCustomValue);
pub(crate) struct EmbeddingListInput(pub(in crate::runtime::embedding) StoredListValueId);

impl EmbeddingInputStorage {
    fn lists(&self) -> std::cell::RefMut<'_, RuntimeListStorage> {
        std::cell::RefMut::map(self.0.borrow_mut(), |storage| {
            storage.get_or_insert_with(RuntimeListStorage::default)
        })
    }
}

impl EmbeddingTupleInput {
    pub(crate) fn new(fields: impl IntoIterator<Item = EmbeddingInput>) -> Self {
        Self(fields.into_iter().map(|field| field.0).collect())
    }
}

impl EmbeddingCustomInput {
    pub(crate) fn new(
        constructor: CustomConstructorId,
        fields: impl IntoIterator<Item = EmbeddingInput>,
    ) -> Self {
        Self(EvaluatedCustomValue::from_fields(
            constructor,
            fields
                .into_iter()
                .map(|field| field.0)
                .collect::<Vec<_>>()
                .into_boxed_slice(),
        ))
    }
}

macro_rules! scalar_input {
    ($type:ty, $list_type:ty, $variant:ident, $list:ident) => {
        impl EmbeddingInputValue for $type {
            type ListType = $list_type;

            fn into_input(self) -> EmbeddingInput {
                EmbeddingInput(EvaluatedValue::$variant(self))
            }

            fn into_list(
                type_: Self::ListType,
                values: impl ExactSizeIterator<Item = Self>,
                storage: &EmbeddingInputStorage,
            ) -> EmbeddingListInput {
                let values = values.collect();
                EmbeddingListInput(storage.lists().$list(type_, values).into())
            }
        }
    };
}

scalar_input!(num_bigint::BigInt, IntListTypeId, Int, int);
scalar_input!(f64, FloatListTypeId, Float, float);
scalar_input!(ecow::EcoString, StringListTypeId, String, string);
scalar_input!(char, UtfCodepointListTypeId, UtfCodepoint, utf_codepoint);
scalar_input!(bool, BoolListTypeId, Bool, bool);

impl EmbeddingInputValue for crate::BitArrayValue {
    type ListType = BitArrayListTypeId;

    fn into_input(self) -> EmbeddingInput {
        EmbeddingInput(EvaluatedValue::BitArray(EvaluatedBitArray::from_value(
            self,
        )))
    }

    fn into_list(
        type_: Self::ListType,
        values: impl ExactSizeIterator<Item = Self>,
        storage: &EmbeddingInputStorage,
    ) -> EmbeddingListInput {
        let values = values.map(EvaluatedBitArray::from_value).collect();
        EmbeddingListInput(storage.lists().bit_array(type_, values).into())
    }
}

impl EmbeddingInputValue for () {
    type ListType = NilListTypeId;

    fn into_input(self) -> EmbeddingInput {
        EmbeddingInput(EvaluatedValue::Nil)
    }

    fn into_list(
        type_: Self::ListType,
        values: impl ExactSizeIterator<Item = Self>,
        storage: &EmbeddingInputStorage,
    ) -> EmbeddingListInput {
        EmbeddingListInput(storage.lists().nil(type_, values.len()).into())
    }
}

impl EmbeddingInputValue for EmbeddingTupleInput {
    type ListType = TupleListTypeId;

    fn into_input(self) -> EmbeddingInput {
        EmbeddingInput(EvaluatedValue::Tuple(self.0))
    }

    fn into_list(
        type_: Self::ListType,
        values: impl ExactSizeIterator<Item = Self>,
        storage: &EmbeddingInputStorage,
    ) -> EmbeddingListInput {
        let values = values.map(|value| value.0).collect();
        EmbeddingListInput(storage.lists().tuple(type_, values).into())
    }
}

impl EmbeddingInputValue for EmbeddingCustomInput {
    type ListType = CustomListTypeId;

    fn into_input(self) -> EmbeddingInput {
        EmbeddingInput(EvaluatedValue::Custom(self.0))
    }

    fn into_list(
        type_: Self::ListType,
        values: impl ExactSizeIterator<Item = Self>,
        storage: &EmbeddingInputStorage,
    ) -> EmbeddingListInput {
        let allocation = CustomListAllocation::new(type_, values.map(|value| value.0).collect());
        EmbeddingListInput(storage.lists().custom(allocation).into())
    }
}

impl EmbeddingInputValue for EmbeddingListInput {
    type ListType = ListListTypeId;

    fn into_input(self) -> EmbeddingInput {
        EmbeddingInput(EvaluatedValue::List(self.0))
    }

    fn into_list(
        type_: Self::ListType,
        values: impl ExactSizeIterator<Item = Self>,
        storage: &EmbeddingInputStorage,
    ) -> EmbeddingListInput {
        let values = values.map(|value| value.0).collect();
        EmbeddingListInput(storage.lists().list(type_, values).into())
    }
}

impl EmbeddingInput {
    pub(crate) fn retain(self, values: &mut RetainedValues) {
        values.push_evaluated(self.0);
    }
}
