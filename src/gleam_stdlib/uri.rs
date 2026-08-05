mod function;
mod schema;

use self::function::{
    UriProvider, codeunit_slice, parse_query, percent_decode, percent_encode, pop_codeunit,
};
use self::schema::{CodeunitPair, PercentDecodeResult, QueryConstructions, QueryResult};
use super::GleamStdlibHostProfile;
use crate::{HostProviderModule, HostRegistrationError};
use ecow::EcoString;
use num_bigint::BigInt;

pub(super) fn host_provider<Profile>() -> Result<HostProviderModule<Profile>, HostRegistrationError>
where
    Profile: GleamStdlibHostProfile,
{
    HostProviderModule::new("gleam_stdlib", "gleam/uri")
        .and_then(|provider| {
            provider.with_scoped_function::<UriProvider<Profile>, (EcoString,), CodeunitPair, _>(
                "pop_codeunit",
                pop_codeunit::<Profile>,
            )
        })
        .and_then(|provider| {
            provider.with_fallible_function::<(EcoString, BigInt, BigInt), EcoString, _>(
                "codeunit_slice",
                codeunit_slice,
            )
        })
        .and_then(|provider| {
            provider.with_scoped_function_and_constructions::<
                UriProvider<Profile>,
                (EcoString,),
                QueryResult,
                QueryConstructions,
                _,
            >("parse_query", parse_query::<Profile>)
        })
        .and_then(|provider| provider.with_function("percent_encode", percent_encode))
        .and_then(|provider| {
            provider
                .with_scoped_function::<UriProvider<Profile>, (EcoString,), PercentDecodeResult, _>(
                    "percent_decode",
                    percent_decode::<Profile>,
                )
        })
}

#[cfg(test)]
mod tests {
    use super::host_provider;
    use crate::ValueType;
    use crate::gleam_stdlib::GleamStdlibProfile;
    use crate::plan::{CustomType, CustomTypeName, FunctionType};

    #[test]
    fn registers_only_the_bodyless_official_uri_externals() {
        let provider =
            host_provider::<GleamStdlibProfile>().expect("official URI provider should register");

        assert_eq!(provider.package(), "gleam_stdlib");
        assert_eq!(provider.module(), "gleam/uri");
        assert_eq!(provider.external_types().count(), 0);
        let result = |success| {
            ValueType::Custom(CustomType::new(
                CustomTypeName::new("".into(), "gleam".into(), "Result".into()),
                vec![success, ValueType::Nil],
            ))
        };
        let expected = [
            (
                "pop_codeunit",
                FunctionType::new(
                    vec![ValueType::String],
                    ValueType::Tuple(vec![ValueType::Int, ValueType::String]),
                ),
            ),
            (
                "codeunit_slice",
                FunctionType::new(
                    vec![ValueType::String, ValueType::Int, ValueType::Int],
                    ValueType::String,
                ),
            ),
            (
                "parse_query",
                FunctionType::new(
                    vec![ValueType::String],
                    result(ValueType::List(Box::new(ValueType::Tuple(vec![
                        ValueType::String,
                        ValueType::String,
                    ])))),
                ),
            ),
            (
                "percent_encode",
                FunctionType::new(vec![ValueType::String], ValueType::String),
            ),
            (
                "percent_decode",
                FunctionType::new(vec![ValueType::String], result(ValueType::String)),
            ),
        ];
        let functions = provider.functions().collect::<Vec<_>>();

        assert_eq!(functions.len(), expected.len());
        for (function, (name, type_)) in functions.into_iter().zip(expected) {
            assert_eq!(function.name(), name);
            assert!(function.scheme().is_monomorphic());
            assert_eq!(function.type_(), &type_);
        }
    }
}
