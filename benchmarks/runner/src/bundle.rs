use crate::cases::{self, Case};
use crate::environment::{self, Host, Source};
use crate::{Result, invalid};
use clap::ValueEnum;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::AtomicBool;

pub(super) struct Bundle {
    root: PathBuf,
    pub(super) manifest: Manifest,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Manifest {
    pub(super) schema: u32,
    pub(super) cases: Vec<Case>,
    pub(super) targets: BTreeSet<Target>,
    pub(super) files: BTreeMap<String, String>,
    pub(super) suite_sources: BTreeMap<String, String>,
    pub(super) runtime_source: Source,
    pub(super) build: Build,
    pub(super) runtimes: BTreeMap<Target, String>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Build {
    pub(super) host: Host,
    pub(super) tools: BTreeMap<String, String>,
    pub(super) geam_cli_sha256: Option<String>,
    pub(super) native_dependencies: BTreeMap<String, String>,
    pub(super) application_lock: Option<String>,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize, ValueEnum)]
#[serde(rename_all = "snake_case")]
#[value(rename_all = "snake_case")]
pub(super) enum Target {
    Geam,
    Erlang,
    #[serde(rename = "javascript")]
    #[value(name = "javascript")]
    JavaScript,
}

impl Bundle {
    pub(super) fn load(root: &Path) -> Result<Self> {
        let root = root.canonicalize()?;
        let manifest: Manifest = serde_json::from_slice(&fs::read(root.join("bundle.json"))?)?;
        manifest.validate()?;
        let bundle = Self { root, manifest };
        bundle.verify()?;
        bundle.admit(environment::controller(std::env::current_exe()))
    }

    fn admit(self, controller: Result<Vec<u8>>) -> Result<Self> {
        let controller = environment::hash(&controller?);
        if self.manifest.files.get("controller") != Some(&controller) {
            return Err(invalid("use the controller shipped with this bundle").into());
        }
        if self.manifest.build.host.os != std::env::consts::OS
            || self.manifest.build.host.architecture != std::env::consts::ARCH
        {
            return Err(invalid(
                "bundle native operating system/architecture does not match this host",
            )
            .into());
        }
        Ok(self)
    }

    pub(super) fn verify(&self) -> Result<()> {
        if environment::files(&self.root.join("payload"))? != self.manifest.files {
            return Err(
                invalid("bundle payload changed, missing, or contains undeclared files").into(),
            );
        }
        Ok(())
    }

    pub(super) fn command(&self, target: Target) -> Result<Command> {
        if !self.manifest.targets.contains(&target) {
            return Err(invalid(format!("bundle does not contain {}", target.name())).into());
        }
        let payload = self.root.join("payload");
        let mut command = match target {
            Target::Geam => Command::new(environment::contained(&payload, "geam/program")?),
            Target::Erlang => {
                let mut command = Command::new("erl");
                command.arg("-pa");
                let paths = self
                    .manifest
                    .files
                    .keys()
                    .filter(|name| name.starts_with("erlang/") && name.ends_with(".beam"))
                    .filter_map(|name| Path::new(name).parent())
                    .collect::<BTreeSet<_>>();
                for path in paths {
                    command.arg(payload.join(path));
                }
                command.args([
                    "-eval",
                    "geam_benchmarks@@main:run(geam_benchmarks)",
                    "-noshell",
                ]);
                command
            }
            Target::JavaScript => {
                let mut command = Command::new("node");
                command.arg(environment::contained(&payload, "javascript/main.mjs")?);
                command
            }
        };
        command.current_dir(&payload).env_remove("GEAM_CONFIG");
        Ok(command)
    }
}

impl Manifest {
    pub(super) fn validate(&self) -> Result<()> {
        if self.schema != 1 || self.cases != cases::suite(None) || self.targets.is_empty() {
            return Err(invalid("unsupported bundle schema, catalogue or empty targets").into());
        }
        if self.runtimes.keys().copied().collect::<BTreeSet<_>>() != self.targets {
            return Err(invalid("bundle runtime inventory does not match its targets").into());
        }
        for required in
            std::iter::once("controller").chain(self.targets.iter().map(|target| match target {
                Target::Geam => "geam/program",
                Target::Erlang => "erlang/geam_benchmarks/ebin/geam_benchmarks@@main.beam",
                Target::JavaScript => "javascript/main.mjs",
            }))
        {
            if !self.files.contains_key(required) {
                return Err(
                    invalid(format!("bundle is missing required payload: {required}")).into(),
                );
            }
        }
        Ok(())
    }

