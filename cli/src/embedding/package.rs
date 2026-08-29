use super::identifier::RustIdentifier;
use crate::cargo::{
    CargoMetadataLoader, CargoMetadataMode, SystemCargoMetadata, canonical_manifest,
};
use crate::error::CliError;
use camino::{Utf8Path, Utf8PathBuf};
use cargo_metadata::{DependencyKind, Metadata, Package, PackageId};
use serde::Deserialize;

const MANIFEST_FILE: &str = "Cargo.toml";

#[derive(Debug)]
pub(super) struct EmbeddingPackage {
    manifest: Utf8PathBuf,
    project_root: Utf8PathBuf,
    root_module: String,
    geam_alias: RustIdentifier,
    geam_package_id: PackageId,
    direct_dependencies: Vec<DirectDependency>,
    output_directory: Utf8PathBuf,
    output_path: Utf8PathBuf,
}

#[derive(Debug, Clone)]
pub(super) struct DirectDependency {
    pub(super) alias: String,
    pub(super) package: Package,
    pub(super) geam_dependencies: Vec<ResolvedGeamDependency>,
}

#[derive(Debug, Clone)]
pub(super) struct ResolvedGeamDependency {
    pub(super) alias: String,
    pub(super) package_id: PackageId,
}

impl EmbeddingPackage {
    pub(super) fn load(
        current_directory: &Utf8Path,
        requested_manifest: Option<Utf8PathBuf>,
    ) -> Result<Self, CliError> {
        Self::load_with(current_directory, requested_manifest, &SystemCargoMetadata)
    }

    fn load_with(
        current_directory: &Utf8Path,
        requested_manifest: Option<Utf8PathBuf>,
        loader: &dyn CargoMetadataLoader,
    ) -> Result<Self, CliError> {
        let manifest = select_manifest(current_directory, requested_manifest)?;
        let metadata = loader.load(current_directory, &manifest, CargoMetadataMode::Locked)?;
        let package = select_package(&metadata, &manifest)?;
        let package_name = package.name.to_string();
        let package_root = manifest.with_file_name("");
        let embedding = embedding_metadata(package)?;
        let project = Utf8Path::new(&embedding.project);
        if embedding.project.is_empty() || project.is_absolute() {
            return Err(CliError::InvalidEmbeddingMetadata {
                package: package_name,
                manifest,
                reason: "`project` must be a non-empty relative path".to_owned(),
            });
        }
        if embedding.module.is_empty() {
            return Err(CliError::InvalidEmbeddingMetadata {
                package: package.name.to_string(),
                manifest,
                reason: "`module` must be non-empty".to_owned(),
            });
        }
        let mut direct_dependencies = direct_normal_dependencies(&metadata, package)?;
        for dependency in &mut direct_dependencies {
            dependency.geam_dependencies =
                direct_normal_dependencies(&metadata, &dependency.package)?
                    .into_iter()
                    .filter(|dependency| dependency.package.name == "geam")
                    .map(|dependency| ResolvedGeamDependency {
                        alias: dependency.alias,
                        package_id: dependency.package.id,
                    })
                    .collect();
        }
        let geam = select_geam_dependency(package, &direct_dependencies)?;
        let output_directory = package_root.join("src");
        let output_path = output_directory.join("geam_bindings.rs");

        Ok(Self {
            manifest,
            project_root: package_root.join(project),
            root_module: embedding.module,
            geam_alias: geam.alias,
            geam_package_id: geam.package_id,
            direct_dependencies,
            output_directory,
            output_path,
        })
    }

    pub(super) fn project_root(&self) -> &Utf8Path {
        &self.project_root
    }

    pub(super) fn root_module(&self) -> &str {
        &self.root_module
    }

    pub(super) fn geam_alias(&self) -> &RustIdentifier {
        &self.geam_alias
    }

    pub(super) fn manifest(&self) -> &Utf8Path {
        &self.manifest
    }

    pub(super) fn geam_package_id(&self) -> &PackageId {
        &self.geam_package_id
    }

    pub(super) fn direct_dependencies(&self) -> &[DirectDependency] {
        &self.direct_dependencies
    }

