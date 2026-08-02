use ecow::EcoString;
use geam::{
    BitArrayValue, ExecutionError, HostCall, HostCallCompletion, HostCallError, HostCallable,
    HostCustom, HostCustomConstructorAt, HostCustomConstructorDefinition,
    HostCustomConstructorList, HostCustomConstructorListEnd, HostCustomField, HostCustomFieldList,
    HostCustomFieldListEnd, HostCustomIndex0, HostCustomSchema, HostCustomType, HostExternal,
    HostExternalEquality, HostExternalHashing, HostExternalInspection, HostExternalSchema,
    HostExternalStorage, HostExternalStore, HostExternalType, HostFailure, HostFunctionType,
    HostList, HostListType, HostModule, HostProfile, HostProvider, HostProviderLinkReason,
    HostProviderModule, HostProviderSet, HostTupleType, HostTypeList, HostTypeListEnd,
    HostTypeParameter, HostValue, HostedExecution, ModuleSource, PackageSource, PanicKind,
    PlanError, Value, compile_typed_host_program, plan_host_program,
};
use num_bigint::BigInt;
use std::collections::hash_map::DefaultHasher;
use std::convert::Infallible;
use std::hash::{Hash, Hasher};

struct ExternalProfile;

#[derive(Default)]
struct ExternalRunState {
    provider: (),
}

#[derive(Default)]
struct ExternalStores {
    counters: HostExternalStore<Counter>,
    dependency_counters: HostExternalStore<Counter>,
    generic_counters: HostExternalStore<Counter>,
}

#[derive(Debug)]
struct Counter {
    value: BigInt,
}

struct CounterSchema;

struct CounterProvider;

type HostCounter = HostExternalType<CounterSchema>;

struct WrappedCounterField;

impl HostCustomField for WrappedCounterField {
    const LABEL: Option<&'static str> = Some("value");

    type Type = HostCounter;
}

struct WrappedCounterDefinition;

impl HostCustomConstructorDefinition for WrappedCounterDefinition {
    const NAME: &'static str = "Wrapped";

    type Fields = HostCustomFieldList<WrappedCounterField, HostCustomFieldListEnd>;
}

struct WrappedCounterSchema;

impl HostCustomSchema for WrappedCounterSchema {
    const PACKAGE: &'static str = "application";
    const MODULE: &'static str = "main";
    const NAME: &'static str = "Wrapped";
    const PARAMETER_COUNT: usize = 0;

    type Constructors =
        HostCustomConstructorList<WrappedCounterDefinition, HostCustomConstructorListEnd>;
}

type HostWrappedCounter = HostCustomType<WrappedCounterSchema>;
type HostWrappedCounterConstructor =
    HostCustomConstructorAt<HostWrappedCounter, HostCustomIndex0, WrappedCounterDefinition>;

struct DependencyCounterSchema;

type HostDependencyCounter = HostExternalType<DependencyCounterSchema>;

struct GenericCounterSchema;

type GenericCounterArguments = HostTypeList<HostTypeParameter<0>, HostTypeListEnd>;
type HostGenericCounter = HostExternalType<GenericCounterSchema, GenericCounterArguments>;
type GenericValue = HostTypeParameter<0>;
type NoArguments = HostTypeListEnd;
type IntArguments = HostTypeList<BigInt, HostTypeListEnd>;
type GenericCallback = HostFunctionType<NoArguments, GenericValue>;
type GenericIntCallback = HostFunctionType<IntArguments, GenericValue>;

impl HostProfile for ExternalProfile {
    type RunState = ExternalRunState;
    type ExternalStores = ExternalStores;
}

impl HostExternalSchema for CounterSchema {
    const PACKAGE: &'static str = "application";
    const MODULE: &'static str = "main";
    const NAME: &'static str = "Counter";
    const PARAMETER_COUNT: usize = 0;
}

impl HostExternalStorage<CounterSchema> for ExternalProfile {
    type Payload = Counter;

    fn store(stores: &Self::ExternalStores) -> &HostExternalStore<Self::Payload> {
        &stores.counters
    }

    fn source_equal(
        _: &HostExternalEquality<'_>,
        left: &Self::Payload,
        right: &Self::Payload,
    ) -> bool {
        left.value == right.value
    }

    fn source_hash(_: &HostExternalHashing<'_>, value: &Self::Payload) -> u64 {
        let mut hasher = DefaultHasher::new();
        value.value.hash(&mut hasher);
        hasher.finish()
    }

    fn inspect(_: &HostExternalInspection<'_>, value: &Self::Payload) -> EcoString {
        format!("Counter({})", value.value).into()
    }
}

impl HostExternalSchema for DependencyCounterSchema {
    const PACKAGE: &'static str = "support";
    const MODULE: &'static str = "support/counter";
    const NAME: &'static str = "Counter";
    const PARAMETER_COUNT: usize = 0;
}

impl HostExternalStorage<DependencyCounterSchema> for ExternalProfile {
    type Payload = Counter;

    fn store(stores: &Self::ExternalStores) -> &HostExternalStore<Self::Payload> {
        &stores.dependency_counters
    }

    fn source_equal(
        _: &HostExternalEquality<'_>,
        left: &Self::Payload,
        right: &Self::Payload,
    ) -> bool {
        left.value == right.value
    }

    fn source_hash(_: &HostExternalHashing<'_>, value: &Self::Payload) -> u64 {
        let mut hasher = DefaultHasher::new();
        value.value.hash(&mut hasher);
        hasher.finish()
    }

    fn inspect(_: &HostExternalInspection<'_>, value: &Self::Payload) -> EcoString {
        format!("SupportCounter({})", value.value).into()
    }
}

impl HostExternalSchema for GenericCounterSchema {
    const PACKAGE: &'static str = "application";
    const MODULE: &'static str = "main";
    const NAME: &'static str = "GenericCounter";
    const PARAMETER_COUNT: usize = 1;
}

impl HostExternalStorage<GenericCounterSchema> for ExternalProfile {
    type Payload = Counter;

    fn store(stores: &Self::ExternalStores) -> &HostExternalStore<Self::Payload> {
        &stores.generic_counters
    }

    fn source_equal(
        _: &HostExternalEquality<'_>,
        left: &Self::Payload,
        right: &Self::Payload,
    ) -> bool {
        left.value == right.value
    }

    fn source_hash(_: &HostExternalHashing<'_>, value: &Self::Payload) -> u64 {
        let mut hasher = DefaultHasher::new();
        value.value.hash(&mut hasher);
        hasher.finish()
    }

    fn inspect(_: &HostExternalInspection<'_>, value: &Self::Payload) -> EcoString {
        format!("GenericCounter({})", value.value).into()
    }
}

impl HostProvider<ExternalProfile> for CounterProvider {
    type State = ();

    fn project(state: &mut ExternalRunState) -> &mut Self::State {
        &mut state.provider
    }
}

#[test]
fn external_profile_preserves_every_callback_return_family() {
    fn new_counter<'call>(
        mut call: HostCall<'call, ExternalProfile, CounterProvider, HostCounter>,
        value: BigInt,
    ) -> Result<HostCallCompletion<'call, HostCounter>, HostCallError> {
        let counter = call.create_external(Counter { value });
        Ok(call.return_value(counter))
    }

    fn invoke<'call>(
        mut call: HostCall<'call, ExternalProfile, CounterProvider, GenericValue>,
        function: HostCallable<'call, NoArguments, GenericValue>,
    ) -> Result<HostCallCompletion<'call, GenericValue>, HostCallError> {
        let value = call.invoke(function, ())?;
        Ok(call.return_value(value))
    }

    fn invoke_with_int<'call>(
        mut call: HostCall<'call, ExternalProfile, CounterProvider, GenericValue>,
        function: HostCallable<'call, IntArguments, GenericValue>,
        value: BigInt,
    ) -> Result<HostCallCompletion<'call, GenericValue>, HostCallError> {
        let value = call.invoke(function, (value, ()))?;
        Ok(call.return_value(value))
    }

    let provider = HostProviderModule::<ExternalProfile>::new("application", "main")
        .expect("provider module should be valid")
        .with_external_type::<CounterSchema>()
        .expect("external type should be valid")
        .with_scoped_function::<CounterProvider, (BigInt,), HostCounter, _>(
            "new_counter",
            new_counter,
        )
        .expect("external constructor should be valid")
        .with_scoped_function::<CounterProvider, (GenericCallback,), GenericValue, _>(
            "invoke", invoke,
        )
        .expect("generic callback should be valid")
        .with_scoped_function::<CounterProvider, (GenericIntCallback, BigInt), GenericValue, _>(
            "invoke_with_int",
            invoke_with_int,
        )
        .expect("generic callback with an argument should be valid");
    let source = r#"
@external(erlang, "host", "Counter")
pub type Counter

pub type Marker {
  Marker(Int)
}

@external(erlang, "host", "new_counter")
fn new_counter(value: Int) -> Counter

@external(erlang, "host", "invoke")
fn invoke(function: fn() -> value) -> value

@external(erlang, "host", "invoke_with_int")
fn invoke_with_int(function: fn(Int) -> value, value: Int) -> value

fn int_value() { 1 }
fn float_value() { 1.5 }
fn string_value() { "text" }
fn bit_array_value() { <<1>> }
fn utf_codepoint_value() -> UtfCodepoint {
  let assert <<value:utf8_codepoint>> = <<65>>
  value
}
fn custom_value() { Marker(2) }
fn external_value() { new_counter(3) }
fn bool_value() { True }
fn nil_value() { Nil }
fn tuple_value() { #(4, False) }
fn list_value() { [5, 6] }
fn increment(value: Int) { value + 1 }
fn function_value() { increment }
fn external_list_value() { [new_counter(10)] }
fn external_function_value() { new_counter }
fn generic_value(value) { value }

pub fn main() {
  let external_identity: fn(Counter) -> Counter = generic_value
  #(
    invoke(int_value),
    invoke(float_value),
    invoke(string_value),
    invoke(bit_array_value),
    invoke(utf_codepoint_value),
    invoke(custom_value),
    invoke_with_int(Marker, 7),
    invoke(external_value),
    invoke(bool_value),
    invoke(nil_value),
    invoke(tuple_value),
    invoke(list_value),
    invoke(function_value)(8),
    invoke(external_list_value),
    invoke(external_function_value)(11),
    external_identity(new_counter(12)),
  )
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
        HostProviderSet::with_providers(Vec::<HostModule<ExternalProfile>>::new(), [provider])
            .expect("provider module should be unique"),
    )
    .expect("callback family source should compile");
    let plan = plan_host_program(typed).expect("callback family source should plan");
    let execution =
        HostedExecution::try_from_module_plan(plan).expect("callback family execution should seal");

    assert_eq!(
        execution
            .run_main(&mut ExternalRunState::default(), &mut Vec::new())
            .expect("every callback family should execute")
            .inspect()
            .to_string(),
        r#"#(1, 1.5, "text", <<1>>, 'A', Marker(2), Marker(7), Counter(3), True, Nil, #(4, False), [5, 6], 9, [Counter(10)], Counter(11), Counter(12))"#,
    );
}

#[test]
fn hashes_symbolic_and_external_function_values_through_the_host_call() {
    fn new_counter<'call>(
        mut call: HostCall<'call, ExternalProfile, CounterProvider, HostCounter>,
        value: BigInt,
    ) -> Result<HostCallCompletion<'call, HostCounter>, HostCallError> {
        let counter = call.create_external(Counter { value });
        Ok(call.return_value(counter))
    }

    fn source_hash<'call>(
        call: HostCall<'call, ExternalProfile, CounterProvider, BigInt>,
        value: HostValue<'call, GenericValue>,
    ) -> Result<HostCallCompletion<'call, BigInt>, HostCallError> {
        let hash = call.source_hash::<GenericValue>(value);
        Ok(call.return_value(BigInt::from(hash)))
    }

    let provider = HostProviderModule::<ExternalProfile>::new("application", "main")
        .expect("provider module should be valid")
        .with_external_type::<CounterSchema>()
        .expect("external type should be valid")
        .with_scoped_function::<CounterProvider, (BigInt,), HostCounter, _>(
            "new_counter",
            new_counter,
        )
        .expect("external constructor should be valid")
        .with_scoped_function::<CounterProvider, (GenericValue,), BigInt, _>(
            "source_hash",
            source_hash,
        )
        .expect("source hash should be valid");
    let source = r#"
@external(erlang, "host", "Counter")
pub type Counter

@external(erlang, "host", "new_counter")
fn new_counter(value: Int) -> Counter

@external(erlang, "host", "source_hash")
fn source_hash(value: value) -> Int

pub type Marker {
  Marker(Int)
}

fn identity(value) {
  value
}

fn never() -> value {
  panic as "source hash must not invoke its argument"
}

