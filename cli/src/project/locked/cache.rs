use crate::error::CliError;
use crate::project::read_toml;
use camino::{Utf8Path, Utf8PathBuf};
use gleam_core::manifest::{ManifestPackage, ManifestPackageSource};
use hexpm::version::Version;
use serde::Deserialize;
use std::collections::BTreeMap;
use std::fs;
use std::io::Write;
use std::path::Path;
use toml_edit::{DocumentMut, Table, value};

pub(super) struct PackageCache {
    directory: Utf8PathBuf,
    state: CacheState,
}

// This is Gleam's disposable build/packages/packages.toml, not a Geam lockfile.
#[derive(Default, Deserialize)]
struct CacheState {
    #[serde(default)]
    packages: BTreeMap<String, Version>,
    #[serde(default)]
    git: BTreeMap<String, GitState>,
}

#[derive(Deserialize)]
struct GitState {
    commit: String,
    #[serde(default)]
    path: Option<Utf8PathBuf>,
}

impl PackageCache {
    pub(super) fn read(project_root: &Utf8Path) -> Result<Self, CliError> {
        let directory = project_root.join("build/packages");
        let path = directory.join("packages.toml");
        let state = match read_toml("Gleam package cache", &path) {
            Err(CliError::FileRead { error, .. })
                if error.kind() == std::io::ErrorKind::NotFound =>
            {
                CacheState::default()
            }
            result => result?,
        };
        Ok(Self { directory, state })
    }

    pub(super) fn contains(&self, package: &ManifestPackage) -> bool {
        if !self.directory.join(package.name.as_str()).is_dir() {
            return false;
        }
        match &package.source {
            ManifestPackageSource::Git { commit, path, .. } => self
                .state
                .git
                .get(package.name.as_str())
                .is_some_and(|state| state.commit == commit.as_str() && state.path == *path),
            _ => self.state.packages.get(package.name.as_str()) == Some(&package.version),
        }
    }

    pub(super) fn publish(
        &mut self,
        workspace: &Path,
        packages: &[ManifestPackage],
    ) -> Result<(), CliError> {
        fs::create_dir_all(&self.directory).map_err(|error| CliError::FileWrite {
            path: self.directory.clone(),
            error,
        })?;
        for package in packages {
            let target = self.directory.join(package.name.as_str());
            match fs::remove_dir_all(&target) {
                Err(error) if error.kind() != std::io::ErrorKind::NotFound => {
                    return Err(CliError::FileWrite {
                        path: target,
                        error,
                    });
                }
                _ => (),
            }
            fs::rename(
                workspace.join("build/packages").join(package.name.as_str()),
                &target,
            )
            .map_err(|error| CliError::FileWrite {
                path: target,
                error,
            })?;
            self.state
                .packages
                .insert(package.name.to_string(), package.version.clone());
            match &package.source {
                ManifestPackageSource::Git { commit, path, .. } => {
                    self.state.git.insert(
                        package.name.to_string(),
                        GitState {
                            commit: commit.to_string(),
                            path: path.clone(),
                        },
                    );
                }
                _ => {
                    self.state.git.remove(package.name.as_str());
                }
            }
        }
        self.save()
    }

    fn save(&self) -> Result<(), CliError> {
        let path = self.directory.join("packages.toml");
        let mut temporary = tempfile::NamedTempFile::new_in(&self.directory).map_err(|error| {
            CliError::FileWrite {
                path: path.clone(),
                error,
            }
        })?;
        write_state(
            &mut temporary,
            &path,
            self.document().to_string().as_bytes(),
        )
        .and_then(|()| {
            temporary
                .persist(&path)
                .map(|_| ())
                .map_err(|error| CliError::FileWrite {
                    path,
                    error: error.error,
                })
        })
    }

    fn document(&self) -> DocumentMut {
        let mut document = DocumentMut::new();
        document["packages"] = Table::new().into();
        for (name, version) in &self.state.packages {
            document["packages"][name] = value(version.to_string());
        }
        let mut git = Table::new();
        git.set_implicit(true);
        for (name, state) in &self.state.git {
            let mut entry = Table::new();
            entry["commit"] = value(&state.commit);
            if let Some(path) = &state.path {
                entry["path"] = value(path.as_str());
            }
            git[name] = entry.into();
        }
        document["git"] = git.into();
        document
    }
}

