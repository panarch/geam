use super::report;
use crate::error::CliError;
use crate::provider::registry::ProviderRegistry;
use crate::provider::{
    ProviderApproval, ProviderCandidate, ProviderDiscovery, RegistryProviderDiscovery,
    TerminalApproval,
};
use hexpm::version::Version as GleamVersion;
use std::collections::BTreeMap;
use std::io::{BufRead, Write};

pub(super) fn select_missing(
    missing: &BTreeMap<String, GleamVersion>,
    registry: &dyn ProviderRegistry,
    terminal: bool,
    input: &mut dyn BufRead,
    progress: &mut dyn Write,
) -> Result<Vec<ProviderCandidate>, CliError> {
    if missing.is_empty() {
        report(
            progress,
            format_args!("Native provider declarations unchanged"),
        )?;
        return Ok(Vec::new());
    }
    let discovery = RegistryProviderDiscovery::new(registry);
    let mut discovered = Vec::new();
    for (package, version) in missing {
        report(
            progress,
            format_args!("Discovering native provider for {package} {version}"),
        )?;
        let candidates = discovery.discover(package, version)?;
        discovered.push((package, version, candidates));
    }
    let mut approval = TerminalApproval::for_embedding(terminal, input, progress);
    let mut approved = Vec::new();
    for (package, version, candidates) in discovered {
        approved.push(approval.approve(package, version, None, &candidates)?);
    }
    Ok(approved)
}

#[cfg(test)]
mod tests {
    use super::select_missing;
    use crate::embedding::{check, package::EmbeddingProject, prepare_with_registry};
    use crate::error::CliError;
    use crate::project::read_existing_resolved_project;
    use crate::provider::registry::{ProviderRegistry, RegistryAccessError};
    use camino::Utf8PathBuf;
    use flate2::{Compression, write::GzEncoder};
    use hexpm::version::Version as GleamVersion;
    use sha2::{Digest, Sha256};
    use std::cell::RefCell;
    use std::collections::BTreeMap;
    use std::fs;
    use std::io::{self, Cursor, Write};
    use std::process::Command;
    use tempfile::{TempDir, tempdir};

