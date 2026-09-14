use super::super::package::EmbeddingPackage;
use crate::error::CliError;
use camino::Utf8Path;
use cargo_metadata::{DependencyKind, Package};
use std::fs;
use toml_edit::{Array, ArrayOfTables, DocumentMut, InlineTable, Item, Table, Value, value};

pub(super) fn helper_manifest(package: &EmbeddingPackage) -> Result<String, CliError> {
    let source = read_document(package.manifest())?;
    let workspace = package.environment().workspace_root.join("Cargo.toml");
    let workspace_source;
    let root = if workspace == package.manifest() {
        &source
    } else {
        workspace_source = read_document(&workspace)?;
        &workspace_source
    };
    prepare_manifest(package.cargo_package(), &source, root, &workspace)
}

fn prepare_manifest(
    package: &Package,
    source: &DocumentMut,
    root: &DocumentMut,
    workspace: &Utf8Path,
) -> Result<String, CliError> {
    let mut document = DocumentMut::new();
    let mut definition = Table::new();
    definition.insert("name", value(package.name.as_str()));
    definition.insert("version", value(package.version.to_string()));
    definition.insert("edition", value(package.edition.to_string()));
    for field in [
        "autolib",
        "autobins",
        "autoexamples",
        "autotests",
        "autobenches",
        "build",
    ] {
        definition.insert(field, value(false));
    }
    if let Some(version) = &package.rust_version {
        definition.insert("rust-version", value(version.to_string()));
    }
    document.insert("package", Item::Table(definition));
    let mut binary = Table::new();
    binary.insert("name", value("geam-prepare"));
    binary.insert("path", value("main.rs"));
    let mut binaries = ArrayOfTables::new();
    binaries.push(binary);
    document.insert("bin", Item::ArrayOfTables(binaries));

    for dependency in &package.dependencies {
        let kind = match dependency.kind {
            DependencyKind::Normal => "dependencies",
            DependencyKind::Development => "dev-dependencies",
            DependencyKind::Build => "build-dependencies",
            _ => return Err(invalid(package, "unknown Cargo dependency kind")),
        };
        let source = match &dependency.target {
            Some(target) => source
                .get("target")
                .and_then(|targets| targets.get(target.to_string()))
                .and_then(|target| target.get(kind)),
            None => source.get(kind),
        };
        let alias = dependency.rename.as_deref().unwrap_or(&dependency.name);
        let mut declaration = source
            .and_then(|source| source.get(alias))
            .cloned()
            .ok_or_else(|| {
                invalid(
                    package,
                    format!("Cargo declaration for `{alias}` changed during preparation"),
                )
            })?;
        if declaration.get("workspace").and_then(Item::as_bool) == Some(true) {
            declaration = root
                .get("workspace")
                .and_then(|workspace| workspace.get("dependencies"))
                .and_then(|dependencies| dependencies.get(alias))
                .cloned()
                .ok_or_else(|| {
                    invalid(package, format!("workspace dependency `{alias}` is absent"))
                })?;
            let table = dependency_table(&mut declaration, package, alias)?;
            let mut features = Array::new();
            for feature in &dependency.features {
                features.push(feature.as_str());
            }
            table.insert("features", value(features));
            table.insert("optional", value(dependency.optional));
            table.insert("default-features", value(dependency.uses_default_features));
        }
        if let Some(path) = &dependency.path {
            dependency_table(&mut declaration, package, alias)?
                .insert("path", value(path.as_str()));
        }
        let destination = match &dependency.target {
            Some(target) => &mut document["target"][target.to_string()][kind],
            None => &mut document[kind],
        };
        destination[alias] = declaration;
    }
    for field in ["features", "profile"] {
        if let Some(item) = (if field == "features" { source } else { root }).get(field) {
            document.insert(field, item.clone());
        }
    }
    for field in ["patch", "replace"] {
        if let Some(item) = root.get(field) {
            let mut item = item.clone();
            if field == "patch" {
                if let Some(registries) = item.as_table_like_mut() {
                    for (_, declarations) in registries.iter_mut() {
                        relocate_overrides(declarations, workspace);
                    }
                }
            } else {
                relocate_overrides(&mut item, workspace);
            }
            document.insert(field, item);
        }
    }
    let resolver = root
        .get("workspace")
        .and_then(|item| item.get("resolver"))
        .or_else(|| root.get("package").and_then(|item| item.get("resolver")))
        .and_then(Item::as_str)
        .unwrap_or_else(|| {
            match root
                .get("package")
                .and_then(|item| item.get("edition"))
                .and_then(|edition| {
                    if edition.get("workspace").and_then(Item::as_bool) == Some(true) {
                        root.get("workspace")
                            .and_then(|workspace| workspace.get("package"))
                            .and_then(|package| package.get("edition"))
                    } else {
                        Some(edition)
                    }
                })
                .and_then(Item::as_str)
            {
                Some("2024") => "3",
                Some("2021") => "2",
                _ => "1",
            }
        });
    document["workspace"]["resolver"] = value(resolver);
    Ok(document.to_string())
}

