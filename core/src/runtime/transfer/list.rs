use std::fmt;
use std::sync::{Arc, Mutex, MutexGuard};

use ecow::EcoString;
use num_bigint::BigInt;

use crate::plan::execution::type_::{
    BitArrayListTypeId, BoolListTypeId, FloatListTypeId, FunctionListTypeId, IntListTypeId,
    ListListTypeId, NilListTypeId, ParameterListListTypeId, StringListTypeId, TupleListTypeId,
    UtfCodepointListTypeId,
};
use crate::runtime::TransferValues;
use crate::runtime::evaluated::{
    EvaluatedBitArray, EvaluatedCustomValue, EvaluatedExternalValue, EvaluatedFunctionValue,
    EvaluatedValue,
};
use crate::runtime::state::list::{
    BitArrayListValueId, BoolListValueId, CustomListAllocation, CustomListValueId,
    ExternalListAllocation, ExternalListValueId, FloatListValueId, FunctionListValueId,
    IntListValueId, ListListValueId, ListValueId, NilListValueId, ParameterListListValueId,
    ParameterListValueId, StoredListValueId, StringListValueId, TupleListValueId,
    UtfCodepointListValueId,
};

#[derive(Clone)]
pub(crate) struct TransferListStorage {
    storage: Arc<SharedTransferListStorage>,
}

#[derive(Clone)]
pub(crate) struct TransferListHandleCore {
    lease: Arc<TransferListLease>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TransferListStorageKey {
    Int(usize),
    String(usize),
    BitArray(usize),
    UtfCodepoint(usize),
    Custom(usize),
    External(usize),
    Float(usize),
    Bool(usize),
    Nil(usize),
    Tuple(usize),
    ParameterList(usize),
    List(usize),
    Function(usize),
}

impl TransferListStorageKey {
    fn slot(self) -> usize {
        match self {
            Self::Int(slot)
            | Self::String(slot)
            | Self::BitArray(slot)
            | Self::UtfCodepoint(slot)
            | Self::Custom(slot)
            | Self::External(slot)
            | Self::Float(slot)
            | Self::Bool(slot)
            | Self::Nil(slot)
            | Self::Tuple(slot)
            | Self::ParameterList(slot)
            | Self::List(slot)
            | Self::Function(slot) => slot,
        }
    }
}

struct TransferListLease {
    key: TransferListStorageKey,
    storage: Arc<SharedTransferListStorage>,
}

impl Drop for TransferListLease {
    fn drop(&mut self) {
        self.storage.release(self.key);
    }
}

impl PartialEq for TransferListHandleCore {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.lease, &other.lease)
    }
}

impl fmt::Debug for TransferListHandleCore {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("TransferListHandleCore")
            .field("key", &self.lease.key)
            .finish()
    }
}

impl TransferListHandleCore {
    fn slot(&self) -> usize {
        self.lease.key.slot()
    }

    fn storage(&self) -> &SharedTransferListStorage {
        &self.lease.storage
    }
}

struct ListPool<Value> {
    slots: Vec<Arc<[Value]>>,
    free: Vec<usize>,
}

impl<Value> Default for ListPool<Value> {
    fn default() -> Self {
        Self {
            slots: Vec::new(),
            free: Vec::new(),
        }
    }
}

impl<Value> ListPool<Value> {
    fn allocate(&mut self, value: Arc<[Value]>) -> usize {
        if let Some(slot) = self.free.pop() {
            self.slots[slot] = value;
            slot
        } else {
            let slot = self.slots.len();
            self.slots.push(value);
            slot
        }
    }

    fn get(&self, slot: usize) -> Arc<[Value]> {
        Arc::clone(&self.slots[slot])
    }

    fn release(&mut self, slot: usize) -> Arc<[Value]> {
        let value = std::mem::take(&mut self.slots[slot]);
        self.free.push(slot);
        value
    }
}

#[derive(Default)]
struct LengthPool {
    slots: Vec<usize>,
    free: Vec<usize>,
}

impl LengthPool {
    fn allocate(&mut self, value: usize) -> usize {
        if let Some(slot) = self.free.pop() {
            self.slots[slot] = value;
            slot
        } else {
            let slot = self.slots.len();
            self.slots.push(value);
            slot
        }
    }

    fn get(&self, slot: usize) -> usize {
        self.slots[slot]
    }

    fn release(&mut self, slot: usize) -> usize {
        let value = std::mem::take(&mut self.slots[slot]);
        self.free.push(slot);
        value
    }
}

