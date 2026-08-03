mod function;
mod schema;

pub(crate) use schema::DecodeError as DynamicDecodeError;

use self::function::{
    DynamicDecodeProvider, bare_index, cast, decode_dict, decode_list, dynamic_bit_array,
    dynamic_float, dynamic_int, dynamic_string, is_null,
};
use self::schema::{
    BitArrayResult, CastValue, DecodeListResult, DecodedItems, DictResult, IndexKey, IndexResult,
    ItemDecoder, PushPath, StringResult,
};
use super::GleamStdlibHostProfile;
use super::dynamic::Dynamic;
use crate::{HostProviderModule, HostRegistrationError};
use num_bigint::BigInt;

pub(super) fn host_provider<Profile>() -> Result<HostProviderModule<Profile>, HostRegistrationError>
where
    Profile: GleamStdlibHostProfile,
{
    HostProviderModule::new("gleam_stdlib", "gleam/dynamic/decode")
        .and_then(|provider| {
            provider.with_scoped_function::<
                DynamicDecodeProvider<Profile>,
                (Dynamic, IndexKey),
                IndexResult,
                _,
            >("bare_index", bare_index::<Profile>)
        })
        .and_then(|provider| {
            provider.with_scoped_function::<
                DynamicDecodeProvider<Profile>,
                (Dynamic,),
                StringResult,
                _,
            >("dynamic_string", dynamic_string::<Profile>)
        })
        .and_then(|provider| {
            provider.with_scoped_function::<
                DynamicDecodeProvider<Profile>,
                (Dynamic,),
                self::schema::IntResult,
                _,
            >("dynamic_int", dynamic_int::<Profile>)
        })
        .and_then(|provider| {
            provider.with_scoped_function::<
                DynamicDecodeProvider<Profile>,
                (Dynamic,),
                self::schema::FloatResult,
                _,
            >("dynamic_float", dynamic_float::<Profile>)
        })
        .and_then(|provider| {
            provider.with_scoped_function::<
                DynamicDecodeProvider<Profile>,
                (Dynamic,),
                BitArrayResult,
                _,
            >("dynamic_bit_array", dynamic_bit_array::<Profile>)
        })
        .and_then(|provider| {
            provider.with_scoped_function::<
                DynamicDecodeProvider<Profile>,
                (Dynamic, ItemDecoder, PushPath, BigInt, DecodedItems),
                DecodeListResult,
                _,
            >("decode_list", decode_list::<Profile>)
        })
        .and_then(|provider| {
            provider
                .with_scoped_function::<DynamicDecodeProvider<Profile>, (Dynamic,), DictResult, _>(
                    "decode_dict",
                    decode_dict::<Profile>,
                )
        })
        .and_then(|provider| {
            provider
                .with_scoped_function::<DynamicDecodeProvider<Profile>, (CastValue,), Dynamic, _>(
                    "cast",
                    cast::<Profile>,
                )
        })
        .and_then(|provider| {
            provider.with_scoped_function::<DynamicDecodeProvider<Profile>, (Dynamic,), bool, _>(
                "is_null",
                is_null::<Profile>,
            )
        })
}

#[cfg(test)]
mod tests {
    use super::host_provider;
    use crate::gleam_stdlib::GleamStdlibProfile;

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
