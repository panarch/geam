use std::cell::{Cell, Ref, RefCell};
use std::fmt;
use std::rc::Rc;

use ecow::EcoString;
use num_bigint::BigInt;

use super::evaluated::{
    EvaluatedBitArray, EvaluatedCustomValue, EvaluatedExternalValue, EvaluatedFunctionValue,
    EvaluatedValue,
};
use crate::plan::execution::type_::{
    BitArrayListTypeId, BoolListTypeId, CustomListTypeId, ExternalListTypeId, FloatListTypeId,
    FunctionListTypeId, IntListTypeId, ListListTypeId, ListTypeId, NilListTypeId,
    ParameterListListTypeId, ParameterListTypeId, StringListTypeId, TupleListTypeId,
    UtfCodepointListTypeId,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ListStorageKey {
    Int { slot: usize },
    String { slot: usize },
    BitArray { slot: usize },
    UtfCodepoint { slot: usize },
    Custom { slot: usize },
    External { slot: usize },
    Float { slot: usize },
    Bool { slot: usize },
    Nil { slot: usize },
    Tuple { slot: usize },
    ParameterList { slot: usize },
    List { slot: usize },
    Function { slot: usize },
}

struct ListLease {
    key: ListStorageKey,
    storage: Rc<SharedListStorage>,
}

impl fmt::Debug for ListLease {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ListLease")
            .field("key", &self.key)
            .finish()
    }
}

impl Drop for ListLease {
    fn drop(&mut self) {
        self.storage.release(self.key);
    }
}

impl ListStorageKey {
    fn slot(self) -> usize {
        match self {
            Self::Int { slot }
            | Self::String { slot }
            | Self::BitArray { slot }
            | Self::UtfCodepoint { slot }
            | Self::Custom { slot }
            | Self::External { slot }
            | Self::Float { slot }
            | Self::Bool { slot }
            | Self::Nil { slot }
            | Self::Tuple { slot }
            | Self::ParameterList { slot }
            | Self::List { slot }
            | Self::Function { slot } => slot,
        }
    }
}

#[derive(Debug, Clone)]
pub(super) struct ListHandleCore {
    lease: Rc<ListLease>,
}

impl PartialEq for ListHandleCore {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.lease, &other.lease)
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

