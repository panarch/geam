use super::select_package;
use crate::cargo::{
    CargoMetadataLoader, CargoMetadataMode, SystemCargoMetadata, canonical_manifest,
};
use crate::error::CliError;
use crate::project::read_package_config;
use camino::{Utf8Path, Utf8PathBuf};
use gleam_core::config::PackageConfig;
use serde_json::json;

#[derive(Debug)]
pub(in crate::embedding) struct EmbeddingProject {
    pub(super) package_name: String,
    pub(super) manifest: Utf8PathBuf,
    pub(super) project_root: Utf8PathBuf,
    pub(super) root_module: String,
    pub(super) output_directory: Utf8PathBuf,
    pub(super) output_path: Utf8PathBuf,
    pub(super) dependencies: Vec<cargo_metadata::Dependency>,
}

impl EmbeddingProject {
    pub(in crate::embedding) fn load(current_directory: &Utf8Path) -> Result<Self, CliError> {
        Self::load_with(current_directory, &SystemCargoMetadata)
    }

    pub(in crate::embedding) fn manifest(&self) -> &Utf8Path {
        &self.manifest
    }

    pub(in crate::embedding) fn project_root(&self) -> &Utf8Path {
        &self.project_root
    }

    pub(in crate::embedding) fn root_module(&self) -> &str {
        &self.root_module
    }

    pub(in crate::embedding) fn output_path(&self) -> &Utf8Path {
        &self.output_path
    }

    pub(in crate::embedding) fn prepare_features(&self, features: &[&str]) -> Result<(), CliError> {
        super::manifest::prepare_features(self, features)
    }

    pub(super) fn load_with(
        current_directory: &Utf8Path,
        loader: &dyn CargoMetadataLoader,
    ) -> Result<Self, CliError> {
        let manifest = find_manifest(current_directory)?;
        let metadata = loader.load(current_directory, &manifest, CargoMetadataMode::Workspace)?;
        let package = select_package(&metadata, &manifest)?;
        let package_name = package.name.to_string();
        if package
            .metadata
            .get("geam")
            .and_then(|geam| geam.get("embedding"))
            .is_some()
        {
            return Err(CliError::InvalidEmbeddingProject {
                package: package_name,
                manifest,
                reason: "remove obsolete [package.metadata.geam.embedding]; embedding uses gleam/ and the Cargo package name".to_owned(),
            });
        }
        let root_module = package_name.replace('-', "_");
        serde_json::from_value::<PackageConfig>(json!({ "name": root_module })).map_err(
            |error| CliError::InvalidEmbeddingProject {
                package: package_name.clone(),
                manifest: manifest.clone(),
                reason: format!("Cargo package name cannot name a Gleam package: {error}"),
            },
        )?;
        let root = manifest.with_file_name("");
        let output_directory = root.join("src");
        let output_path = output_directory.join("geam_bindings.rs");
        Ok(Self {
            package_name,
            manifest,
            project_root: root.join("gleam"),
            root_module,
            output_directory,
            output_path,
            dependencies: package.dependencies.clone(),
        })
    }

    pub(in crate::embedding) fn validate_gleam_config(&self) -> Result<(), CliError> {
        let config = read_package_config(&self.project_root)?;
        if config.name.as_str() != self.root_module {
            return Err(CliError::InvalidEmbeddingProject {
                package: self.package_name.clone(),
                manifest: self.manifest.clone(),
                reason: format!(
                    "{} declares Gleam package `{}`; expected `{}` from the Cargo package name",
                    self.project_root.join("gleam.toml"),
                    config.name,
                    self.root_module,
                ),
            });
        }
        Ok(())
    }
}

fn find_manifest(start: &Utf8Path) -> Result<Utf8PathBuf, CliError> {
    for directory in start.ancestors() {
        match canonical_manifest(directory.join("Cargo.toml")) {
            Err(CliError::FileRead { error, .. })
                if error.kind() == std::io::ErrorKind::NotFound => {}
            result => return result,
        }
    }
    Err(CliError::CargoManifestNotFound {
        start: start.to_path_buf(),
    })
}

