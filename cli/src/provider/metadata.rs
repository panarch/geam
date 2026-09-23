use crate::error::CliError;
use cargo_metadata::Package;
use gleam_core::config::PackageConfig;
use hexpm::version::{Range, Version};
use std::collections::BTreeSet;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ProviderMetadata {
    crate_name: String,
    gleam_package: String,
    gleam_range: Range,
    composition: ProviderComposition,
}

/// Static component shape and service dependencies supplied by a selected provider.
#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct ProviderComposition {
    pub(crate) profile_parameter: bool,
    pub(crate) execution_service: bool,
    pub(crate) required_services: BTreeSet<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct ProviderBinding {
    pub(crate) alias: String,
    pub(crate) composition: ProviderComposition,
}

impl ProviderMetadata {
    pub(super) fn from_package(package: &Package) -> Result<Self, CliError> {
        Self::from_optional_package(package)
            .map_err(|reason| CliError::InvalidProviderMetadata {
                package: package.name.to_string(),
                reason,
            })?
            .ok_or_else(|| CliError::InvalidProviderMetadata {
                package: package.name.to_string(),
                reason: "missing [package.metadata.geam.provider] table".to_owned(),
            })
    }

    pub(crate) fn from_optional_package(package: &Package) -> Result<Option<Self>, String> {
        let provider_present = package
            .metadata
            .as_object()
            .and_then(|metadata| metadata.get("geam"))
            .and_then(serde_json::Value::as_object)
            .is_some_and(|geam| geam.contains_key("provider"));
        if !provider_present {
            return Ok(None);
        }

        let provider = package
            .metadata
            .as_object()
            .and_then(|metadata| metadata.get("geam"))
            .and_then(|geam| geam.as_object())
            .and_then(|geam| geam.get("provider"))
            .and_then(|provider| provider.as_object())
            .ok_or_else(|| "missing [package.metadata.geam.provider] table".to_owned())?;
        let mut metadata = Self::from_fields(
            package.name.to_string(),
            provider.keys().map(String::as_str).collect(),
            provider.get("schema").and_then(|schema| schema.as_i64()),
            provider
                .get("gleam-package")
                .and_then(|package| package.as_str()),
            provider
                .get("gleam-version")
                .and_then(|range| range.as_str()),
        )?;
        if provider.get("schema").and_then(|schema| schema.as_i64()) == Some(2) {
            metadata.composition = ProviderComposition::from_fields(provider)?;
        }
        Ok(Some(metadata))
    }

    fn from_fields(
        crate_name: String,
        fields: BTreeSet<&str>,
        schema: Option<i64>,
        gleam_package: Option<&str>,
        range: Option<&str>,
    ) -> Result<Self, String> {
        let expected = if schema == Some(2) {
            BTreeSet::from([
                "component",
                "execution-service",
                "gleam-package",
                "gleam-version",
                "requires-services",
                "schema",
            ])
        } else {
            BTreeSet::from(["gleam-package", "gleam-version", "schema"])
        };
        if fields != expected {
            if schema == Some(2) {
                return Err(format!(
                    "schema 2 requires exactly {}; found {}",
                    expected.into_iter().collect::<Vec<_>>().join(", "),
                    fields.into_iter().collect::<Vec<_>>().join(", ")
                ));
            }
            return Err(format!(
                "expected exactly schema, gleam-package, and gleam-version fields; found {}",
                fields.into_iter().collect::<Vec<_>>().join(", ")
            ));
        }
        let schema = schema.ok_or_else(|| "schema must be an integer".to_owned())?;
        if !matches!(schema, 1 | 2) {
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
            composition: ProviderComposition::default(),
        })
    }

    pub(crate) fn crate_name(&self) -> &str {
        &self.crate_name
    }

    pub(crate) fn gleam_package(&self) -> &str {
        &self.gleam_package
    }

    pub(crate) fn gleam_range(&self) -> &Range {
        &self.gleam_range
    }

    pub(crate) fn supports(&self, version: &Version) -> bool {
        self.gleam_range.to_pubgrub().contains(version)
    }

    pub(crate) fn composition(&self) -> &ProviderComposition {
        &self.composition
    }

    pub(crate) fn binding(&self, alias: String) -> ProviderBinding {
        ProviderBinding {
            alias,
            composition: self.composition.clone(),
        }
    }
}

