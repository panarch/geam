use super::program::{ParsedModule, compile_parsed_package_program, parse_module};
use super::{FrontendError, ModuleSource, TypedProgram};
use camino::{Utf8Path, Utf8PathBuf};
use ecow::EcoString;
use gleam_core::build::Target;
use gleam_core::config::PackageConfig;
use gleam_core::manifest::{Manifest, ManifestPackage, ManifestPackageSource};
use gleam_core::type_::PRELUDE_MODULE_NAME;
use gleam_core::warning::WarningEmitter;
use std::collections::{BTreeMap, BTreeSet};
use std::ffi::OsString;
use std::fs;
use thiserror::Error;

const CONFIG_FILE: &str = "gleam.toml";
const MANIFEST_FILE: &str = "manifest.toml";

#[derive(Debug, Error)]
pub enum ProjectError {
    #[error("failed to read Gleam package config {path}")]
    ConfigIo {
        path: Utf8PathBuf,
        #[source]
        error: std::io::Error,
    },

    #[error("failed to read Gleam package manifest {path}")]
    ManifestIo {
        path: Utf8PathBuf,
        #[source]
        error: std::io::Error,
    },

    #[error("failed to read Gleam source {path}")]
    SourceIo {
        path: Utf8PathBuf,
        #[source]
        error: std::io::Error,
    },

    #[error("invalid Gleam package config {path}: {reason}")]
    InvalidConfig { path: Utf8PathBuf, reason: String },

    #[error("invalid Gleam package manifest {path}: {reason}")]
    InvalidManifest { path: Utf8PathBuf, reason: String },

    #[error("package {package} is incompatible with the embedded Gleam compiler: {error}")]
    IncompatibleCompilerVersion {
        package: EcoString,
        error: Box<gleam_core::Error>,
    },

    #[error("package {required_by} requires {package}, but it is absent from manifest.toml")]
    MissingManifestPackage {
        required_by: EcoString,
        package: EcoString,
    },

    #[error(
        "resolved package {package} is not downloaded at {path}; run `gleam deps download` first"
    )]
    MissingDownloadedPackage {
        package: EcoString,
        path: Utf8PathBuf,
    },

    #[error(
        "manifest package {expected} resolves to a package config named {actual} at {config_path}"
    )]
    PackageNameMismatch {
        expected: EcoString,
        actual: EcoString,
        config_path: Utf8PathBuf,
    },

    #[error(transparent)]
    Frontend(#[from] FrontendError),
}

pub fn compile_typed_project(
    project_root: impl Into<Utf8PathBuf>,
    root_module: impl Into<EcoString>,
) -> Result<TypedProgram, ProjectError> {
    compile_project(project_root.into(), root_module.into())
}

fn compile_project(
    project_root: Utf8PathBuf,
    root_module: EcoString,
) -> Result<TypedProgram, ProjectError> {
    let root_config = read_config(&project_root.join(CONFIG_FILE))?;
    let manifest = read_manifest(&project_root.join(MANIFEST_FILE))?;
    let manifest_packages = manifest_packages(&project_root, &manifest)?;
    let packages = load_packages(&project_root, root_config, &manifest_packages)?;
    let catalog = source_catalog(&packages)?;
    let parsed = select_import_closure(&packages.root, &root_module, &catalog)?;

    compile_parsed_package_program(
        packages.root.name.clone(),
        root_module,
        parsed,
        WarningEmitter::null(),
    )
    .map_err(ProjectError::from)
}

fn read_config(path: &Utf8Path) -> Result<PackageConfig, ProjectError> {
    let source = fs::read_to_string(path).map_err(|error| ProjectError::ConfigIo {
        path: path.to_path_buf(),
        error,
    })?;
    let config =
        toml::from_str::<PackageConfig>(&source).map_err(|error| ProjectError::InvalidConfig {
            path: path.to_path_buf(),
            reason: error.to_string(),
        })?;
    config.check_gleam_compatibility().map_err(|error| {
        ProjectError::IncompatibleCompilerVersion {
            package: config.name.clone(),
            error: Box::new(error),
        }
    })?;
    Ok(config)
}

fn read_manifest(path: &Utf8Path) -> Result<Manifest, ProjectError> {
    let source = fs::read_to_string(path).map_err(|error| ProjectError::ManifestIo {
        path: path.to_path_buf(),
        error,
    })?;
    toml::from_str(&source).map_err(|error| ProjectError::InvalidManifest {
        path: path.to_path_buf(),
        reason: error.to_string(),
    })
}

fn manifest_packages<'manifest>(
    project_root: &Utf8Path,
    manifest: &'manifest Manifest,
) -> Result<BTreeMap<EcoString, &'manifest ManifestPackage>, ProjectError> {
    let mut packages = BTreeMap::new();
    for package in &manifest.packages {
        if packages.insert(package.name.clone(), package).is_some() {
            return Err(ProjectError::InvalidManifest {
                path: project_root.join(MANIFEST_FILE),
                reason: format!("package {} is listed more than once", package.name),
            });
        }
    }
    Ok(packages)
}

struct LoadedPackage {
    name: EcoString,
    root: Utf8PathBuf,
    direct_dependencies: Box<[EcoString]>,
}

