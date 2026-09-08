use crate::plan::execution::type_::{
    BitArrayListTypeId, BoolListTypeId, CustomConstructorId, CustomListTypeId, ExternalListTypeId,
    FloatListTypeId, IntListTypeId, ListListTypeId, NilListTypeId, StringListTypeId,
    TupleListTypeId, UtfCodepointListTypeId,
};
use crate::runtime::evaluated::{EvaluatedBitArray, EvaluatedCustomValue, EvaluatedValue};
use crate::runtime::graph::ProfiledRetainedValues;
use crate::runtime::state::list::{
    CustomListAllocation, ExternalListAllocation, StoredListValueId,
};
use crate::runtime::{LocalValues, RuntimeListStorage, RuntimeValueProfile};

pub(crate) struct EmbeddingInput<Profile: RuntimeValueProfile = LocalValues>(
    EvaluatedValue<Profile>,
);

pub(crate) trait EmbeddingInputValue<Profile: RuntimeValueProfile = LocalValues>:
    Sized
{
    type ListType: Copy;

    fn into_input(self) -> EmbeddingInput<Profile>;

    fn into_list(
        type_: Self::ListType,
        values: impl ExactSizeIterator<Item = Self>,
        storage: &EmbeddingInputStorage<Profile>,
    ) -> EmbeddingListInput<Profile>;
}

#[derive(Default)]
pub(crate) struct EmbeddingInputStorage<Profile: RuntimeValueProfile = LocalValues>(
    std::cell::RefCell<Option<Profile::ListStorage>>,
);

pub(crate) struct EmbeddingTupleInput<Profile: RuntimeValueProfile = LocalValues>(
    Vec<EvaluatedValue<Profile>>,
);
pub(crate) struct EmbeddingCustomInput<Profile: RuntimeValueProfile = LocalValues>(
    EvaluatedCustomValue<Profile>,
);
pub(crate) struct EmbeddingListInput<Profile: RuntimeValueProfile = LocalValues>(
    pub(in crate::runtime::embedding) StoredListValueId<Profile>,
);

impl<Profile: RuntimeValueProfile> EmbeddingInputStorage<Profile> {
    fn lists(&self) -> std::cell::RefMut<'_, Profile::ListStorage> {
        std::cell::RefMut::map(self.0.borrow_mut(), |storage| {
            storage.get_or_insert_with(Profile::ListStorage::default)
        })
    }
}

impl<Profile: RuntimeValueProfile> EmbeddingTupleInput<Profile> {
    pub(crate) fn new(fields: impl IntoIterator<Item = EmbeddingInput<Profile>>) -> Self {
        Self(fields.into_iter().map(|field| field.0).collect())
    }
}

