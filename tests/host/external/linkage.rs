use super::{
    Counter, CounterProvider, CounterSchema, DependencyCounterSchema, ExternalProfile,
    ExternalRunState, HostCounter, HostDependencyCounter,
};
use geam::{
    HostCall, HostCallCompletion, HostCallError, HostExternal, HostModule, HostProviderLinkReason,
    HostProviderModule, HostProviderSet, HostedExecution, ModuleSource, PackageSource, PlanError,
    Value, compile_typed_host_program, plan_host_program,
};
use num_bigint::BigInt;

#[test]
fn source_less_host_external_values_require_a_source_declaration() {
    fn new_counter<'call>(
        mut call: HostCall<'call, ExternalProfile, CounterProvider, HostCounter>,
        value: BigInt,
    ) -> Result<HostCallCompletion<'call, HostCounter>, HostCallError> {
        let counter = call.create_external(Counter { value });
        Ok(call.return_value(counter))
    }

    let host = HostModule::<ExternalProfile>::new_for_profile("application", "host/counter")
        .expect("source-less host module should be valid")
        .with_scoped_function::<CounterProvider, (BigInt,), HostCounter, _>(
            "new_counter",
            new_counter,
        )
        .expect("source-less external function should be valid");
    let source = r#"
import host/counter

pub fn main() {
  counter.new_counter(1)
}
"#;
    let typed = compile_typed_host_program(
        "application",
        "main",
        [PackageSource::new(
            "application",
            Vec::<&str>::new(),
            [ModuleSource::new("main", "src/main.gleam", source)],
        )],
        HostProviderSet::with_providers([host], Vec::<HostProviderModule<ExternalProfile>>::new())
            .expect("source-less host module should be unique"),
    )
    .expect("source-less external function should compile");

    let PlanError::HostProviderLink {
        package,
        module,
        function,
        reason,
    } = plan_host_program(typed)
        .err()
        .expect("missing source declaration should fail planning")
    else {
        panic!("missing source declaration should retain its host linkage owner");
    };
    let HostProviderLinkReason::MissingExternalType { external_type } = *reason else {
        panic!("missing source declaration should retain its exact reason");
    };

    assert_eq!(package, "application");
    assert_eq!(module, "host/counter");
    assert_eq!(function, "new_counter");
    assert_eq!(external_type.package(), "application");
    assert_eq!(external_type.module(), "main");
    assert_eq!(external_type.name(), "Counter");
}

#[test]
fn links_dependency_package_external_values_by_nominal_identity() {
    fn new_counter<'call>(
        mut call: HostCall<'call, ExternalProfile, CounterProvider, HostDependencyCounter>,
        value: BigInt,
    ) -> Result<HostCallCompletion<'call, HostDependencyCounter>, HostCallError> {
        let counter = call.create_external(Counter { value });
        Ok(call.return_value(counter))
    }

    fn counter_value<'call>(
        call: HostCall<'call, ExternalProfile, CounterProvider, BigInt>,
        counter: HostExternal<'call, HostDependencyCounter>,
    ) -> Result<HostCallCompletion<'call, BigInt>, HostCallError> {
        let value = call.external_payload(counter).value.clone();
        Ok(call.return_value(value))
    }

    let provider = HostProviderModule::<ExternalProfile>::new("support", "support/counter")
        .expect("provider module should be valid")
        .with_external_type::<CounterProvider, DependencyCounterSchema>()
        .expect("external type should be valid")
        .with_scoped_function::<CounterProvider, (BigInt,), HostDependencyCounter, _>(
            "new",
            new_counter,
        )
        .expect("constructor provider should be valid")
        .with_scoped_function::<CounterProvider, (HostDependencyCounter,), BigInt, _>(
            "value",
            counter_value,
        )
        .expect("reader provider should be valid");
    let dependency_source = r#"
pub type Counter

@external(erlang, "host", "new")
pub fn new(value: Int) -> Counter

@external(erlang, "host", "value")
pub fn value(counter: Counter) -> Int

pub const empty: List(Counter) = []
pub const maker: fn(Int) -> Counter = new
"#;
    let root_source = r#"
import support/counter

const imported_empty = counter.empty

