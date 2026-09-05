use super::manifest::ProviderSource;
use super::metadata::ProviderMetadata;
use crate::cargo::{CargoMetadataLoader, CargoMetadataMode, SystemCargoMetadata};
use crate::command::AddProvider;
use crate::error::CliError;
use crate::process::run_checked;
use crate::progress::Progress;
use camino::{Utf8Path, Utf8PathBuf};
use cargo_metadata::{Metadata, Package};
use semver::Version;
use std::fs;
use std::path::Path;
use std::process::Command;
use tempfile::TempDir;

const CANDIDATE_ALIAS: &str = "provider_candidate";

pub(super) struct ResolvedProvider {
    pub(super) metadata: ProviderMetadata,
    pub(super) source: ProviderSource,
}

pub(super) fn resolve(
    project_root: &Utf8Path,
    current_directory: &Path,
    command: AddProvider,
) -> Result<ResolvedProvider, CliError> {
    let request = ProviderRequest::from_command(current_directory, command)?;
    let loader = SystemCargoMetadata;
    resolve_with(project_root, request, &loader)
}

pub(super) fn resolve_selection(
    project_root: &Utf8Path,
    selection: &super::manifest::ProviderSelection,
    progress: &mut Progress<'_>,
) -> Result<ProviderMetadata, CliError> {
    resolve_selection_with(project_root, selection, &SystemCargoMetadata, progress)
}

fn resolve_with(
    project_root: &Utf8Path,
    request: ProviderRequest,
    loader: &dyn CargoMetadataLoader,
) -> Result<ResolvedProvider, CliError> {
    resolve_with_candidate(project_root, request, loader, CandidateWorkspace::new)
}

fn resolve_with_candidate(
    project_root: &Utf8Path,
    request: ProviderRequest,
    loader: &dyn CargoMetadataLoader,
    create_candidate: fn(&ProviderRequest) -> Result<CandidateWorkspace, CliError>,
) -> Result<ResolvedProvider, CliError> {
    let request = complete_package_identity(project_root, request, loader)?;
    let candidate = create_candidate(&request)?;
    let metadata = loader.load(
        project_root,
        candidate.manifest(),
        CargoMetadataMode::Resolve,
        &mut Progress::Hidden,
    )?;
    let package = resolved_dependency(&metadata, CANDIDATE_ALIAS)?;
    let provider = ProviderMetadata::from_package(package)?;
    let source = request.provider_source(package);
    Ok(ResolvedProvider {
        metadata: provider,
        source,
    })
}

fn resolve_selection_with(
    project_root: &Utf8Path,
    selection: &super::manifest::ProviderSelection,
    loader: &dyn CargoMetadataLoader,
    progress: &mut Progress<'_>,
) -> Result<ProviderMetadata, CliError> {
    let manifest = project_root.join("Cargo.toml");
    let metadata = loader.load(project_root, &manifest, CargoMetadataMode::Locked, progress)?;
    let package = resolved_dependency(&metadata, &selection.alias())?;
    ProviderMetadata::from_package(package)
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum ProviderRequest {
    Registry {
        crate_name: String,
        version: Option<Version>,
    },
    Path {
        path: Utf8PathBuf,
        package: Option<String>,
    },
    Git {
        url: String,
        rev: Option<String>,
        package: Option<String>,
    },
}

impl ProviderRequest {
    fn from_command(current_directory: &Path, command: AddProvider) -> Result<Self, CliError> {
        if let Some(spec) = command.crate_spec {
            let (crate_name, version) = parse_registry_specification(&spec)?;
            return Ok(Self::Registry {
                crate_name,
                version,
            });
        }
        if let Some(path) = command.path {
            let path = if path.is_absolute() {
                path
            } else {
                let current = Utf8PathBuf::from_path_buf(current_directory.to_path_buf())
                    .map_err(CliError::NonUtf8Path)?;
                current.join(path)
            };
            return Ok(Self::Path {
                path: canonical_provider_path(&path)?,
                package: command.package,
            });
        }
        let url = command
            .git
            .ok_or_else(|| CliError::InvalidCrateSpecification {
                spec: String::new(),
                reason: "one provider source is required".to_owned(),
            })?;
        Ok(Self::Git {
            url,
            rev: command.rev,
            package: command.package,
        })
    }

    fn provider_source(&self, package: &Package) -> ProviderSource {
        match self {
            Self::Registry { .. } => ProviderSource::Registry {
                version: package.version.clone(),
            },
            Self::Path { path, .. } => ProviderSource::Path { path: path.clone() },
            Self::Git { url, rev, .. } => ProviderSource::Git {
                url: url.clone(),
                rev: rev.clone(),
            },
        }
    }

    fn crate_name(&self) -> Option<&str> {
        match self {
            Self::Registry { crate_name, .. } => Some(crate_name),
            Self::Path { package, .. } | Self::Git { package, .. } => package.as_deref(),
        }
    }
}

fn complete_package_identity(
    project_root: &Utf8Path,
    request: ProviderRequest,
    loader: &dyn CargoMetadataLoader,
) -> Result<ProviderRequest, CliError> {
    match request {
        ProviderRequest::Registry { .. } => Ok(request),
        ProviderRequest::Path { path, package } => {
            let (package, package_path) = inspect_workspace(&path, package.as_deref(), loader)?;
            Ok(ProviderRequest::Path {
                path: package_path,
                package: Some(package),
            })
        }
        ProviderRequest::Git {
            url,
            rev,
            package: Some(package),
        } => Ok(ProviderRequest::Git {
            url,
            rev,
            package: Some(package),
        }),
        ProviderRequest::Git {
            url,
            rev,
            package: None,
        } => {
            let inspection = clone_git_for_inspection(project_root, &url, rev.as_deref())?;
            let (package, _) = inspect_workspace(inspection.path(), None, loader)?;
            Ok(ProviderRequest::Git {
                url,
                rev,
                package: Some(package),
            })
        }
    }
}

fn inspect_workspace(
    path: &Utf8Path,
    requested_package: Option<&str>,
    loader: &dyn CargoMetadataLoader,
) -> Result<(String, Utf8PathBuf), CliError> {
    let manifest = path.join("Cargo.toml");
    if !manifest.is_file() {
        return Err(CliError::MissingProviderManifest { path: manifest });
    }
    let metadata = loader.load(
        path,
        &manifest,
        CargoMetadataMode::Workspace,
        &mut Progress::Hidden,
    )?;
    let workspace = metadata.workspace_packages();
    let package = if let Some(requested) = requested_package {
        workspace
            .into_iter()
            .find(|package| package.name == requested)
            .ok_or_else(|| CliError::MissingProviderPackage {
                package: requested.to_owned(),
            })?
    } else {
        let mut providers = workspace
            .into_iter()
            .filter(|package| has_provider_metadata(package))
            .collect::<Vec<_>>();
        match providers.len() {
            1 => providers.remove(0),
            0 => {
                return Err(CliError::MissingProviderPackage {
                    package: path.to_string(),
                });
            }
            _ => {
                providers.sort_by_key(|package| package.name.to_string());
                return Err(CliError::AmbiguousProviderPackage {
                    packages: providers
                        .into_iter()
                        .map(|package| package.name.to_string())
                        .collect::<Vec<_>>()
                        .join(", "),
                });
            }
        }
    };
    ProviderMetadata::from_package(package)?;
    let package_directory =
        package
            .manifest_path
            .parent()
            .ok_or_else(|| CliError::InvalidProviderMetadata {
                package: package.name.to_string(),
                reason: "Cargo manifest has no package directory".to_owned(),
            })?;
    Ok((package.name.to_string(), package_directory.to_path_buf()))
}

fn has_provider_metadata(package: &Package) -> bool {
    package
        .metadata
        .as_object()
        .and_then(|metadata| metadata.get("geam"))
        .and_then(|geam| geam.as_object())
        .and_then(|geam| geam.get("provider"))
        .is_some()
}

#[derive(Debug)]
struct CandidateWorkspace {
    _directory: TempDir,
    manifest: Utf8PathBuf,
}

impl CandidateWorkspace {
    fn new(request: &ProviderRequest) -> Result<Self, CliError> {
        Self::new_with(request, create_candidate_directory)
    }

    fn new_with(
        request: &ProviderRequest,
        create_directory: fn() -> std::io::Result<TempDir>,
    ) -> Result<Self, CliError> {
        let directory = create_directory().map_err(CliError::TemporaryProviderWorkspace)?;
        Self::from_directory(request, directory)
    }

    fn from_directory(request: &ProviderRequest, directory: TempDir) -> Result<Self, CliError> {
        let path = directory.path().to_path_buf();
        Self::from_directory_path(request, directory, path)
    }

    fn from_directory_path(
        request: &ProviderRequest,
        directory: TempDir,
        path: std::path::PathBuf,
    ) -> Result<Self, CliError> {
        let root = Utf8PathBuf::from_path_buf(path).map_err(CliError::NonUtf8Path)?;
        let source_directory = root.join("src");
        fs::create_dir_all(&source_directory).map_err(|error| CliError::FileWrite {
            path: source_directory.clone(),
            error,
        })?;
        let manifest = root.join("Cargo.toml");
        let source = format!(
            "[package]\nname = \"geam-provider-candidate\"\nversion = \"0.0.0\"\nedition = \"2024\"\npublish = false\n\n[dependencies]\n{CANDIDATE_ALIAS} = {}\n\n[workspace]\nresolver = \"3\"\n",
            request.dependency_value()?,
        );
        fs::write(&manifest, source).map_err(|error| CliError::FileWrite {
            path: manifest.clone(),
            error,
        })?;
        let main = source_directory.join("main.rs");
        fs::write(&main, "fn main() {}\n")
            .map_err(|error| CliError::FileWrite { path: main, error })?;
        Ok(Self {
            _directory: directory,
            manifest,
        })
    }

    fn manifest(&self) -> &Utf8Path {
        &self.manifest
    }
}

fn create_candidate_directory() -> std::io::Result<TempDir> {
    tempfile::Builder::new()
        .prefix("geam-provider-candidate-")
        .tempdir()
}

impl ProviderRequest {
    fn dependency_value(&self) -> Result<String, CliError> {
        let crate_name = self
            .crate_name()
            .ok_or_else(|| CliError::InvalidCrateSpecification {
                spec: String::new(),
                reason: "provider package identity is unresolved".to_owned(),
            })?;
        let mut fields = vec![format!("package = {}", quoted(crate_name))];
        match self {
            Self::Registry { version, .. } => {
                if let Some(version) = version {
                    fields.push(format!("version = {}", quoted(&format!("={version}"))));
                } else {
                    fields.push("version = \"*\"".to_owned());
                }
            }
            Self::Path { path, .. } => {
                fields.push(format!("path = {}", quoted(path.as_str())));
            }
            Self::Git { url, rev, .. } => {
                fields.push(format!("git = {}", quoted(url)));
                if let Some(rev) = rev {
                    fields.push(format!("rev = {}", quoted(rev)));
                }
            }
        }
        Ok(format!("{{ {} }}", fields.join(", ")))
    }
}

fn resolved_dependency<'metadata>(
    metadata: &'metadata Metadata,
    alias: &str,
) -> Result<&'metadata Package, CliError> {
    let resolve = metadata
        .resolve
        .as_ref()
        .ok_or_else(|| CliError::MissingResolvedDependency {
            alias: alias.to_owned(),
        })?;
    let root = resolve
        .root
        .as_ref()
        .ok_or_else(|| CliError::MissingResolvedDependency {
            alias: alias.to_owned(),
        })?;
    let root = resolve
        .nodes
        .iter()
        .find(|node| &node.id == root)
        .ok_or_else(|| CliError::MissingResolvedDependency {
            alias: alias.to_owned(),
        })?;
    let dependency = root
        .deps
        .iter()
        .find(|dependency| dependency.name == alias)
        .ok_or_else(|| CliError::MissingResolvedDependency {
            alias: alias.to_owned(),
        })?;
    metadata
        .packages
        .iter()
        .find(|package| package.id == dependency.pkg)
        .ok_or_else(|| CliError::MissingResolvedDependency {
            alias: alias.to_owned(),
        })
}

