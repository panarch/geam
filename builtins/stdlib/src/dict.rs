mod function;
mod storage;

pub use function::create_dynamic_dict;
pub(super) use function::provider::__GeamStores as Stores;
pub(crate) use function::provider::DictValue as DictDeclaration;
pub use function::provider::{
    __GeamExternalSchema0 as DictSchema, __GeamExternalStorage0 as DictExternalStorage,
};

use self::storage::DictPayload;
use super::GleamStdlibProviderProfile;
use crate::{
    HostExternalType, HostProviderModule, HostRegistrationError, HostTypeList, HostTypeListEnd,
};

pub type DictOf<Key, Item> =
    HostExternalType<DictSchema, HostTypeList<Key, HostTypeList<Item, HostTypeListEnd>>>;

pub(crate) type DynamicDictOutput = function::provider::DictValue<
    crate::dynamic::DynamicPayload,
    crate::dynamic::DynamicPayload,
    geam_core::__macro_support::ProviderExternalOutput<DictPayload>,
>;

pub(super) fn host_provider<Profile>() -> Result<HostProviderModule<Profile>, HostRegistrationError>
where
    Profile: GleamStdlibProviderProfile,
{
    function::host_provider::<Profile>()
}
