use crate::storage::StorageContext;
use ecow::EcoString;
use geam_core::provider::advanced::{
    Equality, Hashing, Index0, Inspection, LocalRetainedContext, Next, Retained,
    RetainedExternalPayload,
};
use im::{HashMap, Vector};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub struct DictPayload<Context: StorageContext = LocalRetainedContext> {
    pub(super) storage: DictStorage<Context>,
}

pub(super) struct DictStorage<Context: StorageContext = LocalRetainedContext> {
    pub(super) buckets: HashMap<u64, Vector<Context::Shared<DictEntry<Context>>>>,
    pub(super) len: usize,
}

pub(super) struct DictEntry<Context: StorageContext = LocalRetainedContext> {
    pub(super) key_hash: u64,
    pub(super) key: Context::Shared<Retained<DictPayload, Index0, Context>>,
    pub(super) value: Context::Shared<Retained<DictPayload, Next<Index0>, Context>>,
}

impl<Context: StorageContext> DictEntry<Context> {
    pub(super) fn new(
        key_hash: u64,
        key: Retained<DictPayload, Index0, Context>,
        value: Retained<DictPayload, Next<Index0>, Context>,
    ) -> Context::Shared<Self> {
        Context::share(Self {
            key_hash,
            key: Context::share(key),
            value: Context::share(value),
        })
    }

    pub(super) fn with_value(
        &self,
        value: Retained<DictPayload, Next<Index0>, Context>,
    ) -> Context::Shared<Self> {
        Context::share(Self {
            key_hash: self.key_hash,
            key: self.key.clone(),
            value: Context::share(value),
        })
    }
}

impl<Context: StorageContext> Clone for DictStorage<Context> {
    fn clone(&self) -> Self {
        Self {
            buckets: self.buckets.clone(),
            len: self.len,
        }
    }
}

impl<Context: StorageContext> Default for DictStorage<Context> {
    fn default() -> Self {
        Self {
            buckets: HashMap::new(),
            len: 0,
        }
    }
}

impl<Context: StorageContext> DictPayload<Context> {
    pub(crate) fn coordinates(&self) -> Vec<(u64, usize)> {
        self.storage
            .buckets
            .iter()
            .flat_map(|(key_hash, bucket)| (0..bucket.len()).map(move |index| (*key_hash, index)))
            .collect()
    }

    pub(crate) fn key(
        &self,
        key_hash: u64,
        index: usize,
    ) -> &Retained<DictPayload, Index0, Context> {
        self.storage.buckets[&key_hash][index].key.as_ref()
    }

    pub(crate) fn value(
        &self,
        key_hash: u64,
        index: usize,
    ) -> &Retained<DictPayload, Next<Index0>, Context> {
        self.storage.buckets[&key_hash][index].value.as_ref()
    }

    pub(crate) fn cloned(&self) -> Self {
        Self {
            storage: self.storage.clone(),
        }
    }
}

impl<Context: StorageContext> RetainedExternalPayload<Context> for DictPayload<Context> {
    fn source_equal(&self, context: &Equality<'_, Context>, other: &Self) -> bool {
        storage_equal(context, &self.storage, &other.storage)
    }

    fn source_hash(&self, context: &Hashing<'_, Context>) -> u64 {
        storage_hash(context, &self.storage)
    }

    fn inspect(&self, context: &Inspection<'_, Context>) -> EcoString {
        inspect_storage(context, &self.storage)
    }
}

fn storage_equal<Context: StorageContext>(
    context: &Equality<'_, Context>,
    left: &DictStorage<Context>,
    right: &DictStorage<Context>,
) -> bool {
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

fn storage_hash<Context: StorageContext>(
    context: &Hashing<'_, Context>,
    storage: &DictStorage<Context>,
) -> u64 {
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

fn inspect_storage<Context: StorageContext>(
    context: &Inspection<'_, Context>,
    storage: &DictStorage<Context>,
) -> EcoString {
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

impl<Context: StorageContext> DictStorage<Context> {
    pub(super) fn with_entry(
        &self,
        key_hash: u64,
        index: Option<usize>,
        entry: Context::Shared<DictEntry<Context>>,
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

    fn entries(&self) -> impl Iterator<Item = &Context::Shared<DictEntry<Context>>> {
        self.buckets.values().flat_map(Vector::iter)
    }
}
