use crate::error::CliError;
use cargo_metadata::Package;
use hexpm::version::{Range, Version};
use std::collections::BTreeSet;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct ProviderMetadata {
    crate_name: String,
    gleam_package: String,
    gleam_range: Range,
}

impl ProviderMetadata {
    pub(super) fn from_package(package: &Package) -> Result<Self, CliError> {
        let provider = package
            .metadata
            .as_object()
            .and_then(|metadata| metadata.get("geam"))
            .and_then(|geam| geam.as_object())
            .and_then(|geam| geam.get("provider"))
            .and_then(|provider| provider.as_object())
            .ok_or_else(|| CliError::InvalidProviderMetadata {
                package: package.name.to_string(),
                reason: "missing [package.metadata.geam.provider] table".to_owned(),
            })?;
        Self::from_fields(
            package.name.to_string(),
            provider.keys().map(String::as_str).collect(),
            provider.get("schema").and_then(|schema| schema.as_i64()),
            provider
                .get("gleam-package")
                .and_then(|package| package.as_str()),
            provider
                .get("gleam-version")
                .and_then(|range| range.as_str()),
        )
        .map_err(|reason| CliError::InvalidProviderMetadata {
            package: package.name.to_string(),
            reason,
        })
    }

    pub(super) fn from_manifest(crate_name: &str, package: &toml::Table) -> Result<Self, String> {
        let provider = package
            .get("metadata")
            .and_then(toml::Value::as_table)
            .and_then(|metadata| metadata.get("geam"))
            .and_then(toml::Value::as_table)
            .and_then(|geam| geam.get("provider"))
            .and_then(toml::Value::as_table)
            .ok_or_else(|| "missing [package.metadata.geam.provider] table".to_owned())?;
        Self::from_fields(
            crate_name.to_owned(),
            provider.keys().map(String::as_str).collect(),
            provider.get("schema").and_then(toml::Value::as_integer),
            provider.get("gleam-package").and_then(toml::Value::as_str),
            provider.get("gleam-version").and_then(toml::Value::as_str),
        )
    }

    fn from_fields(
        crate_name: String,
        fields: BTreeSet<&str>,
        schema: Option<i64>,
        gleam_package: Option<&str>,
        range: Option<&str>,
    ) -> Result<Self, String> {
        let expected = BTreeSet::from(["gleam-package", "gleam-version", "schema"]);
        if fields != expected {
            return Err(format!(
                "expected exactly schema, gleam-package, and gleam-version fields; found {}",
                fields.into_iter().collect::<Vec<_>>().join(", ")
            ));
        }
        let schema = schema.ok_or_else(|| "schema must be an integer".to_owned())?;
        if schema != 1 {
            return Err(format!("unsupported schema {schema}"));
        }
        let gleam_package = gleam_package
            .filter(|package| !package.is_empty())
            .ok_or_else(|| "gleam-package must be a non-empty string".to_owned())?
            .to_owned();
        let range = range.ok_or_else(|| "gleam-version must be a string".to_owned())?;
        let gleam_range = Range::new(range.to_owned())
            .map_err(|parse| format!("invalid Gleam version range: {parse}"))?;
        Ok(Self {
            crate_name,
            gleam_package,
            gleam_range,
        })
    }

    pub(super) fn crate_name(&self) -> &str {
        &self.crate_name
    }

    pub(super) fn gleam_package(&self) -> &str {
        &self.gleam_package
    }

    pub(super) fn gleam_range(&self) -> &Range {
        &self.gleam_range
    }

    pub(super) fn supports(&self, version: &Version) -> bool {
        self.gleam_range.to_pubgrub().contains(version)
    }
}

#[cfg(test)]
mod tests {
    use super::ProviderMetadata;
    use crate::error::CliError;
    use cargo_metadata::MetadataCommand;

    #[test]
    fn parses_exact_schema_one_metadata_and_compatibility() {
        let package = package_with_metadata(
            r#"{"geam":{"provider":{"schema":1,"gleam-package":"images","gleam-version":">= 2.4.0 and < 3.0.0"}}}"#,
        );

        let metadata =
            ProviderMetadata::from_package(&package).expect("provider metadata should be valid");

        assert_eq!(metadata.crate_name(), "provider");
        assert_eq!(metadata.gleam_package(), "images");
        assert_eq!(metadata.gleam_range().as_str(), ">= 2.4.0 and < 3.0.0");
        assert!(metadata.supports(&hexpm::version::Version::new(2, 5, 0)));
        assert!(!metadata.supports(&hexpm::version::Version::new(3, 0, 0)));
    }