    #[test]
    fn approves_resolves_generates_and_runs_only_required_native_dependencies() {
        let fixture = ProviderApplication::new();
        let registry = PackagedRegistry::new(&[("geam-words", &fixture.provider_manifest)]);
        let original =
            fs::read_to_string(fixture.root.join("Cargo.toml")).expect("application manifest");
        let mut input = Cursor::new(b"y\n");
        let mut progress = Vec::new();
        fixture
            .prepare(&registry, true, &mut input, &mut progress)
            .expect("approved preparation");
        assert_eq!(
            String::from_utf8(progress).expect("UTF-8 progress"),
            format!(
                concat!(
                    "geam: Preparing Gleam dependencies in {0}/gleam\n",
                    "geam: Resolving Cargo dependencies for {0}/Cargo.toml\n",
                    "geam: Discovering native provider for words 1.0.0\n",
                    "Gleam package words 1.0.0 requires native provider code.\n",
                    "Metadata compatibility is not an endorsement.\n",
                    "  1. geam-words 1.2.3 (Gleam >= 1.0.0 and < 2.0.0)\n",
                    "Approve geam-words 1.2.3? [y/N] ",
                    "geam: Resolving approved Cargo providers for {0}/Cargo.toml\n",
                    "geam: Updated {0}/src/geam_bindings.rs\n",
                ),
                fixture.root
            )
        );
        let manifest =
            fs::read_to_string(fixture.root.join("Cargo.toml")).expect("updated manifest");
        assert_eq!(manifest, original.replace("\n\n[patch.crates-io]", "\ngeam_provider_words = { package = \"geam-words\", version = \"=1.2.3\" }\n\n[patch.crates-io]"));
        let cargo_lock = fs::read(fixture.root.join("Cargo.lock")).expect("Cargo lock");
        let gleam_lock = fs::read(fixture.root.join("gleam/manifest.toml")).expect("Gleam lock");
        let generated =
            fs::read(fixture.root.join("src/geam_bindings.rs")).expect("generated bindings");
        let source = String::from_utf8(generated.clone()).expect("UTF-8 bindings");
        assert!(source.contains("geam_provider_words::Component"));
        assert!(!source.contains("provider_unused"));
        assert!(!source.contains("provider_fallback"));
        assert!(!fixture.root.join("native/built.marker").exists());
        assert_eq!(
            *registry.calls.borrow(),
            [
                "search:geam-words",
                "configuration",
                "index:geam-words",
                "download:https://fixture.invalid/geam-words/1.2.3/download",
            ]
        );

        let mut progress = Vec::new();
        fixture
            .prepare(&registry, false, &mut Cursor::new(b""), &mut progress)
            .expect("reuses approved provider noninteractively");
        assert_eq!(
            String::from_utf8(progress).expect("UTF-8 progress"),
            format!(
                concat!(
                    "geam: Preparing Gleam dependencies in {0}/gleam\n",
                    "geam: Resolving Cargo dependencies for {0}/Cargo.toml\n",
                    "geam: Native provider declarations unchanged\n",
                    "geam: Unchanged {0}/src/geam_bindings.rs\n",
                ),
                fixture.root
            )
        );
        check(&fixture.root).expect("prepared application checks without native execution");
        assert_eq!(registry.calls.borrow().len(), 4);
        assert!(!fixture.root.join("native/built.marker").exists());
        assert_eq!(
            fs::read_to_string(fixture.root.join("Cargo.toml")).expect("unchanged manifest"),
            manifest
        );
        assert_eq!(
            fs::read(fixture.root.join("Cargo.lock")).expect("unchanged Cargo lock"),
            cargo_lock
        );
        assert_eq!(
            fs::read(fixture.root.join("gleam/manifest.toml")).expect("unchanged Gleam lock"),
            gleam_lock
        );
        assert_eq!(
            fs::read(fixture.root.join("src/geam_bindings.rs")).expect("unchanged output"),
            generated
        );

        let mut command = Command::new("cargo");
        command
            .args(["run", "--quiet", "--locked", "--offline"])
            .current_dir(&fixture.root)
            .env(
                "CARGO_TARGET_DIR",
                fixture
                    .repository
                    .join("target/embedding-provider-acceptance"),
            );
        for variable in [
            "CARGO_ENCODED_RUSTFLAGS",
            "LLVM_PROFILE_FILE",
            "RUSTDOCFLAGS",
            "RUSTFLAGS",
        ] {
            command.env_remove(variable);
        }
        let output = command.output().expect("Cargo application should start");
        assert_eq!(
            (output.status.code(), output.stdout, output.stderr),
            (Some(0), b"<Geam>\n".to_vec(), Vec::new())
        );
        assert!(fixture.root.join("native/built.marker").is_file());
    }

