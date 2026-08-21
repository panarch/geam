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

    fn assert_result(type_: &ValueType, success: ValueType) {
        let ValueType::Custom(type_) = type_ else {
            panic!("URI result should use the prelude Result type: {type_:?}");
        };

        assert_eq!(type_.type_name().package(), "");
        assert_eq!(type_.type_name().module(), "gleam");
        assert_eq!(type_.type_name().name(), "Result");
        assert_eq!(type_.arguments(), &[success, ValueType::Nil]);
    }

    #[test]
    #[should_panic(expected = "URI result should use the prelude Result type")]
    fn result_assertion_rejects_non_result_types() {
        assert_result(&ValueType::Nil, ValueType::String);
    }

    #[test]
    fn registers_only_the_bodyless_official_uri_externals() {
        let provider =
            host_provider::<GleamStdlibProfile>().expect("official URI provider should register");

        assert_eq!(provider.package(), "gleam_stdlib");
        assert_eq!(provider.module(), "gleam/uri");
        assert_eq!(provider.external_types().count(), 0);
        let functions = provider.functions().collect::<Vec<_>>();

        assert_eq!(functions.len(), 5);
        assert_eq!(
            functions
                .iter()
                .map(|function| function.name().as_str())
                .collect::<Vec<_>>(),
            [
                "pop_codeunit",
                "codeunit_slice",
                "parse_query",
                "percent_encode",
                "percent_decode",
            ],
        );
        for function in &functions {
            assert!(function.scheme().parameters().is_empty());
        }

        assert_eq!(functions[0].type_().argument_types(), [ValueType::String]);
        assert_eq!(
            functions[0].type_().return_(),
            &ValueType::Tuple(vec![ValueType::Int, ValueType::String]),
        );
        assert_eq!(
            functions[1].type_().argument_types(),
            [ValueType::String, ValueType::Int, ValueType::Int],
        );
        assert_eq!(functions[1].type_().return_(), &ValueType::String);
        assert_eq!(functions[2].type_().argument_types(), [ValueType::String]);
        assert_result(
            functions[2].type_().return_(),
            ValueType::List(Box::new(ValueType::Tuple(vec![
                ValueType::String,
                ValueType::String,
            ]))),
        );
        assert_eq!(functions[3].type_().argument_types(), [ValueType::String]);
        assert_eq!(functions[3].type_().return_(), &ValueType::String);
        assert_eq!(functions[4].type_().argument_types(), [ValueType::String]);
        assert_result(functions[4].type_().return_(), ValueType::String);
    }
}
