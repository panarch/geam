use super::{binary_path, checked, command};
use geam::__prepared_support::compiled::CompiledCheckpoint;
use geam::embedding::{
    BigInt, CallError, FunctionDeclaration, HostedModuleBuilder, ModuleBuilder, StringValue,
};
use geam::{
    ExecutionError, HostProviderModule, HostProviderSet, ModuleSource, PackageSource, PanicKind,
    StatelessHostProfile, compile_typed_host_program,
};
use std::fs;
use std::mem::size_of;
use std::path::Path;

#[test]
fn large_generated_numeric_and_string_call_tables_are_lint_clean_and_relocatable() {
    // Both callees need a real checkpoint table over Clippy's default 16 KiB
    // limit. A long numeric-only entry would not exercise the call emitters.
    let count = (16_384 / size_of::<CompiledCheckpoint>() + 1).next_multiple_of(2);
    let inversions = "  let accepted = !accepted\n".repeat(count);
    let source = format!(
        r#"
fn numeric(value: Int, accepted: Bool) -> Bool {{
{inversions}  let next = value + 1
  case next {{ 0 -> accepted _ -> False }}
}}

fn prefix(value: String, accepted: Bool) -> Bool {{
{inversions}  case value {{ "pre" <> _ -> accepted _ -> False }}
}}

pub fn verify(value: Int, text: String, accepted: Bool) -> Bool {{
  let calculate = numeric
  let check = prefix
  calculate(value, accepted) == check(text, accepted)
}}
"#
    );
    let mut artifacts = Vec::new();
    for _ in 0..2 {
        let typed = geam::compile_typed_module("example", "src/example.gleam", &source).unwrap();
        let (bindings, _) = ModuleBuilder::new(typed)
            .unwrap()
            .function(FunctionDeclaration::<(BigInt, StringValue, bool), bool>::new("verify"))
            .unwrap();
        artifacts.push(bindings.prepare().emit_rust());
    }
    assert_eq!(artifacts[0], artifacts[1]);
    let large_tables = artifacts[0]
        .lines()
        .filter_map(|line| line.split_once("POINTS: [data::compiled::CompiledCheckpoint; "))
        .map(|(_, declaration)| {
            declaration
                .split_once(']')
                .unwrap()
                .0
                .parse::<usize>()
                .unwrap()
        })
        .filter(|count| count * size_of::<CompiledCheckpoint>() > 16_384)
        .collect::<Vec<_>>();
    assert_eq!(large_tables.len(), 2, "checkpoint counts: {large_tables:?}");

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
name = "large-checkpoint-consumer"
version = "0.1.0"
edition = "2024"

[dependencies]
geam = {{ version = "={}", default-features = false, features = ["embedding"] }}

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
    fs::write(application.join("src/program.rs"), &artifacts[0]).unwrap();
    fs::write(application.join("src/source.gleam"), source).unwrap();
    fs::write(
        application.join("src/main.rs"),
        r#"
use geam::__prepared_support as data;
use geam::embedding::{BigInt, FunctionDeclaration, ModuleBuilder, StringValue};
use std::convert::Infallible;

static PROGRAM: data::ModuleArtifact<Infallible> = include!("program.rs");

fn main() {
    let typed = geam::compile_typed_module("example", "src/example.gleam", include_str!("source.gleam")).unwrap();
    let (bindings, dynamic_function) = ModuleBuilder::new(typed).unwrap()
        .function(FunctionDeclaration::<(BigInt, StringValue, bool), bool>::new("verify")).unwrap();
    let dynamic = bindings.seal();
    let mut bindings = PROGRAM.load().unwrap();
    let prepared_function = bindings.function(FunctionDeclaration::<(BigInt, StringValue, bool), bool>::new("verify")).unwrap();
    let prepared = bindings.seal();
    for (module, function) in [(dynamic, dynamic_function), (prepared, prepared_function)] {
        for _ in 0..2 {
            for (value, text, accepted, expected) in [
                (-1, "prefix", true, true),
                (-1, "other", true, false),
                (0, "prefix", true, false),
                (0, "other", true, true),
                (-1, "prefix", false, true),
                (i128::from(i64::MAX), "prefix", true, false),
                (i128::MAX, "prefix", true, false),
            ] {
                let mut echo = Vec::new();
                assert_eq!(module.call(&function, (BigInt::from(value), StringValue::from(text), accepted), &mut echo).unwrap(), expected);
                assert!(echo.is_empty());
            }
        }
    }
    println!("large checkpoint tables: dynamic and prepared ok");
}
"#,
    )
    .unwrap();
    checked(command("cargo", &application).args(["generate-lockfile", "--offline"]));
    let lock = fs::read(application.join("Cargo.lock")).unwrap();
    checked(command("cargo", &application).args(["fmt", "--all"]));
    assert_eq!(
        fs::read(application.join("src/program.rs")).unwrap(),
        artifacts[0].as_bytes()
    );
    checked(command("cargo", &application).args([
        "clippy",
        "--all-targets",
        "--locked",
        "--offline",
        "--",
        "-D",
        "warnings",
    ]));
    checked(command("cargo", &application).args(["build", "--locked", "--offline"]));
    assert_eq!(fs::read(application.join("Cargo.lock")).unwrap(), lock);
    let expected = b"large checkpoint tables: dynamic and prepared ok\n";
    let executable = binary_path(&target.join("debug"), "large-checkpoint-consumer");
    let output = checked(&mut command(&executable, &application));
    assert_eq!(output.stdout, expected);
    assert!(output.stderr.is_empty());
    let deployed = root.join("deployed");
    fs::create_dir_all(&deployed).unwrap();
    let relocated = binary_path(&deployed, "large checkpoint consumer");
    fs::copy(executable, &relocated).unwrap();
    fs::remove_dir_all(application).unwrap();
    let output = checked(command(relocated, &deployed).env("PATH", ""));
    assert_eq!(output.stdout, expected);
    assert!(output.stderr.is_empty());
}

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
geam = {{ version = '={}', default-features = false, features = ['embedding', 'tokio'] }}
tokio = {{ version = '=1.53.1', features = ['rt'] }}

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
    let source = include_str!("../fixtures/prepared_call_lints/native.gleam");
    let mut artifacts = Vec::new();
    for _ in 0..2 {
        let native = HostProviderModule::<StatelessHostProfile>::new("example", "example")
            .unwrap()
            .with_function::<(bool,), bool, _>("accepted", |value| value)
            .unwrap();
        let typed = compile_typed_host_program(
            "example",
            "example",
            [PackageSource::new(
                "example",
                Vec::<String>::new(),
                [ModuleSource::new("example", "src/example.gleam", source)],
            )],
            HostProviderSet::from_providers([native]).unwrap(),
        )
        .unwrap();
        let (bindings, _) = HostedModuleBuilder::new(typed)
            .unwrap()
            .function(FunctionDeclaration::<(bool,), bool>::new("verify"))
            .unwrap();
        artifacts.push(bindings.prepare().unwrap().emit_rust());
    }
    assert_eq!(artifacts[0], artifacts[1]);
    fs::write(
        application.join("src/native.rs"),
        format!("{}\n", artifacts[0]),
    )
    .unwrap();
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
use geam::{HostProviderModule, HostProviderSet, StatelessHostProfile};
use geam::execution::TokioHost;
use std::convert::Infallible;
use std::sync::atomic::{AtomicUsize, Ordering};

