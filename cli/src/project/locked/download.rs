use crate::error::CliError;
use crate::project::{DependencyDownloader, into_utf8_path, read_toml};
use camino::Utf8Path;
use gleam_core::manifest::{Manifest, ManifestPackage, ManifestPackageSource};
use std::fs;
use toml_edit::{Array, DocumentMut, InlineTable, Table, value};

pub(super) fn acquire(
    project_root: &Utf8Path,
    packages: &[ManifestPackage],
    downloader: &dyn DependencyDownloader,
) -> Result<tempfile::TempDir, CliError> {
    let directory = project_root.join("build");
    let workspace = fs::create_dir_all(&directory)
        .and_then(|()| {
            tempfile::Builder::new()
                .prefix("geam-download-")
                .tempdir_in(&directory)
        })
        .map_err(CliError::TemporaryDependencyWorkspace)?;
    into_utf8_path(workspace.path().to_path_buf()).and_then(|root| {
        let (config, manifest) = documents(packages);
        let path = root.join("manifest.toml");
        write_documents(&root, config, manifest)
            .and_then(|()| read_toml::<Manifest>("Gleam download manifest", &path))
            .and_then(|expected| {
                downloader.download(&root)?;
                let mut actual = read_toml::<Manifest>("Gleam download manifest", &path)?;
                actual.packages.sort();
                let mut expected_packages = expected.packages;
                expected_packages.sort();
                if actual.requirements != expected.requirements
                    || actual.packages != expected_packages
                {
                    return Err(CliError::GleamDownloadChangedLock {
                        path: project_root.join("manifest.toml"),
                    });
                }
                Ok(workspace)
            })
    })
}

fn write_documents(
    root: &Utf8Path,
    config: DocumentMut,
    manifest: DocumentMut,
) -> Result<(), CliError> {
    for (name, document) in [("gleam.toml", config), ("manifest.toml", manifest)] {
        let path = root.join(name);
        fs::write(&path, document.to_string())
            .map_err(|error| CliError::FileWrite { path, error })?;
    }
    Ok(())
}

fn documents(packages: &[ManifestPackage]) -> (DocumentMut, DocumentMut) {
    let mut config = DocumentMut::new();
    config["name"] = value("geam_locked_download");
    config["version"] = value("1.0.0");
    let mut requirements = Table::new();
    let mut entries = Array::new();
    for package in packages {
        let mut entry = InlineTable::new();
        let mut requirement = InlineTable::new();
        match &package.source {
            ManifestPackageSource::Hex { outer_checksum } => {
                entry.insert("source", "hex".into());
                entry.insert(
                    "outer_checksum",
                    outer_checksum.base_16_encoded_string().into(),
                );
                requirement.insert("version", format!("== {}", package.version).into());
            }
            ManifestPackageSource::Git { repo, commit, path } => {
                entry.insert("source", "git".into());
                entry.insert("repo", repo.as_str().into());
                entry.insert("commit", commit.as_str().into());
                requirement.insert("git", repo.as_str().into());
                requirement.insert("ref", commit.as_str().into());
                if let Some(path) = path {
                    entry.insert("path", path.as_str().into());
                    requirement.insert("path", path.as_str().into());
                }
            }
            ManifestPackageSource::Local { .. } => continue,
        }
        entry.insert("name", package.name.as_str().into());
        entry.insert("version", package.version.to_string().into());
        entry.insert(
            "build_tools",
            package
                .build_tools
                .iter()
                .map(|tool| tool.as_str())
                .collect::<Array>()
                .into(),
        );
        entry.insert(
            "requirements",
            package
                .requirements
                .iter()
                .map(|name| name.as_str())
                .collect::<Array>()
                .into(),
        );
        if let Some(app) = &package.otp_app {
            entry.insert("otp_app", app.as_str().into());
        }
        entries.push(entry);
        requirements.insert(package.name.as_str(), value(requirement));
    }
    // Only locked remote packages are direct roots here. No local requirement
    // can cause Gleam to resolve a moving transitive Git ref in this view.
    config["dependencies"] = requirements.clone().into();
    let mut manifest = DocumentMut::new();
    manifest["packages"] = value(entries);
    manifest["requirements"] = requirements.into();
    (config, manifest)
}

