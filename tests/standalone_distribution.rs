use std::fs;
use std::path::Path;
use std::process::{Command, Output};
use std::sync::OnceLock;
use tempfile::{TempDir, tempdir};

#[path = "support/workspace_dependencies.rs"]
mod workspace_dependencies;

#[test]
fn runs_the_canonical_standalone_project_with_independent_path_providers() {
    let fixture = standalone_fixture();
    let project = fixture.path().join("project");
    assert!(!fixture.path().join("providers/target").exists());
    assert!(project.join("build/packages/gleam_stdlib/src").is_dir());
    assert!(!project.join("build/geam").exists());

    for package in ["geam-catalog", "geam-counter"] {
        let add = geam_at(
            &project,
            [
                "provider",
                "add",
                "--path",
                "../providers",
                "--package",
                package,
            ],
        );
        assert!(
            add.status.success(),
            "provider add failed for {package}: {}",
            String::from_utf8_lossy(&add.stderr),
        );
    }

    let prepare = geam_at(&project, ["prepare"]);
    assert!(
        prepare.status.success(),
        "canonical prepare failed: {}",
        String::from_utf8_lossy(&prepare.stderr),
    );
    let manifest = fs::read_to_string(project.join("Cargo.toml"))
        .expect("managed manifest should be readable");
    let runner = fs::read_to_string(project.join("build/geam/runner.rs"))
        .expect("generated runner should be readable");
    let catalog = manifest
        .find("geam_provider_catalog")
        .expect("catalog dependency should be generated");
    let counter = manifest
        .find("geam_provider_counter")
        .expect("counter dependency should be generated");
    assert!(catalog < counter);
    let components = [
        "geam::gleam_stdlib::Component<CliIoSink>",
        "geam::gleam_json::Component",
        "geam::gleam_time::Component",
        "geam_provider_catalog::Component",
        "geam_provider_counter::Component",
    ];
    let mut previous_projection = 0;
    let mut previous_registration = 0;
    for component in components {
        let projection = runner
            .find(&format!(
                "impl geam::HostComponentProfile<{component}> for Profile"
            ))
            .expect("component projection should be generated");
        let registration = runner
            .find(&format!(
                "<{component} as geam::HostProviderComponentRegistration<Profile>>::providers()?"
            ))
            .expect("component registration should be generated");
        assert!(projection > previous_projection);
        assert!(registration > previous_registration);
        previous_projection = projection;
        previous_registration = registration;
    }

    let root_source = project.join("src/standalone_fixture.gleam");
    let mut source = fs::read_to_string(&root_source).expect("root source should be readable");
    source.push_str("\n// Source-only edits do not change the static Rust profile.\n");
    fs::write(&root_source, source).expect("root source should be updated");
    let repeated = geam_at(&project, ["prepare", "--module", "alternate"]);
    assert!(
        repeated.status.success(),
        "repeated prepare failed: {}",
        String::from_utf8_lossy(&repeated.stderr),
    );
    assert_eq!(
        fs::read_to_string(project.join("Cargo.toml"))
            .expect("managed manifest should remain readable"),
        manifest,
    );
    assert_eq!(
        fs::read_to_string(project.join("build/geam/runner.rs"))
            .expect("generated runner should remain readable"),
        runner,
    );

    for _ in 0..2 {
        let run = geam_at(
            &project,
            [
                "run",
                "--provider-config",
                "catalog=config/catalog.toml",
                "--provider-config",
                "counter=config/counter.toml",
            ],
        );
        assert!(
            run.status.success(),
            "canonical run failed: {}",
            String::from_utf8_lossy(&run.stderr),
        );
        assert_eq!(
            String::from_utf8_lossy(&run.stdout),
            "\"count:3/count:4\"\n"
        );
        let stderr = String::from_utf8_lossy(&run.stderr);
        let before = stderr
            .find("provider-before\n")
            .expect("first IO event should be present");
        let echo = stderr
            .find("provider-summary\nSummary(count: 1, items: [\"pure:native:alpha\"])")
            .expect("provider-backed Echo should be present");
        let after = stderr
            .find("provider-after\n")
            .expect("final IO event should be present");
        assert!(before < echo && echo < after);
    }
    assert_eq!(
        fs::read_to_string(project.join("build/geam/runner.rs"))
            .expect("runtime configuration must not rewrite the generated runner"),
        runner,
    );

    let alternate = geam_at(
        &project,
        [
            "run",
            "--module",
            "alternate",
            "--provider-config",
            "catalog=config/catalog.toml",
            "--provider-config",
            "counter=config/counter.toml",
        ],
    );
    assert!(
        alternate.status.success(),
        "alternate run failed: {}",
        String::from_utf8_lossy(&alternate.stderr),
    );
    assert_eq!(String::from_utf8_lossy(&alternate.stdout), "alternate\n");
    assert!(alternate.stderr.is_empty());

    let missing_configuration = geam_at(&project, ["run"]);
    assert!(!missing_configuration.status.success());
    assert!(String::from_utf8_lossy(&missing_configuration.stderr).contains(
        "could not initialize host provider component geam-catalog: configuration key `prefix` must be a String",
    ));
}

