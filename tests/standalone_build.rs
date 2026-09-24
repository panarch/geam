use std::ffi::OsString;
use std::fs;
use std::io::{Read, Seek};
use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};
use std::sync::OnceLock;
use std::time::{Duration, Instant};

#[path = "support/workspace_dependencies.rs"]
mod workspace_dependencies;

#[test]
fn builds_and_relocates_complete_applications_without_development_inputs() {
    let fixture = fixture();
    let project = fixture.path().join("project").canonicalize().unwrap();
    let source = project.join("src/standalone_fixture.gleam");
    let ordinary = fs::read_to_string(&source).unwrap();
    fs::write(&source, "pub fn main() { echo 42 fn(value) { value } }\n").unwrap();
    let destination = tempfile::Builder::new()
        .prefix("geam deployed space-")
        .tempdir()
        .unwrap();
    let deployed_root = destination.path().canonicalize().unwrap();
    let binary = deployed_root.join(format!("application{}", std::env::consts::EXE_SUFFIX));
    let cwd = deployed_root.join("different cwd");
    fs::create_dir(&cwd).unwrap();

    let build = checked(geam(&project, &["build"]).env("GEAM_CONFIG", "must-not-be-read.toml"));
    assert!(build.stdout.is_empty());
    let debug = project.join(format!(
        "build/geam/target/debug/standalone_fixture{}",
        std::env::consts::EXE_SUFFIX
    ));
    let reported = built_executable(&build, &debug);
    fs::copy(reported, &binary).unwrap();
    let output = checked(deployed(&binary, &cwd).args(["--help", "", "x=y", "--provider-config"]));
    assert!(output.stdout.is_empty());
    assert_eq!(
        output.stderr,
        b"standalone_fixture/src/standalone_fixture.gleam:1\n42\n"
    );
    assert_eq!(fs::read_dir(&deployed_root).unwrap().count(), 2);

    for package in ["geam-catalog", "geam-counter"] {
        checked(&mut geam(
            &project,
            &[
                "provider",
                "add",
                "--path",
                "../providers",
                "--package",
                package,
            ],
        ));
    }
    checked(&mut geam(
        &project,
        &["provider", "add", "--path", "../future/provider"],
    ));
    fs::write(project.join("src/ordinary.gleam"), ordinary).unwrap();
    let complete = r#"
import geam/future
import gleam/erlang/application
import gleam/erlang/process
import gleam/io
import ordinary
import standalone_future/native

pub fn main() {
  ordinary.main()
  let assert Ok(root) = application.priv_directory("standalone_fixture")
  let assert Ok(dependency) = application.priv_directory("pure_labels")
  io.println(root)
  io.println(dependency)
  let reply = process.new_subject()
  let _ = process.spawn_unlinked(fn() { process.send(reply, 42) })
  let assert 42 = process.receive_forever(reply)
  use count <- future.map(native.timer())
  let assert 1 = count
  let ready = process.new_subject()
  let _ = process.spawn_unlinked(fn() {
    process.send(ready, Nil)
    process.sleep_forever()
  })
  process.receive_forever(ready)
  native.current()
}
"#
    .trim_start();
    fs::write(&source, complete).unwrap();
    let build = checked(&mut geam(&project, &["build"]));
    assert!(
        build.stdout.is_empty(),
        "preparation initialized application state"
    );
    let program = fs::read_to_string(project.join("build/geam/program.rs")).unwrap();
    assert!(program.contains("pure_labels"));
    assert!(!program.contains(project.to_str().unwrap()));
    let manifest = fs::read(project.join("Cargo.toml")).unwrap();
    let lock = fs::read(project.join("Cargo.lock")).unwrap();
    checked(&mut geam(&project, &["prepare"]));
    let dynamic = checked(&mut geam(
        &project,
        &[
            "run",
            "--provider-config",
            "catalog=config/catalog.toml",
            "--provider-config",
            "counter=config/counter.toml",
        ],
    ));
    assert!(String::from_utf8_lossy(&dynamic.stdout).contains("\"count:3/count:4\"\n"));
    checked(&mut geam(&project, &["build"]));
    assert_eq!(
        fs::read(project.join("build/geam/program.rs")).unwrap(),
        program.as_bytes()
    );
    assert_eq!(fs::read(project.join("Cargo.toml")).unwrap(), manifest);
    assert_eq!(fs::read(project.join("Cargo.lock")).unwrap(), lock);

    fs::create_dir(deployed_root.join("configuration")).unwrap();
    fs::copy(
        project.join("config/catalog.toml"),
        deployed_root.join("configuration/catalog.toml"),
    )
    .unwrap();
    fs::copy(
        project.join("config/counter.toml"),
        deployed_root.join("configuration/counter.toml"),
    )
    .unwrap();
    let config = deployed_root.join("configuration/runtime.toml");
    fs::write(
        &config,
        "[providers]\ncatalog = 'catalog.toml'\ncounter = 'counter.toml'\n",
    )
    .unwrap();
    fs::copy(&debug, &binary).unwrap();
    let arguments = vec![
        OsString::from("--help"),
        OsString::from(""),
        OsString::from("catalog=config.toml"),
        OsString::from("--provider-config"),
        OsString::from("\u{c548}\u{b155}"),
    ];
    #[cfg(unix)]
    let arguments = {
        use std::os::unix::ffi::OsStringExt;
        let mut arguments = arguments;
        arguments.push(OsString::from_vec(b"native-\xff".to_vec()));
        arguments
    };
    let expected = format!(
        "initialized\narguments:{arguments:?}\n\"count:3/count:4\"\n{}\n{}\ntimer-pending\ntimer-complete\nstate:1\nstate-drop:1\n",
        deployed_root.join("priv/standalone_fixture").display(),
        deployed_root.join("priv/pure_labels").display()
    );
    let expected_echo = "provider-before\nstandalone_fixture/src/ordinary.gleam:44 provider-summary\nSummary(count: 1, items: [\"pure:native:alpha\"])\nprovider-after\n";
    for _ in 0..2 {
        let output = checked(
            deployed(&binary, &cwd)
                .env("GEAM_CONFIG", "../configuration/runtime.toml")
                .args(&arguments),
        );
        assert_application_stdout(&output.stdout, &expected, &deployed_root);
        assert_eq!(output.stderr, expected_echo.as_bytes());
    }
    let counter = deployed_root.join("configuration/counter.toml");
    fs::write(&counter, "start = 40\n").unwrap();
    let reconfigured = capture(
        deployed(&binary, &cwd)
            .env("GEAM_CONFIG", &config)
            .args(&arguments),
        Duration::from_secs(30),
    );
    // The fixture asserts a starting value of 3; changed configuration must reach it.
    assert!(!reconfigured.status.success());
    assert_eq!(
        reconfigured.stdout,
        format!("initialized\narguments:{arguments:?}\nstate-drop:0\n").as_bytes()
    );
    assert_eq!(
        reconfigured.stderr,
        b"geam application: assert: Assertion failed.\n"
    );
    fs::write(counter, "start = 3\n").unwrap();
    let absent = capture(&mut deployed(&binary, &cwd), Duration::from_secs(30));
    assert!(!absent.status.success());
    assert!(absent.stdout.is_empty());
    assert!(
        String::from_utf8_lossy(&absent.stderr)
            .contains("configuration key `prefix` must be a String")
    );
    fs::write(&config, "[providers]\ncatalog = 'catalog.toml'\ncounter = 'counter.toml'\n[resources]\nstandalone_fixture = 'custom assets'\n").unwrap();
    let changed = checked(
        deployed(&binary, &cwd)
            .env("GEAM_CONFIG", &config)
            .args(&arguments),
    );
    assert_application_stdout(
        &changed.stdout,
        &expected.replace(
            deployed_root
                .join("priv/standalone_fixture")
                .to_str()
                .unwrap(),
            deployed_root
                .join("configuration/custom assets")
                .to_str()
                .unwrap(),
        ),
        &deployed_root,
    );
    assert!(!deployed_root.join("configuration/custom assets").exists());

    for invalid in [
        "unexpected = true",
        "[providers]\ncatalog = 'missing'",
        "[resources]\nunknown = 'assets'",
    ] {
        fs::write(&config, invalid).unwrap();
        let output = capture(
            deployed(&binary, &cwd).env("GEAM_CONFIG", &config),
            Duration::from_secs(30),
        );
        assert!(!output.status.success());
        assert!(output.stdout.is_empty());
        assert!(String::from_utf8_lossy(&output.stderr).starts_with("geam application: "));
    }
    fs::write(
        &config,
        "[providers]\ncatalog = 'catalog.toml'\ncounter = 'counter.toml'\n",
    )
    .unwrap();

    fs::write(&source, "pub fn main( {\n").unwrap();
    let failed = capture(&mut geam(&project, &["build"]), Duration::from_secs(120));
    assert!(!failed.status.success());
    assert!(!String::from_utf8_lossy(&failed.stderr).contains("geam: Built "));
    assert_eq!(
        fs::read(project.join("build/geam/program.rs")).unwrap(),
        program.as_bytes()
    );
    fs::write(&source, complete).unwrap();
    for (module, body, stdout, failure) in [
        (
            "callback",
            "import standalone_future/native\npub fn main() { let assert 2 = native.apply(fn(_) { native.current() + 1 }) Nil }\n",
            "initialized\narguments:[]\nbefore-callback\nstate:1\nafter-callback\nstate-drop:1\n",
            "",
        ),
        (
            "panic_driver",
            "import standalone_future/native\npub fn main() { native.panic_driver() }\n",
            "initialized\narguments:[]\nstate-drop:0\n",
            "native driver panic",
        ),
        (
            "panic_worker",
            "import standalone_future/native\npub fn main() { native.panic_worker() }\n",
            "initialized\narguments:[]\nstate-drop:0\n",
            "geam application: the host executor failed:",
        ),
        (
            "cancelled",
            "import geam/future\nimport standalone_future/native\npub fn main() { future.all([native.pending(), native.fail()]) }\n",
            "initialized\narguments:[]\npending-started\npending-drop\nstate-drop:0\n",
            "native failure",
        ),
        (
            "shutdown",
            "import standalone_future/native\npub fn main() { native.spawn_worker() }\n",
            "initialized\narguments:[]\nblocking-started\nentry-complete\nstate-drop:0\nblocking-finished\n",
            "",
        ),
        (
            "failure",
            "pub fn main() {\n  echo 7\n  panic as \"deployed source failure\"\n}\n",
            "initialized\narguments:[]\nstate-drop:0\n",
            "deployed source failure",
        ),
    ] {
        fs::write(project.join(format!("src/{module}.gleam")), body).unwrap();
        checked(&mut geam(&project, &["build", "--module", module]));
        fs::copy(&debug, &binary).unwrap();
        let output = capture(
            deployed(&binary, &cwd).env("GEAM_CONFIG", &config),
            Duration::from_secs(30),
        );
        assert_eq!(output.status.success(), failure.is_empty());
        assert_eq!(output.stdout, stdout.as_bytes());
        assert!(String::from_utf8_lossy(&output.stderr).contains(failure));
        if module == "panic_driver" {
            assert!(!String::from_utf8_lossy(&output.stderr).contains("geam application:"));
            assert_eq!(output.status.code(), Some(101));
        }
        if module == "failure" {
            fs::copy(
                &binary,
                deployed_root.join(format!("failure{}", std::env::consts::EXE_SUFFIX)),
            )
            .unwrap();
        }
    }
    let release = checked(&mut geam(&project, &["build", "--release"]));
    assert!(release.stdout.is_empty());
    let release_path = project.join(format!(
        "build/geam/target/release/standalone_fixture{}",
        std::env::consts::EXE_SUFFIX
    ));
    let reported = built_executable(&release, &release_path);
    fs::copy(reported, &binary).unwrap();
    assert_eq!(fs::read(project.join("Cargo.toml")).unwrap(), manifest);
    assert_eq!(fs::read(project.join("Cargo.lock")).unwrap(), lock);
    drop(fixture);
    assert!(!project.exists());
    let output = checked(
        deployed(&binary, &cwd)
            .env("GEAM_CONFIG", &config)
            .args(&arguments),
    );
    assert_application_stdout(&output.stdout, &expected, &deployed_root);
    assert_eq!(output.stderr, expected_echo.as_bytes());
    let failure = capture(
        deployed(
            &deployed_root.join(format!("failure{}", std::env::consts::EXE_SUFFIX)),
            &cwd,
        )
        .env("GEAM_CONFIG", &config),
        Duration::from_secs(30),
    );
    assert!(!failure.status.success());
    let diagnostic = String::from_utf8_lossy(&failure.stderr);
    assert!(diagnostic.contains("standalone_fixture/src/failure.gleam:2\n7\n"));
    assert!(diagnostic.contains("deployed source failure"));
    assert!(!diagnostic.contains(project.to_str().unwrap()));
}

