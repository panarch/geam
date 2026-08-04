use ecow::EcoString;
use geam::{
    ExecutionError, HostCall, HostCallCompletion, HostCallError, HostCallable, HostExternal,
    HostExternalBinding, HostExternalEquality, HostExternalHashing, HostExternalInspection,
    HostExternalPayloadBuilder, HostExternalSchema, HostExternalStorage, HostExternalStore,
    HostExternalType, HostFailure, HostFunctionType, HostListType, HostProfile, HostProvider,
    HostProviderModule, HostProviderSet, HostStoredType, HostStoredValue, HostTypeIndex0,
    HostTypeIndexNext, HostTypeList, HostTypeListEnd, HostTypeParameter, HostValue,
    HostedExecution, ListValue, ModuleSource, PackageSource, PanicKind, Value,
    compile_typed_host_program, plan_host_program,
};
use num_bigint::BigInt;
use std::rc::Rc;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

struct TransientProfile;

#[derive(Default)]
struct TransientRunState {
    payload_drops: Arc<AtomicUsize>,
    entry_drops: Arc<AtomicUsize>,
    token_drops: Arc<AtomicUsize>,
}

#[derive(Default)]
struct TransientStores {
    maps: HostExternalStore<TransientPayload>,
    tokens: HostExternalStore<TokenPayload>,
}

struct TransientMapSchema;

struct TokenSchema;

struct TransientProvider;

struct TransientMapStorage;
struct TokenStorage;

struct TransientPayload {
    entries: Box<[Rc<TransientEntry>]>,
    _drop: PayloadDrop,
}

struct TransientEntry {
    key: HostStoredValue<StoredKey>,
    value: HostStoredValue<StoredItem>,
    _drop: EntryDrop,
}

struct TokenPayload {
    value: BigInt,
    _drop: TokenDrop,
}

struct PayloadDrop(Arc<AtomicUsize>);

struct EntryDrop(Arc<AtomicUsize>);

struct TokenDrop(Arc<AtomicUsize>);

type Key = HostTypeParameter<0>;
type Item = HostTypeParameter<1>;
type MapArguments = HostTypeList<Key, HostTypeList<Item, HostTypeListEnd>>;
type TransientMap = HostExternalType<TransientMapSchema, MapArguments>;
type StoredKey = HostStoredType<HostTypeIndex0>;
type StoredItem = HostStoredType<HostTypeIndexNext<HostTypeIndex0>>;
type ItemFunctionArguments = HostTypeList<Item, HostTypeListEnd>;
type ItemFunction = HostFunctionType<ItemFunctionArguments, Item>;
type Token = HostExternalType<TokenSchema>;
type ValuesKey = HostTypeParameter<1>;
type ValuesItem = HostTypeParameter<0>;
type ValuesMapArguments = HostTypeList<ValuesKey, HostTypeList<ValuesItem, HostTypeListEnd>>;
type ValuesMap = HostExternalType<TransientMapSchema, ValuesMapArguments>;

impl Drop for PayloadDrop {
    fn drop(&mut self) {
        self.0.fetch_add(1, Ordering::Relaxed);
    }
}

impl Drop for EntryDrop {
    fn drop(&mut self) {
        self.0.fetch_add(1, Ordering::Relaxed);
    }
}

impl Drop for TokenDrop {
    fn drop(&mut self) {
        self.0.fetch_add(1, Ordering::Relaxed);
    }
}

impl HostProfile for TransientProfile {
    type RunState = TransientRunState;
    type ExternalStores = TransientStores;
}

impl HostProvider<TransientProfile> for TransientProvider {
    type State = TransientRunState;

    fn project(state: &mut TransientRunState) -> &mut Self::State {
        state
    }
}

impl HostExternalSchema for TransientMapSchema {
    const PACKAGE: &'static str = "application";
    const MODULE: &'static str = "main";
    const NAME: &'static str = "TransientMap";
    const PARAMETER_COUNT: usize = 2;
}

impl HostExternalStorage<TransientProfile, TransientMapSchema> for TransientMapStorage {
    type Payload = TransientPayload;

    fn store(stores: &TransientStores) -> &HostExternalStore<Self::Payload> {
        &stores.maps
    }

