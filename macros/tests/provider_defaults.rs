use ecow::EcoString;
use geam_core::provider::Configuration;
use geam_core::{
    HostProviderComponent, HostProviderComponentInitialization, HostProviderComponentRegistration,
};
use std::collections::BTreeMap;

mod default_state {
    #[derive(Debug, Default)]
    pub struct RunState {
        pub(super) issued: usize,
    }

    #[geam_macros::provider(
        package = "default_state",
        state = RunState,
        modules = [state],
        crate_path = geam_core,
    )]
    pub struct Component;

    #[geam_macros::module(path = "default_state", crate_path = geam_core)]
    mod state {}
}

#[test]
fn default_state_initialization_uses_the_consumer_package_identity() {
    use default_state::Component;

    assert_eq!(Component::ID, "geam-macros");
    let initialized = Component::initialize(&Configuration::empty())
        .expect("empty configuration should construct Default state");
    assert_eq!(initialized.issued, 0);

    let configuration = Configuration::new(BTreeMap::from([(
        EcoString::from("unexpected"),
        true.into(),
    )]));
    let error = Component::initialize(&configuration)
        .expect_err("Default state must not silently accept configuration");
    assert_eq!(error.component_id(), "geam-macros");
    assert_eq!(error.reason(), "provider does not accept configuration");
}

#[test]
fn default_state_component_still_registers_its_declared_module() {
    use default_state::Component;

    struct Profile;

    #[derive(Default)]
    struct Stores {
        component: <Component as HostProviderComponent>::Stores,
    }

    struct State {
        component: <Component as HostProviderComponent>::RunState,
    }

    impl geam_core::HostProfile for Profile {
        type RunState = State;
        type ExternalStores = Stores;
    }

    impl geam_core::HostComponentProfile<Component> for Profile {
        fn component_stores(
            stores: &Self::ExternalStores,
        ) -> &<Component as HostProviderComponent>::Stores {
            &stores.component
        }

        fn component_state(
            state: &mut Self::RunState,
        ) -> &mut <Component as HostProviderComponent>::RunState {
            &mut state.component
        }
    }

    let providers = <Component as HostProviderComponentRegistration<Profile>>::providers()
        .expect("default-state module should register");
    assert_eq!(providers.len(), 1);
    assert_eq!(providers[0].package().as_str(), "default_state");
    assert_eq!(providers[0].module().as_str(), "default_state");
    assert_eq!(providers[0].functions().count(), 0);
}
