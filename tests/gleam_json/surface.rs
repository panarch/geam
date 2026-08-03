use super::{ExpectedSurface, assert_full_project_graph, assert_surface};

const SURFACE: ExpectedSurface = ExpectedSurface {
    values: &[
        "UnableToDecode",
        "UnexpectedByte",
        "UnexpectedEndOfInput",
        "UnexpectedSequence",
        "array",
        "bool",
        "dict",
        "float",
        "int",
        "null",
        "nullable",
        "object",
        "parse",
        "parse_bits",
        "preprocessed_array",
        "string",
        "to_string",
        "to_string_tree",
    ],
    types: &[("DecodeError", 0), ("Json", 0)],
    type_aliases: &[],
    constructors: &[
        ("DecodeError", "UnableToDecode", 1),
        ("DecodeError", "UnexpectedByte", 1),
        ("DecodeError", "UnexpectedEndOfInput", 0),
        ("DecodeError", "UnexpectedSequence", 1),
    ],
    functions: r#"
array: fn(from: List(a), of: fn(a) -> Json) -> Json
bool: fn(Bool) -> Json
dict: fn(Dict(k, v), fn(k) -> String, fn(v) -> Json) -> Json
float: fn(Float) -> Json
int: fn(Int) -> Json
null: fn() -> Json
nullable: fn(from: Option(a), of: fn(a) -> Json) -> Json
object: fn(List(#(String, Json))) -> Json
parse: fn(from: String, using: decode.Decoder(t)) -> Result(t, DecodeError)
parse_bits: fn(from: BitArray, using: decode.Decoder(t)) -> Result(t, DecodeError)
preprocessed_array: fn(List(Json)) -> Json
string: fn(String) -> Json
to_string: fn(Json) -> String
to_string_tree: fn(Json) -> StringTree
"#,
};

#[test]
#[ignore = "requires `gleam deps download` in the gleam_json fixture"]
fn tracks_official_gleam_json_public_surface() {
    assert_surface(&SURFACE);
}

#[test]
#[ignore = "requires `gleam deps download` in the gleam_json fixture"]
fn tracks_the_complete_resolved_json_project_graph() {
    assert_full_project_graph();
}
