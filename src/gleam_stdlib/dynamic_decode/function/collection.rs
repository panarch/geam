use super::DynamicDecodeProvider;
use crate::gleam_stdlib::GleamStdlibHostProfile;
use crate::gleam_stdlib::dynamic::{self, Dynamic, DynamicSequence};
use crate::gleam_stdlib::dynamic_decode::schema::{
    DecodeErrorConstructor, DecodeListConstructions, DecodeListErrorIndex, DecodeListErrorsIndex,
    DecodeListPathIndex, DecodeListResult, DecodeListValuesIndex, DecodedItem, DictError, DictOk,
    DictResult, DynamicDict, ItemDecodeLayer, ItemDecoderArguments, PushPath,
};
use crate::{
    HostCall, HostCallCompletion, HostCallError, HostCallable, HostConstructions, HostExternal,
    HostList, HostValue,
};
use ecow::EcoString;
use geam_core::provider_support::sole_custom_fields;
use num_bigint::BigInt;

pub(in crate::gleam_stdlib::dynamic_decode) fn decode_list<'call, Profile>(
    mut call: HostCall<'call, Profile, DynamicDecodeProvider<Profile>, DecodeListResult>,
    constructions: HostConstructions<'call, DecodeListConstructions>,
    data: HostExternal<'call, Dynamic>,
    item: HostCallable<'call, ItemDecoderArguments, ItemDecodeLayer>,
    _push_path: HostValue<'call, PushPath>,
    mut index: BigInt,
    accumulator: HostList<'call, DecodedItem>,
) -> Result<HostCallCompletion<'call, DecodeListResult>, HostCallError>
where
    Profile: GleamStdlibHostProfile,
{
    let Some(sequence) = dynamic::sequence(&mut call, data) else {
        let found = dynamic::classification(&call, data);
        let path = call.construct_list(constructions.at::<DecodeListPathIndex>(), []);
        let error = call.construct_custom::<DecodeErrorConstructor>(
            constructions.at::<DecodeListErrorIndex>(),
            (EcoString::from("List"), (found, (path, ()))),
        );
        let values = call.construct_list(constructions.at::<DecodeListValuesIndex>(), []);
        let errors = call.construct_list(constructions.at::<DecodeListErrorsIndex>(), [error]);
        return Ok(call.return_tuple((values, (errors, ()))));
    };

    let values = match sequence {
        DynamicSequence::List(values) | DynamicSequence::Array(values) => values,
    };
    let mut decoded = Vec::<HostValue<'call, DecodedItem>>::new();
    let mut accumulator_index = 0;
    while let Some(value) = call.list_item(accumulator, accumulator_index) {
        decoded.push(value);
        accumulator_index += 1;
    }
    decoded.reverse();

    let mut value_index = 0;
    while let Some(value) = call.list_item(values, value_index) {
        let layer = call.invoke(item, (value, ()))?;
        let (value, (errors, ())) = call.tuple_values(layer);
        if call.list_len(errors) != 0 {
            let mut updated_errors = Vec::new();
            let mut error_index = 0;
            while let Some(error) = call.list_item(errors, error_index) {
                let (expected, (found, (path, ()))) =
                    sole_custom_fields::<_, _, _, DecodeErrorConstructor>(&mut call, error);
                let mut updated_path = vec![EcoString::from(index.to_string())];
                let mut path_index = 0;
                while let Some(segment) = call.list_item(path, path_index) {
                    updated_path.push(segment);
                    path_index += 1;
                }
                let path =
                    call.construct_list(constructions.at::<DecodeListPathIndex>(), updated_path);
                updated_errors.push(call.construct_custom::<DecodeErrorConstructor>(
                    constructions.at::<DecodeListErrorIndex>(),
                    (expected, (found, (path, ()))),
                ));
                error_index += 1;
            }
            let empty = call.construct_list(constructions.at::<DecodeListValuesIndex>(), []);
            let errors =
                call.construct_list(constructions.at::<DecodeListErrorsIndex>(), updated_errors);
            return Ok(call.return_tuple((empty, (errors, ()))));
        }
        decoded.push(value);
        value_index += 1;
        index += 1;
    }

    let values = call.construct_list(constructions.at::<DecodeListValuesIndex>(), decoded);
    let errors = call.construct_list(constructions.at::<DecodeListErrorsIndex>(), []);
    Ok(call.return_tuple((values, (errors, ()))))
}

pub(in crate::gleam_stdlib::dynamic_decode) fn decode_dict<'call, Profile>(
    mut call: HostCall<'call, Profile, DynamicDecodeProvider<Profile>, DictResult>,
    value: HostExternal<'call, Dynamic>,
) -> Result<HostCallCompletion<'call, DictResult>, HostCallError>
where
    Profile: GleamStdlibHostProfile,
{
    match dynamic::decode_value::<_, _, _, DynamicDict>(&mut call, value) {
        Some(value) => Ok(call.return_custom::<DictOk>((value, ()))),
        None => Ok(call.return_custom::<DictError>(((), ()))),
    }
}
