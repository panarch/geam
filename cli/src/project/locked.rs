mod cache;
mod download;

use super::{
    DependencyDownloader, MANIFEST_FILE, ProcessDependencyDownloader, into_utf8_path,
    read_package_config, read_toml,
};
use crate::error::CliError;
use camino::{Utf8Component, Utf8Path, Utf8PathBuf};
use gleam_core::manifest::{Manifest, ManifestPackage, ManifestPackageSource};
use gleam_core::requirement::Requirement;
use std::collections::{BTreeMap, BTreeSet};
use std::fs;

pub(crate) fn restore_locked_dependencies(project_root: &Utf8Path) -> Result<(), CliError> {
    restore_with(project_root, &ProcessDependencyDownloader::gleam())
}

fn restore_with(
    project_root: &Utf8Path,
    downloader: &dyn DependencyDownloader,
) -> Result<(), CliError> {
    let config = read_package_config(project_root)?;
    let manifest_path = project_root.join(MANIFEST_FILE);
    let manifest = match read_toml::<Manifest>("Gleam manifest", &manifest_path) {
        Err(CliError::FileRead { error, .. }) if error.kind() == std::io::ErrorKind::NotFound => {
            return Err(CliError::GleamLockOutOfDate {
                path: manifest_path,
                reason: "the file is missing".to_owned(),
            });
        }
        result => result?,
    };
    let root_requirements =
        config
            .all_direct_dependencies()
            .map_err(|error| CliError::InvalidToml {
                kind: "Gleam package config",
                path: project_root.join("gleam.toml"),
                reason: error.to_string(),
            })?;
    if root_requirements.len() != manifest.requirements.len() {
        return Err(CliError::GleamLockOutOfDate {
            path: manifest_path,
            reason: "root dependencies differ from the recorded requirements".to_owned(),
        });
    }
    for (name, requirement) in &root_requirements {
        if !same_requirement(project_root, requirement, manifest.requirements.get(name))? {
            return Err(CliError::GleamLockOutOfDate {
                path: manifest_path,
                reason: format!("the root requirement for {name} has changed"),
            });
        }
    }

    let packages = locked_packages(&manifest, &manifest_path, &config.name)?;
    for (name, requirement) in &root_requirements {
        validate_requirement(
            project_root,
            project_root,
            None,
            name,
            requirement,
            &packages,
        )?;
    }
    for package in packages.values().filter(|package| package.is_local()) {
        validate_package(project_root, package, &packages)?;
    }

    let mut cache = cache::PackageCache::read(project_root)?;
    let missing = packages
        .values()
        .filter(|package| !package.is_local() && !cache.contains(package))
        .map(|package| (*package).clone())
        .collect::<Vec<_>>();
    if !missing.is_empty() {
        let downloaded = download::acquire(project_root, &missing, downloader)?;
        cache.publish(downloaded.path(), &missing)?;
    }
    for package in packages.values().filter(|package| !package.is_local()) {
        validate_package(project_root, package, &packages)?;
    }
    Ok(())
}

fn same_requirement(
    root: &Utf8Path,
    current: &Requirement,
    recorded: Option<&Requirement>,
) -> Result<bool, CliError> {
    match (current, recorded) {
        (Requirement::Path { path: left }, Some(Requirement::Path { path: right })) => {
            Ok(canonical_path(&root.join(left))? == canonical_path(&root.join(right))?)
        }
        (_, recorded) => Ok(Some(current) == recorded),
    }
}

fn locked_packages<'a>(
    manifest: &'a Manifest,
    path: &Utf8Path,
    root_name: &str,
) -> Result<BTreeMap<&'a str, &'a ManifestPackage>, CliError> {
    let mut packages = BTreeMap::new();
    for package in &manifest.packages {
        if let ManifestPackageSource::Git { commit, .. } = &package.source
            && !(matches!(commit.len(), 40 | 64)
                && commit.bytes().all(|byte| byte.is_ascii_hexdigit()))
        {
            return Err(CliError::GleamLockOutOfDate {
                path: path.to_path_buf(),
                reason: format!(
                    "Git package {} is not locked to a full commit hash",
                    package.name
                ),
            });
        }
        if package.name == root_name || packages.insert(package.name.as_str(), package).is_some() {
            return Err(CliError::GleamLockOutOfDate {
                path: path.to_path_buf(),
                reason: format!("package {} is listed more than once", package.name),
            });
        }
    }
    let mut visited = BTreeSet::new();
    let mut pending = manifest
        .requirements
        .keys()
        .map(|name| name.as_str())
        .collect::<Vec<_>>();
    while let Some(name) = pending.pop() {
        if !visited.insert(name) {
            continue;
        }
        let package = packages
            .get(name)
            .ok_or_else(|| CliError::GleamLockOutOfDate {
                path: path.to_path_buf(),
                reason: format!("required package {name} is missing"),
            })?;
        pending.extend(package.requirements.iter().map(|name| name.as_str()));
    }
    if visited.len() != packages.len() {
        return Err(CliError::GleamLockOutOfDate {
            path: path.to_path_buf(),
            reason: "the lock contains packages outside the dependency graph".to_owned(),
        });
    }
    Ok(packages)
}