fn assert_application_stdout(actual: &[u8], expected: &str, directory: &Path) {
    let stdout = std::str::from_utf8(actual).unwrap();
    let actual: Vec<_> = stdout.split('\n').collect();
    let expected: Vec<_> = expected.split('\n').collect();
    assert_eq!(actual.len(), expected.len(), "{stdout}");
    let directory = directory.canonicalize().unwrap();
    for (index, (actual, expected)) in actual.into_iter().zip(expected).enumerate() {
        // Only the two resource fields may differ in filesystem path spelling.
        if matches!(index, 3 | 4) {
            let suffix = Path::new(expected).strip_prefix(&directory).unwrap();
            let mut base = PathBuf::from(actual);
            assert!(
                base.is_absolute(),
                "resource path is not absolute: {actual}"
            );
            assert!(
                base.ends_with(suffix),
                "unexpected resource suffix: {actual}"
            );
            for _ in suffix.components() {
                assert!(base.pop());
            }
            // The resource suffix need not exist; only the deployment root does.
            assert_eq!(base.canonicalize().unwrap(), directory, "{actual}");
        } else {
            assert_eq!(actual, expected, "stdout line {}", index + 1);
        }
    }
}

#[test]
fn application_stdout_checks_resource_locations_without_rewriting_other_output() {
    let directory = tempfile::tempdir().unwrap();
    let root = directory.path().canonicalize().unwrap();
    fs::create_dir(root.join("configuration")).unwrap();
    fs::create_dir(root.join("wrong base")).unwrap();
    let reported_root = directory.path().join("configuration").join("..");

    for resource in ["priv/standalone_fixture", "configuration/custom assets"] {
        let expected = format!(
            "initialized\narguments:[\\\\?\\literal]\nvalue\n{}\n{}\ncomplete\n",
            root.join(resource).display(),
            root.join("priv/pure_labels").display(),
        );
        let actual = format!(
            "initialized\narguments:[\\\\?\\literal]\nvalue\n{}\n{}\ncomplete\n",
            reported_root.join(resource).display(),
            reported_root.join("priv/pure_labels").display(),
        );
        assert_application_stdout(actual.as_bytes(), &expected, &root);
        assert!(!root.join(resource).exists());
        assert!(!root.join("priv/pure_labels").exists());

        let resource_path = root.join(resource);
        let resource_path = resource_path.to_str().unwrap();
        for rejected in [
            expected.replacen("initialized", "unexpected", 1),
            expected.replacen(r"\\?\literal", "literal", 1),
            expected.replacen("complete", "unexpected", 1),
            expected.replacen(resource_path, resource, 1),
            expected.replacen(
                resource_path,
                root.join("wrong base").join(resource).to_str().unwrap(),
                1,
            ),
            expected.replacen(
                resource_path,
                root.join("priv/wrong_package").to_str().unwrap(),
                1,
            ),
            expected.replacen(
                root.join("priv/pure_labels").to_str().unwrap(),
                resource_path,
                1,
            ),
            expected.trim_end().into(),
            format!("{expected}extra\n"),
        ] {
            assert!(
                std::panic::catch_unwind(|| {
                    assert_application_stdout(rejected.as_bytes(), &expected, &root);
                })
                .is_err(),
                "accepted incorrect stdout: {rejected}"
            );
        }
    }
}

