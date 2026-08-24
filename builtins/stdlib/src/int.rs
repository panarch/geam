mod function;
mod parse;

use super::{Component, GleamStdlibHostProfile};
use crate::{HostProviderModule, HostRegistrationError};
use ecow::EcoString;
use geam_core::provider::HostResult;
use num_bigint::BigInt;

#[geam_macros::module(
    path = "gleam/int",
    crate_path = geam_core,
    profile = crate::GleamStdlibHostProfile,
    component = crate::Component<Profile::Io>,
)]
mod provider {
    use super::{BigInt, EcoString, HostResult, function};

    #[geam_macros::function]
    fn parse(source: EcoString) -> Result<BigInt, ()> {
        function::parse(source)
    }

    #[geam_macros::function]
    fn do_base_parse(source: EcoString, base: BigInt) -> Result<BigInt, ()> {
        function::do_base_parse(source, base)
    }

    #[geam_macros::function]
    fn to_string(value: BigInt) -> EcoString {
        function::to_string(value)
    }

    #[geam_macros::function]
    fn do_to_base_string(value: BigInt, base: BigInt) -> HostResult<EcoString> {
        function::do_to_base_string(value, base).map_err(Into::into)
    }

    #[geam_macros::function]
    fn to_float(value: BigInt) -> HostResult<f64> {
        function::to_float(value).map_err(Into::into)
    }

    #[geam_macros::function]
    fn bitwise_and(left: BigInt, right: BigInt) -> BigInt {
        function::bitwise_and(left, right)
    }

    #[geam_macros::function]
    fn bitwise_not(value: BigInt) -> BigInt {
        function::bitwise_not(value)
    }

    #[geam_macros::function]
    fn bitwise_or(left: BigInt, right: BigInt) -> BigInt {
        function::bitwise_or(left, right)
    }

    #[geam_macros::function]
    fn bitwise_exclusive_or(left: BigInt, right: BigInt) -> BigInt {
        function::bitwise_exclusive_or(left, right)
    }

    #[geam_macros::function]
    fn bitwise_shift_left(value: BigInt, shift: BigInt) -> HostResult<BigInt> {
        function::bitwise_shift_left(value, shift).map_err(Into::into)
    }

    #[geam_macros::function]
    fn bitwise_shift_right(value: BigInt, shift: BigInt) -> HostResult<BigInt> {
        function::bitwise_shift_right(value, shift).map_err(Into::into)
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

    #[test]
    fn registers_the_exact_official_int_provider_inventory() {
        let provider =
            host_provider::<GleamStdlibProfile>().expect("official int provider should register");

        assert_eq!(provider.package(), "gleam_stdlib");
        assert_eq!(provider.module(), "gleam/int");
        assert_eq!(
            provider
                .functions()
                .map(|function| function.name().as_str())
                .collect::<Vec<_>>(),
            [
                "parse",
                "do_base_parse",
                "to_string",
                "do_to_base_string",
                "to_float",
                "bitwise_and",
                "bitwise_not",
                "bitwise_or",
                "bitwise_exclusive_or",
                "bitwise_shift_left",
                "bitwise_shift_right",
            ],
        );
    }
}