#[derive(Default)]
struct TransferListPools {
    ints: ListPool<BigInt>,
    strings: ListPool<EcoString>,
    bit_arrays: ListPool<EvaluatedBitArray>,
    utf_codepoints: ListPool<char>,
    customs: ListPool<EvaluatedCustomValue<TransferValues>>,
    externals: ListPool<EvaluatedExternalValue<TransferValues>>,
    floats: ListPool<f64>,
    bools: ListPool<bool>,
    nils: LengthPool,
    tuples: ListPool<Vec<EvaluatedValue<TransferValues>>>,
    parameter_list_lists: LengthPool,
    lists: ListPool<StoredListValueId<TransferValues>>,
    functions: ListPool<EvaluatedFunctionValue<TransferValues>>,
}

enum ReleasedTransferList {
    Int(Arc<[BigInt]>),
    String(Arc<[EcoString]>),
    BitArray(Arc<[EvaluatedBitArray]>),
    UtfCodepoint(Arc<[char]>),
    Custom(Arc<[EvaluatedCustomValue<TransferValues>]>),
    External(Arc<[EvaluatedExternalValue<TransferValues>]>),
    Float(Arc<[f64]>),
    Bool(Arc<[bool]>),
    Nil(usize),
    Tuple(Arc<[Vec<EvaluatedValue<TransferValues>>]>),
    ParameterList(usize),
    List(Arc<[StoredListValueId<TransferValues>]>),
    Function(Arc<[EvaluatedFunctionValue<TransferValues>]>),
}

impl ReleasedTransferList {
    fn drop_values(self) {
        match self {
            Self::Int(values) => drop(values),
            Self::String(values) => drop(values),
            Self::BitArray(values) => drop(values),
            Self::UtfCodepoint(values) => drop(values),
            Self::Custom(values) => drop(values),
            Self::External(values) => drop(values),
            Self::Float(values) => drop(values),
            Self::Bool(values) => drop(values),
            Self::Nil(len) | Self::ParameterList(len) => {
                let _released_len = len;
            }
            Self::Tuple(values) => drop(values),
            Self::List(values) => drop(values),
            Self::Function(values) => drop(values),
        }
    }
}

impl TransferListPools {
    fn release(&mut self, key: TransferListStorageKey) -> ReleasedTransferList {
        match key {
            TransferListStorageKey::Int(slot) => ReleasedTransferList::Int(self.ints.release(slot)),
            TransferListStorageKey::String(slot) => {
                ReleasedTransferList::String(self.strings.release(slot))
            }
            TransferListStorageKey::BitArray(slot) => {
                ReleasedTransferList::BitArray(self.bit_arrays.release(slot))
            }
            TransferListStorageKey::UtfCodepoint(slot) => {
                ReleasedTransferList::UtfCodepoint(self.utf_codepoints.release(slot))
            }
            TransferListStorageKey::Custom(slot) => {
                ReleasedTransferList::Custom(self.customs.release(slot))
            }
            TransferListStorageKey::External(slot) => {
                ReleasedTransferList::External(self.externals.release(slot))
            }
            TransferListStorageKey::Float(slot) => {
                ReleasedTransferList::Float(self.floats.release(slot))
            }
            TransferListStorageKey::Bool(slot) => {
                ReleasedTransferList::Bool(self.bools.release(slot))
            }
            TransferListStorageKey::Nil(slot) => ReleasedTransferList::Nil(self.nils.release(slot)),
            TransferListStorageKey::Tuple(slot) => {
                ReleasedTransferList::Tuple(self.tuples.release(slot))
            }
            TransferListStorageKey::ParameterList(slot) => {
                ReleasedTransferList::ParameterList(self.parameter_list_lists.release(slot))
            }
            TransferListStorageKey::List(slot) => {
                ReleasedTransferList::List(self.lists.release(slot))
            }
            TransferListStorageKey::Function(slot) => {
                ReleasedTransferList::Function(self.functions.release(slot))
            }
        }
    }
}

#[derive(Default)]
struct TransferListStorageState {
    releases: Vec<TransferListStorageKey>,
    draining: bool,
    pools: TransferListPools,
}

#[derive(Default)]
struct SharedTransferListStorage {
    state: Mutex<TransferListStorageState>,
}

impl SharedTransferListStorage {
    fn core(self: &Arc<Self>, key: TransferListStorageKey) -> TransferListHandleCore {
        TransferListHandleCore {
            lease: Arc::new(TransferListLease {
                key,
                storage: Arc::clone(self),
            }),
        }
    }

    fn release(&self, key: TransferListStorageKey) {
        let should_drain = {
            let mut state = lock(&self.state);
            state.releases.push(key);
            if state.draining {
                false
            } else {
                state.draining = true;
                true
            }
        };
        if !should_drain {
            return;
        }

        loop {
            let released = {
                let mut state = lock(&self.state);
                let Some(key) = state.releases.pop() else {
                    state.draining = false;
                    return;
                };
                state.pools.release(key)
            };
            released.drop_values();
        }
    }

