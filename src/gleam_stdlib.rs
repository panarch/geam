use crate::{HostProfile, HostProviderModule, HostRegistrationError};

mod dict;
mod dynamic;
mod float;
mod result;
mod run_state;

pub use run_state::{GleamStdlibRunState, GleamStdlibRunStateError};

/// A host profile that exposes storage for the official Gleam standard library.
pub trait GleamStdlibHostProfile: HostProfile {
    /// Projects the standard-library external stores from this profile.
    fn gleam_stdlib_stores(stores: &Self::ExternalStores) -> &GleamStdlibStores;

    /// Projects caller-owned standard-library run state from this profile.
    fn gleam_stdlib_run_state(state: &mut Self::RunState) -> &mut GleamStdlibRunState;
}

/// External value stores used by the official Gleam standard library providers.
#[derive(Default)]
pub struct GleamStdlibStores {
    dict: dict::Stores,
    dynamic: dynamic::Stores,
}

/// The default profile for using only the official Gleam standard library providers.
#[derive(Debug, Clone, Copy)]
pub struct GleamStdlibProfile;

impl HostProfile for GleamStdlibProfile {
    type RunState = GleamStdlibRunState;
    type ExternalStores = GleamStdlibStores;
}

impl GleamStdlibHostProfile for GleamStdlibProfile {
    fn gleam_stdlib_stores(stores: &Self::ExternalStores) -> &GleamStdlibStores {
        stores
    }

    fn gleam_stdlib_run_state(state: &mut Self::RunState) -> &mut GleamStdlibRunState {
        state
    }
}

/// Registers the Rust providers for the official Gleam standard library.
pub fn host_providers<Profile>() -> Result<Vec<HostProviderModule<Profile>>, HostRegistrationError>
where
    Profile: GleamStdlibHostProfile,
{
    dict::host_provider::<Profile>().and_then(|dict| {
        dynamic::host_provider::<Profile>().and_then(|dynamic| {
            float::host_provider::<Profile>().map(|float| vec![dict, dynamic, float])
        })
    })
}

#[cfg(test)]
mod tests {
    use super::{
        GleamStdlibHostProfile, GleamStdlibProfile, GleamStdlibRunState, GleamStdlibStores,
        host_providers,
    };
    use crate::HostProfile;

    struct CustomProfile;

    #[derive(Default)]
    struct CustomStores {
        stdlib: GleamStdlibStores,
    }

    struct CustomRunState {
        stdlib: GleamStdlibRunState,
    }

    impl HostProfile for CustomProfile {
        type RunState = CustomRunState;
        type ExternalStores = CustomStores;
    }

    impl GleamStdlibHostProfile for CustomProfile {
        fn gleam_stdlib_stores(stores: &Self::ExternalStores) -> &GleamStdlibStores {
            &stores.stdlib
        }

        fn gleam_stdlib_run_state(state: &mut Self::RunState) -> &mut GleamStdlibRunState {
            &mut state.stdlib
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
            ["gleam/dict", "gleam/dynamic", "gleam/float"],
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
    fn custom_profiles_project_their_stdlib_store_bundle() {
        let default_stores = GleamStdlibStores::default();
        let stores = CustomStores::default();
        let mut default_state = GleamStdlibRunState::from_seed([1; 32]);
        let mut state = CustomRunState {
            stdlib: GleamStdlibRunState::from_seed([2; 32]),
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
    }
}
