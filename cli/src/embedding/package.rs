mod manifest;
mod project;

use super::identifier::RustIdentifier;
use crate::cargo::{CargoMetadataLoader, CargoMetadataMode, SystemCargoMetadata};
use crate::error::CliError;
use crate::progress::Progress;
use camino::Utf8Path;
use cargo_metadata::{DependencyKind, Metadata, Package, PackageId};
pub(super) use project::EmbeddingProject;
use std::collections::BTreeSet;

#[derive(Debug)]
pub(super) struct EmbeddingPackage {
    project: EmbeddingProject,
    geam_alias: RustIdentifier,
    geam_package_id: PackageId,
    geam_features: BTreeSet<String>,
    direct_dependencies: Vec<DirectDependency>,
}

#[derive(Debug, Clone)]
pub(super) struct DirectDependency {
    pub(super) alias: String,
    pub(super) package: Package,
    enabled_features: BTreeSet<String>,
    pub(super) geam_dependencies: Vec<ResolvedGeamDependency>,
}

#[derive(Debug, Clone)]
pub(super) struct ResolvedGeamDependency {
    pub(super) alias: String,
    pub(super) package_id: PackageId,
}

impl EmbeddingPackage {
    pub(super) fn load(current_directory: &Utf8Path) -> Result<Self, CliError> {
        Self::load_with(current_directory, &SystemCargoMetadata)
    }

    fn load_with(
        current_directory: &Utf8Path,
        loader: &dyn CargoMetadataLoader,
    ) -> Result<Self, CliError> {
        let project = EmbeddingProject::load_with(current_directory, loader)?;
        project.validate_gleam_config()?;
        Self::resolve_with(project, loader, CargoMetadataMode::Locked)
    }

    pub(super) fn resolve(project: EmbeddingProject) -> Result<Self, CliError> {
        Self::resolve_with(project, &SystemCargoMetadata, CargoMetadataMode::Resolve)
    }

    fn resolve_with(
        project: EmbeddingProject,
        loader: &dyn CargoMetadataLoader,
        mode: CargoMetadataMode,
    ) -> Result<Self, CliError> {
        let metadata = loader.load(
            &project.manifest.with_file_name(""),
            &project.manifest,
            mode,
            &mut Progress::Hidden,
        )?;
        let package = select_package(&metadata, &project.manifest)?;
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
        Ok(Self {
            project,
            geam_alias: geam.alias,
            geam_package_id: geam.package_id,
            geam_features: geam.enabled_features,
            direct_dependencies,
        })
    }

    pub(super) fn project_root(&self) -> &Utf8Path {
        &self.project.project_root
    }

    pub(super) fn project_path(&self) -> &Utf8Path {
        Utf8Path::new("gleam")
    }

    pub(super) fn root_module(&self) -> &str {
        &self.project.root_module
    }

    pub(super) fn geam_alias(&self) -> &RustIdentifier {
        &self.geam_alias
    }

    pub(super) fn manifest(&self) -> &Utf8Path {
        &self.project.manifest
    }

    pub(super) fn geam_package_id(&self) -> &PackageId {
        &self.geam_package_id
    }

    pub(super) fn require_geam_feature(
        &self,
        feature: &str,
        purpose: &str,
    ) -> Result<(), CliError> {
        if self.geam_features.contains(feature) {
            return Ok(());
        }
        Err(CliError::InvalidEmbeddingDependency {
            package: self.project.package_name.clone(),
            manifest: self.project.manifest.clone(),
            reason: format!(
                "enabled Geam feature `{feature}` is required {purpose}; run `geam embedding sync` to enable it on the direct Geam dependency",
            ),
        })
    }

    pub(super) fn direct_dependencies(&self) -> &[DirectDependency] {
        &self.direct_dependencies
    }

    pub(super) fn output_directory(&self) -> &Utf8Path {
        &self.project.output_directory
    }

    pub(super) fn output_path(&self) -> &Utf8Path {
        &self.project.output_path
    }
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
            reason: "the manifest is not a Cargo package; run from a workspace member directory"
                .to_owned(),
        }),
        _ => Err(CliError::EmbeddingPackageSelection {
            manifest: manifest.to_path_buf(),
            reason: "Cargo metadata returned more than one package for the manifest".to_owned(),
        }),
    }
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
        let dependency_node = resolve
            .nodes
            .iter()
            .find(|node| node.id == dependency.pkg)
            .ok_or_else(|| CliError::InvalidCargoMetadata {
                manifest: package.manifest_path.clone(),
                reason: format!(
                    "the resolve graph has no node for package {}",
                    dependency_package.name,
                ),
            })?;
        dependencies.push(DirectDependency {
            alias: dependency.name.clone(),
            package: dependency_package.clone(),
            enabled_features: dependency_node
                .features
                .iter()
                .map(|feature| feature.as_ref().to_owned())
                .collect(),
            geam_dependencies: Vec::new(),
        });
    }
    Ok(dependencies)
}

