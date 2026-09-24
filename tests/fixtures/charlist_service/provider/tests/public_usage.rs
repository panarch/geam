//! Original Gleam Charlist source consuming values created by another Rust provider.
extern crate geam as geam_core;

#[path = "../../../../support/execution_host.rs"]
mod execution_fixture;

use camino::Utf8Path;
use geam::gleam_erlang::{
    Component as ErlangComponent, Configuration, ErlangExecution, GleamErlangHostProfile,
    Stores as ErlangStores,
};
use geam::gleam_stdlib::{
    Component as StdlibComponent, GleamStdlibHostProfile, GleamStdlibRunState, GleamStdlibStores,
    IoOutput,
};
use geam::host::{
    HostCall, HostCallCompletion, HostCallError, HostComponentProfile, HostProfile,
    HostProviderComponentRegistration, HostProviderModule, HostProviderSet, HostTypeParameter,
    HostValue,
};
use geam::{HostedExecution, compile_typed_host_project, plan_host_program};
use geam_charlist_service_fixture::Component;
use std::{fs, process::Command};

struct Profile;

#[derive(Default)]
struct Stores {
    stdlib: GleamStdlibStores,
    erlang: ErlangStores<Profile>,
    provider: (),
}

struct State {
    stdlib: GleamStdlibRunState,
    erlang: Configuration,
    provider: (),
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
    fn component_stores(stores: &Stores) -> &() {
        &stores.provider
    }

    fn component_state(state: &mut State) -> &mut () {
        &mut state.provider
    }
}

impl GleamErlangHostProfile for Profile {
    fn erlang_execution(state: &mut ErlangExecution) -> &mut ErlangExecution {
        state
    }
}

#[test]
fn original_charlist_source_reads_independently_constructed_values() {
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
    providers.extend(geam::gleam_erlang::host_providers::<Profile>().unwrap());
    providers
        .extend(<Component as HostProviderComponentRegistration<Profile>>::providers().unwrap());
    providers.push(
        HostProviderModule::new("charlist_service_fixture", "native_observer")
            .unwrap()
            .with_scoped_function::<Component, (HostTypeParameter<0>, HostTypeParameter<1>), (), _>(
                "assert_equal",
                assert_native_equal,
            )
            .unwrap(),
    );
    let typed = compile_typed_host_project(
        &project,
        "charlist_service_fixture",
        HostProviderSet::from_providers(providers).unwrap(),
    )
    .unwrap();
    let mut state = State {
        stdlib: GleamStdlibRunState::from_seed([0; 32]),
        erlang: Configuration {
            resources: typed.package_resources().clone(),
        },
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
            "#([0, 65, 233, 128578], #(\
#(charlist.from_string(\"HTTP/1.1\"), 200, charlist.from_string(\"OK\")), \
[#(charlist.from_string(\"x-empty\"), []), \
#(charlist.from_string(\"x-unicode\"), [0, 65, 233, 128578])]))"
        );
    }
    assert!(echo.is_empty());
}

fn assert_native_equal<'call>(
    call: HostCall<'call, Profile, Component, ()>,
    actual: HostValue<'call, HostTypeParameter<0>>,
    expected: HostValue<'call, HostTypeParameter<1>>,
) -> Result<HostCallCompletion<'call, ()>, HostCallError> {
    let actual = call.native_value::<HostTypeParameter<0>>(actual);
    let expected = call.native_value::<HostTypeParameter<1>>(expected);
    assert_eq!(actual.kind(), expected.kind());
    assert!(call.native_equal(&actual, &expected));
    assert!(call.native_equal(&expected, &actual));
    assert_eq!(call.native_hash(&actual), call.native_hash(&expected));
    Ok(call.return_value(()))
}
