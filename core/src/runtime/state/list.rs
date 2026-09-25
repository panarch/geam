mod sequence;

pub(in crate::runtime) use sequence::{ListSequence, ListSequenceIter};

use num_bigint::BigInt;
use std::fmt;
use std::sync::{Arc, Mutex, MutexGuard};

use crate::StringValue;
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

#[derive(Clone, Default)]
pub(crate) struct RuntimeListStorage {
    releases: Arc<ListReleaseQueue>,
}

// Typed handles retain both their exact storage identity and their payload.
// Only construction and composite destruction use the storage's release queue.
macro_rules! typed_list_value_id {
    ($name:ident, $type_id:ty, $variant:ident, items $item:ty) => {
        typed_list_value_id!(@owner $name, $type_id, $variant, ListLease<$item>);

        impl $name {
            pub(in crate::runtime) fn values(&self) -> &ListSequence<$item> {
                &self.lease.values
            }
        }
    };
    ($name:ident, $type_id:ty, $variant:ident, composite $item:ty) => {
        typed_list_value_id!(@owner $name, $type_id, $variant, dyn ListReadOwner<$item>);

        impl $name {
            pub(in crate::runtime) fn values(&self) -> &ListSequence<$item> {
                self.lease.values()
            }
        }
    };
    ($name:ident, $type_id:ty, $variant:ident, length) => {
        typed_list_value_id!(@owner $name, $type_id, $variant, usize);

        impl $name {
            pub(in crate::runtime) fn len(&self) -> usize {
                *self.lease
            }
        }
    };
    (@owner $name:ident, $type_id:ty, $variant:ident, $owner:ty) => {
        #[derive(Clone)]
        pub(in crate::runtime) struct $name {
            type_id: $type_id,
            lease: Arc<$owner>,
        }

        impl $name {
            pub(in crate::runtime) fn type_id(&self) -> $type_id {
                self.type_id
            }

            pub(in crate::runtime) fn from_stored(value: &StoredListValueId) -> Option<Self> {
                match value {
                    StoredListValueId::$variant(value) => Some(value.clone()),
                    _ => None,
                }
            }
        }

        // This is handle identity, not source list equality.
        impl PartialEq for $name {
            fn eq(&self, other: &Self) -> bool {
                self.type_id == other.type_id && Arc::ptr_eq(&self.lease, &other.lease)
            }
        }

        impl fmt::Debug for $name {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter
                    .debug_struct(stringify!($name))
                    .field("type_id", &self.type_id)
                    .field("lease", &Arc::as_ptr(&self.lease))
                    .finish()
            }
        }

        impl From<$name> for ListValueId {
            fn from(value: $name) -> Self {
                Self::$variant(value)
            }
        }
    };
}

typed_list_value_id!(IntListValueId, IntListTypeId, Int, items BigInt);
typed_list_value_id!(StringListValueId, StringListTypeId, String, items StringValue);
typed_list_value_id!(BitArrayListValueId, BitArrayListTypeId, BitArray, items EvaluatedBitArray);
typed_list_value_id!(UtfCodepointListValueId, UtfCodepointListTypeId, UtfCodepoint, items char);
typed_list_value_id!(CustomListValueId, CustomListTypeId, Custom, composite EvaluatedCustomValue);
typed_list_value_id!(ExternalListValueId, ExternalListTypeId, External, composite EvaluatedExternalValue);
typed_list_value_id!(FloatListValueId, FloatListTypeId, Float, items f64);
typed_list_value_id!(BoolListValueId, BoolListTypeId, Bool, items bool);
typed_list_value_id!(NilListValueId, NilListTypeId, Nil, length);
typed_list_value_id!(TupleListValueId, TupleListTypeId, Tuple, composite Vec<EvaluatedValue>);
typed_list_value_id!(
    ParameterListListValueId,
    ParameterListListTypeId,
    ParameterList,
    length
);
typed_list_value_id!(ListListValueId, ListListTypeId, List, composite StoredListValueId);
typed_list_value_id!(FunctionListValueId, FunctionListTypeId, Function, composite EvaluatedFunctionValue);

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
}

impl From<StoredListValueId> for ListValueId {
    fn from(value: StoredListValueId) -> Self {
        value.into_value()
    }
}

struct ListLease<Item: ListItem> {
    values: ListSequence<Item>,
    release: Item::Release,
}

impl<Item: ListItem> ListLease<Item> {
    fn new(values: ListSequence<Item>, releases: &Arc<ListReleaseQueue>) -> Self {
        Self {
            values,
            release: Item::release_context(releases),
        }
    }
}

impl<Item: ListItem> Drop for ListLease<Item> {
    fn drop(&mut self) {
        Item::release(&self.release, std::mem::take(&mut self.values));
    }
}

// The item type remains exact, while this narrow read owner stops recursive
// auto-trait expansion in external consumers. Leaf handles stay concrete.
trait ListReadOwner<Item>: Send + Sync {
    fn values(&self) -> &ListSequence<Item>;
}

impl<Item: ListItem + Send + Sync> ListReadOwner<Item> for ListLease<Item>
where
    Item::Release: Send + Sync,
{
    fn values(&self) -> &ListSequence<Item> {
        &self.values
    }
}

// This private, closed trait selects release behavior at construction. Reads
// neither inspect a family tag nor acquire the release lock.
trait ListItem: Sized {
    type Release;

    fn release_context(queue: &Arc<ListReleaseQueue>) -> Self::Release;
    fn release(context: &Self::Release, values: ListSequence<Self>);
}

macro_rules! leaf_list_item {
    ($item:ty) => {
        impl ListItem for $item {
            type Release = ();

            fn release_context(_queue: &Arc<ListReleaseQueue>) {}

            fn release(_context: &(), values: ListSequence<Self>) {
                drop(values);
            }
        }
    };
}

leaf_list_item!(BigInt);
leaf_list_item!(StringValue);
leaf_list_item!(EvaluatedBitArray);
leaf_list_item!(char);
leaf_list_item!(f64);
leaf_list_item!(bool);

macro_rules! composite_list_item {
    ($item:ty, $variant:ident) => {
        impl ListItem for $item {
            type Release = Arc<ListReleaseQueue>;

            fn release_context(queue: &Arc<ListReleaseQueue>) -> Self::Release {
                Arc::clone(queue)
            }

            fn release(context: &Self::Release, values: ListSequence<Self>) {
                context.release(ReleasedList::$variant(values));
            }
        }
    };
}

composite_list_item!(EvaluatedCustomValue, Custom);
composite_list_item!(EvaluatedExternalValue, External);
composite_list_item!(Vec<EvaluatedValue>, Tuple);
composite_list_item!(StoredListValueId, List);
composite_list_item!(EvaluatedFunctionValue, Function);

#[derive(Default)]
struct ListReleaseQueue {
    state: Mutex<ListReleaseState>,
}

#[derive(Default)]
struct ListReleaseState {
    pending: Vec<ReleasedList>,
    draining: bool,
}

// Only pending destruction is type-erased. Live payloads remain in typed leases.
enum ReleasedList {
    Custom(ListSequence<EvaluatedCustomValue>),
    External(ListSequence<EvaluatedExternalValue>),
    Tuple(ListSequence<Vec<EvaluatedValue>>),
    List(ListSequence<StoredListValueId>),
    Function(ListSequence<EvaluatedFunctionValue>),
}

impl ListReleaseQueue {
    fn release(&self, values: ReleasedList) {
        let should_drain = {
            let mut state = lock(&self.state);
            state.pending.push(values);
            if state.draining {
                false
            } else {
                state.draining = true;
                true
            }
        };
        if should_drain {
            self.drain();
        }
    }

