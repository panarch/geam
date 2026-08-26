use ecow::EcoString;
use geam::provider::{Call, Configuration, InitializationError};
use std::collections::BTreeSet;

pub struct RunState {
    environment: EcoString,
    enabled: BTreeSet<EcoString>,
}

fn initialize(configuration: &Configuration) -> Result<RunState, InitializationError> {
    let environment = configuration
        .get("environment")
        .and_then(|value| value.as_string())
        .cloned()
        .ok_or_else(|| {
            InitializationError::new("configuration key `environment` must be a String")
        })?;
    let values = configuration
        .get("enabled")
        .and_then(|value| value.as_array())
        .ok_or_else(invalid_enabled)?;
    let mut enabled = BTreeSet::new();
    for value in values {
        let value = value.as_string().cloned().ok_or_else(invalid_enabled)?;
        enabled.insert(value);
    }

    Ok(RunState {
        environment,
        enabled,
    })
}

fn invalid_enabled() -> InitializationError {
    InitializationError::new("configuration key `enabled` must be an Array of Strings")
}

#[geam::provider(
    package = "example_feature_flags",
    state = RunState,
    initialize = initialize,
    modules = [feature_flags],
)]
pub struct Component;

#[geam::module(path = "example_feature_flags")]
mod feature_flags {
    use super::{Call, EcoString, RunState};

    #[geam::function]
    fn environment(#[geam::call] call: &Call<RunState>) -> EcoString {
        call.state().environment.clone()
    }

    #[geam::function]
    fn enabled(#[geam::call] call: &Call<RunState>, name: EcoString) -> bool {
        call.state().enabled.contains(&name)
    }
}

#[cfg(test)]
mod tests {
    use super::Component;
    use ecow::EcoString;
    use geam::provider::Configuration;
    use geam::{HostProviderComponentInitialization, HostProviderConfigurationValue};
    use std::collections::BTreeMap;

    #[test]
    fn initialization_preserves_exact_configuration_failures() {
        let missing = Component::initialize(&Configuration::empty())
            .err()
            .expect("missing environment should fail");
        assert_eq!(missing.component_id(), "geam-example-feature-flags");
        assert_eq!(
            missing.reason(),
            "configuration key `environment` must be a String",
        );

        let wrong_enabled = Configuration::new(BTreeMap::from([
            (EcoString::from("environment"), "staging".into()),
            (EcoString::from("enabled"), true.into()),
        ]));
        let wrong_enabled = Component::initialize(&wrong_enabled)
            .err()
            .expect("non-array enabled value should fail");
        assert_eq!(wrong_enabled.component_id(), "geam-example-feature-flags");
        assert_eq!(
            wrong_enabled.reason(),
            "configuration key `enabled` must be an Array of Strings",
        );

        let mixed_enabled = Configuration::new(BTreeMap::from([
            (EcoString::from("environment"), "staging".into()),
            (
                EcoString::from("enabled"),
                vec![
                    HostProviderConfigurationValue::from("new_checkout"),
                    HostProviderConfigurationValue::from(1_i64),
                ]
                .into(),
            ),
        ]));
        let mixed_enabled = Component::initialize(&mixed_enabled)
            .err()
            .expect("non-string enabled item should fail");
        assert_eq!(mixed_enabled.component_id(), "geam-example-feature-flags");
        assert_eq!(
            mixed_enabled.reason(),
            "configuration key `enabled` must be an Array of Strings",
        );
    }
}
