mod function;
mod parse;

pub(super) use self::function::do_to_float;

use super::{Component, GleamStdlibHostProfile, GleamStdlibRunState};
use crate::{HostProviderModule, HostRegistrationError};
use ecow::EcoString;
use geam_core::provider::{Call, HostResult};
use num_bigint::BigInt;

#[geam_macros::module(
    path = "gleam/float",
    crate_path = geam_core,
    profile = crate::GleamStdlibHostProfile,
    component = crate::Component<Profile::Io>,
)]
mod provider {
    use super::{BigInt, Call, EcoString, GleamStdlibRunState, HostResult, function};

    #[geam_macros::function]
    fn parse(source: EcoString) -> Result<f64, ()> {
        function::parse(source)
    }

    #[geam_macros::function]
    fn to_string(value: f64) -> EcoString {
        function::to_string(value)
    }

    #[geam_macros::function]
    fn ceiling(value: f64) -> f64 {
        function::ceiling(value)
    }

    #[geam_macros::function]
    fn floor(value: f64) -> f64 {
        function::floor(value)
    }

    #[geam_macros::function]
    fn js_round(value: f64) -> HostResult<BigInt> {
        function::js_round(value).map_err(Into::into)
    }

    #[geam_macros::function]
    fn truncate(value: f64) -> HostResult<BigInt> {
        function::truncate(value).map_err(Into::into)
    }

    #[geam_macros::function]
    fn do_to_float(value: BigInt) -> HostResult<f64> {
        function::do_to_float(value).map_err(Into::into)
    }

    #[geam_macros::function]
    fn do_power(base: f64, exponent: f64) -> f64 {
        function::do_power(base, exponent)
    }

    #[geam_macros::function(profile = Profile)]
    fn random(#[geam_macros::call] call: &mut Call<GleamStdlibRunState<Profile::Io>>) -> f64 {
        call.state_mut().random_float()
    }

    #[geam_macros::function]
    fn do_log(value: f64) -> f64 {
        function::do_log(value)
    }

    #[geam_macros::function]
    fn exponential(value: f64) -> f64 {
        function::exponential(value)
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
    fn registers_the_exact_official_float_provider_inventory() {
        let provider =
            host_provider::<GleamStdlibProfile>().expect("official float provider should register");

        assert_eq!(provider.package(), "gleam_stdlib");
        assert_eq!(provider.module(), "gleam/float");
        assert_eq!(
            provider
                .functions()
                .map(|function| function.name().as_str())
                .collect::<Vec<_>>(),
            [
                "parse",
                "to_string",
                "ceiling",
                "floor",
                "js_round",
                "truncate",
                "do_to_float",
                "do_power",
                "random",
                "do_log",
                "exponential",
            ],
        );
    }
}
