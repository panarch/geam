use super::project::EmbeddingProject;
use crate::embedding::output;
use crate::error::CliError;
use crate::provider::ProviderCandidate;
use cargo_metadata::DependencyKind;
use std::fs;
use toml_edit::{Array, DocumentMut, InlineTable, Item, Table, Value};

pub(super) fn prepare_features(
    project: &EmbeddingProject,
    features: &[&str],
) -> Result<(), CliError> {
    let source = fs::read_to_string(&project.manifest).map_err(|error| CliError::FileRead {
        path: project.manifest.clone(),
        error,
    })?;
    let mut document = source
        .parse::<DocumentMut>()
        .map_err(|error| CliError::InvalidToml {
            kind: "Cargo manifest",
            path: project.manifest.clone(),
            reason: error.to_string(),
        })?;
    let declarations = project
        .dependencies
        .iter()
        .filter(|dependency| dependency.name == "geam" && dependency.kind == DependencyKind::Normal)
        .collect::<Vec<_>>();
    match declarations.as_slice() {
        [] => {
            let dependencies = document.entry("dependencies").or_insert(Item::Table(Table::new()));
            let table = dependencies.as_table_like_mut().ok_or_else(|| CliError::InvalidToml {
                kind: "Cargo dependencies",
                path: project.manifest.clone(),
                reason: "dependencies must be a table".to_owned(),
            })?;
            if table.contains_key("geam") {
                return Err(CliError::InvalidEmbeddingDependency {
                    package: project.package_name.clone(),
                    manifest: project.manifest.clone(),
                    reason: "dependency alias `geam` already refers to another package".to_owned(),
                });
            }
            let mut declaration = InlineTable::new();
            declaration.insert("version", Value::from(concat!("=", env!("CARGO_PKG_VERSION"))));
            declaration.insert("default-features", Value::from(false));
            let mut enabled = Array::new();
            for feature in features {
                enabled.push(*feature);
            }
            declaration.insert("features", Value::Array(enabled));
            table.insert("geam", Item::Value(Value::InlineTable(declaration)));
        }
        [dependency] => {
            let missing = features.iter().copied()
                .filter(|feature| !dependency.features.iter().any(|enabled| enabled == feature))
                .collect::<Vec<_>>();
            if missing.is_empty() {
                return Ok(());
            }
            let alias = dependency.rename.as_deref().unwrap_or(&dependency.name);
            let table = match &dependency.target {
                Some(target) => document.as_table_mut().get_mut("target")
                    .and_then(Item::as_table_like_mut)
                    .and_then(|targets| targets.get_mut(&target.to_string()))
                    .and_then(Item::as_table_like_mut)
                    .and_then(|target| target.get_mut("dependencies")),
                None => document.as_table_mut().get_mut("dependencies"),
            };
            let declaration = table.and_then(Item::as_table_like_mut)
                .and_then(|table| table.get_mut(alias))
                .ok_or_else(|| CliError::InvalidEmbeddingDependency {
                    package: project.package_name.clone(),
                    manifest: project.manifest.clone(),
                    reason: format!("Cargo declaration for alias `{alias}` changed during preparation"),
                })?;
            add_features(declaration, &missing).map_err(|reason| CliError::InvalidEmbeddingDependency {
                package: project.package_name.clone(),
                manifest: project.manifest.clone(),
                reason: format!("invalid dependency `{alias}`: {reason}"),
            })?;
        }
        _ => return Err(CliError::InvalidEmbeddingDependency {
            package: project.package_name.clone(),
            manifest: project.manifest.clone(),
            reason: "more than one normal Geam declaration exists; select one application dependency before running geam embedding sync".to_owned(),
        }),
    }
    output::sync(
        &project.manifest.with_file_name(""),
        &project.manifest,
        document.to_string().as_bytes(),
    )?;
    Ok(())
}