struct LoadedProject {
    root: LoadedPackage,
    dependencies: Vec<LoadedPackage>,
}

fn load_packages(
    project_root: &Utf8Path,
    root_config: PackageConfig,
    manifest_packages: &BTreeMap<EcoString, &ManifestPackage>,
) -> Result<LoadedProject, ProjectError> {
    let root_name = root_config.name.clone();
    let root_dependencies = dependency_names(&root_config);
    let root = LoadedPackage {
        name: root_name.clone(),
        root: project_root.to_path_buf(),
        direct_dependencies: root_dependencies.clone(),
    };
    let mut dependencies = Vec::new();
    let mut loaded = BTreeSet::from([root_name.clone()]);
    let mut pending = root_dependencies
        .iter()
        .cloned()
        .map(|package| (root_name.clone(), package))
        .collect::<BTreeSet<_>>();

    while let Some((required_by, package_name)) = pending.pop_first() {
        if loaded.contains(&package_name) {
            continue;
        }
        let package = manifest_packages
            .get(&package_name)
            .copied()
            .ok_or_else(|| ProjectError::MissingManifestPackage {
                required_by,
                package: package_name.clone(),
            })?;
        loaded.insert(package_name.clone());
        if !package.build_tools.iter().any(|tool| tool == "gleam") {
            continue;
        }

        let package_root = package_root(project_root, package);
        if !package_root.is_dir() && !package.is_local() {
            return Err(ProjectError::MissingDownloadedPackage {
                package: package_name,
                path: package_root,
            });
        }
        let config_path = package_root.join(CONFIG_FILE);
        let config = read_config(&config_path)?;
        if config.name != package.name {
            return Err(ProjectError::PackageNameMismatch {
                expected: package.name.clone(),
                actual: config.name,
                config_path,
            });
        }
        let direct_dependencies = dependency_names(&config);
        for dependency in &direct_dependencies {
            pending.insert((package.name.clone(), dependency.clone()));
        }
        dependencies.push(LoadedPackage {
            name: package.name.clone(),
            root: package_root,
            direct_dependencies,
        });
    }

    Ok(LoadedProject { root, dependencies })
}

fn dependency_names(config: &PackageConfig) -> Box<[EcoString]> {
    config
        .dependencies
        .keys()
        .cloned()
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect()
}

fn package_root(project_root: &Utf8Path, package: &ManifestPackage) -> Utf8PathBuf {
    match &package.source {
        ManifestPackageSource::Hex { .. } | ManifestPackageSource::Git { .. } => project_root
            .join("build")
            .join("packages")
            .join(package.name.as_str()),
        ManifestPackageSource::Local { path } if path.is_absolute() => path.clone(),
        ManifestPackageSource::Local { path } => project_root.join(path),
    }
}

#[derive(Clone)]
struct CatalogModule {
    package: EcoString,
    direct_dependencies: Box<[EcoString]>,
    module: EcoString,
    path: Utf8PathBuf,
}

fn source_catalog(
    project: &LoadedProject,
) -> Result<BTreeMap<EcoString, Vec<CatalogModule>>, ProjectError> {
    let mut catalog = BTreeMap::<EcoString, Vec<CatalogModule>>::new();
    for package in std::iter::once(&project.root).chain(&project.dependencies) {
        let source_root = package.root.join("src");
        for (module, path) in source_paths(&source_root)? {
            catalog
                .entry(module.clone())
                .or_default()
                .push(CatalogModule {
                    package: package.name.clone(),
                    direct_dependencies: package.direct_dependencies.clone(),
                    module,
                    path,
                });
        }
    }
    Ok(catalog)
}

fn source_paths(directory: &Utf8Path) -> Result<Vec<(EcoString, Utf8PathBuf)>, ProjectError> {
    source_paths_from(&FileSystemSourceDirectory, directory, Utf8Path::new(""))
}

trait SourceDirectory {
    fn entries(&self, directory: &Utf8Path)
    -> Result<Vec<std::io::Result<OsString>>, ProjectError>;
}

struct FileSystemSourceDirectory;

impl SourceDirectory for FileSystemSourceDirectory {
    fn entries(
        &self,
        directory: &Utf8Path,
    ) -> Result<Vec<std::io::Result<OsString>>, ProjectError> {
        fs::read_dir(directory)
            .map(|entries| {
                entries
                    .map(|entry| entry.map(|entry| entry.file_name()))
                    .collect()
            })
            .map_err(|error| ProjectError::SourceIo {
                path: directory.to_path_buf(),
                error,
            })
    }
}

fn source_paths_from(
    source_directory: &dyn SourceDirectory,
    directory: &Utf8Path,
    relative_directory: &Utf8Path,
) -> Result<Vec<(EcoString, Utf8PathBuf)>, ProjectError> {
    let mut paths = Vec::new();
    for entry in source_directory.entries(directory)? {
        let file_name = source_file_name(
            directory,
            entry.map_err(|error| ProjectError::SourceIo {
                path: directory.to_path_buf(),
                error,
            })?,
        )?;
        let path = directory.join(&file_name);
        let relative = relative_directory.join(file_name);
        if path.is_dir() {
            paths.extend(source_paths_from(source_directory, &path, &relative)?);
        } else if path.extension() == Some("gleam") {
            let module = relative
                .with_extension("")
                .as_str()
                .replace('\\', "/")
                .into();
            paths.push((module, path));
        }
    }
    paths.sort_by(|left, right| left.1.cmp(&right.1));
    Ok(paths)
}

