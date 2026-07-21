use std::cell::RefCell;
use std::fmt;
use std::rc::{Rc, Weak};

use ecow::EcoString;
use num_bigint::BigInt;

use super::evaluated::{
    EvaluatedBitArray, EvaluatedCustomValue, EvaluatedFunctionValue, EvaluatedValue,
};
use crate::plan::execution::{
    BitArrayListTypeId, BoolListTypeId, CustomListTypeId, FloatListTypeId, FunctionListTypeId,
    IntListTypeId, ListListTypeId, ListTypeId, NilListTypeId, ParameterListListTypeId,
    ParameterListTypeId, StringListTypeId, TupleListTypeId, UtfCodepointListTypeId,
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
    ParameterList { slot: usize },
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

#[derive(Debug, Clone, PartialEq)]
pub(super) enum ListValueId {
    Parameter(ParameterListValueId),
    Int(IntListValueId),
    String(StringListValueId),
    BitArray(BitArrayListValueId),
    UtfCodepoint(UtfCodepointListValueId),
    Custom(CustomListValueId),
    Float(FloatListValueId),
    Bool(BoolListValueId),
    Nil(NilListValueId),
    Tuple(TupleListValueId),
    ParameterList(ParameterListListValueId),
    List(ListListValueId),
    Function(FunctionListValueId),
}

impl ListValueId {
    pub(super) fn list_type(&self) -> ListTypeId {
        match self {
            Self::Parameter(value) => value.type_id().list_type(),
            Self::Int(value) => value.type_id().list_type(),
            Self::String(value) => value.type_id().list_type(),
            Self::BitArray(value) => value.type_id().list_type(),
            Self::UtfCodepoint(value) => value.type_id().list_type(),
            Self::Custom(value) => value.type_id().list_type(),
            Self::Float(value) => value.type_id().list_type(),
            Self::Bool(value) => value.type_id().list_type(),
            Self::Nil(value) => value.type_id().list_type(),
            Self::Tuple(value) => value.type_id().list_type(),
            Self::ParameterList(value) => value.type_id().list_type(),
            Self::List(value) => value.type_id().list_type(),
            Self::Function(value) => value.type_id().list_type(),
        }
    }
}

impl StoredListValueId {
    pub(super) fn into_value(self) -> ListValueId {
        match self {
            Self::Int(value) => ListValueId::Int(value),
            Self::String(value) => ListValueId::String(value),
            Self::BitArray(value) => ListValueId::BitArray(value),
            Self::UtfCodepoint(value) => ListValueId::UtfCodepoint(value),
            Self::Custom(value) => ListValueId::Custom(value),
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
    parameter_list_lists: ListPool<usize>,
    lists: ListPool<Vec<StoredListValueId>>,
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
            parameter_list_lists: ListPool::default(),
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
                ListStorageKey::ParameterList { slot } => {
                    let _released_len = self.parameter_list_lists.release(slot);
                }
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

    pub(super) fn parameter_list_list(
        &mut self,
        type_id: ParameterListListTypeId,
        len: usize,
    ) -> ParameterListListValueId {
        self.prepare_allocation();
        let slot = self.parameter_list_lists.allocate(len);
        ParameterListListValueId::new(type_id, self.core(ListStorageKey::ParameterList { slot }))
    }

    pub(super) fn list(
        &mut self,
        type_id: ListListTypeId,
        values: Vec<StoredListValueId>,
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

    pub(super) fn parameter_list_list_len(&self, value: &ParameterListListValueId) -> usize {
        *self.parameter_list_lists.get(value.core.slot())
    }

    pub(super) fn list_values(&self, value: &ListListValueId) -> &[StoredListValueId] {
        self.lists.get(value.core.slot())
    }

    pub(super) fn function_values(&self, value: &FunctionListValueId) -> &[EvaluatedFunctionValue] {
        self.functions.get(value.core.slot())
    }

    pub(super) fn list_len(&self, value: &ListValueId) -> usize {
        match value {
            ListValueId::Parameter(_) => 0,
            ListValueId::Int(value) => self.int_values(value).len(),
            ListValueId::String(value) => self.string_values(value).len(),
            ListValueId::BitArray(value) => self.bit_array_values(value).len(),
            ListValueId::UtfCodepoint(value) => self.utf_codepoint_values(value).len(),
            ListValueId::Custom(value) => self.custom_values(value).len(),
            ListValueId::Float(value) => self.float_values(value).len(),
            ListValueId::Bool(value) => self.bool_values(value).len(),
            ListValueId::Nil(value) => self.nil_len(value),
            ListValueId::Tuple(value) => self.tuple_values(value).len(),
            ListValueId::ParameterList(value) => self.parameter_list_list_len(value),
            ListValueId::List(value) => self.list_values(value).len(),
            ListValueId::Function(value) => self.function_values(value).len(),
        }
    }

    pub(super) fn evaluated_values(&self, value: &ListValueId) -> Vec<EvaluatedValue> {
        match value {
            ListValueId::Parameter(_) => Vec::new(),
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
            ListValueId::ParameterList(value) => {
                vec![
                    EvaluatedValue::List(ListValueId::Parameter(ParameterListValueId::new(
                        value.type_id().item_type(),
                    )));
                    self.parameter_list_list_len(value)
                ]
            }
            ListValueId::List(value) => self
                .list_values(value)
                .iter()
                .cloned()
                .map(StoredListValueId::into_value)
                .map(EvaluatedValue::List)
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
            ListValueId::Parameter(value) => ListValueId::Parameter(*value),
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
            ListValueId::ParameterList(value) => {
                let len = self.parameter_list_list_len(value).saturating_sub(count);
                self.parameter_list_list(value.type_id(), len).into()
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
    use super::{
        CustomListAllocation, ListListTypeId, ListValueId, ParameterListValueId, RuntimeState,
        StoredListValueId,
    };
    use crate::plan::execution::{ListFunctionId, ListStorageTypeId, RuntimeFunctionId};
    use crate::runtime::environment::RetainedValues;
    use crate::runtime::{
        EvaluatedBitArray, EvaluatedCapture, EvaluatedCustomValue, EvaluatedFunctionValue,
        EvaluatedIntFunction, EvaluatedValue,
    };

    fn int_main(plan: &crate::ExecutionPlan) -> crate::plan::execution::IntFunctionId {
        match plan.main_runtime() {
            RuntimeFunctionId::Int(main) => main,
            _ => panic!("main should lower into the Int function table"),
        }
    }

    fn source_panic(
        result: crate::runtime::error::ExecutionResult<num_bigint::BigInt>,
    ) -> crate::runtime::Panic {
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
        assert_eq!(
            StoredListValueId::from(second.clone()).into_value(),
            ListValueId::BitArray(second.clone()),
        );
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
    fn tail_recursive_block_replacement_reuses_a_fixed_list_slot_set() {
        let plan = crate::runtime::plan_src(include_str!(
            "../../tests/fixtures/execution/functions/tail_call/list_tail_recursion_replaces_allocations.gleam"
        ));
        let main = plan.int_list_function_id(0);
        assert_eq!(
            plan.main_runtime(),
            RuntimeFunctionId::List(ListFunctionId::Int(main)),
        );
        let mut state = RuntimeState::new();

        let value =
            crate::runtime::graph::run_int_list(&plan, &mut state, main, RetainedValues::empty())
                .expect("tail-recursive list graph should return");

        assert_eq!(state.int_values(&value), &[1.into()]);
        assert_eq!(state.ints.slots.len(), 1);
        assert_eq!(state.ints.free.len(), 0);
        drop(value);
        state.drain_releases();
        assert_eq!(state.ints.free.len(), 1);
        assert_eq!(state.releases.borrow().as_slice(), &[]);
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
        let mut state = RuntimeState::new();

        let panic = source_panic(crate::runtime::graph::run_int(
            &plan,
            &mut state,
            main,
            RetainedValues::empty(),
        ));

        assert_eq!(panic.kind(), crate::runtime::PanicKind::Panic);
        assert_eq!(
            panic.message(),
            &crate::runtime::PanicMessage::Explicit("stop".into()),
        );
        assert_eq!(state.ints.slots.len(), 1);
        assert_eq!(state.ints.free, vec![0]);
        assert_eq!(state.releases.borrow().as_slice(), &[]);
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
        let mut state = RuntimeState::new();

        let panic = source_panic(crate::runtime::graph::run_int(
            &plan,
            &mut state,
            main,
            RetainedValues::empty(),
        ));

        assert_eq!(
            panic.message(),
            &crate::runtime::PanicMessage::Explicit("non-empty".into()),
        );
        assert_eq!(state.ints.slots.len(), 1);
        assert_eq!(state.ints.free, vec![0]);
        assert_eq!(state.releases.borrow().as_slice(), &[]);
    }

    #[test]
    #[should_panic(expected = "main should lower into the Int function table")]
    fn int_main_guard_rejects_other_function_tables() {
        int_main(&crate::runtime::plan_src(
            "pub fn main() -> List(Int) { [] }",
        ));
    }

    #[test]
    #[should_panic(expected = "expected source panic, got Ok(0)")]
    fn source_panic_guard_rejects_success() {
        source_panic(Ok(0.into()));
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
        let int_function = EvaluatedIntFunction::reference(
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
        let custom_constructor = plan.custom_constructor_id(0, 0);
        let custom = state.custom(CustomListAllocation::new(
            plan.custom_list_function_id(0).type_id(),
            vec![EvaluatedCustomValue::from_fields(
                custom_constructor,
                vec![EvaluatedValue::Int(1.into())].into_boxed_slice(),
            )],
        ));
        let float = state.float(plan.float_list_function_id(0).type_id(), vec![1.5]);
        let bool_ = state.bool(plan.bool_list_function_id(0).type_id(), vec![true]);
        let nil = state.nil(plan.nil_list_function_id(0).type_id(), 1);
        let tuple = state.tuple(
            plan.tuple_list_function_id(0).type_id(),
            vec![vec![EvaluatedValue::Int(1.into())]],
        );
        let parameter = ParameterListValueId::new(plan.parameter_list_function_id(0).type_id());
        let parameter_list =
            state.parameter_list_list(plan.parameter_list_list_function_id(0).type_id(), 1);
        let child = state.int(plan.int_list_function_id(0).type_id(), vec![1.into()]);
        let list = state.list(plan.list_list_function_id(0).type_id(), vec![child.into()]);
        let function = state.function(
            plan.function_list_function_id(0).type_id(),
            vec![EvaluatedFunctionValue::from(int_function)],
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

        let stored_lists = [
            ListValueId::Int(int),
            ListValueId::String(string),
            ListValueId::BitArray(bit_array),
            ListValueId::UtfCodepoint(utf_codepoint),
            ListValueId::Custom(custom),
            ListValueId::Float(float),
            ListValueId::Bool(bool_),
            ListValueId::Nil(nil),
            ListValueId::Tuple(tuple),
            ListValueId::ParameterList(parameter_list.clone()),
            ListValueId::List(list),
            ListValueId::Function(function),
        ];
        for value in stored_lists {
            let list_type = value.list_type();
            assert_eq!(state.list_len(&value), 1);

            let dropped = state.drop_first(&value, 1);
            assert_eq!(dropped.list_type(), list_type);
            assert_eq!(state.list_len(&dropped), 0);
        }

        assert_eq!(
            StoredListValueId::from(parameter_list.clone()).into_core(),
            parameter_list.clone().into_core(),
        );
        assert_eq!(state.list_len(&ListValueId::Parameter(parameter)), 0);
        assert_eq!(
            state.drop_first(&ListValueId::Parameter(parameter), usize::MAX),
            ListValueId::Parameter(parameter),
        );
        assert_eq!(
            state.list_len(&ListValueId::ParameterList(parameter_list.clone())),
            1
        );
        let dropped = state.drop_first(&ListValueId::ParameterList(parameter_list), usize::MAX);
        assert_eq!(state.list_len(&dropped), 0);
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
        let mut state = RuntimeState::new();
        let child = state.int(child_type, vec![1.into()]);
        let child_slot = child.core.slot();
        let parent = state.list(parent_type, vec![child.clone().into()]);

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

        let mut state = RuntimeState::new();
        let mut value: StoredListValueId = state.int(child_type, vec![1.into()]).into();
        for parent in parents.into_iter().rev() {
            value = state.list(parent, vec![value]).into();
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
        let closure = EvaluatedIntFunction::reference(
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