pub(super) fn add_providers(
    project: &EmbeddingProject,
    approved: &[ProviderCandidate],
) -> Result<(), CliError> {
    let source = fs::read_to_string(&project.manifest).map_err(|error| CliError::FileRead {
        path: project.manifest.clone(),
        error,
    })?;
    let mut document = source
        .parse::<DocumentMut>()
        .map_err(|error| CliError::InvalidToml {
            kind: "Cargo manifest",
            path: project.manifest.clone(),
            reason: error.to_string(),
        })?;
    let dependencies = document
        .entry("dependencies")
        .or_insert(Item::Table(Table::new()));
    let table = dependencies
        .as_table_like_mut()
        .ok_or_else(|| CliError::InvalidToml {
            kind: "Cargo dependencies",
            path: project.manifest.clone(),
            reason: "dependencies must be a table".to_owned(),
        })?;
    for candidate in approved {
        let alias = format!("geam_provider_{}", candidate.gleam_package());
        if table.contains_key(&alias)
            || project.dependencies.iter().any(|dependency| {
                dependency
                    .rename
                    .as_deref()
                    .unwrap_or(&dependency.name)
                    .replace('-', "_")
                    == alias
            })
        {
            return Err(CliError::InvalidEmbeddingProvider {
                package: candidate.gleam_package().to_owned(),
                manifest: project.manifest.clone(),
                reason: format!(
                    "dependency alias `{alias}` is already declared; no provider dependencies were added"
                ),
            });
        }
        let mut declaration = InlineTable::new();
        declaration.insert("package", Value::from(candidate.crate_name()));
        declaration.insert("version", Value::from(format!("={}", candidate.version())));
        table.insert(&alias, Item::Value(Value::InlineTable(declaration)));
    }
    output::sync(
        &project.manifest.with_file_name(""),
        &project.manifest,
        document.to_string().as_bytes(),
    )?;
    Ok(())
}

