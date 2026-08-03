use ecow::EcoString;
use geam::{
    ExecutionError, HostCall, HostCallCompletion, HostCallError, HostCallable, HostExternal,
    HostExternalEquality, HostExternalHashing, HostExternalInspection, HostExternalSchema,
    HostExternalStorage, HostExternalStore, HostExternalType, HostFailure, HostFunctionType,
    HostProfile, HostProvider, HostProviderModule, HostProviderSet, HostStoredType,
    HostStoredValue, HostTypeIndex0, HostTypeIndexNext, HostTypeList, HostTypeListEnd,
    HostTypeParameter, HostValue, HostedExecution, ListValue, ModuleSource, PackageSource,
    PanicKind, Value, compile_typed_host_program, plan_host_program,
};
use num_bigint::BigInt;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

struct StoredProfile;

#[derive(Default)]
struct StoredRunState {
    drops: Arc<AtomicUsize>,
}

#[derive(Default)]
struct StoredStores {
    maps: HostExternalStore<StoredMapPayload>,
    callbacks: HostExternalStore<StoredCallbackPayload>,
}

struct StoredMapSchema;

struct StoredCallbackSchema;

struct StoredProvider;

type FirstParameter = HostTypeParameter<0>;
type SecondParameter = HostTypeParameter<1>;
type StoreMapArguments =
    HostTypeList<FirstParameter, HostTypeList<SecondParameter, HostTypeListEnd>>;
type StoreMap = HostExternalType<StoredMapSchema, StoreMapArguments>;
type ValueMapArguments =
    HostTypeList<SecondParameter, HostTypeList<FirstParameter, HostTypeListEnd>>;
type ValueMap = HostExternalType<StoredMapSchema, ValueMapArguments>;

type FirstStoredType = HostStoredType<HostTypeIndex0>;
type SecondStoredType = HostStoredType<HostTypeIndexNext<HostTypeIndex0>>;

type IntFunctionArguments = HostTypeList<BigInt, HostTypeListEnd>;
type IntFunction = HostFunctionType<IntFunctionArguments, BigInt>;
type StoredCallback = HostExternalType<StoredCallbackSchema>;

struct StoredMapPayload {
    key: HostStoredValue<FirstStoredType>,
    value: HostStoredValue<SecondStoredType>,
    _drop: PayloadDrop,
}

struct StoredCallbackPayload {
    function: HostStoredValue<IntFunction>,
}

struct PayloadDrop(Arc<AtomicUsize>);

impl Drop for PayloadDrop {
    fn drop(&mut self) {
        self.0.fetch_add(1, Ordering::Relaxed);
    }
}

impl HostProfile for StoredProfile {
    type RunState = StoredRunState;
    type ExternalStores = StoredStores;
}

impl HostProvider<StoredProfile> for StoredProvider {
    type State = StoredRunState;

    fn project(state: &mut StoredRunState) -> &mut Self::State {
        state
    }
}

impl HostExternalSchema for StoredMapSchema {
    const PACKAGE: &'static str = "application";
    const MODULE: &'static str = "main";
    const NAME: &'static str = "StoredMap";
    const PARAMETER_COUNT: usize = 2;
}

impl HostExternalStorage<StoredMapSchema> for StoredProfile {
    type Payload = StoredMapPayload;

    fn store(stores: &Self::ExternalStores) -> &HostExternalStore<Self::Payload> {
        &stores.maps
    }

    fn source_equal(
        context: &HostExternalEquality<'_>,
        left: &Self::Payload,
        right: &Self::Payload,
    ) -> bool {
        context.stored_values_equal(&left.key, &right.key)
            && context.stored_values_equal(&left.value, &right.value)
    }

    fn source_hash(context: &HostExternalHashing<'_>, value: &Self::Payload) -> u64 {
        context.stored_value_hash(&value.key).rotate_left(1)
            ^ context.stored_value_hash(&value.value)
    }