    fn drain(&self) {
        let mut finish = FinishReleaseOnUnwind {
            queue: self,
            armed: true,
        };
        loop {
            let released = {
                let mut state = lock(&self.state);
                let Some(values) = state.pending.pop() else {
                    state.draining = false;
                    finish.armed = false;
                    return;
                };
                values
            };
            // Payload Drop can enqueue more work or re-enter the runtime.
            released.drop_values();
        }
    }
}

impl ReleasedList {
    fn drop_values(self) {
        match self {
            Self::Custom(values) => drop(values),
            Self::External(values) => drop(values),
            Self::Tuple(values) => drop(values),
            Self::List(values) => drop(values),
            Self::Function(values) => drop(values),
        }
    }
}

struct FinishReleaseOnUnwind<'queue> {
    queue: &'queue ListReleaseQueue,
    armed: bool,
}

impl Drop for FinishReleaseOnUnwind<'_> {
    fn drop(&mut self) {
        if self.armed {
            self.queue.drain();
        }
    }
}

macro_rules! value_storage {
    ($allocate:ident, $store:ident, $read:ident, $prepend:ident, $tail:ident,
     $type_id:ty, $item:ty, $handle:ident) => {
        pub(in crate::runtime) fn $allocate(
            &self,
            type_id: $type_id,
            values: Vec<$item>,
        ) -> $handle {
            self.$store(type_id, values.into())
        }

        sequence_storage!($store, $read, $prepend, $tail, $type_id, $item, $handle);
    };
}

macro_rules! sequence_storage {
    ($store:ident, $read:ident, $prepend:ident, $tail:ident,
     $type_id:ty, $item:ty, $handle:ident) => {
        fn $store(&self, type_id: $type_id, values: ListSequence<$item>) -> $handle {
            $handle {
                type_id,
                lease: Arc::new(ListLease::new(values, &self.releases)),
            }
        }

        pub(in crate::runtime) fn $read<'value>(
            &self,
            value: &'value $handle,
        ) -> &'value ListSequence<$item> {
            value.values()
        }

        pub(in crate::runtime) fn $prepend(
            &self,
            type_id: $type_id,
            prefix: Vec<$item>,
            tail: &$handle,
        ) -> $handle {
            self.$store(type_id, self.$read(tail).prepend(prefix))
        }

        pub(in crate::runtime) fn $tail(
            &self,
            type_id: $type_id,
            value: &$handle,
            count: usize,
        ) -> $handle {
            self.$store(type_id, self.$read(value).suffix(count))
        }
    };
}

impl RuntimeListStorage {
    value_storage!(
        int,
        store_int,
        int_values,
        prepend_int,
        tail_int,
        IntListTypeId,
        BigInt,
        IntListValueId
    );
    value_storage!(
        string,
        store_string,
        string_values,
        prepend_string,
        tail_string,
        StringListTypeId,
        StringValue,
        StringListValueId
    );
    value_storage!(
        bit_array,
        store_bit_array,
        bit_array_values,
        prepend_bit_array,
        tail_bit_array,
        BitArrayListTypeId,
        EvaluatedBitArray,
        BitArrayListValueId
    );
    value_storage!(
        utf_codepoint,
        store_utf_codepoint,
        utf_codepoint_values,
        prepend_utf_codepoint,
        tail_utf_codepoint,
        UtfCodepointListTypeId,
        char,
        UtfCodepointListValueId
    );
    value_storage!(
        float,
        store_float,
        float_values,
        prepend_float,
        tail_float,
        FloatListTypeId,
        f64,
        FloatListValueId
    );
    value_storage!(
        bool,
        store_bool,
        bool_values,
        prepend_bool,
        tail_bool,
        BoolListTypeId,
        bool,
        BoolListValueId
    );
    value_storage!(
        tuple,
        store_tuple,
        tuple_values,
        prepend_tuple,
        tail_tuple,
        TupleListTypeId,
        Vec<EvaluatedValue>,
        TupleListValueId
    );
    value_storage!(
        list,
        store_list,
        list_values,
        prepend_list,
        tail_list,
        ListListTypeId,
        StoredListValueId,
        ListListValueId
    );
    value_storage!(
        function,
        store_function,
        function_values,
        prepend_function,
        tail_function,
        FunctionListTypeId,
        EvaluatedFunctionValue,
        FunctionListValueId
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
        CustomListValueId
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
        ExternalListValueId
    );

    pub(in crate::runtime) fn nil(&self, type_id: NilListTypeId, len: usize) -> NilListValueId {
        NilListValueId {
            type_id,
            lease: Arc::new(len),
        }
    }

    pub(in crate::runtime) fn nil_len(&self, value: &NilListValueId) -> usize {
        value.len()
    }

    pub(in crate::runtime) fn parameter_list_list(
        &self,
        type_id: ParameterListListTypeId,
        len: usize,
    ) -> ParameterListListValueId {
        ParameterListListValueId {
            type_id,
            lease: Arc::new(len),
        }
    }

    pub(in crate::runtime) fn parameter_list_list_len(
        &self,
        value: &ParameterListListValueId,
    ) -> usize {
        value.len()
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

fn lock<Value>(mutex: &Mutex<Value>) -> MutexGuard<'_, Value> {
    mutex
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
}

#[cfg(test)]
mod storage_tests {
    use super::{ListSequence, RuntimeListStorage, lock};
    use crate::runtime::evaluated::{
        EvaluatedBitArray, EvaluatedCustomValue, EvaluatedExternalValue, EvaluatedFunctionValue,
        EvaluatedIntFunction,
    };

    use crate::host::{
        ExternalTestProfile, HostExternalEquality, HostExternalHashing, HostExternalInspection,
        HostExternalStore,
    };
    use crate::plan::execution::function::IntFunctionId;
    use crate::plan::execution::type_::{ExternalListTypeId, FunctionType, ValueType};
    use crate::runtime::profile::external_test::{RuntimeCounterProvider, RuntimeCounterSchema};
    use crate::runtime::retained::{
        RetainedValueEquality, RetainedValueHashing, RetainedValueInspection, RetainedValueRef,
    };
    use crate::runtime::state::list::{
        CustomListAllocation, ExternalListAllocation, ListValueId, ParameterListValueId,
        StoredListValueId,
    };
    use crate::runtime::{EvaluatedValue, plan_src};
    use crate::{
        HostModule, HostProviderModule, HostProviderSet, HostedExecution, ModuleSource,
        PackageSource, compile_typed_host_program, plan_host_program,
    };
    use ecow::EcoString;
    use num_bigint::BigInt;
    use std::cell::Cell;
    use std::panic::{AssertUnwindSafe, catch_unwind};
    use std::sync::{Arc, Barrier, Mutex};
    use std::{iter, ptr, slice, thread};

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

