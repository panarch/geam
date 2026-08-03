use super::DynamicDecodeProvider;
use crate::gleam_stdlib::GleamStdlibHostProfile;
use crate::gleam_stdlib::dict;
use crate::gleam_stdlib::dynamic::{self, Dynamic, DynamicSequence};
use crate::gleam_stdlib::dynamic_decode::schema::{
    DynamicDict, DynamicNone, DynamicSome, IndexError, IndexKey, IndexOk, IndexResult,
};
use crate::{HostCall, HostCallCompletion, HostCallError, HostExternal, HostValue};
use ecow::EcoString;
use num_bigint::BigInt;
use num_traits::ToPrimitive;

pub(in crate::gleam_stdlib::dynamic_decode) fn bare_index<'call, Profile>(
    mut call: HostCall<'call, Profile, DynamicDecodeProvider<Profile>, IndexResult>,
    data: HostExternal<'call, Dynamic>,
    key: HostValue<'call, IndexKey>,
) -> Result<HostCallCompletion<'call, IndexResult>, HostCallError>
where
    Profile: GleamStdlibHostProfile,
{
    if let Some(dict) = dynamic::decode_value::<_, _, _, DynamicDict>(&mut call, data) {
        let key = dynamic::create_value::<_, _, _, IndexKey>(&mut call, key);
        let value = dict::lookup::<_, _, _, Dynamic, Dynamic>(&mut call, dict, key);
        let value = match value {
            Some(value) => call.create_custom::<DynamicSome>((value, ())),
            None => call.create_custom::<DynamicNone>(()),
        };
        return Ok(call.return_custom::<IndexOk>((value, ())));
    }

    let key = dynamic::create_value::<_, _, _, IndexKey>(&mut call, key);
    let Some(index) = dynamic::decode_value::<_, _, _, BigInt>(&mut call, key) else {
        return Ok(call.return_custom::<IndexError>((EcoString::from("Dict"), ())));
    };
    let Some(sequence) = dynamic::sequence(&mut call, data) else {
        return Ok(call.return_custom::<IndexError>((EcoString::from("Indexable"), ())));
    };

    let index = index.to_usize();
    let value = match sequence {
        DynamicSequence::List(values) => {
            let Some(index) = index.filter(|index| *index < 8) else {
                return Ok(call.return_custom::<IndexError>((EcoString::from("Indexable"), ())));
            };
            let Some(value) = call.list_item(values, index) else {
                return Ok(call.return_custom::<IndexError>((EcoString::from("Indexable"), ())));
            };
            Some(value)
        }
        DynamicSequence::Array(values) => index.and_then(|index| call.list_item(values, index)),
    };
    let value = match value {
        Some(value) => call.create_custom::<DynamicSome>((value, ())),
        None => call.create_custom::<DynamicNone>(()),
    };
    Ok(call.return_custom::<IndexOk>((value, ())))
}
