use std::fs;
use std::path::Path;
use std::process::{Command, Output};
use std::sync::OnceLock;
use tempfile::{TempDir, tempdir};

#[path = "support/workspace_dependencies.rs"]
mod workspace_dependencies;

#[test]
fn runs_the_documented_run_metrics_provider_without_configuration() {
    let fixture = provider_example("run_metrics");
    let project = fixture.path().join("project");

    let add = geam_at(&project, ["provider", "add", "--path", "../provider"]);
    assert!(
        add.status.success(),
        "run metrics provider add failed: {}",
        String::from_utf8_lossy(&add.stderr),
    );

    let prepare = geam_at(&project, ["prepare"]);
    assert!(
        prepare.status.success(),
        "run metrics prepare failed: {}",
        String::from_utf8_lossy(&prepare.stderr),
    );

    let run = geam_at(&project, ["run"]);
    assert!(
        run.status.success(),
        "run metrics execution failed: {}",
        String::from_utf8_lossy(&run.stderr),
    );
    assert!(run.stdout.is_empty());
    assert!(run.stderr.is_empty());

    let independent_run = geam_at(&project, ["run"]);
    assert!(
        independent_run.status.success(),
        "independent run metrics execution failed: {}",
        String::from_utf8_lossy(&independent_run.stderr),
    );
    assert!(independent_run.stdout.is_empty());
    assert!(independent_run.stderr.is_empty());
}

#[test]
fn runs_the_documented_text_tools_provider_across_three_modules() {
    let fixture = provider_example("text_tools");
    let project = fixture.path().join("project");

    let add = geam_at(&project, ["provider", "add", "--path", "../provider"]);
    assert!(
        add.status.success(),
        "text tools provider add failed: {}",
        String::from_utf8_lossy(&add.stderr),
    );

    let prepare = geam_at(&project, ["prepare"]);
    assert!(
        prepare.status.success(),
        "text tools prepare failed: {}",
        String::from_utf8_lossy(&prepare.stderr),
    );

    for _ in 0..2 {
        let run = geam_at(&project, ["run"]);
        assert!(
            run.status.success(),
            "text tools execution failed: {}",
            String::from_utf8_lossy(&run.stderr),
        );
        assert!(run.stdout.is_empty());
        assert!(run.stderr.is_empty());
    }
}

#[test]
fn runs_the_documented_value_types_provider_without_configuration() {
    let fixture = provider_example("value_types");
    let project = fixture.path().join("project");

    let add = geam_at(&project, ["provider", "add", "--path", "../provider"]);
    assert!(
        add.status.success(),
        "value types provider add failed: {}",
        String::from_utf8_lossy(&add.stderr),
    );

    let prepare = geam_at(&project, ["prepare"]);
    assert!(
        prepare.status.success(),
        "value types prepare failed: {}",
        String::from_utf8_lossy(&prepare.stderr),
    );

    for _ in 0..2 {
        let run = geam_at(&project, ["run"]);
        assert!(
            run.status.success(),
            "value types execution failed: {}",
            String::from_utf8_lossy(&run.stderr),
        );
        assert!(run.stdout.is_empty());
        assert!(run.stderr.is_empty());
    }
}

#[test]
fn runs_the_documented_tag_set_provider_without_configuration() {
    let fixture = provider_example("tag_set");
    let project = fixture.path().join("project");

    let add = geam_at(&project, ["provider", "add", "--path", "../provider"]);
    assert!(
        add.status.success(),
        "tag set provider add failed: {}",
        String::from_utf8_lossy(&add.stderr),
    );

    let prepare = geam_at(&project, ["prepare"]);
    assert!(
        prepare.status.success(),
        "tag set prepare failed: {}",
        String::from_utf8_lossy(&prepare.stderr),
    );

    for _ in 0..2 {
        let run = geam_at(&project, ["run"]);
        assert!(
            run.status.success(),
            "tag set execution failed: {}",
            String::from_utf8_lossy(&run.stderr),
        );
        assert!(run.stdout.is_empty());
        assert!(run.stderr.is_empty());
    }
}

