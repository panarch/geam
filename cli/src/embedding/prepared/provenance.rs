use super::super::package::EmbeddingPackage;
use crate::error::CliError;
use camino::Utf8Path;
use cargo_metadata::{Metadata, Package};
use gleam_core::manifest::{Manifest, ManifestPackageSource};
use gleam_core::requirement::Requirement;
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use std::fs;

pub(super) fn fingerprint(
    package: &EmbeddingPackage,
    actual: &Metadata,
) -> Result<String, CliError> {
    let mut inputs = inputs(package, actual)?;
    inputs.sort_all_objects();
    Ok(format!("sha256:{:x}", Sha256::digest(inputs.to_string())))
}

fn inputs(package: &EmbeddingPackage, actual: &Metadata) -> Result<Value, CliError> {
    let gleam: Manifest = read_toml(
        &package.project_root().join("manifest.toml"),
        "Gleam manifest",
    )?;
    let config = crate::project::read_package_config(package.project_root())?;
    let lock: toml::Value = read_toml(
        &package.environment().workspace_root.join("Cargo.lock"),
        "Cargo lock",
    )?;
    let packages = actual
        .packages
        .iter()
        .map(|package| (package.id.clone(), package))
        .collect::<BTreeMap<_, _>>();
    let resolve = actual
        .resolve
        .as_ref()
        .ok_or_else(|| super::invalid(package.cargo_package(), "helper resolve graph is absent"))?;
    let mut cargo = BTreeMap::new();
    for node in &resolve.nodes {
        let current = packages
            .get(&node.id)
            .ok_or_else(|| super::invalid(package.cargo_package(), "helper package is absent"))?;
        let mut dependencies = BTreeMap::new();
        for dependency in &node.deps {
            let target = packages.get(&dependency.pkg).ok_or_else(|| {
                super::invalid(package.cargo_package(), "helper dependency is absent")
            })?;
            let target = identity(target);
            let mut kinds = dependency
                .dep_kinds
                .iter()
                .map(|kind| json!(kind))
                .collect::<Vec<_>>();
            kinds.sort_by_cached_key(Value::to_string);
            dependencies.insert(
                (&dependency.name, target.to_string()),
                json!({
                    "name": dependency.name, "package": target, "kinds": kinds,
                }),
            );
        }
        let identity = identity(current);
        let features = node
            .features
            .iter()
            .collect::<std::collections::BTreeSet<_>>();
        cargo.insert(
            identity.to_string(),
            json!({
                "package": identity, "features": features, "dependencies": dependencies.into_values().collect::<Vec<_>>(),
            }),
        );
    }
    let packages = gleam.packages.into_iter().map(|package| {
        let source = match package.source {
            ManifestPackageSource::Local { .. } => json!({ "source": "local" }),
            source => json!(source),
        };
        (package.name.to_string(), json!({
            "version": package.version, "source": source, "requirements": package.requirements,
        }))
    }).collect::<BTreeMap<_, _>>();
    let requirements = gleam
        .requirements
        .into_iter()
        .map(|(name, requirement)| {
            let requirement = match requirement {
                Requirement::Path { .. } => json!({ "source": "local" }),
                requirement => json!(requirement),
            };
            (name.to_string(), requirement)
        })
        .collect::<BTreeMap<_, _>>();
    Ok(json!({
        "cargo": cargo.into_values().collect::<Vec<_>>(), "cargo_lock": lock,
        "gleam": { "package": config.name, "version": config.version, "packages": packages, "requirements": requirements },
    }))
}

fn identity(package: &Package) -> Value {
    json!({ "name": package.name, "version": package.version, "source": package.source })
}

fn read_toml<Value: serde::de::DeserializeOwned>(
    path: &Utf8Path,
    kind: &'static str,
) -> Result<Value, CliError> {
    let source = fs::read_to_string(path).map_err(|error| CliError::FileRead {
        path: path.to_owned(),
        error,
    })?;
    toml::from_str(&source).map_err(|error| CliError::InvalidToml {
        kind,
        path: path.to_owned(),
        reason: error.to_string(),
    })
}

#[cfg(test)]
mod tests {
    use super::{fingerprint, inputs, read_toml};
    use crate::embedding::package::EmbeddingPackage;
    use camino::Utf8PathBuf;
    use cargo_metadata::{Metadata, MetadataCommand, PackageId};
    use serde_json::json;
    use std::fs;
    use tempfile::TempDir;

