use crate::host::{HostProfile, HostProviderModule, HostRegistrationError};
use ecow::EcoString;
use std::fmt::{self, Display, Formatter};

mod configuration;

pub use configuration::{HostProviderConfiguration, HostProviderConfigurationValue};

/// A statically composed Rust provider component.
pub trait HostProviderComponent: Send + Sync + 'static {
    /// Stable component identity used in initialization diagnostics.
    const ID: &'static str;

    /// External stores owned by this component.
    type Stores: Default + 'static;

    /// Caller-owned mutable state used while executing this component.
    type RunState: 'static;
}

/// Transferable storage owned by one statically composed provider component.
///
/// This is separate from [`HostProviderComponent::Stores`] so an ordinary
/// local-only composition does not pay for synchronization or transferable
/// payload bounds that it does not use.
pub trait AsyncHostProviderComponent: HostProviderComponent {
    /// External stores used when the component participates in async embedding.
    type AsyncStores: Default + 'static;
}

/// Initializes one provider component from explicit read-only configuration.
pub trait HostProviderComponentInitialization: HostProviderComponent {
    /// Initializes caller-owned run state from explicit component configuration.
    fn initialize(
        configuration: &HostProviderConfiguration,
    ) -> Result<Self::RunState, HostProviderInitializationError>;
}

/// Projects one provider component from a statically generated host profile.
pub trait HostComponentProfile<Component>: HostProfile
where
    Component: HostProviderComponent,
{
    fn component_stores(stores: &Self::ExternalStores) -> &Component::Stores;

    fn component_state(state: &mut Self::RunState) -> &mut Component::RunState;
}

/// Projects one provider component from a transferable host profile.
pub trait AsyncHostComponentProfile<Component>: HostProfile
where
    Component: AsyncHostProviderComponent,
{
    fn component_async_stores(stores: &Self::ExternalStores) -> &Component::AsyncStores;

    fn component_state(state: &mut Self::RunState) -> &mut Component::RunState;
}

/// Registers the source-backed provider modules exported by one component.
pub trait HostProviderComponentRegistration<Profile>: HostProviderComponent
where
    Profile: HostComponentProfile<Self>,
    Self: Sized,
{
    fn providers() -> Result<Vec<HostProviderModule<Profile>>, HostRegistrationError>;
}

/// Registers immediate calls and explicit work constructors for transferable execution.
pub trait TransferHostProviderComponentRegistration<Profile>: AsyncHostProviderComponent
where
    Profile: AsyncHostComponentProfile<Self>,
    Self: Sized,
{
    fn providers()
    -> Result<Vec<crate::host::TransferHostProviderModule<Profile>>, HostRegistrationError>;
}

/// Failure to initialize one statically selected provider component.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HostProviderInitializationError {
    component_id: EcoString,
    reason: EcoString,
}

impl HostProviderInitializationError {
    /// Creates an owned initialization failure for the named component type.
    pub fn for_component<Component>(reason: impl Into<EcoString>) -> Self
    where
        Component: HostProviderComponent,
    {
        Self {
            component_id: Component::ID.into(),
            reason: reason.into(),
        }
    }

    pub fn component_id(&self) -> &str {
        &self.component_id
    }

    pub fn reason(&self) -> &str {
        &self.reason
    }
}

impl Display for HostProviderInitializationError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "could not initialize host provider component {}: {}",
            self.component_id, self.reason,
        )
    }
}

impl std::error::Error for HostProviderInitializationError {}

#[cfg(test)]
mod tests {
    use super::{
        AsyncHostComponentProfile, AsyncHostProviderComponent, HostComponentProfile,
        HostProviderComponent, HostProviderComponentInitialization, HostProviderConfiguration,
        HostProviderInitializationError,
    };
    use crate::host::HostProfile;

    struct FirstComponent;
    struct SecondComponent;
    struct AggregateProfile;

    #[derive(Default)]
    struct FirstAsyncStores(Vec<u32>);

    #[derive(Default)]
    struct SecondAsyncStores(Vec<u64>);

    #[derive(Default)]
    struct AggregateStores {
        first: Vec<u8>,
        second: Vec<u16>,
    }

    #[derive(Default)]
    struct AggregateAsyncStores {
        first: FirstAsyncStores,
        second: SecondAsyncStores,
    }

    struct AggregateState {
        first: String,
        second: usize,
    }

    impl HostProviderComponent for FirstComponent {
        const ID: &'static str = "first";
        type Stores = Vec<u8>;
        type RunState = String;
    }

    impl AsyncHostProviderComponent for FirstComponent {
        type AsyncStores = FirstAsyncStores;
    }

