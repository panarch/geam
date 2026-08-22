use ecow::EcoString;
use geam_core::provider::{Configuration, InitializationError};
use geam_core::{
    HostComponentProfile, HostModule, HostProfile, HostProviderComponent,
    HostProviderComponentInitialization, HostProviderComponentRegistration, HostProviderSet,
    HostedExecution, ModuleSource, PackageSource, PlanError, Value, ValueType,
    compile_typed_host_program, plan_host_program,
};
use std::collections::BTreeMap;

#[derive(Debug)]
pub struct RunState {
    next: i64,
}

fn initialize(configuration: &Configuration) -> Result<RunState, InitializationError> {
    let next = configuration
        .get("start")
        .and_then(|value| value.as_integer())
        .ok_or_else(|| InitializationError::new("configuration key `start` must be an Integer"))?;
    Ok(RunState { next })
}

#[geam_macros::provider(
    id = "macro-counter",
    package = "counter",
    state = RunState,
    initialize = initialize,
    modules = [counter, labels],
    crate_path = geam_core,
)]
pub struct Component;

#[geam_macros::module(path = "counter", crate_path = geam_core)]
mod counter {
    use super::{EcoString, RunState};

    fn render(label: EcoString, next: i64) -> EcoString {
        format!("{label}:{next}").into()
    }

    #[geam_macros::function]
    fn next(#[geam_macros::state] state: &mut RunState, label: EcoString) -> EcoString {
        let next = state.next;
        state.next += 1;
        render(label, next)
    }

    #[geam_macros::function]
    fn peek(#[geam_macros::state] state: &RunState, label: EcoString) -> EcoString {
        render(label, state.next)
    }
}

#[geam_macros::module(path = "counter/labels", crate_path = geam_core)]
mod labels {
    use super::EcoString;

    #[geam_macros::function]
    fn identity(label: EcoString) -> EcoString {
        label
    }
}

struct Profile;

#[derive(Default)]
struct ProfileStores {
    component: <Component as HostProviderComponent>::Stores,
}

struct ProfileState {
    component: <Component as HostProviderComponent>::RunState,
}

impl HostProfile for Profile {
    type RunState = ProfileState;
    type ExternalStores = ProfileStores;
}

