use geam_core::{HostProviderConfiguration, HostProviderConfigurationValue};
use std::collections::BTreeMap;
use std::ffi::OsStr;
use std::io;
use std::path::{Path, PathBuf};

pub struct RuntimeInputs {
    pub configurations: BTreeMap<String, HostProviderConfiguration>,
    pub resources: geam_erlang::Configuration,
}

impl RuntimeInputs {
    pub fn read(
        executable: &Path,
        current_directory: &Path,
        configuration: Option<&OsStr>,
        packages: &[&str],
        providers: &[&str],
    ) -> Result<Self, io::Error> {
        let mut resources = geam_erlang::Configuration {
            resources: packages
                .iter()
                .map(|package| {
                    (
                        (*package).into(),
                        executable.with_file_name("").join("priv").join(package),
                    )
                })
                .collect(),
        };
        let Some(configuration) = configuration else {
            return Ok(Self {
                configurations: BTreeMap::new(),
                resources,
            });
        };
        if configuration.is_empty() {
            return Err(invalid(
                "GEAM_CONFIG must name a non-empty configuration path",
            ));
        }
        let path = current_directory.join(configuration);
        let mut table = read_table(&path, "runtime configuration")?;
        let directory = path.with_file_name("");
        let provider_paths = mapped_paths(&mut table, "providers", providers, &directory, &path)?;
        let resource_paths = mapped_paths(&mut table, "resources", packages, &directory, &path)?;
        if let Some(field) = table.keys().next() {
            return Err(invalid(format!(
                "runtime configuration {} has unknown field {field}",
                path.display()
            )));
        }
        let configurations = provider_paths
            .into_iter()
            .map(|(package, path)| read_provider_configuration(&path).map(|value| (package, value)))
            .collect::<Result<_, _>>()?;
        resources.resources.extend(
            resource_paths
                .into_iter()
                .map(|(package, path)| (package.into(), path)),
        );
        Ok(Self {
            configurations,
            resources,
        })
    }
}

pub fn read_provider_configuration(path: &Path) -> Result<HostProviderConfiguration, io::Error> {
    let table = read_table(path, "provider configuration")?;
    configuration_from_table(table).map_err(|error| {
        invalid(format!(
            "invalid provider configuration {}: {error}",
            path.display()
        ))
    })
}

fn read_table(path: &Path, kind: &str) -> Result<toml::Table, io::Error> {
    let source = std::fs::read_to_string(path).map_err(|error| {
        io::Error::new(
            error.kind(),
            format!("failed to read {kind} {}: {error}", path.display()),
        )
    })?;
    toml::from_str(&source).map_err(|error: toml::de::Error| {
        invalid(format!(
            "invalid {kind} {}: {}",
            path.display(),
            error.message()
        ))
    })
}

fn mapped_paths(
    document: &mut toml::Table,
    section: &str,
    packages: &[&str],
    directory: &Path,
    configuration: &Path,
) -> Result<BTreeMap<String, PathBuf>, io::Error> {
    let Some(value) = document.remove(section) else {
        return Ok(BTreeMap::new());
    };
    let toml::Value::Table(table) = value else {
        return Err(invalid(format!(
            "runtime configuration {}: {section} must be a table",
            configuration.display()
        )));
    };
    let mut paths = BTreeMap::new();
    for (package, value) in table {
        if !packages.contains(&package.as_str()) {
            return Err(invalid(format!(
                "runtime configuration {}: unknown {section} package {package}",
                configuration.display()
            )));
        }
        let toml::Value::String(path) = value else {
            return Err(invalid(format!(
                "runtime configuration {}: {section}.{package} must be a path string",
                configuration.display()
            )));
        };
        if path.is_empty() {
            return Err(invalid(format!(
                "runtime configuration {}: {section}.{package} must be non-empty",
                configuration.display()
            )));
        }
        paths.insert(package, directory.join(path));
    }
    Ok(paths)
}

