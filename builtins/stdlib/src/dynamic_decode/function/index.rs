use super::DynamicDecodeProvider;
use crate::GleamStdlibHostProfile;
use crate::dict;
use crate::dynamic::{self, Dynamic, DynamicSequence};
use crate::dynamic_decode::schema::{
    BareIndexConstructions, BareIndexDynamicIndex, BareIndexOptionIndex, DynamicDict, DynamicNone,
    DynamicSome, IndexError, IndexKey, IndexOk, IndexResult,
};
use crate::{
    HostCall, HostCallCompletion, HostCallError, HostConstructions, HostExternal, HostValue,
};
use ecow::EcoString;
use num_bigint::BigInt;
use num_traits::ToPrimitive;

pub(in crate::dynamic_decode) fn bare_index<'call, Profile>(
    mut call: HostCall<'call, Profile, DynamicDecodeProvider<Profile>, IndexResult>,
    constructions: HostConstructions<'call, BareIndexConstructions>,
    data: HostExternal<'call, Dynamic>,
    key: HostValue<'call, IndexKey>,
) -> Result<HostCallCompletion<'call, IndexResult>, HostCallError>
where
    Profile: GleamStdlibHostProfile,
{
    if let Some(dict) = dynamic::decode_value::<_, _, _, DynamicDict>(&mut call, data) {
        let key = dynamic::create_value::<_, _, _, IndexKey>(
            &mut call,
            constructions.at::<BareIndexDynamicIndex>(),
            key,
        );
        let value = dict::lookup::<_, _, _, Dynamic, Dynamic>(&mut call, dict, key);
        let value = match value {
            Some(value) => call.construct_custom::<DynamicSome>(
                constructions.at::<BareIndexOptionIndex>(),
                (value, ()),
            ),
            None => {
                call.construct_custom::<DynamicNone>(constructions.at::<BareIndexOptionIndex>(), ())
            }
        };
        return Ok(call.return_custom::<IndexOk>((value, ())));
    }

    let key = dynamic::create_value::<_, _, _, IndexKey>(
        &mut call,
        constructions.at::<BareIndexDynamicIndex>(),
        key,
    );
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
        Some(value) => call.construct_custom::<DynamicSome>(
            constructions.at::<BareIndexOptionIndex>(),
            (value, ()),
        ),
        None => {
            call.construct_custom::<DynamicNone>(constructions.at::<BareIndexOptionIndex>(), ())
        }
    };
    Ok(call.return_custom::<IndexOk>((value, ())))
}
