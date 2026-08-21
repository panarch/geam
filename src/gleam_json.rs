mod function;
mod schema;
mod storage;

use self::function::{
    JsonProvider, decode_to_dynamic, do_bool, do_float, do_int, do_null, do_object,
    do_preprocessed_array, do_string, do_to_string, to_string_tree,
};
use self::schema::{DecodeConstructions, Json, JsonDynamicResult, JsonList, ObjectEntries};
use crate::{
    BitArrayValue, HostComponentProfile, HostProfile, HostProviderComponent,
    HostProviderComponentRegistration, HostProviderModule, HostRegistrationError,
};
use ecow::EcoString;
use geam_stdlib::provider_support::StringTree;
use geam_stdlib::{
    Component as GleamStdlibComponent, GleamStdlibHostProfile, GleamStdlibRunState,
    GleamStdlibStores, IoOutput,
};
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
            provider.with_scoped_function::<JsonProvider<Profile>, (Json,), StringTree, _>(
                "to_string_tree",
                to_string_tree::<Profile>,
            )
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
    use crate::{HostComponentProfile, HostProviderComponent, HostProviderComponentRegistration};
    use geam_stdlib::{Component as GleamStdlibComponent, GleamStdlibRunState};

    #[test]
    #[should_panic(expected = "decode_to_dynamic should return Result")]
    fn dynamic_result_assertion_rejects_non_result_types() {
        assert_dynamic_result(&crate::ValueType::Nil);
    }

    #[test]
    #[should_panic(expected = "decode_to_dynamic error should be DecodeError")]
    fn dynamic_result_assertion_rejects_non_custom_error_types() {
        assert_decode_error(&crate::ValueType::Nil);
    }

    #[test]
    #[should_panic(expected = "expected external type")]
    fn external_type_assertion_rejects_non_external_types() {
        assert_external_type(&crate::ValueType::Nil, "package", "module", "Type", &[]);
    }

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

        let functions = provider.functions().collect::<Vec<_>>();
        for function in &functions {
            assert!(function.scheme().parameters().is_empty());
        }

        let json = functions[7].type_().return_().clone();
        assert_external_type(&json, "gleam_json", "gleam/json", "Json", &[]);
        assert_eq!(
            functions[0].type_().argument_types(),
            [crate::ValueType::BitArray]
        );
        assert_dynamic_result(functions[0].type_().return_());
        assert_eq!(
            functions[1].type_().argument_types(),
            std::slice::from_ref(&json),
        );
        assert_eq!(functions[1].type_().return_(), &crate::ValueType::String);
        assert_eq!(
            functions[2].type_().argument_types(),
            std::slice::from_ref(&json),
        );
        assert_external_type(
            functions[2].type_().return_(),
            "gleam_stdlib",
            "gleam/string_tree",
            "StringTree",
            &[],
        );
        assert_eq!(
            functions[3].type_().argument_types(),
            [crate::ValueType::String]
        );
        assert_eq!(
            functions[4].type_().argument_types(),
            [crate::ValueType::Bool]
        );
        assert_eq!(
            functions[5].type_().argument_types(),
            [crate::ValueType::Int]
        );
        assert_eq!(
            functions[6].type_().argument_types(),
            [crate::ValueType::Float]
        );
        assert!(functions[7].type_().argument_types().is_empty());
        for function in &functions[3..] {
            assert_eq!(function.type_().return_(), &json);
        }
        assert_eq!(
            functions[8].type_().argument_types(),
            [crate::ValueType::List(Box::new(crate::ValueType::Tuple(
                vec![crate::ValueType::String, json.clone(),]
            )))],
        );
        assert_eq!(
            functions[9].type_().argument_types(),
            [crate::ValueType::List(Box::new(json))],
        );
    }

    fn assert_dynamic_result(type_: &crate::ValueType) {
        let crate::ValueType::Custom(result) = type_ else {
            panic!("decode_to_dynamic should return Result: {type_:?}");
        };
        assert_eq!(result.type_name().package(), "");
        assert_eq!(result.type_name().module(), "gleam");
        assert_eq!(result.type_name().name(), "Result");
        assert_eq!(result.arguments().len(), 2);
        assert_external_type(
            &result.arguments()[0],
            "gleam_stdlib",
            "gleam/dynamic",
            "Dynamic",
            &[],
        );
        assert_decode_error(&result.arguments()[1]);
    }

    fn assert_decode_error(type_: &crate::ValueType) {
        let crate::ValueType::Custom(error) = type_ else {
            panic!("decode_to_dynamic error should be DecodeError");
        };
        assert_eq!(error.type_name().package(), "gleam_json");
        assert_eq!(error.type_name().module(), "gleam/json");
        assert_eq!(error.type_name().name(), "DecodeError");
        assert!(error.arguments().is_empty());
    }

    fn assert_external_type(
        type_: &crate::ValueType,
        package: &str,
        module: &str,
        name: &str,
        arguments: &[crate::ValueType],
    ) {
        let crate::ValueType::External(type_) = type_ else {
            panic!("expected external type {package}::{module}.{name}: {type_:?}");
        };
        assert_eq!(type_.type_name().package(), package);
        assert_eq!(type_.type_name().module(), module);
        assert_eq!(type_.type_name().name(), name);
        assert_eq!(type_.arguments(), arguments);
    }
}
