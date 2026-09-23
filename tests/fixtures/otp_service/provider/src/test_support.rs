//! Execution profile and observations shared by owner-local unit tests.
#[path = "../../../../../tests/support/execution_host.rs"]
mod execution_fixture;

use geam::execution::{
    ExecutionServices, ExecutionUnit, ExecutionUnitId, HostExecutionState, UnitExit,
};
use geam::gleam_erlang::{Configuration, ErlangExecution, GleamErlangHostProfile};
use geam::gleam_stdlib::{GleamStdlibRunState, GleamStdlibStores, IoOutput};
use geam::{
    HostComponentProfile, HostExecutionService, HostProfile, HostProviderComponent,
    HostProviderSet, HostServiceProfile, HostedExecution, Value, plan_host_program,
};
type Component = crate::Component<Profile>;

pub(crate) struct Profile;

#[derive(Default)]
pub(crate) struct Stores {
    stdlib: GleamStdlibStores,
    erlang: geam::gleam_erlang::Stores<Profile>,
    provider: <Component as HostProviderComponent>::Stores,
}

pub(crate) struct State {
    stdlib: GleamStdlibRunState,
    erlang: Configuration,
    provider: crate::State,
    observed: std::sync::Arc<std::sync::Mutex<Observation>>,
}

#[derive(Default)]
struct Observation {
    exits: Vec<UnitExit>,
}

#[derive(Default)]
pub(crate) struct Observer(std::sync::Arc<std::sync::Mutex<Observation>>);
impl HostExecutionState for Observer {
    fn started(&mut self, _unit: ExecutionUnit) {}
    fn finished(&mut self, _unit: ExecutionUnitId, exit: &UnitExit) {
        self.0.lock().unwrap().exits.push(exit.clone());
    }
    fn close(&mut self) {}
}

pub(crate) mod native_boundary;
pub(crate) mod storage;

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
    fn component_state(state: &mut State) -> &mut crate::State {
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

pub(crate) fn run_source(
    packages: impl IntoIterator<Item = geam::PackageSource>,
    providers: impl IntoIterator<Item = geam::HostProviderModule<Profile>>,
) -> Result<Value, geam::execution::RunError> {
    run_observed_source(packages, providers).0
}

pub(crate) fn run_observed_source(
    packages: impl IntoIterator<Item = geam::PackageSource>,
    providers: impl IntoIterator<Item = geam::HostProviderModule<Profile>>,
) -> (Result<Value, geam::execution::RunError>, Vec<UnitExit>) {
    let typed = geam::compile_typed_host_program(
        "application",
        "main",
        packages,
        HostProviderSet::from_providers(providers).unwrap(),
    )
    .unwrap();
    let mut execution =
        HostedExecution::try_from_module_plan(plan_host_program(typed).unwrap()).unwrap();
    let host = execution_fixture::TestHost::default();
    let mut state = State {
        stdlib: GleamStdlibRunState::from_seed([0; 32]),
        erlang: Configuration::default(),
        provider: <Component as geam::HostProviderComponentInitialization>::initialize(
            &geam::HostProviderConfiguration::empty(),
        )
        .unwrap(),
        observed: Default::default(),
    };
    let mut echo = Vec::new();
    let result = host.block_on(execution.run_main(&host, &mut state, &mut echo));
    assert!(echo.is_empty());
    let exits = state.observed.lock().unwrap().exits.clone();
    (result, exits)
}

pub(crate) fn host_failure(result: Result<Value, geam::execution::RunError>) -> Option<String> {
    match result {
        Err(geam::execution::RunError::Execution(geam::ExecutionError::Host(error))) => {
            Some(error.failure().message().to_string())
        }
        _ => None,
    }
}

pub(crate) fn erase_native<'call>(
    mut call: geam::host::HostCall<
        'call,
        Profile,
        geam::gleam_erlang::Component<Profile>,
        geam::gleam_stdlib::provider_support::Dynamic,
    >,
    value: geam::host::HostValue<'call, crate::A>,
) -> Result<
    geam::host::HostCallCompletion<'call, geam::gleam_stdlib::provider_support::Dynamic>,
    geam::host::HostCallError,
> {
    let value = call.native_value::<crate::A>(value);
    let value = call.create_external(geam::gleam_stdlib::Dynamic::from_native(value));
    Ok(call.return_value(value))
}

#[cfg(test)]
mod tests {
    use super::{Component, Profile, State, host_failure, run_source};
    use geam::gleam_erlang::Configuration;
    use geam::gleam_stdlib::GleamStdlibRunState;
    use geam::{HostComponentProfile, ModuleSource, PackageSource, Value};

    #[test]
    fn fixture_profile_projects_the_original_independent_component_states() {
        let mut state = State {
            stdlib: GleamStdlibRunState::from_seed([7; 32]),
            erlang: Configuration::default(),
            provider: crate::State::default(),
            observed: Default::default(),
        };
        let stdlib = &raw const state.stdlib;
        assert!(std::ptr::eq(
            <Profile as HostComponentProfile<geam::gleam_stdlib::Component>>::component_state(
                &mut state
            ),
            stdlib,
        ));
        <Profile as HostComponentProfile<geam::gleam_erlang::Component<Profile>>>::component_state(
            &mut state,
        )
        .resources
        .insert("owner".into(), "owner/resources".into());
        <Profile as HostComponentProfile<Component>>::component_state(&mut state)
            .warnings
            .push("owner warning".into());
        assert_eq!(state.provider.warnings, ["owner warning"]);
        assert_eq!(
            state.erlang.resources.into_iter().collect::<Vec<_>>(),
            [("owner".into(), "owner/resources".into())],
        );
        assert!(state.stdlib.io_outputs().is_empty());
    }

    #[test]
    fn owner_execution_preserves_values_and_source_panics_as_non_host_results() {
        let value = run_source(
            [PackageSource::new(
                "application",
                Vec::<String>::new(),
                [ModuleSource::new(
                    "main",
                    "main.gleam",
                    "pub fn main() { 17 }",
                )],
            )],
            [],
        );
        assert_eq!(value.as_ref().unwrap(), &Value::Int(17.into()));
        assert_eq!(host_failure(value), None);

        let failure = run_source(
            [PackageSource::new(
                "application",
                Vec::<String>::new(),
                [ModuleSource::new(
                    "main",
                    "main.gleam",
                    "pub fn main() { panic as \"owner source failure\" }",
                )],
            )],
            [],
        );
        assert_eq!(
            failure.as_ref().err().unwrap().to_string(),
            "panic: owner source failure",
        );
        assert_eq!(host_failure(failure), None);
    }
}
