mod function;
mod storage;

pub(super) use function::provider::__GeamStores as Stores;
pub use function::provider::{
    __GeamExternalSchema0 as DynamicSchema, __GeamExternalStorage0 as DynamicExternalStorage,
    DynamicPayload,
};

pub(crate) use self::storage::DynamicRepresentation;
use super::GleamStdlibHostProfile;
use crate::{
    HostCall, HostConstruction, HostExternal, HostExternalBinding, HostExternalType, HostProvider,
    HostProviderModule, HostRegistrationError, HostType, HostTypeListEnd, stdlib_stores,
};

pub type Dynamic = HostExternalType<DynamicSchema>;
fn stores<Profile>(stores: &Profile::ExternalStores) -> &Stores
where
    Profile: GleamStdlibHostProfile,
{
    &stdlib_stores::<Profile>(stores).dynamic
}

pub(super) fn host_provider<Profile>() -> Result<HostProviderModule<Profile>, HostRegistrationError>
where
    Profile: GleamStdlibHostProfile,
{
    function::host_provider::<Profile>()
}

pub fn create_value<'call, Profile, Provider, Return, Type>(
    call: &mut HostCall<'call, Profile, Provider, Return>,
    construction: HostConstruction<'call, Dynamic>,
    value: Type::Value<'call>,
) -> HostExternal<'call, Dynamic>
where
    Profile: GleamStdlibHostProfile,
    Provider: HostProvider<Profile>
        + HostExternalBinding<Profile, DynamicSchema, Storage = DynamicExternalStorage>,
    Return: HostType,
    Type: HostType,
{
    call.construct_external_with::<DynamicSchema, HostTypeListEnd>(construction, |builder| {
        DynamicPayload::stored(geam_core::__macro_support::retain_dynamic::<
            _,
            HostTypeListEnd,
            DynamicPayload,
            Type,
        >(builder, value))
    })
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
