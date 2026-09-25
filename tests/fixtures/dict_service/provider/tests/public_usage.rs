//! Original Gleam Dict source and prelude Results consuming another Rust provider's values.
extern crate geam as geam_core;

#[path = "../../../../support/execution_host.rs"]
mod execution_fixture;

use camino::Utf8Path;
use geam::gleam_stdlib::{
    Component as StdlibComponent, GleamStdlibHostProfile, GleamStdlibRunState, GleamStdlibStores,
    IoOutput,
};
use geam::host::{
    HostComponentProfile, HostProfile, HostProviderComponentRegistration, HostProviderSet,
};
use geam::{HostedExecution, compile_typed_host_project, plan_host_program};
use geam_dict_service_fixture::Component;
use std::{fs, process::Command};

struct Profile;

#[derive(Default)]
struct Stores {
    stdlib: GleamStdlibStores,
    provider: (),
}

struct State {
    stdlib: GleamStdlibRunState,
    provider: (),
}

impl HostProfile for Profile {
    type RunState = State;
    type ExternalStores = Stores;
    type ExecutionState = ();
}

impl GleamStdlibHostProfile for Profile {
    type Io = Vec<IoOutput>;
}

impl HostComponentProfile<StdlibComponent> for Profile {
    fn component_stores(stores: &Stores) -> &GleamStdlibStores {
        &stores.stdlib
    }

    fn component_state(state: &mut State) -> &mut GleamStdlibRunState {
        &mut state.stdlib
    }
}

impl HostComponentProfile<Component> for Profile {
    fn component_stores(stores: &Stores) -> &() {
        &stores.provider
    }

    fn component_state(state: &mut State) -> &mut () {
        &mut state.provider
    }
}

#[test]
fn original_dict_source_reads_independently_constructed_dicts_and_results() {
    let project = Utf8Path::new(env!("CARGO_MANIFEST_DIR")).join("../project");
    let manifest = fs::read(project.join("manifest.toml")).unwrap();
    let acquisition = Command::new("gleam")
        .args(["deps", "download"])
        .current_dir(&project)
        .output()
        .unwrap();
    assert!(
        acquisition.status.success(),
        "{}",
        String::from_utf8_lossy(&acquisition.stderr)
    );
    assert_eq!(fs::read(project.join("manifest.toml")).unwrap(), manifest);
    let mut providers = geam::gleam_stdlib::host_providers::<Profile>().unwrap();
    providers
        .extend(<Component as HostProviderComponentRegistration<Profile>>::providers().unwrap());
    let typed = compile_typed_host_project(
        &project,
        "dict_service_fixture",
        HostProviderSet::from_providers(providers).unwrap(),
    )
    .unwrap();
    let mut state = State {
        stdlib: GleamStdlibRunState::from_seed([0; 32]),
        provider: (),
    };
    let mut execution =
        HostedExecution::try_from_module_plan(plan_host_program(typed).unwrap()).unwrap();
    let host = execution_fixture::TestHost::default();
    let mut echo = Vec::new();
    let first = host
        .block_on(execution.run_main(&host, &mut state, &mut echo))
        .unwrap();
    let second = host
        .block_on(execution.run_main(&host, &mut state, &mut echo))
        .unwrap();
    drop(execution);
    drop(state);
    for value in [first, second] {
        assert_eq!(
            value.inspect().to_string(),
            "#(dict.from_list([#(\"LANG\", \"한국어\\0🙂\")]), \
dict.from_list([#(1, [\"one\", \"하나\"]), #(2, [])]), \
#(dict.from_list([#(\"outer\", \"ready\")]), \
[dict.from_list([#(\"inner\", \"nested\")]), dict.from_list([])]), \
Ok(\"한국어\\0🙂\"), Error(Nil), \
Ok(dict.from_list([#(\"status\", \"ready\")])), Error(Nil))"
        );
    }
    assert!(echo.is_empty());
}
