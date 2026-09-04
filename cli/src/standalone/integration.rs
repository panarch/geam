use crate::command::AddProvider;
use crate::process::run_checked;
use crate::progress::Progress;
use crate::provider::SystemProviderValidator;
use crate::runner::SystemCargo;
use camino::{Utf8Path, Utf8PathBuf};
use cargo_metadata::MetadataCommand;
use semver::Version;
use std::fs;
use std::io::{self, Write};
use std::path::Path;
use std::process::Command;
use std::sync::{Arc, Mutex};
use tempfile::{TempDir, tempdir};

#[test]
fn selects_locks_builds_and_runs_explicit_path_providers() {
    let fixture = standalone_fixture();
    let fixture_root = utf8_path(&fixture);
    let project_root = fixture_root.join("project");
    let catalog_path = canonical_utf8(&fixture_root.join("providers/geam-catalog"));
    let counter_path = canonical_utf8(&fixture_root.join("providers/geam-counter"));
    for path in [&catalog_path, &counter_path] {
        crate::provider::add(
            &project_root,
            fixture.path(),
            AddProvider {
                crate_spec: None,
                path: Some(path.clone()),
                git: None,
                rev: None,
                package: None,
            },
        )
        .expect("path provider should be selected explicitly");
    }
    let mut output = Transcript::default();
    let providers = SystemProviderValidator::new();

    super::Preparation {
        project_root: &project_root,
        lock: &SystemCargo,
        providers: &providers,
        progress: Progress::Visible(&mut output),
    }
    .prepare("standalone_fixture".to_owned(), &SystemCargo)
    .expect("selected providers should prepare through the generated runner");

    let initial = output.take();
    assert_eq!(
        initial
            .lines()
            .filter(|line| line.starts_with("geam: "))
            .collect::<Vec<_>>(),
        [
            format!("geam: Preparing standalone_fixture in {project_root}"),
            "geam: Checking Gleam source for standalone_fixture".to_owned(),
            "geam: Resolving provider geam-catalog for catalog 1.0.0".to_owned(),
            "geam: Resolving provider geam-counter for counter 1.0.0".to_owned(),
            "geam: Checking standalone runner for standalone_fixture".to_owned(),
            "geam: Prepared standalone_fixture".to_owned(),
        ],
    );

    let manifest = fs::read_to_string(project_root.join("Cargo.toml"))
        .expect("managed manifest should be readable");
    assert_eq!(
        manifest,
        format!(
            concat!(
                "# Managed by Geam. Use `geam provider` commands to change providers.\n\n",
                "[package]\n",
                "name = \"standalone_fixture-geam-runner\"\n",
                "version = \"0.0.0\"\n",
                "edition = \"2024\"\n",
                "publish = false\n\n",
                "[package.metadata.geam.runner]\n",
                "schema = 1\n\n",
                "[[bin]]\n",
                "name = \"geam-runner\"\n",
                "path = \"build/geam/runner.rs\"\n\n",
                "[dependencies]\n",
                "geam = {{ version = \"={}\", default-features = false, features = [\"builtins\"] }}\n",
                "toml = \"0.9\"\n",
                "geam_provider_catalog = {{ package = \"geam-catalog\", path = {} }}\n",
                "geam_provider_counter = {{ package = \"geam-counter\", path = {} }}\n\n",
                "[workspace]\n",
                "resolver = \"3\"\n",
            ),
            env!("CARGO_PKG_VERSION"),
            toml::Value::String(catalog_path.to_string()).to_string(),
            toml::Value::String(counter_path.to_string()).to_string(),
        ),
    );
    let lock = fs::read(project_root.join("Cargo.lock")).expect("root lock should be readable");
    assert_locked_provider_aliases(&project_root);
    assert_eq!(
        cargo_locks(&project_root),
        [project_root.join("Cargo.lock")]
    );
    assert!(!project_root.join("build/geam/provider-candidate").exists());

    let runner = fs::read_to_string(project_root.join("build/geam/runner.rs"))
        .expect("runner source should be readable");
    let catalog = runner
        .find("geam_provider_catalog::Component")
        .expect("catalog component should be generated");
    let counter = runner
        .find("geam_provider_counter::Component")
        .expect("counter component should be generated");
    assert!(catalog < counter);
    super::Preparation {
        project_root: &project_root,
        lock: &SystemCargo,
        providers: &providers,
        progress: Progress::Visible(&mut output),
    }
    .prepare("standalone_fixture".to_owned(), &SystemCargo)
    .expect("selected providers should prepare repeatedly");
    let repeated = output.take();
    assert_eq!(
        repeated
            .lines()
            .filter(|line| line.starts_with("geam: "))
            .collect::<Vec<_>>(),
        [
            format!("geam: Preparing standalone_fixture in {project_root}"),
            "geam: Checking Gleam source for standalone_fixture".to_owned(),
            "geam: Resolving provider geam-catalog for catalog 1.0.0".to_owned(),
            "geam: Resolving provider geam-counter for counter 1.0.0".to_owned(),
            "geam: Checking standalone runner for standalone_fixture".to_owned(),
            "geam: Prepared standalone_fixture".to_owned(),
        ],
    );
    assert_eq!(
        fs::read_to_string(project_root.join("Cargo.toml"))
            .expect("managed manifest should remain readable"),
        manifest,
    );
    assert_eq!(
        fs::read(project_root.join("Cargo.lock")).expect("root lock should remain readable"),
        lock,
    );
    assert_eq!(
        fs::read_to_string(project_root.join("build/geam/runner.rs"))
            .expect("runner source should remain readable"),
        runner,
    );
    super::Preparation {
        project_root: &project_root,
        lock: &SystemCargo,
        providers: &providers,
        progress: Progress::Visible(&mut output),
    }
    .run(
        &project_root,
        "standalone_fixture".to_owned(),
        vec![
            "catalog=config/catalog.toml".to_owned(),
            "counter=config/counter.toml".to_owned(),
        ],
        &SystemCargo,
    )
    .expect("selected providers should execute through the generated runner");
    let running = output.take();
    assert_eq!(
        running
            .lines()
            .filter(|line| line.starts_with("geam: "))
            .collect::<Vec<_>>(),
        [
            format!("geam: Preparing standalone_fixture in {project_root}"),
            "geam: Checking Gleam source for standalone_fixture".to_owned(),
            "geam: Resolving provider geam-catalog for catalog 1.0.0".to_owned(),
            "geam: Resolving provider geam-counter for counter 1.0.0".to_owned(),
            "geam: Starting standalone runner for standalone_fixture".to_owned(),
        ],
    );
    assert_eq!(
        cargo_locks(&project_root),
        [project_root.join("Cargo.lock")]
    );
}

