mod function;

pub use function::provider::__GeamCustomSchema0 as DynamicDecodeErrorSchema;

use super::GleamStdlibHostProfile;
use crate::{HostCustomType, HostProviderModule, HostRegistrationError};

pub type DynamicDecodeError = HostCustomType<DynamicDecodeErrorSchema>;

pub(super) fn host_provider<Profile>() -> Result<HostProviderModule<Profile>, HostRegistrationError>
where
    Profile: GleamStdlibHostProfile,
{
    function::host_provider::<Profile>()
}

#[cfg(test)]
mod tests {
    use super::host_provider;
    use crate::GleamStdlibProfile;

    #[test]
    fn registers_the_exact_official_dynamic_decode_provider_inventory() {
        let provider = host_provider::<GleamStdlibProfile>()
            .expect("official dynamic decode provider should register");

        assert_eq!(provider.package(), "gleam_stdlib");
        assert_eq!(provider.module(), "gleam/dynamic/decode");
        assert_eq!(provider.external_types().count(), 0);
        assert_eq!(
            provider
                .functions()
                .map(|function| function.name().as_str())
                .collect::<Vec<_>>(),
            [
                "bare_index",
                "dynamic_string",
                "dynamic_int",
                "dynamic_float",
                "dynamic_bit_array",
                "decode_list",
                "decode_dict",
                "cast",
                "is_null",
            ],
        );
    }
}
