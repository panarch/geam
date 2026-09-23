use super::{binary_path, checked, command};
use std::fs;
use std::path::Path;

#[test]
fn process_service_runs_dynamic_and_relocated_prepared() {
    verify_consumer(
        "examples/provider/process_service",
        "process-service-embedding",
        b"named service replied: 42, 17\nrequest timeout and unavailable name handled\nworker stopped and name released\n",
    );
}

#[test]
fn original_otp_runs_dynamic_and_relocated_prepared() {
    verify_consumer(
        "tests/fixtures/otp_service",
        "otp-service-embedding",
        b"original actor: state, suspend, resume\noriginal static child: retained callback in supervisor\noriginal factory: two typed callbacks, named and pid handles\n",
    );
}

fn verify_consumer(fixture: &str, executable: &str, expected: &[u8]) {
    let directory = tempfile::tempdir().unwrap();
    let root = fs::canonicalize(directory.path()).unwrap();
    let repository = Path::new(env!("CARGO_MANIFEST_DIR"));
    let source = repository.join(fixture);
    let fixture_root = root.join(fixture);
    for part in ["project", "provider", "embedding"] {
        copy_source(&source.join(part), &fixture_root.join(part));
    }
    let support = root.join("tests/support");
    fs::create_dir_all(&support).unwrap();
    fs::copy(
        repository.join("tests/support/execution_host.rs"),
        support.join("execution_host.rs"),
    )
    .unwrap();
    let application = fixture_root.join("embedding");
    let target = repository.join("target/prepared-acceptance");
    let mut manifest: toml::Value =
        toml::from_str(&fs::read_to_string(application.join("Cargo.toml")).unwrap()).unwrap();
    manifest["patch"]["crates-io"]["geam"]["path"] = repository.to_str().unwrap().into();
    fs::write(
        application.join("Cargo.toml"),
        toml::to_string(&manifest).unwrap(),
    )
    .unwrap();
    fs::create_dir_all(application.join(".cargo")).unwrap();
    let target_path = target.to_str().unwrap();
    fs::write(
        application.join(".cargo/config.toml"),
        toml::to_string(&toml::toml! {
            [net]
            offline = true
            [build]
            target-dir = target_path
        })
        .unwrap(),
    )
    .unwrap();
    checked(command(env!("CARGO_BIN_EXE_geam"), &application).args(["embedding", "sync"]));
    let generated = fs::read(application.join("src/geam_bindings.rs")).unwrap();
    let prepared = fs::read(application.join("src/geam_bindings/program.rs")).unwrap();
    let lock = fs::read(application.join("Cargo.lock")).unwrap();
    checked(command(env!("CARGO_BIN_EXE_geam"), &application).args(["embedding", "check"]));
    checked(command(env!("CARGO_BIN_EXE_geam"), &application).args(["embedding", "sync"]));
    assert_eq!(
        fs::read(application.join("src/geam_bindings.rs")).unwrap(),
        generated
    );
    assert_eq!(
        fs::read(application.join("src/geam_bindings/program.rs")).unwrap(),
        prepared
    );
    assert_eq!(fs::read(application.join("Cargo.lock")).unwrap(), lock);
    checked(command("cargo", &application).args(["fmt", "--all", "--check"]));
    checked(command("cargo", &application).args([
        "clippy",
        "--all-targets",
        "--locked",
        "--",
        "-D",
        "warnings",
    ]));
    let output = checked(command("cargo", &application).args(["run", "--quiet", "--locked"]));
    assert_eq!(output.stdout, expected.repeat(2));
    assert!(output.stderr.is_empty());

    let deploy = root.join("deployment");
    fs::create_dir(&deploy).unwrap();
    let binary = binary_path(&deploy, "service-consumer");
    fs::copy(binary_path(&target.join("debug"), executable), &binary).unwrap();
    let project = fixture_root.join("project");
    fs::create_dir_all(project.join(".cargo")).unwrap();
    let repository_path = repository.to_str().unwrap();
    fs::write(
        project.join(".cargo/config.toml"),
        toml::to_string(&toml::toml! {
            [patch.crates-io.geam]
            path = repository_path
            [net]
            offline = true
        })
        .unwrap(),
    )
    .unwrap();
    checked(command(env!("CARGO_BIN_EXE_geam"), &project).args([
        "provider",
        "add",
        "--path",
        "../provider",
    ]));
    let build = checked(command(env!("CARGO_BIN_EXE_geam"), &project).arg("build"));
    assert!(build.stdout.is_empty());
    let report = std::str::from_utf8(&build.stderr).unwrap();
    let built = report
        .lines()
        .last()
        .unwrap()
        .strip_prefix("geam: Built ")
        .unwrap();
    let standalone = binary_path(&deploy, "standalone-consumer");
    fs::copy(built, &standalone).unwrap();
    let program = fs::read(project.join("build/geam/program.rs")).unwrap();
    checked(command(env!("CARGO_BIN_EXE_geam"), &project).arg("build"));
    assert_eq!(
        fs::read(project.join("build/geam/program.rs")).unwrap(),
        program
    );
    fs::remove_dir_all(&fixture_root).unwrap();
    fs::remove_dir_all(root.join("tests")).unwrap();
    if root.join("examples").exists() {
        fs::remove_dir_all(root.join("examples")).unwrap();
    }
    for _ in 0..2 {
        let output = checked(command(&binary, &deploy).arg("--prepared").env("PATH", ""));
        assert_eq!(output.stdout, expected);
        assert!(output.stderr.is_empty());
        let output = checked(command(&standalone, &deploy).env("PATH", ""));
        assert_eq!(output.stdout, expected);
        assert!(output.stderr.is_empty());
    }
}

fn copy_source(source: &Path, destination: &Path) {
    fs::create_dir_all(destination).unwrap();
    for entry in fs::read_dir(source).unwrap() {
        let entry = entry.unwrap();
        if matches!(
            entry.file_name().to_str(),
            Some("target" | "build" | ".cargo")
        ) {
            continue;
        }
        let destination = destination.join(entry.file_name());
        if entry.file_type().unwrap().is_dir() {
            copy_source(&entry.path(), &destination);
        } else {
            fs::copy(entry.path(), destination).unwrap();
        }
    }
}