impl<Profile: RuntimeValueProfile> EmbeddingCustomInput<Profile> {
    pub(crate) fn new(
        constructor: CustomConstructorId,
        fields: impl IntoIterator<Item = EmbeddingInput<Profile>>,
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
        impl<Profile: RuntimeValueProfile> EmbeddingInputValue<Profile> for $type {
            type ListType = $list_type;

            fn into_input(self) -> EmbeddingInput<Profile> {
                EmbeddingInput(EvaluatedValue::$variant(self))
            }

            fn into_list(
                type_: Self::ListType,
                values: impl ExactSizeIterator<Item = Self>,
                storage: &EmbeddingInputStorage<Profile>,
            ) -> EmbeddingListInput<Profile> {
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

impl<Profile: RuntimeValueProfile> EmbeddingInputValue<Profile> for crate::BitArrayValue {
    type ListType = BitArrayListTypeId;

    fn into_input(self) -> EmbeddingInput<Profile> {
        EmbeddingInput(EvaluatedValue::BitArray(EvaluatedBitArray::from_value(
            self,
        )))
    }

    fn into_list(
        type_: Self::ListType,
        values: impl ExactSizeIterator<Item = Self>,
        storage: &EmbeddingInputStorage<Profile>,
    ) -> EmbeddingListInput<Profile> {
        let values = values.map(EvaluatedBitArray::from_value).collect();
        EmbeddingListInput(storage.lists().bit_array(type_, values).into())
    }
}

impl<Profile: RuntimeValueProfile> EmbeddingInputValue<Profile> for () {
    type ListType = NilListTypeId;

    fn into_input(self) -> EmbeddingInput<Profile> {
        EmbeddingInput(EvaluatedValue::Nil)
    }

    fn into_list(
        type_: Self::ListType,
        values: impl ExactSizeIterator<Item = Self>,
        storage: &EmbeddingInputStorage<Profile>,
    ) -> EmbeddingListInput<Profile> {
        EmbeddingListInput(storage.lists().nil(type_, values.len()).into())
    }
}

impl<Profile: RuntimeValueProfile> EmbeddingInputValue<Profile> for EmbeddingTupleInput<Profile> {
    type ListType = TupleListTypeId;

    fn into_input(self) -> EmbeddingInput<Profile> {
        EmbeddingInput(EvaluatedValue::Tuple(self.0))
    }

    fn into_list(
        type_: Self::ListType,
        values: impl ExactSizeIterator<Item = Self>,
        storage: &EmbeddingInputStorage<Profile>,
    ) -> EmbeddingListInput<Profile> {
        let values = values.map(|value| value.0).collect();
        EmbeddingListInput(storage.lists().tuple(type_, values).into())
    }
}

impl<Profile: RuntimeValueProfile> EmbeddingInputValue<Profile> for EmbeddingCustomInput<Profile> {
    type ListType = CustomListTypeId;

    fn into_input(self) -> EmbeddingInput<Profile> {
        EmbeddingInput(EvaluatedValue::Custom(self.0))
    }

    fn into_list(
        type_: Self::ListType,
        values: impl ExactSizeIterator<Item = Self>,
        storage: &EmbeddingInputStorage<Profile>,
    ) -> EmbeddingListInput<Profile> {
        let allocation =
            CustomListAllocation::<Profile>::new(type_, values.map(|value| value.0).collect());
        EmbeddingListInput(storage.lists().custom(allocation).into())
    }
}

impl<Profile: RuntimeValueProfile> EmbeddingInputValue<Profile> for EmbeddingListInput<Profile> {
    type ListType = ListListTypeId;

    fn into_input(self) -> EmbeddingInput<Profile> {
        EmbeddingInput(EvaluatedValue::List(self.0))
    }

    fn into_list(
        type_: Self::ListType,
        values: impl ExactSizeIterator<Item = Self>,
        storage: &EmbeddingInputStorage<Profile>,
    ) -> EmbeddingListInput<Profile> {
        let values = values.map(|value| value.0).collect();
        EmbeddingListInput(storage.lists().list(type_, values).into())
    }
}

impl<Profile: RuntimeValueProfile> EmbeddingInputValue<Profile>
    for crate::runtime::EvaluatedExternalValue<Profile>
{
    type ListType = ExternalListTypeId;

    fn into_input(self) -> EmbeddingInput<Profile> {
        EmbeddingInput(EvaluatedValue::External(self))
    }

    fn into_list(
        type_: Self::ListType,
        values: impl ExactSizeIterator<Item = Self>,
        storage: &EmbeddingInputStorage<Profile>,
    ) -> EmbeddingListInput<Profile> {
        let allocation = ExternalListAllocation::<Profile>::new(type_, values.collect());
        EmbeddingListInput(storage.lists().external(allocation).into())
    }
}

impl<Profile: RuntimeValueProfile> EmbeddingInput<Profile> {
    pub(crate) fn retain(self, values: &mut ProfiledRetainedValues<Profile>) {
        values.push_evaluated(self.0);
    }

    pub(in crate::runtime) fn into_value(self) -> EvaluatedValue<Profile> {
        self.0
    }
}
