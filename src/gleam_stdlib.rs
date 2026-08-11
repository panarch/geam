use crate::{
    HostComponentProfile, HostProfile, HostProviderComponent, HostProviderComponentRegistration,
    HostProviderModule, HostRegistrationError,
};
use std::marker::PhantomData;

mod bit_array;
mod dict;
mod dynamic;
mod dynamic_decode;
mod float;
mod int;
mod io;
mod option;
mod result;
mod run_state;
mod string;
mod string_tree;
mod uri;

pub use io::{IoOutput, IoSink, IoStream};
pub use run_state::{GleamStdlibRunState, GleamStdlibRunStateError};

pub(crate) use dict::{DictExternalStorage, DictOf, DictSchema, create_dynamic_dict};
pub(crate) use dynamic::{
    Dynamic, DynamicExternalStorage, DynamicSchema, create_value as create_dynamic_value,
};
pub(crate) use dynamic_decode::DynamicDecodeError;
pub(crate) use result::{GleamError, GleamOk, GleamResult};
pub(crate) use string_tree::{
    StoredStringTree, StringTree, StringTreeExternalStorage, StringTreePayload, StringTreeSchema,
};

/// A host profile that exposes state and storage for the official Gleam standard library.
pub trait GleamStdlibHostProfile: HostComponentProfile<Component<Self::Io>> {
    /// The concrete caller-owned sink used by official Gleam IO functions.
    type Io: IoSink + 'static;
}

/// External value stores used by the official Gleam standard library providers.
#[derive(Default)]
pub struct GleamStdlibStores {
    dict: dict::Stores,
    dynamic: dynamic::Stores,
    string_tree: string_tree::Stores,
}

/// The statically composed provider component for the official Gleam standard library.
#[derive(Debug, Clone, Copy)]
pub struct Component<Io = Vec<IoOutput>>(PhantomData<fn() -> Io>);

impl<Io> HostProviderComponent for Component<Io>
where
    Io: IoSink + 'static,
{
    const ID: &'static str = "gleam_stdlib";
    type Stores = GleamStdlibStores;
    type RunState = GleamStdlibRunState<Io>;
}

/// The default profile for using only the official Gleam standard library providers.
#[derive(Debug, Clone, Copy)]
pub struct GleamStdlibProfile;

impl HostProfile for GleamStdlibProfile {
    type RunState = GleamStdlibRunState;
    type ExternalStores = GleamStdlibStores;
}

impl HostComponentProfile<Component> for GleamStdlibProfile {
    fn component_stores(stores: &Self::ExternalStores) -> &GleamStdlibStores {
        stores
    }

    fn component_state(state: &mut Self::RunState) -> &mut GleamStdlibRunState {
        state
    }
}

impl GleamStdlibHostProfile for GleamStdlibProfile {
    type Io = Vec<IoOutput>;
}

/// Registers the Rust providers for the official Gleam standard library.
pub fn host_providers<Profile>() -> Result<Vec<HostProviderModule<Profile>>, HostRegistrationError>
where
    Profile: GleamStdlibHostProfile,
{
    <Component<Profile::Io> as HostProviderComponentRegistration<Profile>>::providers()
}

impl<Profile, Io> HostProviderComponentRegistration<Profile> for Component<Io>
where
    Profile: GleamStdlibHostProfile<Io = Io>,
    Io: IoSink + 'static,
{
    fn providers() -> Result<Vec<HostProviderModule<Profile>>, HostRegistrationError> {
        register_host_providers::<Profile>()
    }
}

pub(crate) fn stdlib_stores<Profile>(stores: &Profile::ExternalStores) -> &GleamStdlibStores
where
    Profile: GleamStdlibHostProfile,
{
    <Profile as HostComponentProfile<Component<Profile::Io>>>::component_stores(stores)
}

pub(crate) fn stdlib_state<Profile>(
    state: &mut Profile::RunState,
) -> &mut GleamStdlibRunState<Profile::Io>
where
    Profile: GleamStdlibHostProfile,
{
    <Profile as HostComponentProfile<Component<Profile::Io>>>::component_state(state)
}

fn register_host_providers<Profile>()
-> Result<Vec<HostProviderModule<Profile>>, HostRegistrationError>
where
    Profile: GleamStdlibHostProfile,
{
    let registrations: [ProviderRegistration<Profile>; 10] = [
        dict::host_provider::<Profile>,
        dynamic::host_provider::<Profile>,
        float::host_provider::<Profile>,
        int::host_provider::<Profile>,
        string_tree::host_provider::<Profile>,
        string::host_provider::<Profile>,
        bit_array::host_provider::<Profile>,
        dynamic_decode::host_provider::<Profile>,
        io::host_provider::<Profile>,
        uri::host_provider::<Profile>,
    ];

    registrations
        .into_iter()
        .map(|register| register())
        .collect()
}

type ProviderRegistration<Profile> =
    fn() -> Result<HostProviderModule<Profile>, HostRegistrationError>;

