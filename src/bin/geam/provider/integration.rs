use super::ProviderSelectionReconciler;
use super::approval::TerminalApproval;
use super::manifest::{ManagedProject, ProviderSource};
use super::reconcile::{
    ProviderReconciler, RegistryProviderDiscovery, SystemApprovedProviderResolver,
};
use super::registry::{ProviderRegistry, RegistryAccessError};
use crate::error::CliError;
use crate::project::{compile_resolved_project, read_resolved_project};
use crate::runner::CargoLock;
use camino::{Utf8Path, Utf8PathBuf};
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
fn reconciles_packaged_candidates_into_registry_shaped_runner_inputs() {
    let fixture = standalone_fixture();
    let project_root = utf8_path(&fixture).join("project");
    let catalog = provider_archive("geam-catalog");
    let counter = provider_archive("geam-counter");
    let registry = FakeRegistry::new([catalog, counter]);
    let discovery = RegistryProviderDiscovery::new(&registry);
    let resolver = SystemApprovedProviderResolver;
    let mut input = Cursor::new(b"y\ny\n".as_slice());
    let mut prompt = Vec::new();
    let mut approval = TerminalApproval::new(true, &mut input, &mut prompt);
    let mut reconciler = ProviderReconciler::new(&resolver, &discovery, &mut approval);
    let project = read_resolved_project(&project_root).expect("fixture project should resolve");
    let typed = compile_resolved_project(&project_root, "standalone_fixture".to_owned())
        .expect("fixture project should compile");
    let mut managed = ManagedProject::load(&project_root, project.root_package())
        .expect("managed project should initialize");

    reconciler
        .reconcile(&project_root, &project, &typed, &mut managed)
        .expect("packaged provider candidates should be approved");

    for (package, crate_name) in [("catalog", "geam-catalog"), ("counter", "geam-counter")] {
        let selection = managed
            .provider(package)
            .expect("required package should have a selected provider");
        assert_eq!(selection.crate_name(), crate_name);
        assert_eq!(
            selection.source(),
            &ProviderSource::Registry {
                version: Version::new(1, 0, 0),
            },
        );
    }
    let prompt = String::from_utf8(prompt).expect("approval prompt should be UTF-8");
    let catalog_prompt = prompt
        .find("Gleam package catalog 1.0.0 requires native provider code")
        .expect("catalog approval should be presented");
    let counter_prompt = prompt
        .find("Gleam package counter 1.0.0 requires native provider code")
        .expect("counter approval should be presented");
    assert!(catalog_prompt < counter_prompt);
    assert!(
        prompt
            .matches("Metadata compatibility is not an endorsement.")
            .count()
            == 2
    );

    crate::runner::reconcile_source(&project_root, &managed.provider_aliases())
        .expect("runner source should be generated");
    let manifest_changed = managed.write().expect("managed manifest should be written");
    crate::runner::reconcile_lock(&project_root, manifest_changed, &RecordingCargo)
        .expect("registry-shaped dependencies should request a lockfile");

    let manifest = fs::read_to_string(project_root.join("Cargo.toml"))
        .expect("managed manifest should be readable");
    assert!(
        manifest.contains(
            "geam_provider_catalog = { package = \"geam-catalog\", version = \"=1.0.0\" }",
        )
    );
    assert!(
        manifest.contains(
            "geam_provider_counter = { package = \"geam-counter\", version = \"=1.0.0\" }",
        )
    );
    assert!(project_root.join("Cargo.lock").is_file());
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

struct RecordingCargo;

impl CargoLock for RecordingCargo {
    fn generate_lockfile(&self, project_root: &Utf8Path) -> Result<(), CliError> {
        fs::write(project_root.join("Cargo.lock"), "fixture lock\n")
            .expect("fixture lock should be written");
        Ok(())
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

fn standalone_fixture() -> TempDir {
    let fixture = tempdir().expect("temporary standalone fixture should be created");
    let project = fixture.path().join("project");
    fs::create_dir_all(project.join("src")).expect("project source should be created");
    for package in ["catalog", "counter"] {
        copy_directory(
            &fixture_source().join("project/packages").join(package),
            &project.join("packages").join(package),
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
        r#"import catalog
import counter

pub fn main() {
  let _ = catalog.new()
  counter.next("count")
}
"#,
    )
    .expect("project source should be written");
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