#[derive(Debug)]
struct GitInspection {
    _directory: TempDir,
    path: Utf8PathBuf,
}

impl GitInspection {
    fn from_directory(url: &str, rev: Option<&str>, directory: TempDir) -> Result<Self, CliError> {
        let path = directory.path().to_path_buf();
        Self::from_directory_path(url, rev, directory, path)
    }

    fn from_directory_path(
        url: &str,
        rev: Option<&str>,
        directory: TempDir,
        path: std::path::PathBuf,
    ) -> Result<Self, CliError> {
        let path = canonical_utf8_path(path)?;
        run_checked(
            Command::new("git")
                .arg("clone")
                .arg("--quiet")
                .arg("--no-tags")
                .arg(url)
                .arg(&path),
        )?;
        if let Some(rev) = rev {
            run_checked(
                Command::new("git")
                    .arg("-C")
                    .arg(&path)
                    .arg("checkout")
                    .arg("--quiet")
                    .arg(rev),
            )?;
        }
        Ok(Self {
            _directory: directory,
            path,
        })
    }

    fn path(&self) -> &Utf8Path {
        &self.path
    }
}

fn clone_git_for_inspection(
    project_root: &Utf8Path,
    url: &str,
    rev: Option<&str>,
) -> Result<GitInspection, CliError> {
    clone_git_for_inspection_with(project_root, url, rev, create_git_inspection_directory)
}

fn clone_git_for_inspection_with(
    project_root: &Utf8Path,
    url: &str,
    rev: Option<&str>,
    create_directory: fn(&Utf8Path) -> std::io::Result<TempDir>,
) -> Result<GitInspection, CliError> {
    let parent = project_root.join("build/geam");
    fs::create_dir_all(&parent).map_err(|error| CliError::FileWrite {
        path: parent.clone(),
        error,
    })?;
    let directory = create_directory(&parent).map_err(CliError::TemporaryProviderWorkspace)?;
    GitInspection::from_directory(url, rev, directory)
}

fn create_git_inspection_directory(parent: &Utf8Path) -> std::io::Result<TempDir> {
    tempfile::Builder::new()
        .prefix("geam-provider-git-inspection-")
        .tempdir_in(parent)
}

fn canonical_provider_path(path: &Utf8Path) -> Result<Utf8PathBuf, CliError> {
    canonical_provider_path_from(fs::canonicalize(path).map_err(|error| CliError::FileRead {
        path: path.to_path_buf(),
        error,
    }))
}

