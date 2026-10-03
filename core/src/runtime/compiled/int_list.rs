use super::CompiledProgress;
use crate::plan::execution::graph::IntegerLiteral;
use crate::plan::execution::type_::IntListTypeId;
use crate::runtime::borrowed::IntegerReadCell;
use crate::runtime::integer::IntegerValue;
use crate::runtime::state::list::{IntListValueId, ListSequenceIter, RuntimeListStorage};

pub type IntListKernel =
    fn(usize, &mut IntListValues, &IntListOps<'_>, &mut usize) -> CompiledProgress;

/// Only the current block's parameters and completed instruction outputs.
/// List handles move between this scratch and the existing graph environment.
#[derive(Default)]
pub struct IntListValues {
    pub ints: Vec<i128>,
    pub bools: Vec<bool>,
    pub int_lists: Vec<IntList>,
}

/// An opaque existing typed lease, preserving element and read-cache identity.
#[derive(Clone)]
pub struct IntList(pub(in crate::runtime) IntListValueId);

/// Borrowed access to the execution's existing list owner. This adapter never
/// retains a root or reimplements allocation, suffix, or release policy.
pub struct IntListOps<'runtime> {
    storage: &'runtime RuntimeListStorage,
}

pub struct IntListReader<'list> {
    values: ListSequenceIter<'list, IntegerValue, IntegerReadCell>,
}

pub struct IntListElement<'list> {
    value: &'list IntegerValue,
}

impl IntList {
    pub fn len(&self) -> usize {
        self.0.values().len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl<'runtime> IntListOps<'runtime> {
    pub(in crate::runtime) fn new(storage: &'runtime RuntimeListStorage) -> Self {
        Self { storage }
    }

    /// None returns before the canonical instruction runs, including an
    /// invalid index whose original graph owner supplies the diagnostic.
    pub fn index(&self, value: &IntList, index: usize) -> Option<i128> {
        self.storage
            .int_values(&value.0)
            .get(index)
            .and_then(IntegerValue::small)
            .map(i128::from)
    }

    pub fn prefix<'list>(&self, value: &'list IntList, count: usize) -> IntListReader<'list> {
        IntListReader {
            values: self.storage.int_values(&value.0).iter_prefix(count),
        }
    }

    pub fn equal(&self, left: &IntList, right: &IntList) -> bool {
        left.0.type_id() == right.0.type_id()
            && left.len() == right.len()
            && self
                .storage
                .int_values(&left.0)
                .iter()
                .eq(self.storage.int_values(&right.0).iter())
    }

    pub fn tail(&self, value: &IntList, type_id: IntListTypeId, count: usize) -> IntList {
        IntList(self.storage.tail_int(type_id, &value.0, count))
    }
}

impl<'list> Iterator for IntListReader<'list> {
    type Item = IntListElement<'list>;

    fn next(&mut self) -> Option<Self::Item> {
        self.values.next().map(|value| IntListElement { value })
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.values.size_hint()
    }
}

impl IntListElement<'_> {
    pub fn small(&self) -> Option<i128> {
        self.value.small().map(i128::from)
    }

    pub fn matches_literal(&self, literal: &IntegerLiteral) -> bool {
        self.value.matches_literal(literal)
    }
}

#[cfg(test)]
mod tests {
    use super::{IntList, IntListOps};
    use crate::plan::execution::graph::IntegerLiteral;
    use crate::plan::execution::type_::{IntListTypeId, ListTypeId};
    use crate::runtime::state::list::RuntimeListStorage;
    use num_bigint::BigInt;
    use std::ptr;

    #[test]
    fn typed_reads_keep_big_values_prefix_order_and_shared_tail_cache_identity() {
        let storage = RuntimeListStorage::default();
        let ops = IntListOps::new(&storage);
        let plan = crate::runtime::plan_src("pub fn main() { [1] }");
        let type_id = plan.int_list_function_id(0).type_id();
        let big: BigInt = BigInt::from(1) << 180;
        let value = IntList(storage.int(
            type_id,
            vec![1_i64.into(), big.clone().into(), 3_i64.into()],
        ));
        assert_eq!(value.len(), 3);
        assert!(!value.is_empty());
        assert_eq!(ops.index(&value, 0), Some(1));
        assert_eq!(ops.index(&value, 1), None);
        assert_eq!(ops.index(&value, 3), None);
        let mut prefix = ops.prefix(&value, 2);
        assert_eq!(prefix.size_hint(), (2, Some(2)));
        let first = prefix.next().unwrap();
        assert_eq!(first.small(), Some(1));
        assert!(first.matches_literal(&IntegerLiteral::from(BigInt::from(1))));
        assert!(!first.matches_literal(&IntegerLiteral::from(BigInt::from(2))));
        let second = prefix.next().unwrap();
        assert_eq!(second.small(), None);
        assert!(second.matches_literal(&IntegerLiteral::from(big.clone())));
        assert_eq!(prefix.size_hint(), (0, Some(0)));
        assert!(prefix.next().is_none());
        let (element, cell) = value.0.values().get_with_cache(2).unwrap();
        let cached = cell.get_or_init(|| Box::new(BigInt::from(3)));
        let tail = ops.tail(&value, type_id, 2);
        let (retained, retained_cell) = tail.0.values().get_with_cache(0).unwrap();
        assert!(ptr::eq(element, retained));
        assert!(ptr::eq(cell, retained_cell));
        assert!(ptr::eq(
            cached.as_ref(),
            retained_cell.get().unwrap().as_ref()
        ));
        let same = IntList(storage.int(type_id, vec![1_i64.into(), big.into(), 3_i64.into()]));
        assert!(ops.equal(&value, &same));
        assert!(!ops.equal(&value, &tail));
        let changed = IntList(storage.int(type_id, vec![1_i64.into(), 2_i64.into(), 3_i64.into()]));
        assert!(!ops.equal(&value, &changed));
        let empty = ops.tail(&value, type_id, 100);
        assert!(empty.is_empty());
        let other_type = IntList(storage.int(
            IntListTypeId {
                list_type: ListTypeId(1),
            },
            vec![],
        ));
        assert!(!ops.equal(&empty, &other_type));
        assert!(ops.equal(&empty, &empty.clone()));
    }
}