    fn inspect(context: &HostExternalInspection<'_>, value: &Self::Payload) -> EcoString {
        format!(
            "StoredMap({}, {})",
            context.inspect_stored_value(&value.key),
            context.inspect_stored_value(&value.value),
        )
        .into()
    }
}

impl HostExternalSchema for StoredCallbackSchema {
    const PACKAGE: &'static str = "application";
    const MODULE: &'static str = "main";
    const NAME: &'static str = "StoredCallback";
    const PARAMETER_COUNT: usize = 0;
}

impl HostExternalStorage<StoredCallbackSchema> for StoredProfile {
    type Payload = StoredCallbackPayload;

    fn store(stores: &Self::ExternalStores) -> &HostExternalStore<Self::Payload> {
        &stores.callbacks
    }

    fn source_equal(
        context: &HostExternalEquality<'_>,
        left: &Self::Payload,
        right: &Self::Payload,
    ) -> bool {
        context.stored_values_equal(&left.function, &right.function)
    }

    fn source_hash(context: &HostExternalHashing<'_>, value: &Self::Payload) -> u64 {
        context.stored_value_hash(&value.function)
    }

    fn inspect(context: &HostExternalInspection<'_>, value: &Self::Payload) -> EcoString {
        format!(
            "StoredCallback({})",
            context.inspect_stored_value(&value.function),
        )
        .into()
    }
}

#[test]
fn retains_concrete_generic_keys_and_values() {
    fn store<'call>(
        mut call: HostCall<'call, StoredProfile, StoredProvider, StoreMap>,
        key: HostValue<'call, FirstParameter>,
        value: HostValue<'call, SecondParameter>,
    ) -> Result<HostCallCompletion<'call, StoreMap>, HostCallError> {
        let drops = Arc::clone(&call.state().drops);
        let stored = call.create_external_with(|builder| StoredMapPayload {
            key: builder.store_argument::<HostTypeIndex0>(key),
            value: builder.store_argument::<HostTypeIndexNext<HostTypeIndex0>>(value),
            _drop: PayloadDrop(drops),
        });
        Ok(call.return_value(stored))
    }

    fn key<'call>(
        mut call: HostCall<'call, StoredProfile, StoredProvider, FirstParameter>,
        stored: HostExternal<'call, StoreMap>,
    ) -> Result<HostCallCompletion<'call, FirstParameter>, HostCallError> {
        let payload = call.external_payload(stored);
        let key = payload.restore_argument(&mut call, |payload| &payload.key);
        Ok(call.return_value(key))
    }

    fn value<'call>(
        mut call: HostCall<'call, StoredProfile, StoredProvider, FirstParameter>,
        stored: HostExternal<'call, ValueMap>,
    ) -> Result<HostCallCompletion<'call, FirstParameter>, HostCallError> {
        let payload = call.external_payload(stored);
        let value = payload.restore_argument(&mut call, |payload| &payload.value);
        Ok(call.return_value(value))
    }

    let provider = HostProviderModule::<StoredProfile>::new("application", "main")
        .expect("provider module should be valid")
        .with_external_type::<StoredMapSchema>()
        .expect("stored map type should be valid")
        .with_scoped_function::<StoredProvider, (FirstParameter, SecondParameter), StoreMap, _>(
            "store", store,
        )
        .expect("store provider should be valid")
        .with_scoped_function::<StoredProvider, (StoreMap,), FirstParameter, _>("key", key)
        .expect("key provider should be valid")
        .with_scoped_function::<StoredProvider, (ValueMap,), FirstParameter, _>("value", value)
        .expect("value provider should be valid");
    let source = r#"
@external(erlang, "host", "StoredMap")
pub type StoredMap(key, value)

@external(erlang, "host", "store")
fn store(key: key, value: value) -> StoredMap(key, value)

@external(erlang, "host", "key")
fn key(map: StoredMap(key, value)) -> key

@external(erlang, "host", "value")
fn value(map: StoredMap(key, value)) -> value

