mod error;

use self::error::DecodeFailure;
use super::JsonProvider;
use crate::gleam_json::GleamJsonHostProfile;
use crate::gleam_json::schema::{
    DecodeConstructions, DecodeDictIndex, DecodeDynamicIndex, DecodeErrorIndex, DecodeListIndex,
    DynamicDict, DynamicList, JsonDynamicError, JsonDynamicOk, JsonDynamicResult, UnexpectedByte,
    UnexpectedEndOfInput, UnexpectedSequence,
};
use crate::{
    BitArrayValue, HostCall, HostCallCompletion, HostCallError, HostConstructions, HostExternal,
    HostList,
};
use ecow::EcoString;
use geam_stdlib::provider_support::{Dynamic, create_dynamic_dict, create_dynamic_value};
use jiter::{Jiter, Peek};
use num_bigint::BigInt;

pub(in crate::gleam_json) fn decode_to_dynamic<'call, Profile>(
    mut call: HostCall<'call, Profile, JsonProvider<Profile>, JsonDynamicResult>,
    constructions: HostConstructions<'call, DecodeConstructions>,
    json: BitArrayValue,
) -> Result<HostCallCompletion<'call, JsonDynamicResult>, HostCallError>
where
    Profile: GleamJsonHostProfile,
{
    let decoded = if json.bit_len().is_multiple_of(8) {
        parse_dynamic(&mut call, &constructions, json.bytes())
    } else {
        Err(DecodeFailure::Byte(EcoString::new()))
    };

    match decoded {
        Ok(value) => Ok(call.return_custom::<JsonDynamicOk>((value, ()))),
        Err(DecodeFailure::EndOfInput) => {
            let error = call.construct_custom::<UnexpectedEndOfInput>(
                constructions.at::<DecodeErrorIndex>(),
                (),
            );
            Ok(call.return_custom::<JsonDynamicError>((error, ())))
        }
        Err(DecodeFailure::Byte(byte)) => {
            let error = call.construct_custom::<UnexpectedByte>(
                constructions.at::<DecodeErrorIndex>(),
                (byte, ()),
            );
            Ok(call.return_custom::<JsonDynamicError>((error, ())))
        }
        Err(DecodeFailure::Sequence(sequence)) => {
            let error = call.construct_custom::<UnexpectedSequence>(
                constructions.at::<DecodeErrorIndex>(),
                (sequence, ()),
            );
            Ok(call.return_custom::<JsonDynamicError>((error, ())))
        }
    }
}