    #[test]
    fn preserves_application_files_when_approval_or_registry_verification_fails() {
        let fixture = ProviderApplication::new();
        let manifest = fs::read(fixture.root.join("Cargo.toml")).expect("original manifest");
        let mut registry = PackagedRegistry::new(&[("geam-words", &fixture.provider_manifest)]);
        for (terminal, answer, expected) in [
            (
                false,
                b"y\n".as_slice(),
                "Gleam package words requires native provider approval; run Geam interactively or select it explicitly with `geam embedding sync`",
            ),
            (
                true,
                b"n\n".as_slice(),
                "provider selection for Gleam package words was cancelled; no provider selections were changed",
            ),
        ] {
            let mut input = Cursor::new(answer);
            let error = fixture
                .prepare(&registry, terminal, &mut input, &mut Vec::new())
                .expect_err("approval required");
            assert_eq!(error.to_string(), expected);
            assert_eq!(
                fs::read(fixture.root.join("Cargo.toml")).expect("preserved manifest"),
                manifest
            );
            assert!(!fixture.root.join("src/geam_bindings.rs").exists());
            assert!(!fixture.root.join("native/built.marker").exists());
        }
        registry
            .downloads
            .values_mut()
            .next()
            .expect("packaged archive")
            .push(0);
        let error = fixture
            .prepare(&registry, true, &mut Cursor::new(b"y\n"), &mut Vec::new())
            .expect_err("corrupt archive must not be approved");
        assert!(
            error.to_string().contains("archive checksum mismatch"),
            "{error}"
        );
        assert_eq!(
            fs::read(fixture.root.join("Cargo.toml")).expect("preserved manifest"),
            manifest
        );
        assert!(!fixture.root.join("src/geam_bindings.rs").exists());
        assert!(!fixture.root.join("native/built.marker").exists());

        registry.search = Some(br#"{"crates":[],"meta":{"total":0}}"#.to_vec());
        let error = fixture
            .prepare(&registry, true, &mut Cursor::new(b"y\n"), &mut Vec::new())
            .expect_err("no available provider");
        assert_eq!(
            error.to_string(),
            "no metadata-verified provider is available for Gleam package words 1.0.0: no matching crates were found"
        );
        assert_eq!(
            fs::read(fixture.root.join("Cargo.toml")).expect("preserved manifest"),
            manifest
        );
        assert!(!fixture.root.join("native/built.marker").exists());
    }

    #[test]
    fn reports_unchanged_declarations_and_preserves_progress_failures() {
        let manifest = "[package]\nname = 'geam-words'\nversion = '1.2.3'\n[package.metadata.geam.provider]\nschema = 1\ngleam-package = 'words'\ngleam-version = '>= 1.0.0 and < 2.0.0'\n";
        let registry = PackagedRegistry::new(&[("geam-words", manifest)]);
        let missing = BTreeMap::from([("words".to_owned(), GleamVersion::new(1, 0, 0))]);
        let mut input = Cursor::new(b"0\n");
        let mut progress = Vec::new();
        assert_eq!(
            select_missing(
                &BTreeMap::new(),
                &registry,
                false,
                &mut input,
                &mut progress
            )
            .expect("no missing providers"),
            []
        );
        assert_eq!(progress, b"geam: Native provider declarations unchanged\n");
        assert_eq!(input.position(), 0);
        assert!(registry.calls.borrow().is_empty());

        let directory = tempdir().expect("progress fixture");
        let path = directory.path().join("read-only.txt");
        fs::write(&path, "").expect("progress file");
        let mut closed = fs::File::open(path).expect("read-only handle");
        for requirements in [&BTreeMap::new(), &missing] {
            let error = select_missing(requirements, &registry, true, &mut input, &mut closed)
                .expect_err("closed progress stream");
            assert_eq!(error.to_string(), "failed to write embedding progress");
        }
        assert!(registry.calls.borrow().is_empty());
    }

    #[test]
    fn preserves_existing_declarations_and_revalidates_the_resolved_provider() {
        let fixture = ProviderApplication::new();
        let registry = PackagedRegistry::new(&[("geam-words", &fixture.provider_manifest)]);
        let manifest_path = fixture.root.join("Cargo.toml");
        let manifest = fs::read_to_string(&manifest_path).expect("application manifest");
        let collision = manifest.replace(
            "\n\n[patch.crates-io]",
            &format!(
                "\ngeam_provider_words = {{ package = \"geam-core\", path = {:?} }}\n\n[patch.crates-io]",
                fixture.repository.join("core")
            ),
        );
        fs::write(&manifest_path, &collision).expect("unrelated dependency with a reserved alias");
        let error = fixture
            .prepare(&registry, true, &mut Cursor::new(b"y\n"), &mut Vec::new())
            .expect_err("approval does not authorize replacing an unrelated declaration");
        assert_eq!(
            error.to_string(),
            format!(
                "invalid Rust embedding provider graph for package words at {manifest_path}: dependency alias `geam_provider_words` is already declared; no provider dependencies were added"
            )
        );
        assert_eq!(
            fs::read_to_string(&manifest_path).expect("preserved Cargo"),
            collision
        );
        assert!(!fixture.root.join("src/geam_bindings.rs").exists());
        assert!(!fixture.root.join("native/built.marker").exists());

        fs::write(&manifest_path, &manifest).expect("remove the fixture collision");
        fs::write(
            fixture.root.join("native/Cargo.toml"),
            fixture
                .provider_manifest
                .replace(">= 1.0.0 and < 2.0.0", ">= 2.0.0 and < 3.0.0"),
        )
        .expect("application patch resolves to incompatible provider metadata");
        let mut input = Cursor::new(b"y\n");
        let error = fixture
            .prepare(&registry, true, &mut input, &mut Vec::new())
            .expect_err("resolved provider must still match the source package");
        let expected = format!(
            "invalid Rust embedding provider graph for package words at {manifest_path}: provider crate geam-words does not support resolved Gleam version 1.0.0 (declared range >= 2.0.0 and < 3.0.0)"
        );
        assert_eq!(error.to_string(), expected);
        assert_eq!(input.position(), 2);
        assert!(!fixture.root.join("src/geam_bindings.rs").exists());
        assert!(!fixture.root.join("native/built.marker").exists());

        let declared = fs::read(&manifest_path).expect("approved declaration remains retryable");
        let calls = registry.calls.borrow().clone();
        let mut input = Cursor::new(b"y\n");
        let error = fixture
            .prepare(&registry, true, &mut input, &mut Vec::new())
            .expect_err("incompatible selection is not permission to replace it");
        assert_eq!(error.to_string(), expected);
        assert_eq!(input.position(), 0);
        assert_eq!(*registry.calls.borrow(), calls);
        assert_eq!(
            fs::read(&manifest_path).expect("preserved declaration"),
            declared
        );
    }

    #[test]
    fn does_not_insert_approved_dependencies_after_progress_output_closes() {
        struct ClosingProgress {
            remaining_flushes: usize,
        }
        impl Write for ClosingProgress {
            fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
                Ok(bytes.len())
            }

            fn flush(&mut self) -> io::Result<()> {
                if self.remaining_flushes == 0 {
                    Err(io::Error::other("closed after approval"))
                } else {
                    self.remaining_flushes -= 1;
                    Ok(())
                }
            }
        }
        let fixture = ProviderApplication::new();
        let registry = PackagedRegistry::new(&[("geam-words", &fixture.provider_manifest)]);
        let manifest = fs::read(fixture.root.join("Cargo.toml")).expect("application manifest");
        let mut input = Cursor::new(b"y\n");
        let error = fixture
            .prepare(
                &registry,
                true,
                &mut input,
                &mut ClosingProgress {
                    remaining_flushes: 4,
                },
            )
            .expect_err("the next phase cannot report progress");
        assert_eq!(error.to_string(), "failed to write embedding progress");
        assert_eq!(input.position(), 2);
        assert_eq!(
            fs::read(fixture.root.join("Cargo.toml")).expect("unchanged manifest"),
            manifest
        );
        assert!(!fixture.root.join("src/geam_bindings.rs").exists());
        assert!(!fixture.root.join("native/built.marker").exists());
    }