    fn values<Value>(
        &self,
        core: &TransferListHandleCore,
        pool: impl FnOnce(&TransferListPools) -> &ListPool<Value>,
    ) -> Arc<[Value]> {
        let state = lock(&self.state);
        pool(&state.pools).get(core.slot())
    }

    fn len(
        &self,
        core: &TransferListHandleCore,
        pool: impl FnOnce(&TransferListPools) -> &LengthPool,
    ) -> usize {
        let state = lock(&self.state);
        pool(&state.pools).get(core.slot())
    }
}

impl Default for TransferListStorage {
    fn default() -> Self {
        Self {
            storage: Arc::new(SharedTransferListStorage::default()),
        }
    }
}

macro_rules! value_storage {
    ($allocate:ident, $read:ident, $type_id:ty, $item:ty, $handle:ident, $pool:ident, $key:ident) => {
        pub(in crate::runtime) fn $allocate(
            &self,
            type_id: $type_id,
            values: Vec<$item>,
        ) -> $handle<TransferValues> {
            let slot = lock(&self.storage.state)
                .pools
                .$pool
                .allocate(values.into());
            $handle::new(
                type_id,
                self.storage.core(TransferListStorageKey::$key(slot)),
            )
        }

        pub(in crate::runtime) fn $read(&self, value: &$handle<TransferValues>) -> Arc<[$item]> {
            value
                .core()
                .storage()
                .values(value.core(), |pools| &pools.$pool)
        }
    };
}

impl TransferListStorage {
    value_storage!(
        int,
        int_values,
        IntListTypeId,
        BigInt,
        IntListValueId,
        ints,
        Int
    );
    value_storage!(
        string,
        string_values,
        StringListTypeId,
        EcoString,
        StringListValueId,
        strings,
        String
    );
    value_storage!(
        bit_array,
        bit_array_values,
        BitArrayListTypeId,
        EvaluatedBitArray,
        BitArrayListValueId,
        bit_arrays,
        BitArray
    );
    value_storage!(
        utf_codepoint,
        utf_codepoint_values,
        UtfCodepointListTypeId,
        char,
        UtfCodepointListValueId,
        utf_codepoints,
        UtfCodepoint
    );
    value_storage!(
        float,
        float_values,
        FloatListTypeId,
        f64,
        FloatListValueId,
        floats,
        Float
    );
    value_storage!(
        bool,
        bool_values,
        BoolListTypeId,
        bool,
        BoolListValueId,
        bools,
        Bool
    );
    value_storage!(
        tuple,
        tuple_values,
        TupleListTypeId,
        Vec<EvaluatedValue<TransferValues>>,
        TupleListValueId,
        tuples,
        Tuple
    );
    value_storage!(
        list,
        list_values,
        ListListTypeId,
        StoredListValueId<TransferValues>,
        ListListValueId,
        lists,
        List
    );
    value_storage!(
        function,
        function_values,
        FunctionListTypeId,
        EvaluatedFunctionValue<TransferValues>,
        FunctionListValueId,
        functions,
        Function
    );

    pub(in crate::runtime) fn custom(
        &self,
        allocation: CustomListAllocation<TransferValues>,
    ) -> CustomListValueId<TransferValues> {
        let slot = lock(&self.storage.state)
            .pools
            .customs
            .allocate(allocation.values.into());
        CustomListValueId::new(
            allocation.type_id,
            self.storage.core(TransferListStorageKey::Custom(slot)),
        )
    }

    pub(in crate::runtime) fn custom_values(
        &self,
        value: &CustomListValueId<TransferValues>,
    ) -> Arc<[EvaluatedCustomValue<TransferValues>]> {
        value
            .core()
            .storage()
            .values(value.core(), |pools| &pools.customs)
    }

    pub(in crate::runtime) fn external(
        &self,
        allocation: ExternalListAllocation<TransferValues>,
    ) -> ExternalListValueId<TransferValues> {
        let slot = lock(&self.storage.state)
            .pools
            .externals
            .allocate(allocation.values.into());
        ExternalListValueId::new(
            allocation.type_id,
            self.storage.core(TransferListStorageKey::External(slot)),
        )
    }

    pub(in crate::runtime) fn external_values(
        &self,
        value: &ExternalListValueId<TransferValues>,
    ) -> Arc<[EvaluatedExternalValue<TransferValues>]> {
        value
            .core()
            .storage()
            .values(value.core(), |pools| &pools.externals)
    }

    pub(in crate::runtime) fn nil(
        &self,
        type_id: NilListTypeId,
        len: usize,
    ) -> NilListValueId<TransferValues> {
        let slot = lock(&self.storage.state).pools.nils.allocate(len);
        NilListValueId::new(
            type_id,
            self.storage.core(TransferListStorageKey::Nil(slot)),
        )
    }