pub fn main() {
  let stored = store(42, "answer")
  #(key(stored), value(stored))
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
    .expect("stored source should compile");
    let plan = plan_host_program(typed).expect("stored source should plan");
    let execution =
        HostedExecution::try_from_module_plan(plan).expect("stored execution should seal");
    let expected_explanation = r#"
module main
main tuple#0

function int#0
  host application::main.key signature=fn(external_type#0) -> Int

function string#0
  host application::main.value signature=fn(external_type#0) -> String

function external#0
  host application::main.store signature=fn(Int, String) -> external_type#0

function tuple#0
  entry b0 params=[] captures=[]
  block b0 params=[]
    %int#0:shape#0(Int) = int.value 42
    %string#0:shape#1(String) = string.value "answer"
    %external#0:shape#2(external_type#0) = external.call external#0 args=[%int#0, %string#0]
    %int#1:shape#0(Int) = int.call int#0 args=[%external#0]
    %string#1:shape#1(String) = string.call string#0 args=[%external#0]
    %tuple#0:shape#3(#(Int, String)) = tuple.value elements=[%int#1, %string#1]
    return %tuple#0
"#
    .trim();
    let expected = Value::Tuple(vec![Value::Int(42.into()), Value::String("answer".into())]);

    let mut first_state = StoredRunState::default();
    let first = execution.run_main(&mut first_state, &mut Vec::new());
    let mut second_state = StoredRunState::default();
    let second = execution.run_main(&mut second_state, &mut Vec::new());

    assert_eq!(execution.explain().to_string().trim(), expected_explanation);
    assert_eq!(first, Ok(expected.clone()));
    assert_eq!(second, Ok(expected));
    assert_eq!(first_state.drops.load(Ordering::Relaxed), 1);
    assert_eq!(second_state.drops.load(Ordering::Relaxed), 1);
}

#[test]
fn retains_nested_compounds_externals_and_function_identity() {
    fn store<'call>(
        mut call: HostCall<'call, StoredProfile, StoredProvider, StoreMap>,
        key: HostValue<'call, FirstParameter>,
        value: HostValue<'call, SecondParameter>,
    ) -> Result<HostCallCompletion<'call, StoreMap>, HostCallError> {
        let drops = Arc::clone(&call.state().drops);
        let stored = call.create_external_with(|builder| StoredMapPayload {
            key: builder.store_argument::<HostTypeIndex0>(key),
            value: builder.store_argument::<HostTypeIndexNext<HostTypeIndex0>>(value),
            _drop: PayloadDrop(drops),
        });
        Ok(call.return_value(stored))
    }

    fn key<'call>(
        mut call: HostCall<'call, StoredProfile, StoredProvider, FirstParameter>,
        stored: HostExternal<'call, StoreMap>,
    ) -> Result<HostCallCompletion<'call, FirstParameter>, HostCallError> {
        let payload = call.external_payload(stored);
        let key = payload.restore_argument(&mut call, |payload| &payload.key);
        Ok(call.return_value(key))
    }

    fn value<'call>(
        mut call: HostCall<'call, StoredProfile, StoredProvider, FirstParameter>,
        stored: HostExternal<'call, ValueMap>,
    ) -> Result<HostCallCompletion<'call, FirstParameter>, HostCallError> {
        let payload = call.external_payload(stored);
        let value = payload.restore_argument(&mut call, |payload| &payload.value);
        Ok(call.return_value(value))
    }

    let provider = HostProviderModule::<StoredProfile>::new("application", "main")
        .expect("provider module should be valid")
        .with_external_type::<StoredMapSchema>()
        .expect("stored map type should be valid")
        .with_scoped_function::<StoredProvider, (FirstParameter, SecondParameter), StoreMap, _>(
            "store", store,
        )
        .expect("store provider should be valid")
        .with_scoped_function::<StoredProvider, (StoreMap,), FirstParameter, _>("key", key)
        .expect("key provider should be valid")
        .with_scoped_function::<StoredProvider, (ValueMap,), FirstParameter, _>("value", value)
        .expect("value provider should be valid");
    let source = r#"
