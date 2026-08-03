use super::JsonProvider;
use crate::gleam_json::GleamJsonHostProfile;
use crate::gleam_json::schema::{Json, ObjectEntry};
use crate::gleam_json::storage::JsonPayload;
use crate::gleam_stdlib::{StoredStringTree, StringTree, StringTreePayload};
use crate::{HostCall, HostCallCompletion, HostCallError, HostExternal, HostFailure, HostList};
use ecow::EcoString;
use num_bigint::BigInt;

pub(in crate::gleam_json) fn do_to_string<'call, Profile>(
    call: HostCall<'call, Profile, JsonProvider<Profile>, EcoString>,
    json: HostExternal<'call, Json>,
) -> Result<HostCallCompletion<'call, EcoString>, HostCallError>
where
    Profile: GleamJsonHostProfile,
{
    let text = call.external_payload(json).tree.flatten();
    Ok(call.return_value(text))
}

pub(in crate::gleam_json) fn to_string_tree<'call, Profile>(
    mut call: HostCall<'call, Profile, JsonProvider<Profile>, StringTree>,
    json: HostExternal<'call, Json>,
) -> Result<HostCallCompletion<'call, StringTree>, HostCallError>
where
    Profile: GleamJsonHostProfile,
{
    let tree = call.external_payload(json).tree.clone();
    let tree = call.create_external(StringTreePayload { tree });
    Ok(call.return_value(tree))
}

pub(in crate::gleam_json) fn do_string<'call, Profile>(
    mut call: HostCall<'call, Profile, JsonProvider<Profile>, Json>,
    value: EcoString,
) -> Result<HostCallCompletion<'call, Json>, HostCallError>
where
    Profile: GleamJsonHostProfile,
{
    let json = call.create_external(JsonPayload {
        tree: StoredStringTree::text(encode_string(&value)),
    });
    Ok(call.return_value(json))
}

pub(in crate::gleam_json) fn do_bool<'call, Profile>(
    mut call: HostCall<'call, Profile, JsonProvider<Profile>, Json>,
    value: bool,
) -> Result<HostCallCompletion<'call, Json>, HostCallError>
where
    Profile: GleamJsonHostProfile,
{
    let text = if value { "true" } else { "false" };
    let json = call.create_external(JsonPayload {
        tree: StoredStringTree::text(text.into()),
    });
    Ok(call.return_value(json))
}

pub(in crate::gleam_json) fn do_int<'call, Profile>(
    mut call: HostCall<'call, Profile, JsonProvider<Profile>, Json>,
    value: BigInt,
) -> Result<HostCallCompletion<'call, Json>, HostCallError>
where
    Profile: GleamJsonHostProfile,
{
    let json = call.create_external(JsonPayload {
        tree: StoredStringTree::text(value.to_string().into()),
    });
    Ok(call.return_value(json))
}

pub(in crate::gleam_json) fn do_float<'call, Profile>(
    mut call: HostCall<'call, Profile, JsonProvider<Profile>, Json>,
    value: f64,
) -> Result<HostCallCompletion<'call, Json>, HostCallError>
where
    Profile: GleamJsonHostProfile,
{
    if !value.is_finite() {
        return Err(HostFailure::new("JSON cannot encode a non-finite Float").into());
    }
    let json = call.create_external(JsonPayload {
        tree: StoredStringTree::text(encode_float(value).into()),
    });
    Ok(call.return_value(json))
}

pub(in crate::gleam_json) fn do_null<'call, Profile>(
    mut call: HostCall<'call, Profile, JsonProvider<Profile>, Json>,
) -> Result<HostCallCompletion<'call, Json>, HostCallError>
where
    Profile: GleamJsonHostProfile,
{
    let json = call.create_external(JsonPayload {
        tree: StoredStringTree::text("null".into()),
    });
    Ok(call.return_value(json))
}

