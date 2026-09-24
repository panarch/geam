mod dynamic;
mod function;
mod storage;

pub(crate) use dynamic::DynamicDictOutput;
use function::create_dynamic_dict_with;
pub(super) use function::provider::__GeamStores as Stores;
pub(crate) use function::provider::DictValue as DictDeclaration;
pub use function::provider::{
    __GeamExternalSchema0 as DictSchema, __GeamExternalStorage0 as DictExternalStorage,
};
pub use function::{create_dynamic_dict, dict_from_entries};

use self::storage::DictPayload;
use super::GleamStdlibProviderProfile;
use crate::{
    HostExternalType, HostProviderModule, HostRegistrationError, HostTypeList, HostTypeListEnd,
};

/// The original `gleam/dict.Dict(key, item)` with two typed host arguments.
///
/// Construct values through [`crate::service::dict_from_entries`]. The standard
/// library continues to own their storage and Gleam value semantics.
pub type DictOf<Key, Item> =
    HostExternalType<DictSchema, HostTypeList<Key, HostTypeList<Item, HostTypeListEnd>>>;

type ExactDynamicDictOutput = function::provider::DictValue<
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
