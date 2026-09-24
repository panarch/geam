use std::collections::BTreeMap;
use std::collections::btree_map::Entry;
use std::ffi::OsStr;
use std::io;
use std::path::PathBuf;
use toml::{Table, Value};

#[derive(Debug, PartialEq, Eq)]
pub struct RunnerControl {
    pub project_root: String,
    pub module: String,
    pub operation: RunnerOperation,
}

#[derive(Debug, PartialEq, Eq)]
pub enum RunnerOperation {
    Check,
    Run {
        configurations: BTreeMap<String, PathBuf>,
    },
    Prepare {
        output: PathBuf,
    },
}

impl RunnerControl {
    pub fn parse(input: Option<&OsStr>) -> Result<Self, io::Error> {
        let input = input.ok_or_else(|| invalid("missing GEAM_RUNNER_CONTROL"))?;
        let source = input
            .to_str()
            .ok_or_else(|| invalid("GEAM_RUNNER_CONTROL must be Unicode"))?;
        let mut table: Table = toml::from_str(source).map_err(|error: toml::de::Error| {
            invalid(format!("invalid runner control TOML: {}", error.message()))
        })?;
        if table.remove("schema") != Some(Value::Integer(1)) {
            return Err(invalid("runner control schema must be 1"));
        }
        let mode = take_string(&mut table, "mode")?;
        let project_root = take_string(&mut table, "project_root")?;
        let module = take_string(&mut table, "module")?;
        let operation = match mode.as_str() {
            "check" => RunnerOperation::Check,
            "run" => RunnerOperation::Run {
                configurations: take_configurations(&mut table)?,
            },
            "prepare" => RunnerOperation::Prepare {
                output: take_string(&mut table, "output")?.into(),
            },
            _ => return Err(invalid(format!("unknown runner control mode {mode}"))),
        };
        reject_remaining(&table, "runner control")?;
        Ok(Self {
            project_root,
            module,
            operation,
        })
    }
}

fn take_string(table: &mut Table, field: &str) -> Result<String, io::Error> {
    match table.remove(field) {
        Some(Value::String(value)) => Ok(value),
        _ => Err(invalid(format!("runner control {field} must be a String"))),
    }
}

fn take_configurations(table: &mut Table) -> Result<BTreeMap<String, PathBuf>, io::Error> {
    let mut configurations = BTreeMap::new();
    let Some(value) = table.remove("configurations") else {
        return Ok(configurations);
    };
    let Value::Array(entries) = value else {
        return Err(invalid("runner control configurations must be an array"));
    };
    for entry in entries {
        let Value::Table(mut entry) = entry else {
            return Err(invalid("runner control configuration must be a table"));
        };
        let package = take_string(&mut entry, "package")?;
        let path = take_string(&mut entry, "path")?;
        reject_remaining(&entry, "runner control configuration")?;
        match configurations.entry(package) {
            Entry::Vacant(entry) => {
                entry.insert(path.into());
            }
            Entry::Occupied(entry) => {
                return Err(invalid(format!(
                    "provider configuration for {} was supplied more than once",
                    entry.key()
                )));
            }
        }
    }
    Ok(configurations)
}

fn reject_remaining(table: &Table, context: &str) -> Result<(), io::Error> {
    if let Some(field) = table.keys().next() {
        return Err(invalid(format!("{context} has unknown field {field}")));
    }
    Ok(())
}

fn invalid(message: impl Into<String>) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidInput, message.into())
}

#[cfg(test)]
mod tests {
    use super::{RunnerControl, RunnerOperation};
    use std::collections::BTreeMap;
    use std::ffi::OsStr;
    use std::io::ErrorKind;
    use std::path::PathBuf;

    #[test]
    fn admits_only_mode_specific_control_values() {
        for (source, operation) in [
            ("mode = 'check'\n", RunnerOperation::Check),
            (
                "mode = 'run'\n",
                RunnerOperation::Run {
                    configurations: BTreeMap::new(),
                },
            ),
            (
                "mode = 'run'\nconfigurations = []\n",
                RunnerOperation::Run {
                    configurations: BTreeMap::new(),
                },
            ),
            (
                "mode = 'prepare'\noutput = 'out dir/program.rs'\n",
                RunnerOperation::Prepare {
                    output: PathBuf::from("out dir/program.rs"),
                },
            ),
            (
                "mode = 'run'\n[[configurations]]\npackage = 'images'\npath = 'settings/a=b.toml'\n[[configurations]]\npackage = 'search'\npath = 'settings/검색.toml'\n",
                RunnerOperation::Run {
                    configurations: BTreeMap::from([
                        ("images".into(), PathBuf::from("settings/a=b.toml")),
                        ("search".into(), PathBuf::from("settings/검색.toml")),
                    ]),
                },
            ),
        ] {
            let source =
                format!("schema = 1\nproject_root = 'project space'\nmodule = 'worker'\n{source}");
            assert_eq!(
                RunnerControl::parse(Some(OsStr::new(&source))).unwrap(),
                RunnerControl {
                    project_root: "project space".into(),
                    module: "worker".into(),
                    operation,
                }
            );
        }
    }

