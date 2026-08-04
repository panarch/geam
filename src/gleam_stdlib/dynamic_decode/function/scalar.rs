use super::DynamicDecodeProvider;
use crate::gleam_stdlib::GleamStdlibHostProfile;
use crate::gleam_stdlib::dynamic::{self, Dynamic};
use crate::gleam_stdlib::dynamic_decode::schema::{
    BitArrayError, BitArrayOk, BitArrayResult, CastValue, FloatError, FloatOk, FloatResult,
    IntError, IntOk, IntResult, StringError, StringOk, StringResult,
};
use crate::{BitArrayValue, HostCall, HostCallCompletion, HostCallError, HostExternal, HostValue};
use ecow::EcoString;
use num_bigint::BigInt;

pub(in crate::gleam_stdlib::dynamic_decode) fn dynamic_string<'call, Profile>(
    mut call: HostCall<'call, Profile, DynamicDecodeProvider<Profile>, StringResult>,
    value: HostExternal<'call, Dynamic>,
) -> Result<HostCallCompletion<'call, StringResult>, HostCallError>
where
    Profile: GleamStdlibHostProfile,
{
    match dynamic::decode_value::<_, _, _, EcoString>(&mut call, value) {
        Some(value) => Ok(call.return_custom::<StringOk>((value, ()))),
        None => Ok(call.return_custom::<StringError>((EcoString::new(), ()))),
    }
}

pub(in crate::gleam_stdlib::dynamic_decode) fn dynamic_int<'call, Profile>(
    mut call: HostCall<'call, Profile, DynamicDecodeProvider<Profile>, IntResult>,
    value: HostExternal<'call, Dynamic>,
) -> Result<HostCallCompletion<'call, IntResult>, HostCallError>
where
    Profile: GleamStdlibHostProfile,
{
    match dynamic::decode_value::<_, _, _, BigInt>(&mut call, value) {
        Some(value) => Ok(call.return_custom::<IntOk>((value, ()))),
        None => Ok(call.return_custom::<IntError>((BigInt::from(0), ()))),
    }
}

pub(in crate::gleam_stdlib::dynamic_decode) fn dynamic_float<'call, Profile>(
    mut call: HostCall<'call, Profile, DynamicDecodeProvider<Profile>, FloatResult>,
    value: HostExternal<'call, Dynamic>,
) -> Result<HostCallCompletion<'call, FloatResult>, HostCallError>
where
    Profile: GleamStdlibHostProfile,
{
    match dynamic::decode_value::<_, _, _, f64>(&mut call, value) {
        Some(value) => Ok(call.return_custom::<FloatOk>((value, ()))),
        None => Ok(call.return_custom::<FloatError>((0.0, ()))),
    }
}

pub(in crate::gleam_stdlib::dynamic_decode) fn dynamic_bit_array<'call, Profile>(
    mut call: HostCall<'call, Profile, DynamicDecodeProvider<Profile>, BitArrayResult>,
    value: HostExternal<'call, Dynamic>,
) -> Result<HostCallCompletion<'call, BitArrayResult>, HostCallError>
where
    Profile: GleamStdlibHostProfile,
{
    match dynamic::decode_value::<_, _, _, BitArrayValue>(&mut call, value) {
        Some(value) => Ok(call.return_custom::<BitArrayOk>((value, ()))),
        None => {
            Ok(call.return_custom::<BitArrayError>((BitArrayValue::from_bytes(Vec::new()), ())))
        }
    }
}

pub(in crate::gleam_stdlib::dynamic_decode) fn cast<'call, Profile>(
    mut call: HostCall<'call, Profile, DynamicDecodeProvider<Profile>, Dynamic>,
    value: HostValue<'call, CastValue>,
) -> Result<HostCallCompletion<'call, Dynamic>, HostCallError>
where
    Profile: GleamStdlibHostProfile,
{
    let value = dynamic::create_return_value::<_, _, CastValue>(&mut call, value);
    Ok(call.return_value(value))
}

pub(in crate::gleam_stdlib::dynamic_decode) fn is_null<'call, Profile>(
    mut call: HostCall<'call, Profile, DynamicDecodeProvider<Profile>, bool>,
    value: HostExternal<'call, Dynamic>,
) -> Result<HostCallCompletion<'call, bool>, HostCallError>
where
    Profile: GleamStdlibHostProfile,
{
    let is_null = dynamic::decode_value::<_, _, _, ()>(&mut call, value).is_some();
    Ok(call.return_value(is_null))
}
