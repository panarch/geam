mod function;
mod parse;
mod schema;

use self::function::{
    IntProvider, bitwise_and, bitwise_exclusive_or, bitwise_not, bitwise_or, bitwise_shift_left,
    bitwise_shift_right, do_base_parse, do_to_base_string, parse, to_float, to_string,
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
    HostProviderModule::new("gleam_stdlib", "gleam/int")
        .and_then(|provider| {
            provider.with_scoped_function::<IntProvider<Profile>, (EcoString,), ParseResult, _>(
                "parse",
                parse::<Profile>,
            )
        })
        .and_then(|provider| {
            provider
                .with_scoped_function::<IntProvider<Profile>, (EcoString, BigInt), ParseResult, _>(
                    "do_base_parse",
                    do_base_parse::<Profile>,
                )
        })
        .and_then(|provider| provider.with_function("to_string", to_string))
        .and_then(|provider| {
            provider.with_fallible_function::<(BigInt, BigInt), EcoString, _>(
                "do_to_base_string",
                do_to_base_string,
            )
        })
        .and_then(|provider| {
            provider.with_fallible_function::<(BigInt,), f64, _>("to_float", to_float)
        })
        .and_then(|provider| provider.with_function("bitwise_and", bitwise_and))
        .and_then(|provider| provider.with_function("bitwise_not", bitwise_not))
        .and_then(|provider| provider.with_function("bitwise_or", bitwise_or))
        .and_then(|provider| provider.with_function("bitwise_exclusive_or", bitwise_exclusive_or))
        .and_then(|provider| {
            provider.with_fallible_function::<(BigInt, BigInt), BigInt, _>(
                "bitwise_shift_left",
                bitwise_shift_left,
            )
        })
        .and_then(|provider| {
            provider.with_fallible_function::<(BigInt, BigInt), BigInt, _>(
                "bitwise_shift_right",
                bitwise_shift_right,
            )
        })
}

#[cfg(test)]
mod tests {
    use super::host_provider;
    use crate::gleam_stdlib::GleamStdlibProfile;

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
