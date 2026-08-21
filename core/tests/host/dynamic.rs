use ecow::EcoString;
use geam_core::{
    BitArrayValue, ExecutionError, HostCall, HostCallCompletion, HostCallError, HostExternal,
    HostExternalBinding, HostExternalEquality, HostExternalHashing, HostExternalInspection,
    HostExternalSchema, HostExternalStorage, HostExternalStore, HostExternalType, HostFailure,
    HostFunctionType, HostProfile, HostProvider, HostProviderModule, HostProviderSet,
    HostStoredDynamic, HostTypeList, HostTypeListEnd, HostTypeParameter, HostValue,
    HostedExecution, ModuleSource, PackageSource, PanicKind, Value, compile_typed_host_program,
    plan_host_program,
};
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

struct DynamicProfile;

#[derive(Default)]
struct DynamicRunState {
    drops: Arc<AtomicUsize>,
}

#[derive(Default)]
struct DynamicStores {
    values: HostExternalStore<DynamicPayload>,
}

struct DynamicSchema;

struct DynamicProvider;

struct DynamicStorage;

struct DynamicPayload {
    value: HostStoredDynamic,
    _drop: PayloadDrop,
}

struct PayloadDrop(Arc<AtomicUsize>);

type Parameter = HostTypeParameter<0>;
type Dynamic = HostExternalType<DynamicSchema>;
type IntFunctionArguments = HostTypeList<num_bigint::BigInt, HostTypeListEnd>;
type IntFunction = HostFunctionType<IntFunctionArguments, num_bigint::BigInt>;

impl Drop for PayloadDrop {
    fn drop(&mut self) {
        self.0.fetch_add(1, Ordering::Relaxed);
    }
}

impl HostProfile for DynamicProfile {
    type RunState = DynamicRunState;
    type ExternalStores = DynamicStores;
}

impl HostProvider<DynamicProfile> for DynamicProvider {
    type State = DynamicRunState;

    fn project(state: &mut DynamicRunState) -> &mut Self::State {
        state
    }
}

impl HostExternalSchema for DynamicSchema {
    const PACKAGE: &'static str = "application";
    const MODULE: &'static str = "main";
    const NAME: &'static str = "Dynamic";
    const PARAMETER_COUNT: usize = 0;
}

impl HostExternalStorage<DynamicProfile, DynamicSchema> for DynamicStorage {
    type Payload = DynamicPayload;

    fn store(stores: &DynamicStores) -> &HostExternalStore<Self::Payload> {
        &stores.values
    }

    fn source_equal(
        context: &HostExternalEquality<'_>,
        left: &Self::Payload,
        right: &Self::Payload,
    ) -> bool {
        context.dynamic_values_equal(&left.value, &right.value)
    }

    fn source_hash(context: &HostExternalHashing<'_>, value: &Self::Payload) -> u64 {
        context.dynamic_value_hash(&value.value)
    }

    fn inspect(context: &HostExternalInspection<'_>, value: &Self::Payload) -> EcoString {
        format!("Dynamic({})", context.inspect_dynamic_value(&value.value)).into()
    }
}

impl HostExternalBinding<DynamicProfile, DynamicSchema> for DynamicProvider {
    type Storage = DynamicStorage;
}

fn encode<'call>(
    mut call: HostCall<'call, DynamicProfile, DynamicProvider, Dynamic>,
    value: HostValue<'call, Parameter>,
) -> Result<HostCallCompletion<'call, Dynamic>, HostCallError> {
    let drops = Arc::clone(&call.state().drops);
    let value = call.create_external_with(|builder| DynamicPayload {
        value: builder.store_dynamic::<Parameter>(value),
        _drop: PayloadDrop(drops),
    });
    Ok(call.return_value(value))
}

fn decode<'call>(
    mut call: HostCall<'call, DynamicProfile, DynamicProvider, Parameter>,
    dynamic: HostExternal<'call, Dynamic>,
    fallback: HostValue<'call, Parameter>,
) -> Result<HostCallCompletion<'call, Parameter>, HostCallError> {
    let payload = call.external_payload(dynamic);
    let value = payload
        .decode::<_, _, _, Parameter>(&mut call, |payload| &payload.value)
        .unwrap_or(fallback);
    Ok(call.return_value(value))
}

fn invoke_dynamic<'call>(
    mut call: HostCall<'call, DynamicProfile, DynamicProvider, num_bigint::BigInt>,
    dynamic: HostExternal<'call, Dynamic>,
    value: num_bigint::BigInt,
) -> Result<HostCallCompletion<'call, num_bigint::BigInt>, HostCallError> {
    let payload = call.external_payload(dynamic);
    let function = payload
        .decode::<_, _, _, IntFunction>(&mut call, |payload| &payload.value)
        .ok_or_else(|| HostFailure::new("dynamic value is not fn(Int) -> Int"))?;
    let value = call.invoke(function, (value, ()))?;
    Ok(call.return_value(value))
}