fn canonical_provider_path_from(
    canonical: Result<std::path::PathBuf, CliError>,
) -> Result<Utf8PathBuf, CliError> {
    let canonical = canonical_utf8_path(canonical?)?;
    let manifest = canonical.join("Cargo.toml");
    if !manifest.is_file() {
        return Err(CliError::MissingProviderManifest { path: manifest });
    }
    Ok(canonical)
}

fn canonical_utf8_path(path: std::path::PathBuf) -> Result<Utf8PathBuf, CliError> {
    Utf8PathBuf::from_path_buf(path).map_err(CliError::NonUtf8Path)
}

fn parse_registry_specification(spec: &str) -> Result<(String, Option<Version>), CliError> {
    let (crate_name, version) = spec
        .rsplit_once('@')
        .map_or((spec, None), |(crate_name, version)| {
            (crate_name, Some(version))
        });
    if crate_name.is_empty() {
        return Err(CliError::InvalidCrateSpecification {
            spec: spec.to_owned(),
            reason: "crate name is empty".to_owned(),
        });
    }
    let version = version
        .map(|version| {
            if version.is_empty() {
                return Err(CliError::InvalidCrateSpecification {
                    spec: spec.to_owned(),
                    reason: "version is empty".to_owned(),
                });
            }
            version
                .parse()
                .map_err(|error| CliError::InvalidCrateSpecification {
                    spec: spec.to_owned(),
                    reason: format!("version must be exact Cargo SemVer: {error}"),
                })
        })
        .transpose()?;
    Ok((crate_name.to_owned(), version))
}

fn quoted(value: &str) -> String {
    toml::Value::String(value.to_owned()).to_string()
}

#[cfg(test)]
mod tests {
    use super::{
        CANDIDATE_ALIAS, CandidateWorkspace, GitInspection, ProviderRequest,
        canonical_provider_path, canonical_provider_path_from, clone_git_for_inspection,
        clone_git_for_inspection_with, complete_package_identity, inspect_workspace,
        parse_registry_specification, resolve_selection_with, resolve_with, resolve_with_candidate,
        resolved_dependency,
    };
    use crate::cargo::{CargoMetadataLoader, CargoMetadataMode, SystemCargoMetadata};
    use crate::command::AddProvider;
    use crate::error::CliError;
    use crate::progress::Progress;
    use crate::provider::manifest::{ProviderSelection, ProviderSource};
    use camino::{Utf8Path, Utf8PathBuf};
    use cargo_metadata::Metadata;
    use std::cell::RefCell;
    use std::fs;
    use std::io;
    use std::path::Path;
    use std::process::Command;
    use tempfile::{TempDir, tempdir};

    struct FailingLoader;

    impl CargoMetadataLoader for FailingLoader {
        fn load(
            &self,
            _current_directory: &camino::Utf8Path,
            manifest: &camino::Utf8Path,
            _mode: CargoMetadataMode,
            _progress: &mut Progress<'_>,
        ) -> Result<Metadata, CliError> {
            Err(CliError::InvalidCargoMetadata {
                manifest: manifest.to_path_buf(),
                reason: "fixture stop".to_owned(),
            })
        }
    }

    struct FixedLoader(Metadata);

    impl CargoMetadataLoader for FixedLoader {
        fn load(
            &self,
            _current_directory: &camino::Utf8Path,
            _manifest: &camino::Utf8Path,
            _mode: CargoMetadataMode,
            _progress: &mut Progress<'_>,
        ) -> Result<Metadata, CliError> {
            Ok(self.0.clone())
        }
    }

    struct RecordingFailingLoader {
        call: RefCell<Option<(Utf8PathBuf, CargoMetadataMode)>>,
    }

    impl CargoMetadataLoader for RecordingFailingLoader {
        fn load(
            &self,
            _current_directory: &camino::Utf8Path,
            manifest: &camino::Utf8Path,
            mode: CargoMetadataMode,
            _progress: &mut Progress<'_>,
        ) -> Result<Metadata, CliError> {
            self.call.replace(Some((manifest.to_path_buf(), mode)));
            Err(CliError::InvalidCargoMetadata {
                manifest: manifest.to_path_buf(),
                reason: "fixture stop".to_owned(),
            })
        }
    }

    #[test]
    fn converts_each_cli_source_into_a_provider_request() {
        let current = utf8_tempdir();
        let relative_provider = current.join("provider");
        write_provider_package(
            relative_provider.as_std_path(),
            "geam-images",
            "images",
            "1.0.0",
        );
        let absolute_provider = provider_package("geam-search", "search", "1.0.0");
        let absolute_provider_path = utf8_path(&absolute_provider);
        let canonical_absolute_provider_path = Utf8PathBuf::from_path_buf(
            fs::canonicalize(&absolute_provider_path).expect("provider path should canonicalize"),
        )
        .expect("provider path should be valid UTF-8");

        assert_eq!(
            ProviderRequest::from_command(
                current.as_std_path(),
                AddProvider {
                    crate_spec: Some("geam-images@1.2.3".to_owned()),
                    path: None,
                    git: None,
                    rev: None,
                    package: None,
                },
            )
            .expect("registry request should convert"),
            ProviderRequest::Registry {
                crate_name: "geam-images".to_owned(),
                version: Some("1.2.3".parse().expect("version should parse")),
            },
        );
        assert_eq!(
            ProviderRequest::from_command(
                current.as_std_path(),
                AddProvider {
                    crate_spec: None,
                    path: Some("provider".into()),
                    git: None,
                    rev: None,
                    package: None,
                },
            )
            .expect("relative path request should convert"),
            ProviderRequest::Path {
                path: Utf8PathBuf::from_path_buf(
                    fs::canonicalize(relative_provider).expect("provider path should canonicalize"),
                )
                .expect("provider path should be valid UTF-8"),
                package: None,
            },
        );
        assert_eq!(
            ProviderRequest::from_command(
                current.as_std_path(),
                AddProvider {
                    crate_spec: None,
                    path: Some(absolute_provider_path.clone()),
                    git: None,
                    rev: None,
                    package: Some("geam-search".to_owned()),
                },
            )
            .expect("absolute path request should convert"),
            ProviderRequest::Path {
                path: canonical_absolute_provider_path,
                package: Some("geam-search".to_owned()),
            },
        );
        assert_eq!(
            ProviderRequest::from_command(
                current.as_std_path(),
                AddProvider {
                    crate_spec: None,
                    path: None,
                    git: Some("https://example.com/provider.git".to_owned()),
                    rev: Some("abc123".to_owned()),
                    package: Some("geam-images".to_owned()),
                },
            )
            .expect("Git request should convert"),
            ProviderRequest::Git {
                url: "https://example.com/provider.git".to_owned(),
                rev: Some("abc123".to_owned()),
                package: Some("geam-images".to_owned()),
            },
        );
        let error = ProviderRequest::from_command(
            current.as_std_path(),
            AddProvider {
                crate_spec: None,
                path: None,
                git: None,
                rev: None,
                package: None,
            },
        )
        .expect_err("missing source should be rejected");
        assert!(matches!(
            error,
            CliError::InvalidCrateSpecification { spec, reason }
                if spec.is_empty() && reason == "one provider source is required"
        ));
        let error = ProviderRequest::from_command(
            current.as_std_path(),
            AddProvider {
                crate_spec: None,
                path: Some("missing".into()),
                git: None,
                rev: None,
                package: None,
            },
        )
        .expect_err("missing path should be rejected");
        assert!(matches!(
            error,
            CliError::FileRead { path, error }
                if path == current.join("missing")
                    && error.kind() == std::io::ErrorKind::NotFound
        ));
    }

