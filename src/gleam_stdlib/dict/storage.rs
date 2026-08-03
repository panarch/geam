use super::schema::{DictSchema, StoredKey, StoredValue, TransientDictSchema};
use crate::gleam_stdlib::GleamStdlibHostProfile;
use crate::{
    HostExternalEquality, HostExternalHashing, HostExternalInspection, HostExternalStorage,
    HostExternalStore, HostStoredValue,
};
use ecow::EcoString;
use im::{HashMap, Vector};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::rc::Rc;

#[derive(Default)]
pub(in crate::gleam_stdlib) struct Stores {
    dicts: HostExternalStore<DictPayload>,
    transients: HostExternalStore<TransientDictPayload>,
}

pub(in crate::gleam_stdlib) struct DictPayload {
    pub(super) storage: DictStorage,
}

pub(super) struct TransientDictPayload {
    pub(super) storage: DictStorage,
}

#[derive(Clone, Default)]
pub(super) struct DictStorage {
    pub(super) buckets: HashMap<u64, Vector<Rc<DictEntry>>>,
    pub(super) len: usize,
}

pub(super) struct DictEntry {
    pub(super) key_hash: u64,
    pub(super) key: HostStoredValue<StoredKey>,
    pub(super) value: HostStoredValue<StoredValue>,
}

pub(super) trait DictPayloadStorage {
    fn storage(&self) -> &DictStorage;
}

impl<Profile> HostExternalStorage<DictSchema> for Profile
where
    Profile: GleamStdlibHostProfile,
{
    type Payload = DictPayload;

    fn store(stores: &Self::ExternalStores) -> &HostExternalStore<Self::Payload> {
        &Profile::gleam_stdlib_stores(stores).dict.dicts
    }

    fn source_equal(
        context: &HostExternalEquality<'_>,
        left: &Self::Payload,
        right: &Self::Payload,
    ) -> bool {
        storage_equal(context, &left.storage, &right.storage)
    }

    fn source_hash(context: &HostExternalHashing<'_>, value: &Self::Payload) -> u64 {
        storage_hash(context, &value.storage)
    }

    fn inspect(context: &HostExternalInspection<'_>, value: &Self::Payload) -> EcoString {
        inspect_storage(context, &value.storage)
    }
}

impl<Profile> HostExternalStorage<TransientDictSchema> for Profile
where
    Profile: GleamStdlibHostProfile,
{
    type Payload = TransientDictPayload;

    fn store(stores: &Self::ExternalStores) -> &HostExternalStore<Self::Payload> {
        &Profile::gleam_stdlib_stores(stores).dict.transients
    }

    fn source_equal(
        context: &HostExternalEquality<'_>,
        left: &Self::Payload,
        right: &Self::Payload,
    ) -> bool {
        storage_equal(context, &left.storage, &right.storage)
    }

    fn source_hash(context: &HostExternalHashing<'_>, value: &Self::Payload) -> u64 {
        storage_hash(context, &value.storage)
    }

    fn inspect(context: &HostExternalInspection<'_>, value: &Self::Payload) -> EcoString {
        inspect_storage(context, &value.storage)
    }
}

impl DictPayloadStorage for DictPayload {
    fn storage(&self) -> &DictStorage {
        &self.storage
    }
}

impl DictPayloadStorage for TransientDictPayload {
    fn storage(&self) -> &DictStorage {
        &self.storage
    }
}

fn storage_equal(
    context: &HostExternalEquality<'_>,
    left: &DictStorage,
    right: &DictStorage,
) -> bool {
    left.len == right.len
        && left.entries().all(|left| {
            right.buckets.get(&left.key_hash).is_some_and(|bucket| {
                bucket.iter().any(|right| {
                    context.stored_values_equal(&left.key, &right.key)
                        && context.stored_values_equal(&left.value, &right.value)
                })
            })
        })
}

fn storage_hash(context: &HostExternalHashing<'_>, storage: &DictStorage) -> u64 {
    let mut sum = 0_u64;
    let mut xor = 0_u64;
    for entry in storage.entries() {
        let mut hasher = DefaultHasher::new();
        entry.key_hash.hash(&mut hasher);
        context.stored_value_hash(&entry.value).hash(&mut hasher);
        let hash = hasher.finish();
        sum = sum.wrapping_add(hash);
        xor ^= hash.rotate_left(29);
    }

    let mut hasher = DefaultHasher::new();
    storage.len.hash(&mut hasher);
    sum.hash(&mut hasher);
    xor.hash(&mut hasher);
    hasher.finish()
}

