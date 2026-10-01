use crate::runtime::integer::IntegerValue;
use num_bigint::BigInt;
use parking_lot::Mutex;
use std::borrow::Cow;
use std::collections::HashMap;
use std::sync::{Arc, OnceLock};

pub(in crate::runtime) type SharedIntegerReads = OnceLock<Arc<IntegerReadCache>>;
pub(in crate::runtime) type IntegerReadCell = OnceLock<Box<BigInt>>;

pub(in crate::runtime) fn read_cell<'value>(
    value: &'value IntegerValue,
    cell: &'value IntegerReadCell,
) -> IntegerRead<'value> {
    match value.small() {
        Some(number) => IntegerRead::Direct(Cow::Borrowed(
            cell.get_or_init(|| Box::new(number.into())).as_ref(),
        )),
        None => IntegerRead::Direct(value.bigint()),
    }
}

/// An immutable public result retains conversions only for scalar payloads a
/// caller actually reads. Internal integers and unread list items have no cache.
#[derive(Default)]
pub(in crate::runtime) struct IntegerReadCache {
    values: Mutex<HashMap<usize, Arc<BigInt>>>,
}

pub(crate) enum IntegerRead<'value> {
    Direct(Cow<'value, BigInt>),
    Cached(Arc<BigInt>),
}

impl IntegerRead<'_> {
    pub(crate) fn as_ref(&self) -> &BigInt {
        match self {
            Self::Direct(value) => value.as_ref(),
            Self::Cached(value) => value.as_ref(),
        }
    }
}

pub(in crate::runtime) fn shared(reads: &SharedIntegerReads) -> &Arc<IntegerReadCache> {
    reads.get_or_init(|| Arc::new(IntegerReadCache::default()))
}

pub(in crate::runtime) fn read<'value>(
    value: &'value IntegerValue,
    reads: Option<&SharedIntegerReads>,
) -> IntegerRead<'value> {
    match (value.small(), reads) {
        (Some(number), Some(reads)) => {
            // Source addresses are stable within this retained, immutable result.
            // Cloned lazy views retain the same cache and source allocations.
            let key = std::ptr::from_ref(value).addr();
            let mut values = shared(reads).values.lock();
            let value = Arc::clone(values.entry(key).or_insert_with(|| Arc::new(number.into())));
            // No user callback or destructor runs while the map is locked.
            IntegerRead::Cached(value)
        }
        _ => IntegerRead::Direct(value.bigint()),
    }
}

#[cfg(test)]
mod tests {
    use super::{IntegerRead, read};
    use crate::runtime::{BorrowedValue, EvaluatedValue, StoredRuntimeValue};
    use num_bigint::BigInt;
    use std::ptr;
    use std::sync::Arc;

    fn cached(read: IntegerRead<'_>) -> Option<Arc<BigInt>> {
        match read {
            IntegerRead::Cached(value) => Some(value),
            IntegerRead::Direct(_) => None,
        }
    }

    #[test]
    fn cached_public_reads_preserve_alias_identity_and_distinct_scalar_storage() {
        use crate::plan::execution::runtime::RuntimeExecutionPlan;
        let plan = crate::runtime::plan_src("pub fn main() { Nil }");
        let stored = StoredRuntimeValue::new(
            EvaluatedValue::Tuple(vec![
                EvaluatedValue::Int(42.into()),
                EvaluatedValue::Int(42.into()),
                EvaluatedValue::String("unread integer views".into()),
            ]),
            plan.value_metadata(),
        );
        let alias = stored.clone_retained();
        assert_eq!(
            BorrowedValue::from_stored(&stored)
                .tuple_item(2)
                .string()
                .as_ref(),
            "unread integer views"
        );
        assert!(stored.integer_reads().get().is_none());
        let first = BorrowedValue::from_stored(&stored)
            .tuple_item(0)
            .int_bigint();
        let again = BorrowedValue::from_stored(&alias)
            .tuple_item(0)
            .int_bigint();
        let distinct = BorrowedValue::from_stored(&stored)
            .tuple_item(1)
            .int_bigint();
        assert_eq!(first.as_ref(), &BigInt::from(42));
        assert!(ptr::eq(first.as_ref(), again.as_ref()));
        assert!(!ptr::eq(first.as_ref(), distinct.as_ref()));
        let cache = stored.integer_reads().get().unwrap();
        assert_eq!(cache.values.lock().len(), 2);
        let first = cached(first).unwrap();
        drop(again);
        drop(distinct);
        drop(alias);
        drop(stored);
        assert_eq!(first.as_ref(), &BigInt::from(42));
    }

    #[test]
    fn large_views_borrow_the_original_payload_and_uncached_small_views_own_conversion() {
        let large = StoredRuntimeValue::test_int(BigInt::from(1_u8) << 256);
        let row = BorrowedValue::from_stored(&large);
        let value = row.int_bigint();
        let again = row.int_bigint();
        assert!(ptr::eq(value.as_ref(), again.as_ref()));
        assert_eq!(cached(value), None);
        assert!(large.integer_reads().get().is_none());
        let small = 7.into();
        assert_eq!(read(&small, None).as_ref(), &BigInt::from(7));
    }

