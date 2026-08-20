use ecow::EcoString;
use geam::{
    HostCall, HostCallCompletion, HostCallError, HostComponentProfile, HostProvider,
    HostProviderComponent, HostProviderComponentInitialization, HostProviderComponentRegistration,
    HostProviderConfiguration, HostProviderInitializationError, HostProviderModule,
    HostRegistrationError,
};

pub struct Component;

#[derive(Default)]
pub struct Stores;

pub struct RunState {
    next: i64,
}

struct Provider;

impl HostProviderComponent for Component {
    const ID: &'static str = "geam-counter";
    type Stores = Stores;
    type RunState = RunState;
}

impl HostProviderComponentInitialization for Component {
    fn initialize(
        configuration: &HostProviderConfiguration,
    ) -> Result<Self::RunState, HostProviderInitializationError> {
        let next = configuration
            .get("start")
            .and_then(|value| value.as_integer())
            .ok_or_else(|| {
                HostProviderInitializationError::for_component::<Self>(
                    "configuration key `start` must be an Integer",
                )
            })?;
        Ok(RunState { next })
    }
}

impl<Profile> HostProviderComponentRegistration<Profile> for Component
where
    Profile: HostComponentProfile<Self>,
{
    fn providers() -> Result<Vec<HostProviderModule<Profile>>, HostRegistrationError> {
        HostProviderModule::new("counter", "counter")
            .and_then(|provider| {
                provider.with_scoped_function::<Provider, (EcoString,), EcoString, _>(
                    "next",
                    next::<Profile>,
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

fn next<'call, Profile>(
    mut call: HostCall<'call, Profile, Provider, EcoString>,
    label: EcoString,
) -> Result<HostCallCompletion<'call, EcoString>, HostCallError>
where
    Profile: HostComponentProfile<Component>,
{
    let next = call.state().next;
    call.state().next += 1;
    Ok(call.return_value(format!("{label}:{next}").into()))
}
