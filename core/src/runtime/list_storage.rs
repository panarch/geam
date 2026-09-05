use crate::plan::execution::type_::{
    BitArrayListTypeId, BoolListTypeId, FloatListTypeId, FunctionListTypeId, IntListTypeId,
    ListListTypeId, NilListTypeId, ParameterListListTypeId, StringListTypeId, TupleListTypeId,
    UtfCodepointListTypeId,
};
use crate::runtime::evaluated::{
    EvaluatedBitArray, EvaluatedCustomValue, EvaluatedExternalValue, EvaluatedFunctionValue,
    EvaluatedValue,
};
use crate::runtime::state::list::{
    BitArrayListValueId, BoolListValueId, CustomListAllocation, CustomListValueId,
    ExternalListAllocation, ExternalListValueId, FloatListValueId, FunctionListValueId,
    IntListValueId, ListListValueId, ListValueId, NilListValueId, ParameterListListValueId,
    StoredListValueId, StringListValueId, TupleListValueId, UtfCodepointListValueId,
};
use crate::runtime::{RuntimeValueProfile, TransferValues};
use ecow::EcoString;
use num_bigint::BigInt;
use std::ops::Deref;

pub(in crate::runtime) trait RuntimeListStorage<Profile: RuntimeValueProfile>:
    Default
{
    type IntValues<'value>: Deref<Target = [BigInt]>
    where
        Self: 'value;
    type StringValues<'value>: Deref<Target = [EcoString]>
    where
        Self: 'value;
    type BitArrayValues<'value>: Deref<Target = [EvaluatedBitArray]>
    where
        Self: 'value;
    type UtfCodepointValues<'value>: Deref<Target = [char]>
    where
        Self: 'value;
    type CustomValues<'value>: Deref<Target = [EvaluatedCustomValue<Profile>]>
    where
        Self: 'value;
    type ExternalValues<'value>: Deref<Target = [EvaluatedExternalValue<Profile>]>
    where
        Self: 'value;
    type FloatValues<'value>: Deref<Target = [f64]>
    where
        Self: 'value;
    type BoolValues<'value>: Deref<Target = [bool]>
    where
        Self: 'value;
    type TupleValues<'value>: Deref<Target = [Vec<EvaluatedValue<Profile>>]>
    where
        Self: 'value;
    type ListValues<'value>: Deref<Target = [StoredListValueId<Profile>]>
    where
        Self: 'value;
    type FunctionValues<'value>: Deref<Target = [EvaluatedFunctionValue<Profile>]>
    where
        Self: 'value;

    fn drain_releases(&mut self);

    fn int(&mut self, type_id: IntListTypeId, values: Vec<BigInt>) -> IntListValueId<Profile>;
    fn string(
        &mut self,
        type_id: StringListTypeId,
        values: Vec<EcoString>,
    ) -> StringListValueId<Profile>;
    fn bit_array(
        &mut self,
        type_id: BitArrayListTypeId,
        values: Vec<EvaluatedBitArray>,
    ) -> BitArrayListValueId<Profile>;
    fn utf_codepoint(
        &mut self,
        type_id: UtfCodepointListTypeId,
        values: Vec<char>,
    ) -> UtfCodepointListValueId<Profile>;
    fn custom(&mut self, allocation: CustomListAllocation<Profile>) -> CustomListValueId<Profile>;
    fn external(
        &mut self,
        allocation: ExternalListAllocation<Profile>,
    ) -> ExternalListValueId<Profile>;
    fn float(&mut self, type_id: FloatListTypeId, values: Vec<f64>) -> FloatListValueId<Profile>;
    fn bool(&mut self, type_id: BoolListTypeId, values: Vec<bool>) -> BoolListValueId<Profile>;
    fn nil(&mut self, type_id: NilListTypeId, len: usize) -> NilListValueId<Profile>;
    fn tuple(
        &mut self,
        type_id: TupleListTypeId,
        values: Vec<Vec<EvaluatedValue<Profile>>>,
    ) -> TupleListValueId<Profile>;
    fn parameter_list_list(
        &mut self,
        type_id: ParameterListListTypeId,
        len: usize,
    ) -> ParameterListListValueId<Profile>;
    fn list(
        &mut self,
        type_id: ListListTypeId,
        values: Vec<StoredListValueId<Profile>>,
    ) -> ListListValueId<Profile>;
    fn function(
        &mut self,
        type_id: FunctionListTypeId,
        values: Vec<EvaluatedFunctionValue<Profile>>,
    ) -> FunctionListValueId<Profile>;

    fn int_values<'value>(&self, value: &'value IntListValueId<Profile>)
    -> Self::IntValues<'value>;
    fn string_values<'value>(
        &self,
        value: &'value StringListValueId<Profile>,
    ) -> Self::StringValues<'value>;
    fn bit_array_values<'value>(
        &self,
        value: &'value BitArrayListValueId<Profile>,
    ) -> Self::BitArrayValues<'value>;
    fn utf_codepoint_values<'value>(
        &self,
        value: &'value UtfCodepointListValueId<Profile>,
    ) -> Self::UtfCodepointValues<'value>;
    fn custom_values<'value>(
        &self,
        value: &'value CustomListValueId<Profile>,
    ) -> Self::CustomValues<'value>;
    fn external_values<'value>(
        &self,
        value: &'value ExternalListValueId<Profile>,
    ) -> Self::ExternalValues<'value>;
    fn float_values<'value>(
        &self,
        value: &'value FloatListValueId<Profile>,
    ) -> Self::FloatValues<'value>;
    fn bool_values<'value>(
        &self,
        value: &'value BoolListValueId<Profile>,
    ) -> Self::BoolValues<'value>;
    fn tuple_values<'value>(
        &self,
        value: &'value TupleListValueId<Profile>,
    ) -> Self::TupleValues<'value>;
    fn list_values<'value>(
        &self,
        value: &'value ListListValueId<Profile>,
    ) -> Self::ListValues<'value>;
    fn function_values<'value>(
        &self,
        value: &'value FunctionListValueId<Profile>,
    ) -> Self::FunctionValues<'value>;
    fn nil_len(&self, value: &NilListValueId<Profile>) -> usize;
    fn parameter_list_list_len(&self, value: &ParameterListListValueId<Profile>) -> usize;
    fn list_len(&self, value: &ListValueId<Profile>) -> usize;
    fn evaluated_values(&self, value: &StoredListValueId<Profile>) -> Vec<EvaluatedValue<Profile>>;
    fn evaluated_value_at(
        &self,
        value: &ListValueId<Profile>,
        index: usize,
    ) -> Option<EvaluatedValue<Profile>>;
    fn drop_first(
        &mut self,
        value: &StoredListValueId<Profile>,
        count: usize,
    ) -> StoredListValueId<Profile>;
}