fn inspect_storage(context: &HostExternalInspection<'_>, storage: &DictStorage) -> EcoString {
    let mut entries = storage
        .entries()
        .map(|entry| {
            format!(
                "#({}, {})",
                context.inspect_stored_value(&entry.key),
                context.inspect_stored_value(&entry.value),
            )
        })
        .collect::<Vec<_>>();
    entries.sort_unstable();
    format!("dict.from_list([{}])", entries.join(", ")).into()
}

impl DictStorage {
    fn entries(&self) -> impl Iterator<Item = &Rc<DictEntry>> {
        self.buckets.values().flat_map(Vector::iter)
    }

    pub(super) fn with_entry(
        &self,
        key_hash: u64,
        index: Option<usize>,
        entry: Rc<DictEntry>,
    ) -> Self {
        let mut bucket = self.buckets.get(&key_hash).cloned().unwrap_or_default();
        let len = match index {
            Some(index) => {
                bucket[index] = entry;
                self.len
            }
            None => {
                bucket.push_back(entry);
                self.len + 1
            }
        };
        let mut buckets = self.buckets.clone();
        buckets.insert(key_hash, bucket);
        Self { buckets, len }
    }

    pub(super) fn without_entry(&self, key_hash: u64, index: usize) -> Self {
        let mut bucket = self.buckets[&key_hash].clone();
        let removed = bucket.remove(index);
        drop(removed);
        let mut buckets = self.buckets.clone();
        if bucket.is_empty() {
            buckets.remove(&key_hash);
        } else {
            buckets.insert(key_hash, bucket);
        }
        Self {
            buckets,
            len: self.len - 1,
        }
    }

    pub(super) fn matching_index(
        &self,
        key_hash: u64,
        is_equal: &mut dyn FnMut(usize) -> bool,
    ) -> Option<usize> {
        let bucket = self.buckets.get(&key_hash)?;
        (0..bucket.len()).find(|index| is_equal(*index))
    }
}

#[cfg(test)]
mod tests {
    use super::super::schema::{DictSchema, TransientDictSchema};
    use super::{
        DictEntry, DictPayload, DictPayloadStorage, DictStorage, StoredKey, StoredValue,
        TransientDictPayload, inspect_storage, storage_equal, storage_hash,
    };
    use crate::gleam_stdlib::{GleamStdlibProfile, GleamStdlibStores};
    use crate::{
        HostExternalEquality, HostExternalHashing, HostExternalInspection, HostExternalStorage,
    };
    use ecow::EcoString;
    use num_bigint::BigInt;
    use std::cell::Cell;
    use std::rc::Rc;

    fn entry(key_hash: u64, key: i64, value: i64) -> Rc<DictEntry> {
        Rc::new(DictEntry {
            key_hash,
            key: crate::HostStoredValue::<StoredKey>::new(
                crate::runtime::StoredRuntimeValue::test_int(BigInt::from(key)),
            ),
            value: crate::HostStoredValue::<StoredValue>::new(
                crate::runtime::StoredRuntimeValue::test_int(BigInt::from(value)),
            ),
        })
    }

    #[test]
    fn external_storage_protocols_project_and_render_both_nominal_payloads() {
        let stores = GleamStdlibStores::default();
        let stored_equal =
            |left: &crate::runtime::StoredRuntimeValue,
             right: &crate::runtime::StoredRuntimeValue| { std::ptr::eq(left, right) };
        let stored_hash = |_: &crate::runtime::StoredRuntimeValue| 7;
        let inspect = |_: &crate::runtime::StoredRuntimeValue| EcoString::from("stored");
        let equality = HostExternalEquality::new(&stored_equal);
        let hashing = HostExternalHashing::new(&stored_hash);
        let inspection = HostExternalInspection::new(&inspect);
        let storage = DictStorage::default().with_entry(11, None, entry(11, 1, 10));
        let dict = DictPayload {
            storage: storage.clone(),
        };
        let transient = TransientDictPayload { storage };

        assert!(std::ptr::eq(dict.storage(), &dict.storage));
        assert!(std::ptr::eq(transient.storage(), &transient.storage));
        assert!(std::ptr::eq(
            <GleamStdlibProfile as HostExternalStorage<DictSchema>>::store(&stores),
            &stores.dict.dicts,
        ));
        assert!(std::ptr::eq(
            <GleamStdlibProfile as HostExternalStorage<TransientDictSchema>>::store(&stores),
            &stores.dict.transients,
        ));
        assert!(
            <GleamStdlibProfile as HostExternalStorage<DictSchema>>::source_equal(
                &equality, &dict, &dict,
            )
        );
        assert!(<GleamStdlibProfile as HostExternalStorage<
            TransientDictSchema,
        >>::source_equal(&equality, &transient, &transient,));
        assert_eq!(
            <GleamStdlibProfile as HostExternalStorage<DictSchema>>::source_hash(&hashing, &dict),
            <GleamStdlibProfile as HostExternalStorage<TransientDictSchema>>::source_hash(
                &hashing, &transient,
            ),
        );
        assert_eq!(
            <GleamStdlibProfile as HostExternalStorage<DictSchema>>::inspect(&inspection, &dict),
            "dict.from_list([#(stored, stored)])",
        );
        assert_eq!(
            <GleamStdlibProfile as HostExternalStorage<TransientDictSchema>>::inspect(
                &inspection,
                &transient,
            ),
            "dict.from_list([#(stored, stored)])",
        );
    }

