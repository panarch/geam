mod approval;
mod discovery;
mod list;
mod manifest;
mod metadata;
mod reconcile;
pub(crate) mod registry;
mod resolution;

use crate::command::{AddProvider, RemoveProvider};
use crate::error::CliError;
use crate::progress::Progress;
use crate::project::{ResolvedProject, read_resolved_project};
pub(crate) use approval::{ProviderApproval, TerminalApproval};
use camino::Utf8Path;
pub(crate) use discovery::{ProviderDiscovery, RegistryProviderDiscovery};
pub(super) use manifest::ManagedProject;
use manifest::ProviderSelection;
pub(super) use metadata::ProviderMetadata;
pub(super) use reconcile::ProviderSelectionReconciler;
pub(crate) use registry::{CratesIoRegistry, ProviderCandidate};
use std::path::Path;

pub(super) struct SystemProviderReconciler<'io> {
    registry: registry::CratesIoRegistry,
    approval: TerminalApproval<'io>,
}

impl<'io> SystemProviderReconciler<'io> {
    pub(super) fn new(
        terminal: bool,
        reader: &'io mut dyn std::io::BufRead,
        writer: &'io mut dyn std::io::Write,
    ) -> Self {
        Self {
            registry: registry::CratesIoRegistry::default(),
            approval: TerminalApproval::new(terminal, reader, writer),
        }
    }
}

impl ProviderSelectionReconciler for SystemProviderReconciler<'_> {
    fn reconcile(
        &mut self,
        project_root: &Utf8Path,
        project: &ResolvedProject,
        program: &geam_core::TypedProgram,
        managed: &mut ManagedProject,
        progress: &mut Progress<'_>,
    ) -> Result<(), CliError> {
        reconcile_registry(
            &self.registry,
            &mut self.approval,
            project_root,
            project,
            program,
            managed,
            progress,
        )
    }
}

pub(crate) fn reconcile_registry(
    registry: &dyn registry::ProviderRegistry,
    approval: &mut TerminalApproval<'_>,
    project_root: &Utf8Path,
    project: &ResolvedProject,
    program: &geam_core::TypedProgram,
    managed: &mut ManagedProject,
    progress: &mut Progress<'_>,
) -> Result<(), CliError> {
    let discovery = RegistryProviderDiscovery::new(registry);
    let resolver = reconcile::SystemApprovedProviderResolver;
    reconcile::ProviderReconciler::new(&resolver, &discovery, approval).reconcile(
        project_root,
        project,
        program,
        managed,
        progress,
    )
}

pub(super) fn add(
    project_root: &Utf8Path,
    current_directory: &Path,
    command: AddProvider,
) -> Result<(), CliError> {
    add_with(
        project_root,
        current_directory,
        command,
        &crate::runner::SystemCargo,
    )
}

pub(super) fn list(project_root: &Utf8Path) -> Result<(), CliError> {
    list::write(project_root, &mut std::io::stdout().lock())
}

fn add_with(
    project_root: &Utf8Path,
    current_directory: &Path,
    command: AddProvider,
    cargo: &dyn crate::runner::CargoLock,
) -> Result<(), CliError> {
    let project = read_resolved_project(project_root)?;
    let mut managed = ManagedProject::load(project_root, project.root_package())?;
    managed.retain_packages(&project.package_names());
    let resolved = resolution::resolve(project_root, current_directory, command)?;
    let package = resolved.metadata.gleam_package();
    if is_built_in_package(package) {
        return Err(CliError::BuiltInProviderPackage {
            package: package.to_owned(),
        });
    }
    let version =
        project
            .package_version(package)
            .ok_or_else(|| CliError::MissingGleamPackage {
                package: package.to_owned(),
            })?;
    if !resolved.metadata.supports(version) {
        return Err(CliError::IncompatibleProvider {
            provider: resolved.metadata.crate_name().to_owned(),
            package: package.to_owned(),
            version: version.to_string(),
            range: resolved.metadata.gleam_range().to_string(),
        });
    }
    managed.insert(ProviderSelection::new(
        package.to_owned(),
        resolved.metadata.crate_name().to_owned(),
        resolved.source,
    ))?;
    crate::runner::reconcile_source(project_root, &managed.provider_aliases())?;
    let manifest_changed = managed.write()?;
    crate::runner::reconcile_lock(project_root, manifest_changed, cargo, &mut Progress::Hidden)?;
    Ok(())
}