pub(in crate::gleam_json) fn do_object<'call, Profile>(
    mut call: HostCall<'call, Profile, JsonProvider<Profile>, Json>,
    entries: HostList<'call, ObjectEntry>,
) -> Result<HostCallCompletion<'call, Json>, HostCallError>
where
    Profile: GleamJsonHostProfile,
{
    let mut index = 0;
    let mut trees = vec![StoredStringTree::text("{".into())];
    while let Some(entry) = call.list_item::<ObjectEntry>(entries, index) {
        let (key, (value, ())) = call.tuple_values(entry);
        if index != 0 {
            trees.push(StoredStringTree::text(",".into()));
        }
        trees.push(StoredStringTree::text(encode_string(&key)));
        trees.push(StoredStringTree::text(":".into()));
        trees.push(call.external_payload(value).tree.clone());
        index += 1;
    }
    trees.push(StoredStringTree::text("}".into()));
    let json = call.create_external(JsonPayload {
        tree: StoredStringTree::sequence(trees),
    });
    Ok(call.return_value(json))
}

pub(in crate::gleam_json) fn do_preprocessed_array<'call, Profile>(
    mut call: HostCall<'call, Profile, JsonProvider<Profile>, Json>,
    values: HostList<'call, Json>,
) -> Result<HostCallCompletion<'call, Json>, HostCallError>
where
    Profile: GleamJsonHostProfile,
{
    let mut index = 0;
    let mut trees = vec![StoredStringTree::text("[".into())];
    while let Some(value) = call.list_item(values, index) {
        if index != 0 {
            trees.push(StoredStringTree::text(",".into()));
        }
        trees.push(call.external_payload(value).tree.clone());
        index += 1;
    }
    trees.push(StoredStringTree::text("]".into()));
    let json = call.create_external(JsonPayload {
        tree: StoredStringTree::sequence(trees),
    });
    Ok(call.return_value(json))
}

fn encode_string(value: &str) -> EcoString {
    let mut output = String::with_capacity(value.len() + 2);
    output.push('"');
    for character in value.chars() {
        match character {
            '"' => output.push_str("\\\""),
            '\\' => output.push_str("\\\\"),
            '\u{0008}' => output.push_str("\\b"),
            '\t' => output.push_str("\\t"),
            '\n' => output.push_str("\\n"),
            '\u{000c}' => output.push_str("\\f"),
            '\r' => output.push_str("\\r"),
            character if character <= '\u{001f}' => {
                push_control_escape(&mut output, character);
            }
            character => output.push(character),
        }
    }
    output.push('"');
    output.into()
}

fn push_control_escape(output: &mut String, character: char) {
    const HEX: &[u8; 16] = b"0123456789ABCDEF";
    let value = u32::from(character) as usize;
    output.push_str("\\u00");
    output.push(char::from(HEX[(value >> 4) & 0x0f]));
    output.push(char::from(HEX[value & 0x0f]));
}

fn encode_float(value: f64) -> String {
    let mut buffer = ryu::Buffer::new();
    let encoded = buffer.format_finite(value);
    let Some(exponent_index) = encoded.find('e') else {
        return encoded.to_owned();
    };
    let (mantissa, exponent) = encoded.split_at(exponent_index);
    if mantissa.contains('.') {
        encoded.to_owned()
    } else {
        format!("{mantissa}.0{exponent}")
    }
}

#[cfg(test)]
mod tests {
    use super::{encode_float, encode_string};

    #[test]
    fn encodes_strings_with_the_otp_json_escape_set() {
        assert_eq!(
            encode_string("\"\\/\u{0008}\t\n\u{000c}\r\u{0000}\u{001f}é"),
            r#""\"\\/\b\t\n\f\r\u0000\u001Fé""#,
        );
    }

    #[test]
    fn normalizes_finite_float_exponents_to_the_otp_shape() {
        assert_eq!(encode_float(1.0), "1.0");
        assert_eq!(encode_float(-0.0), "-0.0");
        assert_eq!(encode_float(1.0e20), "1.0e20");
        assert_eq!(encode_float(1.0e-7), "1.0e-7");
        assert_eq!(encode_float(1.25e20), "1.25e20");
    }
}