    pub(in crate::runtime) fn nil_len(&self, value: &NilListValueId<TransferValues>) -> usize {
        value
            .core()
            .storage()
            .len(value.core(), |pools| &pools.nils)
    }

    pub(in crate::runtime) fn parameter_list_list(
        &self,
        type_id: ParameterListListTypeId,
        len: usize,
    ) -> ParameterListListValueId<TransferValues> {
        let slot = lock(&self.storage.state)
            .pools
            .parameter_list_lists
            .allocate(len);
        ParameterListListValueId::new(
            type_id,
            self.storage
                .core(TransferListStorageKey::ParameterList(slot)),
        )
    }

    pub(in crate::runtime) fn parameter_list_list_len(
        &self,
        value: &ParameterListListValueId<TransferValues>,
    ) -> usize {
        value
            .core()
            .storage()
            .len(value.core(), |pools| &pools.parameter_list_lists)
    }

    pub(in crate::runtime) fn list_len(&self, value: &ListValueId<TransferValues>) -> usize {
        match value {
            ListValueId::Parameter(_) => 0,
            ListValueId::Int(value) => self.int_values(value).len(),
            ListValueId::String(value) => self.string_values(value).len(),
            ListValueId::BitArray(value) => self.bit_array_values(value).len(),
            ListValueId::UtfCodepoint(value) => self.utf_codepoint_values(value).len(),
            ListValueId::Custom(value) => self.custom_values(value).len(),
            ListValueId::External(value) => self.external_values(value).len(),
            ListValueId::Float(value) => self.float_values(value).len(),
            ListValueId::Bool(value) => self.bool_values(value).len(),
            ListValueId::Nil(value) => self.nil_len(value),
            ListValueId::Tuple(value) => self.tuple_values(value).len(),
            ListValueId::ParameterList(value) => self.parameter_list_list_len(value),
            ListValueId::List(value) => self.list_values(value).len(),
            ListValueId::Function(value) => self.function_values(value).len(),
        }
    }

    pub(in crate::runtime) fn evaluated_values(
        &self,
        value: &StoredListValueId<TransferValues>,
    ) -> Vec<EvaluatedValue<TransferValues>> {
        match value {
            StoredListValueId::Int(value) => self
                .int_values(value)
                .iter()
                .cloned()
                .map(EvaluatedValue::Int)
                .collect(),
            StoredListValueId::String(value) => self
                .string_values(value)
                .iter()
                .cloned()
                .map(EvaluatedValue::String)
                .collect(),
            StoredListValueId::BitArray(value) => self
                .bit_array_values(value)
                .iter()
                .cloned()
                .map(EvaluatedValue::BitArray)
                .collect(),
            StoredListValueId::UtfCodepoint(value) => self
                .utf_codepoint_values(value)
                .iter()
                .copied()
                .map(EvaluatedValue::UtfCodepoint)
                .collect(),
            StoredListValueId::Custom(value) => self
                .custom_values(value)
                .iter()
                .cloned()
                .map(EvaluatedValue::Custom)
                .collect(),
            StoredListValueId::External(value) => self
                .external_values(value)
                .iter()
                .cloned()
                .map(EvaluatedValue::External)
                .collect(),
            StoredListValueId::Float(value) => self
                .float_values(value)
                .iter()
                .copied()
                .map(EvaluatedValue::Float)
                .collect(),
            StoredListValueId::Bool(value) => self
                .bool_values(value)
                .iter()
                .copied()
                .map(EvaluatedValue::Bool)
                .collect(),
            StoredListValueId::Nil(value) => vec![EvaluatedValue::Nil; self.nil_len(value)],
            StoredListValueId::Tuple(value) => self
                .tuple_values(value)
                .iter()
                .cloned()
                .map(EvaluatedValue::Tuple)
                .collect(),
            StoredListValueId::ParameterList(value) => vec![
                EvaluatedValue::ParameterList(
                    ParameterListValueId::new(value.type_id().item_type())
                );
                self.parameter_list_list_len(value)
            ],
            StoredListValueId::List(value) => self
                .list_values(value)
                .iter()
                .cloned()
                .map(EvaluatedValue::from)
                .collect(),
            StoredListValueId::Function(value) => self
                .function_values(value)
                .iter()
                .cloned()
                .map(EvaluatedValue::Function)
                .collect(),
        }
    }

