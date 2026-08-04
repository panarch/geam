use ecow::EcoString;
use geam::{
    HostCall, HostCallCompletion, HostCallError, HostCallable, HostComponentProfile,
    HostFunctionType, HostProvider, HostProviderComponent, HostProviderComponentRegistration,
    HostProviderConfiguration, HostProviderInitializationError, HostProviderModule,
    HostRegistrationError, HostTypeList, HostTypeListEnd,
};

pub struct Component;

#[derive(Debug, Default)]
pub struct Stores;

#[derive(Debug)]
pub struct RunState {
    prefix: EcoString,
    calls: usize,
}

struct Provider;

type TransformArguments = HostTypeList<EcoString, HostTypeListEnd>;
type Transform = HostFunctionType<TransformArguments, EcoString>;

impl HostProviderComponent for Component {
    const ID: &'static str = "provider-sdk-example";
    type Stores = Stores;
    type RunState = RunState;

    fn initialize(
        configuration: &HostProviderConfiguration,
    ) -> Result<Self::RunState, HostProviderInitializationError> {
        let prefix = configuration
            .get("prefix")
            .and_then(|value| value.as_string())
            .cloned()
            .ok_or_else(|| {
                HostProviderInitializationError::for_component::<Self>(
                    "configuration key `prefix` must be a String",
                )
            })?;

        Ok(RunState { prefix, calls: 0 })
    }
}

impl<Profile> HostProviderComponentRegistration<Profile> for Component
where
    Profile: HostComponentProfile<Self>,
{
    fn providers() -> Result<Vec<HostProviderModule<Profile>>, HostRegistrationError> {
        HostProviderModule::new("provider_sdk_example", "provider/sdk")
            .and_then(|provider| {
                provider.with_scoped_function::<Provider, (EcoString, Transform), EcoString, _>(
                    "decorate",
                    decorate::<Profile>,
                )
            })
            .map(|provider| vec![provider])
    }
}

impl<Profile> HostProvider<Profile> for Provider
where
    Profile: HostComponentProfile<Component>,
{
    type State = RunState;

    fn project(state: &mut Profile::RunState) -> &mut Self::State {
        Profile::component_state(state)
    }
}

impl RunState {
    pub fn prefix(&self) -> &str {
        &self.prefix
    }

    pub fn calls(&self) -> usize {
        self.calls
    }
}

fn decorate<'call, Profile>(
    mut call: HostCall<'call, Profile, Provider, EcoString>,
    value: EcoString,
    transform: HostCallable<'call, TransformArguments, EcoString>,
) -> Result<HostCallCompletion<'call, EcoString>, HostCallError>
where
    Profile: HostComponentProfile<Component>,
{
    let decorated = {
        let state = call.state();
        state.calls += 1;
        format!("{}{}", state.prefix, value)
    };
    let transformed = call.invoke(transform, (decorated.into(), ()))?;
    Ok(call.return_value(transformed))
}

#[cfg(test)]
mod tests {
    use super::{Component, RunState};
    use ecow::EcoString;
    use geam::{HostProviderComponent, HostProviderConfiguration, HostProviderInitializationError};
    use std::collections::BTreeMap;

    #[test]
    fn component_initialization_requires_an_explicit_string_prefix() {
        let configuration = HostProviderConfiguration::new(BTreeMap::from([(
            EcoString::from("prefix"),
            EcoString::from("docs:").into(),
        )]));

        let state = Component::initialize(&configuration)
            .expect("string prefix should initialize provider state");
        assert_eq!(state.prefix(), "docs:");
        assert_eq!(state.calls(), 0);

        let error = Component::initialize(&HostProviderConfiguration::empty())
            .expect_err("missing prefix should fail initialization");
        assert_eq!(
            error,
            HostProviderInitializationError::for_component::<Component>(
                "configuration key `prefix` must be a String",
            )
        );

        let wrong_type = HostProviderConfiguration::new(BTreeMap::from([(
            EcoString::from("prefix"),
            true.into(),
        )]));
        assert_eq!(
            Component::initialize(&wrong_type)
                .expect_err("non-string prefix should fail initialization"),
            HostProviderInitializationError::for_component::<Component>(
                "configuration key `prefix` must be a String",
            )
        );
    }

    #[test]
    fn run_state_accessors_report_current_component_state() {
        let state = RunState {
            prefix: "sdk:".into(),
            calls: 3,
        };

        assert_eq!(state.prefix(), "sdk:");
        assert_eq!(state.calls(), 3);
    }
}