    pub(super) fn output_directory(&self) -> &Utf8Path {
        &self.output_directory
    }

    pub(super) fn output_path(&self) -> &Utf8Path {
        &self.output_path
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct EmbeddingMetadata {
    project: String,
    module: String,
}

fn select_manifest(
    current_directory: &Utf8Path,
    requested_manifest: Option<Utf8PathBuf>,
) -> Result<Utf8PathBuf, CliError> {
    let path = match requested_manifest {
        Some(path) if path.is_absolute() => path,
        Some(path) => current_directory.join(path),
        None => current_directory
            .ancestors()
            .map(|directory| directory.join(MANIFEST_FILE))
            .find(|manifest| manifest.is_file())
            .ok_or_else(|| CliError::CargoManifestNotFound {
                start: current_directory.to_path_buf(),
            })?,
    };
    canonical_manifest(path)
}

fn select_package<'metadata>(
    metadata: &'metadata Metadata,
    manifest: &Utf8Path,
) -> Result<&'metadata Package, CliError> {
    let selected = metadata
        .packages
        .iter()
        .filter(|package| package.manifest_path == manifest)
        .collect::<Vec<_>>();
    match selected.as_slice() {
        [package] => Ok(*package),
        [] => Err(CliError::EmbeddingPackageSelection {
            manifest: manifest.to_path_buf(),
            reason: "the manifest is not a Cargo package; select a workspace member manifest"
                .to_owned(),
        }),
        _ => Err(CliError::EmbeddingPackageSelection {
            manifest: manifest.to_path_buf(),
            reason: "Cargo metadata returned more than one package for the manifest".to_owned(),
        }),
    }
}

fn embedding_metadata(package: &Package) -> Result<EmbeddingMetadata, CliError> {
    let manifest = package.manifest_path.clone();
    let package_name = package.name.to_string();
    let table = package
        .metadata
        .get("geam")
        .and_then(|geam| geam.get("embedding"))
        .cloned()
        .ok_or_else(|| CliError::InvalidEmbeddingMetadata {
            package: package_name.clone(),
            manifest: manifest.clone(),
            reason: "missing [package.metadata.geam.embedding] table".to_owned(),
        })?;
    serde_json::from_value(table).map_err(|error| CliError::InvalidEmbeddingMetadata {
        package: package_name,
        manifest,
        reason: error.to_string(),
    })
}

fn direct_normal_dependencies(
    metadata: &Metadata,
    package: &Package,
) -> Result<Vec<DirectDependency>, CliError> {
    let resolve = metadata
        .resolve
        .as_ref()
        .ok_or_else(|| CliError::InvalidCargoMetadata {
            manifest: package.manifest_path.clone(),
            reason: "the locked resolve graph is absent".to_owned(),
        })?;
    let node = resolve
        .nodes
        .iter()
        .find(|node| node.id == package.id)
        .ok_or_else(|| CliError::InvalidCargoMetadata {
            manifest: package.manifest_path.clone(),
            reason: format!("the resolve graph has no node for package {}", package.name),
        })?;
    let mut dependencies = Vec::new();
    for dependency in &node.deps {
        if !dependency.dep_kinds.is_empty()
            && !dependency
                .dep_kinds
                .iter()
                .any(|kind| kind.kind == DependencyKind::Normal)
        {
            continue;
        }
        let dependency_package = metadata
            .packages
            .iter()
            .find(|candidate| candidate.id == dependency.pkg)
            .ok_or_else(|| CliError::InvalidCargoMetadata {
                manifest: package.manifest_path.clone(),
                reason: format!(
                    "the resolve graph references missing package {}",
                    dependency.pkg
                ),
            })?;
        dependencies.push(DirectDependency {
            alias: dependency.name.clone(),
            package: dependency_package.clone(),
            geam_dependencies: Vec::new(),
        });
    }
    Ok(dependencies)
}

struct GeamDependency {
    alias: RustIdentifier,
    package_id: PackageId,
}