fn int_value() { 1 }
fn float_value() { 1.5 }
fn string_value() { "text" }
fn bit_array_value() { <<1>> }
fn utf_codepoint_value() -> UtfCodepoint {
  let assert <<value:utf8_codepoint>> = <<65>>
  value
}
fn custom_value() { Marker(2) }
fn external(value: Int) {
  new_counter(value)
}
fn bool_value() { True }
fn nil_value() { Nil }
fn tuple_value() { #(4, False) }
fn list_value() { [5, 6] }
fn function_value() { int_value }
fn parameter_list() -> List(value) { [] }

pub fn main() {
  let parameters = parameter_list()
  let generic = identity
  let no_return = never
  let constructor = Marker
  let closure = fn(value) { value + 1 }
  #(
    source_hash(parameters) == source_hash(parameters),
    source_hash(generic) == source_hash(generic),
    source_hash(no_return) == source_hash(no_return),
    source_hash(int_value) == source_hash(int_value),
    source_hash(float_value) == source_hash(float_value),
    source_hash(string_value) == source_hash(string_value),
    source_hash(bit_array_value) == source_hash(bit_array_value),
    source_hash(utf_codepoint_value) == source_hash(utf_codepoint_value),
    source_hash(custom_value) == source_hash(custom_value),
    source_hash(constructor) == source_hash(constructor),
    source_hash(external) == source_hash(external),
    source_hash(bool_value) == source_hash(bool_value),
    source_hash(nil_value) == source_hash(nil_value),
    source_hash(tuple_value) == source_hash(tuple_value),
    source_hash(list_value) == source_hash(list_value),
    source_hash(function_value) == source_hash(function_value),
    source_hash(closure) == source_hash(closure),
  )
}
"#;
    let typed = compile_typed_host_program(
        "application",
        "main",
        [PackageSource::new(
            "application",
            Vec::<EcoString>::new(),
            [ModuleSource::new("main", "src/main.gleam", source)],
        )],
        HostProviderSet::with_providers(Vec::new(), [provider])
            .expect("provider module should be unique"),
    )
    .expect("source hash source should compile");
    let plan = plan_host_program(typed).expect("source hash source should plan");
    let execution =
        HostedExecution::try_from_module_plan(plan).expect("source hash execution should seal");

    assert_eq!(
        execution.run_main(&mut ExternalRunState::default(), &mut Vec::new()),
        Ok(Value::Tuple(vec![Value::Bool(true); 17])),
    );
}

#[test]
fn external_profile_reports_diverging_external_function_returns() {
    type CounterCallable = HostFunctionType<IntArguments, HostCounter>;

    fn stop<'call>(
        _call: HostCall<'call, ExternalProfile, CounterProvider, CounterCallable>,
    ) -> Result<Infallible, HostCallError> {
        Err(HostFailure::new("external function stopped").into())
    }

    let provider = HostProviderModule::<ExternalProfile>::new("application", "main")
        .expect("provider module should be valid")
        .with_external_type::<CounterSchema>()
        .expect("external type should be valid")
        .with_scoped_diverging_function::<CounterProvider, (), CounterCallable, _>("stop", stop)
        .expect("diverging external function provider should be valid");
    let source = r#"
@external(erlang, "host", "Counter")
pub type Counter

@external(erlang, "host", "stop")
fn stop() -> fn(Int) -> Counter

pub fn main() {
  echo Nil as "before stop"
  stop()
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
        HostProviderSet::with_providers(Vec::<HostModule<ExternalProfile>>::new(), [provider])
            .expect("provider module should be unique"),
    )
    .expect("diverging external function source should compile");
    let plan = plan_host_program(typed).expect("diverging external function source should plan");
    let execution = HostedExecution::try_from_module_plan(plan)
        .expect("diverging external function execution should seal");
    let mut echoes = Vec::new();
    let error = execution
        .run_main(&mut ExternalRunState::default(), &mut echoes)
        .expect_err("diverging external function should fail");
    let ExecutionError::Host(error) = error else {
        panic!("diverging external function should produce a host error");
    };

    assert_eq!(error.package(), "application");
    assert_eq!(error.module(), "main");
    assert_eq!(error.function(), "stop");
    assert_eq!(error.failure().message(), "external function stopped");
    assert_eq!(echoes.len(), 1);
    assert_eq!(
        echoes[0].message().map(|message| message.as_str()),
        Some("before stop"),
    );
}

#[test]
fn external_function_block_stops_before_returning_its_callable() {
    fn new_counter<'call>(
        mut call: HostCall<'call, ExternalProfile, CounterProvider, HostCounter>,
        value: BigInt,
    ) -> Result<HostCallCompletion<'call, HostCounter>, HostCallError> {
        let counter = call.create_external(Counter { value });
        Ok(call.return_value(counter))
    }

    let provider = HostProviderModule::<ExternalProfile>::new("application", "main")
        .expect("provider module should be valid")
        .with_external_type::<CounterSchema>()
        .expect("external type should be valid")
        .with_scoped_function::<CounterProvider, (BigInt,), HostCounter, _>(
            "new_counter",
            new_counter,
        )
        .expect("external constructor should be valid");
    let source = r#"
@external(erlang, "host", "Counter")
pub type Counter

@external(erlang, "host", "new_counter")
fn new_counter(value: Int) -> Counter

fn stop() -> value {
  panic as "before external callable"
}

pub fn main() {
  let callable = {
    stop()
    fn(_value) { new_counter(1) }
  }
  callable
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
        HostProviderSet::with_providers(Vec::new(), [provider])
            .expect("provider module should be unique"),
    )
    .expect("external function block source should compile");
    let plan =
        plan_host_program(typed).expect("external function block source should plan completely");
    let execution = HostedExecution::try_from_module_plan(plan)
        .expect("external function block execution should seal");
    let error = execution
        .run_main(&mut ExternalRunState::default(), &mut Vec::new())
        .expect_err("the source panic should stop before the callable is returned");
    let ExecutionError::Panic(error) = error else {
        panic!("the block should preserve its source panic");
    };

    assert_eq!(error.kind(), PanicKind::Panic);
    assert_eq!(error.site().function(), "stop");
}

#[test]
fn external_function_block_rejects_an_unrepresentable_step_before_its_callable() {
    type Item = HostTypeParameter<0>;

    fn produce<'call>(
        _call: HostCall<'call, ExternalProfile, CounterProvider, Item>,
    ) -> Result<HostCallCompletion<'call, Item>, HostCallError> {
        Err(HostFailure::new("produce should not run").into())
    }

    fn new_counter<'call>(
        mut call: HostCall<'call, ExternalProfile, CounterProvider, HostCounter>,
        value: BigInt,
    ) -> Result<HostCallCompletion<'call, HostCounter>, HostCallError> {
        let counter = call.create_external(Counter { value });
        Ok(call.return_value(counter))
    }

    let provider = HostProviderModule::<ExternalProfile>::new("application", "main")
        .expect("provider module should be valid")
        .with_external_type::<CounterSchema>()
        .expect("external type should be valid")
        .with_scoped_function::<CounterProvider, (), Item, _>("produce", produce)
        .expect("generic producer should be valid")
        .with_scoped_function::<CounterProvider, (BigInt,), HostCounter, _>(
            "new_counter",
            new_counter,
        )
        .expect("external constructor should be valid");
    let source = r#"
@external(erlang, "host", "Counter")
pub type Counter

@external(erlang, "host", "produce")
fn produce() -> value

@external(erlang, "host", "new_counter")
fn new_counter(value: Int) -> Counter

pub fn main() {
  let callable = {
    let _ = produce()
    fn(_value) { new_counter(1) }
  }
  callable
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
        HostProviderSet::with_providers(Vec::new(), [provider])
            .expect("provider module should be unique"),
    )
    .expect("external function block source should compile");
    let plan =
        plan_host_program(typed).expect("external function block source should plan completely");
    let Err(error) = HostedExecution::try_from_module_plan(plan) else {
        panic!("the unresolved producer should prevent executable sealing");
    };

    assert_eq!(error.package(), "application");
    assert_eq!(error.module(), "main");
    assert_eq!(error.function(), "produce");
    assert!(error.signature().argument_types().is_empty());
    assert!(matches!(
        error.signature().return_(),
        geam::ValueType::Parameter(_)
    ));
}

#[test]
fn external_profile_preserves_a_nested_failure_from_a_never_callback() {
    type NeverReturn = HostTypeParameter<0>;
    type NeverCallable = HostFunctionType<NoArguments, NeverReturn>;

    fn invoke_never<'call>(
        mut call: HostCall<'call, ExternalProfile, CounterProvider, BigInt>,
        function: HostCallable<'call, NoArguments, NeverReturn>,
    ) -> Result<HostCallCompletion<'call, BigInt>, HostCallError> {
        let _ = call.invoke(function, ())?;
        Ok(call.return_value(0.into()))
    }

    let provider = HostProviderModule::<ExternalProfile>::new("application", "main")
        .expect("provider module should be valid")
        .with_scoped_function::<CounterProvider, (NeverCallable,), BigInt, _>(
            "invoke_never",
            invoke_never,
        )
        .expect("Never callback should register");
    let source = r#"
@external(erlang, "host", "invoke_never")
fn invoke_never(function: fn() -> value) -> Int

fn stop() -> value {
  panic as "nested Never callback"
}

pub fn main() {
  invoke_never(stop)
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
        HostProviderSet::with_providers(Vec::new(), [provider])
            .expect("provider module should be unique"),
    )
    .expect("Never callback source should compile");
    let plan = plan_host_program(typed).expect("Never callback source should plan");
    let execution =
        HostedExecution::try_from_module_plan(plan).expect("Never callback execution should seal");
    let error = execution
        .run_main(&mut ExternalRunState::default(), &mut Vec::new())
        .expect_err("nested Never callback should preserve its panic");
    let ExecutionError::Panic(error) = error else {
        panic!("nested Never callback should remain a source panic");
    };

    assert_eq!(error.kind(), PanicKind::Panic);
    assert_eq!(error.site().function(), "stop");
}

#[test]
fn creates_compares_and_returns_opaque_external_values() {
    fn new_counter<'call>(
        mut call: HostCall<'call, ExternalProfile, CounterProvider, HostCounter>,
        value: BigInt,
    ) -> Result<HostCallCompletion<'call, HostCounter>, HostCallError> {
        let counter = call.create_external(Counter { value });
        Ok(call.return_value(counter))
    }

    fn counter_value<'call>(
        call: HostCall<'call, ExternalProfile, CounterProvider, BigInt>,
        counter: HostExternal<'call, HostCounter>,
    ) -> Result<HostCallCompletion<'call, BigInt>, HostCallError> {
        let value = call.external_payload(counter).value.clone();
        Ok(call.return_value(value))
    }

    let provider = HostProviderModule::<ExternalProfile>::new("application", "main")
        .expect("provider module should be valid")
        .with_external_type::<CounterSchema>()
        .expect("external type should be valid")
        .with_scoped_function::<CounterProvider, (BigInt,), HostCounter, _>(
            "new_counter",
            new_counter,
        )
        .expect("constructor provider should be valid")
        .with_scoped_function::<CounterProvider, (HostCounter,), BigInt, _>(
            "counter_value",
            counter_value,
        )
        .expect("reader provider should be valid");
    let source = r#"
@external(erlang, "host", "Counter")
pub type Counter

@external(erlang, "host", "new_counter")
fn new_counter(value: Int) -> Counter

@external(erlang, "host", "counter_value")
fn counter_value(counter: Counter) -> Int

pub fn main() {
  let first = new_counter(42)
  let same_value = new_counter(42)
  echo first as "created"
  #(first, counter_value(first), first == first, first == same_value)
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
        HostProviderSet::with_providers(Vec::<HostModule<ExternalProfile>>::new(), [provider])
            .expect("provider module should be unique"),
    )
    .expect("external source should compile");
    let plan = plan_host_program(typed).expect("external source should plan");
    let planned_external = &plan.modules()[0].external_types()[0];
    assert_eq!(planned_external.name().package(), "application");
    assert_eq!(planned_external.name().module(), "main");
    assert_eq!(planned_external.name().name(), "Counter");
    assert!(planned_external.parameters().is_empty());
    let (external, echoes, read, identity_equal, source_equal) = {
        let execution =
            HostedExecution::try_from_module_plan(plan).expect("external execution should seal");
        let mut state = ExternalRunState::default();
        let mut echoes = Vec::new();
        let returned = execution
            .run_main(&mut state, &mut echoes)
            .expect("external source should execute");
        let Value::Tuple(mut values) = returned else {
            panic!("main should return an external value tuple");
        };
        let Value::Bool(source_equal) = values.pop().expect("source equality") else {
            panic!("last tuple field should be Bool");
        };
        let Value::Bool(identity_equal) = values.pop().expect("identity equality") else {
            panic!("third tuple field should be Bool");
        };
        let Value::Int(read) = values.pop().expect("read value") else {
            panic!("second tuple field should be Int");
        };
        let Value::External(external) = values.pop().expect("external value") else {
            panic!("first tuple field should be external");
        };

        (external, echoes, read, identity_equal, source_equal)
    };

    assert_eq!(read, BigInt::from(42));
    assert!(identity_equal);
    assert!(source_equal);
    assert_eq!(external.inspection(), "Counter(42)");
    assert_eq!(external.type_().type_name().package(), "application");
    assert_eq!(external.type_().type_name().module(), "main");
    assert_eq!(external.type_().type_name().name(), "Counter");
    assert_eq!(echoes.len(), 1);
    assert_eq!(
        echoes[0].message().map(|message| message.as_str()),
        Some("created"),
    );
    let Value::External(echoed) = echoes[0].value() else {
        panic!("echo should preserve the external value");
    };
    assert_eq!(echoed.identity(), external.identity());
    assert_eq!(echoed.inspection(), "Counter(42)");

    let cloned = external.clone();
    assert_eq!(cloned.identity(), external.identity());
    assert_eq!(cloned.inspection(), "Counter(42)");
}

