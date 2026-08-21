use super::schema::{DictSchema, StoredKey, StoredValue, TransientDictSchema};
use crate::gleam_stdlib::{GleamStdlibHostProfile, stdlib_stores};
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

pub(crate) struct DictPayload {
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

pub(crate) struct DictExternalStorage;
pub(super) struct TransientDictExternalStorage;

impl<Profile> HostExternalStorage<Profile, DictSchema> for DictExternalStorage
where
    Profile: GleamStdlibHostProfile,
{
    type Payload = DictPayload;

    fn store(stores: &Profile::ExternalStores) -> &HostExternalStore<Self::Payload> {
        &stdlib_stores::<Profile>(stores).dict.dicts
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

impl<Profile> HostExternalStorage<Profile, TransientDictSchema> for TransientDictExternalStorage
where
    Profile: GleamStdlibHostProfile,
{
    type Payload = TransientDictPayload;

    fn store(stores: &Profile::ExternalStores) -> &HostExternalStore<Self::Payload> {
        &stdlib_stores::<Profile>(stores).dict.transients
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

    fn entries(&self) -> impl Iterator<Item = &Rc<DictEntry>> {
        self.buckets.values().flat_map(Vector::iter)
    }
}