#[cfg(test)]
impl From<&str> for ProviderBinding {
    fn from(alias: &str) -> Self {
        Self {
            alias: alias.to_owned(),
            composition: ProviderComposition::default(),
        }
    }
}

impl ProviderComposition {
    fn from_fields(fields: &serde_json::Map<String, serde_json::Value>) -> Result<Self, String> {
        let profile_parameter = match fields.get("component").and_then(serde_json::Value::as_str) {
            Some("plain") => false,
            Some("profile") => true,
            _ => return Err("component must be \"plain\" or \"profile\"".to_owned()),
        };
        let execution_service = fields
            .get("execution-service")
            .and_then(serde_json::Value::as_bool)
            .ok_or_else(|| "execution-service must be a boolean".to_owned())?;
        let services = fields
            .get("requires-services")
            .and_then(serde_json::Value::as_array)
            .ok_or_else(|| {
                "requires-services must be an array of Gleam package names".to_owned()
            })?;
        let mut required_services = BTreeSet::new();
        for value in services {
            let package = value
                .as_str()
                .filter(|name| {
                    serde_json::from_value::<PackageConfig>(serde_json::json!({ "name": name }))
                        .is_ok()
                })
                .ok_or_else(|| {
                    "requires-services entries must be non-empty Gleam package names".to_owned()
                })?;
            if !required_services.insert(package.to_owned()) {
                return Err(format!("duplicate required service {package}"));
            }
        }
        Ok(Self {
            profile_parameter,
            execution_service,
            required_services,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::{ProviderBinding, ProviderComposition, ProviderMetadata};
    use crate::error::CliError;
    use cargo_metadata::MetadataCommand;
    use std::collections::BTreeSet;

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
        assert_eq!(metadata.composition(), &ProviderComposition::default());
        assert_eq!(
            metadata.binding("pictures".to_owned()),
            ProviderBinding::from("pictures")
        );
    }

    #[test]
    fn parses_exact_schema_two_component_and_service_contracts() {
        for (component, profile_parameter, execution_service, required) in [
            ("plain", false, false, Vec::new()),
            ("profile", true, true, vec!["tickets", "gleam_erlang"]),
        ] {
            let package = package_with_metadata(&serde_json::json!({
                "geam": { "provider": {
                    "schema": 2, "gleam-package": "images", "gleam-version": ">= 1.0.0 and < 2.0.0",
                    "component": component, "execution-service": execution_service,
                    "requires-services": required,
                }}
            }).to_string());
            let metadata = ProviderMetadata::from_package(&package).unwrap();
            let composition = ProviderComposition {
                profile_parameter,
                execution_service,
                required_services: required.into_iter().map(str::to_owned).collect(),
            };
            assert_eq!(metadata.crate_name(), "provider");
            assert_eq!(metadata.gleam_package(), "images");
            assert_eq!(metadata.composition(), &composition);
            assert!(metadata.supports(&hexpm::version::Version::new(1, 7, 0)));
            assert!(!metadata.supports(&hexpm::version::Version::new(2, 0, 0)));
            assert_eq!(
                metadata.binding("renamed".to_owned()),
                ProviderBinding {
                    alias: "renamed".to_owned(),
                    composition,
                }
            );
        }
    }

    #[test]
    fn rejects_malformed_service_declarations_at_the_metadata_owner() {
        let base = serde_json::json!({
            "schema": 2, "gleam-package": "images", "gleam-version": "1.0.0",
            "component": "profile", "execution-service": true,
            "requires-services": ["gleam_erlang"],
        });
        let mut cases = vec![
            (
                "component",
                serde_json::json!("generic"),
                "component must be \"plain\" or \"profile\"",
            ),
            (
                "component",
                serde_json::Value::Null,
                "component must be \"plain\" or \"profile\"",
            ),
            (
                "execution-service",
                serde_json::json!("true"),
                "execution-service must be a boolean",
            ),
            (
                "requires-services",
                serde_json::json!("gleam_erlang"),
                "requires-services must be an array of Gleam package names",
            ),
            (
                "requires-services",
                serde_json::json!(["gleam_erlang", "gleam_erlang"]),
                "duplicate required service gleam_erlang",
            ),
        ];
        for value in [
            serde_json::json!(1),
            serde_json::json!(""),
            serde_json::json!("1service"),
            serde_json::json!("_service"),
            serde_json::json!("Service"),
            serde_json::json!("some-service"),
        ] {
            cases.push((
                "requires-services",
                serde_json::json!([value]),
                "requires-services entries must be non-empty Gleam package names",
            ));
        }
        for (field, value, expected) in cases {
            let mut provider = base.clone();
            provider[field] = value;
            let package = package_with_metadata(
                &serde_json::json!({ "geam": { "provider": provider }}).to_string(),
            );
            assert_eq!(
                ProviderMetadata::from_optional_package(&package),
                Err(expected.to_owned()),
                "{field}"
            );
        }

        let fields = base
            .as_object()
            .unwrap()
            .keys()
            .cloned()
            .collect::<BTreeSet<_>>();
        let expected_fields = fields.iter().cloned().collect::<Vec<_>>().join(", ");
        for removed in &fields {
            if removed == "schema" {
                continue;
            }
            let mut provider = base.clone();
            provider.as_object_mut().unwrap().remove(removed);
            let found = provider
                .as_object()
                .unwrap()
                .keys()
                .cloned()
                .collect::<Vec<_>>()
                .join(", ");
            let package = package_with_metadata(
                &serde_json::json!({ "geam": { "provider": provider }}).to_string(),
            );
            assert_eq!(
                ProviderMetadata::from_optional_package(&package),
                Err(format!(
                    "schema 2 requires exactly {expected_fields}; found {found}"
                ))
            );
        }
        let mut provider = base;
        provider["extra"] = serde_json::json!(false);
        let package = package_with_metadata(
            &serde_json::json!({ "geam": { "provider": provider }}).to_string(),
        );
        assert_eq!(
            ProviderMetadata::from_optional_package(&package),
            Err(format!(
                "schema 2 requires exactly {expected_fields}; found component, execution-service, extra, gleam-package, gleam-version, requires-services, schema"
            ))
        );
    }

    #[test]
    fn rejects_missing_malformed_and_unknown_metadata() {
        let cases = [
            (
                "null",
                "missing [package.metadata.geam.provider] table",
                true,
            ),
            (
                r#"{"geam":{"provider":{"schema":"one","gleam-package":"images","gleam-version":"1.0.0"}}}"#,
                "schema must be an integer",
                true,
            ),
            (
                r#"{"geam":{"provider":{"schema":3,"gleam-package":"images","gleam-version":"1.0.0"}}}"#,
                "unsupported schema 3",
                true,
            ),
            (
                r#"{"geam":{"provider":{"schema":1,"gleam-package":"","gleam-version":"1.0.0"}}}"#,
                "gleam-package must be a non-empty string",
                true,
            ),
            (
                r#"{"geam":{"provider":{"schema":1,"gleam-package":"images","gleam-version":1}}}"#,
                "gleam-version must be a string",
                true,
            ),
            (
                r#"{"geam":{"provider":{"schema":1,"gleam-package":"images","gleam-version":"not a range"}}}"#,
                "invalid Gleam version range",
                false,
            ),
            (
                r#"{"geam":{"provider":{"schema":1,"gleam-package":"images","gleam-version":"1.0.0","extra":true}}}"#,
                "expected exactly schema, gleam-package, and gleam-version fields; found extra, gleam-package, gleam-version, schema",
                true,
            ),
        ];

        for (source, expected, exact) in cases {
            let package = package_with_metadata(source);
            let error = ProviderMetadata::from_package(&package)
                .expect_err("provider metadata should be rejected");
            assert!(
                matches!(
                    &error,
                    CliError::InvalidProviderMetadata { package, reason }
                        if package == "provider"
                            && if exact {
                                reason == expected
                            } else {
                                reason.contains(expected)
                            }
                ),
                "expected {expected}: {error}",
            );
        }
    }

    #[test]
    fn distinguishes_non_provider_packages_from_malformed_provider_metadata() {
        let unrelated = package_with_metadata(r#"{"geam":{"other":true}}"#);
        assert_eq!(
            ProviderMetadata::from_optional_package(&unrelated)
                .expect("unrelated package metadata should be ignored"),
            None,
        );

        let malformed = package_with_metadata(r#"{"geam":{"provider":true}}"#);
        assert_eq!(
            ProviderMetadata::from_optional_package(&malformed),
            Err("missing [package.metadata.geam.provider] table".to_owned()),
        );
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
}