impl RuntimeListStorage<crate::runtime::LocalValues>
    for crate::runtime::state::list::RuntimeListStorage
{
    type IntValues<'value> = std::cell::Ref<'value, [BigInt]>;
    type StringValues<'value> = std::cell::Ref<'value, [EcoString]>;
    type BitArrayValues<'value> = std::cell::Ref<'value, [EvaluatedBitArray]>;
    type UtfCodepointValues<'value> = std::cell::Ref<'value, [char]>;
    type CustomValues<'value> =
        std::cell::Ref<'value, [EvaluatedCustomValue<crate::runtime::LocalValues>]>;
    type ExternalValues<'value> =
        std::cell::Ref<'value, [EvaluatedExternalValue<crate::runtime::LocalValues>]>;
    type FloatValues<'value> = std::cell::Ref<'value, [f64]>;
    type BoolValues<'value> = std::cell::Ref<'value, [bool]>;
    type TupleValues<'value> =
        std::cell::Ref<'value, [Vec<EvaluatedValue<crate::runtime::LocalValues>>]>;
    type ListValues<'value> =
        std::cell::Ref<'value, [StoredListValueId<crate::runtime::LocalValues>]>;
    type FunctionValues<'value> =
        std::cell::Ref<'value, [EvaluatedFunctionValue<crate::runtime::LocalValues>]>;

    fn drain_releases(&mut self) {
        self.drain_releases();
    }

    fn int(&mut self, type_id: IntListTypeId, values: Vec<BigInt>) -> IntListValueId {
        self.int(type_id, values)
    }

    fn string(&mut self, type_id: StringListTypeId, values: Vec<EcoString>) -> StringListValueId {
        self.string(type_id, values)
    }

    fn bit_array(
        &mut self,
        type_id: BitArrayListTypeId,
        values: Vec<EvaluatedBitArray>,
    ) -> BitArrayListValueId {
        self.bit_array(type_id, values)
    }

    fn utf_codepoint(
        &mut self,
        type_id: UtfCodepointListTypeId,
        values: Vec<char>,
    ) -> UtfCodepointListValueId {
        self.utf_codepoint(type_id, values)
    }

    fn custom(&mut self, allocation: CustomListAllocation) -> CustomListValueId {
        self.custom(allocation)
    }

    fn external(&mut self, allocation: ExternalListAllocation) -> ExternalListValueId {
        self.external(allocation)
    }

    fn float(&mut self, type_id: FloatListTypeId, values: Vec<f64>) -> FloatListValueId {
        self.float(type_id, values)
    }

    fn bool(&mut self, type_id: BoolListTypeId, values: Vec<bool>) -> BoolListValueId {
        self.bool(type_id, values)
    }

    fn nil(&mut self, type_id: NilListTypeId, len: usize) -> NilListValueId {
        self.nil(type_id, len)
    }

    fn tuple(
        &mut self,
        type_id: TupleListTypeId,
        values: Vec<Vec<EvaluatedValue>>,
    ) -> TupleListValueId {
        self.tuple(type_id, values)
    }

    fn parameter_list_list(
        &mut self,
        type_id: ParameterListListTypeId,
        len: usize,
    ) -> ParameterListListValueId {
        self.parameter_list_list(type_id, len)
    }

    fn list(&mut self, type_id: ListListTypeId, values: Vec<StoredListValueId>) -> ListListValueId {
        self.list(type_id, values)
    }

    fn function(
        &mut self,
        type_id: FunctionListTypeId,
        values: Vec<EvaluatedFunctionValue>,
    ) -> FunctionListValueId {
        self.function(type_id, values)
    }

    fn int_values<'value>(&self, value: &'value IntListValueId) -> Self::IntValues<'value> {
        self.int_values(value)
    }

    fn string_values<'value>(
        &self,
        value: &'value StringListValueId,
    ) -> Self::StringValues<'value> {
        self.string_values(value)
    }

    fn bit_array_values<'value>(
        &self,
        value: &'value BitArrayListValueId,
    ) -> Self::BitArrayValues<'value> {
        self.bit_array_values(value)
    }

    fn utf_codepoint_values<'value>(
        &self,
        value: &'value UtfCodepointListValueId,
    ) -> Self::UtfCodepointValues<'value> {
        self.utf_codepoint_values(value)
    }

    fn custom_values<'value>(
        &self,
        value: &'value CustomListValueId,
    ) -> Self::CustomValues<'value> {
        self.custom_values(value)
    }

    fn external_values<'value>(
        &self,
        value: &'value ExternalListValueId,
    ) -> Self::ExternalValues<'value> {
        self.external_values(value)
    }

    fn float_values<'value>(&self, value: &'value FloatListValueId) -> Self::FloatValues<'value> {
        self.float_values(value)
    }

    fn bool_values<'value>(&self, value: &'value BoolListValueId) -> Self::BoolValues<'value> {
        self.bool_values(value)
    }

    fn tuple_values<'value>(&self, value: &'value TupleListValueId) -> Self::TupleValues<'value> {
        self.tuple_values(value)
    }

    fn list_values<'value>(&self, value: &'value ListListValueId) -> Self::ListValues<'value> {
        self.list_values(value)
    }

    fn function_values<'value>(
        &self,
        value: &'value FunctionListValueId,
    ) -> Self::FunctionValues<'value> {
        self.function_values(value)
    }

    fn nil_len(&self, value: &NilListValueId) -> usize {
        self.nil_len(value)
    }

    fn parameter_list_list_len(&self, value: &ParameterListListValueId) -> usize {
        self.parameter_list_list_len(value)
    }

    fn list_len(&self, value: &ListValueId) -> usize {
        self.list_len(value)
    }

    fn evaluated_values(&self, value: &StoredListValueId) -> Vec<EvaluatedValue> {
        self.evaluated_values(value)
    }

    fn evaluated_value_at(&self, value: &ListValueId, index: usize) -> Option<EvaluatedValue> {
        self.evaluated_value_at(value, index)
    }

    fn drop_first(&mut self, value: &StoredListValueId, count: usize) -> StoredListValueId {
        self.drop_first(value, count)
    }
}