#[test]
fn runs_the_documented_request_ids_provider_with_fresh_default_state() {
    let fixture = provider_example("request_ids");
    let project = fixture.path().join("project");

    let add = geam_at(&project, ["provider", "add", "--path", "../provider"]);
    assert!(
        add.status.success(),
        "request IDs provider add failed: {}",
        String::from_utf8_lossy(&add.stderr),
    );

    let prepare = geam_at(&project, ["prepare"]);
    assert!(
        prepare.status.success(),
        "request IDs prepare failed: {}",
        String::from_utf8_lossy(&prepare.stderr),
    );

    for _ in 0..2 {
        let run = geam_at(&project, ["run"]);
        assert!(
            run.status.success(),
            "request IDs execution failed: {}",
            String::from_utf8_lossy(&run.stderr),
        );
        assert!(run.stdout.is_empty());
        assert!(run.stderr.is_empty());
    }
}

#[test]
fn runs_the_documented_call_tracing_provider_with_fresh_callback_state() {
    let fixture = provider_example("call_tracing");
    let project = fixture.path().join("project");

    let add = geam_at(&project, ["provider", "add", "--path", "../provider"]);
    assert!(
        add.status.success(),
        "call tracing provider add failed: {}",
        String::from_utf8_lossy(&add.stderr),
    );

    let prepare = geam_at(&project, ["prepare"]);
    assert!(
        prepare.status.success(),
        "call tracing prepare failed: {}",
        String::from_utf8_lossy(&prepare.stderr),
    );

    for _ in 0..2 {
        let run = geam_at(&project, ["run"]);
        assert!(
            run.status.success(),
            "call tracing execution failed: {}",
            String::from_utf8_lossy(&run.stderr),
        );
        assert!(run.stdout.is_empty());
        assert!(run.stderr.is_empty());
    }
}

#[test]
fn runs_the_documented_generic_box_provider_with_persistent_values() {
    let fixture = provider_example("generic_box");
    let project = fixture.path().join("project");

    let add = geam_at(&project, ["provider", "add", "--path", "../provider"]);
    assert!(
        add.status.success(),
        "generic box provider add failed: {}",
        String::from_utf8_lossy(&add.stderr),
    );

    let prepare = geam_at(&project, ["prepare"]);
    assert!(
        prepare.status.success(),
        "generic box prepare failed: {}",
        String::from_utf8_lossy(&prepare.stderr),
    );

    for _ in 0..2 {
        let run = geam_at(&project, ["run"]);
        assert!(
            run.status.success(),
            "generic box execution failed: {}",
            String::from_utf8_lossy(&run.stderr),
        );
        assert!(run.stdout.is_empty());
        assert!(run.stderr.is_empty());
    }
}

#[test]
fn runs_the_documented_feature_flags_provider_with_explicit_configuration() {
    let fixture = provider_example("feature_flags");
    let project = fixture.path().join("project");

    let add = geam_at(&project, ["provider", "add", "--path", "../provider"]);
    assert!(
        add.status.success(),
        "feature flags provider add failed: {}",
        String::from_utf8_lossy(&add.stderr),
    );

    let prepare = geam_at(&project, ["prepare"]);
    assert!(
        prepare.status.success(),
        "feature flags prepare failed: {}",
        String::from_utf8_lossy(&prepare.stderr),
    );

    for _ in 0..2 {
        let run = geam_at(
            &project,
            [
                "run",
                "--provider-config",
                "example_feature_flags=config/feature_flags.toml",
            ],
        );
        assert!(
            run.status.success(),
            "feature flags execution failed: {}",
            String::from_utf8_lossy(&run.stderr),
        );
        assert!(run.stdout.is_empty());
        assert!(run.stderr.is_empty());
    }

    let missing = geam_at(&project, ["run"]);
    assert!(!missing.status.success());
    assert!(String::from_utf8_lossy(&missing.stderr).contains(
        "could not initialize host provider component geam-example-feature-flags: configuration key `environment` must be a String",
    ));

    let wrong = geam_at(
        &project,
        [
            "run",
            "--provider-config",
            "example_feature_flags=config/wrong_enabled.toml",
        ],
    );
    assert!(!wrong.status.success());
    assert!(String::from_utf8_lossy(&wrong.stderr).contains(
        "could not initialize host provider component geam-example-feature-flags: configuration key `enabled` must be an Array of Strings",
    ));
}