pub(super) fn remove(project_root: &Utf8Path, command: RemoveProvider) -> Result<(), CliError> {
    remove_with(project_root, command, &crate::runner::SystemCargo)
}

fn remove_with(
    project_root: &Utf8Path,
    command: RemoveProvider,
    cargo: &dyn crate::runner::CargoLock,
) -> Result<(), CliError> {
    let project = read_resolved_project(project_root)?;
    let mut managed = ManagedProject::load(project_root, project.root_package())?;
    managed.remove(&command.gleam_package)?;
    managed.retain_packages(&project.package_names());
    crate::runner::reconcile_source(project_root, &managed.provider_aliases())?;
    let manifest_changed = managed.write()?;
    crate::runner::reconcile_lock(project_root, manifest_changed, cargo, &mut Progress::Hidden)?;
    Ok(())
}

pub(super) fn is_built_in_package(package: &str) -> bool {
    crate::builtin::BuiltInProvider::from_package(package).is_some()
}

#[cfg(test)]
mod tests {
    use super::{add_with, remove_with};
    use crate::command::{AddProvider, RemoveProvider};
    use crate::error::CliError;
    use crate::progress::Progress;
    use crate::runner::CargoLock;
    use camino::Utf8PathBuf;
    use std::fs;
    use tempfile::{TempDir, tempdir};

    struct TestCargo;

    impl CargoLock for TestCargo {
        fn generate_lockfile(
            &self,
            project_root: &camino::Utf8Path,
            _progress: &mut Progress<'_>,
        ) -> Result<(), CliError> {
            fs::write(project_root.join("Cargo.lock"), "fixture lock\n")
                .expect("fixture lock should be written");
            Ok(())
        }
    }

    struct FailingCargoLock;

    impl CargoLock for FailingCargoLock {
        fn generate_lockfile(
            &self,
            _project_root: &camino::Utf8Path,
            _progress: &mut Progress<'_>,
        ) -> Result<(), CliError> {
            Err(CliError::ProcessFailure {
                command: "cargo generate-lockfile".to_owned(),
                status: Some(1),
                stderr: "fixture lock failed".to_owned(),
            })
        }
    }

    #[test]
    fn adds_and_removes_a_valid_path_provider() {
        let project = gleam_project("images", "2.5.0");
        let provider = provider_package("geam-images", "images", ">= 2.0.0 and < 3.0.0");
        let root = utf8_path(&project);
        add_with(
            &root,
            project.path(),
            path_command(provider.path(), None),
            &TestCargo,
        )
        .expect("compatible provider should be selected");
        let source = fs::read_to_string(project.path().join("Cargo.toml"))
            .expect("managed manifest should be written");
        assert!(source.contains("geam_provider_images"));
        assert!(source.contains("package = \"geam-images\""));

        remove_with(
            &root,
            RemoveProvider {
                gleam_package: "images".to_owned(),
            },
            &TestCargo,
        )
        .expect("selected provider should be removed");
        let source = fs::read_to_string(project.path().join("Cargo.toml"))
            .expect("managed manifest should be readable");
        assert!(!source.contains("geam_provider_images"));
    }