#[test]
fn forwards_application_arguments_through_run_and_relocated_native_execution() {
    let fixture = fixture();
    let project = fixture.path().join("project").canonicalize().unwrap();
    let manifest_path = project.join("gleam.toml");
    let mut manifest: toml::Table = fs::read_to_string(&manifest_path).unwrap().parse().unwrap();
    manifest["dependencies"].as_table_mut().unwrap().insert(
        "application_arguments".into(),
        toml::toml! { path = "packages/application_arguments" }.into(),
    );
    fs::write(&manifest_path, toml::to_string(&manifest).unwrap()).unwrap();
    for provider in ["geam-counter", "geam-arguments-fixture"] {
        checked(&mut geam(
            &project,
            &[
                "provider",
                "add",
                "--path",
                "../providers",
                "--package",
                provider,
            ],
        ));
    }
    let mut native: Vec<OsString> = [
        "",
        "space value",
        "--help",
        "--module",
        "--provider-config",
        "counter=not-a-config",
        "--",
        "한글",
        "quote\"'\\",
        "line\nbreak",
        "repeat",
        "repeat",
    ]
    .map(Into::into)
    .into();
    #[cfg(unix)]
    {
        use std::os::unix::ffi::OsStringExt;
        native.push(OsString::from_vec(b"native-\xff".to_vec()));
    }
    #[cfg(windows)]
    {
        use std::os::windows::ffi::OsStringExt;
        native.push(OsString::from_wide(&[0x61, 0xd800]));
    }
    let cases = [
        (false, Vec::new()),
        (true, Vec::new()),
        (true, vec![OsString::from("alpha"), OsString::from("beta")]),
        (true, native.clone()),
    ];
    for (separator, arguments) in cases {
        let strings: Vec<_> = arguments
            .iter()
            .map(|value| value.to_string_lossy().into_owned())
            .collect();
        let units: Vec<Vec<u32>> = arguments
            .iter()
            .map(|value| {
                #[cfg(unix)]
                {
                    use std::os::unix::ffi::OsStrExt;
                    value.as_bytes().iter().copied().map(u32::from).collect()
                }
                #[cfg(windows)]
                {
                    use std::os::windows::ffi::OsStrExt;
                    value.encode_wide().map(u32::from).collect()
                }
            })
            .collect();
        let source = format!(
            r#"import application_arguments
import counter
import gleam/io

pub fn main() {{
  let snapshot = application_arguments.snapshot()
  let assert {strings:?} = snapshot.0
  let assert {units:?} = snapshot.1
  let assert "args:3" = counter.next("args")
  io.println("arguments-observed")
}}
"#
        );
        fs::write(project.join("src/arguments.gleam"), source).unwrap();
        let mut command = geam(
            &project,
            &[
                "run",
                "--module",
                "arguments",
                "--provider-config",
                "counter=config/counter.toml",
            ],
        );
        if separator {
            command.arg("--");
        }
        let output = checked(command.args(&arguments));
        assert_eq!(
            output.stdout,
            b"arguments-initialized\narguments-observed\n"
        );
    }
    let runner = project.join(format!(
        "build/geam/target/debug/geam-runner{}",
        std::env::consts::EXE_SUFFIX
    ));
    for (control, diagnostic) in [
        (None, "missing GEAM_RUNNER_CONTROL"),
        (
            Some("="),
            "invalid runner control TOML: unquoted keys cannot be empty, expected letters, numbers, `-`, `_`",
        ),
        (Some("schema = 2"), "runner control schema must be 1"),
        (
            Some("schema=1\nmode='other'\nproject_root='unused'\nmodule='unused'"),
            "unknown runner control mode other",
        ),
    ] {
        let mut command = Command::new(&runner);
        command
            .current_dir(&project)
            .env_remove("GEAM_RUNNER_CONTROL");
        if let Some(control) = control {
            command.env("GEAM_RUNNER_CONTROL", control);
        }
        // Legacy positional control is application data, never a fallback protocol.
        command.args(["run", "unused", "unused"]);
        let output = capture(&mut command, Duration::from_secs(30));
        assert_eq!(output.status.code(), Some(1));
        assert!(
            output.stdout.is_empty(),
            "invalid control initialized provider state"
        );
        assert_eq!(
            output.stderr,
            format!("geam runner: {diagnostic}\n").as_bytes()
        );
    }
    let prepared = checked(&mut geam(&project, &["prepare", "--module", "arguments"]));
    assert!(
        prepared.stdout.is_empty(),
        "checking initialized provider state"
    );
    let build = checked(&mut geam(&project, &["build", "--module", "arguments"]));
    assert!(
        build.stdout.is_empty(),
        "building initialized provider state"
    );
    let binary = project.join(format!(
        "build/geam/target/debug/standalone_fixture{}",
        std::env::consts::EXE_SUFFIX
    ));
    let destination = tempfile::Builder::new()
        .prefix("geam arguments deployed ")
        .tempdir()
        .unwrap();
    let deployed_binary = destination
        .path()
        .join(format!("application{}", std::env::consts::EXE_SUFFIX));
    fs::copy(built_executable(&build, &binary), &deployed_binary).unwrap();
    fs::write(destination.path().join("counter.toml"), "start = 3\n").unwrap();
    let config = destination.path().join("runtime.toml");
    fs::write(&config, "[providers]\ncounter = 'counter.toml'\n").unwrap();
    fixture.close().unwrap();
    for private_control in [None, Some("malformed and irrelevant to native execution")] {
        let mut command = deployed(&deployed_binary, destination.path());
        command
            .env("GEAM_CONFIG", &config)
            .env_remove("GEAM_RUNNER_CONTROL")
            .args(&native);
        if let Some(value) = private_control {
            command.env("GEAM_RUNNER_CONTROL", value);
        }
        let output = checked(&mut command);
        assert_eq!(
            output.stdout,
            b"arguments-initialized\narguments-observed\n"
        );
        assert!(output.stderr.is_empty());
    }
}