    pub(in crate::runtime) fn evaluated_value_at(
        &self,
        value: &ListValueId<TransferValues>,
        index: usize,
    ) -> Option<EvaluatedValue<TransferValues>> {
        match value {
            ListValueId::Parameter(_) => None,
            ListValueId::Int(value) => self
                .int_values(value)
                .get(index)
                .cloned()
                .map(EvaluatedValue::Int),
            ListValueId::String(value) => self
                .string_values(value)
                .get(index)
                .cloned()
                .map(EvaluatedValue::String),
            ListValueId::BitArray(value) => self
                .bit_array_values(value)
                .get(index)
                .cloned()
                .map(EvaluatedValue::BitArray),
            ListValueId::UtfCodepoint(value) => self
                .utf_codepoint_values(value)
                .get(index)
                .copied()
                .map(EvaluatedValue::UtfCodepoint),
            ListValueId::Custom(value) => self
                .custom_values(value)
                .get(index)
                .cloned()
                .map(EvaluatedValue::Custom),
            ListValueId::External(value) => self
                .external_values(value)
                .get(index)
                .cloned()
                .map(EvaluatedValue::External),
            ListValueId::Float(value) => self
                .float_values(value)
                .get(index)
                .copied()
                .map(EvaluatedValue::Float),
            ListValueId::Bool(value) => self
                .bool_values(value)
                .get(index)
                .copied()
                .map(EvaluatedValue::Bool),
            ListValueId::Nil(value) => (index < self.nil_len(value)).then_some(EvaluatedValue::Nil),
            ListValueId::Tuple(value) => self
                .tuple_values(value)
                .get(index)
                .cloned()
                .map(EvaluatedValue::Tuple),
            ListValueId::ParameterList(value) => (index < self.parameter_list_list_len(value))
                .then_some(EvaluatedValue::ParameterList(ParameterListValueId::new(
                    value.type_id().item_type(),
                ))),
            ListValueId::List(value) => self
                .list_values(value)
                .get(index)
                .cloned()
                .map(EvaluatedValue::from),
            ListValueId::Function(value) => self
                .function_values(value)
                .get(index)
                .cloned()
                .map(EvaluatedValue::Function),
        }
    }

    pub(in crate::runtime) fn drop_first(
        &self,
        value: &StoredListValueId<TransferValues>,
        count: usize,
    ) -> StoredListValueId<TransferValues> {
        match value {
            StoredListValueId::Int(value) => self
                .int(
                    value.type_id(),
                    suffix(&self.int_values(value), count).to_vec(),
                )
                .into(),
            StoredListValueId::String(value) => self
                .string(
                    value.type_id(),
                    suffix(&self.string_values(value), count).to_vec(),
                )
                .into(),
            StoredListValueId::BitArray(value) => self
                .bit_array(
                    value.type_id(),
                    suffix(&self.bit_array_values(value), count).to_vec(),
                )
                .into(),
            StoredListValueId::UtfCodepoint(value) => self
                .utf_codepoint(
                    value.type_id(),
                    suffix(&self.utf_codepoint_values(value), count).to_vec(),
                )
                .into(),
            StoredListValueId::Custom(value) => self
                .custom(CustomListAllocation::new(
                    value.type_id(),
                    suffix(&self.custom_values(value), count).to_vec(),
                ))
                .into(),
            StoredListValueId::External(value) => self
                .external(ExternalListAllocation::new(
                    value.type_id(),
                    suffix(&self.external_values(value), count).to_vec(),
                ))
                .into(),
            StoredListValueId::Float(value) => self
                .float(
                    value.type_id(),
                    suffix(&self.float_values(value), count).to_vec(),
                )
                .into(),
            StoredListValueId::Bool(value) => self
                .bool(
                    value.type_id(),
                    suffix(&self.bool_values(value), count).to_vec(),
                )
                .into(),
            StoredListValueId::Nil(value) => self
                .nil(value.type_id(), self.nil_len(value).saturating_sub(count))
                .into(),
            StoredListValueId::Tuple(value) => self
                .tuple(
                    value.type_id(),
                    suffix(&self.tuple_values(value), count).to_vec(),
                )
                .into(),
            StoredListValueId::ParameterList(value) => self
                .parameter_list_list(
                    value.type_id(),
                    self.parameter_list_list_len(value).saturating_sub(count),
                )
                .into(),
            StoredListValueId::List(value) => self
                .list(
                    value.type_id(),
                    suffix(&self.list_values(value), count).to_vec(),
                )
                .into(),
            StoredListValueId::Function(value) => self
                .function(
                    value.type_id(),
                    suffix(&self.function_values(value), count).to_vec(),
                )
                .into(),
        }
    }
}

fn suffix<Value>(values: &Arc<[Value]>, count: usize) -> &[Value] {
    &values[count.min(values.len())..]
}

fn lock<Value>(mutex: &Mutex<Value>) -> MutexGuard<'_, Value> {
    mutex
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
}

