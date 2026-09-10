use crate::schema::SelectorSchema;
use crate::{Component, GleamErlangHostProfile};
use ecow::EcoString;
use geam_core::host::native::NativeCallable;
use geam_core::host::{
    HostComponentProfile, HostExternalEquality, HostExternalHashing, HostExternalInspection,
    HostExternalStorage, HostExternalStore, HostProfile,
};
use geam_core::provider::advanced::{NativeMap, NativeMapEntry, NativeValue};
use std::sync::Arc;

pub struct Selector<Profile: HostProfile> {
    pub(crate) entries: im::HashMap<u64, im::Vector<Arc<Entry<Profile>>>>,
    pub(crate) len: usize,
}

pub(crate) struct Entry<Profile: HostProfile> {
    pub(crate) key: NativeValue,
    pub(crate) handler: Arc<Handler<Profile>>,
}

pub(crate) struct Handler<Profile: HostProfile> {
    pub(crate) callback: NativeCallable<Profile>,
    pub(crate) mappings: im::OrdMap<usize, NativeCallable<Profile>>,
    pub(crate) native: NativeValue,
}

impl<Profile: HostProfile> Clone for Selector<Profile> {
    fn clone(&self) -> Self {
        Self {
            entries: self.entries.clone(),
            len: self.len,
        }
    }
}

impl<Profile: HostProfile> Default for Selector<Profile> {
    fn default() -> Self {
        Self {
            entries: im::HashMap::new(),
            len: 0,
        }
    }
}

impl<Profile: HostProfile> Selector<Profile> {
    fn native(&self) -> NativeValue {
        let map =
            NativeMap::new(
                self.clone(),
                |selector| selector.len,
                |selector| {
                    selector
                        .entries
                        .clone()
                        .into_iter()
                        .flat_map(|(hash, entries)| {
                            entries.into_iter().map(move |entry| NativeMapEntry {
                                key_hash: hash,
                                key: entry.key.clone(),
                                value: entry.handler.native.clone(),
                            })
                        })
                },
                |selector, hash, key, equal| {
                    selector.entries.get(&hash)?.iter().find_map(|entry| {
                        equal(&entry.key, key).then(|| entry.handler.native.clone())
                    })
                },
            );
        NativeValue::tuple([NativeValue::symbol("selector"), NativeValue::map(map)])
    }
}

pub struct Storage;

impl<Profile: GleamErlangHostProfile> HostExternalStorage<Profile, SelectorSchema> for Storage {
    type Payload = Selector<Profile>;

    fn store(stores: &Profile::ExternalStores) -> &HostExternalStore<Selector<Profile>> {
        &<Profile as HostComponentProfile<Component<Profile>>>::component_stores(stores).selectors
    }

    fn source_equal(
        context: &HostExternalEquality<'_>,
        left: &Selector<Profile>,
        right: &Selector<Profile>,
    ) -> bool {
        left.native().source_equal(context, &right.native())
    }

    fn source_hash(context: &HostExternalHashing<'_>, value: &Selector<Profile>) -> u64 {
        value.native().source_hash(context)
    }

    fn inspect(context: &HostExternalInspection<'_>, value: &Selector<Profile>) -> EcoString {
        value.native().inspect(context)
    }

    fn native_view(value: &Selector<Profile>) -> Option<NativeValue> {
        Some(value.native())
    }
}

#[cfg(test)]
mod tests {
    use super::{Selector, Storage};
    use crate::GleamErlangProfile;
    use crate::schema::SelectorSchema;
    use geam_core::host::HostExternalStorage;
    use geam_core::provider::advanced::NativeValue;

    #[test]
    fn selector_storage_uses_its_native_record_in_each_operation() {
        crate::test_support::with_contexts(
            |context| {
                let value = Selector::<GleamErlangProfile>::default();
                let equal = <Storage as HostExternalStorage<GleamErlangProfile, SelectorSchema>>::source_equal;
                assert!(equal(context, &value, &value.clone()));
                assert!(
                    !value
                        .native()
                        .source_equal(context, &NativeValue::symbol("selector"))
                );
            },
            |context| {
                let value = Selector::<GleamErlangProfile>::default();
                let hash = <Storage as HostExternalStorage<GleamErlangProfile, SelectorSchema>>::source_hash;
                assert_eq!(hash(context, &value), value.native().source_hash(context));
            },
            |context| {
                let value = Selector::<GleamErlangProfile>::default();
                let inspect =
                    <Storage as HostExternalStorage<GleamErlangProfile, SelectorSchema>>::inspect;
                assert_eq!(inspect(context, &value), "Selector(dict.from_list([]))");
            },
        );
    }
}
