use ecow::EcoString;
use geam_core::provider::{Configuration, ExternalPayload};
use geam_core::{
    HostComponentProfile, HostModule, HostProfile, HostProviderComponent,
    HostProviderComponentInitialization, HostProviderComponentRegistration, HostProviderSet,
    HostedExecution, ModuleSource, PackageSource, PlanError, Value, ValueType,
    compile_typed_host_program, plan_host_program,
};
use num_bigint::BigInt;
use std::collections::BTreeMap;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

#[geam_macros::provider(
    package = "metrics",
    modules = [metrics],
    crate_path = geam_core,
)]
pub struct Component;

#[geam_macros::module(path = "metrics", crate_path = geam_core)]
mod metrics {
    use super::{BTreeMap, BigInt, DefaultHasher, EcoString, ExternalPayload, Hash, Hasher};

    #[geam_macros::external(name = "Metrics", manual)]
    #[derive(Clone, Default, PartialEq)]
    pub(super) struct Metrics {
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

    #[geam_macros::function]
    fn new() -> Metrics {
        Metrics::default()
    }

    #[geam_macros::function]
    pub(super) fn record(metrics: &Metrics, name: EcoString, value: f64) -> Metrics {
        let mut updated = metrics.clone();
        let metric = updated.entries.entry(name).or_default();
        metric.count += 1u8;
        metric.total += value;
        updated
    }

    #[geam_macros::function]
    fn count(#[geam_macros::state] _state: &(), metrics: &Metrics, name: EcoString) -> BigInt {
        metrics
            .entries
            .get(&name)
            .map(|metric| metric.count.clone())
            .unwrap_or_default()
    }

    #[geam_macros::function]
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

struct Profile;

#[derive(Default)]
struct ProfileStores {
    component: <Component as HostProviderComponent>::Stores,
}

struct ProfileState {
    component: <Component as HostProviderComponent>::RunState,
}

impl HostProfile for Profile {
    type RunState = ProfileState;
    type ExternalStores = ProfileStores;
}

impl HostComponentProfile<Component> for Profile {
    fn component_stores(
        stores: &Self::ExternalStores,
    ) -> &<Component as HostProviderComponent>::Stores {
        &stores.component
    }

