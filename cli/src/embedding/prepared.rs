mod manifest;
mod provenance;

use super::package::EmbeddingPackage;
use crate::cargo::{CargoMetadataLoader, CargoMetadataMode, SystemCargoMetadata};
use crate::error::CliError;
use crate::process::run_checked;
use crate::progress::Progress;
use camino::Utf8Path;
use cargo_metadata::{Metadata, Package, PackageId, Resolve};
use std::fs;
use std::path::Path;
use std::process::Command;

pub(super) struct Preparation {
    pub(super) source: String,
    pub(super) artifact: Artifact,
}

pub(super) enum Artifact {
    Plain,
    Hosted,
}

impl Preparation {
    pub(super) fn generate(self, package: &EmbeddingPackage) -> Result<String, CliError> {
        let directory = temporary_workspace(&std::env::temp_dir(), package.output_path())?;
        self.generate_in(package, directory.path(), &SystemCargoMetadata)
    }

    fn generate_in(
        self,
        package: &EmbeddingPackage,
        root: &Path,
        loader: &dyn CargoMetadataLoader,
    ) -> Result<String, CliError> {
        let root = crate::project::into_utf8_path(root.to_owned())?;
        let manifest = root.join("Cargo.toml");
        write(&manifest, manifest::helper_manifest(package)?.as_bytes())?;
        write(&root.join("main.rs"), self.source.as_bytes())?;
        let lock = package.environment().workspace_root.join("Cargo.lock");
        let locked = fs::read(&lock).map_err(|error| CliError::FileRead { path: lock, error })?;
        write(&root.join("Cargo.lock"), &locked)?;
        let current = package.manifest().with_file_name("");
        let metadata = loader.load(
            &current,
            &manifest,
            CargoMetadataMode::Locked,
            &mut Progress::Hidden,
        )?;
        check_resolution(
            package.cargo_package(),
            &package.environment().resolve,
            &metadata,
        )?;
        let destination = root.join("program.rs");
        run_checked(
            Command::new("cargo")
                .arg("run")
                .arg("--quiet")
                .arg("--locked")
                .arg("--manifest-path")
                .arg(&manifest)
                .arg("--bin")
                .arg("geam-prepare")
                .arg("--")
                .arg(package.project_root())
                .arg(&destination)
                .env(
                    "CARGO_TARGET_DIR",
                    package
                        .environment()
                        .target_directory
                        .join("geam-embedding"),
                )
                .current_dir(&current),
        )?;
        let expression = fs::read_to_string(&destination).map_err(|error| CliError::FileRead {
            path: destination,
            error,
        })?;
        Ok(self.artifact.render(
            package.geam_alias().as_str(),
            &provenance::inputs(package, &metadata)?,
            &expression,
        ))
    }
}

impl Artifact {
    fn render(&self, alias: &str, inputs: &serde_json::Value, expression: &str) -> String {
        let artifact = match self {
            Self::Plain => "data::ModuleArtifact<std::convert::Infallible>",
            Self::Hosted => "data::HostedModuleArtifact",
        };
        let mut output = format!(
            "{}\nuse {alias}::__prepared_support as data;\n\n#[rustfmt::skip]\npub(super) static PROGRAM: {artifact} = {expression};\n\n// Preparation inputs:\n",
            super::GENERATED_HEADER,
        );
        for line in format!("{inputs:#}").lines() {
            output.push_str("// ");
            output.push_str(line);
            output.push('\n');
        }
        output
    }
}

fn temporary_workspace(parent: &Path, output: &Utf8Path) -> Result<tempfile::TempDir, CliError> {
    tempfile::Builder::new()
        .prefix("geam-embedding-")
        .tempdir_in(parent)
        .map_err(|error| CliError::FileWrite {
            path: output.to_owned(),
            error,
        })
}

