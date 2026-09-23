use super::{binary_path, checked, command};
use std::fs;
use std::path::Path;

#[test]
fn service_only_producer_is_shared_by_two_generated_consumers() {
    let directory = tempfile::tempdir().unwrap();
    let root = fs::canonicalize(directory.path()).unwrap();
    let application = root.join("application");
    let repository = Path::new(env!("CARGO_MANIFEST_DIR"));
    let target = repository.join("target/prepared-acceptance");
    fs::create_dir_all(application.join("src")).unwrap();
    fs::create_dir_all(application.join("gleam/src")).unwrap();
    fs::create_dir_all(application.join(".cargo")).unwrap();
    let repository_path = repository.to_str().unwrap();
    let target_path = target.to_str().unwrap();
    fs::write(
        application.join(".cargo/config.toml"),
        toml::to_string(&toml::toml! {
            [patch.crates-io.geam]
            path = repository_path
            [net]
            offline = true
            [build]
            target-dir = target_path
        })
        .unwrap(),
    )
    .unwrap();
    fs::write(application.join("Cargo.toml"), format!(r#"
[package]
name = "service-composition-consumer"
version = "0.1.0"
edition = "2024"
[package.metadata.geam.embedding]
generate = "both"
[dependencies]
geam = {{ version = "={}", default-features = false, features = ["embedding", "geam-builtin", "tokio"] }}
producer_alias = {{ package = "fixture-tickets", path = "../tickets" }}
first_alias = {{ package = "fixture-first", path = "../first" }}
second_alias = {{ package = "fixture-second", path = "../second" }}
tokio = {{ version = "1", features = ["rt", "time"] }}
[workspace]
"#, env!("CARGO_PKG_VERSION"))).unwrap();
    fs::write(application.join("src/main.rs"), "fn main() {}\n").unwrap();
    fs::write(
        application.join("gleam/gleam.toml"),
        r#"
name = "service_composition_consumer"
version = "1.0.0"
[dependencies]
first_service = { path = "packages/first_service" }
second_service = { path = "packages/second_service" }
"#,
    )
    .unwrap();
    fs::write(
        application.join("gleam/src/service_composition_consumer.gleam"),
        r#"
import first_service
import second_service
pub fn main() {
  let assert 40 = first_service.next()
  let assert 41 = second_service.next()
  let assert 42 = first_service.next()
  Nil
}
"#,
    )
    .unwrap();

    let tickets = root.join("tickets");
    fs::create_dir_all(tickets.join("src")).unwrap();
    fs::write(
        tickets.join("Cargo.toml"),
        format!(
            r#"
[package]
name = "fixture-tickets"
version = "1.0.0"
edition = "2024"
[package.metadata.geam.provider]
schema = 2
gleam-package = "ticket_service"
gleam-version = "1.0.0"
component = "plain"
execution-service = true
requires-services = []
[dependencies]
geam = {{ version = "={}", default-features = false, features = ["provider"] }}
[workspace]
"#,
            env!("CARGO_PKG_VERSION")
        ),
    )
    .unwrap();
    fs::write(tickets.join("src/lib.rs"), r#"
use geam::{HostComponentProfile, HostExecutionService, HostProviderComponent, HostProviderComponentInitialization, HostProviderComponentRegistration, HostProviderConfiguration, HostProviderInitializationError, HostProviderModule, HostRegistrationError};

pub struct Component;
#[derive(Default)]
pub struct Counter { pub next: usize }
impl geam::execution::HostExecutionState for Counter {
    fn started(&mut self, _: geam::execution::ExecutionUnit) {}
    fn finished(&mut self, _: geam::execution::ExecutionUnitId, _: &geam::execution::UnitExit) {}
    fn close(&mut self) { self.next = 0; }
}
impl HostProviderComponent for Component {
    const ID: &'static str = "ticket_service";
    type Stores = ();
    type RunState = usize;
}
impl HostProviderComponentInitialization for Component {
    fn initialize(configuration: &HostProviderConfiguration) -> Result<usize, HostProviderInitializationError> {
        let start = configuration.get("start").and_then(|value| value.as_string())
            .ok_or_else(|| HostProviderInitializationError::for_component::<Self>("start is required"))?;
        start.parse().map_err(|error| HostProviderInitializationError::for_component::<Self>(format!("invalid start: {error}")))
    }
}
impl HostExecutionService for Component {
    type State = Counter;
    fn initialize_service(start: &mut usize) -> Counter { Counter { next: *start } }
}
impl<Profile: HostComponentProfile<Self>> HostProviderComponentRegistration<Profile> for Component {
    fn providers() -> Result<Vec<HostProviderModule<Profile>>, HostRegistrationError> { Ok(Vec::new()) }
}
"#).unwrap();
    let source = application.join("gleam/packages/ticket_service");
    fs::create_dir_all(source.join("src")).unwrap();
    fs::write(
        source.join("gleam.toml"),
        "name = \"ticket_service\"\nversion = \"1.0.0\"\n",
    )
    .unwrap();
    fs::write(
        source.join("src/ticket_service.gleam"),
        "pub fn available() { Nil }\n",
    )
    .unwrap();

    for name in ["first", "second"] {
        let provider = root.join(name);
        fs::create_dir_all(provider.join("src")).unwrap();
        fs::write(
            provider.join("Cargo.toml"),
            format!(
                r#"
[package]
name = "fixture-{name}"
version = "1.0.0"
edition = "2024"
[package.metadata.geam.provider]
schema = 2
gleam-package = "{name}_service"
gleam-version = "1.0.0"
component = "plain"
execution-service = false
requires-services = ["ticket_service"]
[dependencies]
geam = {{ version = "={version}", default-features = false, features = ["provider"] }}
tickets = {{ package = "fixture-tickets", path = "../tickets" }}
[workspace]
"#,
                version = env!("CARGO_PKG_VERSION")
            ),
        )
        .unwrap();
        fs::write(provider.join("src/lib.rs"), format!(r#"
pub trait ServiceProfile: geam::HostServiceProfile<tickets::Component> {{}}
impl<Profile: geam::HostServiceProfile<tickets::Component>> ServiceProfile for Profile {{}}
#[geam::provider(package = "{name}_service", modules = [api])]
pub struct Component;
#[geam::module(path = "{name}_service", profile = crate::ServiceProfile, component = crate::Component, crate_path = geam)]
mod api {{
    use geam::provider::{{BigInt, Call}};
    #[geam::function(profile = Profile)]
    fn next(#[geam::call] call: &mut Call<()>) -> BigInt {{
        let state = call.service::<tickets::Component>();
        let result = state.next;
        state.next += 1;
        result.into()
    }}
}}
"#)).unwrap();
        let source = application.join(format!("gleam/packages/{name}_service"));
        fs::create_dir_all(source.join("src")).unwrap();
        fs::write(source.join("gleam.toml"), format!("name = \"{name}_service\"\nversion = \"1.0.0\"\n[dependencies]\nticket_service = {{ path = \"../ticket_service\" }}\n")).unwrap();
        fs::write(
            source.join(format!("src/{name}_service.gleam")),
            "@external(erlang, \"service\", \"next\")\npub fn next() -> Int\n",
        )
        .unwrap();
    }

    checked(command(env!("CARGO_BIN_EXE_geam"), &application).args(["embedding", "sync"]));
    fs::write(
        application.join("src/main.rs"),
        r#"
mod geam_bindings;
use geam::{HostProviderConfiguration, HostProviderConfigurationValue};
use geam::embedding::HostedModuleBuilder;
use geam::execution::TokioHost;
use std::collections::BTreeMap;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let executor = tokio::runtime::Builder::new_current_thread().enable_time().build()?;
    let host = TokioHost::new(executor.handle().clone());
    let mut executions = Vec::new();
    if std::env::args().nth(1).as_deref() != Some("--prepared") {
        let program = geam_bindings::project().compile()?;
        let (bindings, functions) = geam_bindings::bind(HostedModuleBuilder::new(program)?)?;
        executions.push((bindings.seal()?, functions));
    }
    executions.push(geam_bindings::load()?);
    for (mut module, functions) in executions {
        let mut state = geam_bindings::RunStateInputs {
            first_service: HostProviderConfiguration::empty(),
            second_service: HostProviderConfiguration::empty(),
            ticket_service: HostProviderConfiguration::new(BTreeMap::from([
                ("start".into(), HostProviderConfigurationValue::String("40".into())),
            ])),
        }.initialize()?;
        for _ in 0..2 {
            let mut echo = Vec::new();
            executor.block_on(module.with_execution(&host, &mut state, &mut echo, async |scope| {
                scope.call(&functions.main, ()).await
            }))??;
            assert!(echo.is_empty());
            println!("shared services: 40, 41, 42");
        }
    }
    Ok(())
}
"#,
    )
    .unwrap();
    let generated = fs::read(application.join("src/geam_bindings.rs")).unwrap();
    let artifact = fs::read(application.join("src/geam_bindings/program.rs")).unwrap();
    let lock = fs::read(application.join("Cargo.lock")).unwrap();
    checked(command(env!("CARGO_BIN_EXE_geam"), &application).args(["embedding", "sync"]));
    assert_eq!(
        fs::read(application.join("src/geam_bindings.rs")).unwrap(),
        generated
    );
    assert_eq!(
        fs::read(application.join("src/geam_bindings/program.rs")).unwrap(),
        artifact
    );
    assert_eq!(fs::read(application.join("Cargo.lock")).unwrap(), lock);
    let output = checked(command("cargo", &application).args(["run", "--quiet", "--locked"]));
    assert_eq!(output.stdout, b"shared services: 40, 41, 42\n".repeat(4));
    assert!(output.stderr.is_empty());

    let deploy = root.join("deployment");
    fs::create_dir(&deploy).unwrap();
    let binary = binary_path(&deploy, "services");
    fs::copy(
        binary_path(&target.join("debug"), "service-composition-consumer"),
        &binary,
    )
    .unwrap();
    fs::remove_dir_all(application).unwrap();
    for name in ["tickets", "first", "second"] {
        fs::remove_dir_all(root.join(name)).unwrap();
    }
    let output = checked(command(binary, &deploy).arg("--prepared").env("PATH", ""));
    assert_eq!(output.stdout, b"shared services: 40, 41, 42\n".repeat(2));
    assert!(output.stderr.is_empty());
}
