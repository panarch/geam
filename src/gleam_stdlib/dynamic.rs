mod function;
mod schema;
mod storage;

pub(crate) use schema::{Dynamic, DynamicList, DynamicSchema};
pub(crate) use storage::DynamicExternalStorage;
pub(super) use storage::Stores;

pub(crate) use self::function::create_value;
pub(in crate::gleam_stdlib) use self::function::{
    DynamicProvider, classification, decode_value, sequence,
};
use self::function::{array, cast, classify};
use self::schema::Parameter;
use super::GleamStdlibHostProfile;
use crate::{HostList, HostProviderModule, HostRegistrationError};
use ecow::EcoString;
use num_bigint::BigInt;

pub(super) fn host_provider<Profile>() -> Result<HostProviderModule<Profile>, HostRegistrationError>
where
    Profile: GleamStdlibHostProfile,
{
    HostProviderModule::new("gleam_stdlib", "gleam/dynamic")
        .and_then(HostProviderModule::with_external_type::<DynamicProvider<Profile>, DynamicSchema>)
        .and_then(|provider| {
            provider.with_scoped_function::<DynamicProvider<Profile>, (Dynamic,), EcoString, _>(
                "classify",
                classify::<Profile>,
            )
        })
        .and_then(|provider| {
            provider.with_scoped_function::<DynamicProvider<Profile>, (bool,), Dynamic, _>(
                "bool",
                cast::<Profile, bool>,
            )
        })
        .and_then(|provider| {
            provider.with_scoped_function::<DynamicProvider<Profile>, (EcoString,), Dynamic, _>(
                "string",
                cast::<Profile, EcoString>,
            )
        })
        .and_then(|provider| {
            provider.with_scoped_function::<DynamicProvider<Profile>, (f64,), Dynamic, _>(
                "float",
                cast::<Profile, f64>,
            )
        })
        .and_then(|provider| {
            provider.with_scoped_function::<DynamicProvider<Profile>, (BigInt,), Dynamic, _>(
                "int",
                cast::<Profile, BigInt>,
            )
        })
        .and_then(|provider| {
            provider.with_scoped_function::<
                DynamicProvider<Profile>,
                (crate::BitArrayValue,),
                Dynamic,
                _,
            >("bit_array", cast::<Profile, crate::BitArrayValue>)
        })
        .and_then(|provider| {
            provider.with_scoped_function::<DynamicProvider<Profile>, (DynamicList,), Dynamic, _>(
                "list",
                cast::<Profile, DynamicList>,
            )
        })
        .and_then(|provider| {
            provider.with_scoped_function::<DynamicProvider<Profile>, (DynamicList,), Dynamic, _>(
                "array",
                array::<Profile>,
            )
        })
        .and_then(|provider| {
            provider.with_scoped_function::<DynamicProvider<Profile>, (Parameter,), Dynamic, _>(
                "cast",
                cast::<Profile, Parameter>,
            )
        })
}

pub(in crate::gleam_stdlib) enum DynamicSequence<'call> {
    List(HostList<'call, Dynamic>),
    Array(HostList<'call, Dynamic>),
}

#[cfg(test)]
mod tests {
    use super::host_provider;
    use crate::gleam_stdlib::GleamStdlibProfile;

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