@external(erlang, "host", "StoredMap")
pub type StoredMap(key, value)

pub type Wrapper(value) {
  Wrapper(value: value)
}

@external(erlang, "host", "store")
fn store(key: key, value: value) -> StoredMap(key, value)

@external(erlang, "host", "key")
fn key(map: StoredMap(key, value)) -> key

@external(erlang, "host", "value")
fn value(map: StoredMap(key, value)) -> value

fn increment(value: Int) {
  value + 1
}

pub fn main() {
  let inner = store(1, "one")
  let outer = store([#(Wrapper(inner), True)], #(inner, [False]))
  let assert [#(Wrapper(restored), flag)] = key(outer)
  let #(tuple_inner, flags) = value(outer)
  let function = value(store(Nil, increment))

  #(
    key(restored),
    value(restored),
    flag,
    key(tuple_inner),
    value(tuple_inner),
    flags,
    function == increment,
    function(41),
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
    .expect("stored source should compile");
    let plan = plan_host_program(typed).expect("stored source should plan");
    let execution =
        HostedExecution::try_from_module_plan(plan).expect("stored execution should seal");
    let expected = Value::Tuple(vec![
        Value::Int(1.into()),
        Value::String("one".into()),
        Value::Bool(true),
        Value::Int(1.into()),
        Value::String("one".into()),
        Value::List(ListValue::bool(vec![false])),
        Value::Bool(true),
        Value::Int(42.into()),
    ]);

    let mut state = StoredRunState::default();
    let actual = execution.run_main(&mut state, &mut Vec::new());

    assert_eq!(actual, Ok(expected));
    assert_eq!(state.drops.load(Ordering::Relaxed), 3);
}

#[test]
fn invokes_a_retained_callable_through_nested_host_reentry() {
    fn store_callback<'call>(
        mut call: HostCall<'call, StoredProfile, StoredProvider, StoredCallback>,
        function: HostCallable<'call, IntFunctionArguments, BigInt>,
    ) -> Result<HostCallCompletion<'call, StoredCallback>, HostCallError> {
        let stored = call.create_external_with(|builder| StoredCallbackPayload {
            function: builder.store::<IntFunction>(function),
        });
        Ok(call.return_value(stored))
    }

    fn restore_callback<'call>(
        mut call: HostCall<'call, StoredProfile, StoredProvider, IntFunction>,
        stored: HostExternal<'call, StoredCallback>,
    ) -> Result<HostCallCompletion<'call, IntFunction>, HostCallError> {
        let payload = call.external_payload(stored);
        let function = payload.restore(&mut call, |payload| &payload.function);
        Ok(call.return_value(function))
    }

    fn invoke_callback<'call>(
        mut call: HostCall<'call, StoredProfile, StoredProvider, BigInt>,
        stored: HostExternal<'call, StoredCallback>,
        value: BigInt,
    ) -> Result<HostCallCompletion<'call, BigInt>, HostCallError> {
        let payload = call.external_payload(stored);
        let function = payload.restore(&mut call, |payload| &payload.function);
        let value = call.invoke(function, (value, ()))?;
        Ok(call.return_value(value))
    }

    let provider = HostProviderModule::<StoredProfile>::new("application", "main")
        .expect("provider module should be valid")
        .with_external_type::<StoredCallbackSchema>()
        .expect("stored callback type should be valid")
        .with_scoped_function::<StoredProvider, (IntFunction,), StoredCallback, _>(
            "store_callback",
            store_callback,
        )
        .expect("callback store provider should be valid")
        .with_scoped_function::<StoredProvider, (StoredCallback,), IntFunction, _>(
            "restore_callback",
            restore_callback,
        )
        .expect("callback restore provider should be valid")
        .with_scoped_function::<StoredProvider, (StoredCallback, BigInt), BigInt, _>(
            "invoke_callback",
            invoke_callback,
        )
        .expect("callback invocation provider should be valid")
        .with_function("increment", |value: BigInt| value + 1)
        .expect("nested scalar provider should be valid");
    let source = r#"