fn dependency_table<'item>(
    item: &'item mut Item,
    package: &Package,
    alias: &str,
) -> Result<&'item mut dyn toml_edit::TableLike, CliError> {
    if let Some(version) = item.as_str() {
        let mut table = InlineTable::new();
        table.insert("version", Value::from(version));
        *item = value(table);
    }
    item.as_table_like_mut()
        .ok_or_else(|| invalid(package, format!("invalid dependency `{alias}`")))
}

fn relocate_overrides(item: &mut Item, manifest: &Utf8Path) {
    if let Some(declarations) = item.as_table_like_mut() {
        for (_, declaration) in declarations.iter_mut() {
            if let Some(table) = declaration.as_table_like_mut()
                && let Some(path) = table.get("path").and_then(Item::as_str)
            {
                let path = manifest.with_file_name("").join(path);
                table.insert("path", value(path.as_str()));
            }
        }
    }
}

fn read_document(path: &Utf8Path) -> Result<DocumentMut, CliError> {
    fs::read_to_string(path)
        .map_err(|error| CliError::FileRead {
            path: path.to_owned(),
            error,
        })?
        .parse()
        .map_err(|error: toml_edit::TomlError| CliError::InvalidToml {
            kind: "Cargo manifest",
            path: path.to_owned(),
            reason: error.to_string(),
        })
}

fn invalid(package: &Package, reason: impl Into<String>) -> CliError {
    CliError::InvalidEmbeddingDependency {
        package: package.name.to_string(),
        manifest: package.manifest_path.clone(),
        reason: reason.into(),
    }
}

#[cfg(test)]
mod tests {
    use super::{prepare_manifest, read_document};
    use camino::Utf8PathBuf;
    use cargo_metadata::{Dependency, MetadataCommand, Package};
    use serde_json::json;
    use std::fs;
    use tempfile::TempDir;
    use toml_edit::DocumentMut;

    fn fixture() -> (TempDir, Package) {
        let directory = tempfile::tempdir().unwrap();
        fs::create_dir(directory.path().join("src")).unwrap();
        fs::write(
            directory.path().join("src/main.rs"),
            "compile_error!(\"application is not a helper\");",
        )
        .unwrap();
        let manifest = directory.path().join("Cargo.toml");
        fs::write(&manifest, "[package]\nname = 'manifest-fixture'\nversion = '1.2.3'\nedition = '2024'\nrust-version = '1.96'\n[workspace]\n").unwrap();
        let metadata = MetadataCommand::new()
            .manifest_path(manifest)
            .no_deps()
            .exec()
            .unwrap();
        (directory, metadata.packages.into_iter().next().unwrap())
    }

    fn dependency(alias: &str, kind: &str) -> Dependency {
        serde_json::from_value(json!({
            "name": "native", "source": null, "req": "^1", "kind": kind,
            "rename": alias, "optional": false, "uses_default_features": false,
            "features": ["base", "selected"], "target": null, "registry": null,
        }))
        .unwrap()
    }