    fn source_equal(
        context: &HostExternalEquality<'_>,
        left: &Self::Payload,
        right: &Self::Payload,
    ) -> bool {
        left.entries.len() == right.entries.len()
            && left
                .entries
                .iter()
                .zip(&right.entries)
                .all(|(left, right)| {
                    context.stored_values_equal(&left.key, &right.key)
                        && context.stored_values_equal(&left.value, &right.value)
                })
    }

    fn source_hash(context: &HostExternalHashing<'_>, value: &Self::Payload) -> u64 {
        value.entries.iter().fold(0, |hash, entry| {
            hash.rotate_left(1)
                ^ context.stored_value_hash(&entry.key).rotate_left(1)
                ^ context.stored_value_hash(&entry.value)
        })
    }

    fn inspect(context: &HostExternalInspection<'_>, value: &Self::Payload) -> EcoString {
        let entries = value
            .entries
            .iter()
            .map(|entry| {
                format!(
                    "#({}, {})",
                    context.inspect_stored_value(&entry.key),
                    context.inspect_stored_value(&entry.value),
                )
            })
            .collect::<Vec<_>>()
            .join(", ");
        format!("TransientMap([{entries}])").into()
    }
}

impl HostExternalSchema for TokenSchema {
    const PACKAGE: &'static str = "application";
    const MODULE: &'static str = "main";
    const NAME: &'static str = "Token";
    const PARAMETER_COUNT: usize = 0;
}

impl HostExternalStorage<TransientProfile, TokenSchema> for TokenStorage {
    type Payload = TokenPayload;

    fn store(stores: &TransientStores) -> &HostExternalStore<Self::Payload> {
        &stores.tokens
    }

    fn source_equal(
        _: &HostExternalEquality<'_>,
        left: &Self::Payload,
        right: &Self::Payload,
    ) -> bool {
        left.value == right.value
    }

    fn source_hash(_: &HostExternalHashing<'_>, value: &Self::Payload) -> u64 {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        std::hash::Hash::hash(&value.value, &mut hasher);
        std::hash::Hasher::finish(&hasher)
    }

    fn inspect(_: &HostExternalInspection<'_>, value: &Self::Payload) -> EcoString {
        format!("Token({})", value.value).into()
    }
}

impl HostExternalBinding<TransientProfile, TransientMapSchema> for TransientProvider {
    type Storage = TransientMapStorage;
}

impl HostExternalBinding<TransientProfile, TokenSchema> for TransientProvider {
    type Storage = TokenStorage;
}

fn new_entry(
    builder: &mut HostExternalPayloadBuilder<'_, TransientProfile, MapArguments>,
    key: HostValue<'_, Key>,
    value: HostValue<'_, Item>,
    drops: Arc<AtomicUsize>,
) -> Rc<TransientEntry> {
    Rc::new(TransientEntry {
        key: builder.store_argument::<HostTypeIndex0>(key),
        value: builder.store_argument::<HostTypeIndexNext<HostTypeIndex0>>(value),
        _drop: EntryDrop(drops),
    })
}

fn empty<'call>(
    mut call: HostCall<'call, TransientProfile, TransientProvider, TransientMap>,
) -> Result<HostCallCompletion<'call, TransientMap>, HostCallError> {
    let drops = Arc::clone(&call.state().payload_drops);
    let map = call.create_external(TransientPayload {
        entries: Box::new([]),
        _drop: PayloadDrop(drops),
    });
    Ok(call.return_value(map))
}

fn insert<'call>(
    mut call: HostCall<'call, TransientProfile, TransientProvider, TransientMap>,
    map: HostExternal<'call, TransientMap>,
    key: HostValue<'call, Key>,
    value: HostValue<'call, Item>,
) -> Result<HostCallCompletion<'call, TransientMap>, HostCallError> {
    let payload = call.external_payload(map);
    let mut replacement = None;
    for index in 0..payload.entries.len() {
        let current = payload.restore_argument(&mut call, |payload| &payload.entries[index].key);
        if call.equal::<Key>(current, key) {
            replacement = Some(index);
            break;
        }
    }
    let mut entries = payload.entries.to_vec();
    let payload_drops = Arc::clone(&call.state().payload_drops);
    let entry_drops = Arc::clone(&call.state().entry_drops);
    let map = call.create_external_with(move |builder| {
        let entry = new_entry(builder, key, value, entry_drops);
        match replacement {
            Some(index) => entries[index] = entry,
            None => entries.push(entry),
        }
        TransientPayload {
            entries: entries.into_boxed_slice(),
            _drop: PayloadDrop(payload_drops),
        }
    });
    Ok(call.return_value(map))
}

