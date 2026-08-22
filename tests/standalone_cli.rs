use std::fs;
use std::path::Path;
#[cfg(unix)]
use std::process::Stdio;
use std::process::{Command, Output};
use std::sync::OnceLock;
use tempfile::{TempDir, tempdir};

#[path = "support/workspace_dependencies.rs"]
mod workspace_dependencies;

#[test]
fn reports_missing_projects_through_the_binary_boundary() {
    let directory = tempdir().expect("temporary directory should be created");

    let output = geam(&directory, ["prepare"]);

    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("no gleam.toml was found"));
}

#[cfg(unix)]
#[test]
fn reports_current_directory_failures_through_the_binary_boundary() {
    let directory = tempdir().expect("temporary directory should be created");
    let deleted = directory.path().join("deleted");
    fs::create_dir(&deleted).expect("working directory should be created");

    let mut command = Command::new("/bin/sh");
    command
        .args([
            "-c",
            "cd \"$1\" && rmdir \"$1\" && exec \"$2\" prepare",
            "sh",
        ])
        .arg(&deleted)
        .arg(env!("CARGO_BIN_EXE_geam"));
    remove_nested_cargo_instrumentation(&mut command);
    let output = command.output().expect("Geam CLI should start");

    assert!(!output.status.success());
    assert_eq!(
        String::from_utf8_lossy(&output.stderr),
        "geam: failed to determine the current directory\n",
    );
}

#[test]
fn reports_entry_resolution_failures_through_the_binary_boundary() {
    let project = tempdir().expect("temporary directory should be created");
    fs::write(project.path().join("gleam.toml"), "invalid")
        .expect("invalid Gleam config should be written");

    for arguments in [["prepare"], ["run"]] {
        let output = geam(&project, arguments);
        assert!(!output.status.success());
        assert!(String::from_utf8_lossy(&output.stderr).contains("invalid Gleam package config"),);
    }
}

#[test]
fn reports_project_compilation_failures_through_the_binary_boundary() {
    let project = gleam_project();

    for command in ["prepare", "run"] {
        let output = geam(&project, [command, "--module", "missing"]);
        assert!(!output.status.success());
        assert_eq!(
            String::from_utf8_lossy(&output.stderr),
            "geam: root module missing was not supplied by package application\n",
        );
    }
}

#[test]
fn prepares_and_runs_pure_projects_without_printing_main_results() {
    let project = gleam_project();

    let prepare = geam(&project, ["prepare"]);
    assert!(
        prepare.status.success(),
        "prepare failed: {}",
        String::from_utf8_lossy(&prepare.stderr),
    );
    assert!(project.path().join("Cargo.lock").is_file());
    assert!(project.path().join("build/geam/runner.rs").is_file());

    let run = geam(&project, ["run", "--module", "application"]);
    assert!(
        run.status.success(),
        "run failed: {}",
        String::from_utf8_lossy(&run.stderr),
    );
    assert!(run.stdout.is_empty());
    assert!(
        run.stderr.is_empty(),
        "run wrote unexpected stderr: {}",
        String::from_utf8_lossy(&run.stderr),
    );
}

#[test]
fn streams_gleam_io_and_echo_in_source_order_before_runtime_failure() {
    let project = io_project();

    let run = geam(&project, ["run"]);
    assert!(
        run.status.success(),
        "IO run failed: {}",
        String::from_utf8_lossy(&run.stderr),
    );
    assert_eq!(String::from_utf8_lossy(&run.stdout), "stdout");
    let stderr = String::from_utf8_lossy(&run.stderr);
    let first = stderr
        .find("stderr-one\n")
        .expect("first stderr IO should be present");
    let echo = stderr
        .find(" middle\nNil\n")
        .expect("echo output should be present");
    let last = stderr
        .find("stderr-two\n")
        .expect("last stderr IO should be present");
    assert!(first < echo && echo < last);

    #[cfg(unix)]
    {
        let mut child = geam_command(&project, ["run"])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("Geam CLI should start with piped output");
        drop(child.stdout.take());
        let failed_output = child
            .wait_with_output()
            .expect("Geam CLI output failure should complete");
        assert!(!failed_output.status.success());
        let stderr = String::from_utf8_lossy(&failed_output.stderr);
        assert!(stderr.contains("geam runner:"));
        assert!(stderr.contains("after writing its output directly"));
    }

    fs::write(
        project.path().join("src/application.gleam"),
        r#"import gleam/io

pub fn main() {
  io.print_error("before panic\n")
  panic as "stop"
}
"#,
    )
    .expect("failing source should be written");
    let failed = geam(&project, ["run"]);
    assert!(!failed.status.success());
    assert!(failed.stdout.is_empty());
    let stderr = String::from_utf8_lossy(&failed.stderr);
    assert!(stderr.starts_with("before panic\n"));
    assert!(stderr.contains("stop"));
}

