//! Failure admission uses the real CLI, suite and compiled payloads.
use std::fs;
use std::path::Path;
use std::process::{Command, Output};

pub(super) struct Preparation<'a> {
    pub(super) runner: &'a Path,
    pub(super) checkout: &'a Path,
    pub(super) suite: &'a Path,
    pub(super) geam: &'a Path,
    pub(super) workspace: &'a Path,
    pub(super) root: &'a Path,
}

impl Preparation<'_> {
    fn command(&self, target: &str, output: &Path) -> Command {
        let mut command = Command::new(self.runner);
        command
            .args(["prepare", "--checkout"])
            .arg(self.checkout)
            .arg("--suite")
            .arg(self.suite)
            .arg("--geam")
            .arg(self.geam)
            .arg("--build-directory")
            .arg(self.workspace)
            .arg("--targets")
            .arg(target)
            .arg("--output")
            .arg(output);
        command
    }

    pub(super) fn verify(&self) {
        fs::create_dir_all(self.root).unwrap();
        for (name, targets, expected) in [
            (
                "duplicate-targets",
                "javascript,javascript",
                "prepare requires unique, nonempty targets",
            ),
            (
                "required-cli",
                "geam",
                "--geam is required when preparing Geam",
            ),
        ] {
            let output = self.root.join(name);
            let result = Command::new(self.runner)
                .args(["prepare", "--checkout"])
                .arg(self.checkout)
                .arg("--suite")
                .arg(self.suite)
                .args(["--targets", targets, "--output"])
                .arg(&output)
                .output()
                .unwrap();
            rejected(&result, &output, "bundle.json");
            assert!(String::from_utf8_lossy(&result.stderr).contains(expected));
            assert!(!output.exists());
        }
        for (name, field) in [
            ("missing-checkout", "--checkout"),
            ("missing-suite", "--suite"),
            ("missing-cli", "--geam"),
        ] {
            let output = self.root.join(name);
            let mut command = Command::new(self.runner);
            command
                .args(["prepare", "--targets", "javascript", "--output"])
                .arg(&output);
            for (option, value) in [
                ("--checkout", self.checkout),
                ("--suite", self.suite),
                ("--geam", self.geam),
            ] {
                command
                    .arg(option)
                    .arg(if option == field { &output } else { value });
            }
            rejected(&command.output().unwrap(), &output, "bundle.json");
            assert!(!output.exists());
        }
        let output = self.root.join("collision");
        fs::create_dir(&output).unwrap();
        rejected(
            &self.command("javascript", &output).output().unwrap(),
            &output,
            "bundle.json",
        );
    }
}

pub(super) fn execution(runner: &Path, bundle: &Path, candidate: &Path, root: &Path) {
    fs::create_dir_all(root).unwrap();
    let command = |output: &Path, target: &str| {
        let mut command = Command::new(runner);
        command
            .args(["run", "--bundle"])
            .arg(bundle)
            .args([
                "--case",
                "callback_control/1",
                "--targets",
                target,
                "--output",
            ])
            .arg(output);
        command
    };
    for (name, args) in [
        ("case", vec!["--case", "missing/1"]),
        ("duplicate-target", vec!["--targets", "geam,geam"]),
        ("empty-machine", vec!["--machine", " "]),
    ] {
        let output = root.join(name);
        let mut invalid = Command::new(runner);
        invalid
            .args(["run", "--bundle"])
            .arg(bundle)
            .arg("--output")
            .arg(&output)
            .args(args);
        rejected(&invalid.output().unwrap(), &output, "complete.json");
    }
    let output = root.join("collision");
    fs::create_dir(&output).unwrap();
    rejected(
        &command(&output, "geam").output().unwrap(),
        &output,
        "complete.json",
    );
    for (name, second) in [
        ("missing-candidate", root.join("missing")),
        ("incompatible-candidate", candidate.to_owned()),
    ] {
        let output = root.join(name);
        let manifest = candidate.join("bundle.json");
        let original = fs::read(&manifest).unwrap();
        if name == "incompatible-candidate" {
            let mut json: serde_json::Value = serde_json::from_slice(&original).unwrap();
            json["suite_sources"] = serde_json::json!({"different":"source"});
            fs::write(&manifest, json.to_string()).unwrap();
        }
        let result = Command::new(runner)
            .args(["compare", "--baseline"])
            .arg(bundle)
            .arg("--candidate")
            .arg(second)
            .arg("--output")
            .arg(&output)
            .output()
            .unwrap();
        fs::write(manifest, original).unwrap();
        rejected(&result, &output, "complete.json");
    }
}

fn rejected(result: &Output, output: &Path, marker: &str) {
    assert!(!result.status.success(), "{}: {result:?}", output.display());
    assert!(
        !output.join(marker).exists(),
        "false acceptance: {}",
        output.display()
    );
    assert!(!result.stderr.is_empty(), "{result:?}");
}