macro_rules! typed_list_value_id {
    ($name:ident, $type_id:ty, $variant:ident) => {
        #[derive(Debug, Clone, PartialEq)]
        pub(super) struct $name {
            type_id: $type_id,
            core: ListHandleCore,
        }

        impl $name {
            pub(super) fn new(type_id: $type_id, core: ListHandleCore) -> Self {
                Self { type_id, core }
            }

            pub(super) fn type_id(&self) -> $type_id {
                self.type_id
            }

            pub(super) fn into_core(self) -> ListHandleCore {
                self.core
            }

            pub(super) fn from_stored(value: &StoredListValueId) -> Option<Self> {
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
pub(super) struct ParameterListValueId {
    type_id: ParameterListTypeId,
}

impl ParameterListValueId {
    pub(super) fn new(type_id: ParameterListTypeId) -> Self {
        Self { type_id }
    }

    pub(super) fn type_id(self) -> ParameterListTypeId {
        self.type_id
    }
}

impl From<ParameterListValueId> for ListValueId {
    fn from(value: ParameterListValueId) -> Self {
        Self::Parameter(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub(super) enum StoredListValueId {
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
    ($value:ty, $variant:ident) => {
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

pub(super) struct CustomListAllocation {
    type_id: CustomListTypeId,
    values: Vec<EvaluatedCustomValue>,
}

impl CustomListAllocation {
    pub(super) fn new(type_id: CustomListTypeId, values: Vec<EvaluatedCustomValue>) -> Self {
        Self { type_id, values }
    }

    fn from_value(value: &CustomListValueId, values: Vec<EvaluatedCustomValue>) -> Self {
        Self {
            type_id: value.type_id(),
            values,
        }
    }
}

pub(super) struct ExternalListAllocation {
    type_id: ExternalListTypeId,
    values: Vec<EvaluatedExternalValue>,
}

impl ExternalListAllocation {
    pub(super) fn new(type_id: ExternalListTypeId, values: Vec<EvaluatedExternalValue>) -> Self {
        Self { type_id, values }
    }

    fn from_value(value: &ExternalListValueId, values: Vec<EvaluatedExternalValue>) -> Self {
        Self {
            type_id: value.type_id(),
            values,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub(super) enum ListValueId {
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
    pub(super) fn list_type(&self) -> ListTypeId {
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

    pub(super) fn into_value(self) -> ListValueId {
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

    pub(super) fn into_core(self) -> ListHandleCore {
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

#[derive(Default)]
struct ListPool<Value: Default> {
    slots: Vec<Value>,
    free: Vec<usize>,
}

impl<Value: Default> ListPool<Value> {
    fn allocate(&mut self, value: Value) -> usize {
        if let Some(slot) = self.free.pop() {
            self.slots[slot] = value;
            slot
        } else {
            let slot = self.slots.len();
            self.slots.push(value);
            slot
        }
    }

    fn get(&self, slot: usize) -> &Value {
        &self.slots[slot]
    }

    fn release(&mut self, slot: usize) -> Value {
        let value = std::mem::take(&mut self.slots[slot]);
        self.free.push(slot);
        value
    }
}

#[derive(Default)]
struct ListPools {
    ints: ListPool<Vec<BigInt>>,
    strings: ListPool<Vec<EcoString>>,
    bit_arrays: ListPool<Vec<EvaluatedBitArray>>,
    utf_codepoints: ListPool<Vec<char>>,
    customs: ListPool<Vec<EvaluatedCustomValue>>,
    externals: ListPool<Vec<EvaluatedExternalValue>>,
    floats: ListPool<Vec<f64>>,
    bools: ListPool<Vec<bool>>,
    nils: ListPool<usize>,
    tuples: ListPool<Vec<Vec<EvaluatedValue>>>,
    parameter_list_lists: ListPool<usize>,
    lists: ListPool<Vec<StoredListValueId>>,
    functions: ListPool<Vec<EvaluatedFunctionValue>>,
}

enum ReleasedList {
    Int(Vec<BigInt>),
    String(Vec<EcoString>),
    BitArray(Vec<EvaluatedBitArray>),
    UtfCodepoint(Vec<char>),
    Custom(Vec<EvaluatedCustomValue>),
    External(Vec<EvaluatedExternalValue>),
    Float(Vec<f64>),
    Bool(Vec<bool>),
    Nil(usize),
    Tuple(Vec<Vec<EvaluatedValue>>),
    ParameterList(usize),
    List(Vec<StoredListValueId>),
    Function(Vec<EvaluatedFunctionValue>),
}

#[derive(Default)]
struct SharedListStorage {
    releases: RefCell<Vec<ListStorageKey>>,
    draining: Cell<bool>,
    pools: RefCell<ListPools>,
}

impl ListPools {
    fn release(&mut self, key: ListStorageKey) -> ReleasedList {
        match key {
            ListStorageKey::Int { slot } => ReleasedList::Int(self.ints.release(slot)),
            ListStorageKey::String { slot } => ReleasedList::String(self.strings.release(slot)),
            ListStorageKey::BitArray { slot } => {
                ReleasedList::BitArray(self.bit_arrays.release(slot))
            }
            ListStorageKey::UtfCodepoint { slot } => {
                ReleasedList::UtfCodepoint(self.utf_codepoints.release(slot))
            }
            ListStorageKey::Custom { slot } => ReleasedList::Custom(self.customs.release(slot)),
            ListStorageKey::External { slot } => {
                ReleasedList::External(self.externals.release(slot))
            }
            ListStorageKey::Float { slot } => ReleasedList::Float(self.floats.release(slot)),
            ListStorageKey::Bool { slot } => ReleasedList::Bool(self.bools.release(slot)),
            ListStorageKey::Nil { slot } => ReleasedList::Nil(self.nils.release(slot)),
            ListStorageKey::Tuple { slot } => ReleasedList::Tuple(self.tuples.release(slot)),
            ListStorageKey::ParameterList { slot } => {
                ReleasedList::ParameterList(self.parameter_list_lists.release(slot))
            }
            ListStorageKey::List { slot } => ReleasedList::List(self.lists.release(slot)),
            ListStorageKey::Function { slot } => {
                ReleasedList::Function(self.functions.release(slot))
            }
        }
    }
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
            Self::Nil(len) => {
                let _released_len = len;
            }
            Self::Tuple(values) => drop(values),
            Self::ParameterList(len) => {
                let _released_len = len;
            }
            Self::List(values) => drop(values),
            Self::Function(values) => drop(values),
        }
    }
}

impl SharedListStorage {
    fn release(&self, key: ListStorageKey) {
        self.releases.borrow_mut().push(key);
        self.drain_releases();
    }

    fn drain_releases(&self) {
        if self.draining.replace(true) {
            return;
        }

        loop {
            let Some(key) = self.releases.borrow_mut().pop() else {
                break;
            };
            let Ok(mut pools) = self.pools.try_borrow_mut() else {
                self.releases.borrow_mut().push(key);
                break;
            };
            let released = pools.release(key);
            drop(pools);
            released.drop_values();
        }

        self.draining.set(false);
    }

    fn core(self: &Rc<Self>, key: ListStorageKey) -> ListHandleCore {
        ListHandleCore {
            lease: Rc::new(ListLease {
                key,
                storage: Rc::clone(self),
            }),
        }
    }
}

pub(in crate::runtime) trait RuntimeHostState {
    type State;

    fn state(&mut self) -> &mut Self::State;
}

impl RuntimeHostState for () {
    type State = ();

    fn state(&mut self) -> &mut Self::State {
        self
    }
}

impl<State> RuntimeHostState for &mut State {
    type State = State;

    fn state(&mut self) -> &mut Self::State {
        self
    }
}

pub(in crate::runtime) struct RuntimeValueStorage {
    storage: Rc<SharedListStorage>,
}

impl Default for RuntimeValueStorage {
    fn default() -> Self {
        Self {
            storage: Rc::new(SharedListStorage::default()),
        }
    }
}

pub(in crate::runtime) struct RuntimeState<'run, Host = ()> {
    echo: &'run mut dyn crate::runtime::EchoSink,
    host: Host,
    values: RuntimeValueStorage,
}

pub(in crate::runtime) type RuntimeStateFor<'run, Plan> =
    RuntimeState<'run, <Plan as crate::runtime::ExecutableRuntimePlan>::RuntimeHost<'run>>;

impl<'run> RuntimeState<'run, ()> {
    pub(super) fn new(echo: &'run mut dyn crate::runtime::EchoSink) -> Self {
        Self::with_storage(echo, ())
    }
}

impl<'run, Host> RuntimeState<'run, Host> {
    pub(super) fn with_host(echo: &'run mut dyn crate::runtime::EchoSink, host: Host) -> Self {
        Self::with_storage(echo, host)
    }

    fn with_storage(echo: &'run mut dyn crate::runtime::EchoSink, host: Host) -> Self {
        Self {
            echo,
            host,
            values: RuntimeValueStorage::default(),
        }
    }

    pub(super) fn emit_echo(&mut self, output: crate::runtime::EchoOutput) {
        self.echo.emit(output);
    }

    pub(super) fn values(&self) -> &RuntimeValueStorage {
        &self.values
    }

    pub(super) fn values_mut(&mut self) -> &mut RuntimeValueStorage {
        &mut self.values
    }
}

impl<Host: RuntimeHostState> RuntimeState<'_, Host> {
    pub(super) fn host_state(&mut self) -> &mut Host::State {
        self.host.state()
    }
}

impl RuntimeValueStorage {
    pub(super) fn drain_releases(&mut self) {
        self.storage.drain_releases();
    }

    fn prepare_allocation(&mut self) {
        self.drain_releases();
    }

    pub(super) fn int(&mut self, type_id: IntListTypeId, values: Vec<BigInt>) -> IntListValueId {
        self.prepare_allocation();
        let slot = self.storage.pools.borrow_mut().ints.allocate(values);
        IntListValueId::new(type_id, self.storage.core(ListStorageKey::Int { slot }))
    }

    pub(super) fn string(
        &mut self,
        type_id: StringListTypeId,
        values: Vec<EcoString>,
    ) -> StringListValueId {
        self.prepare_allocation();
        let slot = self.storage.pools.borrow_mut().strings.allocate(values);
        StringListValueId::new(type_id, self.storage.core(ListStorageKey::String { slot }))
    }

    pub(super) fn bit_array(
        &mut self,
        type_id: BitArrayListTypeId,
        values: Vec<EvaluatedBitArray>,
    ) -> BitArrayListValueId {
        self.prepare_allocation();
        let slot = self.storage.pools.borrow_mut().bit_arrays.allocate(values);
        BitArrayListValueId::new(
            type_id,
            self.storage.core(ListStorageKey::BitArray { slot }),
        )
    }

    pub(super) fn utf_codepoint(
        &mut self,
        type_id: UtfCodepointListTypeId,
        values: Vec<char>,
    ) -> UtfCodepointListValueId {
        self.prepare_allocation();
        let slot = self
            .storage
            .pools
            .borrow_mut()
            .utf_codepoints
            .allocate(values);
        UtfCodepointListValueId::new(
            type_id,
            self.storage.core(ListStorageKey::UtfCodepoint { slot }),
        )
    }

    pub(super) fn custom(&mut self, allocation: CustomListAllocation) -> CustomListValueId {
        self.prepare_allocation();
        let slot = self
            .storage
            .pools
            .borrow_mut()
            .customs
            .allocate(allocation.values);
        CustomListValueId::new(
            allocation.type_id,
            self.storage.core(ListStorageKey::Custom { slot }),
        )
    }

    pub(super) fn external(&mut self, allocation: ExternalListAllocation) -> ExternalListValueId {
        self.prepare_allocation();
        let slot = self
            .storage
            .pools
            .borrow_mut()
            .externals
            .allocate(allocation.values);
        ExternalListValueId::new(
            allocation.type_id,
            self.storage.core(ListStorageKey::External { slot }),
        )
    }

    pub(super) fn float(&mut self, type_id: FloatListTypeId, values: Vec<f64>) -> FloatListValueId {
        self.prepare_allocation();
        let slot = self.storage.pools.borrow_mut().floats.allocate(values);
        FloatListValueId::new(type_id, self.storage.core(ListStorageKey::Float { slot }))
    }

    pub(super) fn bool(&mut self, type_id: BoolListTypeId, values: Vec<bool>) -> BoolListValueId {
        self.prepare_allocation();
        let slot = self.storage.pools.borrow_mut().bools.allocate(values);
        BoolListValueId::new(type_id, self.storage.core(ListStorageKey::Bool { slot }))
    }

    pub(super) fn nil(&mut self, type_id: NilListTypeId, len: usize) -> NilListValueId {
        self.prepare_allocation();
        let slot = self.storage.pools.borrow_mut().nils.allocate(len);
        NilListValueId::new(type_id, self.storage.core(ListStorageKey::Nil { slot }))
    }

    pub(super) fn tuple(
        &mut self,
        type_id: TupleListTypeId,
        values: Vec<Vec<EvaluatedValue>>,
    ) -> TupleListValueId {
        self.prepare_allocation();
        let slot = self.storage.pools.borrow_mut().tuples.allocate(values);
        TupleListValueId::new(type_id, self.storage.core(ListStorageKey::Tuple { slot }))
    }

    pub(super) fn parameter_list_list(
        &mut self,
        type_id: ParameterListListTypeId,
        len: usize,
    ) -> ParameterListListValueId {
        self.prepare_allocation();
        let slot = self
            .storage
            .pools
            .borrow_mut()
            .parameter_list_lists
            .allocate(len);
        ParameterListListValueId::new(
            type_id,
            self.storage.core(ListStorageKey::ParameterList { slot }),
        )
    }

    pub(super) fn list(
        &mut self,
        type_id: ListListTypeId,
        values: Vec<StoredListValueId>,
    ) -> ListListValueId {
        self.prepare_allocation();
        let slot = self.storage.pools.borrow_mut().lists.allocate(values);
        ListListValueId::new(type_id, self.storage.core(ListStorageKey::List { slot }))
    }

    pub(super) fn function(
        &mut self,
        type_id: FunctionListTypeId,
        values: Vec<EvaluatedFunctionValue>,
    ) -> FunctionListValueId {
        self.prepare_allocation();
        let slot = self.storage.pools.borrow_mut().functions.allocate(values);
        FunctionListValueId::new(
            type_id,
            self.storage.core(ListStorageKey::Function { slot }),
        )
    }

    pub(super) fn int_values<'value>(
        &self,
        value: &'value IntListValueId,
    ) -> Ref<'value, [BigInt]> {
        value.core.storage().drain_releases();
        Ref::map(value.core.storage().pools.borrow(), |pools| {
            pools.ints.get(value.core.slot()).as_slice()
        })
    }

    pub(super) fn string_values<'value>(
        &self,
        value: &'value StringListValueId,
    ) -> Ref<'value, [EcoString]> {
        value.core.storage().drain_releases();
        Ref::map(value.core.storage().pools.borrow(), |pools| {
            pools.strings.get(value.core.slot()).as_slice()
        })
    }

    pub(super) fn bit_array_values<'value>(
        &self,
        value: &'value BitArrayListValueId,
    ) -> Ref<'value, [EvaluatedBitArray]> {
        value.core.storage().drain_releases();
        Ref::map(value.core.storage().pools.borrow(), |pools| {
            pools.bit_arrays.get(value.core.slot()).as_slice()
        })
    }

    pub(super) fn utf_codepoint_values<'value>(
        &self,
        value: &'value UtfCodepointListValueId,
    ) -> Ref<'value, [char]> {
        value.core.storage().drain_releases();
        Ref::map(value.core.storage().pools.borrow(), |pools| {
            pools.utf_codepoints.get(value.core.slot()).as_slice()
        })
    }

    pub(super) fn custom_values<'value>(
        &self,
        value: &'value CustomListValueId,
    ) -> Ref<'value, [EvaluatedCustomValue]> {
        value.core.storage().drain_releases();
        Ref::map(value.core.storage().pools.borrow(), |pools| {
            pools.customs.get(value.core.slot()).as_slice()
        })
    }

    pub(super) fn external_values<'value>(
        &self,
        value: &'value ExternalListValueId,
    ) -> Ref<'value, [EvaluatedExternalValue]> {
        value.core.storage().drain_releases();
        Ref::map(value.core.storage().pools.borrow(), |pools| {
            pools.externals.get(value.core.slot()).as_slice()
        })
    }

    pub(super) fn float_values<'value>(
        &self,
        value: &'value FloatListValueId,
    ) -> Ref<'value, [f64]> {
        value.core.storage().drain_releases();
        Ref::map(value.core.storage().pools.borrow(), |pools| {
            pools.floats.get(value.core.slot()).as_slice()
        })
    }

    pub(super) fn bool_values<'value>(
        &self,
        value: &'value BoolListValueId,
    ) -> Ref<'value, [bool]> {
        value.core.storage().drain_releases();
        Ref::map(value.core.storage().pools.borrow(), |pools| {
            pools.bools.get(value.core.slot()).as_slice()
        })
    }

    pub(super) fn nil_len(&self, value: &NilListValueId) -> usize {
        value.core.storage().drain_releases();
        *value
            .core
            .storage()
            .pools
            .borrow()
            .nils
            .get(value.core.slot())
    }

    pub(super) fn tuple_values<'value>(
        &self,
        value: &'value TupleListValueId,
    ) -> Ref<'value, [Vec<EvaluatedValue>]> {
        value.core.storage().drain_releases();
        Ref::map(value.core.storage().pools.borrow(), |pools| {
            pools.tuples.get(value.core.slot()).as_slice()
        })
    }

    pub(super) fn parameter_list_list_len(&self, value: &ParameterListListValueId) -> usize {
        value.core.storage().drain_releases();
        *value
            .core
            .storage()
            .pools
            .borrow()
            .parameter_list_lists
            .get(value.core.slot())
    }

    pub(super) fn list_values<'value>(
        &self,
        value: &'value ListListValueId,
    ) -> Ref<'value, [StoredListValueId]> {
        value.core.storage().drain_releases();
        Ref::map(value.core.storage().pools.borrow(), |pools| {
            pools.lists.get(value.core.slot()).as_slice()
        })
    }

    pub(super) fn function_values<'value>(
        &self,
        value: &'value FunctionListValueId,
    ) -> Ref<'value, [EvaluatedFunctionValue]> {
        value.core.storage().drain_releases();
        Ref::map(value.core.storage().pools.borrow(), |pools| {
            pools.functions.get(value.core.slot()).as_slice()
        })
    }

    pub(super) fn list_len(&self, value: &ListValueId) -> usize {
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

    pub(super) fn evaluated_values(&self, value: &StoredListValueId) -> Vec<EvaluatedValue> {
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
            StoredListValueId::ParameterList(value) => {
                vec![
                    EvaluatedValue::ParameterList(ParameterListValueId::new(
                        value.type_id().item_type(),
                    ));
                    self.parameter_list_list_len(value)
                ]
            }
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

    pub(super) fn evaluated_value_at(
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

    pub(super) fn drop_first(
        &mut self,
        value: &StoredListValueId,
        count: usize,
    ) -> StoredListValueId {
        match value {
            StoredListValueId::Int(value) => {
                let values = {
                    let values = self.int_values(value);
                    values[count.min(values.len())..].to_vec()
                };
                self.int(value.type_id(), values).into()
            }
            StoredListValueId::String(value) => {
                let values = {
                    let values = self.string_values(value);
                    values[count.min(values.len())..].to_vec()
                };
                self.string(value.type_id(), values).into()
            }
            StoredListValueId::BitArray(value) => {
                let values = {
                    let values = self.bit_array_values(value);
                    values[count.min(values.len())..].to_vec()
                };
                self.bit_array(value.type_id(), values).into()
            }
            StoredListValueId::UtfCodepoint(value) => {
                let values = {
                    let values = self.utf_codepoint_values(value);
                    values[count.min(values.len())..].to_vec()
                };
                self.utf_codepoint(value.type_id(), values).into()
            }
            StoredListValueId::Custom(value) => {
                let values = {
                    let values = self.custom_values(value);
                    values[count.min(values.len())..].to_vec()
                };
                self.custom(CustomListAllocation::from_value(value, values))
                    .into()
            }
            StoredListValueId::External(value) => {
                let values = {
                    let values = self.external_values(value);
                    values[count.min(values.len())..].to_vec()
                };
                self.external(ExternalListAllocation::from_value(value, values))
                    .into()
            }
            StoredListValueId::Float(value) => {
                let values = {
                    let values = self.float_values(value);
                    values[count.min(values.len())..].to_vec()
                };
                self.float(value.type_id(), values).into()
            }
            StoredListValueId::Bool(value) => {
                let values = {
                    let values = self.bool_values(value);
                    values[count.min(values.len())..].to_vec()
                };
                self.bool(value.type_id(), values).into()
            }
            StoredListValueId::Nil(value) => {
                let len = self.nil_len(value).saturating_sub(count);
                self.nil(value.type_id(), len).into()
            }
            StoredListValueId::Tuple(value) => {
                let values = {
                    let values = self.tuple_values(value);
                    values[count.min(values.len())..].to_vec()
                };
                self.tuple(value.type_id(), values).into()
            }
            StoredListValueId::ParameterList(value) => {
                let len = self.parameter_list_list_len(value).saturating_sub(count);
                self.parameter_list_list(value.type_id(), len).into()
            }
            StoredListValueId::List(value) => {
                let values = {
                    let values = self.list_values(value);
                    values[count.min(values.len())..].to_vec()
                };
                self.list(value.type_id(), values).into()
            }
            StoredListValueId::Function(value) => {
                let values = {
                    let values = self.function_values(value);
                    values[count.min(values.len())..].to_vec()
                };
                self.function(value.type_id(), values).into()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        CustomListAllocation, ListListTypeId, ListValueId, ParameterListValueId, RuntimeState,
        StoredListValueId,
    };
    use crate::host::test::StatelessTestProvider;
    use crate::plan::execution::function::{
        CoreRuntimeFunctionId, ListFunctionId, RuntimeFunctionId, RuntimeListFunctionId,
    };
    use crate::plan::execution::type_::ListStorageTypeId;
    use crate::runtime::graph::RetainedValues;
    use crate::runtime::{
        EvaluatedBitArray, EvaluatedCapture, EvaluatedCustomValue, EvaluatedFunctionValue,
        EvaluatedIntFunction, EvaluatedValue,
    };
    use crate::{
        HostCall, HostCallCompletion, HostCallError, HostCallable, HostFailure, HostFunctionType,
        HostList, HostListType, HostTypeList, HostTypeListEnd, HostTypeParameter, HostValue,
        StatelessHostProfile,
    };
    use num_bigint::BigInt;
    use std::rc::Rc;

    fn return_host_list<'call>(
        call: HostCall<'call, StatelessHostProfile, StatelessTestProvider, HostListType<BigInt>>,
        value: BigInt,
    ) -> Result<HostCallCompletion<'call, HostListType<BigInt>>, HostCallError> {
        Ok(call.return_list([value]))
    }

    fn fail_with_host_list<'call>(
        _call: HostCall<'call, StatelessHostProfile, StatelessTestProvider, BigInt>,
        _values: HostList<'call, BigInt>,
    ) -> Result<HostCallCompletion<'call, BigInt>, HostCallError> {
        Err(HostFailure::new("stop").into())
    }

    type CallbackValue = HostTypeParameter<0>;
    type CallbackArguments = HostTypeList<CallbackValue, HostTypeListEnd>;
    type Callback = HostFunctionType<CallbackArguments, CallbackValue>;

    fn invoke_generic_callback<'call>(
        mut call: HostCall<'call, StatelessHostProfile, StatelessTestProvider, CallbackValue>,
        function: HostCallable<'call, CallbackArguments, CallbackValue>,
        value: HostValue<'call, CallbackValue>,
    ) -> Result<HostCallCompletion<'call, CallbackValue>, HostCallError> {
        let returned = call.invoke(function, (value, ()))?;
        Ok(call.return_value(returned))
    }

    fn int_main(plan: &crate::ExecutionPlan) -> crate::plan::execution::function::IntFunctionId {
        match plan.main_runtime() {
            RuntimeFunctionId::Core(CoreRuntimeFunctionId::Int(main)) => main,
            _ => panic!("main should lower into the Int function table"),
        }
    }

    fn source_panic(result: Result<(), crate::runtime::ExecutionError>) -> crate::runtime::Panic {
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
    fn runtime_state_exposes_owned_and_borrowed_host_state() {
        let mut echo = Vec::new();
        let mut plain = RuntimeState::new(&mut echo);

        assert_eq!(plain.host_state(), &mut ());

        let mut host = (num_bigint::BigInt::from(41), true);
        let mut echo = Vec::new();
        let mut hosted = RuntimeState::with_host(&mut echo, &mut host);
        hosted.host_state().0 += 1;

        assert!(hosted.host_state().1);

        drop(hosted);
        assert_eq!(host, (num_bigint::BigInt::from(42), true));
    }

    #[test]
    fn last_owner_enqueues_release_and_reuses_the_exact_slot() {
        let plan = crate::runtime::plan_src("pub fn main() -> List(Int) { [1] }");
        let type_id = plan.int_list_function_id(0).type_id();
        let mut echo = Vec::new();
        let mut state = RuntimeState::new(&mut echo);
        let value = state.values_mut().int(type_id, vec![1.into()]);
        let slot = value.core.slot();
        let retained = value.clone();

        drop(value);
        assert_eq!(state.values.storage.releases.borrow().as_slice(), &[]);
        drop(retained);
        assert_eq!(state.values.storage.releases.borrow().as_slice(), &[]);

        state.values_mut().drain_releases();
        assert_eq!(state.values.storage.pools.borrow().ints.free, vec![slot],);
        let reused = state.values_mut().int(type_id, vec![2.into()]);
        assert_eq!(reused.core.slot(), slot);
        assert_eq!(&*state.values().int_values(&reused), &[2.into()]);
    }

    #[test]
    fn release_waits_for_an_active_pool_borrow_before_reusing_the_slot() {
        let plan = crate::runtime::plan_src("pub fn main() -> List(Int) { [1] }");
        let type_id = plan.int_list_function_id(0).type_id();
        let mut echo = Vec::new();
        let mut state = RuntimeState::new(&mut echo);
        let value = state.values_mut().int(type_id, vec![1.into()]);
        let slot = value.core.slot();
        let pools = state.values.storage.pools.borrow_mut();

        drop(value);
        assert_eq!(state.values.storage.releases.borrow().len(), 1);

        drop(pools);
        state.values_mut().drain_releases();
        assert_eq!(state.values.storage.pools.borrow().ints.free, vec![slot]);
        assert!(state.values.storage.releases.borrow().is_empty());
    }

    #[test]
    fn bit_array_list_pool_preserves_type_and_reuses_released_slots() {
        let plan = crate::runtime::plan_src("pub fn main() -> List(BitArray) { [<<1>>] }");
        let type_id = plan.bit_array_list_function_id(0).type_id();
        let mut echo = Vec::new();
        let mut state = RuntimeState::new(&mut echo);
        let first = state.values_mut().bit_array(
            type_id,
            vec![crate::runtime::EvaluatedBitArray::new(
                bitvec::vec::BitVec::from_vec(vec![1]),
            )],
        );
        let slot = first.core.slot();

        assert_eq!(first.type_id(), type_id);
        assert_eq!(state.values().bit_array_values(&first)[0].bits().len(), 8);
        drop(first);
        state.values_mut().drain_releases();

        let second = state.values_mut().bit_array(type_id, Vec::new());
        assert_eq!(second.core.slot(), slot);
        assert_eq!(&*state.values().bit_array_values(&second), &[]);

        let value = ListValueId::BitArray(second.clone());
        assert_eq!(state.values().list_len(&value), 0);
        assert_eq!(
            StoredListValueId::from(second.clone()).into_value(),
            ListValueId::BitArray(second.clone()),
        );
        let dropped = state
            .values_mut()
            .drop_first(&StoredListValueId::BitArray(second), 0);
        assert_eq!(state.values().list_len(&dropped.clone().into_value()), 0);
    }

    #[test]
    fn repeated_release_and_allocation_keeps_one_slot_high_water_mark() {
        let plan = crate::runtime::plan_src("pub fn main() -> List(Int) { [1] }");
        let type_id = plan.int_list_function_id(0).type_id();
        let mut echo = Vec::new();
        let mut state = RuntimeState::new(&mut echo);

        for value in 0..10_000 {
            let list = state.values_mut().int(type_id, vec![value.into()]);
            drop(list);
            state.values_mut().drain_releases();
        }

        let pools = state.values.storage.pools.borrow();
        assert_eq!(pools.ints.slots.len(), 1);
        assert_eq!(pools.ints.free, vec![0]);
        assert_eq!(state.values.storage.releases.borrow().as_slice(), &[]);
    }

    #[test]
    fn host_calls_release_scoped_list_leases_after_success_and_failure() {
        let returned = crate::HostModule::new("host_support", "host/lists")
            .expect("host module should be valid")
            .with_scoped_function::<StatelessTestProvider, (BigInt,), HostListType<BigInt>, _>(
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
            )],
            crate::HostProviderSet::new([returned]).expect("host module should be unique"),
        )
        .expect("host source should compile");
        let plan = crate::plan_host_program(typed).expect("host source should plan");
        let execution = crate::HostedExecution::try_from_module_plan(plan)
            .expect("hosted execution should seal");
        let mut host = ();
        let mut echo = Vec::new();
        let mut state = RuntimeState::with_host(&mut echo, &mut host);

        assert_eq!(
            crate::runtime::run_hosted_program(&execution, &mut state),
            Ok(crate::Value::Int(BigInt::from(0))),
        );
        {
            let pools = state.values.storage.pools.borrow();
            assert_eq!(pools.ints.slots.len(), 1);
            assert_eq!(pools.ints.free, [0]);
        }
        assert!(state.values.storage.releases.borrow().is_empty());

        let failed = crate::HostModule::new("host_support", "host/lists")
            .expect("host module should be valid")
            .with_scoped_function::<StatelessTestProvider, (HostListType<BigInt>,), BigInt, _>(
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
        let execution = crate::HostedExecution::try_from_module_plan(plan)
            .expect("hosted execution should seal");
        let mut host = ();
        let mut echo = Vec::new();
        let mut state = RuntimeState::with_host(&mut echo, &mut host);

        let error = crate::runtime::run_hosted_program(&execution, &mut state)
            .expect_err("the host callback should fail");
        assert_eq!(
            error.to_string(),
            "host function host_support::host/lists.fail failed: stop",
        );
        {
            let pools = state.values.storage.pools.borrow();
            assert_eq!(pools.ints.slots.len(), 1);
            assert_eq!(pools.ints.free, [0]);
        }
        assert!(state.values.storage.releases.borrow().is_empty());
    }

    #[test]
    fn nested_callbacks_release_retained_custom_list_values_after_success() {
        let host = crate::HostModule::new("host_support", "host/callback")
            .expect("host module should be valid")
            .with_scoped_function::<
                StatelessTestProvider,
                (Callback, CallbackValue),
                CallbackValue,
                _,
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
        let execution = crate::HostedExecution::try_from_module_plan(plan)
            .expect("successful callback execution should seal");
        let mut host_state = ();
        let mut echo = Vec::new();
        let mut state = RuntimeState::with_host(&mut echo, &mut host_state);

        assert_eq!(
            crate::runtime::run_hosted_program(&execution, &mut state),
            Ok(crate::Value::Int(BigInt::from(0))),
        );
        {
            let pools = state.values.storage.pools.borrow();
            assert_eq!(pools.ints.slots.len(), 1);
            assert_eq!(pools.ints.free, [0]);
        }
        assert!(state.values.storage.releases.borrow().is_empty());
    }

    #[test]
    fn nested_callbacks_release_retained_custom_list_values_after_panic() {
        let host = crate::HostModule::new("host_support", "host/callback")
            .expect("host module should be valid")
            .with_scoped_function::<
                StatelessTestProvider,
                (Callback, CallbackValue),
                CallbackValue,
                _,
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
        let execution = crate::HostedExecution::try_from_module_plan(plan)
            .expect("panicking callback execution should seal");
        let mut host_state = ();
        let mut echo = Vec::new();
        let mut state = RuntimeState::with_host(&mut echo, &mut host_state);

        let panic =
            source_panic(crate::runtime::run_hosted_program(&execution, &mut state).map(drop));
        assert_eq!(panic.kind(), crate::PanicKind::Panic);
        assert_eq!(panic.site().function(), "stop");
        {
            let pools = state.values.storage.pools.borrow();
            assert_eq!(pools.ints.slots.len(), 1);
            assert_eq!(pools.ints.free, [0]);
        }
        assert!(state.values.storage.releases.borrow().is_empty());
    }

    #[test]
    fn tail_recursive_block_replacement_reuses_a_fixed_list_slot_set() {
        let plan = crate::runtime::plan_src(include_str!(
            "../../tests/fixtures/execution/functions/tail_call/list_tail_recursion_replaces_allocations.gleam"
        ));
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

        assert_eq!(&*state.values().int_values(&value), &[1.into()]);
        {
            let pools = state.values.storage.pools.borrow();
            assert_eq!(pools.ints.slots.len(), 1);
            assert_eq!(pools.ints.free.len(), 0);
        }
        drop(value);
        state.values_mut().drain_releases();
        assert_eq!(state.values.storage.pools.borrow().ints.free.len(), 1);
        assert_eq!(state.values.storage.releases.borrow().as_slice(), &[]);
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
        let pools = state.values.storage.pools.borrow();
        assert_eq!(pools.ints.slots.len(), 1);
        assert_eq!(pools.ints.free, vec![0]);
        assert_eq!(state.values.storage.releases.borrow().as_slice(), &[]);
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
        let pools = state.values.storage.pools.borrow();
        assert_eq!(pools.ints.slots.len(), 1);
        assert_eq!(pools.ints.free, vec![0]);
        assert_eq!(state.values.storage.releases.borrow().as_slice(), &[]);
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
        let value = state.values_mut().int(type_id, vec![1.into()]);
        let clone = value.clone();
        let storage = Rc::clone(&value.core.lease.storage);
        let discarded = state.values_mut().int(type_id, vec![2.into()]);
        let discarded_slot = discarded.core.slot();
        let mut other_echo = Vec::new();
        let mut other_state = RuntimeState::new(&mut other_echo);
        let other = other_state.values_mut().int(type_id, vec![1.into()]);

        assert_eq!(value, clone);
        assert_ne!(value, other);
        assert_eq!(
            format!("{:?}", value.core.lease),
            "ListLease { key: Int { slot: 0 } }"
        );

        drop(discarded);
        assert_eq!(
            storage.pools.borrow().ints.get(discarded_slot).as_slice(),
            &[],
        );
        drop(state);
        assert_eq!(
            storage
                .pools
                .borrow()
                .ints
                .get(value.core.slot())
                .as_slice(),
            &[1.into()],
        );
        drop(value);
        assert_eq!(
            storage
                .pools
                .borrow()
                .ints
                .get(clone.core.slot())
                .as_slice(),
            &[1.into()],
        );
        drop(clone);
        let pools = storage.pools.borrow();
        assert_eq!(pools.ints.slots, vec![Vec::<BigInt>::new(), Vec::new()]);
        assert_eq!(pools.ints.free, vec![discarded_slot, 0]);

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
            Vec::new(),
            crate::plan::execution::type_::FunctionType::new(
                Vec::new(),
                crate::plan::execution::type_::ValueType::Int,
            ),
        );
        let int = state
            .values_mut()
            .int(plan.int_list_function_id(0).type_id(), vec![1.into()]);
        let string = state.values_mut().string(
            plan.string_list_function_id(0).type_id(),
            vec!["one".into()],
        );
        let bit_array = state.values_mut().bit_array(
            plan.bit_array_list_function_id(0).type_id(),
            vec![EvaluatedBitArray::new(bitvec::vec::BitVec::from_vec(vec![
                1,
            ]))],
        );
        let utf_codepoint = state.values_mut().utf_codepoint(
            plan.utf_codepoint_list_function_id(0).type_id(),
            vec!['\u{10ffff}'],
        );
        let custom_constructor = plan.custom_constructor_id(0, 0);
        let custom = state.values_mut().custom(CustomListAllocation::new(
            plan.custom_list_function_id(0).type_id(),
            vec![EvaluatedCustomValue::from_fields(
                custom_constructor,
                vec![EvaluatedValue::Int(1.into())].into_boxed_slice(),
            )],
        ));
        let float = state
            .values_mut()
            .float(plan.float_list_function_id(0).type_id(), vec![1.5]);
        let bool_ = state
            .values_mut()
            .bool(plan.bool_list_function_id(0).type_id(), vec![true]);
        let nil = state
            .values_mut()
            .nil(plan.nil_list_function_id(0).type_id(), 1);
        let tuple = state.values_mut().tuple(
            plan.tuple_list_function_id(0).type_id(),
            vec![vec![EvaluatedValue::Int(1.into())]],
        );
        let parameter = ParameterListValueId::new(plan.parameter_list_function_id(0).type_id());
        let parameter_list = state
            .values_mut()
            .parameter_list_list(plan.parameter_list_list_function_id(0).type_id(), 1);
        let child = state
            .values_mut()
            .int(plan.int_list_function_id(0).type_id(), vec![1.into()]);
        let list = state.values_mut().list(
            plan.list_list_function_id(0).type_id(),
            vec![child.clone().into()],
        );
        let function = state.values_mut().function(
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
                .values()
                .evaluated_value_at(&ListValueId::Parameter(parameter), 0),
            None,
        );
        assert_eq!(
            state
                .values()
                .evaluated_value_at(&ListValueId::Int(int.clone()), 0),
            Some(EvaluatedValue::Int(1.into())),
        );
        assert_eq!(
            state
                .values()
                .evaluated_value_at(&ListValueId::String(string.clone()), 0),
            Some(EvaluatedValue::String("one".into())),
        );
        assert_eq!(
            state
                .values()
                .evaluated_value_at(&ListValueId::BitArray(bit_array.clone()), 0),
            Some(EvaluatedValue::BitArray(EvaluatedBitArray::new(
                bitvec::vec::BitVec::from_vec(vec![1]),
            ))),
        );
        assert_eq!(
            state
                .values()
                .evaluated_value_at(&ListValueId::UtfCodepoint(utf_codepoint.clone()), 0),
            Some(EvaluatedValue::UtfCodepoint('\u{10ffff}')),
        );
        assert_eq!(
            state
                .values()
                .evaluated_value_at(&ListValueId::Custom(custom.clone()), 0),
            Some(EvaluatedValue::Custom(expected_custom)),
        );
        assert_eq!(
            state
                .values()
                .evaluated_value_at(&ListValueId::Float(float.clone()), 0),
            Some(EvaluatedValue::Float(1.5)),
        );
        assert_eq!(
            state
                .values()
                .evaluated_value_at(&ListValueId::Bool(bool_.clone()), 0),
            Some(EvaluatedValue::Bool(true)),
        );
        assert_eq!(
            state
                .values()
                .evaluated_value_at(&ListValueId::Nil(nil.clone()), 0),
            Some(EvaluatedValue::Nil),
        );
        assert_eq!(
            state
                .values()
                .evaluated_value_at(&ListValueId::Tuple(tuple.clone()), 0),
            Some(EvaluatedValue::Tuple(vec![EvaluatedValue::Int(1.into())])),
        );
        assert_eq!(
            state
                .values()
                .evaluated_value_at(&ListValueId::ParameterList(parameter_list.clone()), 0),
            Some(EvaluatedValue::ParameterList(ParameterListValueId::new(
                parameter_list.type_id().item_type(),
            ))),
        );
        assert_eq!(
            state
                .values()
                .evaluated_value_at(&ListValueId::List(list.clone()), 0),
            Some(expected_child),
        );
        assert_eq!(
            state
                .values()
                .evaluated_value_at(&ListValueId::Function(function.clone()), 0),
            Some(EvaluatedValue::Function(EvaluatedFunctionValue::from(
                int_function,
            ))),
        );
        assert_eq!(
            state
                .values()
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
            assert_eq!(state.values().list_len(&value.clone().into_value()), 1);

            let dropped = state.values_mut().drop_first(&value, 1);
            assert_eq!(dropped.list_type(), list_type);
            assert_eq!(state.values().list_len(&dropped.into_value()), 0);
        }

        assert_eq!(
            StoredListValueId::from(parameter_list.clone()).into_core(),
            parameter_list.clone().into_core(),
        );
        assert_eq!(
            state.values().list_len(&ListValueId::Parameter(parameter)),
            0
        );
        assert_eq!(
            state
                .values()
                .list_len(&ListValueId::ParameterList(parameter_list.clone())),
            1
        );
        let dropped = state.values_mut().drop_first(
            &StoredListValueId::ParameterList(parameter_list),
            usize::MAX,
        );
        assert_eq!(state.values().list_len(&dropped.clone().into_value()), 0);
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
        let child = state.values_mut().int(child_type, vec![1.into()]);
        let child_slot = child.core.slot();
        let parent = state
            .values_mut()
            .list(parent_type, vec![child.clone().into()]);

        drop(parent);
        state.values_mut().drain_releases();
        {
            let pools = state.values.storage.pools.borrow();
            assert_eq!(pools.lists.free.len(), 1);
            assert_eq!(pools.ints.free, Vec::<usize>::new());
        }
        assert_eq!(&*state.values().int_values(&child), &[1.into()]);

        drop(child);
        state.values_mut().drain_releases();
        assert_eq!(
            state.values.storage.pools.borrow().ints.free,
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
        let mut value: StoredListValueId =
            state.values_mut().int(child_type, vec![1.into()]).into();
        for parent in parents.into_iter().rev() {
            value = state.values_mut().list(parent, vec![value]).into();
        }
        let allocated_list_slots = state.values.storage.pools.borrow().lists.slots.len();

        drop(value);
        state.values_mut().drain_releases();
        let pools = state.values.storage.pools.borrow();
        assert_eq!(pools.ints.free.len(), 1);
        assert_eq!(pools.lists.free.len(), allocated_list_slots);
        assert_eq!(state.values.storage.releases.borrow().as_slice(), &[]);
    }

    #[test]
    fn closure_capture_retains_its_list_until_the_closure_is_dropped() {
        let plan = crate::runtime::plan_src(
            "fn keep(values: List(Int)) { fn() { values } } pub fn main() { keep([1]) }",
        );
        let type_id = plan.int_list_function_id(0).type_id();
        let mut echo = Vec::new();
        let mut state = RuntimeState::new(&mut echo);
        let value = state.values_mut().int(type_id, vec![1.into()]);
        let slot = value.core.slot();
        let closure = EvaluatedIntFunction::reference(
            crate::plan::execution::function::IntFunctionId(0),
            Vec::new(),
            vec![EvaluatedCapture::list(
                crate::runtime::EvaluatedListCapture::Int {
                    local: crate::plan::execution::graph::IntListLocalId(0),
                    value: value.clone(),
                },
            )],
            crate::plan::execution::type_::FunctionType::new(
                Vec::new(),
                crate::plan::execution::type_::ValueType::Int,
            ),
        );

        drop(value);
        state.values_mut().drain_releases();
        assert_eq!(
            state.values.storage.pools.borrow().ints.free,
            Vec::<usize>::new(),
        );
        drop(closure);
        state.values_mut().drain_releases();
        assert_eq!(state.values.storage.pools.borrow().ints.free, vec![slot],);
    }

    fn nested_list_storage(storage: ListStorageTypeId) -> Option<ListListTypeId> {
        match storage {
            ListStorageTypeId::List(type_id) => Some(type_id),
            _ => None,
        }
    }
}
