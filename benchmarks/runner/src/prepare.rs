use crate::bundle::{Build, Manifest, Target};
use crate::cases;
use crate::environment::{self, copy_files, files, hash, probe};
use crate::{Preparation as Arguments, Result, invalid, new_directory, process};
use std::collections::{BTreeMap, BTreeSet};
use std::fs::{self, File, OpenOptions};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::AtomicBool;
use std::time::Duration;

struct Preparation<'a> {
    checkout: &'a Path,
    suite: &'a Path,
    backends: &'a [Backend<'a>],
    output: &'a Path,
    logs: &'a Path,
    build_directory: Option<&'a Path>,
}

enum Backend<'a> {
    Geam(&'a Path),
    Erlang,
    JavaScript,
}

enum Workspace {
    Temporary(tempfile::TempDir),
    Reused { path: PathBuf, _lock: File },
}

pub(super) fn execute(arguments: Arguments, cancelled: &AtomicBool) -> Result<()> {
    let checkout = arguments.checkout.canonicalize()?;
    let suite = arguments.suite.canonicalize()?;
    let targets = arguments.targets.iter().copied().collect::<BTreeSet<_>>();
    if targets.is_empty() || targets.len() != arguments.targets.len() {
        return Err(invalid("prepare requires unique, nonempty targets").into());
    }
    let geam = arguments.geam.map(|path| path.canonicalize()).transpose()?;
    let mut backends = Vec::new();
    for target in targets {
        backends.push(match target {
            Target::Geam => Backend::Geam(
                geam.as_deref()
                    .ok_or_else(|| invalid("--geam is required when preparing Geam"))?,
            ),
            Target::Erlang => Backend::Erlang,
            Target::JavaScript => Backend::JavaScript,
        });
    }
    let output = new_directory(&arguments.output)?;
    let logs = output.join("preparation");
    let preparation = Preparation {
        checkout: &checkout,
        suite: &suite,
        backends: &backends,
        output: &output,
        logs: &logs,
        build_directory: arguments.build_directory.as_deref(),
    };
    let outcome = preparation.start(cancelled);
    crate::evidence::finish(outcome, &output, &mut std::io::stderr().lock())
}

struct Staged<'a> {
    preparation: Preparation<'a>,
    workspace: Workspace,
    inputs: BTreeMap<String, String>,
    source: environment::Source,
    build: Build,
    runtimes: BTreeMap<Target, String>,
}

impl<'a> Preparation<'a> {
    fn start(self, cancelled: &AtomicBool) -> Result<()> {
        fs::create_dir(self.logs)?;
        self.build(cancelled)
    }

    fn build(self, cancelled: &AtomicBool) -> Result<()> {
        let mut staged = self.stage(cancelled)?;
        staged.compile(cancelled)?;
        staged.seal(cancelled)
    }

    fn stage(self, cancelled: &AtomicBool) -> Result<Staged<'a>> {
        let inputs = suite_inputs(self.suite)?;
        let workspace = Workspace::open(self.build_directory, self.output)?;
        let source = environment::source(
            self.checkout,
            &[self.output, workspace.path()],
            self.logs,
            cancelled,
        )?;
        let host = environment::host("build host".into(), self.logs, cancelled)?;
        let tools = BTreeMap::from([(
            "gleam".into(),
            probe(
                Command::new("gleam").arg("--version"),
                &self.logs.join("gleam-version"),
                cancelled,
            )?,
        )]);
        stage_suite(
            self.suite,
            &workspace.path().join("suite"),
            self.checkout,
            &inputs,
        )?;
        create_payload(self.output, std::env::current_exe())?;
        Ok(Staged {
            preparation: self,
            workspace,
            inputs,
            source,
            build: Build {
                host,
                tools,
                geam_cli_sha256: None,
                native_dependencies: BTreeMap::new(),
                application_lock: None,
            },
            runtimes: BTreeMap::new(),
        })
    }
}

