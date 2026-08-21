use super::JsonProvider;
use crate::gleam_json::GleamJsonHostProfile;
use crate::gleam_json::schema::{Json, ObjectEntry};
use crate::gleam_json::storage::JsonPayload;
use crate::{HostCall, HostCallCompletion, HostCallError, HostExternal, HostFailure, HostList};
use ecow::EcoString;
use geam_stdlib::provider_support::{StoredStringTree, StringTree, StringTreePayload};
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
    let tree = call.create_external(StringTreePayload::from_stored(tree));
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
    use crate::gleam_json::GleamJsonProfile;
    use crate::gleam_json::test_support::{execution, execution_with_modules, run_state};
    use crate::{ExecutionError, HostError, HostModule, InvariantError, ValueType};

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

    #[test]
    fn executes_scalar_encoding_through_the_hosted_pipeline() {
        let execution = execution(
            r#"
pub fn main() {
  #(
    do_to_string(do_string("a\"\n\u{0000}é")),
    do_to_string(do_bool(True)),
    do_to_string(do_bool(False)),
    do_to_string(do_int(123456789012345678901234567890)),
    do_to_string(do_float(1.0e20)),
    do_to_string(do_null()),
    do_to_string(do_preprocessed_array([])),
    do_to_string(do_object([])),
  )
}
"#,
        );
        let value = execution
            .run_main(&mut run_state([0; 32]), &mut Vec::new())
            .expect("scalar JSON encoding should run");

        assert_eq!(
            value.inspect().to_string(),
            r#"#("\"a\\\"\\n\\u0000é\"", "true", "false", "123456789012345678901234567890", "1.0e20", "null", "[]", "{}")"#,
        );
    }

    #[test]
    fn constructs_nested_json_and_shares_string_tree_output() {
        let execution = execution(
            r#"
pub fn main() {
  #(
    do_to_string(do_object([
      #("a", do_int(1)),
      #("a", do_int(2)),
      #("b", do_preprocessed_array([do_bool(True), do_null()])),
    ])),
    to_string_tree(do_preprocessed_array([do_int(1), do_string("two")])),
  )
}
"#,
        );
        let value = execution
            .run_main(&mut run_state([0; 32]), &mut Vec::new())
            .expect("nested JSON construction should run");

        assert_eq!(
            value.inspect().to_string(),
            r#"#("{\"a\":1,\"a\":2,\"b\":[true,null]}", string_tree.from_string("[1,\"two\"]"))"#,
        );
    }

    #[test]
    fn rejects_non_finite_float_encoding_as_the_json_host_function() {
        for (name, value) in [("infinity", f64::INFINITY), ("nan", f64::NAN)] {
            let non_finite =
                HostModule::<GleamJsonProfile>::new_for_profile("gleam_json", "host/non_finite")
                    .expect("non-finite host module should be valid")
                    .with_function(name, move || value)
                    .expect("non-finite function should register");
            let source = format!(
                r#"
import host/non_finite

pub fn main() {{
  do_float(non_finite.{name}())
}}
"#,
            );
            let execution = execution_with_modules(&source, [non_finite]);
            let error = execution
                .run_main(&mut run_state([0; 32]), &mut Vec::new())
                .expect_err("non-finite JSON float should fail");
            let error = expect_json_host_error(error);

            assert_eq!(error.package(), "gleam_json");
            assert_eq!(error.module(), "gleam/json");
            assert_eq!(error.function(), "do_float");
            assert_eq!(
                error.failure().message(),
                "JSON cannot encode a non-finite Float",
            );
        }
    }

    #[test]
    #[should_panic(expected = "JSON encoding failure should remain a host failure")]
    fn host_failure_assertion_rejects_other_execution_errors() {
        let _ = expect_json_host_error(ExecutionError::Invariant(
            InvariantError::ListIndexOutOfBounds {
                item_type: ValueType::Int,
                index: 1,
                length: 0,
            },
        ));
    }

    fn expect_json_host_error(error: ExecutionError) -> Box<HostError> {
        let ExecutionError::Host(error) = error else {
            panic!("JSON encoding failure should remain a host failure");
        };
        error
    }
}