fn remove<'call>(
    mut call: HostCall<'call, TransientProfile, TransientProvider, TransientMap>,
    map: HostExternal<'call, TransientMap>,
    key: HostValue<'call, Key>,
) -> Result<HostCallCompletion<'call, TransientMap>, HostCallError> {
    let payload = call.external_payload(map);
    let mut entries = Vec::new();
    for index in 0..payload.entries.len() {
        let current = payload.restore_argument(&mut call, |payload| &payload.entries[index].key);
        if !call.equal::<Key>(current, key) {
            entries.push(Rc::clone(&payload.entries[index]));
        }
    }
    let drops = Arc::clone(&call.state().payload_drops);
    let map = call.create_external(TransientPayload {
        entries: entries.into_boxed_slice(),
        _drop: PayloadDrop(drops),
    });
    Ok(call.return_value(map))
}

fn merge<'call>(
    mut call: HostCall<'call, TransientProfile, TransientProvider, TransientMap>,
    left: HostExternal<'call, TransientMap>,
    right: HostExternal<'call, TransientMap>,
) -> Result<HostCallCompletion<'call, TransientMap>, HostCallError> {
    let left = call.external_payload(left);
    let right = call.external_payload(right);
    let mut entries = left.entries.to_vec();
    let mut keys = Vec::with_capacity(left.entries.len() + right.entries.len());
    for index in 0..left.entries.len() {
        keys.push(left.restore_argument(&mut call, |payload| &payload.entries[index].key));
    }
    for right_index in 0..right.entries.len() {
        let key = right.restore_argument(&mut call, |payload| &payload.entries[right_index].key);
        let replacement = keys
            .iter()
            .position(|current| call.equal::<Key>(*current, key));
        match replacement {
            Some(target_index) => {
                keys[target_index] = key;
                entries[target_index] = Rc::clone(&right.entries[right_index]);
            }
            None => {
                keys.push(key);
                entries.push(Rc::clone(&right.entries[right_index]));
            }
        }
    }
    let drops = Arc::clone(&call.state().payload_drops);
    let map = call.create_external(TransientPayload {
        entries: entries.into_boxed_slice(),
        _drop: PayloadDrop(drops),
    });
    Ok(call.return_value(map))
}

fn keys<'call>(
    mut call: HostCall<'call, TransientProfile, TransientProvider, HostListType<Key>>,
    map: HostExternal<'call, TransientMap>,
) -> Result<HostCallCompletion<'call, HostListType<Key>>, HostCallError> {
    let payload = call.external_payload(map);
    let mut values = Vec::with_capacity(payload.entries.len());
    for index in 0..payload.entries.len() {
        values.push(payload.restore_argument(&mut call, |payload| &payload.entries[index].key));
    }
    Ok(call.return_list(values))
}

fn values<'call>(
    mut call: HostCall<'call, TransientProfile, TransientProvider, HostListType<ValuesItem>>,
    map: HostExternal<'call, ValuesMap>,
) -> Result<HostCallCompletion<'call, HostListType<ValuesItem>>, HostCallError> {
    let payload = call.external_payload(map);
    let mut values = Vec::with_capacity(payload.entries.len());
    for index in 0..payload.entries.len() {
        values.push(payload.restore_argument(&mut call, |payload| &payload.entries[index].value));
    }
    Ok(call.return_list(values))
}

fn length<'call>(
    call: HostCall<'call, TransientProfile, TransientProvider, BigInt>,
    map: HostExternal<'call, TransientMap>,
) -> Result<HostCallCompletion<'call, BigInt>, HostCallError> {
    let length = call.external_payload(map).entries.len();
    Ok(call.return_value(BigInt::from(length)))
}