pub fn main() {
  let created = counter.new(73)
  #(created, counter.value(created), imported_empty, counter.maker(74))
}
"#;
    let typed = compile_typed_host_program(
        "application",
        "main",
        [
            PackageSource::new(
                "support",
                Vec::<&str>::new(),
                [ModuleSource::new(
                    "support/counter",
                    "support/src/support/counter.gleam",
                    dependency_source,
                )],
            ),
            PackageSource::new(
                "application",
                ["support"],
                [ModuleSource::new("main", "src/main.gleam", root_source)],
            ),
        ],
        HostProviderSet::with_providers(Vec::<HostModule<ExternalProfile>>::new(), [provider])
            .expect("provider module should be unique"),
    )
    .expect("dependency external source should compile");
    let plan = plan_host_program(typed).expect("dependency external source should plan");
    assert_eq!(
        plan.modules()
            .iter()
            .map(|module| (module.package().as_str(), module.module().as_str()))
            .collect::<Vec<_>>(),
        [("support", "support/counter"), ("application", "main")],
    );
    let execution =
        HostedExecution::try_from_module_plan(plan).expect("external execution should seal");
    let returned = execution
        .run_main(&mut ExternalRunState::default(), &mut Vec::new())
        .expect("dependency external source should execute");
    let Value::Tuple(values) = returned else {
        panic!("main should return a tuple");
    };
    let Value::External(counter) = &values[0] else {
        panic!("first tuple field should be external");
    };
    assert_eq!(counter.inspection(), "SupportCounter(73)");
    assert_eq!(counter.type_().type_name().package(), "support");
    assert_eq!(counter.type_().type_name().module(), "support/counter");
    assert_eq!(values[1], Value::Int(BigInt::from(73)));
    let Value::List(empty) = &values[2] else {
        panic!("third tuple field should be a list");
    };
    assert!(empty.is_empty());
    assert_eq!(
        empty.item_type(),
        geam::ValueType::External(counter.type_().clone()),
    );
    let Value::External(from_constant) = &values[3] else {
        panic!("fourth tuple field should be external");
    };
    assert_eq!(from_constant.inspection(), "SupportCounter(74)");
}

#[test]
fn uses_source_declared_external_types_in_source_less_host_modules() {
    fn new_counter<'call>(
        mut call: HostCall<'call, ExternalProfile, CounterProvider, HostCounter>,
        value: BigInt,
    ) -> Result<HostCallCompletion<'call, HostCounter>, HostCallError> {
        let counter = call.create_external(Counter { value });
        Ok(call.return_value(counter))
    }

    fn identity<'call>(
        call: HostCall<'call, ExternalProfile, CounterProvider, HostCounter>,
        counter: HostExternal<'call, HostCounter>,
    ) -> Result<HostCallCompletion<'call, HostCounter>, HostCallError> {
        Ok(call.return_value(counter))
    }

    let provider = HostProviderModule::<ExternalProfile>::new("application", "main")
        .expect("provider module should be valid")
        .with_external_type::<CounterProvider, CounterSchema>()
        .expect("external type should be valid")
        .with_scoped_function::<CounterProvider, (BigInt,), HostCounter, _>(
            "new_counter",
            new_counter,
        )
        .expect("constructor provider should be valid");
    let host = HostModule::<ExternalProfile>::new_for_profile("application", "host/counter")
        .expect("source-less host module should be valid")
        .with_scoped_function::<CounterProvider, (HostCounter,), HostCounter, _>(
            "identity", identity,
        )
        .expect("source-less external function should be valid");
    let source = r#"
import host/counter

@external(erlang, "host", "Counter")
pub type Counter

@external(erlang, "host", "new_counter")
fn new_counter(value: Int) -> Counter

pub fn main() {
  counter.identity(new_counter(31))
}
"#;
    let typed = compile_typed_host_program(
        "application",
        "main",
        [PackageSource::new(
            "application",
            Vec::<&str>::new(),
            [ModuleSource::new("main", "src/main.gleam", source)],
        )],
        HostProviderSet::with_providers([host], [provider])
            .expect("host and provider modules should be unique"),
    )
    .expect("source-less external source should compile");
    let plan = plan_host_program(typed).expect("source-less external source should plan");
    let execution = HostedExecution::try_from_module_plan(plan)
        .expect("source-less external execution should seal");

    let returned = execution
        .run_main(&mut ExternalRunState::default(), &mut Vec::new())
        .expect("source-less external source should execute");

    assert_eq!(returned.inspect().to_string(), "Counter(31)");
}