    pub(super) fn compatible(&self, other: &Self) -> Result<()> {
        self.validate()?;
        other.validate()?;
        if !self.targets.contains(&Target::Geam) || !other.targets.contains(&Target::Geam) {
            return Err(invalid("paired comparison requires Geam in both bundles").into());
        }
        if self.suite_sources != other.suite_sources
            || self.files.get("controller") != other.files.get("controller")
            || self.build.tools != other.build.tools
            || self.build.host.overrides != other.build.host.overrides
            || self.build.host.os != other.build.host.os
            || self.build.host.architecture != other.build.host.architecture
        {
            return Err(invalid(
                "paired bundles have different suite/controller/compiler/build settings",
            )
            .into());
        }
        Ok(())
    }
}

impl Target {
    pub(super) fn name(self) -> &'static str {
        match self {
            Self::Geam => "geam",
            Self::Erlang => "erlang",
            Self::JavaScript => "javascript",
        }
    }

    pub(super) fn runtime(self, logs: &Path, cancelled: &AtomicBool) -> Result<String> {
        match self {
            Self::Geam => Ok(
                "native executable; runtime source and library requirements in bundle.json".into(),
            ),
            Self::Erlang => environment::probe(
                Command::new("erl").args([
                    "-noshell",
                    "-eval",
                    "io:format(\"OTP ~s; ERTS ~s; emulator ~p~n\", [erlang:system_info(otp_release), erlang:system_info(version), erlang:system_info(emu_flavor)]), halt().",
                ]),
                logs,
                cancelled,
            ),
            Self::JavaScript => environment::probe(
                Command::new("node").args([
                    "--eval",
                    "console.log(JSON.stringify(process.versions))",
                ]),
                logs,
                cancelled,
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Build, Manifest, Target};
    use crate::cases;
    use crate::environment::{Host, Source, hash};
    use std::collections::{BTreeMap, BTreeSet};

    #[test]
    fn manifest_admission_separates_runtime_changes_from_shared_comparison_inputs() {
        let manifest = Manifest {
            schema: 1,
            cases: cases::suite(None),
            targets: BTreeSet::from([Target::Geam]),
            files: BTreeMap::from([
                ("controller".into(), hash(b"controller")),
                ("geam/program".into(), hash(b"program")),
            ]),
            suite_sources: BTreeMap::from([(
                "project/src/main.gleam".into(),
                hash(b"pub fn main() { 1 }"),
            )]),
            runtime_source: Source {
                commit: "base".into(),
                changes: String::new(),
                excluded_outputs: vec![],
                files: BTreeMap::from([("core/src/lib.rs".into(), hash(b"runtime"))]),
            },
            build: Build {
                host: Host {
                    os: "macos".into(),
                    architecture: "aarch64".into(),
                    system: "test build host".into(),
                    logical_cpus: 4,
                    machine: "build".into(),
                    overrides: BTreeMap::new(),
                },
                tools: BTreeMap::from([("rustc".into(), "rustc fixture".into())]),
                geam_cli_sha256: Some(hash(b"cli")),
                native_dependencies: BTreeMap::new(),
                application_lock: Some("version = 4".into()),
            },
            runtimes: BTreeMap::from([(Target::Geam, "native".into())]),
        };
        let json = serde_json::json!({
            "schema":1,"cases":cases::suite(None),"targets":["geam"],
            "files":{"controller":hash(b"controller"),"geam/program":hash(b"program")},
            "suite_sources":{"project/src/main.gleam":hash(b"pub fn main() { 1 }")},
            "runtime_source":{"commit":"base","changes":"","excluded_outputs":[],"files":{"core/src/lib.rs":hash(b"runtime")}},
            "build":{"host":{"os":"macos","architecture":"aarch64","system":"test build host","logical_cpus":4,"machine":"build","overrides":{}},"tools":{"rustc":"rustc fixture"},"geam_cli_sha256":hash(b"cli"),"native_dependencies":{},"application_lock":"version = 4"},
            "runtimes":{"geam":"native"}
        });
        assert_eq!(serde_json::to_value(&manifest).unwrap(), json);
        assert_eq!(serde_json::from_value::<Manifest>(json).unwrap(), manifest);
        manifest.validate().unwrap();
        let mut candidate = manifest.clone();
        candidate.runtime_source.commit = "candidate".into();
        candidate
            .runtime_source
            .files
            .insert("core/src/lib.rs".into(), hash(b"improved runtime"));
        candidate
            .files
            .insert("geam/program".into(), hash(b"new program"));
        candidate.build.geam_cli_sha256 = Some(hash(b"new cli"));
        candidate.build.application_lock =
            Some("version = 4\n# different selected runtime dependencies".into());
        manifest.compatible(&candidate).unwrap();
        candidate
            .suite_sources
            .insert("project/src/main.gleam".into(), hash(b"changed workload"));
        assert_eq!(
            manifest.compatible(&candidate).unwrap_err().to_string(),
            "paired bundles have different suite/controller/compiler/build settings"
        );
        for field in [
            "schema",
            "cases",
            "targets",
            "runtimes",
            "controller",
            "entry",
        ] {
            let mut wrong = manifest.clone();
            match field {
                "schema" => wrong.schema = 2,
                "cases" => {
                    wrong.cases.pop();
                }
                "targets" => wrong.targets.clear(),
                "runtimes" => wrong.runtimes.clear(),
                "controller" => {
                    wrong.files.remove("controller");
                }
                _ => {
                    wrong.files.remove("geam/program");
                }
            }
            assert!(wrong.validate().is_err(), "{field}");
        }
        let mut invalid = manifest.clone();
        invalid.schema = 2;
        assert!(manifest.compatible(&invalid).is_err());
        assert!(invalid.compatible(&manifest).is_err());
        for target in [Target::Erlang, Target::JavaScript] {
            let mut other = manifest.clone();
            other.targets = BTreeSet::from([target]);
            other.runtimes = BTreeMap::from([(target, "runtime".into())]);
            assert!(other.validate().is_err());
            let name = match target {
                Target::Erlang => "erlang/geam_benchmarks/ebin/geam_benchmarks@@main.beam",
                _ => "javascript/main.mjs",
            };
            other.files.insert(name.into(), hash(b"entry"));
            other.validate().unwrap();
            assert_eq!(
                manifest.compatible(&other).unwrap_err().to_string(),
                "paired comparison requires Geam in both bundles"
            );
        }
    }

    #[test]
    fn bundle_admission_checks_controller_host_and_complete_payload() {
        use super::Bundle;
        use std::fs;
        let temporary = tempfile::tempdir().unwrap();
        let root = temporary.path();
        assert!(Bundle::load(&root.join("missing")).is_err());
        assert!(Bundle::load(root).is_err());
        fs::write(root.join("bundle.json"), "not JSON").unwrap();
        assert!(Bundle::load(root).is_err());
        fs::create_dir_all(root.join("payload/geam")).unwrap();
        fs::copy(
            std::env::current_exe().unwrap(),
            root.join("payload/controller"),
        )
        .unwrap();
        fs::create_dir_all(root.join("payload/javascript")).unwrap();
        fs::write(root.join("payload/javascript/main.mjs"), b"entry bytes").unwrap();
        // Admission checks bytes and metadata; it does not execute the payload.
        fs::write(root.join("payload/geam/program"), b"artifact bytes").unwrap();
        let mut json = serde_json::json!({
            "schema":1,"cases":cases::suite(None),"targets":["geam", "javascript"],
            "files":crate::environment::files(&root.join("payload")).unwrap(),
            "suite_sources":{"project/src/main.gleam":hash(b"source")},
            "runtime_source":{"commit":"revision","changes":"","excluded_outputs":[],"files":{"src/lib.rs":hash(b"runtime")}},
            "build":{"host":{"os":std::env::consts::OS,"architecture":std::env::consts::ARCH,"system":"test","logical_cpus":4,"machine":"build","overrides":{}},"tools":{},"geam_cli_sha256":null,"native_dependencies":{},"application_lock":null},
            "runtimes":{"geam":"native", "javascript":"node"}
        });
        fs::write(root.join("bundle.json"), json.to_string()).unwrap();
        json["schema"] = serde_json::json!(2);
        fs::write(root.join("bundle.json"), json.to_string()).unwrap();
        assert_eq!(
            Bundle::load(root).err().unwrap().to_string(),
            "unsupported bundle schema, catalogue or empty targets"
        );
        json["schema"] = serde_json::json!(1);
        fs::write(root.join("bundle.json"), json.to_string()).unwrap();
        let bundle = Bundle::load(root).unwrap();
        let invalid = Bundle::load(root)
            .unwrap()
            .admit(Err(std::io::Error::other("controller unavailable").into()));
        assert_eq!(invalid.err().unwrap().to_string(), "controller unavailable");
        fs::remove_file(root.join("payload/javascript/main.mjs")).unwrap();
        assert!(bundle.command(Target::JavaScript).is_err());
        fs::write(root.join("payload/javascript/main.mjs"), b"entry bytes").unwrap();
        let javascript = bundle.command(Target::JavaScript).unwrap();
        assert_eq!(javascript.get_program(), "node");
        assert_eq!(
            javascript.get_args().collect::<Vec<_>>(),
            [root
                .join("payload/javascript/main.mjs")
                .canonicalize()
                .unwrap()]
        );
        let command = bundle.command(Target::Geam).unwrap();
        assert_eq!(
            command.get_program(),
            root.join("payload/geam/program").canonicalize().unwrap()
        );
        assert_eq!(command.get_args().count(), 0);
        assert_eq!(
            command.get_envs().collect::<Vec<_>>(),
            [(std::ffi::OsStr::new("GEAM_CONFIG"), None)]
        );
        assert!(bundle.command(Target::Erlang).is_err());
        fs::remove_file(root.join("payload/geam/program")).unwrap();
        assert!(bundle.command(Target::Geam).is_err());
        fs::write(root.join("payload/geam/program"), b"artifact bytes").unwrap();
        fs::write(root.join("payload/extra"), b"undeclared").unwrap();
        assert!(bundle.verify().is_err());
        assert!(Bundle::load(root).is_err());
        fs::remove_file(root.join("payload/extra")).unwrap();
        for name in [
            "geam_benchmarks/ebin/geam_benchmarks@@main.beam",
            "geam_benchmarks/ebin/geam_benchmarks.beam",
            "gleam_stdlib/ebin/list.beam",
            "gleam_stdlib/ebin/gleam_stdlib.app",
        ] {
            let path = root.join("payload/erlang").join(name);
            fs::create_dir_all(path.parent().unwrap()).unwrap();
            fs::write(path, b"artifact bytes").unwrap();
        }
        json["targets"] = serde_json::json!(["geam", "erlang", "javascript"]);
        json["runtimes"]["erlang"] = serde_json::json!("OTP");
        json["files"] =
            serde_json::to_value(crate::environment::files(&root.join("payload")).unwrap())
                .unwrap();
        fs::write(root.join("bundle.json"), json.to_string()).unwrap();
        let bundle = Bundle::load(root).unwrap();
        let erlang = bundle.command(Target::Erlang).unwrap();
        assert_eq!(erlang.get_program(), "erl");
        let payload = root.canonicalize().unwrap().join("payload");
        assert_eq!(erlang.get_current_dir(), Some(payload.as_path()));
        assert_eq!(
            erlang
                .get_args()
                .map(|arg| arg.to_str().unwrap())
                .collect::<Vec<_>>(),
            [
                "-pa",
                payload
                    .join("erlang/geam_benchmarks/ebin")
                    .to_str()
                    .unwrap(),
                payload.join("erlang/gleam_stdlib/ebin").to_str().unwrap(),
                "-eval",
                "geam_benchmarks@@main:run(geam_benchmarks)",
                "-noshell",
            ]
        );
        for field in ["os", "architecture"] {
            let original = json["build"]["host"][field].clone();
            json["build"]["host"][field] = serde_json::json!("different");
            fs::write(root.join("bundle.json"), json.to_string()).unwrap();
            assert_eq!(
                Bundle::load(root).err().unwrap().to_string(),
                "bundle native operating system/architecture does not match this host"
            );
            json["build"]["host"][field] = original;
        }
        fs::write(root.join("payload/controller"), b"different controller").unwrap();
        json["files"]["controller"] = serde_json::json!(hash(b"different controller"));
        fs::write(root.join("bundle.json"), json.to_string()).unwrap();
        assert_eq!(
            Bundle::load(root).err().unwrap().to_string(),
            "use the controller shipped with this bundle"
        );
        fs::remove_dir_all(root.join("payload")).unwrap();
        assert!(bundle.verify().is_err());
    }

    #[test]
    fn target_protocol_uses_official_backend_names() {
        for (target, name) in [
            (Target::Geam, "geam"),
            (Target::Erlang, "erlang"),
            (Target::JavaScript, "javascript"),
        ] {
            assert_eq!(target.name(), name);
            assert_eq!(
                serde_json::to_string(&target).unwrap(),
                format!("\"{name}\"")
            );
            assert_eq!(
                serde_json::from_str::<Target>(&format!("\"{name}\"")).unwrap(),
                target
            );
        }
        assert!(serde_json::from_str::<Target>("\"node\"").is_err());
    }
}