    #[test]
    fn expands_workspace_dependencies_without_compiling_application_targets() {
        let (_directory, mut package) = fixture();
        let workspace = package.manifest_path.with_file_name("workspace/Cargo.toml");
        package.dependencies = vec![
            dependency("service", "normal"),
            dependency("tools", "build"),
            dependency("testing", "dev"),
            dependency("platform", "normal"),
        ];
        package.dependencies[0].path = Some(workspace.with_file_name("native"));
        package.dependencies[3].target = Some("cfg(unix)".parse().unwrap());
        let source: DocumentMut = "[dependencies]\nservice = { workspace = true, features = ['selected'] }\n[build-dependencies]\ntools = { package = 'native', version = '1', optional = true }\n[dev-dependencies]\ntesting = { package = 'native', version = '1' }\n[target.'cfg(unix)'.dependencies]\nplatform = { package = 'native', version = '1' }\n[features]\ndefault = ['selected']\nselected = ['service/selected']\n".parse().unwrap();
        let root: DocumentMut = "[workspace]\nresolver = '3'\n[workspace.dependencies]\nservice = { package = 'native', path = 'native', version = '1', default-features = false, features = ['base'] }\n[profile.dev]\nopt-level = 1\n[patch.crates-io]\npatched = { path = 'patches/patched' }\nremote = { git = 'https://example.invalid/native', rev = 'fixed' }\n".parse().unwrap();
        let generated: DocumentMut = prepare_manifest(&package, &source, &root, &workspace)
            .unwrap()
            .parse()
            .unwrap();
        assert_eq!(
            generated["package"]["name"].as_str(),
            Some("manifest-fixture")
        );
        assert_eq!(generated["package"]["version"].as_str(), Some("1.2.3"));
        assert_eq!(generated["package"]["edition"].as_str(), Some("2024"));
        assert_eq!(
            generated["package"]["rust-version"].as_str(),
            Some("1.96.0")
        );
        assert_eq!(generated["package"]["build"].as_bool(), Some(false));
        for target in [
            "autolib",
            "autobins",
            "autoexamples",
            "autotests",
            "autobenches",
        ] {
            assert_eq!(generated["package"][target].as_bool(), Some(false));
        }
        let binary = generated["bin"]
            .as_array_of_tables()
            .unwrap()
            .get(0)
            .unwrap();
        assert_eq!(binary["name"].as_str(), Some("geam-prepare"));
        assert_eq!(binary["path"].as_str(), Some("main.rs"));
        let service = &generated["dependencies"]["service"];
        assert!(service.get("workspace").is_none());
        assert_eq!(
            service["path"].as_str(),
            Some(workspace.with_file_name("native").as_str())
        );
        assert_eq!(
            service["features"]
                .as_array()
                .unwrap()
                .iter()
                .map(|feature| feature.as_str().unwrap())
                .collect::<Vec<_>>(),
            ["base", "selected"]
        );
        assert_eq!(service["default-features"].as_bool(), Some(false));
        assert_eq!(service["optional"].as_bool(), Some(false));
        assert_eq!(
            generated["build-dependencies"]["tools"]["optional"].as_bool(),
            Some(true)
        );
        assert_eq!(
            generated["dev-dependencies"]["testing"]["package"].as_str(),
            Some("native")
        );
        assert_eq!(
            generated["target"]["cfg(unix)"]["dependencies"]["platform"]["version"].as_str(),
            Some("1")
        );
        assert_eq!(generated["workspace"]["resolver"].as_str(), Some("3"));
        assert_eq!(
            generated["profile"]["dev"]["opt-level"].as_integer(),
            Some(1)
        );
        assert_eq!(
            generated["features"].to_string(),
            source["features"].to_string()
        );
        assert_eq!(
            generated["patch"]["crates-io"]["patched"]["path"].as_str(),
            Some(workspace.with_file_name("patches/patched").as_str())
        );
        assert_eq!(
            generated["patch"]["crates-io"]["remote"]["rev"].as_str(),
            Some("fixed")
        );
    }

    #[test]
    fn preserves_git_registry_and_replacement_declarations() {
        let (_directory, mut package) = fixture();
        package.rust_version = None;
        package.dependencies = vec![
            dependency("git_service", "normal"),
            dependency("registry_service", "normal"),
            dependency("inherited", "normal"),
        ];
        let source: DocumentMut = "[dependencies]\ngit_service = { package = 'native', git = 'https://example.invalid/native', rev = 'deadbeef', features = ['selected'] }\nregistry_service = { package = 'native', registry = 'private', version = '=1.0.0' }\ninherited.workspace = true\n".parse().unwrap();
        let root: DocumentMut = "[workspace.dependencies]\ninherited = '1'\n[replace]\n'native:1.0.0' = { path = 'replacement' }\n'other:1.0.0' = { git = 'https://example.invalid/other', branch = 'stable' }\n".parse().unwrap();
        let generated: DocumentMut =
            prepare_manifest(&package, &source, &root, &package.manifest_path)
                .unwrap()
                .parse()
                .unwrap();
        for alias in ["git_service", "registry_service"] {
            assert_eq!(
                generated["dependencies"][alias].to_string(),
                source["dependencies"][alias].to_string()
            );
        }
        assert_eq!(
            generated["dependencies"]["inherited"]["version"].as_str(),
            Some("1")
        );
        assert_eq!(
            generated["replace"]["native:1.0.0"]["path"].as_str(),
            Some(package.manifest_path.with_file_name("replacement").as_str())
        );
        assert_eq!(
            generated["replace"]["other:1.0.0"]["branch"].as_str(),
            Some("stable")
        );
        assert!(generated["package"].get("rust-version").is_none());
    }

    #[test]
    fn preserves_explicit_and_edition_inferred_workspace_resolvers() {
        let (_directory, package) = fixture();
        for (root, expected) in [
            ("", "1"),
            ("[package]\nedition = '2018'", "1"),
            ("[package]\nedition = '2021'", "2"),
            ("[package]\nedition = '2024'", "3"),
            (
                "[package]\nedition.workspace = true\n[workspace.package]\nedition = '2024'",
                "3",
            ),
            ("[package]\nedition = '2024'\nresolver = '1'", "1"),
            ("[workspace]\nresolver = '2'", "2"),
        ] {
            let root = root.parse::<DocumentMut>().unwrap();
            let generated: DocumentMut =
                prepare_manifest(&package, &DocumentMut::new(), &root, &package.manifest_path)
                    .unwrap()
                    .parse()
                    .unwrap();
            assert_eq!(generated["workspace"]["resolver"].as_str(), Some(expected));
        }
    }