    #[test]
    fn rejects_built_in_missing_incompatible_and_duplicate_targets() {
        let project = gleam_project("gleam_stdlib", "1.0.3");
        let provider = provider_package("geam-provider", "gleam_stdlib", "1.0.3");
        assert!(matches!(
            add_with(
                &utf8_path(&project),
                project.path(),
                path_command(provider.path(), None),
                &TestCargo,
            )
            .expect_err("built-in package should be rejected"),
            CliError::BuiltInProviderPackage { package } if package == "gleam_stdlib"
        ));

        let project = gleam_project("images", "2.5.0");
        let provider = provider_package("geam-provider", "missing", "1.0.0");
        assert!(matches!(
            add_with(
                &utf8_path(&project),
                project.path(),
                path_command(provider.path(), None),
                &TestCargo,
            )
            .expect_err("missing package should be rejected"),
            CliError::MissingGleamPackage { package } if package == "missing"
        ));

        let project = gleam_project("images", "2.5.0");
        let provider = provider_package("geam-provider", "images", "1.0.0");
        assert!(matches!(
            add_with(
                &utf8_path(&project),
                project.path(),
                path_command(provider.path(), None),
                &TestCargo,
            )
            .expect_err("incompatible provider should be rejected"),
            CliError::IncompatibleProvider { provider, package, version, range }
                if provider == "geam-provider"
                    && package == "images"
                    && version == "2.5.0"
                    && range == "1.0.0"
        ));

        let project = gleam_project("images", "2.5.0");
        let provider = provider_package("geam-images", "images", "2.5.0");
        let root = utf8_path(&project);
        add_with(
            &root,
            project.path(),
            path_command(provider.path(), None),
            &TestCargo,
        )
        .expect("first provider should be selected");
        assert!(matches!(
            add_with(
                &root,
                project.path(),
                path_command(provider.path(), None),
                &TestCargo,
            )
            .expect_err("duplicate provider should be rejected"),
            CliError::ProviderAlreadySelected { package } if package == "images"
        ));
    }

    #[test]
    fn remove_requires_an_existing_selection() {
        let project = gleam_project("images", "1.0.0");
        assert!(matches!(
            remove_with(
                &utf8_path(&project),
                RemoveProvider {
                    gleam_package: "images".to_owned(),
                },
                &TestCargo,
            )
            .expect_err("missing provider should be rejected"),
            CliError::ProviderNotSelected { package } if package == "images"
        ));
    }

    #[test]
    fn preserves_project_manifest_and_resolution_failures() {
        let provider = provider_package("geam-images", "images", "1.0.0");

        let invalid_add = gleam_project("images", "1.0.0");
        fs::write(invalid_add.path().join("manifest.toml"), "invalid")
            .expect("invalid manifest should be written");
        let error = add_with(
            &utf8_path(&invalid_add),
            invalid_add.path(),
            path_command(provider.path(), None),
            &TestCargo,
        )
        .expect_err("invalid manifest should stop provider add");
        assert!(matches!(
            error,
            CliError::InvalidToml { kind, path, reason }
                if kind == "Gleam manifest"
                    && path == utf8_path(&invalid_add).join("manifest.toml")
                    && reason.contains("expected")
        ));

        let invalid_remove = gleam_project("images", "1.0.0");
        fs::write(invalid_remove.path().join("manifest.toml"), "invalid")
            .expect("invalid manifest should be written");
        let error = remove_with(
            &utf8_path(&invalid_remove),
            RemoveProvider {
                gleam_package: "images".to_owned(),
            },
            &TestCargo,
        )
        .expect_err("invalid manifest should stop provider remove");
        assert!(matches!(
            error,
            CliError::InvalidToml { kind, path, reason }
                if kind == "Gleam manifest"
                    && path == utf8_path(&invalid_remove).join("manifest.toml")
                    && reason.contains("expected")
        ));

        let user_add = gleam_project("images", "1.0.0");
        fs::write(user_add.path().join("Cargo.toml"), "[workspace]\n")
            .expect("user Cargo manifest should be written");
        let error = add_with(
            &utf8_path(&user_add),
            user_add.path(),
            path_command(provider.path(), None),
            &TestCargo,
        )
        .expect_err("user Cargo manifest should stop provider add");
        assert!(matches!(
            error,
            CliError::UserOwnedCargoManifest { path }
                if path == utf8_path(&user_add).join("Cargo.toml")
        ));

        let user_remove = gleam_project("images", "1.0.0");
        fs::write(user_remove.path().join("Cargo.toml"), "[workspace]\n")
            .expect("user Cargo manifest should be written");
        let error = remove_with(
            &utf8_path(&user_remove),
            RemoveProvider {
                gleam_package: "images".to_owned(),
            },
            &TestCargo,
        )
        .expect_err("user Cargo manifest should stop provider remove");
        assert!(matches!(
            error,
            CliError::UserOwnedCargoManifest { path }
                if path == utf8_path(&user_remove).join("Cargo.toml")
        ));

        let unresolved = gleam_project("images", "1.0.0");
        let error = add_with(
            &utf8_path(&unresolved),
            unresolved.path(),
            path_command(&unresolved.path().join("missing"), None),
            &TestCargo,
        )
        .expect_err("missing provider path should stop resolution");
        assert!(matches!(
            error,
            CliError::FileRead { path, error }
                if path == utf8_path(&unresolved).join("missing")
                    && error.kind() == std::io::ErrorKind::NotFound
        ));
    }

