use std::fmt;
use std::sync::{Arc, Mutex, MutexGuard};

use ecow::EcoString;
use imbl::Vector;
use num_bigint::BigInt;

use crate::plan::execution::type_::{
    BitArrayListTypeId, BoolListTypeId, CustomListTypeId, ExternalListTypeId, FloatListTypeId,
    FunctionListTypeId, IntListTypeId, ListListTypeId, ListTypeId, NilListTypeId,
    ParameterListListTypeId, ParameterListTypeId, StringListTypeId, TupleListTypeId,
    UtfCodepointListTypeId,
};

use crate::runtime::evaluated::{
    EvaluatedBitArray, EvaluatedCustomValue, EvaluatedExternalValue, EvaluatedFunctionValue,
    EvaluatedValue,
};

macro_rules! typed_list_value_id {
    ($name:ident, $type_id:ty, $variant:ident) => {
        #[derive(Debug, Clone, PartialEq)]
        pub(in crate::runtime) struct $name {
            type_id: $type_id,
            core: crate::runtime::state::list::ListHandleCore,
        }

        impl $name {
            pub(in crate::runtime) fn new(
                type_id: $type_id,
                core: crate::runtime::state::list::ListHandleCore,
            ) -> Self {
                Self { type_id, core }
            }

            pub(in crate::runtime) fn type_id(&self) -> $type_id {
                self.type_id
            }

            pub(in crate::runtime) fn core(&self) -> &crate::runtime::state::list::ListHandleCore {
                &self.core
            }

            pub(in crate::runtime) fn into_core(
                self,
            ) -> crate::runtime::state::list::ListHandleCore {
                self.core
            }

            pub(in crate::runtime) fn from_stored(value: &StoredListValueId) -> Option<Self> {
                match value {
                    StoredListValueId::$variant(value) => Some(value.clone()),
                    _ => None,
                }
            }
        }

        impl From<$name> for ListValueId {
            fn from(value: $name) -> Self {
                Self::$variant(value)
            }
        }
    };
}

typed_list_value_id!(IntListValueId, IntListTypeId, Int);
typed_list_value_id!(StringListValueId, StringListTypeId, String);
typed_list_value_id!(BitArrayListValueId, BitArrayListTypeId, BitArray);
typed_list_value_id!(
    UtfCodepointListValueId,
    UtfCodepointListTypeId,
    UtfCodepoint
);
typed_list_value_id!(CustomListValueId, CustomListTypeId, Custom);
typed_list_value_id!(ExternalListValueId, ExternalListTypeId, External);
typed_list_value_id!(FloatListValueId, FloatListTypeId, Float);
typed_list_value_id!(BoolListValueId, BoolListTypeId, Bool);
typed_list_value_id!(NilListValueId, NilListTypeId, Nil);
typed_list_value_id!(TupleListValueId, TupleListTypeId, Tuple);
typed_list_value_id!(
    ParameterListListValueId,
    ParameterListListTypeId,
    ParameterList
);
typed_list_value_id!(ListListValueId, ListListTypeId, List);
typed_list_value_id!(FunctionListValueId, FunctionListTypeId, Function);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::runtime) struct ParameterListValueId {
    type_id: ParameterListTypeId,
}

impl ParameterListValueId {
    pub(in crate::runtime) fn new(type_id: ParameterListTypeId) -> Self {
        Self { type_id }
    }

    pub(in crate::runtime) fn type_id(self) -> ParameterListTypeId {
        self.type_id
    }
}