#[cfg(test)]
mod tests {
    use super::EmbeddingProject;
    use crate::cargo::SystemCargoMetadata;
    use crate::error::CliError;
    use camino::Utf8PathBuf;
    use std::fs;
    use tempfile::{TempDir, tempdir};

    #[test]
    fn selects_the_cargo_name_before_dependency_resolution_or_gleam_initialization() {
        let fixture = ProjectFixture::new(
            r#"[package]
name = "inventory-app"
version = "0.1.0"

[dependencies]
geam-unresolved-embedding-fixture = "=99.0.0"

[workspace]
"#,
        );
        fs::create_dir_all(fixture.root.join("src/nested"))
            .expect("nested source directory should be created");
        let project =
            EmbeddingProject::load_with(&fixture.root.join("src/nested"), &SystemCargoMetadata)
                .expect("initial selection should not resolve dependencies");
        assert_eq!(project.package_name, "inventory-app");
        assert_eq!(project.root_module, "inventory_app");
        assert_eq!(project.project_root, fixture.root.join("gleam"));
        assert_eq!(project.output_directory, fixture.root.join("src"));
        assert_eq!(
            project.output_path,
            fixture.root.join("src/geam_bindings.rs")
        );
        assert!(!fixture.root.join("Cargo.lock").exists());
        assert!(!fixture.root.join("gleam").exists());

        let error = project
            .validate_gleam_config()
            .expect_err("missing config should fail");
        assert!(matches!(error, CliError::FileRead { path, error }
            if path == fixture.root.join("gleam/gleam.toml")
                && error.kind() == std::io::ErrorKind::NotFound));
    }

    #[test]
    fn validates_the_actual_gleam_package_name_and_preserves_project_bytes() {
        let fixture = ProjectFixture::new(
            "[package]\nname = \"inventory_app2\"\nversion = \"0.1.0\"\n[workspace]\n",
        );
        let project = EmbeddingProject::load_with(&fixture.root, &SystemCargoMetadata)
            .expect("an existing valid Gleam name should be unchanged");
        assert_eq!(project.root_module, "inventory_app2");
        fs::create_dir(&project.project_root).expect("Gleam directory should be created");
        let config = project.project_root.join("gleam.toml");
        fs::write(&config, "name = \"other\"\n").expect("mismatched config should be written");
        let error = project
            .validate_gleam_config()
            .expect_err("different Gleam name should fail");
        assert_eq!(
            error.to_string(),
            format!(
                "invalid Rust embedding project for package inventory_app2 at {}: {} declares Gleam package `other`; expected `inventory_app2` from the Cargo package name",
                fixture.root.join("Cargo.toml"),
                config,
            )
        );
        assert_eq!(
            fs::read_to_string(&config).expect("config should remain readable"),
            "name = \"other\"\n"
        );
        fs::write(&config, "invalid").expect("invalid config should be written");
        assert!(
            matches!(project.validate_gleam_config(), Err(CliError::InvalidToml { path, .. }) if path == config)
        );
        fs::write(&config, "name = \"inventory_app2\"\n")
            .expect("matching config should be written");
        project
            .validate_gleam_config()
            .expect("matching package should be accepted");
        assert_eq!(
            fs::read_to_string(&config).expect("config should remain readable"),
            "name = \"inventory_app2\"\n"
        );
    }

    #[test]
    fn rejects_cargo_names_that_gleam_cannot_use_without_inventing_a_name() {
        let fixture = ProjectFixture::new(
            "[package]\nname = \"Inventory\"\nversion = \"0.1.0\"\n[workspace]\n",
        );
        let error = EmbeddingProject::load_with(&fixture.root, &SystemCargoMetadata)
            .expect_err("uppercase Cargo name cannot name the Gleam package");
        assert_eq!(
            error.to_string(),
            format!(
                "invalid Rust embedding project for package Inventory at {}: Cargo package name cannot name a Gleam package: Package names may only contain lowercase letters, numbers, and underscores",
                fixture.root.join("Cargo.toml"),
            )
        );
        assert!(!fixture.root.join("gleam").exists());
    }

