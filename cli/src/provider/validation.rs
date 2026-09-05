use super::manifest::{ManagedProject, ProviderSelection};
use super::metadata::ProviderMetadata;
use crate::error::CliError;
use crate::progress::Progress;
use crate::project::ResolvedProject;
use camino::Utf8Path;
use std::collections::BTreeSet;

pub(crate) trait ProviderSelectionValidator {
    fn validate(
        &self,
        project_root: &Utf8Path,
        project: &ResolvedProject,
        program: &geam_core::TypedProgram,
        managed: &ManagedProject,
        progress: &mut Progress<'_>,
    ) -> Result<(), CliError>;
}

pub(super) trait ProviderResolver {
    fn resolve(
        &self,
        project_root: &Utf8Path,
        selection: &ProviderSelection,
        progress: &mut Progress<'_>,
    ) -> Result<ProviderMetadata, CliError>;
}

pub(super) struct SystemProviderResolver;

impl ProviderResolver for SystemProviderResolver {
    fn resolve(
        &self,
        project_root: &Utf8Path,
        selection: &ProviderSelection,
        progress: &mut Progress<'_>,
    ) -> Result<ProviderMetadata, CliError> {
        super::resolution::resolve_selection(project_root, selection, progress)
    }
}

pub(super) struct ProviderValidator<'resolver> {
    resolver: &'resolver dyn ProviderResolver,
}

impl<'resolver> ProviderValidator<'resolver> {
    pub(super) fn new(resolver: &'resolver dyn ProviderResolver) -> Self {
        Self { resolver }
    }

    fn validate_packages(
        &self,
        project_root: &Utf8Path,
        project: &ResolvedProject,
        required_packages: BTreeSet<String>,
        managed: &ManagedProject,
        progress: &mut Progress<'_>,
    ) -> Result<(), CliError> {
        let mut provided = BTreeSet::new();

        for (package, version) in project.packages() {
            let Some(selection) = managed.provider(package) else {
                continue;
            };
            if super::is_built_in_package(package) {
                return Err(CliError::BuiltInProviderPackage {
                    package: package.to_owned(),
                });
            }
            progress.report(format_args!(
                "Resolving provider {} for {package} {version}",
                selection.crate_name()
            ))?;
            let metadata = self.resolver.resolve(project_root, selection, progress)?;
            validate_selected_metadata(selection, &metadata)?;
            if !metadata.supports(version) {
                return Err(CliError::IncompatibleProvider {
                    provider: metadata.crate_name().to_owned(),
                    package: package.to_owned(),
                    version: version.to_string(),
                    range: metadata.gleam_range().to_string(),
                });
            }
            provided.insert(package.to_owned());
        }

        for package in required_packages {
            if super::is_built_in_package(&package) || provided.contains(&package) {
                continue;
            }
            let version =
                project
                    .package_version(&package)
                    .ok_or_else(|| CliError::MissingGleamPackage {
                        package: package.clone(),
                    })?;
            return Err(CliError::MissingStandaloneProvider {
                package,
                version: version.to_string(),
            });
        }
        Ok(())
    }
}

impl ProviderSelectionValidator for ProviderValidator<'_> {
    fn validate(
        &self,
        project_root: &Utf8Path,
        project: &ResolvedProject,
        program: &geam_core::TypedProgram,
        managed: &ManagedProject,
        progress: &mut Progress<'_>,
    ) -> Result<(), CliError> {
        let required_packages = geam_core::required_host_functions(program)
            .into_iter()
            .map(|requirement| requirement.package().to_string())
            .collect();
        self.validate_packages(project_root, project, required_packages, managed, progress)
    }
}