fn fixture() -> tempfile::TempDir {
    let repository = Path::new(env!("CARGO_MANIFEST_DIR"));
    let canonical = repository.join("cli/tests/fixtures/standalone_cli");
    let future = repository.join("cli/tests/fixtures/standalone_future");
    static PROVIDERS: OnceLock<Result<(), String>> = OnceLock::new();
    workspace_dependencies::prepare(
        &PROVIDERS,
        &canonical.join("providers"),
        "cargo",
        &["fetch", "--locked", "--config", "net.offline=false"],
        "provider fixture dependencies",
    );
    static FUTURE: OnceLock<Result<(), String>> = OnceLock::new();
    workspace_dependencies::prepare(
        &FUTURE,
        &future.join("provider"),
        "cargo",
        &["fetch", "--locked", "--config", "net.offline=false"],
        "Future provider dependencies",
    );
    let directory = tempfile::tempdir().unwrap();
    copy_directory(&canonical, directory.path());
    copy_directory(&future, &directory.path().join("future"));
    let root = directory.path();
    let project = root.join("project");
    for file in [
        "Cargo.toml",
        "Cargo.lock",
        "Cargo.toml.geam.tmp",
        "manifest.toml",
    ] {
        if project.join(file).is_file() {
            fs::remove_file(project.join(file)).unwrap();
        }
    }
    let future_gleam = root.join("future/project/gleam.toml");
    let mut manifest: toml::Table = fs::read_to_string(&future_gleam).unwrap().parse().unwrap();
    manifest["dependencies"]["geam"]["path"] = toml::Value::String(
        repository
            .join("builtins/geam/gleam")
            .to_str()
            .unwrap()
            .into(),
    );
    fs::write(future_gleam, toml::to_string(&manifest).unwrap()).unwrap();
    let manifest_path = project.join("gleam.toml");
    let mut manifest: toml::Table = fs::read_to_string(&manifest_path).unwrap().parse().unwrap();
    let dependencies = manifest["dependencies"].as_table_mut().unwrap();
    let future_path = repository.join("builtins/geam/gleam");
    let future_path = future_path.to_str().unwrap();
    dependencies.insert("geam".into(), toml::toml! { path = future_path }.into());
    dependencies.insert(
        "standalone_future".into(),
        toml::toml! { path = "../future/project" }.into(),
    );
    dependencies.insert(
        "gleam_erlang".into(),
        toml::Value::String(">= 1.3.0 and < 1.3.1".into()),
    );
    fs::write(manifest_path, toml::to_string(&manifest).unwrap()).unwrap();
    let geam_path = toml::Value::String(repository.to_str().unwrap().into());
    for cargo_root in [
        &project,
        &root.join("providers"),
        &root.join("future/provider"),
    ] {
        fs::create_dir_all(cargo_root.join(".cargo")).unwrap();
        fs::write(
            cargo_root.join(".cargo/config.toml"),
            format!("[patch.crates-io]\ngeam = {{ path = {geam_path} }}\n[net]\noffline = true\n"),
        )
        .unwrap();
    }
    let provider = root.join("future/provider/Cargo.toml");
    let mut manifest: toml::Table = fs::read_to_string(&provider).unwrap().parse().unwrap();
    manifest.remove("patch");
    fs::write(provider, toml::to_string(&manifest).unwrap()).unwrap();
    let provider = root.join("future/provider/src/lib.rs");
    let source = fs::read_to_string(&provider).unwrap().replace("println!(\"initialized\");", "println!(\"initialized\");\n        println!(\"arguments:{:?}\", std::env::args_os().skip(1).collect::<Vec<_>>());");
    fs::write(provider, source).unwrap();
    directory
}

