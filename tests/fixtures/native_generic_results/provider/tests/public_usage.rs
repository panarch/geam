//! Independent public assembly of native generic results.
use camino::{Utf8Path, Utf8PathBuf};
use geam::compile_typed_host_project;
use geam::embedding::{FunctionDeclaration, HostedModuleBuilder};
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
use geam_native_generic_results_fixture::Component;
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
fn public_results_keep_payloads_and_mist_declaration_in_live_execution() {
    let project: Utf8PathBuf = Utf8Path::new(env!("CARGO_MANIFEST_DIR")).join("../project");
    let manifest = fs::read(project.join("manifest.toml")).unwrap();
    let downloaded = Command::new("gleam")
        .args(["deps", "download"])
        .current_dir(&project)
        .output()
        .unwrap();
    assert!(
        downloaded.status.success(),
        "{}",
        String::from_utf8_lossy(&downloaded.stderr)
    );
    assert_eq!(fs::read(project.join("manifest.toml")).unwrap(), manifest);
    let mut providers = geam::gleam_stdlib::host_providers::<Profile>().unwrap();
    providers.extend(geam::gleam_erlang::host_providers::<Profile>().unwrap());
    providers
        .extend(<Component as HostProviderComponentRegistration<Profile>>::providers().unwrap());
    let typed = compile_typed_host_project(
        &project,
        "native_generic_results_fixture",
        HostProviderSet::from_providers(providers).unwrap(),
    )
    .unwrap();
    let resources = typed.package_resources().clone();
    let (mut bindings, check) = HostedModuleBuilder::new(typed)
        .unwrap()
        .function(FunctionDeclaration::<(), bool>::new("check"))
        .unwrap();
    let rejected = [
        (
            "known_failure",
            "produce",
            "payload does not match the registered target",
        ),
        (
            "known_pair_failure",
            "pair",
            "pair does not match the registered target",
        ),
        (
            "known_result_failure",
            "native_result",
            "result does not match the registered target",
        ),
        (
            "known_list_failure",
            "native_list",
            "list does not match the registered target",
        ),
    ]
    .map(|(name, native, reason)| {
        (
            bindings
                .function(FunctionDeclaration::<(), bool>::new(name))
                .unwrap(),
            format!(
                "host function native_generic_results_fixture::native_generic_results_fixture.{native} failed: {reason}"
            ),
        )
    });
    let mut module = bindings.seal().unwrap();
    let executor = tokio::runtime::Builder::new_current_thread()
        .build()
        .unwrap();
    let host = geam::execution::TokioHost::new(executor.handle().clone());
    let mut state = State {
        stdlib: GleamStdlibRunState::from_seed([0; 32]),
        erlang: Configuration { resources },
        provider: (),
    };
    let expected = &raw mut state.provider;
    assert!(std::ptr::eq(
        <Component as geam::HostProvider<Profile>>::project(&mut state),
        expected
    ));
    let stores = Stores::default();
    assert!(std::ptr::eq(
        <Profile as HostComponentProfile<Component>>::component_stores(&stores),
        &stores.provider
    ));
    let mut echo = Vec::new();
    for _ in 0..2 {
        executor
            .block_on(
                module.with_execution(&host, &mut state, &mut echo, async |scope| {
                    for (function, reason) in &rejected {
                        let error = scope.call(function, ()).await.unwrap_err();
                        assert_eq!(error.to_string(), *reason);
                    }
                    assert!(scope.call(&check, ()).await.unwrap());
                }),
            )
            .unwrap()
            .try_into_value()
            .unwrap();
    }
    assert!(echo.is_empty());
    assert!(state.stdlib.io_outputs().is_empty());
}