impl RuntimeListStorage<TransferValues> for crate::runtime::transfer::TransferListStorage {
    type IntValues<'value> = std::sync::Arc<[BigInt]>;
    type StringValues<'value> = std::sync::Arc<[EcoString]>;
    type BitArrayValues<'value> = std::sync::Arc<[EvaluatedBitArray]>;
    type UtfCodepointValues<'value> = std::sync::Arc<[char]>;
    type CustomValues<'value> = std::sync::Arc<[EvaluatedCustomValue<TransferValues>]>;
    type ExternalValues<'value> = std::sync::Arc<[EvaluatedExternalValue<TransferValues>]>;
    type FloatValues<'value> = std::sync::Arc<[f64]>;
    type BoolValues<'value> = std::sync::Arc<[bool]>;
    type TupleValues<'value> = std::sync::Arc<[Vec<EvaluatedValue<TransferValues>>]>;
    type ListValues<'value> = std::sync::Arc<[StoredListValueId<TransferValues>]>;
    type FunctionValues<'value> = std::sync::Arc<[EvaluatedFunctionValue<TransferValues>]>;

    fn drain_releases(&mut self) {}

    fn int(
        &mut self,
        type_id: IntListTypeId,
        values: Vec<BigInt>,
    ) -> IntListValueId<TransferValues> {
        crate::runtime::transfer::TransferListStorage::int(self, type_id, values)
    }

