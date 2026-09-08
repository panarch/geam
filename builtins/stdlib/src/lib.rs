pub(crate) use geam_core::{
    BitArrayValue, HostCall, HostComponentProfile, HostConstruction, HostExternal,
    HostExternalType, HostFailure, HostProfile, HostProvider, HostProviderComponent,
    HostProviderComponentRegistration, HostProviderModule, HostRegistrationError, HostType,
    HostTypeIndex0, HostTypeIndexNext, HostTypeList, HostTypeListEnd,
};
#[cfg(test)]
pub(crate) use geam_core::{
    ExecutionError, HostCallCompletion, HostCallError, HostExternalBinding, HostExternalEquality,
    HostExternalHashing, HostExternalInspection, HostExternalSchema, HostExternalStorage,
    HostExternalStore, HostExternalTypeSchema, HostModule, HostProviderSet, HostedExecution,
    ModuleSource, PackageSource, PanicKind, PanicMessage, Value, ValueType,
    compile_typed_host_program, plan_host_program,
};
use std::marker::PhantomData;

mod bit_array;
mod dict;
mod dynamic;
mod dynamic_decode;
mod float;
mod int;
mod io;
mod result;
mod run_state;
mod storage;
mod string;
mod string_tree;
mod uri;

pub use io::{IoOutput, IoSink, IoStream};
pub use run_state::{GleamStdlibRunState, GleamStdlibRunStateError};

/// Narrow implementation contract used by sibling official provider packages.
#[doc(hidden)]
pub mod provider_support {
    pub use crate::dict::{
        DictExternalStorage, DictOf, DictSchema, create_dynamic_dict, create_transfer_dynamic_dict,
    };
    pub use crate::dynamic::{
        Dynamic, DynamicExternalStorage, DynamicSchema,
        create_transfer_value as create_transfer_dynamic_value,
        create_value as create_dynamic_value,
    };
    pub use crate::dynamic_decode::DynamicDecodeErrorValue;
    pub use crate::result::{GleamError, GleamOk, GleamResult};
    pub use crate::storage::StorageContext;
    pub use crate::string_tree::{
        StoredStringTree, StringTree, StringTreeExternalStorage, StringTreePayload,
        StringTreeSchema,
    };
}

/// A host profile that exposes state and storage for the official Gleam standard library.
pub trait GleamStdlibHostProfile: HostProfile {
    /// The concrete caller-owned sink used by official Gleam IO functions.
    type Io: IoSink + 'static;
}

/// The standard-library capability and local-store projections used together.
#[doc(hidden)]
pub trait GleamStdlibLocalProfile:
    GleamStdlibHostProfile + HostComponentProfile<Component<Self::Io>>
{
}

impl<Profile> GleamStdlibLocalProfile for Profile where
    Profile: GleamStdlibHostProfile + HostComponentProfile<Component<Profile::Io>>
{
}

/// The standard-library capability and transferable-store projections used together.
#[doc(hidden)]
pub trait GleamStdlibTransferProfile:
    GleamStdlibHostProfile + geam_core::AsyncHostComponentProfile<Component<Self::Io>>
{
}

impl<Profile> GleamStdlibTransferProfile for Profile where
    Profile: GleamStdlibHostProfile + geam_core::AsyncHostComponentProfile<Component<Profile::Io>>
{
}

/// External value stores used by the official Gleam standard library providers.
#[derive(Default)]
pub struct GleamStdlibStores {
    dict: dict::Stores,
    dynamic: dynamic::Stores,
    string_tree: string_tree::Stores,
}