fn source_file_name(directory: &Utf8Path, file_name: OsString) -> Result<String, ProjectError> {
    file_name
        .into_string()
        .map_err(|file_name| ProjectError::SourceIo {
            path: directory.to_path_buf(),
            error: std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("non-UTF-8 source path: {}", file_name.to_string_lossy()),
            ),
        })
}

fn select_import_closure(
    root_package: &LoadedPackage,
    root_module: &EcoString,
    catalog: &BTreeMap<EcoString, Vec<CatalogModule>>,
) -> Result<Vec<ParsedModule>, ProjectError> {
    let root = catalog
        .get(root_module)
        .and_then(|modules| {
            modules
                .iter()
                .find(|module| module.package == root_package.name)
        })
        .cloned()
        .ok_or_else(|| {
            ProjectError::Frontend(FrontendError::MissingRootModule {
                package: root_package.name.clone(),
                module: root_module.clone(),
            })
        })?;
    let warnings = WarningEmitter::null();
    let mut pending = BTreeMap::from([((root.package.clone(), root.module.clone()), root)]);
    let mut selected = BTreeSet::new();
    let mut parsed_modules = Vec::new();

    while let Some((identity, module)) = pending.pop_first() {
        if !selected.insert(identity) {
            continue;
        }
        let source = fs::read_to_string(&module.path).map_err(|error| ProjectError::SourceIo {
            path: module.path.clone(),
            error,
        })?;
        let parsed = parse_module(
            module.package,
            module.direct_dependencies,
            ModuleSource::new(module.module, module.path, source),
            &warnings,
        )?;
        for (dependency, _) in parsed.module.dependencies(Target::Erlang) {
            if dependency == PRELUDE_MODULE_NAME {
                continue;
            }
            if let Some(modules) = catalog.get(&dependency) {
                for module in modules {
                    pending.insert(
                        (module.package.clone(), module.module.clone()),
                        module.clone(),
                    );
                }
            }
        }
        parsed_modules.push(parsed);
    }

    Ok(parsed_modules)
}

#[cfg(test)]
mod tests {
    use super::{ProjectError, SourceDirectory, compile_typed_project, source_paths_from};
    use crate::planner::UnsupportedFunctionReason;
    use crate::{PlanError, plan_program};
    use camino::{Utf8Path, Utf8PathBuf};
    use std::fs;
    use tempfile::{TempDir, tempdir};

    struct FailedSourceDirectoryEntry;

    impl SourceDirectory for FailedSourceDirectoryEntry {
        fn entries(
            &self,
            _directory: &Utf8Path,
        ) -> Result<Vec<std::io::Result<std::ffi::OsString>>, ProjectError> {
            Ok(vec![Err(std::io::Error::new(
                std::io::ErrorKind::PermissionDenied,
                "entry unavailable",
            ))])
        }
    }

    #[cfg(unix)]
    struct NonUtf8SourceDirectoryEntry;

    #[cfg(unix)]
    impl SourceDirectory for NonUtf8SourceDirectoryEntry {
        fn entries(
            &self,
            _directory: &Utf8Path,
        ) -> Result<Vec<std::io::Result<std::ffi::OsString>>, ProjectError> {
            use std::os::unix::ffi::OsStringExt;

            Ok(vec![Ok(std::ffi::OsString::from_vec(
                b"invalid-\xff.gleam".to_vec(),
            ))])
        }
    }

    #[test]
    fn loads_selected_hex_git_and_local_package_modules() {
        let project = tempdir().expect("temporary project should be created");
        let root = project_root(&project);
        write_file(
            &root,
            "gleam.toml",
            r#"
name = "application"
version = "1.0.0"

[dependencies]
hex_dep = ">= 1.0.0 and < 2.0.0"
git_dep = ">= 1.0.0 and < 2.0.0"
"#,
        );
        write_file(
            &root,
            "manifest.toml",
            r#"
packages = [
  { name = "hex_dep", version = "1.0.0", build_tools = ["gleam"], requirements = ["local_dep"], source = "hex", outer_checksum = "00" },
  { name = "git_dep", version = "1.0.0", build_tools = ["gleam"], requirements = [], source = "git", repo = "https://example.com/git_dep", commit = "0123456789abcdef" },
  { name = "local_dep", version = "1.0.0", build_tools = ["gleam"], requirements = [], source = "local", path = "packages/local_dep" },
]

[requirements]
"#,
        );
        write_file(
            &root,
            "src/main.gleam",
            r#"
import gleam
import hex/nested

pub fn main() {
  nested.answer()
}
"#,
        );
        write_file(
            &root,
            "build/packages/hex_dep/gleam.toml",
            r#"
name = "hex_dep"
version = "1.0.0"

[dependencies]
local_dep = { path = "../../../packages/local_dep" }
"#,
        );
        write_file(
            &root,
            "build/packages/hex_dep/src/hex/nested.gleam",
            r#"
import local/value

pub fn answer() {
  value.answer()
}
"#,
        );
        write_file(
            &root,
            "build/packages/git_dep/gleam.toml",
            r#"
name = "git_dep"
version = "1.0.0"

[dependencies]
local_dep = { path = "../../../packages/local_dep" }
"#,
        );
        write_file(
            &root,
            "build/packages/git_dep/src/git/unselected.gleam",
            "pub fn broken(",
        );
        write_file(
            &root,
            "packages/local_dep/gleam.toml",
            r#"
name = "local_dep"
version = "1.0.0"
"#,
        );
        write_file(
            &root,
            "packages/local_dep/src/local/value.gleam",
            "pub fn answer() { 42 }",
        );
        write_file(&root, "src/README.md", "not a Gleam module");

        let program =
            compile_typed_project(root, "main").expect("resolved package project should compile");

        assert_eq!(program.root_package(), "application");
        assert_eq!(program.root_module(), "main");
        assert_eq!(
            program
                .modules()
                .map(|module| (module.type_info.package.as_str(), module.name.as_str()))
                .collect::<Vec<_>>(),
            [
                ("local_dep", "local/value"),
                ("hex_dep", "hex/nested"),
                ("application", "main"),
            ],
        );
    }