fn select_geam_dependency(
    package: &Package,
    dependencies: &[DirectDependency],
) -> Result<GeamDependency, CliError> {
    let geam = dependencies
        .iter()
        .filter(|dependency| dependency.package.name == "geam")
        .collect::<Vec<_>>();
    let dependency = match geam.as_slice() {
        [dependency] => *dependency,
        [] => {
            return Err(CliError::InvalidEmbeddingDependency {
                package: package.name.to_string(),
                manifest: package.manifest_path.clone(),
                reason: "an enabled direct normal dependency on package `geam` is required"
                    .to_owned(),
            });
        }
        dependencies => {
            return Err(CliError::InvalidEmbeddingDependency {
                package: package.name.to_string(),
                manifest: package.manifest_path.clone(),
                reason: format!(
                    "multiple direct Geam dependencies resolve through aliases: {}",
                    dependencies
                        .iter()
                        .map(|dependency| dependency.alias.as_str())
                        .collect::<Vec<_>>()
                        .join(", ")
                ),
            });
        }
    };
    let alias = dependency.alias.as_str();
    let alias = RustIdentifier::crate_alias(alias).map_err(|reason| {
        CliError::InvalidEmbeddingDependency {
            package: package.name.to_string(),
            manifest: package.manifest_path.clone(),
            reason: format!("Cargo alias `{alias}` is unusable in generated Rust: {reason}"),
        }
    })?;
    Ok(GeamDependency {
        alias,
        package_id: dependency.package.id.clone(),
    })
}

#[cfg(test)]
mod tests {
    use super::EmbeddingPackage;
    use crate::cargo::{CargoMetadataLoader, CargoMetadataMode};
    use crate::error::CliError;
    use camino::{Utf8Path, Utf8PathBuf};
    use cargo_metadata::{Metadata, MetadataCommand};
    use serde_json::{Value, json};
    use std::fs;
    use tempfile::{TempDir, tempdir};

    struct FixedMetadata {
        source: String,
    }

    struct FailedMetadata;

    impl CargoMetadataLoader for FixedMetadata {
        fn load(
            &self,
            _current_directory: &Utf8Path,
            _manifest: &Utf8Path,
            mode: CargoMetadataMode,
        ) -> Result<Metadata, CliError> {
            assert_eq!(mode, CargoMetadataMode::Locked);
            Ok(MetadataCommand::parse(&self.source)
                .expect("fixed Cargo metadata fixture should be valid"))
        }
    }

    impl CargoMetadataLoader for FailedMetadata {
        fn load(
            &self,
            _current_directory: &Utf8Path,
            manifest: &Utf8Path,
            mode: CargoMetadataMode,
        ) -> Result<Metadata, CliError> {
            assert_eq!(mode, CargoMetadataMode::Locked);
            Err(CliError::InvalidCargoMetadata {
                manifest: manifest.to_path_buf(),
                reason: "fixture metadata failure".to_owned(),
            })
        }
    }

    #[test]
    fn selects_nearest_and_explicit_packages_with_the_actual_geam_alias() {
        let fixture = package_fixture();
        let package = fixture.root.join("application");
        let manifest = package.join("Cargo.toml");
        let nested = package.join("src/nested");
        fs::create_dir_all(&nested).expect("nested package directory should be created");
        let renamed_alias = metadata(
            &manifest,
            embedding_metadata("gleam", "inventory_rules"),
            &[("runtime", "geam-one", "normal")],
        );
        let nearest = EmbeddingPackage::load_with(&nested, None, &renamed_alias)
            .expect("nearest package should be selected");
        assert_eq!(nearest.project_root, package.join("gleam"));
        assert_eq!(nearest.root_module, "inventory_rules");
        assert_eq!(nearest.geam_alias.as_str(), "runtime");
        assert_eq!(nearest.output_path, package.join("src/geam_bindings.rs"));

        let explicit = EmbeddingPackage::load_with(
            &fixture.root,
            Some("application/Cargo.toml".into()),
            &renamed_alias,
        )
        .expect("explicit member package should be selected");
        assert_eq!(explicit.project_root, package.join("gleam"));

        let default_alias = metadata(
            &manifest,
            embedding_metadata("gleam", "inventory_rules"),
            &[("geam", "geam-one", "normal")],
        );
        let package = EmbeddingPackage::load_with(&nested, None, &default_alias)
            .expect("default Geam alias should be accepted");
        assert_eq!(package.geam_alias.as_str(), "geam");

        let mixed = metadata(
            &manifest,
            embedding_metadata("gleam", "inventory_rules"),
            &[
                ("other", "other-one", "normal"),
                ("runtime", "geam-one", "normal"),
            ],
        );
        let mut mixed_value = metadata_value(&mixed);
        mixed_value["packages"][1]["name"] = json!("other");
        let package = EmbeddingPackage::load_with(
            &nested,
            None,
            &FixedMetadata {
                source: mixed_value.to_string(),
            },
        )
        .expect("non-Geam dependencies should not become embedding aliases");
        assert_eq!(package.geam_alias.as_str(), "runtime");
    }