    #[test]
    fn large_list_items_borrow_original_numbers_without_initializing_read_cells() {
        use crate::plan::execution::runtime::RuntimeExecutionPlan;
        use crate::runtime::{EmbeddingList, RuntimeListStorage};
        let plan = crate::runtime::plan_src("pub fn main() { [1] }");
        let storage = RuntimeListStorage::default();
        let huge: BigInt = BigInt::from(1_u8) << 256;
        let values = storage.int(
            plan.int_list_function_id(0).type_id(),
            vec![huge.clone().into()],
        );
        let stored = StoredRuntimeValue::new(
            EvaluatedValue::List(values.clone().into()),
            plan.value_metadata(),
        );
        let list = EmbeddingList::from_borrowed(&BorrowedValue::from_stored(&stored));
        list.read_item(0, |row| {
            let first = row.int_bigint();
            let second = row.int_bigint();
            assert_eq!(first.as_ref(), &huge);
            assert!(ptr::eq(first.as_ref(), second.as_ref()));
            assert_eq!(cached(first), None);
        })
        .unwrap();
        assert!(values.values().get_with_cache(0).unwrap().1.get().is_none());
    }

    #[test]
    fn custom_aliases_keep_integer_views_in_the_shared_field_owner() {
        use crate::plan::execution::type_::{CustomConstructorId, CustomTypeId};
        use crate::runtime::EvaluatedCustomValue;
        let value = EvaluatedValue::Custom(EvaluatedCustomValue::from_fields(
            CustomConstructorId {
                type_id: CustomTypeId(0),
                index: 0,
            },
            vec![EvaluatedValue::Int(42.into())].into_boxed_slice(),
        ));
        let alias = value.clone();
        let first = BorrowedValue::from_value(&value)
            .custom_field(0)
            .int_bigint();
        let second = BorrowedValue::from_value(&alias)
            .custom_field(0)
            .int_bigint();
        assert!(ptr::eq(first.as_ref(), second.as_ref()));
        assert_eq!(first.as_ref(), &BigInt::from(42));
    }

    #[test]
    fn tuple_list_item_views_preserve_identity_across_suffixes_and_new_result_roots() {
        use crate::plan::execution::runtime::RuntimeExecutionPlan;
        use crate::runtime::{EmbeddingList, RuntimeListStorage};
        let plan = crate::runtime::plan_src("pub fn main() { [#(1, 2), #(3, 4)] }");
        let storage = RuntimeListStorage::default();
        let type_id = plan.tuple_list_function_id(0).type_id();
        let values = storage.tuple(
            type_id,
            vec![
                vec![EvaluatedValue::Int(1.into()), EvaluatedValue::Int(2.into())],
                vec![EvaluatedValue::Int(3.into()), EvaluatedValue::Int(4.into())],
            ],
        );
        let original = StoredRuntimeValue::new(
            EvaluatedValue::List(values.clone().into()),
            plan.value_metadata(),
        );
        let tail = storage.tail_tuple(type_id, &values, 1);
        let returned =
            StoredRuntimeValue::new(EvaluatedValue::List(tail.into()), plan.value_metadata());
        let first = EmbeddingList::from_borrowed(&BorrowedValue::from_stored(&original));
        let second = EmbeddingList::from_borrowed(&BorrowedValue::from_stored(&returned));
        assert!(values.values().get_with_cache(1).unwrap().1.get().is_none());
        first
            .read_item(1, |row| {
                let left = row.tuple_item(0).int_bigint();
                second
                    .read_item(0, |row| {
                        let right = row.tuple_item(0).int_bigint();
                        assert!(ptr::eq(left.as_ref(), right.as_ref()));
                        assert_eq!(left.as_ref(), &BigInt::from(3));
                    })
                    .unwrap();
            })
            .unwrap();
        assert!(values.values().get_with_cache(0).unwrap().1.get().is_none());
        drop(first);
        drop(original);
        drop(values);
        drop(storage);
        assert_eq!(
            second.read_item(0, |row| row.tuple_item(1).int_bigint().as_ref().clone()),
            Some(BigInt::from(4))
        );
    }

    #[test]
    fn lazy_list_aliases_share_only_requested_integer_conversions() {
        use crate::plan::execution::runtime::RuntimeExecutionPlan;
        use crate::runtime::{EmbeddingList, RuntimeListStorage};
        let plan = crate::runtime::plan_src("pub fn main() { [1, 2] }");
        let storage = RuntimeListStorage::default();
        let values = storage.int(
            plan.int_list_function_id(0).type_id(),
            vec![1.into(), 2.into()],
        );
        let stored = StoredRuntimeValue::new(
            EvaluatedValue::List(values.clone().into()),
            plan.value_metadata(),
        );
        let first = EmbeddingList::from_borrowed(&BorrowedValue::from_stored(&stored));
        let second = EmbeddingList::from_borrowed(&BorrowedValue::from_stored(&stored));
        assert!(stored.integer_reads().get().is_none());
        let cache = values.values().get_with_cache(0).unwrap().1;
        assert!(cache.get().is_none());
        first
            .read_item(0, |row| {
                let value = row.int_bigint();
                second
                    .read_item(0, |row| {
                        let alias = row.int_bigint();
                        assert!(ptr::eq(value.as_ref(), alias.as_ref()));
                    })
                    .unwrap();
            })
            .unwrap();
        assert!(cache.get().is_some());
        assert!(values.values().get_with_cache(1).unwrap().1.get().is_none());
        drop(values);
        drop(stored);
        drop(storage);
        assert_eq!(
            second.read_item(1, |row| row.int_bigint().as_ref().clone()),
            Some(BigInt::from(2))
        );
        assert_eq!(second.len(), 2);
    }
}