    #[test]
    fn excludes_root_dev_dependencies_from_the_resolved_program() {
        let project = tempdir().expect("temporary project should be created");
        let root = project_root(&project);
        write_file(
            &root,
            "gleam.toml",
            r#"
name = "application"
version = "1.0.0"

[dev-dependencies]
dev_only = ">= 1.0.0 and < 2.0.0"
"#,
        );
        write_file(
            &root,
            "manifest.toml",
            r#"
packages = [
  { name = "dev_only", version = "1.0.0", build_tools = ["gleam"], requirements = [], source = "hex", outer_checksum = "00" },
]

[requirements]
"#,
        );
        write_file(&root, "src/main.gleam", "pub fn main() { 1 }");

        let program =
            compile_typed_project(root, "main").expect("development dependency should be ignored");

        assert_eq!(
            program
                .modules()
                .map(|module| module.name.as_str())
                .collect::<Vec<_>>(),
            ["main"],
        );
    }

    #[test]
    fn skips_non_gleam_resolved_packages() {
        let project = tempdir().expect("temporary project should be created");
        let root = project_root(&project);
        write_file(
            &root,
            "gleam.toml",
            r#"
name = "application"
version = "1.0.0"

[dependencies]
build_only = ">= 1.0.0 and < 2.0.0"
"#,
        );
        write_file(
            &root,
            "manifest.toml",
            r#"
packages = [
  { name = "build_only", version = "1.0.0", build_tools = ["rebar3"], requirements = [], source = "hex", outer_checksum = "00" },
]

[requirements]
"#,
        );
        write_file(&root, "src/main.gleam", "pub fn main() { 1 }");

        let program =
            compile_typed_project(root, "main").expect("non-Gleam package should be skipped");

        assert_eq!(
            program
                .modules()
                .map(|module| module.name.as_str())
                .collect::<Vec<_>>(),
            ["main"],
        );
    }

    #[test]
    fn ignores_unselected_modules_but_plans_every_selected_body() {
        let project = tempdir().expect("temporary project should be created");
        let root = project_root(&project);
        write_file(
            &root,
            "gleam.toml",
            r#"
name = "application"
version = "1.0.0"

[dependencies]
library = ">= 1.0.0 and < 2.0.0"
"#,
        );
        write_file(
            &root,
            "manifest.toml",
            r#"
packages = [
  { name = "library", version = "1.0.0", build_tools = ["gleam"], requirements = [], source = "hex", outer_checksum = "00" },
]

[requirements]
"#,
        );
        write_file(
            &root,
            "src/main.gleam",
            "import selected\npub fn main() { selected.value() }",
        );
        write_file(
            &root,
            "build/packages/library/gleam.toml",
            "name = \"library\"\nversion = \"1.0.0\"\n",
        );
        write_file(
            &root,
            "build/packages/library/src/selected.gleam",
            r#"
pub fn value() {
  1
}

fn unsupported_unused_body() {
  <<1:native>>
}
"#,
        );
        write_file(
            &root,
            "build/packages/library/src/unselected.gleam",
            "pub fn broken(",
        );

        let program =
            compile_typed_project(root, "main").expect("selected source closure should compile");
        assert_eq!(
            program
                .modules()
                .map(|module| module.name.as_str())
                .collect::<Vec<_>>(),
            ["selected", "main"],
        );
        assert_eq!(
            plan_program(program),
            Err(PlanError::UnsupportedBitArraySegment {
                reason: crate::planner::UnsupportedBitArraySegmentReason::NativeEndianness,
            }),
        );
    }