#[test]
fn explains_external_targets_lists_and_captures() {
    fn new_counter<'call>(
        mut call: HostCall<'call, ExternalProfile, CounterProvider, HostCounter>,
        value: BigInt,
    ) -> Result<HostCallCompletion<'call, HostCounter>, HostCallError> {
        let counter = call.create_external(Counter { value });
        Ok(call.return_value(counter))
    }

    let provider = HostProviderModule::<ExternalProfile>::new("application", "main")
        .expect("provider module should be valid")
        .with_external_type::<CounterSchema>()
        .expect("external type should be valid")
        .with_scoped_function::<CounterProvider, (BigInt,), HostCounter, _>(
            "new_counter",
            new_counter,
        )
        .expect("constructor provider should be valid");
    let source = r#"
@external(erlang, "host", "Counter")
pub type Counter

@external(erlang, "host", "new_counter")
fn new_counter(value: Int) -> Counter

fn preserve(
  counter: Counter,
  values: List(Counter),
  maker: fn(Int) -> Counter,
) {
  fn() { #(counter, values, maker) }
}

pub fn main() {
  let counter = new_counter(1)
  let values = [counter]
  let maker = new_counter
  preserve(counter, values, maker)
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
        HostProviderSet::with_providers(Vec::<HostModule<ExternalProfile>>::new(), [provider])
            .expect("provider module should be unique"),
    )
    .expect("external source should compile");
    let plan = plan_host_program(typed).expect("external source should plan");
    let execution =
        HostedExecution::try_from_module_plan(plan).expect("external execution should seal");
    let expected = r#"
module main
main function.tuple#0

function external#0
  host application::main.new_counter signature=fn(Int) -> external_type#0

function tuple#0
  entry b0 params=[] captures=[%external#0:shape#1(external_type#0), %list.external#0:shape#2(list_type#0), %function.external#0:shape#3(fn(Int) -> external_type#0)]
  block b0 params=[%external#0:shape#1(external_type#0), %list.external#0:shape#2(list_type#0), %function.external#0:shape#3(fn(Int) -> external_type#0)]
    %tuple#0:shape#4(#(external_type#0, list_type#0, fn(Int) -> external_type#0)) = tuple.value elements=[%external#0, %list.external#0, %function.external#0]
    return %tuple#0

function function.tuple#0
  entry b0 params=[] captures=[]
  block b0 params=[]
    %int#0:shape#0(Int) = int.value 1
    %external#0:shape#1(external_type#0) = external.call external#0 args=[%int#0]
    %list.external#0:shape#2(list_type#0) = list.external[type#0] value elements=[%external#0]
    %function.external#0:shape#3(fn(Int) -> external_type#0) = function[External] reference external#0
    tail function.tuple#1 args=[%external#0, %list.external#0, %function.external#0]

function function.tuple#1
  entry b0 params=[%external#0:shape#1(external_type#0), %list.external#0:shape#2(list_type#0), %function.external#0:shape#3(fn(Int) -> external_type#0)] captures=[]
  block b0 params=[%external#0:shape#1(external_type#0), %list.external#0:shape#2(list_type#0), %function.external#0:shape#3(fn(Int) -> external_type#0)]
    %function.tuple#0:shape#5(fn() -> #(external_type#0, list_type#0, fn(Int) -> external_type#0)) = function[Tuple] closure target=tuple#0 captures=[%external#0<-%external#0, %list.external#0<-%list.external#0, %function.external#0<-%function.external#0]
    return %function.tuple#0
"#;

    assert_eq!(execution.explain().to_string().trim(), expected.trim());
}

#[test]
fn external_storage_profiles_preserve_existing_runtime_families() {
    fn new_counter<'call>(
        mut call: HostCall<'call, ExternalProfile, CounterProvider, HostCounter>,
        value: BigInt,
    ) -> Result<HostCallCompletion<'call, HostCounter>, HostCallError> {
        let counter = call.create_external(Counter { value });
        Ok(call.return_value(counter))
    }

    let scalars = HostModule::<ExternalProfile>::new_for_profile("host_support", "host/scalars")
        .expect("host module should be valid")
        .with_function("int", |value: BigInt| value)
        .expect("Int host function should be valid")
        .with_function("float", |value: f64| value)
        .expect("Float host function should be valid")
        .with_function("string", |value: EcoString| value)
        .expect("String host function should be valid")
        .with_function("bit_array", |value: BitArrayValue| value)
        .expect("BitArray host function should be valid")
        .with_function("utf_codepoint", |value: char| value)
        .expect("UtfCodepoint host function should be valid")
        .with_function("bool", |value: bool| value)
        .expect("Bool host function should be valid")
        .with_function("nil", |(): ()| ())
        .expect("Nil host function should be valid");
    let provider = HostProviderModule::<ExternalProfile>::new("application", "main")
        .expect("provider module should be valid")
        .with_external_type::<CounterSchema>()
        .expect("external type should be valid")
        .with_scoped_function::<CounterProvider, (BigInt,), HostCounter, _>(
            "new_counter",
            new_counter,
        )
        .expect("constructor provider should be valid");
    let source = r#"
import host/scalars

@external(erlang, "host", "Counter")
pub type Counter

pub type Boxed {
  Boxed(Int)
}

@external(erlang, "host", "new_counter")
fn new_counter(value: Int) -> Counter

fn custom_value() -> Boxed {
  Boxed(1)
}

fn tuple_value() -> #(Int) {
  #(1)
}

fn list_value() -> List(Int) {
  [1]
}

fn function_value() -> fn(Int) -> Int {
  scalars.int
}

pub fn main() {
  let assert <<codepoint:utf8_codepoint>> = <<65>>
  let int = scalars.int(1)
  let float = scalars.float(1.0)
  let string = scalars.string("one")
  let bit_array = scalars.bit_array(<<1>>)
  let utf_codepoint = scalars.utf_codepoint(codepoint)
  let custom = custom_value()
  let external = new_counter(8)
  let bool = scalars.bool(True)
  let nil = scalars.nil(Nil)
  let tuple = tuple_value()
  let list = list_value()
  let function = scalars.int

  echo #(
    int,
    float,
    string,
    bit_array,
    utf_codepoint,
    custom,
    external,
    bool,
    nil,
    tuple,
    list,
    function,
  ) as "values"

  let int_list = [int]
  let float_list = [float]
  let string_list = [string]
  let bit_array_list = [bit_array]
  let utf_codepoint_list = [utf_codepoint]
  let custom_list = [custom]
  let external_list = [external]
  let bool_list = [bool]
  let nil_list = [nil]
  let tuple_list = [tuple]
  let nested_list = [list]
  let function_list = [function]

  echo #(
    int_list,
    float_list,
    string_list,
    bit_array_list,
    utf_codepoint_list,
    custom_list,
    external_list,
    bool_list,
    nil_list,
    tuple_list,
    nested_list,
    function_list,
  ) as "lists"

  echo #(
    scalars.int,
    scalars.float,
    scalars.string,
    scalars.bit_array,
    scalars.utf_codepoint,
    custom_value,
    new_counter,
    scalars.bool,
    scalars.nil,
    tuple_value,
    list_value,
    function_value,
  ) as "functions"

  echo #(
    fn() { int },
    fn() { float },
    fn() { string },
    fn() { bit_array },
    fn() { utf_codepoint },
    fn() { custom },
    fn() { external },
    fn() { bool },
    fn() { nil },
    fn() { tuple },
    fn() { list },
    fn() { function },
  ) as "value captures"

  echo #(
    fn() { int_list },
    fn() { float_list },
    fn() { string_list },
    fn() { bit_array_list },
    fn() { utf_codepoint_list },
    fn() { custom_list },
    fn() { external_list },
    fn() { bool_list },
    fn() { nil_list },
    fn() { tuple_list },
    fn() { nested_list },
    fn() { function_list },
  ) as "list captures"

  echo #(
    fn() { scalars.int },
    fn() { scalars.float },
    fn() { scalars.string },
    fn() { scalars.bit_array },
    fn() { scalars.utf_codepoint },
    fn() { custom_value },
    fn() { Boxed },
    fn() { new_counter },
    fn() { scalars.bool },
    fn() { scalars.nil },
    fn() { tuple_value },
    fn() { list_value },
    fn() { function_value },
  ) as "function captures"

  external
}
"#;
    let typed = compile_typed_host_program(
        "application",
        "main",
        [
            PackageSource::new(
                "application",
                ["host_support"],
                [ModuleSource::new("main", "src/main.gleam", source)],
            ),
            PackageSource::new(
                "host_support",
                Vec::<&str>::new(),
                Vec::<ModuleSource>::new(),
            ),
        ],
        HostProviderSet::with_providers([scalars], [provider])
            .expect("host modules should be unique"),
    )
    .expect("external source should compile");
    let plan = plan_host_program(typed).expect("external source should plan");
    let execution =
        HostedExecution::try_from_module_plan(plan).expect("external execution should seal");
    let mut echoes = Vec::new();
    let returned = execution
        .run_main(&mut ExternalRunState::default(), &mut echoes)
        .expect("external source should execute");
    let Value::External(counter) = returned else {
        panic!("main should return an external value");
    };

    let expected_values = r#"#(1, 1.0, "one", <<1>>, 'A', Boxed(1), Counter(8), True, Nil, #(1), [1], //fn(a) { ... })"#;
    let expected_lists = r#"#([1], [1.0], ["one"], [<<1>>], ['A'], [Boxed(1)], [Counter(8)], [True], [Nil], [#(1)], [[1]], [//fn(a) { ... }])"#;
    let expected_functions = r#"#(//fn(a) { ... }, //fn(a) { ... }, //fn(a) { ... }, //fn(a) { ... }, //fn(a) { ... }, //fn() { ... }, //fn(a) { ... }, //fn(a) { ... }, //fn(a) { ... }, //fn() { ... }, //fn() { ... }, //fn() { ... })"#;

    assert_eq!(echoes.len(), 6);
    assert_eq!(echoes[0].value().inspect().to_string(), expected_values);
    assert_eq!(echoes[1].value().inspect().to_string(), expected_lists);
    assert_eq!(echoes[2].value().inspect().to_string(), expected_functions,);
    let Value::Tuple(captures) = echoes[3].value() else {
        panic!("value captures echo should contain a tuple");
    };
    assert_eq!(captures.len(), 12);
    assert!(
        captures
            .iter()
            .all(|capture| capture.inspect().to_string() == "//fn() { ... }"),
    );
    let Value::Tuple(captures) = echoes[4].value() else {
        panic!("list captures echo should contain a tuple");
    };
    assert_eq!(captures.len(), 12);
    assert!(
        captures
            .iter()
            .all(|capture| capture.inspect().to_string() == "//fn() { ... }"),
    );
    let Value::Tuple(captures) = echoes[5].value() else {
        panic!("function captures echo should contain a tuple");
    };
    assert_eq!(captures.len(), 13);
    assert!(
        captures
            .iter()
            .all(|capture| capture.inspect().to_string() == "//fn() { ... }"),
    );
    assert_eq!(counter.inspection(), "Counter(8)");
}

#[test]
fn specializes_generic_external_tail_calls_and_list_cases() {
    fn new_counter<'call>(
        mut call: HostCall<'call, ExternalProfile, CounterProvider, HostCounter>,
        value: BigInt,
    ) -> Result<HostCallCompletion<'call, HostCounter>, HostCallError> {
        let counter = call.create_external(Counter { value });
        Ok(call.return_value(counter))
    }

    let provider = HostProviderModule::<ExternalProfile>::new("application", "main")
        .expect("provider module should be valid")
        .with_external_type::<CounterSchema>()
        .expect("external type should be valid")
        .with_scoped_function::<CounterProvider, (BigInt,), HostCounter, _>(
            "new_counter",
            new_counter,
        )
        .expect("constructor provider should be valid");
    let source = r#"
@external(erlang, "host", "Counter")
pub type Counter

@external(erlang, "host", "new_counter")
fn new_counter(value: Int) -> Counter

fn identity(value) {
  value
}

fn forward(value) {
  identity(value)
}

fn list_identity(values: List(value)) -> List(value) {
  values
}

fn forward_list(values: List(value)) -> List(value) {
  list_identity(values)
}

fn thunk(value) -> fn() -> value {
  fn() { value }
}

fn forward_thunk(value) -> fn() -> value {
  thunk(value)
}

fn list_thunk(value) -> fn() -> List(value) {
  fn() { [value] }
}

fn forward_list_thunk(value) -> fn() -> List(value) {
  list_thunk(value)
}

fn choose_list(selected: Bool, value: Counter) -> List(Counter) {
  case selected {
    True -> [value]
    False -> []
  }
}

pub fn main() {
  let counter = new_counter(9)
  #(
    forward(counter),
    forward([counter]),
    forward_list([counter]),
    forward_thunk(counter)(),
    forward_list_thunk(counter)(),
    forward(fn() { [counter] })(),
    choose_list(True, counter),
  )
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
        HostProviderSet::with_providers(Vec::<HostModule<ExternalProfile>>::new(), [provider])
            .expect("provider module should be unique"),
    )
    .expect("generic external source should compile");
    let plan = plan_host_program(typed).expect("generic external source should plan");
    let execution = HostedExecution::try_from_module_plan(plan)
        .expect("generic external execution should seal");
    let returned = execution
        .run_main(&mut ExternalRunState::default(), &mut Vec::new())
        .expect("generic external source should execute");

    assert_eq!(
        returned.inspect().to_string(),
        "#(Counter(9), [Counter(9)], [Counter(9)], Counter(9), [Counter(9)], [Counter(9)], [Counter(9)])",
    );
}