    fn string(
        &mut self,
        type_id: StringListTypeId,
        values: Vec<EcoString>,
    ) -> StringListValueId<TransferValues> {
        crate::runtime::transfer::TransferListStorage::string(self, type_id, values)
    }

    fn bit_array(
        &mut self,
        type_id: BitArrayListTypeId,
        values: Vec<EvaluatedBitArray>,
    ) -> BitArrayListValueId<TransferValues> {
        crate::runtime::transfer::TransferListStorage::bit_array(self, type_id, values)
    }

    fn utf_codepoint(
        &mut self,
        type_id: UtfCodepointListTypeId,
        values: Vec<char>,
    ) -> UtfCodepointListValueId<TransferValues> {
        crate::runtime::transfer::TransferListStorage::utf_codepoint(self, type_id, values)
    }

    fn custom(
        &mut self,
        allocation: CustomListAllocation<TransferValues>,
    ) -> CustomListValueId<TransferValues> {
        crate::runtime::transfer::TransferListStorage::custom(self, allocation)
    }

    fn external(
        &mut self,
        allocation: ExternalListAllocation<TransferValues>,
    ) -> ExternalListValueId<TransferValues> {
        crate::runtime::transfer::TransferListStorage::external(self, allocation)
    }

    fn float(
        &mut self,
        type_id: FloatListTypeId,
        values: Vec<f64>,
    ) -> FloatListValueId<TransferValues> {
        crate::runtime::transfer::TransferListStorage::float(self, type_id, values)
    }

    fn bool(
        &mut self,
        type_id: BoolListTypeId,
        values: Vec<bool>,
    ) -> BoolListValueId<TransferValues> {
        crate::runtime::transfer::TransferListStorage::bool(self, type_id, values)
    }

    fn nil(&mut self, type_id: NilListTypeId, len: usize) -> NilListValueId<TransferValues> {
        crate::runtime::transfer::TransferListStorage::nil(self, type_id, len)
    }

