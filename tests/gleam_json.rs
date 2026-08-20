use camino::{Utf8Path, Utf8PathBuf};
use geam::gleam_json::{
    GleamJsonProfile, GleamJsonRunState, host_providers as json_host_providers,
};
use geam::gleam_stdlib::{GleamStdlibRunState, host_providers as stdlib_host_providers};
use geam::{
    HostModule, HostProviderSet, HostedExecution, TypedProgram, Value, compile_typed_host_project,
    compile_typed_project, plan_host_program,
};

#[path = "gleam_json/decode.rs"]
mod decode;
#[path = "gleam_json/encode.rs"]
mod encode;
#[path = "gleam_json/error.rs"]
mod error;
#[path = "gleam_json/surface.rs"]
mod surface;
#[path = "support/upstream_surface.rs"]
mod upstream_surface;
#[path = "support/workspace_dependencies.rs"]
mod workspace_dependencies;

use upstream_surface::{ExpectedSurface, assert_module_surface};

const FULL_DEPENDENCY_ORDER: &[(&str, &str)] = &[
    ("gleam_stdlib", "gleam/order"),
    ("gleam_stdlib", "gleam/float"),
    ("gleam_stdlib", "gleam/int"),
    ("gleam_stdlib", "gleam/option"),
    ("gleam_stdlib", "gleam/dict"),
    ("gleam_stdlib", "gleam/list"),
    ("gleam_stdlib", "gleam/string_tree"),
    ("gleam_stdlib", "gleam/string"),
    ("gleam_stdlib", "gleam/bit_array"),
    ("gleam_stdlib", "gleam/dynamic"),
    ("gleam_stdlib", "gleam/dynamic/decode"),
    ("gleam_stdlib", "gleam/result"),
    ("gleam_json", "gleam/json"),
    ("geam_json_test", "gleam_json_roundtrip"),
];

fn assert_surface(expected: &ExpectedSurface) {
    let program = compile_fixture("gleam_json_roundtrip");
    assert_module_surface(&program, "gleam_json", "gleam/json", expected);
}

fn assert_full_project_graph() {
    let program = compile_fixture("gleam_json_roundtrip");
    assert_eq!(typed_module_order(&program), FULL_DEPENDENCY_ORDER);

    let typed = compile_typed_host_project(project_root(), "gleam_json_roundtrip", json_hosts())
        .expect("resolved hosted JSON fixture should compile");
    let plan = plan_host_program(typed).expect("official JSON modules should plan");
    assert_eq!(
        plan.modules()
            .iter()
            .map(|module| (module.package().as_str(), module.module().as_str()))
            .collect::<Vec<_>>(),
        FULL_DEPENDENCY_ORDER,
    );
}

fn run_fixture(root_module: &str) -> Value {
    let expected = fixture_expected(root_module);
    let execution = fixture_execution(root_module);
    let actual = execution
        .run_main(
            &mut GleamJsonRunState::new(GleamStdlibRunState::from_seed([0; 32])),
            &mut Vec::new(),
        )
        .expect("official JSON fixture should run");

    assert_eq!(actual.inspect().to_string(), expected);
    actual
}

fn run_fixture_repeated(root_module: &str) {
    let expected = fixture_expected(root_module);
    let execution = fixture_execution(root_module);
    let mut first_state = GleamJsonRunState::new(GleamStdlibRunState::from_seed([1; 32]));
    let mut second_state = GleamJsonRunState::new(GleamStdlibRunState::from_seed([2; 32]));

    let first = execution
        .run_main(&mut first_state, &mut Vec::new())
        .expect("official JSON fixture should run the first time");
    let repeated = execution
        .run_main(&mut first_state, &mut Vec::new())
        .expect("official JSON fixture should repeat with the same state");
    let independent = execution
        .run_main(&mut second_state, &mut Vec::new())
        .expect("official JSON fixture should run with an independent state");

    for actual in [first, repeated, independent] {
        assert_eq!(actual.inspect().to_string(), expected);
    }
}

fn fixture_execution(root_module: &str) -> HostedExecution<GleamJsonProfile> {
    let typed = compile_typed_host_project(project_root(), root_module, json_hosts())
        .expect("resolved hosted JSON fixture should compile");
    let plan = plan_host_program(typed).expect("official JSON fixture should plan");
    HostedExecution::try_from_module_plan(plan).expect("official JSON fixture should seal")
}

fn fixture_expected(root_module: &str) -> String {
    let source = std::fs::read_to_string(
        project_root()
            .join("src")
            .join(root_module)
            .with_extension("gleam"),
    )
    .expect("JSON fixture source should be readable");

    source
        .lines()
        .rev()
        .find(|line| !line.trim().is_empty())
        .expect("JSON fixture should not be empty")
        .trim()
        .strip_prefix("// @geam:expect ")
        .expect("last non-empty JSON fixture line should contain `// @geam:expect`")
        .to_owned()
}

fn compile_fixture(root_module: &str) -> TypedProgram {
    compile_typed_project(project_root(), root_module)
        .expect("resolved JSON fixture should compile")
}

fn typed_module_order(program: &TypedProgram) -> Vec<(&str, &str)> {
    program
        .modules()
        .map(|module| (module.type_info.package.as_str(), module.name.as_str()))
        .collect()
}

fn json_hosts() -> HostProviderSet<GleamJsonProfile> {
    let mut providers = stdlib_host_providers::<GleamJsonProfile>()
        .expect("official stdlib providers should register");
    providers.extend(
        json_host_providers::<GleamJsonProfile>().expect("official JSON providers should register"),
    );
    HostProviderSet::with_providers(Vec::<HostModule<GleamJsonProfile>>::new(), providers)
        .expect("official stdlib and JSON provider modules should be unique")
}

fn project_root() -> Utf8PathBuf {
    let root = Utf8Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/projects/gleam_json");
    static PREPARED: std::sync::OnceLock<Result<(), String>> = std::sync::OnceLock::new();
    workspace_dependencies::prepare(
        &PREPARED,
        root.as_std_path(),
        "gleam",
        &["deps", "download"],
        "`gleam deps download`",
    );
    root
}
