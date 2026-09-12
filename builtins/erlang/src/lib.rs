//! Rust process services and native values for the `gleam_erlang` package.

mod application;
mod atom;
mod charlist;
mod execution;
mod node;
mod process;
mod reference;
mod schema;
mod selector;

#[cfg(test)]
#[path = "../../../tests/support/execution_host.rs"]
mod execution_fixture;

#[cfg(test)]
mod test_support;

pub use execution::ErlangExecution;
pub use schema::{Atom, AtomSchema, Pid, PidSchema, Reference, ReferenceSchema};

use ecow::EcoString;
use geam_core::host::{
    HostComponentProfile, HostExternalStore, HostProfile, HostProvider, HostProviderComponent,
    HostProviderComponentRegistration, HostProviderModule, HostRegistrationError,
};
use geam_core::provider::advanced::NativeValue;
use geam_stdlib::{GleamStdlibProviderProfile, GleamStdlibRunState, GleamStdlibStores, IoOutput};
use std::collections::BTreeMap;
use std::marker::PhantomData;
use std::path::PathBuf;

/// Resolved application resources supplied by the loader or a source-only host.
#[derive(Default)]
pub struct Configuration {
    pub resources: BTreeMap<EcoString, PathBuf>,
}

/// The statically composed provider component for `gleam_erlang`.
pub struct Component<Profile>(PhantomData<fn() -> Profile>);

/// Profile-local stores. Process lifetimes belong to `ErlangExecution` instead.
pub struct Stores<Profile: HostProfile> {
    native: HostExternalStore<NativeValue>,
    references: HostExternalStore<reference::Payload>,
    pids: HostExternalStore<geam_core::execution::ExecutionUnit>,
    names: HostExternalStore<EcoString>,
    charlists:
        HostExternalStore<geam_core::host::HostStoredValue<geam_core::host::HostListType<char>>>,
    selectors: HostExternalStore<selector::Selector<Profile>>,
    ports: HostExternalStore<std::convert::Infallible>,
}

impl<Profile: HostProfile> Default for Stores<Profile> {
    fn default() -> Self {
        Self {
            native: HostExternalStore::default(),
            references: HostExternalStore::default(),
            pids: HostExternalStore::default(),
            names: HostExternalStore::default(),
            charlists: HostExternalStore::default(),
            selectors: HostExternalStore::default(),
            ports: HostExternalStore::default(),
        }
    }
}

impl<Profile: HostProfile> HostProviderComponent for Component<Profile> {
    const ID: &'static str = "gleam_erlang";
    type Stores = Stores<Profile>;
    type RunState = Configuration;
}

/// Composes the process component with its domain-owned execution services.
pub trait GleamErlangHostProfile:
    GleamStdlibProviderProfile + HostComponentProfile<Component<Self>> + Sized
{
    fn erlang_execution(state: &mut Self::ExecutionState) -> &mut ErlangExecution;
}

impl<Profile: GleamErlangHostProfile> HostProvider<Profile> for Component<Profile> {
    type State = Configuration;

    fn project(state: &mut Profile::RunState) -> &mut Configuration {
        <Profile as HostComponentProfile<Self>>::component_state(state)
    }
}

/// Registers the native implementations for the official Erlang package.
pub fn host_providers<Profile: GleamErlangHostProfile>()
-> Result<Vec<HostProviderModule<Profile>>, HostRegistrationError> {
    <Component<Profile> as HostProviderComponentRegistration<Profile>>::providers()
}

impl<Profile: GleamErlangHostProfile> HostProviderComponentRegistration<Profile>
    for Component<Profile>
{
    fn providers() -> Result<Vec<HostProviderModule<Profile>>, HostRegistrationError> {
        [
            application::host_provider::<Profile>,
            atom::host_provider::<Profile>,
            charlist::host_provider::<Profile>,
            node::host_provider::<Profile>,
            process::port_provider::<Profile>,
            process::host_provider::<Profile>,
            reference::host_provider::<Profile>,
        ]
        .into_iter()
        .map(|register| register())
        .collect()
    }
}

pub struct GleamErlangProfile;

pub struct GleamErlangRunState {
    pub stdlib: GleamStdlibRunState,
    pub erlang: Configuration,
}

#[derive(Default)]
pub struct GleamErlangStores {
    stdlib: GleamStdlibStores,
    erlang: Stores<GleamErlangProfile>,
}

impl HostProfile for GleamErlangProfile {
    type RunState = GleamErlangRunState;
    type ExternalStores = GleamErlangStores;
    type ExecutionState = ErlangExecution;
}

impl geam_stdlib::GleamStdlibHostProfile for GleamErlangProfile {
    type Io = Vec<IoOutput>;
}

impl HostComponentProfile<geam_stdlib::Component> for GleamErlangProfile {
    fn component_stores(stores: &Self::ExternalStores) -> &GleamStdlibStores {
        &stores.stdlib
    }

    fn component_state(state: &mut Self::RunState) -> &mut GleamStdlibRunState {
        &mut state.stdlib
    }
}

impl HostComponentProfile<Component<Self>> for GleamErlangProfile {
    fn component_stores(stores: &Self::ExternalStores) -> &Stores<Self> {
        &stores.erlang
    }

    fn component_state(state: &mut Self::RunState) -> &mut Configuration {
        &mut state.erlang
    }
}

impl GleamErlangHostProfile for GleamErlangProfile {
    fn erlang_execution(state: &mut ErlangExecution) -> &mut ErlangExecution {
        state
    }
}

#[cfg(test)]
mod tests {
    use super::{
        Component, Configuration, ErlangExecution, GleamErlangHostProfile, GleamErlangProfile,
        GleamErlangRunState, GleamErlangStores,
    };
    use geam_core::host::HostComponentProfile;

    #[test]
    fn composed_profile_projects_original_states_stores_and_execution_services() {
        let mut state = GleamErlangRunState {
            stdlib: geam_stdlib::GleamStdlibRunState::from_seed([0; 32]),
            erlang: Configuration::default(),
        };
        let stdlib = std::ptr::from_mut(&mut state.stdlib);
        let erlang = std::ptr::from_mut(&mut state.erlang);
        assert_eq!(
            std::ptr::from_mut(<GleamErlangProfile as HostComponentProfile<
                geam_stdlib::Component,
            >>::component_state(&mut state)),
            stdlib,
        );
        assert_eq!(
            std::ptr::from_mut(<GleamErlangProfile as HostComponentProfile<
                Component<GleamErlangProfile>,
            >>::component_state(&mut state)),
            erlang,
        );
        let stores = GleamErlangStores::default();
        assert!(std::ptr::eq(
            <GleamErlangProfile as HostComponentProfile<geam_stdlib::Component>>::component_stores(
                &stores
            ),
            &stores.stdlib,
        ));
        assert!(std::ptr::eq(
            <GleamErlangProfile as HostComponentProfile<Component<GleamErlangProfile>>>::component_stores(&stores),
            &stores.erlang,
        ));
        let mut execution = ErlangExecution::default();
        let original = std::ptr::from_mut(&mut execution);
        assert_eq!(
            std::ptr::from_mut(GleamErlangProfile::erlang_execution(&mut execution)),
            original,
        );
    }
}