fn has_unresolved_type<'call>(
    mut call: HostCall<'call, DynamicProfile, DynamicProvider, bool>,
    dynamic: HostExternal<'call, Dynamic>,
) -> Result<HostCallCompletion<'call, bool>, HostCallError> {
    let payload = call.external_payload(dynamic);
    let unresolved = payload
        .decode::<_, _, _, HostTypeParameter<0>>(&mut call, |payload| &payload.value)
        .is_none();
    Ok(call.return_value(unresolved))
}

fn has_resolved_type<'call>(
    mut call: HostCall<'call, DynamicProfile, DynamicProvider, bool>,
    dynamic: HostExternal<'call, Dynamic>,
    _witness: HostValue<'call, HostTypeParameter<0>>,
) -> Result<HostCallCompletion<'call, bool>, HostCallError> {
    let payload = call.external_payload(dynamic);
    let resolved = payload
        .decode::<_, _, _, HostTypeParameter<0>>(&mut call, |payload| &payload.value)
        .is_some();
    Ok(call.return_value(resolved))
}

#[test]
fn decodes_every_scalar_and_rejects_mismatched_shapes() {
    let provider = HostProviderModule::<DynamicProfile>::new("application", "main")
        .expect("provider module should be valid")
        .with_external_type::<DynamicProvider, DynamicSchema>()
        .expect("dynamic type should be valid")
        .with_scoped_function::<DynamicProvider, (Parameter,), Dynamic, _>("encode", encode)
        .expect("encode provider should be valid")
        .with_scoped_function::<DynamicProvider, (Dynamic, Parameter), Parameter, _>(
            "decode", decode,
        )
        .expect("decode provider should be valid");
    let source = r#"
@external(erlang, "host", "Dynamic")
pub type Dynamic

@external(erlang, "host", "encode")
fn encode(value: value) -> Dynamic

@external(erlang, "host", "decode")
fn decode(value: Dynamic, fallback: value) -> value

pub fn main() {
  let assert <<first:utf8_codepoint, second:utf8_codepoint>> = <<"AB":utf8>>
  let integer = encode(42)
  let float = encode(1.5)
  let string = encode("answer")
  let bits = encode(<<1, 2>>)
  let codepoint = encode(first)
  let bool = encode(True)
  let nil = encode(Nil)
  #(
    decode(integer, 0),
    decode(float, 0.0),
    decode(string, "fallback"),
    decode(bits, <<0>>),
    decode(codepoint, second),
    decode(bool, False),
    decode(nil, Nil),
    decode(integer, "fallback"),
    decode(string, 0),
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
    .expect("dynamic source should compile");
    let plan = plan_host_program(typed).expect("dynamic source should plan");
    let execution =
        HostedExecution::try_from_module_plan(plan).expect("dynamic execution should seal");
    let expected = Value::Tuple(vec![
        Value::Int(42.into()),
        Value::Float(1.5),
        Value::String("answer".into()),
        Value::BitArray(BitArrayValue::from_bytes(vec![1, 2])),
        Value::UtfCodepoint('A'),
        Value::Bool(true),
        Value::Nil,
        Value::String("fallback".into()),
        Value::Int(0.into()),
    ]);
    let mut state = DynamicRunState::default();

    let actual = execution.run_main(&mut state, &mut Vec::new());

    assert_eq!(actual, Ok(expected));
    assert_eq!(state.drops.load(Ordering::Relaxed), 7);
}

#[test]
fn decodes_compounds_functions_and_nested_external_values() {
    let provider = HostProviderModule::<DynamicProfile>::new("application", "main")
        .expect("provider module should be valid")
        .with_external_type::<DynamicProvider, DynamicSchema>()
        .expect("dynamic type should be valid")
        .with_scoped_function::<DynamicProvider, (Parameter,), Dynamic, _>("encode", encode)
        .expect("encode provider should be valid")
        .with_scoped_function::<DynamicProvider, (Dynamic, Parameter), Parameter, _>(
            "decode", decode,
        )
        .expect("decode provider should be valid");
    let source = r#"
@external(erlang, "host", "Dynamic")
pub type Dynamic

pub type Boxed(value) {
  Boxed(value: value)
}

@external(erlang, "host", "encode")
fn encode(value: value) -> Dynamic

@external(erlang, "host", "decode")
fn decode(value: Dynamic, fallback: value) -> value

fn increment(value: Int) {
  value + 1
}

fn identity(value: Int) {
  value
}

pub fn main() {
  let list = decode(encode([1, 2]), [])
  let tuple = decode(encode(#("one", True)), #("fallback", False))
  let Boxed(custom) = decode(encode(Boxed(42)), Boxed(0))
  let function = decode(encode(increment), identity)
  let inner = encode("inside")
  let external = decode(encode(inner), encode("fallback"))
  let mismatched_list = decode(encode([1, 2]), ["fallback"])
  let assert [first, second] = list
  let assert [mismatched] = mismatched_list

  #(
    first + second,
    tuple,
    custom,
    function == increment,
    function(41),
    external == inner,
    mismatched,
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
    .expect("dynamic compound source should compile");
    let plan = plan_host_program(typed).expect("dynamic compound source should plan");
    let execution = HostedExecution::try_from_module_plan(plan)
        .expect("dynamic compound execution should seal");
    let expected = Value::Tuple(vec![
        Value::Int(3.into()),
        Value::Tuple(vec![Value::String("one".into()), Value::Bool(true)]),
        Value::Int(42.into()),
        Value::Bool(true),
        Value::Int(42.into()),
        Value::Bool(true),
        Value::String("fallback".into()),
    ]);
    let mut state = DynamicRunState::default();

    let actual = execution.run_main(&mut state, &mut Vec::new());

    assert_eq!(actual, Ok(expected));
    assert_eq!(state.drops.load(Ordering::Relaxed), 8);
}

#[test]
fn invokes_a_decoded_callable_through_nested_host_reentry() {
    let provider = HostProviderModule::<DynamicProfile>::new("application", "main")
        .expect("provider module should be valid")
        .with_external_type::<DynamicProvider, DynamicSchema>()
        .expect("dynamic type should be valid")
        .with_scoped_function::<DynamicProvider, (Parameter,), Dynamic, _>("encode", encode)
        .expect("encode provider should be valid")
        .with_scoped_function::<
            DynamicProvider,
            (Dynamic, num_bigint::BigInt),
            num_bigint::BigInt,
            _,
        >("invoke_dynamic", invoke_dynamic)
        .expect("dynamic invocation provider should be valid")
        .with_function("increment", |value: num_bigint::BigInt| value + 1)
        .expect("nested scalar provider should be valid");
    let source = r#"
@external(erlang, "host", "Dynamic")
pub type Dynamic

@external(erlang, "host", "encode")
fn encode(value: value) -> Dynamic

@external(erlang, "host", "invoke_dynamic")
fn invoke_dynamic(value: Dynamic, argument: Int) -> Int

@external(erlang, "host", "increment")
fn increment(value: Int) -> Int

fn callback(value: Int) {
  increment(value) + 1
}

pub fn main() {
  invoke_dynamic(encode(callback), 40)
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
    .expect("dynamic callback source should compile");
    let plan = plan_host_program(typed).expect("dynamic callback source should plan");
    let execution = HostedExecution::try_from_module_plan(plan)
        .expect("dynamic callback execution should seal");
    let mut state = DynamicRunState::default();

    let actual = execution.run_main(&mut state, &mut Vec::new());

    assert_eq!(actual, Ok(Value::Int(42.into())));
    assert_eq!(state.drops.load(Ordering::Relaxed), 1);
}

#[test]
fn reports_decode_mismatch_as_provider_semantics() {
    let provider = HostProviderModule::<DynamicProfile>::new("application", "main")
        .expect("provider module should be valid")
        .with_external_type::<DynamicProvider, DynamicSchema>()
        .expect("dynamic type should be valid")
        .with_scoped_function::<DynamicProvider, (Parameter,), Dynamic, _>("encode", encode)
        .expect("encode provider should be valid")
        .with_scoped_function::<
            DynamicProvider,
            (Dynamic, num_bigint::BigInt),
            num_bigint::BigInt,
            _,
        >("invoke_dynamic", invoke_dynamic)
        .expect("dynamic invocation provider should be valid");
    let source = r#"
@external(erlang, "host", "Dynamic")
pub type Dynamic

@external(erlang, "host", "encode")
fn encode(value: value) -> Dynamic

@external(erlang, "host", "invoke_dynamic")
fn invoke_dynamic(value: Dynamic, argument: Int) -> Int

pub fn main() {
  invoke_dynamic(encode("not a function"), 40)
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
    .expect("dynamic mismatch source should compile");
    let plan = plan_host_program(typed).expect("dynamic mismatch source should plan");
    let execution = HostedExecution::try_from_module_plan(plan)
        .expect("dynamic mismatch execution should seal");
    let mut state = DynamicRunState::default();

    let error = execution
        .run_main(&mut state, &mut Vec::new())
        .expect_err("provider should reject the requested decode");

    assert_eq!(
        error.to_string(),
        "host function application::main.invoke_dynamic failed: dynamic value is not fn(Int) -> Int",
    );
    assert_eq!(state.drops.load(Ordering::Relaxed), 1);
}

#[test]
fn distinguishes_unresolved_resolved_and_mismatched_parameters() {
    let provider = HostProviderModule::<DynamicProfile>::new("application", "main")
        .expect("provider module should be valid")
        .with_external_type::<DynamicProvider, DynamicSchema>()
        .expect("dynamic type should be valid")
        .with_scoped_function::<DynamicProvider, (Parameter,), Dynamic, _>("encode", encode)
        .expect("encode provider should be valid")
        .with_scoped_function::<DynamicProvider, (Dynamic,), bool, _>(
            "has_unresolved_type",
            has_unresolved_type,
        )
        .expect("unresolved type provider should be valid")
        .with_scoped_function::<DynamicProvider, (Dynamic, Parameter), bool, _>(
            "has_resolved_type",
            has_resolved_type,
        )
        .expect("resolved type provider should be valid");
    let source = r#"
@external(erlang, "host", "Dynamic")
pub type Dynamic

@external(erlang, "host", "encode")
fn encode(value: value) -> Dynamic

@external(erlang, "host", "has_unresolved_type")
fn has_unresolved_type(value: Dynamic) -> Bool

@external(erlang, "host", "has_resolved_type")
fn has_resolved_type(value: Dynamic, witness: value) -> Bool

pub fn main() {
  #(
    has_unresolved_type(encode(42)),
    has_resolved_type(encode(42), 0),
    has_resolved_type(encode(42), True),
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
    .expect("unresolved dynamic source should compile");
    let plan = plan_host_program(typed).expect("unresolved dynamic source should plan");
    let execution = HostedExecution::try_from_module_plan(plan)
        .expect("unresolved dynamic execution should seal");
    let mut state = DynamicRunState::default();

    let actual = execution.run_main(&mut state, &mut Vec::new());

    assert_eq!(
        actual,
        Ok(Value::Tuple(vec![
            Value::Bool(true),
            Value::Bool(true),
            Value::Bool(false),
        ])),
    );
    assert_eq!(state.drops.load(Ordering::Relaxed), 3);
}

#[test]
fn escaped_dynamic_preserves_inspection_identity_and_cleanup() {
    let provider = HostProviderModule::<DynamicProfile>::new("application", "main")
        .expect("provider module should be valid")
        .with_external_type::<DynamicProvider, DynamicSchema>()
        .expect("dynamic type should be valid")
        .with_scoped_function::<DynamicProvider, (Parameter,), Dynamic, _>("encode", encode)
        .expect("encode provider should be valid");
    let source = r#"
@external(erlang, "host", "Dynamic")
pub type Dynamic

@external(erlang, "host", "encode")
fn encode(value: value) -> Dynamic

pub fn main() {
  let first = encode([1, 2])
  let alias = first
  let distinct = encode([1, 2])
  #(first, first == alias, first == distinct)
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
    .expect("escaping dynamic source should compile");
    let plan = plan_host_program(typed).expect("escaping dynamic source should plan");
    let execution =
        HostedExecution::try_from_module_plan(plan).expect("dynamic execution should seal");
    let mut state = DynamicRunState::default();
    let drops = Arc::clone(&state.drops);

    let result = execution
        .run_main(&mut state, &mut Vec::new())
        .expect("dynamic value should escape");

    assert_eq!(
        result.inspect().to_string(),
        "#(Dynamic([1, 2]), True, True)",
    );
    assert_eq!(drops.load(Ordering::Relaxed), 1);

    drop(state);
    drop(execution);
    assert_eq!(drops.load(Ordering::Relaxed), 1);

    drop(result);
    assert_eq!(drops.load(Ordering::Relaxed), 2);
}

#[test]
fn releases_dynamic_storage_after_source_panic() {
    let provider = HostProviderModule::<DynamicProfile>::new("application", "main")
        .expect("provider module should be valid")
        .with_external_type::<DynamicProvider, DynamicSchema>()
        .expect("dynamic type should be valid")
        .with_scoped_function::<DynamicProvider, (Parameter,), Dynamic, _>("encode", encode)
        .expect("encode provider should be valid");
    let source = r#"
@external(erlang, "host", "Dynamic")
pub type Dynamic

@external(erlang, "host", "encode")
fn encode(value: value) -> Dynamic

pub fn main() {
  let dynamic = encode(#([1, 2], "retained"))
  let assert True = False
  dynamic
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
    .expect("dynamic panic source should compile");
    let plan = plan_host_program(typed).expect("dynamic panic source should plan");
    let execution =
        HostedExecution::try_from_module_plan(plan).expect("dynamic panic execution should seal");
    let mut state = DynamicRunState::default();

    let error = execution
        .run_main(&mut state, &mut Vec::new())
        .expect_err("source panic should be returned");
    let ExecutionError::Panic(panic) = error else {
        panic!("let assert should remain a source panic");
    };

    assert_eq!(panic.kind(), PanicKind::LetAssert);
    assert_eq!(state.drops.load(Ordering::Relaxed), 1);
}