#[test]
fn specializes_symbolic_external_function_constants_and_capturing_closures() {
    fn new_counter<'call>(
        mut call: HostCall<'call, ExternalProfile, CounterProvider, HostCounter>,
        value: BigInt,
    ) -> Result<HostCallCompletion<'call, HostCounter>, HostCallError> {
        let counter = call.create_external(Counter { value });
        Ok(call.return_value(counter))
    }

    let provider = HostProviderModule::<ExternalProfile>::new("application", "main")
        .expect("provider module should be valid")
        .with_external_type::<CounterSchema>()
        .expect("external type should be valid")
        .with_scoped_function::<CounterProvider, (BigInt,), HostCounter, _>(
            "new_counter",
            new_counter,
        )
        .expect("constructor provider should be valid");
    let source = r#"
@external(erlang, "host", "Counter")
pub type Counter

pub type Never

@external(erlang, "host", "new_counter")
fn new_counter(value: Int) -> Counter

const keep_constant = keep

fn keep(_ignored: Never) -> Counter {
  new_counter(11)
}

fn capture(counter: Counter) -> fn(Never) -> Counter {
  fn(_ignored) { counter }
}

pub fn main() {
  let counter = new_counter(9)
  #(keep_constant, capture(counter))
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
        HostProviderSet::with_providers(Vec::<HostModule<ExternalProfile>>::new(), [provider])
            .expect("provider module should be unique"),
    )
    .expect("symbolic external callable source should compile");
    let plan = plan_host_program(typed).expect("symbolic external callable source should plan");
    let execution = HostedExecution::try_from_module_plan(plan)
        .expect("symbolic external callable execution should seal");
    let returned = execution
        .run_main(&mut ExternalRunState::default(), &mut Vec::new())
        .expect("symbolic external callable source should execute");

    assert_eq!(
        returned.inspect().to_string(),
        "#(//fn(a) { ... }, //fn(a) { ... })",
    );
}

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
fn preserves_external_values_through_lists_customs_captures_and_calls() {
    fn new_counter<'call>(
        mut call: HostCall<'call, ExternalProfile, CounterProvider, HostCounter>,
        value: BigInt,
    ) -> Result<HostCallCompletion<'call, HostCounter>, HostCallError> {
        let counter = call.create_external(Counter { value });
        Ok(call.return_value(counter))
    }

    fn counter_value<'call>(
        call: HostCall<'call, ExternalProfile, CounterProvider, BigInt>,
        counter: HostExternal<'call, HostCounter>,
    ) -> Result<HostCallCompletion<'call, BigInt>, HostCallError> {
        let value = call.external_payload(counter).value.clone();
        Ok(call.return_value(value))
    }

    fn first_counter<'call>(
        mut call: HostCall<'call, ExternalProfile, CounterProvider, HostCounter>,
        values: HostList<'call, HostCounter>,
    ) -> Result<HostCallCompletion<'call, HostCounter>, HostCallError> {
        let first = call
            .list_item(values, 0)
            .ok_or_else(|| HostFailure::new("counter list should not be empty"))?;
        Ok(call.return_value(first))
    }

    fn duplicate_counter<'call>(
        call: HostCall<'call, ExternalProfile, CounterProvider, HostListType<HostCounter>>,
        counter: HostExternal<'call, HostCounter>,
    ) -> Result<HostCallCompletion<'call, HostListType<HostCounter>>, HostCallError> {
        Ok(call.return_list([counter, counter]))
    }

    type IntArguments = HostTypeList<BigInt, HostTypeListEnd>;
    type CounterCallable = HostFunctionType<IntArguments, HostCounter>;

    fn invoke_counter<'call>(
        mut call: HostCall<'call, ExternalProfile, CounterProvider, HostCounter>,
        function: HostCallable<'call, IntArguments, HostCounter>,
        value: BigInt,
    ) -> Result<HostCallCompletion<'call, HostCounter>, HostCallError> {
        let counter = call.invoke(function, (value, ()))?;
        Ok(call.return_value(counter))
    }

    fn forward_counter_function<'call>(
        call: HostCall<'call, ExternalProfile, CounterProvider, CounterCallable>,
        function: HostCallable<'call, IntArguments, HostCounter>,
    ) -> Result<HostCallCompletion<'call, CounterCallable>, HostCallError> {
        Ok(call.return_value(function))
    }

    fn counter_from_wrapped<'call>(
        mut call: HostCall<'call, ExternalProfile, CounterProvider, BigInt>,
        wrapped: HostCustom<'call, HostWrappedCounter>,
    ) -> Result<HostCallCompletion<'call, BigInt>, HostCallError> {
        let (counter, ()) = call
            .custom_fields::<HostWrappedCounterConstructor>(wrapped)
            .ok_or_else(|| HostFailure::new("wrapped counter should use Wrapped"))?;
        let value = call.external_payload(counter).value.clone();
        Ok(call.return_value(value))
    }

    let provider = HostProviderModule::<ExternalProfile>::new("application", "main")
        .expect("provider module should be valid")
        .with_external_type::<CounterSchema>()
        .expect("external type should be valid")
        .with_scoped_function::<CounterProvider, (BigInt,), HostCounter, _>(
            "new_counter",
            new_counter,
        )
        .expect("constructor provider should be valid")
        .with_scoped_function::<CounterProvider, (HostCounter,), BigInt, _>(
            "counter_value",
            counter_value,
        )
        .expect("reader provider should be valid")
        .with_scoped_function::<CounterProvider, (HostListType<HostCounter>,), HostCounter, _>(
            "first_counter",
            first_counter,
        )
        .expect("external list reader provider should be valid")
        .with_scoped_function::<CounterProvider, (HostCounter,), HostListType<HostCounter>, _>(
            "duplicate_counter",
            duplicate_counter,
        )
        .expect("external list builder provider should be valid")
        .with_scoped_function::<CounterProvider, (CounterCallable, BigInt), HostCounter, _>(
            "invoke_counter",
            invoke_counter,
        )
        .expect("external callback provider should be valid")
        .with_scoped_function::<CounterProvider, (CounterCallable,), CounterCallable, _>(
            "forward_counter_function",
            forward_counter_function,
        )
        .expect("external callback forwarding provider should be valid")
        .with_scoped_function::<CounterProvider, (HostWrappedCounter,), BigInt, _>(
            "counter_from_wrapped",
            counter_from_wrapped,
        )
        .expect("external custom-field provider should be valid");
    let source = r#"
@external(erlang, "host", "Counter")
pub type Counter

pub type Wrapped {
  Wrapped(value: Counter)
}

pub type ValueBox(value) {
  ValueBox(value: value)
}

pub type ListBox(value) {
  ListBox(values: List(value))
}

@external(erlang, "host", "new_counter")
fn new_counter(value: Int) -> Counter

@external(erlang, "host", "counter_value")
fn counter_value(counter: Counter) -> Int

@external(erlang, "host", "first_counter")
fn first_counter(values: List(Counter)) -> Counter

@external(erlang, "host", "duplicate_counter")
fn duplicate_counter(counter: Counter) -> List(Counter)

@external(erlang, "host", "invoke_counter")
fn invoke_counter(function: fn(Int) -> Counter, value: Int) -> Counter

@external(erlang, "host", "forward_counter_function")
fn forward_counter_function(function: fn(Int) -> Counter) -> fn(Int) -> Counter

@external(erlang, "host", "counter_from_wrapped")
fn counter_from_wrapped(wrapped: Wrapped) -> Int

fn identity(value: value) -> value {
  value
}

fn transform_value(
  value: value,
  fallback: value,
  mapper: fn(value) -> value,
) -> value {
  let local = value
  let direct = identity(local)
  let called = mapper(direct)
  let tuple = #(called)
  let from_tuple = tuple.0
  let boxed = ValueBox(value: from_tuple)
  let from_field = boxed.value
  let from_list = case [from_field] {
    [first] -> first
    _ -> fallback
  }
  let from_int = case 1 {
    1 -> from_list
    _ -> fallback
  }
  let from_string = case "selected" {
    "selected" -> from_int
    _ -> fallback
  }
  let from_float = case 1.0 {
    1.0 -> from_string
    _ -> fallback
  }
  let from_block = {
    let _ = Nil
    from_float
  }
  case True {
    True -> from_block
    False -> panic as "unselected generic external value"
  }
}

fn forward_list(values: List(value)) -> List(value) {
  values
}

fn provide_list(values: List(value)) -> fn() -> List(value) {
  fn() { values }
}

fn transform_list(values: List(value)) -> List(value) {
  let direct = forward_list(values)
  let provider = provide_list(direct)
  let called = provider()
  let tuple = #(called)
  let from_tuple = tuple.0
  let boxed = ListBox(values: from_tuple)
  let from_field = boxed.values
  let from_nested = case [from_field] {
    [first] -> first
    _ -> values
  }
  let from_int = case 1 {
    1 -> from_nested
    _ -> values
  }
  let from_string = case "selected" {
    "selected" -> from_int
    _ -> values
  }
  let from_float = case 1.0 {
    1.0 -> from_string
    _ -> values
  }
  let from_block = {
    let _ = Nil
    from_float
  }
  case True {
    True -> from_block
    False -> panic as "unselected generic external list"
  }
}

fn prepend(value: value, values: List(value)) -> List(value) {
  [value, ..values]
}

fn drop_generic(values: List(value)) -> List(value) {
  case values {
    [_, ..tail] -> tail
    _ -> []
  }
}

fn tail_counter(value: Int) -> Counter {
  new_counter(value)
}

fn captured_reader(counter: Counter) -> fn() -> Int {
  fn() { counter_value(counter) }
}

fn bind_external(counter: Counter) -> Counter {
  case counter {
    selected -> selected
  }
}

fn discard_external(counter: Counter, fallback: Counter) -> Counter {
  case counter {
    _ -> fallback
  }
}

fn alias_external(counter: Counter) -> Counter {
  case counter {
    selected as alias -> {
      let _ = selected
      alias
    }
  }
}

fn preserve_external(counter: Counter) -> fn() -> Counter {
  fn() { counter }
}

fn preserve_external_list(values: List(Counter)) -> fn() -> List(Counter) {
  fn() { values }
}

fn preserve_external_function(function: fn(Int) -> Counter) -> fn() -> fn(Int) -> Counter {
  fn() { function }
}

pub fn main() {
  let make = new_counter
  let first = make(10)
  let second = tail_counter(20)
  let values = [first, second]
  let identity_values = identity(values)
  echo identity_values as "external list"
  let wrapped = Wrapped(second)
  let read = captured_reader(first)
  let transformed = transform_value(first, second, identity)
  let transformed_values = transform_list(identity_values)
  let prepended = prepend(transformed, transformed_values)
  let remaining = drop_generic(prepended)
  #(
    values,
    wrapped,
    read(),
    transformed,
    remaining,
    preserve_external(first),
    preserve_external_list(values),
    preserve_external_function(make),
    bind_external(first),
    discard_external(first, second),
    alias_external(first),
    first_counter(values),
    duplicate_counter(first),
    invoke_counter(new_counter, 30),
    forward_counter_function(make)(40),
    counter_from_wrapped(wrapped),
  )
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
        HostProviderSet::with_providers(Vec::<HostModule<ExternalProfile>>::new(), [provider])
            .expect("provider module should be unique"),
    )
    .expect("external source should compile");
    let plan = plan_host_program(typed).expect("external source should plan");
    let execution =
        HostedExecution::try_from_module_plan(plan).expect("external execution should seal");

    let mut echoes = Vec::new();
    let returned = execution
        .run_main(&mut ExternalRunState::default(), &mut echoes)
        .expect("external source should execute");
    drop(execution);
    assert_eq!(echoes.len(), 1);
    assert_eq!(
        echoes[0].message().map(|message| message.as_str()),
        Some("external list"),
    );
    assert_eq!(
        echoes[0].value().inspect().to_string(),
        "[Counter(10), Counter(20)]",
    );
    let Value::Tuple(values) = returned else {
        panic!("main should return a tuple");
    };
    let Value::List(counters) = &values[0] else {
        panic!("first tuple field should be a list");
    };
    assert_eq!(
        counters
            .to_values()
            .iter()
            .map(|value| match value {
                Value::External(value) => value.inspection().as_str(),
                _ => panic!("list item should be external"),
            })
            .collect::<Vec<_>>(),
        ["Counter(10)", "Counter(20)"],
    );
    let Value::Custom(wrapped) = &values[1] else {
        panic!("second tuple field should be custom");
    };
    let Value::External(wrapped_counter) = wrapped.fields()[0].value() else {
        panic!("custom field should be external");
    };
    assert_eq!(wrapped_counter.inspection(), "Counter(20)");
    assert_eq!(values[2], Value::Int(BigInt::from(10)));
    let Value::External(transformed) = &values[3] else {
        panic!("fourth tuple field should be external");
    };
    assert_eq!(transformed.inspection(), "Counter(10)");
    let Value::List(remaining) = &values[4] else {
        panic!("fifth tuple field should be a list");
    };
    assert_eq!(
        remaining
            .to_values()
            .iter()
            .map(|value| match value {
                Value::External(value) => value.inspection().as_str(),
                _ => panic!("remaining list item should be external"),
            })
            .collect::<Vec<_>>(),
        ["Counter(10)", "Counter(20)"],
    );
    assert_eq!(values[5].inspect().to_string(), "//fn() { ... }");
    assert_eq!(values[6].inspect().to_string(), "//fn() { ... }");
    assert_eq!(values[7].inspect().to_string(), "//fn() { ... }");
    let Value::External(bound) = &values[8] else {
        panic!("ninth tuple field should be external");
    };
    assert_eq!(bound.inspection(), "Counter(10)");
    let Value::External(discarded) = &values[9] else {
        panic!("tenth tuple field should be external");
    };
    assert_eq!(discarded.inspection(), "Counter(20)");
    let Value::External(aliased) = &values[10] else {
        panic!("eleventh tuple field should be external");
    };
    assert_eq!(aliased.inspection(), "Counter(10)");
    let Value::External(from_list) = &values[11] else {
        panic!("twelfth tuple field should be external");
    };
    assert_eq!(from_list.inspection(), "Counter(10)");
    let Value::List(duplicated) = &values[12] else {
        panic!("thirteenth tuple field should be a list");
    };
    assert_eq!(
        duplicated
            .to_values()
            .iter()
            .map(|value| match value {
                Value::External(value) => value.inspection().as_str(),
                _ => panic!("duplicated list item should be external"),
            })
            .collect::<Vec<_>>(),
        ["Counter(10)", "Counter(10)"],
    );
    let Value::External(from_callback) = &values[13] else {
        panic!("fourteenth tuple field should be external");
    };
    assert_eq!(from_callback.inspection(), "Counter(30)");
    let Value::External(from_forwarded_callback) = &values[14] else {
        panic!("fifteenth tuple field should be external");
    };
    assert_eq!(from_forwarded_callback.inspection(), "Counter(40)");
    assert_eq!(values[15], Value::Int(BigInt::from(20)));
}