    #[test]
    fn returns_no_selections_when_a_later_provider_is_declined() {
        let words = "[package]\nname = 'geam-words'\nversion = '1.2.3'\n[package.metadata.geam.provider]\nschema = 1\ngleam-package = 'words'\ngleam-version = '>= 1.0.0'\n";
        let zebra = "[package]\nname = 'geam-zebra'\nversion = '1.2.3'\n[package.metadata.geam.provider]\nschema = 1\ngleam-package = 'zebra'\ngleam-version = '>= 1.0.0'\n";
        let registry = PackagedRegistry::new(&[("geam-words", words), ("geam-zebra", zebra)]);
        let missing = BTreeMap::from([
            ("words".to_owned(), GleamVersion::new(1, 0, 0)),
            ("zebra".to_owned(), GleamVersion::new(1, 0, 0)),
        ]);
        let mut progress = Vec::new();
        let error = select_missing(
            &missing,
            &registry,
            true,
            &mut Cursor::new(b"y\nn\n"),
            &mut progress,
        )
        .expect_err("all required approvals must succeed");
        assert_eq!(
            error.to_string(),
            "provider selection for Gleam package zebra was cancelled; no provider selections were changed"
        );
        assert_eq!(
            String::from_utf8(progress).expect("UTF-8 protocol"),
            concat!(
                "geam: Discovering native provider for words 1.0.0\n",
                "geam: Discovering native provider for zebra 1.0.0\n",
                "Gleam package words 1.0.0 requires native provider code.\n",
                "Metadata compatibility is not an endorsement.\n",
                "  1. geam-words 1.2.3 (Gleam >= 1.0.0)\n",
                "Approve geam-words 1.2.3? [y/N] ",
                "Gleam package zebra 1.0.0 requires native provider code.\n",
                "Metadata compatibility is not an endorsement.\n",
                "  1. geam-zebra 1.2.3 (Gleam >= 1.0.0)\n",
                "Approve geam-zebra 1.2.3? [y/N] ",
            )
        );
        assert_eq!(
            *registry.calls.borrow(),
            [
                "search:geam-words",
                "configuration",
                "index:geam-words",
                "download:https://fixture.invalid/geam-words/1.2.3/download",
                "search:geam-zebra",
                "configuration",
                "index:geam-zebra",
                "download:https://fixture.invalid/geam-zebra/1.2.3/download",
            ]
        );
    }