fn configuration_from_table(table: toml::Table) -> Result<HostProviderConfiguration, io::Error> {
    let values = table
        .into_iter()
        .map(|(key, value)| configuration_value(value).map(|value| (key.into(), value)))
        .collect::<Result<_, _>>()?;
    Ok(HostProviderConfiguration::new(values))
}

fn configuration_value(value: toml::Value) -> Result<HostProviderConfigurationValue, io::Error> {
    Ok(match value {
        toml::Value::String(value) => HostProviderConfigurationValue::String(value.into()),
        toml::Value::Integer(value) => HostProviderConfigurationValue::Integer(value),
        toml::Value::Float(value) => HostProviderConfigurationValue::Float(value),
        toml::Value::Boolean(value) => HostProviderConfigurationValue::Bool(value),
        toml::Value::Array(values) => HostProviderConfigurationValue::Array(
            values
                .into_iter()
                .map(configuration_value)
                .collect::<Result<_, _>>()?,
        ),
        toml::Value::Table(value) => {
            HostProviderConfigurationValue::Table(configuration_from_table(value)?)
        }
        toml::Value::Datetime(_) => {
            return Err(invalid(
                "TOML datetime configuration values are unsupported",
            ));
        }
    })
}

fn invalid(message: impl Into<String>) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidInput, message.into())
}

#[cfg(test)]
mod tests {
    use super::{RuntimeInputs, read_provider_configuration};
    use geam_core::{HostProviderConfiguration, HostProviderConfigurationValue as Value};
    use std::collections::BTreeMap;
    use std::ffi::OsStr;
    use std::fs;
    use std::io;

    #[test]
    fn absent_and_empty_documents_use_only_executable_relative_resources() {
        let directory = tempfile::tempdir().unwrap();
        let root = directory.path();
        let executable = root.join("deployment/app");
        let config = root.join("runtime.toml");
        fs::write(&config, "").unwrap();
        for selected in [None, Some(config.as_os_str())] {
            let inputs = RuntimeInputs::read(
                &executable,
                &root.join("other cwd"),
                selected,
                &["app", "dependency", "native_package"],
                &["provider"],
            )
            .unwrap();
            assert_eq!(inputs.configurations, BTreeMap::new());
            assert_eq!(
                inputs.resources.resources,
                BTreeMap::from([
                    ("app".into(), root.join("deployment/priv/app")),
                    ("dependency".into(), root.join("deployment/priv/dependency")),
                    (
                        "native_package".into(),
                        root.join("deployment/priv/native_package")
                    ),
                ])
            );
        }
        assert!(!root.join("deployment").exists());
    }