    #[cfg(unix)]
    #[test]
    fn rejects_non_utf8_command_and_canonical_paths() {
        use std::ffi::OsString;
        use std::os::unix::ffi::OsStringExt;

        let invalid_current = std::path::PathBuf::from(OsString::from_vec(vec![0xff]));
        let error = ProviderRequest::from_command(
            &invalid_current,
            AddProvider {
                crate_spec: None,
                path: Some("provider".into()),
                git: None,
                rev: None,
                package: None,
            },
        )
        .expect_err("non-UTF-8 current directory should be rejected");
        assert!(matches!(
            error,
            CliError::NonUtf8Path(path) if path == invalid_current
        ));

        let invalid_canonical = std::path::PathBuf::from(OsString::from_vec(vec![0xfe]));
        let error = canonical_provider_path_from(Ok(invalid_canonical.clone()))
            .expect_err("non-UTF-8 canonical path should be rejected");
        assert!(matches!(
            error,
            CliError::NonUtf8Path(path) if path == invalid_canonical
        ));
    }

    #[test]
    fn parses_registry_crate_specifications() {
        let unversioned =
            parse_registry_specification("geam-images").expect("unversioned crate should parse");
        assert_eq!(unversioned, ("geam-images".to_owned(), None),);
        let explicitly_named = parse_registry_specification("geam-company_image")
            .expect("explicit selection should accept an arbitrary crate name");
        assert_eq!(explicitly_named, ("geam-company_image".to_owned(), None));
        let versioned = parse_registry_specification("geam-images@1.2.3")
            .expect("versioned crate should parse");
        assert_eq!(
            versioned,
            (
                "geam-images".to_owned(),
                Some("1.2.3".parse().expect("version should parse")),
            ),
        );
        for (specification, expected_reason, exact) in [
            ("@1.0.0", "crate name is empty", true),
            ("geam-images@", "version is empty", true),
            (
                "geam-images@^1",
                "version must be exact Cargo SemVer:",
                false,
            ),
        ] {
            let error = parse_registry_specification(specification)
                .expect_err("invalid registry specification should be rejected");
            assert!(
                matches!(
                    &error,
                    CliError::InvalidCrateSpecification { spec, reason }
                        if spec == specification
                            && if exact {
                                reason == expected_reason
                            } else {
                                reason.starts_with(expected_reason) && reason.contains('^')
                            }
                ),
                "expected {expected_reason}: {error}",
            );
        }

        let error = ProviderRequest::from_command(
            Path::new("/workspace"),
            AddProvider {
                crate_spec: Some("geam-images@^1".to_owned()),
                path: None,
                git: None,
                rev: None,
                package: None,
            },
        )
        .expect_err("invalid registry command should preserve its parse failure");
        assert!(matches!(
            error,
            CliError::InvalidCrateSpecification { spec, reason }
                if spec == "geam-images@^1"
                    && reason == "version must be exact Cargo SemVer: unexpected character '^' while parsing major version number"
        ));
    }

    #[test]
    fn renders_all_candidate_dependency_sources_in_disposable_workspaces() {
        let requests = [
            ProviderRequest::Registry {
                crate_name: "geam-images".to_owned(),
                version: None,
            },
            ProviderRequest::Registry {
                crate_name: "geam-images".to_owned(),
                version: Some("1.2.3".parse().expect("version should parse")),
            },
            ProviderRequest::Path {
                path: "/provider".into(),
                package: Some("geam-images".to_owned()),
            },
            ProviderRequest::Git {
                url: "https://example.com/provider.git".to_owned(),
                rev: None,
                package: Some("geam-images".to_owned()),
            },
            ProviderRequest::Git {
                url: "https://example.com/provider.git".to_owned(),
                rev: Some("abc123".to_owned()),
                package: Some("geam-images".to_owned()),
            },
        ];

        let expected = [
            "version = \"*\"",
            "version = \"=1.2.3\"",
            "path = \"/provider\"",
            "git = \"https://example.com/provider.git\"",
            "rev = \"abc123\"",
        ];
        for (request, expected) in requests.into_iter().zip(expected) {
            let candidate =
                CandidateWorkspace::new(&request).expect("candidate workspace should be written");
            let directory = candidate
                .manifest()
                .parent()
                .expect("candidate manifest should have a directory")
                .to_path_buf();
            let source =
                fs::read_to_string(candidate.manifest()).expect("manifest should be readable");
            assert!(source.contains(expected), "missing {expected}: {source}");
            drop(candidate);
            assert!(!directory.exists());
        }
    }

    #[test]
    fn preserves_temporary_candidate_directory_creation_failures() {
        let error = CandidateWorkspace::new_with(&registry_request(), || {
            Err(io::Error::new(
                io::ErrorKind::PermissionDenied,
                "fixture temporary-directory failure",
            ))
        })
        .expect_err("temporary directory failure should be preserved");

        assert!(matches!(
            error,
            CliError::TemporaryProviderWorkspace(error)
                if error.kind() == io::ErrorKind::PermissionDenied
                    && error.to_string() == "fixture temporary-directory failure"
        ));
    }

    #[test]
    fn propagates_candidate_workspace_failures_from_explicit_resolution() {
        let project = utf8_tempdir();
        let error = resolve_with_candidate(&project, registry_request(), &FailingLoader, |_| {
            Err(CliError::TemporaryProviderWorkspace(io::Error::new(
                io::ErrorKind::PermissionDenied,
                "fixture candidate failure",
            )))
        })
        .err()
        .expect("candidate construction failure should be preserved");

        assert!(matches!(
            error,
            CliError::TemporaryProviderWorkspace(error)
                if error.kind() == io::ErrorKind::PermissionDenied
                    && error.to_string() == "fixture candidate failure"
        ));
    }

    #[cfg(unix)]
    #[test]
    fn removes_candidate_workspace_after_each_file_construction_failure() {
        assert_candidate_file_failure(
            |root| fs::write(root.join("src"), "blocked").expect("blocker should be written"),
            "src",
            io::ErrorKind::AlreadyExists,
        );
        assert_candidate_file_failure(
            |root| {
                fs::create_dir(root.join("src")).expect("source directory should be created");
                fs::create_dir(root.join("Cargo.toml"))
                    .expect("manifest blocker should be created");
            },
            "Cargo.toml",
            io::ErrorKind::IsADirectory,
        );
        assert_candidate_file_failure(
            |root| {
                fs::create_dir(root.join("src")).expect("source directory should be created");
                fs::create_dir(root.join("src/main.rs")).expect("source blocker should be created");
            },
            "src/main.rs",
            io::ErrorKind::IsADirectory,
        );
    }

    #[cfg(unix)]
    #[test]
    fn removes_non_utf8_candidate_workspace_after_rejection() {
        use std::ffi::OsString;
        use std::os::unix::ffi::OsStringExt;

        let directory = tempdir().expect("candidate directory should be created");
        let candidate_path = directory.path().to_path_buf();
        let non_utf8_path = std::path::PathBuf::from(OsString::from_vec(vec![0xff]));

        let error = CandidateWorkspace::from_directory_path(
            &registry_request(),
            directory,
            non_utf8_path.clone(),
        )
        .expect_err("non-UTF-8 candidate path should be rejected");

        assert!(matches!(
            error,
            CliError::NonUtf8Path(ref path) if path == &non_utf8_path
        ));
        assert!(!candidate_path.exists());
    }