#[test]
fn passes_external_values_and_lists_through_scoped_callbacks() {
    type CounterArguments = HostTypeList<HostCounter, HostTypeListEnd>;
    type CounterReader = HostFunctionType<CounterArguments, BigInt>;
    type CounterListFunction = HostFunctionType<CounterArguments, HostListType<HostCounter>>;
    type CounterTuple = HostTupleType<CounterArguments>;

    fn new_counter<'call>(
        mut call: HostCall<'call, ExternalProfile, CounterProvider, HostCounter>,
        value: BigInt,
    ) -> Result<HostCallCompletion<'call, HostCounter>, HostCallError> {
        let counter = call.create_external(Counter { value });
        Ok(call.return_value(counter))
    }

    fn read_counter<'call>(
        call: HostCall<'call, ExternalProfile, CounterProvider, BigInt>,
        counter: HostExternal<'call, HostCounter>,
    ) -> Result<HostCallCompletion<'call, BigInt>, HostCallError> {
        let value = call.external_payload(counter).value.clone();
        Ok(call.return_value(value))
    }

    fn duplicate<'call>(
        call: HostCall<'call, ExternalProfile, CounterProvider, HostListType<HostCounter>>,
        counter: HostExternal<'call, HostCounter>,
    ) -> Result<HostCallCompletion<'call, HostListType<HostCounter>>, HostCallError> {
        Ok(call.return_list([counter, counter]))
    }

    fn wrap<'call>(
        call: HostCall<'call, ExternalProfile, CounterProvider, CounterTuple>,
        counter: HostExternal<'call, HostCounter>,
    ) -> Result<HostCallCompletion<'call, CounterTuple>, HostCallError> {
        Ok(call.return_tuple((counter, ())))
    }

    fn invoke_reader<'call>(
        mut call: HostCall<'call, ExternalProfile, CounterProvider, BigInt>,
        function: HostCallable<'call, CounterArguments, BigInt>,
        counter: HostExternal<'call, HostCounter>,
    ) -> Result<HostCallCompletion<'call, BigInt>, HostCallError> {
        let value = call.invoke(function, (counter, ()))?;
        Ok(call.return_value(value))
    }

    fn invoke_list<'call>(
        mut call: HostCall<'call, ExternalProfile, CounterProvider, HostListType<HostCounter>>,
        function: HostCallable<'call, CounterArguments, HostListType<HostCounter>>,
        counter: HostExternal<'call, HostCounter>,
    ) -> Result<HostCallCompletion<'call, HostListType<HostCounter>>, HostCallError> {
        let values = call.invoke(function, (counter, ()))?;
        let first = call
            .list_item(values, 0)
            .ok_or_else(|| HostFailure::new("callback list should not be empty"))?;
        Ok(call.return_list([first]))
    }

    fn forward_list_function<'call>(
        call: HostCall<'call, ExternalProfile, CounterProvider, CounterListFunction>,
        function: HostCallable<'call, CounterArguments, HostListType<HostCounter>>,
    ) -> Result<HostCallCompletion<'call, CounterListFunction>, HostCallError> {
        Ok(call.return_value(function))
    }

    let provider = HostProviderModule::<ExternalProfile>::new("application", "main")
        .expect("provider module should be valid")
        .with_external_type::<CounterSchema>()
        .expect("external type should be valid")
        .with_scoped_function::<CounterProvider, (BigInt,), HostCounter, _>(
            "new_counter",
            new_counter,
        )
        .expect("counter constructor should be valid")
        .with_scoped_function::<CounterProvider, (HostCounter,), BigInt, _>(
            "read_counter",
            read_counter,
        )
        .expect("counter reader should be valid")
        .with_scoped_function::<CounterProvider, (HostCounter,), HostListType<HostCounter>, _>(
            "duplicate",
            duplicate,
        )
        .expect("counter list builder should be valid")
        .with_scoped_function::<CounterProvider, (HostCounter,), CounterTuple, _>("wrap", wrap)
        .expect("counter tuple builder should be valid")
        .with_scoped_function::<CounterProvider, (CounterReader, HostCounter), BigInt, _>(
            "invoke_reader",
            invoke_reader,
        )
        .expect("counter reader callback should be valid")
        .with_scoped_function::<
            CounterProvider,
            (CounterListFunction, HostCounter),
            HostListType<HostCounter>,
            _,
        >("invoke_list", invoke_list)
        .expect("counter list callback should be valid")
        .with_scoped_function::<
            CounterProvider,
            (CounterListFunction,),
            CounterListFunction,
            _,
        >("forward_list_function", forward_list_function)
        .expect("counter list callback forwarding should be valid");
    let source = r#"
@external(erlang, "host", "Counter")
pub type Counter

@external(erlang, "host", "new_counter")
fn new_counter(value: Int) -> Counter

@external(erlang, "host", "read_counter")
fn read_counter(counter: Counter) -> Int

@external(erlang, "host", "duplicate")
fn duplicate(counter: Counter) -> List(Counter)

@external(erlang, "host", "wrap")
fn wrap(counter: Counter) -> #(Counter)

@external(erlang, "host", "invoke_reader")
fn invoke_reader(function: fn(Counter) -> Int, counter: Counter) -> Int

@external(erlang, "host", "invoke_list")
fn invoke_list(
  function: fn(Counter) -> List(Counter),
  counter: Counter,
) -> List(Counter)

@external(erlang, "host", "forward_list_function")
fn forward_list_function(
  function: fn(Counter) -> List(Counter),
) -> fn(Counter) -> List(Counter)

fn tail_list(counter: Counter) -> List(Counter) {
  duplicate(counter)
}

pub fn main() {
  let counter = new_counter(7)
  let reader = read_counter
  let list_function = duplicate
  let forwarded_list_function = forward_list_function(list_function)
  let wrapped = wrap(counter)
  #(
    invoke_reader(reader, wrapped.0),
    invoke_list(list_function, counter),
    tail_list(counter),
    list_function(counter),
    forwarded_list_function(counter),
  )
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
        HostProviderSet::with_providers(Vec::<HostModule<ExternalProfile>>::new(), [provider])
            .expect("provider module should be unique"),
    )
    .expect("external callback source should compile");
    let plan = plan_host_program(typed).expect("external callback source should plan");
    let execution =
        HostedExecution::try_from_module_plan(plan).expect("external callback source should seal");

    assert_eq!(
        execution
            .run_main(&mut ExternalRunState::default(), &mut Vec::new())
            .expect("external callback source should execute")
            .inspect()
            .to_string(),
        "#(7, [Counter(7)], [Counter(7), Counter(7)], [Counter(7), Counter(7)], [Counter(7), Counter(7)])",
    );
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
        .with_external_type::<DependencyCounterSchema>()
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
fn executes_external_values_across_expression_and_function_table_boundaries() {
    fn new_counter<'call>(
        mut call: HostCall<'call, ExternalProfile, CounterProvider, HostCounter>,
        value: BigInt,
    ) -> Result<HostCallCompletion<'call, HostCounter>, HostCallError> {
        let counter = call.create_external(Counter { value });
        Ok(call.return_value(counter))
    }

    let provider = HostProviderModule::<ExternalProfile>::new("application", "main")
        .expect("provider module should be valid")
        .with_external_type::<CounterSchema>()
        .expect("external type should be valid")
        .with_scoped_function::<CounterProvider, (BigInt,), HostCounter, _>(
            "new_counter",
            new_counter,
        )
        .expect("constructor provider should be valid");
    let source = r#"
@external(erlang, "host", "Counter")
pub type Counter

pub type Wrapped {
  Wrapped(value: Counter)
}

@external(erlang, "host", "new_counter")
fn new_counter(value: Int) -> Counter

const make_counter = new_counter

fn from_bool(first: Counter, second: Counter, value: Bool) -> Counter {
  case value {
    True -> first
    False -> second
  }
}

fn from_int(first: Counter, second: Counter, value: Int) -> Counter {
  case value {
    1 -> first
    _ -> second
  }
}

fn from_float(first: Counter, second: Counter, value: Float) -> Counter {
  case value {
    1.5 -> first
    _ -> second
  }
}

fn from_string(first: Counter, second: Counter, value: String) -> Counter {
  case value {
    "first" -> first
    _ -> second
  }
}

fn from_tuple(value: Counter) -> Counter {
  #(value, 1).0
}

fn from_custom(value: Counter) -> Counter {
  Wrapped(value).value
}

fn from_list(value: Counter) -> Counter {
  let assert [head] = [value]
  head
}

fn from_function(value: Counter) -> Counter {
  let read = fn() { value }
  read()
}

fn from_block(value: Counter) -> Counter {
  {
    let returned = value
    returned
  }
}

fn with_unreached_panic(value: Counter) -> Counter {
  case True {
    True -> value
    False -> panic
  }
}

fn external_value() -> Counter {
  new_counter(11)
}

fn external_list() -> List(Counter) {
  [external_value()]
}

fn external_function() -> fn() -> Counter {
  external_value
}

fn nested_external_function() -> fn() -> fn() -> Counter {
  external_function
}

fn identity(value: value) -> value {
  value
}

fn transform_value(
  value: value,
  fallback: value,
  transform: fn(value) -> value,
) -> value {
  case True {
    True -> transform(value)
    False -> fallback
  }
}

fn transform_list(values: List(value)) -> List(value) {
  values
}

fn prepend(value: value, values: List(value)) -> List(value) {
  [value, ..values]
}

fn drop_generic(values: List(value)) -> List(value) {
  let assert [_, ..remaining] = values
  remaining
}

pub fn main() -> Counter {
  let first = make_counter(11)
  let second = new_counter(12)
  let selected = case first {
    value -> value
  }
  let discarded = case selected {
    _ -> selected
  }
  let aliased = case discarded {
    value as alias if value == alias -> alias
    _ -> second
  }
  let assert [listed] = external_list()
  let read = external_function()
  let nested = nested_external_function()
  let transformed = transform_value(aliased, second, identity)
  let assert [transformed] = transform_list([transformed])
  let assert [transformed] = prepend(transformed, [])
  let assert [transformed] = drop_generic([second, transformed])
  let result = from_bool(
    from_int(
      from_float(
        from_string(
          from_tuple(
            from_custom(
              from_list(
                from_function(
                  from_block(with_unreached_panic(transformed)),
                ),
              ),
            ),
          ),
          second,
          "first",
        ),
        second,
        1.5,
      ),
      second,
      1,
    ),
    second,
    True,
  )
  let assert True = result == listed
  let assert True = result == read()
  let assert True = result == nested()()
  result
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
        HostProviderSet::with_providers(Vec::<HostModule<ExternalProfile>>::new(), [provider])
            .expect("provider module should be unique"),
    )
    .expect("external source should compile");
    let plan = plan_host_program(typed).expect("external source should plan");
    let execution =
        HostedExecution::try_from_module_plan(plan).expect("external execution should seal");

    let returned = execution
        .run_main(&mut ExternalRunState::default(), &mut Vec::new())
        .expect("external source should execute");
    let Value::External(counter) = returned else {
        panic!("main should return an external value");
    };

    assert_eq!(counter.inspection(), "Counter(11)");
}