    #[test]
    fn rejects_missing_and_virtual_package_manifests() {
        let directory = tempdir().expect("temporary directory should be created");
        let root = utf8_path(&directory);
        let error = EmbeddingPackage::load_with(
            &root,
            None,
            &FixedMetadata {
                source: String::new(),
            },
        )
        .expect_err("missing Cargo manifest should fail");
        assert!(matches!(
            error,
            CliError::CargoManifestNotFound { start } if start == root
        ));

        let fixture = package_fixture();
        let workspace_manifest = fixture.root.join("Cargo.toml");
        fs::write(
            &workspace_manifest,
            "[workspace]\nmembers = [\"application\"]\n",
        )
        .expect("virtual workspace manifest should be written");
        let member_manifest = fixture.root.join("application/Cargo.toml");
        let metadata = metadata(
            &member_manifest,
            embedding_metadata("gleam", "inventory_rules"),
            &[("geam", "geam-one", "normal")],
        );
        let error = EmbeddingPackage::load_with(&fixture.root, Some(workspace_manifest), &metadata)
            .expect_err("virtual workspace should not select a member");
        assert!(matches!(
            error,
            CliError::EmbeddingPackageSelection { manifest, reason }
                if manifest == fixture.root.join("Cargo.toml")
                    && reason.contains("workspace member manifest")
        ));
    }

    #[test]
    fn propagates_cargo_metadata_loader_failures() {
        let fixture = package_fixture();
        let manifest = fixture.root.join("application/Cargo.toml");

        let error =
            EmbeddingPackage::load_with(&fixture.root, Some(manifest.clone()), &FailedMetadata)
                .expect_err("Cargo metadata loader failure should propagate");
        assert!(matches!(
            error,
            CliError::InvalidCargoMetadata { manifest: path, reason }
                if path == manifest && reason == "fixture metadata failure"
        ));
    }

    #[test]
    fn validates_exact_embedding_metadata_and_relative_project_paths() {
        let fixture = package_fixture();
        let manifest = fixture.root.join("application/Cargo.toml");
        for (metadata_value, expected) in [
            (json!({}), "missing [package.metadata.geam.embedding] table"),
            (
                embedding_metadata("gleam", "inventory_rules")
                    .as_object()
                    .map(|metadata| {
                        let mut metadata = metadata.clone();
                        metadata["geam"]["embedding"]["extra"] = json!(true);
                        Value::Object(metadata)
                    })
                    .expect("metadata fixture should be an object"),
                "unknown field `extra`",
            ),
            (
                embedding_metadata("/absolute/gleam", "inventory_rules"),
                "`project` must be a non-empty relative path",
            ),
            (
                embedding_metadata("gleam", ""),
                "`module` must be non-empty",
            ),
        ] {
            let loader = metadata(&manifest, metadata_value, &[("geam", "geam-one", "normal")]);
            let error = EmbeddingPackage::load_with(&fixture.root, Some(manifest.clone()), &loader)
                .expect_err("invalid embedding metadata should fail");
            assert!(matches!(
                error,
                CliError::InvalidEmbeddingMetadata { package, manifest: path, reason }
                    if package == "application" && path == manifest && reason.contains(expected)
            ));
        }
    }