    #[test]
    fn resolves_a_path_provider_through_real_cargo_metadata() {
        let project = utf8_tempdir();
        let provider = provider_package("geam-images", "images", ">= 2.0.0 and < 3.0.0");
        let provider_path = utf8_path(&provider);
        let loader = SystemCargoMetadata;

        let resolved = resolve_with(
            &project,
            ProviderRequest::Path {
                path: provider_path.clone(),
                package: None,
            },
            &loader,
        )
        .expect("path provider should resolve");

        assert_eq!(resolved.metadata.crate_name(), "geam-images");
        assert_eq!(
            resolved.source,
            ProviderSource::Path {
                path: provider_path.clone(),
            },
        );
        assert!(
            !provider_path.join("Cargo.lock").exists(),
            "workspace inspection must not create a provider-owned lock",
        );
        assert!(
            !project.join("build/geam/provider-candidate").exists(),
            "candidate resolution must not persist a project-owned workspace",
        );
    }

    #[test]
    fn removes_candidate_state_after_metadata_failure_without_touching_the_root_lock() {
        let project = utf8_tempdir();
        let root_lock = project.join("Cargo.lock");
        fs::write(&root_lock, "root lock\n").expect("root lock fixture should be written");
        let loader = RecordingFailingLoader {
            call: RefCell::new(None),
        };

        let error = resolve_with(
            &project,
            ProviderRequest::Registry {
                crate_name: "geam-images".to_owned(),
                version: None,
            },
            &loader,
        )
        .err()
        .expect("metadata failure should be preserved");

        let (manifest, mode) = loader
            .call
            .borrow()
            .clone()
            .expect("candidate metadata call should be recorded");
        assert_eq!(mode, CargoMetadataMode::Resolve);
        assert!(matches!(
            error,
            CliError::InvalidCargoMetadata {
                manifest: error_manifest,
                reason,
            } if error_manifest == manifest && reason == "fixture stop"
        ));
        assert!(
            !manifest
                .parent()
                .expect("candidate manifest should have a parent")
                .exists(),
        );
        assert_eq!(
            fs::read_to_string(root_lock).expect("root lock should remain readable"),
            "root lock\n",
        );
    }

    #[test]
    fn applies_project_cargo_configuration_to_candidate_and_selected_resolution() {
        let project = utf8_tempdir();
        let provider = provider_package("geam-images", "images", ">= 2.0.0 and < 3.0.0");
        let provider_path = utf8_path(&provider);
        fs::create_dir(project.join(".cargo"))
            .expect("project Cargo configuration directory should be created");
        fs::write(
            project.join(".cargo/config.toml"),
            format!(
                "[net]\noffline = true\n\n[patch.crates-io]\ngeam-images = {{ path = {path} }}\n",
                path = super::quoted(provider_path.as_str()),
            ),
        )
        .expect("project Cargo configuration should be written");
        let version: semver::Version = "1.2.3".parse().expect("version should parse");

        let resolved = resolve_with(
            &project,
            ProviderRequest::Registry {
                crate_name: "geam-images".to_owned(),
                version: Some(version.clone()),
            },
            &SystemCargoMetadata,
        )
        .expect("candidate resolution should use the project Cargo patch");
        assert_eq!(resolved.metadata.crate_name(), "geam-images");
        assert_eq!(resolved.metadata.gleam_package(), "images");
        assert_eq!(
            resolved.source,
            ProviderSource::Registry {
                version: version.clone(),
            },
        );

        let manifest = project.join("Cargo.toml");
        fs::write(
            &manifest,
            format!(
                "[package]\nname = \"fixture-runner\"\nversion = \"0.0.0\"\nedition = \"2024\"\n\n[dependencies]\ngeam_provider_images = {{ package = \"geam-images\", version = \"={version}\" }}\n\n[workspace]\nresolver = \"3\"\n",
            ),
        )
        .expect("managed manifest fixture should be written");
        fs::create_dir(project.join("src")).expect("runner source directory should be created");
        fs::write(project.join("src/main.rs"), "fn main() {}\n")
            .expect("runner source fixture should be written");
        let status = Command::new("cargo")
            .arg("generate-lockfile")
            .arg("--manifest-path")
            .arg(&manifest)
            .current_dir(&project)
            .status()
            .expect("Cargo should start");
        assert!(status.success());
        let root_lock = project.join("Cargo.lock");
        let lock_before = fs::read(&root_lock).expect("root lock should be readable");
        let selection = ProviderSelection::new(
            "images".to_owned(),
            "geam-images".to_owned(),
            ProviderSource::Registry { version },
        );

        let metadata = resolve_selection_with(
            &project,
            &selection,
            &SystemCargoMetadata,
            &mut Progress::Hidden,
        )
        .expect("selected provider should resolve from the managed graph");

        assert_eq!(metadata.crate_name(), "geam-images");
        assert_eq!(metadata.gleam_package(), "images");
        assert_eq!(
            fs::read(&root_lock).expect("root lock should remain readable"),
            lock_before,
        );
        fs::remove_file(&root_lock).expect("root lock should be removable");
        let error = resolve_selection_with(
            &project,
            &selection,
            &SystemCargoMetadata,
            &mut Progress::Hidden,
        )
        .expect_err("selected provider resolution must not recreate a missing root lock");
        assert!(matches!(
            error,
            CliError::ProcessFailure {
                command,
                status: Some(101),
                stderr,
            } if command
                == format!(
                    "cargo metadata --format-version 1 --manifest-path {manifest} --locked"
                )
                && stderr.contains("the lock file")
        ));
        assert!(!root_lock.exists());
    }

    #[test]
    fn preserves_missing_selected_dependency_aliases_from_the_root_graph() {
        let project = utf8_tempdir();
        let provider = provider_package("geam-images", "images", "1.0.0");
        let request = ProviderRequest::Path {
            path: utf8_path(&provider),
            package: Some("geam-images".to_owned()),
        };
        let candidate =
            CandidateWorkspace::new(&request).expect("candidate workspace should be written");
        let metadata = SystemCargoMetadata
            .load(
                &project,
                candidate.manifest(),
                CargoMetadataMode::Resolve,
                &mut Progress::Hidden,
            )
            .expect("candidate metadata should load");
        let selection = ProviderSelection::new(
            "images".to_owned(),
            "geam-images".to_owned(),
            ProviderSource::Path {
                path: utf8_path(&provider),
            },
        );

        let error = resolve_selection_with(
            &project,
            &selection,
            &FixedLoader(metadata),
            &mut Progress::Hidden,
        )
        .expect_err("missing selected dependency alias should be preserved");

        assert!(matches!(
            error,
            CliError::MissingResolvedDependency { ref alias }
                if alias == "geam_provider_images"
        ));
    }