fn map_values<'call>(
    mut call: HostCall<'call, TransientProfile, TransientProvider, TransientMap>,
    map: HostExternal<'call, TransientMap>,
    function: HostCallable<'call, ItemFunctionArguments, Item>,
) -> Result<HostCallCompletion<'call, TransientMap>, HostCallError> {
    let payload = call.external_payload(map);
    let mut mapped = Vec::with_capacity(payload.entries.len());
    for index in 0..payload.entries.len() {
        let key = payload.restore_argument(&mut call, |payload| &payload.entries[index].key);
        let value = payload.restore_argument(&mut call, |payload| &payload.entries[index].value);
        let value = call.invoke(function, (value, ()))?;
        mapped.push((key, value));
    }
    let payload_drops = Arc::clone(&call.state().payload_drops);
    let entry_drops = Arc::clone(&call.state().entry_drops);
    let map = call.create_external_with(move |builder| TransientPayload {
        entries: mapped
            .into_iter()
            .map(|(key, value)| new_entry(builder, key, value, Arc::clone(&entry_drops)))
            .collect::<Vec<_>>()
            .into_boxed_slice(),
        _drop: PayloadDrop(payload_drops),
    });
    Ok(call.return_value(map))
}

fn make_token<'call>(
    mut call: HostCall<'call, TransientProfile, TransientProvider, Token>,
    value: BigInt,
) -> Result<HostCallCompletion<'call, Token>, HostCallError> {
    let drops = Arc::clone(&call.state().token_drops);
    let token = call.create_external(TokenPayload {
        value,
        _drop: TokenDrop(drops),
    });
    Ok(call.return_value(token))
}

#[test]
fn round_trips_grouped_insert_replace_remove_and_merge() {
    let provider = HostProviderModule::<TransientProfile>::new("application", "main")
        .expect("provider module should be valid")
        .with_external_type::<TransientProvider, TransientMapSchema>()
        .expect("transient map type should be valid")
        .with_scoped_function::<TransientProvider, (), TransientMap, _>("empty", empty)
        .expect("empty provider should be valid")
        .with_scoped_function::<TransientProvider, (TransientMap, Key, Item), TransientMap, _>(
            "insert", insert,
        )
        .expect("insert provider should be valid")
        .with_scoped_function::<TransientProvider, (TransientMap, Key), TransientMap, _>(
            "remove", remove,
        )
        .expect("remove provider should be valid")
        .with_scoped_function::<TransientProvider, (TransientMap, TransientMap), TransientMap, _>(
            "merge", merge,
        )
        .expect("merge provider should be valid")
        .with_scoped_function::<TransientProvider, (TransientMap,), HostListType<Key>, _>(
            "keys", keys,
        )
        .expect("keys provider should be valid")
        .with_scoped_function::<TransientProvider, (ValuesMap,), HostListType<ValuesItem>, _>(
            "values", values,
        )
        .expect("values provider should be valid")
        .with_scoped_function::<TransientProvider, (TransientMap,), BigInt, _>("length", length)
        .expect("length provider should be valid");
    let source = r#"
pub type Map(key, value) {
  Map(keys: List(key), values: List(value))
}

@external(erlang, "host", "TransientMap")
pub type TransientMap(key, value)

@external(erlang, "host", "empty")
fn empty() -> TransientMap(key, value)

@external(erlang, "host", "insert")
fn insert(map: TransientMap(key, value), key: key, value: value) -> TransientMap(key, value)

@external(erlang, "host", "remove")
fn remove(map: TransientMap(key, value), key: key) -> TransientMap(key, value)

@external(erlang, "host", "merge")
fn merge(
  left: TransientMap(key, value),
  right: TransientMap(key, value),
) -> TransientMap(key, value)

@external(erlang, "host", "keys")
fn keys(map: TransientMap(key, value)) -> List(key)

@external(erlang, "host", "values")
fn values(map: TransientMap(key, value)) -> List(value)

@external(erlang, "host", "length")
fn length(map: TransientMap(key, value)) -> Int

fn from_map(map: Map(key, value)) -> TransientMap(key, value) {
  let Map(map_keys, map_values) = map
  from_lists(map_keys, map_values, empty())
}

fn from_lists(
  map_keys: List(key),
  map_values: List(value),
  transient: TransientMap(key, value),
) -> TransientMap(key, value) {
  case #(map_keys, map_values) {
    #([], []) -> transient
    #([key, ..keys], [value, ..values]) ->
      from_lists(keys, values, insert(transient, key, value))
    _ -> transient
  }
}

