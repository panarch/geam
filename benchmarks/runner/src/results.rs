use crate::cases::{Case, Workload};
use crate::invalid;
use crate::run::Variant;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, btree_map::Entry};
use std::error::Error;

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Budget {
    pub(super) warmup_ms: u64,
    pub(super) sample_ms: u64,
    pub(super) samples: usize,
    pub(super) rounds: usize,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Sample {
    pub(super) workload: Workload,
    pub(super) size: u64,
    pub(super) iterations: u64,
    pub(super) warmup_elapsed_ns: u64,
    pub(super) warmup_iterations: u64,
    pub(super) sample: usize,
    pub(super) elapsed_ns: u64,
    pub(super) checksum: i64,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Record {
    pub(super) variant: Variant,
    pub(super) round: usize,
    pub(super) sample: Sample,
}

#[derive(Debug, PartialEq)]
pub(super) struct Analysis {
    summaries: Vec<Summary>,
    comparisons: Vec<Comparison>,
}

#[derive(Debug, PartialEq, Serialize)]
pub(super) struct Summary {
    workload: Workload,
    size: u64,
    variant: Variant,
    samples: usize,
    ns_per_op: f64,
    ops_per_second: f64,
    run_medians_ns: Vec<f64>,
    sample_p25_ns: f64,
    sample_p75_ns: f64,
    min_batch_ms: f64,
    max_batch_ms: f64,
}

#[derive(Debug, PartialEq, Serialize)]
pub(super) struct Comparison {
    workload: Workload,
    size: u64,
    numerator: Variant,
    denominator: Variant,
    ratio: f64,
    round_ratios: Option<Vec<f64>>,
}

pub(super) fn parse_samples(
    output: &[u8],
    case: Case,
    budget: Budget,
) -> Result<Vec<Sample>, Box<dyn Error>> {
    let samples = serde_json::Deserializer::from_slice(output)
        .into_iter::<Sample>()
        .collect::<Result<Vec<_>, _>>()?;
    if samples.len() != budget.samples || samples.is_empty() {
        return Err(invalid("unexpected sample count").into());
    }
    let first = &samples[0];
    for (index, sample) in samples.iter().enumerate() {
        if sample.workload != case.workload
            || sample.size != case.size
            || !(1..=1_000_000).contains(&sample.iterations)
            || sample.sample != index + 1
            || sample.elapsed_ns == 0
            || sample.warmup_elapsed_ns < budget.warmup_ms * 1_000_000
            || sample.warmup_iterations == 0
            || sample.checksum != case.expected_checksum()
            || sample.iterations != first.iterations
            || sample.warmup_elapsed_ns != first.warmup_elapsed_ns
            || sample.warmup_iterations != first.warmup_iterations
        {
            return Err(invalid(format!("unexpected sample: {sample:?}")).into());
        }
    }
    Ok(samples)
}

impl Analysis {
    pub(super) fn summaries(&self) -> &[Summary] {
        &self.summaries
    }
    pub(super) fn comparisons(&self) -> &[Comparison] {
        &self.comparisons
    }
}

pub(super) fn summarize(records: &[Record]) -> Result<Analysis, Box<dyn Error>> {
    if records.is_empty() {
        return Err(invalid("cannot summarize an empty run").into());
    }
    let mut groups = BTreeMap::<_, (&Record, Vec<&Record>)>::new();
    for record in records {
        if record.sample.elapsed_ns == 0 || record.sample.iterations == 0 {
            return Err(invalid("statistics require positive elapsed time and iterations").into());
        }
        match groups.entry((record.sample.workload, record.sample.size, record.variant)) {
            Entry::Vacant(entry) => {
                entry.insert((record, Vec::new()));
            }
            Entry::Occupied(mut entry) => entry.get_mut().1.push(record),
        }
    }
    let mut summaries = BTreeMap::new();
    for ((workload, size, variant), (first, rest)) in groups {
        let first_per_op = first.sample.elapsed_ns as f64 / first.sample.iterations as f64;
        let mut values = Measurements::new(first_per_op);
        let mut first_run = Measurements::new(first_per_op);
        let mut other_runs = BTreeMap::<usize, Measurements>::new();
        let mut min_batch_ms = first.sample.elapsed_ns as f64 / 1_000_000.0;
        let mut max_batch_ms = min_batch_ms;
        for record in &rest {
            let per_op = record.sample.elapsed_ns as f64 / record.sample.iterations as f64;
            values.rest.push(per_op);
            if record.round == first.round {
                first_run.rest.push(per_op);
            } else {
                match other_runs.entry(record.round) {
                    Entry::Vacant(entry) => {
                        entry.insert(Measurements::new(per_op));
                    }
                    Entry::Occupied(mut entry) => entry.get_mut().rest.push(per_op),
                }
            }
            let batch_ms = record.sample.elapsed_ns as f64 / 1_000_000.0;
            min_batch_ms = min_batch_ms.min(batch_ms);
            max_batch_ms = max_batch_ms.max(batch_ms);
        }
        let first_median = first_run.quartiles()[1];
        let other_medians = other_runs
            .into_iter()
            .map(|(round, run)| (round, run.quartiles()[1]));
        let mut medians = Measurements::new(first_median);
        let mut ordered_medians = BTreeMap::from([(first.round, first_median)]);
        for (round, median) in other_medians {
            medians.rest.push(median);
            ordered_medians.insert(round, median);
        }
        let ns_per_op = medians.quartiles()[1];
        let [sample_p25_ns, _, sample_p75_ns] = values.quartiles();
        let rounds = ordered_medians.keys().copied().collect::<Vec<_>>();
        summaries.insert(
            (workload, size, variant),
            (
                rounds,
                Summary {
                    workload,
                    size,
                    variant,
                    samples: rest.len() + 1,
                    ns_per_op,
                    ops_per_second: 1_000_000_000.0 / ns_per_op,
                    run_medians_ns: ordered_medians.into_values().collect(),
                    sample_p25_ns,
                    sample_p75_ns,
                    min_batch_ms,
                    max_batch_ms,
                },
            ),
        );
    }
    let mut comparisons = Vec::new();
    for (&(workload, size, numerator), (a_rounds, a)) in &summaries {
        let denominators: &[Variant] = match numerator {
            Variant::Geam => &[Variant::Erlang, Variant::JavaScript],
            Variant::Candidate => &[Variant::Baseline],
            Variant::Erlang | Variant::JavaScript | Variant::Baseline => &[],
        };
        for &denominator in denominators {
            if let Some((b_rounds, b)) = summaries.get(&(workload, size, denominator)) {
                let round_ratios = if numerator == Variant::Candidate {
                    if a_rounds != b_rounds {
                        return Err(invalid("paired summaries have different rounds").into());
                    }
                    Some(
                        a.run_medians_ns
                            .iter()
                            .zip(&b.run_medians_ns)
                            .map(|(a, b)| a / b)
                            .collect(),
                    )
                } else {
                    None
                };
                comparisons.push(Comparison {
                    workload,
                    size,
                    numerator,
                    denominator,
                    ratio: a.ns_per_op / b.ns_per_op,
                    round_ratios,
                });
            }
        }
    }
    Ok(Analysis {
        summaries: summaries
            .into_values()
            .map(|(_, summary)| summary)
            .collect(),
        comparisons,
    })
}

pub(super) fn markdown(analysis: &Analysis, budget: Budget) -> String {
    let mut text = String::from("# Prepared Execution Results\n\n");
    text.push_str(&format!(
        "{} independent runs; {} ms warmup; {} samples/run targeting {} ms/batch.\n\n",
        budget.rounds, budget.warmup_ms, budget.samples, budget.sample_ms
    ));
    text.push_str("Times are microseconds per workload invocation: the median of process medians.\nSetup, startup, warmup and validation are excluded; allocation, reclamation,\nthe shared callback/repetition path and result consumption are included.\nNo outliers are removed. Smoke timings are correctness diagnostics only.\n\n");
    text.push_str("| Workload | Size | Variant | us/op | Run medians (us/op) | Batch-average IQR (us/op) | Batch range (ms) |\n| --- | ---: | --- | ---: | --- | --- | --- |\n");
    for summary in analysis.summaries() {
        let medians = summary
            .run_medians_ns
            .iter()
            .map(|value| format!("{:.6}", value / 1000.0))
            .collect::<Vec<_>>()
            .join(", ");
        text.push_str(&format!(
            "| {} | {} | {} | {:.6} | {} | {:.6} - {:.6} | {:.6} - {:.6} |\n",
            summary.workload.name(),
            summary.size,
            summary.variant.name(),
            summary.ns_per_op / 1000.0,
            medians,
            summary.sample_p25_ns / 1000.0,
            summary.sample_p75_ns / 1000.0,
            summary.min_batch_ms,
            summary.max_batch_ms
        ));
    }
    text.push_str("\nThe IQR describes batch averages, not operation latency or a confidence interval.\nRatios are numerator / denominator; values above 1 mean a slower numerator.\n\n| Workload | Size | Numerator / denominator | Central ratio | Paired round ratios |\n| --- | ---: | --- | ---: | --- |\n");
    for comparison in analysis.comparisons() {
        let pairs = comparison
            .round_ratios
            .as_ref()
            .map(|ratios| {
                ratios
                    .iter()
                    .map(|ratio| format!("{ratio:.6}x"))
                    .collect::<Vec<_>>()
                    .join(", ")
            })
            .unwrap_or_else(|| "unpaired targets".into());
        text.push_str(&format!(
            "| {} | {} | {} / {} | {:.6}x | {} |\n",
            comparison.workload.name(),
            comparison.size,
            comparison.numerator.name(),
            comparison.denominator.name(),
            comparison.ratio,
            pairs
        ));
    }
    text
}

// Every group and process starts with an admitted positive measurement. Keeping
// that first value separate makes interpolation total, including a single batch.
struct Measurements {
    first: f64,
    rest: Vec<f64>,
}
impl Measurements {
    fn new(first: f64) -> Self {
        Self {
            first,
            rest: Vec::new(),
        }
    }
    fn quartiles(&self) -> [f64; 3] {
        let mut sorted = Vec::with_capacity(self.rest.len() + 1);
        sorted.push(self.first);
        sorted.extend_from_slice(&self.rest);
        sorted.sort_by(f64::total_cmp);
        [0.25, 0.5, 0.75].map(|fraction| {
            let index = (sorted.len() - 1) as f64 * fraction;
            let lower = sorted[index.floor() as usize];
            let upper = sorted[index.ceil() as usize];
            lower + (upper - lower) * index.fract()
        })
    }
}

#[cfg(test)]
mod tests {
    use super::{Budget, Measurements, Record, Sample, markdown, parse_samples, summarize};
    use crate::cases::{Case, Workload};
    use crate::run::Variant;

    #[test]
    fn validates_measurement_metadata_and_exact_results() {
        let budget = Budget {
            warmup_ms: 20,
            sample_ms: 10,
            samples: 1,
            rounds: 1,
        };
        let case = Case {
            workload: Workload::CountOnes,
            size: 100,
        };
        let raw = br#"{"workload":"count_ones","size":100,"iterations":50,"warmup_elapsed_ns":20000000,"warmup_iterations":100,"sample":1,"elapsed_ns":123456,"checksum":34}"#;
        let expected = Sample {
            workload: Workload::CountOnes,
            size: 100,
            iterations: 50,
            warmup_elapsed_ns: 20_000_000,
            warmup_iterations: 100,
            sample: 1,
            elapsed_ns: 123_456,
            checksum: 34,
        };
        assert_eq!(parse_samples(raw, case, budget).unwrap(), [expected]);
        for (field, value) in [
            ("workload", serde_json::json!("arithmetic")),
            ("size", serde_json::json!(101)),
            ("iterations", serde_json::json!(0)),
            ("warmup_elapsed_ns", serde_json::json!(1)),
            ("warmup_iterations", serde_json::json!(0)),
            ("sample", serde_json::json!(2)),
            ("elapsed_ns", serde_json::json!(0)),
            ("checksum", serde_json::json!(33)),
            ("extra", serde_json::json!(1)),
        ] {
            let mut wrong: serde_json::Value = serde_json::from_slice(raw).unwrap();
            wrong[field] = value;
            assert!(parse_samples(&serde_json::to_vec(&wrong).unwrap(), case, budget).is_err());
        }
        assert!(parse_samples(b"", case, budget).is_err());
        assert!(parse_samples(b"not JSON", case, budget).is_err());
        let mut second: serde_json::Value = serde_json::from_slice(raw).unwrap();
        second["sample"] = serde_json::json!(2);
        let mut two = raw.to_vec();
        two.extend(serde_json::to_vec(&second).unwrap());
        let two_budget = Budget {
            samples: 2,
            ..budget
        };
        assert_eq!(parse_samples(&two, case, two_budget).unwrap().len(), 2);
        for (field, value) in [
            ("iterations", serde_json::json!(51)),
            ("iterations", serde_json::json!(1_000_001)),
            ("warmup_elapsed_ns", serde_json::json!(20_000_001)),
            ("warmup_iterations", serde_json::json!(101)),
            ("sample", serde_json::json!(1)),
        ] {
            let mut wrong = second.clone();
            wrong[field] = value;
            let mut output = raw.to_vec();
            output.extend(serde_json::to_vec(&wrong).unwrap());
            assert!(parse_samples(&output, case, two_budget).is_err());
        }
    }

    #[test]
    fn medians_normalize_iterations_and_keep_independent_runs() {
        assert_eq!(
            Measurements {
                first: 4.0,
                rest: vec![1.0, 3.0, 2.0]
            }
            .quartiles(),
            [1.75, 2.5, 3.25]
        );
        assert_eq!(Measurements::new(7.0).quartiles(), [7.0, 7.0, 7.0]);
        let mut records =
            [(1, 100, 10), (1, 200, 10), (2, 400, 10)].map(|(round, elapsed_ns, iterations)| {
                Record {
                    variant: Variant::Geam,
                    round,
                    sample: Sample {
                        workload: Workload::CountOnes,
                        size: 100,
                        iterations,
                        warmup_elapsed_ns: 1,
                        warmup_iterations: 1,
                        sample: 1,
                        elapsed_ns,
                        checksum: 34,
                    },
                }
            });
        let summary = summarize(&records).unwrap();
        assert_eq!(summary.summaries()[0].run_medians_ns, [15.0, 40.0]);
        assert_eq!(summary.summaries()[0].ns_per_op, 27.5);
        assert_eq!(summary.summaries()[0].sample_p25_ns, 15.0);
        assert_eq!(summary.summaries()[0].sample_p75_ns, 30.0);
        records.rotate_right(1);
        assert_eq!(summarize(&records).unwrap(), summary);
        assert_eq!(
            summarize(&[]).unwrap_err().to_string(),
            "cannot summarize an empty run"
        );
        for (elapsed_ns, iterations) in [(0, 1), (1, 0)] {
            let mut invalid = Record {
                variant: records[0].variant,
                round: 1,
                sample: records[0].sample.clone(),
            };
            invalid.sample.elapsed_ns = elapsed_ns;
            invalid.sample.iterations = iterations;
            assert_eq!(
                summarize(&[invalid]).unwrap_err().to_string(),
                "statistics require positive elapsed time and iterations"
            );
        }
    }

    #[test]
    fn reports_selected_targets_and_every_paired_round() {
        let mut records = Vec::new();
        for variant in [Variant::Baseline, Variant::Candidate] {
            for round in 1..=2 {
                for sample in 1..=3 {
                    records.push(Record {
                        variant,
                        round,
                        sample: Sample {
                            workload: Workload::CallbackControl,
                            size: 1,
                            iterations: 10_000,
                            warmup_elapsed_ns: 20_000_000,
                            warmup_iterations: 10_000,
                            sample,
                            elapsed_ns: round as u64 * 10_000_000,
                            checksum: 1,
                        },
                    });
                }
            }
        }
        let analysis = summarize(&records).unwrap();
        let budget = Budget {
            warmup_ms: 20,
            sample_ms: 10,
            samples: 3,
            rounds: 2,
        };
        let rendered = markdown(&analysis, budget);
        assert_eq!(
            rendered,
            "# Prepared Execution Results\n\n2 independent runs; 20 ms warmup; 3 samples/run targeting 10 ms/batch.\n\nTimes are microseconds per workload invocation: the median of process medians.\nSetup, startup, warmup and validation are excluded; allocation, reclamation,\nthe shared callback/repetition path and result consumption are included.\nNo outliers are removed. Smoke timings are correctness diagnostics only.\n\n| Workload | Size | Variant | us/op | Run medians (us/op) | Batch-average IQR (us/op) | Batch range (ms) |\n| --- | ---: | --- | ---: | --- | --- | --- |\n| callback_control | 1 | baseline | 1.500000 | 1.000000, 2.000000 | 1.000000 - 2.000000 | 10.000000 - 20.000000 |\n| callback_control | 1 | candidate | 1.500000 | 1.000000, 2.000000 | 1.000000 - 2.000000 | 10.000000 - 20.000000 |\n\nThe IQR describes batch averages, not operation latency or a confidence interval.\nRatios are numerator / denominator; values above 1 mean a slower numerator.\n\n| Workload | Size | Numerator / denominator | Central ratio | Paired round ratios |\n| --- | ---: | --- | ---: | --- |\n| callback_control | 1 | candidate / baseline | 1.000000x | 1.000000x, 1.000000x |\n"
        );
        assert_eq!(
            serde_json::to_string(analysis.comparisons()).unwrap(),
            r#"[{"workload":"callback_control","size":1,"numerator":"candidate","denominator":"baseline","ratio":1.0,"round_ratios":[1.0,1.0]}]"#
        );
        assert_eq!(
            serde_json::to_string(&analysis.summaries()[0]).unwrap(),
            r#"{"workload":"callback_control","size":1,"variant":"baseline","samples":6,"ns_per_op":1500.0,"ops_per_second":666666.6666666666,"run_medians_ns":[1000.0,2000.0],"sample_p25_ns":1000.0,"sample_p75_ns":2000.0,"min_batch_ms":10.0,"max_batch_ms":20.0}"#
        );
    }

    #[test]
    fn paired_statistics_require_the_same_round_identities() {
        let mut records = Vec::new();
        for (variant, round) in [(Variant::Baseline, 1), (Variant::Candidate, 2)] {
            records.push(Record {
                variant,
                round,
                sample: Sample {
                    workload: Workload::CallbackControl,
                    size: 1,
                    iterations: 1,
                    warmup_elapsed_ns: 20_000_000,
                    warmup_iterations: 1,
                    sample: 1,
                    elapsed_ns: 100,
                    checksum: 1,
                },
            });
        }
        assert_eq!(
            summarize(&records).unwrap_err().to_string(),
            "paired summaries have different rounds"
        );
        records[1].round = 1;
        assert_eq!(
            summarize(&records).unwrap().comparisons()[0].round_ratios,
            Some(vec![1.0])
        );
    }
    #[test]
    fn ratios_preserve_direction_and_each_slower_pair() {
        let mut records = Vec::new();
        for (variant, medians) in [
            (Variant::Geam, vec![20]),
            (Variant::Erlang, vec![10]),
            (Variant::JavaScript, vec![40]),
            (Variant::Baseline, vec![10, 20, 20, 30]),
            (Variant::Candidate, vec![40, 30, 30, 10]),
        ] {
            for (round, per_op) in medians.into_iter().enumerate() {
                records.push(Record {
                    variant,
                    round: round + 1,
                    sample: Sample {
                        workload: Workload::CallbackControl,
                        size: 1,
                        iterations: 1000,
                        warmup_elapsed_ns: 20_000_000,
                        warmup_iterations: 1000,
                        sample: 1,
                        elapsed_ns: per_op * 1000,
                        checksum: 1,
                    },
                });
            }
        }
        let analysis = summarize(&records).unwrap();
        assert_eq!(
            serde_json::to_string(analysis.comparisons()).unwrap(),
            r#"[{"workload":"callback_control","size":1,"numerator":"geam","denominator":"erlang","ratio":2.0,"round_ratios":null},{"workload":"callback_control","size":1,"numerator":"geam","denominator":"javascript","ratio":0.5,"round_ratios":null},{"workload":"callback_control","size":1,"numerator":"candidate","denominator":"baseline","ratio":1.5,"round_ratios":[4.0,1.5,1.5,0.3333333333333333]}]"#
        );
    }
}
