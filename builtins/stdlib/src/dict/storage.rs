use ecow::EcoString;
use geam_core::provider::advanced::{
    Equality, Hashing, Index0, Inspection, Next, Retained, RetainedExternalPayload,
};
use im::{HashMap, Vector};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::rc::Rc;

pub struct DictPayload {
    pub(super) storage: DictStorage,
}

#[derive(Clone, Default)]
pub(super) struct DictStorage {
    pub(super) buckets: HashMap<u64, Vector<Rc<DictEntry>>>,
    pub(super) len: usize,
}

pub(super) struct DictEntry {
    pub(super) key_hash: u64,
    pub(super) key: Rc<Retained<DictPayload, Index0>>,
    pub(super) value: Rc<Retained<DictPayload, Next<Index0>>>,
}

impl DictPayload {
    pub(crate) fn coordinates(&self) -> Vec<(u64, usize)> {
        self.storage
            .buckets
            .iter()
            .flat_map(|(key_hash, bucket)| (0..bucket.len()).map(move |index| (*key_hash, index)))
            .collect()
    }

    pub(crate) fn key(&self, key_hash: u64, index: usize) -> &Retained<Self, Index0> {
        self.storage.buckets[&key_hash][index].key.as_ref()
    }

    pub(crate) fn value(&self, key_hash: u64, index: usize) -> &Retained<Self, Next<Index0>> {
        self.storage.buckets[&key_hash][index].value.as_ref()
    }

    pub(crate) fn cloned(&self) -> Self {
        Self {
            storage: self.storage.clone(),
        }
    }
}

impl RetainedExternalPayload for DictPayload {
    fn source_equal(&self, context: &Equality<'_>, other: &Self) -> bool {
        storage_equal(context, &self.storage, &other.storage)
    }

    fn source_hash(&self, context: &Hashing<'_>) -> u64 {
        storage_hash(context, &self.storage)
    }

    fn inspect(&self, context: &Inspection<'_>) -> EcoString {
        inspect_storage(context, &self.storage)
    }
}

fn storage_equal(context: &Equality<'_>, left: &DictStorage, right: &DictStorage) -> bool {
    left.len == right.len
        && left.entries().all(|left| {
            right.buckets.get(&left.key_hash).is_some_and(|bucket| {
                bucket.iter().any(|right| {
                    left.key.source_equal(context, &right.key)
                        && left.value.source_equal(context, &right.value)
                })
            })
        })
}

fn storage_hash(context: &Hashing<'_>, storage: &DictStorage) -> u64 {
    let mut sum = 0_u64;
    let mut xor = 0_u64;
    for entry in storage.entries() {
        let mut hasher = DefaultHasher::new();
        entry.key_hash.hash(&mut hasher);
        entry.value.source_hash(context).hash(&mut hasher);
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

fn inspect_storage(context: &Inspection<'_>, storage: &DictStorage) -> EcoString {
    let mut entries = storage
        .entries()
        .map(|entry| {
            format!(
                "#({}, {})",
                entry.key.inspect(context),
                entry.value.inspect(context),
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
