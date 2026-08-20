use crate::error::CliError;
use crate::process::run_checked;
use crate::project::ResolvedProject;
use crate::provider::registry::{ProviderRegistry, RegistryAccessError};
use crate::provider::{ManagedProject, ProviderSelectionReconciler, TerminalApproval};
use crate::runner::SystemCargo;
use camino::{Utf8Path, Utf8PathBuf};
use cargo_metadata::MetadataCommand;
use flate2::{Compression, write::GzEncoder};
use semver::Version;
use sha2::{Digest, Sha256};
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::fs;
use std::io::Cursor;
use std::path::Path;
use tempfile::{TempDir, tempdir};

#[test]
fn discovers_approves_locks_builds_and_runs_registry_providers() {
    let fixture = standalone_fixture();
    let project_root = utf8_path(&fixture).join("project");
    let catalog = provider_archive("geam-catalog");
    let counter = provider_archive("geam-counter");
    let registry = FakeRegistry::new([catalog, counter]);
    let mut reconciler = RegistryReconciler::new(&registry, b"y\ny\n".to_vec());

    super::prepare_with(
        &project_root,
        "standalone_fixture".to_owned(),
        &SystemCargo,
        &SystemCargo,
        &mut reconciler,
    )
    .expect("discovered providers should prepare through the generated runner");

    let prompt =
        String::from_utf8(reconciler.prompt().to_vec()).expect("approval prompt should be UTF-8");
    assert_eq!(
        prompt,
        concat!(
            "Gleam package catalog 1.0.0 requires native provider code.\n",
            "Metadata compatibility is not an endorsement.\n",
            "  1. geam-catalog 1.0.0 (Gleam >= 1.0.0 and < 2.0.0)\n",
            "Approve geam-catalog 1.0.0? [y/N] ",
            "Gleam package counter 1.0.0 requires native provider code.\n",
            "Metadata compatibility is not an endorsement.\n",
            "  1. geam-counter 1.0.0 (Gleam >= 1.0.0 and < 2.0.0)\n",
            "Approve geam-counter 1.0.0? [y/N] ",
        ),
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
                "geam = \"={}\"\n",
                "toml = \"0.9\"\n",
                "geam_provider_catalog = {{ package = \"geam-catalog\", version = \"=1.0.0\" }}\n",
                "geam_provider_counter = {{ package = \"geam-counter\", version = \"=1.0.0\" }}\n\n",
                "[workspace]\n",
                "resolver = \"3\"\n",
            ),
            env!("CARGO_PKG_VERSION"),
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
    assert_eq!(
        registry.calls(),
        [
            "search:geam-catalog",
            "configuration",
            "index:geam-catalog",
            "download:https://fixture.invalid/geam-catalog/1.0.0/download",
            "search:geam-counter",
            "configuration",
            "index:geam-counter",
            "download:https://fixture.invalid/geam-counter/1.0.0/download",
        ],
    );

    super::prepare_with(
        &project_root,
        "standalone_fixture".to_owned(),
        &SystemCargo,
        &SystemCargo,
        &mut reconciler,
    )
    .expect("approved providers should prepare without rediscovery");
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
    assert_eq!(registry.calls().len(), 8);

    super::run_with(
        &project_root,
        &project_root,
        "standalone_fixture".to_owned(),
        vec![
            "catalog=config/catalog.toml".to_owned(),
            "counter=config/counter.toml".to_owned(),
        ],
        &SystemCargo,
        &SystemCargo,
        &mut reconciler,
    )
    .expect("approved providers should execute through the generated runner");
    assert_eq!(registry.calls().len(), 8);
    assert_eq!(
        cargo_locks(&project_root),
        [project_root.join("Cargo.lock")]
    );
}

struct RegistryReconciler<'registry> {
    registry: &'registry dyn ProviderRegistry,
    input: Cursor<Vec<u8>>,
    prompt: Vec<u8>,
}

impl<'registry> RegistryReconciler<'registry> {
    fn new(registry: &'registry dyn ProviderRegistry, input: Vec<u8>) -> Self {
        Self {
            registry,
            input: Cursor::new(input),
            prompt: Vec::new(),
        }
    }

    fn prompt(&self) -> &[u8] {
        &self.prompt
    }
}