fn validate_requirement(
    project_root: &Utf8Path,
    owner_root: &Utf8Path,
    owner_source: Option<&ManifestPackageSource>,
    name: &str,
    requirement: &Requirement,
    packages: &BTreeMap<&str, &ManifestPackage>,
) -> Result<(), CliError> {
    let matches = match (requirement, packages.get(name)) {
        (Requirement::Hex { version }, Some(package)) => {
            version.to_pubgrub().contains(&package.version)
        }
        (Requirement::Git { git, path, .. }, Some(package)) => {
            matches!(&package.source, ManifestPackageSource::Git { repo, path: selected, .. }
                if git == repo && path == selected)
        }
        (Requirement::Path { path }, Some(package)) => match (owner_source, &package.source) {
            (
                Some(ManifestPackageSource::Git {
                    repo,
                    commit,
                    path: parent,
                }),
                ManifestPackageSource::Git {
                    repo: selected_repo,
                    commit: selected_commit,
                    path: selected_path,
                },
            ) => {
                repo == selected_repo
                    && commit == selected_commit
                    && repository_path(parent.as_deref(), path).as_ref()
                        == Some(&selected_path.clone().unwrap_or_default())
            }
            (Some(ManifestPackageSource::Git { .. }), _) => false,
            (_, ManifestPackageSource::Local { path: selected }) => {
                canonical_path(&owner_root.join(path))?
                    == canonical_path(&project_root.join(selected))?
            }
            _ => false,
        },
        (_, None) => false,
    };
    if matches {
        Ok(())
    } else {
        Err(CliError::GleamLockOutOfDate {
            path: project_root.join(MANIFEST_FILE),
            reason: format!(
                "the dependency {name} in {} does not match its locked package",
                owner_root.join("gleam.toml")
            ),
        })
    }
}

fn canonical_path(path: &Utf8Path) -> Result<Utf8PathBuf, CliError> {
    fs::canonicalize(path)
        .map_err(|error| CliError::FileRead {
            path: path.to_path_buf(),
            error,
        })
        .and_then(into_utf8_path)
}

fn repository_path(parent: Option<&Utf8Path>, child: &Utf8Path) -> Option<Utf8PathBuf> {
    let joined = parent.unwrap_or(Utf8Path::new("")).join(child);
    let mut path = Utf8PathBuf::new();
    for component in joined.components() {
        match component {
            Utf8Component::Normal(name) => path.push(name),
            Utf8Component::CurDir => (),
            Utf8Component::ParentDir if path.pop() => (),
            _ => return None,
        }
    }
    Some(path)
}