#[cfg(test)]
mod tests {
    use super::{TransferListStorage, lock};
    use crate::runtime::evaluated::{
        EvaluatedBitArray, EvaluatedCustomValue, EvaluatedExternalValue, EvaluatedFunctionValue,
        EvaluatedIntFunction,
    };
    use crate::runtime::list_storage::RuntimeListStorage;
    use crate::runtime::profile::external_test::{RuntimeCounterProvider, RuntimeCounterSchema};
    use crate::runtime::state::list::{
        CustomListAllocation, ExternalListAllocation, ListValueId, ParameterListValueId,
        StoredListValueId,
    };
    use crate::runtime::transfer::{
        TransferExternalEquality, TransferExternalHashing, TransferExternalInspection,
        TransferExternalStore, TransferStoredRuntimeValue,
    };
    use crate::runtime::{EvaluatedValue, TransferValues};
    use crate::{
        HostModule, HostProviderModule, HostProviderSet, HostedExecution, ModuleSource,
        PackageSource, compile_typed_host_program, plan_host_program,
    };
    use ecow::EcoString;
    use num_bigint::BigInt;
    use std::sync::Arc;

    const EVERY_LIST_FAMILY_SOURCE: &str = r#"
fn ints() -> List(Int) { [] }
fn strings() -> List(String) { [] }
fn bit_arrays() -> List(BitArray) { [] }
fn utf_codepoints() -> List(UtfCodepoint) { [] }
pub type Boxed { Boxed(Int) }
fn customs() -> List(Boxed) { [Boxed(1)] }
fn floats() -> List(Float) { [] }
fn bools() -> List(Bool) { [] }
fn nils() -> List(Nil) { [] }
fn tuples() -> List(#(Int)) { [] }
fn lists() -> List(List(Int)) { [] }
fn functions() -> List(fn() -> Int) { [] }
fn parameters(values: List(value)) { values }
fn parameter_lists(values: List(List(value))) { values }
pub fn main() {
  let _ = #(
    ints,
    strings,
    bit_arrays,
    utf_codepoints,
    customs,
    floats,
    bools,
    nils,
    tuples,
    lists,
    functions,
  )
  let _ = parameters([])
  let _ = parameter_lists([[]])
  0
}
"#;

    fn assert_send<Value: Send>() {}

    fn external_list_type() -> crate::plan::execution::type_::ExternalListTypeId {
        let provider =
            HostProviderModule::<crate::host::ExternalTestProfile>::new("application", "main")
                .expect("provider module should be valid")
                .with_external_type::<RuntimeCounterProvider, RuntimeCounterSchema>()
                .expect("external type should be valid");
        let typed = compile_typed_host_program(
            "application",
            "main",
            [PackageSource::new(
                "application",
                Vec::<EcoString>::new(),
                [ModuleSource::new(
                    "main",
                    "main.gleam",
                    r#"
@external(erlang, "host", "Counter")
pub type Counter

pub fn main() -> List(Counter) {
  []
}
"#,
                )],
            )],
            HostProviderSet::with_providers(
                Vec::<HostModule<crate::host::ExternalTestProfile>>::new(),
                [provider],
            )
            .expect("provider module should be unique"),
        )
        .expect("external List should compile");
        let plan = plan_host_program(typed).expect("external List should plan");
        let execution =
            HostedExecution::try_from_module_plan(plan).expect("external List should seal");
        execution.external_list_function_id(0).type_id()
    }

    fn external_equal(
        context: &TransferExternalEquality<'_>,
        left: &BigInt,
        right: &BigInt,
    ) -> bool {
        context.stored_values_equal(
            &TransferStoredRuntimeValue::new(EvaluatedValue::Int(left.clone())),
            &TransferStoredRuntimeValue::new(EvaluatedValue::Int(right.clone())),
        )
    }

    fn external_hash(context: &TransferExternalHashing<'_>, value: &BigInt) -> u64 {
        context.stored_value_hash(&TransferStoredRuntimeValue::new(EvaluatedValue::Int(
            value.clone(),
        )))
    }

    fn external_inspect(context: &TransferExternalInspection<'_>, value: &BigInt) -> EcoString {
        format!(
            "Counter({})",
            context.inspect_stored_value(&TransferStoredRuntimeValue::new(EvaluatedValue::Int(
                value.clone()
            )))
        )
        .into()
    }

    #[test]
    fn transfer_list_graph_is_worker_transferable() {
        assert_send::<TransferListStorage>();
        assert_send::<EvaluatedValue<TransferValues>>();
        assert_send::<StoredListValueId<TransferValues>>();
    }

