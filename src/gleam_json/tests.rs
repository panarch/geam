use super::function::JsonProvider;
use super::storage::JsonStorage;
use super::{
    GleamJsonHostProfile, GleamJsonProfile, GleamJsonProfileStores, GleamJsonStores, host_providers,
};
use crate::gleam_stdlib::{
    DictSchema, DynamicSchema, GleamStdlibHostProfile, GleamStdlibRunState, GleamStdlibStores,
    IoOutput, StoredStringTree, StringTreeSchema,
};
use crate::{
    ExecutionError, HostExternalEquality, HostExternalHashing, HostExternalInspection,
    HostExternalStorage, HostModule, HostProfile, HostProvider, HostProviderModule,
    HostProviderSet, HostedExecution, ModuleSource, PackageSource, compile_typed_host_program,
    plan_host_program,
};
use ecow::EcoString;

struct CustomProfile;

#[derive(Default)]
struct CustomStores {
    stdlib: GleamStdlibStores,
    json: GleamJsonStores,
}

impl HostProfile for CustomProfile {
    type RunState = GleamStdlibRunState;
    type ExternalStores = CustomStores;
}

impl GleamStdlibHostProfile for CustomProfile {
    type Io = Vec<IoOutput>;

    fn gleam_stdlib_stores(stores: &Self::ExternalStores) -> &GleamStdlibStores {
        &stores.stdlib
    }

    fn gleam_stdlib_run_state(state: &mut Self::RunState) -> &mut GleamStdlibRunState {
        state
    }

    fn gleam_stdlib_io(state: &mut Self::RunState) -> &mut Self::Io {
        <crate::gleam_stdlib::GleamStdlibProfile as GleamStdlibHostProfile>::gleam_stdlib_io(state)
    }
}

impl GleamJsonHostProfile for CustomProfile {
    fn gleam_json_stores(stores: &Self::ExternalStores) -> &GleamJsonStores {
        &stores.json
    }
}

const DYNAMIC_SOURCE: &str = "pub type Dynamic";
const DICT_SOURCE: &str = "pub type Dict(key, value)";
const STRING_TREE_SOURCE: &str = "pub type StringTree";
const DYNAMIC_DECODE_SOURCE: &str = r#"
pub type DecodeError {
  DecodeError(expected: String, found: String, path: List(String))
}
"#;
const JSON_DECLARATIONS: &str = r#"
import gleam/dynamic.{type Dynamic}
import gleam/dynamic/decode
import gleam/string_tree.{type StringTree}

pub type Json

pub type DecodeError {
  UnexpectedEndOfInput
  UnexpectedByte(String)
  UnexpectedSequence(String)
  UnableToDecode(List(decode.DecodeError))
}

@external(erlang, "host", "decode_to_dynamic")
fn decode_to_dynamic(json: BitArray) -> Result(Dynamic, DecodeError)

@external(erlang, "host", "do_to_string")
fn do_to_string(json: Json) -> String

@external(erlang, "host", "to_string_tree")
fn to_string_tree(json: Json) -> StringTree

@external(erlang, "host", "do_string")
fn do_string(value: String) -> Json

@external(erlang, "host", "do_bool")
fn do_bool(value: Bool) -> Json

@external(erlang, "host", "do_int")
fn do_int(value: Int) -> Json

@external(erlang, "host", "do_float")
fn do_float(value: Float) -> Json

@external(erlang, "host", "do_null")
fn do_null() -> Json

