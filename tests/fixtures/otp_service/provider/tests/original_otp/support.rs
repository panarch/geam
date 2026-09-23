//! Original OTP acquisition, public provider composition, and lifecycle observations.
#[path = "../../../../../../tests/support/execution_host.rs"]
pub(super) mod execution_fixture;

use geam::execution::{
    ExecutionServices, ExecutionUnit, ExecutionUnitId, HostExecutionState, UnitExit,
};
use geam::gleam_erlang::{Configuration, ErlangExecution, GleamErlangHostProfile};
use geam::gleam_stdlib::{GleamStdlibRunState, GleamStdlibStores, IoOutput};
use geam::{
    HostComponentProfile, HostExecutionService, HostProfile, HostProviderComponent,
    HostProviderComponentRegistration, HostProviderSet, HostServiceProfile, HostedExecution,
    compile_typed_host_project, plan_host_program,
};
use sha2::{Digest, Sha256};
type Component = geam_otp_service_fixture::Component<Profile>;

pub(super) struct Profile;

#[derive(Default)]
pub(super) struct Stores {
    stdlib: GleamStdlibStores,
    erlang: geam::gleam_erlang::Stores<Profile>,
    provider: <Component as HostProviderComponent>::Stores,
    witnesses: geam::HostExternalStore<lifetime::Witness>,
}

pub(super) struct State {
    pub(super) stdlib: GleamStdlibRunState,
    erlang: Configuration,
    pub(super) provider: geam_otp_service_fixture::State,
    pub(super) observed: std::sync::Arc<std::sync::Mutex<Observation>>,
}

#[derive(Default)]
pub(super) struct Observation {
    pub(super) units: Vec<ExecutionUnit>,
    pub(super) finished: Vec<ExecutionUnitId>,
    pub(super) closed: usize,
    pub(super) dropped_captures: usize,
    pub(super) exits: Vec<UnitExit>,
}

#[derive(Default)]
pub(super) struct Observer(std::sync::Arc<std::sync::Mutex<Observation>>);
impl HostExecutionState for Observer {
    fn started(&mut self, unit: ExecutionUnit) {
        self.0.lock().unwrap().units.push(unit);
    }
    fn finished(&mut self, unit: ExecutionUnitId, exit: &UnitExit) {
        let mut observed = self.0.lock().unwrap();
        observed.finished.push(unit);
        observed.exits.push(exit.clone());
    }
    fn close(&mut self) {
        self.0.lock().unwrap().closed += 1;
    }
}

#[path = "support/lifetime.rs"]
mod lifetime;

impl HostProfile for Profile {
    type RunState = State;
    type ExternalStores = Stores;
    type ExecutionState = ExecutionServices<ErlangExecution, Observer>;

    fn initialize_execution(state: &mut State) -> Self::ExecutionState {
        ExecutionServices {
            first:
                <geam::gleam_erlang::Component<Profile> as HostExecutionService>::initialize_service(
                    &mut state.erlang,
                ),
            rest: Observer(std::sync::Arc::clone(&state.observed)),
        }
    }
}

impl geam::gleam_stdlib::GleamStdlibHostProfile for Profile {
    type Io = Vec<IoOutput>;
}

impl HostComponentProfile<geam::gleam_stdlib::Component> for Profile {
    fn component_stores(stores: &Stores) -> &GleamStdlibStores {
        &stores.stdlib
    }
    fn component_state(state: &mut State) -> &mut GleamStdlibRunState {
        &mut state.stdlib
    }
}

impl HostComponentProfile<geam::gleam_erlang::Component<Profile>> for Profile {
    fn component_stores(stores: &Stores) -> &geam::gleam_erlang::Stores<Profile> {
        &stores.erlang
    }
    fn component_state(state: &mut State) -> &mut Configuration {
        &mut state.erlang
    }
}

impl HostComponentProfile<Component> for Profile {
    fn component_stores(stores: &Stores) -> &<Component as HostProviderComponent>::Stores {
        &stores.provider
    }
    fn component_state(state: &mut State) -> &mut geam_otp_service_fixture::State {
        &mut state.provider
    }
}

impl HostServiceProfile<geam::gleam_erlang::Component<Profile>> for Profile {
    fn service(state: &mut Self::ExecutionState) -> &mut ErlangExecution {
        &mut state.first
    }
}

impl GleamErlangHostProfile for Profile {
    fn erlang_execution(state: &mut Self::ExecutionState) -> &mut ErlangExecution {
        <Self as HostServiceProfile<geam::gleam_erlang::Component<Profile>>>::service(state)
    }
}

pub(super) fn execution(entry: &str) -> (HostedExecution<Profile>, State) {
    let root = camino::Utf8Path::new(env!("CARGO_MANIFEST_DIR")).join("../project");
    let acquisition = std::process::Command::new("gleam")
        .args(["deps", "download"])
        .current_dir(&root)
        .output()
        .unwrap();
    assert!(
        acquisition.status.success(),
        "{}",
        String::from_utf8_lossy(&acquisition.stderr)
    );
    let upstream = root.join("build/packages/gleam_otp");
    let files = include_str!("../../../upstream.sha256")
        .lines()
        .collect::<Vec<_>>();
    assert_eq!(files.len(), 30);
    for entry in files {
        let (expected, path) = entry.split_once("  ").unwrap();
        let actual = Sha256::digest(std::fs::read(upstream.join(path)).unwrap());
        assert_eq!(format!("{actual:x}"), expected, "upstream {path}");
    }
    let mut providers = geam::gleam_stdlib::host_providers::<Profile>().unwrap();
    providers.extend(geam::gleam_erlang::host_providers::<Profile>().unwrap());
    providers
        .extend(<Component as HostProviderComponentRegistration<Profile>>::providers().unwrap());
    providers.push(lifetime::provider());
    let typed = compile_typed_host_project(
        &root,
        entry,
        HostProviderSet::from_providers(providers).unwrap(),
    )
    .unwrap();
    let resources = typed.package_resources().clone();
    let execution =
        HostedExecution::try_from_module_plan(plan_host_program(typed).unwrap()).unwrap();
    let state = State {
        stdlib: GleamStdlibRunState::from_seed([0; 32]),
        erlang: Configuration { resources },
        provider: <Component as geam::HostProviderComponentInitialization>::initialize(
            &geam::HostProviderConfiguration::empty(),
        )
        .unwrap(),
        observed: Default::default(),
    };
    (execution, state)
}