    struct ProviderApplication {
        _directory: TempDir,
        root: Utf8PathBuf,
        repository: Utf8PathBuf,
        provider_manifest: String,
    }

    impl ProviderApplication {
        fn new() -> Self {
            let directory = tempdir().expect("application fixture");
            let root = Utf8PathBuf::from_path_buf(
                fs::canonicalize(directory.path()).expect("canonical application root"),
            )
            .expect("UTF-8 root");
            let repository = Utf8PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .parent()
                .expect("workspace root")
                .to_path_buf();
            for path in ["src", "gleam/src", "native/src", ".cargo"] {
                fs::create_dir_all(root.join(path)).expect("fixture directory");
            }
            fs::write(root.join(".cargo/config.toml"), "[net]\noffline = true\n")
                .expect("fixture-only offline configuration");
            fs::write(root.join("Cargo.toml"), format!(r#"[package]
name = "provider-application"
version = "0.1.0"
edition = "2024"

# Keep the application alias and its explicit feature selection.
[dependencies]
runtime = {{ package = "geam", path = {repository:?}, default-features = false, features = ["embedding"] }}

[patch.crates-io]
geam = {{ path = {repository:?} }}
geam-words = {{ path = "native" }}

[workspace]
resolver = "3"
"#)).expect("application Cargo manifest");
            let provider_manifest = format!(
                r#"[package]
name = "geam-words"
version = "1.2.3"
edition = "2024"

[package.metadata.geam.provider]
schema = 1
gleam-package = "words"
gleam-version = ">= 1.0.0 and < 2.0.0"

[dependencies]
geam = {{ path = {repository:?}, default-features = false, features = ["provider"] }}
ecow = "0.2.6"
"#
            );
            fs::write(root.join("native/Cargo.toml"), &provider_manifest)
                .expect("provider Cargo manifest");
            fs::write(
                root.join("native/src/lib.rs"),
                r#"#[geam::provider(package = "words", modules = [words])]
pub struct Component;

#[geam::module(path = "words")]
mod words {
    use ecow::EcoString;

    #[geam::function]
    fn surround(value: EcoString) -> EcoString {
        format!("<{value}>").into()
    }
}
"#,
            )
            .expect("native provider source");
            fs::write(
                root.join("native/build.rs"),
                r#"fn main() {
    let root = std::env::var("CARGO_MANIFEST_DIR").expect("provider root");
    std::fs::write(format!("{root}/built.marker"), "built").expect("build marker");
}
"#,
            )
            .expect("native execution probe");
            fs::write(root.join("gleam/gleam.toml"), "name = 'provider_application'\nversion = '0.1.0'\n[dependencies]\nwords = { path = 'packages/words' }\nunused = { path = 'packages/unused' }\nfallback = { path = 'packages/fallback' }\n").expect("Gleam configuration");
            for (name, source) in [
                (
                    "words",
                    "@external(erlang, \"words_ffi\", \"surround\")\npub fn surround(value: String) -> String\n",
                ),
                (
                    "unused",
                    "@external(erlang, \"unused_ffi\", \"answer\")\npub fn answer() -> Int\n",
                ),
                (
                    "fallback",
                    "@external(erlang, \"fallback_ffi\", \"answer\")\npub fn answer() -> Int { 42 }\n",
                ),
            ] {
                let package = root.join("gleam/packages").join(name);
                fs::create_dir_all(package.join("src")).expect("Gleam dependency directory");
                fs::write(
                    package.join("gleam.toml"),
                    format!("name = '{name}'\nversion = '1.0.0'\n"),
                )
                .expect("Gleam dependency config");
                fs::write(package.join("src").join(format!("{name}.gleam")), source)
                    .expect("Gleam dependency source");
            }
            fs::write(root.join("gleam/src/provider_application.gleam"), "import words\nimport fallback\n\npub fn surround(value: String) -> String { words.surround(value) }\npub fn fallback_answer() -> Int { fallback.answer() }\n").expect("selected Gleam source");
            fs::write(
                root.join("src/main.rs"),
                r#"mod geam_bindings;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let program = geam_bindings::project()?.compile()?;
    let builder = runtime::embedding::HostedModuleBuilder::new(program)?;
    let (bindings, functions) = geam_bindings::bind(builder)?;
    let module = bindings.seal()?;
    let mut state = geam_bindings::RunStateInputs {
        words: runtime::HostProviderConfiguration::empty(),
    }.initialize()?;
    let mut echo = Vec::new();
    let value = module.call(&functions.surround, ("Geam".into(),), &mut state, &mut echo)?;
    let fallback = module.call(&functions.fallback_answer, (), &mut state, &mut echo)?;
    assert_eq!(fallback, 42.into());
    assert!(echo.is_empty());
    println!("{value}");
    Ok(())
}
"#,
            )
            .expect("application calls generated bindings");
            Self {
                _directory: directory,
                root,
                repository,
                provider_manifest,
            }
        }