fn built_executable(build: &Output, expected: &Path) -> PathBuf {
    let stderr = std::str::from_utf8(&build.stderr).unwrap();
    let reported = stderr
        .lines()
        .last()
        .and_then(|line| line.strip_prefix("geam: Built "));
    assert!(reported.is_some(), "missing executable report:\n{stderr}");
    let reported = PathBuf::from(reported.unwrap());
    assert!(
        reported.is_absolute(),
        "reported executable is not absolute:\n{stderr}"
    );
    assert_eq!(
        reported.canonicalize().expect(stderr),
        expected.canonicalize().unwrap(),
        "{stderr}"
    );
    reported
}

fn geam(directory: &Path, arguments: &[&str]) -> Command {
    let mut command = Command::new(env!("CARGO_BIN_EXE_geam"));
    command
        .current_dir(directory)
        .args(arguments)
        .env_remove("CARGO_ENCODED_RUSTFLAGS")
        .env_remove("CARGO_LLVM_COV")
        .env_remove("CARGO_LLVM_COV_TARGET_DIR")
        .env_remove("RUSTFLAGS")
        .env_remove("RUSTDOCFLAGS");
    command
}

fn deployed(binary: &Path, cwd: &Path) -> Command {
    let mut command = Command::new(binary);
    command
        .current_dir(cwd)
        .env("PATH", "")
        .env_remove("GEAM_CONFIG");
    command
}