fn check_resolution(
    package: &Package,
    expected: &Resolve,
    actual: &Metadata,
) -> Result<(), CliError> {
    let actual = actual
        .resolve
        .as_ref()
        .ok_or_else(|| invalid(package, "helper resolve graph is absent"))?;
    let root = actual
        .root
        .as_ref()
        .ok_or_else(|| invalid(package, "helper root is absent"))?;
    let original = &package.id;
    let original_id = |id: &PackageId| {
        if id == root {
            original.clone()
        } else {
            id.clone()
        }
    };
    for node in &actual.nodes {
        let id = original_id(&node.id);
        let expected = expected
            .nodes
            .iter()
            .find(|node| node.id == id)
            .ok_or_else(|| invalid(package, format!("helper selected an unlocked package {id}")))?;
        for dependency in &node.deps {
            let id = original_id(&dependency.pkg);
            if !expected.deps.iter().any(|expected| {
                expected.name == dependency.name
                    && expected.pkg == id
                    && dependency
                        .dep_kinds
                        .iter()
                        .all(|kind| expected.dep_kinds.contains(kind))
            }) {
                return Err(invalid(
                    package,
                    format!(
                        "helper dependency `{}` differs from the consumer lock",
                        dependency.name
                    ),
                ));
            }
        }
    }
    Ok(())
}

fn write(path: &Utf8Path, bytes: &[u8]) -> Result<(), CliError> {
    fs::write(path, bytes).map_err(|error| CliError::FileWrite {
        path: path.to_owned(),
        error,
    })
}

fn invalid(package: &Package, reason: impl Into<String>) -> CliError {
    CliError::InvalidEmbeddingDependency {
        package: package.name.to_string(),
        manifest: package.manifest_path.clone(),
        reason: reason.into(),
    }
}

#[cfg(test)]
mod tests {
    use super::{Artifact, Preparation, check_resolution, temporary_workspace, write};
    use crate::cargo::{CargoMetadataLoader, CargoMetadataMode, SystemCargoMetadata};
    use crate::embedding::package::EmbeddingPackage;
    use crate::error::CliError;
    use crate::progress::Progress;
    use camino::Utf8Path;
    use camino::Utf8PathBuf;
    use cargo_metadata::{Metadata, MetadataCommand, PackageId};
    use std::fs;

    #[test]
    fn renders_readable_artifacts_with_complete_line_commented_inputs() {
        let inputs = serde_json::json!({
            "cargo": [{ "name": "example", "features": ["embedding"] }],
            "source": "*/\n\"quoted\"\\path",
        });
        for (artifact, declaration) in [
            (
                Artifact::Plain,
                "data::ModuleArtifact<std::convert::Infallible>",
            ),
            (Artifact::Hosted, "data::HostedModuleArtifact"),
        ] {
            let generated =
                artifact.render("runtime", &inputs, "data::Example {\n    value: 42,\n}");
            assert_eq!(
                generated,
                r#"
// Generated by `geam embedding sync`. Do not edit.

use runtime::__prepared_support as data;

#[rustfmt::skip]
pub(super) static PROGRAM: <artifact> = data::Example {
    value: 42,
};

// Preparation inputs:
// {
//   "cargo": [
//     {
//       "features": [
//         "embedding"
//       ],
//       "name": "example"
//     }
//   ],
//   "source": "*/\n\"quoted\"\\path"
// }
"#
                .trim_start_matches('\n')
                .replace("<artifact>", declaration)
            );
        }
    }