static ASSERTED: data::ModuleArtifact<Infallible> = include!("asserted.rs");
static CASE: data::ModuleArtifact<Infallible> = include!("case.rs");
static BOOLEAN: data::ModuleArtifact<Infallible> = include!("boolean.rs");
static RECOVERY: data::ModuleArtifact<Infallible> = include!("recovery.rs");
static NATIVE: data::HostedModuleArtifact = include!("native.rs");
static ACCEPTED: AtomicUsize = AtomicUsize::new(0);

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
    let provider = HostProviderModule::<StatelessHostProfile>::new("example", "example").unwrap()
        .with_function::<(bool,), bool, _>("accepted", |value| {
            ACCEPTED.fetch_add(1, Ordering::SeqCst);
            value
        }).unwrap();
    let mut bindings = NATIVE.load(HostProviderSet::from_providers([provider]).unwrap()).unwrap();
    let verify = bindings.function(FunctionDeclaration::<(bool,), bool>::new("verify")).unwrap();
    let mut module = bindings.seal();
    let runtime = tokio::runtime::Builder::new_current_thread().build().unwrap();
    let host = TokioHost::new(runtime.handle().clone());
    for _ in 0..2 {
        let error = runtime.block_on(module.with_execution(&host, &mut (), &mut echo, async |scope| {
            scope.call(&verify, (false,)).await
        })).unwrap().try_into_value().unwrap().unwrap_err();
        assert!(matches!(error, CallError::Execution(ExecutionError::Panic(ref panic)) if panic.kind() == PanicKind::LetAssert));
        let value = runtime.block_on(module.with_execution(&host, &mut (), &mut echo, async |scope| {
            scope.call(&verify, (true,)).await
        })).unwrap().try_into_value().unwrap().unwrap();
        assert!(value);
    }
    assert_eq!(ACCEPTED.load(Ordering::SeqCst), 6);
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