    #[test]
    fn rejects_obsolete_selector_metadata_before_resolving_or_writing_files() {
        let fixture = ProjectFixture::new(
            r#"[package]
name = "inventory"
version = "0.1.0"
[package.metadata.geam.embedding]
project = "another-project"
module = "another_module"
[workspace]
"#,
        );
        let before =
            fs::read(fixture.root.join("Cargo.toml")).expect("manifest should be readable");
        let error = EmbeddingProject::load_with(&fixture.root, &SystemCargoMetadata)
            .expect_err("obsolete selectors must not be silently ignored");
        assert_eq!(
            error.to_string(),
            format!(
                "invalid Rust embedding project for package inventory at {}: remove obsolete [package.metadata.geam.embedding]; embedding uses gleam/ and the Cargo package name",
                fixture.root.join("Cargo.toml"),
            )
        );
        assert_eq!(
            fs::read(fixture.root.join("Cargo.toml")).expect("manifest should remain readable"),
            before
        );
        assert!(!fixture.root.join("Cargo.lock").exists());
    }

    #[test]
    fn requires_a_member_directory_at_a_virtual_workspace_root() {
        let fixture =
            ProjectFixture::new("[workspace]\nmembers = [\"member\"]\nresolver = \"3\"\n");
        fs::create_dir_all(fixture.root.join("member/src")).expect("member should be created");
        fs::write(
            fixture.root.join("member/Cargo.toml"),
            "[package]\nname = \"inventory\"\nversion = \"0.1.0\"\n",
        )
        .expect("member manifest should be written");
        fs::write(fixture.root.join("member/src/lib.rs"), "")
            .expect("member source should be written");
        let error = EmbeddingProject::load_with(&fixture.root, &SystemCargoMetadata)
            .expect_err("virtual root must not pick a member implicitly");
        assert_eq!(
            error.to_string(),
            format!(
                "cannot select an embedding package from {}: the manifest is not a Cargo package; run from a workspace member directory",
                fixture.root.join("Cargo.toml"),
            )
        );
        let member =
            EmbeddingProject::load_with(&fixture.root.join("member/src"), &SystemCargoMetadata)
                .expect("member directory should select its own package");
        assert_eq!(member.manifest, fixture.root.join("member/Cargo.toml"));
        assert_eq!(member.project_root, fixture.root.join("member/gleam"));
    }

    #[cfg(unix)]
    #[test]
    fn preserves_manifest_lookup_failures_instead_of_skipping_them() {
        use std::os::unix::fs::symlink;

        let directory = tempdir().expect("temporary directory should be created");
        let root = Utf8PathBuf::from_path_buf(directory.path().to_path_buf())
            .expect("temporary path should be UTF-8");
        let manifest = root.join("Cargo.toml");
        symlink("Cargo.toml", &manifest).expect("cyclic manifest link should be created");
        let expected = fs::canonicalize(&manifest)
            .expect_err("cyclic manifest link should fail canonicalization");
        let error = EmbeddingProject::load_with(&root, &SystemCargoMetadata)
            .expect_err("lookup failure should be reported before Cargo runs");
        assert!(matches!(error, CliError::FileRead { path, error }
            if path == manifest && error.to_string() == expected.to_string()));
    }

    struct ProjectFixture {
        _directory: TempDir,
        root: Utf8PathBuf,
    }

    impl ProjectFixture {
        fn new(manifest: &str) -> Self {
            let directory = tempdir().expect("temporary directory should be created");
            let root = Utf8PathBuf::from_path_buf(
                fs::canonicalize(directory.path()).expect("temporary path should canonicalize"),
            )
            .expect("path should be UTF-8");
            fs::create_dir(root.join("src")).expect("source directory should be created");
            fs::write(root.join("src/lib.rs"), "").expect("Rust source should be written");
            fs::write(root.join("Cargo.toml"), manifest).expect("Cargo manifest should be written");
            Self {
                _directory: directory,
                root,
            }
        }
    }
}
