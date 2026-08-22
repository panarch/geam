pub use crate::host::{
    HostCall, HostCallCompletion, HostCallError, HostComponentProfile, HostExternal,
    HostExternalBinding, HostExternalEquality, HostExternalHashing, HostExternalInspection,
    HostExternalSchema, HostExternalStorage, HostExternalStore, HostExternalType, HostProvider,
    HostProviderComponent, HostProviderComponentInitialization, HostProviderComponentRegistration,
    HostProviderConfiguration, HostProviderInitializationError, HostProviderModule,
    HostRegistrationError,
};
pub use crate::provider::ExternalPayload;

/// Package identity shared by provider and module macro expansions.
pub trait ProviderPackage: HostProviderComponent {
    const PACKAGE: &'static str;
}

pub fn component_initialization_error<Component>(
    error: crate::provider::InitializationError,
) -> crate::HostProviderInitializationError
where
    Component: HostProviderComponent,
{
    crate::HostProviderInitializationError::for_component::<Component>(error.reason())
}

#[cfg(test)]
mod tests {
    use super::{HostProviderComponent, ProviderPackage, component_initialization_error};
    use crate::provider::InitializationError;

    struct Component;

    impl HostProviderComponent for Component {
        const ID: &'static str = "macro-support";
        type Stores = ();
        type RunState = ();
    }

    impl ProviderPackage for Component {
        const PACKAGE: &'static str = "macro_support";
    }

    #[test]
    fn initialization_support_adds_the_static_component_identity() {
        let error = component_initialization_error::<Component>(InitializationError::new(
            "configuration is incomplete",
        ));

        assert_eq!(error.component_id(), "macro-support");
        assert_eq!(error.reason(), "configuration is incomplete");
        assert_eq!(Component::PACKAGE, "macro_support");
    }
}
