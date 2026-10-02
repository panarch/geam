use super::Component;
use geam::execution::ExecutionServices;
use geam::gleam_erlang::{Configuration, ErlangExecution, GleamErlangHostProfile};
use geam::gleam_stdlib::{GleamStdlibRunState, GleamStdlibStores, IoOutput};
use geam::{
    HostComponentProfile, HostExecutionService, HostProfile, HostProviderComponent,
    HostProviderComponentRegistration, HostProviderSet, HostServiceProfile,
};

pub struct Profile;

#[derive(Default)]
pub struct Stores {
    stdlib: GleamStdlibStores,
    erlang: geam::gleam_erlang::Stores<Profile>,
    provider: <Component as HostProviderComponent>::Stores,
}

pub struct State {
    pub stdlib: GleamStdlibRunState,
    pub erlang: Configuration,
    pub provider: (),
}

impl HostProfile for Profile {
    type RunState = State;
    type ExternalStores = Stores;
    type ExecutionState = ExecutionServices<ErlangExecution, ()>;

    fn initialize_execution(state: &mut State) -> Self::ExecutionState {
        ExecutionServices {
            first:
                <geam::gleam_erlang::Component<Profile> as HostExecutionService>::initialize_service(
                    &mut state.erlang,
                ),
            rest: (),
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
    fn component_state(state: &mut State) -> &mut () {
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

pub fn providers() -> HostProviderSet<Profile> {
    let mut providers = geam::gleam_stdlib::host_providers::<Profile>().unwrap();
    providers.extend(geam::gleam_erlang::host_providers::<Profile>().unwrap());
    providers
        .extend(<Component as HostProviderComponentRegistration<Profile>>::providers().unwrap());
    HostProviderSet::from_providers(providers).unwrap()
}