    impl HostProviderComponentInitialization for FirstComponent {
        fn initialize(
            _configuration: &HostProviderConfiguration,
        ) -> Result<Self::RunState, HostProviderInitializationError> {
            Ok("ready".into())
        }
    }

    impl HostProviderComponent for SecondComponent {
        const ID: &'static str = "second";
        type Stores = Vec<u16>;
        type RunState = usize;
    }

    impl AsyncHostProviderComponent for SecondComponent {
        type AsyncStores = SecondAsyncStores;
    }

    impl HostProviderComponentInitialization for SecondComponent {
        fn initialize(
            _configuration: &HostProviderConfiguration,
        ) -> Result<Self::RunState, HostProviderInitializationError> {
            Err(HostProviderInitializationError::for_component::<Self>(
                "missing endpoint",
            ))
        }
    }

    impl HostProfile for AggregateProfile {
        type RunState = AggregateState;
        type ExternalStores = AggregateStores;
    }

    struct AggregateAsyncProfile;

    impl HostProfile for AggregateAsyncProfile {
        type RunState = AggregateState;
        type ExternalStores = AggregateAsyncStores;
    }

    impl HostComponentProfile<FirstComponent> for AggregateProfile {
        fn component_stores(stores: &Self::ExternalStores) -> &Vec<u8> {
            &stores.first
        }

        fn component_state(state: &mut Self::RunState) -> &mut String {
            &mut state.first
        }
    }

    impl HostComponentProfile<SecondComponent> for AggregateProfile {
        fn component_stores(stores: &Self::ExternalStores) -> &Vec<u16> {
            &stores.second
        }

        fn component_state(state: &mut Self::RunState) -> &mut usize {
            &mut state.second
        }
    }

    impl AsyncHostComponentProfile<FirstComponent> for AggregateAsyncProfile {
        fn component_async_stores(stores: &Self::ExternalStores) -> &FirstAsyncStores {
            &stores.first
        }

        fn component_state(state: &mut Self::RunState) -> &mut String {
            &mut state.first
        }
    }

    impl AsyncHostComponentProfile<SecondComponent> for AggregateAsyncProfile {
        fn component_async_stores(stores: &Self::ExternalStores) -> &SecondAsyncStores {
            &stores.second
        }

        fn component_state(state: &mut Self::RunState) -> &mut usize {
            &mut state.second
        }
    }

    #[test]
    fn generated_profiles_project_each_component_without_erasure() {
        let stores = AggregateStores::default();
        let mut state = AggregateState {
            first: "initial".into(),
            second: 7,
        };

        assert!(
            <AggregateProfile as HostComponentProfile<FirstComponent>>::component_stores(&stores)
                .is_empty()
        );
        assert!(
            <AggregateProfile as HostComponentProfile<SecondComponent>>::component_stores(&stores)
                .is_empty()
        );
        <AggregateProfile as HostComponentProfile<FirstComponent>>::component_state(&mut state)
            .push_str(" first");
        *<AggregateProfile as HostComponentProfile<SecondComponent>>::component_state(
            &mut state,
        ) += 1;

        assert_eq!(state.first, "initial first");
        assert_eq!(state.second, 8);
    }

    #[test]
    fn transferable_profiles_project_separate_component_stores_and_the_same_state() {
        let mut stores = AggregateAsyncStores::default();
        stores.first.0.push(3);
        stores.second.0.push(5);
        let mut state = AggregateState {
            first: "initial".into(),
            second: 7,
        };

        assert_eq!(
            <AggregateAsyncProfile as AsyncHostComponentProfile<FirstComponent>>::component_async_stores(
                &stores,
            )
            .0,
            [3]
        );
        assert_eq!(
            <AggregateAsyncProfile as AsyncHostComponentProfile<SecondComponent>>::component_async_stores(
                &stores,
            )
            .0,
            [5]
        );
        <AggregateAsyncProfile as AsyncHostComponentProfile<FirstComponent>>::component_state(
            &mut state,
        )
        .push_str(" async");
        *<AggregateAsyncProfile as AsyncHostComponentProfile<SecondComponent>>::component_state(
            &mut state,
        ) += 2;

        assert_eq!(state.first, "initial async");
        assert_eq!(state.second, 9);
    }

    #[test]
    fn component_initialization_preserves_identity_and_owned_reason() {
        let configuration = HostProviderConfiguration::empty();

        assert_eq!(
            FirstComponent::initialize(&configuration),
            Ok("ready".into())
        );
        let error = SecondComponent::initialize(&configuration)
            .expect_err("second component should reject missing configuration");
        assert_eq!(error.component_id(), "second");
        assert_eq!(error.reason(), "missing endpoint");
        assert_eq!(
            error.to_string(),
            "could not initialize host provider component second: missing endpoint"
        );
        assert_eq!(error.clone(), error);
    }
}