#[cfg(test)]
mod tests {
    use super::{acquire, documents, write_documents};
    use crate::error::CliError;
    use crate::project::{DependencyDownloader, ProcessDependencyDownloader};
    use camino::{Utf8Path, Utf8PathBuf};
    use gleam_core::config::PackageConfig;
    use gleam_core::manifest::Manifest;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn renders_an_exact_remote_download_view_without_local_resolution_roots() {
        let original: Manifest = toml::from_str(r#"packages = [
{ name = "hex_dep", version = "1.2.3", build_tools = ["gleam", "rebar3"], requirements = ["git_subdir"], source = "hex", outer_checksum = "00AB", otp_app = "hex_app" },
{ name = "git_subdir", version = "2.0.0", build_tools = ["gleam"], requirements = ["git_root"], source = "git", repo = "https://example.invalid/repository", commit = "locked", path = "packages/child" },
{ name = "git_root", version = "3.0.0", build_tools = [], requirements = [], source = "git", repo = "https://example.invalid/root", commit = "root_commit" },
{ name = "local_dep", version = "1.0.0", build_tools = ["gleam"], requirements = ["hex_dep"], source = "local", path = "../local_dep" },
]
[requirements]
local_dep = { path = "../local_dep" }
"#).expect("original locked packages");
        let (config, manifest) = documents(&original.packages);
        assert_eq!(
            config.to_string(),
            r#"name = "geam_locked_download"
version = "1.0.0"

[dependencies]
hex_dep = { version = "== 1.2.3" }
git_subdir = { git = "https://example.invalid/repository", ref = "locked", path = "packages/child" }
git_root = { git = "https://example.invalid/root", ref = "root_commit" }
"#
        );
        assert_eq!(
            manifest.to_string(),
            concat!(
                "packages = [{ source = \"hex\", outer_checksum = \"00AB\", name = \"hex_dep\", version = \"1.2.3\", build_tools = [\"gleam\", \"rebar3\"], requirements = [\"git_subdir\"], otp_app = \"hex_app\" }, ",
                "{ source = \"git\", repo = \"https://example.invalid/repository\", commit = \"locked\", path = \"packages/child\", name = \"git_subdir\", version = \"2.0.0\", build_tools = [\"gleam\"], requirements = [\"git_root\"] }, ",
                "{ source = \"git\", repo = \"https://example.invalid/root\", commit = \"root_commit\", name = \"git_root\", version = \"3.0.0\", build_tools = [], requirements = [] }]\n",
                "\n[requirements]\n",
                "hex_dep = { version = \"== 1.2.3\" }\n",
                "git_subdir = { git = \"https://example.invalid/repository\", ref = \"locked\", path = \"packages/child\" }\n",
                "git_root = { git = \"https://example.invalid/root\", ref = \"root_commit\" }\n",
            )
        );
        let parsed_config: PackageConfig =
            toml::from_str(&config.to_string()).expect("Gleam config protocol");
        let parsed_manifest: Manifest =
            toml::from_str(&manifest.to_string()).expect("Gleam lock protocol");
        assert_eq!(parsed_config.dependencies, parsed_manifest.requirements);
        assert_eq!(parsed_manifest.packages, original.packages[..3]);
    }

