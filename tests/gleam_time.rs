use camino::{Utf8Path, Utf8PathBuf};
use geam::gleam_stdlib::{GleamStdlibRunState, host_providers as stdlib_host_providers};
use geam::gleam_time::{
    GleamTimeProfile, GleamTimeRunState, TimeSource, host_providers as time_host_providers,
};
use geam::{
    HostFailure, HostModule, HostProviderSet, HostedExecution, TypedProgram, Value,
    compile_typed_host_project, compile_typed_project, plan_host_program,
};
use std::collections::VecDeque;
use std::time::SystemTime;

#[path = "gleam_time/calendar.rs"]
mod calendar;
#[path = "gleam_time/duration.rs"]
mod duration;
#[path = "gleam_time/effects.rs"]
mod effects;
#[path = "gleam_time/surface.rs"]
mod surface;
#[path = "gleam_time/timestamp.rs"]
mod timestamp;
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
    ("gleam_time", "gleam/time/duration"),
    ("gleam_time", "gleam/time/calendar"),
    ("gleam_time", "gleam/time/timestamp"),
    ("geam_time_test", "gleam_time_effects"),
];

struct ScriptedSource {
    events: VecDeque<ScriptedEvent>,
}

enum ScriptedEvent {
    SystemTime(SystemTime),
    LocalOffset(i32),
}

impl ScriptedSource {
    fn new(events: impl IntoIterator<Item = ScriptedEvent>) -> Self {
        Self {
            events: events.into_iter().collect(),
        }
    }
}

impl TimeSource for ScriptedSource {
    fn system_time(&mut self) -> Result<SystemTime, HostFailure> {
        match self.events.pop_front() {
            Some(ScriptedEvent::SystemTime(time)) => Ok(time),
            Some(ScriptedEvent::LocalOffset(_)) => {
                Err(HostFailure::new("expected a local offset call"))
            }
            None => Err(HostFailure::new("scripted Time source exhausted")),
        }
    }

    fn local_offset_seconds(&mut self) -> Result<i32, HostFailure> {
        match self.events.pop_front() {
            Some(ScriptedEvent::LocalOffset(offset)) => Ok(offset),
            Some(ScriptedEvent::SystemTime(_)) => {
                Err(HostFailure::new("expected a system time call"))
            }
            None => Err(HostFailure::new("scripted Time source exhausted")),
        }
    }
}

fn assert_surface(module: &str, expected: &ExpectedSurface) {
    let program = compile_fixture("gleam_time_effects");
    assert_module_surface(&program, "gleam_time", module, expected);
}

fn assert_full_project_graph() {
    let program = compile_fixture("gleam_time_effects");
    assert_eq!(typed_module_order(&program), FULL_DEPENDENCY_ORDER);

    let typed = compile_typed_host_project(project_root(), "gleam_time_effects", time_hosts())
        .expect("resolved hosted Time fixture should compile");
    let plan = plan_host_program(typed).expect("official Time modules should plan");
    assert_eq!(
        plan.modules()
            .iter()
            .map(|module| (module.package().as_str(), module.module().as_str()))
            .collect::<Vec<_>>(),
        FULL_DEPENDENCY_ORDER,
    );
}

fn run_fixture(root_module: &str, source: ScriptedSource) -> Value {
    let mut state = GleamTimeRunState::new(GleamStdlibRunState::from_seed([0; 32]), source);
    run_fixture_with_state(root_module, &mut state)
}

fn run_fixture_with_state(
    root_module: &str,
    state: &mut GleamTimeRunState<ScriptedSource>,
) -> Value {
    let expected = fixture_expected(root_module);
    let execution = fixture_execution(root_module);
    let actual = execution
        .run_main(state, &mut Vec::new())
        .expect("official Time fixture should run");

    assert_eq!(actual.inspect().to_string(), expected);
    actual
}

fn fixture_execution(root_module: &str) -> HostedExecution<GleamTimeProfile<ScriptedSource>> {
    let typed = compile_typed_host_project(project_root(), root_module, time_hosts())
        .expect("resolved hosted Time fixture should compile");
    let plan = plan_host_program(typed).expect("official Time fixture should plan");
    HostedExecution::try_from_module_plan(plan).expect("official Time fixture should seal")
}

fn fixture_expected(root_module: &str) -> String {
    let source = std::fs::read_to_string(
        project_root()
            .join("src")
            .join(root_module)
            .with_extension("gleam"),
    )
    .expect("Time fixture source should be readable");

    source
        .lines()
        .rev()
        .find(|line| !line.trim().is_empty())
        .expect("Time fixture should not be empty")
        .trim()
        .strip_prefix("// @geam:expect ")
        .expect("last non-empty Time fixture line should contain `// @geam:expect`")
        .to_owned()
}

fn compile_fixture(root_module: &str) -> TypedProgram {
    compile_typed_project(project_root(), root_module)
        .expect("resolved Time fixture should compile")
}

fn typed_module_order(program: &TypedProgram) -> Vec<(&str, &str)> {
    program
        .modules()
        .map(|module| (module.type_info.package.as_str(), module.name.as_str()))
        .collect()
}

fn time_hosts() -> HostProviderSet<GleamTimeProfile<ScriptedSource>> {
    let mut providers = stdlib_host_providers::<GleamTimeProfile<ScriptedSource>>()
        .expect("official stdlib providers should register");
    providers.extend(
        time_host_providers::<GleamTimeProfile<ScriptedSource>>()
            .expect("official Time providers should register"),
    );
    HostProviderSet::with_providers(
        Vec::<HostModule<GleamTimeProfile<ScriptedSource>>>::new(),
        providers,
    )
    .expect("official stdlib and Time provider modules should be unique")
}

fn project_root() -> Utf8PathBuf {
    Utf8Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/projects/gleam_time")
}