    #[test]
    fn paths_use_config_parent_while_provider_values_remain_unchanged_and_fresh() {
        let directory = tempfile::tempdir().unwrap();
        let root = directory.path();
        fs::create_dir(root.join("settings")).unwrap();
        fs::write(
            root.join("settings/provider.toml"),
            r#"
text = "relative/provider/value"
number = 41
weight = 1.5
enabled = true
items = [2, "item", false]
[nested]
name = "nested"
"#,
        )
        .unwrap();
        let absolute_provider = root.join("absolute.toml");
        fs::write(&absolute_provider, "number = 99").unwrap();
        let absolute_resource = root.join("absolute resource");
        let config = root.join("settings/runtime.toml");
        let source = format!(
            r#"
[providers]
first = "provider.toml"
second = {}
[resources]
app = "../assets"
dependency = {}
"#,
            toml::Value::String(absolute_provider.to_string_lossy().into_owned()),
            toml::Value::String(absolute_resource.to_string_lossy().into_owned())
        );
        fs::write(&config, source).unwrap();
        for selected in [config.as_os_str(), OsStr::new("settings/runtime.toml")] {
            let inputs = RuntimeInputs::read(
                &root.join("bin/app"),
                root,
                Some(selected),
                &["app", "dependency", "native_package"],
                &["first", "second", "omitted"],
            )
            .unwrap();
            assert_eq!(
                inputs.configurations,
                BTreeMap::from([
                    (
                        "first".into(),
                        HostProviderConfiguration::new(BTreeMap::from([
                            (
                                "text".into(),
                                Value::String("relative/provider/value".into())
                            ),
                            ("number".into(), Value::Integer(41)),
                            ("weight".into(), Value::Float(1.5)),
                            ("enabled".into(), Value::Bool(true)),
                            (
                                "items".into(),
                                Value::Array(vec![
                                    Value::Integer(2),
                                    Value::String("item".into()),
                                    Value::Bool(false)
                                ])
                            ),
                            (
                                "nested".into(),
                                Value::Table(HostProviderConfiguration::new(BTreeMap::from([(
                                    "name".into(),
                                    Value::String("nested".into())
                                ),])))
                            ),
                        ]))
                    ),
                    (
                        "second".into(),
                        HostProviderConfiguration::new(BTreeMap::from([(
                            "number".into(),
                            Value::Integer(99)
                        )]))
                    ),
                ])
            );
            assert_eq!(
                inputs.resources.resources,
                BTreeMap::from([
                    ("app".into(), root.join("settings/../assets")),
                    ("dependency".into(), absolute_resource.clone()),
                    (
                        "native_package".into(),
                        root.join("bin/priv/native_package")
                    ),
                ])
            );
        }
        fs::write(&absolute_provider, "number = 101").unwrap();
        let inputs = RuntimeInputs::read(
            &root.join("bin/app"),
            root,
            Some(config.as_os_str()),
            &["app", "dependency", "native_package"],
            &["first", "second"],
        )
        .unwrap();
        assert_eq!(
            inputs.configurations["second"].get("number"),
            Some(&Value::Integer(101))
        );
        assert!(!absolute_resource.exists());
    }

    #[test]
    fn rejects_invalid_runtime_selections_before_reading_provider_files() {
        let directory = tempfile::tempdir().unwrap();
        let root = directory.path();
        let config = root.join("runtime.toml");
        for (source, reason) in [
            ("unexpected = true", "has unknown field unexpected"),
            ("providers = 1", ": providers must be a table"),
            ("resources = []", ": resources must be a table"),
            (
                "[providers]\nunknown = 'missing'",
                ": unknown providers package unknown",
            ),
            (
                "[resources]\nunknown = 'missing'",
                ": unknown resources package unknown",
            ),
            (
                "[providers]\nprovider = 1",
                ": providers.provider must be a path string",
            ),
            (
                "[resources]\napp = false",
                ": resources.app must be a path string",
            ),
            (
                "[providers]\nprovider = ''",
                ": providers.provider must be non-empty",
            ),
            ("[resources]\napp = ''", ": resources.app must be non-empty"),
            (
                "[providers]\nprovider = 'missing'\n[resources]\napp = ''",
                ": resources.app must be non-empty",
            ),
            (
                "unexpected = true\n[providers]\nprovider = 'missing'",
                "has unknown field unexpected",
            ),
        ] {
            fs::write(&config, source).unwrap();
            let error = RuntimeInputs::read(
                &root.join("bin/app"),
                root,
                Some(config.as_os_str()),
                &["app"],
                &["provider"],
            )
            .err()
            .unwrap();
            assert_eq!(error.kind(), io::ErrorKind::InvalidInput);
            let separator = if reason.starts_with(':') { "" } else { " " };
            assert_eq!(
                error.to_string(),
                format!(
                    "runtime configuration {}{separator}{reason}",
                    config.display()
                )
            );
        }
        let error = RuntimeInputs::read(&root.join("app"), root, Some(OsStr::new("")), &[], &[])
            .err()
            .unwrap();
        assert_eq!(
            error.to_string(),
            "GEAM_CONFIG must name a non-empty configuration path"
        );
    }

