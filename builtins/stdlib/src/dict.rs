mod function;
mod storage;

pub(super) use function::provider::__GeamAsyncStores as TransferStores;
pub(super) use function::provider::__GeamStores as Stores;
pub(crate) use function::provider::DictValue as DictDeclaration;
pub use function::provider::{
    __GeamExternalSchema0 as DictSchema, __GeamExternalStorage0 as DictExternalStorage,
};
pub use function::{create_dynamic_dict, create_transfer_dynamic_dict};

use self::storage::DictPayload;
use super::GleamStdlibLocalProfile;
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
    Profile: GleamStdlibLocalProfile,
{
    function::host_provider::<Profile>()
}

pub(super) fn transfer_host_provider<Profile>()
-> Result<geam_core::TransferHostProviderModule<Profile>, HostRegistrationError>
where
    Profile: crate::GleamStdlibTransferProfile,
    Profile::RunState: Send,
{
    function::transfer_host_provider::<Profile>()
}
