use super::{binary_path, checked, command};
use geam::embedding::{CallError, FunctionDeclaration, ModuleBuilder};
use geam::{ExecutionError, PanicKind};
use std::fs;
use std::path::Path;

#[test]
fn generated_call_protocols_are_lint_clean_and_run_without_gleam_sources() {
    let directory = tempfile::tempdir().unwrap();
    let root = fs::canonicalize(directory.path()).unwrap();
    let application = root.join("application");
    let repository = Path::new(env!("CARGO_MANIFEST_DIR"));
    let target = repository.join("target/prepared-acceptance");
    fs::create_dir_all(application.join("src")).unwrap();
    fs::create_dir_all(application.join(".cargo")).unwrap();
    fs::write(
        application.join("Cargo.toml"),
        format!(
            r#"
[package]
name = 'prepared-call-lints'
version = '0.1.0'
edition = '2024'

[dependencies]
geam = {{ version = '={}', default-features = false, features = ['embedding'] }}

[workspace]
"#,
            env!("CARGO_PKG_VERSION"),
        ),
    )
    .unwrap();
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
    for (name, source, function, parameterized) in [
        (
            "asserted",
            include_str!("../../core/tests/fixtures/prepared/boolean_bridge.gleam"),
            "verify",
            false,
        ),
        (
            "case",
            include_str!("../fixtures/prepared_call_lints/case.gleam"),
            "verify",
            false,
        ),
        (
            "boolean",
            include_str!("../../core/tests/fixtures/prepared/boolean_calls.gleam"),
            "flip",
            true,
        ),
        (
            "recovery",
            include_str!("../fixtures/prepared_call_lints/recovery.gleam"),
            "verify",
            true,
        ),
    ] {
        let mut artifacts = Vec::new();
        for _ in 0..2 {
            let typed = geam::compile_typed_module("example", "src/example.gleam", source).unwrap();
            let builder = ModuleBuilder::new(typed).unwrap();
            let bindings = if parameterized {
                builder
                    .function(FunctionDeclaration::<(bool,), bool>::new(function))
                    .unwrap()
                    .0
            } else {
                builder
                    .function(FunctionDeclaration::<(), bool>::new(function))
                    .unwrap()
                    .0
            };
            artifacts.push(bindings.prepare().emit_rust());
        }
        assert_eq!(artifacts[0], artifacts[1], "{name}");
        fs::write(
            application.join(format!("src/{name}.rs")),
            format!("{}\n", artifacts[0]),
        )
        .unwrap();
    }
    let source = include_str!("../fixtures/prepared_call_lints/recovery.gleam");
    let typed = geam::compile_typed_module("example", "src/example.gleam", source).unwrap();
    let (bindings, verify) = ModuleBuilder::new(typed)
        .unwrap()
        .function(FunctionDeclaration::<(bool,), bool>::new("verify"))
        .unwrap();
    let live = bindings.seal();
    let mut echo = Vec::new();
    for _ in 0..2 {
        let error = live.call(&verify, (false,), &mut echo).unwrap_err();
        assert!(
            matches!(error, CallError::Execution(ExecutionError::Panic(ref panic)) if panic.kind() == PanicKind::LetAssert)
        );
        assert!(live.call(&verify, (true,), &mut echo).unwrap());
    }
    assert!(echo.is_empty());
    fs::write(
        application.join("src/main.rs"),
        r#"
use geam::__prepared_support as data;
use geam::embedding::{CallError, FunctionDeclaration};
use geam::{ExecutionError, PanicKind};
use std::convert::Infallible;

static ASSERTED: data::ModuleArtifact<Infallible> = include!("asserted.rs");
static CASE: data::ModuleArtifact<Infallible> = include!("case.rs");
static BOOLEAN: data::ModuleArtifact<Infallible> = include!("boolean.rs");
static RECOVERY: data::ModuleArtifact<Infallible> = include!("recovery.rs");

fn main() {
    let mut echo = Vec::new();
    for artifact in [&ASSERTED, &CASE] {
        let mut bindings = artifact.load().unwrap();
        let verify = bindings.function(FunctionDeclaration::<(), bool>::new("verify")).unwrap();
        let module = bindings.seal();
        for _ in 0..3 {
            assert!(module.call(&verify, (), &mut echo).unwrap());
        }
    }
    let mut bindings = BOOLEAN.load().unwrap();
    let flip = bindings.function(FunctionDeclaration::<(bool,), bool>::new("flip")).unwrap();
    let module = bindings.seal();
    for (input, expected) in [(false, true), (true, false), (false, true)] {
        assert_eq!(module.call(&flip, (input,), &mut echo).unwrap(), expected);
    }
    let mut bindings = RECOVERY.load().unwrap();
    let verify = bindings.function(FunctionDeclaration::<(bool,), bool>::new("verify")).unwrap();
    let module = bindings.seal();
    for _ in 0..2 {
        let error = module.call(&verify, (false,), &mut echo).unwrap_err();
        assert!(matches!(error, CallError::Execution(ExecutionError::Panic(ref panic)) if panic.kind() == PanicKind::LetAssert));
        assert!(module.call(&verify, (true,), &mut echo).unwrap());
    }
    assert!(echo.is_empty());
    println!("terminal bridges, looping calls, and recovery completed");
}
"#,
    )
    .unwrap();
    checked(command("cargo", &application).args(["generate-lockfile", "--offline"]));
    let lock = fs::read(application.join("Cargo.lock")).unwrap();
    checked(command("cargo", &application).args(["fmt", "--all"]));
    checked(command("cargo", &application).args(["fmt", "--all", "--", "--check"]));
    checked(command("cargo", &application).args([
        "clippy",
        "--all-targets",
        "--locked",
        "--",
        "-D",
        "warnings",
    ]));
    checked(command("cargo", &application).args(["build", "--quiet", "--locked"]));
    assert_eq!(fs::read(application.join("Cargo.lock")).unwrap(), lock);
    let deployed = root.join("deployed");
    fs::create_dir_all(&deployed).unwrap();
    let executable = binary_path(&deployed, "prepared-call-lints");
    fs::copy(
        binary_path(&target.join("debug"), "prepared-call-lints"),
        &executable,
    )
    .unwrap();
    fs::remove_dir_all(application).unwrap();
    let output = checked(command(&executable, &deployed).env("PATH", ""));
    assert_eq!(
        output.stdout,
        b"terminal bridges, looping calls, and recovery completed\n"
    );
    assert!(output.stderr.is_empty());
}