    #[test]
    fn preserves_preparation_workspace_and_cargo_failures() {
        let (directory, package) = preparation_fixture();
        let root = package.manifest().with_file_name("");
        let gleam_lock = root.join("gleam/manifest.toml");
        let source =
            "fn main() { std::fs::write(std::env::args().nth(2).unwrap(), \"42\").unwrap(); }";
        let prepare = |root: &Utf8Path, loader: &dyn CargoMetadataLoader, source: &str| {
            Preparation {
                source: source.into(),
                artifact: Artifact::Plain,
            }
            .generate_in(&package, root.as_std_path(), loader)
        };
        let helper = temporary_workspace(directory.path(), package.output_path()).unwrap();
        let helper = Utf8PathBuf::from_path_buf(helper.path().to_owned()).unwrap();
        for file in ["Cargo.toml", "main.rs", "Cargo.lock"] {
            let blocked = helper.join(file);
            fs::create_dir(&blocked).unwrap();
            let error = prepare(&helper, &SystemCargoMetadata, source).unwrap_err();
            assert!(matches!(error, CliError::FileWrite { path, .. } if path == blocked));
            fs::remove_dir(blocked).unwrap();
            for file in ["Cargo.toml", "main.rs", "Cargo.lock"] {
                let path = helper.join(file);
                if path.is_file() {
                    fs::remove_file(path).unwrap();
                }
            }
        }
        let manifest = root.join("Cargo.toml");
        let original = fs::read(&manifest).unwrap();
        fs::remove_file(&manifest).unwrap();
        let error = prepare(&helper, &SystemCargoMetadata, source).unwrap_err();
        assert!(matches!(error, CliError::FileRead { path, .. } if path == manifest));
        fs::write(&manifest, original).unwrap();
        let lock = root.join("Cargo.lock");
        let original = fs::read(&lock).unwrap();
        fs::remove_file(&lock).unwrap();
        let error = prepare(&helper, &SystemCargoMetadata, source).unwrap_err();
        assert!(matches!(error, CliError::FileRead { path, .. } if path == lock));
        fs::write(&lock, original).unwrap();
        let error = prepare(&helper, &PreparationMetadata::Failure, source).unwrap_err();
        assert!(
            matches!(error, CliError::InvalidCargoMetadata { reason, .. } if reason == "metadata process failed")
        );
        let error = prepare(&helper, &PreparationMetadata::MissingGraph, source).unwrap_err();
        assert!(error.to_string().contains("helper resolve graph is absent"));
        let native_manifest = root.join("runtime/Cargo.toml");
        let original = fs::read(&native_manifest).unwrap();
        fs::remove_file(&native_manifest).unwrap();
        let error = prepare(&helper, &PreparationMetadata::MissingGraph, source).unwrap_err();
        assert!(
            matches!(error, CliError::ProcessFailure { command, .. } if command.starts_with("cargo metadata"))
        );
        fs::write(&native_manifest, original).unwrap();
        let error = prepare(&helper, &SystemCargoMetadata, "fn main() {}").unwrap_err();
        assert!(
            matches!(error, CliError::FileRead { path, .. } if path == helper.join("program.rs"))
        );
        let error = prepare(&helper, &PreparationMetadata::MissingProvenance, source).unwrap_err();
        assert!(matches!(error, CliError::FileRead { path, .. } if path == gleam_lock));
        fs::write(&gleam_lock, "packages = []\n[requirements]\n").unwrap();
        let generated = prepare(&helper, &SystemCargoMetadata, source).unwrap();
        assert!(generated.starts_with("// Generated by `geam embedding sync`. Do not edit.\n\nuse runtime::__prepared_support as data;\n\n#[rustfmt::skip]\npub(super) static PROGRAM: data::ModuleArtifact<std::convert::Infallible> = 42;\n\n// Preparation inputs:\n// {\n"));
        let error = temporary_workspace(root.join("missing").as_std_path(), package.output_path())
            .unwrap_err();
        assert!(matches!(error, CliError::FileWrite { path, .. } if path == package.output_path()));
    }