    fn fixture() -> (TempDir, EmbeddingPackage, Metadata) {
        let directory = tempfile::tempdir().unwrap();
        let root = Utf8PathBuf::from_path_buf(fs::canonicalize(directory.path()).unwrap()).unwrap();
        for path in ["src", "runtime/src", "gleam/src"] {
            fs::create_dir_all(root.join(path)).unwrap();
        }
        fs::write(root.join("Cargo.toml"), "[package]\nname = 'provenance'\nversion = '1.0.0'\nedition = '2024'\n[dependencies]\nruntime = { package = 'geam', path = 'runtime', features = ['embedding'] }\n[workspace]\n").unwrap();
        fs::write(root.join("src/main.rs"), "fn main() {}\n").unwrap();
        fs::write(
            root.join("runtime/Cargo.toml"),
            "[package]\nname = 'geam'\nversion = '1.0.0'\n[features]\nembedding = []\n",
        )
        .unwrap();
        fs::write(root.join("runtime/src/lib.rs"), "").unwrap();
        fs::write(
            root.join("gleam/gleam.toml"),
            "name = 'provenance'\nversion = '1.0.0'\n",
        )
        .unwrap();
        fs::write(root.join("gleam/manifest.toml"), "packages = [{ name = 'local_data', version = '0.1.0', build_tools = ['gleam'], requirements = [], source = 'local', path = '/temporary/package' }]\n[requirements]\nlocal_data = { path = '/temporary/package' }\n").unwrap();
        let metadata = MetadataCommand::new()
            .manifest_path(root.join("Cargo.toml"))
            .exec()
            .unwrap();
        let package = EmbeddingPackage::load(&root).unwrap();
        (directory, package, metadata)
    }

    #[test]
    fn records_exact_inputs_without_checkout_or_helper_identity() {
        let (_directory, package, mut metadata) = fixture();
        let lock: toml::Value = read_toml(
            &package.environment().workspace_root.join("Cargo.lock"),
            "Cargo lock",
        )
        .unwrap();
        let expected = json!({
            "cargo": [
                { "package": { "name": "geam", "version": "1.0.0", "source": null }, "features": ["embedding"], "dependencies": [] },
                { "package": { "name": "provenance", "version": "1.0.0", "source": null }, "features": [], "dependencies": [
                    { "name": "runtime", "package": { "name": "geam", "version": "1.0.0", "source": null }, "kinds": [{ "kind": "normal", "target": null }] }
                ] }
            ],
            "cargo_lock": lock,
            "gleam": { "package": "provenance", "version": "1.0.0", "packages": { "local_data": { "version": "0.1.0", "requirements": [], "source": { "source": "local" } } }, "requirements": { "local_data": { "source": "local" } } }
        });
        assert_eq!(inputs(&package, &metadata).unwrap(), expected);
        let expected_fingerprint =
            "sha256:8c495a8671ca6847452e255c0770e5587c7c4832130c2a8ac4573416108d8ed8";
        assert_eq!(
            fingerprint(&package, &metadata).unwrap(),
            expected_fingerprint
        );
        metadata.packages.reverse();
        metadata.resolve.as_mut().unwrap().nodes.reverse();
        assert_eq!(inputs(&package, &metadata).unwrap(), expected);
        assert_eq!(
            fingerprint(&package, &metadata).unwrap(),
            expected_fingerprint
        );
        let (_relocated_directory, relocated, relocated_metadata) = fixture();
        assert_eq!(
            fingerprint(&relocated, &relocated_metadata).unwrap(),
            expected_fingerprint
        );

        let original = metadata
            .packages
            .iter()
            .find(|package| package.name == "geam")
            .unwrap()
            .clone();
        let mut second = original.clone();
        second.version = "2.0.0".parse().unwrap();
        second.id = PackageId {
            repr: "path+file:///different#geam@2.0.0".into(),
        };
        metadata.packages.push(second.clone());
        let resolve = metadata.resolve.as_mut().unwrap();
        let mut node = resolve
            .nodes
            .iter()
            .find(|node| node.id == original.id)
            .unwrap()
            .clone();
        node.id = second.id.clone();
        node.features = serde_json::from_value(json!(["z", "a"])).unwrap();
        resolve.nodes.push(node);
        let root = resolve
            .nodes
            .iter_mut()
            .find(|node| node.id == package.cargo_package().id)
            .unwrap();
        let mut alternate = root.deps[0].clone();
        alternate.pkg = second.id;
        alternate.dep_kinds[0].target = Some("cfg(windows)".parse().unwrap());
        root.deps.push(alternate);
        let generated = inputs(&package, &metadata).unwrap();
        let dependencies = generated["cargo"][2]["dependencies"].as_array().unwrap();
        assert_eq!(dependencies.len(), 2);
        assert_eq!(dependencies[0]["name"], "runtime");
        assert_eq!(dependencies[0]["package"]["version"], "1.0.0");
        assert_eq!(dependencies[1]["name"], "runtime");
        assert_eq!(dependencies[1]["package"]["version"], "2.0.0");
        assert_eq!(generated["cargo"][1]["features"], json!(["a", "z"]));
        assert_ne!(
            fingerprint(&package, &metadata).unwrap(),
            expected_fingerprint
        );
    }