impl ProviderSelectionReconciler for RegistryReconciler<'_> {
    fn reconcile(
        &mut self,
        project_root: &Utf8Path,
        project: &ResolvedProject,
        program: &geam::TypedProgram,
        managed: &mut ManagedProject,
    ) -> Result<(), CliError> {
        let mut approval = TerminalApproval::new(true, &mut self.input, &mut self.prompt);
        crate::provider::reconcile_registry(
            self.registry,
            &mut approval,
            project_root,
            project,
            program,
            managed,
        )
    }
}

struct ProviderArchive {
    crate_name: String,
    version: Version,
    bytes: Vec<u8>,
    checksum: String,
}

fn provider_archive(crate_name: &str) -> ProviderArchive {
    // Standalone CI verifies Cargo packaging; this unit path keeps registry
    // reconciliation independent of Cargo's shared download cache.
    let version = Version::new(1, 0, 0);
    let manifest = fs::read(
        fixture_source()
            .join("providers")
            .join(crate_name)
            .join("Cargo.toml"),
    )
    .expect("provider manifest should be readable");
    let encoder = GzEncoder::new(Vec::new(), Compression::default());
    let mut archive = tar::Builder::new(encoder);
    let mut header = tar::Header::new_gnu();
    header.set_mode(0o644);
    header.set_size(manifest.len() as u64);
    header.set_cksum();
    archive
        .append_data(
            &mut header,
            format!("{crate_name}-{version}/Cargo.toml"),
            manifest.as_slice(),
        )
        .expect("provider manifest should enter the archive");
    let encoder = archive
        .into_inner()
        .expect("provider archive should finish writing");
    let bytes = encoder
        .finish()
        .expect("provider archive should finish compressing");
    let checksum = hex::encode(Sha256::digest(&bytes));
    ProviderArchive {
        crate_name: crate_name.to_owned(),
        version,
        bytes,
        checksum,
    }
}

struct FakeRegistry {
    indexes: BTreeMap<String, Vec<u8>>,
    downloads: BTreeMap<String, Vec<u8>>,
    calls: RefCell<Vec<String>>,
}

impl FakeRegistry {
    fn new<const N: usize>(providers: [ProviderArchive; N]) -> Self {
        let mut indexes = BTreeMap::new();
        let mut downloads = BTreeMap::new();
        for provider in providers {
            let crate_name = provider.crate_name;
            let record = serde_json::json!({
                "name": crate_name,
                "vers": provider.version.to_string(),
                "cksum": provider.checksum,
                "yanked": false,
            });
            indexes.insert(crate_name.clone(), format!("{record}\n").into_bytes());
            downloads.insert(download_url(&crate_name), provider.bytes);
        }
        Self {
            indexes,
            downloads,
            calls: RefCell::new(Vec::new()),
        }
    }

    fn calls(&self) -> Vec<String> {
        self.calls.borrow().clone()
    }
}

impl ProviderRegistry for FakeRegistry {
    fn search(&self, query: &str) -> Result<Vec<u8>, RegistryAccessError> {
        self.calls.borrow_mut().push(format!("search:{query}"));
        Ok(serde_json::to_vec(&serde_json::json!({
            "crates": [{ "id": query }],
            "meta": { "total": 1 },
        }))
        .expect("search fixture should serialize"))
    }

    fn index(&self, crate_name: &str) -> Result<Vec<u8>, RegistryAccessError> {
        self.calls.borrow_mut().push(format!("index:{crate_name}"));
        Ok(self.indexes[crate_name].clone())
    }

    fn configuration(&self) -> Result<Vec<u8>, RegistryAccessError> {
        self.calls.borrow_mut().push("configuration".to_owned());
        Ok(br#"{"dl":"https://fixture.invalid/{crate}/{version}/download"}"#.to_vec())
    }

    fn download(&self, url: &str) -> Result<Vec<u8>, RegistryAccessError> {
        self.calls.borrow_mut().push(format!("download:{url}"));
        Ok(self.downloads[url].clone())
    }
}

fn download_url(crate_name: &str) -> String {
    format!("https://fixture.invalid/{crate_name}/1.0.0/download")
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
description = "Standalone registry integration fixture"
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
    let geam_path = toml::Value::String(env!("CARGO_MANIFEST_DIR").to_owned()).to_string();
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