    #[test]
    fn reports_unavailable_temporary_directories() {
        const CHILD: &str = "GEAM_PREPARATION_TEMP_FIXTURE";
        const EXPECTED: &str = "GEAM_PREPARATION_TEMP_ERROR";
        if let Some(root) = std::env::var_os(CHILD) {
            let package = EmbeddingPackage::load(Utf8Path::new(root.to_str().unwrap())).unwrap();
            let error = Preparation {
                source: String::new(),
                artifact: Artifact::Plain,
            }
            .generate(&package)
            .unwrap_err();
            assert!(
                error
                    .to_string()
                    .starts_with(&std::env::var(EXPECTED).unwrap()),
                "{error}"
            );
            return;
        }
        let (directory, package) = preparation_fixture();
        let temporary = directory.path().join("missing");
        let output = std::process::Command::new(std::env::current_exe().unwrap())
            .args([
                "--exact",
                "embedding::prepared::tests::reports_unavailable_temporary_directories",
                "--nocapture",
            ])
            .env(CHILD, package.manifest().with_file_name(""))
            .env(EXPECTED, "failed to write")
            .env("TMPDIR", &temporary)
            .env("TMP", &temporary)
            .env("TEMP", &temporary)
            .output()
            .unwrap();
        assert!(output.status.success(), "{output:?}");
    }

    #[cfg(unix)]
    #[test]
    fn rejects_non_unicode_helper_paths_before_writing_inputs() {
        use std::os::unix::ffi::OsStringExt;
        let (_directory, package) = preparation_fixture();
        let path = std::path::PathBuf::from(std::ffi::OsString::from_vec(vec![0xff]));
        let error = Preparation {
            source: String::new(),
            artifact: Artifact::Plain,
        }
        .generate_in(&package, &path, &SystemCargoMetadata)
        .unwrap_err();
        assert!(matches!(error, CliError::NonUtf8Path(actual) if actual == path));
    }

    fn preparation_fixture() -> (tempfile::TempDir, EmbeddingPackage) {
        let directory = tempfile::tempdir().unwrap();
        let root = Utf8PathBuf::from_path_buf(fs::canonicalize(directory.path()).unwrap()).unwrap();
        for path in ["src", "runtime/src", "gleam/src"] {
            fs::create_dir_all(root.join(path)).unwrap();
        }
        fs::write(root.join("Cargo.toml"), "[package]\nname = 'preparation'\nversion = '1.0.0'\nedition = '2024'\n[dependencies]\nruntime = { package = 'geam', path = 'runtime', features = ['embedding'] }\n[workspace]\n").unwrap();
        fs::write(root.join("src/main.rs"), "fn main() {}\n").unwrap();
        fs::write(
            root.join("runtime/Cargo.toml"),
            "[package]\nname = 'geam'\nversion = '1.0.0'\n[features]\nembedding = []\n",
        )
        .unwrap();
        fs::write(root.join("runtime/src/lib.rs"), "").unwrap();
        fs::write(
            root.join("gleam/gleam.toml"),
            "name = 'preparation'\nversion = '1.0.0'\n",
        )
        .unwrap();
        fs::write(
            root.join("gleam/manifest.toml"),
            "packages = []\n[requirements]\n",
        )
        .unwrap();
        MetadataCommand::new()
            .manifest_path(root.join("Cargo.toml"))
            .exec()
            .unwrap();
        let package = EmbeddingPackage::load(&root).unwrap();
        (directory, package)
    }

    enum PreparationMetadata {
        Failure,
        MissingGraph,
        MissingProvenance,
    }

    impl CargoMetadataLoader for PreparationMetadata {
        fn load(
            &self,
            current: &Utf8Path,
            manifest: &Utf8Path,
            mode: CargoMetadataMode,
            progress: &mut Progress<'_>,
        ) -> Result<Metadata, CliError> {
            if matches!(self, Self::Failure) {
                return Err(CliError::InvalidCargoMetadata {
                    manifest: manifest.to_owned(),
                    reason: "metadata process failed".into(),
                });
            }
            let mut metadata = SystemCargoMetadata.load(current, manifest, mode, progress)?;
            if matches!(self, Self::MissingGraph) {
                metadata.resolve = None;
            } else {
                fs::remove_file(current.join("gleam/manifest.toml")).unwrap();
            }
            Ok(metadata)
        }
    }

