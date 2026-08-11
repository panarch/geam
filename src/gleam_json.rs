mod function;
mod schema;
mod storage;

use self::function::{
    JsonProvider, decode_to_dynamic, do_bool, do_float, do_int, do_null, do_object,
    do_preprocessed_array, do_string, do_to_string, to_string_tree,
};
use self::schema::{DecodeConstructions, Json, JsonDynamicResult, JsonList, ObjectEntries};
use crate::gleam_stdlib::{
    Component as GleamStdlibComponent, GleamStdlibHostProfile, GleamStdlibRunState,
    GleamStdlibStores, IoOutput,
};
use crate::{
    BitArrayValue, HostComponentProfile, HostProfile, HostProviderComponent,
    HostProviderComponentRegistration, HostProviderModule, HostRegistrationError,
};
use ecow::EcoString;
use num_bigint::BigInt;

/// A host profile that composes the official Gleam JSON and standard-library components.
pub trait GleamJsonHostProfile: GleamStdlibHostProfile + HostComponentProfile<Component> {}

impl<Profile> GleamJsonHostProfile for Profile where
    Profile: GleamStdlibHostProfile + HostComponentProfile<Component>
{
}

/// External value stores used by the official Gleam JSON provider.
#[derive(Default)]
pub struct GleamJsonStores {
    json: storage::Stores,
}

/// The statically composed provider component for the official Gleam JSON package.
#[derive(Debug, Clone, Copy)]
pub struct Component;

impl HostProviderComponent for Component {
    const ID: &'static str = "gleam_json";
    type Stores = GleamJsonStores;
    type RunState = ();
}

/// External stores for the default combined standard-library and JSON profile.
#[derive(Default)]
pub struct GleamJsonProfileStores {
    stdlib: GleamStdlibStores,
    json: GleamJsonStores,
}

/// Caller-owned run state for the default combined standard-library and JSON profile.
pub struct GleamJsonRunState {
    stdlib: GleamStdlibRunState,
    json: (),
}

/// The default profile for composing the official standard-library and JSON providers.
#[derive(Debug, Clone, Copy)]
pub struct GleamJsonProfile;

impl GleamJsonRunState {
    /// Combines caller-owned standard-library state with the stateless JSON component.
    pub fn new(stdlib: GleamStdlibRunState) -> Self {
        Self { stdlib, json: () }
    }

    /// Returns the standard-library state.
    pub fn stdlib(&self) -> &GleamStdlibRunState {
        &self.stdlib
    }

    /// Returns mutable standard-library state.
    pub fn stdlib_mut(&mut self) -> &mut GleamStdlibRunState {
        &mut self.stdlib
    }
}

impl HostProfile for GleamJsonProfile {
    type RunState = GleamJsonRunState;
    type ExternalStores = GleamJsonProfileStores;
}

impl HostComponentProfile<GleamStdlibComponent> for GleamJsonProfile {
    fn component_stores(stores: &Self::ExternalStores) -> &GleamStdlibStores {
        &stores.stdlib
    }

    fn component_state(state: &mut Self::RunState) -> &mut GleamStdlibRunState {
        &mut state.stdlib
    }
}

impl GleamStdlibHostProfile for GleamJsonProfile {
    type Io = Vec<IoOutput>;
}

impl HostComponentProfile<Component> for GleamJsonProfile {
    fn component_stores(stores: &Self::ExternalStores) -> &GleamJsonStores {
        &stores.json
    }

    fn component_state(state: &mut Self::RunState) -> &mut () {
        &mut state.json
    }
}

/// Registers the Rust provider for the official Gleam JSON package.
pub fn host_providers<Profile>() -> Result<Vec<HostProviderModule<Profile>>, HostRegistrationError>
where
    Profile: GleamJsonHostProfile,
{
    <Component as HostProviderComponentRegistration<Profile>>::providers()
}

impl<Profile> HostProviderComponentRegistration<Profile> for Component
where
    Profile: GleamJsonHostProfile,
{
    fn providers() -> Result<Vec<HostProviderModule<Profile>>, HostRegistrationError> {
        host_provider::<Profile>().map(|provider| vec![provider])
    }
}

pub(crate) fn json_stores<Profile>(stores: &Profile::ExternalStores) -> &GleamJsonStores
where
    Profile: GleamJsonHostProfile,
{
    <Profile as HostComponentProfile<Component>>::component_stores(stores)
}

pub(crate) fn json_state<Profile>(state: &mut Profile::RunState) -> &mut ()
where
    Profile: GleamJsonHostProfile,
{
    <Profile as HostComponentProfile<Component>>::component_state(state)
}

fn host_provider<Profile>() -> Result<HostProviderModule<Profile>, HostRegistrationError>
where
    Profile: GleamJsonHostProfile,
{
    HostProviderModule::new("gleam_json", "gleam/json")
        .and_then(
            HostProviderModule::with_external_type::<JsonProvider<Profile>, schema::JsonSchema>,
        )
        .and_then(|provider| {
            provider.with_scoped_function_and_constructions::<
                JsonProvider<Profile>,
                (BitArrayValue,),
                JsonDynamicResult,
                DecodeConstructions,
                _,
            >("decode_to_dynamic", decode_to_dynamic::<Profile>)
        })
        .and_then(|provider| {
            provider.with_scoped_function::<JsonProvider<Profile>, (Json,), EcoString, _>(
                "do_to_string",
                do_to_string::<Profile>,
            )
        })
        .and_then(|provider| {
            provider.with_scoped_function::<
                JsonProvider<Profile>,
                (Json,),
                crate::gleam_stdlib::StringTree,
                _,
            >("to_string_tree", to_string_tree::<Profile>)
        })
        .and_then(|provider| {
            provider.with_scoped_function::<JsonProvider<Profile>, (EcoString,), Json, _>(
                "do_string",
                do_string::<Profile>,
            )
        })
        .and_then(|provider| {
            provider.with_scoped_function::<JsonProvider<Profile>, (bool,), Json, _>(
                "do_bool",
                do_bool::<Profile>,
            )
        })
        .and_then(|provider| {
            provider.with_scoped_function::<JsonProvider<Profile>, (BigInt,), Json, _>(
                "do_int",
                do_int::<Profile>,
            )
        })
        .and_then(|provider| {
            provider.with_scoped_function::<JsonProvider<Profile>, (f64,), Json, _>(
                "do_float",
                do_float::<Profile>,
            )
        })
        .and_then(|provider| {
            provider.with_scoped_function::<JsonProvider<Profile>, (), Json, _>(
                "do_null",
                do_null::<Profile>,
            )
        })
        .and_then(|provider| {
            provider.with_scoped_function::<JsonProvider<Profile>, (ObjectEntries,), Json, _>(
                "do_object",
                do_object::<Profile>,
            )
        })
        .and_then(|provider| {
            provider.with_scoped_function::<JsonProvider<Profile>, (JsonList,), Json, _>(
                "do_preprocessed_array",
                do_preprocessed_array::<Profile>,
            )
        })
}

#[cfg(test)]
mod tests;