fn to_map(transient: TransientMap(key, value)) -> Map(key, value) {
  Map(keys(transient), values(transient))
}

pub fn main() {
  let grouped = from_map(Map([1, 2, 1], ["one", "two", "updated"]))
  let removed = remove(grouped, 2)
  let other = insert(insert(empty(), 2, "TWO"), 3, "three")
  let merged = merge(grouped, other)
  let Map(grouped_keys, grouped_values) = to_map(grouped)
  let Map(removed_keys, removed_values) = to_map(removed)
  let Map(other_keys, other_values) = to_map(other)
  let Map(merged_keys, merged_values) = to_map(merged)

  #(
    grouped_keys,
    grouped_values,
    removed_keys,
    removed_values,
    other_keys,
    other_values,
    merged_keys,
    merged_values,
    length(merged),
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
    .expect("transient map source should compile");
    let plan = plan_host_program(typed).expect("transient map source should plan");
    let execution =
        HostedExecution::try_from_module_plan(plan).expect("transient map execution should seal");
    let mut state = TransientRunState::default();

    let actual = execution.run_main(&mut state, &mut Vec::new());

    assert_eq!(
        actual,
        Ok(Value::Tuple(vec![
            Value::List(ListValue::int(vec![1.into(), 2.into()])),
            Value::List(ListValue::string(vec!["updated".into(), "two".into()])),
            Value::List(ListValue::int(vec![1.into()])),
            Value::List(ListValue::string(vec!["updated".into()])),
            Value::List(ListValue::int(vec![2.into(), 3.into()])),
            Value::List(ListValue::string(vec!["TWO".into(), "three".into()])),
            Value::List(ListValue::int(vec![1.into(), 2.into(), 3.into()])),
            Value::List(ListValue::string(vec![
                "updated".into(),
                "TWO".into(),
                "three".into(),
            ])),
            Value::Int(3.into()),
        ])),
    );
    assert_eq!(state.payload_drops.load(Ordering::Relaxed), 9);
    assert_eq!(state.entry_drops.load(Ordering::Relaxed), 5);
}