/// Standard-library stores for explicitly transferable embedding composition.
#[derive(Default)]
pub struct GleamStdlibTransferStores {
    dict: dict::TransferStores,
    dynamic: dynamic::TransferStores,
    string_tree: string_tree::TransferStores,
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

impl<Io> geam_core::__macro_support::ProviderPackage for Component<Io>
where
    Io: IoSink + 'static,
{
    const PACKAGE: &'static str = "gleam_stdlib";
}

impl<Io> geam_core::AsyncHostProviderComponent for Component<Io>
where
    Io: IoSink + 'static,
{
    type AsyncStores = GleamStdlibTransferStores;
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
    Profile: GleamStdlibLocalProfile,
{
    <Component<Profile::Io> as HostProviderComponentRegistration<Profile>>::providers()
}

/// Registers the official standard library for explicit transferable execution.
pub fn transfer_host_providers<Profile>()
-> Result<Vec<geam_core::TransferHostProviderModule<Profile>>, HostRegistrationError>
where
    Profile: GleamStdlibTransferProfile,
    Profile::RunState: Send,
{
    <Component<Profile::Io> as geam_core::TransferHostProviderComponentRegistration<Profile>>::providers()
}

impl<Profile, Io> geam_core::TransferHostProviderComponentRegistration<Profile> for Component<Io>
where
    Profile: GleamStdlibTransferProfile<Io = Io>,
    Profile::RunState: Send,
    Io: IoSink + 'static,
{
    fn providers()
    -> Result<Vec<geam_core::TransferHostProviderModule<Profile>>, HostRegistrationError> {
        let registrations: [TransferProviderRegistration<Profile>; 10] = [
            dict::transfer_host_provider::<Profile>,
            dynamic::transfer_host_provider::<Profile>,
            float::transfer_host_provider::<Profile>,
            int::transfer_host_provider::<Profile>,
            string_tree::transfer_host_provider::<Profile>,
            string::transfer_host_provider::<Profile>,
            bit_array::transfer_host_provider::<Profile>,
            dynamic_decode::transfer_host_provider::<Profile>,
            io::transfer_host_provider::<Profile>,
            uri::transfer_host_provider::<Profile>,
        ];
        registrations
            .into_iter()
            .map(|register| register())
            .collect()
    }
}

type TransferProviderRegistration<Profile> =
    fn() -> Result<geam_core::TransferHostProviderModule<Profile>, HostRegistrationError>;

impl<Profile, Io> HostProviderComponentRegistration<Profile> for Component<Io>
where
    Profile: GleamStdlibLocalProfile<Io = Io>,
    Io: IoSink + 'static,
{
    fn providers() -> Result<Vec<HostProviderModule<Profile>>, HostRegistrationError> {
        register_host_providers::<Profile>()
    }
}

fn register_host_providers<Profile>()
-> Result<Vec<HostProviderModule<Profile>>, HostRegistrationError>
where
    Profile: GleamStdlibLocalProfile,
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
        GleamStdlibStores, IoOutput, IoSink, IoStream, host_providers,
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
            <GleamStdlibProfile as HostComponentProfile<Component>>::component_stores(
                &default_stores
            ),
            &default_stores,
        ));
        assert!(std::ptr::eq(
            <CustomProfile as HostComponentProfile<Component<RecordingSink>>>::component_stores(
                &stores
            ),
            &stores.stdlib,
        ));
        let default_state_pointer = &mut default_state as *mut GleamStdlibRunState;
        assert!(std::ptr::eq(
            <GleamStdlibProfile as HostComponentProfile<Component>>::component_state(
                &mut default_state,
            ),
            default_state_pointer,
        ));
        let state_pointer = &mut state.stdlib as *mut GleamStdlibRunState<RecordingSink>;
        assert!(std::ptr::eq(
            <CustomProfile as HostComponentProfile<Component<RecordingSink>>>::component_state(
                &mut state,
            ),
            state_pointer,
        ));

        let default_io = <GleamStdlibProfile as HostComponentProfile<Component>>::component_state(
            &mut default_state,
        )
        .io_sink();
        default_io.emit(IoOutput::new(IoStream::Stdout, "default".into()));
        assert_eq!(default_state.io_outputs()[0].text(), "default");

        let custom_io =
            <CustomProfile as HostComponentProfile<Component<RecordingSink>>>::component_state(
                &mut state,
            )
            .io_sink();
        custom_io.emit(IoOutput::new(IoStream::Stderr, "custom".into()));
        assert_eq!(state.stdlib.io_sink().outputs[0].stream(), IoStream::Stderr);
        assert_eq!(state.stdlib.io_sink().outputs[0].text(), "custom");
    }
}