    #[test]
    fn derives_recorded_sources_from_completed_requests() {
        let provider = provider_package("geam-images", "images", "1.0.0");
        let path = utf8_path(&provider);
        let metadata = SystemCargoMetadata
            .load(
                &path,
                &path.join("Cargo.toml"),
                CargoMetadataMode::Workspace,
                &mut Progress::Hidden,
            )
            .expect("provider metadata should load");
        let package = metadata
            .packages
            .first()
            .expect("provider package should be present");

        assert_eq!(
            ProviderRequest::Registry {
                crate_name: "geam-images".to_owned(),
                version: None,
            }
            .provider_source(package),
            ProviderSource::Registry {
                version: "1.2.3".parse().expect("version should parse"),
            },
        );
        assert_eq!(
            ProviderRequest::Path {
                path: path.clone(),
                package: Some("geam-images".to_owned()),
            }
            .provider_source(package),
            ProviderSource::Path { path: path.clone() },
        );
        assert_eq!(
            ProviderRequest::Git {
                url: "https://example.com/provider.git".to_owned(),
                rev: Some("abc123".to_owned()),
                package: Some("geam-images".to_owned()),
            }
            .provider_source(package),
            ProviderSource::Git {
                url: "https://example.com/provider.git".to_owned(),
                rev: Some("abc123".to_owned()),
            },
        );
    }

    #[test]
    fn preserves_already_complete_registry_and_git_identities() {
        let project = utf8_tempdir();
        let registry = ProviderRequest::Registry {
            crate_name: "geam-images".to_owned(),
            version: None,
        };
        assert_eq!(
            complete_package_identity(&project, registry.clone(), &FailingLoader)
                .expect("registry identity should already be complete"),
            registry,
        );

        let git = ProviderRequest::Git {
            url: "https://example.com/provider.git".to_owned(),
            rev: None,
            package: Some("geam-images".to_owned()),
        };
        assert_eq!(
            complete_package_identity(&project, git.clone(), &FailingLoader)
                .expect("explicit Git package should already be complete"),
            git,
        );
    }

    #[test]
    fn rejects_missing_ambiguous_and_unknown_workspace_packages() {
        let project = utf8_tempdir();
        let workspace = tempdir().expect("workspace should be created");
        fs::write(
            workspace.path().join("Cargo.toml"),
            "[workspace]\nmembers = [\"one\", \"two\"]\nresolver = \"3\"\n",
        )
        .expect("workspace manifest should be written");
        for (directory, package) in [("one", "geam-one"), ("two", "geam-two")] {
            write_provider_package(
                &workspace.path().join(directory),
                package,
                directory,
                "1.0.0",
            );
        }
        let workspace_path = Utf8PathBuf::from_path_buf(workspace.path().to_path_buf())
            .expect("workspace path should be valid UTF-8");
        let loader = SystemCargoMetadata;

        let error = complete_package_identity(
            &project,
            ProviderRequest::Path {
                path: workspace_path.clone(),
                package: None,
            },
            &loader,
        )
        .expect_err("ambiguous workspace should be rejected");
        assert!(matches!(
            error,
            CliError::AmbiguousProviderPackage { packages }
                if packages == "geam-one, geam-two"
        ));
        let error = complete_package_identity(
            &project,
            ProviderRequest::Path {
                path: workspace_path.clone(),
                package: Some("missing".to_owned()),
            },
            &loader,
        )
        .expect_err("unknown explicit package should be rejected");
        assert!(matches!(
            error,
            CliError::MissingProviderPackage { package } if package == "missing"
        ));

        let plain = tempdir().expect("plain package should be created");
        fs::create_dir(plain.path().join("src")).expect("source should be created");
        fs::write(
            plain.path().join("Cargo.toml"),
            "[package]\nname = \"plain\"\nversion = \"1.0.0\"\nedition = \"2024\"\n",
        )
        .expect("plain manifest should be written");
        fs::write(plain.path().join("src/lib.rs"), "").expect("source should be written");
        let plain = Utf8PathBuf::from_path_buf(plain.path().to_path_buf())
            .expect("plain path should be valid UTF-8");
        let error = complete_package_identity(
            &project,
            ProviderRequest::Path {
                path: plain.clone(),
                package: None,
            },
            &loader,
        )
        .expect_err("metadata-free workspace should be rejected");
        assert!(matches!(
            error,
            CliError::MissingProviderPackage { package } if package == plain.as_str()
        ));
        let error = complete_package_identity(
            &project,
            ProviderRequest::Path {
                path: plain,
                package: Some("plain".to_owned()),
            },
            &loader,
        )
        .expect_err("explicit non-provider package should be rejected");
        assert!(matches!(
            error,
            CliError::InvalidProviderMetadata { package, reason }
                if package == "plain"
                    && reason == "missing [package.metadata.geam.provider] table"
        ));
    }

    #[test]
    fn validates_explicit_workspace_package_metadata() {
        let project = utf8_tempdir();
        let provider = provider_package("geam-images", "images", "1.0.0");
        let loader = SystemCargoMetadata;
        let path = utf8_path(&provider);

        let completed = complete_package_identity(
            &project,
            ProviderRequest::Path {
                path: path.clone(),
                package: Some("geam-images".to_owned()),
            },
            &loader,
        )
        .expect("explicit package should validate");

        assert_eq!(
            completed,
            ProviderRequest::Path {
                path,
                package: Some("geam-images".to_owned()),
            },
        );
    }

    #[test]
    fn rejects_missing_manifests_and_manifest_paths_without_a_parent() {
        let project = utf8_tempdir();
        let error = inspect_workspace(&project, None, &FailingLoader)
            .expect_err("missing provider manifest should be rejected");
        assert!(matches!(
            error,
            CliError::MissingProviderManifest { path } if path == project.join("Cargo.toml")
        ));

        let provider = provider_package("geam-images", "images", "1.0.0");
        let path = utf8_path(&provider);
        let error = inspect_workspace(&path, None, &FailingLoader)
            .expect_err("Cargo metadata failure should be preserved");
        assert!(matches!(
            error,
            CliError::InvalidCargoMetadata { manifest, reason }
                if manifest == path.join("Cargo.toml") && reason == "fixture stop"
        ));
        let mut metadata = SystemCargoMetadata
            .load(
                &path,
                &path.join("Cargo.toml"),
                CargoMetadataMode::Workspace,
                &mut Progress::Hidden,
            )
            .expect("provider metadata should load");
        metadata
            .packages
            .first_mut()
            .expect("provider package should be present")
            .manifest_path = Utf8PathBuf::new();
        assert!(matches!(
            inspect_workspace(&path, None, &FixedLoader(metadata))
                .expect_err("missing package directory should be rejected"),
            CliError::InvalidProviderMetadata { package, reason }
                if package == "geam-images"
                    && reason == "Cargo manifest has no package directory"
        ));
    }

    #[test]
    fn rejects_candidate_requests_without_a_resolved_package_identity() {
        let unresolved = ProviderRequest::Path {
            path: "/provider".into(),
            package: None,
        };
        let directory = tempdir().expect("candidate directory should be created");
        let candidate_path = directory.path().to_path_buf();
        let error = CandidateWorkspace::from_directory(&unresolved, directory)
            .expect_err("unresolved provider identity should be rejected");
        assert!(matches!(
            error,
            CliError::InvalidCrateSpecification { spec, reason }
                if spec.is_empty() && reason == "provider package identity is unresolved"
        ));
        assert!(!candidate_path.exists());
    }