    #[test]
    fn requires_one_enabled_direct_normal_geam_dependency() {
        let fixture = package_fixture();
        let manifest = fixture.root.join("application/Cargo.toml");
        for (dependencies, expected) in [
            (Vec::new(), "an enabled direct normal dependency"),
            (
                vec![
                    ("first", "geam-one", "normal"),
                    ("second", "geam-two", "normal"),
                ],
                "multiple direct Geam dependencies",
            ),
            (
                vec![("geam", "geam-one", "dev")],
                "an enabled direct normal dependency",
            ),
        ] {
            let loader = metadata(
                &manifest,
                embedding_metadata("gleam", "inventory_rules"),
                &dependencies,
            );
            let error = EmbeddingPackage::load_with(&fixture.root, Some(manifest.clone()), &loader)
                .expect_err("invalid Geam dependency graph should fail");
            assert!(matches!(
                error,
                CliError::InvalidEmbeddingDependency { package, manifest: path, reason }
                    if package == "application" && path == manifest && reason.contains(expected)
            ));
        }
    }

    #[test]
    fn rejects_ambiguous_or_incomplete_cargo_metadata() {
        let fixture = package_fixture();
        let manifest = fixture.root.join("application/Cargo.toml");
        let base = metadata(
            &manifest,
            embedding_metadata("gleam", "inventory_rules"),
            &[("geam", "geam-one", "normal")],
        );

        let mut ambiguous = metadata_value(&base);
        let duplicate = ambiguous["packages"][0].clone();
        ambiguous["packages"]
            .as_array_mut()
            .expect("packages fixture should be an array")
            .push(duplicate);
        let error = load_metadata_error(&fixture, &manifest, ambiguous);
        assert!(matches!(
            error,
            CliError::EmbeddingPackageSelection { reason, .. }
                if reason.contains("more than one package")
        ));

        let mut missing_resolve = metadata_value(&base);
        missing_resolve["resolve"] = Value::Null;
        let error = load_metadata_error(&fixture, &manifest, missing_resolve);
        assert!(matches!(
            error,
            CliError::InvalidCargoMetadata { reason, .. }
                if reason == "the locked resolve graph is absent"
        ));

        let mut missing_node = metadata_value(&base);
        missing_node["resolve"]["nodes"] = json!([]);
        let error = load_metadata_error(&fixture, &manifest, missing_node);
        assert!(matches!(
            error,
            CliError::InvalidCargoMetadata { reason, .. }
                if reason.contains("no node for package application")
        ));

        let mut missing_direct_dependency_node = metadata_value(&base);
        missing_direct_dependency_node["resolve"]["nodes"]
            .as_array_mut()
            .expect("resolve nodes fixture should be an array")
            .retain(|node| node["id"] == "path+file:///application#1.0.0");
        let error = load_metadata_error(&fixture, &manifest, missing_direct_dependency_node);
        assert!(matches!(
            error,
            CliError::InvalidCargoMetadata { reason, .. }
                if reason.contains("no node for package geam")
        ));

        let mut missing_dependency = metadata_value(&base);
        missing_dependency["packages"]
            .as_array_mut()
            .expect("packages fixture should be an array")
            .retain(|package| package["name"] != "geam");
        let error = load_metadata_error(&fixture, &manifest, missing_dependency);
        assert!(matches!(
            error,
            CliError::InvalidCargoMetadata { reason, .. }
                if reason.contains("references missing package")
        ));
    }

    #[test]
    fn rejects_geam_aliases_that_cannot_name_a_rust_crate() {
        let fixture = package_fixture();
        let manifest = fixture.root.join("application/Cargo.toml");
        let loader = metadata(
            &manifest,
            embedding_metadata("gleam", "inventory_rules"),
            &[("self", "geam-one", "normal")],
        );

        let error = EmbeddingPackage::load_with(&fixture.root, Some(manifest), &loader)
            .expect_err("unusable Geam alias should fail");
        assert!(matches!(
            error,
            CliError::InvalidEmbeddingDependency { reason, .. }
                if reason.contains("Cargo alias `self` is unusable")
        ));
    }