    #[test]
    fn cloned_and_escaped_lists_keep_the_exact_allocation_alive() {
        let plan = crate::runtime::plan_src("pub fn main() -> List(Int) { [1] }");
        let type_id = plan.int_list_function_id(0).type_id();
        let storage = TransferListStorage::default();
        let value = storage.int(type_id, vec![1.into(), 2.into()]);
        let retained = value.clone();
        let slot = value.core().slot();

        assert_eq!(value, retained);
        assert_eq!(storage.list_len(&ListValueId::Int(value.clone())), 2);
        drop(value);
        assert!(lock(&storage.storage.state).pools.ints.free.is_empty());

        drop(storage);
        let reader = TransferListStorage::default();
        assert_eq!(&*reader.int_values(&retained), &[1.into(), 2.into()]);

        let owner = Arc::clone(&retained.core().lease.storage);
        drop(retained);
        assert_eq!(lock(&owner.state).pools.ints.free, [slot]);
    }

    #[test]
    fn released_list_slots_are_reused_without_changing_live_values() {
        let plan = crate::runtime::plan_src("pub fn main() -> List(Int) { [1] }");
        let type_id = plan.int_list_function_id(0).type_id();
        let storage = TransferListStorage::default();
        let first = storage.int(type_id, vec![1.into()]);
        let slot = first.core().slot();

        drop(first);
        let second = storage.int(type_id, vec![2.into()]);

        assert_eq!(second.core().slot(), slot);
        assert_eq!(&*storage.int_values(&second), &[2.into()]);
    }

    #[test]
    fn deeply_nested_lists_release_iteratively() {
        let plan = crate::runtime::plan_src(
            r#"
fn inner() -> List(Int) { [] }
pub fn main() -> List(List(Int)) { [inner()] }
"#,
        );
        let storage = TransferListStorage::default();
        let int = storage
            .int(plan.int_list_function_id(0).type_id(), vec![1.into()])
            .into();
        let mut nested = int;

        for _ in 0..10_000 {
            nested = storage
                .list(plan.list_list_function_id(0).type_id(), vec![nested])
                .into();
        }

        drop(nested);
        let state = lock(&storage.storage.state);
        assert_eq!(state.pools.ints.free.len(), 1);
        assert_eq!(state.pools.lists.free.len(), 10_000);
        assert!(state.releases.is_empty());
        assert!(!state.draining);
    }

    #[test]
    fn indexing_and_suffixes_preserve_lazy_typed_storage() {
        let plan = crate::runtime::plan_src("pub fn main() -> List(Int) { [1, 2, 3] }");
        let storage = TransferListStorage::default();
        let value = storage.int(
            plan.int_list_function_id(0).type_id(),
            vec![1.into(), 2.into(), 3.into()],
        );
        let stored = StoredListValueId::Int(value.clone());

        assert_eq!(
            storage.evaluated_value_at(&ListValueId::Int(value), 1),
            Some(EvaluatedValue::Int(2.into())),
        );
        assert_eq!(
            storage.evaluated_value_at(&stored.clone().into_value(), 3),
            None
        );

        let suffix = storage.drop_first(&stored, 2);
        assert_eq!(
            storage.evaluated_values(&suffix),
            [EvaluatedValue::Int(3.into())],
        );
    }