    #[test]
    fn rejects_missing_malformed_and_unknown_metadata() {
        let cases = [
            ("null", "missing [package.metadata.geam.provider]"),
            (
                r#"{"geam":{"provider":{"schema":"one","gleam-package":"images","gleam-version":"1.0.0"}}}"#,
                "schema must be an integer",
            ),
            (
                r#"{"geam":{"provider":{"schema":2,"gleam-package":"images","gleam-version":"1.0.0"}}}"#,
                "unsupported schema 2",
            ),
            (
                r#"{"geam":{"provider":{"schema":1,"gleam-package":"","gleam-version":"1.0.0"}}}"#,
                "gleam-package must be a non-empty string",
            ),
            (
                r#"{"geam":{"provider":{"schema":1,"gleam-package":"images","gleam-version":1}}}"#,
                "gleam-version must be a string",
            ),
            (
                r#"{"geam":{"provider":{"schema":1,"gleam-package":"images","gleam-version":"not a range"}}}"#,
                "invalid Gleam version range",
            ),
            (
                r#"{"geam":{"provider":{"schema":1,"gleam-package":"images","gleam-version":"1.0.0","extra":true}}}"#,
                "expected exactly schema, gleam-package, and gleam-version fields",
            ),
        ];

        for (source, expected) in cases {
            let package = package_with_metadata(source);
            let error = ProviderMetadata::from_package(&package)
                .expect_err("provider metadata should be rejected");
            assert!(
                matches!(error, CliError::InvalidProviderMetadata { ref reason, .. } if reason.contains(expected)),
                "expected {expected}: {error}",
            );
        }
    }

    #[test]
    fn parses_and_rejects_packaged_toml_metadata_through_the_same_schema() {
        let package = packaged_table(
            r#"
[package]
name = "geam-images"
version = "1.2.3"

[package.metadata.geam.provider]
schema = 1
gleam-package = "images"
gleam-version = ">= 1.0.0 and < 2.0.0"
"#,
        );
        let metadata = ProviderMetadata::from_manifest("geam-images", &package)
            .expect("packaged provider metadata should parse");
        assert_eq!(metadata.crate_name(), "geam-images");
        assert_eq!(metadata.gleam_package(), "images");

        let missing_tables = [
            "[package]",
            "[package]\nmetadata = 1",
            "[package.metadata]",
            "[package.metadata]\ngeam = 1",
            "[package.metadata.geam]",
            "[package.metadata.geam]\nprovider = 1",
        ];
        for source in missing_tables {
            let package = packaged_table(source);
            assert_eq!(
                ProviderMetadata::from_manifest("geam-images", &package)
                    .expect_err("missing provider table should fail"),
                "missing [package.metadata.geam.provider] table",
            );
        }
    }

    fn package_with_metadata(metadata: &str) -> cargo_metadata::Package {
        let source = format!(
            r#"{{
  "packages": [{{
    "name": "provider",
    "version": "1.2.3",
    "id": "path+file:///provider#1.2.3",
    "license": null,
    "license_file": null,
    "description": null,
    "source": null,
    "dependencies": [],
    "targets": [],
    "features": {{}},
    "manifest_path": "/provider/Cargo.toml",
    "categories": [],
    "keywords": [],
    "readme": null,
    "repository": null,
    "homepage": null,
    "documentation": null,
    "edition": "2024",
    "metadata": {metadata},
    "links": null,
    "publish": null,
    "authors": [],
    "default_run": null,
    "rust_version": "1.96"
  }}],
  "workspace_members": ["path+file:///provider#1.2.3"],
  "workspace_default_members": ["path+file:///provider#1.2.3"],
  "resolve": null,
  "target_directory": "/target",
  "build_directory": "/target",
  "version": 1,
  "workspace_root": "/provider",
  "metadata": null
}}"#,
        );
        MetadataCommand::parse(source)
            .expect("metadata fixture should parse")
            .packages
            .pop()
            .expect("package should be present")
    }

    fn packaged_table(source: &str) -> toml::Table {
        source
            .parse::<toml::Table>()
            .expect("packaged manifest fixture should parse")
            .remove("package")
            .and_then(|package| package.as_table().cloned())
            .expect("packaged manifest fixture should contain [package]")
    }
}