#[test]
fn runs_the_documented_text_pattern_provider_from_its_local_path() {
    let fixture = provider_example("text_pattern");
    let project = fixture.path().join("project");

    let add = geam_at(&project, ["provider", "add", "--path", "../provider"]);
    assert!(
        add.status.success(),
        "text pattern provider add failed: {}",
        String::from_utf8_lossy(&add.stderr),
    );

    let prepare = geam_at(&project, ["prepare"]);
    assert!(
        prepare.status.success(),
        "text pattern prepare failed: {}",
        String::from_utf8_lossy(&prepare.stderr),
    );
    let manifest = fs::read_to_string(project.join("Cargo.toml"))
        .expect("managed text pattern manifest should be readable");
    let runner = fs::read_to_string(project.join("build/geam/runner.rs"))
        .expect("text pattern runner should be readable");
    let manifest = toml::from_str::<toml::Value>(&manifest)
        .expect("managed text pattern manifest should be valid TOML");
    let dependency = manifest["dependencies"]["geam_provider_example_text_pattern"]
        .as_table()
        .expect("text pattern dependency should be a table");
    assert_eq!(
        dependency["package"].as_str(),
        Some("geam-example-text-pattern"),
    );
    assert_eq!(
        dependency["path"].as_str(),
        Some(
            fs::canonicalize(fixture.path().join("provider"))
                .expect("text pattern provider should canonicalize")
                .to_str()
                .expect("text pattern provider path should be valid UTF-8"),
        ),
    );
    assert!(runner.contains("geam_provider_example_text_pattern::Component"));

    let run = geam_at(&project, ["run"]);
    assert!(
        run.status.success(),
        "text pattern run failed: {}",
        String::from_utf8_lossy(&run.stderr),
    );
    assert!(run.stdout.is_empty());
    let canonical_project =
        fs::canonicalize(&project).expect("text pattern project should canonicalize");
    assert_eq!(
        String::from_utf8_lossy(&run.stderr),
        format!(
            "{}/src/text_pattern_example.gleam:8 compiled pattern\nPattern(\"[A-Za-z]+\")\n",
            canonical_project.display(),
        ),
    );
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

fn provider_example(name: &str) -> TempDir {
    prepare_provider_dependency(name);
    let source = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("examples")
        .join(name);

    let fixture = tempdir().expect("temporary provider example fixture should be created");
    copy_directory(&source, fixture.path());
    let project = fixture.path().join("project");
    for generated in ["Cargo.toml", "Cargo.lock", "Cargo.toml.geam.tmp"] {
        let path = project.join(generated);
        match fs::remove_file(&path) {
            Ok(()) => {}
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
            Err(error) => panic!(
                "generated provider example file {} should be removable: {error}",
                path.display(),
            ),
        }
    }
    fs::create_dir_all(project.join(".cargo"))
        .expect("provider example Cargo config directory should be created");
    let geam_path = toml::Value::String(env!("CARGO_MANIFEST_DIR").to_owned()).to_string();
    fs::write(
        project.join(".cargo/config.toml"),
        format!("[patch.crates-io]\ngeam = {{ path = {geam_path} }}\n\n[net]\noffline = true\n",),
    )
    .expect("provider example Cargo config should be written");
    fixture
}

fn prepare_provider_dependency(name: &str) {
    static TEXT_TOOLS: OnceLock<Result<(), String>> = OnceLock::new();
    static VALUE_TYPES: OnceLock<Result<(), String>> = OnceLock::new();
    static TAG_SET: OnceLock<Result<(), String>> = OnceLock::new();
    static REQUEST_IDS: OnceLock<Result<(), String>> = OnceLock::new();
    static CALL_TRACING: OnceLock<Result<(), String>> = OnceLock::new();
    static GENERIC_BOX: OnceLock<Result<(), String>> = OnceLock::new();
    static FEATURE_FLAGS: OnceLock<Result<(), String>> = OnceLock::new();
    static RUN_METRICS: OnceLock<Result<(), String>> = OnceLock::new();
    static TEXT_PATTERN: OnceLock<Result<(), String>> = OnceLock::new();

    let prepared = match name {
        "text_tools" => &TEXT_TOOLS,
        "value_types" => &VALUE_TYPES,
        "tag_set" => &TAG_SET,
        "request_ids" => &REQUEST_IDS,
        "call_tracing" => &CALL_TRACING,
        "generic_box" => &GENERIC_BOX,
        "feature_flags" => &FEATURE_FLAGS,
        "run_metrics" => &RUN_METRICS,
        "text_pattern" => &TEXT_PATTERN,
        _ => panic!("unknown provider example {name}"),
    };
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    workspace_dependencies::prepare(
        prepared,
        &root.join("examples").join(name).join("provider"),
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