    #[test]
    fn preserves_dependency_source_paths_in_module_plans() {
        let project = tempdir().expect("temporary project should be created");
        let root = project_root(&project);
        write_file(
            &root,
            "gleam.toml",
            r#"
name = "application"
version = "1.0.0"

[dependencies]
library = { path = "packages/library" }
"#,
        );
        write_file(
            &root,
            "manifest.toml",
            r#"
packages = [
  { name = "library", version = "1.0.0", build_tools = ["gleam"], requirements = [], source = "local", path = "packages/library" },
]

[requirements]
"#,
        );
        write_file(
            &root,
            "src/main.gleam",
            "import support\npub fn main() { support.value() }",
        );
        write_file(
            &root,
            "packages/library/gleam.toml",
            "name = \"library\"\nversion = \"1.0.0\"\n",
        );
        write_file(
            &root,
            "packages/library/src/support.gleam",
            "pub fn value() { 42 }",
        );

        let plan = plan_program(
            compile_typed_project(root.clone(), "main")
                .expect("dependency source project should compile"),
        )
        .expect("dependency source project should plan");

        assert_eq!(
            plan.modules()
                .iter()
                .map(|module| {
                    (
                        module.package().as_str(),
                        module
                            .source_context()
                            .expect("filesystem module should preserve source context")
                            .path()
                            .as_str(),
                    )
                })
                .collect::<Vec<_>>(),
            [
                (
                    "library",
                    root.join("packages/library/src/support.gleam").as_str(),
                ),
                ("application", root.join("src/main.gleam").as_str()),
            ],
        );
    }

    #[test]
    fn keeps_selected_external_functions_at_the_existing_planner_boundary() {
        let project = tempdir().expect("temporary project should be created");
        let root = project_root(&project);
        write_file(
            &root,
            "gleam.toml",
            r#"
name = "application"
version = "1.0.0"

[dependencies]
library = { path = "packages/library" }
"#,
        );
        write_file(
            &root,
            "manifest.toml",
            r#"
packages = [
  { name = "library", version = "1.0.0", build_tools = ["gleam"], requirements = [], source = "local", path = "packages/library" },
]

[requirements]
"#,
        );
        write_file(
            &root,
            "src/main.gleam",
            "import support\npub fn main() { support.value() }",
        );
        write_file(
            &root,
            "packages/library/gleam.toml",
            "name = \"library\"\nversion = \"1.0.0\"\n",
        );
        write_file(
            &root,
            "packages/library/src/support.gleam",
            r#"
@external(erlang, "support", "value")
pub fn value() -> Int
"#,
        );

        let program =
            compile_typed_project(root, "main").expect("external source should type-check");

        assert_eq!(
            plan_program(program),
            Err(PlanError::UnsupportedFunction {
                name: "value".into(),
                reason: UnsupportedFunctionReason::External,
            }),
        );
    }

    #[test]
    fn resolves_absolute_local_package_paths() {
        let project = tempdir().expect("temporary project should be created");
        let dependency = tempdir().expect("temporary dependency should be created");
        let root = project_root(&project);
        let dependency_root = project_root(&dependency);
        write_file(
            &root,
            "gleam.toml",
            r#"
name = "application"
version = "1.0.0"

[dependencies]
library = { path = "../library" }
"#,
        );
        write_file(
            &root,
            "manifest.toml",
            &format!(
                r#"
packages = [
  {{ name = "library", version = "1.0.0", build_tools = ["gleam"], requirements = [], source = "local", path = "{dependency_root}" }},
]

[requirements]
"#,
            ),
        );
        write_file(
            &root,
            "src/main.gleam",
            "import support\npub fn main() { support.value() }",
        );
        write_file(
            &dependency_root,
            "gleam.toml",
            "name = \"library\"\nversion = \"1.0.0\"\n",
        );
        write_file(
            &dependency_root,
            "src/support.gleam",
            "pub fn value() { 42 }",
        );

        let program =
            compile_typed_project(root, "main").expect("absolute local package should compile");

        assert_eq!(
            program
                .modules()
                .map(|module| (module.type_info.package.as_str(), module.name.as_str()))
                .collect::<Vec<_>>(),
            [("library", "support"), ("application", "main")],
        );
    }

    #[test]
    fn rejects_missing_package_config() {
        let project = tempdir().expect("temporary project should be created");
        let root = project_root(&project);

        let error = compile_typed_project(root.clone(), "main")
            .expect_err("missing package config should fail");

        assert_eq!(
            error.to_string(),
            format!(
                "failed to read Gleam package config {}",
                root.join("gleam.toml"),
            ),
        );
    }

    #[test]
    fn rejects_invalid_package_config() {
        let project = tempdir().expect("temporary project should be created");
        let root = project_root(&project);
        write_file(&root, "gleam.toml", "name = 1");

        let error = compile_typed_project(root.clone(), "main")
            .expect_err("invalid package config should fail");

        assert_eq!(
            error.to_string(),
            format!(
                "invalid Gleam package config {}: TOML parse error at line 1, column 8\n  |\n1 | name = 1\n  |        ^\ninvalid type: integer `1`, expected a package name\n",
                root.join("gleam.toml"),
            ),
        );
    }