enum ParseFrame<'call> {
    Array(Vec<HostExternal<'call, Dynamic>>),
    Object {
        entries: Vec<(EcoString, HostExternal<'call, Dynamic>)>,
        pending_key: EcoString,
    },
}

fn parse_dynamic<'call, Profile>(
    call: &mut HostCall<'call, Profile, JsonProvider<Profile>, JsonDynamicResult>,
    constructions: &HostConstructions<'call, DecodeConstructions>,
    input: &[u8],
) -> Result<HostExternal<'call, Dynamic>, DecodeFailure>
where
    Profile: GleamJsonHostProfile,
{
    let mut parser = Jiter::new(input);
    let mut frames = Vec::new();
    let mut next = parser
        .peek()
        .map_err(|error| DecodeFailure::from_jiter(input, error))?;

    'parse: loop {
        let mut value = if next == Peek::Null {
            parser
                .known_null()
                .map_err(|error| DecodeFailure::from_jiter(input, error))?;
            create_dynamic_value::<Profile, JsonProvider<Profile>, JsonDynamicResult, ()>(
                call,
                constructions.at::<DecodeDynamicIndex>(),
                (),
            )
        } else if matches!(next, Peek::True | Peek::False) {
            let value = parser
                .known_bool(next)
                .map_err(|error| DecodeFailure::from_jiter(input, error))?;
            create_dynamic_value::<Profile, JsonProvider<Profile>, JsonDynamicResult, bool>(
                call,
                constructions.at::<DecodeDynamicIndex>(),
                value,
            )
        } else if next == Peek::String {
            let value = parser
                .known_str()
                .map(EcoString::from)
                .map_err(|error| DecodeFailure::from_jiter(input, error))?;
            create_dynamic_value::<Profile, JsonProvider<Profile>, JsonDynamicResult, EcoString>(
                call,
                constructions.at::<DecodeDynamicIndex>(),
                value,
            )
        } else if next == Peek::Array {
            match parser
                .known_array()
                .map_err(|error| DecodeFailure::from_jiter(input, error))?
            {
                Some(first) => {
                    frames.push(ParseFrame::Array(Vec::new()));
                    next = first;
                    continue 'parse;
                }
                None => create_dynamic_list(call, constructions, Vec::new()),
            }
        } else if next == Peek::Object {
            match parser
                .known_object()
                .map(|key| key.map(EcoString::from))
                .map_err(|error| DecodeFailure::from_jiter(input, error))?
            {
                Some(pending_key) => {
                    frames.push(ParseFrame::Object {
                        entries: Vec::new(),
                        pending_key,
                    });
                    next = parser
                        .peek()
                        .map_err(|error| DecodeFailure::from_jiter(input, error))?;
                    continue 'parse;
                }
                None => create_dynamic_object(call, constructions, Vec::new()),
            }
        } else if next.is_num() {
            let number = parser
                .known_number_bytes(next)
                .map_err(|error| DecodeFailure::from_jiter(input, error))?;
            create_dynamic_number(call, constructions, number)?
        } else {
            return Err(DecodeFailure::Byte(
                format!("0x{:02X}", next.into_inner()).into(),
            ));
        };

        loop {
            value = match frames.pop() {
                None => {
                    parser
                        .finish()
                        .map_err(|error| DecodeFailure::from_jiter(input, error))?;
                    return Ok(value);
                }
                Some(ParseFrame::Array(mut values)) => {
                    values.push(value);
                    match parser
                        .array_step()
                        .map_err(|error| DecodeFailure::from_jiter(input, error))?
                    {
                        Some(peek) => {
                            frames.push(ParseFrame::Array(values));
                            next = peek;
                            continue 'parse;
                        }
                        None => create_dynamic_list(call, constructions, values),
                    }
                }
                Some(ParseFrame::Object {
                    mut entries,
                    pending_key,
                }) => {
                    entries.push((pending_key, value));
                    match parser
                        .next_key()
                        .map(|key| key.map(EcoString::from))
                        .map_err(|error| DecodeFailure::from_jiter(input, error))?
                    {
                        Some(next_key) => {
                            frames.push(ParseFrame::Object {
                                entries,
                                pending_key: next_key,
                            });
                            next = parser
                                .peek()
                                .map_err(|error| DecodeFailure::from_jiter(input, error))?;
                            continue 'parse;
                        }
                        None => create_dynamic_object(call, constructions, entries),
                    }
                }
            };
        }
    }
}

enum ParsedNumber {
    Int(BigInt),
    Float(f64),
}

fn create_dynamic_number<'call, Profile>(
    call: &mut HostCall<'call, Profile, JsonProvider<Profile>, JsonDynamicResult>,
    constructions: &HostConstructions<'call, DecodeConstructions>,
    number: &[u8],
) -> Result<HostExternal<'call, Dynamic>, DecodeFailure>
where
    Profile: GleamJsonHostProfile,
{
    match parse_number(number)? {
        ParsedNumber::Int(value) => Ok(create_dynamic_value::<
            Profile,
            JsonProvider<Profile>,
            JsonDynamicResult,
            BigInt,
        >(
            call, constructions.at::<DecodeDynamicIndex>(), value
        )),
        ParsedNumber::Float(value) => Ok(create_dynamic_value::<
            Profile,
            JsonProvider<Profile>,
            JsonDynamicResult,
            f64,
        >(
            call, constructions.at::<DecodeDynamicIndex>(), value
        )),
    }
}

fn parse_number(number: &[u8]) -> Result<ParsedNumber, DecodeFailure> {
    if !number.contains(&b'.') && !number.contains(&b'e') && !number.contains(&b'E') {
        let Some(value) = BigInt::parse_bytes(number, 10) else {
            return Err(DecodeFailure::Byte(
                number
                    .first()
                    .map_or_else(EcoString::new, |byte| format!("0x{byte:02X}").into()),
            ));
        };
        return Ok(ParsedNumber::Int(value));
    }

    let Ok(text) = std::str::from_utf8(number) else {
        return Err(DecodeFailure::Byte(EcoString::new()));
    };
    let Ok(value) = text.parse::<f64>() else {
        return Err(DecodeFailure::overflow(number));
    };
    if !value.is_finite() {
        return Err(DecodeFailure::overflow(number));
    }
    Ok(ParsedNumber::Float(value))
}

fn create_dynamic_list<'call, Profile>(
    call: &mut HostCall<'call, Profile, JsonProvider<Profile>, JsonDynamicResult>,
    constructions: &HostConstructions<'call, DecodeConstructions>,
    values: Vec<HostExternal<'call, Dynamic>>,
) -> HostExternal<'call, Dynamic>
where
    Profile: GleamJsonHostProfile,
{
    let values: HostList<'call, Dynamic> =
        call.construct_list(constructions.at::<DecodeListIndex>(), values);
    create_dynamic_value::<Profile, JsonProvider<Profile>, JsonDynamicResult, DynamicList>(
        call,
        constructions.at::<DecodeDynamicIndex>(),
        values,
    )
}

