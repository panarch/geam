use std::fs;
use std::os::unix::fs::symlink;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

#[path = "support/cli_failures.rs"]
mod cli_failures;

#[test]
fn public_workflow_prepares_relocates_validates_and_reconstructs_the_maintained_suite() {
    let suite = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap();
    let repository = suite.parent().unwrap();
    let cli = std::env::var_os("GEAM_BENCH_TEST_GEAM")
        .map(PathBuf::from)
        .unwrap_or_else(|| repository.join("target/debug/geam"));
    assert!(
        cli.is_file(),
        "build the checkout CLI or set GEAM_BENCH_TEST_GEAM"
    );
    let temporary = tempfile::tempdir().unwrap();
    let root = temporary.path();
    let checkout = root.join("checkout");
    let cloned = Command::new("git")
        .args(["clone", "--quiet", "--shared"])
        .arg(repository)
        .arg(&checkout)
        .output()
        .unwrap();
    assert!(
        cloned.status.success(),
        "{}",
        String::from_utf8_lossy(&cloned.stderr)
    );
    // A local review must exercise tracked working-tree changes as well as HEAD.
    // CI normally has an empty patch; --allow-empty keeps that same path valid.
    let patch = Command::new("git")
        .current_dir(repository)
        .args(["diff", "--binary", "HEAD"])
        .output()
        .unwrap();
    assert!(patch.status.success());
    let patch_path = root.join("checkout.patch");
    fs::write(&patch_path, patch.stdout).unwrap();
    let applied = Command::new("git")
        .current_dir(&checkout)
        .args(["apply", "--allow-empty"])
        .arg(&patch_path)
        .output()
        .unwrap();
    assert!(
        applied.status.success(),
        "{}",
        String::from_utf8_lossy(&applied.stderr)
    );
    let inputs = root.join("suite");
    copy_source(suite, &inputs);
    // Owned output inside the checkout must not contaminate source provenance.
    let bundle = checkout.join("benchmark-bundle");
    let workspace = root.join("build-workspace");
    let runner = Path::new(env!("CARGO_BIN_EXE_geam-bench"));
    let prepared = Command::new(runner)
        .arg("prepare")
        .arg("--checkout")
        .arg(&checkout)
        .arg("--suite")
        .arg(&inputs)
        .arg("--geam")
        .arg(cli.canonicalize().unwrap())
        .arg("--build-directory")
        .arg(&workspace)
        .arg("--output")
        .arg(&bundle)
        .output()
        .unwrap();
    assert!(
        prepared.status.success(),
        "{}\n{}\nprovider add: {}\nnative build: {}",
        String::from_utf8_lossy(&prepared.stdout),
        String::from_utf8_lossy(&prepared.stderr),
        fs::read_to_string(bundle.join("preparation/provider-add/stderr")).unwrap_or_default(),
        fs::read_to_string(bundle.join("preparation/geam-build/stderr")).unwrap_or_default()
    );
    let relocated = root.join("relocated bundle");
    fs::rename(&bundle, &relocated).unwrap();
    let reused_geam = root.join("reused-geam");
    let output = Command::new(runner)
        .args(["prepare", "--checkout"])
        .arg(&checkout)
        .arg("--suite")
        .arg(&inputs)
        .arg("--geam")
        .arg(cli.canonicalize().unwrap())
        .args(["--targets", "geam", "--build-directory"])
        .arg(&workspace)
        .arg("--output")
        .arg(&reused_geam)
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(reused_geam.join("payload/geam/program").is_file());
    let refreshed = root.join("javascript-only");
    let output = Command::new(runner)
        .args(["prepare", "--checkout"])
        .arg(&checkout)
        .arg("--suite")
        .arg(&inputs)
        .args(["--targets", "javascript", "--build-directory"])
        .arg(&workspace)
        .arg("--output")
        .arg(&refreshed)
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let metadata: serde_json::Value =
        serde_json::from_slice(&fs::read(refreshed.join("bundle.json")).unwrap()).unwrap();
    assert_eq!(metadata["targets"], serde_json::json!(["javascript"]));
    assert_eq!(
        metadata["build"]["geam_cli_sha256"],
        serde_json::Value::Null
    );
    assert!(metadata["build"]["tools"].get("cargo").is_none());
    let node_version = fs::read(inputs.join(".node-version")).unwrap();
    fs::write(inputs.join(".node-version"), "0.0.0\n").unwrap();
    let rejected = root.join("wrong-node-version");
    let output = Command::new(runner)
        .args(["prepare", "--checkout"])
        .arg(&checkout)
        .arg("--suite")
        .arg(&inputs)
        .args(["--targets", "javascript", "--output"])
        .arg(&rejected)
        .output()
        .unwrap();
    assert!(!output.status.success());
    assert_eq!(
        fs::read_to_string(rejected.join("failure.txt")).unwrap(),
        "Node version does not match the suite's .node-version"
    );
    assert!(!rejected.join("bundle.json").exists());
    fs::write(inputs.join(".node-version"), node_version).unwrap();
    cli_failures::Preparation {
        runner,
        checkout: &checkout,
        suite: &inputs,
        geam: &cli.canonicalize().unwrap(),
        workspace: &workspace,
        root: &root.join("prepare-failures"),
    }
    .verify();
    fs::remove_dir_all(&checkout).unwrap();
    fs::remove_dir_all(&inputs).unwrap();
    fs::remove_dir_all(&workspace).unwrap();
    let controller = relocated.join("payload/controller");
    let runtimes = root.join("runtimes");
    fs::create_dir(&runtimes).unwrap();
    for program in ["node", "erl"] {
        symlink(resolve(program), runtimes.join(program)).unwrap();
    }
    let path = std::env::join_paths([runtimes.as_path(), Path::new("/usr/bin"), Path::new("/bin")])
        .unwrap();
    for compiler in ["cargo", "rustc", "gleam"] {
        assert!(
            !std::env::split_paths(&path).any(|directory| directory.join(compiler).is_file()),
            "{compiler} must be absent from execution PATH"
        );
    }
    let manifest_path = relocated.join("bundle.json");
    let original_manifest = fs::read(&manifest_path).unwrap();
    let mut manifest: serde_json::Value = serde_json::from_slice(&original_manifest).unwrap();
    manifest["runtimes"]["geam"] = serde_json::json!("incompatible runtime");
    fs::write(&manifest_path, manifest.to_string()).unwrap();
    let output = Command::new(&controller)
        .args(["run", "--bundle"])
        .arg(&relocated)
        .args(["--targets", "geam", "--output"])
        .arg(root.join("runtime-mismatch"))
        .env("PATH", &path)
        .output()
        .unwrap();
    assert!(!output.status.success());
    assert!(
        fs::read_to_string(root.join("runtime-mismatch/failure.txt"))
            .unwrap()
            .contains("runtime differs from bundle preparation")
    );
    assert!(!root.join("runtime-mismatch/complete.json").exists());
    manifest = serde_json::from_slice(&original_manifest).unwrap();
    manifest["targets"] = serde_json::json!(["geam"]);
    manifest["runtimes"]
        .as_object_mut()
        .unwrap()
        .retain(|name, _| name == "geam");
    fs::write(&manifest_path, manifest.to_string()).unwrap();
    let missing = root.join("missing-target");
    let output = Command::new(&controller)
        .args(["run", "--bundle"])
        .arg(&relocated)
        .args(["--targets", "erlang", "--output"])
        .arg(&missing)
        .env("PATH", &path)
        .output()
        .unwrap();
    assert!(!output.status.success());
    assert!(!missing.exists());
    fs::write(&manifest_path, original_manifest).unwrap();
    let smoke = root.join("smoke");
    let output = Command::new(&controller)
        .arg("run")
        .arg("--bundle")
        .arg(&relocated)
        .arg("--output")
        .arg(&smoke)
        .env("PATH", &path)
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(
        fs::read_to_string(smoke.join("raw.jsonl"))
            .unwrap()
            .lines()
            .count(),
        486
    );
    let summary: Vec<serde_json::Value> =
        serde_json::from_slice(&fs::read(smoke.join("summary.json")).unwrap()).unwrap();
    assert_eq!(summary.len(), 162);
    assert_eq!(
        summary
            .iter()
            .map(|item| item["samples"].as_u64().unwrap())
            .sum::<u64>(),
        486
    );
    let paired = root.join("paired");
    let output = Command::new(&controller)
        .args(["compare", "--baseline"])
        .arg(&relocated)
        .arg("--candidate")
        .arg(&relocated)
        .args(["--case", "callback_control/1,bit_checksum/100", "--output"])
        .arg(&paired)
        .env("PATH", &path)
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(
        fs::read_to_string(paired.join("raw.jsonl"))
            .unwrap()
            .lines()
            .count(),
        48
    );
    cli_failures::execution(runner, &relocated, &reused_geam, &root.join("run-failures"));
    let empty_path = root.join("no-tools");
    fs::create_dir(&empty_path).unwrap();
    let geam_only = root.join("geam-only");
    let output = Command::new(&controller)
        .args(["run", "--bundle"])
        .arg(&relocated)
        .args([
            "--targets",
            "geam",
            "--case",
            "callback_control/1",
            "--output",
        ])
        .arg(&geam_only)
        .env("PATH", &empty_path)
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(
        fs::read_to_string(geam_only.join("raw.jsonl"))
            .unwrap()
            .lines()
            .count(),
        3
    );
    for (input, name) in [
        (&smoke, "smoke-analysis"),
        (&paired, "paired-analysis"),
        (&geam_only, "single-analysis"),
    ] {
        let output = root.join(name);
        let analyzed = Command::new(&controller)
            .arg("analyze")
            .arg("--input")
            .arg(input)
            .arg("--output")
            .arg(&output)
            .env("PATH", &empty_path)
            .output()
            .unwrap();
        assert!(
            analyzed.status.success(),
            "{}",
            String::from_utf8_lossy(&analyzed.stderr)
        );
        assert_eq!(
            fs::read(input.join("summary.json")).unwrap(),
            fs::read(output.join("summary.json")).unwrap()
        );
        assert_eq!(
            fs::read(input.join("comparisons.json")).unwrap(),
            fs::read(output.join("comparisons.json")).unwrap()
        );
        assert_eq!(
            fs::read(input.join("REPORT.md")).unwrap(),
            fs::read(output.join("REPORT.md")).unwrap()
        );
    }
    let collision = Command::new(&controller)
        .arg("analyze")
        .arg("--input")
        .arg(&smoke)
        .arg("--output")
        .arg(root.join("smoke-analysis"))
        .env("PATH", &empty_path)
        .output()
        .unwrap();
    assert!(!collision.status.success());
    let cancelled = root.join("cancelled");
    let mut child = Command::new(&controller)
        .args(["run", "--bundle"])
        .arg(&relocated)
        .args(["--targets", "geam", "--output"])
        .arg(&cancelled)
        .env("PATH", &empty_path)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .unwrap();
    let deadline = Instant::now() + Duration::from_secs(90);
    while !cancelled.join("processes/000000/start.json").exists() {
        assert!(
            child.try_wait().unwrap().is_none(),
            "run must reach the declared process boundary"
        );
        assert!(Instant::now() < deadline, "waiting for process admission");
        std::thread::sleep(Duration::from_millis(1));
    }
    nix::sys::signal::kill(
        nix::unistd::Pid::from_raw(child.id() as i32),
        nix::sys::signal::Signal::SIGTERM,
    )
    .unwrap();
    assert!(!child.wait().unwrap().success());
    assert!(!cancelled.join("complete.json").exists());
    assert!(cancelled.join("failure.txt").is_file());
    let interrupted = Command::new(&controller)
        .args(["analyze", "--input"])
        .arg(&cancelled)
        .arg("--output")
        .arg(root.join("interrupted-analysis"))
        .env("PATH", &empty_path)
        .output()
        .unwrap();
    assert!(!interrupted.status.success());
    let raw = smoke.join("raw.jsonl");
    let original = fs::read(&raw).unwrap();
    fs::write(&raw, b"{}\n").unwrap();
    let corrupt = Command::new(&controller)
        .args(["analyze", "--input"])
        .arg(&smoke)
        .arg("--output")
        .arg(root.join("corrupt-analysis"))
        .env("PATH", &empty_path)
        .output()
        .unwrap();
    assert!(!corrupt.status.success());
    fs::write(raw, original).unwrap();
    let payload = relocated.join("payload/geam/program");
    let original = fs::read(&payload).unwrap();
    fs::write(&payload, b"changed executable").unwrap();
    let corrupt = Command::new(&controller)
        .args(["run", "--bundle"])
        .arg(&relocated)
        .arg("--output")
        .arg(root.join("corrupt-bundle"))
        .env("PATH", &empty_path)
        .output()
        .unwrap();
    assert!(!corrupt.status.success());
    fs::write(payload, original).unwrap();
    let cases = Command::new(&controller).arg("cases").output().unwrap();
    assert!(cases.status.success());
    assert_eq!(String::from_utf8(cases.stdout).unwrap().lines().count(), 54);
}

fn copy_source(source: &Path, destination: &Path) {
    fs::create_dir(destination).unwrap();
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
        let target = destination.join(name);
        fs::create_dir_all(target.parent().unwrap()).unwrap();
        fs::copy(source.join(name), target).unwrap();
    }
    for name in ["project/src", "provider/src", "runner/src"] {
        copy_directory(&source.join(name), &destination.join(name));
    }
}

fn copy_directory(source: &Path, destination: &Path) {
    fs::create_dir(destination).unwrap();
    for entry in fs::read_dir(source).unwrap() {
        let entry = entry.unwrap();
        let target = destination.join(entry.file_name());
        if entry.file_type().unwrap().is_dir() {
            copy_directory(&entry.path(), &target);
        } else {
            fs::copy(entry.path(), target).unwrap();
        }
    }
}

fn resolve(program: &str) -> PathBuf {
    std::env::split_paths(&std::env::var_os("PATH").unwrap())
        .map(|directory| directory.join(program))
        .find(|path| path.is_file())
        .unwrap()
}
