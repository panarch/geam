//! An independent native function-view consumer of the public stdlib service.

use geam::gleam_stdlib::{Component as StdlibComponent, GleamStdlibHostProfile, service};
use geam::host::native::{NativeCall, NativeRules};
use geam::{
    HostCallCompletion, HostCallError, HostComponentProfile, HostFailure, HostProvider,
    HostProviderComponent, HostProviderComponentInitialization, HostProviderComponentRegistration,
    HostProviderConfiguration, HostProviderInitializationError, HostProviderModule,
    HostRegistrationError, HostTypeIndex0, HostTypeList, HostTypeListEnd, HostTypeParameter,
    HostValue,
};

pub struct Component;
type Output = HostTypeParameter<0>;
type Input = HostTypeParameter<1>;
type One<Type> = HostTypeList<Type, HostTypeListEnd>;

impl HostProviderComponent for Component {
    const ID: &'static str = "native_function_views_fixture";
    type Stores = ();
    type RunState = ();
}

impl HostProviderComponentInitialization for Component {
    fn initialize(
        configuration: &HostProviderConfiguration,
    ) -> Result<(), HostProviderInitializationError> {
        if configuration.is_empty() {
            Ok(())
        } else {
            Err(HostProviderInitializationError::for_component::<Self>(
                "configuration must be empty",
            ))
        }
    }
}

impl<Profile: HostComponentProfile<Self>> HostProvider<Profile> for Component {
    type State = ();
    fn project(state: &mut Profile::RunState) -> &mut () {
        Profile::component_state(state)
    }
}

impl<Profile> HostProviderComponentRegistration<Profile> for Component
where
    Profile: GleamStdlibHostProfile
        + HostComponentProfile<StdlibComponent<Profile::Io>>
        + HostComponentProfile<Self>,
    Profile::RunState: Send,
{
    fn providers() -> Result<Vec<HostProviderModule<Profile>>, HostRegistrationError> {
        let rules =
            service::with_native_dynamic(NativeRules::default().retained_views::<One<Input>>());
        HostProviderModule::new(
            "native_function_views_fixture",
            "native_function_views_fixture",
        )
        .and_then(|provider| {
            provider.with_native_function::<Self, (Input,), Output, One<Output>, _>(
                "coerce",
                rules,
                coerce::<Profile>,
            )
        })
        .map(|provider| vec![provider])
    }
}

fn coerce<'call, Profile>(
    mut call: NativeCall<'call, Profile, Component, Output, One<Output>>,
    input: HostValue<'call, Input>,
) -> Result<HostCallCompletion<'call, Output>, HostCallError>
where
    Profile: HostComponentProfile<Component>,
{
    let native = call.source::<Input>(input);
    let output = call
        .convert::<HostTypeIndex0>(&native)
        .ok_or_else(|| HostFailure::new("checked identity cannot convert target"))?;
    Ok(call.finish(output))
}

#[cfg(test)]
mod tests {
    use super::Component;
    use geam::{HostProviderComponentInitialization, HostProviderConfiguration};
    use std::collections::BTreeMap;

    #[test]
    fn component_accepts_only_empty_configuration() {
        assert_eq!(
            Component::initialize(&HostProviderConfiguration::empty()),
            Ok(())
        );
        let configuration =
            HostProviderConfiguration::new(BTreeMap::from([("unexpected".into(), true.into())]));
        let error = Component::initialize(&configuration).unwrap_err();
        assert_eq!(error.component_id(), "native_function_views_fixture");
        assert_eq!(error.reason(), "configuration must be empty");
    }
}