impl Staged<'_> {
    fn compile(&mut self, cancelled: &AtomicBool) -> Result<()> {
        let project = self.workspace.path().join("suite/project");
        let payload = self.preparation.output.join("payload");
        for backend in self.preparation.backends {
            let target = backend.target();
            eprintln!("Preparing {}", target.name());
            let compiled = compile_backend(
                backend,
                &project,
                self.preparation.logs,
                &mut self.build,
                cancelled,
            );
            package_backend(
                target,
                compiled,
                &project,
                &payload,
                self.preparation.logs,
                &mut self.build,
                cancelled,
            )?;
            let runtime = target.runtime(
                &self
                    .preparation
                    .logs
                    .join(format!("runtime-{}", target.name())),
                cancelled,
            );
            self.runtimes.insert(
                target,
                admit_runtime(target, runtime, self.preparation.suite)?,
            );
        }
        #[cfg(target_os = "macos")]
        if self
            .preparation
            .backends
            .iter()
            .any(|backend| backend.target() == Target::Geam)
        {
            self.build.tools.insert(
                "macos_sdk".into(),
                probe(
                    Command::new("xcrun").args(["--sdk", "macosx", "--show-sdk-version"]),
                    &self.preparation.logs.join("sdk-version"),
                    cancelled,
                )?,
            );
        }
        Ok(())
    }

    fn seal(self, cancelled: &AtomicBool) -> Result<()> {
        let output = self.preparation.output;
        let final_logs = self.preparation.logs.join("source-after");
        fs::create_dir(&final_logs)?;
        if environment::source(
            self.preparation.checkout,
            &[output, self.workspace.path()],
            &final_logs,
            cancelled,
        )? != self.source
            || suite_inputs(self.preparation.suite)? != self.inputs
        {
            return Err(invalid("source changed during preparation; bundle not accepted").into());
        }
        let manifest = Manifest {
            schema: 1,
            cases: cases::suite(None),
            targets: self
                .preparation
                .backends
                .iter()
                .map(Backend::target)
                .collect(),
            files: files(&output.join("payload"))?,
            suite_sources: self.inputs,
            runtime_source: self.source,
            build: self.build,
            runtimes: self.runtimes,
        };
        manifest.validate()?;
        drop(self.workspace);
        crate::evidence::seal_json_new(&output.join("bundle.json"), &manifest)?;
        eprintln!("Prepared bundle: {}", output.display());
        Ok(())
    }
}

fn stage_suite(
    suite: &Path,
    copied: &Path,
    checkout: &Path,
    inputs: &BTreeMap<String, String>,
) -> Result<()> {
    for relative in ["project/src", "provider/src", "runner/src"] {
        let path = copied.join(relative);
        if path.exists() {
            fs::remove_dir_all(path)?;
        }
    }
    copy_files(suite, copied, inputs.keys().cloned())?;
    // Reuse compiled output and the resolved lock; provider selection belongs
    // to this preparation, so regenerate the owned application manifest.
    let generated_manifest = copied.join("project/Cargo.toml");
    if generated_manifest.exists() {
        fs::remove_file(generated_manifest)?;
    }
    patch_checkout(copied, checkout)
}

fn create_payload(output: &Path, controller: std::io::Result<PathBuf>) -> Result<()> {
    let payload = output.join("payload");
    fs::create_dir(&payload)?;
    fs::copy(controller?, payload.join("controller"))?;
    Ok(())
}

fn compile_backend(
    backend: &Backend<'_>,
    project: &Path,
    logs: &Path,
    build: &mut Build,
    cancelled: &AtomicBool,
) -> Result<()> {
    match backend {
        Backend::Geam(cli) => {
            build.geam_cli_sha256 = Some(hash(&fs::read(cli)?));
            build.tools.insert(
                "rustc".into(),
                probe(
                    Command::new("rustc").arg("-Vv"),
                    &logs.join("rustc-version"),
                    cancelled,
                )?,
            );
            build.tools.insert(
                "cargo".into(),
                probe(
                    Command::new("cargo").arg("--version"),
                    &logs.join("cargo-version"),
                    cancelled,
                )?,
            );
            build
                .tools
                .insert("profile".into(), "release; CARGO_INCREMENTAL=0".into());
            build_command(
                Command::new(cli)
                    .current_dir(project)
                    .args(["provider", "add", "--path"])
                    .arg(project.with_file_name("provider")),
                &logs.join("provider-add"),
                cancelled,
            )?;
            build_command(
                Command::new(cli)
                    .current_dir(project)
                    .args(["build", "--release"]),
                &logs.join("geam-build"),
                cancelled,
            )
        }
        Backend::Erlang => build_command(
            Command::new("gleam")
                .current_dir(project)
                .args(["export", "erlang-shipment"]),
            &logs.join("erlang-export"),
            cancelled,
        ),
        Backend::JavaScript => build_command(
            Command::new("gleam")
                .current_dir(project)
                .args(["build", "--target", "javascript"]),
            &logs.join("javascript-build"),
            cancelled,
        ),
    }
}

fn package_backend(
    target: Target,
    compiled: Result<()>,
    project: &Path,
    payload: &Path,
    logs: &Path,
    build: &mut Build,
    cancelled: &AtomicBool,
) -> Result<()> {
    compiled?;
    match target {
        Target::Geam => {
            let executable = project.join("build/geam/target/release/geam_benchmarks");
            fs::create_dir(payload.join("geam"))?;
            fs::copy(&executable, payload.join("geam/program"))?;
            build.application_lock = Some(fs::read_to_string(project.join("Cargo.lock"))?);
            build.native_dependencies = native_dependencies(&executable, logs, cancelled)?;
        }
        Target::Erlang => {
            let shipment = project.join("build/erlang-shipment");
            copy_files(
                &shipment,
                &payload.join("erlang"),
                files(&shipment)?.into_keys(),
            )?;
        }
        Target::JavaScript => {
            let compiled = project.join("build/dev/javascript");
            copy_files(
                &compiled,
                &payload.join("javascript"),
                files(&compiled)?.into_keys(),
            )?;
            fs::write(
                payload.join("javascript/main.mjs"),
                "import { main } from './geam_benchmarks/geam_benchmarks.mjs';\nmain();\n",
            )?;
        }
    }
    Ok(())
}

