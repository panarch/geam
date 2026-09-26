use ecow::EcoString;
extern crate geam as geam_core;
#[path = "../../../../support/execution_host.rs"]
mod execution_fixture;
use geam::{
    ExecutionError, HostComponentProfile, HostModule, HostProfile, HostProviderComponent,
    HostProviderComponentInitialization, HostProviderComponentRegistration,
    HostProviderConfiguration, HostProviderSet, HostedExecution, ModuleSource, PackageSource,
    Value, compile_typed_host_program, plan_host_program,
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
    type ExecutionState = ();
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

const PROVIDER_SOURCE: &str = r#"
@external(erlang, "provider_sdk", "Catalog")
pub type Catalog

@external(erlang, "provider_sdk", "decorate")
pub fn decorate(value: String, transform: fn(String) -> String) -> String

@external(erlang, "provider_sdk", "make_transform")
pub fn make_transform(prefix: String) -> fn(String) -> String

@external(erlang, "provider_sdk", "catalog_new")
pub fn catalog_new() -> Catalog

@external(erlang, "provider_sdk", "catalog_insert")
pub fn catalog_insert(catalog: Catalog, key: String, value: String) -> Catalog

@external(erlang, "provider_sdk", "catalog_hash")
pub fn catalog_hash(catalog: Catalog) -> Int

pub type Summary {
  Summary(count: Int, items: List(String))
}

@external(erlang, "provider_sdk", "summarize")
pub fn summarize(value: String, transform: fn(String) -> String) -> Summary

@external(erlang, "provider_sdk", "non_empty")
pub fn non_empty(value: String) -> Result(String, Int)
@external(erlang, "provider_sdk", "around")
pub fn around(cleanup: fn(Nil) -> b, body: fn(Nil) -> a) -> a
@external(erlang, "provider_sdk", "produce")
pub fn produce() -> a
@external(erlang, "provider_sdk", "late_failure")
pub fn late_failure() -> a
@external(erlang, "provider_sdk", "native_value")
pub fn native_value() -> a

"#;

#[test]
fn independent_path_provider_public_usage() {
    let main_source = r#"
import provider/sdk

pub fn main() {
  let decorated = sdk.decorate("item", fn(value) { value <> "!" })
  let empty = sdk.catalog_new()
  let catalog = sdk.catalog_insert(empty, "one", decorated)
  let matching = sdk.catalog_insert(sdk.catalog_new(), "one", "sdk:item!")
  assert empty != catalog
  assert catalog == matching
  assert sdk.catalog_hash(catalog) == sdk.catalog_hash(matching)
  let transform = sdk.make_transform("native:")
  let alias = transform
  assert alias == transform
  assert transform != sdk.make_transform("native:")
  assert alias("x") == "native:x"
  let summary = sdk.summarize(decorated, transform)
  let rejected = sdk.non_empty("")
  let assert Error(0) = rejected
  assert rejected == Error(0)
  let accepted = sdk.non_empty("한국어\u{0}🙂")
  let assert Ok("한국어\u{0}🙂") = accepted
  assert accepted == Ok("한국어\u{0}🙂")
  #(catalog, summary, accepted, rejected)
}
"#;
    let configuration = HostProviderConfiguration::new(BTreeMap::from([(
        EcoString::from("prefix"),
        EcoString::from("sdk:").into(),
    )]));
    let component_state = Component::initialize(&configuration)
        .expect("explicit component configuration should initialize");
    let provider_modules = <Component as HostProviderComponentRegistration<Profile>>::providers()
        .expect("path component should register its provider modules");
    let hosts =
        HostProviderSet::with_providers(Vec::<HostModule<Profile>>::new(), provider_modules)
            .expect("path component modules should be unique");
    let typed = compile_typed_host_program(
        "provider_sdk_example",
        "main",
        [PackageSource::new(
            "provider_sdk_example",
            Vec::<&str>::new(),
            [
                ModuleSource::new("provider/sdk", "src/provider/sdk.gleam", PROVIDER_SOURCE),
                ModuleSource::new("main", "src/main.gleam", main_source),
            ],
        )],
        hosts,
    )
    .expect("complete path provider example should compile");
    let plan = plan_host_program(typed).expect("complete path provider example should plan");
    let mut execution =
        HostedExecution::try_from_module_plan(plan).expect("path provider example should seal");
    let mut state = RunState {
        provider: component_state,
    };

    let returned = crate::execution_fixture::run(&mut execution, &mut state, &mut Vec::new())
        .expect("path provider example should run");

    assert_eq!(
        returned.inspect().to_string(),
        r#"#(Catalog([#("one", "sdk:item!")]), Summary(count: 1, items: ["native:sdk:item!"]), Ok("한국어\0🙂"), Error(0))"#,
    );
    assert_eq!(state.provider.prefix(), "sdk:");
    assert_eq!(state.provider.calls(), 3);
}

#[test]
fn component_run_states_are_independent() {
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
    let provider_modules = <Component as HostProviderComponentRegistration<Profile>>::providers()
        .expect("path component should register its provider modules");
    let hosts =
        HostProviderSet::with_providers(Vec::<HostModule<Profile>>::new(), provider_modules)
            .expect("path component modules should be unique");
    let typed = compile_typed_host_program(
        "provider_sdk_example",
        "main",
        [PackageSource::new(
            "provider_sdk_example",
            Vec::<&str>::new(),
            [
                ModuleSource::new("provider/sdk", "src/provider/sdk.gleam", PROVIDER_SOURCE),
                ModuleSource::new("main", "src/main.gleam", main_source),
            ],
        )],
        hosts,
    )
    .expect("independent state example should compile");
    let plan = plan_host_program(typed).expect("independent state example should plan");
    let mut execution =
        HostedExecution::try_from_module_plan(plan).expect("independent state example should seal");
    let mut first = RunState {
        provider: Component::initialize(&configuration)
            .expect("first component state should initialize"),
    };
    let mut second = RunState {
        provider: Component::initialize(&configuration)
            .expect("second component state should initialize"),
    };

    assert_eq!(
        crate::execution_fixture::run(&mut execution, &mut first, &mut Vec::new()),
        Ok(Value::String("sdk:item!".into())),
    );
    assert_eq!(
        crate::execution_fixture::run(&mut execution, &mut first, &mut Vec::new()),
        Ok(Value::String("sdk:item!".into())),
    );
    assert_eq!(
        crate::execution_fixture::run(&mut execution, &mut second, &mut Vec::new()),
        Ok(Value::String("sdk:item!".into())),
    );
    assert_eq!(first.provider.calls(), 2);
    assert_eq!(second.provider.calls(), 1);
}

#[test]
fn escaped_external_value_owns_its_payload() {
    let main_source = r#"
import provider/sdk

pub fn main() {
  let empty = sdk.catalog_new()
  let first = sdk.catalog_insert(empty, "one", "1")
  let same = sdk.catalog_insert(sdk.catalog_new(), "one", "1")
  assert empty != first
  assert first == same
  assert sdk.catalog_hash(first) == sdk.catalog_hash(same)
  first
}
"#;
    let configuration = HostProviderConfiguration::new(BTreeMap::from([(
        EcoString::from("prefix"),
        EcoString::from("sdk:").into(),
    )]));
    let component_state = Component::initialize(&configuration)
        .expect("explicit component configuration should initialize");
    let provider_modules = <Component as HostProviderComponentRegistration<Profile>>::providers()
        .expect("path component should register its provider modules");
    let hosts =
        HostProviderSet::with_providers(Vec::<HostModule<Profile>>::new(), provider_modules)
            .expect("path component modules should be unique");
    let typed = compile_typed_host_program(
        "provider_sdk_example",
        "main",
        [PackageSource::new(
            "provider_sdk_example",
            Vec::<&str>::new(),
            [
                ModuleSource::new("provider/sdk", "src/provider/sdk.gleam", PROVIDER_SOURCE),
                ModuleSource::new("main", "src/main.gleam", main_source),
            ],
        )],
        hosts,
    )
    .expect("external ownership example should compile");
    let plan = plan_host_program(typed).expect("external ownership example should plan");
    let mut execution = HostedExecution::try_from_module_plan(plan)
        .expect("external ownership example should seal");
    let mut state = RunState {
        provider: component_state,
    };
    let returned = crate::execution_fixture::run(&mut execution, &mut state, &mut Vec::new())
        .expect("external ownership example should run");

    drop(state);
    drop(execution);

    let Value::External(catalog) = returned else {
        panic!("external ownership example should return an opaque external value");
    };
    assert_eq!(catalog.inspection(), "Catalog([#(\"one\", \"1\")])");
    assert_eq!(catalog.clone(), catalog);
}

#[test]
fn stateful_callbacks_preserve_nested_source_failures() {
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
    let component_state = Component::initialize(&configuration)
        .expect("explicit component configuration should initialize");
    let provider_modules = <Component as HostProviderComponentRegistration<Profile>>::providers()
        .expect("path component should register its provider modules");
    let hosts =
        HostProviderSet::with_providers(Vec::<HostModule<Profile>>::new(), provider_modules)
            .expect("path component modules should be unique");
    let typed = compile_typed_host_program(
        "provider_sdk_example",
        "main",
        [PackageSource::new(
            "provider_sdk_example",
            Vec::<&str>::new(),
            [
                ModuleSource::new("provider/sdk", "src/provider/sdk.gleam", PROVIDER_SOURCE),
                ModuleSource::new("main", "src/main.gleam", main_source),
            ],
        )],
        hosts,
    )
    .expect("stateful callback failure source should compile");
    let plan = plan_host_program(typed).expect("stateful callback failure source should plan");
    let mut execution = HostedExecution::try_from_module_plan(plan)
        .expect("stateful callback failure source should seal");
    let mut state = RunState {
        provider: component_state,
    };

    let error = crate::execution_fixture::run(&mut execution, &mut state, &mut Vec::new())
        .expect_err("nested callback should preserve the source panic");

    assert!(matches!(error, ExecutionError::Panic(_)));
    assert_eq!(state.provider.calls(), 1);
}

#[test]
fn constructing_callbacks_preserve_nested_source_failures() {
    let main_source = r#"
import provider/sdk

pub fn main() {
  sdk.summarize("item", fn(_) { panic as "summary callback failed" })
}
"#;
    let configuration = HostProviderConfiguration::new(BTreeMap::from([(
        EcoString::from("prefix"),
        EcoString::from("sdk:").into(),
    )]));
    let component_state = Component::initialize(&configuration)
        .expect("explicit component configuration should initialize");
    let provider_modules = <Component as HostProviderComponentRegistration<Profile>>::providers()
        .expect("path component should register its provider modules");
    let hosts =
        HostProviderSet::with_providers(Vec::<HostModule<Profile>>::new(), provider_modules)
            .expect("path component modules should be unique");
    let typed = compile_typed_host_program(
        "provider_sdk_example",
        "main",
        [PackageSource::new(
            "provider_sdk_example",
            Vec::<&str>::new(),
            [
                ModuleSource::new("provider/sdk", "src/provider/sdk.gleam", PROVIDER_SOURCE),
                ModuleSource::new("main", "src/main.gleam", main_source),
            ],
        )],
        hosts,
    )
    .expect("constructing callback failure source should compile");
    let plan = plan_host_program(typed).expect("constructing callback failure source should plan");
    let mut execution = HostedExecution::try_from_module_plan(plan)
        .expect("constructing callback failure source should seal");
    let mut state = RunState {
        provider: component_state,
    };

    let error = crate::execution_fixture::run(&mut execution, &mut state, &mut Vec::new())
        .expect_err("constructing callback should preserve the source panic");

    assert!(matches!(error, ExecutionError::Panic(_)));
    assert_eq!(state.provider.calls(), 0);
}

#[test]
fn generic_public_registrations_keep_concrete_values_and_unresolved_errors() {
    for (body, expected, effects, calls) in [
        (
            r#"sdk.around(fn(_) { echo "cleanup" }, fn(_) { echo "body" 42 })"#,
            Ok("42"),
            vec!["\"body\"", "\"cleanup\""],
            1,
        ),
        (
            r#"let _ = sdk.around(fn(_) { echo "cleanup" }, fn(_) { echo "body" panic as "stopped" }) 42"#,
            Err("panic: stopped"),
            vec!["\"body\"", "\"cleanup\""],
            1,
        ),
        (
            r#"let _ = sdk.around(fn(_) { echo "cleanup" panic as "cleanup stopped" }, fn(_) { echo "body" panic as "stopped" }) 42"#,
            Err("panic: cleanup stopped"),
            vec!["\"body\"", "\"cleanup\""],
            1,
        ),
        ("let _ = sdk.produce 42", Ok("42"), vec![], 0),
        (
            "let _ = sdk.produce() 42",
            Err(
                "host function provider_sdk_example::provider/sdk.produce failed: producer stopped",
            ),
            vec![],
            1,
        ),
        (
            "sdk.produce() + 1",
            Err(
                "host function provider_sdk_example::provider/sdk.produce failed: producer stopped",
            ),
            vec![],
            1,
        ),
        (
            "let _ = sdk.late_failure() 42",
            Err(
                "host function provider_sdk_example::provider/sdk.late_failure failed: codec stopped",
            ),
            vec![],
            1,
        ),
        (
            "sdk.late_failure() + 1",
            Err(
                "host function provider_sdk_example::provider/sdk.late_failure failed: codec stopped",
            ),
            vec![],
            1,
        ),
        ("let _: Nil = sdk.native_value() 42", Ok("42"), vec![], 1),
        (
            "let _ = sdk.native_value() 42",
            Err(
                "host function provider_sdk_example::provider/sdk.native_value failed: native value does not match the registered return type",
            ),
            vec![],
            1,
        ),
    ] {
        let configuration = HostProviderConfiguration::new(BTreeMap::from([(
            "prefix".into(),
            EcoString::from("sdk:").into(),
        )]));
        let mut state = RunState {
            provider: Component::initialize(&configuration).unwrap(),
        };
        let providers =
            <Component as HostProviderComponentRegistration<Profile>>::providers().unwrap();
        let hosts = HostProviderSet::from_providers(providers).unwrap();
        let source = format!("import provider/sdk\npub fn main() {{ {body} }}");
        let typed = compile_typed_host_program(
            "provider_sdk_example",
            "main",
            [PackageSource::new(
                "provider_sdk_example",
                Vec::<&str>::new(),
                [
                    ModuleSource::new("provider/sdk", "src/provider/sdk.gleam", PROVIDER_SOURCE),
                    ModuleSource::new("main", "src/main.gleam", source),
                ],
            )],
            hosts,
        )
        .unwrap();
        let mut execution =
            HostedExecution::try_from_module_plan(plan_host_program(typed).unwrap()).unwrap();
        let mut echo = Vec::new();
        let result = execution_fixture::run(&mut execution, &mut state, &mut echo);
        assert_eq!(
            result
                .map(|value| value.inspect().to_string())
                .map_err(|error| error.to_string()),
            expected.map(str::to_owned).map_err(str::to_owned),
            "{body}"
        );
        assert_eq!(state.provider.calls(), calls);
        assert_eq!(
            echo.iter()
                .map(|echo| echo.value().inspect().to_string())
                .collect::<Vec<_>>(),
            effects
        );
    }
}
