mod function;
mod parse;
mod schema;

use self::function::{
    FloatProvider, ceiling, do_log, do_power, do_to_float, exponential, floor, js_round, parse,
    random, to_string, truncate,
};
use self::schema::ParseResult;
use super::GleamStdlibHostProfile;
use crate::{HostProviderModule, HostRegistrationError};
use ecow::EcoString;
use num_bigint::BigInt;

pub(super) fn host_provider<Profile>() -> Result<HostProviderModule<Profile>, HostRegistrationError>
where
    Profile: GleamStdlibHostProfile,
{
    HostProviderModule::new("gleam_stdlib", "gleam/float")
        .and_then(|provider| {
            provider.with_scoped_function::<FloatProvider<Profile>, (EcoString,), ParseResult, _>(
                "parse",
                parse::<Profile>,
            )
        })
        .and_then(|provider| provider.with_function("to_string", to_string))
        .and_then(|provider| provider.with_function("ceiling", ceiling))
        .and_then(|provider| provider.with_function("floor", floor))
        .and_then(|provider| {
            provider.with_fallible_function::<(f64,), BigInt, _>("js_round", js_round)
        })
        .and_then(|provider| {
            provider.with_fallible_function::<(f64,), BigInt, _>("truncate", truncate)
        })
        .and_then(|provider| {
            provider.with_fallible_function::<(BigInt,), f64, _>("do_to_float", do_to_float)
        })
        .and_then(|provider| provider.with_function("do_power", do_power))
        .and_then(|provider| {
            provider.with_scoped_function::<FloatProvider<Profile>, (), f64, _>(
                "random",
                random::<Profile>,
            )
        })
        .and_then(|provider| provider.with_function("do_log", do_log))
        .and_then(|provider| provider.with_function("exponential", exponential))
}

#[cfg(test)]
mod tests {
    use super::host_provider;
    use crate::gleam_stdlib::GleamStdlibProfile;

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