    #[test]
    fn fingerprints_parsed_locks_and_configuration_not_their_source_text() {
        let (_directory, package, metadata) = fixture();
        let original = fingerprint(&package, &metadata).unwrap();
        let manifest = package.project_root().join("manifest.toml");
        let source = fs::read_to_string(&manifest).unwrap();
        fs::write(
            &manifest,
            format!(
                "# Relocated package\n{}",
                source.replace("/temporary/package", "/another/package")
            ),
        )
        .unwrap();
        assert_eq!(fingerprint(&package, &metadata).unwrap(), original);
        fs::write(&manifest, source.replace("0.1.0", "0.1.1")).unwrap();
        assert_ne!(fingerprint(&package, &metadata).unwrap(), original);
        fs::write(&manifest, source).unwrap();

        let lock = package.environment().workspace_root.join("Cargo.lock");
        let source = fs::read_to_string(&lock).unwrap();
        fs::write(&lock, format!("# Unchanged resolution\n{source}\n")).unwrap();
        assert_eq!(fingerprint(&package, &metadata).unwrap(), original);
        fs::write(&lock, source.replace("1.0.0", "1.0.1")).unwrap();
        assert_ne!(fingerprint(&package, &metadata).unwrap(), original);
        fs::write(&lock, source).unwrap();

        let config = package.project_root().join("gleam.toml");
        let source = fs::read_to_string(&config).unwrap();
        fs::write(&config, source.replace("1.0.0", "1.0.1")).unwrap();
        assert_ne!(fingerprint(&package, &metadata).unwrap(), original);
    }

    #[test]
    fn preserves_registry_source_identity_and_reports_incomplete_metadata() {
        let (_directory, package, metadata) = fixture();
        let path = package.project_root().join("manifest.toml");
        fs::write(&path, "packages = [{ name = 'published', version = '1.2.0', build_tools = ['gleam'], requirements = [], source = 'hex', outer_checksum = 'ABC123' }]\n[requirements]\npublished = { version = '>= 1.0.0 and < 2.0.0' }\n").unwrap();
        let generated = inputs(&package, &metadata).unwrap();
        assert_eq!(
            generated["gleam"]["packages"]["published"]["source"],
            json!({ "source": "hex", "outer_checksum": "ABC123" })
        );
        assert_eq!(
            generated["gleam"]["requirements"]["published"],
            json!({ "version": ">= 1.0.0 and < 2.0.0" })
        );
        let mut changed = metadata.clone();
        changed.resolve = None;
        assert!(
            fingerprint(&package, &changed)
                .unwrap_err()
                .to_string()
                .contains("helper resolve graph is absent")
        );
        let mut changed = metadata.clone();
        changed.resolve.as_mut().unwrap().nodes[0].id = PackageId {
            repr: "missing".into(),
        };
        assert!(
            fingerprint(&package, &changed)
                .unwrap_err()
                .to_string()
                .contains("helper package is absent")
        );
        let mut changed = metadata.clone();
        changed
            .resolve
            .as_mut()
            .unwrap()
            .nodes
            .iter_mut()
            .find(|node| node.id == package.cargo_package().id)
            .unwrap()
            .deps[0]
            .pkg = PackageId {
            repr: "missing".into(),
        };
        assert!(
            fingerprint(&package, &changed)
                .unwrap_err()
                .to_string()
                .contains("helper dependency is absent")
        );
        fs::write(&path, "broken").unwrap();
        assert!(matches!(
            fingerprint(&package, &metadata).unwrap_err(),
            crate::error::CliError::InvalidToml {
                kind: "Gleam manifest",
                ..
            }
        ));
        fs::remove_file(&path).unwrap();
        assert!(matches!(
            fingerprint(&package, &metadata).unwrap_err(),
            crate::error::CliError::FileRead { path: actual, .. } if actual == path
        ));
        fs::write(&path, "packages = []\n[requirements]\n").unwrap();
        let config = package.project_root().join("gleam.toml");
        let original = fs::read(&config).unwrap();
        fs::remove_file(&config).unwrap();
        let error = fingerprint(&package, &metadata).unwrap_err();
        assert!(matches!(error, crate::error::CliError::FileRead { path, .. } if path == config));
        fs::write(config, original).unwrap();
        let lock = package.environment().workspace_root.join("Cargo.lock");
        fs::write(&lock, "not toml").unwrap();
        let error = fingerprint(&package, &metadata).unwrap_err();
        assert!(matches!(
            error,
            crate::error::CliError::InvalidToml {
                kind: "Cargo lock",
                ..
            }
        ));
    }
}