struct GeamDependency {
    alias: RustIdentifier,
    package_id: PackageId,
    enabled_features: BTreeSet<String>,
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
        enabled_features: dependency.enabled_features.clone(),
    })
}

#[cfg(test)]
mod tests {
    use super::EmbeddingPackage;
    use crate::cargo::{CargoMetadataLoader, CargoMetadataMode};
    use crate::error::CliError;
    use crate::progress::Progress;
    use camino::{Utf8Path, Utf8PathBuf};
    use cargo_metadata::{Metadata, MetadataCommand};
    use serde_json::{Value, json};
    use std::fs;
    use tempfile::{TempDir, tempdir};

    struct FixedMetadata {
        source: String,
    }

    struct FailedMetadata;

    struct ResolvedMetadata<'loader> {
        workspace: &'loader FixedMetadata,
        resolved: &'loader dyn CargoMetadataLoader,
    }

    impl CargoMetadataLoader for ResolvedMetadata<'_> {
        fn load(
            &self,
            current_directory: &Utf8Path,
            manifest: &Utf8Path,
            mode: CargoMetadataMode,
            progress: &mut Progress<'_>,
        ) -> Result<Metadata, CliError> {
            if mode == CargoMetadataMode::Workspace {
                self.workspace
                    .load(current_directory, manifest, mode, progress)
            } else {
                self.resolved
                    .load(current_directory, manifest, mode, progress)
            }
        }
    }

    impl CargoMetadataLoader for FixedMetadata {
        fn load(
            &self,
            _current_directory: &Utf8Path,
            _manifest: &Utf8Path,
            _mode: CargoMetadataMode,
            _progress: &mut Progress<'_>,
        ) -> Result<Metadata, CliError> {
            Ok(MetadataCommand::parse(&self.source)
                .expect("fixed Cargo metadata fixture should be valid"))
        }
    }

    impl CargoMetadataLoader for FailedMetadata {
        fn load(
            &self,
            _current_directory: &Utf8Path,
            manifest: &Utf8Path,
            _mode: CargoMetadataMode,
            _progress: &mut Progress<'_>,
        ) -> Result<Metadata, CliError> {
            Err(CliError::InvalidCargoMetadata {
                manifest: manifest.to_path_buf(),
                reason: "fixture metadata failure".to_owned(),
            })
        }
    }

    #[test]
    fn preserves_actual_geam_aliases_and_unrelated_dependencies() {
        let fixture = package_fixture();
        let application = fixture.root.join("application");
        let manifest = application.join("Cargo.toml");
        for alias in ["runtime", "geam"] {
            let loader = metadata(&manifest, json!({}), &[(alias, "geam-one", "normal")]);
            let package = EmbeddingPackage::load_with(&application, &loader)
                .expect("the actual Geam alias should be accepted");
            assert_eq!(package.geam_alias.as_str(), alias);
            assert_eq!(package.project_path(), Utf8Path::new("gleam"));
            assert_eq!(package.root_module(), "application");
            assert_eq!(
                package.output_path(),
                application.join("src/geam_bindings.rs")
            );
        }
        let mixed = metadata(
            &manifest,
            json!({}),
            &[
                ("other", "other-one", "normal"),
                ("runtime", "geam-one", "normal"),
            ],
        );
        let mut value = metadata_value(&mixed);
        value["packages"][1]["name"] = json!("other");
        let package = EmbeddingPackage::load_with(
            &application,
            &FixedMetadata {
                source: value.to_string(),
            },
        )
        .expect("unrelated dependencies should not become Geam aliases");
        assert_eq!(package.geam_alias.as_str(), "runtime");
    }

    #[test]
    fn propagates_cargo_metadata_loader_failures() {
        let fixture = package_fixture();
        let application = fixture.root.join("application");
        let error = EmbeddingPackage::load_with(&application, &FailedMetadata)
            .expect_err("Cargo metadata failure should propagate");
        assert!(matches!(
            error,
            CliError::InvalidCargoMetadata { manifest, reason }
                if manifest == application.join("Cargo.toml") && reason == "fixture metadata failure"
        ));

        let initial = metadata(
            &application.join("Cargo.toml"),
            json!({}),
            &[("geam", "geam-one", "normal")],
        );
        let error = EmbeddingPackage::load_with(
            &application,
            &ResolvedMetadata {
                workspace: &initial,
                resolved: &FailedMetadata,
            },
        )
        .expect_err("locked metadata failure should propagate after package inspection");
        assert!(matches!(
            error,
            CliError::InvalidCargoMetadata { manifest, reason }
                if manifest == application.join("Cargo.toml") && reason == "fixture metadata failure"
        ));
    }

    #[test]
    fn validates_configuration_before_resolution_and_rechecks_resolved_identity() {
        let fixture = package_fixture();
        let application = fixture.root.join("application");
        let manifest = application.join("Cargo.toml");
        let initial = metadata(&manifest, json!({}), &[("geam", "geam-one", "normal")]);
        let config = application.join("gleam/gleam.toml");
        fs::write(&config, "name = \"another_application\"\n")
            .expect("conflicting config should be written");
        let error = EmbeddingPackage::load_with(
            &application,
            &ResolvedMetadata {
                workspace: &initial,
                resolved: &FailedMetadata,
            },
        )
        .expect_err("config failure should precede locked metadata inspection");
        assert_eq!(
            error.to_string(),
            format!(
                "invalid Rust embedding project for package application at {manifest}: {config} declares Gleam package `another_application`; expected `application` from the Cargo package name"
            ),
        );

        fs::write(&config, "name = \"application\"\n").expect("matching config should be restored");
        let mut changed = metadata_value(&initial);
        changed["packages"][0]["manifest_path"] = json!(application.join("changed/Cargo.toml"));
        let error = EmbeddingPackage::load_with(
            &application,
            &ResolvedMetadata {
                workspace: &initial,
                resolved: &FixedMetadata {
                    source: changed.to_string(),
                },
            },
        )
        .expect_err("resolved metadata must still identify the selected application");
        assert_eq!(
            error.to_string(),
            format!(
                "cannot select an embedding package from {manifest}: the manifest is not a Cargo package; run from a workspace member directory"
            ),
        );
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
            let loader = metadata(&manifest, json!({}), &dependencies);
            let error = EmbeddingPackage::load_with(&fixture.root.join("application"), &loader)
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
        let base = metadata(&manifest, json!({}), &[("geam", "geam-one", "normal")]);

        let mut ambiguous = metadata_value(&base);
        let duplicate = ambiguous["packages"][0].clone();
        ambiguous["packages"]
            .as_array_mut()
            .expect("packages fixture should be an array")
            .push(duplicate);
        let error = load_metadata_error(&fixture, ambiguous);
        assert!(matches!(
            error,
            CliError::EmbeddingPackageSelection { reason, .. }
                if reason.contains("more than one package")
        ));

        let mut missing_resolve = metadata_value(&base);
        missing_resolve["resolve"] = Value::Null;
        let error = load_metadata_error(&fixture, missing_resolve);
        assert!(matches!(
            error,
            CliError::InvalidCargoMetadata { reason, .. }
                if reason == "the locked resolve graph is absent"
        ));

        let mut missing_node = metadata_value(&base);
        missing_node["resolve"]["nodes"] = json!([]);
        let error = load_metadata_error(&fixture, missing_node);
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
        let error = load_metadata_error(&fixture, missing_direct_dependency_node);
        assert!(matches!(
            error,
            CliError::InvalidCargoMetadata { reason, .. }
                if reason.contains("no node for package geam")
        ));

        let mut missing_nested_dependency = metadata_value(&base);
        let geam_node = missing_nested_dependency["resolve"]["nodes"]
            .as_array_mut()
            .expect("resolve nodes fixture should be an array")
            .iter_mut()
            .find(|node| node["id"] == "path+file:///geam-one#1.0.0")
            .expect("Geam resolve node should exist");
        geam_node["dependencies"] = json!(["path+file:///missing#1.0.0"]);
        geam_node["deps"] = json!([{
            "name": "missing",
            "pkg": "path+file:///missing#1.0.0",
            "dep_kinds": [{ "kind": "normal", "target": null }],
        }]);
        let error = load_metadata_error(&fixture, missing_nested_dependency);
        assert!(matches!(
            error,
            CliError::InvalidCargoMetadata { reason, .. }
                if reason.contains("references missing package")
        ));

        let mut missing_dependency = metadata_value(&base);
        missing_dependency["packages"]
            .as_array_mut()
            .expect("packages fixture should be an array")
            .retain(|package| package["name"] != "geam");
        let error = load_metadata_error(&fixture, missing_dependency);
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
        let loader = metadata(&manifest, json!({}), &[("self", "geam-one", "normal")]);

        let error = EmbeddingPackage::load_with(&fixture.root.join("application"), &loader)
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
        fs::create_dir_all(root.join("application/gleam"))
            .expect("Gleam directory should be created");
        fs::write(
            root.join("application/gleam/gleam.toml"),
            "name = \"application\"\n",
        )
        .expect("Gleam config should be written");
        PackageFixture {
            _directory: directory,
            root,
        }
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

    fn load_metadata_error(fixture: &PackageFixture, value: Value) -> CliError {
        EmbeddingPackage::load_with(
            &fixture.root.join("application"),
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