    fn component_state(
        state: &mut Self::RunState,
    ) -> &mut <Component as HostProviderComponent>::RunState {
        &mut state.component
    }
}

const METRICS_SOURCE: &str = r#"
@external(erlang, "macro_metrics", "Metrics")
pub type Metrics

@external(erlang, "macro_metrics", "new")
pub fn new() -> Metrics

@external(erlang, "macro_metrics", "record")
pub fn record(metrics: Metrics, name: String, value: Float) -> Metrics

@external(erlang, "macro_metrics", "count")
pub fn count(metrics: Metrics, name: String) -> Int

@external(erlang, "macro_metrics", "total")
pub fn total(metrics: Metrics, name: String) -> Float

pub fn main() {
  let empty = new()
  let one = record(empty, "latency_ms", 12.5)
  let measured = record(one, "latency_ms", 7.5)
  let measured = record(measured, "payload_kb", 4.0)

  let equivalent = record(new(), "latency_ms", 12.5)
  let equivalent = record(equivalent, "latency_ms", 7.5)
  let equivalent = record(equivalent, "payload_kb", 4.0)
  let positive_zero = record(new(), "zero", 0.0)
  let negative_zero = record(new(), "zero", -0.0)

  assert empty == new()
  assert empty != one
  assert measured == equivalent
  assert positive_zero == negative_zero
  assert count(empty, "latency_ms") == 0
  assert total(empty, "latency_ms") == 0.0
  assert count(one, "latency_ms") == 1
  assert total(one, "latency_ms") == 12.5
  assert count(measured, "latency_ms") == 2
  assert total(measured, "latency_ms") == 20.0
  assert count(measured, "payload_kb") == 1
  assert total(measured, "payload_kb") == 4.0
  assert count(measured, "missing") == 0
  assert total(measured, "missing") == 0.0

  measured
}
"#;

fn providers() -> Vec<geam_core::HostProviderModule<Profile>> {
    <Component as HostProviderComponentRegistration<Profile>>::providers()
        .expect("macro-authored external component should register")
}

fn execution(source: &str) -> Result<HostedExecution<Profile>, PlanError> {
    let hosts = HostProviderSet::with_providers(Vec::<HostModule<Profile>>::new(), providers())
        .expect("macro-authored module should be unique");
    let typed = compile_typed_host_program(
        "metrics",
        "metrics",
        [PackageSource::new(
            "metrics",
            Vec::<&str>::new(),
            [ModuleSource::new("metrics", "src/metrics.gleam", source)],
        )],
        hosts,
    )
    .expect("complete external provider source should compile");
    let plan = plan_host_program(typed)?;
    Ok(
        HostedExecution::try_from_module_plan(plan)
            .expect("matching external provider should seal"),
    )
}

#[test]
fn macro_authored_external_schema_and_function_shapes_are_exact() {
    assert_eq!(Component::ID, "geam-macros");
    assert_eq!(Component::initialize(&Configuration::empty()), Ok(()));
    let configured = Configuration::new(BTreeMap::from([(EcoString::from("unused"), true.into())]));
    let error = Component::initialize(&configured)
        .expect_err("default initialization must reject unused configuration");
    assert_eq!(error.component_id(), "geam-macros");
    assert_eq!(error.reason(), "provider does not accept configuration");

    let providers = providers();
    assert_eq!(providers.len(), 1);
    let provider = &providers[0];
    assert_eq!(provider.package().as_str(), "metrics");
    assert_eq!(provider.module().as_str(), "metrics");
    let external_types = provider.external_types().collect::<Vec<_>>();
    assert_eq!(external_types.len(), 1);
    assert_eq!(external_types[0].package().as_str(), "metrics");
    assert_eq!(external_types[0].module().as_str(), "metrics");
    assert_eq!(external_types[0].name().as_str(), "Metrics");
    assert_eq!(external_types[0].parameter_count(), 0);

    let functions = provider.functions().collect::<Vec<_>>();
    assert_eq!(
        functions
            .iter()
            .map(|function| function.name().as_str())
            .collect::<Vec<_>>(),
        ["new", "record", "count", "total"],
    );
    let external = functions[0].type_().return_().clone();
    assert!(matches!(external, ValueType::External(_)));
    assert_eq!(functions[0].type_().argument_types(), []);
    assert_eq!(
        functions[1].type_().argument_types(),
        &[external.clone(), ValueType::String, ValueType::Float]
    );
    assert_eq!(functions[1].type_().return_(), &external);
    assert_eq!(
        functions[2].type_().argument_types(),
        &[external.clone(), ValueType::String]
    );
    assert_eq!(functions[2].type_().return_(), &ValueType::Int);
    assert_eq!(
        functions[3].type_().argument_types(),
        &[external, ValueType::String]
    );
    assert_eq!(functions[3].type_().return_(), &ValueType::Float);
}

#[test]
fn macro_authored_external_values_preserve_updates_equality_and_lifetime() {
    use metrics::Metrics;

    let zero = Metrics::default();
    let positive_zero = metrics::record(&zero, "zero".into(), 0.0);
    let negative_zero = metrics::record(&zero, "zero".into(), -0.0);
    assert!(positive_zero.source_equal(&negative_zero));
    assert_eq!(positive_zero.source_hash(), negative_zero.source_hash());
    assert_eq!(positive_zero.inspect(), negative_zero.inspect());
    assert_eq!(zero.inspect(), "Metrics([])");

    let (first, second) = {
        let execution = execution(METRICS_SOURCE).expect("matching external provider should plan");
        let mut first_state = ProfileState { component: () };
        let mut second_state = ProfileState { component: () };
        let first = execution
            .run_main(&mut first_state, &mut Vec::new())
            .expect("external provider should execute");
        let second = execution
            .run_main(&mut second_state, &mut Vec::new())
            .expect("external provider should repeat independently");
        (first, second)
    };

    let Value::External(first) = first else {
        panic!("main should return the final external value");
    };
    let Value::External(second) = second else {
        panic!("repeated main should return the final external value");
    };
    let expected = "Metrics([#(\"latency_ms\", #(2, 20.0)), #(\"payload_kb\", #(1, 4.0))])";
    assert_eq!(first.inspection(), expected);
    assert_eq!(second.inspection(), expected);
    assert_ne!(first.identity(), second.identity());
    assert_eq!(first.clone(), first);
}

#[test]
fn external_return_shape_mismatch_remains_a_structured_link_error() {
    let mismatched = r#"
@external(erlang, "macro_metrics", "Metrics")
pub type Metrics

@external(erlang, "macro_metrics", "new")
pub fn new() -> String

pub fn main() {
  new()
}
"#;
    let error = match execution(mismatched) {
        Err(error) => error,
        Ok(_) => panic!("mismatched return should fail during linkage"),
    };
    let PlanError::HostProviderLink {
        package,
        module,
        function,
        reason,
    } = error
    else {
        panic!("mismatch should remain a host provider linkage error");
    };
    assert_eq!(package.as_str(), "metrics");
    assert_eq!(module.as_str(), "metrics");
    assert_eq!(function.as_str(), "new");
    let geam_core::HostProviderLinkReason::SchemeMismatch {
        expected_scheme,
        expected_type,
        actual_scheme,
        actual_type,
    } = *reason
    else {
        panic!("linkage error should preserve the exact scheme mismatch");
    };
    assert!(expected_scheme.parameters().is_empty());
    assert!(expected_type.argument_types().is_empty());
    assert_eq!(expected_type.return_(), &ValueType::String);
    assert!(actual_scheme.parameters().is_empty());
    assert!(actual_type.argument_types().is_empty());
    assert!(matches!(actual_type.return_(), ValueType::External(_)));
}