impl HostComponentProfile<Component> for Profile {
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

const COUNTER_SOURCE: &str = r#"
@external(erlang, "macro_counter", "next")
pub fn next(label: String) -> String

@external(erlang, "macro_counter", "peek")
pub fn peek(label: String) -> String

pub fn main() {
  let _ = next("count")
  peek("count")
}
"#;

const LABELS_SOURCE: &str = r#"
@external(erlang, "macro_counter", "identity")
pub fn identity(label: String) -> String
"#;

fn configuration(start: i64) -> Configuration {
    Configuration::new(BTreeMap::from([(EcoString::from("start"), start.into())]))
}

fn providers() -> Vec<geam_core::HostProviderModule<Profile>> {
    <Component as HostProviderComponentRegistration<Profile>>::providers()
        .expect("macro-authored component should register")
}

fn execution() -> HostedExecution<Profile> {
    let hosts = HostProviderSet::with_providers(Vec::<HostModule<Profile>>::new(), providers())
        .expect("macro-authored modules should be unique");
    let typed = compile_typed_host_program(
        "counter",
        "counter",
        [PackageSource::new(
            "counter",
            Vec::<&str>::new(),
            [
                ModuleSource::new("counter", "src/counter.gleam", COUNTER_SOURCE),
                ModuleSource::new("counter/labels", "src/counter/labels.gleam", LABELS_SOURCE),
            ],
        )],
        hosts,
    )
    .expect("complete scalar provider source should compile");
    let plan = plan_host_program(typed).expect("matching scalar provider should plan");
    HostedExecution::try_from_module_plan(plan).expect("matching scalar provider should seal")
}

#[test]
fn macro_authored_schema_preserves_component_module_and_function_order() {
    assert_eq!(Component::ID, "macro-counter");
    let providers = providers();
    assert_eq!(providers.len(), 2);
    assert_eq!(providers[0].package().as_str(), "counter");
    assert_eq!(providers[0].module().as_str(), "counter");
    assert_eq!(
        providers[0]
            .functions()
            .map(|function| function.name().as_str())
            .collect::<Vec<_>>(),
        vec!["next", "peek"],
    );
    assert_eq!(providers[1].package().as_str(), "counter");
    assert_eq!(providers[1].module().as_str(), "counter/labels");
    assert_eq!(
        providers[1]
            .functions()
            .map(|function| function.name().as_str())
            .collect::<Vec<_>>(),
        vec!["identity"],
    );
    let next = providers[0]
        .functions()
        .next()
        .expect("next schema should be present")
        .type_();
    assert_eq!(next.argument_types(), &[ValueType::String]);
    assert_eq!(next.return_(), &ValueType::String);
    let peek = providers[0]
        .functions()
        .nth(1)
        .expect("peek schema should be present")
        .type_();
    assert_eq!(peek.argument_types(), &[ValueType::String]);
    assert_eq!(peek.return_(), &ValueType::String);
}

#[test]
fn generated_initialization_adds_identity_to_owned_provider_failures() {
    let error = Component::initialize(&Configuration::empty())
        .expect_err("missing start configuration should fail");

    assert_eq!(error.component_id(), "macro-counter");
    assert_eq!(
        error.reason(),
        "configuration key `start` must be an Integer"
    );
    assert_eq!(
        Component::initialize(&configuration(3))
            .expect("integer start should initialize")
            .next,
        3,
    );
}

#[test]
fn macro_authored_scalar_provider_runs_with_repeated_and_independent_state() {
    let execution = execution();
    let mut first = ProfileState {
        component: Component::initialize(&configuration(3)).expect("first state should initialize"),
    };
    let mut second = ProfileState {
        component: Component::initialize(&configuration(3))
            .expect("second state should initialize"),
    };

    assert_eq!(
        execution.run_main(&mut first, &mut Vec::new()),
        Ok(Value::String("count:4".into())),
    );
    assert_eq!(
        execution.run_main(&mut first, &mut Vec::new()),
        Ok(Value::String("count:5".into())),
    );
    assert_eq!(
        execution.run_main(&mut second, &mut Vec::new()),
        Ok(Value::String("count:4".into())),
    );
    assert_eq!(first.component.next, 5);
    assert_eq!(second.component.next, 4);
}

#[test]
fn gleam_and_rust_shape_mismatch_remains_a_structured_link_error() {
    let mismatched_counter = r#"
@external(erlang, "macro_counter", "next")
pub fn next(label: Bool) -> Bool

@external(erlang, "macro_counter", "peek")
pub fn peek(label: String) -> String

pub fn main() {
  next(True)
}
"#;
    let hosts = HostProviderSet::with_providers(Vec::<HostModule<Profile>>::new(), providers())
        .expect("macro-authored modules should be unique");
    let typed = compile_typed_host_program(
        "counter",
        "counter",
        [PackageSource::new(
            "counter",
            Vec::<&str>::new(),
            [
                ModuleSource::new("counter", "src/counter.gleam", mismatched_counter),
                ModuleSource::new("counter/labels", "src/counter/labels.gleam", LABELS_SOURCE),
            ],
        )],
        hosts,
    )
    .expect("mismatched host source should still compile");

    let error = match plan_host_program(typed) {
        Err(error) => error,
        Ok(_) => panic!("mismatched signature should fail during linkage"),
    };
    let PlanError::HostProviderLink {
        package,
        module,
        function,
        reason,
    } = error
    else {
        panic!("signature mismatch should remain a host provider linkage error");
    };
    assert_eq!(package.as_str(), "counter");
    assert_eq!(module.as_str(), "counter");
    assert_eq!(function.as_str(), "next");
    let geam_core::HostProviderLinkReason::SchemeMismatch {
        expected_scheme,
        expected_type,
        actual_scheme,
        actual_type,
    } = *reason
    else {
        panic!("linkage error should preserve the exact scheme mismatch");
    };
    assert!(expected_scheme.parameters().is_empty());
    assert_eq!(expected_type.argument_types(), &[ValueType::Bool]);
    assert_eq!(expected_type.return_(), &ValueType::Bool);
    assert!(actual_scheme.parameters().is_empty());
    assert_eq!(actual_type.argument_types(), &[ValueType::String]);
    assert_eq!(actual_type.return_(), &ValueType::String);
}