    fn tuple(
        &mut self,
        type_id: TupleListTypeId,
        values: Vec<Vec<EvaluatedValue<TransferValues>>>,
    ) -> TupleListValueId<TransferValues> {
        crate::runtime::transfer::TransferListStorage::tuple(self, type_id, values)
    }

    fn parameter_list_list(
        &mut self,
        type_id: ParameterListListTypeId,
        len: usize,
    ) -> ParameterListListValueId<TransferValues> {
        crate::runtime::transfer::TransferListStorage::parameter_list_list(self, type_id, len)
    }

    fn list(
        &mut self,
        type_id: ListListTypeId,
        values: Vec<StoredListValueId<TransferValues>>,
    ) -> ListListValueId<TransferValues> {
        crate::runtime::transfer::TransferListStorage::list(self, type_id, values)
    }

    fn function(
        &mut self,
        type_id: FunctionListTypeId,
        values: Vec<EvaluatedFunctionValue<TransferValues>>,
    ) -> FunctionListValueId<TransferValues> {
        crate::runtime::transfer::TransferListStorage::function(self, type_id, values)
    }

    fn int_values<'value>(
        &self,
        value: &'value IntListValueId<TransferValues>,
    ) -> Self::IntValues<'value> {
        self.int_values(value)
    }

    fn string_values<'value>(
        &self,
        value: &'value StringListValueId<TransferValues>,
    ) -> Self::StringValues<'value> {
        self.string_values(value)
    }

    fn bit_array_values<'value>(
        &self,
        value: &'value BitArrayListValueId<TransferValues>,
    ) -> Self::BitArrayValues<'value> {
        self.bit_array_values(value)
    }

    fn utf_codepoint_values<'value>(
        &self,
        value: &'value UtfCodepointListValueId<TransferValues>,
    ) -> Self::UtfCodepointValues<'value> {
        self.utf_codepoint_values(value)
    }

    fn custom_values<'value>(
        &self,
        value: &'value CustomListValueId<TransferValues>,
    ) -> Self::CustomValues<'value> {
        self.custom_values(value)
    }

    fn external_values<'value>(
        &self,
        value: &'value ExternalListValueId<TransferValues>,
    ) -> Self::ExternalValues<'value> {
        self.external_values(value)
    }

    fn float_values<'value>(
        &self,
        value: &'value FloatListValueId<TransferValues>,
    ) -> Self::FloatValues<'value> {
        self.float_values(value)
    }

    fn bool_values<'value>(
        &self,
        value: &'value BoolListValueId<TransferValues>,
    ) -> Self::BoolValues<'value> {
        self.bool_values(value)
    }

    fn tuple_values<'value>(
        &self,
        value: &'value TupleListValueId<TransferValues>,
    ) -> Self::TupleValues<'value> {
        self.tuple_values(value)
    }

    fn list_values<'value>(
        &self,
        value: &'value ListListValueId<TransferValues>,
    ) -> Self::ListValues<'value> {
        self.list_values(value)
    }

    fn function_values<'value>(
        &self,
        value: &'value FunctionListValueId<TransferValues>,
    ) -> Self::FunctionValues<'value> {
        self.function_values(value)
    }

    fn nil_len(&self, value: &NilListValueId<TransferValues>) -> usize {
        self.nil_len(value)
    }

    fn parameter_list_list_len(&self, value: &ParameterListListValueId<TransferValues>) -> usize {
        self.parameter_list_list_len(value)
    }

    fn list_len(&self, value: &ListValueId<TransferValues>) -> usize {
        self.list_len(value)
    }

    fn evaluated_values(
        &self,
        value: &StoredListValueId<TransferValues>,
    ) -> Vec<EvaluatedValue<TransferValues>> {
        self.evaluated_values(value)
    }

    fn evaluated_value_at(
        &self,
        value: &ListValueId<TransferValues>,
        index: usize,
    ) -> Option<EvaluatedValue<TransferValues>> {
        self.evaluated_value_at(value, index)
    }

    fn drop_first(
        &mut self,
        value: &StoredListValueId<TransferValues>,
        count: usize,
    ) -> StoredListValueId<TransferValues> {
        crate::runtime::transfer::TransferListStorage::drop_first(self, value, count)
    }
}