    struct ManifestDownload<'a> {
        manifest: Option<&'a str>,
    }

    impl DependencyDownloader for ManifestDownload<'_> {
        fn download(&self, root: &Utf8Path) -> Result<(), CliError> {
            fs::create_dir_all(root.join("build/packages/unexpected"))
                .expect("downloaded cache fixture");
            match self.manifest {
                Some(source) => {
                    fs::write(root.join("manifest.toml"), source).expect("tool-produced manifest")
                }
                None => {
                    fs::remove_file(root.join("manifest.toml")).expect("lost download manifest")
                }
            }
            Ok(())
        }
    }

    #[test]
    fn rejects_changed_download_locks_before_publishing_any_cache_material() {
        let directory = tempdir().expect("download fixture");
        let root = Utf8PathBuf::from_path_buf(directory.path().to_owned()).expect("UTF-8 path");
        let config = "name = \"application\"\nversion = \"1.0.0\"\n";
        let lock = "packages = []\n[requirements]\n";
        fs::write(root.join("gleam.toml"), config).expect("original config");
        fs::write(root.join("manifest.toml"), lock).expect("original lock");
        for changed in [
            "packages = []\n[requirements]\nnew_dep = { version = \"1.0.0\" }\n",
            r#"packages = [{ name = "new_dep", version = "1.0.0", build_tools = [], requirements = [], source = "hex", outer_checksum = "00" }]
[requirements]
"#,
        ] {
            let error = acquire(
                &root,
                &[],
                &ManifestDownload {
                    manifest: Some(changed),
                },
            )
            .expect_err("changed locked selection");
            assert_eq!(
                error.to_string(),
                format!(
                    "Gleam dependency download for {} did not preserve the locked package set",
                    root.join("manifest.toml")
                )
            );
            assert_eq!(
                fs::read_to_string(root.join("manifest.toml")).expect("preserved original lock"),
                lock
            );
            assert_eq!(
                fs::read_to_string(root.join("gleam.toml")).expect("preserved original config"),
                config
            );
            assert!(!root.join("build/packages").exists());
            assert_eq!(
                fs::read_dir(root.join("build"))
                    .expect("temporary view removed")
                    .count(),
                0
            );
        }
    }

    #[test]
    fn accepts_package_reordering_but_not_a_different_identity() {
        let source = r#"packages = [
{ name = "second", version = "2.0.0", build_tools = ["gleam"], requirements = [], source = "git", repo = "https://example.invalid/repo", commit = "locked" },
{ name = "first", version = "1.0.0", build_tools = ["gleam"], requirements = ["second"], source = "hex", outer_checksum = "00" },
]
[requirements]
first = { version = "== 1.0.0" }
second = { git = "https://example.invalid/repo", ref = "locked" }
"#;
        let mut manifest: Manifest = toml::from_str(source).expect("tool-produced order");
        manifest.packages.reverse();
        let directory = tempdir().expect("download fixture");
        let root = Utf8PathBuf::from_path_buf(directory.path().to_owned()).expect("UTF-8 path");
        let workspace = acquire(
            &root,
            &manifest.packages,
            &ManifestDownload {
                manifest: Some(source),
            },
        )
        .expect("unchanged package identity");
        assert!(workspace.path().join("build/packages/unexpected").is_dir());
        assert!(!root.join("build/packages").exists());
        for changed in [
            source.replace("outer_checksum = \"00\"", "outer_checksum = \"01\""),
            source.replace("commit = \"locked\"", "commit = \"different\""),
        ] {
            assert!(matches!(
                acquire(
                    &root,
                    &manifest.packages,
                    &ManifestDownload {
                        manifest: Some(&changed)
                    }
                )
                .expect_err("changed locked identity"),
                CliError::GleamDownloadChangedLock { path }
                    if path == root.join("manifest.toml")
            ));
        }
    }

    #[test]
    fn preserves_download_process_and_file_failures_without_project_writes() {
        let directory = tempdir().expect("download fixture");
        let root = Utf8PathBuf::from_path_buf(directory.path().to_owned()).expect("UTF-8 path");
        fs::write(root.join("manifest.toml"), "original lock").expect("original project file");
        assert!(matches!(
            acquire(&root, &[], &ManifestDownload { manifest: None })
                .expect_err("removed download manifest"),
            CliError::FileRead { path, error }
                if path.starts_with(root.join("build"))
                    && path.file_name() == Some("manifest.toml")
                    && error.kind() == std::io::ErrorKind::NotFound
        ));
        assert!(matches!(
            acquire(
                &root,
                &[],
                &ManifestDownload {
                    manifest: Some("invalid")
                }
            )
            .expect_err("malformed download manifest"),
            CliError::InvalidToml {
                kind: "Gleam download manifest",
                ..
            }
        ));
        let missing =
            ProcessDependencyDownloader::new(root.join("missing-gleam"), ["deps", "download"]);
        assert!(matches!(
            acquire(&root, &[], &missing).expect_err("missing download tool"),
            CliError::ProcessIo { command, error }
                if command == format!("{} deps download", root.join("missing-gleam"))
                    && error.kind() == std::io::ErrorKind::NotFound
        ));
        fs::remove_dir(root.join("build")).expect("download views cleaned on error");
        fs::write(root.join("build"), "not a directory").expect("blocked cache directory");
        assert!(matches!(
            acquire(&root, &[], &missing).expect_err("blocked download directory"),
            CliError::TemporaryDependencyWorkspace(error)
                if error.kind() == std::io::ErrorKind::AlreadyExists
        ));
        assert_eq!(
            fs::read_to_string(root.join("manifest.toml")).expect("unchanged lock"),
            "original lock"
        );

        fs::create_dir(root.join("gleam.toml")).expect("blocked download config");
        let (config, manifest) = documents(&[]);
        assert!(
            matches!(write_documents(&root, config, manifest), Err(CliError::FileWrite { path, .. }) if path == root.join("gleam.toml"))
        );
        fs::remove_dir(root.join("gleam.toml")).expect("remove blocked config");
        fs::remove_file(root.join("manifest.toml")).expect("prepare blocked manifest");
        fs::create_dir(root.join("manifest.toml")).expect("blocked download manifest");
        let (config, manifest) = documents(&[]);
        assert!(
            matches!(write_documents(&root, config, manifest), Err(CliError::FileWrite { path, .. }) if path == root.join("manifest.toml"))
        );
    }
}
