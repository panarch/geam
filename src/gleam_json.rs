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
mod test_support;

#[cfg(test)]
mod tests {
    use super::test_support::{CustomProfile, CustomRunState, CustomStores};
    use super::{
        Component, GleamJsonProfile, GleamJsonProfileStores, GleamJsonRunState, host_providers,
        json_stores,
    };
    use crate::gleam_stdlib::{Component as GleamStdlibComponent, GleamStdlibRunState};
    use crate::{HostComponentProfile, HostProviderComponent, HostProviderComponentRegistration};

    #[test]
    fn default_and_custom_profiles_project_independent_stdlib_and_json_components() {
        let default = GleamJsonProfileStores::default();
        let custom = CustomStores::default();

        assert!(std::ptr::eq(
            <GleamJsonProfile as HostComponentProfile<GleamStdlibComponent>>::component_stores(
                &default,
            ),
            &default.stdlib,
        ));
        assert!(std::ptr::eq(
            json_stores::<GleamJsonProfile>(&default),
            &default.json,
        ));
        assert!(std::ptr::eq(
            <CustomProfile as HostComponentProfile<GleamStdlibComponent>>::component_stores(
                &custom,
            ),
            &custom.stdlib,
        ));
        assert!(std::ptr::eq(
            json_stores::<CustomProfile>(&custom),
            &custom.json,
        ));

        let mut default_state = GleamJsonRunState::new(GleamStdlibRunState::from_seed([1; 32]));
        let mut custom_state = CustomRunState {
            stdlib: GleamStdlibRunState::from_seed([2; 32]),
            json: (),
        };
        let default_pointer = default_state.stdlib_mut() as *mut GleamStdlibRunState;
        assert!(std::ptr::eq(
            <GleamJsonProfile as HostComponentProfile<GleamStdlibComponent>>::component_state(
                &mut default_state,
            ),
            default_pointer,
        ));
        let custom_pointer = &mut custom_state.stdlib as *mut GleamStdlibRunState;
        assert!(std::ptr::eq(
            <CustomProfile as HostComponentProfile<GleamStdlibComponent>>::component_state(
                &mut custom_state,
            ),
            custom_pointer,
        ));
        let default_json = &mut default_state.json as *mut ();
        assert!(std::ptr::eq(
            <GleamJsonProfile as HostComponentProfile<Component>>::component_state(
                &mut default_state,
            ),
            default_json,
        ));
        let custom_json = &mut custom_state.json as *mut ();
        assert!(std::ptr::eq(
            <CustomProfile as HostComponentProfile<Component>>::component_state(&mut custom_state),
            custom_json,
        ));
        assert!(default_state.stdlib().io_outputs().is_empty());
        assert!(custom_state.stdlib.io_outputs().is_empty());
    }

    #[test]
    fn registers_only_the_exact_official_erlang_json_provider_inventory() {
        assert_eq!(<Component as HostProviderComponent>::ID, "gleam_json");
        let mut providers =
            <Component as HostProviderComponentRegistration<GleamJsonProfile>>::providers()
                .expect("JSON component should register");
        let facade =
            host_providers::<GleamJsonProfile>().expect("official JSON provider should register");
        assert_eq!(facade.len(), providers.len());
        assert_eq!(facade[0].module(), providers[0].module());
        assert_eq!(providers.len(), 1);
        let provider = providers
            .pop()
            .expect("JSON package should have one provider module");

        assert_eq!(provider.package(), "gleam_json");
        assert_eq!(provider.module(), "gleam/json");
        assert_eq!(
            provider
                .external_types()
                .map(|schema| (schema.name().as_str(), schema.parameter_count()))
                .collect::<Vec<_>>(),
            [("Json", 0)],
        );
        assert_eq!(
            provider
                .functions()
                .map(|function| function.name().as_str())
                .collect::<Vec<_>>(),
            [
                "decode_to_dynamic",
                "do_to_string",
                "to_string_tree",
                "do_string",
                "do_bool",
                "do_int",
                "do_float",
                "do_null",
                "do_object",
                "do_preprocessed_array",
            ],
        );

        use crate::host::HostAbiType;
        let json = <super::schema::Json as HostAbiType>::descriptor().value_type();
        let dynamic_result =
            <super::schema::JsonDynamicResult as HostAbiType>::descriptor().value_type();
        let string_tree =
            <crate::gleam_stdlib::StringTree as HostAbiType>::descriptor().value_type();
        let object_entries =
            <super::schema::ObjectEntries as HostAbiType>::descriptor().value_type();
        let json_list = <super::schema::JsonList as HostAbiType>::descriptor().value_type();
        let expected = [
            (vec![crate::ValueType::BitArray], dynamic_result),
            (vec![json.clone()], crate::ValueType::String),
            (vec![json.clone()], string_tree),
            (vec![crate::ValueType::String], json.clone()),
            (vec![crate::ValueType::Bool], json.clone()),
            (vec![crate::ValueType::Int], json.clone()),
            (vec![crate::ValueType::Float], json.clone()),
            (Vec::new(), json.clone()),
            (vec![object_entries], json.clone()),
            (vec![json_list], json),
        ];
        for (function, (arguments, return_)) in provider.functions().zip(expected) {
            assert!(function.scheme().is_monomorphic());
            assert_eq!(
                function.type_(),
                &crate::FunctionType::new(arguments, return_),
            );
        }
    }
}