fn write_state(writer: &mut dyn Write, path: &Utf8Path, contents: &[u8]) -> Result<(), CliError> {
    writer
        .write_all(contents)
        .and_then(|()| writer.flush())
        .map_err(|error| CliError::FileWrite {
            path: path.to_path_buf(),
            error,
        })
}

#[cfg(test)]
mod tests {
    use super::{PackageCache, write_state};
    use crate::error::CliError;
    use camino::{Utf8Path, Utf8PathBuf};
    use gleam_core::manifest::{Manifest, ManifestPackage, ManifestPackageSource};
    use std::fs;
    use std::io::{self, Write};
    use tempfile::tempdir;

    #[test]
    fn matches_gleam_cache_versions_git_commits_subdirectories_and_present_sources() {
        let directory = tempdir().expect("cache fixture");
        let root = Utf8PathBuf::from_path_buf(directory.path().to_owned()).expect("UTF-8 path");
        let manifest: Manifest = toml::from_str(r#"packages = [
{ name = "hex_dep", version = "1.2.3", build_tools = ["gleam"], requirements = [], source = "hex", outer_checksum = "00" },
{ name = "git_dep", version = "1.0.0", build_tools = ["gleam"], requirements = [], source = "git", repo = "https://example.invalid/repo", commit = "first" },
{ name = "subdir", version = "1.0.0", build_tools = ["gleam"], requirements = [], source = "git", repo = "https://example.invalid/repo", commit = "first", path = "packages/child" },
]
[requirements]
"#).expect("cache identities");
        let empty = PackageCache::read(&root).expect("absent cache");
        for package in &manifest.packages {
            assert!(!empty.contains(package));
            fs::create_dir_all(root.join("build/packages").join(package.name.as_str()))
                .expect("source directory");
            assert!(!empty.contains(package));
        }
        fs::write(
            root.join("build/packages/packages.toml"),
            r#"[packages]
hex_dep = "1.2.3"
git_dep = "1.0.0"
subdir = "1.0.0"
[git.git_dep]
commit = "first"
[git.subdir]
commit = "first"
path = "packages/child"
"#,
        )
        .expect("Gleam cache protocol");
        let cache = PackageCache::read(&root).expect("Gleam cache metadata");
        for package in &manifest.packages {
            assert!(cache.contains(package));
        }
        let mut different = manifest.packages.clone();
        different[0].version = hexpm::version::Version::new(1, 2, 4);
        different[1].source = ManifestPackageSource::Git {
            repo: "https://example.invalid/repo".into(),
            commit: "second".into(),
            path: None,
        };
        different[2].source = ManifestPackageSource::Git {
            repo: "https://example.invalid/repo".into(),
            commit: "first".into(),
            path: Some("packages/other".into()),
        };
        for package in &different {
            assert!(!cache.contains(package));
        }
        fs::remove_dir(root.join("build/packages/git_dep")).expect("missing cached source");
        assert!(!cache.contains(&manifest.packages[1]));
    }

    #[test]
    fn publishes_downloaded_sources_and_merges_only_cache_metadata() {
        let directory = tempdir().expect("cache fixture");
        let root = Utf8PathBuf::from_path_buf(directory.path().to_owned()).expect("UTF-8 path");
        fs::create_dir_all(root.join("build/packages/library")).expect("old source");
        fs::write(root.join("build/packages/library/old"), "old").expect("old content");
        fs::write(
            root.join("build/packages/packages.toml"),
            r#"[packages]
unrelated = "2.0.0"
library = "1.0.0"
[git.library]
commit = "old"
"#,
        )
        .expect("existing metadata");
        let packages = toml::from_str::<Manifest>(r#"packages = [
{ name = "library", version = "1.2.3", build_tools = ["gleam"], requirements = [], source = "hex", outer_checksum = "00" },
{ name = "subdir", version = "1.0.0", build_tools = ["gleam"], requirements = [], source = "git", repo = "https://example.invalid/repo", commit = "locked", path = "packages/child" },
{ name = "git_root", version = "1.0.0", build_tools = ["gleam"], requirements = [], source = "git", repo = "https://example.invalid/repo", commit = "locked" },
]
[requirements]
"#).expect("downloaded identities").packages;
        let workspace = root.join("download");
        for package in &packages {
            let source = workspace.join("build/packages").join(package.name.as_str());
            fs::create_dir_all(&source).expect("download directory");
            fs::write(source.join("marker"), package.name.as_str()).expect("download content");
        }
        let mut cache = PackageCache::read(&root).expect("old cache");
        cache
            .publish(workspace.as_std_path(), &packages)
            .expect("publish checked sources");
        for package in &packages {
            assert!(cache.contains(package));
            assert!(
                !workspace
                    .join("build/packages")
                    .join(package.name.as_str())
                    .exists()
            );
            assert_eq!(
                fs::read_to_string(
                    root.join("build/packages")
                        .join(package.name.as_str())
                        .join("marker")
                )
                .expect("published source"),
                package.name.as_str()
            );
        }
        assert!(!root.join("build/packages/library/old").exists());
        assert_eq!(
            fs::read_to_string(root.join("build/packages/packages.toml")).expect("cache protocol"),
            r#"[packages]
git_root = "1.0.0"
library = "1.2.3"
subdir = "1.0.0"
unrelated = "2.0.0"

[git.git_root]
commit = "locked"

[git.subdir]
commit = "locked"
path = "packages/child"
"#
        );
        let loaded = PackageCache::read(&root).expect("published cache metadata");
        for package in &packages {
            assert!(loaded.contains(package));
        }
    }