    #[test]
    fn persistent_updates_defer_external_hashing_and_inspection_until_demanded() {
        let stores = GleamStdlibStores::default();
        let first = entry(7, 1, 10);
        let replacement = entry(7, 1, 20);
        let initial = DictStorage::default().with_entry(7, None, first);
        let updated = initial.with_entry(7, Some(0), replacement);
        let removed = updated.without_entry(7, 0);
        let store = <GleamStdlibProfile as HostExternalStorage<DictSchema>>::store(&stores);
        let initial = store.insert(
            DictPayload { storage: initial },
            <GleamStdlibProfile as HostExternalStorage<DictSchema>>::source_equal,
            <GleamStdlibProfile as HostExternalStorage<DictSchema>>::source_hash,
            <GleamStdlibProfile as HostExternalStorage<DictSchema>>::inspect,
        );
        let updated = store.insert(
            DictPayload { storage: updated },
            <GleamStdlibProfile as HostExternalStorage<DictSchema>>::source_equal,
            <GleamStdlibProfile as HostExternalStorage<DictSchema>>::source_hash,
            <GleamStdlibProfile as HostExternalStorage<DictSchema>>::inspect,
        );
        let removed = store.insert(
            DictPayload { storage: removed },
            <GleamStdlibProfile as HostExternalStorage<DictSchema>>::source_equal,
            <GleamStdlibProfile as HostExternalStorage<DictSchema>>::source_hash,
            <GleamStdlibProfile as HostExternalStorage<DictSchema>>::inspect,
        );
        let hash_calls = Cell::new(0);
        let inspection_calls = Cell::new(0);
        let stored_hash = |_: &crate::runtime::StoredRuntimeValue| {
            hash_calls.set(hash_calls.get() + 1);
            11
        };
        let inspect = |_: &crate::runtime::StoredRuntimeValue| {
            inspection_calls.set(inspection_calls.get() + 1);
            EcoString::from("stored")
        };
        let hashing = HostExternalHashing::new(&stored_hash);
        let inspection = HostExternalInspection::new(&inspect);

        assert_eq!(hash_calls.get(), 0);
        assert_eq!(inspection_calls.get(), 0);
        assert_eq!(
            removed.source_hash(&hashing),
            storage_hash(&hashing, &DictStorage::default()),
        );
        assert_eq!(hash_calls.get(), 0);
        assert_eq!(
            updated.inspection(&inspection),
            "dict.from_list([#(stored, stored)])"
        );
        assert_eq!(
            updated.inspection(&inspection),
            "dict.from_list([#(stored, stored)])"
        );
        assert_eq!(inspection_calls.get(), 2);
        let initial_hash = initial.source_hash(&hashing);
        assert_eq!(initial.source_hash(&hashing), initial_hash);
        assert_eq!(hash_calls.get(), 1);
    }