fn checked(command: &mut Command) -> Output {
    let output = capture(command, Duration::from_secs(1800));
    assert!(
        output.status.success(),
        "{command:?}\n{}\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    output
}

fn capture(command: &mut Command, timeout: Duration) -> Output {
    let mut stdout = tempfile::tempfile().unwrap();
    let mut stderr = tempfile::tempfile().unwrap();
    let mut child = command
        .stdin(Stdio::null())
        .stdout(stdout.try_clone().unwrap())
        .stderr(stderr.try_clone().unwrap())
        .spawn()
        .unwrap();
    let deadline = Instant::now() + timeout;
    let status = loop {
        if let Some(status) = child.try_wait().unwrap() {
            break status;
        }
        if Instant::now() >= deadline {
            child.kill().unwrap();
            child.wait().unwrap();
            panic!("subprocess exceeded {timeout:?}: {command:?}");
        }
        std::thread::sleep(Duration::from_millis(20));
    };
    stdout.rewind().unwrap();
    stderr.rewind().unwrap();
    let mut output = Output {
        status,
        stdout: Vec::new(),
        stderr: Vec::new(),
    };
    stdout.read_to_end(&mut output.stdout).unwrap();
    stderr.read_to_end(&mut output.stderr).unwrap();
    output
}

fn copy_directory(source: &Path, destination: &Path) {
    fs::create_dir_all(destination).unwrap();
    for entry in fs::read_dir(source).unwrap() {
        let entry = entry.unwrap();
        let name = entry.file_name();
        let source = entry.path();
        let destination = destination.join(&name);
        if source.is_dir() {
            if !matches!(name.to_str(), Some("build" | "target")) {
                copy_directory(&source, &destination);
            }
        } else {
            fs::copy(source, destination).unwrap();
        }
    }
}
