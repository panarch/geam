mod function;

use super::{Component, GleamStdlibHostProfile};
use crate::{HostProviderModule, HostRegistrationError};
use ecow::EcoString;
use geam_core::provider::HostResult;
use num_bigint::BigInt;

#[geam_macros::module(
    path = "gleam/uri",
    crate_path = geam_core,
    profile = crate::GleamStdlibHostProfile,
    component = crate::Component<Profile::Io>,
)]
mod provider {
    use super::{BigInt, EcoString, HostResult, function};

    #[geam_macros::function]
    fn pop_codeunit(string: EcoString) -> (BigInt, EcoString) {
        function::pop_codeunit(string)
    }

    #[geam_macros::function]
    fn codeunit_slice(string: EcoString, from: BigInt, length: BigInt) -> HostResult<EcoString> {
        function::codeunit_slice(string, from, length).map_err(Into::into)
    }

    #[geam_macros::function]
    fn parse_query(query: EcoString) -> Result<Vec<(EcoString, EcoString)>, ()> {
        function::parse_query(query)
    }

    #[geam_macros::function]
    fn percent_encode(value: EcoString) -> EcoString {
        function::percent_encode(value)
    }

    #[geam_macros::function]
    fn percent_decode(value: EcoString) -> Result<EcoString, ()> {
        function::percent_decode(value)
    }
}

pub(super) fn host_provider<Profile>() -> Result<HostProviderModule<Profile>, HostRegistrationError>
where
    Profile: GleamStdlibHostProfile,
{
    <provider::__GeamModule as geam_core::__macro_support::ProviderModuleRegistration<
        Profile,
    >>::module()
}

#[cfg(test)]
mod tests {
    use super::host_provider;
    use crate::GleamStdlibProfile;
    use crate::ValueType;

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