    fn external_list_type() -> ExternalListTypeId {
        let provider = HostProviderModule::<ExternalTestProfile>::new("application", "main")
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
                Vec::<HostModule<ExternalTestProfile>>::new(),
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

    fn external_equal(context: &HostExternalEquality<'_>, left: &BigInt, right: &BigInt) -> bool {
        context.0.stored_values_equal(
            &RetainedValueRef::new(&EvaluatedValue::Int(left.clone())),
            &RetainedValueRef::new(&EvaluatedValue::Int(right.clone())),
        )
    }

    fn external_hash(context: &HostExternalHashing<'_>, value: &BigInt) -> u64 {
        context
            .0
            .stored_value_hash(&RetainedValueRef::new(&EvaluatedValue::Int(value.clone())))
    }

    fn external_inspect(context: &HostExternalInspection<'_>, value: &BigInt) -> EcoString {
        format!(
            "Counter({})",
            context
                .0
                .inspect_stored_value(&RetainedValueRef::new(&EvaluatedValue::Int(value.clone())))
        )
        .into()
    }

    // The Cell makes the payload Send but not Sync; it deliberately has no Clone.
    struct ReleaseProbe {
        id: Cell<usize>,
        drops: Arc<Mutex<Vec<usize>>>,
        on_drop: Option<Box<dyn FnOnce() + Send>>,
    }

    impl Drop for ReleaseProbe {
        fn drop(&mut self) {
            lock(&self.drops).push(self.id.get());
            if let Some(on_drop) = self.on_drop.take() {
                on_drop();
            }
        }
    }

    fn external_probe(
        store: &HostExternalStore<ReleaseProbe>,
        type_id: ExternalListTypeId,
        value: ReleaseProbe,
    ) -> EvaluatedExternalValue {
        EvaluatedExternalValue::new(
            type_id.item_type(),
            store.insert(
                value,
                |context, left, right| {
                    external_equal(context, &left.id.get().into(), &right.id.get().into())
                },
                |context, value| external_hash(context, &value.id.get().into()),
                |context, value| external_inspect(context, &value.id.get().into()),
                |_| None,
            ),
        )
    }

    #[test]
    fn external_payload_reentry_and_single_unwind_finish_the_lifo_drain() {
        let type_id = external_list_type();
        let int_plan = plan_src("pub fn main() -> List(Int) { [7] }");
        let int_type = int_plan.int_list_function_id(0).type_id();
        for unwind in [false, true] {
            let storage = RuntimeListStorage::default();
            let store = HostExternalStore::default();
            let drops = Arc::new(Mutex::new(Vec::new()));
            let mut children = Vec::new();
            let mut owners = Vec::new();
            for id in 1..=2 {
                let child = storage.external(ExternalListAllocation::new(
                    type_id,
                    vec![external_probe(
                        &store,
                        type_id,
                        ReleaseProbe {
                            id: id.into(),
                            drops: Arc::clone(&drops),
                            on_drop: None,
                        },
                    )],
                ));
                owners.push(Arc::downgrade(&child.lease));
                children.push(child);
            }
            let reentrant_storage = storage.clone();
            let reentrant_drops = Arc::clone(&drops);
            let root = storage.external(ExternalListAllocation::new(
                type_id,
                vec![external_probe(
                    &store,
                    type_id,
                    ReleaseProbe {
                        id: 0.into(),
                        drops: Arc::clone(&drops),
                        on_drop: Some(Box::new(move || {
                            assert!(
                                reentrant_storage
                                    .releases
                                    .state
                                    .try_lock()
                                    .unwrap()
                                    .draining
                            );
                            // The active drainer owns both releases; neither child drops here.
                            drop(children);
                            assert_eq!(*lock(&reentrant_drops), [0]);
                            assert_eq!(lock(&reentrant_storage.releases.state).pending.len(), 2);
                            let value = reentrant_storage.int(int_type, vec![7.into()]);
                            assert_eq!(value.values().get(0), Some(&BigInt::from(7)));
                            drop(value);
                            if unwind {
                                panic!("list payload unwind");
                            }
                        })),
                    },
                )],
            ));
            owners.push(Arc::downgrade(&root.lease));
            let result = catch_unwind(AssertUnwindSafe(|| drop(root)));
            assert_eq!(result.is_err(), unwind);
            if let Err(error) = result {
                assert_eq!(error.downcast_ref::<&str>(), Some(&"list payload unwind"));
            }
            assert_eq!(*lock(&drops), [0, 2, 1]);
            assert!(owners.iter().all(|owner| owner.strong_count() == 0));
            assert!(lock(&storage.releases.state).pending.is_empty());
            assert!(!lock(&storage.releases.state).draining);

            let next = storage.external(ExternalListAllocation::new(
                type_id,
                vec![external_probe(
                    &store,
                    type_id,
                    ReleaseProbe {
                        id: 3.into(),
                        drops: Arc::clone(&drops),
                        on_drop: None,
                    },
                )],
            ));
            drop(next);
            assert_eq!(*lock(&drops), [0, 2, 1, 3]);
            assert!(lock(&storage.releases.state).pending.is_empty());
            assert!(!lock(&storage.releases.state).draining);
        }
    }

    #[test]
    fn concurrent_last_owners_enqueue_while_a_payload_drop_is_running() {
        let type_id = external_list_type();
        let storage = RuntimeListStorage::default();
        let store = HostExternalStore::default();
        let drops = Arc::new(Mutex::new(Vec::new()));
        let entered = Arc::new(Barrier::new(2));
        let release = Arc::new(Barrier::new(2));
        let drop_entered = Arc::clone(&entered);
        let drop_release = Arc::clone(&release);
        let active = storage.external(ExternalListAllocation::new(
            type_id,
            vec![external_probe(
                &store,
                type_id,
                ReleaseProbe {
                    id: 0.into(),
                    drops: Arc::clone(&drops),
                    on_drop: Some(Box::new(move || {
                        drop_entered.wait();
                        drop_release.wait();
                    })),
                },
            )],
        ));
        let mut owners = vec![Arc::downgrade(&active.lease)];
        let mut batches = Vec::new();
        for batch in 0..4 {
            let mut values = Vec::new();
            for index in 1..=256 {
                values.push(external_probe(
                    &store,
                    type_id,
                    ReleaseProbe {
                        id: (batch * 256 + index).into(),
                        drops: Arc::clone(&drops),
                        on_drop: None,
                    },
                ));
            }
            let values = storage.external(ExternalListAllocation::new(type_id, values));
            owners.push(Arc::downgrade(&values.lease));
            batches.push(values);
        }
        thread::scope(|threads| {
            let drainer = threads.spawn(move || drop(active));
            entered.wait();
            let workers: Vec<_> = batches
                .into_iter()
                .map(|batch| threads.spawn(move || drop(batch)))
                .collect();
            for worker in workers {
                worker.join().unwrap();
            }
            let observed_drops = lock(&drops).clone();
            let pending = lock(&storage.releases.state).pending.len();
            let draining = lock(&storage.releases.state).draining;
            release.wait();
            drainer.join().unwrap();
            assert_eq!(observed_drops, [0]);
            assert_eq!(pending, 4);
            assert!(draining);
        });
        lock(&drops).sort_unstable();
        assert_eq!(*lock(&drops), (0..=1024).collect::<Vec<_>>());
        assert!(owners.iter().all(|owner| owner.strong_count() == 0));
        assert!(lock(&storage.releases.state).pending.is_empty());
        assert!(!lock(&storage.releases.state).draining);
    }

    #[test]
    fn wide_release_reuses_pending_capacity_without_retaining_payloads() {
        let type_id = external_list_type();
        let storage = RuntimeListStorage::default();
        let store = HostExternalStore::default();
        let drops = Arc::new(Mutex::new(Vec::new()));
        let mut capacities = Vec::new();
        for _ in 0..3 {
            lock(&drops).clear();
            let mut children = Vec::new();
            let mut owners = Vec::new();
            for id in 1..=10_000 {
                let child = storage.external(ExternalListAllocation::new(
                    type_id,
                    vec![external_probe(
                        &store,
                        type_id,
                        ReleaseProbe {
                            id: id.into(),
                            drops: Arc::clone(&drops),
                            on_drop: None,
                        },
                    )],
                ));
                owners.push(Arc::downgrade(&child.lease));
                children.push(child);
            }
            let reentrant_storage = storage.clone();
            let root = storage.external(ExternalListAllocation::new(
                type_id,
                vec![external_probe(
                    &store,
                    type_id,
                    ReleaseProbe {
                        id: 0.into(),
                        drops: Arc::clone(&drops),
                        on_drop: Some(Box::new(move || {
                            drop(children);
                            assert_eq!(
                                lock(&reentrant_storage.releases.state).pending.len(),
                                10_000
                            );
                        })),
                    },
                )],
            ));
            owners.push(Arc::downgrade(&root.lease));
            let alias = root.clone();
            drop(root);
            assert!(lock(&drops).is_empty());
            thread::Builder::new()
                .stack_size(128 * 1024)
                .spawn(move || drop(alias))
                .unwrap()
                .join()
                .unwrap();
            assert_eq!(
                *lock(&drops),
                iter::once(0).chain((1..=10_000).rev()).collect::<Vec<_>>()
            );
            assert!(owners.iter().all(|owner| owner.strong_count() == 0));
            let state = lock(&storage.releases.state);
            assert!(state.pending.is_empty());
            assert!(!state.draining);
            capacities.push(state.pending.capacity());
        }
        assert!(capacities[0] >= 10_000);
        assert_eq!(capacities, vec![capacities[0]; 3]);
        let queue = Arc::downgrade(&storage.releases);
        drop(storage);
        assert_eq!(queue.strong_count(), 0);
    }

    #[test]
    fn non_clone_send_only_payloads_keep_identity_and_drop_at_the_suffix_boundary() {
        let type_id = external_list_type();
        let storage = RuntimeListStorage::default();
        let store = HostExternalStore::default();
        let drops = Arc::new(Mutex::new(Vec::new()));
        let mut values = Vec::new();
        for id in 0..65 {
            values.push(external_probe(
                &store,
                type_id,
                ReleaseProbe {
                    id: id.into(),
                    drops: Arc::clone(&drops),
                    on_drop: None,
                },
            ));
        }
        let value = storage.external(ExternalListAllocation::new(type_id, values));
        let first = value.values().get(0).unwrap();
        let equality =
            |left: &RetainedValueRef, right: &RetainedValueRef| left.value() == right.value();
        let hash = |_: &RetainedValueRef| 0;
        let inspect = |_: &RetainedValueRef| "0".into();
        assert!(first.source_equal(&RetainedValueEquality::new(&equality), first));
        assert!(!first.source_equal(
            &RetainedValueEquality::new(&equality),
            value.values().get(1).unwrap()
        ));
        assert_eq!(first.source_hash(&RetainedValueHashing::new(&hash)), 0);
        assert_eq!(
            first
                .lease()
                .inspection(&RetainedValueInspection::new(&inspect)),
            "Counter(0)"
        );
        let suffix = storage.tail_external(type_id, &value, 1);
        assert!(ptr::eq(
            value.values().get(1).unwrap(),
            suffix.values().get(0).unwrap()
        ));
        let independent = suffix.values().clone();
        drop(value);
        assert_eq!(*lock(&drops), [0]);
        drop(storage);
        drop(suffix);
        assert_eq!(*lock(&drops), [0]);
        thread::spawn(move || drop(independent)).join().unwrap();
        lock(&drops).sort_unstable();
        assert_eq!(*lock(&drops), (0..65).collect::<Vec<_>>());
        drop(store);
    }

    #[test]
    fn transfer_list_graph_is_worker_transferable() {
        assert_send::<RuntimeListStorage>();
        assert_send::<EvaluatedValue>();
        assert_send::<StoredListValueId>();
    }

    #[test]
    fn cloned_and_escaped_lists_keep_the_exact_allocation_alive() {
        let plan = plan_src("pub fn main() -> List(Int) { [1] }");
        let type_id = plan.int_list_function_id(0).type_id();
        let storage = RuntimeListStorage::default();
        let value = storage.int(type_id, vec![1.into(), 2.into()]);
        let retained = value.clone();
        let weak = Arc::downgrade(&value.lease);

        assert_eq!(value, retained);
        assert_eq!(storage.list_len(&ListValueId::Int(value.clone())), 2);
        assert_eq!(weak.strong_count(), 2);
        drop(value);
        assert_eq!(weak.strong_count(), 1);
        drop(storage);
        let reader = RuntimeListStorage::default();
        assert_eq!(
            reader
                .int_values(&retained)
                .iter()
                .cloned()
                .collect::<Vec<_>>(),
            vec![1.into(), 2.into()]
        );
        drop(retained);
        assert_eq!(weak.strong_count(), 0);
    }

    #[test]
    fn releasing_and_allocating_lists_preserves_independent_live_values() {
        let plan = plan_src("pub fn main() -> List(Int) { [1] }");
        let type_id = plan.int_list_function_id(0).type_id();
        let storage = RuntimeListStorage::default();
        let first = storage.int(type_id, vec![1.into()]);
        let live = storage.int(type_id, vec![3.into()]);
        let weak = Arc::downgrade(&first.lease);
        drop(first);
        assert_eq!(weak.strong_count(), 0);
        let second = storage.int(type_id, vec![2.into()]);
        assert_eq!(
            second.values().iter().cloned().collect::<Vec<_>>(),
            vec![2.into()]
        );
        assert_eq!(
            live.values().iter().cloned().collect::<Vec<_>>(),
            vec![3.into()]
        );
        assert_ne!(live, second);
    }

    #[test]
    fn deeply_nested_recursive_custom_lists_release_iteratively() {
        let plan = plan_src(
            "pub type Chain { Link(List(Chain)) } pub fn main() -> List(Chain) { [Link([])] }",
        );
        let type_id = plan.custom_list_function_id(0).type_id();
        let constructor = plan.custom_constructor_id(0, 0);
        let storage = RuntimeListStorage::default();
        let mut value = storage.custom(CustomListAllocation::new(type_id, Vec::new()));
        let leaf = Arc::downgrade(&value.lease);
        let mut owners = Vec::new();
        for _ in 0..50_000 {
            value = storage.custom(CustomListAllocation::new(
                type_id,
                vec![EvaluatedCustomValue::from_fields(
                    constructor,
                    vec![EvaluatedValue::List(value.into())].into_boxed_slice(),
                )],
            ));
            owners.push(Arc::downgrade(&value.lease));
        }
        let queue = Arc::downgrade(&storage.releases);
        thread::Builder::new()
            .stack_size(128 * 1024)
            .spawn(move || drop(value))
            .unwrap()
            .join()
            .unwrap();
        assert_eq!(leaf.strong_count(), 0);
        assert!(owners.iter().all(|owner| owner.strong_count() == 0));
        let state = lock(&storage.releases.state);
        assert!(state.pending.is_empty());
        assert!(!state.draining);
        drop(state);
        drop(storage);
        assert_eq!(queue.strong_count(), 0);
    }

    #[test]
    fn persistent_operations_share_non_clone_items_and_keep_versions_independent() {
        struct Item {
            value: usize,
        }

        for len in [1_000, 10_000] {
            let input: Vec<_> = (0..len).map(|value| Item { value }).collect();
            let original = ListSequence::from(input);
            let alias = original.clone();
            assert!(ptr::eq(
                original.get(0).expect("original item"),
                alias.get(0).expect("shared item")
            ));
            assert_eq!(
                original.iter().map(|item| item.value).collect::<Vec<_>>(),
                (0..len).collect::<Vec<_>>()
            );
            let mut remaining = original.clone();
            for first in 0..len {
                assert_eq!(remaining.get(0).map(|item| item.value), Some(first));
                remaining = remaining.suffix(1);
                assert_eq!(remaining.len(), len - first - 1);
            }
            assert_eq!(remaining.len(), 0);
            assert_eq!(
                alias.iter().map(|item| item.value).collect::<Vec<_>>(),
                (0..len).collect::<Vec<_>>()
            );

            let mut growing = ListSequence::default();
            for value in (0..len).rev() {
                growing = growing.prepend(vec![Item { value }]);
            }
            assert_eq!(
                growing.iter().map(|item| item.value).collect::<Vec<_>>(),
                (0..len).collect::<Vec<_>>()
            );
        }
    }

    #[test]
    fn tail_and_prepend_preserve_order_at_chunk_and_tree_boundaries() {
        let plan = plan_src("pub fn main() -> List(Int) { [] }");
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
        let plan = plan_src(
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
        let plan = plan_src("pub fn main() -> List(Int) { [1, 2, 3] }");
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
        let plan = plan_src(EVERY_LIST_FAMILY_SOURCE);
        let storage = RuntimeListStorage::default();
        let int_function = EvaluatedIntFunction::reference(
            IntFunctionId(0),
            Vec::new(),
            Default::default(),
            FunctionType::new(Vec::new(), ValueType::Int),
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
        let external_store = HostExternalStore::default();
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
        macro_rules! identity {
            ($value:ident, $name:ident) => {{
                let alias = $value.clone();
                assert_eq!($value, alias);
                let stored: StoredListValueId = $value.clone().into();
                assert_ne!(stored, storage.drop_first(&stored, 0));
                let mut relabeled = alias;
                relabeled.type_id.list_type.0 += 1000;
                assert_ne!($value, relabeled);
                assert_eq!(
                    format!("{:?}", $value),
                    format!(
                        concat!(stringify!($name), " {{ type_id: {:?}, lease: {:p} }}"),
                        $value.type_id(),
                        Arc::as_ptr(&$value.lease)
                    ),
                );
            }};
        }
        identity!(int, IntListValueId);
        identity!(string, StringListValueId);
        identity!(bit_array, BitArrayListValueId);
        identity!(utf_codepoint, UtfCodepointListValueId);
        identity!(custom, CustomListValueId);
        identity!(external, ExternalListValueId);
        identity!(float, FloatListValueId);
        identity!(bool_, BoolListValueId);
        identity!(nil, NilListValueId);
        identity!(tuple, TupleListValueId);
        identity!(parameter_list, ParameterListListValueId);
        identity!(list, ListListValueId);
        identity!(function, FunctionListValueId);

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
                slice::from_ref(&expected)
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
    use super::{
        CustomListAllocation, ListLease, ListListTypeId, ListValueId, ParameterListListValueId,
        ParameterListValueId, RuntimeListStorage, StoredListValueId, lock,
    };
    use crate::plan::execution::function::{
        CoreRuntimeFunctionId, IntFunctionId, ListFunctionId, ProfiledListFunctionId,
        RuntimeFunctionId, RuntimeListFunctionId, TupleFunctionId,
    };
    use crate::plan::execution::runtime::RuntimeExecutionPlan;
    use crate::plan::execution::type_::{FunctionType, ListStorageTypeId, ValueType};
    use crate::provider::{self, ProviderValueContext};
    use crate::runtime::error::HostCallOrigin;
    use crate::runtime::function::{run_int_list, run_list, run_tuple};
    use crate::runtime::graph::RetainedValues;
    use crate::runtime::retained_list::RetainedList;
    use crate::runtime::{
        BorrowedValue, EvaluatedBitArray, EvaluatedCustomValue, EvaluatedFunctionValue,
        EvaluatedIntFunction, EvaluatedValue, ExecutionError, Panic, PanicMessage, PanicValue,
        StoredRuntimeValue, plan_src,
    };
    use crate::{
        HostCall, HostCallCompletion, HostCallContinuation, HostCallError, HostCallable,
        HostConstructions, HostFailure, HostFunctionType, HostList, HostListType, HostModule,
        HostOwnedCompletion, HostProfile, HostProvider, HostProviderModule, HostProviderSet,
        HostTypeIndex0, HostTypeList, HostTypeListEnd, HostTypeParameter, HostValue,
        HostedExecution, ModuleSource, PackageSource, PanicKind, Value, compile_typed_host_program,
        execution_fixture, plan_host_program,
    };
    use num_bigint::BigInt;
    use std::sync::{Arc, Weak};
    use std::thread;

    struct LeaseProfile;
    struct LeaseProvider;
    impl HostProfile for LeaseProfile {
        type RunState = Option<Weak<ListLease<BigInt>>>;
        type ExternalStores = ();
        type ExecutionState = ();
    }
    impl HostProvider<LeaseProfile> for LeaseProvider {
        type State = Option<Weak<ListLease<BigInt>>>;
        fn project(state: &mut Self::State) -> &mut Self::State {
            state
        }
    }

    fn return_host_list<'call>(
        mut call: HostCall<'call, LeaseProfile, LeaseProvider, HostListType<BigInt>>,
        constructions: HostConstructions<
            'call,
            HostTypeList<HostListType<BigInt>, HostTypeListEnd>,
        >,
        value: BigInt,
    ) -> Result<HostCallCompletion<'call, HostListType<BigInt>>, HostCallError> {
        if let Some(previous) = call.state().as_ref() {
            assert_eq!(
                previous.strong_count(),
                0,
                "the previous call's list must be released"
            );
        }
        let value = call.construct_list(constructions.at::<HostTypeIndex0>(), [value]);
        let storage = list_lease(&call.retain_value::<HostListType<BigInt>>(value));
        *call.state() = Some(storage);
        Ok(call.return_value(value))
    }

    fn fail_with_host_list<'call>(
        mut call: HostCall<'call, LeaseProfile, LeaseProvider, BigInt>,
        values: HostList<'call, BigInt>,
    ) -> Result<HostCallCompletion<'call, BigInt>, HostCallError> {
        let storage = list_lease(&call.retain_value::<HostListType<BigInt>>(values));
        *call.state() = Some(storage);
        Err(HostFailure::new("stop").into())
    }

