mod function;
mod schema;
mod storage;

use self::function::{
    JsonProvider, decode_to_dynamic, do_bool, do_float, do_int, do_null, do_object,
    do_preprocessed_array, do_string, do_to_string, to_string_tree,
};
use self::schema::{DecodeConstructions, Json, JsonDynamicResult, JsonList, ObjectEntries};
use crate::gleam_stdlib::{
    GleamStdlibHostProfile, GleamStdlibRunState, GleamStdlibStores, IoOutput,
};
use crate::{BitArrayValue, HostProfile, HostProviderModule, HostRegistrationError};
use ecow::EcoString;
use num_bigint::BigInt;

/// A host profile that exposes the official Gleam JSON and standard-library stores.
pub trait GleamJsonHostProfile: GleamStdlibHostProfile {
    /// Projects the JSON external stores from this profile.
    fn gleam_json_stores(stores: &Self::ExternalStores) -> &GleamJsonStores;
}

/// External value stores used by the official Gleam JSON provider.
#[derive(Default)]
pub struct GleamJsonStores {
    json: storage::Stores,
}

/// External stores for the default combined standard-library and JSON profile.
#[derive(Default)]
pub struct GleamJsonProfileStores {
    stdlib: GleamStdlibStores,
    json: GleamJsonStores,
}

/// The default profile for composing the official standard-library and JSON providers.
#[derive(Debug, Clone, Copy)]
pub struct GleamJsonProfile;

impl HostProfile for GleamJsonProfile {
    type RunState = GleamStdlibRunState;
    type ExternalStores = GleamJsonProfileStores;
}

impl GleamStdlibHostProfile for GleamJsonProfile {
    type Io = Vec<IoOutput>;

    fn gleam_stdlib_stores(stores: &Self::ExternalStores) -> &GleamStdlibStores {
        &stores.stdlib
    }

    fn gleam_stdlib_run_state(state: &mut Self::RunState) -> &mut GleamStdlibRunState {
        state
    }

    fn gleam_stdlib_io(state: &mut Self::RunState) -> &mut Self::Io {
        <crate::gleam_stdlib::GleamStdlibProfile as GleamStdlibHostProfile>::gleam_stdlib_io(state)
    }
}

impl GleamJsonHostProfile for GleamJsonProfile {
    fn gleam_json_stores(stores: &Self::ExternalStores) -> &GleamJsonStores {
        &stores.json
    }
}

/// Registers the Rust provider for the official Gleam JSON package.
pub fn host_providers<Profile>() -> Result<Vec<HostProviderModule<Profile>>, HostRegistrationError>
where
    Profile: GleamJsonHostProfile,
{
    [host_provider::<Profile>]
        .into_iter()
        .map(|register| register())
        .collect()
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
