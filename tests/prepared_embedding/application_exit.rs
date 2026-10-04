use super::{binary_path, checked, command};
use std::fs;
use std::path::Path;

#[path = "../support/application_exit_fixture.rs"]
mod fixture;

#[test]
fn a_compiled_provider_exit_preserves_the_host_and_a_fresh_scope() {
    let directory = tempfile::tempdir().unwrap();
    let root = fs::canonicalize(directory.path()).unwrap();
    let (project, provider) = fixture::copy(&root);
    let application = root.join("application");
    fs::create_dir_all(application.join("src")).unwrap();
    fs::rename(project, application.join("gleam")).unwrap();
    let repository = Path::new(env!("CARGO_MANIFEST_DIR"));
    let target = repository.join("target/prepared-acceptance");
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
name = "application-exit-fixture"
version = "0.1.0"
edition = "2024"
[package.metadata.geam.embedding]
generate = "both"
[dependencies]
geam = {{ version = "={}", default-features = false, features = ["embedding", "geam-builtin", "tokio"] }}
exit_provider = {{ package = "geam-application-exit-fixture", path = "../provider" }}
tokio = {{ version = "1", features = ["rt", "time"] }}
[workspace]
"#, env!("CARGO_PKG_VERSION"))).unwrap();
    fs::write(application.join("src/main.rs"), "fn main() {}\n").unwrap();
    checked(command(env!("CARGO_BIN_EXE_geam"), &application).args(["embedding", "init"]));
    fs::write(application.join("src/main.rs"), r#"
mod geam_bindings;
use geam::embedding::HostedModuleBuilder;
use geam::execution::{ExecutionOutcome, ExitStatus, TokioHost};
use geam::HostProviderConfiguration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let runtime = tokio::runtime::Builder::new_current_thread().enable_time().build()?;
    let host = TokioHost::new(runtime.handle().clone());
    let mut executions = Vec::new();
    if std::env::args().nth(1).as_deref() != Some("--prepared") {
        let typed = geam_bindings::project().compile()?;
        let (bindings, functions) = geam_bindings::bind(HostedModuleBuilder::new(typed)?)?;
        executions.push((bindings.seal()?, functions));
    }
    executions.push(geam_bindings::load()?);
    for (mut module, functions) in executions {
        let mut state = geam_bindings::RunStateInputs { application_exit_fixture: HostProviderConfiguration::empty() }.initialize()?;
        let mut echo = Vec::new();
        for is_async in [false, true] {
            for code in [0, 7] {
                let outcome = runtime.block_on(module.with_execution(&host, &mut state, &mut echo, async |scope| {
                    if is_async {
                        scope.call(&functions.stop_async, (code.into(),)).await
                    } else {
                        scope.call(&functions.stop, (code.into(),)).await
                    }
                }))?;
                assert_eq!(outcome, ExecutionOutcome::Exited(ExitStatus::new(code)));
                assert_eq!(echo.len(), 1);
                assert_eq!(echo[0].value().inspect().to_string(), if is_async { "\"before async\"" } else { "\"before\"" });
                echo.clear();
            }
        }
        let outcome = runtime.block_on(module.with_execution(&host, &mut state, &mut echo, async |scope| {
            let count = scope.call(&functions.normal, ()).await?;
            let ordinary = scope.call(&functions.ordinary_error, ()).await?;
            let integer = scope.call(&functions.ordinary_int, ()).await?;
            assert_eq!(scope.call(&functions.fail, ()).await.unwrap_err().to_string(), "panic: source failure");
            Ok::<_, geam::embedding::CallError>((count, ordinary, integer))
        }))?;
        assert_eq!(outcome, ExecutionOutcome::Returned(Ok((4.into(), Err(7.into()), 7.into()))));
        assert_eq!(echo.len(), 1);
        assert_eq!(echo[0].value().inspect().to_string(), "\"normal\"");
        println!("host alive after exits 0/7; new scope: 4");
    }
    Ok(())
}
"#).unwrap();
    checked(command("cargo", &application).args(["run", "--quiet", "--locked"]));
    let deployment = root.join("deployment");
    fs::create_dir(&deployment).unwrap();
    let binary = binary_path(&deployment, "consumer");
    fs::copy(
        binary_path(&target.join("debug"), "application-exit-fixture"),
        &binary,
    )
    .unwrap();
    fs::remove_dir_all(application).unwrap();
    fs::remove_dir_all(provider).unwrap();
    for _ in 0..2 {
        let output = checked(
            command(&binary, &deployment)
                .arg("--prepared")
                .env("PATH", ""),
        );
        assert_eq!(output.stdout, b"host alive after exits 0/7; new scope: 4\n");
        assert!(output.stderr.is_empty());
    }
}