    #[test]
    fn inspects_local_git_repositories_when_package_is_omitted() {
        let project = utf8_tempdir();
        let repository = provider_package("geam-images", "images", "1.0.0");
        initialize_git_repository(&repository);
        let repository_path = utf8_path(&repository);
        let inspected = clone_git_for_inspection(&project, repository_path.as_str(), Some("HEAD"))
            .expect("local repository should clone");
        let inspection_path = inspected.path().to_path_buf();
        assert!(inspection_path.join("Cargo.toml").is_file());
        drop(inspected);
        assert_eq!(
            inspection_path.parent(),
            Some(project.join("build/geam").as_path()),
        );
        assert!(
            inspection_path
                .file_name()
                .expect("inspection path should have a name")
                .starts_with("geam-provider-git-inspection-"),
        );
        assert!(!inspection_path.exists());

        let completed = complete_package_identity(
            &project,
            ProviderRequest::Git {
                url: repository_path.to_string(),
                rev: Some("HEAD".to_owned()),
                package: None,
            },
            &SystemCargoMetadata,
        )
        .expect("Git package should be found");
        assert_eq!(completed.crate_name(), Some("geam-images"));
        assert_git_inspection_removed(&project);
    }

    #[test]
    fn preserves_git_inspection_filesystem_and_process_failures() {
        let blocked_parent = utf8_tempdir();
        fs::write(blocked_parent.join("build"), "not a directory")
            .expect("blocking file should be written");
        let blocked_parent_path = blocked_parent.join("build/geam");
        let expected_kind = fs::create_dir_all(&blocked_parent_path)
            .expect_err("creating a directory below a file should fail")
            .kind();
        let error = clone_git_for_inspection(&blocked_parent, "missing", None)
            .expect_err("blocked inspection parent should fail");
        assert!(matches!(
            error,
            CliError::FileWrite { path, error }
                if path == blocked_parent_path && error.kind() == expected_kind
        ));

        let temporary_failure = utf8_tempdir();
        let error = clone_git_for_inspection_with(&temporary_failure, "missing", None, |_| {
            Err(io::Error::new(
                io::ErrorKind::PermissionDenied,
                "fixture Git inspection failure",
            ))
        })
        .expect_err("temporary inspection directory failure should be preserved");
        assert!(matches!(
            error,
            CliError::TemporaryProviderWorkspace(error)
                if error.kind() == io::ErrorKind::PermissionDenied
                    && error.to_string() == "fixture Git inspection failure"
        ));
        assert_git_inspection_removed(&temporary_failure);

        let missing_repository = utf8_tempdir();
        let error =
            clone_git_for_inspection(&missing_repository, "/repository/that/does/not/exist", None)
                .expect_err("missing repository should fail to clone");
        assert_missing_git_clone(error, &missing_repository);
        let error = complete_package_identity(
            &missing_repository,
            ProviderRequest::Git {
                url: "/repository/that/does/not/exist".to_owned(),
                rev: None,
                package: None,
            },
            &SystemCargoMetadata,
        )
        .expect_err("Git clone failure should stop package completion");
        assert_missing_git_clone(error, &missing_repository);
        assert_git_inspection_removed(&missing_repository);

        let plain = tempdir().expect("plain package should be created");
        fs::create_dir(plain.path().join("src")).expect("source should be created");
        fs::write(
            plain.path().join("Cargo.toml"),
            "[package]\nname = \"plain\"\nversion = \"1.0.0\"\nedition = \"2024\"\n",
        )
        .expect("plain manifest should be written");
        fs::write(plain.path().join("src/lib.rs"), "").expect("source should be written");
        initialize_git_repository(&plain);
        let inspection_project = utf8_tempdir();
        let error = complete_package_identity(
            &inspection_project,
            ProviderRequest::Git {
                url: utf8_path(&plain).to_string(),
                rev: None,
                package: None,
            },
            &SystemCargoMetadata,
        )
        .expect_err("metadata-free Git package should be rejected after inspection");
        let inspection_prefix = format!(
            "{}/geam-provider-git-inspection-",
            inspection_project.join("build/geam"),
        );
        assert!(matches!(
            error,
            CliError::MissingProviderPackage { package }
                if package.starts_with(&inspection_prefix)
        ));
        assert_git_inspection_removed(&inspection_project);

        let repository = provider_package("geam-images", "images", "1.0.0");
        initialize_git_repository(&repository);
        let repository_path = utf8_path(&repository);
        let project = utf8_tempdir();
        clone_git_for_inspection(&project, repository_path.as_str(), None)
            .expect("repository should clone without an explicit revision");
        assert_git_inspection_removed(&project);
        let error =
            clone_git_for_inspection(&project, repository_path.as_str(), Some("missing-revision"))
                .expect_err("missing revision should fail checkout");
        let checkout_prefix = format!(
            "git -C {}/geam-provider-git-inspection-",
            project.join("build/geam"),
        );
        assert!(matches!(
            error,
            CliError::ProcessFailure {
                command,
                status: Some(1),
                stderr,
            } if stderr.contains("pathspec")
                && command.starts_with(&checkout_prefix)
                && command.ends_with(" checkout --quiet missing-revision")
        ));
        assert_git_inspection_removed(&project);
    }

    #[cfg(unix)]
    #[test]
    fn removes_non_utf8_git_inspection_directories_after_rejection() {
        use std::ffi::OsString;
        use std::os::unix::ffi::OsStringExt;

        let directory = tempdir().expect("inspection directory should be created");
        let directory_path = directory.path().to_path_buf();
        let path = std::path::PathBuf::from(OsString::from_vec(b"inspection-\xff".to_vec()));
        let error = GitInspection::from_directory_path("missing", None, directory, path.clone())
            .expect_err("non-UTF-8 inspection path should be rejected");

        assert!(matches!(
            error,
            CliError::NonUtf8Path(actual) if actual == path
        ));
        assert!(!directory_path.exists());
    }

    #[test]
    fn distinguishes_missing_provider_paths_from_missing_manifests() {
        let project = utf8_tempdir();
        let missing = project.join("missing");
        let error =
            canonical_provider_path(&missing).expect_err("missing provider path should fail");
        assert!(matches!(
            error,
            CliError::FileRead { path, error }
                if path == missing && error.kind() == io::ErrorKind::NotFound
        ));

        let empty = project.join("empty");
        fs::create_dir(&empty).expect("empty provider directory should be created");
        let canonical_empty = Utf8PathBuf::from_path_buf(
            fs::canonicalize(&empty).expect("empty provider directory should canonicalize"),
        )
        .expect("canonical provider directory should be valid UTF-8");
        let error = canonical_provider_path(&empty)
            .expect_err("provider directory without Cargo manifest should fail");
        assert!(matches!(
            error,
            CliError::MissingProviderManifest { path }
                if path == canonical_empty.join("Cargo.toml")
        ));
    }

    #[test]
    fn preserves_candidate_metadata_failures() {
        let project = utf8_tempdir();
        let error = resolve_with(
            &project,
            ProviderRequest::Registry {
                crate_name: "geam-images".to_owned(),
                version: None,
            },
            &FailingLoader,
        )
        .err()
        .expect("failing metadata should be preserved");

        assert!(matches!(
            error,
            CliError::InvalidCargoMetadata { manifest, reason }
                if manifest.file_name() == Some("Cargo.toml")
                    && manifest
                        .parent()
                        .is_some_and(|parent| parent.file_name().is_some_and(|name| {
                            name.starts_with("geam-provider-candidate-")
                        }))
                    && reason == "fixture stop"
        ));
    }