#[cfg(test)]
mod tests {
    use super::{
        Component, GleamStdlibHostProfile, GleamStdlibProfile, GleamStdlibRunState,
        GleamStdlibStores, IoOutput, IoSink, IoStream, host_providers, stdlib_state, stdlib_stores,
    };
    use crate::{
        HostComponentProfile, HostProfile, HostProviderComponent, HostProviderComponentRegistration,
    };

    struct CustomProfile;

    #[derive(Default)]
    struct CustomStores {
        stdlib: GleamStdlibStores,
    }

    struct CustomRunState {
        stdlib: GleamStdlibRunState<RecordingSink>,
    }

    #[derive(Default)]
    struct RecordingSink {
        outputs: Vec<IoOutput>,
    }

    impl IoSink for RecordingSink {
        fn emit(&mut self, output: IoOutput) {
            self.outputs.push(output);
        }
    }

    impl HostProfile for CustomProfile {
        type RunState = CustomRunState;
        type ExternalStores = CustomStores;
    }

    impl HostComponentProfile<Component<RecordingSink>> for CustomProfile {
        fn component_stores(stores: &Self::ExternalStores) -> &GleamStdlibStores {
            &stores.stdlib
        }

        fn component_state(state: &mut Self::RunState) -> &mut GleamStdlibRunState<RecordingSink> {
            &mut state.stdlib
        }
    }

    impl GleamStdlibHostProfile for CustomProfile {
        type Io = RecordingSink;
    }

    #[test]
    fn registers_providers_in_dependency_first_module_order() {
        assert_eq!(<Component as HostProviderComponent>::ID, "gleam_stdlib");
        let providers =
            <Component as HostProviderComponentRegistration<GleamStdlibProfile>>::providers()
                .expect("stdlib component should register");
        let facade = host_providers::<GleamStdlibProfile>()
            .expect("official stdlib provider facade should register");
        assert_eq!(
            facade
                .iter()
                .map(|provider| provider.module().as_str())
                .collect::<Vec<_>>(),
            providers
                .iter()
                .map(|provider| provider.module().as_str())
                .collect::<Vec<_>>(),
        );

        assert_eq!(
            providers
                .iter()
                .map(|provider| provider.module().as_str())
                .collect::<Vec<_>>(),
            [
                "gleam/dict",
                "gleam/dynamic",
                "gleam/float",
                "gleam/int",
                "gleam/string_tree",
                "gleam/string",
                "gleam/bit_array",
                "gleam/dynamic/decode",
                "gleam/io",
                "gleam/uri",
            ],
        );
        let provider = &providers[0];
        assert_eq!(provider.package(), "gleam_stdlib");
        assert_eq!(provider.module(), "gleam/dict");
        assert_eq!(
            provider
                .external_types()
                .map(|schema| {
                    (
                        schema.package().as_str(),
                        schema.module().as_str(),
                        schema.name().as_str(),
                        schema.parameter_count(),
                    )
                })
                .collect::<Vec<_>>(),
            [
                ("gleam_stdlib", "gleam/dict", "Dict", 2),
                ("gleam_stdlib", "gleam/dict", "TransientDict", 2),
            ],
        );
        assert_eq!(
            provider
                .functions()
                .map(|function| function.name().as_str())
                .collect::<Vec<_>>(),
            [
                "to_transient",
                "from_transient",
                "size",
                "do_has_key",
                "new",
                "get",
                "do_insert",
                "transient_insert",
                "do_map_values",
                "transient_delete",
                "do_fold",
                "transient_update_with",
            ],
        );
    }

    #[test]
    fn custom_profiles_project_stdlib_stores_state_and_io() {
        let default_stores = GleamStdlibStores::default();
        let stores = CustomStores::default();
        let mut default_state = GleamStdlibRunState::from_seed([1; 32]);
        let mut state = CustomRunState {
            stdlib: GleamStdlibRunState::from_seed_with_io([2; 32], RecordingSink::default()),
        };

        assert!(std::ptr::eq(
            stdlib_stores::<GleamStdlibProfile>(&default_stores),
            &default_stores,
        ));
        assert!(std::ptr::eq(
            stdlib_stores::<CustomProfile>(&stores),
            &stores.stdlib,
        ));
        let default_state_pointer = &mut default_state as *mut GleamStdlibRunState;
        assert!(std::ptr::eq(
            stdlib_state::<GleamStdlibProfile>(&mut default_state),
            default_state_pointer,
        ));
        let state_pointer = &mut state.stdlib as *mut GleamStdlibRunState<RecordingSink>;
        assert!(std::ptr::eq(
            stdlib_state::<CustomProfile>(&mut state),
            state_pointer,
        ));

        let default_io = stdlib_state::<GleamStdlibProfile>(&mut default_state).io_sink();
        default_io.emit(IoOutput::new(IoStream::Stdout, "default".into()));
        assert_eq!(default_state.io_outputs()[0].text(), "default");

        let custom_io = stdlib_state::<CustomProfile>(&mut state).io_sink();
        custom_io.emit(IoOutput::new(IoStream::Stderr, "custom".into()));
        assert_eq!(state.stdlib.io_sink().outputs[0].stream(), IoStream::Stderr);
        assert_eq!(state.stdlib.io_sink().outputs[0].text(), "custom");
    }
}