#[test]
fn dispatches_provider_add_and_remove_through_the_binary_boundary() {
    let project = gleam_project_with_dependency("images", "1.2.3");
    let provider = provider_package();

    let add = geam(
        &project,
        [
            "provider",
            "add",
            "--path",
            provider
                .path()
                .to_str()
                .expect("path should be valid UTF-8"),
        ],
    );
    assert!(
        add.status.success(),
        "provider add failed: {}",
        String::from_utf8_lossy(&add.stderr),
    );

    let remove = geam(&project, ["provider", "remove", "images"]);
    assert!(
        remove.status.success(),
        "provider remove failed: {}",
        String::from_utf8_lossy(&remove.stderr),
    );
}

#[test]
fn prepares_an_independent_path_provider_through_the_generated_runner() {
    let project = provider_backed_project();
    let provider = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/provider_sdk/provider");

    let add = geam(
        &project,
        [
            "provider",
            "add",
            "--path",
            provider
                .to_str()
                .expect("provider path should be valid UTF-8"),
        ],
    );
    assert!(
        add.status.success(),
        "provider add failed: {}",
        String::from_utf8_lossy(&add.stderr),
    );

    let prepare = geam(&project, ["prepare"]);
    assert!(
        prepare.status.success(),
        "provider-backed prepare failed: {}",
        String::from_utf8_lossy(&prepare.stderr),
    );
    let manifest = fs::read_to_string(project.path().join("Cargo.toml"))
        .expect("managed manifest should be readable");
    let runner = fs::read_to_string(project.path().join("build/geam/runner.rs"))
        .expect("runner source should be readable");
    assert!(manifest.contains("geam_provider_provider_sdk_example"));
    assert!(runner.contains("geam_provider_provider_sdk_example::Component"));

    let repeated = geam(&project, ["prepare", "--module", "application"]);
    assert!(
        repeated.status.success(),
        "repeated prepare failed: {}",
        String::from_utf8_lossy(&repeated.stderr),
    );
    assert_eq!(
        fs::read_to_string(project.path().join("Cargo.toml"))
            .expect("managed manifest should remain readable"),
        manifest,
    );
    assert_eq!(
        fs::read_to_string(project.path().join("build/geam/runner.rs"))
            .expect("runner source should remain readable"),
        runner,
    );

    fs::write(
        project.path().join("provider.toml"),
        r#"prefix = "sdk:"
integer = -7
float = 1.5
enabled = true
array = ["text", 1, 2.5, false, { nested = "value" }]

[table]
nested = true
"#,
    )
    .expect("provider configuration should be written");
    for _ in 0..2 {
        let run = geam(
            &project,
            [
                "run",
                "--provider-config",
                "provider_sdk_example=provider.toml",
            ],
        );
        assert!(
            run.status.success(),
            "configured provider run failed: {}",
            String::from_utf8_lossy(&run.stderr),
        );
        assert!(run.stdout.is_empty());
        assert!(run.stderr.is_empty());
    }

    let missing = geam(&project, ["run"]);
    assert!(!missing.status.success());
    assert!(String::from_utf8_lossy(&missing.stderr).contains(
        "could not initialize host provider component provider-sdk-example: configuration key `prefix` must be a String"
    ));

    fs::write(
        project.path().join("invalid.toml"),
        "prefix = \"sdk:\"\ncreated = 1979-05-27T07:32:00Z\n",
    )
    .expect("unsupported provider configuration should be written");
    let invalid = geam(
        &project,
        [
            "run",
            "--provider-config",
            "provider_sdk_example=invalid.toml",
        ],
    );
    assert!(!invalid.status.success());
    assert!(
        String::from_utf8_lossy(&invalid.stderr)
            .contains("TOML datetime configuration values are unsupported"),
    );
}