@external(erlang, "host", "StoredCallback")
pub type StoredCallback

@external(erlang, "host", "store_callback")
fn store_callback(function: fn(Int) -> Int) -> StoredCallback

@external(erlang, "host", "restore_callback")
fn restore_callback(stored: StoredCallback) -> fn(Int) -> Int

@external(erlang, "host", "invoke_callback")
fn invoke_callback(stored: StoredCallback, value: Int) -> Int

@external(erlang, "host", "increment")
fn increment(value: Int) -> Int

fn callback(value: Int) {
  increment(value) + 1
}

pub fn main() {
  let stored = store_callback(callback)
  let restored = restore_callback(stored)
  #(restored == callback, restored(40), invoke_callback(stored, 40))
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
    .expect("stored callback source should compile");
    let plan = plan_host_program(typed).expect("stored callback source should plan");
    let execution =
        HostedExecution::try_from_module_plan(plan).expect("stored callback execution should seal");
    let expected = Value::Tuple(vec![
        Value::Bool(true),
        Value::Int(42.into()),
        Value::Int(42.into()),
    ]);

    assert_eq!(
        execution.run_main(&mut StoredRunState::default(), &mut Vec::new()),
        Ok(expected),
    );
}

#[test]
fn retained_graph_outlives_run_state_and_hosted_execution() {
    fn store<'call>(
        mut call: HostCall<'call, StoredProfile, StoredProvider, StoreMap>,
        key: HostValue<'call, FirstParameter>,
        value: HostValue<'call, SecondParameter>,
    ) -> Result<HostCallCompletion<'call, StoreMap>, HostCallError> {
        let drops = Arc::clone(&call.state().drops);
        let stored = call.create_external_with(|builder| StoredMapPayload {
            key: builder.store_argument::<HostTypeIndex0>(key),
            value: builder.store_argument::<HostTypeIndexNext<HostTypeIndex0>>(value),
            _drop: PayloadDrop(drops),
        });
        Ok(call.return_value(stored))
    }

    let provider = HostProviderModule::<StoredProfile>::new("application", "main")
        .expect("provider module should be valid")
        .with_external_type::<StoredMapSchema>()
        .expect("stored map type should be valid")
        .with_scoped_function::<StoredProvider, (FirstParameter, SecondParameter), StoreMap, _>(
            "store", store,
        )
        .expect("store provider should be valid");
    let source = r#"
@external(erlang, "host", "StoredMap")
pub type StoredMap(key, value)

@external(erlang, "host", "store")
fn store(key: key, value: value) -> StoredMap(key, value)

pub fn main() {
  let inner = store(1, "one")
  store([inner], #(inner))
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
    .expect("escaping stored source should compile");
    let plan = plan_host_program(typed).expect("escaping stored source should plan");
    let execution =
        HostedExecution::try_from_module_plan(plan).expect("stored execution should seal");
    let mut state = StoredRunState::default();
    let drops = Arc::clone(&state.drops);

    let result = execution
        .run_main(&mut state, &mut Vec::new())
        .expect("stored graph should escape as an opaque value");

    assert_eq!(
        result.inspect().to_string(),
        r#"StoredMap([StoredMap(1, "one")], #(StoredMap(1, "one")))"#,
    );
    assert_eq!(drops.load(Ordering::Relaxed), 0);

    drop(state);
    drop(execution);
    assert_eq!(drops.load(Ordering::Relaxed), 0);

    drop(result);
    assert_eq!(drops.load(Ordering::Relaxed), 2);
}

#[test]
fn releases_a_retained_value_after_host_failure() {
    fn store<'call>(
        mut call: HostCall<'call, StoredProfile, StoredProvider, StoreMap>,
        key: HostValue<'call, FirstParameter>,
        value: HostValue<'call, SecondParameter>,
    ) -> Result<HostCallCompletion<'call, StoreMap>, HostCallError> {
        let drops = Arc::clone(&call.state().drops);
        let stored = call.create_external_with(|builder| StoredMapPayload {
            key: builder.store_argument::<HostTypeIndex0>(key),
            value: builder.store_argument::<HostTypeIndexNext<HostTypeIndex0>>(value),
            _drop: PayloadDrop(drops),
        });
        Ok(call.return_value(stored))
    }

    fn fail<'call>(
        _call: HostCall<'call, StoredProfile, StoredProvider, ()>,
        _stored: HostExternal<'call, StoreMap>,
    ) -> Result<HostCallCompletion<'call, ()>, HostCallError> {
        Err(HostFailure::new("stored failure").into())
    }

    let provider = HostProviderModule::<StoredProfile>::new("application", "main")
        .expect("provider module should be valid")
        .with_external_type::<StoredMapSchema>()
        .expect("stored map type should be valid")
        .with_scoped_function::<StoredProvider, (FirstParameter, SecondParameter), StoreMap, _>(
            "store", store,
        )
        .expect("store provider should be valid")
        .with_scoped_function::<StoredProvider, (StoreMap,), (), _>("fail", fail)
        .expect("failure provider should be valid");
    let source = r#"