    #[test]
    fn keeps_provider_add_recoverable_across_generated_input_failures() {
        let blocked_source = gleam_project("images", "1.0.0");
        let provider = provider_package("geam-images", "images", "1.0.0");
        fs::create_dir_all(blocked_source.path().join("build/geam/runner.rs"))
            .expect("blocking runner source directory should be created");
        let blocked_source_path = utf8_path(&blocked_source).join("build/geam/runner.rs");
        let expected_kind = fs::read_to_string(&blocked_source_path)
            .expect_err("runner source directory should not be readable as a file")
            .kind();
        let error = add_with(
            &utf8_path(&blocked_source),
            blocked_source.path(),
            path_command(provider.path(), None),
            &TestCargo,
        )
        .expect_err("runner source failure should stop provider add");
        assert!(matches!(
            error,
            CliError::FileRead { path, error }
                if path == blocked_source_path && error.kind() == expected_kind
        ));
        assert!(!blocked_source.path().join("Cargo.toml").exists());

        let blocked_manifest = gleam_project("images", "1.0.0");
        fs::create_dir(blocked_manifest.path().join("Cargo.toml.geam.tmp"))
            .expect("blocking manifest directory should be created");
        let blocked_manifest_path = utf8_path(&blocked_manifest).join("Cargo.toml.geam.tmp");
        let expected_kind = fs::write(&blocked_manifest_path, "manifest")
            .expect_err("manifest directory should reject file writes")
            .kind();
        let error = add_with(
            &utf8_path(&blocked_manifest),
            blocked_manifest.path(),
            path_command(provider.path(), None),
            &TestCargo,
        )
        .expect_err("manifest failure should stop provider add");
        assert!(matches!(
            error,
            CliError::FileWrite { path, error }
                if path == blocked_manifest_path && error.kind() == expected_kind
        ));
        assert!(!blocked_manifest.path().join("Cargo.toml").exists());

        let failed_lock = gleam_project("images", "1.0.0");
        let root = utf8_path(&failed_lock);
        let error = add_with(
            &root,
            failed_lock.path(),
            path_command(provider.path(), None),
            &FailingCargoLock,
        )
        .expect_err("lock failure should remain a Cargo process error");
        assert!(matches!(
            error,
            CliError::ProcessFailure { command, status: Some(1), stderr }
                if command == "cargo generate-lockfile" && stderr == "fixture lock failed"
        ));
        assert!(
            fs::read_to_string(root.join("Cargo.toml"))
                .expect("selected manifest should remain readable")
                .contains("geam_provider_images")
        );
        assert!(!root.join("Cargo.lock").exists());
    }