impl From<ParameterListValueId> for ListValueId {
    fn from(value: ParameterListValueId) -> Self {
        Self::Parameter(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub(in crate::runtime) enum StoredListValueId {
    Int(IntListValueId),
    String(StringListValueId),
    BitArray(BitArrayListValueId),
    UtfCodepoint(UtfCodepointListValueId),
    Custom(CustomListValueId),
    External(ExternalListValueId),
    Float(FloatListValueId),
    Bool(BoolListValueId),
    Nil(NilListValueId),
    Tuple(TupleListValueId),
    ParameterList(ParameterListListValueId),
    List(ListListValueId),
    Function(FunctionListValueId),
}

macro_rules! stored_list_value_id_from {
    ($value:ident, $variant:ident) => {
        impl From<$value> for StoredListValueId {
            fn from(value: $value) -> Self {
                Self::$variant(value)
            }
        }
    };
}

stored_list_value_id_from!(IntListValueId, Int);
stored_list_value_id_from!(StringListValueId, String);
stored_list_value_id_from!(BitArrayListValueId, BitArray);
stored_list_value_id_from!(UtfCodepointListValueId, UtfCodepoint);
stored_list_value_id_from!(CustomListValueId, Custom);
stored_list_value_id_from!(ExternalListValueId, External);
stored_list_value_id_from!(FloatListValueId, Float);
stored_list_value_id_from!(BoolListValueId, Bool);
stored_list_value_id_from!(NilListValueId, Nil);
stored_list_value_id_from!(TupleListValueId, Tuple);
stored_list_value_id_from!(ParameterListListValueId, ParameterList);
stored_list_value_id_from!(ListListValueId, List);
stored_list_value_id_from!(FunctionListValueId, Function);

pub(in crate::runtime) struct CustomListAllocation {
    pub(in crate::runtime) type_id: CustomListTypeId,
    pub(in crate::runtime) values: Vec<EvaluatedCustomValue>,
}

impl CustomListAllocation {
    pub(in crate::runtime) fn new(
        type_id: CustomListTypeId,
        values: Vec<EvaluatedCustomValue>,
    ) -> Self {
        Self { type_id, values }
    }
}

pub(in crate::runtime) struct ExternalListAllocation {
    pub(in crate::runtime) type_id: ExternalListTypeId,
    pub(in crate::runtime) values: Vec<EvaluatedExternalValue>,
}

impl ExternalListAllocation {
    pub(in crate::runtime) fn new(
        type_id: ExternalListTypeId,
        values: Vec<EvaluatedExternalValue>,
    ) -> Self {
        Self { type_id, values }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub(in crate::runtime) enum ListValueId {
    Parameter(ParameterListValueId),
    Int(IntListValueId),
    String(StringListValueId),
    BitArray(BitArrayListValueId),
    UtfCodepoint(UtfCodepointListValueId),
    Custom(CustomListValueId),
    External(ExternalListValueId),
    Float(FloatListValueId),
    Bool(BoolListValueId),
    Nil(NilListValueId),
    Tuple(TupleListValueId),
    ParameterList(ParameterListListValueId),
    List(ListListValueId),
    Function(FunctionListValueId),
}

impl StoredListValueId {
    pub(in crate::runtime) fn list_type(&self) -> ListTypeId {
        match self {
            Self::Int(value) => value.type_id().list_type(),
            Self::String(value) => value.type_id().list_type(),
            Self::BitArray(value) => value.type_id().list_type(),
            Self::UtfCodepoint(value) => value.type_id().list_type(),
            Self::Custom(value) => value.type_id().list_type(),
            Self::External(value) => value.type_id().list_type(),
            Self::Float(value) => value.type_id().list_type(),
            Self::Bool(value) => value.type_id().list_type(),
            Self::Nil(value) => value.type_id().list_type(),
            Self::Tuple(value) => value.type_id().list_type(),
            Self::ParameterList(value) => value.type_id().list_type(),
            Self::List(value) => value.type_id().list_type(),
            Self::Function(value) => value.type_id().list_type(),
        }
    }

    pub(in crate::runtime) fn into_value(self) -> ListValueId {
        match self {
            Self::Int(value) => ListValueId::Int(value),
            Self::String(value) => ListValueId::String(value),
            Self::BitArray(value) => ListValueId::BitArray(value),
            Self::UtfCodepoint(value) => ListValueId::UtfCodepoint(value),
            Self::Custom(value) => ListValueId::Custom(value),
            Self::External(value) => ListValueId::External(value),
            Self::Float(value) => ListValueId::Float(value),
            Self::Bool(value) => ListValueId::Bool(value),
            Self::Nil(value) => ListValueId::Nil(value),
            Self::Tuple(value) => ListValueId::Tuple(value),
            Self::ParameterList(value) => ListValueId::ParameterList(value),
            Self::List(value) => ListValueId::List(value),
            Self::Function(value) => ListValueId::Function(value),
        }
    }

    pub(in crate::runtime) fn into_core(self) -> crate::runtime::state::list::ListHandleCore {
        match self {
            Self::Int(value) => value.into_core(),
            Self::String(value) => value.into_core(),
            Self::BitArray(value) => value.into_core(),
            Self::UtfCodepoint(value) => value.into_core(),
            Self::Custom(value) => value.into_core(),
            Self::External(value) => value.into_core(),
            Self::Float(value) => value.into_core(),
            Self::Bool(value) => value.into_core(),
            Self::Nil(value) => value.into_core(),
            Self::Tuple(value) => value.into_core(),
            Self::ParameterList(value) => value.into_core(),
            Self::List(value) => value.into_core(),
            Self::Function(value) => value.into_core(),
        }
    }
}

impl From<StoredListValueId> for ListValueId {
    fn from(value: StoredListValueId) -> Self {
        value.into_value()
    }
}

#[derive(Clone)]
pub(crate) struct RuntimeListStorage {
    storage: Arc<SharedListStorage>,
}

#[derive(Clone)]
pub(crate) struct ListHandleCore {
    lease: Arc<ListLease>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ListStorageKey {
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

impl ListStorageKey {
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

struct ListLease {
    key: ListStorageKey,
    storage: Arc<SharedListStorage>,
}

impl Drop for ListLease {
    fn drop(&mut self) {
        self.storage.release(self.key);
    }
}

impl PartialEq for ListHandleCore {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.lease, &other.lease)
    }
}

impl fmt::Debug for ListHandleCore {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ListHandleCore")
            .field("key", &self.lease.key)
            .finish()
    }
}

impl ListHandleCore {
    fn slot(&self) -> usize {
        self.lease.key.slot()
    }

    fn storage(&self) -> &SharedListStorage {
        &self.lease.storage
    }
}

struct ListPool<Value: Clone> {
    // Retaining even an inline vector shares its items instead of cloning them.
    slots: Vec<Arc<Vector<Value>>>,
    free: Vec<usize>,
}

impl<Value: Clone> Default for ListPool<Value> {
    fn default() -> Self {
        Self {
            slots: Vec::new(),
            free: Vec::new(),
        }
    }
}

impl<Value: Clone> ListPool<Value> {
    fn allocate(&mut self, value: Arc<Vector<Value>>) -> usize {
        if let Some(slot) = self.free.pop() {
            self.slots[slot] = value;
            slot
        } else {
            let slot = self.slots.len();
            self.slots.push(value);
            slot
        }
    }

    fn get(&self, slot: usize) -> Arc<Vector<Value>> {
        Arc::clone(&self.slots[slot])
    }

    fn release(&mut self, slot: usize) -> Arc<Vector<Value>> {
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
struct ListPools {
    ints: ListPool<BigInt>,
    strings: ListPool<EcoString>,
    bit_arrays: ListPool<EvaluatedBitArray>,
    utf_codepoints: ListPool<char>,
    customs: ListPool<EvaluatedCustomValue>,
    externals: ListPool<EvaluatedExternalValue>,
    floats: ListPool<f64>,
    bools: ListPool<bool>,
    nils: LengthPool,
    tuples: ListPool<Vec<EvaluatedValue>>,
    parameter_list_lists: LengthPool,
    lists: ListPool<StoredListValueId>,
    functions: ListPool<EvaluatedFunctionValue>,
}

enum ReleasedList {
    Int(Arc<Vector<BigInt>>),
    String(Arc<Vector<EcoString>>),
    BitArray(Arc<Vector<EvaluatedBitArray>>),
    UtfCodepoint(Arc<Vector<char>>),
    Custom(Arc<Vector<EvaluatedCustomValue>>),
    External(Arc<Vector<EvaluatedExternalValue>>),
    Float(Arc<Vector<f64>>),
    Bool(Arc<Vector<bool>>),
    Nil(usize),
    Tuple(Arc<Vector<Vec<EvaluatedValue>>>),
    ParameterList(usize),
    List(Arc<Vector<StoredListValueId>>),
    Function(Arc<Vector<EvaluatedFunctionValue>>),
}

impl ReleasedList {
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

impl ListPools {
    fn release(&mut self, key: ListStorageKey) -> ReleasedList {
        match key {
            ListStorageKey::Int(slot) => ReleasedList::Int(self.ints.release(slot)),
            ListStorageKey::String(slot) => ReleasedList::String(self.strings.release(slot)),
            ListStorageKey::BitArray(slot) => ReleasedList::BitArray(self.bit_arrays.release(slot)),
            ListStorageKey::UtfCodepoint(slot) => {
                ReleasedList::UtfCodepoint(self.utf_codepoints.release(slot))
            }
            ListStorageKey::Custom(slot) => ReleasedList::Custom(self.customs.release(slot)),
            ListStorageKey::External(slot) => ReleasedList::External(self.externals.release(slot)),
            ListStorageKey::Float(slot) => ReleasedList::Float(self.floats.release(slot)),
            ListStorageKey::Bool(slot) => ReleasedList::Bool(self.bools.release(slot)),
            ListStorageKey::Nil(slot) => ReleasedList::Nil(self.nils.release(slot)),
            ListStorageKey::Tuple(slot) => ReleasedList::Tuple(self.tuples.release(slot)),
            ListStorageKey::ParameterList(slot) => {
                ReleasedList::ParameterList(self.parameter_list_lists.release(slot))
            }
            ListStorageKey::List(slot) => ReleasedList::List(self.lists.release(slot)),
            ListStorageKey::Function(slot) => ReleasedList::Function(self.functions.release(slot)),
        }
    }
}

#[derive(Default)]
struct ListStorageState {
    releases: Vec<ListStorageKey>,
    draining: bool,
    pools: ListPools,
}

#[derive(Default)]
struct SharedListStorage {
    state: Mutex<ListStorageState>,
}

struct FinishReleaseOnUnwind<'storage> {
    storage: &'storage SharedListStorage,
    armed: bool,
}

impl SharedListStorage {
    fn core(self: &Arc<Self>, key: ListStorageKey) -> ListHandleCore {
        ListHandleCore {
            lease: Arc::new(ListLease {
                key,
                storage: Arc::clone(self),
            }),
        }
    }

    fn release(&self, key: ListStorageKey) {
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
        self.drain();
    }

    fn drain(&self) {
        let mut finish = FinishReleaseOnUnwind {
            storage: self,
            armed: true,
        };
        loop {
            let released = {
                let mut state = lock(&self.state);
                let Some(key) = state.releases.pop() else {
                    state.draining = false;
                    finish.armed = false;
                    return;
                };
                state.pools.release(key)
            };
            released.drop_values();
        }
    }

    fn values<Value: Clone>(
        &self,
        core: &ListHandleCore,
        pool: impl FnOnce(&ListPools) -> &ListPool<Value>,
    ) -> Arc<Vector<Value>> {
        let state = lock(&self.state);
        pool(&state.pools).get(core.slot())
    }

    fn len(&self, core: &ListHandleCore, pool: impl FnOnce(&ListPools) -> &LengthPool) -> usize {
        let state = lock(&self.state);
        pool(&state.pools).get(core.slot())
    }
}

impl Drop for FinishReleaseOnUnwind<'_> {
    fn drop(&mut self) {
        if self.armed {
            self.storage.drain();
        }
    }
}

impl Default for RuntimeListStorage {
    fn default() -> Self {
        Self {
            storage: Arc::new(SharedListStorage::default()),
        }
    }
}

macro_rules! value_storage {
    ($allocate:ident, $store:ident, $read:ident, $prepend:ident, $tail:ident,
     $type_id:ty, $item:ty, $handle:ident, $pool:ident, $key:ident) => {
        pub(in crate::runtime) fn $allocate(
            &self,
            type_id: $type_id,
            values: Vec<$item>,
        ) -> $handle {
            self.$store(type_id, values.into())
        }

        sequence_storage!(
            $store, $read, $prepend, $tail, $type_id, $item, $handle, $pool, $key
        );
    };
}

macro_rules! sequence_storage {
    ($store:ident, $read:ident, $prepend:ident, $tail:ident,
     $type_id:ty, $item:ty, $handle:ident, $pool:ident, $key:ident) => {
        fn $store(&self, type_id: $type_id, values: Vector<$item>) -> $handle {
            let values = Arc::new(values);
            let slot = lock(&self.storage.state).pools.$pool.allocate(values);
            $handle::new(type_id, self.storage.core(ListStorageKey::$key(slot)))
        }

        pub(in crate::runtime) fn $read(&self, value: &$handle) -> Arc<Vector<$item>> {
            value
                .core()
                .storage()
                .values(value.core(), |pools| &pools.$pool)
        }

        pub(in crate::runtime) fn $prepend(
            &self,
            type_id: $type_id,
            prefix: Vec<$item>,
            tail: &$handle,
        ) -> $handle {
            self.$store(type_id, prepend(&self.$read(tail), prefix))
        }

        pub(in crate::runtime) fn $tail(
            &self,
            type_id: $type_id,
            value: &$handle,
            count: usize,
        ) -> $handle {
            self.$store(type_id, suffix(&self.$read(value), count))
        }
    };
}

impl RuntimeListStorage {
    pub(in crate::runtime) fn from_handle(handle: &ListHandleCore) -> Self {
        Self {
            storage: Arc::clone(&handle.lease.storage),
        }
    }

    value_storage!(
        int,
        store_int,
        int_values,
        prepend_int,
        tail_int,
        IntListTypeId,
        BigInt,
        IntListValueId,
        ints,
        Int
    );
    value_storage!(
        string,
        store_string,
        string_values,
        prepend_string,
        tail_string,
        StringListTypeId,
        EcoString,
        StringListValueId,
        strings,
        String
    );
    value_storage!(
        bit_array,
        store_bit_array,
        bit_array_values,
        prepend_bit_array,
        tail_bit_array,
        BitArrayListTypeId,
        EvaluatedBitArray,
        BitArrayListValueId,
        bit_arrays,
        BitArray
    );
    value_storage!(
        utf_codepoint,
        store_utf_codepoint,
        utf_codepoint_values,
        prepend_utf_codepoint,
        tail_utf_codepoint,
        UtfCodepointListTypeId,
        char,
        UtfCodepointListValueId,
        utf_codepoints,
        UtfCodepoint
    );
    value_storage!(
        float,
        store_float,
        float_values,
        prepend_float,
        tail_float,
        FloatListTypeId,
        f64,
        FloatListValueId,
        floats,
        Float
    );
    value_storage!(
        bool,
        store_bool,
        bool_values,
        prepend_bool,
        tail_bool,
        BoolListTypeId,
        bool,
        BoolListValueId,
        bools,
        Bool
    );
    value_storage!(
        tuple,
        store_tuple,
        tuple_values,
        prepend_tuple,
        tail_tuple,
        TupleListTypeId,
        Vec<EvaluatedValue>,
        TupleListValueId,
        tuples,
        Tuple
    );
    value_storage!(
        list,
        store_list,
        list_values,
        prepend_list,
        tail_list,
        ListListTypeId,
        StoredListValueId,
        ListListValueId,
        lists,
        List
    );
    value_storage!(
        function,
        store_function,
        function_values,
        prepend_function,
        tail_function,
        FunctionListTypeId,
        EvaluatedFunctionValue,
        FunctionListValueId,
        functions,
        Function
    );

    pub(in crate::runtime) fn custom(&self, allocation: CustomListAllocation) -> CustomListValueId {
        self.store_custom(allocation.type_id, allocation.values.into())
    }

    sequence_storage!(
        store_custom,
        custom_values,
        prepend_custom,
        tail_custom,
        CustomListTypeId,
        EvaluatedCustomValue,
        CustomListValueId,
        customs,
        Custom
    );

    pub(in crate::runtime) fn external(
        &self,
        allocation: ExternalListAllocation,
    ) -> ExternalListValueId {
        self.store_external(allocation.type_id, allocation.values.into())
    }

    sequence_storage!(
        store_external,
        external_values,
        prepend_external,
        tail_external,
        ExternalListTypeId,
        EvaluatedExternalValue,
        ExternalListValueId,
        externals,
        External
    );

    pub(in crate::runtime) fn nil(&self, type_id: NilListTypeId, len: usize) -> NilListValueId {
        let slot = lock(&self.storage.state).pools.nils.allocate(len);
        NilListValueId::new(type_id, self.storage.core(ListStorageKey::Nil(slot)))
    }

    pub(in crate::runtime) fn nil_len(&self, value: &NilListValueId) -> usize {
        value
            .core()
            .storage()
            .len(value.core(), |pools| &pools.nils)
    }

    pub(in crate::runtime) fn parameter_list_list(
        &self,
        type_id: ParameterListListTypeId,
        len: usize,
    ) -> ParameterListListValueId {
        let slot = lock(&self.storage.state)
            .pools
            .parameter_list_lists
            .allocate(len);
        ParameterListListValueId::new(
            type_id,
            self.storage.core(ListStorageKey::ParameterList(slot)),
        )
    }

    pub(in crate::runtime) fn parameter_list_list_len(
        &self,
        value: &ParameterListListValueId,
    ) -> usize {
        value
            .core()
            .storage()
            .len(value.core(), |pools| &pools.parameter_list_lists)
    }

    pub(in crate::runtime) fn list_len(&self, value: &ListValueId) -> usize {
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
        value: &StoredListValueId,
    ) -> Vec<EvaluatedValue> {
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
        value: &ListValueId,
        index: usize,
    ) -> Option<EvaluatedValue> {
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

    pub(in crate::runtime) fn tail_nil(
        &self,
        type_id: NilListTypeId,
        value: &NilListValueId,
        count: usize,
    ) -> NilListValueId {
        self.nil(type_id, self.nil_len(value).saturating_sub(count))
    }

    pub(in crate::runtime) fn prepend_nil(
        &self,
        type_id: NilListTypeId,
        len: usize,
        tail: &NilListValueId,
    ) -> NilListValueId {
        self.nil(type_id, len + self.nil_len(tail))
    }

    pub(in crate::runtime) fn tail_parameter_list_list(
        &self,
        type_id: ParameterListListTypeId,
        value: &ParameterListListValueId,
        count: usize,
    ) -> ParameterListListValueId {
        self.parameter_list_list(
            type_id,
            self.parameter_list_list_len(value).saturating_sub(count),
        )
    }

    pub(in crate::runtime) fn prepend_parameter_list_list(
        &self,
        type_id: ParameterListListTypeId,
        len: usize,
        tail: &ParameterListListValueId,
    ) -> ParameterListListValueId {
        self.parameter_list_list(type_id, len + self.parameter_list_list_len(tail))
    }

    pub(in crate::runtime) fn drop_first(
        &self,
        value: &StoredListValueId,
        count: usize,
    ) -> StoredListValueId {
        match value {
            StoredListValueId::Int(value) => self.tail_int(value.type_id(), value, count).into(),
            StoredListValueId::String(value) => {
                self.tail_string(value.type_id(), value, count).into()
            }
            StoredListValueId::BitArray(value) => {
                self.tail_bit_array(value.type_id(), value, count).into()
            }
            StoredListValueId::UtfCodepoint(value) => self
                .tail_utf_codepoint(value.type_id(), value, count)
                .into(),
            StoredListValueId::Custom(value) => {
                self.tail_custom(value.type_id(), value, count).into()
            }
            StoredListValueId::External(value) => {
                self.tail_external(value.type_id(), value, count).into()
            }
            StoredListValueId::Float(value) => {
                self.tail_float(value.type_id(), value, count).into()
            }
            StoredListValueId::Bool(value) => self.tail_bool(value.type_id(), value, count).into(),
            StoredListValueId::Nil(value) => self.tail_nil(value.type_id(), value, count).into(),
            StoredListValueId::Tuple(value) => {
                self.tail_tuple(value.type_id(), value, count).into()
            }
            StoredListValueId::ParameterList(value) => self
                .tail_parameter_list_list(value.type_id(), value, count)
                .into(),
            StoredListValueId::List(value) => self.tail_list(value.type_id(), value, count).into(),
            StoredListValueId::Function(value) => {
                self.tail_function(value.type_id(), value, count).into()
            }
        }
    }
}

fn suffix<Value: Clone>(values: &Vector<Value>, count: usize) -> Vector<Value> {
    values.skip(count.min(values.len()))
}

fn prepend<Value: Clone>(values: &Vector<Value>, prefix: Vec<Value>) -> Vector<Value> {
    let mut result = values.clone();
    for item in prefix.into_iter().rev() {
        result.push_front(item);
    }
    result
}

fn lock<Value>(mutex: &Mutex<Value>) -> MutexGuard<'_, Value> {
    mutex
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
}

#[cfg(test)]
mod storage_tests {
    use super::{RuntimeListStorage, lock};
    use crate::runtime::evaluated::{
        EvaluatedBitArray, EvaluatedCustomValue, EvaluatedExternalValue, EvaluatedFunctionValue,
        EvaluatedIntFunction,
    };

    use crate::runtime::EvaluatedValue;
    use crate::runtime::profile::external_test::{RuntimeCounterProvider, RuntimeCounterSchema};
    use crate::runtime::retained::{
        RetainedValueEquality, RetainedValueHashing, RetainedValueInspection, RetainedValueRef,
    };
    use crate::runtime::state::list::{
        CustomListAllocation, ExternalListAllocation, ListValueId, ParameterListValueId,
        StoredListValueId,
    };
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
        context: &crate::host::HostExternalEquality<'_>,
        left: &BigInt,
        right: &BigInt,
    ) -> bool {
        context.0.stored_values_equal(
            &RetainedValueRef::new(&EvaluatedValue::Int(left.clone())),
            &RetainedValueRef::new(&EvaluatedValue::Int(right.clone())),
        )
    }

    fn external_hash(context: &crate::host::HostExternalHashing<'_>, value: &BigInt) -> u64 {
        context
            .0
            .stored_value_hash(&RetainedValueRef::new(&EvaluatedValue::Int(value.clone())))
    }

    fn external_inspect(
        context: &crate::host::HostExternalInspection<'_>,
        value: &BigInt,
    ) -> EcoString {
        format!(
            "Counter({})",
            context
                .0
                .inspect_stored_value(&RetainedValueRef::new(&EvaluatedValue::Int(value.clone())))
        )
        .into()
    }

    #[test]
    fn transfer_list_graph_is_worker_transferable() {
        assert_send::<RuntimeListStorage>();
        assert_send::<EvaluatedValue>();
        assert_send::<StoredListValueId>();
    }

    #[test]
    fn cloned_and_escaped_lists_keep_the_exact_allocation_alive() {
        let plan = crate::runtime::plan_src("pub fn main() -> List(Int) { [1] }");
        let type_id = plan.int_list_function_id(0).type_id();
        let storage = RuntimeListStorage::default();
        let value = storage.int(type_id, vec![1.into(), 2.into()]);
        let retained = value.clone();
        let slot = value.core().slot();

        assert_eq!(value, retained);
        assert_eq!(storage.list_len(&ListValueId::Int(value.clone())), 2);
        drop(value);
        assert!(lock(&storage.storage.state).pools.ints.free.is_empty());

        drop(storage);
        let reader = RuntimeListStorage::default();
        assert_eq!(
            reader.int_values(&retained).as_ref(),
            &imbl::Vector::from(vec![1.into(), 2.into()])
        );

        let owner = Arc::clone(&retained.core().lease.storage);
        drop(retained);
        assert_eq!(lock(&owner.state).pools.ints.free, [slot]);
    }

    #[test]
    fn released_list_slots_are_reused_without_changing_live_values() {
        let plan = crate::runtime::plan_src("pub fn main() -> List(Int) { [1] }");
        let type_id = plan.int_list_function_id(0).type_id();
        let storage = RuntimeListStorage::default();
        let first = storage.int(type_id, vec![1.into()]);
        let slot = first.core().slot();

        drop(first);
        let second = storage.int(type_id, vec![2.into()]);

        assert_eq!(second.core().slot(), slot);
        assert_eq!(
            storage.int_values(&second).as_ref(),
            &imbl::Vector::from(vec![2.into()])
        );
    }

    #[test]
    fn deeply_nested_lists_release_iteratively() {
        let plan = crate::runtime::plan_src(
            r#"
fn inner() -> List(Int) { [] }
pub fn main() -> List(List(Int)) { [inner()] }
"#,
        );
        let storage = RuntimeListStorage::default();
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
    fn persistent_operations_bound_item_cloning_and_keep_versions_independent() {
        use std::sync::atomic::{AtomicUsize, Ordering};

        struct Item {
            value: usize,
            clones: Arc<AtomicUsize>,
        }

        impl Clone for Item {
            fn clone(&self) -> Self {
                self.clones.fetch_add(1, Ordering::Relaxed);
                Self {
                    value: self.value,
                    clones: Arc::clone(&self.clones),
                }
            }
        }

        for len in [1_000, 10_000] {
            let clones = Arc::new(AtomicUsize::new(0));
            let input: Vec<_> = (0..len)
                .map(|value| Item {
                    value,
                    clones: Arc::clone(&clones),
                })
                .collect();
            let mut pool = super::ListPool::default();
            let slot = pool.allocate(Arc::new(input.into()));
            assert_eq!(clones.load(Ordering::Relaxed), 0);

            let original = pool.get(slot);
            let alias = pool.get(slot);
            assert!(Arc::ptr_eq(&original, &alias));
            assert_eq!(clones.load(Ordering::Relaxed), 0);
            assert_eq!(
                original.iter().map(|item| item.value).collect::<Vec<_>>(),
                (0..len).collect::<Vec<_>>()
            );
            assert_eq!(clones.load(Ordering::Relaxed), 0);

            let mut remaining = original.as_ref().clone();
            for first in 0..len {
                assert_eq!(remaining.front().map(|item| item.value), Some(first));
                remaining = super::suffix(&remaining, 1);
                assert_eq!(remaining.len(), len - first - 1);
            }
            assert!(remaining.is_empty());
            assert!(clones.load(Ordering::Relaxed) < 128 * len);
            assert_eq!(
                alias.iter().map(|item| item.value).collect::<Vec<_>>(),
                (0..len).collect::<Vec<_>>()
            );

            clones.store(0, Ordering::Relaxed);
            let mut growing = imbl::Vector::new();
            for value in (0..len).rev() {
                growing = super::prepend(
                    &growing,
                    vec![Item {
                        value,
                        clones: Arc::clone(&clones),
                    }],
                );
            }
            assert!(clones.load(Ordering::Relaxed) < 128 * len);
            assert_eq!(
                growing.iter().map(|item| item.value).collect::<Vec<_>>(),
                (0..len).collect::<Vec<_>>()
            );
        }
    }

    #[test]
    fn tail_and_prepend_preserve_order_at_chunk_and_tree_boundaries() {
        let plan = crate::runtime::plan_src("pub fn main() -> List(Int) { [] }");
        let type_id = plan.int_list_function_id(0).type_id();
        let storage = RuntimeListStorage::default();

        for len in [
            0_usize, 1, 2, 63, 64, 65, 127, 128, 129, 1_000, 4_096, 4_097,
        ] {
            let expected: Vec<BigInt> = (0..len).map(BigInt::from).collect();
            let original = storage.int(type_id, expected.clone());
            let alias = original.clone();
            for count in [0, 1, 63, 64, 65, len, len + 1, usize::MAX] {
                let tail = storage.tail_int(type_id, &original, count);
                assert_eq!(
                    storage
                        .int_values(&tail)
                        .iter()
                        .cloned()
                        .collect::<Vec<_>>(),
                    expected[count.min(len)..],
                );
            }
            for prefix in [vec![], vec![(-2).into(), (-1).into()]] {
                let combined = storage.prepend_int(type_id, prefix.clone(), &original);
                let expected_combined: Vec<_> =
                    prefix.into_iter().chain(expected.iter().cloned()).collect();
                assert_eq!(
                    storage
                        .int_values(&combined)
                        .iter()
                        .cloned()
                        .collect::<Vec<_>>(),
                    expected_combined
                );
            }
            assert_eq!(
                storage
                    .int_values(&alias)
                    .iter()
                    .cloned()
                    .collect::<Vec<_>>(),
                expected
            );
        }
    }

    #[test]
    fn nil_and_parameter_list_updates_use_only_lengths() {
        let plan = crate::runtime::plan_src(
            r#"
fn nils() -> List(Nil) { [] }
fn parameters(values: List(List(a))) { values }
pub fn main() {
  let _ = nils()
  parameters([[]])
}
"#,
        );
        let storage = RuntimeListStorage::default();
        let len = usize::MAX / 2;
        let nil_type = plan.nil_list_function_id(0).type_id();
        let nils = storage.nil(nil_type, len);
        let grown_nils = storage.prepend_nil(nil_type, 17, &nils);
        assert_eq!(storage.nil_len(&nils), len);
        assert_eq!(storage.nil_len(&grown_nils), len + 17);
        assert_eq!(
            storage.nil_len(&storage.tail_nil(nil_type, &grown_nils, 17)),
            len
        );
        assert_eq!(
            storage.nil_len(&storage.tail_nil(nil_type, &grown_nils, usize::MAX)),
            0
        );

        let parameter_type = plan.parameter_list_list_function_id(0).type_id();
        let parameters = storage.parameter_list_list(parameter_type, len);
        let grown_parameters = storage.prepend_parameter_list_list(parameter_type, 17, &parameters);
        assert_eq!(storage.parameter_list_list_len(&parameters), len);
        assert_eq!(storage.parameter_list_list_len(&grown_parameters), len + 17);
        assert_eq!(
            storage.parameter_list_list_len(&storage.tail_parameter_list_list(
                parameter_type,
                &grown_parameters,
                17
            )),
            len
        );
        assert_eq!(
            storage.parameter_list_list_len(&storage.tail_parameter_list_list(
                parameter_type,
                &grown_parameters,
                usize::MAX
            )),
            0
        );
    }

    #[test]
    fn indexing_and_suffixes_preserve_lazy_typed_storage() {
        let plan = crate::runtime::plan_src("pub fn main() -> List(Int) { [1, 2, 3] }");
        let storage = RuntimeListStorage::default();
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
        let storage = RuntimeListStorage::default();
        let int_function = EvaluatedIntFunction::reference(
            crate::plan::execution::function::IntFunctionId(0),
            Vec::new(),
            Default::default(),
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
        let custom_value = EvaluatedCustomValue::from_fields(
            plan.custom_constructor_id(0, 0),
            vec![EvaluatedValue::Int(1.into())].into_boxed_slice(),
        );
        let custom = storage.custom(CustomListAllocation::new(
            plan.custom_list_function_id(0).type_id(),
            vec![custom_value.clone()],
        ));
        let external_list_type = external_list_type();
        let external_store = crate::host::HostExternalStore::default();
        let external_value = EvaluatedExternalValue::new(
            external_list_type.item_type(),
            external_store.insert(
                BigInt::from(2),
                external_equal,
                external_hash,
                external_inspect,
                |_| None,
            ),
        );
        let external_peer = EvaluatedExternalValue::new(
            external_list_type.item_type(),
            external_store.insert(
                BigInt::from(2),
                external_equal,
                external_hash,
                external_inspect,
                |_| None,
            ),
        );
        let external = RuntimeListStorage::external(
            &storage,
            ExternalListAllocation::new(external_list_type, vec![external_value.clone()]),
        );
        drop(external_store);
        let stored_equal =
            |left: &RetainedValueRef, right: &RetainedValueRef| left.value() == right.value();
        let equality = RetainedValueEquality::new(&stored_equal);
        let stored_hash = |_: &RetainedValueRef| 2;
        let hashing = RetainedValueHashing::new(&stored_hash);
        let stored_inspect = |_: &RetainedValueRef| "2".into();
        let inspection = RetainedValueInspection::new(&stored_inspect);
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

        assert!(format!("{int:?}").contains("ListHandleCore"));
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

#[cfg(test)]
mod tests {
    use super::super::RuntimeState;
    use super::lock;
    use super::{
        CustomListAllocation, ListListTypeId, ListValueId, ParameterListValueId, StoredListValueId,
    };
    use crate::plan::execution::function::{
        CoreRuntimeFunctionId, ListFunctionId, RuntimeFunctionId, RuntimeListFunctionId,
    };
    use crate::plan::execution::runtime::RuntimeExecutionPlan;
    use crate::plan::execution::type_::ListStorageTypeId;
    use crate::runtime::graph::RetainedValues;
    use crate::runtime::{
        EvaluatedBitArray, EvaluatedCapture, EvaluatedCustomValue, EvaluatedFunctionValue,
        EvaluatedIntFunction, EvaluatedValue, StoredRuntimeValue,
    };
    use crate::{
        HostCall, HostCallCompletion, HostCallError, HostCallable, HostFailure, HostFunctionType,
        HostList, HostListType, HostTypeList, HostTypeListEnd, HostTypeParameter, HostValue,
    };
    use num_bigint::BigInt;
    use std::sync::Arc;

    struct LeaseProfile;
    struct LeaseProvider;
    impl crate::HostProfile for LeaseProfile {
        type RunState = Option<super::RuntimeListStorage>;
        type ExternalStores = ();
        type ExecutionState = ();
    }
    impl crate::HostProvider<LeaseProfile> for LeaseProvider {
        type State = Option<super::RuntimeListStorage>;
        fn project(state: &mut Self::State) -> &mut Self::State {
            state
        }
    }

    fn return_host_list<'call>(
        mut call: HostCall<'call, LeaseProfile, LeaseProvider, HostListType<BigInt>>,
        constructions: crate::HostConstructions<
            'call,
            HostTypeList<HostListType<BigInt>, HostTypeListEnd>,
        >,
        value: BigInt,
    ) -> Result<HostCallCompletion<'call, HostListType<BigInt>>, HostCallError> {
        let value = call.construct_list(constructions.at::<crate::HostTypeIndex0>(), [value]);
        let storage = list_storage(&call.retain_value::<HostListType<BigInt>>(value));
        *call.state() = Some(storage);
        Ok(call.return_value(value))
    }

    fn fail_with_host_list<'call>(
        mut call: HostCall<'call, LeaseProfile, LeaseProvider, BigInt>,
        values: HostList<'call, BigInt>,
    ) -> Result<HostCallCompletion<'call, BigInt>, HostCallError> {
        let storage = list_storage(&call.retain_value::<HostListType<BigInt>>(values));
        *call.state() = Some(storage);
        Err(HostFailure::new("stop").into())
    }

    type CallbackValue = HostTypeParameter<0>;
    type CallbackArguments = HostTypeList<CallbackValue, HostTypeListEnd>;
    type Callback = HostFunctionType<CallbackArguments, CallbackValue>;

    fn invoke_generic_callback<'call>(
        mut call: HostCall<'call, LeaseProfile, LeaseProvider, CallbackValue>,
        constructions: crate::HostConstructions<'call, HostTypeListEnd>,
        function: HostCallable<'call, CallbackArguments, CallbackValue>,
        value: HostValue<'call, CallbackValue>,
    ) -> Result<crate::HostCallContinuation<'call, CallbackValue>, HostCallError> {
        type Owned = crate::provider::Value<
            CallbackValue,
            crate::provider::ProviderValueContext<CallbackValue>,
        >;
        let storage = list_storage(&call.retain_value::<CallbackValue>(value));
        *call.state() = Some(storage);
        let value = Owned::from_host(&call, value);
        let callback = call.owned_callable(function, &constructions);
        Ok(call.resume(constructions, move |context| {
            Box::pin(async move {
                let value = callback
                    .invoke(
                        &context,
                        move |mut call, _| (value.into_host(&mut call), ()),
                        |call, _, value| Ok(Owned::from_host(&call, value)),
                    )
                    .await?;
                Ok(crate::HostOwnedCompletion::new(move |mut call, _| {
                    let value = value.into_host(&mut call);
                    Ok(call.return_value(value))
                }))
            })
        }))
    }

    fn list_storage(value: &StoredRuntimeValue) -> super::RuntimeListStorage {
        let value = match value.value() {
            EvaluatedValue::Custom(custom) => custom.fields().first(),
            value => Some(value),
        };
        let Some(EvaluatedValue::List(StoredListValueId::Int(value))) = value else {
            panic!("expected an integer list lease");
        };
        super::RuntimeListStorage {
            storage: Arc::clone(&value.core.lease.storage),
        }
    }

    #[test]
    #[should_panic(expected = "expected an integer list lease")]
    fn list_storage_rejects_a_scalar_fixture() {
        list_storage(&StoredRuntimeValue::test_int(1.into()));
    }

    fn int_main(plan: &crate::ExecutionPlan) -> crate::plan::execution::function::IntFunctionId {
        match plan.main_runtime() {
            RuntimeFunctionId::Core(CoreRuntimeFunctionId::Int(main)) => main,
            _ => panic!("main should lower into the Int function table"),
        }
    }

    fn source_panic(
        result: Result<(), crate::runtime::ExecutionError>,
    ) -> crate::runtime::Panic<crate::runtime::PanicValue> {
        match result {
            Err(crate::runtime::ExecutionError::Panic(panic)) => panic,
            other => panic!("expected source panic, got {other:?}"),
        }
    }

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

    #[test]
    fn last_owner_enqueues_release_and_reuses_the_exact_slot() {
        let plan = crate::runtime::plan_src("pub fn main() -> List(Int) { [1] }");
        let type_id = plan.int_list_function_id(0).type_id();
        let mut echo = Vec::new();
        let mut state = RuntimeState::new(&mut echo);
        let value = state.lists_mut().int(type_id, vec![1.into()]);
        let slot = value.core.slot();
        let retained = value.clone();

        drop(value);
        assert_eq!(lock(&state.lists.storage.state).releases.as_slice(), &[]);
        drop(retained);
        assert_eq!(lock(&state.lists.storage.state).releases.as_slice(), &[]);

        assert_eq!(lock(&state.lists.storage.state).pools.ints.free, vec![slot],);
        let reused = state.lists_mut().int(type_id, vec![2.into()]);
        assert_eq!(reused.core.slot(), slot);
        assert_eq!(
            state.lists().int_values(&reused).as_ref(),
            &imbl::Vector::from(vec![2.into()])
        );
    }

    #[test]
    fn owned_read_view_survives_releasing_and_reusing_its_slot() {
        let plan = crate::runtime::plan_src("pub fn main() -> List(Int) { [1] }");
        let type_id = plan.int_list_function_id(0).type_id();
        let mut echo = Vec::new();
        let mut state = RuntimeState::new(&mut echo);
        let value = state.lists_mut().int(type_id, vec![1.into()]);
        let slot = value.core.slot();
        let items = state.lists().int_values(&value);

        drop(value);
        assert_eq!(lock(&state.lists.storage.state).pools.ints.free, vec![slot]);
        let replacement = state.lists_mut().int(type_id, vec![2.into()]);
        assert_eq!(replacement.core.slot(), slot);
        assert_eq!(items.as_ref(), &imbl::Vector::from(vec![1.into()]));
        assert_eq!(
            state.lists().int_values(&replacement).as_ref(),
            &imbl::Vector::from(vec![2.into()])
        );
    }

    #[test]
    fn bit_array_list_pool_preserves_type_and_reuses_released_slots() {
        let plan = crate::runtime::plan_src("pub fn main() -> List(BitArray) { [<<1>>] }");
        let type_id = plan.bit_array_list_function_id(0).type_id();
        let mut echo = Vec::new();
        let mut state = RuntimeState::new(&mut echo);
        let first = state.lists_mut().bit_array(
            type_id,
            vec![crate::runtime::EvaluatedBitArray::new(
                bitvec::vec::BitVec::from_vec(vec![1]),
            )],
        );
        let slot = first.core.slot();

        assert_eq!(first.type_id(), type_id);
        assert_eq!(state.lists().bit_array_values(&first)[0].bits().len(), 8);
        drop(first);

        let second = state.lists_mut().bit_array(type_id, Vec::new());
        assert_eq!(second.core.slot(), slot);
        assert_eq!(
            state.lists().bit_array_values(&second).as_ref(),
            &imbl::Vector::new()
        );

        let value = ListValueId::BitArray(second.clone());
        assert_eq!(state.lists().list_len(&value), 0);
        assert_eq!(
            StoredListValueId::from(second.clone()).into_value(),
            ListValueId::BitArray(second.clone()),
        );
        let dropped = state
            .lists_mut()
            .drop_first(&StoredListValueId::BitArray(second), 0);
        assert_eq!(state.lists().list_len(&dropped.clone().into_value()), 0);
    }

    #[test]
    fn repeated_release_and_allocation_keeps_one_slot_high_water_mark() {
        let plan = crate::runtime::plan_src("pub fn main() -> List(Int) { [1] }");
        let type_id = plan.int_list_function_id(0).type_id();
        let mut echo = Vec::new();
        let mut state = RuntimeState::new(&mut echo);

        for value in 0..10_000 {
            let list = state.lists_mut().int(type_id, vec![value.into()]);
            drop(list);
        }

        let storage_state = lock(&state.lists.storage.state);
        assert_eq!(storage_state.pools.ints.slots.len(), 1);
        assert_eq!(storage_state.pools.ints.free, vec![0]);
        assert_eq!(storage_state.releases.as_slice(), &[]);
    }

    #[test]
    fn host_calls_release_scoped_list_leases_after_success_and_failure() {
        let returned = crate::HostProviderModule::<LeaseProfile>::new("host_support", "host/lists")
            .expect("host module should be valid")
            .with_scoped_function_and_constructions::<LeaseProvider, (BigInt,), HostListType<BigInt>, HostTypeList<HostListType<BigInt>, HostTypeListEnd>, _>(
                "wrap",
                return_host_list,
            )
            .expect("host function should be valid");
        let source = r#"
import host/lists

pub fn main() {
  let _ = lists.wrap(1)
  0
}
"#;
        let typed = crate::compile_typed_host_program(
            "application",
            "main",
            [crate::PackageSource::new(
                "application",
                ["host_support"],
                [crate::ModuleSource::new("main", "src/main.gleam", source)],
            ), crate::PackageSource::new("host_support", Vec::<String>::new(), [
                crate::ModuleSource::new("host/lists", "src/host/lists.gleam", "@external(erlang, \"native\", \"wrap\") pub fn wrap(value: Int) -> List(Int)")
            ])],
            crate::HostProviderSet::from_providers([returned]).expect("host module should be unique"),
        )
        .expect("host source should compile");
        let plan = crate::plan_host_program(typed).expect("host source should plan");
        let mut execution = crate::HostedExecution::try_from_module_plan(plan)
            .expect("hosted execution should seal");
        let mut storage = None;
        let mut echo = Vec::new();
        let result = crate::execution_fixture::run(&mut execution, &mut storage, &mut echo);
        let lists = storage.expect("native output storage was recorded");
        assert_eq!(result, Ok(crate::Value::Int(BigInt::from(0))),);
        {
            let storage_state = lock(&lists.storage.state);
            assert_eq!(storage_state.pools.ints.slots.len(), 1);
            assert_eq!(storage_state.pools.ints.free, [0]);
        }
        assert!(lock(&lists.storage.state).releases.is_empty());

        let failed =
            crate::HostModule::<LeaseProfile>::new_for_profile("host_support", "host/lists")
                .expect("host module should be valid")
                .with_scoped_function::<LeaseProvider, (HostListType<BigInt>,), BigInt, _>(
                    "fail",
                    fail_with_host_list,
                )
                .expect("host function should be valid");
        let source = r#"
import host/lists

pub fn main() {
  lists.fail([1])
}
"#;
        let typed = crate::compile_typed_host_program(
            "application",
            "main",
            [crate::PackageSource::new(
                "application",
                ["host_support"],
                [crate::ModuleSource::new("main", "src/main.gleam", source)],
            )],
            crate::HostProviderSet::new([failed]).expect("host module should be unique"),
        )
        .expect("host source should compile");
        let plan = crate::plan_host_program(typed).expect("host source should plan");
        let mut execution = crate::HostedExecution::try_from_module_plan(plan)
            .expect("hosted execution should seal");
        let mut storage = None;
        let mut echo = Vec::new();
        let error = crate::execution_fixture::run(&mut execution, &mut storage, &mut echo)
            .expect_err("the host callback should fail");
        let lists = storage.expect("native input storage was recorded");
        assert_eq!(
            error.to_string(),
            "host function host_support::host/lists.fail failed: stop",
        );
        {
            let storage_state = lock(&lists.storage.state);
            assert_eq!(storage_state.pools.ints.slots.len(), 1);
            assert_eq!(storage_state.pools.ints.free, [0]);
        }
        assert!(lock(&lists.storage.state).releases.is_empty());
    }

    #[test]
    fn nested_callbacks_release_retained_custom_list_values_after_success() {
        let host = crate::HostModule::<LeaseProfile>::new_for_profile("host_support", "host/callback")
            .expect("host module should be valid")
            .with_resumable_function::<
                LeaseProvider,
                (Callback, CallbackValue),
                CallbackValue,
                geam_core::HostTypeListEnd, _,
            >("invoke", invoke_generic_callback)
            .expect("generic callback should be valid");
        let source = r#"
import host/callback

pub type Boxed {
  Boxed(List(Int))
}

fn identity(value: Boxed) {
  value
}

pub fn main() {
  let _ = callback.invoke(identity, Boxed([1]))
  0
}
"#;
        let typed = crate::compile_typed_host_program(
            "application",
            "main",
            [crate::PackageSource::new(
                "application",
                ["host_support"],
                [crate::ModuleSource::new("main", "src/main.gleam", source)],
            )],
            crate::HostProviderSet::new([host]).expect("host module should be unique"),
        )
        .expect("successful callback source should compile");
        let plan = crate::plan_host_program(typed).expect("successful callback source should plan");
        let mut execution = crate::HostedExecution::try_from_module_plan(plan)
            .expect("successful callback execution should seal");
        let mut storage = None;
        let mut echo = Vec::new();
        let result = crate::execution_fixture::run(&mut execution, &mut storage, &mut echo);
        let lists = storage.expect("callback input storage was recorded");
        assert_eq!(result, Ok(crate::Value::Int(BigInt::from(0))),);
        {
            let storage_state = lock(&lists.storage.state);
            assert_eq!(storage_state.pools.ints.slots.len(), 1);
            assert_eq!(storage_state.pools.ints.free, [0]);
        }
        assert!(lock(&lists.storage.state).releases.is_empty());
    }

    #[test]
    fn nested_callbacks_release_retained_custom_list_values_after_panic() {
        let host = crate::HostModule::<LeaseProfile>::new_for_profile("host_support", "host/callback")
            .expect("host module should be valid")
            .with_resumable_function::<
                LeaseProvider,
                (Callback, CallbackValue),
                CallbackValue,
                geam_core::HostTypeListEnd, _,
            >("invoke", invoke_generic_callback)
            .expect("generic callback should be valid");
        let source = r#"
import host/callback

pub type Boxed {
  Boxed(List(Int))
}

fn stop(_value: Boxed) -> Boxed {
  panic as "nested"
}

pub fn main() {
  callback.invoke(stop, Boxed([1]))
}
"#;
        let typed = crate::compile_typed_host_program(
            "application",
            "main",
            [crate::PackageSource::new(
                "application",
                ["host_support"],
                [crate::ModuleSource::new("main", "src/main.gleam", source)],
            )],
            crate::HostProviderSet::new([host]).expect("host module should be unique"),
        )
        .expect("panicking callback source should compile");
        let plan = crate::plan_host_program(typed).expect("panicking callback source should plan");
        let mut execution = crate::HostedExecution::try_from_module_plan(plan)
            .expect("panicking callback execution should seal");
        let mut storage = None;
        let mut echo = Vec::new();
        let panic = source_panic(
            crate::execution_fixture::run(&mut execution, &mut storage, &mut echo).map(drop),
        );
        let lists = storage.expect("failed callback input storage was recorded");
        assert_eq!(panic.kind(), crate::PanicKind::Panic);
        assert_eq!(panic.site().function(), "stop");
        {
            let storage_state = lock(&lists.storage.state);
            assert_eq!(storage_state.pools.ints.slots.len(), 1);
            assert_eq!(storage_state.pools.ints.free, [0]);
        }
        assert!(lock(&lists.storage.state).releases.is_empty());
    }

    #[test]
    fn tail_recursive_block_replacement_reuses_a_fixed_list_slot_set() {
        let plan = crate::runtime::plan_src(
            r#"fn done(count: Int, values: List(Int)) {
  case count {
    0 -> values
    _ -> done(count - 1, [count])
  }
}

pub fn main() {
  done(10000, [])
}

// @geam:expect List(Int)([Int(1)])
"#,
        );
        let main = plan.int_list_function_id(0);
        assert_eq!(
            plan.main_runtime(),
            RuntimeFunctionId::Core(CoreRuntimeFunctionId::List(RuntimeListFunctionId::Core(
                ListFunctionId::Int(main),
            ))),
        );
        let mut echo = Vec::new();
        let mut state = RuntimeState::new(&mut echo);

        let value = crate::runtime::function::run_int_list(
            &plan,
            &mut state,
            main,
            crate::runtime::error::HostCallOrigin::Entry,
            RetainedValues::empty(),
        )
        .expect("tail-recursive list graph should return");

        assert_eq!(
            state.lists().int_values(&value).as_ref(),
            &imbl::Vector::from(vec![1.into()])
        );
        {
            let storage_state = lock(&state.lists.storage.state);
            assert_eq!(storage_state.pools.ints.slots.len(), 1);
            assert_eq!(storage_state.pools.ints.free.len(), 0);
        }
        drop(value);
        assert_eq!(lock(&state.lists.storage.state).pools.ints.free.len(), 1);
        assert_eq!(lock(&state.lists.storage.state).releases.as_slice(), &[]);
    }

    #[test]
    fn never_terminator_releases_the_caller_environment_before_running_the_callee() {
        let plan = crate::runtime::plan_src(
            r#"
fn stop() -> value { panic as "stop" }

pub fn main() -> Int {
  let values = [1]
  let _ = values
  stop()
}
"#,
        );
        let main = int_main(&plan);
        let mut echo = Vec::new();
        let mut state = RuntimeState::new(&mut echo);

        let panic = source_panic(
            crate::runtime::function::run_int(
                &plan,
                &mut state,
                main,
                crate::runtime::error::HostCallOrigin::Entry,
                RetainedValues::empty(),
            )
            .map(drop),
        );

        assert_eq!(panic.kind(), crate::runtime::PanicKind::Panic);
        assert_eq!(
            panic.message(),
            &crate::runtime::PanicMessage::Explicit("stop".into()),
        );
        let storage_state = lock(&state.lists.storage.state);
        assert_eq!(storage_state.pools.ints.slots.len(), 1);
        assert_eq!(storage_state.pools.ints.free, vec![0]);
        assert_eq!(storage_state.releases.as_slice(), &[]);
    }

    #[test]
    fn match_transition_releases_unretained_subject_before_the_target_runs() {
        let plan = crate::runtime::plan_src(
            r#"
pub fn main() -> Int {
  case [1] {
    [] -> panic as "empty"
    _ -> panic as "non-empty"
  }
}
"#,
        );
        let main = int_main(&plan);
        let mut echo = Vec::new();
        let mut state = RuntimeState::new(&mut echo);

        let panic = source_panic(
            crate::runtime::function::run_int(
                &plan,
                &mut state,
                main,
                crate::runtime::error::HostCallOrigin::Entry,
                RetainedValues::empty(),
            )
            .map(drop),
        );

        assert_eq!(
            panic.message(),
            &crate::runtime::PanicMessage::Explicit("non-empty".into()),
        );
        let storage_state = lock(&state.lists.storage.state);
        assert_eq!(storage_state.pools.ints.slots.len(), 1);
        assert_eq!(storage_state.pools.ints.free, vec![0]);
        assert_eq!(storage_state.releases.as_slice(), &[]);
    }

    #[test]
    #[should_panic(expected = "main should lower into the Int function table")]
    fn int_main_guard_rejects_other_function_tables() {
        int_main(&crate::runtime::plan_src(
            "pub fn main() -> List(Int) { [] }",
        ));
    }

    #[test]
    #[should_panic(expected = "expected source panic, got Ok(())")]
    fn source_panic_guard_rejects_success() {
        source_panic(Ok(()));
    }

    #[test]
    fn list_handles_own_live_allocations_after_runtime_state_drop() {
        let plan = crate::runtime::plan_src("pub fn main() -> List(Int) { [1] }");
        let type_id = plan.int_list_function_id(0).type_id();
        let mut echo = Vec::new();
        let mut state = RuntimeState::new(&mut echo);
        let value = state.lists_mut().int(type_id, vec![1.into()]);
        let clone = value.clone();
        let storage = Arc::clone(&value.core.lease.storage);
        let discarded = state.lists_mut().int(type_id, vec![2.into()]);
        let discarded_slot = discarded.core.slot();
        let mut other_echo = Vec::new();
        let mut other_state = RuntimeState::new(&mut other_echo);
        let other = other_state.lists_mut().int(type_id, vec![1.into()]);

        assert_eq!(value, clone);
        assert_ne!(value, other);
        drop(discarded);
        assert_eq!(
            lock(&storage.state).pools.ints.get(discarded_slot).as_ref(),
            &imbl::Vector::new(),
        );
        drop(state);
        assert_eq!(
            lock(&storage.state)
                .pools
                .ints
                .get(value.core.slot())
                .as_ref(),
            &imbl::Vector::from(vec![1.into()]),
        );
        drop(value);
        assert_eq!(
            lock(&storage.state)
                .pools
                .ints
                .get(clone.core.slot())
                .as_ref(),
            &imbl::Vector::from(vec![1.into()]),
        );
        drop(clone);
        {
            let state = lock(&storage.state);
            assert_eq!(state.pools.ints.slots.len(), 2);
            assert!(
                state
                    .pools
                    .ints
                    .slots
                    .iter()
                    .all(|values| values.is_empty())
            );
            assert_eq!(state.pools.ints.free, vec![discarded_slot, 0]);
        }

        drop(other_state);
        drop(other);
    }

    #[test]
    fn list_value_facade_reconstructs_every_exact_storage_family() {
        let plan = crate::runtime::plan_src(EVERY_LIST_FAMILY_SOURCE);
        let mut echo = Vec::new();
        let mut state = RuntimeState::new(&mut echo);
        let int_function = EvaluatedIntFunction::reference(
            crate::plan::execution::function::IntFunctionId(0),
            Vec::new(),
            Default::default(),
            crate::plan::execution::type_::FunctionType::new(
                Vec::new(),
                crate::plan::execution::type_::ValueType::Int,
            ),
        );
        let int = state
            .lists_mut()
            .int(plan.int_list_function_id(0).type_id(), vec![1.into()]);
        let string = state.lists_mut().string(
            plan.string_list_function_id(0).type_id(),
            vec!["one".into()],
        );
        let bit_array = state.lists_mut().bit_array(
            plan.bit_array_list_function_id(0).type_id(),
            vec![EvaluatedBitArray::new(bitvec::vec::BitVec::from_vec(vec![
                1,
            ]))],
        );
        let utf_codepoint = state.lists_mut().utf_codepoint(
            plan.utf_codepoint_list_function_id(0).type_id(),
            vec!['\u{10ffff}'],
        );
        let custom_constructor = plan.custom_constructor_id(0, 0);
        let custom = state.lists_mut().custom(CustomListAllocation::new(
            plan.custom_list_function_id(0).type_id(),
            vec![EvaluatedCustomValue::from_fields(
                custom_constructor,
                vec![EvaluatedValue::Int(1.into())].into_boxed_slice(),
            )],
        ));
        let float = state
            .lists_mut()
            .float(plan.float_list_function_id(0).type_id(), vec![1.5]);
        let bool_ = state
            .lists_mut()
            .bool(plan.bool_list_function_id(0).type_id(), vec![true]);
        let nil = state
            .lists_mut()
            .nil(plan.nil_list_function_id(0).type_id(), 1);
        let tuple = state.lists_mut().tuple(
            plan.tuple_list_function_id(0).type_id(),
            vec![vec![EvaluatedValue::Int(1.into())]],
        );
        let parameter = ParameterListValueId::new(plan.parameter_list_function_id(0).type_id());
        let parameter_list = state
            .lists_mut()
            .parameter_list_list(plan.parameter_list_list_function_id(0).type_id(), 1);
        let child = state
            .lists_mut()
            .int(plan.int_list_function_id(0).type_id(), vec![1.into()]);
        let list = state.lists_mut().list(
            plan.list_list_function_id(0).type_id(),
            vec![child.clone().into()],
        );
        let function = state.lists_mut().function(
            plan.function_list_function_id(0).type_id(),
            vec![EvaluatedFunctionValue::from(int_function.clone())],
        );
        let values = [
            (
                StoredListValueId::from(int.clone()),
                ListValueId::Int(int.clone()),
            ),
            (
                StoredListValueId::from(string.clone()),
                ListValueId::String(string.clone()),
            ),
            (
                StoredListValueId::from(bit_array.clone()),
                ListValueId::BitArray(bit_array.clone()),
            ),
            (
                StoredListValueId::from(utf_codepoint.clone()),
                ListValueId::UtfCodepoint(utf_codepoint.clone()),
            ),
            (
                StoredListValueId::from(custom.clone()),
                ListValueId::Custom(custom.clone()),
            ),
            (
                StoredListValueId::from(float.clone()),
                ListValueId::Float(float.clone()),
            ),
            (
                StoredListValueId::from(bool_.clone()),
                ListValueId::Bool(bool_.clone()),
            ),
            (
                StoredListValueId::from(nil.clone()),
                ListValueId::Nil(nil.clone()),
            ),
            (
                StoredListValueId::from(tuple.clone()),
                ListValueId::Tuple(tuple.clone()),
            ),
            (
                StoredListValueId::from(parameter_list.clone()),
                ListValueId::ParameterList(parameter_list.clone()),
            ),
            (
                StoredListValueId::from(list.clone()),
                ListValueId::List(list.clone()),
            ),
            (
                StoredListValueId::from(function.clone()),
                ListValueId::Function(function.clone()),
            ),
        ];

        for (stored, value) in values {
            assert_eq!(stored.into_value(), value);
        }

        let expected_custom = EvaluatedCustomValue::from_fields(
            custom_constructor,
            vec![EvaluatedValue::Int(1.into())].into_boxed_slice(),
        );
        let expected_child = EvaluatedValue::from(StoredListValueId::from(child.clone()));

        assert_eq!(
            state
                .lists()
                .evaluated_value_at(&ListValueId::Parameter(parameter), 0),
            None,
        );
        assert_eq!(
            state
                .lists()
                .evaluated_value_at(&ListValueId::Int(int.clone()), 0),
            Some(EvaluatedValue::Int(1.into())),
        );
        assert_eq!(
            state
                .lists()
                .evaluated_value_at(&ListValueId::String(string.clone()), 0),
            Some(EvaluatedValue::String("one".into())),
        );
        assert_eq!(
            state
                .lists()
                .evaluated_value_at(&ListValueId::BitArray(bit_array.clone()), 0),
            Some(EvaluatedValue::BitArray(EvaluatedBitArray::new(
                bitvec::vec::BitVec::from_vec(vec![1]),
            ))),
        );
        assert_eq!(
            state
                .lists()
                .evaluated_value_at(&ListValueId::UtfCodepoint(utf_codepoint.clone()), 0),
            Some(EvaluatedValue::UtfCodepoint('\u{10ffff}')),
        );
        assert_eq!(
            state
                .lists()
                .evaluated_value_at(&ListValueId::Custom(custom.clone()), 0),
            Some(EvaluatedValue::Custom(expected_custom)),
        );
        assert_eq!(
            state
                .lists()
                .evaluated_value_at(&ListValueId::Float(float.clone()), 0),
            Some(EvaluatedValue::Float(1.5)),
        );
        assert_eq!(
            state
                .lists()
                .evaluated_value_at(&ListValueId::Bool(bool_.clone()), 0),
            Some(EvaluatedValue::Bool(true)),
        );
        assert_eq!(
            state
                .lists()
                .evaluated_value_at(&ListValueId::Nil(nil.clone()), 0),
            Some(EvaluatedValue::Nil),
        );
        assert_eq!(
            state
                .lists()
                .evaluated_value_at(&ListValueId::Tuple(tuple.clone()), 0),
            Some(EvaluatedValue::Tuple(vec![EvaluatedValue::Int(1.into())])),
        );
        assert_eq!(
            state
                .lists()
                .evaluated_value_at(&ListValueId::ParameterList(parameter_list.clone()), 0),
            Some(EvaluatedValue::ParameterList(ParameterListValueId::new(
                parameter_list.type_id().item_type(),
            ))),
        );
        assert_eq!(
            state
                .lists()
                .evaluated_value_at(&ListValueId::List(list.clone()), 0),
            Some(expected_child),
        );
        assert_eq!(
            state
                .lists()
                .evaluated_value_at(&ListValueId::Function(function.clone()), 0),
            Some(EvaluatedValue::Function(EvaluatedFunctionValue::from(
                int_function,
            ))),
        );
        assert_eq!(
            state
                .lists()
                .evaluated_value_at(&ListValueId::Int(int.clone()), 1),
            None,
        );

        let stored_lists = [
            StoredListValueId::Int(int),
            StoredListValueId::String(string),
            StoredListValueId::BitArray(bit_array),
            StoredListValueId::UtfCodepoint(utf_codepoint),
            StoredListValueId::Custom(custom),
            StoredListValueId::Float(float),
            StoredListValueId::Bool(bool_),
            StoredListValueId::Nil(nil),
            StoredListValueId::Tuple(tuple),
            StoredListValueId::ParameterList(parameter_list.clone()),
            StoredListValueId::List(list),
            StoredListValueId::Function(function),
        ];
        for value in stored_lists {
            let list_type = value.list_type();
            assert_eq!(state.lists().list_len(&value.clone().into_value()), 1);

            let dropped = state.lists_mut().drop_first(&value, 1);
            assert_eq!(dropped.list_type(), list_type);
            assert_eq!(state.lists().list_len(&dropped.into_value()), 0);
        }

        assert_eq!(
            StoredListValueId::from(parameter_list.clone()).into_core(),
            parameter_list.clone().into_core(),
        );
        assert_eq!(
            state.lists().list_len(&ListValueId::Parameter(parameter)),
            0
        );
        assert_eq!(
            state
                .lists()
                .list_len(&ListValueId::ParameterList(parameter_list.clone())),
            1
        );
        let dropped = state.lists_mut().drop_first(
            &StoredListValueId::ParameterList(parameter_list),
            usize::MAX,
        );
        assert_eq!(state.lists().list_len(&dropped.clone().into_value()), 0);
        assert_eq!(
            dropped.list_type(),
            plan.parameter_list_list_function_id(0)
                .type_id()
                .list_type(),
        );
    }

    #[test]
    fn parent_release_preserves_a_separately_owned_child() {
        let plan = crate::runtime::plan_src(
            "fn ints() -> List(Int) { [] } pub fn main() -> List(List(Int)) { let _ = ints [[1]] }",
        );
        let parent_type = plan.list_list_function_id(0).type_id();
        let child_type = plan.int_list_function_id(0).type_id();
        assert_eq!(parent_type.item_type(), child_type.list_type());
        let mut echo = Vec::new();
        let mut state = RuntimeState::new(&mut echo);
        let child = state.lists_mut().int(child_type, vec![1.into()]);
        let child_slot = child.core.slot();
        let parent = state
            .lists_mut()
            .list(parent_type, vec![child.clone().into()]);

        drop(parent);
        {
            let storage_state = lock(&state.lists.storage.state);
            assert_eq!(storage_state.pools.lists.free.len(), 1);
            assert_eq!(storage_state.pools.ints.free, Vec::<usize>::new());
        }
        assert_eq!(
            state.lists().int_values(&child).as_ref(),
            &imbl::Vector::from(vec![1.into()])
        );

        drop(child);
        assert_eq!(
            lock(&state.lists.storage.state).pools.ints.free,
            vec![child_slot],
        );
    }

    #[test]
    fn exclusive_nested_children_are_released_iteratively() {
        let depth = 64;
        let nested_type = "List(".repeat(depth) + "Int" + &")".repeat(depth);
        let source = format!(
            "fn ints() -> List(Int) {{ [] }} pub fn main() -> {nested_type} {{ let _ = ints [] }}"
        );
        let plan = crate::runtime::plan_src(&source);
        let mut type_id = plan.list_list_function_id(0).type_id().list_type();
        let mut parents = Vec::new();
        let child_type = plan.int_list_function_id(0).type_id();
        while type_id != child_type.list_type() {
            let parent = nested_list_storage(plan.list_storage_type(type_id))
                .expect("remaining recursive storage must be a nested list");
            parents.push(parent);
            type_id = parent.item_type();
        }
        assert_eq!(parents.len(), depth - 1);
        assert_eq!(
            nested_list_storage(ListStorageTypeId::Int(child_type)),
            None
        );

        let mut echo = Vec::new();
        let mut state = RuntimeState::new(&mut echo);
        let mut value: StoredListValueId = state.lists_mut().int(child_type, vec![1.into()]).into();
        for parent in parents.into_iter().rev() {
            value = state.lists_mut().list(parent, vec![value]).into();
        }
        let allocated_list_slots = lock(&state.lists.storage.state).pools.lists.slots.len();

        std::thread::Builder::new()
            .stack_size(128 * 1024)
            .spawn(move || drop(value))
            .expect("small-stack release worker")
            .join()
            .expect("nested lists release without recursion");
        let storage_state = lock(&state.lists.storage.state);
        assert_eq!(storage_state.pools.ints.free.len(), 1);
        assert_eq!(storage_state.pools.lists.free.len(), allocated_list_slots);
        assert_eq!(storage_state.releases.as_slice(), &[]);
    }

    #[test]
    fn retained_owners_preserve_and_iteratively_release_nested_lists() {
        let depth = 64;
        let nested_type = "List(".repeat(depth) + "Int" + &")".repeat(depth);
        let source = format!(
            "fn ints() -> List(Int) {{ [] }} pub fn main() -> {nested_type} {{ let _ = ints [] }}"
        );
        let plan = crate::runtime::plan_src(&source);
        let mut type_id = plan.list_list_function_id(0).type_id().list_type();
        let mut parents = Vec::new();
        let child_type = plan.int_list_function_id(0).type_id();
        while type_id != child_type.list_type() {
            let parent = nested_list_storage(plan.list_storage_type(type_id))
                .expect("remaining recursive storage must be a nested list");
            parents.push(parent);
            type_id = parent.item_type();
        }

        let mut echo = Vec::new();
        let mut state = RuntimeState::new(&mut echo);
        let mut value: StoredListValueId = state.lists_mut().int(child_type, vec![1.into()]).into();
        for parent in parents.into_iter().rev() {
            value = state.lists_mut().list(parent, vec![value]).into();
        }
        let allocated_list_slots = lock(&state.lists.storage.state).pools.lists.slots.len();
        let evaluated = EvaluatedValue::List(value.clone());
        let stored = StoredRuntimeValue::new(evaluated, plan.value_metadata());
        let retained = crate::runtime::retained_list::RetainedList::new(value.clone());

        drop(value);
        {
            let storage_state = lock(&state.lists.storage.state);
            assert!(storage_state.pools.ints.free.is_empty());
            assert!(storage_state.pools.lists.free.is_empty());
        }

        drop(stored);
        assert!(lock(&state.lists.storage.state).pools.ints.free.is_empty());
        assert!(lock(&state.lists.storage.state).pools.lists.free.is_empty());
        assert_eq!(retained.len(), 1);
        drop(retained);
        let storage_state = lock(&state.lists.storage.state);
        assert_eq!(storage_state.pools.ints.free.len(), 1);
        assert_eq!(storage_state.pools.lists.free.len(), allocated_list_slots);
        assert_eq!(storage_state.releases.as_slice(), &[]);
    }

    #[test]
    fn list_suffix_releases_removed_closure_captures_while_preserving_the_rest() {
        let plan = crate::runtime::plan_src(
            r#"
fn keep(value: List(Int)) { fn() { value } }
fn build(count: Int) {
  case count {
    0 -> []
    _ -> [keep([count]), ..build(count - 1)]
  }
}
pub fn main() { build(65) }
"#,
        );
        let main = plan.function_list_function_id(0);
        assert_eq!(
            plan.main_runtime(),
            RuntimeFunctionId::Core(CoreRuntimeFunctionId::List(RuntimeListFunctionId::Core(
                ListFunctionId::Function(main),
            )))
        );
        let mut echo = Vec::new();
        let mut state = RuntimeState::new(&mut echo);
        let original = EvaluatedValue::from(
            crate::runtime::function::run_list(
                &plan,
                &mut state,
                crate::plan::execution::function::ProfiledListFunctionId::Core(
                    ListFunctionId::Function(main),
                ),
                crate::runtime::error::HostCallOrigin::Entry,
                RetainedValues::empty(),
            )
            .expect("captured lists"),
        );
        let suffix = state.lists().drop_first(
            crate::runtime::BorrowedValue::from_value(&original).stored_list(),
            1,
        );
        {
            let storage = lock(&state.lists.storage.state);
            assert_eq!(storage.pools.ints.slots.len(), 65);
            assert!(storage.pools.ints.free.is_empty());
        }
        drop(original);
        assert_eq!(lock(&state.lists.storage.state).pools.ints.free.len(), 1);
        assert_eq!(state.lists().list_len(&suffix.clone().into_value()), 64);
        drop(suffix);
        let storage = lock(&state.lists.storage.state);
        assert_eq!(storage.pools.ints.free.len(), 65);
        assert_eq!(
            storage.pools.functions.free.len(),
            storage.pools.functions.slots.len()
        );
        assert!(storage.releases.is_empty());
    }

    #[test]
    fn closure_capture_retains_its_list_until_the_closure_is_dropped() {
        let plan = crate::runtime::plan_src(
            "fn keep(values: List(Int)) { fn() { values } } pub fn main() { keep([1]) }",
        );
        let type_id = plan.int_list_function_id(0).type_id();
        let mut echo = Vec::new();
        let mut state = RuntimeState::new(&mut echo);
        let value = state.lists_mut().int(type_id, vec![1.into()]);
        let slot = value.core.slot();
        let closure = EvaluatedIntFunction::reference(
            crate::plan::execution::function::IntFunctionId(0),
            Vec::new(),
            state.captures().capture(vec![EvaluatedCapture::list(
                crate::runtime::EvaluatedListCapture::Int {
                    local: crate::plan::execution::graph::IntListLocalId(0),
                    value: value.clone(),
                },
            )]),
            crate::plan::execution::type_::FunctionType::new(
                Vec::new(),
                crate::plan::execution::type_::ValueType::Int,
            ),
        );

        drop(value);
        assert_eq!(
            lock(&state.lists.storage.state).pools.ints.free,
            Vec::<usize>::new(),
        );
        drop(closure);
        assert_eq!(lock(&state.lists.storage.state).pools.ints.free, vec![slot],);
    }

    fn nested_list_storage(storage: ListStorageTypeId) -> Option<ListListTypeId> {
        match storage {
            ListStorageTypeId::List(type_id) => Some(type_id),
            _ => None,
        }
    }
}
