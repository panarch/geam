use super::{binary_path, checked, command};
use std::fs;
use std::path::Path;

#[path = "../support/large_custom_fixture.rs"]
mod fixture;

const OUTPUT: &[u8] = b"all 8, 79 and 160 constructors: ok\n";

#[test]
fn large_custom_provider_runs_dynamic_and_relocated_compiled_prepared() {
    let directory = tempfile::tempdir().unwrap();
    let root = fs::canonicalize(directory.path()).unwrap();
    let application = root.join("application");
    let provider = root.join("provider");
    fs::create_dir_all(application.join("src")).unwrap();
    fixture::create_project(&application.join("gleam"));
    fixture::create_provider(&provider);
    fixture::configure_cargo(&application);
    fs::write(
        application.join("Cargo.toml"),
        format!(
            r#"
[package]
name = "custom-repro"
version = "0.1.0"
edition = "2024"
[package.metadata.geam.embedding]
generate = "both"
[dependencies]
geam = {{ version = "={}", default-features = false, features = ["embedding", "tokio"] }}
large_alias = {{ package = "geam-large-custom-fixture", path = "../provider" }}
tokio = {{ version = "1", features = ["rt"] }}
[workspace]
"#,
            env!("CARGO_PKG_VERSION")
        ),
    )
    .unwrap();
    fs::write(
        application.join("src/main.rs"),
        r#"
#[allow(dead_code)]
mod geam_bindings;

use geam::embedding::HostedModuleBuilder;
use geam::execution::TokioHost;
use geam::HostProviderConfiguration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let executor = tokio::runtime::Builder::new_current_thread().build()?;
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
            custom_repro: HostProviderConfiguration::empty(),
        }.initialize()?;
        for _ in 0..2 {
            let mut echo = Vec::new();
            let result = executor.block_on(module.with_execution(&host, &mut state, &mut echo, async |scope| {
                scope.call(&functions.main, ()).await
            }))?.try_into_value().unwrap()?;
            assert!(result);
            assert!(echo.is_empty());
            println!("all 8, 79 and 160 constructors: ok");
        }
    }
    Ok(())
}
"#,
    )
    .unwrap();
    checked(command(env!("CARGO_BIN_EXE_geam"), &application).args(["embedding", "sync"]));
    let bindings = application.join("src/geam_bindings.rs");
    assert!(
        !fs::read_to_string(&bindings)
            .unwrap()
            .contains("recursion_limit")
    );
    let program = application.join("src/geam_bindings/program.rs");
    let artifact = fs::read(&program).unwrap();
    let lock = fs::read(application.join("Cargo.lock")).unwrap();
    checked(command(env!("CARGO_BIN_EXE_geam"), &application).args(["embedding", "check"]));
    checked(command(env!("CARGO_BIN_EXE_geam"), &application).args(["embedding", "sync"]));
    assert_eq!(fs::read(&program).unwrap(), artifact);
    assert_eq!(fs::read(application.join("Cargo.lock")).unwrap(), lock);
    checked(command("cargo", &application).args(["fmt", "--all"]));
    checked(command("cargo", &application).args([
        "clippy",
        "--all-targets",
        "--locked",
        "--",
        "-D",
        "warnings",
    ]));
    let output = checked(command("cargo", &application).args(["run", "--quiet", "--locked"]));
    assert_eq!(output.stdout, OUTPUT.repeat(4));
    assert!(output.stderr.is_empty());

    let repository = Path::new(env!("CARGO_MANIFEST_DIR"));
    let executable = binary_path(
        &repository.join("target/prepared-acceptance/debug"),
        "custom-repro",
    );
    let deployment = root.join("deployment");
    fs::create_dir_all(&deployment).unwrap();
    let deployed = binary_path(&deployment, "large consumer");
    fs::copy(&executable, &deployed).unwrap();
    assert_eq!(fs::read(&deployed).unwrap(), fs::read(executable).unwrap());
    fs::remove_dir_all(&application).unwrap();
    fs::remove_dir_all(&provider).unwrap();
    for _ in 0..2 {
        let output = checked(
            command(&deployed, &deployment)
                .arg("--prepared")
                .env("PATH", ""),
        );
        assert_eq!(output.stdout, OUTPUT.repeat(2));
        assert!(output.stderr.is_empty());
    }
}