@external(erlang, "host", "StoredMap")
pub type StoredMap(key, value)

@external(erlang, "host", "store")
fn store(key: key, value: value) -> StoredMap(key, value)

@external(erlang, "host", "fail")
fn fail(map: StoredMap(key, value)) -> Nil

pub fn main() {
  fail(store([1, 2], #("retained", True)))
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
    .expect("stored failure source should compile");
    let plan = plan_host_program(typed).expect("stored failure source should plan");
    let execution =
        HostedExecution::try_from_module_plan(plan).expect("stored failure execution should seal");
    let mut state = StoredRunState::default();

    let error = execution
        .run_main(&mut state, &mut Vec::new())
        .expect_err("host failure should be returned");

    assert_eq!(
        error.to_string(),
        "host function application::main.fail failed: stored failure"
    );
    assert_eq!(state.drops.load(Ordering::Relaxed), 1);
}

#[test]
fn releases_a_retained_value_after_source_panic() {
    fn store<'call>(
        mut call: HostCall<'call, StoredProfile, StoredProvider, StoreMap>,
        key: HostValue<'call, FirstParameter>,
        value: HostValue<'call, SecondParameter>,
    ) -> Result<HostCallCompletion<'call, StoreMap>, HostCallError> {
        let drops = Arc::clone(&call.state().drops);
        let stored = call.create_external_with(|builder| StoredMapPayload {
            key: builder.store_argument::<HostTypeIndex0>(key),
            value: builder.store_argument::<HostTypeIndexNext<HostTypeIndex0>>(value),
            _drop: PayloadDrop(drops),
        });
        Ok(call.return_value(stored))
    }

    let provider = HostProviderModule::<StoredProfile>::new("application", "main")
        .expect("provider module should be valid")
        .with_external_type::<StoredMapSchema>()
        .expect("stored map type should be valid")
        .with_scoped_function::<StoredProvider, (FirstParameter, SecondParameter), StoreMap, _>(
            "store", store,
        )
        .expect("store provider should be valid");
    let source = r#"
@external(erlang, "host", "StoredMap")
pub type StoredMap(key, value)

@external(erlang, "host", "store")
fn store(key: key, value: value) -> StoredMap(key, value)

pub fn main() {
  let stored = store([1, 2], #("retained", True))
  let assert True = False
  stored
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
    .expect("stored panic source should compile");
    let plan = plan_host_program(typed).expect("stored panic source should plan");
    let execution =
        HostedExecution::try_from_module_plan(plan).expect("stored panic execution should seal");
    let mut state = StoredRunState::default();

    let error = execution
        .run_main(&mut state, &mut Vec::new())
        .expect_err("source panic should be returned");
    let ExecutionError::Panic(panic) = error else {
        panic!("let assert should remain a source panic");
    };

    assert_eq!(panic.kind(), PanicKind::LetAssert);
    assert_eq!(state.drops.load(Ordering::Relaxed), 1);
}
