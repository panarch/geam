use super::{binary_path, checked, command, workspace_dependencies};
use std::fs;
use std::path::Path;

#[path = "../support/guard_constructor_fixture.rs"]
mod fixture;

#[test]
fn guard_locals_multi_subject_patterns_and_original_clip_run_dynamic_and_relocated_prepared() {
    let directory = tempfile::tempdir().unwrap();
    let root = fs::canonicalize(directory.path()).unwrap();
    let application = root.join("application");
    let repository = Path::new(env!("CARGO_MANIFEST_DIR"));
    let target = repository.join("target/prepared-acceptance");
    fs::create_dir_all(application.join("src")).unwrap();
    fs::create_dir_all(application.join(".cargo")).unwrap();
    fixture::copy_project(&application.join("gleam"));
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
name = "guard-constructor-locals"
version = "0.1.0"
edition = "2024"
[package.metadata.geam.embedding]
generate = "both"
[dependencies]
geam = {{ version = "={}", default-features = false, features = ["embedding", "gleam-stdlib", "tokio"] }}
tokio = {{ version = "1", features = ["rt"] }}
[workspace]
"#, env!("CARGO_PKG_VERSION"))).unwrap();
    fs::write(
        application.join("src/main.rs"),
        r#"
#[allow(dead_code)]
mod geam_bindings;

use geam::embedding::HostedModuleBuilder;
use geam::execution::TokioHost;
use geam::gleam_stdlib::GleamStdlibRunState;

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
            stdlib: GleamStdlibRunState::from_seed([0; 32]),
        }.initialize();
        let mut echo = Vec::new();
        executor.block_on(module.with_execution(&host, &mut state, &mut echo, async |scope| {
            scope.call(&functions.main, ()).await?;
            assert!(scope.call(&functions.matches_int, (7.into(), 7.into())).await?);
            assert!(!scope.call(&functions.matches_int, (7.into(), 8.into())).await?);
            assert!(scope.call(&functions.matches_string, ("ab".into(), "ab".into())).await?);
            assert!(!scope.call(&functions.matches_string, ("ab".into(), "ac".into())).await?);
            assert!(scope.call(&functions.captured_match, (7.into(), 7.into())).await?);
            for (input, expected) in [
                ((true, false), true), ((false, false), false),
                ((true, true), false), ((true, false), true),
            ] {
                assert_eq!(scope.call(&functions.remainder_bool, input).await?, expected);
                assert_eq!(scope.call(&functions.remainder_custom, input).await?, expected);
            }
            for (value, offset, expected, matches) in [
                ("9223372036854775807", "1", "9223372036854775807", true),
                ("7", "170141183460469231731687303715884105727", "7", true),
                ("170141183460469231731687303715884105727", "1", "170141183460469231731687303715884105727", true),
                ("170141183460469231731687303715884105727", "1", "170141183460469231731687303715884105726", false),
                ("7", "-9223372036854775816", "7", true),
                ("7", "170141183460469231731687303715884105727", "7", true),
            ] {
                assert_eq!(scope.call(&functions.arithmetic_captured_match, (
                    value.parse().unwrap(), offset.parse().unwrap(), expected.parse().unwrap(),
                )).await?, matches);
            }
            assert!(!scope.call(&functions.captured_match, (7.into(), 8.into())).await?);
            assert!(scope.call(&functions.captured_match, (7.into(), 7.into())).await?);
            for (namespace, inline, fragment, expected) in [
                ("", true, false, false),
                ("html", true, false, true),
                ("", false, false, true),
                ("html", false, true, false),
                ("html", true, true, false),
                ("svg", true, false, true),
                ("", true, false, false),
                ("html", true, false, true),
            ] {
                assert_eq!(scope.call(&functions.guard_record_field, (
                    namespace.into(), inline, fragment,
                )).await?, expected);
            }
            Ok::<_, geam::embedding::CallError>(())
        }))?.try_into_value().unwrap()?;
        assert!(echo.is_empty());
        assert_eq!(state.stdlib().io_outputs().len(), 1);
        for output in state.stdlib_mut().take_io_outputs() {
            print!("{}", output.text());
        }
    }
    Ok(())
}
"#,
    )
    .unwrap();
    checked(command(env!("CARGO_BIN_EXE_geam"), &application).args(["embedding", "sync"]));
    let program = application.join("src/geam_bindings/program.rs");
    let prepared = fs::read(&program).unwrap();
    checked(command(env!("CARGO_BIN_EXE_geam"), &application).args(["embedding", "check"]));
    checked(command(env!("CARGO_BIN_EXE_geam"), &application).args(["embedding", "sync"]));
    assert_eq!(fs::read(&program).unwrap(), prepared);
    checked(command("cargo", &application).args(["fmt", "--all"]));
    assert_eq!(fs::read(&program).unwrap(), prepared);
    checked(command("cargo", &application).args([
        "clippy",
        "--all-targets",
        "--locked",
        "--",
        "-D",
        "warnings",
    ]));
    let output = checked(command("cargo", &application).args(["run", "--quiet", "--locked"]));
    assert_eq!(output.stdout, fixture::OUTPUT.repeat(2));
    assert!(output.stderr.is_empty());
    let deployment = root.join("deployment");
    fs::create_dir_all(&deployment).unwrap();
    let executable = binary_path(&deployment, "guard consumer");
    fs::copy(
        binary_path(&target.join("debug"), "guard-constructor-locals"),
        &executable,
    )
    .unwrap();
    fs::remove_dir_all(&application).unwrap();
    assert!(!application.exists());
    for _ in 0..2 {
        let output = checked(
            command(&executable, &deployment)
                .arg("--prepared")
                .env("PATH", ""),
        );
        assert_eq!(output.stdout, fixture::OUTPUT);
        assert!(output.stderr.is_empty());
    }
}
