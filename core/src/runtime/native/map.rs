use super::NativeValue;
use std::sync::Arc;

/// An immutable map view over a provider-owned snapshot.
///
/// The snapshot and its accessors must describe the same immutable entries.
/// Creating a view does not enumerate its entries. Each traversal owns its
/// cursor so no payload borrow is held while nested values are inspected.
#[derive(Clone)]
pub struct NativeMap(Arc<dyn Mapping + Send + Sync>);

/// One native map entry with its already-computed native key hash.
pub struct NativeMapEntry {
    pub key_hash: u64,
    pub key: NativeValue,
    pub value: NativeValue,
}

type KeyEquality<'value> = dyn Fn(&NativeValue, &NativeValue) -> bool + 'value;
type Lookup<Snapshot> = fn(&Snapshot, u64, &NativeValue, &KeyEquality<'_>) -> Option<NativeValue>;

trait Mapping {
    fn len(&self) -> usize;
    fn entries(&self) -> Box<dyn Iterator<Item = NativeMapEntry> + Send>;
    fn get(&self, hash: u64, key: &NativeValue, equal: &KeyEquality<'_>) -> Option<NativeValue>;
}

struct SnapshotMap<Snapshot, Entries> {
    snapshot: Snapshot,
    len: fn(&Snapshot) -> usize,
    entries: fn(&Snapshot) -> Entries,
    lookup: Lookup<Snapshot>,
}

impl NativeMap {
    /// Binds a read-only snapshot and its accessors at the representation owner.
    ///
    /// Entry hashes and `lookup` must use native equality and native key hashes.
    /// `lookup` resolves collisions with `equal`, and returns only retained views.
    /// The snapshot is a shareable view; the original Rust payload need not be
    /// Clone or Sync. Its accessors must not retain payload or state guards.
    pub fn new<Snapshot, Entries>(
        snapshot: Snapshot,
        len: fn(&Snapshot) -> usize,
        entries: fn(&Snapshot) -> Entries,
        lookup: Lookup<Snapshot>,
    ) -> Self
    where
        Snapshot: Send + Sync + 'static,
        Entries: Iterator<Item = NativeMapEntry> + Send + 'static,
    {
        Self(Arc::new(SnapshotMap {
            snapshot,
            len,
            entries,
            lookup,
        }))
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn entries(&self) -> impl Iterator<Item = NativeMapEntry> + Send + 'static {
        self.0.entries()
    }

    pub fn get(
        &self,
        hash: u64,
        key: &NativeValue,
        equal: impl Fn(&NativeValue, &NativeValue) -> bool,
    ) -> Option<NativeValue> {
        self.0.get(hash, key, &equal)
    }
}

impl<Snapshot, Entries> Mapping for SnapshotMap<Snapshot, Entries>
where
    Snapshot: Send + Sync + 'static,
    Entries: Iterator<Item = NativeMapEntry> + Send + 'static,
{
    fn len(&self) -> usize {
        (self.len)(&self.snapshot)
    }

    fn entries(&self) -> Box<dyn Iterator<Item = NativeMapEntry> + Send> {
        Box::new((self.entries)(&self.snapshot))
    }

    fn get(&self, hash: u64, key: &NativeValue, equal: &KeyEquality<'_>) -> Option<NativeValue> {
        (self.lookup)(&self.snapshot, hash, key, equal)
    }
}

#[cfg(test)]
mod tests {
    use super::{KeyEquality, NativeMap, NativeMapEntry};
    use crate::runtime::NativeValue;
    use std::sync::Arc;
    use std::sync::atomic::{AtomicUsize, Ordering};

    struct Snapshot {
        entries: Arc<[NativeMapEntry]>,
        visits: Arc<AtomicUsize>,
    }

    struct Entries {
        entries: Arc<[NativeMapEntry]>,
        visits: Arc<AtomicUsize>,
        index: usize,
    }

    impl Snapshot {
        fn len(&self) -> usize {
            self.entries.len()
        }

        fn entries(&self) -> Entries {
            Entries {
                entries: Arc::clone(&self.entries),
                visits: Arc::clone(&self.visits),
                index: 0,
            }
        }

        fn get(
            &self,
            hash: u64,
            key: &NativeValue,
            equal: &KeyEquality<'_>,
        ) -> Option<NativeValue> {
            self.entries.iter().find_map(|entry| {
                (entry.key_hash == hash && equal(&entry.key, key)).then(|| entry.value.clone())
            })
        }
    }

    impl Iterator for Entries {
        type Item = NativeMapEntry;

        fn next(&mut self) -> Option<Self::Item> {
            let entry = self.entries.get(self.index)?;
            self.index += 1;
            self.visits.fetch_add(1, Ordering::Relaxed);
            Some(NativeMapEntry {
                key_hash: entry.key_hash,
                key: entry.key.clone(),
                value: entry.value.clone(),
            })
        }
    }

    #[test]
    fn snapshot_access_is_lazy_and_iteration_outlives_the_view() {
        let visits = Arc::new(AtomicUsize::new(0));
        let entries: Arc<[NativeMapEntry]> = Arc::from([
            NativeMapEntry {
                key_hash: 7,
                key: NativeValue::symbol("first"),
                value: NativeValue::symbol("one"),
            },
            NativeMapEntry {
                key_hash: 7,
                key: NativeValue::symbol("second"),
                value: NativeValue::symbol("two"),
            },
        ]);
        let released = Arc::downgrade(&entries);
        let map = NativeMap::new(
            Snapshot {
                entries,
                visits: Arc::clone(&visits),
            },
            Snapshot::len,
            Snapshot::entries,
            Snapshot::get,
        );
        let alias = map.clone();
        assert_eq!(map.len(), 2);
        assert!(!map.is_empty());
        let mut cursor = map.entries();
        assert_eq!(visits.load(Ordering::Relaxed), 0);

        let comparisons = AtomicUsize::new(0);
        let equal = |left: &NativeValue, right: &NativeValue| {
            comparisons.fetch_add(1, Ordering::Relaxed);
            left.as_symbol() == right.as_symbol()
        };
        assert_eq!(
            map.get(7, &NativeValue::symbol("second"), equal)
                .unwrap()
                .as_symbol()
                .as_deref(),
            Some("two")
        );
        assert_eq!(comparisons.load(Ordering::Relaxed), 2);
        assert!(map.get(8, &NativeValue::symbol("second"), equal).is_none());
        assert_eq!(comparisons.load(Ordering::Relaxed), 2);
        assert!(map.get(7, &NativeValue::symbol("missing"), equal).is_none());
        assert_eq!(comparisons.load(Ordering::Relaxed), 4);
        assert_eq!(visits.load(Ordering::Relaxed), 0);

        drop(map);
        drop(alias);
        let first = cursor.next().unwrap();
        assert_eq!(first.key_hash, 7);
        assert_eq!(first.key.as_symbol().as_deref(), Some("first"));
        assert_eq!(first.value.as_symbol().as_deref(), Some("one"));
        assert_eq!(visits.load(Ordering::Relaxed), 1);
        assert_eq!(
            cursor.next().unwrap().key.as_symbol().as_deref(),
            Some("second")
        );
        assert!(cursor.next().is_none());
        assert_eq!(visits.load(Ordering::Relaxed), 2);
        assert!(released.upgrade().is_some());
        drop(cursor);
        assert!(released.upgrade().is_none());
    }

    #[test]
    fn native_maps_compare_and_hash_by_contents_independently_of_iteration_order() {
        use crate::host::{HostExternalInspection, RetainedValueInspection};
        use crate::runtime::native::{value_hash, values_equal};
        use crate::runtime::{RetainedValueRef, RuntimeListStorage};

        let storage = RuntimeListStorage::default();
        let inspection_calls = AtomicUsize::new(0);
        let inspect_opaque = |_: &RetainedValueRef| {
            inspection_calls.fetch_add(1, Ordering::Relaxed);
            "unexpected opaque value".into()
        };
        let inspection = RetainedValueInspection::new(&inspect_opaque);
        let inspection = HostExternalInspection(&inspection);
        let mut values = Vec::new();
        for entries in [
            vec![("first", "one"), ("second", "two")],
            vec![("second", "two"), ("first", "one")],
            vec![("first", "one")],
            vec![("first", "one"), ("third", "two")],
            vec![("first", "one"), ("second", "other")],
            vec![],
        ] {
            let entries = entries
                .into_iter()
                .map(|(key, value)| {
                    let key = NativeValue::symbol(key);
                    NativeMapEntry {
                        key_hash: value_hash(&storage, &key),
                        key,
                        value: NativeValue::symbol(value),
                    }
                })
                .collect::<Arc<[_]>>();
            values.push(NativeValue::map(NativeMap::new(
                Snapshot {
                    entries,
                    visits: Arc::new(AtomicUsize::new(0)),
                },
                Snapshot::len,
                Snapshot::entries,
                Snapshot::get,
            )));
        }
        assert_eq!(values[0].kind(), crate::runtime::NativeKind::Map);
        assert_eq!(values[0].as_map().unwrap().len(), 2);
        assert!(values[5].as_map().unwrap().is_empty());
        assert!(NativeValue::symbol("map").as_map().is_none());
        assert!(values_equal(&storage, &values[0], &values[1]));
        assert!(values_equal(&storage, &values[1], &values[0]));
        for other in &values[2..] {
            assert!(!values_equal(&storage, &values[0], other));
            assert!(!values_equal(&storage, other, &values[0]));
        }
        assert!(!values_equal(&storage, &values[0], &NativeValue::tuple([])));
        assert!(values_equal(&storage, &values[5], &values[5]));
        assert_eq!(
            value_hash(&storage, &values[0]),
            value_hash(&storage, &values[1])
        );
        assert_eq!(
            values[0].inspect(&inspection),
            "dict.from_list([#(First, One), #(Second, Two)])"
        );
        assert_eq!(
            values[1].inspect(&inspection),
            values[0].inspect(&inspection)
        );
        assert_eq!(values[5].inspect(&inspection), "dict.from_list([])");
        let nested = NativeValue::tuple([values[0].clone(), values[5].clone()]);
        let equal_nested = NativeValue::tuple([values[1].clone(), values[5].clone()]);
        assert!(values_equal(&storage, &nested, &equal_nested));
        assert_eq!(
            value_hash(&storage, &nested),
            value_hash(&storage, &equal_nested)
        );
        assert_eq!(inspection_calls.load(Ordering::Relaxed), 0);
        let callback = super::super::native_source("pub fn main() { fn(value) { value + 1 } }");
        let key = NativeValue::symbol("callback");
        let map = NativeValue::map(NativeMap::new(
            Snapshot {
                entries: Arc::from([NativeMapEntry {
                    key_hash: value_hash(&storage, &key),
                    key,
                    value: callback,
                }]),
                visits: Arc::new(AtomicUsize::new(0)),
            },
            Snapshot::len,
            Snapshot::entries,
            Snapshot::get,
        ));
        assert_eq!(
            map.inspect(&inspection),
            "dict.from_list([#(Callback, unexpected opaque value)])"
        );
        assert_eq!(inspection_calls.load(Ordering::Relaxed), 1);
    }
}