    #[test]
    fn rejects_incompatible_embedded_compiler_versions() {
        let project = tempdir().expect("temporary project should be created");
        let root = project_root(&project);
        write_file(
            &root,
            "gleam.toml",
            r#"
name = "application"
version = "1.0.0"
gleam = ">= 99.0.0"
"#,
        );

        let error = compile_typed_project(root, "main")
            .expect_err("incompatible Gleam compiler requirement should fail");

        assert_eq!(
            error.to_string(),
            "package application is incompatible with the embedded Gleam compiler: The package application requires a Gleam version satisfying >=99.0.0 and you are using v1.17.0",
        );
    }

    #[test]
    fn rejects_missing_manifests() {
        let project = tempdir().expect("temporary project should be created");
        let root = project_root(&project);
        write_file(
            &root,
            "gleam.toml",
            "name = \"application\"\nversion = \"1.0.0\"\n",
        );

        let error =
            compile_typed_project(root.clone(), "main").expect_err("missing manifest should fail");

        assert_eq!(
            error.to_string(),
            format!(
                "failed to read Gleam package manifest {}",
                root.join("manifest.toml"),
            ),
        );
    }

    #[test]
    fn rejects_invalid_manifests() {
        let project = tempdir().expect("temporary project should be created");
        let root = project_root(&project);
        write_file(
            &root,
            "gleam.toml",
            "name = \"application\"\nversion = \"1.0.0\"\n",
        );
        write_file(&root, "manifest.toml", "packages = 1");

        let error =
            compile_typed_project(root.clone(), "main").expect_err("invalid manifest should fail");

        assert_eq!(
            error.to_string(),
            format!(
                "invalid Gleam package manifest {}: TOML parse error at line 1, column 12\n  |\n1 | packages = 1\n  |            ^\ninvalid type: integer `1`, expected a sequence\n",
                root.join("manifest.toml"),
            ),
        );
    }

    #[test]
    fn rejects_duplicate_manifest_packages() {
        let project = tempdir().expect("temporary project should be created");
        let root = project_root(&project);
        write_file(
            &root,
            "gleam.toml",
            "name = \"application\"\nversion = \"1.0.0\"\n",
        );
        write_file(
            &root,
            "manifest.toml",
            r#"
packages = [
  { name = "library", version = "1.0.0", build_tools = ["gleam"], requirements = [], source = "hex", outer_checksum = "00" },
  { name = "library", version = "1.0.1", build_tools = ["gleam"], requirements = [], source = "hex", outer_checksum = "00" },
]

[requirements]
"#,
        );

        let error = compile_typed_project(root.clone(), "main")
            .expect_err("duplicate manifest package should fail");

        assert_eq!(
            error.to_string(),
            format!(
                "invalid Gleam package manifest {}: package library is listed more than once",
                root.join("manifest.toml"),
            ),
        );
    }

    #[test]
    fn rejects_dependencies_absent_from_the_manifest() {
        let project = tempdir().expect("temporary project should be created");
        let root = project_root(&project);
        write_file(
            &root,
            "gleam.toml",
            r#"
name = "application"
version = "1.0.0"

[dependencies]
library = ">= 1.0.0 and < 2.0.0"
"#,
        );
        write_file(&root, "manifest.toml", "packages = []\n\n[requirements]\n");

        let error = compile_typed_project(root, "main")
            .expect_err("dependency absent from manifest should fail");

        assert_eq!(
            error.to_string(),
            "package application requires library, but it is absent from manifest.toml",
        );
    }

    #[test]
    fn rejects_undownloaded_resolved_packages() {
        let project = tempdir().expect("temporary project should be created");
        let root = project_root(&project);
        write_file(
            &root,
            "gleam.toml",
            r#"
name = "application"
version = "1.0.0"

[dependencies]
library = ">= 1.0.0 and < 2.0.0"
"#,
        );
        write_file(
            &root,
            "manifest.toml",
            r#"
packages = [
  { name = "library", version = "1.0.0", build_tools = ["gleam"], requirements = [], source = "hex", outer_checksum = "00" },
]

[requirements]
"#,
        );

        let error = compile_typed_project(root.clone(), "main")
            .expect_err("undownloaded package should fail");

        assert_eq!(
            error.to_string(),
            format!(
                "resolved package library is not downloaded at {}; run `gleam deps download` first",
                root.join("build/packages/library"),
            ),
        );
    }

    #[test]
    fn rejects_missing_dependency_configs() {
        let project = tempdir().expect("temporary project should be created");
        let root = project_root(&project);
        write_file(
            &root,
            "gleam.toml",
            r#"
name = "application"
version = "1.0.0"

[dependencies]
library = { path = "packages/library" }
"#,
        );
        write_file(
            &root,
            "manifest.toml",
            r#"
packages = [
  { name = "library", version = "1.0.0", build_tools = ["gleam"], requirements = [], source = "local", path = "packages/library" },
]

[requirements]
"#,
        );

        let error = compile_typed_project(root.clone(), "main")
            .expect_err("missing dependency config should fail");

        assert_eq!(
            error.to_string(),
            format!(
                "failed to read Gleam package config {}",
                root.join("packages/library/gleam.toml"),
            ),
        );
    }