@external(erlang, "host", "do_object")
fn do_object(entries: List(#(String, Json))) -> Json

@external(erlang, "host", "do_preprocessed_array")
fn do_preprocessed_array(values: List(Json)) -> Json
"#;

fn execution(source: &str) -> HostedExecution<GleamJsonProfile> {
    execution_with_modules(source, Vec::<HostModule<GleamJsonProfile>>::new())
}

fn execution_with_modules(
    source: &str,
    modules: impl IntoIterator<Item = HostModule<GleamJsonProfile>>,
) -> HostedExecution<GleamJsonProfile> {
    let providers =
        [
            HostProviderModule::new("gleam_stdlib", "gleam/dynamic")
                .and_then(
                    HostProviderModule::with_external_type::<
                        JsonProvider<GleamJsonProfile>,
                        DynamicSchema,
                    >,
                )
                .expect("synthetic Dynamic declaration should register"),
            HostProviderModule::new("gleam_stdlib", "gleam/dict")
                .and_then(
                    HostProviderModule::with_external_type::<
                        JsonProvider<GleamJsonProfile>,
                        DictSchema,
                    >,
                )
                .expect("synthetic Dict declaration should register"),
            HostProviderModule::new("gleam_stdlib", "gleam/string_tree")
                .and_then(
                    HostProviderModule::with_external_type::<
                        JsonProvider<GleamJsonProfile>,
                        StringTreeSchema,
                    >,
                )
                .expect("synthetic StringTree declaration should register"),
            super::host_provider::<GleamJsonProfile>()
                .expect("official JSON provider should register"),
        ];
    let source = format!("{JSON_DECLARATIONS}\n{source}");
    let packages = [
        PackageSource::new(
            "gleam_stdlib",
            Vec::<EcoString>::new(),
            [
                ModuleSource::new("gleam/dynamic", "src/gleam/dynamic.gleam", DYNAMIC_SOURCE),
                ModuleSource::new("gleam/dict", "src/gleam/dict.gleam", DICT_SOURCE),
                ModuleSource::new(
                    "gleam/string_tree",
                    "src/gleam/string_tree.gleam",
                    STRING_TREE_SOURCE,
                ),
                ModuleSource::new(
                    "gleam/dynamic/decode",
                    "src/gleam/dynamic/decode.gleam",
                    DYNAMIC_DECODE_SOURCE,
                ),
            ],
        ),
        PackageSource::new(
            "gleam_json",
            [EcoString::from("gleam_stdlib")],
            [ModuleSource::new(
                "gleam/json",
                "src/gleam/json.gleam",
                source,
            )],
        ),
    ];
    let hosts = HostProviderSet::with_providers(modules, providers)
        .expect("synthetic provider modules should be unique");
    let typed = compile_typed_host_program("gleam_json", "gleam/json", packages, hosts)
        .expect("synthetic JSON source should compile");
    let plan = plan_host_program(typed).expect("synthetic JSON source should plan");
    HostedExecution::try_from_module_plan(plan).expect("synthetic JSON execution should seal")
}

#[test]
fn default_and_custom_profiles_project_independent_stdlib_and_json_stores() {
    let default = GleamJsonProfileStores::default();
    let custom = CustomStores::default();

    assert!(std::ptr::eq(
        GleamJsonProfile::gleam_stdlib_stores(&default),
        &default.stdlib,
    ));
    assert!(std::ptr::eq(
        GleamJsonProfile::gleam_json_stores(&default),
        &default.json,
    ));
    assert!(std::ptr::eq(
        CustomProfile::gleam_stdlib_stores(&custom),
        &custom.stdlib,
    ));
    assert!(std::ptr::eq(
        CustomProfile::gleam_json_stores(&custom),
        &custom.json,
    ));
    assert!(std::ptr::eq(
        <JsonStorage as HostExternalStorage<CustomProfile, super::schema::JsonSchema>>::store(
            &custom,
        ),
        &custom.json.json.values,
    ));

    let payload = super::storage::JsonPayload {
        tree: StoredStringTree::text("null".into()),
    };
    let equal =
        |_: &crate::runtime::StoredRuntimeValue, _: &crate::runtime::StoredRuntimeValue| true;
    let hash = |_: &crate::runtime::StoredRuntimeValue| 0;
    let inspect = |_: &crate::runtime::StoredRuntimeValue| "unused".into();
    let stored = crate::runtime::StoredRuntimeValue::test_int(0.into());
    assert_eq!(inspect(&stored), "unused");
    assert!(<JsonStorage as HostExternalStorage<
        CustomProfile,
        super::schema::JsonSchema,
    >>::source_equal(
        &HostExternalEquality::new(&equal),
        &payload,
        &payload,
    ));
    assert_eq!(
        <JsonStorage as HostExternalStorage<CustomProfile, super::schema::JsonSchema>>::source_hash(
            &HostExternalHashing::new(&hash),
            &payload,
        ),
        payload.tree.structural_hash(),
    );
    assert_eq!(
        <JsonStorage as HostExternalStorage<CustomProfile, super::schema::JsonSchema>>::inspect(
            &HostExternalInspection::new(&inspect),
            &payload,
        ),
        r#""null""#,
    );

    let mut default_state = GleamStdlibRunState::from_seed([1; 32]);
    let mut custom_state = GleamStdlibRunState::from_seed([2; 32]);
    let default_projected = GleamJsonProfile::gleam_stdlib_run_state(&mut default_state);
    assert!(std::ptr::eq(default_projected, &default_state));
    let custom_projected = CustomProfile::gleam_stdlib_run_state(&mut custom_state);
    assert!(std::ptr::eq(custom_projected, &custom_state));
    assert!(GleamJsonProfile::gleam_stdlib_io(&mut default_state).is_empty());
    assert!(CustomProfile::gleam_stdlib_io(&mut custom_state).is_empty());
}

#[test]
fn registers_only_the_exact_official_erlang_json_provider_inventory() {
    let mut providers =
        host_providers::<GleamJsonProfile>().expect("official JSON provider should register");
    assert_eq!(providers.len(), 1);
    let provider = providers
        .pop()
        .expect("JSON package should have one provider module");

    assert_eq!(provider.package(), "gleam_json");
    assert_eq!(provider.module(), "gleam/json");
    assert_eq!(
        provider
            .external_types()
            .map(|schema| (schema.name().as_str(), schema.parameter_count()))
            .collect::<Vec<_>>(),
        [("Json", 0)],
    );
    assert_eq!(
        provider
            .functions()
            .map(|function| function.name().as_str())
            .collect::<Vec<_>>(),
        [
            "decode_to_dynamic",
            "do_to_string",
            "to_string_tree",
            "do_string",
            "do_bool",
            "do_int",
            "do_float",
            "do_null",
            "do_object",
            "do_preprocessed_array",
        ],
    );

    use crate::host::HostAbiType;
    let json = <super::schema::Json as HostAbiType>::descriptor().value_type();
    let dynamic_result =
        <super::schema::JsonDynamicResult as HostAbiType>::descriptor().value_type();
    let string_tree = <crate::gleam_stdlib::StringTree as HostAbiType>::descriptor().value_type();
    let object_entries = <super::schema::ObjectEntries as HostAbiType>::descriptor().value_type();
    let json_list = <super::schema::JsonList as HostAbiType>::descriptor().value_type();
    let expected = [
        (vec![crate::ValueType::BitArray], dynamic_result),
        (vec![json.clone()], crate::ValueType::String),
        (vec![json.clone()], string_tree),
        (vec![crate::ValueType::String], json.clone()),
        (vec![crate::ValueType::Bool], json.clone()),
        (vec![crate::ValueType::Int], json.clone()),
        (vec![crate::ValueType::Float], json.clone()),
        (Vec::new(), json.clone()),
        (vec![object_entries], json.clone()),
        (vec![json_list], json),
    ];
    for (function, (arguments, return_)) in provider.functions().zip(expected) {
        assert!(function.scheme().is_monomorphic());
        assert_eq!(
            function.type_(),
            &crate::FunctionType::new(arguments, return_),
        );
    }
}

#[test]
fn executes_scalar_encoding_and_decoding_through_the_hosted_pipeline() {
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
        .run_main(
            &mut GleamStdlibRunState::from_seed([0; 32]),
            &mut Vec::new(),
        )
        .expect("scalar JSON operations should run");

    assert_eq!(
        value.inspect().to_string(),
        r#"#("\"a\\\"\\n\\u0000é\"", "true", "false", "123456789012345678901234567890", "1.0e20", "null", "[]", "{}", Ok(Nil), Ok(True), Ok("text"), Ok(123456789012345678901234567890), Ok(1.25), Ok([]), Ok(dict.from_list([])))"#,
    );
}

#[test]
fn reaches_nested_json_construction_through_the_hosted_pipeline() {
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
    decode_to_dynamic(<<"[1,{\"a\":true,\"a\":false}]":utf8>>),
  )
}
"#,
    );
    let value = execution
        .run_main(
            &mut GleamStdlibRunState::from_seed([0; 32]),
            &mut Vec::new(),
        )
        .expect("nested JSON operations should run");

    assert_eq!(
        value.inspect().to_string(),
        r#"#("{\"a\":1,\"a\":2,\"b\":[true,null]}", string_tree.from_string("[1,\"two\"]"), Ok([1, dict.from_list([#("a", True)])]))"#,
    );
}