fn validate_package(
    project_root: &Utf8Path,
    package: &ManifestPackage,
    packages: &BTreeMap<&str, &ManifestPackage>,
) -> Result<(), CliError> {
    if !package.build_tools.iter().any(|tool| tool == "gleam") {
        return Ok(());
    }
    let root = match &package.source {
        ManifestPackageSource::Local { path } => project_root.join(path),
        _ => project_root
            .join("build/packages")
            .join(package.name.as_str()),
    };
    let config = read_package_config(&root)?;
    let declared = config
        .dependencies
        .keys()
        .map(|name| name.as_str())
        .collect::<BTreeSet<_>>();
    let recorded = package
        .requirements
        .iter()
        .map(|name| name.as_str())
        .collect::<BTreeSet<_>>();
    if config.name != package.name || config.version != package.version || declared != recorded {
        return Err(CliError::GleamLockOutOfDate {
            path: project_root.join(MANIFEST_FILE),
            reason: format!(
                "package name, version, or dependencies in {} differ from the lock",
                root.join("gleam.toml")
            ),
        });
    }
    for (name, requirement) in &config.dependencies {
        validate_requirement(
            project_root,
            &root,
            Some(&package.source),
            name,
            requirement,
            packages,
        )?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{
        locked_packages, repository_path, restore_locked_dependencies, restore_with,
        same_requirement, validate_requirement,
    };
    use crate::error::CliError;
    use crate::process::run_checked;
    use crate::project::{DependencyDownloader, ProcessDependencyDownloader, prepare_dependencies};
    use camino::{Utf8Path, Utf8PathBuf};
    use geam_core::{ExecutionPlan, Value, compile_typed_project, plan_program, run_main};
    use gleam_core::manifest::{Manifest, ManifestPackageSource};
    use gleam_core::requirement::Requirement;
    use std::cell::Cell;
    use std::collections::BTreeMap;
    use std::fs;
    use std::process::Command;
    use tempfile::tempdir;

    #[derive(Default)]
    struct UnavailableDownloader(Cell<usize>);

    impl DependencyDownloader for UnavailableDownloader {
        fn download(&self, _root: &Utf8Path) -> Result<(), CliError> {
            self.0.set(self.0.get() + 1);
            Err(CliError::ProcessFailure {
                command: "gleam deps download".to_owned(),
                status: Some(1),
                stderr: "download unavailable".to_owned(),
            })
        }
    }

    #[test]
    fn validates_local_constraints_without_a_download_or_project_write() {
        let directory = tempdir().expect("project directory");
        let root = Utf8PathBuf::from_path_buf(directory.path().to_owned()).expect("UTF-8 project");
        let config = "name = \"application\"\nversion = \"1.0.0\"\n[dependencies]\nlibrary = { path = \"library\" }\ntokens = { path = \"tokens\" }\n";
        let manifest = r#"packages = [
  { name = "library", version = "1.0.0", build_tools = ["gleam"], requirements = ["tokens"], source = "local", path = "library" },
  { name = "tokens", version = "1.2.3", build_tools = ["gleam"], requirements = [], source = "local", path = "tokens" },
]
[requirements]
library = { path = "./library" }
tokens = { path = "tokens" }
"#;
        let library = "name = \"library\"\nversion = \"1.0.0\"\n[dependencies]\ntokens = \">= 1.0.0 and < 2.0.0\"\n";
        write(&root.join("gleam.toml"), config);
        write(&root.join("manifest.toml"), manifest);
        write(&root.join("library/gleam.toml"), library);
        write(
            &root.join("tokens/gleam.toml"),
            "name = \"tokens\"\nversion = \"1.2.3\"\n",
        );
        let downloader = UnavailableDownloader::default();
        restore_with(&root, &downloader).expect("provided local version satisfies the Hex range");
        assert_eq!(downloader.0.get(), 0);
        assert!(!root.join("build").exists());
        for (changed, reason) in [
            (
                library.replace("< 2.0.0", "< 1.2.0"),
                format!(
                    "the dependency tokens in {} does not match its locked package",
                    root.join("library/gleam.toml")
                ),
            ),
            (
                library.replace("1.0.0\"", "1.0.1\""),
                format!(
                    "package name, version, or dependencies in {} differ from the lock",
                    root.join("library/gleam.toml")
                ),
            ),
            (
                library.replace("tokens =", "new_tokens ="),
                format!(
                    "package name, version, or dependencies in {} differ from the lock",
                    root.join("library/gleam.toml")
                ),
            ),
        ] {
            write(&root.join("library/gleam.toml"), &changed);
            assert_eq!(
                restore_with(&root, &downloader)
                    .expect_err("stale local constraint")
                    .to_string(),
                format!(
                    "Gleam lock {} is not ready: {reason}; run `geam embedding sync`",
                    root.join("manifest.toml")
                )
            );
            assert_eq!(
                fs::read_to_string(root.join("library/gleam.toml")).expect("local declaration"),
                changed
            );
            assert_eq!(
                fs::read_to_string(root.join("manifest.toml")).expect("unchanged lock"),
                manifest
            );
            assert_eq!(
                fs::read_to_string(root.join("gleam.toml")).expect("unchanged root config"),
                config
            );
        }
        assert_eq!(downloader.0.get(), 0);
    }

    #[test]
    fn distinguishes_missing_invalid_and_changed_lock_inputs_before_acquisition() {
        let directory = tempdir().expect("project directory");
        let root = Utf8PathBuf::from_path_buf(directory.path().to_owned()).expect("UTF-8 project");
        let downloader = UnavailableDownloader::default();
        let config = "name = \"application\"\nversion = \"1.0.0\"\n";
        write(&root.join("gleam.toml"), config);
        assert_eq!(
            restore_with(&root, &downloader)
                .expect_err("missing lock")
                .to_string(),
            format!(
                "Gleam lock {} is not ready: the file is missing; run `geam embedding sync`",
                root.join("manifest.toml")
            )
        );
        assert!(!root.join("manifest.toml").exists());
        write(&root.join("manifest.toml"), "invalid");
        assert!(matches!(
            restore_with(&root, &downloader),
            Err(CliError::InvalidToml {
                kind: "Gleam manifest",
                ..
            })
        ));
        fs::remove_file(root.join("manifest.toml")).expect("remove invalid fixture");
        fs::create_dir(root.join("manifest.toml")).expect("unreadable lock path");
        assert!(matches!(
            restore_with(&root, &downloader).expect_err("unreadable lock path"),
            CliError::FileRead { path, error }
                if path == root.join("manifest.toml")
                    && error.kind() == std::io::ErrorKind::IsADirectory
        ));
        fs::remove_dir(root.join("manifest.toml")).expect("remove lock directory");

        write(
            &root.join("manifest.toml"),
            "packages = []\n[requirements]\n",
        );
        write(
            &root.join("gleam.toml"),
            &format!(
                "{config}[dependencies]\nlibrary = \"1.0.0\"\n[dev-dependencies]\nlibrary = \"1.0.0\"\n"
            ),
        );
        assert!(matches!(
            restore_with(&root, &downloader),
            Err(CliError::InvalidToml {
                kind: "Gleam package config",
                ..
            })
        ));
        write(
            &root.join("gleam.toml"),
            &format!("{config}[dependencies]\nlibrary = \"1.0.0\"\n"),
        );
        assert_eq!(
            restore_with(&root, &downloader)
                .expect_err("missing recorded dependency")
                .to_string(),
            format!(
                "Gleam lock {} is not ready: root dependencies differ from the recorded requirements; run `geam embedding sync`",
                root.join("manifest.toml")
            )
        );
        for requirement in [
            "library = { version = \"2.0.0\" }",
            "other = { version = \"1.0.0\" }",
        ] {
            write(
                &root.join("manifest.toml"),
                &format!("packages = []\n[requirements]\n{requirement}\n"),
            );
            assert_eq!(
                restore_with(&root, &downloader)
                    .expect_err("changed root requirement")
                    .to_string(),
                format!(
                    "Gleam lock {} is not ready: the root requirement for library has changed; run `geam embedding sync`",
                    root.join("manifest.toml")
                )
            );
        }
        assert_eq!(downloader.0.get(), 0);
        assert!(!root.join("build").exists());
        assert!(matches!(
            same_requirement(
                &root,
                &Requirement::path("missing"),
                Some(&Requirement::path("other"))
            )
            .expect_err("missing requested local path"),
            CliError::FileRead { path, error }
                if path == root.join("missing")
                    && error.kind() == std::io::ErrorKind::NotFound
        ));
        fs::create_dir(root.join("present")).expect("local directory");
        assert!(matches!(
            same_requirement(
                &root,
                &Requirement::path("present"),
                Some(&Requirement::path("missing"))
            )
            .expect_err("missing recorded local path"),
            CliError::FileRead { path, error }
                if path == root.join("missing")
                    && error.kind() == std::io::ErrorKind::NotFound
        ));
    }

    #[test]
    fn rejects_duplicate_missing_and_unreachable_locked_packages() {
        let path = Utf8Path::new("project/manifest.toml");
        for (source, reason) in [
            (
                r#"packages = [{ name = "application", version = "1.0.0", build_tools = [], requirements = [], source = "local", path = "." }]
[requirements]
"#,
                "package application is listed more than once",
            ),
            (
                r#"packages = [
{ name = "library", version = "1.0.0", build_tools = [], requirements = [], source = "local", path = "library" },
{ name = "library", version = "1.0.0", build_tools = [], requirements = [], source = "local", path = "library" }]
[requirements]
library = { path = "library" }
"#,
                "package library is listed more than once",
            ),
            (
                r#"packages = []
[requirements]
library = { path = "library" }
"#,
                "required package library is missing",
            ),
            (
                r#"packages = [{ name = "library", version = "1.0.0", build_tools = [], requirements = [], source = "local", path = "library" }]
[requirements]
"#,
                "the lock contains packages outside the dependency graph",
            ),
        ] {
            let manifest: Manifest = toml::from_str(source).expect("lock shape");
            assert_eq!(
                locked_packages(&manifest, path, "application")
                    .expect_err("invalid graph")
                    .to_string(),
                format!("Gleam lock {path} is not ready: {reason}; run `geam embedding sync`")
            );
        }
    }

    #[test]
    fn requires_git_locks_to_name_full_commits_not_moving_refs() {
        let path = Utf8Path::new("project/manifest.toml");
        for commit in ["main".to_owned(), "a".repeat(39), "g".repeat(40)] {
            let manifest: Manifest = toml::from_str(&format!(r#"packages = [
{{ name = "library", version = "1.0.0", build_tools = ["gleam"], requirements = [], source = "git", repo = "https://example.invalid/repo", commit = "{commit}" }}]
[requirements]
library = {{ git = "https://example.invalid/repo", ref = "main" }}
"#)).expect("syntactically valid but unlocked Git selection");
            assert_eq!(
                locked_packages(&manifest, path, "application")
                    .expect_err("symbolic or invalid Git commit")
                    .to_string(),
                "Gleam lock project/manifest.toml is not ready: Git package library is not locked to a full commit hash; run `geam embedding sync`"
            );
        }
        for commit in ["a".repeat(40), "B".repeat(64)] {
            let manifest: Manifest = toml::from_str(&format!(r#"packages = [
{{ name = "library", version = "1.0.0", build_tools = ["gleam"], requirements = [], source = "git", repo = "https://example.invalid/repo", commit = "{commit}" }}]
[requirements]
library = {{ git = "https://example.invalid/repo", ref = "main" }}
"#)).expect("full Git identity");
            assert_eq!(
                locked_packages(&manifest, path, "application")
                    .expect("exact Git commit")
                    .keys()
                    .copied()
                    .collect::<Vec<_>>(),
                ["library"]
            );
        }
    }

    #[test]
    fn rejects_unavailable_or_inconsistent_project_inputs_before_downloading() {
        let directory = tempdir().expect("project fixture");
        let root = Utf8PathBuf::from_path_buf(directory.path().to_owned()).expect("UTF-8 project");
        let downloader = UnavailableDownloader::default();
        let missing = restore_with(&root, &downloader).expect_err("missing project config");
        assert_eq!(
            missing.to_string(),
            format!("failed to read {}", root.join("gleam.toml"))
        );
        write(
            &root.join("gleam.toml"),
            "name = \"application\"\nversion = \"1.0.0\"\n[dependencies]\nlibrary = \"1.0.0\"\n",
        );
        for (lock, reason) in [
            (
                "packages = []\n[requirements]\nlibrary = { version = \"1.0.0\" }\n",
                "required package library is missing".to_owned(),
            ),
            (
                r#"packages = [{ name = "library", version = "2.0.0", build_tools = ["gleam"], requirements = [], source = "hex", outer_checksum = "00" }]
[requirements]
library = { version = "1.0.0" }
"#,
                format!(
                    "the dependency library in {} does not match its locked package",
                    root.join("gleam.toml")
                ),
            ),
        ] {
            write(&root.join("manifest.toml"), lock);
            assert_eq!(
                restore_with(&root, &downloader)
                    .expect_err("inconsistent lock")
                    .to_string(),
                format!(
                    "Gleam lock {} is not ready: {reason}; run `geam embedding sync`",
                    root.join("manifest.toml")
                )
            );
            assert_eq!(
                fs::read_to_string(root.join("manifest.toml")).expect("preserved lock"),
                lock
            );
        }
        write(
            &root.join("gleam.toml"),
            "name = \"application\"\nversion = \"1.0.0\"\n[dependencies]\nlibrary = { path = \"library\" }\n",
        );
        let lock = r#"packages = [{ name = "library", version = "1.0.0", build_tools = ["gleam"], requirements = [], source = "local", path = "missing" }]
[requirements]
library = { path = "library" }
"#;
        write(&root.join("manifest.toml"), lock);
        assert_eq!(
            restore_with(&root, &downloader)
                .expect_err("missing declared local package")
                .to_string(),
            format!("failed to read {}", root.join("library"))
        );
        write(
            &root.join("library/gleam.toml"),
            "name = \"library\"\nversion = \"1.0.0\"\n",
        );
        assert_eq!(
            restore_with(&root, &downloader)
                .expect_err("missing locked local package")
                .to_string(),
            format!("failed to read {}", root.join("missing"))
        );
        assert_eq!(
            fs::read_to_string(root.join("manifest.toml")).expect("local lock unchanged"),
            lock
        );
        assert_eq!(downloader.0.get(), 0);
        assert!(!root.join("build").exists());
    }

    #[test]
    fn reports_incomplete_downloads_and_invalid_cached_packages_without_repairing_locks() {
        let directory = tempdir().expect("project fixture");
        let root = Utf8PathBuf::from_path_buf(directory.path().to_owned()).expect("UTF-8 project");
        let config =
            "name = \"application\"\nversion = \"1.0.0\"\n[dependencies]\nlibrary = \"1.0.0\"\n";
        let lock = r#"packages = [{ name = "library", version = "1.0.0", build_tools = ["gleam"], requirements = [], source = "hex", outer_checksum = "00" }]
[requirements]
library = { version = "1.0.0" }
"#;
        write(&root.join("gleam.toml"), config);
        write(&root.join("manifest.toml"), lock);
        let incomplete = ProcessDependencyDownloader::new("true", ["--"]);
        assert_eq!(
            restore_with(&root, &incomplete)
                .expect_err("tool did not produce the requested source")
                .to_string(),
            format!("failed to write {}", root.join("build/packages/library"))
        );
        write(
            &root.join("build/packages/packages.toml"),
            "[packages]\nlibrary = \"1.0.0\"\n",
        );
        fs::create_dir(root.join("build/packages/library")).expect("incomplete source cache");
        let downloader = UnavailableDownloader::default();
        assert_eq!(
            restore_with(&root, &downloader)
                .expect_err("missing cached config")
                .to_string(),
            format!(
                "failed to read {}",
                root.join("build/packages/library/gleam.toml")
            )
        );
        write(
            &root.join("build/packages/library/gleam.toml"),
            "name = \"library\"\nversion = \"2.0.0\"\n",
        );
        assert_eq!(
            restore_with(&root, &downloader)
                .expect_err("cached source differs from its lock")
                .to_string(),
            format!(
                "Gleam lock {} is not ready: package name, version, or dependencies in {} differ from the lock; run `geam embedding sync`",
                root.join("manifest.toml"),
                root.join("build/packages/library/gleam.toml")
            )
        );
        assert_eq!(
            fs::read_to_string(root.join("manifest.toml")).expect("unchanged lock"),
            lock
        );
        assert_eq!(
            fs::read_to_string(root.join("gleam.toml")).expect("unchanged config"),
            config
        );
        write(
            &root.join("build/packages/library/gleam.toml"),
            "name = \"library\"\nversion = \"1.0.0\"\n",
        );
        restore_with(&root, &downloader).expect("matching cached source");

        let erlang_lock = lock.replace("[\"gleam\"]", "[\"rebar3\"]");
        write(&root.join("manifest.toml"), &erlang_lock);
        fs::remove_file(root.join("build/packages/library/gleam.toml"))
            .expect("Erlang package has no Gleam config");
        restore_with(&root, &downloader).expect("other build tools do not require a Gleam config");
        assert_eq!(
            fs::read_to_string(root.join("manifest.toml")).expect("unchanged Erlang lock"),
            erlang_lock
        );
        assert_eq!(downloader.0.get(), 0);
    }

    #[test]
    fn restores_a_transitive_locked_git_commit_after_its_branch_moves() {
        let directory = tempdir().expect("project directory");
        let root =
            Utf8PathBuf::from_path_buf(fs::canonicalize(directory.path()).expect("canonical path"))
                .expect("UTF-8 fixture");
        let repo = root.join("git_dep");
        let local = root.join("local_dep");
        let app = root.join("application");
        write(
            &repo.join("gleam.toml"),
            "name = \"git_dep\"\nversion = \"1.0.0\"\n",
        );
        write(&repo.join("src/git_dep.gleam"), "pub fn value() { 1 }\n");
        git(&repo, &["init", "--initial-branch=main"]);
        git(&repo, &["add", "."]);
        git(&repo, &["commit", "-m", "Locked dependency"]);
        let first = git(&repo, &["rev-parse", "HEAD"]);
        write(
            &local.join("gleam.toml"),
            &format!(
                "name = \"local_dep\"\nversion = \"1.0.0\"\n[dependencies]\ngit_dep = {{ git = \"file://{repo}\", ref = \"main\" }}\n"
            ),
        );
        write(
            &local.join("src/local_dep.gleam"),
            "import git_dep\npub fn value() { git_dep.value() }\n",
        );
        write(
            &app.join("gleam.toml"),
            "name = \"application\"\nversion = \"1.0.0\"\n[dependencies]\nlocal_dep = { path = \"../local_dep\" }\n",
        );
        write(
            &app.join("src/application.gleam"),
            "import local_dep\npub fn main() { local_dep.value() }\n",
        );
        prepare_dependencies(&app).expect("initial Gleam resolution");
        let manifest = fs::read(app.join("manifest.toml")).expect("committed lock");
        let config = fs::read(app.join("gleam.toml")).expect("committed config");
        let local_config = fs::read(local.join("gleam.toml")).expect("local config");
        write(&repo.join("src/git_dep.gleam"), "pub fn value() { 2 }\n");
        git(&repo, &["add", "."]);
        git(&repo, &["commit", "-m", "Advance branch"]);
        assert_ne!(git(&repo, &["rev-parse", "HEAD"]), first);
        fs::remove_dir_all(app.join("build")).expect("empty dependency cache");

        restore_locked_dependencies(&app).expect("exact locked source acquisition");
        assert_eq!(
            git(&app.join("build/packages/git_dep"), &["rev-parse", "HEAD"]),
            first
        );
        assert_eq!(
            fs::read(app.join("manifest.toml")).expect("preserved lock"),
            manifest
        );
        assert_eq!(
            fs::read(app.join("gleam.toml")).expect("preserved config"),
            config
        );
        assert_eq!(
            fs::read(local.join("gleam.toml")).expect("preserved local config"),
            local_config
        );
        let program = compile_typed_project(&app, "application").expect("real source closure");
        let plan = plan_program(program).expect("locked source plans");
        assert_eq!(
            plan.modules()
                .iter()
                .map(|module| module
                    .source_context()
                    .expect("source context")
                    .path()
                    .to_string())
                .collect::<Vec<_>>(),
            [
                app.join("build/packages/git_dep/src/git_dep.gleam")
                    .to_string(),
                app.join("../local_dep/src/local_dep.gleam").to_string(),
                app.join("src/application.gleam").to_string()
            ]
        );
        let execution = ExecutionPlan::from_module_plan(plan);
        assert_eq!(
            run_main(&execution, &mut Vec::new()),
            Ok(Value::Int(1.into()))
        );
        let unavailable = UnavailableDownloader::default();
        restore_with(&app, &unavailable).expect("warm cache does not invoke Gleam");
        assert_eq!(unavailable.0.get(), 0);
        fs::remove_dir_all(app.join("build/packages/git_dep")).expect("one missing cached package");
        let failed = restore_with(&app, &unavailable).expect_err("download failure");
        assert_eq!(
            failed.to_string(),
            "`gleam deps download` failed with status Some(1): download unavailable"
        );
        assert_eq!(unavailable.0.get(), 1);
        assert_eq!(
            fs::read(app.join("manifest.toml")).expect("lock after failure"),
            manifest
        );
    }

    #[test]
    fn restores_git_subdirectory_packages_and_their_repository_local_dependencies() {
        let directory = tempdir().expect("project directory");
        let root =
            Utf8PathBuf::from_path_buf(fs::canonicalize(directory.path()).expect("canonical path"))
                .expect("UTF-8 fixture");
        let repo = root.join("repo");
        let app = root.join("application");
        write(
            &repo.join("packages/base/gleam.toml"),
            "name = \"base\"\nversion = \"1.0.0\"\n",
        );
        write(
            &repo.join("packages/base/src/base.gleam"),
            "pub fn value() { 7 }\n",
        );
        write(
            &repo.join("packages/child/gleam.toml"),
            "name = \"child\"\nversion = \"1.0.0\"\n[dependencies]\nbase = { path = \"../base\" }\n",
        );
        write(
            &repo.join("packages/child/src/child.gleam"),
            "import base\npub fn value() { base.value() }\n",
        );
        git(&repo, &["init", "--initial-branch=main"]);
        git(&repo, &["add", "."]);
        git(&repo, &["commit", "-m", "Git subdirectory packages"]);
        write(
            &app.join("gleam.toml"),
            &format!(
                "name = \"application\"\nversion = \"1.0.0\"\n[dependencies]\nchild = {{ git = \"file://{repo}\", ref = \"main\", path = \"packages/child\" }}\n"
            ),
        );
        write(
            &app.join("src/application.gleam"),
            "import child\npub fn main() { child.value() }\n",
        );
        prepare_dependencies(&app).expect("Gleam resolves repository local sources");
        let manifest = fs::read(app.join("manifest.toml")).expect("locked subdirectories");
        fs::remove_dir_all(app.join("build")).expect("cold repository cache");
        restore_locked_dependencies(&app).expect("restore both exact Git packages");
        let base = fs::read(app.join("build/packages/base/src/base.gleam"))
            .expect("cached dependency source");
        fs::remove_dir_all(app.join("build/packages/child"))
            .expect("partially populated source cache");
        restore_locked_dependencies(&app)
            .expect("restore one package without resolving its already cached dependency");
        assert_eq!(
            fs::read(app.join("build/packages/base/src/base.gleam"))
                .expect("preserved cached dependency"),
            base
        );
        let plan = plan_program(compile_typed_project(&app, "application").expect("typed source"))
            .expect("planned source");
        assert_eq!(
            run_main(&ExecutionPlan::from_module_plan(plan), &mut Vec::new()),
            Ok(Value::Int(7.into()))
        );
        assert_eq!(
            fs::read(app.join("manifest.toml")).expect("unchanged lock"),
            manifest
        );
    }

    #[test]
    fn checks_git_and_path_source_identity_without_resolving_branch_names() {
        let source = r#"packages = [
{ name = "base", version = "1.0.0", build_tools = ["gleam"], requirements = [], source = "git", repo = "https://example.invalid/repo", commit = "abc123", path = "packages/base" },
{ name = "root", version = "1.0.0", build_tools = ["gleam"], requirements = [], source = "git", repo = "https://example.invalid/repo", commit = "abc123" },
{ name = "local", version = "1.0.0", build_tools = ["gleam"], requirements = [], source = "local", path = "local" }
]
[requirements]
"#;
        let manifest: Manifest = toml::from_str(source).expect("locked source identities");
        let packages = manifest
            .packages
            .iter()
            .map(|package| (package.name.as_str(), package))
            .collect::<BTreeMap<_, _>>();
        let parent = ManifestPackageSource::Git {
            repo: "https://example.invalid/repo".into(),
            commit: "abc123".into(),
            path: Some("packages/child".into()),
        };
        let root = Utf8Path::new("project");
        let owner = root.join("build/packages/child");
        for (name, requirement) in [
            ("base", Requirement::path("../base")),
            ("root", Requirement::path("../..")),
            (
                "base",
                Requirement::git_with_path(
                    "https://example.invalid/repo",
                    "moving_branch",
                    "packages/base",
                ),
            ),
        ] {
            validate_requirement(root, &owner, Some(&parent), name, &requirement, &packages)
                .expect("locked source identity");
        }
        for (name, requirement) in [
            (
                "base",
                Requirement::git("https://example.invalid/other", "main"),
            ),
            (
                "base",
                Requirement::git("https://example.invalid/repo", "main"),
            ),
            ("base", Requirement::path("../../outside")),
            ("base", Requirement::path("../../../escape")),
            ("local", Requirement::path("../base")),
            ("missing", Requirement::hex("1.0.0").expect("range")),
        ] {
            assert_eq!(
                validate_requirement(root, &owner, Some(&parent), name, &requirement, &packages)
                    .expect_err("source mismatch")
                    .to_string(),
                format!(
                    "Gleam lock project/manifest.toml is not ready: the dependency {name} in project/build/packages/child/gleam.toml does not match its locked package; run `geam embedding sync`"
                )
            );
        }
        assert_eq!(
            repository_path(None, Utf8Path::new("./package")),
            Some("package".into())
        );
        assert_eq!(repository_path(None, Utf8Path::new("../escape")), None);
        assert_eq!(repository_path(None, Utf8Path::new("/outside")), None);
        assert!(matches!(
            validate_requirement(
                root,
                root,
                None,
                "local",
                &Requirement::path("missing"),
                &packages
            )
            .expect_err("unavailable local dependency"),
            CliError::FileRead { path, error }
                if path == root.join("missing")
                    && error.kind() == std::io::ErrorKind::NotFound
        ));
        assert!(matches!(
            validate_requirement(
                root,
                root,
                None,
                "base",
                &Requirement::path("base"),
                &packages
            )
            .expect_err("different package source kind"),
            CliError::GleamLockOutOfDate { path, reason }
                if path == root.join("manifest.toml")
                    && reason == "the dependency base in project/gleam.toml does not match its locked package"
        ));
    }

    fn write(path: &Utf8Path, source: &str) {
        fs::create_dir_all(path.parent().expect("fixture parent")).expect("fixture directories");
        fs::write(path, source).expect("fixture source");
    }

    fn git(root: &Utf8Path, args: &[&str]) -> String {
        let output = run_checked(
            Command::new("git")
                .args([
                    "-c",
                    "user.name=Geam Fixture",
                    "-c",
                    "user.email=fixture@example.invalid",
                ])
                .args(args)
                .current_dir(root),
        )
        .expect("local Git fixture command");
        String::from_utf8(output.stdout)
            .expect("Git output")
            .trim()
            .to_owned()
    }
}