#[test]
fn specializes_generic_external_types_by_their_source_arguments() {
    type ValueType = HostTypeParameter<0>;
    type SingletonElements = HostTypeList<ValueType, HostTypeListEnd>;
    type Singleton = HostTupleType<SingletonElements>;

    fn tag<'call>(
        mut call: HostCall<'call, ExternalProfile, CounterProvider, HostGenericCounter>,
        _value: HostValue<'call, ValueType>,
        tag: BigInt,
    ) -> Result<HostCallCompletion<'call, HostGenericCounter>, HostCallError> {
        let counter = call.create_external(Counter { value: tag });
        Ok(call.return_value(counter))
    }

    fn read<'call>(
        call: HostCall<'call, ExternalProfile, CounterProvider, BigInt>,
        counter: HostExternal<'call, HostGenericCounter>,
    ) -> Result<HostCallCompletion<'call, BigInt>, HostCallError> {
        let value = call.external_payload(counter).value.clone();
        Ok(call.return_value(value))
    }

    fn singleton<'call>(
        call: HostCall<'call, ExternalProfile, CounterProvider, Singleton>,
        value: HostValue<'call, ValueType>,
    ) -> Result<HostCallCompletion<'call, Singleton>, HostCallError> {
        Ok(call.return_tuple((value, ())))
    }

    let provider = HostProviderModule::<ExternalProfile>::new("application", "main")
        .expect("provider module should be valid")
        .with_external_type::<GenericCounterSchema>()
        .expect("external type should be valid")
        .with_scoped_function::<CounterProvider, (ValueType, BigInt), HostGenericCounter, _>(
            "tag", tag,
        )
        .expect("generic constructor provider should be valid")
        .with_scoped_function::<CounterProvider, (HostGenericCounter,), BigInt, _>("read", read)
        .expect("generic reader provider should be valid")
        .with_scoped_function::<CounterProvider, (ValueType,), Singleton, _>("singleton", singleton)
        .expect("generic tuple provider should be valid");
    let source = r#"
@external(erlang, "host", "GenericCounter")
pub type GenericCounter(value)

@external(erlang, "host", "tag")
fn tag(value: value, tag: Int) -> GenericCounter(value)

@external(erlang, "host", "read")
fn read(counter: GenericCounter(value)) -> Int

@external(erlang, "host", "singleton")
fn singleton(value: value) -> #(value)

pub fn main() {
  let int = tag(1, 10)
  let bool = tag(True, 20)
  #(int, bool, read(int), read(bool), singleton(int))
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
        HostProviderSet::with_providers(Vec::<HostModule<ExternalProfile>>::new(), [provider])
            .expect("provider module should be unique"),
    )
    .expect("generic external source should compile");
    let plan = plan_host_program(typed).expect("generic external source should plan");
    let execution = HostedExecution::try_from_module_plan(plan)
        .expect("generic external execution should seal");
    let returned = execution
        .run_main(&mut ExternalRunState::default(), &mut Vec::new())
        .expect("generic external source should execute");

    assert_eq!(
        returned.inspect().to_string(),
        "#(GenericCounter(10), GenericCounter(20), 10, 20, #(GenericCounter(10)))",
    );
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
        .with_external_type::<CounterSchema>()
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

#[test]
fn preserves_external_function_values_across_expression_boundaries() {
    fn new_counter<'call>(
        mut call: HostCall<'call, ExternalProfile, CounterProvider, HostCounter>,
        value: BigInt,
    ) -> Result<HostCallCompletion<'call, HostCounter>, HostCallError> {
        let counter = call.create_external(Counter { value });
        Ok(call.return_value(counter))
    }

    let provider = HostProviderModule::<ExternalProfile>::new("application", "main")
        .expect("provider module should be valid")
        .with_external_type::<CounterSchema>()
        .expect("external type should be valid")
        .with_scoped_function::<CounterProvider, (BigInt,), HostCounter, _>(
            "new_counter",
            new_counter,
        )
        .expect("constructor provider should be valid");
    let source = r#"
@external(erlang, "host", "Counter")
pub type Counter

pub type Maker {
  Maker(run: fn(Int) -> Counter)
}

@external(erlang, "host", "new_counter")
fn new_counter(value: Int) -> Counter

const constant_maker = new_counter
const generic_identity = identity

fn reference_maker() -> fn(Int) -> Counter {
  new_counter
}

fn constant_value() -> fn(Int) -> Counter {
  constant_maker
}

fn closure_value() -> fn(Int) -> Counter {
  fn(value) { new_counter(value + 1) }
}

fn direct_call_value() -> fn(Int) -> Counter {
  reference_maker()
}

fn function_call_value() -> fn(Int) -> Counter {
  let get = reference_maker
  get()
}

fn tuple_value() -> fn(Int) -> Counter {
  #(new_counter, 0).0
}

fn custom_value() -> fn(Int) -> Counter {
  Maker(new_counter).run
}

fn list_value() -> fn(Int) -> Counter {
  let assert [run] = [new_counter]
  run
}

fn panic_value() -> fn(Int) -> Counter {
  case True {
    True -> new_counter
    False -> panic
  }
}

fn bool_value(value: Bool) -> fn(Int) -> Counter {
  case value {
    True -> new_counter
    False -> closure_value()
  }
}

fn int_value(value: Int) -> fn(Int) -> Counter {
  case value {
    1 -> new_counter
    _ -> closure_value()
  }
}

fn float_value(value: Float) -> fn(Int) -> Counter {
  case value {
    1.5 -> new_counter
    _ -> closure_value()
  }
}

fn string_value(value: String) -> fn(Int) -> Counter {
  case value {
    "maker" -> new_counter
    _ -> closure_value()
  }
}

fn block_value() -> fn(Int) -> Counter {
  {
    let run = new_counter
    run
  }
}

fn tail_value() -> fn(Int) -> Counter {
  direct_call_value()
}

fn select_maker(_value: Int) -> fn(Int) -> Counter {
  new_counter
}

fn direct_call_argument_diverges(diverge: Bool) -> fn(Int) -> Counter {
  case diverge {
    True -> {
      let selected = select_maker(panic as "unreached direct function argument")
      selected
    }
    False -> new_counter
  }
}

fn function_call_source_diverges(diverge: Bool) -> fn(Int) -> Counter {
  case diverge {
    True -> {
      { panic as "unreached function-valued source" }(0)
    }
    False -> new_counter
  }
}

fn function_call_argument_diverges(diverge: Bool) -> fn(Int) -> Counter {
  case diverge {
    True -> {
      let get = select_maker
      let selected = get(panic as "unreached function-valued argument")
      selected
    }
    False -> new_counter
  }
}

fn function_tuple_source_diverges(diverge: Bool) -> fn(Int) -> Counter {
  case diverge {
    True -> {
      #(panic as "unreached function tuple source", new_counter).1
    }
    False -> new_counter
  }
}

fn function_custom_source_diverges(diverge: Bool) -> fn(Int) -> Counter {
  case diverge {
    True -> {
      Maker(run: panic as "unreached function custom source").run
    }
    False -> new_counter
  }
}

fn function_list_source_diverges(diverge: Bool) -> fn(Int) -> Counter {
  case diverge {
    True -> {
      case [panic as "unreached function list source"] {
        [selected] -> selected
        _ -> new_counter
      }
    }
    False -> new_counter
  }
}

fn function_block_step_diverges(diverge: Bool) -> fn(Int) -> Counter {
  case diverge {
    True -> {
      let selected = {
        let _: Nil = panic as "unreached function block step"
        new_counter
      }
      selected
    }
    False -> new_counter
  }
}

fn choose_divergent_maker(which: Int) -> fn(Int) -> Counter {
  case which {
    1 -> direct_call_argument_diverges(True)
    2 -> function_call_source_diverges(True)
    3 -> function_call_argument_diverges(True)
    4 -> function_tuple_source_diverges(True)
    5 -> function_custom_source_diverges(True)
    6 -> function_list_source_diverges(True)
    7 -> function_block_step_diverges(True)
    _ -> new_counter
  }
}

fn identity(value: value) -> value {
  value
}

fn wrap(value: value) -> List(value) {
  [value]
}

fn first(values: List(value)) -> value {
  let assert [head, ..] = values
  head
}

fn thunk(value: value) -> fn() -> value {
  fn() { value }
}

fn fail() {
  panic
}

pub fn main() {
  let direct = echo reference_maker() as "external function"
  let from_generic = identity(new_counter)
  let from_list = first(wrap(new_counter))
  let from_thunk = thunk(new_counter)()
  let safe_from_divergence = choose_divergent_maker(0)
  let external = identity(new_counter(20))
  let external_from_list = first(wrap(new_counter(21)))
  let external_from_thunk = thunk(new_counter(22))()
  let external_identity: fn(Counter) -> Counter = generic_identity
  let external_from_constant = external_identity(external)
  let guarded = case True {
    True -> external
    False -> fail()
  }
  #(
    direct(1),
    constant_value()(2),
    closure_value()(2),
    direct_call_value()(4),
    function_call_value()(5),
    tuple_value()(6),
    custom_value()(7),
    list_value()(8),
    panic_value()(9),
    bool_value(True)(10),
    int_value(1)(11),
    float_value(1.5)(12),
    string_value("maker")(13),
    block_value()(14),
    tail_value()(15),
    from_generic(16),
    from_list(17),
    from_thunk(18),
    safe_from_divergence(19),
    direct_call_argument_diverges(False)(23),
    function_call_source_diverges(False)(24),
    function_call_argument_diverges(False)(25),
    function_tuple_source_diverges(False)(26),
    function_custom_source_diverges(False)(27),
    function_list_source_diverges(False)(28),
    function_block_step_diverges(False)(29),
    external,
    external_from_list,
    external_from_thunk,
    external_from_constant,
    guarded,
  )
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
        HostProviderSet::with_providers(Vec::<HostModule<ExternalProfile>>::new(), [provider])
            .expect("provider module should be unique"),
    )
    .expect("external function source should compile");
    let plan = plan_host_program(typed).expect("external function source should plan");
    let execution = HostedExecution::try_from_module_plan(plan)
        .expect("external function execution should seal");

    let mut echoes = Vec::new();
    let returned = execution
        .run_main(&mut ExternalRunState::default(), &mut echoes)
        .expect("external function source should execute");

    assert_eq!(echoes.len(), 1);
    assert_eq!(
        echoes[0].message().map(|message| message.as_str()),
        Some("external function"),
    );
    assert_eq!(echoes[0].value().inspect().to_string(), "//fn(a) { ... }");
    assert_eq!(
        returned.inspect().to_string(),
        "#(Counter(1), Counter(2), Counter(3), Counter(4), Counter(5), Counter(6), Counter(7), Counter(8), Counter(9), Counter(10), Counter(11), Counter(12), Counter(13), Counter(14), Counter(15), Counter(16), Counter(17), Counter(18), Counter(19), Counter(23), Counter(24), Counter(25), Counter(26), Counter(27), Counter(28), Counter(29), Counter(20), Counter(21), Counter(22), Counter(20), Counter(20))",
    );
}

#[test]
fn returns_captured_external_function_values_from_the_program_entry() {
    fn new_counter<'call>(
        mut call: HostCall<'call, ExternalProfile, CounterProvider, HostCounter>,
        value: BigInt,
    ) -> Result<HostCallCompletion<'call, HostCounter>, HostCallError> {
        let counter = call.create_external(Counter { value });
        Ok(call.return_value(counter))
    }

    let provider = HostProviderModule::<ExternalProfile>::new("application", "main")
        .expect("provider module should be valid")
        .with_external_type::<CounterSchema>()
        .expect("external type should be valid")
        .with_scoped_function::<CounterProvider, (BigInt,), HostCounter, _>(
            "new_counter",
            new_counter,
        )
        .expect("constructor provider should be valid");
    let source = r#"
@external(erlang, "host", "Counter")
pub type Counter

@external(erlang, "host", "new_counter")
fn new_counter(value: Int) -> Counter

fn provide() -> fn(Int) -> Counter {
  new_counter
}

fn retain_provider(
  provider: fn() -> fn(Int) -> Counter,
) -> fn() -> fn() -> fn(Int) -> Counter {
  fn() { provider }
}

pub fn main() -> fn(Int) -> Counter {
  let restore = retain_provider(provide)
  let provider = restore()
  let make = provider()
  let _ = make(41)
  make
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
        HostProviderSet::with_providers(Vec::<HostModule<ExternalProfile>>::new(), [provider])
            .expect("provider module should be unique"),
    )
    .expect("captured external function source should compile");
    let plan = plan_host_program(typed).expect("captured external function source should plan");
    let returned = {
        let execution = HostedExecution::try_from_module_plan(plan)
            .expect("captured external function execution should seal");
        let mut state = ExternalRunState::default();

        execution
            .run_main(&mut state, &mut Vec::new())
            .expect("captured external function source should execute")
    };
    assert_eq!(returned.inspect().to_string(), "//fn(a) { ... }");
}

#[test]
fn returns_core_function_values_from_an_external_profile() {
    let provider = HostProviderModule::<ExternalProfile>::new("application", "main")
        .expect("provider module should be valid")
        .with_external_type::<CounterSchema>()
        .expect("external type should be valid");
    let source = r#"
@external(erlang, "host", "Counter")
pub type Counter

fn increment(value: Int) -> Int {
  value + 1
}

pub fn main() -> fn(Int) -> Int {
  echo Nil as "core function"
  increment
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
        HostProviderSet::with_providers(Vec::<HostModule<ExternalProfile>>::new(), [provider])
            .expect("provider module should be unique"),
    )
    .expect("core function source should compile");
    let plan = plan_host_program(typed).expect("core function source should plan");
    let execution =
        HostedExecution::try_from_module_plan(plan).expect("core function execution should seal");
    let mut echoes = Vec::new();

    let returned = execution
        .run_main(&mut ExternalRunState::default(), &mut echoes)
        .expect("core function source should execute");

    assert_eq!(returned.inspect().to_string(), "//fn(a) { ... }");
    assert_eq!(echoes.len(), 1);
    assert_eq!(
        echoes[0].message().map(|message| message.as_str()),
        Some("core function"),
    );
}