fn geam_at<const N: usize>(directory: &Path, arguments: [&str; N]) -> Output {
    geam_command_at(directory, arguments)
        .output()
        .expect("Geam CLI should start")
}

fn geam_command_at<const N: usize>(directory: &Path, arguments: [&str; N]) -> Command {
    let mut command = Command::new(env!("CARGO_BIN_EXE_geam"));
    command.args(arguments).current_dir(directory);
    remove_nested_cargo_instrumentation(&mut command);
    command
}

fn remove_nested_cargo_instrumentation(command: &mut Command) {
    // Keep the CLI process profiled while preventing its nested Cargo build
    // from compiling a second instrumented copy of Geam into the report.
    command
        .env_remove("CARGO_ENCODED_RUSTFLAGS")
        .env_remove("CARGO_LLVM_COV")
        .env_remove("CARGO_LLVM_COV_TARGET_DIR")
        .env_remove("RUSTFLAGS")
        .env_remove("RUSTDOCFLAGS");
}

fn standalone_fixture() -> TempDir {
    prepare_provider_dependencies();
    let fixture = tempdir().expect("temporary standalone fixture should be created");
    let source = Path::new(env!("CARGO_MANIFEST_DIR")).join("cli/tests/fixtures/standalone_cli");
    static GLEAM_DEPENDENCIES: OnceLock<Result<(), String>> = OnceLock::new();
    workspace_dependencies::prepare(
        &GLEAM_DEPENDENCIES,
        &source.join("project"),
        "gleam",
        &["deps", "download"],
        "`gleam deps download`",
    );
    copy_directory(&source, fixture.path());
    copy_directory(
        &source.join("project/build/packages"),
        &fixture.path().join("project/build/packages"),
    );
    for generated in ["Cargo.toml", "Cargo.lock", "Cargo.toml.geam.tmp"] {
        let path = fixture.path().join("project").join(generated);
        match fs::remove_file(&path) {
            Ok(()) => {}
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
            Err(error) => panic!(
                "generated fixture file {} should be removable: {error}",
                path.display()
            ),
        }
    }

    let geam_path = toml::Value::String(env!("CARGO_MANIFEST_DIR").to_owned()).to_string();
    fs::write(
        fixture.path().join("project/.cargo/config.toml"),
        format!(
            "[patch.crates-io]\ngeam = {{ path = {geam_path} }}\ngeam-catalog = {{ path = \"../providers/geam-catalog\" }}\ngeam-counter = {{ path = \"../providers/geam-counter\" }}\nstandalone-catalog-domain = {{ path = \"../providers/catalog-domain\" }}\n\n[net]\noffline = true\n",
        ),
    )
    .expect("project Cargo config should be written");
    fs::write(
        fixture.path().join("providers/.cargo/config.toml"),
        format!("[patch.crates-io]\ngeam = {{ path = {geam_path} }}\n\n[net]\noffline = true\n"),
    )
    .expect("provider Cargo config should be written");
    fixture
}

fn prepare_provider_dependencies() {
    static PREPARED: OnceLock<Result<(), String>> = OnceLock::new();
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    workspace_dependencies::prepare(
        &PREPARED,
        &root.join("cli/tests/fixtures/standalone_cli/providers"),
        "cargo",
        &["fetch", "--locked", "--config", "net.offline=false"],
        "`cargo fetch --locked --config net.offline=false`",
    );
}

fn copy_directory(source: &Path, destination: &Path) {
    fs::create_dir_all(destination).expect("fixture directory should be created");
    for entry in fs::read_dir(source).expect("fixture directory should be readable") {
        let entry = entry.expect("fixture entry should be readable");
        let source = entry.path();
        let destination = destination.join(entry.file_name());
        if source.is_dir() {
            if matches!(entry.file_name().to_str(), Some("build") | Some("target")) {
                continue;
            }
            copy_directory(&source, &destination);
        } else {
            fs::copy(&source, &destination).expect("fixture file should be copied");
        }
    }
}