        fn prepare(
            &self,
            registry: &dyn ProviderRegistry,
            terminal: bool,
            input: &mut dyn std::io::BufRead,
            progress: &mut dyn std::io::Write,
        ) -> Result<(), CliError> {
            let project = EmbeddingProject::load(&self.root).expect("valid application fixture");
            prepare_with_registry(
                project,
                read_existing_resolved_project,
                registry,
                terminal,
                input,
                progress,
            )
        }
    }

    struct PackagedRegistry {
        search: Option<Vec<u8>>,
        downloads: BTreeMap<String, Vec<u8>>,
        indexes: BTreeMap<String, Vec<u8>>,
        calls: RefCell<Vec<String>>,
    }

    impl PackagedRegistry {
        fn new(manifests: &[(&str, &str)]) -> Self {
            let mut downloads = BTreeMap::new();
            let mut indexes = BTreeMap::new();
            for (crate_name, manifest) in manifests {
                let encoder = GzEncoder::new(Vec::new(), Compression::default());
                let mut archive = tar::Builder::new(encoder);
                let mut header = tar::Header::new_gnu();
                header.set_mode(0o644);
                header.set_size(manifest.len() as u64);
                header.set_cksum();
                archive
                    .append_data(
                        &mut header,
                        format!("{crate_name}-1.2.3/Cargo.toml"),
                        manifest.as_bytes(),
                    )
                    .expect("packaged metadata");
                let archive = archive
                    .into_inner()
                    .expect("archive writer")
                    .finish()
                    .expect("gzip archive");
                let checksum = hex::encode(Sha256::digest(&archive));
                downloads.insert(
                    format!("https://fixture.invalid/{crate_name}/1.2.3/download"),
                    archive,
                );
                indexes.insert((*crate_name).to_owned(), format!("{}\n", serde_json::json!({"name":crate_name,"vers":"1.2.3","cksum":checksum,"yanked":false})).into_bytes());
            }
            Self {
                search: None,
                downloads,
                indexes,
                calls: RefCell::new(Vec::new()),
            }
        }
    }

    impl ProviderRegistry for PackagedRegistry {
        fn search(&self, query: &str) -> Result<Vec<u8>, RegistryAccessError> {
            self.calls.borrow_mut().push(format!("search:{query}"));
            Ok(self.search.clone().unwrap_or_else(|| {
                let crates = self
                    .indexes
                    .keys()
                    .map(|name| serde_json::json!({"id":name}))
                    .collect::<Vec<_>>();
                serde_json::to_vec(
                    &serde_json::json!({"crates":crates,"meta":{"total":crates.len()}}),
                )
                .expect("search fixture")
            }))
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
}