    #[test]
    fn propagates_each_candidate_resolution_phase_failure() {
        let project = utf8_tempdir();
        let missing = ProviderRequest::Path {
            path: project.join("missing"),
            package: None,
        };
        let error = resolve_with(&project, missing, &FailingLoader)
            .err()
            .expect("missing path should fail package completion");
        assert!(matches!(
            error,
            CliError::MissingProviderManifest { path }
                if path == project.join("missing/Cargo.toml")
        ));

        let provider = provider_package("geam-images", "images", "1.0.0");
        let path_request = ProviderRequest::Path {
            path: utf8_path(&provider),
            package: Some("geam-images".to_owned()),
        };
        let candidate =
            CandidateWorkspace::new(&path_request).expect("candidate manifest should be written");
        let metadata = SystemCargoMetadata
            .load(
                &project,
                candidate.manifest(),
                CargoMetadataMode::Resolve,
                &mut Progress::Hidden,
            )
            .expect("candidate metadata should load");
        let resolution_request = ProviderRequest::Registry {
            crate_name: "geam-images".to_owned(),
            version: None,
        };

        let mut missing_edge = metadata.clone();
        missing_edge.resolve = None;
        let error = resolve_with(
            &project,
            resolution_request.clone(),
            &FixedLoader(missing_edge),
        )
        .err()
        .expect("missing candidate edge should fail");
        assert!(matches!(
            error,
            CliError::MissingResolvedDependency { alias } if alias == CANDIDATE_ALIAS
        ));

        let mut invalid_provider = metadata;
        let candidate = resolved_dependency(&invalid_provider, CANDIDATE_ALIAS)
            .expect("candidate dependency should be present")
            .id
            .clone();
        invalid_provider
            .packages
            .iter_mut()
            .find(|package| package.id == candidate)
            .expect("candidate package should be present")
            .metadata = Default::default();
        let error = resolve_with(&project, resolution_request, &FixedLoader(invalid_provider))
            .err()
            .expect("invalid provider metadata should fail");
        assert!(matches!(
            error,
            CliError::InvalidProviderMetadata { package, reason }
                if package == "geam-images"
                    && reason == "missing [package.metadata.geam.provider] table"
        ));
    }

    #[test]
    fn reports_missing_candidate_edges() {
        let project = utf8_tempdir();
        let provider = provider_package("geam-images", "images", "1.0.0");
        let request = ProviderRequest::Path {
            path: utf8_path(&provider),
            package: Some("geam-images".to_owned()),
        };
        let candidate =
            CandidateWorkspace::new(&request).expect("candidate manifest should be written");
        let metadata = SystemCargoMetadata
            .load(
                &project,
                candidate.manifest(),
                CargoMetadataMode::Resolve,
                &mut Progress::Hidden,
            )
            .expect("candidate metadata should load");
        assert_eq!(
            resolved_dependency(&metadata, CANDIDATE_ALIAS)
                .expect("candidate dependency should be present")
                .name,
            "geam-images",
        );

        let mut missing_resolve = metadata.clone();
        missing_resolve.resolve = None;
        assert_missing_candidate(&missing_resolve);

        let mut missing_root = metadata.clone();
        missing_root
            .resolve
            .as_mut()
            .expect("resolve should be present")
            .root = None;
        assert_missing_candidate(&missing_root);

        let mut missing_root_node = metadata.clone();
        missing_root_node
            .resolve
            .as_mut()
            .expect("resolve should be present")
            .nodes
            .clear();
        assert_missing_candidate(&missing_root_node);

        let mut missing_edge = metadata.clone();
        let resolve = missing_edge
            .resolve
            .as_mut()
            .expect("resolve should be present");
        let root = resolve.root.clone().expect("root should be present");
        resolve
            .nodes
            .iter_mut()
            .find(|node| node.id == root)
            .expect("root node should be present")
            .deps
            .clear();
        assert_missing_candidate(&missing_edge);

        let mut missing_package = metadata;
        missing_package.packages.clear();
        assert_missing_candidate(&missing_package);
    }

    fn assert_missing_candidate(metadata: &Metadata) {
        assert!(matches!(
            resolved_dependency(metadata, CANDIDATE_ALIAS)
                .expect_err("candidate dependency should be absent"),
            CliError::MissingResolvedDependency { alias } if alias == CANDIDATE_ALIAS
        ));
    }

    fn assert_missing_git_clone(error: CliError, project: &Utf8Path) {
        let command_prefix = format!(
            "git clone --quiet --no-tags /repository/that/does/not/exist {}/geam-provider-git-inspection-",
            project.join("build/geam"),
        );
        assert!(matches!(
            error,
            CliError::ProcessFailure {
                command,
                status: Some(128),
                stderr,
            } if stderr.contains("does not exist")
                && command.starts_with(&command_prefix)
        ));
    }

    fn assert_git_inspection_removed(project: &Utf8Path) {
        let entries = fs::read_dir(project.join("build/geam"))
            .expect("inspection parent should remain readable")
            .count();
        assert_eq!(entries, 0);
    }

    fn provider_package(name: &str, gleam_package: &str, range: &str) -> TempDir {
        let directory = tempdir().expect("provider package should be created");
        write_provider_package(directory.path(), name, gleam_package, range);
        directory
    }

    fn write_provider_package(
        directory: &std::path::Path,
        name: &str,
        gleam_package: &str,
        range: &str,
    ) {
        fs::create_dir_all(directory.join("src")).expect("source directory should be created");
        fs::write(
            directory.join("Cargo.toml"),
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
        fs::write(directory.join("src/lib.rs"), "pub struct Component;\n")
            .expect("provider source should be written");
    }

    fn initialize_git_repository(repository: &TempDir) {
        for arguments in [
            vec!["init", "--quiet"],
            vec!["add", "."],
            vec![
                "-c",
                "user.name=Geam Test",
                "-c",
                "user.email=geam@example.com",
                "commit",
                "--quiet",
                "-m",
                "fixture",
            ],
        ] {
            assert!(
                Command::new("git")
                    .args(arguments)
                    .current_dir(repository.path())
                    .status()
                    .expect("git should start")
                    .success(),
            );
        }
    }

    fn utf8_tempdir() -> Utf8PathBuf {
        let path = tempdir()
            .expect("temporary directory should be created")
            .keep();
        Utf8PathBuf::from_path_buf(path).expect("temporary path should be valid UTF-8")
    }

    fn utf8_path(directory: &TempDir) -> Utf8PathBuf {
        Utf8PathBuf::from_path_buf(directory.path().to_path_buf())
            .expect("temporary path should be valid UTF-8")
    }

    fn registry_request() -> ProviderRequest {
        ProviderRequest::Registry {
            crate_name: "geam-images".to_owned(),
            version: None,
        }
    }

    #[cfg(unix)]
    fn assert_candidate_file_failure(
        setup: impl FnOnce(&camino::Utf8Path),
        expected_path: &str,
        expected_kind: io::ErrorKind,
    ) {
        let directory = tempdir().expect("candidate directory should be created");
        let root = Utf8PathBuf::from_path_buf(directory.path().to_path_buf())
            .expect("candidate path should be UTF-8");
        setup(&root);

        let error = CandidateWorkspace::from_directory(&registry_request(), directory)
            .expect_err("candidate file construction should fail");

        assert!(matches!(
            error,
            CliError::FileWrite { path, error }
                if path == root.join(expected_path) && error.kind() == expected_kind
        ));
        assert!(!root.exists());
    }
}
