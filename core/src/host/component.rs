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
    type Stores: Default + Send + 'static;

    /// Caller-owned mutable state used while executing this component.
    type RunState: Send + 'static;
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

/// A provider component that owns a service for each execution domain.
///
/// Configuration is interpreted by component initialization. Service creation
/// receives that initialized state before the domain admits its first unit.
pub trait HostExecutionService: HostProviderComponent {
    type State: crate::execution::HostExecutionState;

    fn initialize_service(state: &mut Self::RunState) -> Self::State;
}

/// Projects a producer's unique service from the current execution domain.
///
/// Consumers use the producer's identity; their own component state does not
/// contain another instance of the shared service.
pub trait HostServiceProfile<Service>: HostComponentProfile<Service>
where
    Service: HostExecutionService,
{
    fn service(state: &mut Self::ExecutionState) -> &mut Service::State;
}

/// Registers the source-backed provider modules exported by one component.
pub trait HostProviderComponentRegistration<Profile>: HostProviderComponent
where
    Profile: HostComponentProfile<Self>,
    Self: Sized,
{
    fn providers() -> Result<Vec<HostProviderModule<Profile>>, HostRegistrationError>;
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
        HostComponentProfile, HostExecutionService, HostProviderComponent,
        HostProviderComponentInitialization, HostProviderConfiguration,
        HostProviderInitializationError, HostServiceProfile,
    };
    use crate::host::HostProfile;

    struct FirstComponent;
    struct SecondComponent;
    struct AggregateProfile;

    #[derive(Default)]
    struct AggregateStores {
        first: Vec<u8>,
        second: Vec<u16>,
    }

    struct AggregateState {
        first: String,
        second: usize,
    }

    #[derive(Default)]
    struct Sequence {
        next: usize,
    }

    impl crate::execution::HostExecutionState for Sequence {
        fn started(&mut self, _: crate::execution::ExecutionUnit) {}
        fn finished(
            &mut self,
            _: crate::execution::ExecutionUnitId,
            _: &crate::execution::UnitExit,
        ) {
        }
        fn close(&mut self) {}
    }

    impl HostExecutionService for SecondComponent {
        type State = Sequence;
        fn initialize_service(seed: &mut usize) -> Sequence {
            Sequence { next: *seed }
        }
    }

    impl HostServiceProfile<SecondComponent> for AggregateProfile {
        fn service(state: &mut Sequence) -> &mut Sequence {
            state
        }
    }

    impl crate::HostProvider<AggregateProfile> for SecondComponent {
        type State = usize;
        fn project(state: &mut AggregateState) -> &mut usize {
            <AggregateProfile as HostComponentProfile<Self>>::component_state(state)
        }
    }

    impl HostProviderComponent for FirstComponent {
        const ID: &'static str = "first";
        type Stores = Vec<u8>;
        type RunState = String;
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
        type ExecutionState = Sequence;

        fn initialize_execution(state: &mut AggregateState) -> Sequence {
            SecondComponent::initialize_service(&mut state.second)
        }
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
    fn components_keep_their_stores_and_state_when_moved_to_a_worker() {
        let stores = AggregateStores {
            first: vec![3],
            second: vec![5],
        };
        let mut state = AggregateState {
            first: "initial".into(),
            second: 7,
        };

        let (stores, state) = std::thread::spawn(move || {
            assert_eq!(
                <AggregateProfile as HostComponentProfile<FirstComponent>>::component_stores(
                    &stores
                ),
                &[3],
            );
            assert_eq!(
                <AggregateProfile as HostComponentProfile<SecondComponent>>::component_stores(
                    &stores
                ),
                &[5],
            );
            <AggregateProfile as HostComponentProfile<FirstComponent>>::component_state(&mut state)
                .push_str(" worker");
            *<AggregateProfile as HostComponentProfile<SecondComponent>>::component_state(
                &mut state,
            ) += 2;
            (stores, state)
        })
        .join()
        .expect("worker");
        assert_eq!(stores.first, [3]);
        assert_eq!(stores.second, [5]);
        assert_eq!(state.first, "initial worker");
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

    #[test]
    fn typed_and_authoring_calls_share_one_explicitly_initialized_domain_service() {
        fn next<'call>(
            mut call: crate::HostCall<'call, AggregateProfile, SecondComponent, num_bigint::BigInt>,
            authoring: bool,
        ) -> Result<crate::HostCallCompletion<'call, num_bigint::BigInt>, crate::HostCallError>
        {
            assert_eq!(*call.state(), 42);
            if authoring {
                let mut call = crate::provider::Call::from_host_call(call);
                let service = call.service::<SecondComponent>();
                let next = service.next;
                service.next += 1;
                Ok(call.into_host_call().return_value(next.into()))
            } else {
                let service = call.service::<SecondComponent>();
                let next = service.next;
                service.next += 1;
                Ok(call.return_value(next.into()))
            }
        }
        let provider = crate::HostProviderModule::new("application", "main")
            .unwrap()
            .with_scoped_function::<SecondComponent, (bool,), num_bigint::BigInt, _>("next", next)
            .unwrap();
        let typed = crate::compile_typed_host_program(
            "application",
            "main",
            [crate::PackageSource::new(
                "application",
                Vec::<String>::new(),
                [crate::ModuleSource::new(
                    "main",
                    "main.gleam",
                    r#"
@external(erlang, "host", "next") fn next(authoring: Bool) -> Int
pub fn main() { #(next(False), next(True), next(False)) }
"#,
                )],
            )],
            crate::HostProviderSet::from_providers([provider]).unwrap(),
        )
        .unwrap();
        let mut execution =
            crate::HostedExecution::try_from_module_plan(crate::plan_host_program(typed).unwrap())
                .unwrap();
        let host = crate::execution_fixture::TestHost::default();
        let mut state = AggregateState {
            first: "unrelated".into(),
            second: 42,
        };
        let mut echo = Vec::new();
        for _ in 0..2 {
            let result = host
                .block_on(execution.run_main(&host, &mut state, &mut echo))
                .unwrap();
            assert_eq!(result.inspect().to_string(), "#(42, 43, 44)");
            assert_eq!(state.first, "unrelated");
            assert_eq!(state.second, 42);
            assert!(echo.is_empty());
        }
    }
}
