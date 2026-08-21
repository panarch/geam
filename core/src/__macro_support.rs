pub use crate::host::{
    HostCall, HostCallCompletion, HostCallError, HostComponentProfile, HostProvider,
    HostProviderComponent, HostProviderComponentInitialization, HostProviderComponentRegistration,
    HostProviderConfiguration, HostProviderInitializationError, HostProviderModule,
    HostRegistrationError,
};

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
    use super::{HostProviderComponent, component_initialization_error};
    use crate::provider::InitializationError;

    struct Component;

    impl HostProviderComponent for Component {
        const ID: &'static str = "macro-support";
        type Stores = ();
        type RunState = ();
    }

    #[test]
    fn initialization_support_adds_the_static_component_identity() {
        let error = component_initialization_error::<Component>(InitializationError::new(
            "configuration is incomplete",
        ));

        assert_eq!(error.component_id(), "macro-support");
        assert_eq!(error.reason(), "configuration is incomplete");
    }
}
