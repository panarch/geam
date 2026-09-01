use geam::provider::{BigInt, EcoString, ExternalPayload};
use std::collections::BTreeMap;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

#[geam::provider(
    package = "example_run_metrics",
    modules = [metrics],
)]
pub struct Component;

#[geam::module(path = "example_run_metrics")]
mod metrics {
    use super::{BTreeMap, BigInt, DefaultHasher, EcoString, ExternalPayload, Hash, Hasher};

    #[geam::external(name = "Metrics", manual)]
    #[derive(Clone, Default, PartialEq)]
    struct Metrics {
        entries: BTreeMap<EcoString, Metric>,
    }

    #[derive(Clone, Default, PartialEq)]
    struct Metric {
        count: BigInt,
        total: f64,
    }

    impl ExternalPayload for Metrics {
        fn source_equal(&self, other: &Self) -> bool {
            self == other
        }

        fn source_hash(&self) -> u64 {
            let mut hasher = DefaultHasher::new();
            for (name, metric) in &self.entries {
                name.hash(&mut hasher);
                metric.count.hash(&mut hasher);
                normalized_float_bits(metric.total).hash(&mut hasher);
            }
            hasher.finish()
        }

        fn inspect(&self) -> EcoString {
            let entries = self
                .entries
                .iter()
                .map(|(name, metric)| {
                    let total = f64::from_bits(normalized_float_bits(metric.total));
                    format!("#({name:?}, #({}, {total:?}))", metric.count)
                })
                .collect::<Vec<_>>()
                .join(", ");
            format!("Metrics([{entries}])").into()
        }
    }

    #[geam::function]
    fn new() -> Metrics {
        Metrics::default()
    }

    #[geam::function]
    fn record(metrics: &Metrics, name: EcoString, value: f64) -> Metrics {
        let mut updated = metrics.clone();
        let metric = updated.entries.entry(name).or_default();
        metric.count += 1u8;
        metric.total += value;
        updated
    }

    #[geam::function]
    fn count(metrics: &Metrics, name: EcoString) -> BigInt {
        metrics
            .entries
            .get(&name)
            .map(|metric| metric.count.clone())
            .unwrap_or_default()
    }

    #[geam::function]
    fn total(metrics: &Metrics, name: EcoString) -> f64 {
        metrics
            .entries
            .get(&name)
            .map_or(0.0, |metric| metric.total)
    }

    fn normalized_float_bits(value: f64) -> u64 {
        if value == 0.0 { 0 } else { value.to_bits() }
    }
}