    struct PackageFixture {
        _directory: TempDir,
        root: Utf8PathBuf,
    }

    fn package_fixture() -> PackageFixture {
        let directory = tempdir().expect("temporary directory should be created");
        let root = utf8_path(&directory);
        fs::create_dir_all(root.join("application/src"))
            .expect("application source directory should be created");
        fs::write(
            root.join("application/Cargo.toml"),
            "[package]\nname = \"application\"\nversion = \"1.0.0\"\n",
        )
        .expect("application manifest should be written");
        PackageFixture {
            _directory: directory,
            root,
        }
    }

    fn embedding_metadata(project: &str, module: &str) -> Value {
        json!({
            "geam": {
                "embedding": {
                    "project": project,
                    "module": module,
                }
            }
        })
    }

    fn metadata(
        manifest: &Utf8Path,
        package_metadata: Value,
        dependencies: &[(&str, &str, &str)],
    ) -> FixedMetadata {
        let application_id = "path+file:///application#1.0.0";
        let mut packages = vec![package(
            "application",
            application_id,
            manifest,
            package_metadata,
        )];
        let mut node_dependencies = Vec::new();
        let mut nodes = Vec::new();
        for (alias, id, kind) in dependencies {
            let package_id = format!("path+file:///{id}#1.0.0");
            packages.push(package(
                "geam",
                &package_id,
                &manifest.with_file_name(format!("{id}.toml")),
                json!(null),
            ));
            node_dependencies.push(json!({
                "name": alias,
                "pkg": package_id,
                "dep_kinds": [{ "kind": kind, "target": null }],
            }));
            nodes.push(json!({
                "id": package_id,
                "dependencies": [],
                "deps": [],
                "features": [],
            }));
        }
        nodes.push(json!({
            "id": application_id,
            "dependencies": node_dependencies
                .iter()
                .map(|dependency| dependency["pkg"].clone())
                .collect::<Vec<_>>(),
            "deps": node_dependencies,
            "features": [],
        }));
        let root = manifest
            .parent()
            .expect("fixture manifest should have a parent");
        let source = json!({
            "packages": packages,
            "workspace_members": [application_id],
            "workspace_default_members": [application_id],
            "resolve": {
                "nodes": nodes,
                "root": application_id,
            },
            "target_directory": root.join("target"),
            "build_directory": root.join("target"),
            "version": 1,
            "workspace_root": root,
            "metadata": null,
        })
        .to_string();
        FixedMetadata { source }
    }

    fn metadata_value(metadata: &FixedMetadata) -> Value {
        serde_json::from_str(&metadata.source).expect("Cargo metadata fixture should be JSON")
    }

    fn load_metadata_error(
        fixture: &PackageFixture,
        manifest: &Utf8Path,
        value: Value,
    ) -> CliError {
        EmbeddingPackage::load_with(
            &fixture.root,
            Some(manifest.to_path_buf()),
            &FixedMetadata {
                source: value.to_string(),
            },
        )
        .expect_err("invalid Cargo metadata should fail")
    }

    fn package(name: &str, id: &str, manifest: &Utf8Path, metadata: Value) -> Value {
        json!({
            "name": name,
            "version": "1.0.0",
            "id": id,
            "license": null,
            "license_file": null,
            "description": null,
            "source": null,
            "dependencies": [],
            "targets": [],
            "features": {},
            "manifest_path": manifest,
            "categories": [],
            "keywords": [],
            "readme": null,
            "repository": null,
            "homepage": null,
            "documentation": null,
            "edition": "2024",
            "metadata": metadata,
            "links": null,
            "publish": null,
            "authors": [],
            "default_run": null,
            "rust_version": "1.96",
        })
    }

    fn utf8_path(directory: &TempDir) -> Utf8PathBuf {
        Utf8PathBuf::from_path_buf(
            fs::canonicalize(directory.path()).expect("temporary path should canonicalize"),
        )
        .expect("temporary path should be valid UTF-8")
    }
}