fn admit_runtime(target: Target, runtime: Result<String>, suite: &Path) -> Result<String> {
    let runtime = runtime?;
    if target == Target::JavaScript {
        let versions: BTreeMap<String, String> = serde_json::from_str(&runtime)?;
        if versions.get("node").map(String::as_str)
            != Some(fs::read_to_string(suite.join(".node-version"))?.trim())
        {
            return Err(invalid("Node version does not match the suite's .node-version").into());
        }
    }
    Ok(runtime)
}

impl Backend<'_> {
    fn target(&self) -> Target {
        match self {
            Self::Geam(_) => Target::Geam,
            Self::Erlang => Target::Erlang,
            Self::JavaScript => Target::JavaScript,
        }
    }
}

impl Workspace {
    fn open(path: Option<&Path>, output: &Path) -> Result<Self> {
        let Some(path) = path else {
            return Ok(Self::Temporary(
                tempfile::Builder::new()
                    .prefix("geam-bench-build-")
                    .tempdir_in(output)?,
            ));
        };
        let path = if path.exists() {
            path.canonicalize().map_err(Into::into)
        } else {
            Self::claim(new_directory(path))
        };
        Self::lock(path)
    }

    fn claim(path: Result<PathBuf>) -> Result<PathBuf> {
        let path = path?;
        fs::write(path.join("owner"), "geam-bench build workspace v1\n")?;
        Ok(path)
    }

    fn lock(path: Result<PathBuf>) -> Result<Self> {
        let path = path?;
        if fs::read_to_string(path.join("owner"))? != "geam-bench build workspace v1\n" {
            return Err(invalid("build directory is not owned by geam-bench").into());
        }
        let lock = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(false)
            .open(path.join("lock"))?;
        lock.try_lock()?;
        Ok(Self::Reused { path, _lock: lock })
    }

    fn path(&self) -> &Path {
        match self {
            Self::Temporary(path) => path.path(),
            Self::Reused { path, .. } => path,
        }
    }
}

impl Drop for Workspace {
    fn drop(&mut self) {
        if let Self::Reused { _lock, .. } = self {
            // Unlock explicitly: a concurrent fork may briefly retain the same
            // open-file description until exec closes its inherited descriptor.
            let _ = _lock.unlock();
        }
    }
}

fn suite_inputs(suite: &Path) -> Result<BTreeMap<String, String>> {
    let mut inputs = BTreeMap::new();
    for name in [
        "Cargo.toml",
        "Cargo.lock",
        ".node-version",
        "provider/Cargo.toml",
        "runner/Cargo.toml",
        "project/gleam.toml",
        "project/manifest.toml",
        "project/.cargo/config.toml",
    ] {
        inputs.insert(name.into(), hash(&fs::read(suite.join(name))?));
    }
    for directory in ["project/src", "provider/src", "runner/src"] {
        for (name, digest) in files(&suite.join(directory))? {
            inputs.insert(format!("{directory}/{name}"), digest);
        }
    }
    Ok(inputs)
}

fn patch_checkout(suite: &Path, checkout: &Path) -> Result<()> {
    let path = suite.join("Cargo.toml");
    let mut manifest: toml_edit::DocumentMut = fs::read_to_string(&path)?.parse()?;
    let checkout = checkout
        .to_str()
        .ok_or_else(|| invalid("checkout path is not UTF-8"))?;
    let dependency = manifest
        .as_table_mut()
        .get_mut("workspace")
        .and_then(toml_edit::Item::as_table_like_mut)
        .and_then(|table| table.get_mut("dependencies"))
        .and_then(toml_edit::Item::as_table_like_mut)
        .and_then(|table| table.get_mut("geam"))
        .and_then(toml_edit::Item::as_table_like_mut)
        .and_then(|table| table.get_mut("path"))
        .ok_or_else(|| invalid("suite workspace must declare the Geam path dependency"))?;
    *dependency = toml_edit::value(checkout);
    fs::write(path, manifest.to_string())?;
    let patch = format!(
        "[patch.crates-io]\ngeam = {{ path = {} }}\n",
        toml_edit::Value::from(checkout)
    );
    fs::write(suite.join("project/.cargo/config.toml"), patch)?;
    Ok(())
}

fn build_command(command: &mut Command, logs: &Path, cancelled: &AtomicBool) -> Result<()> {
    process::checked(
        command
            .env("CARGO_INCREMENTAL", "0")
            .env_remove("CARGO_TARGET_DIR"),
        logs,
        Duration::from_secs(3600),
        cancelled,
    )?;
    Ok(())
}