fn add_features(declaration: &mut Item, required: &[&str]) -> Result<(), &'static str> {
    if let Item::Value(Value::String(version)) = declaration {
        let decor = version.decor().clone();
        let mut table = InlineTable::new();
        table.insert("version", Value::from(version.value().as_str()));
        let mut value = Value::InlineTable(table);
        *value.decor_mut() = decor;
        *declaration = Item::Value(value);
    }
    let table = declaration
        .as_table_like_mut()
        .ok_or("expected a version string or dependency table")?;
    let features = table
        .entry("features")
        .or_insert(Item::Value(Value::Array(Array::new())));
    let features = features
        .as_array_mut()
        .ok_or("features must be an array of strings")?;
    if features.iter().any(|feature| feature.as_str().is_none()) {
        return Err("features must be an array of strings");
    }
    for feature in required {
        if !features
            .iter()
            .any(|enabled| enabled.as_str() == Some(feature))
        {
            features.push(*feature);
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{add_features, add_providers, prepare_features};
    use crate::embedding::package::EmbeddingProject;
    use crate::error::CliError;
    use crate::provider::registry::{ProviderRegistry, RegistryAccessError};
    use crate::provider::{ProviderCandidate, ProviderDiscovery, RegistryProviderDiscovery};
    use camino::Utf8PathBuf;
    use flate2::{Compression, write::GzEncoder};
    use sha2::{Digest, Sha256};
    use std::fs;
    use tempfile::{TempDir, tempdir};
    use toml_edit::DocumentMut;

    #[test]
    fn preserves_source_alias_comments_and_existing_features_in_each_cargo_syntax() {
        for (source, expected) in [
            (
                "[dependencies]\n# keep alias\nruntime = '=0.2.1' # pinned\nother = '1'\n",
                "[dependencies]\n# keep alias\nruntime = { version = \"=0.2.1\", features = [\"embedding\", \"gleam-json\"] } # pinned\nother = '1'\n",
            ),
            (
                "[dependencies]\nruntime = { package = 'geam', path = '../geam', default-features = false, features = ['provider'] } # local\n",
                "[dependencies]\nruntime = { package = 'geam', path = '../geam', default-features = false, features = ['provider', \"embedding\", \"gleam-json\"] } # local\n",
            ),
            (
                "[dependencies.runtime]\npackage = 'geam'\ngit = 'https://example.test/geam'\nrev = 'fixed'\n# keep capability\nfeatures = ['embedding']\n",
                "[dependencies.runtime]\npackage = 'geam'\ngit = 'https://example.test/geam'\nrev = 'fixed'\n# keep capability\nfeatures = ['embedding', \"gleam-json\"]\n",
            ),
            (
                "[dependencies]\nruntime = { workspace = true }\n",
                "[dependencies]\nruntime = { workspace = true , features = [\"embedding\", \"gleam-json\"] }\n",
            ),
        ] {
            let mut document = source.parse::<DocumentMut>().expect("fixture must parse");
            add_features(
                &mut document["dependencies"]["runtime"],
                &["embedding", "gleam-json"],
            )
            .expect("required features should be added");
            assert_eq!(document.to_string(), expected);
            add_features(
                &mut document["dependencies"]["runtime"],
                &["embedding", "gleam-json"],
            )
            .expect("repeated feature addition should be unchanged");
            assert_eq!(document.to_string(), expected);
        }
    }

    #[test]
    fn appends_exact_approved_providers_without_rewriting_application_declarations() {
        let fixture = ManifestFixture::new(
            "# keep this selection\n[dependencies]\nengine = { package = 'geam', path = 'runtime', features = ['provider'] } # custom alias\n",
        );
        let project = EmbeddingProject::load(&fixture.root).expect("application declaration");
        add_providers(&project, &[verified_candidate("words")])
            .expect("append exact registry selection");
        assert_eq!(
            fs::read_to_string(project.manifest()).expect("updated manifest"),
            concat!(
                "[package]\nname = 'manifest_app'\nversion = '0.1.0'\n\n",
                "# keep this selection\n[dependencies]\n",
                "engine = { package = 'geam', path = 'runtime', features = ['provider'] } # custom alias\n",
                "geam_provider_words = { package = \"geam-words\", version = \"=1.2.3\" }\n",
                "\n[workspace]\n",
            )
        );

        let before = fs::read(project.manifest()).expect("approved manifest");
        let error = add_providers(
            &project,
            &[verified_candidate("numbers"), verified_candidate("words")],
        )
        .expect_err("one collision must preserve every declaration");
        assert_eq!(
            error.to_string(),
            format!(
                "invalid Rust embedding provider graph for package words at {}: dependency alias `geam_provider_words` is already declared; no provider dependencies were added",
                project.manifest()
            )
        );
        assert_eq!(
            fs::read(project.manifest()).expect("no partial insertion"),
            before
        );

        let target = ManifestFixture::new(
            "[target.'cfg(unix)'.dependencies]\ngeam-provider-words = { package = 'unrelated', version = '1', optional = true }\n",
        );
        let project = EmbeddingProject::load(&target.root).expect("target dependency");
        let before = fs::read(project.manifest()).expect("target manifest");
        let error = add_providers(&project, &[verified_candidate("words")])
            .expect_err("normalized alias already exists");
        assert_eq!(
            error.to_string(),
            format!(
                "invalid Rust embedding provider graph for package words at {}: dependency alias `geam_provider_words` is already declared; no provider dependencies were added",
                project.manifest()
            )
        );
        assert_eq!(
            fs::read(project.manifest()).expect("unchanged target declaration"),
            before
        );
    }

    #[test]
    fn preserves_provider_manifest_io_and_parse_failures() {
        let fixture = ManifestFixture::new("");
        let project = EmbeddingProject::load(&fixture.root).expect("application declaration");
        let approved = [verified_candidate("words")];
        fs::remove_file(project.manifest()).expect("external file removal");
        assert!(
            matches!(add_providers(&project, &approved), Err(CliError::FileRead { error, .. }) if error.kind() == std::io::ErrorKind::NotFound)
        );
        fs::write(project.manifest(), "[").expect("external invalid edit");
        assert!(matches!(
            add_providers(&project, &approved),
            Err(CliError::InvalidToml {
                kind: "Cargo manifest",
                ..
            })
        ));
        fs::write(project.manifest(), "dependencies = false\n")
            .expect("external invalid dependencies");
        assert_eq!(
            add_providers(&project, &approved)
                .expect_err("invalid dependencies must be preserved")
                .to_string(),
            format!(
                "invalid Cargo dependencies at {}: dependencies must be a table",
                project.manifest()
            )
        );
        assert_eq!(
            fs::read_to_string(project.manifest()).expect("preserved invalid declaration"),
            "dependencies = false\n"
        );
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            fs::write(project.manifest(), "[dependencies]\n").expect("empty dependencies");
            fs::set_permissions(&fixture.root, fs::Permissions::from_mode(0o500))
                .expect("read-only manifest directory");
            let error = add_providers(&project, &approved).expect_err("manifest write failure");
            fs::set_permissions(&fixture.root, fs::Permissions::from_mode(0o700))
                .expect("restore permissions");
            assert!(
                matches!(error, CliError::FileWrite { path, .. } if path == project.manifest())
            );
            assert_eq!(
                fs::read_to_string(project.manifest()).expect("preserved declaration"),
                "[dependencies]\n"
            );
        }
    }

    #[test]
    fn rejects_invalid_declaration_and_feature_shapes() {
        for (source, expected) in [
            (
                "runtime = false",
                "expected a version string or dependency table",
            ),
            (
                "runtime = { features = false }",
                "features must be an array of strings",
            ),
            (
                "runtime = { features = [3] }",
                "features must be an array of strings",
            ),
        ] {
            let mut document = source.parse::<DocumentMut>().expect("fixture must parse");
            assert_eq!(
                add_features(&mut document["runtime"], &["embedding"]),
                Err(expected)
            );
        }
    }

    #[test]
    fn edits_only_the_selected_workspace_members_inherited_features() {
        let directory = tempdir().expect("workspace fixture");
        let root = Utf8PathBuf::from_path_buf(directory.path().to_owned()).expect("UTF-8 fixture");
        let workspace = "[workspace]\nmembers = ['app', 'runtime']\nresolver = '3'\n\n[workspace.dependencies]\nengine = { package = 'geam', path = 'runtime', default-features = false, features = ['provider'] } # shared choice\n";
        fs::write(root.join("Cargo.toml"), workspace).expect("workspace manifest");
        for (name, manifest) in [
            (
                "runtime",
                "[package]\nname = 'geam'\nversion = '0.2.1'\n[features]\nembedding = []\nprovider = []\n",
            ),
            (
                "app",
                "[package]\nname = 'consumer'\nversion = '0.1.0'\n\n[dependencies]\nengine = { workspace = true } # member choice\n",
            ),
        ] {
            fs::create_dir_all(root.join(name).join("src")).expect("member directory");
            fs::write(root.join(name).join("Cargo.toml"), manifest).expect("member manifest");
            fs::write(root.join(name).join("src/lib.rs"), "").expect("member library");
        }
        let app = root.join("app");
        let project = EmbeddingProject::load(&app).expect("select member with inherited alias");
        prepare_features(&project, &["embedding", "provider"])
            .expect("member-local feature addition");
        assert_eq!(
            fs::read_to_string(app.join("Cargo.toml")).expect("member manifest"),
            "[package]\nname = 'consumer'\nversion = '0.1.0'\n\n[dependencies]\nengine = { workspace = true , features = [\"embedding\"] } # member choice\n"
        );
        assert_eq!(
            fs::read_to_string(root.join("Cargo.toml")).expect("workspace remains"),
            workspace
        );
        assert!(!root.join("Cargo.lock").exists());
        let project = EmbeddingProject::load(&app).expect("inspect updated member");
        prepare_features(&project, &["embedding", "provider"]).expect("already enabled features");
        assert_eq!(
            fs::read_to_string(root.join("Cargo.toml")).expect("workspace still remains"),
            workspace
        );
    }

    #[test]
    fn preserves_target_scoped_dependencies_and_refuses_alias_collisions_or_ambiguity() {
        let fixture = ManifestFixture::new(
            "[target.'cfg(unix)'.dependencies]\nengine = { package = 'geam', path = 'runtime', default-features = false }\n",
        );
        let project = EmbeddingProject::load(&fixture.root).expect("target dependency inspection");
        prepare_features(&project, &["embedding"]).expect("target-local feature addition");
        assert_eq!(
            fs::read_to_string(fixture.root.join("Cargo.toml")).expect("target manifest"),
            "[package]\nname = 'manifest_app'\nversion = '0.1.0'\n\n[target.'cfg(unix)'.dependencies]\nengine = { package = 'geam', path = 'runtime', default-features = false , features = [\"embedding\"] }\n\n[workspace]\n"
        );

        let ambiguous = ManifestFixture::new(
            "[dependencies]\nfirst = { package = 'geam', path = 'runtime' }\nsecond = { package = 'geam', path = 'runtime' }\n",
        );
        let project =
            EmbeddingProject::load(&ambiguous.root).expect("inspect ambiguous declarations");
        let original = fs::read(project.manifest()).expect("original manifest");
        assert!(
            matches!(prepare_features(&project, &["embedding"]), Err(CliError::InvalidEmbeddingDependency { reason, .. }) if reason == "more than one normal Geam declaration exists; select one application dependency before running geam embedding sync")
        );
        assert_eq!(
            fs::read(project.manifest()).expect("unchanged ambiguous manifest"),
            original
        );

        let collision = ManifestFixture::new(
            "[dependencies]\ngeam = { package = 'unrelated', version = '1' }\n",
        );
        let project = EmbeddingProject::load(&collision.root)
            .expect("inspect unrelated alias without resolution");
        let original = fs::read(project.manifest()).expect("original manifest");
        assert_eq!(
            prepare_features(&project, &["embedding"])
                .expect_err("alias collision")
                .to_string(),
            format!(
                "invalid Geam dependency for embedding package manifest_app at {}: dependency alias `geam` already refers to another package",
                project.manifest()
            )
        );
        assert_eq!(
            fs::read(project.manifest()).expect("unchanged unrelated manifest"),
            original
        );
    }

    #[test]
    fn preserves_read_parse_changed_declaration_and_write_failures_without_replacing_content() {
        let fixture = ManifestFixture::new(
            "[dependencies]\nengine = { package = 'geam', path = 'runtime' }\n",
        );
        let project = EmbeddingProject::load(&fixture.root).expect("inspect original declaration");
        let path = project.manifest();
        fs::remove_file(path).expect("external file removal");
        assert!(
            matches!(prepare_features(&project, &["embedding"]), Err(CliError::FileRead { error, .. }) if error.kind() == std::io::ErrorKind::NotFound)
        );
        fs::write(path, "[").expect("external invalid edit");
        assert!(matches!(
            prepare_features(&project, &["embedding"]),
            Err(CliError::InvalidToml {
                kind: "Cargo manifest",
                ..
            })
        ));
        for (source, reason) in [
            (
                "",
                "Cargo declaration for alias `engine` changed during preparation",
            ),
            (
                "[dependencies]\n",
                "Cargo declaration for alias `engine` changed during preparation",
            ),
            (
                "[dependencies]\nengine = false\n",
                "invalid dependency `engine`: expected a version string or dependency table",
            ),
            (
                "[dependencies]\nengine = { features = false }\n",
                "invalid dependency `engine`: features must be an array of strings",
            ),
        ] {
            fs::write(path, source).expect("external declaration edit");
            assert_eq!(
                prepare_features(&project, &["embedding"])
                    .expect_err("changed declaration must be preserved")
                    .to_string(),
                format!(
                    "invalid Geam dependency for embedding package manifest_app at {path}: {reason}"
                )
            );
            assert_eq!(
                fs::read_to_string(path).expect("failed edit preserves current content"),
                source
            );
        }
        let empty = ManifestFixture::new("");
        let project = EmbeddingProject::load(&empty.root).expect("package without Geam");
        fs::write(project.manifest(), "dependencies = false\n")
            .expect("external invalid table edit");
        assert!(matches!(
            prepare_features(&project, &["embedding"]),
            Err(CliError::InvalidToml {
                kind: "Cargo dependencies",
                ..
            })
        ));
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            fs::write(project.manifest(), "[dependencies]\n").expect("empty dependency table");
            fs::set_permissions(&empty.root, fs::Permissions::from_mode(0o500))
                .expect("read-only package directory");
            let error = prepare_features(&project, &["embedding"])
                .expect_err("manifest replacement should fail");
            fs::set_permissions(&empty.root, fs::Permissions::from_mode(0o700))
                .expect("restore directory permissions");
            assert!(
                matches!(error, CliError::FileWrite { path, .. } if path == project.manifest())
            );
            assert_eq!(
                fs::read_to_string(project.manifest()).expect("unchanged manifest"),
                "[dependencies]\n"
            );
        }
    }

    fn verified_candidate(package: &str) -> ProviderCandidate {
        let crate_name = format!("geam-{package}");
        let manifest = format!(
            "[package]\nname = '{crate_name}'\nversion = '1.2.3'\n[package.metadata.geam.provider]\nschema = 1\ngleam-package = '{package}'\ngleam-version = '>= 1.0.0'\n"
        );
        let mut archive = tar::Builder::new(GzEncoder::new(Vec::new(), Compression::default()));
        let mut header = tar::Header::new_gnu();
        header.set_mode(0o644);
        header.set_size(manifest.len() as u64);
        header.set_cksum();
        archive
            .append_data(
                &mut header,
                format!("{crate_name}-1.2.3/Cargo.toml"),
                manifest.as_bytes(),
            )
            .expect("packaged metadata");
        let archive = archive
            .into_inner()
            .expect("archive writer")
            .finish()
            .expect("gzip archive");
        let checksum = hex::encode(Sha256::digest(&archive));
        let registry = ManifestRegistry { crate_name: crate_name.clone(), archive, index: serde_json::to_vec(&serde_json::json!({"name":crate_name,"vers":"1.2.3","cksum":checksum,"yanked":false})).expect("index fixture") };
        RegistryProviderDiscovery::new(&registry)
            .discover(package, &hexpm::version::Version::new(1, 0, 0))
            .expect("verified candidate")
            .remove(0)
    }

    struct ManifestRegistry {
        crate_name: String,
        archive: Vec<u8>,
        index: Vec<u8>,
    }

    impl ProviderRegistry for ManifestRegistry {
        fn search(&self, _query: &str) -> Result<Vec<u8>, RegistryAccessError> {
            Ok(serde_json::to_vec(
                &serde_json::json!({"crates":[{"id":self.crate_name}],"meta":{"total":1}}),
            )
            .expect("search fixture"))
        }
        fn configuration(&self) -> Result<Vec<u8>, RegistryAccessError> {
            Ok(br#"{"dl":"https://fixture.invalid/{crate}/{version}/download"}"#.to_vec())
        }
        fn index(&self, _crate_name: &str) -> Result<Vec<u8>, RegistryAccessError> {
            Ok(self.index.clone())
        }
        fn download(&self, _url: &str) -> Result<Vec<u8>, RegistryAccessError> {
            Ok(self.archive.clone())
        }
    }

    struct ManifestFixture {
        _directory: TempDir,
        root: Utf8PathBuf,
    }

    impl ManifestFixture {
        fn new(dependencies: &str) -> Self {
            let directory = tempdir().expect("manifest fixture");
            let root =
                Utf8PathBuf::from_path_buf(directory.path().to_owned()).expect("UTF-8 fixture");
            fs::create_dir(root.join("src")).expect("Rust source directory");
            fs::create_dir_all(root.join("runtime/src")).expect("local Geam directory");
            fs::write(root.join("src/lib.rs"), "").expect("Rust library");
            fs::write(root.join("runtime/src/lib.rs"), "").expect("local Geam library");
            fs::write(root.join("runtime/Cargo.toml"), "[package]\nname = 'geam'\nversion = '0.2.1'\n[features]\nembedding = []\nprovider = []\n").expect("local Geam manifest");
            fs::write(root.join("Cargo.toml"), format!("[package]\nname = 'manifest_app'\nversion = '0.1.0'\n\n{dependencies}\n[workspace]\n")).expect("application manifest");
            Self {
                _directory: directory,
                root,
            }
        }
    }
}
