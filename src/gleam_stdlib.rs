use crate::{HostProfile, HostProviderModule, HostRegistrationError};

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

#[cfg(test)]
pub(crate) use dict::DictSchema;
pub(crate) use dict::{DictOf, create_string_dynamic_dict};
#[cfg(test)]
pub(crate) use dynamic::DynamicSchema;
pub(crate) use dynamic::{Dynamic, create_value as create_dynamic_value};
pub(crate) use dynamic_decode::DynamicDecodeError;
pub(crate) use result::{GleamError, GleamOk, GleamResult};
#[cfg(test)]
pub(crate) use string_tree::StringTreeSchema;
pub(crate) use string_tree::{StoredStringTree, StringTree, StringTreePayload};

/// A host profile that exposes state and storage for the official Gleam standard library.
pub trait GleamStdlibHostProfile: HostProfile {
    /// The concrete caller-owned sink used by official Gleam IO functions.
    type Io: IoSink;

    /// Projects the standard-library external stores from this profile.
    fn gleam_stdlib_stores(stores: &Self::ExternalStores) -> &GleamStdlibStores;

    /// Projects caller-owned standard-library run state from this profile.
    fn gleam_stdlib_run_state(state: &mut Self::RunState) -> &mut GleamStdlibRunState;

    /// Projects the caller-owned standard-library IO sink from this profile.
    fn gleam_stdlib_io(state: &mut Self::RunState) -> &mut Self::Io;
}

/// External value stores used by the official Gleam standard library providers.
#[derive(Default)]
pub struct GleamStdlibStores {
    dict: dict::Stores,
    dynamic: dynamic::Stores,
    string_tree: string_tree::Stores,
}

/// The default profile for using only the official Gleam standard library providers.
#[derive(Debug, Clone, Copy)]
pub struct GleamStdlibProfile;

impl HostProfile for GleamStdlibProfile {
    type RunState = GleamStdlibRunState;
    type ExternalStores = GleamStdlibStores;
}

impl GleamStdlibHostProfile for GleamStdlibProfile {
    type Io = Vec<IoOutput>;

    fn gleam_stdlib_stores(stores: &Self::ExternalStores) -> &GleamStdlibStores {
        stores
    }

    fn gleam_stdlib_run_state(state: &mut Self::RunState) -> &mut GleamStdlibRunState {
        state
    }

    fn gleam_stdlib_io(state: &mut Self::RunState) -> &mut Self::Io {
        state.io_sink()
    }
}

/// Registers the Rust providers for the official Gleam standard library.
pub fn host_providers<Profile>() -> Result<Vec<HostProviderModule<Profile>>, HostRegistrationError>
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
        GleamStdlibHostProfile, GleamStdlibProfile, GleamStdlibRunState, GleamStdlibStores,
        IoOutput, IoSink, IoStream, host_providers,
    };
    use crate::HostProfile;

    struct CustomProfile;

    #[derive(Default)]
    struct CustomStores {
        stdlib: GleamStdlibStores,
    }

    struct CustomRunState {
        stdlib: GleamStdlibRunState,
        io: RecordingSink,
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

    impl GleamStdlibHostProfile for CustomProfile {
        type Io = RecordingSink;

        fn gleam_stdlib_stores(stores: &Self::ExternalStores) -> &GleamStdlibStores {
            &stores.stdlib
        }

        fn gleam_stdlib_run_state(state: &mut Self::RunState) -> &mut GleamStdlibRunState {
            &mut state.stdlib
        }

        fn gleam_stdlib_io(state: &mut Self::RunState) -> &mut Self::Io {
            &mut state.io
        }
    }

    #[test]
    fn registers_providers_in_dependency_first_module_order() {
        let providers = host_providers::<GleamStdlibProfile>()
            .expect("official stdlib providers should register");

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
            stdlib: GleamStdlibRunState::from_seed([2; 32]),
            io: RecordingSink::default(),
        };

        assert!(std::ptr::eq(
            GleamStdlibProfile::gleam_stdlib_stores(&default_stores),
            &default_stores,
        ));
        assert!(std::ptr::eq(
            CustomProfile::gleam_stdlib_stores(&stores),
            &stores.stdlib,
        ));
        let default_projected = GleamStdlibProfile::gleam_stdlib_run_state(&mut default_state);
        assert!(std::ptr::eq(default_projected, &default_state));
        let projected = CustomProfile::gleam_stdlib_run_state(&mut state);
        assert!(std::ptr::eq(projected, &state.stdlib));

        let default_io = GleamStdlibProfile::gleam_stdlib_io(&mut default_state);
        default_io.emit(IoOutput::new(IoStream::Stdout, "default".into()));
        assert_eq!(default_state.io_outputs()[0].text(), "default");

        let custom_io = CustomProfile::gleam_stdlib_io(&mut state);
        custom_io.emit(IoOutput::new(IoStream::Stderr, "custom".into()));
        assert_eq!(state.io.outputs[0].stream(), IoStream::Stderr);
        assert_eq!(state.io.outputs[0].text(), "custom");
    }
}