    #[test]
    fn decodes_escaped_control_without_delimiter_splitting() {
        let input = concat!(
            "schema = 1\nmode = \"run\"\nproject_root = \"root\\t\\\"path\"\nmodule = \"tools/report\"\n",
            "\n[[configurations]]\npackage = \"images\"\npath = \"C:\\\\space dir\\\\quote\\\"=한글\\n.toml\"\n",
        );
        assert_eq!(
            RunnerControl::parse(Some(OsStr::new(input))).unwrap(),
            RunnerControl {
                project_root: "root\t\"path".into(),
                module: "tools/report".into(),
                operation: RunnerOperation::Run {
                    configurations: BTreeMap::from([(
                        "images".into(),
                        PathBuf::from("C:\\space dir\\quote\"=한글\n.toml"),
                    )]),
                },
            }
        );
    }

    #[test]
    fn rejects_absent_and_malformed_control_at_admission() {
        for (input, expected) in [
            (None, "missing GEAM_RUNNER_CONTROL"),
            (Some(""), "runner control schema must be 1"),
            (Some("schema = 2"), "runner control schema must be 1"),
            (Some("schema = '1'"), "runner control schema must be 1"),
            (Some("schema = 1"), "runner control mode must be a String"),
            (
                Some("schema = 1\nmode = false"),
                "runner control mode must be a String",
            ),
            (
                Some("schema = 1\nmode = 'run'"),
                "runner control project_root must be a String",
            ),
            (
                Some("schema = 1\nmode = 'run'\nproject_root = 'root'"),
                "runner control module must be a String",
            ),
        ] {
            let error = RunnerControl::parse(input.map(OsStr::new)).unwrap_err();
            assert_eq!(error.kind(), ErrorKind::InvalidInput);
            assert_eq!(error.to_string(), expected);
        }
        for (input, expected) in [
            (
                "=",
                "invalid runner control TOML: unquoted keys cannot be empty, expected letters, numbers, `-`, `_`",
            ),
            (
                "schema=1\nschema=1",
                "invalid runner control TOML: duplicate key",
            ),
        ] {
            let error = RunnerControl::parse(Some(OsStr::new(input))).unwrap_err();
            assert_eq!(error.kind(), ErrorKind::InvalidInput);
            assert_eq!(error.to_string(), expected);
        }
    }

    #[test]
    fn rejects_unknown_or_incompatible_fields_and_duplicate_configurations() {
        for (fields, expected) in [
            ("mode = 'other'", "unknown runner control mode other"),
            (
                "mode = 'check'\noutput = 'out'",
                "runner control has unknown field output",
            ),
            (
                "mode = 'check'\nconfigurations = []",
                "runner control has unknown field configurations",
            ),
            (
                "mode = 'run'\nextra = true",
                "runner control has unknown field extra",
            ),
            ("mode = 'prepare'", "runner control output must be a String"),
            (
                "mode = 'prepare'\noutput = false",
                "runner control output must be a String",
            ),
            (
                "mode = 'prepare'\noutput = 'out'\nconfigurations = []",
                "runner control has unknown field configurations",
            ),
            (
                "mode = 'run'\nconfigurations = {}",
                "runner control configurations must be an array",
            ),
            (
                "mode = 'run'\nconfigurations = [1]",
                "runner control configuration must be a table",
            ),
            (
                "mode = 'run'\nconfigurations = [{path = 'a'}]",
                "runner control package must be a String",
            ),
            (
                "mode = 'run'\nconfigurations = [{package = 'images'}]",
                "runner control path must be a String",
            ),
            (
                "mode = 'run'\nconfigurations = [{package = 'images', path = 2}]",
                "runner control path must be a String",
            ),
            (
                "mode = 'run'\nconfigurations = [{package = 'images', path = 'a', extra = true}]",
                "runner control configuration has unknown field extra",
            ),
            (
                "mode = 'run'\nconfigurations = [{package = 'images', path = 'a'}, {package = 'images', path = 'b'}]",
                "provider configuration for images was supplied more than once",
            ),
        ] {
            let source = format!("schema=1\nproject_root='root'\nmodule='worker'\n{fields}");
            let error = RunnerControl::parse(Some(OsStr::new(&source))).unwrap_err();
            assert_eq!(error.kind(), ErrorKind::InvalidInput);
            assert_eq!(error.to_string(), expected);
        }
    }

    #[test]
    fn rejects_non_unicode_control_without_touching_process_environment() {
        #[cfg(unix)]
        let input = {
            use std::os::unix::ffi::OsStringExt;
            std::ffi::OsString::from_vec(vec![0xff])
        };
        #[cfg(windows)]
        let input = {
            use std::os::windows::ffi::OsStringExt;
            std::ffi::OsString::from_wide(&[0xd800])
        };
        let error = RunnerControl::parse(Some(&input)).unwrap_err();
        assert_eq!(error.kind(), ErrorKind::InvalidInput);
        assert_eq!(error.to_string(), "GEAM_RUNNER_CONTROL must be Unicode");
    }
}