    #[test]
    fn persistent_buckets_replace_remove_and_share_only_unchanged_entries() {
        let first = entry(7, 1, 10);
        let second = entry(7, 2, 20);
        let replacement = entry(7, 1, 30);
        let distinct = entry(11, 3, 40);

        let one = DictStorage::default().with_entry(7, None, Rc::clone(&first));
        let alias = one.clone();
        let collided =
            one.with_entry(7, None, Rc::clone(&second))
                .with_entry(11, None, Rc::clone(&distinct));
        let replaced = collided.with_entry(7, Some(0), Rc::clone(&replacement));
        let retained_collision = replaced.without_entry(7, 0);
        let removed_bucket = retained_collision.without_entry(7, 0);

        assert_eq!(alias.len, 1);
        assert!(Rc::ptr_eq(&alias.buckets[&7][0], &first));
        assert_eq!(collided.len, 3);
        assert!(Rc::ptr_eq(&collided.buckets[&7][0], &first));
        assert!(Rc::ptr_eq(&collided.buckets[&7][1], &second));
        assert!(Rc::ptr_eq(&collided.buckets[&11][0], &distinct));
        assert_eq!(replaced.len, 3);
        assert!(Rc::ptr_eq(&replaced.buckets[&7][0], &replacement));
        assert!(Rc::ptr_eq(&replaced.buckets[&7][1], &second));
        assert_eq!(retained_collision.len, 2);
        assert!(Rc::ptr_eq(&retained_collision.buckets[&7][0], &second));
        assert_eq!(removed_bucket.len, 1);
        assert!(!removed_bucket.buckets.contains_key(&7));
        assert!(Rc::ptr_eq(&removed_bucket.buckets[&11][0], &distinct));

        let mut visited = Vec::new();
        assert_eq!(
            collided.matching_index(7, &mut |index| {
                visited.push(index);
                index == 1
            }),
            Some(1),
        );
        assert_eq!(visited, [0, 1]);
        assert_eq!(collided.matching_index(7, &mut |_| false), None);
        assert_eq!(collided.matching_index(99, &mut |_| true), None);
    }

    #[test]
    fn source_semantics_resolve_collisions_and_ignore_bucket_iteration_order() {
        let first = entry(7, 1, 10);
        let equal_first = entry(7, 1, 10);
        let collision = entry(7, 2, 20);
        let equal_collision = entry(7, 2, 20);
        let different_value = entry(7, 2, 30);

        let left = DictStorage::default()
            .with_entry(7, None, Rc::clone(&first))
            .with_entry(7, None, Rc::clone(&collision));
        let equal = DictStorage::default()
            .with_entry(7, None, Rc::clone(&equal_collision))
            .with_entry(7, None, Rc::clone(&equal_first));
        let different = DictStorage::default()
            .with_entry(7, None, Rc::clone(&equal_first))
            .with_entry(7, None, Rc::clone(&different_value));

        let stored_equal = |left: &crate::runtime::StoredRuntimeValue,
                            right: &crate::runtime::StoredRuntimeValue| {
            std::ptr::eq(left, right)
                || std::ptr::eq(left, &first.key.value)
                    && std::ptr::eq(right, &equal_first.key.value)
                || std::ptr::eq(left, &first.value.value)
                    && std::ptr::eq(right, &equal_first.value.value)
                || std::ptr::eq(left, &collision.key.value)
                    && (std::ptr::eq(right, &equal_collision.key.value)
                        || std::ptr::eq(right, &different_value.key.value))
                || std::ptr::eq(left, &collision.value.value)
                    && std::ptr::eq(right, &equal_collision.value.value)
        };
        let stored_hash = |value: &crate::runtime::StoredRuntimeValue| {
            if std::ptr::eq(value, &first.value.value)
                || std::ptr::eq(value, &equal_first.value.value)
            {
                10
            } else if std::ptr::eq(value, &collision.value.value)
                || std::ptr::eq(value, &equal_collision.value.value)
            {
                20
            } else {
                30
            }
        };
        let inspect = |value: &crate::runtime::StoredRuntimeValue| {
            if std::ptr::eq(value, &first.key.value) {
                EcoString::from("z-key")
            } else if std::ptr::eq(value, &first.value.value) {
                EcoString::from("one")
            } else if std::ptr::eq(value, &collision.key.value) {
                EcoString::from("a-key")
            } else {
                EcoString::from("two")
            }
        };
        let equality = HostExternalEquality::new(&stored_equal);
        let hashing = HostExternalHashing::new(&stored_hash);
        let inspection = HostExternalInspection::new(&inspect);

        assert!(storage_equal(&equality, &left, &equal));
        assert!(!storage_equal(&equality, &left, &different));
        assert!(!storage_equal(&equality, &left, &DictStorage::default()));
        assert_eq!(
            storage_hash(&hashing, &left),
            storage_hash(&hashing, &equal)
        );
        assert_ne!(
            storage_hash(&hashing, &left),
            storage_hash(&hashing, &different),
        );
        assert_eq!(
            inspect_storage(&inspection, &left),
            "dict.from_list([#(a-key, two), #(z-key, one)])",
        );
        assert_eq!(
            inspect_storage(&inspection, &DictStorage::default()),
            "dict.from_list([])",
        );
    }
}