#[derive(Clone, Default)]
struct Transcript(Arc<Mutex<Vec<u8>>>);

impl Transcript {
    fn take(&self) -> String {
        let bytes = std::mem::take(&mut *self.0.lock().expect("transcript should be available"));
        String::from_utf8(bytes).expect("fixture transcript should be UTF-8")
    }
}

impl Write for Transcript {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        self.0
            .lock()
            .expect("transcript should be available")
            .extend_from_slice(bytes);
        Ok(bytes.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

fn assert_locked_provider_aliases(project_root: &Utf8Path) {
    let output = run_checked(
        std::process::Command::new("cargo")
            .arg("metadata")
            .arg("--format-version")
            .arg("1")
            .arg("--locked")
            .arg("--manifest-path")
            .arg(project_root.join("Cargo.toml"))
            .current_dir(project_root),
    )
    .expect("locked root metadata should resolve");
    let metadata = MetadataCommand::parse(String::from_utf8_lossy(&output.stdout))
        .expect("Cargo metadata should be valid");
    let resolve = metadata
        .resolve
        .expect("metadata should include resolution");
    let root = resolve.root.expect("metadata should identify its root");
    let root = resolve
        .nodes
        .iter()
        .find(|node| node.id == root)
        .expect("root resolution node should exist");
    let aliases = root
        .deps
        .iter()
        .filter(|dependency| dependency.name.starts_with("geam_provider_"))
        .map(|dependency| {
            let package = metadata
                .packages
                .iter()
                .find(|package| package.id == dependency.pkg)
                .expect("resolved provider package should exist");
            (
                dependency.name.as_str(),
                package.name.as_str(),
                package.version.clone(),
            )
        })
        .collect::<Vec<_>>();
    assert_eq!(
        aliases,
        [
            (
                "geam_provider_catalog",
                "geam-catalog",
                Version::new(1, 0, 0),
            ),
            (
                "geam_provider_counter",
                "geam-counter",
                Version::new(1, 0, 0),
            ),
        ],
    );
}

fn cargo_locks(root: &Utf8Path) -> Vec<Utf8PathBuf> {
    let mut locks = Vec::new();
    collect_cargo_locks(root, &mut locks);
    locks.sort();
    locks
}

fn collect_cargo_locks(directory: &Utf8Path, locks: &mut Vec<Utf8PathBuf>) {
    for entry in fs::read_dir(directory).expect("project tree should be readable") {
        let entry = entry.expect("project entry should be readable");
        let path = Utf8PathBuf::from_path_buf(entry.path())
            .expect("temporary project paths should be valid UTF-8");
        if path.is_dir() {
            collect_cargo_locks(&path, locks);
        } else if path.file_name() == Some("Cargo.lock") {
            locks.push(path);
        }
    }
}

fn standalone_fixture() -> TempDir {
    prepare_provider_dependencies();
    let fixture = tempdir().expect("temporary standalone fixture should be created");
    let project = fixture.path().join("project");
    fs::create_dir_all(project.join("src")).expect("project source should be created");
    fs::create_dir_all(project.join("config")).expect("provider config should be created");
    fs::create_dir_all(project.join(".cargo")).expect("Cargo config should be created");
    for package in ["catalog", "counter"] {
        copy_directory(
            &fixture_source().join("project/packages").join(package),
            &project.join("packages").join(package),
        );
    }
    for package in ["catalog-domain", "geam-catalog", "geam-counter"] {
        copy_directory(
            &fixture_source().join("providers").join(package),
            &fixture.path().join("providers").join(package),
        );
    }
    fs::write(
        project.join("gleam.toml"),
        r#"name = "standalone_fixture"
version = "0.0.0"
description = "Standalone explicit provider integration fixture"
licences = ["Apache-2.0"]

[dependencies]
catalog = { path = "packages/catalog" }
counter = { path = "packages/counter" }
"#,
    )
    .expect("project configuration should be written");
    fs::write(
        project.join("manifest.toml"),
        r#"packages = [
  { name = "catalog", version = "1.0.0", build_tools = ["gleam"], requirements = [], source = "local", path = "packages/catalog" },
  { name = "counter", version = "1.0.0", build_tools = ["gleam"], requirements = [], source = "local", path = "packages/counter" },
]

[requirements]
catalog = { path = "packages/catalog" }
counter = { path = "packages/counter" }
"#,
    )
    .expect("project manifest should be written");
    fs::write(
        project.join("src/standalone_fixture.gleam"),
        r#"import catalog.{Summary}
import counter

pub fn main() {
  let empty = catalog.new()
  let populated = catalog.insert(empty, "one", "alpha")
  let matching = catalog.insert(catalog.new(), "one", "alpha")
  assert empty != populated
  assert populated == matching

  let summary = catalog.summarize(populated, fn(value) { "callback:" <> value })
  let Summary(count, items) = summary
  assert count == 1 && items == ["callback:native:alpha"]

  let first = counter.next("count")
  let second = counter.next("count")
  assert first == "count:3" && second == "count:4"
}
"#,
    )
    .expect("project source should be written");
    fs::write(
        project.join("config/catalog.toml"),
        "prefix = \"native:\"\n",
    )
    .expect("catalog config should be written");
    fs::write(project.join("config/counter.toml"), "start = 3\n")
        .expect("counter config should be written");
    let geam_path = toml::Value::String(
        repository_root()
            .to_str()
            .expect("repository path should be valid UTF-8")
            .to_owned(),
    )
    .to_string();
    fs::write(
        project.join(".cargo/config.toml"),
        format!(
            "[patch.crates-io]\ngeam = {{ path = {geam_path} }}\ngeam-catalog = {{ path = \"../providers/geam-catalog\" }}\ngeam-counter = {{ path = \"../providers/geam-counter\" }}\nstandalone-catalog-domain = {{ path = \"../providers/catalog-domain\" }}\n\n[net]\noffline = true\n",
        ),
    )
    .expect("project Cargo patch should be written");
    fixture
}

fn fixture_source() -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/standalone_cli")
}

