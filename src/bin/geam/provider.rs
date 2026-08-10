#[path = "provider/manifest.rs"]
mod manifest;
#[path = "provider/metadata.rs"]
mod metadata;
#[path = "provider/resolution.rs"]
mod resolution;

use crate::command::{AddProvider, RemoveProvider};
use crate::error::CliError;
use crate::project::read_resolved_project;
use camino::Utf8Path;
use manifest::{ManagedProject, ProviderSelection};
use std::path::Path;

const BUILT_IN_PROVIDER_PACKAGES: [&str; 3] = ["gleam_json", "gleam_stdlib", "gleam_time"];

pub(super) fn add(
    project_root: &Utf8Path,
    current_directory: &Path,
    command: AddProvider,
) -> Result<(), CliError> {
    let project = read_resolved_project(project_root)?;
    let mut managed = ManagedProject::load(project_root, project.root_package())?;
    managed.retain_packages(&project.package_names());
    let resolved = resolution::resolve(project_root, current_directory, command)?;
    let package = resolved.metadata.gleam_package();
    if BUILT_IN_PROVIDER_PACKAGES.contains(&package) {
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
    managed.write()?;
    Ok(())
}

pub(super) fn remove(project_root: &Utf8Path, command: RemoveProvider) -> Result<(), CliError> {
    let project = read_resolved_project(project_root)?;
    let mut managed = ManagedProject::load(project_root, project.root_package())?;
    managed.remove(&command.gleam_package)?;
    managed.retain_packages(&project.package_names());
    managed.write()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{add, remove};
    use crate::command::{AddProvider, RemoveProvider};
    use crate::error::CliError;
    use camino::Utf8PathBuf;
    use std::fs;
    use tempfile::{TempDir, tempdir};

    #[test]
    fn adds_and_removes_a_valid_path_provider() {
        let project = gleam_project("images", "2.5.0");
        let provider = provider_package("geam-images", "images", ">= 2.0.0 and < 3.0.0");
        let root = utf8_path(&project);

        add(&root, project.path(), path_command(provider.path(), None))
            .expect("compatible provider should be selected");
        let source = fs::read_to_string(project.path().join("Cargo.toml"))
            .expect("managed manifest should be written");
        assert!(source.contains("geam_provider_images"));
        assert!(source.contains("package = \"geam-images\""));

        remove(
            &root,
            RemoveProvider {
                gleam_package: "images".to_owned(),
            },
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
        assert_eq!(
            add(
                &utf8_path(&project),
                project.path(),
                path_command(provider.path(), None),
            )
            .expect_err("built-in package should be rejected")
            .to_string(),
            "Gleam package gleam_stdlib is provided by Geam and cannot use an external provider",
        );

        let project = gleam_project("images", "2.5.0");
        let provider = provider_package("geam-provider", "missing", "1.0.0");
        assert_eq!(
            add(
                &utf8_path(&project),
                project.path(),
                path_command(provider.path(), None),
            )
            .expect_err("missing package should be rejected")
            .to_string(),
            "provider targets Gleam package missing, which is absent from the resolved project",
        );

        let project = gleam_project("images", "2.5.0");
        let provider = provider_package("geam-provider", "images", "1.0.0");
        assert_eq!(
            add(
                &utf8_path(&project),
                project.path(),
                path_command(provider.path(), None),
            )
            .expect_err("incompatible provider should be rejected")
            .to_string(),
            "provider geam-provider targets images 2.5.0, which is outside its Gleam range 1.0.0",
        );

        let project = gleam_project("images", "2.5.0");
        let provider = provider_package("geam-images", "images", "2.5.0");
        let root = utf8_path(&project);
        add(&root, project.path(), path_command(provider.path(), None))
            .expect("first provider should be selected");
        assert_eq!(
            add(&root, project.path(), path_command(provider.path(), None),)
                .expect_err("duplicate provider should be rejected")
                .to_string(),
            "provider for Gleam package images is already selected",
        );
    }

    #[test]
    fn remove_requires_an_existing_selection() {
        let project = gleam_project("images", "1.0.0");
        assert_eq!(
            remove(
                &utf8_path(&project),
                RemoveProvider {
                    gleam_package: "images".to_owned(),
                },
            )
            .expect_err("missing provider should be rejected")
            .to_string(),
            "no provider is selected for Gleam package images",
        );
    }

    #[test]
    fn preserves_project_manifest_and_resolution_failures() {
        let provider = provider_package("geam-images", "images", "1.0.0");

        let invalid_add = gleam_project("images", "1.0.0");
        fs::write(invalid_add.path().join("manifest.toml"), "invalid")
            .expect("invalid manifest should be written");
        let error = add(
            &utf8_path(&invalid_add),
            invalid_add.path(),
            path_command(provider.path(), None),
        )
        .expect_err("invalid manifest should stop provider add");
        assert_eq!(
            std::mem::discriminant(&error),
            std::mem::discriminant(&CliError::InvalidToml {
                kind: "Gleam manifest",
                path: Utf8PathBuf::new(),
                reason: String::new(),
            }),
        );

        let invalid_remove = gleam_project("images", "1.0.0");
        fs::write(invalid_remove.path().join("manifest.toml"), "invalid")
            .expect("invalid manifest should be written");
        let error = remove(
            &utf8_path(&invalid_remove),
            RemoveProvider {
                gleam_package: "images".to_owned(),
            },
        )
        .expect_err("invalid manifest should stop provider remove");
        assert_eq!(
            std::mem::discriminant(&error),
            std::mem::discriminant(&CliError::InvalidToml {
                kind: "Gleam manifest",
                path: Utf8PathBuf::new(),
                reason: String::new(),
            }),
        );

        let user_add = gleam_project("images", "1.0.0");
        fs::write(user_add.path().join("Cargo.toml"), "[workspace]\n")
            .expect("user Cargo manifest should be written");
        let error = add(
            &utf8_path(&user_add),
            user_add.path(),
            path_command(provider.path(), None),
        )
        .expect_err("user Cargo manifest should stop provider add");
        assert_eq!(
            std::mem::discriminant(&error),
            std::mem::discriminant(&CliError::UserOwnedCargoManifest {
                path: Utf8PathBuf::new(),
            }),
        );

        let user_remove = gleam_project("images", "1.0.0");
        fs::write(user_remove.path().join("Cargo.toml"), "[workspace]\n")
            .expect("user Cargo manifest should be written");
        let error = remove(
            &utf8_path(&user_remove),
            RemoveProvider {
                gleam_package: "images".to_owned(),
            },
        )
        .expect_err("user Cargo manifest should stop provider remove");
        assert_eq!(
            std::mem::discriminant(&error),
            std::mem::discriminant(&CliError::UserOwnedCargoManifest {
                path: Utf8PathBuf::new(),
            }),
        );

        let unresolved = gleam_project("images", "1.0.0");
        let error = add(
            &utf8_path(&unresolved),
            unresolved.path(),
            path_command(&unresolved.path().join("missing"), None),
        )
        .expect_err("missing provider path should stop resolution");
        assert_eq!(
            std::mem::discriminant(&error),
            std::mem::discriminant(&CliError::FileRead {
                path: Utf8PathBuf::new(),
                error: std::io::Error::new(std::io::ErrorKind::NotFound, ""),
            }),
        );
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