#[test]
fn preserves_aliased_compound_keys_and_external_values() {
    let provider = HostProviderModule::<TransientProfile>::new("application", "main")
        .expect("provider module should be valid")
        .with_external_type::<TransientProvider, TransientMapSchema>()
        .expect("transient map type should be valid")
        .with_external_type::<TransientProvider, TokenSchema>()
        .expect("token type should be valid")
        .with_scoped_function::<TransientProvider, (), TransientMap, _>("empty", empty)
        .expect("empty provider should be valid")
        .with_scoped_function::<TransientProvider, (TransientMap, Key, Item), TransientMap, _>(
            "insert", insert,
        )
        .expect("insert provider should be valid")
        .with_scoped_function::<TransientProvider, (TransientMap,), HostListType<Key>, _>(
            "keys", keys,
        )
        .expect("keys provider should be valid")
        .with_scoped_function::<TransientProvider, (ValuesMap,), HostListType<ValuesItem>, _>(
            "values", values,
        )
        .expect("values provider should be valid")
        .with_scoped_function::<TransientProvider, (BigInt,), Token, _>("make_token", make_token)
        .expect("token provider should be valid");
    let source = r#"
pub type Boxed {
  Boxed(Bool)
}

@external(erlang, "host", "TransientMap")
pub type TransientMap(key, value)

@external(erlang, "host", "Token")
pub type Token

@external(erlang, "host", "empty")
fn empty() -> TransientMap(key, value)

@external(erlang, "host", "insert")
fn insert(map: TransientMap(key, value), key: key, value: value) -> TransientMap(key, value)

@external(erlang, "host", "keys")
fn keys(map: TransientMap(key, value)) -> List(key)

@external(erlang, "host", "values")
fn values(map: TransientMap(key, value)) -> List(value)

@external(erlang, "host", "make_token")
fn make_token(value: Int) -> Token

pub fn main() {
  let key = #(1, Boxed(True))
  let first = insert(empty(), key, make_token(10))
  let alias = first
  let updated = insert(first, key, make_token(20))

  #(keys(alias), values(alias), keys(updated), values(updated))
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
    .expect("aliased transient source should compile");
    let plan = plan_host_program(typed).expect("aliased transient source should plan");
    let execution = HostedExecution::try_from_module_plan(plan)
        .expect("aliased transient execution should seal");
    let mut state = TransientRunState::default();

    let actual = execution
        .run_main(&mut state, &mut Vec::new())
        .expect("aliased transient source should run");

    assert_eq!(
        actual.inspect().to_string(),
        "#([#(1, Boxed(True))], [Token(10)], [#(1, Boxed(True))], [Token(20)])",
    );
    assert_eq!(state.payload_drops.load(Ordering::Relaxed), 3);
    assert_eq!(state.entry_drops.load(Ordering::Relaxed), 2);
    assert_eq!(state.token_drops.load(Ordering::Relaxed), 0);

    drop(actual);
    assert_eq!(state.token_drops.load(Ordering::Relaxed), 2);
}

#[test]
fn maps_large_transient_values_through_nested_reentry() {
    let provider = HostProviderModule::<TransientProfile>::new("application", "main")
        .expect("provider module should be valid")
        .with_external_type::<TransientProvider, TransientMapSchema>()
        .expect("transient map type should be valid")
        .with_scoped_function::<TransientProvider, (), TransientMap, _>("empty", empty)
        .expect("empty provider should be valid")
        .with_scoped_function::<TransientProvider, (TransientMap, Key, Item), TransientMap, _>(
            "insert", insert,
        )
        .expect("insert provider should be valid")
        .with_scoped_function::<TransientProvider, (ValuesMap,), HostListType<ValuesItem>, _>(
            "values", values,
        )
        .expect("values provider should be valid")
        .with_scoped_function::<TransientProvider, (TransientMap,), BigInt, _>("length", length)
        .expect("length provider should be valid")
        .with_scoped_function::<TransientProvider, (TransientMap, ItemFunction), TransientMap, _>(
            "map_values",
            map_values,
        )
        .expect("map values provider should be valid")
        .with_function("increment", |value: BigInt| value + 1)
        .expect("increment provider should be valid");
    let source = r#"
@external(erlang, "host", "TransientMap")
pub type TransientMap(key, value)

@external(erlang, "host", "empty")
fn empty() -> TransientMap(key, value)

@external(erlang, "host", "insert")
fn insert(map: TransientMap(key, value), key: key, value: value) -> TransientMap(key, value)

@external(erlang, "host", "values")
fn values(map: TransientMap(key, value)) -> List(value)

@external(erlang, "host", "length")
fn length(map: TransientMap(key, value)) -> Int

@external(erlang, "host", "map_values")
fn map_values(
  map: TransientMap(key, value),
  function: fn(value) -> value,
) -> TransientMap(key, value)

@external(erlang, "host", "increment")
fn increment(value: Int) -> Int

fn fill(
  map: TransientMap(Int, Int),
  current: Int,
) -> TransientMap(Int, Int) {
  case current == 0 {
    True -> map
    False -> fill(insert(map, current, current), current - 1)
  }
}

fn bump(value: Int) -> Int {
  increment(value)
}

pub fn main() {
  let base = fill(empty(), 128)
  let mapped = map_values(base, bump)
  let assert [first, ..] = values(mapped)

  #(length(base), length(mapped), first)
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
    .expect("large transient source should compile");
    let plan = plan_host_program(typed).expect("large transient source should plan");
    let execution =
        HostedExecution::try_from_module_plan(plan).expect("large transient execution should seal");
    let mut state = TransientRunState::default();

    let actual = execution.run_main(&mut state, &mut Vec::new());

    assert_eq!(
        actual,
        Ok(Value::Tuple(vec![
            Value::Int(128.into()),
            Value::Int(128.into()),
            Value::Int(129.into()),
        ])),
    );
    assert_eq!(state.payload_drops.load(Ordering::Relaxed), 130);
    assert_eq!(state.entry_drops.load(Ordering::Relaxed), 256);
}

#[test]
fn releases_transient_storage_after_nested_host_failure() {
    let provider = HostProviderModule::<TransientProfile>::new("application", "main")
        .expect("provider module should be valid")
        .with_external_type::<TransientProvider, TransientMapSchema>()
        .expect("transient map type should be valid")
        .with_scoped_function::<TransientProvider, (), TransientMap, _>("empty", empty)
        .expect("empty provider should be valid")
        .with_scoped_function::<TransientProvider, (TransientMap, Key, Item), TransientMap, _>(
            "insert", insert,
        )
        .expect("insert provider should be valid")
        .with_scoped_function::<TransientProvider, (TransientMap, ItemFunction), TransientMap, _>(
            "map_values",
            map_values,
        )
        .expect("map values provider should be valid")
        .with_fallible_function("fail_at_two", |value: BigInt| {
            if value == BigInt::from(2) {
                Err(HostFailure::new("value two is unavailable"))
            } else {
                Ok(value)
            }
        })
        .expect("failure provider should be valid");
    let source = r#"
@external(erlang, "host", "TransientMap")
pub type TransientMap(key, value)

@external(erlang, "host", "empty")
fn empty() -> TransientMap(key, value)

@external(erlang, "host", "insert")
fn insert(map: TransientMap(key, value), key: key, value: value) -> TransientMap(key, value)

@external(erlang, "host", "map_values")
fn map_values(
  map: TransientMap(key, value),
  function: fn(value) -> value,
) -> TransientMap(key, value)

@external(erlang, "host", "fail_at_two")
fn fail_at_two(value: Int) -> Int

fn reject_two(value: Int) -> Int {
  fail_at_two(value)
}

pub fn main() {
  let map =
    insert(
      insert(
        insert(empty(), 1, 1),
        2,
        2,
      ),
      3,
      3,
    )

  map_values(map, reject_two)
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
    .expect("failing transient source should compile");
    let plan = plan_host_program(typed).expect("failing transient source should plan");
    let execution = HostedExecution::try_from_module_plan(plan)
        .expect("failing transient execution should seal");
    let mut state = TransientRunState::default();

    let error = execution
        .run_main(&mut state, &mut Vec::new())
        .expect_err("nested host callback should fail");
    let ExecutionError::Host(error) = error else {
        panic!("nested provider failure should remain a host error");
    };

    assert_eq!(error.package(), "application");
    assert_eq!(error.module(), "main");
    assert_eq!(error.function(), "fail_at_two");
    assert_eq!(error.failure().message(), "value two is unavailable");
    assert_eq!(state.payload_drops.load(Ordering::Relaxed), 4);
    assert_eq!(state.entry_drops.load(Ordering::Relaxed), 3);
}

#[test]
fn releases_transient_storage_after_source_panic() {
    let provider = HostProviderModule::<TransientProfile>::new("application", "main")
        .expect("provider module should be valid")
        .with_external_type::<TransientProvider, TransientMapSchema>()
        .expect("transient map type should be valid")
        .with_scoped_function::<TransientProvider, (), TransientMap, _>("empty", empty)
        .expect("empty provider should be valid")
        .with_scoped_function::<TransientProvider, (TransientMap, Key, Item), TransientMap, _>(
            "insert", insert,
        )
        .expect("insert provider should be valid");
    let source = r#"
@external(erlang, "host", "TransientMap")
pub type TransientMap(key, value)

@external(erlang, "host", "empty")
fn empty() -> TransientMap(key, value)

@external(erlang, "host", "insert")
fn insert(map: TransientMap(key, value), key: key, value: value) -> TransientMap(key, value)

pub fn main() {
  let map = insert(insert(empty(), 1, "one"), 2, "two")
  let assert True = False
  map
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
    .expect("panicking transient source should compile");
    let plan = plan_host_program(typed).expect("panicking transient source should plan");
    let execution = HostedExecution::try_from_module_plan(plan)
        .expect("panicking transient execution should seal");
    let mut state = TransientRunState::default();

    let error = execution
        .run_main(&mut state, &mut Vec::new())
        .expect_err("source should panic");
    let ExecutionError::Panic(panic) = error else {
        panic!("let assert should remain a source panic");
    };

    assert_eq!(panic.kind(), PanicKind::LetAssert);
    assert_eq!(state.payload_drops.load(Ordering::Relaxed), 3);
    assert_eq!(state.entry_drops.load(Ordering::Relaxed), 2);
}