    #[test]
    fn reports_file_and_toml_failures_without_dumping_configuration_source() {
        let directory = tempfile::tempdir().unwrap();
        let root = directory.path();
        let config = root.join("runtime.toml");
        let error = RuntimeInputs::read(
            &root.join("app"),
            root,
            Some(config.as_os_str()),
            &["app"],
            &["provider"],
        )
        .err()
        .unwrap();
        let os_error = fs::read_to_string(&config).unwrap_err();
        assert_eq!(error.kind(), os_error.kind());
        assert_eq!(
            error.to_string(),
            format!(
                "failed to read runtime configuration {}: {os_error}",
                config.display()
            )
        );
        for source in [
            "[providers",
            "[providers]\nprovider = 'one'\nprovider = 'two'",
        ] {
            fs::write(&config, source).unwrap();
            let parse_error = toml::from_str::<toml::Table>(source).unwrap_err();
            let error = RuntimeInputs::read(
                &root.join("app"),
                root,
                Some(config.as_os_str()),
                &["app"],
                &["provider"],
            )
            .err()
            .unwrap();
            assert_eq!(
                error.to_string(),
                format!(
                    "invalid runtime configuration {}: {}",
                    config.display(),
                    parse_error.message()
                )
            );
        }
        fs::write(&config, "[providers]\nprovider = 'provider.toml'").unwrap();
        let provider = root.join("provider.toml");
        let error = RuntimeInputs::read(
            &root.join("app"),
            root,
            Some(config.as_os_str()),
            &["app"],
            &["provider"],
        )
        .err()
        .unwrap();
        let os_error = fs::read_to_string(&provider).unwrap_err();
        assert_eq!(
            error.to_string(),
            format!(
                "failed to read provider configuration {}: {os_error}",
                provider.display()
            )
        );
        fs::write(&provider, "password = 'secret'\ninvalid").unwrap();
        let error = read_provider_configuration(&provider).unwrap_err();
        assert!(error.to_string().starts_with(&format!(
            "invalid provider configuration {}:",
            provider.display()
        )));
        assert!(!error.to_string().contains("secret"));
        for source in [
            "date = 2026-09-14",
            "items = [2026-09-14]",
            "[nested]\ndate = 2026-09-14",
        ] {
            fs::write(&provider, source).unwrap();
            let error = read_provider_configuration(&provider).unwrap_err();
            assert_eq!(
                error.to_string(),
                format!(
                    "invalid provider configuration {}: TOML datetime configuration values are unsupported",
                    provider.display()
                )
            );
        }
        fs::write(&provider, [0xff]).unwrap();
        assert_eq!(
            read_provider_configuration(&provider).unwrap_err().kind(),
            io::ErrorKind::InvalidData
        );
    }

    #[cfg(unix)]
    #[test]
    fn os_native_configuration_and_executable_paths_need_no_unicode_conversion() {
        use std::ffi::OsString;
        use std::os::unix::ffi::OsStringExt;
        let directory = tempfile::tempdir().unwrap();
        let native = directory
            .path()
            .join(OsString::from_vec(b"native-\xff".to_vec()));
        let config = directory.path().join("runtime.toml");
        fs::write(&config, "[resources]\napp = 'assets'").unwrap();
        let inputs = RuntimeInputs::read(
            &native.join("app"),
            directory.path(),
            Some(config.as_os_str()),
            &["app", "dependency"],
            &[],
        )
        .unwrap();
        assert_eq!(
            inputs.resources.resources,
            BTreeMap::from([
                ("app".into(), directory.path().join("assets")),
                ("dependency".into(), native.join("priv/dependency")),
            ])
        );
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn reads_configuration_from_a_non_unicode_filesystem_path() {
        use std::ffi::OsString;
        use std::os::unix::ffi::OsStringExt;
        let directory = tempfile::tempdir().unwrap();
        let native = directory
            .path()
            .join(OsString::from_vec(b"native-\xff".to_vec()));
        fs::create_dir(&native).unwrap();
        let config = native.join("runtime.toml");
        fs::write(&config, "[resources]\napp = 'assets'").unwrap();
        let inputs = RuntimeInputs::read(
            &native.join("app"),
            directory.path(),
            Some(config.as_os_str()),
            &["app"],
            &[],
        )
        .unwrap();
        assert_eq!(
            inputs.resources.resources,
            BTreeMap::from([("app".into(), native.join("assets"))])
        );
    }
}