    #[test]
    fn admits_only_the_consumers_locked_dependency_edges() {
        let directory = tempfile::tempdir().unwrap();
        let root = Utf8PathBuf::from_path_buf(directory.path().to_owned()).unwrap();
        fs::create_dir_all(root.join("src")).unwrap();
        fs::create_dir_all(root.join("native/src")).unwrap();
        write(&root.join("Cargo.toml"), b"[package]\nname = 'consumer'\nversion = '1.0.0'\nedition = '2024'\n[dependencies]\nservice = { package = 'native', path = 'native' }\n[workspace]\n").unwrap();
        write(&root.join("src/main.rs"), b"fn main() {}\n").unwrap();
        write(
            &root.join("native/Cargo.toml"),
            b"[package]\nname = 'native'\nversion = '1.0.0'\n",
        )
        .unwrap();
        write(&root.join("native/src/lib.rs"), b"").unwrap();
        let metadata = MetadataCommand::new()
            .manifest_path(root.join("Cargo.toml"))
            .exec()
            .unwrap();
        let package = metadata.root_package().unwrap();
        let expected = metadata.resolve.as_ref().unwrap();
        check_resolution(package, expected, &metadata).unwrap();
        let helper_id = PackageId {
            repr: "path+file:///temporary-helper#consumer@1.0.0".into(),
        };
        let mut relocated = metadata.clone();
        let resolve = relocated.resolve.as_mut().unwrap();
        resolve.root = Some(helper_id.clone());
        for node in &mut resolve.nodes {
            if node.id == package.id {
                node.id = helper_id.clone();
            }
        }
        check_resolution(package, expected, &relocated).unwrap();
        let mut missing = relocated.clone();
        missing.resolve = None;
        assert!(
            check_resolution(package, expected, &missing)
                .unwrap_err()
                .to_string()
                .contains("helper resolve graph is absent")
        );
        missing.resolve = Some(expected.clone());
        missing.resolve.as_mut().unwrap().root = None;
        assert!(
            check_resolution(package, expected, &missing)
                .unwrap_err()
                .to_string()
                .contains("helper root is absent")
        );
        let consumer = expected
            .nodes
            .iter()
            .position(|node| node.id == package.id)
            .unwrap();
        let mut changed = relocated.clone();
        changed.resolve.as_mut().unwrap().nodes[consumer].deps[0].name = "other_alias".into();
        assert!(
            check_resolution(package, expected, &changed)
                .unwrap_err()
                .to_string()
                .contains("helper dependency `other_alias` differs from the consumer lock")
        );
        let mut changed = relocated.clone();
        changed.resolve.as_mut().unwrap().nodes[consumer].deps[0].dep_kinds[0].kind =
            cargo_metadata::DependencyKind::Build;
        assert!(
            check_resolution(package, expected, &changed)
                .unwrap_err()
                .to_string()
                .contains("differs from the consumer lock")
        );
        let mut changed = relocated.clone();
        changed.resolve.as_mut().unwrap().nodes[consumer].deps[0].pkg = helper_id.clone();
        assert!(
            check_resolution(package, expected, &changed)
                .unwrap_err()
                .to_string()
                .contains("differs from the consumer lock")
        );
        let mut changed = relocated.clone();
        let native = expected
            .nodes
            .iter()
            .position(|node| node.id != package.id)
            .unwrap();
        changed.resolve.as_mut().unwrap().nodes[native].id = PackageId {
            repr: "registry+https://example.invalid/index#native@2.0.0".into(),
        };
        assert!(
            check_resolution(package, expected, &changed)
                .unwrap_err()
                .to_string()
                .contains("helper selected an unlocked package")
        );
        assert!(matches!(
            write(&root, b"not a file"),
            Err(crate::error::CliError::FileWrite { path, .. }) if path == root
        ));
    }
}
