use ecow::EcoString;
use geam::{
    ExecutionError, HostComponentProfile, HostModule, HostProfile, HostProviderComponent,
    HostProviderComponentRegistration, HostProviderConfiguration, HostProviderSet, HostedExecution,
    ModuleSource, PackageSource, Value, compile_typed_host_program, plan_host_program,
};
use provider_sdk_example_provider::Component;
use std::collections::BTreeMap;

struct Profile;

#[derive(Default)]
struct Stores {
    provider: <Component as HostProviderComponent>::Stores,
}

struct RunState {
    provider: <Component as HostProviderComponent>::RunState,
}

impl HostProfile for Profile {
    type RunState = RunState;
    type ExternalStores = Stores;
}

impl HostComponentProfile<Component> for Profile {
    fn component_stores(
        stores: &Self::ExternalStores,
    ) -> &<Component as HostProviderComponent>::Stores {
        &stores.provider
    }

    fn component_state(
        state: &mut Self::RunState,
    ) -> &mut <Component as HostProviderComponent>::RunState {
        &mut state.provider
    }
}

#[test]
fn path_component_runs_with_explicit_configuration_state_and_callback() {
    let provider_source = r#"
@external(erlang, "provider_sdk", "decorate")
pub fn decorate(value: String, transform: fn(String) -> String) -> String
"#;
    let main_source = r#"
import provider/sdk

pub fn main() {
  sdk.decorate("item", fn(value) { value <> "!" })
}
"#;
    let configuration = HostProviderConfiguration::new(BTreeMap::from([(
        EcoString::from("prefix"),
        EcoString::from("sdk:").into(),
    )]));
    let provider_state = Component::initialize(&configuration)
        .expect("explicit component configuration should initialize");
    let providers = <Component as HostProviderComponentRegistration<Profile>>::providers()
        .expect("path component should register its provider modules");
    let hosts = HostProviderSet::with_providers(Vec::<HostModule<Profile>>::new(), providers)
        .expect("path component modules should be unique");
    let typed = compile_typed_host_program(
        "provider_sdk_example",
        "main",
        [PackageSource::new(
            "provider_sdk_example",
            Vec::<&str>::new(),
            [
                ModuleSource::new("provider/sdk", "src/provider/sdk.gleam", provider_source),
                ModuleSource::new("main", "src/main.gleam", main_source),
            ],
        )],
        hosts,
    )
    .expect("complete provider example should compile");
    let plan = plan_host_program(typed).expect("complete provider example should plan");
    let execution =
        HostedExecution::try_from_module_plan(plan).expect("provider example should seal");
    let mut state = RunState {
        provider: provider_state,
    };

    assert_eq!(
        execution.run_main(&mut state, &mut Vec::new()),
        Ok(Value::String("sdk:item!".into())),
    );
    assert_eq!(state.provider.prefix(), "sdk:");
    assert_eq!(state.provider.calls(), 1);

    assert_eq!(
        execution.run_main(&mut state, &mut Vec::new()),
        Ok(Value::String("sdk:item!".into())),
    );
    assert_eq!(state.provider.calls(), 2);

    let independent_provider_state = Component::initialize(&configuration)
        .expect("same configuration should create independent state");
    let mut independent_state = RunState {
        provider: independent_provider_state,
    };
    assert_eq!(
        execution.run_main(&mut independent_state, &mut Vec::new()),
        Ok(Value::String("sdk:item!".into())),
    );
    assert_eq!(independent_state.provider.calls(), 1);
    assert_eq!(state.provider.calls(), 2);
}

#[test]
fn path_component_preserves_nested_callback_failures_after_state_updates() {
    let provider_source = r#"
@external(erlang, "provider_sdk", "decorate")
pub fn decorate(value: String, transform: fn(String) -> String) -> String
"#;
    let main_source = r#"
import provider/sdk

pub fn main() {
  sdk.decorate("item", fn(_) { panic as "callback failed" })
}
"#;
    let configuration = HostProviderConfiguration::new(BTreeMap::from([(
        EcoString::from("prefix"),
        EcoString::from("sdk:").into(),
    )]));
    let providers = <Component as HostProviderComponentRegistration<Profile>>::providers()
        .expect("path component should register its provider modules");
    let hosts = HostProviderSet::with_providers(Vec::<HostModule<Profile>>::new(), providers)
        .expect("path component modules should be unique");
    let typed = compile_typed_host_program(
        "provider_sdk_example",
        "main",
        [PackageSource::new(
            "provider_sdk_example",
            Vec::<&str>::new(),
            [
                ModuleSource::new("provider/sdk", "src/provider/sdk.gleam", provider_source),
                ModuleSource::new("main", "src/main.gleam", main_source),
            ],
        )],
        hosts,
    )
    .expect("nested callback failure source should compile");
    let plan = plan_host_program(typed).expect("nested callback failure source should plan");
    let execution =
        HostedExecution::try_from_module_plan(plan).expect("callback failure source should seal");
    let provider = Component::initialize(&configuration)
        .expect("explicit component configuration should initialize");
    let mut state = RunState { provider };

    let error = execution
        .run_main(&mut state, &mut Vec::new())
        .expect_err("nested callback should preserve the source panic");
    assert!(matches!(error, ExecutionError::Panic(_)));
    assert_eq!(state.provider.calls(), 1);
}