    #[test]
    fn rejects_dependency_package_name_mismatches() {
        let project = tempdir().expect("temporary project should be created");
        let root = project_root(&project);
        write_file(
            &root,
            "gleam.toml",
            r#"
name = "application"
version = "1.0.0"

[dependencies]
library = ">= 1.0.0 and < 2.0.0"
"#,
        );
        write_file(
            &root,
            "manifest.toml",
            r#"
packages = [
  { name = "library", version = "1.0.0", build_tools = ["gleam"], requirements = [], source = "hex", outer_checksum = "00" },
]

[requirements]
"#,
        );
        write_file(
            &root,
            "build/packages/library/gleam.toml",
            "name = \"different\"\nversion = \"1.0.0\"\n",
        );

        let error = compile_typed_project(root.clone(), "main")
            .expect_err("dependency package name mismatch should fail");

        assert_eq!(
            error.to_string(),
            format!(
                "manifest package library resolves to a package config named different at {}",
                root.join("build/packages/library/gleam.toml"),
            ),
        );
    }

    #[test]
    fn rejects_missing_source_directories() {
        let project = tempdir().expect("temporary project should be created");
        let root = project_root(&project);
        write_file(
            &root,
            "gleam.toml",
            "name = \"application\"\nversion = \"1.0.0\"\n",
        );
        write_file(&root, "manifest.toml", "packages = []\n\n[requirements]\n");

        let error =
            compile_typed_project(root.clone(), "main").expect_err("missing source should fail");

        assert_eq!(
            error.to_string(),
            format!("failed to read Gleam source {}", root.join("src")),
        );
    }

    #[test]
    fn preserves_frontend_errors_for_missing_root_modules() {
        let project = tempdir().expect("temporary project should be created");
        let root = project_root(&project);
        write_file(
            &root,
            "gleam.toml",
            "name = \"application\"\nversion = \"1.0.0\"\n",
        );
        write_file(&root, "manifest.toml", "packages = []\n\n[requirements]\n");
        write_file(&root, "src/other.gleam", "pub fn value() { 1 }");

        let error =
            compile_typed_project(root, "main").expect_err("missing root module should fail");

        assert_eq!(
            error.to_string(),
            "root module main was not supplied by package application",
        );
    }

    #[test]
    fn preserves_selected_dependency_parse_paths() {
        let project = tempdir().expect("temporary project should be created");
        let root = project_root(&project);
        write_file(
            &root,
            "gleam.toml",
            r#"
name = "application"
version = "1.0.0"

[dependencies]
library = { path = "packages/library" }
"#,
        );
        write_file(
            &root,
            "manifest.toml",
            r#"
packages = [
  { name = "library", version = "1.0.0", build_tools = ["gleam"], requirements = [], source = "local", path = "packages/library" },
]

[requirements]
"#,
        );
        write_file(
            &root,
            "src/main.gleam",
            "import support\npub fn main() { 1 }",
        );
        write_file(
            &root,
            "packages/library/gleam.toml",
            "name = \"library\"\nversion = \"1.0.0\"\n",
        );
        write_file(
            &root,
            "packages/library/src/support.gleam",
            "pub fn broken(",
        );

        let error = compile_typed_project(root.clone(), "main")
            .expect_err("selected dependency parse failure should be preserved");

        assert_eq!(
            error.to_string(),
            format!(
                "failed to parse Gleam module {}",
                root.join("packages/library/src/support.gleam"),
            ),
        );
    }

    #[test]
    fn preserves_selected_module_import_cycles() {
        let project = tempdir().expect("temporary project should be created");
        let root = project_root(&project);
        write_file(
            &root,
            "gleam.toml",
            "name = \"application\"\nversion = \"1.0.0\"\n",
        );
        write_file(&root, "manifest.toml", "packages = []\n\n[requirements]\n");
        write_file(
            &root,
            "src/main.gleam",
            "import support\npub fn main() { support.value() }",
        );
        write_file(
            &root,
            "src/support.gleam",
            "import main\npub fn value() { main.main() }",
        );

        let error =
            compile_typed_project(root, "main").expect_err("module import cycle should fail");

        assert_eq!(
            format!("{error:?}"),
            "Frontend(ImportCycle { modules: [\"main\", \"support\", \"main\"] })",
        );
    }

    #[test]
    fn rejects_selected_duplicate_module_owners() {
        let project = tempdir().expect("temporary project should be created");
        let root = project_root(&project);
        write_file(
            &root,
            "gleam.toml",
            r#"
name = "application"
version = "1.0.0"

[dependencies]
first = { path = "packages/first" }
second = { path = "packages/second" }
"#,
        );
        write_file(
            &root,
            "manifest.toml",
            r#"
packages = [
  { name = "first", version = "1.0.0", build_tools = ["gleam"], requirements = [], source = "local", path = "packages/first" },
  { name = "second", version = "1.0.0", build_tools = ["gleam"], requirements = [], source = "local", path = "packages/second" },
]

[requirements]
"#,
        );
        write_file(
            &root,
            "src/main.gleam",
            "import shared\npub fn main() { shared.value() }",
        );
        write_file(
            &root,
            "packages/first/gleam.toml",
            "name = \"first\"\nversion = \"1.0.0\"\n",
        );
        write_file(
            &root,
            "packages/first/src/shared.gleam",
            "pub fn value() { 1 }",
        );
        write_file(
            &root,
            "packages/second/gleam.toml",
            "name = \"second\"\nversion = \"1.0.0\"\n",
        );
        write_file(
            &root,
            "packages/second/src/shared.gleam",
            "pub fn value() { 2 }",
        );

        let error = compile_typed_project(root.clone(), "main")
            .expect_err("duplicate selected module owners should fail");

        assert_eq!(
            format!("{error:?}"),
            format!(
                "Frontend(DuplicateModule {{ module: \"shared\", first_package: \"first\", first_path: \"{}\", second_package: \"second\", second_path: \"{}\" }})",
                root.join("packages/first/src/shared.gleam"),
                root.join("packages/second/src/shared.gleam"),
            ),
        );
    }

