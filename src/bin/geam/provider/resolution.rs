use super::manifest::ProviderSource;
use super::metadata::ProviderMetadata;
use crate::command::AddProvider;
use crate::error::CliError;
use crate::process::run_checked;
use camino::{Utf8Path, Utf8PathBuf};
use cargo_metadata::{Metadata, MetadataCommand, Package};
use semver::Version;
use std::fs;
use std::path::Path;
use std::process::Command;

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
) -> Result<ProviderMetadata, CliError> {
    resolve_with(
        project_root,
        selection_request(selection),
        &SystemCargoMetadata,
    )
    .map(|resolved| resolved.metadata)
}

fn selection_request(selection: &super::manifest::ProviderSelection) -> ProviderRequest {
    match selection.source() {
        ProviderSource::Registry { version } => ProviderRequest::Registry {
            crate_name: selection.crate_name().to_owned(),
            version: Some(version.clone()),
        },
        ProviderSource::Path { path } => ProviderRequest::Path {
            path: path.clone(),
            package: Some(selection.crate_name().to_owned()),
        },
        ProviderSource::Git { url, rev } => ProviderRequest::Git {
            url: url.clone(),
            rev: rev.clone(),
            package: Some(selection.crate_name().to_owned()),
        },
    }
}

fn resolve_with(
    project_root: &Utf8Path,
    request: ProviderRequest,
    loader: &dyn CargoMetadataLoader,
) -> Result<ResolvedProvider, CliError> {
    let request = complete_package_identity(project_root, request, loader)?;
    let candidate_manifest = write_candidate_manifest(project_root, &request)?;
    let metadata = loader.load(project_root, &candidate_manifest)?;
    let package = candidate_dependency(&metadata)?;
    let provider = ProviderMetadata::from_package(package)?;
    let source = request.provider_source(package);
    Ok(ResolvedProvider {
        metadata: provider,
        source,
    })
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
            let (package, _) = inspect_workspace(&inspection, None, loader)?;
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
    let metadata = loader.load(path, &manifest)?;
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

fn write_candidate_manifest(
    project_root: &Utf8Path,
    request: &ProviderRequest,
) -> Result<Utf8PathBuf, CliError> {
    let directory = project_root.join("build/geam/provider-candidate");
    let source_directory = directory.join("src");
    fs::create_dir_all(&source_directory).map_err(|error| CliError::FileWrite {
        path: source_directory.clone(),
        error,
    })?;
    let manifest = directory.join("Cargo.toml");
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
    Ok(manifest)
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

trait CargoMetadataLoader {
    fn load(&self, current_directory: &Utf8Path, manifest: &Utf8Path)
    -> Result<Metadata, CliError>;
}

struct SystemCargoMetadata;

impl CargoMetadataLoader for SystemCargoMetadata {
    fn load(
        &self,
        current_directory: &Utf8Path,
        manifest: &Utf8Path,
    ) -> Result<Metadata, CliError> {
        let output = run_checked(
            Command::new("cargo")
                .arg("metadata")
                .arg("--format-version")
                .arg("1")
                .arg("--manifest-path")
                .arg(manifest)
                .current_dir(current_directory),
        )?;
        parse_metadata_output(manifest, &output.stdout)
    }
}

fn parse_metadata_output(manifest: &Utf8Path, output: &[u8]) -> Result<Metadata, CliError> {
    MetadataCommand::parse(String::from_utf8_lossy(output)).map_err(|error| {
        CliError::InvalidCargoMetadata {
            manifest: manifest.to_path_buf(),
            reason: error.to_string(),
        }
    })
}

fn candidate_dependency(metadata: &Metadata) -> Result<&Package, CliError> {
    let resolve =
        metadata
            .resolve
            .as_ref()
            .ok_or_else(|| CliError::MissingCandidateDependency {
                alias: CANDIDATE_ALIAS.to_owned(),
            })?;
    let root = resolve
        .root
        .as_ref()
        .ok_or_else(|| CliError::MissingCandidateDependency {
            alias: CANDIDATE_ALIAS.to_owned(),
        })?;
    let root = resolve
        .nodes
        .iter()
        .find(|node| &node.id == root)
        .ok_or_else(|| CliError::MissingCandidateDependency {
            alias: CANDIDATE_ALIAS.to_owned(),
        })?;
    let dependency = root
        .deps
        .iter()
        .find(|dependency| dependency.name == CANDIDATE_ALIAS)
        .ok_or_else(|| CliError::MissingCandidateDependency {
            alias: CANDIDATE_ALIAS.to_owned(),
        })?;
    metadata
        .packages
        .iter()
        .find(|package| package.id == dependency.pkg)
        .ok_or_else(|| CliError::MissingCandidateDependency {
            alias: CANDIDATE_ALIAS.to_owned(),
        })
}

fn clone_git_for_inspection(
    project_root: &Utf8Path,
    url: &str,
    rev: Option<&str>,
) -> Result<Utf8PathBuf, CliError> {
    let parent = project_root.join("build/geam");
    let directory = parent.join("provider-git-inspection");
    if directory.exists() {
        fs::remove_dir_all(&directory).map_err(|error| CliError::FileWrite {
            path: directory.clone(),
            error,
        })?;
    }
    fs::create_dir_all(&parent).map_err(|error| CliError::FileWrite {
        path: parent,
        error,
    })?;
    run_checked(
        Command::new("git")
            .arg("clone")
            .arg("--quiet")
            .arg("--no-tags")
            .arg(url)
            .arg(&directory),
    )?;
    if let Some(rev) = rev {
        run_checked(
            Command::new("git")
                .arg("-C")
                .arg(&directory)
                .arg("checkout")
                .arg("--quiet")
                .arg(rev),
        )?;
    }
    Ok(directory)
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
        CargoMetadataLoader, ProviderRequest, SystemCargoMetadata, candidate_dependency,
        canonical_provider_path, canonical_provider_path_from, clone_git_for_inspection,
        complete_package_identity, inspect_workspace, parse_metadata_output,
        parse_registry_specification, resolve_with, selection_request, write_candidate_manifest,
    };
    use crate::command::AddProvider;
    use crate::error::CliError;
    use crate::provider::manifest::{ProviderSelection, ProviderSource};
    use camino::Utf8PathBuf;
    use cargo_metadata::Metadata;
    use std::fs;
    use std::path::Path;
    use std::process::Command;
    use tempfile::{TempDir, tempdir};

    struct FailingLoader;

    impl CargoMetadataLoader for FailingLoader {
        fn load(
            &self,
            _current_directory: &camino::Utf8Path,
            manifest: &camino::Utf8Path,
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
        ) -> Result<Metadata, CliError> {
            Ok(self.0.clone())
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
        assert_eq!(
            std::mem::discriminant(&error),
            std::mem::discriminant(&CliError::InvalidCrateSpecification {
                spec: String::new(),
                reason: String::new(),
            }),
        );
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
        assert_eq!(
            std::mem::discriminant(&error),
            std::mem::discriminant(&CliError::FileRead {
                path: Utf8PathBuf::new(),
                error: std::io::Error::new(std::io::ErrorKind::NotFound, ""),
            }),
        );
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
        assert_eq!(
            std::mem::discriminant(&error),
            std::mem::discriminant(&CliError::NonUtf8Path(std::path::PathBuf::new())),
        );

        let error =
            canonical_provider_path_from(Ok(std::path::PathBuf::from(OsString::from_vec(vec![
                0xfe,
            ]))))
            .expect_err("non-UTF-8 canonical path should be rejected");
        assert_eq!(
            std::mem::discriminant(&error),
            std::mem::discriminant(&CliError::NonUtf8Path(std::path::PathBuf::new())),
        );
    }

    #[test]
    fn parses_registry_crate_specifications() {
        let unversioned =
            parse_registry_specification("geam-images").expect("unversioned crate should parse");
        assert_eq!(unversioned, ("geam-images".to_owned(), None),);
        let versioned = parse_registry_specification("geam-images@1.2.3")
            .expect("versioned crate should parse");
        assert_eq!(
            versioned,
            (
                "geam-images".to_owned(),
                Some("1.2.3".parse().expect("version should parse")),
            ),
        );
        for specification in ["@1.0.0", "geam-images@", "geam-images@^1"] {
            let error = parse_registry_specification(specification)
                .expect_err("invalid registry specification should be rejected");
            assert_eq!(
                std::mem::discriminant(&error),
                std::mem::discriminant(&CliError::InvalidCrateSpecification {
                    spec: String::new(),
                    reason: String::new(),
                }),
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
    fn renders_all_candidate_dependency_sources_before_resolution() {
        let project = utf8_tempdir();
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
            let manifest = write_candidate_manifest(&project, &request)
                .expect("candidate manifest should be written");
            let source = fs::read_to_string(manifest).expect("manifest should be readable");
            assert!(source.contains(expected), "missing {expected}: {source}");
        }
    }

    #[test]
    fn reconstructs_exact_requests_from_every_recorded_provider_source() {
        let selections = [
            (
                ProviderSelection::new(
                    "images".to_owned(),
                    "geam-images".to_owned(),
                    ProviderSource::Registry {
                        version: "1.2.3".parse().expect("version should parse"),
                    },
                ),
                ProviderRequest::Registry {
                    crate_name: "geam-images".to_owned(),
                    version: Some("1.2.3".parse().expect("version should parse")),
                },
            ),
            (
                ProviderSelection::new(
                    "images".to_owned(),
                    "geam-images".to_owned(),
                    ProviderSource::Path {
                        path: "/providers/images".into(),
                    },
                ),
                ProviderRequest::Path {
                    path: "/providers/images".into(),
                    package: Some("geam-images".to_owned()),
                },
            ),
            (
                ProviderSelection::new(
                    "images".to_owned(),
                    "geam-images".to_owned(),
                    ProviderSource::Git {
                        url: "https://example.com/images.git".to_owned(),
                        rev: Some("abc123".to_owned()),
                    },
                ),
                ProviderRequest::Git {
                    url: "https://example.com/images.git".to_owned(),
                    rev: Some("abc123".to_owned()),
                    package: Some("geam-images".to_owned()),
                },
            ),
        ];

        for (selection, expected) in selections {
            assert_eq!(selection_request(&selection), expected);
        }
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
                path: provider_path,
            },
        );
    }

    #[test]
    fn derives_recorded_sources_from_completed_requests() {
        let provider = provider_package("geam-images", "images", "1.0.0");
        let path = utf8_path(&provider);
        let metadata = SystemCargoMetadata
            .load(&path, &path.join("Cargo.toml"))
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

        assert_eq!(
            complete_package_identity(
                &project,
                ProviderRequest::Path {
                    path: workspace_path.clone(),
                    package: None,
                },
                &loader,
            )
            .expect_err("ambiguous workspace should be rejected")
            .to_string(),
            "provider workspace has multiple metadata-bearing packages; use --package with one of: geam-one, geam-two",
        );
        assert_eq!(
            complete_package_identity(
                &project,
                ProviderRequest::Path {
                    path: workspace_path.clone(),
                    package: Some("missing".to_owned()),
                },
                &loader,
            )
            .expect_err("unknown explicit package should be rejected")
            .to_string(),
            "provider package missing was not found in Cargo metadata",
        );

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
        assert_eq!(
            std::mem::discriminant(&error),
            std::mem::discriminant(&CliError::MissingProviderPackage {
                package: String::new(),
            }),
        );
        let error = complete_package_identity(
            &project,
            ProviderRequest::Path {
                path: plain,
                package: Some("plain".to_owned()),
            },
            &loader,
        )
        .expect_err("explicit non-provider package should be rejected");
        assert_eq!(
            std::mem::discriminant(&error),
            std::mem::discriminant(&CliError::InvalidProviderMetadata {
                package: String::new(),
                reason: String::new(),
            }),
        );
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
        assert_eq!(
            std::mem::discriminant(&error),
            std::mem::discriminant(&CliError::MissingProviderManifest {
                path: Utf8PathBuf::new(),
            }),
        );

        let provider = provider_package("geam-images", "images", "1.0.0");
        let path = utf8_path(&provider);
        let error = inspect_workspace(&path, None, &FailingLoader)
            .expect_err("Cargo metadata failure should be preserved");
        assert_eq!(
            std::mem::discriminant(&error),
            std::mem::discriminant(&CliError::InvalidCargoMetadata {
                manifest: Utf8PathBuf::new(),
                reason: String::new(),
            }),
        );
        let mut metadata = SystemCargoMetadata
            .load(&path, &path.join("Cargo.toml"))
            .expect("provider metadata should load");
        metadata
            .packages
            .first_mut()
            .expect("provider package should be present")
            .manifest_path = Utf8PathBuf::new();
        assert_eq!(
            inspect_workspace(&path, None, &FixedLoader(metadata))
                .expect_err("missing package directory should be rejected")
                .to_string(),
            "crate geam-images has invalid Geam provider metadata: Cargo manifest has no package directory",
        );
    }

    #[test]
    fn preserves_candidate_manifest_filesystem_failures() {
        let request = ProviderRequest::Registry {
            crate_name: "geam-images".to_owned(),
            version: None,
        };

        let blocked_directory = utf8_tempdir();
        fs::write(blocked_directory.join("build"), "blocked")
            .expect("blocking file should be written");
        let error = write_candidate_manifest(&blocked_directory, &request)
            .expect_err("blocked candidate directory should fail");
        assert_eq!(
            std::mem::discriminant(&error),
            std::mem::discriminant(&CliError::FileWrite {
                path: Utf8PathBuf::new(),
                error: std::io::Error::other(""),
            }),
        );

        let blocked_manifest = utf8_tempdir();
        fs::create_dir_all(blocked_manifest.join("build/geam/provider-candidate/src"))
            .expect("candidate source should be created");
        fs::create_dir(blocked_manifest.join("build/geam/provider-candidate/Cargo.toml"))
            .expect("blocking manifest directory should be created");
        let error = write_candidate_manifest(&blocked_manifest, &request)
            .expect_err("blocked candidate manifest should fail");
        assert_eq!(
            std::mem::discriminant(&error),
            std::mem::discriminant(&CliError::FileWrite {
                path: Utf8PathBuf::new(),
                error: std::io::Error::other(""),
            }),
        );

        let blocked_source = utf8_tempdir();
        fs::create_dir_all(blocked_source.join("build/geam/provider-candidate/src/main.rs"))
            .expect("blocking source directory should be created");
        let error = write_candidate_manifest(&blocked_source, &request)
            .expect_err("blocked candidate source should fail");
        assert_eq!(
            std::mem::discriminant(&error),
            std::mem::discriminant(&CliError::FileWrite {
                path: Utf8PathBuf::new(),
                error: std::io::Error::other(""),
            }),
        );

        let unresolved = ProviderRequest::Path {
            path: "/provider".into(),
            package: None,
        };
        let error = write_candidate_manifest(&utf8_tempdir(), &unresolved)
            .expect_err("unresolved provider identity should be rejected");
        assert_eq!(
            std::mem::discriminant(&error),
            std::mem::discriminant(&CliError::InvalidCrateSpecification {
                spec: String::new(),
                reason: String::new(),
            }),
        );
    }

    #[test]
    fn inspects_local_git_repositories_when_package_is_omitted() {
        let project = utf8_tempdir();
        let repository = provider_package("geam-images", "images", "1.0.0");
        initialize_git_repository(&repository);
        let repository_path = utf8_path(&repository);
        let inspected = clone_git_for_inspection(&project, repository_path.as_str(), Some("HEAD"))
            .expect("local repository should clone");
        assert!(inspected.join("Cargo.toml").is_file());

        let completed = complete_package_identity(
            &project,
            ProviderRequest::Git {
                url: repository_path.to_string(),
                rev: Some("HEAD".to_owned()),
                package: None,
            },
            &SystemCargoMetadata,
        )
        .expect("Git package should be discovered");
        assert_eq!(completed.crate_name(), Some("geam-images"));
    }

    #[test]
    fn preserves_git_inspection_filesystem_and_process_failures() {
        let blocked_removal = utf8_tempdir();
        fs::create_dir_all(blocked_removal.join("build/geam"))
            .expect("inspection parent should be created");
        fs::write(
            blocked_removal.join("build/geam/provider-git-inspection"),
            "not a directory",
        )
        .expect("blocking file should be written");
        let error = clone_git_for_inspection(&blocked_removal, "missing", None)
            .expect_err("non-directory inspection path should fail");
        assert_eq!(
            std::mem::discriminant(&error),
            std::mem::discriminant(&CliError::FileWrite {
                path: Utf8PathBuf::new(),
                error: std::io::Error::other(""),
            }),
        );

        let blocked_parent = utf8_tempdir();
        fs::write(blocked_parent.join("build"), "not a directory")
            .expect("blocking file should be written");
        let error = clone_git_for_inspection(&blocked_parent, "missing", None)
            .expect_err("blocked inspection parent should fail");
        assert_eq!(
            std::mem::discriminant(&error),
            std::mem::discriminant(&CliError::FileWrite {
                path: Utf8PathBuf::new(),
                error: std::io::Error::other(""),
            }),
        );

        let missing_repository = utf8_tempdir();
        let error =
            clone_git_for_inspection(&missing_repository, "/repository/that/does/not/exist", None)
                .expect_err("missing repository should fail to clone");
        assert_eq!(
            std::mem::discriminant(&error),
            std::mem::discriminant(&CliError::ProcessFailure {
                command: String::new(),
                status: None,
                stderr: String::new(),
            }),
        );
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
        assert_eq!(
            std::mem::discriminant(&error),
            std::mem::discriminant(&CliError::ProcessFailure {
                command: String::new(),
                status: None,
                stderr: String::new(),
            }),
        );

        let plain = tempdir().expect("plain package should be created");
        fs::create_dir(plain.path().join("src")).expect("source should be created");
        fs::write(
            plain.path().join("Cargo.toml"),
            "[package]\nname = \"plain\"\nversion = \"1.0.0\"\nedition = \"2024\"\n",
        )
        .expect("plain manifest should be written");
        fs::write(plain.path().join("src/lib.rs"), "").expect("source should be written");
        initialize_git_repository(&plain);
        let error = complete_package_identity(
            &utf8_tempdir(),
            ProviderRequest::Git {
                url: utf8_path(&plain).to_string(),
                rev: None,
                package: None,
            },
            &SystemCargoMetadata,
        )
        .expect_err("metadata-free Git package should be rejected after inspection");
        assert_eq!(
            std::mem::discriminant(&error),
            std::mem::discriminant(&CliError::MissingProviderPackage {
                package: String::new(),
            }),
        );

        let repository = provider_package("geam-images", "images", "1.0.0");
        initialize_git_repository(&repository);
        let repository_path = utf8_path(&repository);
        let project = utf8_tempdir();
        clone_git_for_inspection(&project, repository_path.as_str(), None)
            .expect("repository should clone without an explicit revision");
        let error =
            clone_git_for_inspection(&project, repository_path.as_str(), Some("missing-revision"))
                .expect_err("missing revision should fail checkout");
        assert_eq!(
            std::mem::discriminant(&error),
            std::mem::discriminant(&CliError::ProcessFailure {
                command: String::new(),
                status: None,
                stderr: String::new(),
            }),
        );
    }

    #[test]
    fn distinguishes_missing_provider_paths_from_missing_manifests() {
        let project = utf8_tempdir();
        let error = canonical_provider_path(&project.join("missing"))
            .expect_err("missing provider path should fail");
        assert_eq!(
            std::mem::discriminant(&error),
            std::mem::discriminant(&CliError::FileRead {
                path: Utf8PathBuf::new(),
                error: std::io::Error::new(std::io::ErrorKind::NotFound, ""),
            }),
        );

        let empty = project.join("empty");
        fs::create_dir(&empty).expect("empty provider directory should be created");
        let error = canonical_provider_path(&empty)
            .expect_err("provider directory without Cargo manifest should fail");
        assert_eq!(
            std::mem::discriminant(&error),
            std::mem::discriminant(&CliError::MissingProviderManifest {
                path: Utf8PathBuf::new(),
            }),
        );
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

        assert!(
            matches!(error, CliError::InvalidCargoMetadata { reason, .. } if reason == "fixture stop")
        );
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
        assert_eq!(
            std::mem::discriminant(&error),
            std::mem::discriminant(&CliError::MissingProviderManifest {
                path: Utf8PathBuf::new(),
            }),
        );

        let blocked = utf8_tempdir();
        fs::write(blocked.join("build"), "blocked").expect("blocking file should be written");
        let registry = ProviderRequest::Registry {
            crate_name: "geam-images".to_owned(),
            version: None,
        };
        let error = resolve_with(&blocked, registry, &FailingLoader)
            .err()
            .expect("blocked candidate manifest should fail");
        assert_eq!(
            std::mem::discriminant(&error),
            std::mem::discriminant(&CliError::FileWrite {
                path: Utf8PathBuf::new(),
                error: std::io::Error::other(""),
            }),
        );

        let provider = provider_package("geam-images", "images", "1.0.0");
        let path_request = ProviderRequest::Path {
            path: utf8_path(&provider),
            package: Some("geam-images".to_owned()),
        };
        let manifest = write_candidate_manifest(&project, &path_request)
            .expect("candidate manifest should be written");
        let metadata = SystemCargoMetadata
            .load(&project, &manifest)
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
        assert_eq!(
            std::mem::discriminant(&error),
            std::mem::discriminant(&CliError::MissingCandidateDependency {
                alias: String::new(),
            }),
        );

        let mut invalid_provider = metadata;
        let candidate = candidate_dependency(&invalid_provider)
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
        assert_eq!(
            std::mem::discriminant(&error),
            std::mem::discriminant(&CliError::InvalidProviderMetadata {
                package: String::new(),
                reason: String::new(),
            }),
        );
    }

    #[test]
    fn preserves_cargo_process_and_metadata_parse_failures() {
        let project = utf8_tempdir();
        let missing_manifest = project.join("missing.toml");
        let error = SystemCargoMetadata
            .load(&project, &missing_manifest)
            .expect_err("missing Cargo manifest should fail metadata resolution");
        assert_eq!(
            std::mem::discriminant(&error),
            std::mem::discriminant(&CliError::ProcessFailure {
                command: String::new(),
                status: None,
                stderr: String::new(),
            }),
        );
        let parse_reason = cargo_metadata::MetadataCommand::parse("not JSON")
            .expect_err("invalid Cargo output fixture should be rejected");
        assert_eq!(
            parse_metadata_output(&missing_manifest, b"not JSON")
                .expect_err("invalid Cargo output should be rejected")
                .to_string(),
            format!("Cargo returned invalid metadata for {missing_manifest}: {parse_reason}"),
        );
    }

    #[test]
    fn reports_missing_candidate_edges() {
        let project = utf8_tempdir();
        let provider = provider_package("geam-images", "images", "1.0.0");
        let request = ProviderRequest::Path {
            path: utf8_path(&provider),
            package: Some("geam-images".to_owned()),
        };
        let manifest = write_candidate_manifest(&project, &request)
            .expect("candidate manifest should be written");
        let metadata = SystemCargoMetadata
            .load(&project, &manifest)
            .expect("candidate metadata should load");
        assert_eq!(
            candidate_dependency(&metadata)
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
        assert_eq!(
            candidate_dependency(metadata)
                .expect_err("candidate dependency should be absent")
                .to_string(),
            "Cargo metadata did not contain the candidate dependency provider_candidate",
        );
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
}