#[test]
fn runs_the_documented_run_metrics_provider_without_configuration() {
    let fixture = run_metrics_example();
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
fn runs_the_documented_tag_set_provider_without_configuration() {
    let fixture = tag_set_example();
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
    let fixture = request_ids_example();
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
fn runs_the_documented_feature_flags_provider_with_explicit_configuration() {
    let fixture = feature_flags_example();
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
    let fixture = text_pattern_example();
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

#[test]
fn refuses_to_adopt_user_owned_cargo_projects() {
    let project = gleam_project();
    fs::write(
        project.path().join("Cargo.toml"),
        "[package]\nname = \"application\"\nversion = \"1.0.0\"\n",
    )
    .expect("user Cargo manifest should be written");

    let output = geam(&project, ["prepare"]);

    assert!(!output.status.success());
    let manifest = fs::canonicalize(project.path())
        .expect("temporary project should canonicalize")
        .join("Cargo.toml");
    assert_eq!(
        String::from_utf8_lossy(&output.stderr),
        format!(
            "geam: refusing to modify user-owned Cargo manifest {}; use the manual embedding workflow tracked by #115\n",
            manifest.display(),
        ),
    );
}

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

fn geam<const N: usize>(directory: &TempDir, arguments: [&str; N]) -> Output {
    geam_at(directory.path(), arguments)
}

fn geam_at<const N: usize>(directory: &Path, arguments: [&str; N]) -> Output {
    geam_command_at(directory, arguments)
        .output()
        .expect("Geam CLI should start")
}

fn geam_command<const N: usize>(directory: &TempDir, arguments: [&str; N]) -> Command {
    geam_command_at(directory.path(), arguments)
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

fn gleam_project() -> TempDir {
    let project = tempdir().expect("temporary project should be created");
    fs::create_dir(project.path().join("src")).expect("source directory should be created");
    write_cargo_config(&project);
    fs::write(
        project.path().join("gleam.toml"),
        "name = \"application\"\nversion = \"1.0.0\"\n",
    )
    .expect("Gleam config should be written");
    fs::write(
        project.path().join("manifest.toml"),
        "packages = []\n[requirements]\n",
    )
    .expect("Gleam manifest should be written");
    fs::write(
        project.path().join("src/application.gleam"),
        "pub fn main() { 1 }\n",
    )
    .expect("Gleam source should be written");
    project
}

fn provider_backed_project() -> TempDir {
    let project = gleam_project();
    fs::write(
        project.path().join("gleam.toml"),
        r#"name = "application"
version = "1.0.0"

[dependencies]
provider_sdk_example = { path = "packages/provider_sdk_example" }
"#,
    )
    .expect("Gleam config should be written");
    fs::write(
        project.path().join("manifest.toml"),
        r#"packages = [
  { name = "provider_sdk_example", version = "1.0.0", build_tools = ["gleam"], requirements = [], source = "local", path = "packages/provider_sdk_example" },
]

[requirements]
provider_sdk_example = { path = "packages/provider_sdk_example" }
"#,
    )
    .expect("Gleam manifest should be written");
    fs::write(
        project.path().join("src/application.gleam"),
        r#"import provider/sdk

pub fn main() {
  sdk.decorate("item", fn(value) { value <> "!" })
}
"#,
    )
    .expect("application source should be written");
    let package = project.path().join("packages/provider_sdk_example");
    fs::create_dir_all(package.join("src/provider"))
        .expect("provider package source directory should be created");
    fs::write(
        package.join("gleam.toml"),
        "name = \"provider_sdk_example\"\nversion = \"1.0.0\"\n",
    )
    .expect("provider package config should be written");
    fs::write(
        package.join("src/provider/sdk.gleam"),
        r#"@external(erlang, "provider_sdk", "Catalog")
pub type Catalog

@external(erlang, "provider_sdk", "decorate")
pub fn decorate(value: String, transform: fn(String) -> String) -> String

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
"#,
    )
    .expect("provider package source should be written");
    project
}

fn io_project() -> TempDir {
    let project = gleam_project();
    fs::write(
        project.path().join("gleam.toml"),
        r#"name = "application"
version = "1.0.0"

[dependencies]
gleam_stdlib = { path = "packages/gleam_stdlib" }
"#,
    )
    .expect("Gleam config should be written");
    fs::write(
        project.path().join("manifest.toml"),
        r#"packages = [
  { name = "gleam_stdlib", version = "1.0.3", build_tools = ["gleam"], requirements = [], source = "local", path = "packages/gleam_stdlib" },
]

[requirements]
gleam_stdlib = { path = "packages/gleam_stdlib" }
"#,
    )
    .expect("Gleam manifest should be written");
    fs::write(
        project.path().join("src/application.gleam"),
        r#"import gleam/io

pub fn main() {
  io.print("stdout")
  io.print_error("stderr-one\n")
  echo Nil as "middle"
  io.print_error("stderr-two\n")
  Nil
}
"#,
    )
    .expect("application source should be written");
    let package = project.path().join("packages/gleam_stdlib");
    fs::create_dir_all(package.join("src/gleam"))
        .expect("stdlib package source directory should be created");
    fs::write(
        package.join("gleam.toml"),
        "name = \"gleam_stdlib\"\nversion = \"1.0.3\"\n",
    )
    .expect("stdlib package config should be written");
    fs::write(
        package.join("src/gleam/io.gleam"),
        r#"@external(erlang, "gleam_stdlib", "print")
pub fn print(output: String) -> Nil

@external(erlang, "gleam_stdlib", "print_error")
pub fn print_error(output: String) -> Nil

@external(erlang, "gleam_stdlib", "println")
pub fn println(output: String) -> Nil

@external(erlang, "gleam_stdlib", "println_error")
pub fn println_error(output: String) -> Nil
"#,
    )
    .expect("stdlib IO source should be written");
    project
}

fn write_cargo_config(project: &TempDir) {
    fs::create_dir(project.path().join(".cargo"))
        .expect("Cargo config directory should be created");
    let geam_path = toml::Value::String(env!("CARGO_MANIFEST_DIR").to_owned()).to_string();
    fs::write(
        project.path().join(".cargo/config.toml"),
        format!("[patch.crates-io]\ngeam = {{ path = {geam_path} }}\n\n[net]\noffline = true\n"),
    )
    .expect("Cargo config should be written");
}

fn standalone_fixture() -> TempDir {
    let fixture = tempdir().expect("temporary standalone fixture should be created");
    let source = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/standalone_cli");
    static GLEAM_DEPENDENCIES: OnceLock<Result<(), String>> = OnceLock::new();
    workspace_dependencies::prepare(
        &GLEAM_DEPENDENCIES,
        &source.join("project"),
        "gleam",
        &["deps", "download"],
        "`gleam deps download`",
    );
    static PROVIDER_DEPENDENCIES: OnceLock<Result<(), String>> = OnceLock::new();
    workspace_dependencies::prepare(
        &PROVIDER_DEPENDENCIES,
        &source.join("providers"),
        "cargo",
        &["fetch", "--locked", "--config", "net.offline=false"],
        "`cargo fetch --locked --config net.offline=false`",
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
        format!("[patch.crates-io]\ngeam = {{ path = {geam_path} }}\n\n[net]\noffline = true\n",),
    )
    .expect("provider Cargo config should be written");
    fixture
}

fn run_metrics_example() -> TempDir {
    static PROVIDER_DEPENDENCIES: OnceLock<Result<(), String>> = OnceLock::new();
    provider_example("run_metrics", &PROVIDER_DEPENDENCIES)
}

fn tag_set_example() -> TempDir {
    static PROVIDER_DEPENDENCIES: OnceLock<Result<(), String>> = OnceLock::new();
    provider_example("tag_set", &PROVIDER_DEPENDENCIES)
}

fn request_ids_example() -> TempDir {
    static PROVIDER_DEPENDENCIES: OnceLock<Result<(), String>> = OnceLock::new();
    provider_example("request_ids", &PROVIDER_DEPENDENCIES)
}

fn feature_flags_example() -> TempDir {
    static PROVIDER_DEPENDENCIES: OnceLock<Result<(), String>> = OnceLock::new();
    provider_example("feature_flags", &PROVIDER_DEPENDENCIES)
}

fn text_pattern_example() -> TempDir {
    static PROVIDER_DEPENDENCIES: OnceLock<Result<(), String>> = OnceLock::new();
    provider_example("text_pattern", &PROVIDER_DEPENDENCIES)
}

fn provider_example(
    name: &str,
    provider_dependencies: &'static OnceLock<Result<(), String>>,
) -> TempDir {
    let source = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("examples")
        .join(name);
    workspace_dependencies::prepare(
        provider_dependencies,
        &source.join("provider"),
        "cargo",
        &["fetch", "--locked", "--config", "net.offline=false"],
        "`cargo fetch --locked --config net.offline=false`",
    );

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

fn gleam_project_with_dependency(package: &str, version: &str) -> TempDir {
    let project = gleam_project();
    fs::write(
        project.path().join("gleam.toml"),
        format!(
            "name = \"application\"\nversion = \"1.0.0\"\n\n[dependencies]\n{package} = \"{version}\"\n",
        ),
    )
    .expect("Gleam config should be written");
    fs::write(
        project.path().join("manifest.toml"),
        format!(
            "packages = [\n  {{ name = \"{package}\", version = \"{version}\", build_tools = [\"gleam\"], requirements = [], source = \"hex\", outer_checksum = \"00\" }},\n]\n\n[requirements]\n",
        ),
    )
    .expect("Gleam manifest should be written");
    project
}

fn provider_package() -> TempDir {
    let provider = tempdir().expect("provider package should be created");
    fs::create_dir(provider.path().join("src")).expect("source directory should be created");
    fs::write(
        provider.path().join("Cargo.toml"),
        r#"[package]
name = "geam-images"
version = "1.0.0"
edition = "2024"

[package.metadata.geam.provider]
schema = 1
gleam-package = "images"
gleam-version = ">= 1.0.0 and < 2.0.0"
"#,
    )
    .expect("provider manifest should be written");
    fs::write(
        provider.path().join("src/lib.rs"),
        "pub struct Component;\n",
    )
    .expect("provider source should be written");
    provider
}