    type CallbackValue = HostTypeParameter<0>;
    type CallbackArguments = HostTypeList<CallbackValue, HostTypeListEnd>;
    type Callback = HostFunctionType<CallbackArguments, CallbackValue>;

    fn invoke_generic_callback<'call>(
        mut call: HostCall<'call, LeaseProfile, LeaseProvider, CallbackValue>,
        constructions: HostConstructions<'call, HostTypeListEnd>,
        function: HostCallable<'call, CallbackArguments, CallbackValue>,
        value: HostValue<'call, CallbackValue>,
    ) -> Result<HostCallContinuation<'call, CallbackValue>, HostCallError> {
        type Owned = provider::Value<CallbackValue, ProviderValueContext<CallbackValue>>;
        let storage = list_lease(&call.retain_value::<CallbackValue>(value));
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
                Ok(HostOwnedCompletion::new(move |mut call, _| {
                    let value = value.into_host(&mut call);
                    Ok(call.return_value(value))
                }))
            })
        }))
    }

    fn list_lease(value: &StoredRuntimeValue) -> Weak<ListLease<BigInt>> {
        let value = match value.value() {
            EvaluatedValue::Custom(custom) => custom.fields().first(),
            value => Some(value),
        };
        let Some(EvaluatedValue::List(StoredListValueId::Int(value))) = value else {
            panic!("expected an integer list lease");
        };
        Arc::downgrade(&value.lease)
    }

    #[test]
    #[should_panic(expected = "expected an integer list lease")]
    fn list_lease_rejects_a_scalar_fixture() {
        list_lease(&StoredRuntimeValue::test_int(1.into()));
    }

    fn source_panic(result: Result<(), ExecutionError>) -> Panic<PanicValue> {
        match result {
            Err(ExecutionError::Panic(panic)) => panic,
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
    fn last_owner_releases_leaf_values_without_using_the_release_queue() {
        let plan = plan_src("pub fn main() -> List(Int) { [1] }");
        let type_id = plan.int_list_function_id(0).type_id();
        let storage = RuntimeListStorage::default();
        let value = storage.int(type_id, vec![1.into()]);
        let weak = Arc::downgrade(&value.lease);
        let retained = value.clone();
        let queue = lock(&storage.releases.state);
        for _ in 0..10_000 {
            assert_eq!(value.values().get(0), Some(&BigInt::from(1)));
            assert_eq!(Arc::strong_count(&value.lease), 2);
        }
        drop(value);
        assert_eq!(weak.strong_count(), 1);
        drop(retained);
        assert_eq!(weak.strong_count(), 0);
        assert!(queue.pending.is_empty());
        assert_eq!(queue.pending.capacity(), 0);
        assert!(!queue.draining);
    }

    #[test]
    fn explicitly_owned_read_survives_its_handle_and_state() {
        let plan = plan_src("pub fn main() -> List(Int) { [1] }");
        let type_id = plan.int_list_function_id(0).type_id();
        let storage = RuntimeListStorage::default();
        let value = storage.int(type_id, vec![1.into()]);
        let weak = Arc::downgrade(&value.lease);
        let items = storage.int_values(&value).clone();
        drop(value);
        assert_eq!(weak.strong_count(), 0);
        let replacement = storage.int(type_id, vec![2.into()]);
        drop(storage);
        assert_eq!(items.iter().cloned().collect::<Vec<_>>(), vec![1.into()]);
        assert_eq!(
            replacement.values().iter().cloned().collect::<Vec<_>>(),
            vec![2.into()]
        );
    }

    #[test]
    fn bit_array_list_owners_preserve_type_and_release_independently() {
        let plan = plan_src("pub fn main() -> List(BitArray) { [<<1>>] }");
        let type_id = plan.bit_array_list_function_id(0).type_id();
        let mut echo = Vec::new();
        let mut state = RuntimeState::new(&mut echo);
        let first = state.lists_mut().bit_array(
            type_id,
            vec![EvaluatedBitArray::new(bitvec::vec::BitVec::from_vec(vec![
                1,
            ]))],
        );
        let weak = Arc::downgrade(&first.lease);

        assert_eq!(first.type_id(), type_id);
        assert_eq!(
            state
                .lists()
                .bit_array_values(&first)
                .get(0)
                .map(|value| value.bits().len()),
            Some(8)
        );
        drop(first);

        let second = state.lists_mut().bit_array(type_id, Vec::new());
        assert_eq!(weak.strong_count(), 0);
        assert_eq!(state.lists().bit_array_values(&second).len(), 0);

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
    fn repeated_creation_leaves_no_leaf_owner_or_release_capacity() {
        let plan = plan_src("pub fn main() -> List(Int) { [1] }");
        let type_id = plan.int_list_function_id(0).type_id();
        let storage = RuntimeListStorage::default();
        for value in 0..10_000 {
            let list = storage.int(type_id, vec![value.into()]);
            let weak = Arc::downgrade(&list.lease);
            assert_eq!(weak.strong_count(), 1);
            drop(list);
            assert_eq!(weak.strong_count(), 0);
        }
        assert_eq!(Arc::strong_count(&storage.releases), 1);
        let state = lock(&storage.releases.state);
        assert!(state.pending.is_empty());
        assert_eq!(state.pending.capacity(), 0);
        assert!(!state.draining);
    }

    #[test]
    fn host_calls_release_scoped_list_leases_after_success_and_failure() {
        let returned = HostProviderModule::<LeaseProfile>::new("host_support", "host/lists")
            .expect("host module should be valid")
            .with_scoped_function_and_constructions::<
                LeaseProvider,
                (BigInt,),
                HostListType<BigInt>,
                HostTypeList<HostListType<BigInt>, HostTypeListEnd>,
                _,
            >(
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
        let typed = compile_typed_host_program(
            "application",
            "main",
            [PackageSource::new(
                "application",
                ["host_support"],
                [ModuleSource::new("main", "src/main.gleam", source)],
            ), PackageSource::new("host_support", Vec::<String>::new(), [
                ModuleSource::new("host/lists", "src/host/lists.gleam", "@external(erlang, \"native\", \"wrap\") pub fn wrap(value: Int) -> List(Int)")
            ])],
            HostProviderSet::from_providers([returned]).expect("host module should be unique"),
        )
        .expect("host source should compile");
        let plan = plan_host_program(typed).expect("host source should plan");
        let mut execution =
            HostedExecution::try_from_module_plan(plan).expect("hosted execution should seal");
        let mut storage = None;
        let mut echo = Vec::new();
        let result = execution_fixture::run(&mut execution, &mut storage, &mut echo);
        let lists = storage.expect("native output storage was recorded");
        assert_eq!(result, Ok(Value::Int(BigInt::from(0))),);
        assert_eq!(lists.strong_count(), 0);

        let failed = HostModule::<LeaseProfile>::new_for_profile("host_support", "host/lists")
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
        let typed = compile_typed_host_program(
            "application",
            "main",
            [PackageSource::new(
                "application",
                ["host_support"],
                [ModuleSource::new("main", "src/main.gleam", source)],
            )],
            HostProviderSet::new([failed]).expect("host module should be unique"),
        )
        .expect("host source should compile");
        let plan = plan_host_program(typed).expect("host source should plan");
        let mut execution =
            HostedExecution::try_from_module_plan(plan).expect("hosted execution should seal");
        let mut storage = None;
        let mut echo = Vec::new();
        let error = execution_fixture::run(&mut execution, &mut storage, &mut echo)
            .expect_err("the host callback should fail");
        let lists = storage.expect("native input storage was recorded");
        assert_eq!(
            error.to_string(),
            "host function host_support::host/lists.fail failed: stop",
        );
        assert_eq!(lists.strong_count(), 0);
    }

    #[test]
    fn nested_callbacks_release_retained_custom_list_values_after_success() {
        let host = HostModule::<LeaseProfile>::new_for_profile("host_support", "host/callback")
            .expect("host module should be valid")
            .with_resumable_function::<
                LeaseProvider,
                (Callback, CallbackValue),
                CallbackValue,
                HostTypeListEnd, _,
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
        let typed = compile_typed_host_program(
            "application",
            "main",
            [PackageSource::new(
                "application",
                ["host_support"],
                [ModuleSource::new("main", "src/main.gleam", source)],
            )],
            HostProviderSet::new([host]).expect("host module should be unique"),
        )
        .expect("successful callback source should compile");
        let plan = plan_host_program(typed).expect("successful callback source should plan");
        let mut execution = HostedExecution::try_from_module_plan(plan)
            .expect("successful callback execution should seal");
        let mut storage = None;
        let mut echo = Vec::new();
        let result = execution_fixture::run(&mut execution, &mut storage, &mut echo);
        let lists = storage.expect("callback input storage was recorded");
        assert_eq!(result, Ok(Value::Int(BigInt::from(0))),);
        assert_eq!(lists.strong_count(), 0);
    }

    #[test]
    fn nested_callbacks_release_retained_custom_list_values_after_panic() {
        let host = HostModule::<LeaseProfile>::new_for_profile("host_support", "host/callback")
            .expect("host module should be valid")
            .with_resumable_function::<
                LeaseProvider,
                (Callback, CallbackValue),
                CallbackValue,
                HostTypeListEnd, _,
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
        let typed = compile_typed_host_program(
            "application",
            "main",
            [PackageSource::new(
                "application",
                ["host_support"],
                [ModuleSource::new("main", "src/main.gleam", source)],
            )],
            HostProviderSet::new([host]).expect("host module should be unique"),
        )
        .expect("panicking callback source should compile");
        let plan = plan_host_program(typed).expect("panicking callback source should plan");
        let mut execution = HostedExecution::try_from_module_plan(plan)
            .expect("panicking callback execution should seal");
        let mut storage = None;
        let mut echo = Vec::new();
        let panic =
            source_panic(execution_fixture::run(&mut execution, &mut storage, &mut echo).map(drop));
        let lists = storage.expect("failed callback input storage was recorded");
        assert_eq!(panic.kind(), PanicKind::Panic);
        assert_eq!(panic.site().function(), "stop");
        assert_eq!(lists.strong_count(), 0);
    }

    #[test]
    fn tail_recursive_block_replacement_returns_one_independent_list() {
        let plan = plan_src(
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

        let value = run_int_list(
            &plan,
            &mut state,
            main,
            HostCallOrigin::Entry,
            RetainedValues::empty(),
        )
        .expect("tail-recursive list graph should return");

        assert_eq!(
            state
                .lists()
                .int_values(&value)
                .iter()
                .cloned()
                .collect::<Vec<_>>(),
            vec![1.into()]
        );
        let weak = Arc::downgrade(&value.lease);
        assert_eq!(weak.strong_count(), 1);
        drop(value);
        assert_eq!(weak.strong_count(), 0);
        assert!(lock(&state.lists.releases.state).pending.is_empty());
    }

    #[test]
    fn never_terminator_releases_the_caller_environment_before_running_the_callee() {
        let source = r#"
import host/lists
fn stop() -> value {
  let _ = lists.wrap(2)
  panic as "stop"
}
pub fn main() -> Int {
  let values = lists.wrap(1)
  let _ = values
  stop()
}
"#;
        let mut execution = observed_list_execution(source);
        let mut storage = None;
        let mut echo = Vec::new();
        let panic =
            source_panic(execution_fixture::run(&mut execution, &mut storage, &mut echo).map(drop));
        assert_eq!(panic.kind(), PanicKind::Panic);
        assert_eq!(panic.message(), &PanicMessage::Explicit("stop".into()));
        assert_eq!(storage.unwrap().strong_count(), 0);
    }

    #[test]
    fn match_transition_releases_unretained_subject_before_the_target_runs() {
        let source = r#"
import host/lists
pub fn main() -> Int {
  case lists.wrap(1) {
    [] -> panic as "empty"
    _ -> {
      let _ = lists.wrap(2)
      panic as "non-empty"
    }
  }
}
"#;
        let mut execution = observed_list_execution(source);
        let mut storage = None;
        let mut echo = Vec::new();
        let panic =
            source_panic(execution_fixture::run(&mut execution, &mut storage, &mut echo).map(drop));
        assert_eq!(panic.message(), &PanicMessage::Explicit("non-empty".into()));
        assert_eq!(storage.unwrap().strong_count(), 0);
    }

    #[test]
    fn tail_iterations_release_previous_lists_before_the_next_allocation() {
        let source = r#"
import host/lists
fn done(count, values) {
  case count {
    0 -> values
    _ -> done(count - 1, lists.wrap(count))
  }
}
pub fn main() { done(10000, []) == [1] }
"#;
        let mut execution = observed_list_execution(source);
        let mut storage = None;
        let mut echo = Vec::new();
        assert_eq!(
            execution_fixture::run(&mut execution, &mut storage, &mut echo),
            Ok(Value::Bool(true))
        );
        assert_eq!(storage.unwrap().strong_count(), 0);
    }

    fn observed_list_execution(source: &str) -> HostedExecution<LeaseProfile> {
        let provider = HostProviderModule::<LeaseProfile>::new("host_support", "host/lists")
            .unwrap()
            .with_scoped_function_and_constructions::<
                LeaseProvider,
                (BigInt,),
                HostListType<BigInt>,
                HostTypeList<HostListType<BigInt>, HostTypeListEnd>,
                _,
            >("wrap", return_host_list)
            .unwrap();
        let typed = compile_typed_host_program(
            "application",
            "main",
            [
                PackageSource::new(
                    "application",
                    ["host_support"],
                    [ModuleSource::new("main", "src/main.gleam", source)],
                ),
                PackageSource::new(
                    "host_support",
                    Vec::<String>::new(),
                    [ModuleSource::new(
                        "host/lists",
                        "src/host/lists.gleam",
                        r#"@external(erlang, "native", "wrap") pub fn wrap(value: Int) -> List(Int)"#,
                    )],
                ),
            ],
            HostProviderSet::from_providers([provider]).unwrap(),
        ).unwrap();
        HostedExecution::try_from_module_plan(plan_host_program(typed).unwrap()).unwrap()
    }

    #[test]
    #[should_panic(expected = "expected source panic, got Ok(())")]
    fn source_panic_guard_rejects_success() {
        source_panic(Ok(()));
    }

    #[test]
    fn list_handles_own_live_allocations_after_runtime_state_drop() {
        let plan = plan_src("pub fn main() -> List(Int) { [1] }");
        let type_id = plan.int_list_function_id(0).type_id();
        let storage = RuntimeListStorage::default();
        let value = storage.int(type_id, vec![1.into()]);
        let clone = value.clone();
        let weak = Arc::downgrade(&value.lease);
        let discarded = storage.int(type_id, vec![2.into()]);
        let discarded_weak = Arc::downgrade(&discarded.lease);
        let other_storage = RuntimeListStorage::default();
        let other = other_storage.int(type_id, vec![1.into()]);
        assert_eq!(value, clone);
        assert_ne!(value, other);
        drop(discarded);
        assert_eq!(discarded_weak.strong_count(), 0);
        drop(storage);
        assert_eq!(
            value.values().iter().cloned().collect::<Vec<_>>(),
            vec![1.into()]
        );
        drop(value);
        assert_eq!(
            clone.values().iter().cloned().collect::<Vec<_>>(),
            vec![1.into()]
        );
        drop(clone);
        assert_eq!(weak.strong_count(), 0);
        assert_eq!(other.values().get(0), Some(&BigInt::from(1)));
    }

    #[test]
    fn list_value_facade_reconstructs_every_exact_storage_family() {
        let plan = plan_src(EVERY_LIST_FAMILY_SOURCE);
        let mut echo = Vec::new();
        let mut state = RuntimeState::new(&mut echo);
        let int_function = EvaluatedIntFunction::reference(
            IntFunctionId(0),
            Vec::new(),
            Default::default(),
            FunctionType::new(Vec::new(), ValueType::Int),
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
            ParameterListListValueId::from_stored(&parameter_list.clone().into()),
            Some(parameter_list.clone()),
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
        let plan = plan_src(
            "fn ints() -> List(Int) { [] } pub fn main() -> List(List(Int)) { let _ = ints [[1]] }",
        );
        let parent_type = plan.list_list_function_id(0).type_id();
        let child_type = plan.int_list_function_id(0).type_id();
        assert_eq!(parent_type.item_type(), child_type.list_type());
        let storage = RuntimeListStorage::default();
        let child = storage.int(child_type, vec![1.into()]);
        let child_weak = Arc::downgrade(&child.lease);
        let parent = storage.list(parent_type, vec![child.clone().into()]);
        let parent_weak = Arc::downgrade(&parent.lease);
        drop(parent);
        assert_eq!(parent_weak.strong_count(), 0);
        assert_eq!(child_weak.strong_count(), 1);
        assert_eq!(
            child.values().iter().cloned().collect::<Vec<_>>(),
            vec![1.into()]
        );
        drop(child);
        assert_eq!(child_weak.strong_count(), 0);
        assert!(lock(&storage.releases.state).pending.is_empty());
    }

    #[test]
    fn exclusive_nested_children_are_released_iteratively() {
        let depth = 64;
        let nested_type = "List(".repeat(depth) + "Int" + &")".repeat(depth);
        let source = format!(
            "fn ints() -> List(Int) {{ [] }} pub fn main() -> {nested_type} {{ let _ = ints [] }}"
        );
        let plan = plan_src(&source);
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
        let state = RuntimeState::new(&mut echo);
        let child = state.lists().int(child_type, vec![1.into()]);
        let child_weak = Arc::downgrade(&child.lease);
        let mut value: StoredListValueId = child.into();
        let mut owners = Vec::new();
        for parent in parents.into_iter().rev() {
            let list = state.lists().list(parent, vec![value]);
            owners.push(Arc::downgrade(&list.lease));
            value = list.into();
        }

        thread::Builder::new()
            .stack_size(128 * 1024)
            .spawn(move || drop(value))
            .expect("small-stack release worker")
            .join()
            .expect("nested lists release without recursion");
        assert_eq!(child_weak.strong_count(), 0);
        assert!(owners.iter().all(|owner| owner.strong_count() == 0));
        let state = lock(&state.lists.releases.state);
        assert!(state.pending.is_empty());
        assert!(!state.draining);
    }

    #[test]
    fn retained_owners_preserve_and_iteratively_release_nested_lists() {
        let depth = 64;
        let nested_type = "List(".repeat(depth) + "Int" + &")".repeat(depth);
        let source = format!(
            "fn ints() -> List(Int) {{ [] }} pub fn main() -> {nested_type} {{ let _ = ints [] }}"
        );
        let plan = plan_src(&source);
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
        let state = RuntimeState::new(&mut echo);
        let child = state.lists().int(child_type, vec![1.into()]);
        let child_weak = Arc::downgrade(&child.lease);
        let mut value: StoredListValueId = child.into();
        let mut owners = Vec::new();
        for parent in parents.into_iter().rev() {
            let list = state.lists().list(parent, vec![value]);
            owners.push(Arc::downgrade(&list.lease));
            value = list.into();
        }
        let evaluated = EvaluatedValue::List(value.clone());
        let stored = StoredRuntimeValue::new(evaluated, plan.value_metadata());
        let retained = RetainedList::new(value.clone());

        drop(value);
        assert!(owners.iter().all(|owner| owner.strong_count() > 0));
        assert_eq!(child_weak.strong_count(), 1);
        drop(stored);
        assert!(owners.iter().all(|owner| owner.strong_count() > 0));
        assert_eq!(child_weak.strong_count(), 1);
        assert_eq!(retained.len(), 1);
        drop(retained);
        assert_eq!(child_weak.strong_count(), 0);
        assert!(owners.iter().all(|owner| owner.strong_count() == 0));
        let state = lock(&state.lists.releases.state);
        assert!(state.pending.is_empty());
        assert!(!state.draining);
    }

    #[test]
    fn list_suffix_releases_removed_closure_captures_while_preserving_the_rest() {
        let plan = plan_src(
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
            run_list(
                &plan,
                &mut state,
                ProfiledListFunctionId::Core(ListFunctionId::Function(main)),
                HostCallOrigin::Entry,
                RetainedValues::empty(),
            )
            .unwrap(),
        );
        let owners = state
            .lists()
            .evaluated_values(BorrowedValue::from_value(&original).stored_list())
            .iter()
            .map(captured_int_list)
            .collect::<Option<Vec<_>>>()
            .unwrap();
        assert_eq!(owners.len(), 65);
        assert!(owners.iter().all(|owner| owner.strong_count() == 1));
        let suffix = state
            .lists()
            .drop_first(BorrowedValue::from_value(&original).stored_list(), 1);
        drop(original);
        assert_eq!(owners[0].strong_count(), 0);
        assert!(owners[1..].iter().all(|owner| owner.strong_count() == 1));
        assert_eq!(state.lists().list_len(&suffix.clone().into()), 64);
        drop(suffix);
        assert!(owners.iter().all(|owner| owner.strong_count() == 0));
        let state = lock(&state.lists.releases.state);
        assert!(state.pending.is_empty());
        assert!(!state.draining);
    }

    #[test]
    fn closure_capture_retains_its_list_until_the_closure_is_dropped() {
        let plan = plan_src(
            "fn keep(values: List(Int)) { fn() { values } } pub fn main() { [keep([1])] }",
        );
        let mut echo = Vec::new();
        let mut state = RuntimeState::new(&mut echo);
        let original = run_list(
            &plan,
            &mut state,
            ProfiledListFunctionId::Core(ListFunctionId::Function(
                plan.function_list_function_id(0),
            )),
            HostCallOrigin::Entry,
            RetainedValues::empty(),
        )
        .unwrap();
        let original = EvaluatedValue::from(original);
        let values = state
            .lists()
            .evaluated_values(BorrowedValue::from_value(&original).stored_list());
        assert_eq!(values.len(), 1);
        let closure = values.into_iter().next().unwrap();
        let weak = captured_int_list(&closure).unwrap();
        drop(original);
        drop(state);
        drop(plan);
        assert_eq!(weak.strong_count(), 1);
        drop(closure);
        assert_eq!(weak.strong_count(), 0);
    }

    fn captured_int_list(value: &EvaluatedValue) -> Option<Weak<ListLease<BigInt>>> {
        use crate::runtime::EvaluatedListCapture;
        use crate::runtime::evaluated::{EvaluatedCaptureKind, EvaluatedFunctionValueKind};
        match value {
            EvaluatedValue::Function(function) => {
                match function.kind() {
                    EvaluatedFunctionValueKind::List(function) => function
                        .captures()
                        .iter()
                        .find_map(|capture| match capture.kind() {
                            EvaluatedCaptureKind::List(EvaluatedListCapture::Int {
                                value, ..
                            }) => Some(Arc::downgrade(&value.lease)),
                            _ => None,
                        }),
                    _ => None,
                }
            }
            _ => None,
        }
    }

    #[test]
    fn capture_observation_rejects_non_list_functions_and_non_list_captures() {
        let plan = plan_src(
            "fn keep(value: Int) { fn() { [value] } } pub fn main() { #(1, fn() { 2 }, keep(3)) }",
        );
        let mut echo = Vec::new();
        let mut state = RuntimeState::new(&mut echo);
        let values = run_tuple(
            &plan,
            &mut state,
            TupleFunctionId(0),
            HostCallOrigin::Entry,
            RetainedValues::empty(),
        )
        .unwrap();
        assert_eq!(values.len(), 3);
        assert!(
            values
                .iter()
                .all(|value| captured_int_list(value).is_none())
        );
    }

    fn nested_list_storage(storage: ListStorageTypeId) -> Option<ListListTypeId> {
        match storage {
            ListStorageTypeId::List(type_id) => Some(type_id),
            _ => None,
        }
    }
}
