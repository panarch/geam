use super::{Component, GleamJsonProfile, GleamJsonRunState, GleamJsonStores};
use crate::{
    HostComponentProfile, HostModule, HostProfile, HostProviderSet, HostedExecution, ModuleSource,
    PackageSource, compile_typed_host_program, plan_host_program,
};
use ecow::EcoString;
use geam_stdlib::{
    Component as GleamStdlibComponent, GleamStdlibHostProfile, GleamStdlibRunState,
    GleamStdlibStores, IoOutput,
};

pub(super) struct CustomProfile;

#[derive(Default)]
pub(super) struct CustomStores {
    pub(super) stdlib: GleamStdlibStores,
    pub(super) json: GleamJsonStores,
}

pub(super) struct CustomRunState {
    pub(super) stdlib: GleamStdlibRunState,
    pub(super) json: (),
}

impl HostProfile for CustomProfile {
    type RunState = CustomRunState;
    type ExternalStores = CustomStores;
}

impl HostComponentProfile<GleamStdlibComponent> for CustomProfile {
    fn component_stores(stores: &Self::ExternalStores) -> &GleamStdlibStores {
        &stores.stdlib
    }

    fn component_state(state: &mut Self::RunState) -> &mut GleamStdlibRunState {
        &mut state.stdlib
    }
}

impl GleamStdlibHostProfile for CustomProfile {
    type Io = Vec<IoOutput>;
}

impl HostComponentProfile<Component> for CustomProfile {
    fn component_stores(stores: &Self::ExternalStores) -> &GleamJsonStores {
        &stores.json
    }

    fn component_state(state: &mut Self::RunState) -> &mut () {
        &mut state.json
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

@external(erlang, "host", "test_source_hash")
fn test_source_hash(json: Json) -> Int
"#;

pub(super) fn execution(source: &str) -> HostedExecution<GleamJsonProfile> {
    execution_with_modules(source, Vec::<HostModule<GleamJsonProfile>>::new())
}

pub(super) fn execution_with_modules(
    source: &str,
    modules: impl IntoIterator<Item = HostModule<GleamJsonProfile>>,
) -> HostedExecution<GleamJsonProfile> {
    let providers = super::function::test_host_providers();
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

pub(super) fn run_state(seed: [u8; 32]) -> GleamJsonRunState {
    GleamJsonRunState::new(GleamStdlibRunState::from_seed(seed))
}