fn validate_selected_metadata(
    selection: &ProviderSelection,
    metadata: &ProviderMetadata,
) -> Result<(), CliError> {
    if metadata.crate_name() != selection.crate_name() {
        return Err(CliError::InvalidProviderMetadata {
            package: selection.crate_name().to_owned(),
            reason: format!(
                "Cargo resolved package {}, expected {}",
                metadata.crate_name(),
                selection.crate_name(),
            ),
        });
    }
    if metadata.gleam_package() != selection.gleam_package() {
        return Err(CliError::InvalidProviderMetadata {
            package: selection.crate_name().to_owned(),
            reason: format!(
                "provider targets Gleam package {}, expected {}",
                metadata.gleam_package(),
                selection.gleam_package(),
            ),
        });
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{ProviderResolver, ProviderValidator, SystemProviderResolver};
    use crate::error::CliError;
    use crate::progress::Progress;
    use crate::project::read_resolved_project;
    use crate::provider::manifest::{ManagedProject, ProviderSelection, ProviderSource};
    use crate::provider::metadata::ProviderMetadata;
    use camino::{Utf8Path, Utf8PathBuf};
    use cargo_metadata::MetadataCommand;
    use std::cell::RefCell;
    use std::collections::{BTreeMap, BTreeSet};
    use std::fs;
    use std::process::Command;
    use tempfile::{TempDir, tempdir};

    #[test]
    fn validates_every_selection_and_requires_only_host_packages() {
        let project = resolved_project(&[
            ("fallback", "1.0.0"),
            ("images", "1.5.0"),
            ("search", "2.0.0"),
        ]);
        let root = utf8_path(&project);
        let resolved = read_resolved_project(&root).expect("project should resolve");
        let mut managed =
            ManagedProject::load(&root, "application").expect("managed project should initialize");
        managed
            .insert(selection("fallback", "geam-fallback", "4.0.0"))
            .expect("fallback provider should be selected");
        managed
            .insert(selection("images", "geam-images", "1.0.0"))
            .expect("images provider should be selected");
        let resolver = FixedResolver::new([
            metadata("geam-fallback", "fallback", ">= 1.0.0 and < 2.0.0"),
            metadata("geam-images", "images", ">= 1.0.0 and < 2.0.0"),
        ]);
        let mut output = Vec::new();

        ProviderValidator::new(&resolver)
            .validate_packages(
                &root,
                &resolved,
                BTreeSet::from(["images".to_owned()]),
                &managed,
                &mut Progress::Visible(&mut output),
            )
            .expect("explicit selections should be valid");

        assert_eq!(
            output,
            concat!(
                "geam: Resolving provider geam-fallback for fallback 1.0.0\n",
                "geam: Resolving provider geam-images for images 1.5.0\n",
            )
            .as_bytes(),
        );
        assert_eq!(
            resolver.calls.borrow().as_slice(),
            ["geam-fallback", "geam-images"],
        );
    }

    #[test]
    fn reports_missing_and_incompatible_required_providers() {
        let project = resolved_project(&[("images", "2.0.0")]);
        let root = utf8_path(&project);
        let resolved = read_resolved_project(&root).expect("project should resolve");
        let mut managed =
            ManagedProject::load(&root, "application").expect("managed project should initialize");
        let resolver = FixedResolver::new([]);

        assert!(matches!(
            ProviderValidator::new(&resolver).validate_packages(
                &root,
                &resolved,
                BTreeSet::from(["images".to_owned()]),
                &managed,
                &mut Progress::Hidden,
            ),
            Err(CliError::MissingStandaloneProvider { package, version })
                if package == "images" && version == "2.0.0"
        ));

        managed
            .insert(selection("images", "geam-images", "1.0.0"))
            .expect("images provider should be selected");
        let resolver =
            FixedResolver::new([metadata("geam-images", "images", ">= 1.0.0 and < 2.0.0")]);
        assert!(matches!(
            ProviderValidator::new(&resolver).validate_packages(
                &root,
                &resolved,
                BTreeSet::from(["images".to_owned()]),
                &managed,
                &mut Progress::Hidden,
            ),
            Err(CliError::IncompatibleProvider { provider, package, version, range })
                if provider == "geam-images"
                    && package == "images"
                    && version == "2.0.0"
                    && range == ">= 1.0.0 and < 2.0.0"
        ));
    }

    #[test]
    fn rejects_a_required_package_outside_the_resolved_project() {
        let project = resolved_project(&[]);
        let root = utf8_path(&project);
        let resolved = read_resolved_project(&root).expect("project should resolve");
        let managed =
            ManagedProject::load(&root, "application").expect("managed project should initialize");
        let resolver = FixedResolver::new([]);

        assert!(matches!(
            ProviderValidator::new(&resolver).validate_packages(
                &root,
                &resolved,
                BTreeSet::from(["missing".to_owned()]),
                &managed,
                &mut Progress::Hidden,
            ),
            Err(CliError::MissingGleamPackage { package }) if package == "missing"
        ));
    }

    #[test]
    fn accepts_required_builtins_and_rejects_selected_builtins() {
        let project = resolved_project(&[("gleam_json", "3.1.0")]);
        let root = utf8_path(&project);
        let resolved = read_resolved_project(&root).expect("project should resolve");
        let mut managed =
            ManagedProject::load(&root, "application").expect("managed project should initialize");
        let resolver = FixedResolver::new([]);
        ProviderValidator::new(&resolver)
            .validate_packages(
                &root,
                &resolved,
                BTreeSet::from(["gleam_json".to_owned()]),
                &managed,
                &mut Progress::Hidden,
            )
            .expect("built-in packages need no selection");

        managed
            .insert(selection("gleam_json", "geam-json-other", "1.0.0"))
            .expect("built-in provider fixture should be selected");
        assert!(matches!(
            ProviderValidator::new(&resolver).validate_packages(
                &root,
                &resolved,
                BTreeSet::new(),
                &managed,
                &mut Progress::Hidden,
            ),
            Err(CliError::BuiltInProviderPackage { package }) if package == "gleam_json"
        ));
    }

    #[test]
    fn rejects_mismatched_selected_metadata() {
        let project = resolved_project(&[("images", "1.0.0")]);
        let root = utf8_path(&project);
        let resolved = read_resolved_project(&root).expect("project should resolve");
        let mut managed =
            ManagedProject::load(&root, "application").expect("managed project should initialize");
        managed
            .insert(selection("images", "geam-images", "1.0.0"))
            .expect("images provider should be selected");

        for (metadata, expected_reason) in [
            (
                metadata("different-crate", "images", ">= 1.0.0"),
                "Cargo resolved package different-crate, expected geam-images",
            ),
            (
                metadata("geam-images", "search", ">= 1.0.0"),
                "provider targets Gleam package search, expected images",
            ),
        ] {
            let resolver = FixedResolver {
                metadata: BTreeMap::from([("geam-images".to_owned(), metadata)]),
                calls: RefCell::new(Vec::new()),
            };
            assert!(matches!(
                ProviderValidator::new(&resolver).validate_packages(
                    &root,
                    &resolved,
                    BTreeSet::new(),
                    &managed,
                    &mut Progress::Hidden,
                ),
                Err(CliError::InvalidProviderMetadata { package, reason })
                    if package == "geam-images" && reason == expected_reason
            ));
        }
    }

    #[test]
    fn preserves_progress_and_resolution_failures() {
        let project = resolved_project(&[("images", "1.0.0")]);
        let root = utf8_path(&project);
        let resolved = read_resolved_project(&root).expect("project should resolve");
        let mut managed =
            ManagedProject::load(&root, "application").expect("managed project should initialize");
        managed
            .insert(selection("images", "geam-images", "1.0.0"))
            .expect("images provider should be selected");
        let resolver = FixedResolver::new([]);
        let mut output = fs::File::open(root.join("gleam.toml")).expect("read-only output");
        assert_eq!(
            ProviderValidator::new(&resolver)
                .validate_packages(
                    &root,
                    &resolved,
                    BTreeSet::new(),
                    &managed,
                    &mut Progress::Visible(&mut output),
                )
                .expect_err("progress write should fail")
                .to_string(),
            "failed to write preparation progress",
        );
        assert!(resolver.calls.borrow().is_empty());

        assert!(matches!(
            ProviderValidator::new(&FailingResolver).validate_packages(
                &root,
                &resolved,
                BTreeSet::new(),
                &managed,
                &mut Progress::Hidden,
            ),
            Err(CliError::InvalidProviderMetadata { package, reason })
                if package == "geam-images" && reason == "fixture resolution failure"
        ));
    }

    #[test]
    fn connects_the_system_metadata_adapter() {
        let provider = provider_package("geam-images", "images", ">= 1.0.0 and < 2.0.0");
        let project = tempdir().expect("temporary project should be created");
        let root = utf8_path(&project);
        let provider_path = utf8_path(&provider);
        fs::create_dir(root.join("src")).expect("runner source directory should be created");
        fs::write(root.join("src/main.rs"), "fn main() {}\n")
            .expect("runner source should be written");
        fs::write(
            root.join("Cargo.toml"),
            format!(
                "[package]\nname = \"fixture-runner\"\nversion = \"0.0.0\"\nedition = \"2024\"\n\n[dependencies]\ngeam_provider_images = {{ package = \"geam-images\", path = {:?} }}\n\n[workspace]\nresolver = \"3\"\n",
                provider_path.as_str(),
            ),
        )
        .expect("runner manifest should be written");
        assert!(
            Command::new("cargo")
                .arg("generate-lockfile")
                .current_dir(&root)
                .status()
                .expect("Cargo should start")
                .success(),
        );
        let selection = ProviderSelection::new(
            "images".to_owned(),
            "geam-images".to_owned(),
            ProviderSource::Path {
                path: provider_path,
            },
        );
        let metadata = SystemProviderResolver
            .resolve(&root, &selection, &mut Progress::Hidden)
            .expect("path provider should resolve through Cargo metadata");
        assert_eq!(metadata.gleam_package(), "images");
    }

    struct FixedResolver {
        metadata: BTreeMap<String, ProviderMetadata>,
        calls: RefCell<Vec<String>>,
    }

    impl FixedResolver {
        fn new<const N: usize>(metadata: [ProviderMetadata; N]) -> Self {
            Self {
                metadata: metadata
                    .into_iter()
                    .map(|metadata| (metadata.crate_name().to_owned(), metadata))
                    .collect(),
                calls: RefCell::new(Vec::new()),
            }
        }
    }

    impl ProviderResolver for FixedResolver {
        fn resolve(
            &self,
            _project_root: &Utf8Path,
            selection: &ProviderSelection,
            _progress: &mut Progress<'_>,
        ) -> Result<ProviderMetadata, CliError> {
            self.calls
                .borrow_mut()
                .push(selection.crate_name().to_owned());
            Ok(self
                .metadata
                .get(selection.crate_name())
                .expect("fixture metadata should exist")
                .clone())
        }
    }

    struct FailingResolver;

    impl ProviderResolver for FailingResolver {
        fn resolve(
            &self,
            _project_root: &Utf8Path,
            selection: &ProviderSelection,
            _progress: &mut Progress<'_>,
        ) -> Result<ProviderMetadata, CliError> {
            Err(CliError::InvalidProviderMetadata {
                package: selection.crate_name().to_owned(),
                reason: "fixture resolution failure".to_owned(),
            })
        }
    }

    fn selection(package: &str, crate_name: &str, version: &str) -> ProviderSelection {
        ProviderSelection::new(
            package.to_owned(),
            crate_name.to_owned(),
            ProviderSource::Registry {
                version: version.parse().expect("version should parse"),
            },
        )
    }

    fn metadata(crate_name: &str, package: &str, range: &str) -> ProviderMetadata {
        let package_id = format!("path+file:///provider#{crate_name}@1.0.0");
        let source = serde_json::json!({
            "packages": [{
                "name": crate_name,
                "version": "1.0.0",
                "id": package_id,
                "license": null,
                "license_file": null,
                "description": null,
                "source": null,
                "dependencies": [],
                "targets": [],
                "features": {},
                "manifest_path": "/provider/Cargo.toml",
                "categories": [],
                "keywords": [],
                "readme": null,
                "repository": null,
                "homepage": null,
                "documentation": null,
                "edition": "2024",
                "metadata": {
                    "geam": {
                        "provider": {
                            "schema": 1,
                            "gleam-package": package,
                            "gleam-version": range,
                        }
                    }
                },
                "links": null,
                "publish": null,
                "authors": [],
                "default_run": null,
                "rust_version": "1.96"
            }],
            "workspace_members": [package_id],
            "workspace_default_members": [package_id],
            "resolve": null,
            "target_directory": "/target",
            "build_directory": "/target",
            "version": 1,
            "workspace_root": "/provider",
            "metadata": null
        });
        let package = MetadataCommand::parse(source.to_string())
            .expect("Cargo metadata fixture should parse")
            .packages
            .pop()
            .expect("provider package should be present");
        ProviderMetadata::from_package(&package).expect("provider metadata should be valid")
    }

    fn resolved_project(packages: &[(&str, &str)]) -> TempDir {
        let project = tempdir().expect("temporary project should be created");
        fs::write(
            project.path().join("gleam.toml"),
            "name = \"application\"\nversion = \"1.0.0\"\n",
        )
        .expect("Gleam config should be written");
        let packages = packages
            .iter()
            .map(|(name, version)| {
                format!(
                    "  {{ name = \"{name}\", version = \"{version}\", build_tools = [\"gleam\"], requirements = [], source = \"hex\", outer_checksum = \"00\" }},\n"
                )
            })
            .collect::<String>();
        fs::write(
            project.path().join("manifest.toml"),
            format!("packages = [\n{packages}]\n\n[requirements]\n"),
        )
        .expect("Gleam manifest should be written");
        project
    }

    fn provider_package(crate_name: &str, package: &str, range: &str) -> TempDir {
        let provider = tempdir().expect("provider directory should be created");
        fs::create_dir(provider.path().join("src"))
            .expect("provider source directory should be created");
        fs::write(
            provider.path().join("Cargo.toml"),
            format!(
                "[package]\nname = \"{crate_name}\"\nversion = \"1.0.0\"\nedition = \"2024\"\n\n[package.metadata.geam.provider]\nschema = 1\ngleam-package = \"{package}\"\ngleam-version = \"{range}\"\n"
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