#[test]
fn maps_every_json_parse_error_without_turning_it_into_a_host_failure() {
    let execution = execution(
        r#"
pub fn main() {
  #(
    decode_to_dynamic(<<>>),
    decode_to_dynamic(<<91>>),
    decode_to_dynamic(<<125>>),
    decode_to_dynamic(<<34, 92, 117, 120, 120, 120, 120, 34>>),
    decode_to_dynamic(<<34, 92, 117, 68, 56, 48, 48, 34>>),
    decode_to_dynamic(<<49, 101, 52, 48, 48>>),
    decode_to_dynamic(<<255>>),
    decode_to_dynamic(<<116, 114, 117, 101, 32, 102, 97, 108, 115, 101>>),
    decode_to_dynamic(<<1:size(1)>>),
    decode_to_dynamic(<<"nul":utf8>>),
    decode_to_dynamic(<<"tru":utf8>>),
    decode_to_dynamic(<<"{":utf8>>),
    decode_to_dynamic(<<"{\"a\":":utf8>>),
    decode_to_dynamic(<<"1e":utf8>>),
    decode_to_dynamic(<<"[true":utf8>>),
    decode_to_dynamic(<<"{\"a\":true":utf8>>),
    decode_to_dynamic(<<"{\"a\":true,\"b\":":utf8>>),
  )
}
"#,
    );
    let value = execution
        .run_main(
            &mut GleamStdlibRunState::from_seed([0; 32]),
            &mut Vec::new(),
        )
        .expect("malformed JSON should remain source-level DecodeError values");

    assert_eq!(
        value.inspect().to_string(),
        r#"#(Error(UnexpectedEndOfInput), Error(UnexpectedEndOfInput), Error(UnexpectedByte("0x7D")), Error(UnexpectedSequence("\\uxxxx")), Error(UnexpectedEndOfInput), Error(UnexpectedSequence("1.0e400")), Error(UnexpectedByte("0xFF")), Error(UnexpectedByte("0x66")), Error(UnexpectedByte("")), Error(UnexpectedEndOfInput), Error(UnexpectedEndOfInput), Error(UnexpectedEndOfInput), Error(UnexpectedEndOfInput), Error(UnexpectedEndOfInput), Error(UnexpectedEndOfInput), Error(UnexpectedEndOfInput), Error(UnexpectedEndOfInput))"#,
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
            .run_main(
                &mut GleamStdlibRunState::from_seed([0; 32]),
                &mut Vec::new(),
            )
            .expect_err("non-finite JSON float should fail");
        let ExecutionError::Host(error) = error else {
            panic!("JSON encoding failure should remain a host failure");
        };

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
fn escaped_json_remains_self_contained_after_execution_and_state_drop() {
    let value = {
        let execution = execution(
            r#"
pub fn main() {
  do_object([#("items", do_preprocessed_array([do_int(1), do_int(2)]))])
}
"#,
        );
        let mut state = GleamStdlibRunState::from_seed([0; 32]);
        let value = execution
            .run_main(&mut state, &mut Vec::new())
            .expect("JSON value should escape the run");
        drop(state);
        drop(execution);
        value
    };

    assert_eq!(value.inspect().to_string(), r#""{\"items\":[1,2]}""#);
    assert_eq!(value.clone(), value);
}

#[test]
fn repeated_execution_is_independent_of_the_caller_owned_run_state() {
    let execution = execution(
        r#"
pub fn main() {
  do_object([#("items", do_preprocessed_array([do_int(1), do_int(2)]))])
}
"#,
    );
    let mut first_state = GleamStdlibRunState::from_seed([1; 32]);
    let mut second_state = GleamStdlibRunState::from_seed([2; 32]);

    let first = execution
        .run_main(&mut first_state, &mut Vec::new())
        .expect("first JSON execution should run");
    let repeated = execution
        .run_main(&mut first_state, &mut Vec::new())
        .expect("repeated JSON execution should run");
    let independent = execution
        .run_main(&mut second_state, &mut Vec::new())
        .expect("independent JSON execution should run");

    let first_inspection = first.inspect().to_string();
    assert_ne!(first, repeated);
    assert_ne!(first, independent);
    assert_eq!(first_inspection, repeated.inspect().to_string());
    assert_eq!(first_inspection, independent.inspect().to_string());
    assert_eq!(first_inspection, r#""{\"items\":[1,2]}""#);
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
        execution.run_main(
            &mut GleamStdlibRunState::from_seed([0; 32]),
            &mut Vec::new(),
        ),
        Ok(crate::Value::Nil),
    );
}

#[test]
fn json_provider_projects_the_complete_run_state() {
    let mut state = GleamStdlibRunState::from_seed([0; 32]);
    let projected = <super::function::JsonProvider<GleamJsonProfile> as HostProvider<
        GleamJsonProfile,
    >>::project(&mut state);

    assert!(std::ptr::eq(projected, &state));
}