fn create_dynamic_object<'call, Profile>(
    call: &mut HostCall<'call, Profile, JsonProvider<Profile>, JsonDynamicResult>,
    constructions: &HostConstructions<'call, DecodeConstructions>,
    entries: Vec<(EcoString, HostExternal<'call, Dynamic>)>,
) -> HostExternal<'call, Dynamic>
where
    Profile: GleamJsonHostProfile,
{
    let entries = entries
        .into_iter()
        .map(|(key, value)| {
            let key = create_dynamic_value::<
                Profile,
                JsonProvider<Profile>,
                JsonDynamicResult,
                EcoString,
            >(call, constructions.at::<DecodeDynamicIndex>(), key);
            (key, value)
        })
        .collect::<Vec<_>>();
    let dict = create_dynamic_dict(call, constructions.at::<DecodeDictIndex>(), entries);
    create_dynamic_value::<Profile, JsonProvider<Profile>, JsonDynamicResult, DynamicDict>(
        call,
        constructions.at::<DecodeDynamicIndex>(),
        dict,
    )
}

#[cfg(test)]
mod tests {
    use super::{DecodeFailure, ParsedNumber, parse_number};
    use crate::gleam_json::test_support::{execution, run_state};

    #[test]
    fn parses_validated_number_tokens_and_maps_defensive_failures() {
        assert!(
            matches!(parse_number(b"-123"), Ok(ParsedNumber::Int(value)) if value == (-123).into())
        );
        assert!(matches!(
            parse_number(b"1.25"),
            Ok(ParsedNumber::Float(1.25))
        ));
        assert!(matches!(parse_number(b""), Err(DecodeFailure::Byte(byte)) if byte.is_empty()));
        assert!(matches!(parse_number(b"x"), Err(DecodeFailure::Byte(byte)) if byte == "0x78"));
        assert!(
            matches!(parse_number(b"\xFF."), Err(DecodeFailure::Byte(byte)) if byte.is_empty())
        );
        assert!(
            matches!(parse_number(b"x."), Err(DecodeFailure::Sequence(sequence)) if sequence == "x.")
        );
        assert!(
            matches!(parse_number(b"1e400"), Err(DecodeFailure::Sequence(sequence)) if sequence == "1.0e400")
        );
    }

    #[test]
    fn executes_scalar_decoding_through_the_hosted_pipeline() {
        let execution = execution(
            r#"
pub fn main() {
  #(
    decode_to_dynamic(<<"null":utf8>>),
    decode_to_dynamic(<<"true":utf8>>),
    decode_to_dynamic(<<"\"text\"":utf8>>),
    decode_to_dynamic(<<"123456789012345678901234567890":utf8>>),
    decode_to_dynamic(<<"1.25":utf8>>),
    decode_to_dynamic(<<"[]":utf8>>),
    decode_to_dynamic(<<"{}":utf8>>),
  )
}
"#,
        );
        let value = execution
            .run_main(&mut run_state([0; 32]), &mut Vec::new())
            .expect("scalar JSON decoding should run");

        assert_eq!(
            value.inspect().to_string(),
            r#"#(Ok(Nil), Ok(True), Ok("text"), Ok(123456789012345678901234567890), Ok(1.25), Ok([]), Ok(dict.from_list([])))"#,
        );
    }

    #[test]
    fn constructs_nested_dynamic_collections_through_the_hosted_pipeline() {
        let execution = execution(
            r#"
pub fn main() {
  decode_to_dynamic(<<"[1,{\"a\":true,\"a\":false}]":utf8>>)
}
"#,
        );
        let value = execution
            .run_main(&mut run_state([0; 32]), &mut Vec::new())
            .expect("nested JSON decoding should run");

        assert_eq!(
            value.inspect().to_string(),
            r#"Ok([1, dict.from_list([#("a", True)])])"#,
        );
    }

    #[test]
    fn deeply_nested_json_parses_and_releases_without_rust_stack_recursion() {
        let depth = 5_000;
        let json = format!("{}0{}", "[".repeat(depth), "]".repeat(depth));
        let source = format!(
            r#"
pub fn main() {{
  case decode_to_dynamic(<<"{json}":utf8>>) {{
    Ok(_) -> Nil
    Error(_) -> Nil
  }}
}}
"#,
        );
        let execution = execution(&source);

        assert_eq!(
            execution.run_main(&mut run_state([0; 32]), &mut Vec::new(),),
            Ok(crate::Value::Nil),
        );
    }
}
