use camino::{Utf8Path, Utf8PathBuf};
use geam::gleam_stdlib::{GleamStdlibProfile, GleamStdlibRunState, host_providers};
use geam::{
    HostModule, HostProviderSet, HostedExecution, TypedProgram, Value, compile_typed_host_project,
    compile_typed_project, plan_host_program,
};

#[path = "gleam_http/cookie.rs"]
mod cookie;
#[path = "gleam_http/http.rs"]
mod http;
#[path = "gleam_http/request.rs"]
mod request;
#[path = "gleam_http/response.rs"]
mod response;
#[path = "gleam_http/service.rs"]
mod service;
#[path = "support/upstream_surface.rs"]
mod upstream_surface;

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
    ("gleam_stdlib", "gleam/bool"),
    ("gleam_stdlib", "gleam/result"),
    ("gleam_stdlib", "gleam/uri"),
    ("gleam_http", "gleam/http"),
    ("gleam_http", "gleam/http/cookie"),
    ("gleam_http", "gleam/http/request"),
    ("gleam_http", "gleam/http/response"),
    ("gleam_http", "gleam/http/service"),
    ("geam_http_test", "gleam_http_service"),
];

fn assert_surface(module: &str, expected: &ExpectedSurface) {
    let program = compile_fixture("gleam_http_service");
    assert_module_surface(&program, "gleam_http", module, expected);
}

fn assert_full_project_graph() {
    let program = compile_fixture("gleam_http_service");
    assert_eq!(typed_module_order(&program), FULL_DEPENDENCY_ORDER);

    let typed = compile_typed_host_project(project_root(), "gleam_http_service", stdlib_hosts())
        .expect("resolved hosted HTTP fixture should compile");
    let plan = plan_host_program(typed).expect("official HTTP modules should plan");
    assert_eq!(
        plan.modules()
            .iter()
            .map(|module| (module.package().as_str(), module.module().as_str()))
            .collect::<Vec<_>>(),
        FULL_DEPENDENCY_ORDER,
    );
}

fn run_fixture(root_module: &str) -> Value {
    let source = std::fs::read_to_string(
        project_root()
            .join("src")
            .join(root_module)
            .with_extension("gleam"),
    )
    .expect("HTTP fixture source should be readable");
    let expected = source
        .lines()
        .rev()
        .find(|line| !line.trim().is_empty())
        .expect("HTTP fixture should not be empty")
        .trim()
        .strip_prefix("// @geam:expect ")
        .expect("last non-empty HTTP fixture line should contain `// @geam:expect`");

    let typed = compile_typed_host_project(project_root(), root_module, stdlib_hosts())
        .expect("resolved hosted HTTP fixture should compile");
    let plan = plan_host_program(typed).expect("official HTTP fixture should plan");
    let execution =
        HostedExecution::try_from_module_plan(plan).expect("official HTTP fixture should seal");
    let actual = execution
        .run_main(
            &mut GleamStdlibRunState::from_seed([0; 32]),
            &mut Vec::new(),
        )
        .expect("official HTTP fixture should run");

    assert_eq!(actual.inspect().to_string(), expected);
    actual
}

fn compile_fixture(root_module: &str) -> TypedProgram {
    compile_typed_project(project_root(), root_module)
        .expect("resolved HTTP fixture should compile")
}

fn typed_module_order(program: &TypedProgram) -> Vec<(&str, &str)> {
    program
        .modules()
        .map(|module| (module.type_info.package.as_str(), module.name.as_str()))
        .collect()
}

fn stdlib_hosts() -> HostProviderSet<GleamStdlibProfile> {
    let providers =
        host_providers::<GleamStdlibProfile>().expect("official stdlib providers should register");
    HostProviderSet::with_providers(Vec::<HostModule<GleamStdlibProfile>>::new(), providers)
        .expect("official stdlib provider modules should be unique")
}

fn project_root() -> Utf8PathBuf {
    Utf8Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/projects/gleam_http")
}