#[test]
fn preserves_symbolic_external_function_handoffs() {
    let provider = HostProviderModule::<ExternalProfile>::new("application", "main")
        .expect("provider module should be valid")
        .with_external_type::<CounterSchema>()
        .expect("external type should be valid");
    let source = r#"
@external(erlang, "host", "Counter")
pub type Counter

pub type CallableBox(input) {
  CallableBox(run: fn(input) -> Counter)
}

fn diverge(_value) -> Counter {
  panic as "unreached external function"
}

const diverging_constant = diverge

fn provider() {
  diverge
}

fn forwarded_provider() {
  provider()
}

fn divergence_provider(_value: Int) {
  diverge
}

fn divergence_provider_factory() {
  divergence_provider
}

fn failing_divergence_provider() {
  panic as "unreached symbolic function source"
}

fn unreachable_divergence(which: Int) -> fn(input) -> Counter {
  case which {
    1 -> {
      divergence_provider(panic as "unreached symbolic direct argument")
    }
    2 -> {
      let callable = divergence_provider_factory()
      callable(panic as "unreached symbolic function argument")
    }
    3 -> {
      let callable = failing_divergence_provider()
      callable(0)
    }
    4 -> {
      #(panic as "unreached symbolic tuple source", diverge).1
    }
    5 -> {
      CallableBox(run: panic as "unreached symbolic custom source").run
    }
    6 -> {
      case [panic as "unreached symbolic list source"] {
        [selected] -> selected
        _ -> diverge
      }
    }
    7 -> {
      let failed: Int = panic as "unreached symbolic block step"
      let _ = failed
      diverge
    }
    _ -> diverge
  }
}

fn exercise(
  function: fn(input) -> Counter,
  bool_selector: Bool,
  int_selector: Int,
  string_selector: String,
  float_selector: Float,
) -> fn(input) -> Counter {
  let local = function
  let from_list = case [local] {
    [first] -> first
    _ -> function
  }
  let from_block = {
    let _ = Nil
    from_list
  }
  let from_int = case int_selector {
    0 -> diverging_constant
    1 -> diverge
    2 -> fn(value) { function(value) }
    3 -> local
    4 -> exercise(function, False, 0, "", 0.0)
    5 -> {
      let selected = fn() { function }
      selected()
    }
    6 -> #(function).0
    7 -> CallableBox(run: function).run
    8 -> from_block
    9 -> provider()
    _ -> panic as "unselected symbolic external function"
  }
  let from_string = case string_selector {
    "selected" -> from_int
    _ -> function
  }
  let from_float = case float_selector {
    1.0 -> from_string
    _ -> function
  }
  case bool_selector {
    True -> from_float
    False -> function
  }
}

fn same(function: fn(input) -> Counter) {
  function == function
}

fn panic_value() -> fn(input) -> Counter {
  panic as "unselected external function panic"
}

pub fn main() {
  #(
    same(echo diverge as "symbolic external function"),
    same(forwarded_provider()),
    same(unreachable_divergence(0)),
    same(exercise(diverge, True, 0, "selected", 1.0)),
    same(exercise(diverge, True, 1, "selected", 1.0)),
    same(exercise(diverge, True, 2, "selected", 1.0)),
    same(exercise(diverge, True, 3, "selected", 1.0)),
    same(exercise(diverge, True, 4, "selected", 1.0)),
    same(exercise(diverge, True, 5, "selected", 1.0)),
    same(exercise(diverge, True, 6, "selected", 1.0)),
    same(exercise(diverge, True, 7, "selected", 1.0)),
    same(exercise(diverge, True, 8, "selected", 1.0)),
    same(exercise(diverge, True, 9, "selected", 1.0)),
    panic_value == panic_value,
  )
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
        HostProviderSet::with_providers(Vec::<HostModule<ExternalProfile>>::new(), [provider])
            .expect("provider module should be unique"),
    )
    .expect("symbolic external function source should compile");
    let plan = plan_host_program(typed).expect("symbolic external function source should plan");
    let execution = HostedExecution::try_from_module_plan(plan)
        .expect("symbolic external function execution should seal");

    let mut echoes = Vec::new();
    let returned = execution
        .run_main(&mut ExternalRunState::default(), &mut echoes)
        .expect("symbolic external function source should execute");

    assert_eq!(returned, Value::Tuple(vec![Value::Bool(true); 14]));
    assert_eq!(echoes.len(), 1);
    assert_eq!(
        echoes[0].message().map(|message| message.as_str()),
        Some("symbolic external function"),
    );
    assert_eq!(echoes[0].value().inspect().to_string(), "//fn(a) { ... }");
}

#[test]
fn preserves_executable_external_function_handoffs() {
    let provider = HostProviderModule::<ExternalProfile>::new("application", "main")
        .expect("provider module should be valid")
        .with_external_type::<CounterSchema>()
        .expect("external type should be valid");
    let source = r#"
@external(erlang, "host", "Counter")
pub type Counter

pub type CallableBox {
  CallableBox(run: fn(Int) -> Counter)
}

fn diverge(_value: Int) -> Counter {
  panic as "unreached external function"
}

const diverging_constant = diverge

fn provider() {
  diverge
}

fn exercise(
  function: fn(Int) -> Counter,
  bool_selector: Bool,
  int_selector: Int,
  string_selector: String,
  float_selector: Float,
) -> fn(Int) -> Counter {
  let local = function
  let from_list = case [local] {
    [first] -> first
    _ -> function
  }
  let from_block = {
    let _ = Nil
    from_list
  }
  let from_int = case int_selector {
    0 -> diverging_constant
    1 -> diverge
    2 -> fn(value) { function(value) }
    3 -> local
    4 -> exercise(function, False, 0, "", 0.0)
    5 -> {
      let selected = fn() { function }
      selected()
    }
    6 -> #(function).0
    7 -> CallableBox(run: function).run
    8 -> from_block
    9 -> provider()
    _ -> panic as "unselected executable external function"
  }
  let from_string = case string_selector {
    "selected" -> from_int
    _ -> function
  }
  let from_float = case float_selector {
    1.0 -> from_string
    _ -> function
  }
  case bool_selector {
    True -> from_float
    False -> function
  }
}

fn same(function: fn(Int) -> Counter) {
  function == function
}

fn panic_value() -> fn(Int) -> Counter {
  panic as "unselected external function panic"
}

pub fn main() {
  #(
    same(exercise(diverge, True, 0, "selected", 1.0)),
    same(exercise(diverge, True, 1, "selected", 1.0)),
    same(exercise(diverge, True, 2, "selected", 1.0)),
    same(exercise(diverge, True, 3, "selected", 1.0)),
    same(exercise(diverge, True, 4, "selected", 1.0)),
    same(exercise(diverge, True, 5, "selected", 1.0)),
    same(exercise(diverge, True, 6, "selected", 1.0)),
    same(exercise(diverge, True, 7, "selected", 1.0)),
    same(exercise(diverge, True, 8, "selected", 1.0)),
    same(exercise(diverge, True, 9, "selected", 1.0)),
    panic_value == panic_value,
  )
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
        HostProviderSet::with_providers(Vec::<HostModule<ExternalProfile>>::new(), [provider])
            .expect("provider module should be unique"),
    )
    .expect("executable external function source should compile");
    let plan = plan_host_program(typed).expect("executable external function source should plan");
    let execution = HostedExecution::try_from_module_plan(plan)
        .expect("executable external function execution should seal");

    let returned = execution
        .run_main(&mut ExternalRunState::default(), &mut Vec::new())
        .expect("executable external function source should execute");

    assert_eq!(returned, Value::Tuple(vec![Value::Bool(true); 11]));
}

#[test]
fn preserves_generic_external_return_function_handoffs() {
    let provider = HostProviderModule::<ExternalProfile>::new("application", "main")
        .expect("provider module should be valid")
        .with_external_type::<GenericCounterSchema>()
        .expect("generic external type should be valid");
    let source = r#"
@external(erlang, "host", "GenericCounter")
pub type GenericCounter(value)

pub type CallableBox(value) {
  CallableBox(run: fn(Int) -> GenericCounter(value))
}

fn diverge(_value: Int) -> GenericCounter(value) {
  panic as "unreached generic external function"
}

const diverging_constant = diverge

fn from_constant() -> fn(Int) -> GenericCounter(value) {
  diverging_constant
}

fn from_reference() -> fn(Int) -> GenericCounter(value) {
  diverge
}

fn from_closure() -> fn(Int) -> GenericCounter(value) {
  fn(_value: Int) { panic as "unreached generic external closure" }
}

fn from_argument(
  function: fn(Int) -> GenericCounter(value),
) -> fn(Int) -> GenericCounter(value) {
  let local = function
  local
}

fn from_call() -> fn(Int) -> GenericCounter(value) {
  from_reference()
}

fn from_function_call(
  provider: fn() -> fn(Int) -> GenericCounter(value),
) -> fn(Int) -> GenericCounter(value) {
  let selected = provider()
  selected
}

fn from_tuple() -> fn(Int) -> GenericCounter(value) {
  #(diverge).0
}

fn from_custom() -> fn(Int) -> GenericCounter(value) {
  CallableBox(run: diverge).run
}

fn from_list(
  functions: List(fn(Int) -> GenericCounter(value)),
) -> fn(Int) -> GenericCounter(value) {
  let assert [function] = functions
  function
}

fn from_bool(selector: Bool) -> fn(Int) -> GenericCounter(value) {
  case selector {
    True -> diverge
    False -> diverging_constant
  }
}

fn from_int(selector: Int) -> fn(Int) -> GenericCounter(value) {
  case selector {
    0 -> diverge
    _ -> diverging_constant
  }
}

fn from_string(selector: String) -> fn(Int) -> GenericCounter(value) {
  case selector {
    "reference" -> diverge
    _ -> diverging_constant
  }
}

fn from_float(selector: Float) -> fn(Int) -> GenericCounter(value) {
  case selector {
    1.0 -> diverge
    _ -> diverging_constant
  }
}

fn from_block() -> fn(Int) -> GenericCounter(value) {
  {
    let _ = Nil
    diverge
  }
}

fn from_panic() -> fn(Int) -> GenericCounter(value) {
  panic as "unselected generic external function panic"
}

fn same(function: fn(Int) -> GenericCounter(value)) {
  function == function
}

fn same_int(function: fn(Int) -> GenericCounter(Int)) {
  function == function
}

pub fn main() {
  #(
    same(from_constant()),
    same(from_reference()),
    same(from_closure()),
    same(from_argument(diverge)),
    same(from_call()),
    same(from_function_call(from_reference)),
    same(from_tuple()),
    same(from_custom()),
    same(from_list([diverge])),
    same_int(from_list([diverge])),
    same(from_bool(True)),
    same(from_bool(False)),
    same(from_int(0)),
    same(from_string("reference")),
    same(from_float(1.0)),
    same(from_block()),
    from_panic == from_panic,
  )
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
        HostProviderSet::with_providers(Vec::<HostModule<ExternalProfile>>::new(), [provider])
            .expect("provider module should be unique"),
    )
    .expect("generic external return function source should compile");
    let plan =
        plan_host_program(typed).expect("generic external return function source should plan");
    let execution = HostedExecution::try_from_module_plan(plan)
        .expect("generic external return function execution should seal");

    let returned = execution
        .run_main(&mut ExternalRunState::default(), &mut Vec::new())
        .expect("generic external return function source should execute");

    assert_eq!(returned, Value::Tuple(vec![Value::Bool(true); 17]));
}

#[test]
fn specializes_generic_function_expressions_to_external_returns() {
    let provider = HostProviderModule::<ExternalProfile>::new("application", "main")
        .expect("provider module should be valid")
        .with_external_type::<CounterSchema>()
        .expect("external type should be valid");
    let source = r#"
@external(erlang, "host", "Counter")
pub type Counter

pub type CallableBox(input, output) {
  CallableBox(function: fn(input) -> output)
}

fn external_function(_value: Int) -> Counter {
  panic as "unreached external function"
}

fn identity(value: value) -> value {
  value
}

const function_constant = identity

fn forward_function(function: fn(input) -> output) -> fn(input) -> output {
  function
}

fn provide_function(
  function: fn(input) -> output,
) -> fn() -> fn(input) -> output {
  fn() { function }
}

fn first_function(
  functions: List(fn(input) -> output),
) -> fn(input) -> output {
  case functions {
    [function] -> function
    _ -> panic as "expected one function"
  }
}

fn transform_function(
  function: fn(input) -> output,
) -> fn(input) -> output {
  let function = function_constant(function)
  let closure = fn(value) { function(value) }
  let direct = forward_function(closure)
  let provider = provide_function(direct)
  let called = provider()
  let tuple = #(called)
  let from_tuple = tuple.0
  let boxed = CallableBox(function: from_tuple)
  let from_field = boxed.function
  let from_list = case [from_field] {
    [first] -> first
    _ -> function
  }
  let from_int = case 1 {
    1 -> from_list
    _ -> function
  }
  let from_string = case "selected" {
    "selected" -> from_int
    _ -> function
  }
  let from_float = case 1.0 {
    1.0 -> from_string
    _ -> function
  }
  let from_block = {
    let _ = Nil
    from_float
  }
  case True {
    True -> from_block
    False -> panic as "unselected generic external function"
  }
}

fn select_function(
  function: fn(input) -> output,
  selector: Bool,
) -> fn(input) -> output {
  case selector {
    True -> function
    False -> function
  }
}

pub fn main() {
  let transformed = transform_function(external_function)
  let selected = select_function(external_function, True)
  let listed: fn(Int) -> Counter = first_function([external_function])
  #(
    transformed == transformed,
    selected == external_function,
    listed == external_function,
  )
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
        HostProviderSet::with_providers(Vec::<HostModule<ExternalProfile>>::new(), [provider])
            .expect("provider module should be unique"),
    )
    .expect("generic function expression source should compile");
    let plan = plan_host_program(typed).expect("generic function expression source should plan");
    let execution = HostedExecution::try_from_module_plan(plan)
        .expect("generic function expression execution should seal");

    let returned = execution
        .run_main(&mut ExternalRunState::default(), &mut Vec::new())
        .expect("generic function expression source should execute");

    assert_eq!(
        returned,
        Value::Tuple(vec![
            Value::Bool(true),
            Value::Bool(true),
            Value::Bool(true),
        ]),
    );
}