fn native_dependencies(
    executable: &Path,
    logs: &Path,
    cancelled: &AtomicBool,
) -> Result<BTreeMap<String, String>> {
    let mut result = BTreeMap::new();
    result.insert(
        "format".into(),
        probe(
            Command::new("file").arg(executable),
            &logs.join("native-format"),
            cancelled,
        )?,
    );
    #[cfg(target_os = "macos")]
    {
        result.insert(
            "libraries".into(),
            probe(
                Command::new("otool").arg("-L").arg(executable),
                &logs.join("native-libraries"),
                cancelled,
            )?,
        );
        result.insert(
            "load_commands_and_deployment_target".into(),
            probe(
                Command::new("otool").arg("-l").arg(executable),
                &logs.join("native-load-commands"),
                cancelled,
            )?,
        );
    }
    #[cfg(target_os = "linux")]
    {
        result.insert(
            "libraries".into(),
            probe(
                Command::new("ldd").arg(executable),
                &logs.join("native-libraries"),
                cancelled,
            )?,
        );
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::{Workspace, patch_checkout};
    use std::fs;

    #[test]
    fn build_workspace_requires_ownership_and_exclusive_access() {
        let root = tempfile::tempdir().unwrap();
        let temporary = Workspace::open(None, root.path()).unwrap();
        let removed = temporary.path().to_path_buf();
        assert!(removed.exists());
        drop(temporary);
        assert!(!removed.exists());
        let path = root.path().join("reuse");
        let first = Workspace::open(Some(&path), root.path()).unwrap();
        assert_eq!(first.path(), path.canonicalize().unwrap());
        assert!(Workspace::open(Some(&path), root.path()).is_err());
        drop(first);
        let second = Workspace::open(Some(&path), root.path()).unwrap();
        drop(second);
        fs::write(path.join("owner"), "someone else's workspace").unwrap();
        assert_eq!(
            Workspace::open(Some(&path), root.path())
                .err()
                .unwrap()
                .to_string(),
            "build directory is not owned by geam-bench"
        );
    }

    #[test]
    fn build_workspace_reports_creation_marker_and_lock_failures() {
        let root = tempfile::tempdir().unwrap();
        assert!(Workspace::open(None, &root.path().join("missing")).is_err());
        let blocker = root.path().join("file");
        fs::write(&blocker, b"not a directory").unwrap();
        assert!(Workspace::open(Some(&blocker.join("workspace")), root.path()).is_err());
        let unowned = root.path().join("unowned");
        fs::create_dir(&unowned).unwrap();
        assert!(Workspace::open(Some(&unowned), root.path()).is_err());
        fs::write(unowned.join("owner"), "geam-bench build workspace v1\n").unwrap();
        fs::create_dir(unowned.join("lock")).unwrap();
        assert!(Workspace::open(Some(&unowned), root.path()).is_err());
    }

    #[test]
    fn manifest_patching_preserves_io_and_parse_failures() {
        use std::os::unix::{ffi::OsStringExt, fs::PermissionsExt};
        let temporary = tempfile::tempdir().unwrap();
        let root = temporary.path();
        let suite = root.join("suite");
        fs::create_dir_all(suite.join("project/.cargo")).unwrap();
        let manifest = suite.join("Cargo.toml");
        assert!(patch_checkout(&suite, root).is_err());
        fs::write(&manifest, "invalid[").unwrap();
        assert!(patch_checkout(&suite, root).is_err());
        let source = "# preserved comment\n[workspace.dependencies]\ngeam = { path = \"..\" }\n";
        fs::write(&manifest, source).unwrap();
        let invalid_path = std::path::PathBuf::from(std::ffi::OsString::from_vec(vec![255]));
        assert_eq!(
            patch_checkout(&suite, &invalid_path)
                .unwrap_err()
                .to_string(),
            "checkout path is not UTF-8"
        );
        fs::set_permissions(&manifest, fs::Permissions::from_mode(0o400)).unwrap();
        let result = patch_checkout(&suite, root);
        fs::set_permissions(&manifest, fs::Permissions::from_mode(0o600)).unwrap();
        assert_eq!(
            result
                .unwrap_err()
                .downcast_ref::<std::io::Error>()
                .unwrap()
                .kind(),
            std::io::ErrorKind::PermissionDenied
        );
        let configuration = suite.join("project/.cargo/config.toml");
        fs::create_dir(&configuration).unwrap();
        assert!(patch_checkout(&suite, root).is_err());
        fs::remove_dir(&configuration).unwrap();
        patch_checkout(&suite, root).unwrap();
        assert!(
            fs::read_to_string(manifest)
                .unwrap()
                .starts_with("# preserved comment\n")
        );
    }

    #[test]
    fn copied_manifests_select_the_explicit_runtime_checkout() {
        let root = tempfile::tempdir().unwrap();
        fs::create_dir_all(root.path().join("project/.cargo")).unwrap();
        fs::write(root.path().join("Cargo.toml"),"[workspace.dependencies]\ngeam = {path = \"..\", default-features = false, features = [\"provider\"]}\n").unwrap();
        let checkout = root.path().join("checkout with spaces");
        patch_checkout(root.path(), &checkout).unwrap();
        let manifest: toml_edit::DocumentMut = fs::read_to_string(root.path().join("Cargo.toml"))
            .unwrap()
            .parse()
            .unwrap();
        assert_eq!(
            manifest["workspace"]["dependencies"]["geam"]["path"].as_str(),
            checkout.to_str()
        );
        assert_eq!(
            manifest["workspace"]["dependencies"]["geam"]["features"]
                .as_array()
                .unwrap()
                .iter()
                .map(|value| value.as_str().unwrap())
                .collect::<Vec<_>>(),
            &["provider"]
        );
        let patch: toml_edit::DocumentMut =
            fs::read_to_string(root.path().join("project/.cargo/config.toml"))
                .unwrap()
                .parse()
                .unwrap();
        assert_eq!(
            patch["patch"]["crates-io"]["geam"]["path"].as_str(),
            checkout.to_str()
        );
        fs::write(
            root.path().join("Cargo.toml"),
            "[workspace]\nmembers = []\n",
        )
        .unwrap();
        assert_eq!(
            patch_checkout(root.path(), &checkout)
                .unwrap_err()
                .to_string(),
            "suite workspace must declare the Geam path dependency"
        );
    }
    #[test]
    fn preparation_rejects_invalid_selection_before_building() {
        use crate::Preparation;
        use crate::bundle::Target;
        use std::sync::atomic::AtomicBool;
        let root = tempfile::tempdir().unwrap();
        for targets in [vec![], vec![Target::Geam, Target::Geam], vec![Target::Geam]] {
            let count = targets.len();
            let output = root.path().join("output");
            let arguments = Preparation {
                build_directory: None,
                checkout: root.path().into(),
                suite: root.path().into(),
                geam: None,
                output: output.clone(),
                targets,
            };
            let error = super::execute(arguments, &AtomicBool::new(false)).unwrap_err();
            assert_eq!(
                error.to_string(),
                if count == 1 {
                    "--geam is required when preparing Geam"
                } else {
                    "prepare requires unique, nonempty targets"
                }
            );
            assert!(!output.exists());
        }
        let output = root.path().join("failed");
        let arguments = Preparation {
            build_directory: None,
            checkout: root.path().into(),
            suite: root.path().into(),
            geam: None,
            output: output.clone(),
            targets: vec![Target::JavaScript],
        };
        assert!(super::execute(arguments, &AtomicBool::new(false)).is_err());
        assert!(output.join("failure.txt").is_file());
        assert!(!output.join("bundle.json").exists());
    }
    #[test]
    fn acquired_paths_and_runtime_identity_are_required_before_acceptance() {
        use super::{Target, admit_runtime, create_payload, suite_inputs};
        use std::io;
        let root = tempfile::tempdir().unwrap();
        let failed = || io::Error::other("acquisition failed");
        assert!(Workspace::claim(Err(failed().into())).is_err());
        assert!(Workspace::lock(Err(failed().into())).is_err());
        fs::create_dir(root.path().join("owner")).unwrap();
        assert!(Workspace::claim(Ok(root.path().into())).is_err());
        fs::remove_dir(root.path().join("owner")).unwrap();
        Workspace::claim(Ok(root.path().into())).unwrap();
        assert!(create_payload(root.path(), Err(failed())).is_err());
        fs::remove_dir(root.path().join("payload")).unwrap();
        assert!(create_payload(root.path(), Ok(root.path().join("missing"))).is_err());
        assert!(create_payload(root.path(), std::env::current_exe()).is_err());
        assert!(admit_runtime(Target::Geam, Err(failed().into()), root.path()).is_err());
        assert_eq!(
            admit_runtime(Target::Geam, Ok("native".into()), root.path()).unwrap(),
            "native"
        );
        assert!(admit_runtime(Target::JavaScript, Ok("not JSON".into()), root.path()).is_err());
        let identity = r#"{"node":"24.21.0"}"#;
        assert!(admit_runtime(Target::JavaScript, Ok(identity.into()), root.path()).is_err());
        fs::write(root.path().join(".node-version"), "other\n").unwrap();
        assert_eq!(
            admit_runtime(Target::JavaScript, Ok(identity.into()), root.path())
                .unwrap_err()
                .to_string(),
            "Node version does not match the suite's .node-version"
        );
        fs::write(root.path().join(".node-version"), "24.21.0\n").unwrap();
        assert_eq!(
            admit_runtime(Target::JavaScript, Ok(identity.into()), root.path()).unwrap(),
            identity
        );
        for name in [
            "Cargo.toml",
            "Cargo.lock",
            "provider/Cargo.toml",
            "runner/Cargo.toml",
            "project/gleam.toml",
            "project/manifest.toml",
            "project/.cargo/config.toml",
        ] {
            let path = root.path().join(name);
            fs::create_dir_all(path.parent().unwrap()).unwrap();
            fs::write(path, b"input").unwrap();
        }
        assert!(suite_inputs(root.path()).is_err());
    }

    #[test]
    fn real_preparation_and_rust_boundaries_close_failure_admission() {
        use super::{
            Backend, Preparation, Target, compile_backend, package_backend, stage_suite,
            suite_inputs,
        };
        use crate::bundle::Manifest;
        use crate::environment::{copy_files, files};
        use std::path::{Path, PathBuf};
        use std::process::Command;
        use std::sync::atomic::AtomicBool;
        let temporary = tempfile::tempdir().unwrap();
        let root = temporary.path();
        let repository = Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .parent()
            .unwrap();
        let checkout = root.join("checkout");
        assert!(
            Command::new("git")
                .args(["clone", "--quiet", "--shared"])
                .arg(repository)
                .arg(&checkout)
                .status()
                .unwrap()
                .success()
        );
        let patch = Command::new("git")
            .current_dir(repository)
            .args(["diff", "--binary", "HEAD"])
            .output()
            .unwrap();
        assert!(patch.status.success());
        let patch_path = root.join("changes.patch");
        fs::write(&patch_path, patch.stdout).unwrap();
        assert!(
            Command::new("git")
                .current_dir(&checkout)
                .args(["apply", "--allow-empty"])
                .arg(patch_path)
                .status()
                .unwrap()
                .success()
        );
        let suite = root.join("suite");
        let originals = suite_inputs(&repository.join("benchmarks")).unwrap();
        copy_files(
            &repository.join("benchmarks"),
            &suite,
            originals.keys().cloned(),
        )
        .unwrap();
        let cli = std::env::var_os("GEAM_BENCH_TEST_GEAM")
            .map(PathBuf::from)
            .unwrap_or_else(|| repository.join("target/debug/geam"))
            .canonicalize()
            .unwrap();
        let workspace = root.join("workspace");
        let baseline = root.join("baseline");
        let active = AtomicBool::new(false);
        let backends = [Backend::Geam(&cli), Backend::Erlang, Backend::JavaScript];
        // This fixture is compiled from the maintained sources. No alternate
        // evaluator supplies either its outputs or expected checksums.
        super::execute(
            crate::Preparation {
                build_directory: Some(workspace.clone()),
                checkout: checkout.clone(),
                suite: suite.clone(),
                geam: Some(cli.clone()),
                output: baseline.clone(),
                targets: vec![Target::Geam, Target::Erlang, Target::JavaScript],
            },
            &active,
        )
        .unwrap();
        let manifest: Manifest =
            serde_json::from_slice(&fs::read(baseline.join("bundle.json")).unwrap()).unwrap();
        crate::run::tests::verify_real_bundle(&baseline, &root.join("execution"));

        let output = root.join("logs-collision");
        let logs = output.join("preparation");
        fs::create_dir_all(&logs).unwrap();
        assert!(
            Preparation {
                checkout: &checkout,
                suite: &suite,
                backends: &backends,
                output: &output,
                logs: &logs,
                build_directory: Some(&workspace)
            }
            .start(&active)
            .is_err()
        );
        for name in [
            "suite",
            "workspace",
            "source",
            "host",
            "version",
            "refresh",
            "copy",
            "reset",
            "patch",
            "payload",
        ] {
            eprintln!("Rust preparation staging rejection: {name}");
            let output = root.join(format!("stage-{name}"));
            let logs = output.join("preparation");
            fs::create_dir_all(&logs).unwrap();
            let copied = workspace.join("suite");
            let original_manifest = fs::read(suite.join("Cargo.toml")).unwrap();
            let mut removed = None;
            match name {
                "workspace" => {
                    fs::write(workspace.join("owner"), "unowned").unwrap();
                }
                "host" => fs::create_dir(logs.join("host-system")).unwrap(),
                "version" => fs::create_dir(logs.join("gleam-version")).unwrap(),
                "refresh" => {
                    fs::remove_dir_all(copied.join("project/src")).unwrap();
                    fs::write(copied.join("project/src"), b"blocker").unwrap();
                }
                "copy" => {
                    fs::remove_file(copied.join("Cargo.lock")).unwrap();
                    fs::create_dir(copied.join("Cargo.lock")).unwrap();
                }
                "reset" => {
                    let path = copied.join("project/Cargo.toml");
                    fs::remove_file(&path).unwrap();
                    fs::create_dir(&path).unwrap();
                }
                "patch" => fs::write(suite.join("Cargo.toml"), "invalid[").unwrap(),
                "payload" => fs::create_dir(output.join("payload")).unwrap(),
                _ => {}
            }
            let missing = root.join("missing");
            let preparation = Preparation {
                checkout: if name == "source" {
                    &missing
                } else {
                    &checkout
                },
                suite: if name == "suite" { &missing } else { &suite },
                backends: &backends,
                output: &output,
                logs: &logs,
                build_directory: Some(&workspace),
            };
            assert!(preparation.build(&active).is_err(), "{name}");
            assert!(!output.join("bundle.json").exists());
            match name {
                "workspace" => {
                    fs::write(workspace.join("owner"), "geam-bench build workspace v1\n").unwrap()
                }
                "refresh" => {
                    fs::remove_file(copied.join("project/src")).unwrap();
                    removed = Some("project/src");
                }
                "copy" => {
                    fs::remove_dir(copied.join("Cargo.lock")).unwrap();
                    fs::copy(suite.join("Cargo.lock"), copied.join("Cargo.lock")).unwrap();
                }
                "reset" => fs::remove_dir(copied.join("project/Cargo.toml")).unwrap(),
                "patch" => fs::write(suite.join("Cargo.toml"), original_manifest).unwrap(),
                _ => {}
            }
            if let Some(relative) = removed {
                copy_files(
                    &suite.join(relative),
                    &copied.join(relative),
                    files(&suite.join(relative)).unwrap().into_keys(),
                )
                .unwrap();
            }
        }
        let javascript_only = root.join("javascript-only");
        super::execute(
            crate::Preparation {
                build_directory: Some(workspace.clone()),
                checkout: checkout.clone(),
                suite: suite.clone(),
                geam: None,
                output: javascript_only.clone(),
                targets: vec![Target::JavaScript],
            },
            &active,
        )
        .unwrap();
        let javascript: Manifest =
            serde_json::from_slice(&fs::read(javascript_only.join("bundle.json")).unwrap())
                .unwrap();
        assert_eq!(
            javascript.targets,
            std::collections::BTreeSet::from([Target::JavaScript])
        );
        assert_eq!(javascript.build.geam_cli_sha256, None);
        assert_eq!(javascript.build.application_lock, None);
        assert!(javascript.build.native_dependencies.is_empty());
        for name in ["rustc", "cargo", "macos_sdk"] {
            assert!(!javascript.build.tools.contains_key(name));
        }
        // Restage once, then test build-tool failure receipts against that real
        // project. Successful compilation above covers the same backend owner.
        let copied = workspace.join("suite");
        stage_suite(&suite, &copied, &checkout, &suite_inputs(&suite).unwrap()).unwrap();
        let project = copied.join("project");
        for name in [
            "cli",
            "rustc-version",
            "cargo-version",
            "provider-add",
            "geam-build",
            "erlang-export",
            "javascript-build",
        ] {
            eprintln!("Rust compilation rejection: {name}");
            let logs = root.join(format!("compile-{name}"));
            fs::create_dir(&logs).unwrap();
            let missing = root.join("missing-cli");
            let backend = match name {
                "cli" => Backend::Geam(&missing),
                "erlang-export" => Backend::Erlang,
                "javascript-build" => Backend::JavaScript,
                _ => Backend::Geam(&cli),
            };
            if name != "cli" {
                fs::create_dir(logs.join(name)).unwrap();
            }
            assert!(
                compile_backend(
                    &backend,
                    &project,
                    &logs,
                    &mut manifest.build.clone(),
                    &active
                )
                .is_err(),
                "{name}"
            );
        }
        for name in [
            "compiled",
            "geam-directory",
            "geam-copy",
            "lock",
            "native-format",
            "native-libraries",
            "erlang-inventory",
            "erlang-copy",
            "javascript-inventory",
            "javascript-copy",
            "javascript-entry",
        ] {
            eprintln!("Rust artifact packaging rejection: {name}");
            let output = root.join(format!("package-{name}"));
            let payload = output.join("payload");
            let logs = output.join("logs");
            fs::create_dir_all(&payload).unwrap();
            fs::create_dir(&logs).unwrap();
            let target = if name.starts_with("erlang") {
                Target::Erlang
            } else if name.starts_with("javascript") {
                Target::JavaScript
            } else {
                Target::Geam
            };
            let mut moved = None;
            let missing = root.join("missing-project");
            match name {
                "geam-directory" => fs::create_dir(payload.join("geam")).unwrap(),
                "lock" => moved = Some(project.join("Cargo.lock")),
                "native-format" | "native-libraries" => fs::create_dir(logs.join(name)).unwrap(),
                "erlang-inventory" => moved = Some(project.join("build/erlang-shipment")),
                "javascript-inventory" => moved = Some(project.join("build/dev/javascript")),
                "erlang-copy" => fs::write(payload.join("erlang"), b"blocker").unwrap(),
                "javascript-copy" => fs::write(payload.join("javascript"), b"blocker").unwrap(),
                "javascript-entry" => {
                    fs::create_dir_all(payload.join("javascript/main.mjs")).unwrap()
                }
                _ => {}
            }
            let saved = output.join("saved");
            if let Some(path) = &moved {
                fs::rename(path, &saved).unwrap();
            }
            let result = package_backend(
                target,
                if name == "compiled" {
                    Err(std::io::Error::other("compiler failed").into())
                } else {
                    Ok(())
                },
                if name == "geam-copy" {
                    &missing
                } else {
                    &project
                },
                &payload,
                &logs,
                &mut manifest.build.clone(),
                &active,
            );
            if let Some(path) = moved {
                fs::rename(saved, path).unwrap();
            }
            assert!(result.is_err(), "{name}");
        }
        #[cfg(target_os = "macos")]
        {
            let logs = root.join("native-load");
            fs::create_dir_all(logs.join("native-load-commands")).unwrap();
            assert!(
                super::native_dependencies(
                    &project.join("build/geam/target/release/geam_benchmarks"),
                    &logs,
                    &active
                )
                .is_err()
            );
        }
        let phases = ["compiler", "runtime", "node-pin", "geam-compiler"].into_iter();
        #[cfg(target_os = "macos")]
        let phases = phases.chain(["sdk"]);
        for name in phases {
            let output = root.join(format!("compile-phase-{name}"));
            let logs = output.join("preparation");
            fs::create_dir_all(&logs).unwrap();
            let target = if name == "sdk" || name == "geam-compiler" {
                Backend::Geam(&cli)
            } else {
                Backend::JavaScript
            };
            let selected = [target];
            let mut staged = Preparation {
                checkout: &checkout,
                suite: &suite,
                backends: &selected,
                output: &output,
                logs: &logs,
                build_directory: Some(&workspace),
            }
            .stage(&active)
            .unwrap();
            let pin = fs::read(suite.join(".node-version")).unwrap();
            let blocked = match name {
                "compiler" => "javascript-build",
                "runtime" => "runtime-javascript",
                "geam-compiler" => "geam-build",
                _ => "sdk-version",
            };
            if name == "node-pin" {
                fs::write(suite.join(".node-version"), "wrong\n").unwrap();
            } else {
                fs::create_dir(logs.join(blocked)).unwrap();
            }
            assert!(staged.compile(&active).is_err(), "{name}");
            assert!(!output.join("bundle.json").exists());
            fs::write(suite.join(".node-version"), pin).unwrap();
        }
        // Build's compile-result propagation is tested with an actual rejected
        // runtime pin, while the complete successful path used all three tools.
        let pin = fs::read(suite.join(".node-version")).unwrap();
        fs::write(suite.join(".node-version"), "wrong\n").unwrap();
        let output = root.join("build-rejected-runtime");
        let logs = output.join("preparation");
        fs::create_dir_all(&logs).unwrap();
        assert!(
            Preparation {
                checkout: &checkout,
                suite: &suite,
                backends: &[Backend::JavaScript],
                output: &output,
                logs: &logs,
                build_directory: Some(&workspace)
            }
            .build(&active)
            .is_err()
        );
        fs::write(suite.join(".node-version"), pin).unwrap();
        for name in [
            "logs",
            "source-query",
            "source-change",
            "suite-read",
            "suite-change",
            "inventory",
            "manifest",
            "seal",
        ] {
            eprintln!("Rust bundle acceptance rejection: {name}");
            let output = root.join(format!("seal-{name}"));
            let logs = output.join("preparation");
            fs::create_dir_all(&logs).unwrap();
            let mut staged = Preparation {
                checkout: &checkout,
                suite: &suite,
                backends: &backends,
                output: &output,
                logs: &logs,
                build_directory: Some(&workspace),
            }
            .stage(&active)
            .unwrap();
            copy_files(
                &baseline.join("payload"),
                &output.join("payload"),
                files(&baseline.join("payload")).unwrap().into_keys(),
            )
            .unwrap();
            staged.build = manifest.build.clone();
            staged.runtimes = manifest.runtimes.clone();
            let checkout_manifest = fs::read(checkout.join("Cargo.toml")).unwrap();
            let suite_manifest = fs::read(suite.join("Cargo.toml")).unwrap();
            match name {
                "logs" => fs::create_dir(logs.join("source-after")).unwrap(),
                "source-change" => {
                    let mut changed = checkout_manifest.clone();
                    changed.extend_from_slice(b"\n# changed during preparation\n");
                    fs::write(checkout.join("Cargo.toml"), changed).unwrap();
                }
                "suite-read" => fs::remove_file(suite.join("Cargo.toml")).unwrap(),
                "suite-change" => {
                    let mut changed = suite_manifest.clone();
                    changed.extend_from_slice(b"\n# changed during preparation\n");
                    fs::write(suite.join("Cargo.toml"), changed).unwrap();
                }
                "inventory" => {
                    std::os::unix::fs::symlink(&baseline, output.join("payload/link")).unwrap()
                }
                "manifest" => fs::remove_file(output.join("payload/javascript/main.mjs")).unwrap(),
                "seal" => fs::create_dir(output.join("bundle.json")).unwrap(),
                _ => {}
            }
            let result = staged.seal(&AtomicBool::new(name == "source-query"));
            fs::write(checkout.join("Cargo.toml"), checkout_manifest).unwrap();
            fs::write(suite.join("Cargo.toml"), suite_manifest).unwrap();
            assert!(result.is_err(), "{name}");
            assert!(!output.join("bundle.json").is_file(), "{name}");
        }
    }
}
