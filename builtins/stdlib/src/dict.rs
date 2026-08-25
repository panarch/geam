mod function;
mod storage;

pub(super) use function::provider::__GeamStores as Stores;
pub(crate) use function::provider::DictValue as DictDeclaration;
pub use function::provider::{
    __GeamExternalSchema0 as DictSchema, __GeamExternalStorage0 as DictExternalStorage,
};

use self::storage::{DictEntry, DictPayload, DictStorage};
use super::GleamStdlibHostProfile;
use crate::dynamic::Dynamic;
use crate::{
    HostCall, HostConstruction, HostExternal, HostExternalBinding, HostExternalType, HostProvider,
    HostProviderModule, HostRegistrationError, HostType, HostTypeIndex0, HostTypeIndexNext,
    HostTypeList, HostTypeListEnd, stdlib_stores,
};
use std::collections::HashMap;
use std::rc::Rc;

pub type DictOf<Key, Item> =
    HostExternalType<DictSchema, HostTypeList<Key, HostTypeList<Item, HostTypeListEnd>>>;

type KeyIndex = HostTypeIndex0;
type ItemIndex = HostTypeIndexNext<KeyIndex>;

pub(crate) type DynamicDictOutput = function::provider::DictValue<
    crate::dynamic::DynamicPayload,
    crate::dynamic::DynamicPayload,
    geam_core::__macro_support::ProviderExternalOutput<DictPayload>,
>;

pub fn create_dynamic_dict<'call, Profile, Provider, Return>(
    call: &mut HostCall<'call, Profile, Provider, Return>,
    construction: HostConstruction<'call, DictOf<Dynamic, Dynamic>>,
    entries: impl IntoIterator<Item = (HostExternal<'call, Dynamic>, HostExternal<'call, Dynamic>)>,
) -> HostExternal<'call, DictOf<Dynamic, Dynamic>>
where
    Profile: GleamStdlibHostProfile,
    Provider: HostProvider<Profile>
        + HostExternalBinding<Profile, DictSchema, Storage = DictExternalStorage>,
    Return: HostType,
{
    let mut buckets = HashMap::new();
    for (key, value) in entries {
        let key_hash = call.source_hash::<Dynamic>(key);
        function::insert_first(&mut buckets, key_hash, key, value, |stored, candidate| {
            call.equal::<Dynamic>(*stored, *candidate)
        });
    }

    call.construct_external_with(construction, move |builder| {
        let len = buckets.values().map(Vec::len).sum();
        let buckets = buckets
            .into_iter()
            .map(|(key_hash, entries)| {
                let entries = entries
                    .into_iter()
                    .map(|(key, value)| {
                        Rc::new(DictEntry {
                            key_hash,
                            key: Rc::new(geam_core::__macro_support::retain_argument::<
                                _,
                                _,
                                DictPayload,
                                KeyIndex,
                            >(builder, key)),
                            value: Rc::new(geam_core::__macro_support::retain_argument::<
                                _,
                                _,
                                DictPayload,
                                ItemIndex,
                            >(builder, value)),
                        })
                    })
                    .collect();
                (key_hash, entries)
            })
            .collect();
        DictPayload {
            storage: DictStorage { buckets, len },
        }
    })
}

fn stores<Profile>(stores: &Profile::ExternalStores) -> &Stores
where
    Profile: GleamStdlibHostProfile,
{
    &stdlib_stores::<Profile>(stores).dict
}

pub(super) fn host_provider<Profile>() -> Result<HostProviderModule<Profile>, HostRegistrationError>
where
    Profile: GleamStdlibHostProfile,
{
    function::host_provider::<Profile>()
}