    #[test]
    fn rejects_changed_or_unreadable_manifest_inputs() {
        let (directory, mut package) = fixture();
        package.dependencies = vec![dependency("service", "normal")];
        for (source, root, reason) in [
            ("", "", "Cargo declaration for `service` changed"),
            (
                "[dependencies]\nservice.workspace = true",
                "",
                "workspace dependency `service` is absent",
            ),
            (
                "[dependencies]\nservice.workspace = true",
                "[workspace.dependencies]\nservice = false",
                "invalid dependency `service`",
            ),
        ] {
            let error = prepare_manifest(
                &package,
                &source.parse().unwrap(),
                &root.parse().unwrap(),
                &package.manifest_path,
            )
            .unwrap_err();
            assert!(error.to_string().contains(reason), "{error}");
        }
        package.dependencies[0].path = Some(package.manifest_path.with_file_name("service"));
        let error = prepare_manifest(
            &package,
            &"[dependencies]\nservice = false".parse().unwrap(),
            &DocumentMut::new(),
            &package.manifest_path,
        )
        .unwrap_err();
        assert!(error.to_string().contains("invalid dependency `service`"));
        package.dependencies[0].kind = cargo_metadata::DependencyKind::Unknown;
        assert!(
            prepare_manifest(
                &package,
                &DocumentMut::new(),
                &DocumentMut::new(),
                &package.manifest_path
            )
            .unwrap_err()
            .to_string()
            .contains("unknown Cargo dependency kind")
        );
        let path = Utf8PathBuf::from_path_buf(directory.path().join("missing")).unwrap();
        assert!(matches!(
            read_document(&path).unwrap_err(),
            crate::error::CliError::FileRead { path: actual, .. } if actual == path
        ));
        fs::write(&path, "not toml").unwrap();
        assert!(matches!(
            read_document(&path).unwrap_err(),
            crate::error::CliError::InvalidToml { path: actual, kind, .. } if actual == path && kind == "Cargo manifest"
        ));
    }

    #[test]
    fn keeps_unrecognized_override_shapes_for_cargo_to_diagnose() {
        let (_directory, package) = fixture();
        let root: DocumentMut = "patch = false\nreplace = 1".parse().unwrap();
        let output: DocumentMut =
            prepare_manifest(&package, &DocumentMut::new(), &root, &package.manifest_path)
                .unwrap()
                .parse()
                .unwrap();
        assert_eq!(output["patch"].as_bool(), Some(false));
        assert_eq!(output["replace"].as_integer(), Some(1));
    }

    #[test]
    fn reports_a_workspace_manifest_removed_after_package_resolution() {
        let directory = tempfile::tempdir().unwrap();
        let root = Utf8PathBuf::from_path_buf(fs::canonicalize(directory.path()).unwrap()).unwrap();
        for path in ["app/src", "app/gleam/src", "runtime/src"] {
            fs::create_dir_all(root.join(path)).unwrap();
        }
        let manifest = root.join("Cargo.toml");
        fs::write(
            &manifest,
            "[workspace]\nmembers = ['app', 'runtime']\nresolver = '3'\n",
        )
        .unwrap();
        fs::write(root.join("app/Cargo.toml"), "[package]\nname = 'consumer'\nversion = '1.0.0'\nedition = '2024'\n[dependencies]\ngeam = { path = '../runtime', features = ['embedding'] }\n").unwrap();
        fs::write(root.join("app/src/main.rs"), "fn main() {}\n").unwrap();
        fs::write(
            root.join("app/gleam/gleam.toml"),
            "name = 'consumer'\nversion = '1.0.0'\n",
        )
        .unwrap();
        fs::write(
            root.join("runtime/Cargo.toml"),
            "[package]\nname = 'geam'\nversion = '1.0.0'\n[features]\nembedding = []\n",
        )
        .unwrap();
        fs::write(root.join("runtime/src/lib.rs"), "").unwrap();
        MetadataCommand::new()
            .manifest_path(&manifest)
            .exec()
            .unwrap();
        let package = crate::embedding::package::EmbeddingPackage::load(&root.join("app")).unwrap();
        fs::remove_file(&manifest).unwrap();
        let error = super::helper_manifest(&package).unwrap_err();
        assert!(matches!(error, crate::error::CliError::FileRead { path, .. } if path == manifest));
    }
}