    #[test]
    fn keeps_provider_remove_recoverable_across_manifest_and_lock_failures() {
        let provider = provider_package("geam-images", "images", "1.0.0");

        let blocked_source = gleam_project("images", "1.0.0");
        let root = utf8_path(&blocked_source);
        add_with(
            &root,
            blocked_source.path(),
            path_command(provider.path(), None),
            &TestCargo,
        )
        .expect("provider should first be selected");
        fs::remove_file(root.join("build/geam/runner.rs"))
            .expect("generated source should be removed");
        fs::create_dir(root.join("build/geam/runner.rs"))
            .expect("blocking source directory should be created");
        let blocked_source_path = root.join("build/geam/runner.rs");
        let expected_kind = fs::read_to_string(&blocked_source_path)
            .expect_err("runner source directory should not be readable as a file")
            .kind();
        let error = remove_with(
            &root,
            RemoveProvider {
                gleam_package: "images".to_owned(),
            },
            &TestCargo,
        )
        .expect_err("runner source failure should stop provider removal");
        assert!(matches!(
            error,
            CliError::FileRead { path, error }
                if path == blocked_source_path && error.kind() == expected_kind
        ));
        assert!(
            fs::read_to_string(root.join("Cargo.toml"))
                .expect("existing manifest should remain readable")
                .contains("geam_provider_images")
        );

        let blocked_manifest = gleam_project("images", "1.0.0");
        let root = utf8_path(&blocked_manifest);
        add_with(
            &root,
            blocked_manifest.path(),
            path_command(provider.path(), None),
            &TestCargo,
        )
        .expect("provider should first be selected");
        fs::create_dir(root.join("Cargo.toml.geam.tmp"))
            .expect("blocking manifest directory should be created");
        let blocked_manifest_path = root.join("Cargo.toml.geam.tmp");
        let expected_kind = fs::write(&blocked_manifest_path, "manifest")
            .expect_err("manifest directory should reject file writes")
            .kind();
        let error = remove_with(
            &root,
            RemoveProvider {
                gleam_package: "images".to_owned(),
            },
            &TestCargo,
        )
        .expect_err("manifest failure should stop provider removal");
        assert!(matches!(
            error,
            CliError::FileWrite { path, error }
                if path == blocked_manifest_path && error.kind() == expected_kind
        ));
        assert!(
            fs::read_to_string(root.join("Cargo.toml"))
                .expect("existing manifest should remain readable")
                .contains("geam_provider_images")
        );

        let failed_lock = gleam_project("images", "1.0.0");
        let root = utf8_path(&failed_lock);
        add_with(
            &root,
            failed_lock.path(),
            path_command(provider.path(), None),
            &TestCargo,
        )
        .expect("provider should first be selected");
        let error = remove_with(
            &root,
            RemoveProvider {
                gleam_package: "images".to_owned(),
            },
            &FailingCargoLock,
        )
        .expect_err("lock failure should remain a Cargo process error");
        assert!(matches!(
            error,
            CliError::ProcessFailure { command, status: Some(1), stderr }
                if command == "cargo generate-lockfile" && stderr == "fixture lock failed"
        ));
        assert!(
            !fs::read_to_string(root.join("Cargo.toml"))
                .expect("updated manifest should remain readable")
                .contains("geam_provider_images")
        );
        assert!(!root.join("Cargo.lock").exists());
    }

    fn path_command(path: &std::path::Path, package: Option<&str>) -> AddProvider {
        AddProvider {
            crate_spec: None,
            path: Some(
                Utf8PathBuf::from_path_buf(path.to_path_buf())
                    .expect("provider path should be valid UTF-8"),
            ),
            git: None,
            rev: None,
            package: package.map(str::to_owned),
        }
    }

    fn gleam_project(dependency: &str, version: &str) -> TempDir {
        let project = tempdir().expect("Gleam project should be created");
        fs::create_dir(project.path().join("src")).expect("source directory should be created");
        fs::write(
            project.path().join("gleam.toml"),
            format!(
                "name = \"application\"\nversion = \"1.0.0\"\n\n[dependencies]\n{dependency} = \"{version}\"\n",
            ),
        )
        .expect("Gleam config should be written");
        fs::write(
            project.path().join("manifest.toml"),
            format!(
                "packages = [\n  {{ name = \"{dependency}\", version = \"{version}\", build_tools = [\"gleam\"], requirements = [], source = \"hex\", outer_checksum = \"00\" }},\n]\n\n[requirements]\n",
            ),
        )
        .expect("Gleam manifest should be written");
        fs::write(
            project.path().join("src/application.gleam"),
            "pub fn main() { 1 }\n",
        )
        .expect("Gleam source should be written");
        project
    }

    fn provider_package(name: &str, gleam_package: &str, range: &str) -> TempDir {
        let provider = tempdir().expect("provider package should be created");
        fs::create_dir(provider.path().join("src"))
            .expect("provider source directory should be created");
        fs::write(
            provider.path().join("Cargo.toml"),
            format!(
                r#"[package]
name = "{name}"
version = "1.2.3"
edition = "2024"

[package.metadata.geam.provider]
schema = 1
gleam-package = "{gleam_package}"
gleam-version = "{range}"
"#,
            ),
        )
        .expect("provider manifest should be written");
        fs::write(
            provider.path().join("src/lib.rs"),
            "pub struct Component;\n",
        )
        .expect("provider source should be written");
        provider
    }

    fn utf8_path(directory: &TempDir) -> Utf8PathBuf {
        Utf8PathBuf::from_path_buf(directory.path().to_path_buf())
            .expect("temporary path should be valid UTF-8")
    }
}