    #[test]
    fn reports_cache_read_parse_publication_and_persist_failures() {
        let directory = tempdir().expect("cache fixture");
        let root = Utf8PathBuf::from_path_buf(directory.path().to_owned()).expect("UTF-8 path");
        let path = root.join("build/packages/packages.toml");
        fs::create_dir_all(&path).expect("unreadable metadata path");
        assert!(matches!(
            PackageCache::read(&root)
                .err()
                .expect("unreadable cache metadata"),
            CliError::FileRead { path: actual, error }
                if actual == path && error.kind() == io::ErrorKind::IsADirectory
        ));
        fs::remove_dir(&path).expect("remove conflicting directory");
        fs::write(&path, "invalid").expect("invalid cache document");
        assert!(matches!(
            PackageCache::read(&root),
            Err(CliError::InvalidToml {
                kind: "Gleam package cache",
                ..
            })
        ));
        fs::remove_file(&path).expect("remove malformed cache");
        let mut cache = PackageCache::read(&root).expect("absent metadata");
        let package: ManifestPackage = toml::from_str(
            r#"name = "library"
version = "1.0.0"
build_tools = ["gleam"]
requirements = []
source = "hex"
outer_checksum = "00"
"#,
        )
        .expect("locked package");
        let destination = root.join("build/packages/library");
        fs::write(&destination, "not a directory").expect("conflicting cache entry");
        assert!(
            matches!(cache.publish(root.join("absent").as_std_path(), std::slice::from_ref(&package)), Err(CliError::FileWrite { path, .. }) if path == destination)
        );
        fs::remove_file(&destination).expect("remove conflicting file");
        assert!(
            matches!(cache.publish(root.join("absent").as_std_path(), std::slice::from_ref(&package)), Err(CliError::FileWrite { path, .. }) if path == destination)
        );
        fs::create_dir(&path).expect("block metadata persist");
        assert!(
            matches!(cache.save(), Err(CliError::FileWrite { path: failed, .. }) if failed == path)
        );
        fs::remove_dir_all(root.join("build")).expect("remove fixture cache");
        assert!(
            matches!(cache.save(), Err(CliError::FileWrite { path: failed, .. }) if failed == path)
        );
        fs::write(root.join("build"), "not a directory").expect("block cache creation");
        assert!(
            matches!(cache.publish(root.as_std_path(), &[package]), Err(CliError::FileWrite { path, .. }) if path == root.join("build/packages"))
        );
    }

    #[test]
    fn preserves_cache_content_write_and_flush_errors() {
        struct FailedWriter {
            fail_write: bool,
        }
        impl Write for FailedWriter {
            fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
                if self.fail_write {
                    Err(io::Error::other("write failed"))
                } else {
                    Ok(bytes.len())
                }
            }
            fn flush(&mut self) -> io::Result<()> {
                Err(io::Error::other("flush failed"))
            }
        }
        let path = Utf8Path::new("build/packages/packages.toml");
        for (fail_write, expected) in [(true, "write failed"), (false, "flush failed")] {
            let error = write_state(&mut FailedWriter { fail_write }, path, b"state")
                .expect_err("write boundary failure");
            assert!(
                matches!(error, CliError::FileWrite { path: failed, error } if failed == path && error.to_string() == expected)
            );
        }
    }
}