    #[test]
    fn every_transfer_list_family_preserves_lazy_typed_storage() {
        let plan = crate::runtime::plan_src(EVERY_LIST_FAMILY_SOURCE);
        let mut storage = TransferListStorage::default();
        let int_function = EvaluatedIntFunction::<TransferValues>::reference(
            crate::plan::execution::function::IntFunctionId(0),
            Vec::new(),
            Vec::new(),
            crate::plan::execution::type_::FunctionType::new(
                Vec::new(),
                crate::plan::execution::type_::ValueType::Int,
            ),
        );
        let int = storage.int(plan.int_list_function_id(0).type_id(), vec![1.into()]);
        let string = storage.string(
            plan.string_list_function_id(0).type_id(),
            vec!["one".into()],
        );
        let bit_array_value = EvaluatedBitArray::new(bitvec::vec::BitVec::from_vec(vec![1]));
        let bit_array = storage.bit_array(
            plan.bit_array_list_function_id(0).type_id(),
            vec![bit_array_value.clone()],
        );
        let utf_codepoint = storage.utf_codepoint(
            plan.utf_codepoint_list_function_id(0).type_id(),
            vec!['\u{10ffff}'],
        );
        let custom_value = EvaluatedCustomValue::<TransferValues>::from_fields(
            plan.custom_constructor_id(0, 0),
            vec![EvaluatedValue::Int(1.into())].into_boxed_slice(),
        );
        let custom = storage.custom(CustomListAllocation::new(
            plan.custom_list_function_id(0).type_id(),
            vec![custom_value.clone()],
        ));
        let external_list_type = external_list_type();
        let external_store = TransferExternalStore::default();
        let external_value = EvaluatedExternalValue::<TransferValues>::new(
            external_list_type.item_type(),
            external_store.insert(
                BigInt::from(2),
                external_equal,
                external_hash,
                external_inspect,
            ),
        );
        let external_peer = EvaluatedExternalValue::<TransferValues>::new(
            external_list_type.item_type(),
            external_store.insert(
                BigInt::from(2),
                external_equal,
                external_hash,
                external_inspect,
            ),
        );
        let external = <TransferListStorage as RuntimeListStorage<TransferValues>>::external(
            &mut storage,
            ExternalListAllocation::new(external_list_type, vec![external_value.clone()]),
        );
        drop(external_store);
        let stored_equal = |left: &TransferStoredRuntimeValue,
                            right: &TransferStoredRuntimeValue| {
            left.value() == right.value()
        };
        let equality = TransferExternalEquality::new(&stored_equal);
        let stored_hash = |_: &TransferStoredRuntimeValue| 2;
        let hashing = TransferExternalHashing::new(&stored_hash);
        let stored_inspect = |_: &TransferStoredRuntimeValue| "2".into();
        let inspection = TransferExternalInspection::new(&stored_inspect);
        assert!(external_value.source_equal(&equality, &external_peer));
        assert_eq!(external_value.source_hash(&hashing), 2);
        assert_eq!(external_value.lease().inspection(&inspection), "Counter(2)");
        let float = storage.float(plan.float_list_function_id(0).type_id(), vec![1.5]);
        let bool_ = storage.bool(plan.bool_list_function_id(0).type_id(), vec![true]);
        let nil = storage.nil(plan.nil_list_function_id(0).type_id(), 1);
        let tuple_value = vec![EvaluatedValue::Int(1.into())];
        let tuple = storage.tuple(
            plan.tuple_list_function_id(0).type_id(),
            vec![tuple_value.clone()],
        );
        let parameter = ParameterListValueId::new(plan.parameter_list_function_id(0).type_id());
        let parameter_list =
            storage.parameter_list_list(plan.parameter_list_list_function_id(0).type_id(), 1);
        let child = storage.int(plan.int_list_function_id(0).type_id(), vec![1.into()]);
        let child_value = EvaluatedValue::from(StoredListValueId::from(child.clone()));
        let list = storage.list(plan.list_list_function_id(0).type_id(), vec![child.into()]);
        let function_value = EvaluatedFunctionValue::from(int_function);
        let function = storage.function(
            plan.function_list_function_id(0).type_id(),
            vec![function_value.clone()],
        );
        let values = [
            (
                StoredListValueId::from(int.clone()),
                EvaluatedValue::Int(1.into()),
            ),
            (
                StoredListValueId::from(string),
                EvaluatedValue::String("one".into()),
            ),
            (
                StoredListValueId::from(bit_array),
                EvaluatedValue::BitArray(bit_array_value),
            ),
            (
                StoredListValueId::from(utf_codepoint),
                EvaluatedValue::UtfCodepoint('\u{10ffff}'),
            ),
            (
                StoredListValueId::from(custom),
                EvaluatedValue::Custom(custom_value),
            ),
            (
                StoredListValueId::from(external),
                EvaluatedValue::External(external_value),
            ),
            (StoredListValueId::from(float), EvaluatedValue::Float(1.5)),
            (StoredListValueId::from(bool_), EvaluatedValue::Bool(true)),
            (StoredListValueId::from(nil), EvaluatedValue::Nil),
            (
                StoredListValueId::from(tuple),
                EvaluatedValue::Tuple(tuple_value),
            ),
            (
                StoredListValueId::from(parameter_list),
                EvaluatedValue::ParameterList(ParameterListValueId::new(
                    plan.parameter_list_list_function_id(0)
                        .type_id()
                        .item_type(),
                )),
            ),
            (StoredListValueId::from(list), child_value),
            (
                StoredListValueId::from(function),
                EvaluatedValue::Function(function_value),
            ),
        ];

        assert!(format!("{int:?}").contains("TransferListHandleCore"));
        assert_eq!(storage.list_len(&ListValueId::Parameter(parameter)), 0,);
        assert_eq!(
            storage.evaluated_value_at(&ListValueId::Parameter(parameter), 0),
            None,
        );

        for (stored, expected) in values {
            let value = stored.clone().into_value();
            assert_eq!(storage.list_len(&value), 1);
            assert_eq!(
                storage.evaluated_values(&stored),
                std::slice::from_ref(&expected)
            );
            assert_eq!(storage.evaluated_value_at(&value, 0), Some(expected));
            assert_eq!(storage.evaluated_value_at(&value, 1), None);

            let dropped = storage.drop_first(&stored, usize::MAX);
            assert_eq!(dropped.list_type(), stored.list_type());
            assert_eq!(storage.list_len(&dropped.into_value()), 0);
        }
    }
}
