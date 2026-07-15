use std::cell::RefCell;
use std::fmt;
use std::rc::{Rc, Weak};

use ecow::EcoString;
use num_bigint::BigInt;

use super::evaluated::{
    EvaluatedBitArray, EvaluatedCustomValue, EvaluatedFunctionValue, EvaluatedValue,
};
use crate::plan::execution::{
    BitArrayListTypeId, BoolListTypeId, CustomListItem, CustomListTypeId, ExecutionPlan,
    FloatListTypeId, FunctionListTypeId, IntListTypeId, ListListTypeId, ListStorageTypeId,
    ListTypeId, NilListTypeId, StringListTypeId, TupleListTypeId, UtfCodepointListTypeId,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ListStorageKey {
    Int { slot: usize },
    String { slot: usize },
    BitArray { slot: usize },
    UtfCodepoint { slot: usize },
    Custom { slot: usize },
    Float { slot: usize },
    Bool { slot: usize },
    Nil { slot: usize },
    Tuple { slot: usize },
    List { slot: usize },
    Function { slot: usize },
}

struct ListLease {
    key: ListStorageKey,
    releases: Weak<RefCell<Vec<ListStorageKey>>>,
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
        if let Some(releases) = self.releases.upgrade() {
            releases.borrow_mut().push(self.key);
        }
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
            | Self::Float { slot }
            | Self::Bool { slot }
            | Self::Nil { slot }
            | Self::Tuple { slot }
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
typed_list_value_id!(FloatListValueId, FloatListTypeId, Float);
typed_list_value_id!(BoolListValueId, BoolListTypeId, Bool);
typed_list_value_id!(NilListValueId, NilListTypeId, Nil);
typed_list_value_id!(TupleListValueId, TupleListTypeId, Tuple);
typed_list_value_id!(ListListValueId, ListListTypeId, List);
typed_list_value_id!(FunctionListValueId, FunctionListTypeId, Function);

pub(super) struct CustomListAllocation {
    type_id: CustomListTypeId,
    values: Vec<EvaluatedCustomValue>,
}

impl CustomListAllocation {
    pub(super) fn from_item(item: &CustomListItem, values: Vec<EvaluatedCustomValue>) -> Self {
        Self {
            type_id: item.type_id(),
            values,
        }
    }

    fn from_value(value: &CustomListValueId, values: Vec<EvaluatedCustomValue>) -> Self {
        Self {
            type_id: value.type_id(),
            values,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub(super) enum ListValueId {
    Int(IntListValueId),
    String(StringListValueId),
    BitArray(BitArrayListValueId),
    UtfCodepoint(UtfCodepointListValueId),
    Custom(CustomListValueId),
    Float(FloatListValueId),
    Bool(BoolListValueId),
    Nil(NilListValueId),
    Tuple(TupleListValueId),
    List(ListListValueId),
    Function(FunctionListValueId),
}

impl ListValueId {
    pub(super) fn list_type(&self) -> ListTypeId {
        match self {
            Self::Int(value) => value.type_id().list_type(),
            Self::String(value) => value.type_id().list_type(),
            Self::BitArray(value) => value.type_id().list_type(),
            Self::UtfCodepoint(value) => value.type_id().list_type(),
            Self::Custom(value) => value.type_id().list_type(),
            Self::Float(value) => value.type_id().list_type(),
            Self::Bool(value) => value.type_id().list_type(),
            Self::Nil(value) => value.type_id().list_type(),
            Self::Tuple(value) => value.type_id().list_type(),
            Self::List(value) => value.type_id().list_type(),
            Self::Function(value) => value.type_id().list_type(),
        }
    }

    pub(super) fn into_core(self) -> ListHandleCore {
        match self {
            Self::Int(value) => value.into_core(),
            Self::String(value) => value.into_core(),
            Self::BitArray(value) => value.into_core(),
            Self::UtfCodepoint(value) => value.into_core(),
            Self::Custom(value) => value.into_core(),
            Self::Float(value) => value.into_core(),
            Self::Bool(value) => value.into_core(),
            Self::Nil(value) => value.into_core(),
            Self::Tuple(value) => value.into_core(),
            Self::List(value) => value.into_core(),
            Self::Function(value) => value.into_core(),
        }
    }

    pub(super) fn from_core(
        plan: &ExecutionPlan,
        list_type: ListTypeId,
        core: ListHandleCore,
    ) -> Self {
        match plan.list_storage_type(list_type) {
            ListStorageTypeId::Int(type_id) => Self::Int(IntListValueId::new(type_id, core)),
            ListStorageTypeId::String(type_id) => {
                Self::String(StringListValueId::new(type_id, core))
            }
            ListStorageTypeId::BitArray(type_id) => {
                Self::BitArray(BitArrayListValueId::new(type_id, core))
            }
            ListStorageTypeId::UtfCodepoint(type_id) => {
                Self::UtfCodepoint(UtfCodepointListValueId::new(type_id, core))
            }
            ListStorageTypeId::Custom(type_id) => {
                Self::Custom(CustomListValueId::new(type_id, core))
            }
            ListStorageTypeId::Float(type_id) => Self::Float(FloatListValueId::new(type_id, core)),
            ListStorageTypeId::Bool(type_id) => Self::Bool(BoolListValueId::new(type_id, core)),
            ListStorageTypeId::Nil(type_id) => Self::Nil(NilListValueId::new(type_id, core)),
            ListStorageTypeId::Tuple(type_id) => Self::Tuple(TupleListValueId::new(type_id, core)),
            ListStorageTypeId::List(type_id) => Self::List(ListListValueId::new(type_id, core)),
            ListStorageTypeId::Function(type_id) => {
                Self::Function(FunctionListValueId::new(type_id, core))
            }
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

pub(in crate::runtime) struct RuntimeState {
    releases: Rc<RefCell<Vec<ListStorageKey>>>,
    ints: ListPool<Vec<BigInt>>,
    strings: ListPool<Vec<EcoString>>,
    bit_arrays: ListPool<Vec<EvaluatedBitArray>>,
    utf_codepoints: ListPool<Vec<char>>,
    customs: ListPool<Vec<EvaluatedCustomValue>>,
    floats: ListPool<Vec<f64>>,
    bools: ListPool<Vec<bool>>,
    nils: ListPool<usize>,
    tuples: ListPool<Vec<Vec<EvaluatedValue>>>,
    lists: ListPool<Vec<ListHandleCore>>,
    functions: ListPool<Vec<EvaluatedFunctionValue>>,
}

impl RuntimeState {
    pub(super) fn new() -> Self {
        Self {
            releases: Rc::new(RefCell::new(Vec::new())),
            ints: ListPool::default(),
            strings: ListPool::default(),
            bit_arrays: ListPool::default(),
            utf_codepoints: ListPool::default(),
            customs: ListPool::default(),
            floats: ListPool::default(),
            bools: ListPool::default(),
            nils: ListPool::default(),
            tuples: ListPool::default(),
            lists: ListPool::default(),
            functions: ListPool::default(),
        }
    }

    pub(super) fn drain_releases(&mut self) {
        loop {
            let key = { self.releases.borrow_mut().pop() };
            let Some(key) = key else {
                break;
            };

            match key {
                ListStorageKey::Int { slot } => drop(self.ints.release(slot)),
                ListStorageKey::String { slot } => drop(self.strings.release(slot)),
                ListStorageKey::BitArray { slot } => drop(self.bit_arrays.release(slot)),
                ListStorageKey::UtfCodepoint { slot } => drop(self.utf_codepoints.release(slot)),
                ListStorageKey::Custom { slot } => drop(self.customs.release(slot)),
                ListStorageKey::Float { slot } => drop(self.floats.release(slot)),
                ListStorageKey::Bool { slot } => drop(self.bools.release(slot)),
                ListStorageKey::Nil { slot } => {
                    let _released_len = self.nils.release(slot);
                }
                ListStorageKey::Tuple { slot } => drop(self.tuples.release(slot)),
                ListStorageKey::List { slot } => drop(self.lists.release(slot)),
                ListStorageKey::Function { slot } => drop(self.functions.release(slot)),
            }
        }
    }

    fn core(&self, key: ListStorageKey) -> ListHandleCore {
        ListHandleCore {
            lease: Rc::new(ListLease {
                key,
                releases: Rc::downgrade(&self.releases),
            }),
        }
    }

    fn prepare_allocation(&mut self) {
        self.drain_releases();
    }

    pub(super) fn int(&mut self, type_id: IntListTypeId, values: Vec<BigInt>) -> IntListValueId {
        self.prepare_allocation();
        let slot = self.ints.allocate(values);
        IntListValueId::new(type_id, self.core(ListStorageKey::Int { slot }))
    }

    pub(super) fn string(
        &mut self,
        type_id: StringListTypeId,
        values: Vec<EcoString>,
    ) -> StringListValueId {
        self.prepare_allocation();
        let slot = self.strings.allocate(values);
        StringListValueId::new(type_id, self.core(ListStorageKey::String { slot }))
    }

    pub(super) fn bit_array(
        &mut self,
        type_id: BitArrayListTypeId,
        values: Vec<EvaluatedBitArray>,
    ) -> BitArrayListValueId {
        self.prepare_allocation();
        let slot = self.bit_arrays.allocate(values);
        BitArrayListValueId::new(type_id, self.core(ListStorageKey::BitArray { slot }))
    }

    pub(super) fn utf_codepoint(
        &mut self,
        type_id: UtfCodepointListTypeId,
        values: Vec<char>,
    ) -> UtfCodepointListValueId {
        self.prepare_allocation();
        let slot = self.utf_codepoints.allocate(values);
        UtfCodepointListValueId::new(type_id, self.core(ListStorageKey::UtfCodepoint { slot }))
    }

    pub(super) fn custom(&mut self, allocation: CustomListAllocation) -> CustomListValueId {
        self.prepare_allocation();
        let slot = self.customs.allocate(allocation.values);
        CustomListValueId::new(
            allocation.type_id,
            self.core(ListStorageKey::Custom { slot }),
        )
    }

    pub(super) fn empty_custom(&mut self, type_id: CustomListTypeId) -> CustomListValueId {
        self.prepare_allocation();
        let slot = self.customs.allocate(Vec::new());
        CustomListValueId::new(type_id, self.core(ListStorageKey::Custom { slot }))
    }

    pub(super) fn float(&mut self, type_id: FloatListTypeId, values: Vec<f64>) -> FloatListValueId {
        self.prepare_allocation();
        let slot = self.floats.allocate(values);
        FloatListValueId::new(type_id, self.core(ListStorageKey::Float { slot }))
    }

    pub(super) fn bool(&mut self, type_id: BoolListTypeId, values: Vec<bool>) -> BoolListValueId {
        self.prepare_allocation();
        let slot = self.bools.allocate(values);
        BoolListValueId::new(type_id, self.core(ListStorageKey::Bool { slot }))
    }

    pub(super) fn nil(&mut self, type_id: NilListTypeId, len: usize) -> NilListValueId {
        self.prepare_allocation();
        let slot = self.nils.allocate(len);
        NilListValueId::new(type_id, self.core(ListStorageKey::Nil { slot }))
    }

    pub(super) fn tuple(
        &mut self,
        type_id: TupleListTypeId,
        values: Vec<Vec<EvaluatedValue>>,
    ) -> TupleListValueId {
        self.prepare_allocation();
        let slot = self.tuples.allocate(values);
        TupleListValueId::new(type_id, self.core(ListStorageKey::Tuple { slot }))
    }

    pub(super) fn list(
        &mut self,
        type_id: ListListTypeId,
        values: Vec<ListHandleCore>,
    ) -> ListListValueId {
        self.prepare_allocation();
        let slot = self.lists.allocate(values);
        ListListValueId::new(type_id, self.core(ListStorageKey::List { slot }))
    }

    pub(super) fn function(
        &mut self,
        type_id: FunctionListTypeId,
        values: Vec<EvaluatedFunctionValue>,
    ) -> FunctionListValueId {
        self.prepare_allocation();
        let slot = self.functions.allocate(values);
        FunctionListValueId::new(type_id, self.core(ListStorageKey::Function { slot }))
    }

    pub(super) fn int_values(&self, value: &IntListValueId) -> &[BigInt] {
        self.ints.get(value.core.slot())
    }

    pub(super) fn string_values(&self, value: &StringListValueId) -> &[EcoString] {
        self.strings.get(value.core.slot())
    }

    pub(super) fn bit_array_values(&self, value: &BitArrayListValueId) -> &[EvaluatedBitArray] {
        self.bit_arrays.get(value.core.slot())
    }

    pub(super) fn utf_codepoint_values(&self, value: &UtfCodepointListValueId) -> &[char] {
        self.utf_codepoints.get(value.core.slot())
    }

    pub(super) fn custom_values(&self, value: &CustomListValueId) -> &[EvaluatedCustomValue] {
        self.customs.get(value.core.slot())
    }

    pub(super) fn float_values(&self, value: &FloatListValueId) -> &[f64] {
        self.floats.get(value.core.slot())
    }

    pub(super) fn bool_values(&self, value: &BoolListValueId) -> &[bool] {
        self.bools.get(value.core.slot())
    }

    pub(super) fn nil_len(&self, value: &NilListValueId) -> usize {
        *self.nils.get(value.core.slot())
    }

    pub(super) fn tuple_values(&self, value: &TupleListValueId) -> &[Vec<EvaluatedValue>] {
        self.tuples.get(value.core.slot())
    }

    pub(super) fn list_values(&self, value: &ListListValueId) -> &[ListHandleCore] {
        self.lists.get(value.core.slot())
    }

    pub(super) fn function_values(&self, value: &FunctionListValueId) -> &[EvaluatedFunctionValue] {
        self.functions.get(value.core.slot())
    }

    pub(super) fn list_len(&self, value: &ListValueId) -> usize {
        match value {
            ListValueId::Int(value) => self.int_values(value).len(),
            ListValueId::String(value) => self.string_values(value).len(),
            ListValueId::BitArray(value) => self.bit_array_values(value).len(),
            ListValueId::UtfCodepoint(value) => self.utf_codepoint_values(value).len(),
            ListValueId::Custom(value) => self.custom_values(value).len(),
            ListValueId::Float(value) => self.float_values(value).len(),
            ListValueId::Bool(value) => self.bool_values(value).len(),
            ListValueId::Nil(value) => self.nil_len(value),
            ListValueId::Tuple(value) => self.tuple_values(value).len(),
            ListValueId::List(value) => self.list_values(value).len(),
            ListValueId::Function(value) => self.function_values(value).len(),
        }
    }

    pub(super) fn evaluated_values(
        &self,
        plan: &ExecutionPlan,
        value: &ListValueId,
    ) -> Vec<EvaluatedValue> {
        match value {
            ListValueId::Int(value) => self
                .int_values(value)
                .iter()
                .cloned()
                .map(EvaluatedValue::Int)
                .collect(),
            ListValueId::String(value) => self
                .string_values(value)
                .iter()
                .cloned()
                .map(EvaluatedValue::String)
                .collect(),
            ListValueId::BitArray(value) => self
                .bit_array_values(value)
                .iter()
                .cloned()
                .map(EvaluatedValue::BitArray)
                .collect(),
            ListValueId::UtfCodepoint(value) => self
                .utf_codepoint_values(value)
                .iter()
                .copied()
                .map(EvaluatedValue::UtfCodepoint)
                .collect(),
            ListValueId::Custom(value) => self
                .custom_values(value)
                .iter()
                .cloned()
                .map(EvaluatedValue::Custom)
                .collect(),
            ListValueId::Float(value) => self
                .float_values(value)
                .iter()
                .copied()
                .map(EvaluatedValue::Float)
                .collect(),
            ListValueId::Bool(value) => self
                .bool_values(value)
                .iter()
                .copied()
                .map(EvaluatedValue::Bool)
                .collect(),
            ListValueId::Nil(value) => vec![EvaluatedValue::Nil; self.nil_len(value)],
            ListValueId::Tuple(value) => self
                .tuple_values(value)
                .iter()
                .cloned()
                .map(EvaluatedValue::Tuple)
                .collect(),
            ListValueId::List(value) => self
                .list_values(value)
                .iter()
                .cloned()
                .map(|core| {
                    EvaluatedValue::List(ListValueId::from_core(
                        plan,
                        value.type_id().item_type(),
                        core,
                    ))
                })
                .collect(),
            ListValueId::Function(value) => self
                .function_values(value)
                .iter()
                .cloned()
                .map(EvaluatedValue::Function)
                .collect(),
        }
    }

    pub(super) fn drop_first(&mut self, value: &ListValueId, count: usize) -> ListValueId {
        match value {
            ListValueId::Int(value) => {
                let values = self.int_values(value);
                let values = values[count.min(values.len())..].to_vec();
                self.int(value.type_id(), values).into()
            }
            ListValueId::String(value) => {
                let values = self.string_values(value);
                let values = values[count.min(values.len())..].to_vec();
                self.string(value.type_id(), values).into()
            }
            ListValueId::BitArray(value) => {
                let values = self.bit_array_values(value);
                let values = values[count.min(values.len())..].to_vec();
                self.bit_array(value.type_id(), values).into()
            }
            ListValueId::UtfCodepoint(value) => {
                let values = self.utf_codepoint_values(value);
                let values = values[count.min(values.len())..].to_vec();
                self.utf_codepoint(value.type_id(), values).into()
            }
            ListValueId::Custom(value) => {
                let values = self.custom_values(value);
                let values = values[count.min(values.len())..].to_vec();
                self.custom(CustomListAllocation::from_value(value, values))
                    .into()
            }
            ListValueId::Float(value) => {
                let values = self.float_values(value);
                let values = values[count.min(values.len())..].to_vec();
                self.float(value.type_id(), values).into()
            }
            ListValueId::Bool(value) => {
                let values = self.bool_values(value);
                let values = values[count.min(values.len())..].to_vec();
                self.bool(value.type_id(), values).into()
            }
            ListValueId::Nil(value) => {
                let len = self.nil_len(value).saturating_sub(count);
                self.nil(value.type_id(), len).into()
            }
            ListValueId::Tuple(value) => {
                let values = self.tuple_values(value);
                let values = values[count.min(values.len())..].to_vec();
                self.tuple(value.type_id(), values).into()
            }
            ListValueId::List(value) => {
                let values = self.list_values(value);
                let values = values[count.min(values.len())..].to_vec();
                self.list(value.type_id(), values).into()
            }
            ListValueId::Function(value) => {
                let values = self.function_values(value);
                let values = values[count.min(values.len())..].to_vec();
                self.function(value.type_id(), values).into()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{ListListTypeId, ListStorageTypeId, ListValueId, RuntimeState};
    use crate::plan::execution::{ListFunctionId, RuntimeFunctionId};
    use crate::runtime::frame::Frame;
    use crate::runtime::function::return_body::run_int_list_loop;
    use crate::runtime::{
        EvaluatedBitArray, EvaluatedCapture, EvaluatedFunctionValue, EvaluatedIntFunction,
        EvaluatedValue,
    };

    const EVERY_LIST_FAMILY_SOURCE: &str = r#"
fn ints() -> List(Int) { [] }
fn strings() -> List(String) { [] }
fn bit_arrays() -> List(BitArray) { [] }
fn utf_codepoints() -> List(UtfCodepoint) { [] }
pub type Boxed { Boxed(Int) }
fn customs() -> List(Boxed) { [] }
fn floats() -> List(Float) { [] }
fn bools() -> List(Bool) { [] }
fn nils() -> List(Nil) { [] }
fn tuples() -> List(#(Int)) { [] }
fn lists() -> List(List(Int)) { [] }
fn functions() -> List(fn() -> Int) { [] }
pub fn main() { 0 }
"#;

    #[test]
    fn last_owner_enqueues_release_and_reuses_the_exact_slot() {
        let plan = crate::runtime::plan_src("pub fn main() -> List(Int) { [1] }");
        let type_id = plan.int_list_function_id(0).type_id();
        let mut state = RuntimeState::new();
        let value = state.int(type_id, vec![1.into()]);
        let slot = value.core.slot();
        let retained = value.clone();

        drop(value);
        assert_eq!(state.releases.borrow().as_slice(), &[]);
        drop(retained);
        assert_eq!(state.releases.borrow().len(), 1);

        state.drain_releases();
        assert_eq!(state.ints.free, vec![slot]);
        let reused = state.int(type_id, vec![2.into()]);
        assert_eq!(reused.core.slot(), slot);
        assert_eq!(state.int_values(&reused), &[2.into()]);
    }

    #[test]
    fn bit_array_list_pool_preserves_type_and_reuses_released_slots() {
        let plan = crate::runtime::plan_src("pub fn main() -> List(BitArray) { [<<1>>] }");
        let type_id = plan.bit_array_list_function_id(0).type_id();
        let mut state = RuntimeState::new();
        let first = state.bit_array(
            type_id,
            vec![crate::runtime::EvaluatedBitArray::new(
                bitvec::vec::BitVec::from_vec(vec![1]),
            )],
        );
        let slot = first.core.slot();

        assert_eq!(first.type_id(), type_id);
        assert_eq!(state.bit_array_values(&first)[0].bits().len(), 8);
        drop(first);
        state.drain_releases();

        let second = state.bit_array(type_id, Vec::new());
        assert_eq!(second.core.slot(), slot);
        assert_eq!(state.bit_array_values(&second), &[]);

        let value = ListValueId::BitArray(second.clone());
        assert_eq!(state.list_len(&value), 0);
        let rebuilt = ListValueId::from_core(&plan, type_id.list_type(), value.into_core());
        assert_eq!(rebuilt, ListValueId::BitArray(second.clone()));
        let dropped = state.drop_first(&ListValueId::BitArray(second), 0);
        assert_eq!(state.list_len(&dropped), 0);
    }

    #[test]
    fn repeated_release_and_allocation_keeps_one_slot_high_water_mark() {
        let plan = crate::runtime::plan_src("pub fn main() -> List(Int) { [1] }");
        let type_id = plan.int_list_function_id(0).type_id();
        let mut state = RuntimeState::new();

        for value in 0..10_000 {
            let list = state.int(type_id, vec![value.into()]);
            drop(list);
            state.drain_releases();
        }

        assert_eq!(state.ints.slots.len(), 1);
        assert_eq!(state.ints.free, vec![0]);
        assert_eq!(state.releases.borrow().as_slice(), &[]);
    }

    #[test]
    fn tail_recursive_frame_replacement_reuses_a_fixed_list_slot_set() {
        let plan = crate::runtime::plan_src(include_str!(
            "../../tests/fixtures/execution/functions/tail_call/list_tail_recursion_replaces_allocations.gleam"
        ));
        let main = plan.int_list_function_id(0);
        assert_eq!(
            plan.main_runtime(),
            RuntimeFunctionId::List(ListFunctionId::Int(main)),
        );
        let function = plan.int_list_function(main);
        let mut state = RuntimeState::new();
        let frame = Frame::new(function.frame_layout(), &mut state);

        let value = run_int_list_loop(&plan, &mut state, main, frame)
            .expect("tail-recursive list function should return");

        assert_eq!(state.int_values(&value), &[1.into()]);
        assert_eq!(state.ints.slots.len(), 3);
        assert_eq!(state.ints.free.len(), 2);
        drop(value);
        state.drain_releases();
        assert_eq!(state.ints.free.len(), 3);
        assert_eq!(state.releases.borrow().as_slice(), &[]);
    }

    #[test]
    fn list_handles_compare_by_allocation_identity_and_outlive_the_state_queue() {
        let plan = crate::runtime::plan_src("pub fn main() -> List(Int) { [1] }");
        let type_id = plan.int_list_function_id(0).type_id();
        let mut state = RuntimeState::new();
        let value = state.int(type_id, vec![1.into()]);
        let clone = value.clone();
        let mut other_state = RuntimeState::new();
        let other = other_state.int(type_id, vec![1.into()]);

        assert_eq!(value, clone);
        assert_ne!(value, other);
        assert_eq!(
            format!("{:?}", value.core.lease),
            "ListLease { key: Int { slot: 0 } }"
        );

        drop(state);
        drop(value);
        drop(clone);
        drop(other_state);
        drop(other);
    }

    #[test]
    fn list_value_facade_reconstructs_every_exact_storage_family() {
        let plan = crate::runtime::plan_src(EVERY_LIST_FAMILY_SOURCE);
        let mut state = RuntimeState::new();
        let int_function = EvaluatedIntFunction::new(
            crate::plan::execution::IntFunctionId(0),
            Vec::new(),
            Vec::new(),
            crate::plan::execution::FunctionType::new(
                Vec::new(),
                crate::plan::execution::ValueType::Int,
            ),
        );
        let int = state.int(plan.int_list_function_id(0).type_id(), vec![1.into()]);
        let string = state.string(
            plan.string_list_function_id(0).type_id(),
            vec!["one".into()],
        );
        let bit_array = state.bit_array(
            plan.bit_array_list_function_id(0).type_id(),
            vec![EvaluatedBitArray::new(bitvec::vec::BitVec::from_vec(vec![
                1,
            ]))],
        );
        let utf_codepoint = state.utf_codepoint(
            plan.utf_codepoint_list_function_id(0).type_id(),
            vec!['\u{10ffff}'],
        );
        let custom = state.empty_custom(plan.custom_list_function_id(0).type_id());
        let float = state.float(plan.float_list_function_id(0).type_id(), vec![1.5]);
        let bool_ = state.bool(plan.bool_list_function_id(0).type_id(), vec![true]);
        let nil = state.nil(plan.nil_list_function_id(0).type_id(), 1);
        let tuple = state.tuple(
            plan.tuple_list_function_id(0).type_id(),
            vec![vec![EvaluatedValue::Int(1.into())]],
        );
        let child = state.int(plan.int_list_function_id(0).type_id(), vec![1.into()]);
        let list = state.list(
            plan.list_list_function_id(0).type_id(),
            vec![child.into_core()],
        );
        let function = state.function(
            plan.function_list_function_id(0).type_id(),
            vec![EvaluatedFunctionValue::from(int_function)],
        );
        let values = [
            ListValueId::Int(int),
            ListValueId::String(string),
            ListValueId::BitArray(bit_array),
            ListValueId::UtfCodepoint(utf_codepoint),
            ListValueId::Custom(custom),
            ListValueId::Float(float),
            ListValueId::Bool(bool_),
            ListValueId::Nil(nil),
            ListValueId::Tuple(tuple),
            ListValueId::List(list),
            ListValueId::Function(function),
        ];

        for value in values {
            assert_eq!(
                ListValueId::from_core(&plan, value.list_type(), value.clone().into_core()),
                value,
            );
        }
    }

    #[test]
    fn parent_release_preserves_a_separately_owned_child() {
        let plan = crate::runtime::plan_src(
            "fn ints() -> List(Int) { [] } pub fn main() -> List(List(Int)) { [[1]] }",
        );
        let parent_type = plan.list_list_function_id(0).type_id();
        let child_type = plan.int_list_function_id(0).type_id();
        assert_eq!(parent_type.item_type(), child_type.list_type());
        let mut state = RuntimeState::new();
        let child = state.int(child_type, vec![1.into()]);
        let child_slot = child.core.slot();
        let parent = state.list(parent_type, vec![child.clone().into_core()]);

        drop(parent);
        state.drain_releases();
        assert_eq!(state.lists.free.len(), 1);
        assert_eq!(state.ints.free, Vec::<usize>::new());
        assert_eq!(state.int_values(&child), &[1.into()]);

        drop(child);
        state.drain_releases();
        assert_eq!(state.ints.free, vec![child_slot]);
    }

    #[test]
    fn exclusive_nested_children_are_released_iteratively() {
        let depth = 64;
        let nested_type = "List(".repeat(depth) + "Int" + &")".repeat(depth);
        let source =
            format!("fn ints() -> List(Int) {{ [] }} pub fn main() -> {nested_type} {{ [] }}");
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

        let mut state = RuntimeState::new();
        let mut value: ListValueId = state.int(child_type, vec![1.into()]).into();
        for parent in parents.into_iter().rev() {
            value = state.list(parent, vec![value.into_core()]).into();
        }
        let allocated_list_slots = state.lists.slots.len();

        drop(value);
        state.drain_releases();
        assert_eq!(state.ints.free.len(), 1);
        assert_eq!(state.lists.free.len(), allocated_list_slots);
        assert_eq!(state.releases.borrow().as_slice(), &[]);
    }

    #[test]
    fn closure_capture_retains_its_list_until_the_closure_is_dropped() {
        let plan = crate::runtime::plan_src(
            "fn keep(values: List(Int)) { fn() { values } } pub fn main() { keep([1]) }",
        );
        let type_id = plan.int_list_function_id(0).type_id();
        let mut state = RuntimeState::new();
        let value = state.int(type_id, vec![1.into()]);
        let slot = value.core.slot();
        let closure = EvaluatedIntFunction::new(
            crate::plan::execution::IntFunctionId(0),
            Vec::new(),
            vec![EvaluatedCapture::list(
                crate::runtime::EvaluatedListCapture::Int {
                    local: crate::plan::execution::IntListLocalId(0),
                    value: value.clone(),
                },
            )],
            crate::plan::execution::FunctionType::new(
                Vec::new(),
                crate::plan::execution::ValueType::Int,
            ),
        );

        drop(value);
        state.drain_releases();
        assert_eq!(state.ints.free, Vec::<usize>::new());
        drop(closure);
        state.drain_releases();
        assert_eq!(state.ints.free, vec![slot]);
    }

    fn nested_list_storage(storage: ListStorageTypeId) -> Option<ListListTypeId> {
        match storage {
            ListStorageTypeId::List(type_id) => Some(type_id),
            _ => None,
        }
    }
}
