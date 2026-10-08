//! Original stdlib Decoder source crossing an independent manual and macro provider.
extern crate geam as geam_core;

#[path = "../../../../support/execution_host.rs"]
mod execution_fixture;

use camino::{Utf8Path, Utf8PathBuf};
use geam::gleam_erlang::{
    Component as ErlangComponent, Configuration, ErlangExecution, GleamErlangHostProfile,
    Stores as ErlangStores,
};
use geam::gleam_stdlib::{
    Component as StdlibComponent, GleamStdlibHostProfile, GleamStdlibRunState, GleamStdlibStores,
    IoOutput,
};
use geam::host::{
    HostComponentProfile, HostProfile, HostProviderComponentRegistration, HostProviderSet,
};
use geam::{HostedExecution, Value, compile_typed_host_project, plan_host_program};
use geam_decoder_service_fixture::{Component, Stores as ProviderStores};
use std::{fs, process::Command};

struct Profile;

#[derive(Default)]
struct Stores {
    stdlib: GleamStdlibStores,
    erlang: ErlangStores<Profile>,
    provider: ProviderStores,
}

struct State {
    stdlib: GleamStdlibRunState,
    erlang: Configuration,
    provider: usize,
}

impl HostProfile for Profile {
    type RunState = State;
    type ExternalStores = Stores;
    type ExecutionState = ErlangExecution;
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

impl HostComponentProfile<ErlangComponent<Self>> for Profile {
    fn component_stores(stores: &Stores) -> &ErlangStores<Self> {
        &stores.erlang
    }
    fn component_state(state: &mut State) -> &mut Configuration {
        &mut state.erlang
    }
}

impl HostComponentProfile<Component> for Profile {
    fn component_stores(stores: &Stores) -> &ProviderStores {
        &stores.provider
    }
    fn component_state(state: &mut State) -> &mut usize {
        &mut state.provider
    }
}

impl GleamErlangHostProfile for Profile {
    fn erlang_execution(state: &mut ErlangExecution) -> &mut ErlangExecution {
        state
    }
}

#[test]
fn original_decoders_round_trip_through_lists_fields_and_subject_messages() {
    let project: Utf8PathBuf = Utf8Path::new(env!("CARGO_MANIFEST_DIR")).join("../project");
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
    providers.extend(geam::gleam_erlang::host_providers::<Profile>().unwrap());
    providers
        .extend(<Component as HostProviderComponentRegistration<Profile>>::providers().unwrap());
    let typed = compile_typed_host_project(
        &project,
        "decoder_service_fixture",
        HostProviderSet::from_providers(providers).unwrap(),
    )
    .unwrap();
    let mut state = State {
        stdlib: GleamStdlibRunState::from_seed([0; 32]),
        erlang: Configuration {
            resources: typed.package_resources().clone(),
        },
        provider: 0,
    };
    let mut loaded =
        HostedExecution::try_from_module_plan(plan_host_program(typed).unwrap()).unwrap();
    let mut echoes = Vec::new();
    for _ in 0..2 {
        assert_eq!(
            execution_fixture::run(&mut loaded, &mut state, &mut echoes).unwrap(),
            Value::Nil
        );
    }
    assert!(echoes.is_empty());
    assert_eq!(state.provider, 2);
    assert_eq!(
        state
            .stdlib
            .io_outputs()
            .iter()
            .map(|output| output.text().as_str().unwrap())
            .collect::<Vec<_>>(),
        ["decoder SDK: 42\n", "decoder SDK: 42\n"]
    );
}