fn repository_root() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("CLI package should be directly inside the repository root")
}

fn prepare_provider_dependencies() {
    run_checked(
        Command::new("cargo")
            .arg("fetch")
            .arg("--locked")
            .arg("--config")
            .arg("net.offline=false")
            .current_dir(fixture_source().join("providers")),
    )
    .expect("locked standalone provider dependencies should fetch");
}

fn copy_directory(source: &Path, destination: &Path) {
    fs::create_dir_all(destination).expect("fixture directory should be created");
    for entry in fs::read_dir(source).expect("fixture directory should be readable") {
        let entry = entry.expect("fixture entry should be readable");
        let source = entry.path();
        let destination = destination.join(entry.file_name());
        if source.is_dir() {
            copy_directory(&source, &destination);
        } else {
            fs::copy(&source, &destination).expect("fixture file should be copied");
        }
    }
}

fn utf8_path(directory: &TempDir) -> Utf8PathBuf {
    Utf8PathBuf::from_path_buf(directory.path().to_path_buf())
        .expect("temporary path should be valid UTF-8")
}

fn canonical_utf8(path: &Utf8Path) -> Utf8PathBuf {
    Utf8PathBuf::from_path_buf(fs::canonicalize(path).expect("fixture path should canonicalize"))
        .expect("canonical fixture path should be valid UTF-8")
}
