mod function;
mod storage;

pub(super) use function::provider::__GeamAsyncStores as TransferStores;
pub(super) use function::provider::__GeamStores as Stores;
pub(crate) use function::provider::DynamicPayload;
pub use function::provider::{
    __GeamExternalSchema0 as DynamicSchema, __GeamExternalStorage0 as DynamicExternalStorage,
};
pub use function::{create_transfer_value, create_value};

pub(crate) use self::storage::DynamicRepresentation;
use super::GleamStdlibLocalProfile;
use crate::{HostExternalType, HostProviderModule, HostRegistrationError};

pub type Dynamic = HostExternalType<DynamicSchema>;
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

#[cfg(test)]
mod tests {
    use super::host_provider;
    use crate::GleamStdlibProfile;

    #[test]
    fn registers_the_exact_official_dynamic_provider_inventory() {
        let provider = host_provider::<GleamStdlibProfile>()
            .expect("official dynamic provider should register");

        assert_eq!(provider.package(), "gleam_stdlib");
        assert_eq!(provider.module(), "gleam/dynamic");
        assert_eq!(
            provider
                .external_types()
                .map(|schema| {
                    (
                        schema.package().as_str(),
                        schema.module().as_str(),
                        schema.name().as_str(),
                        schema.parameter_count(),
                    )
                })
                .collect::<Vec<_>>(),
            [("gleam_stdlib", "gleam/dynamic", "Dynamic", 0)],
        );
        assert_eq!(
            provider
                .functions()
                .map(|function| function.name().as_str())
                .collect::<Vec<_>>(),
            [
                "classify",
                "bool",
                "string",
                "float",
                "int",
                "bit_array",
                "list",
                "array",
                "cast",
            ],
        );
    }
}