    #[test]
    fn leaves_unknown_imports_to_gleam_analysis() {
        let project = tempdir().expect("temporary project should be created");
        let root = project_root(&project);
        write_file(
            &root,
            "gleam.toml",
            "name = \"application\"\nversion = \"1.0.0\"\n",
        );
        write_file(&root, "manifest.toml", "packages = []\n\n[requirements]\n");
        write_file(
            &root,
            "src/main.gleam",
            "import unknown\npub fn main() { 1 }",
        );

        let error =
            compile_typed_project(root, "main").expect_err("unknown import should fail analysis");

        assert_eq!(error.to_string(), "failed to analyse Gleam module");
    }

    #[cfg(unix)]
    #[test]
    fn rejects_non_utf8_source_paths() {
        let error = source_paths_from(
            &NonUtf8SourceDirectoryEntry,
            Utf8Path::new("src"),
            Utf8Path::new(""),
        )
        .expect_err("non-UTF-8 path should fail");

        assert_eq!(error.to_string(), "failed to read Gleam source src");
    }

    #[test]
    fn rejects_failed_source_directory_entries() {
        let error = source_paths_from(
            &FailedSourceDirectoryEntry,
            Utf8Path::new("src"),
            Utf8Path::new(""),
        )
        .expect_err("failed directory entry should fail");

        assert_eq!(error.to_string(), "failed to read Gleam source src");
    }

    #[cfg(unix)]
    #[test]
    fn rejects_unreadable_selected_source_files() {
        use std::os::unix::fs::PermissionsExt;

        let project = tempdir().expect("temporary project should be created");
        let root = project_root(&project);
        write_file(
            &root,
            "gleam.toml",
            "name = \"application\"\nversion = \"1.0.0\"\n",
        );
        write_file(&root, "manifest.toml", "packages = []\n\n[requirements]\n");
        write_file(&root, "src/main.gleam", "pub fn main() { 1 }");
        let source_path = root.join("src/main.gleam");
        let mut permissions = fs::metadata(&source_path)
            .expect("source metadata should be readable")
            .permissions();
        permissions.set_mode(0o000);
        fs::set_permissions(&source_path, permissions)
            .expect("source permissions should be changed");

        let error = compile_typed_project(root, "main")
            .expect_err("unreadable selected source should fail");

        let mut permissions = fs::metadata(&source_path)
            .expect("source metadata should remain readable")
            .permissions();
        permissions.set_mode(0o600);
        fs::set_permissions(&source_path, permissions)
            .expect("source permissions should be restored");
        assert_eq!(
            error.to_string(),
            format!("failed to read Gleam source {source_path}"),
        );
    }

    #[cfg(unix)]
    #[test]
    fn rejects_unreadable_nested_source_directories() {
        use std::os::unix::fs::PermissionsExt;

        let project = tempdir().expect("temporary project should be created");
        let root = project_root(&project);
        write_file(
            &root,
            "gleam.toml",
            "name = \"application\"\nversion = \"1.0.0\"\n",
        );
        write_file(&root, "manifest.toml", "packages = []\n\n[requirements]\n");
        write_file(&root, "src/main.gleam", "pub fn main() { 1 }");
        write_file(&root, "src/private/hidden.gleam", "pub fn value() { 1 }");
        let private_path = root.join("src/private");
        let mut permissions = fs::metadata(&private_path)
            .expect("source directory metadata should be readable")
            .permissions();
        permissions.set_mode(0o000);
        fs::set_permissions(&private_path, permissions)
            .expect("source directory permissions should be changed");

        let error = compile_typed_project(root, "main")
            .expect_err("unreadable nested source directory should fail");

        let mut permissions = fs::metadata(&private_path)
            .expect("source directory metadata should remain readable")
            .permissions();
        permissions.set_mode(0o700);
        fs::set_permissions(&private_path, permissions)
            .expect("source directory permissions should be restored");
        assert_eq!(
            error.to_string(),
            format!("failed to read Gleam source {private_path}"),
        );
    }

    fn project_root(project: &TempDir) -> Utf8PathBuf {
        Utf8PathBuf::from_path_buf(project.path().to_path_buf())
            .expect("temporary project path should be UTF-8")
    }

    fn write_file(root: &Utf8Path, relative: &str, source: &str) {
        let path = root.join(relative);
        fs::create_dir_all(path.parent().expect("fixture file should have a parent"))
            .expect("fixture directory should be created");
        fs::write(path, source).expect("fixture file should be written");
    }
}
