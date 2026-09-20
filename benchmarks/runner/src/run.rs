use crate::bundle::{Bundle, Manifest, Target};
use crate::cases::{self, Case};
use crate::environment::{self, Host};
use crate::process::{self, Completion, Receipt, Status};
use crate::results::{self, Budget, Record};
use crate::{Execution, Result, invalid, new_directory, write_json_new};
use clap::ValueEnum;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use std::fs::{self, File};
use std::path::Path;
use std::process::Command;
use std::sync::atomic::AtomicBool;
use std::time::Duration;

#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
struct Configuration {
    schema: u32,
    mode: Mode,
    budget: Budget,
    selection: Selection,
    cases: Vec<Case>,
    host: Host,
    bundles: BTreeMap<Variant, Manifest>,
    timeout_seconds: u64,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize, ValueEnum)]
#[serde(rename_all = "snake_case")]
pub(super) enum Mode {
    Smoke,
    Baseline,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(
    tag = "kind",
    content = "targets",
    rename_all = "snake_case",
    deny_unknown_fields
)]
enum Selection {
    Targets(Vec<Target>),
    Paired,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub(super) enum Variant {
    Geam,
    Erlang,
    #[serde(rename = "javascript")]
    JavaScript,
    Baseline,
    Candidate,
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
struct Step {
    variant: Variant,
    round: usize,
    case: Case,
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct Accepted {
    schema: u32,
    files: BTreeMap<String, String>,
}

pub(super) fn execute(
    first: &Path,
    second: Option<&Path>,
    targets: Vec<Target>,
    arguments: Execution,
    cancelled: &AtomicBool,
) -> Result<()> {
    let first = Bundle::load(first)?;
    let second = second.map(Bundle::load).transpose()?;
    if let Some(other) = &second {
        first.manifest.compatible(&other.manifest)?;
    }
    let cases = cases::select(&arguments.cases)?;
    let (selection, inputs) = match &second {
        Some(other) => (
            Selection::Paired,
            BTreeMap::from([(Variant::Baseline, &first), (Variant::Candidate, other)]),
        ),
        None => {
            let selection = Selection::Targets(targets);
            let inputs = selection
                .variants()?
                .into_iter()
                .map(|variant| (variant, &first))
                .collect();
            (selection, inputs)
        }
    };
    let mut bundles = BTreeMap::new();
    for (variant, bundle) in &inputs {
        if !bundle.manifest.targets.contains(&variant.target()) {
            return Err(invalid(format!("missing target {}", variant.target().name())).into());
        }
        bundles.insert(*variant, bundle.manifest.clone());
    }
    let output = new_directory(&arguments.output)?;
    let configuration =
        Configuration::record(&output, arguments, selection, cases, bundles, cancelled);
    let outcome = measure(&output, configuration, &inputs, cancelled).and_then(|configuration| {
        let verified = first
            .verify()
            .and_then(|()| second.as_ref().map(Bundle::verify).unwrap_or(Ok(())));
        seal_run(&output, &configuration, verified)
    });
    crate::evidence::finish(outcome, &output, &mut std::io::stderr().lock())
}

fn measure(
    output: &Path,
    configuration: Result<Configuration>,
    inputs: &BTreeMap<Variant, &Bundle>,
    cancelled: &AtomicBool,
) -> Result<Configuration> {
    let configuration = configuration?;
    let processes = output.join("processes");
    fs::create_dir(&processes)?;
    let mut raw = File::create_new(output.join("raw.jsonl"))?;
    measure_steps(&processes, &configuration, inputs, &mut raw, cancelled)?;
    Ok(configuration)
}

fn measure_steps(
    processes: &Path,
    configuration: &Configuration,
    inputs: &BTreeMap<Variant, &Bundle>,
    raw: &mut dyn crate::evidence::DurableWrite,
    cancelled: &AtomicBool,
) -> Result<()> {
    let steps = configuration.steps()?;
    for (index, step) in steps.iter().enumerate() {
        eprintln!(
            "{}/{}: {} {}/{} round {}",
            index + 1,
            steps.len(),
            step.variant.name(),
            step.case.workload.name(),
            step.case.size,
            step.round
        );
        let mut command = inputs[&step.variant].command(step.variant.target())?;
        configure(&mut command, step.case, configuration.budget);
        let directory = processes.join(format!("{index:06}"));
        let completion = process::execute(
            &mut command,
            &directory,
            Duration::from_secs(configuration.timeout_seconds),
            cancelled,
        );
        step.record(&directory, configuration.budget, completion, raw)?;
    }
    Ok(())
}

impl Step {
    fn record(
        &self,
        directory: &Path,
        budget: Budget,
        completion: Result<Completion>,
        raw: &mut dyn crate::evidence::DurableWrite,
    ) -> Result<()> {
        let completion = completion?;
        write_json_new(&directory.join("step.json"), self)?;
        if completion.status != Status::Success {
            return Err(invalid(format!(
                "measurement process {:?}; receipts: {}",
                completion.status,
                directory.display()
            ))
            .into());
        }
        let samples =
            results::parse_samples(&fs::read(directory.join("stdout"))?, self.case, budget)?;
        crate::evidence::write_batch(
            raw,
            samples.into_iter().map(|sample| Record {
                variant: self.variant,
                round: self.round,
                sample,
            }),
        )
    }
}

fn seal_run(output: &Path, configuration: &Configuration, verified: Result<()>) -> Result<()> {
    verified?;
    let records = read_records(output, configuration)?;
    reports(output, &records, configuration.budget)?;
    let accepted = Accepted {
        schema: 1,
        files: raw_files(output)?,
    };
    crate::evidence::seal_json_new(&output.join("complete.json"), &accepted)?;
    eprintln!("Accepted {} samples: {}", records.len(), output.display());
    Ok(())
}

pub(super) fn analyze(input: &Path, output: &Path) -> Result<()> {
    let accepted: Accepted = serde_json::from_slice(&fs::read(input.join("complete.json"))?)?;
    if accepted.schema != 1 || accepted.files != raw_files(input)? {
        return Err(invalid("completed run evidence is missing, changed or unsupported").into());
    }
    let configuration: Configuration =
        serde_json::from_slice(&fs::read(input.join("configuration.json"))?)?;
    let records = read_records(input, &configuration)?;
    let output = new_directory(output)?;
    reconstruct(&output, &records, configuration.budget, &accepted)
}

fn reconstruct(
    output: &Path,
    records: &[Record],
    budget: Budget,
    accepted: &Accepted,
) -> Result<()> {
    reports(output, records, budget)?;
    write_json_new(&output.join("source.json"), accepted)?;
    eprintln!(
        "Reconstructed {} samples: {}",
        records.len(),
        output.display()
    );
    Ok(())
}

impl Configuration {
    fn record(
        output: &Path,
        arguments: Execution,
        selection: Selection,
        cases: Vec<Case>,
        bundles: BTreeMap<Variant, Manifest>,
        cancelled: &AtomicBool,
    ) -> Result<Self> {
        let logs = output.join("metadata");
        fs::create_dir(&logs)?;
        let host = environment::host(
            arguments
                .machine
                .unwrap_or_else(|| "smoke correctness verification".into()),
            &logs,
            cancelled,
        )?;
        let configuration = Self {
            schema: 1,
            mode: arguments.mode,
            budget: arguments
                .mode
                .budget(matches!(selection, Selection::Paired)),
            selection,
            cases,
            host,
            bundles,
            timeout_seconds: arguments.timeout_seconds,
        };
        configuration.steps()?;
        write_json_new(&output.join("configuration.json"), &configuration)?;
        configuration.verify_runtimes(&logs, cancelled)?;
        Ok(configuration)
    }

    fn verify_runtimes(&self, logs: &Path, cancelled: &AtomicBool) -> Result<()> {
        for (variant, bundle) in &self.bundles {
            let actual = variant
                .target()
                .runtime(&logs.join(format!("runtime-{}", variant.name())), cancelled)?;
            if bundle.runtimes.get(&variant.target()) != Some(&actual) {
                return Err(invalid(format!(
                    "{} runtime differs from bundle preparation; see metadata receipts",
                    variant.target().name()
                ))
                .into());
            }
        }
        Ok(())
    }

    fn steps(&self) -> Result<Vec<Step>> {
        let variants = self.selection.variants()?;
        let paired = matches!(self.selection, Selection::Paired);
        if self.schema != 1
            || self.budget != self.mode.budget(paired)
            || self.timeout_seconds == 0
            || self.cases.is_empty()
            || self.host.machine.trim().is_empty()
        {
            return Err(invalid(
                "unsupported run schema, budget, timeout, cases or host description",
            )
            .into());
        }
        let maintained = cases::suite(None);
        let mut unique = BTreeSet::new();
        for case in &self.cases {
            if !maintained.contains(case) || !unique.insert(*case) {
                return Err(
                    invalid("unknown or duplicate maintained case in run configuration").into(),
                );
            }
        }
        if self.bundles.keys().copied().collect::<BTreeSet<_>>()
            != variants.iter().copied().collect()
        {
            return Err(invalid("run bundle inventory does not match selected variants").into());
        }
        for (variant, manifest) in &self.bundles {
            manifest.validate()?;
            if !manifest.targets.contains(&variant.target()) {
                return Err(invalid("run variant is absent from its bundle").into());
            }
        }
        if paired {
            self.bundles[&Variant::Baseline].compatible(&self.bundles[&Variant::Candidate])?;
        }
        Ok(schedule(&self.cases, &variants, self.budget.rounds, paired))
    }
}

fn schedule(cases: &[Case], variants: &[Variant], rounds: usize, paired: bool) -> Vec<Step> {
    let mut steps = Vec::new();
    for round in 0..rounds {
        let mut cases = cases.to_vec();
        if paired && round % 2 == 1 {
            cases.reverse();
        }
        for case in cases {
            for position in 0..variants.len() {
                steps.push(Step {
                    variant: variants[(round + position) % variants.len()],
                    round: round + 1,
                    case,
                });
            }
        }
    }
    steps
}

impl Selection {
    fn variants(&self) -> Result<Vec<Variant>> {
        match self {
            Self::Paired => Ok(vec![Variant::Baseline, Variant::Candidate]),
            Self::Targets(targets) => {
                if targets.is_empty()
                    || targets.iter().collect::<BTreeSet<_>>().len() != targets.len()
                {
                    return Err(invalid("run requires unique, nonempty targets").into());
                }
                Ok(targets
                    .iter()
                    .map(|target| match target {
                        Target::Geam => Variant::Geam,
                        Target::Erlang => Variant::Erlang,
                        Target::JavaScript => Variant::JavaScript,
                    })
                    .collect())
            }
        }
    }
}

impl Mode {
    pub(super) fn budget(self, paired: bool) -> Budget {
        let mut budget = match self {
            Self::Smoke => Budget {
                warmup_ms: 20,
                sample_ms: 10,
                samples: 3,
                rounds: 1,
            },
            Self::Baseline => Budget {
                warmup_ms: 2000,
                sample_ms: 100,
                samples: 20,
                rounds: 3,
            },
        };
        if paired {
            budget.rounds = 4;
        }
        budget
    }
}

impl Variant {
    pub(super) fn name(self) -> &'static str {
        match self {
            Self::Geam => "geam",
            Self::Erlang => "erlang",
            Self::JavaScript => "javascript",
            Self::Baseline => "baseline",
            Self::Candidate => "candidate",
        }
    }

    fn target(self) -> Target {
        match self {
            Self::Geam | Self::Baseline | Self::Candidate => Target::Geam,
            Self::Erlang => Target::Erlang,
            Self::JavaScript => Target::JavaScript,
        }
    }
}

fn configure(command: &mut Command, case: Case, budget: Budget) {
    command
        .envs(measurement_environment(case, budget))
        .env_remove("GEAM_CONFIG");
}

fn measurement_environment(case: Case, budget: Budget) -> BTreeMap<String, String> {
    BTreeMap::from([
        ("GEAM_BENCH_WORKLOAD".into(), case.workload.name().into()),
        ("GEAM_BENCH_SIZE".into(), case.size.to_string()),
        ("GEAM_BENCH_WARMUP_MS".into(), budget.warmup_ms.to_string()),
        ("GEAM_BENCH_SAMPLE_MS".into(), budget.sample_ms.to_string()),
        ("GEAM_BENCH_SAMPLES".into(), budget.samples.to_string()),
    ])
}

fn check_attempts(
    attempts: std::io::Result<Vec<std::fs::DirEntry>>,
    expected: usize,
) -> Result<()> {
    if attempts?.len() != expected {
        return Err(invalid("process matrix is incomplete or contains unexpected attempts").into());
    }
    Ok(())
}

fn read_records(input: &Path, configuration: &Configuration) -> Result<Vec<Record>> {
    let steps = configuration.steps()?;
    let mut records = Vec::new();
    check_attempts(
        fs::read_dir(input.join("processes"))
            .and_then(Iterator::collect::<std::io::Result<Vec<_>>>),
        steps.len(),
    )?;
    for (index, expected) in steps.iter().enumerate() {
        let directory = input.join("processes").join(format!("{index:06}"));
        let step: Step = serde_json::from_slice(&fs::read(directory.join("step.json"))?)?;
        let completion: Completion =
            serde_json::from_slice(&fs::read(directory.join("finish.json"))?)?;
        let receipt: Receipt = serde_json::from_slice(&fs::read(directory.join("start.json"))?)?;
        if step != *expected
            || completion.status != Status::Success
            || completion.exit_code != Some(0)
            || receipt.timeout_ms != u128::from(configuration.timeout_seconds) * 1000
        {
            return Err(invalid(format!(
                "invalid process receipt at matrix position {index}"
            ))
            .into());
        }
        for (name, value) in measurement_environment(step.case, configuration.budget) {
            if receipt.environment.get(&name) != Some(&Some(value)) {
                return Err(invalid(format!(
                    "process environment disagrees with run configuration: {name}"
                ))
                .into());
            }
        }
        if receipt.environment.get("GEAM_CONFIG") != Some(&None) {
            return Err(invalid("process did not isolate GEAM_CONFIG").into());
        }
        let samples = results::parse_samples(
            &fs::read(directory.join("stdout"))?,
            step.case,
            configuration.budget,
        )?;
        records.extend(samples.into_iter().map(|sample| Record {
            variant: step.variant,
            round: step.round,
            sample,
        }));
    }
    let raw = fs::read(input.join("raw.jsonl"))?;
    let recorded = serde_json::Deserializer::from_slice(&raw)
        .into_iter::<Record>()
        .collect::<std::result::Result<Vec<_>, _>>()?;
    if records != recorded {
        return Err(invalid("raw observations disagree with the complete process matrix").into());
    }
    Ok(records)
}

fn reports(output: &Path, records: &[Record], budget: Budget) -> Result<()> {
    let analysis = results::summarize(records)?;
    write_json_new(&output.join("summary.json"), &analysis.summaries())?;
    write_json_new(&output.join("comparisons.json"), &analysis.comparisons())?;
    let text = results::markdown(&analysis, budget);
    crate::evidence::write_text_new(&output.join("REPORT.md"), &text)
}

fn raw_files(input: &Path) -> Result<BTreeMap<String, String>> {
    let mut files = environment::files(input)?;
    for generated in [
        "complete.json",
        "summary.json",
        "comparisons.json",
        "REPORT.md",
    ] {
        files.remove(generated);
    }
    Ok(files)
}

#[cfg(test)]
pub(super) mod tests {
    use super::{Mode, Selection, Variant, measurement_environment, schedule};
    use crate::bundle::Target;
    use crate::cases::{Case, Workload};
    use crate::results::Budget;

    #[test]
    fn declared_modes_keep_the_existing_budgets_and_balanced_pairing() {
        assert_eq!(
            Mode::Smoke.budget(false),
            Budget {
                warmup_ms: 20,
                sample_ms: 10,
                samples: 3,
                rounds: 1
            }
        );
        assert_eq!(
            Mode::Baseline.budget(false),
            Budget {
                warmup_ms: 2000,
                sample_ms: 100,
                samples: 20,
                rounds: 3
            }
        );
        assert_eq!(
            Mode::Baseline.budget(true),
            Budget {
                warmup_ms: 2000,
                sample_ms: 100,
                samples: 20,
                rounds: 4
            }
        );
        assert_eq!(Mode::Smoke.budget(true).rounds, 4);
        let cases = [
            Case {
                workload: Workload::CallbackControl,
                size: 1,
            },
            Case {
                workload: Workload::BitChecksum,
                size: 100,
            },
        ];
        let steps = schedule(&cases, &[Variant::Baseline, Variant::Candidate], 4, true);
        let observed = steps
            .iter()
            .map(|step| (step.round, step.case.workload.name(), step.variant.name()))
            .collect::<Vec<_>>();
        assert_eq!(
            observed,
            [
                (1, "callback_control", "baseline"),
                (1, "callback_control", "candidate"),
                (1, "bit_checksum", "baseline"),
                (1, "bit_checksum", "candidate"),
                (2, "bit_checksum", "candidate"),
                (2, "bit_checksum", "baseline"),
                (2, "callback_control", "candidate"),
                (2, "callback_control", "baseline"),
                (3, "callback_control", "baseline"),
                (3, "callback_control", "candidate"),
                (3, "bit_checksum", "baseline"),
                (3, "bit_checksum", "candidate"),
                (4, "bit_checksum", "candidate"),
                (4, "bit_checksum", "baseline"),
                (4, "callback_control", "candidate"),
                (4, "callback_control", "baseline")
            ]
        );
        let rotation = schedule(
            &cases[..1],
            &[Variant::Geam, Variant::Erlang, Variant::JavaScript],
            3,
            false,
        );
        assert_eq!(
            rotation.iter().map(|step| step.variant).collect::<Vec<_>>(),
            [
                Variant::Geam,
                Variant::Erlang,
                Variant::JavaScript,
                Variant::Erlang,
                Variant::JavaScript,
                Variant::Geam,
                Variant::JavaScript,
                Variant::Geam,
                Variant::Erlang
            ]
        );
        assert!(Selection::Targets(vec![]).variants().is_err());
        assert!(
            Selection::Targets(vec![Target::Geam, Target::Geam])
                .variants()
                .is_err()
        );
        assert_eq!(
            Selection::Targets(vec![Target::JavaScript])
                .variants()
                .unwrap(),
            [Variant::JavaScript]
        );
        assert_eq!(
            measurement_environment(cases[0], Mode::Smoke.budget(false))
                .get("GEAM_BENCH_SIZE")
                .map(String::as_str),
            Some("1")
        );
    }

    #[test]
    fn completed_matrix_requires_every_declared_receipt_and_exact_raw_order() {
        use super::{Configuration, read_records};
        use crate::cases;
        use serde_json::json;
        use std::fs;

        let root = tempfile::tempdir().unwrap();
        let directory = root.path().join("processes/000000");
        fs::create_dir_all(&directory).unwrap();
        let manifest = json!({
            "schema":1,"cases":cases::suite(None),"targets":["geam"],
            "files":{"controller":"controller-digest","geam/program":"program-digest"},
            "suite_sources":{"project/src/main.gleam":"source-digest"},
            "runtime_source":{"commit":"revision","changes":"","excluded_outputs":[],"files":{"src/lib.rs":"runtime-digest"}},
            "build":{"host":{"os":"linux","architecture":"aarch64","system":"build host","logical_cpus":4,"machine":"builder","overrides":{}},"tools":{"rustc":"pinned"},"geam_cli_sha256":"cli-digest","native_dependencies":{},"application_lock":"version = 4"},
            "runtimes":{"geam":"native"}
        });
        let configuration = json!({
            "schema":1,"mode":"smoke","budget":{"warmup_ms":20,"sample_ms":10,"samples":3,"rounds":1},
            "selection":{"kind":"targets","targets":["geam"]},
            "cases":[{"workload":"callback_control","size":1}],
            "host":{"os":"linux","architecture":"aarch64","system":"measurement host","logical_cpus":4,"machine":"test host","overrides":{}},
            "bundles":{"geam":manifest},"timeout_seconds":240
        });
        let parsed: Configuration = serde_json::from_value(configuration.clone()).unwrap();
        assert_eq!(serde_json::to_value(&parsed).unwrap(), configuration);
        let step =
            json!({"variant":"geam","round":1,"case":{"workload":"callback_control","size":1}});
        assert_eq!(
            serde_json::to_value(parsed.steps().unwrap()).unwrap(),
            json!([step])
        );
        let receipt = json!({"program":"/bundle/payload/geam/program","arguments":[],"directory":"/bundle/payload","environment":{"GEAM_BENCH_WORKLOAD":"callback_control","GEAM_BENCH_SIZE":"1","GEAM_BENCH_WARMUP_MS":"20","GEAM_BENCH_SAMPLE_MS":"10","GEAM_BENCH_SAMPLES":"3","GEAM_CONFIG":null},"started_unix_ms":1,"timeout_ms":240000});
        let finish = json!({"status":"success","exit_code":0,"elapsed_ms":50});
        let samples = (1..=3).map(|sample| json!({"workload":"callback_control","size":1,"iterations":10,"warmup_elapsed_ns":20000000,"warmup_iterations":10,"sample":sample,"elapsed_ns":sample*100,"checksum":1})).collect::<Vec<_>>();
        let stdout = samples
            .iter()
            .map(|sample| format!("{sample}\n"))
            .collect::<String>();
        let raw = samples
            .iter()
            .map(|sample| format!("{}\n", json!({"variant":"geam","round":1,"sample":sample})))
            .collect::<String>();
        let originals = [
            ("step.json", step.to_string()),
            ("start.json", receipt.to_string()),
            ("finish.json", finish.to_string()),
            ("stdout", stdout),
        ];
        for (name, bytes) in &originals {
            fs::write(directory.join(name), bytes).unwrap();
        }
        fs::write(root.path().join("raw.jsonl"), &raw).unwrap();
        let records = read_records(root.path(), &parsed).unwrap();
        assert_eq!(
            serde_json::to_value(&records).unwrap(),
            json!([
                {"variant":"geam","round":1,"sample":samples[0]},
                {"variant":"geam","round":1,"sample":samples[1]},
                {"variant":"geam","round":1,"sample":samples[2]}
            ])
        );
        fs::remove_file(root.path().join("raw.jsonl")).unwrap();
        assert!(read_records(root.path(), &parsed).is_err());
        fs::write(root.path().join("raw.jsonl"), &raw).unwrap();
        fs::rename(
            root.path().join("processes"),
            root.path().join("unavailable-processes"),
        )
        .unwrap();
        assert!(read_records(root.path(), &parsed).is_err());
        fs::rename(
            root.path().join("unavailable-processes"),
            root.path().join("processes"),
        )
        .unwrap();
        let reports_root = tempfile::tempdir().unwrap();
        assert!(super::reports(reports_root.path(), &[], parsed.budget).is_err());
        for file in ["summary.json", "comparisons.json", "REPORT.md"] {
            let output = reports_root.path().join(file);
            fs::create_dir(&output).unwrap();
            fs::create_dir(output.join(file)).unwrap();
            assert!(super::reports(&output, &records, parsed.budget).is_err());
        }
        let config_path = root.path().join("configuration.json");
        fs::write(&config_path, configuration.to_string()).unwrap();
        let seal = root.path().join("complete.json");
        let accepted = json!({"schema":1, "files":super::raw_files(root.path()).unwrap()});
        fs::write(&seal, accepted.to_string()).unwrap();
        let reconstructed = reports_root.path().join("reconstructed");
        super::analyze(root.path(), &reconstructed).unwrap();
        assert_eq!(
            serde_json::from_slice::<serde_json::Value>(
                &fs::read(reconstructed.join("source.json")).unwrap()
            )
            .unwrap(),
            accepted,
        );
        assert!(super::analyze(root.path(), &reconstructed).is_err());
        fs::remove_file(&seal).unwrap();
        assert!(super::analyze(root.path(), &reports_root.path().join("missing-seal")).is_err());
        let mut unsupported = accepted.clone();
        unsupported["schema"] = json!(2);
        fs::write(&seal, unsupported.to_string()).unwrap();
        assert_eq!(
            super::analyze(root.path(), &reports_root.path().join("unsupported-seal"))
                .unwrap_err()
                .to_string(),
            "completed run evidence is missing, changed or unsupported",
        );
        fs::write(&seal, accepted.to_string()).unwrap();
        fs::write(&seal, "invalid JSON").unwrap();
        assert!(super::analyze(root.path(), &reports_root.path().join("invalid-seal")).is_err());
        for (name, bytes) in [
            ("parse", "invalid JSON".to_owned()),
            ("admission", {
                let mut changed = configuration.clone();
                changed["schema"] = json!(2);
                changed.to_string()
            }),
        ] {
            fs::write(&config_path, bytes).unwrap();
            let accepted = json!({"schema":1, "files":super::raw_files(root.path()).unwrap()});
            fs::write(&seal, accepted.to_string()).unwrap();
            assert!(super::analyze(root.path(), &reports_root.path().join(name)).is_err());
        }
        fs::write(&config_path, configuration.to_string()).unwrap();
        for (file, field, value) in [
            ("step.json", "round", json!(2)),
            ("finish.json", "status", json!("timed_out")),
            ("finish.json", "exit_code", json!(7)),
            ("start.json", "timeout_ms", json!(1)),
            ("start.json", "environment", json!({})),
        ] {
            let path = directory.join(file);
            let original = fs::read(&path).unwrap();
            let mut changed: serde_json::Value = serde_json::from_slice(&original).unwrap();
            changed[field] = value;
            fs::write(&path, changed.to_string()).unwrap();
            assert!(
                read_records(root.path(), &parsed).is_err(),
                "{file}:{field}"
            );
            fs::write(path, original).unwrap();
        }
        let mut unisolated = receipt.clone();
        unisolated["environment"]["GEAM_CONFIG"] = json!("ambient config");
        fs::write(directory.join("start.json"), unisolated.to_string()).unwrap();
        assert_eq!(
            read_records(root.path(), &parsed).unwrap_err().to_string(),
            "process did not isolate GEAM_CONFIG"
        );
        fs::write(directory.join("start.json"), receipt.to_string()).unwrap();
        for (file, original) in &originals {
            fs::remove_file(directory.join(file)).unwrap();
            assert!(
                read_records(root.path(), &parsed).is_err(),
                "missing {file}"
            );
            fs::write(directory.join(file), "invalid JSON").unwrap();
            assert!(
                read_records(root.path(), &parsed).is_err(),
                "invalid {file}"
            );
            fs::write(directory.join(file), original).unwrap();
        }
        for wrong in [
            "invalid JSON".to_owned(),
            String::new(),
            raw.lines().rev().collect::<Vec<_>>().join("\n"),
            format!("{raw}{raw}"),
        ] {
            fs::write(root.path().join("raw.jsonl"), wrong).unwrap();
            assert!(read_records(root.path(), &parsed).is_err());
        }
        fs::write(root.path().join("raw.jsonl"), &raw).unwrap();
        fs::create_dir(root.path().join("processes/unexpected")).unwrap();
        assert_eq!(
            read_records(root.path(), &parsed).unwrap_err().to_string(),
            "process matrix is incomplete or contains unexpected attempts"
        );
        fs::remove_dir(root.path().join("processes/unexpected")).unwrap();

        for (field, value) in [
            ("schema", json!(2)),
            ("selection", json!({"kind":"targets","targets":[]})),
            ("bundles", {
                let mut invalid = manifest.clone();
                invalid["schema"] = json!(2);
                json!({"geam":invalid})
            }),
            (
                "budget",
                json!({"warmup_ms":1,"sample_ms":10,"samples":3,"rounds":1}),
            ),
            ("timeout_seconds", json!(0)),
            ("cases", json!([])),
            ("cases", json!([{"workload":"callback_control","size":999}])),
            (
                "cases",
                json!([{"workload":"callback_control","size":1},{"workload":"callback_control","size":1}]),
            ),
            ("bundles", json!({})),
        ] {
            let mut invalid = configuration.clone();
            invalid[field] = value;
            let invalid: Configuration = serde_json::from_value(invalid).unwrap();
            assert!(invalid.steps().is_err(), "{field}");
            assert!(read_records(root.path(), &invalid).is_err());
        }
        let mut invalid = configuration.clone();
        invalid["selection"] = json!({"kind":"targets","targets":["erlang"]});
        invalid["bundles"] = json!({"erlang":manifest});
        assert_eq!(
            serde_json::from_value::<Configuration>(invalid)
                .unwrap()
                .steps()
                .unwrap_err()
                .to_string(),
            "run variant is absent from its bundle"
        );
        let mut paired = configuration;
        paired["selection"] = json!({"kind":"paired"});
        paired["budget"]["rounds"] = json!(4);
        paired["bundles"] = json!({"baseline":manifest,"candidate":manifest});
        let parsed: Configuration = serde_json::from_value(paired.clone()).unwrap();
        assert_eq!(parsed.steps().unwrap().len(), 8);
        paired["bundles"]["candidate"]["suite_sources"] = json!({"different":"source"});
        assert!(
            serde_json::from_value::<Configuration>(paired)
                .unwrap()
                .steps()
                .is_err()
        );
    }
    pub(crate) fn verify_real_bundle(bundle_path: &std::path::Path, root: &std::path::Path) {
        use super::{
            Bundle, Configuration, Execution, Mode, Selection, Status, Target, Variant,
            check_attempts, measure, measure_steps, seal_run,
        };
        use crate::environment::{copy_files, files};
        use std::collections::BTreeMap;
        use std::fs::{self, File};
        use std::sync::atomic::AtomicBool;
        fs::create_dir(root).unwrap();
        let active = AtomicBool::new(false);
        let arguments = |output: &std::path::Path| Execution {
            output: output.into(),
            mode: Mode::Smoke,
            machine: None,
            cases: vec!["callback_control/1".into()],
            timeout_seconds: 240,
        };
        let complete = root.join("complete");
        super::execute(
            bundle_path,
            None,
            vec![Target::Geam, Target::Erlang, Target::JavaScript],
            arguments(&complete),
            &active,
        )
        .unwrap();
        let paired = root.join("paired");
        super::execute(
            bundle_path,
            Some(bundle_path),
            vec![Target::Geam],
            arguments(&paired),
            &active,
        )
        .unwrap();
        let original = fs::read(complete.join("configuration.json")).unwrap();
        let configuration = || serde_json::from_slice::<Configuration>(&original).unwrap();
        let bundle = Bundle::load(bundle_path).unwrap();
        let inputs = BTreeMap::from([
            (Variant::Geam, &bundle),
            (Variant::Erlang, &bundle),
            (Variant::JavaScript, &bundle),
        ]);
        let admitted = super::read_records(&complete, &configuration()).unwrap();
        assert_eq!(admitted.len(), 9);
        assert_eq!(
            super::read_records(
                &paired,
                &serde_json::from_slice(&fs::read(paired.join("configuration.json")).unwrap())
                    .unwrap()
            )
            .unwrap()
            .len(),
            24
        );
        assert!(
            check_attempts(Err(std::io::Error::other("directory entry unavailable")), 1).is_err()
        );
        for name in [
            "metadata",
            "host",
            "configuration",
            "runtime-mismatch",
            "empty-host",
        ] {
            let output = root.join(format!("configuration-{name}"));
            fs::create_dir(&output).unwrap();
            if name == "metadata" {
                fs::create_dir(output.join("metadata")).unwrap();
            }
            if name == "configuration" {
                fs::create_dir(output.join("configuration.json")).unwrap();
            }
            let mut actual = configuration();
            if name == "runtime-mismatch" {
                actual
                    .bundles
                    .get_mut(&Variant::JavaScript)
                    .unwrap()
                    .runtimes
                    .insert(Target::JavaScript, "different".into());
            }
            let mut args = arguments(&output);
            if name == "empty-host" {
                args.machine = Some(" ".into());
            }
            assert!(
                Configuration::record(
                    &output,
                    args,
                    Selection::Targets(vec![Target::Geam, Target::Erlang, Target::JavaScript]),
                    actual.cases,
                    actual.bundles,
                    &AtomicBool::new(name == "host")
                )
                .is_err(),
                "{name}"
            );
        }
        let logs = root.join("failed-runtime");
        fs::create_dir_all(logs.join("runtime-javascript")).unwrap();
        assert!(configuration().verify_runtimes(&logs, &active).is_err());
        for name in ["configuration", "processes", "raw", "measurement"] {
            let output = root.join(format!("measurement-{name}"));
            fs::create_dir(&output).unwrap();
            match name {
                "processes" => fs::create_dir(output.join("processes")).unwrap(),
                "raw" => fs::create_dir(output.join("raw.jsonl")).unwrap(),
                _ => {}
            }
            let config = if name == "configuration" {
                Err(std::io::Error::other("configuration unavailable").into())
            } else {
                Ok(configuration())
            };
            assert!(
                measure(
                    &output,
                    config,
                    &inputs,
                    &AtomicBool::new(name == "measurement")
                )
                .is_err(),
                "{name}"
            );
            assert!(!output.join("complete.json").exists());
        }
        for name in ["invalid-configuration", "command", "process"] {
            let output = root.join(format!("steps-{name}"));
            fs::create_dir(&output).unwrap();
            let mut raw = File::create_new(output.join("raw.jsonl")).unwrap();
            let mut config = configuration();
            if name == "invalid-configuration" {
                config.timeout_seconds = 0;
            }
            if name == "process" {
                fs::create_dir(output.join("000000")).unwrap();
            }
            let program = bundle_path.join("payload/geam/program");
            let saved = bundle_path.join("payload/geam/held-program");
            if name == "command" {
                fs::rename(&program, &saved).unwrap();
            }
            let result = measure_steps(&output, &config, &inputs, &mut raw, &active);
            if name == "command" {
                fs::rename(saved, program).unwrap();
            }
            assert!(result.is_err(), "{name}");
        }
        // Corrupt or block acquired evidence after real workload execution;
        // recording admission must reject it without manufacturing samples.
        let step = configuration().steps().unwrap()[0];
        let original_stdout = fs::read(complete.join("processes/000000/stdout")).unwrap();
        for name in ["completion", "step", "status", "stdout", "samples", "raw"] {
            let output = root.join(format!("step-{name}"));
            fs::create_dir(&output).unwrap();
            fs::write(output.join("stdout"), &original_stdout).unwrap();
            let raw_path = output.join("raw.jsonl");
            fs::write(&raw_path, b"").unwrap();
            let mut raw = if name == "raw" {
                File::open(&raw_path).unwrap()
            } else {
                File::options().write(true).open(&raw_path).unwrap()
            };
            match name {
                "step" => fs::create_dir(output.join("step.json")).unwrap(),
                "stdout" => fs::remove_file(output.join("stdout")).unwrap(),
                "samples" => fs::write(output.join("stdout"), "{}\n").unwrap(),
                _ => {}
            }
            let completion = if name == "completion" {
                Err(std::io::Error::other("process unavailable").into())
            } else {
                Ok(crate::process::Completion {
                    status: if name == "status" {
                        Status::Failed
                    } else {
                        Status::Success
                    },
                    exit_code: Some(if name == "status" { 7 } else { 0 }),
                    elapsed_ms: 50,
                })
            };
            assert!(
                step.record(&output, configuration().budget, completion, &mut raw)
                    .is_err(),
                "{name}"
            );
        }
        for name in ["verified", "matrix", "reports", "inventory", "seal"] {
            let output = root.join(format!("accept-{name}"));
            copy_files(
                &complete,
                &output,
                super::raw_files(&complete).unwrap().into_keys(),
            )
            .unwrap();
            match name {
                "matrix" => fs::remove_file(output.join("processes/000000/stdout")).unwrap(),
                "reports" => fs::create_dir(output.join("summary.json")).unwrap(),
                "inventory" => std::os::unix::fs::symlink(&complete, output.join("link")).unwrap(),
                "seal" => fs::create_dir(output.join("complete.json")).unwrap(),
                _ => {}
            }
            assert!(
                seal_run(
                    &output,
                    &configuration(),
                    if name == "verified" {
                        Err(std::io::Error::other("bundle changed").into())
                    } else {
                        Ok(())
                    }
                )
                .is_err(),
                "{name}"
            );
            assert!(!output.join("complete.json").is_file(), "{name}");
        }
        let input = root.join("unreadable-inventory");
        copy_files(&complete, &input, files(&complete).unwrap().into_keys()).unwrap();
        std::os::unix::fs::symlink(&complete, input.join("link")).unwrap();
        assert!(super::analyze(&input, &root.join("invalid-analysis")).is_err());
        let input = root.join("missing-configuration");
        fs::create_dir(&input).unwrap();
        fs::write(input.join("complete.json"), r#"{"schema":1,"files":{}}"#).unwrap();
        assert!(super::analyze(&input, &root.join("missing-config-analysis")).is_err());
        let accepted =
            serde_json::from_slice(&fs::read(complete.join("complete.json")).unwrap()).unwrap();
        for name in ["summary.json", "source.json"] {
            let output = root.join(format!("reconstruct-{name}"));
            fs::create_dir(&output).unwrap();
            fs::create_dir(output.join(name)).unwrap();
            assert!(
                super::reconstruct(&output, &admitted, configuration().budget, &accepted).is_err()
            );
        }
    }
}
