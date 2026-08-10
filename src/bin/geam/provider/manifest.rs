use crate::error::CliError;
use camino::{Utf8Path, Utf8PathBuf};
use semver::Version;
use std::collections::{BTreeMap, BTreeSet};
use std::fs;

pub(super) const MANAGED_HEADER: &str =
    "# Managed by Geam. Use `geam provider` commands to change providers.\n";
const PROVIDER_ALIAS_PREFIX: &str = "geam_provider_";
const RUNNER_SCHEMA: i64 = 1;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct ProviderSelection {
    gleam_package: String,
    crate_name: String,
    source: ProviderSource,
}

impl ProviderSelection {
    pub(super) fn new(gleam_package: String, crate_name: String, source: ProviderSource) -> Self {
        Self {
            gleam_package,
            crate_name,
            source,
        }
    }

    pub(super) fn crate_name(&self) -> &str {
        &self.crate_name
    }

    pub(super) fn gleam_package(&self) -> &str {
        &self.gleam_package
    }

    pub(super) fn source(&self) -> &ProviderSource {
        &self.source
    }

    pub(super) fn alias(&self) -> String {
        format!("{PROVIDER_ALIAS_PREFIX}{}", self.gleam_package)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum ProviderSource {
    Registry { version: Version },
    Path { path: Utf8PathBuf },
    Git { url: String, rev: Option<String> },
}

pub(crate) struct ManagedProject {
    root: Utf8PathBuf,
    root_package: String,
    providers: BTreeMap<String, ProviderSelection>,
}

impl ManagedProject {
    pub(crate) fn load(root: &Utf8Path, root_package: impl Into<String>) -> Result<Self, CliError> {
        let path = root.join("Cargo.toml");
        let root_package = root_package.into();
        if !path.exists() {
            return Ok(Self {
                root: root.to_path_buf(),
                root_package,
                providers: BTreeMap::new(),
            });
        }
        let source = fs::read_to_string(&path).map_err(|error| CliError::FileRead {
            path: path.clone(),
            error,
        })?;
        if !source.starts_with(MANAGED_HEADER) {
            return Err(CliError::UserOwnedCargoManifest { path });
        }
        let document = source
            .parse::<toml::Table>()
            .map_err(|error| CliError::InvalidToml {
                kind: "managed Cargo manifest",
                path: path.clone(),
                reason: error.to_string(),
            })?;
        let schema = document
            .get("package")
            .and_then(toml::Value::as_table)
            .and_then(|package| package.get("metadata"))
            .and_then(toml::Value::as_table)
            .and_then(|metadata| metadata.get("geam"))
            .and_then(toml::Value::as_table)
            .and_then(|geam| geam.get("runner"))
            .and_then(toml::Value::as_table)
            .and_then(|runner| runner.get("schema"))
            .and_then(toml::Value::as_integer)
            .ok_or_else(|| CliError::InvalidToml {
                kind: "managed Cargo manifest",
                path: path.clone(),
                reason: "missing package.metadata.geam.runner.schema".to_owned(),
            })?;
        if schema != RUNNER_SCHEMA {
            return Err(CliError::UnsupportedRunnerSchema { path, schema });
        }
        let dependencies = document
            .get("dependencies")
            .and_then(toml::Value::as_table)
            .ok_or_else(|| CliError::InvalidToml {
                kind: "managed Cargo manifest",
                path: path.clone(),
                reason: "missing dependencies table".to_owned(),
            })?;
        let providers = dependencies
            .iter()
            .filter(|(alias, _)| alias.starts_with(PROVIDER_ALIAS_PREFIX))
            .map(|(alias, value)| parse_provider_dependency(alias, value))
            .collect::<Result<BTreeMap<_, _>, _>>()?;
        Ok(Self {
            root: root.to_path_buf(),
            root_package,
            providers,
        })
    }

    pub(super) fn insert(&mut self, provider: ProviderSelection) -> Result<(), CliError> {
        let package = provider.gleam_package.clone();
        if self.providers.contains_key(&package) {
            return Err(CliError::ProviderAlreadySelected { package });
        }
        self.providers.insert(package, provider);
        Ok(())
    }

    pub(super) fn replace(&mut self, provider: ProviderSelection) {
        self.providers
            .insert(provider.gleam_package.clone(), provider);
    }

    pub(super) fn remove(&mut self, gleam_package: &str) -> Result<(), CliError> {
        self.providers
            .remove(gleam_package)
            .map(|_| ())
            .ok_or_else(|| CliError::ProviderNotSelected {
                package: gleam_package.to_owned(),
            })
    }

    pub(crate) fn retain_packages(&mut self, packages: &BTreeSet<String>) {
        self.providers
            .retain(|gleam_package, _| packages.contains(gleam_package));
    }

    pub(crate) fn has_provider(&self, gleam_package: &str) -> bool {
        self.providers.contains_key(gleam_package)
    }

    pub(super) fn provider(&self, gleam_package: &str) -> Option<&ProviderSelection> {
        self.providers.get(gleam_package)
    }

    pub(crate) fn provider_aliases(&self) -> Vec<String> {
        self.providers
            .values()
            .map(ProviderSelection::alias)
            .collect()
    }

    pub(crate) fn write(&self) -> Result<bool, CliError> {
        let path = self.root.join("Cargo.toml");
        let source = self.render();
        match fs::read_to_string(&path) {
            Ok(current) if current == source => return Ok(false),
            Ok(_) => {}
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
            Err(error) => {
                return Err(CliError::FileRead {
                    path: path.clone(),
                    error,
                });
            }
        }
        let temporary = self.root.join("Cargo.toml.geam.tmp");
        fs::write(&temporary, source).map_err(|error| CliError::FileWrite {
            path: temporary.clone(),
            error,
        })?;
        replace_file(&temporary, &path)?;
        Ok(true)
    }

    fn render(&self) -> String {
        let mut source = String::from(MANAGED_HEADER);
        source.push_str("\n[package]\nname = ");
        source.push_str(&quoted(&format!("{}-geam-runner", self.root_package)));
        source.push_str(
            "\nversion = \"0.0.0\"\nedition = \"2024\"\npublish = false\n\n[package.metadata.geam.runner]\nschema = 1\n\n[[bin]]\nname = \"geam-runner\"\npath = \"build/geam/runner.rs\"\n\n[dependencies]\n",
        );
        source.push_str("geam = ");
        source.push_str(&quoted(&format!("={}", env!("CARGO_PKG_VERSION"))));
        source.push_str("\ntoml = \"0.9\"\n");
        for provider in self.providers.values() {
            source.push_str(&provider.alias());
            source.push_str(" = { package = ");
            source.push_str(&quoted(provider.crate_name()));
            match provider.source() {
                ProviderSource::Registry { version } => {
                    source.push_str(", version = ");
                    source.push_str(&quoted(&format!("={version}")));
                }
                ProviderSource::Path { path } => {
                    source.push_str(", path = ");
                    source.push_str(&quoted(path.as_str()));
                }
                ProviderSource::Git { url, rev } => {
                    source.push_str(", git = ");
                    source.push_str(&quoted(url));
                    if let Some(rev) = rev {
                        source.push_str(", rev = ");
                        source.push_str(&quoted(rev));
                    }
                }
            }
            source.push_str(" }\n");
        }
        source.push_str("\n[workspace]\nresolver = \"3\"\n");
        source
    }
}

fn replace_file(temporary: &Utf8Path, destination: &Utf8Path) -> Result<(), CliError> {
    fs::rename(temporary, destination).map_err(|error| CliError::FileWrite {
        path: destination.to_path_buf(),
        error,
    })
}

fn parse_provider_dependency(
    alias: &str,
    value: &toml::Value,
) -> Result<(String, ProviderSelection), CliError> {
    let gleam_package = alias
        .strip_prefix(PROVIDER_ALIAS_PREFIX)
        .filter(|package| !package.is_empty())
        .ok_or_else(|| CliError::InvalidManagedDependency {
            alias: alias.to_owned(),
            reason: "provider alias has no Gleam package suffix".to_owned(),
        })?
        .to_owned();
    let dependency = value
        .as_table()
        .ok_or_else(|| CliError::InvalidManagedDependency {
            alias: alias.to_owned(),
            reason: "dependency must be a table".to_owned(),
        })?;
    let crate_name = dependency
        .get("package")
        .and_then(toml::Value::as_str)
        .ok_or_else(|| CliError::InvalidManagedDependency {
            alias: alias.to_owned(),
            reason: "dependency must declare package".to_owned(),
        })?
        .to_owned();
    let version = dependency.get("version").and_then(toml::Value::as_str);
    let path = dependency.get("path").and_then(toml::Value::as_str);
    let git = dependency.get("git").and_then(toml::Value::as_str);
    let source = match (version, path, git) {
        (Some(version), None, None) => {
            let version = version
                .strip_prefix('=')
                .ok_or_else(|| CliError::InvalidManagedDependency {
                    alias: alias.to_owned(),
                    reason: "registry version must be exact".to_owned(),
                })?
                .parse()
                .map_err(|error| CliError::InvalidManagedDependency {
                    alias: alias.to_owned(),
                    reason: format!("invalid registry version: {error}"),
                })?;
            ProviderSource::Registry { version }
        }
        (None, Some(path), None) => ProviderSource::Path { path: path.into() },
        (None, None, Some(git)) => ProviderSource::Git {
            url: git.to_owned(),
            rev: dependency
                .get("rev")
                .and_then(toml::Value::as_str)
                .map(str::to_owned),
        },
        _ => {
            return Err(CliError::InvalidManagedDependency {
                alias: alias.to_owned(),
                reason: "dependency must declare exactly one of version, path, or git".to_owned(),
            });
        }
    };
    Ok((
        gleam_package.clone(),
        ProviderSelection::new(gleam_package, crate_name, source),
    ))
}

fn quoted(value: &str) -> String {
    toml::Value::String(value.to_owned()).to_string()
}

#[cfg(test)]
mod tests {
    use super::{ManagedProject, ProviderSelection, ProviderSource, replace_file};
    use crate::error::CliError;
    use camino::Utf8PathBuf;
    use std::collections::BTreeSet;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn writes_and_reloads_canonical_provider_sources() {
        let project = tempdir().expect("temporary project should be created");
        let root = Utf8PathBuf::from_path_buf(project.path().to_path_buf())
            .expect("temporary path should be valid UTF-8");
        let mut managed = ManagedProject::load(&root, "application")
            .expect("missing manifest should initialize managed project");
        managed
            .insert(ProviderSelection::new(
                "images".to_owned(),
                "geam-images".to_owned(),
                ProviderSource::Registry {
                    version: "1.2.3".parse().expect("version should parse"),
                },
            ))
            .expect("registry provider should be inserted");
        managed
            .insert(ProviderSelection::new(
                "search".to_owned(),
                "geam-search".to_owned(),
                ProviderSource::Path {
                    path: "/providers/search".into(),
                },
            ))
            .expect("path provider should be inserted");
        managed
            .insert(ProviderSelection::new(
                "video".to_owned(),
                "geam-video".to_owned(),
                ProviderSource::Git {
                    url: "https://example.com/video.git".to_owned(),
                    rev: Some("abc123".to_owned()),
                },
            ))
            .expect("Git provider should be inserted");
        managed
            .insert(ProviderSelection::new(
                "websocket".to_owned(),
                "geam-websocket".to_owned(),
                ProviderSource::Git {
                    url: "https://example.com/websocket.git".to_owned(),
                    rev: None,
                },
            ))
            .expect("unpinned Git provider should be inserted");

        assert!(managed.write().expect("manifest should be written"));
        assert!(
            !managed
                .write()
                .expect("unchanged manifest should not be written")
        );
        let source = fs::read_to_string(root.join("Cargo.toml"))
            .expect("managed manifest should be readable");
        assert!(source.starts_with("# Managed by Geam."));
        assert!(source.contains(
            "geam_provider_images = { package = \"geam-images\", version = \"=1.2.3\" }"
        ));
        assert!(source.contains(
            "geam_provider_search = { package = \"geam-search\", path = \"/providers/search\" }"
        ));
        assert!(source.contains("geam_provider_video = { package = \"geam-video\", git = \"https://example.com/video.git\", rev = \"abc123\" }"));
        assert!(source.contains("geam_provider_websocket = { package = \"geam-websocket\", git = \"https://example.com/websocket.git\" }"));

        let reloaded =
            ManagedProject::load(&root, "application").expect("managed manifest should reload");
        assert_eq!(
            reloaded
                .providers
                .values()
                .map(|provider| provider.gleam_package.as_str())
                .collect::<Vec<_>>(),
            ["images", "search", "video", "websocket"],
        );
        assert_eq!(
            reloaded
                .providers
                .values()
                .next()
                .map(ProviderSelection::alias),
            Some("geam_provider_images".to_owned()),
        );
    }

    #[test]
    fn rejects_duplicate_insert_and_missing_remove() {
        let project = tempdir().expect("temporary project should be created");
        let root = Utf8PathBuf::from_path_buf(project.path().to_path_buf())
            .expect("temporary path should be valid UTF-8");
        let mut managed =
            ManagedProject::load(&root, "application").expect("managed project should initialize");
        let provider = ProviderSelection::new(
            "images".to_owned(),
            "geam-images".to_owned(),
            ProviderSource::Path {
                path: "/provider".into(),
            },
        );

        managed
            .insert(provider.clone())
            .expect("first insert should work");
        assert_eq!(
            managed
                .insert(provider)
                .expect_err("duplicate provider should be rejected")
                .to_string(),
            "provider for Gleam package images is already selected",
        );
        managed
            .remove("images")
            .expect("selected provider should be removed");
        assert_eq!(
            managed
                .remove("images")
                .expect_err("missing provider should be rejected")
                .to_string(),
            "no provider is selected for Gleam package images",
        );
    }

    #[test]
    fn prunes_only_absent_resolved_packages() {
        let project = tempdir().expect("temporary project should be created");
        let root = Utf8PathBuf::from_path_buf(project.path().to_path_buf())
            .expect("temporary path should be valid UTF-8");
        let mut managed =
            ManagedProject::load(&root, "application").expect("managed project should initialize");
        for package in ["images", "search"] {
            managed
                .insert(ProviderSelection::new(
                    package.to_owned(),
                    format!("geam-{package}"),
                    ProviderSource::Path {
                        path: format!("/{package}").into(),
                    },
                ))
                .expect("provider should be inserted");
        }

        managed.retain_packages(&BTreeSet::from(["images".to_owned()]));

        assert_eq!(
            managed
                .providers
                .values()
                .map(|provider| provider.gleam_package.as_str())
                .collect::<Vec<_>>(),
            ["images"],
        );
    }

    #[test]
    fn refuses_user_owned_and_unsupported_manifests() {
        let project = tempdir().expect("temporary project should be created");
        let root = Utf8PathBuf::from_path_buf(project.path().to_path_buf())
            .expect("temporary path should be valid UTF-8");
        fs::write(root.join("Cargo.toml"), "[package]\nname = \"user\"\n")
            .expect("user manifest should be written");
        let error = ManagedProject::load(&root, "application")
            .err()
            .expect("user manifest should be refused");
        assert_eq!(
            std::mem::discriminant(&error),
            std::mem::discriminant(&CliError::UserOwnedCargoManifest {
                path: Utf8PathBuf::new(),
            }),
        );

        fs::write(
            root.join("Cargo.toml"),
            format!(
                "{}\n[package.metadata.geam.runner]\nschema = 2\n\n[dependencies]\n",
                super::MANAGED_HEADER,
            ),
        )
        .expect("unsupported manifest should be written");
        assert_eq!(
            ManagedProject::load(&root, "application")
                .err()
                .expect("unsupported schema should be rejected")
                .to_string(),
            format!(
                "managed Cargo manifest {} uses unsupported runner schema 2",
                root.join("Cargo.toml"),
            ),
        );
    }

    #[test]
    fn rejects_malformed_managed_dependencies() {
        let invalid_version = "bad"
            .parse::<semver::Version>()
            .expect_err("invalid version fixture should be rejected");
        let cases = [
            (
                "geam_provider_ = \"1\"",
                "geam_provider_",
                "provider alias has no Gleam package suffix".to_owned(),
            ),
            (
                "geam_provider_images = \"1\"",
                "geam_provider_images",
                "dependency must be a table".to_owned(),
            ),
            (
                "geam_provider_images = { version = \"=1.0.0\" }",
                "geam_provider_images",
                "dependency must declare package".to_owned(),
            ),
            (
                "geam_provider_images = { package = \"provider\" }",
                "geam_provider_images",
                "dependency must declare exactly one of version, path, or git".to_owned(),
            ),
            (
                "geam_provider_images = { package = \"provider\", version = \"1.0.0\" }",
                "geam_provider_images",
                "registry version must be exact".to_owned(),
            ),
            (
                "geam_provider_images = { package = \"provider\", version = \"=bad\" }",
                "geam_provider_images",
                format!("invalid registry version: {invalid_version}"),
            ),
            (
                "geam_provider_images = { package = \"provider\", version = \"=1.0.0\", path = \"x\" }",
                "geam_provider_images",
                "dependency must declare exactly one of version, path, or git".to_owned(),
            ),
        ];
        for (dependency, alias, reason) in cases {
            let project = tempdir().expect("temporary project should be created");
            let root = Utf8PathBuf::from_path_buf(project.path().to_path_buf())
                .expect("temporary path should be valid UTF-8");
            fs::write(
                root.join("Cargo.toml"),
                format!(
                    "{}\n[package.metadata.geam.runner]\nschema = 1\n\n[dependencies]\n{dependency}\n",
                    super::MANAGED_HEADER,
                ),
            )
            .expect("malformed manifest should be written");

            let error = ManagedProject::load(&root, "application")
                .err()
                .expect("managed dependency should be rejected");
            assert_eq!(
                error.to_string(),
                format!("managed Cargo dependency {alias} is malformed: {reason}"),
            );
        }
    }

    #[test]
    fn rejects_invalid_managed_toml_and_missing_schema_or_dependencies() {
        for source in [
            format!("{}invalid", super::MANAGED_HEADER),
            format!("{}\n[dependencies]\n", super::MANAGED_HEADER),
            format!(
                "{}\n[package.metadata.geam.runner]\nschema = 1\n",
                super::MANAGED_HEADER,
            ),
        ] {
            let project = tempdir().expect("temporary project should be created");
            let root = Utf8PathBuf::from_path_buf(project.path().to_path_buf())
                .expect("temporary path should be valid UTF-8");
            fs::write(root.join("Cargo.toml"), source).expect("manifest should be written");

            let error = ManagedProject::load(&root, "application")
                .err()
                .expect("invalid managed manifest should be rejected");
            assert_eq!(
                std::mem::discriminant(&error),
                std::mem::discriminant(&CliError::InvalidToml {
                    kind: "managed Cargo manifest",
                    path: Utf8PathBuf::new(),
                    reason: String::new(),
                }),
            );
        }
    }

    #[test]
    fn preserves_managed_manifest_filesystem_failures() {
        let unreadable = tempdir().expect("temporary project should be created");
        let unreadable_root = Utf8PathBuf::from_path_buf(unreadable.path().to_path_buf())
            .expect("temporary path should be valid UTF-8");
        fs::create_dir(unreadable_root.join("Cargo.toml"))
            .expect("manifest directory should be created");
        let error = ManagedProject::load(&unreadable_root, "application")
            .err()
            .expect("unreadable manifest should fail");
        assert_eq!(
            std::mem::discriminant(&error),
            std::mem::discriminant(&CliError::FileRead {
                path: Utf8PathBuf::new(),
                error: std::io::Error::other(""),
            }),
        );

        let blocked_read = tempdir().expect("temporary project should be created");
        let blocked_read_root = Utf8PathBuf::from_path_buf(blocked_read.path().to_path_buf())
            .expect("temporary path should be valid UTF-8");
        let blocked_read_managed = ManagedProject::load(&blocked_read_root, "application")
            .expect("managed project should initialize");
        fs::create_dir(blocked_read_root.join("Cargo.toml"))
            .expect("manifest directory should be created");
        let error = blocked_read_managed
            .write()
            .expect_err("unreadable destination should fail");
        assert_eq!(
            std::mem::discriminant(&error),
            std::mem::discriminant(&CliError::FileRead {
                path: Utf8PathBuf::new(),
                error: std::io::Error::other(""),
            }),
        );

        let blocked_write = tempdir().expect("temporary project should be created");
        let blocked_write_root = Utf8PathBuf::from_path_buf(blocked_write.path().to_path_buf())
            .expect("temporary path should be valid UTF-8");
        let blocked_write_managed = ManagedProject::load(&blocked_write_root, "application")
            .expect("managed project should initialize");
        fs::create_dir(blocked_write_root.join("Cargo.toml.geam.tmp"))
            .expect("temporary manifest directory should be created");
        let error = blocked_write_managed
            .write()
            .expect_err("blocked temporary file should fail");
        assert_eq!(
            std::mem::discriminant(&error),
            std::mem::discriminant(&CliError::FileWrite {
                path: Utf8PathBuf::new(),
                error: std::io::Error::other(""),
            }),
        );

        let rename = tempdir().expect("temporary project should be created");
        let rename_root = Utf8PathBuf::from_path_buf(rename.path().to_path_buf())
            .expect("temporary path should be valid UTF-8");
        let temporary = rename_root.join("temporary");
        let destination = rename_root.join("destination");
        fs::write(&temporary, "source").expect("temporary file should be written");
        fs::create_dir(&destination).expect("blocking destination should be created");
        let error = replace_file(&temporary, &destination)
            .expect_err("directory destination should reject replacement");
        assert_eq!(
            std::mem::discriminant(&error),
            std::mem::discriminant(&CliError::FileWrite {
                path: Utf8PathBuf::new(),
                error: std::io::Error::other(""),
            }),
        );

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;

            let atomic_write = tempdir().expect("temporary project should be created");
            let root = Utf8PathBuf::from_path_buf(atomic_write.path().to_path_buf())
                .expect("temporary path should be valid UTF-8");
            let managed = ManagedProject::load(&root, "application")
                .expect("managed project should initialize");
            fs::write(root.join("Cargo.toml.geam.tmp"), "stale")
                .expect("existing temporary file should be written");
            let original = fs::metadata(&root)
                .expect("project metadata should be readable")
                .permissions();
            let mut restricted = original.clone();
            restricted.set_mode(0o500);
            fs::set_permissions(&root, restricted)
                .expect("project directory should become read-only");
            let result = managed.write();
            fs::set_permissions(&root, original)
                .expect("project directory permissions should be restored");
            let error = result.expect_err("atomic replacement should preserve rename failures");
            assert_eq!(
                std::mem::discriminant(&error),
                std::mem::discriminant(&CliError::FileWrite {
                    path: Utf8PathBuf::new(),
                    error: std::io::Error::other(""),
                }),
            );
        }
    }
}