#[test]
fn evaluates_external_return_function_sources_before_uninhabited_arguments() {
    let provider = HostProviderModule::<ExternalProfile>::new("application", "main")
        .expect("provider module should be valid")
        .with_external_type::<CounterSchema>()
        .expect("external type should be valid");
    let source = r#"
@external(erlang, "host", "Counter")
pub type Counter

pub type Never

fn select(
  provider: fn() -> fn(trigger) -> fn(input) -> output,
  fail: fn() -> trigger,
) -> fn(input) -> output {
  provider()(fail())
}

fn external_function(_value: Int) -> Counter {
  panic as "external function should not run"
}

fn make(_trigger: Never) -> fn(Int) -> Counter {
  external_function
}

fn provide_factory() -> fn(Never) -> fn(Int) -> Counter {
  echo make as "function source"
}

fn fail_argument() -> Never {
  panic as "argument failed"
}

pub fn main() {
  select(provide_factory, fail_argument)
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
        HostProviderSet::with_providers(Vec::<HostModule<ExternalProfile>>::new(), [provider])
            .expect("provider module should be unique"),
    )
    .expect("diverging external function call source should compile");
    let plan =
        plan_host_program(typed).expect("diverging external function call source should plan");
    let execution = HostedExecution::try_from_module_plan(plan)
        .expect("diverging external function call execution should seal");

    let mut echoes = Vec::new();
    let error = execution
        .run_main(&mut ExternalRunState::default(), &mut echoes)
        .expect_err("the uninhabited argument should panic");
    let ExecutionError::Panic(error) = error else {
        panic!("the uninhabited argument should remain a source panic");
    };

    assert_eq!(error.kind(), PanicKind::Panic);
    assert_eq!(error.site().function(), "fail_argument");
    assert_eq!(echoes.len(), 1);
    assert_eq!(
        echoes[0].message().map(|message| message.as_str()),
        Some("function source"),
    );
    assert_eq!(echoes[0].value().inspect().to_string(), "//fn(a) { ... }");
}

#[test]
fn preserves_external_constants_and_list_expression_owners() {
    fn new_counter<'call>(
        mut call: HostCall<'call, ExternalProfile, CounterProvider, HostCounter>,
        value: BigInt,
    ) -> Result<HostCallCompletion<'call, HostCounter>, HostCallError> {
        let counter = call.create_external(Counter { value });
        Ok(call.return_value(counter))
    }

    let provider = HostProviderModule::<ExternalProfile>::new("application", "main")
        .expect("provider module should be valid")
        .with_external_type::<CounterSchema>()
        .expect("external type should be valid")
        .with_scoped_function::<CounterProvider, (BigInt,), HostCounter, _>(
            "new_counter",
            new_counter,
        )
        .expect("constructor provider should be valid");
    let source = r#"
@external(erlang, "host", "Counter")
pub type Counter

pub type ListHolder {
  ListHolder(selected: List(Counter))
}

pub type GenericListHolder(value) {
  GenericListHolder(selected: List(value))
}

pub type FunctionHolder(value) {
  FunctionHolder(run: value)
}

@external(erlang, "host", "new_counter")
fn new_counter(value: Int) -> Counter

fn identity(value: value) -> value {
  value
}

fn second(_ignored: first, value: second) -> second {
  value
}

const generic_empty = []
const generic_empty_alias = generic_empty
const exact_empty: List(Counter) = []
const exact_empty_alias: List(Counter) = exact_empty
const selected_constant: List(Counter) = generic_empty_alias
const selected_holder = ListHolder(selected: exact_empty_alias)
const generic_holder = GenericListHolder(selected: [])
const external_holder: GenericListHolder(Counter) = generic_holder
const generic_nested_empty = [generic_empty_alias]
const exact_nested_empty: List(List(Counter)) = generic_nested_empty
const direct_nested_empty: List(List(Counter)) = [[]]
const external_function: fn(Int) -> Counter = new_counter
const external_function_alias = external_function
const direct_function_holder = FunctionHolder(run: new_counter)
const alias_function_holder = FunctionHolder(run: external_function_alias)
const generic_function = identity
const generic_function_alias = generic_function
const generic_second = second
const generic_second_alias = generic_second
const external_second:
  fn(Nil, Counter) -> Counter =
  generic_second_alias
const specialized_identity:
  fn(fn(Int) -> Counter) -> fn(Int) -> Counter =
  generic_function_alias

fn provider(value: Int) -> List(Counter) {
  [new_counter(value)]
}

fn provider_function() -> fn(Int) -> List(Counter) {
  provider
}

fn from_tuple(value: Counter) -> List(Counter) {
  #([value], Nil).0
}

fn from_custom(value: Counter) -> List(Counter) {
  ListHolder(selected: [value]).selected
}

fn from_nested_list(value: Counter) -> List(Counter) {
  let assert [selected] = [[value]]
  selected
}

fn from_bool(value: Counter, selector: Bool) -> List(Counter) {
  case selector {
    True -> [value]
    False -> []
  }
}

fn from_int(value: Counter, selector: Int) -> List(Counter) {
  case selector {
    1 -> [value]
    _ -> []
  }
}

fn from_string(value: Counter, selector: String) -> List(Counter) {
  case selector {
    "selected" -> [value]
    _ -> []
  }
}

fn from_float(value: Counter, selector: Float) -> List(Counter) {
  case selector {
    1.0 -> [value]
    _ -> []
  }
}

fn from_block(value: Counter) -> List(Counter) {
  {
    let selected = value
    [selected]
  }
}

fn from_unreached_panic(value: Counter) -> List(Counter) {
  case True {
    True -> [value]
    False -> panic as "unselected external list panic"
  }
}

fn drop_first(values: List(Counter)) -> List(Counter) {
  case values {
    [_, ..tail] -> tail
    _ -> []
  }
}

fn list_first(values: List(Counter)) -> Counter {
  case values {
    [head, ..] -> head
    _ -> panic as "missing external list head"
  }
}

fn same(values: List(value)) -> Bool {
  values == values
}

fn forwarded_empty() -> List(value) {
  generic_empty_alias
}

fn forwarded_function() -> fn(value) -> value {
  generic_function_alias
}

pub fn main() {
  let first = new_counter(1)
  let second = external_function_alias(2)
  let specialized = specialized_identity(new_counter)
  let spread = [first, second, ..exact_empty_alias]
  let assert [head, ..tail] = spread
  let callable = provider_function()

  #(
    generic_empty_alias == selected_constant,
    forwarded_empty() == selected_constant,
    same(spread),
    head == first,
    list_first(spread) == first,
    tail == [second],
    drop_first(spread) == [second],
    provider(1) == [first],
    callable(2) == [second],
    from_tuple(first) == [first],
    from_custom(first) == [first],
    from_nested_list(first) == [first],
    from_bool(first, True) == [first],
    from_bool(first, False) == [],
    from_int(first, 1) == [first],
    from_int(first, 0) == [],
    from_string(first, "selected") == [first],
    from_string(first, "fallback") == [],
    from_float(first, 1.0) == [first],
    from_float(first, 0.0) == [],
    from_block(first) == [first],
    from_unreached_panic(first) == [first],
    specialized(1) == first,
    forwarded_function()(first) == first,
    selected_holder.selected == [],
    external_holder.selected == [],
    exact_nested_empty == [[]],
    direct_nested_empty == [[]],
    direct_function_holder.run(3) == new_counter(3),
    alias_function_holder.run(4) == new_counter(4),
    external_second(Nil, first) == first,
    provider == provider,
    provider_function == provider_function,
  )
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
        HostProviderSet::with_providers(Vec::<HostModule<ExternalProfile>>::new(), [provider])
            .expect("provider module should be unique"),
    )
    .expect("external constant and list source should compile");
    let plan = plan_host_program(typed).expect("external constant and list source should plan");
    let execution = HostedExecution::try_from_module_plan(plan)
        .expect("external constant and list execution should seal");

    let returned = execution
        .run_main(&mut ExternalRunState::default(), &mut Vec::new())
        .expect("external constant and list source should execute");

    assert_eq!(returned, Value::Tuple(vec![Value::Bool(true); 33]));
}

#[test]
fn preserves_external_expression_divergence_in_specialized_graphs() {
    fn new_counter<'call>(
        mut call: HostCall<'call, ExternalProfile, CounterProvider, HostCounter>,
        value: BigInt,
    ) -> Result<HostCallCompletion<'call, HostCounter>, HostCallError> {
        let counter = call.create_external(Counter { value });
        Ok(call.return_value(counter))
    }

    let provider = HostProviderModule::<ExternalProfile>::new("application", "main")
        .expect("provider module should be valid")
        .with_external_type::<CounterSchema>()
        .expect("external type should be valid")
        .with_scoped_function::<CounterProvider, (BigInt,), HostCounter, _>(
            "new_counter",
            new_counter,
        )
        .expect("constructor provider should be valid");
    let source = r#"
@external(erlang, "host", "Counter")
pub type Counter

pub type Wrapped {
  Wrapped(value: Counter)
}

@external(erlang, "host", "new_counter")
fn new_counter(value: Int) -> Counter

fn direct_call_argument(diverge: Bool) -> Counter {
  case diverge {
    True -> {
      let selected = new_counter(panic as "unreached direct argument")
      selected
    }
    False -> new_counter(1)
  }
}

fn function_call_source(diverge: Bool) -> Counter {
  case diverge {
    True -> {
      { panic as "unreached function source" }(1)
    }
    False -> new_counter(2)
  }
}

fn function_call_argument(diverge: Bool) -> Counter {
  case diverge {
    True -> {
      let function = new_counter
      let selected = function(panic as "unreached function argument")
      selected
    }
    False -> new_counter(3)
  }
}

fn tuple_source(diverge: Bool) -> Counter {
  case diverge {
    True -> {
      #(panic as "unreached tuple source", new_counter(4)).1
    }
    False -> new_counter(4)
  }
}

fn custom_source(diverge: Bool) -> Counter {
  case diverge {
    True -> {
      Wrapped(value: panic as "unreached custom source").value
    }
    False -> new_counter(5)
  }
}

fn list_source(diverge: Bool) -> Counter {
  case diverge {
    True -> {
      case [panic as "unreached list source"] {
        [selected] -> selected
        _ -> new_counter(6)
      }
    }
    False -> new_counter(6)
  }
}

fn block_step(diverge: Bool) -> Counter {
  case diverge {
    True -> {
      let selected = {
        let _: Nil = panic as "unreached block step"
        new_counter(7)
      }
      selected
    }
    False -> new_counter(7)
  }
}

fn choose_divergence(which: Int) -> Counter {
  case which {
    1 -> direct_call_argument(True)
    2 -> function_call_source(True)
    3 -> function_call_argument(True)
    4 -> tuple_source(True)
    5 -> custom_source(True)
    6 -> list_source(True)
    7 -> block_step(True)
    _ -> new_counter(0)
  }
}

fn route(
  value: Counter,
  fallback: Counter,
  bool_selector: Bool,
  int_selector: Int,
  string_selector: String,
  float_selector: Float,
) -> Counter {
  let from_bool = case bool_selector {
    True -> value
    False -> fallback
  }
  let from_int = case int_selector {
    1 -> from_bool
    _ -> fallback
  }
  let from_string = case string_selector {
    "selected" -> from_int
    _ -> fallback
  }
  let from_float = case float_selector {
    1.0 -> from_string
    _ -> fallback
  }
  from_float
}

pub fn main() {
  #(
    route(
      choose_divergence(0),
      new_counter(9),
      True,
      1,
      "selected",
      1.0,
    ),
    direct_call_argument(False),
    function_call_source(False),
    function_call_argument(False),
    tuple_source(False),
    custom_source(False),
    list_source(False),
    block_step(False),
    direct_call_argument == direct_call_argument,
    function_call_source == function_call_source,
    function_call_argument == function_call_argument,
    tuple_source == tuple_source,
    custom_source == custom_source,
    list_source == list_source,
    block_step == block_step,
  )
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
        HostProviderSet::with_providers(Vec::<HostModule<ExternalProfile>>::new(), [provider])
            .expect("provider module should be unique"),
    )
    .expect("external divergence source should compile");
    let plan = plan_host_program(typed).expect("external divergence source should plan");
    let execution = HostedExecution::try_from_module_plan(plan)
        .expect("external divergence execution should seal");

    let returned = execution
        .run_main(&mut ExternalRunState::default(), &mut Vec::new())
        .expect("external divergence source should execute");

    assert_eq!(
        returned.inspect().to_string(),
        "#(Counter(0), Counter(1), Counter(2), Counter(3), Counter(4), Counter(5), Counter(6), Counter(7), True, True, True, True, True, True, True)",
    );
}
