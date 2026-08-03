use super::schema::{
    Dict, FoldAccumulator, FoldDict, FoldFunctionArguments, GetDict, GetError, GetKey, GetOk,
    GetResult, Item, ItemIndex, Key, KeyIndex, MapFunctionArguments, MapInputDict, MapOutput,
    MapOutputDict, TransientDict, UpdateFunctionArguments,
};
use super::storage::{
    DictEntry, DictPayload, DictPayloadStorage, DictStorage, TransientDictPayload,
};
use crate::gleam_stdlib::GleamStdlibHostProfile;
use crate::{
    HostCall, HostCallCompletion, HostCallError, HostCallable, HostExternal,
    HostExternalPayloadBuilder, HostExternalPayloadView, HostProfile, HostProvider, HostType,
    HostTypeAt, HostTypeSequence, HostValue,
};
use num_bigint::BigInt;
use std::marker::PhantomData;
use std::rc::Rc;

pub(super) struct DictProvider<Profile>(PhantomData<Profile>);

impl<Profile> HostProvider<Profile> for DictProvider<Profile>
where
    Profile: GleamStdlibHostProfile,
{
    type State = Profile::RunState;

    fn project(state: &mut Profile::RunState) -> &mut Self::State {
        state
    }
}

fn matching_entry<'call, Profile, Provider, Return, Payload, Arguments>(
    call: &mut HostCall<'call, Profile, Provider, Return>,
    payload: &HostExternalPayloadView<'call, Payload, Arguments>,
    key_hash: u64,
    key: <<Arguments as HostTypeAt<KeyIndex>>::Type as HostType>::Value<'call>,
) -> Option<usize>
where
    Profile: HostProfile,
    Provider: HostProvider<Profile>,
    Return: HostType,
    Payload: DictPayloadStorage,
    Arguments: HostTypeSequence + HostTypeAt<KeyIndex>,
{
    payload.storage().matching_index(key_hash, &mut |index| {
        let candidate = payload
            .restore_argument::<Profile, Provider, Return, KeyIndex>(call, |payload| {
                &payload.storage().buckets[&key_hash][index].key
            });
        call.equal::<<Arguments as HostTypeAt<KeyIndex>>::Type>(candidate, key.clone())
    })
}

pub(in crate::gleam_stdlib) fn lookup<'call, Profile, Provider, Return, KeyType, ValueType>(
    call: &mut HostCall<'call, Profile, Provider, Return>,
    dict: HostExternal<'call, super::schema::DictOf<KeyType, ValueType>>,
    key: KeyType::Value<'call>,
) -> Option<ValueType::Value<'call>>
where
    Profile: GleamStdlibHostProfile,
    Provider: HostProvider<Profile>,
    Return: HostType,
    KeyType: HostType,
    ValueType: HostType,
{
    let key_hash = call.source_hash::<KeyType>(key.clone());
    let payload = call.external_payload(dict);
    let index = matching_entry(call, &payload, key_hash, key)?;
    Some(
        payload.restore_argument::<Profile, Provider, Return, ItemIndex>(call, |payload| {
            &payload.storage.buckets[&key_hash][index].value
        }),
    )
}

fn create_entry<'call, Profile, Arguments>(
    builder: &mut HostExternalPayloadBuilder<'_, Profile, Arguments>,
    key_hash: u64,
    key: <<Arguments as HostTypeAt<KeyIndex>>::Type as HostType>::Value<'call>,
    value: <<Arguments as HostTypeAt<ItemIndex>>::Type as HostType>::Value<'call>,
) -> Rc<DictEntry>
where
    Profile: HostProfile,
    Arguments: HostTypeSequence + HostTypeAt<KeyIndex> + HostTypeAt<ItemIndex>,
{
    Rc::new(DictEntry {
        key_hash,
        key: builder.store_argument::<KeyIndex>(key),
        value: builder.store_argument::<ItemIndex>(value),
    })
}

pub(super) fn to_transient<'call, Profile>(
    mut call: HostCall<'call, Profile, DictProvider<Profile>, TransientDict>,
    dict: HostExternal<'call, Dict>,
) -> Result<HostCallCompletion<'call, TransientDict>, HostCallError>
where
    Profile: GleamStdlibHostProfile,
{
    let storage = call.external_payload(dict).storage.clone();
    let transient = call.create_external(TransientDictPayload { storage });
    Ok(call.return_value(transient))
}

pub(super) fn from_transient<'call, Profile>(
    mut call: HostCall<'call, Profile, DictProvider<Profile>, Dict>,
    transient: HostExternal<'call, TransientDict>,
) -> Result<HostCallCompletion<'call, Dict>, HostCallError>
where
    Profile: GleamStdlibHostProfile,
{
    let storage = call.external_payload(transient).storage.clone();
    let dict = call.create_external(DictPayload { storage });
    Ok(call.return_value(dict))
}

pub(super) fn size<'call, Profile>(
    call: HostCall<'call, Profile, DictProvider<Profile>, BigInt>,
    dict: HostExternal<'call, Dict>,
) -> Result<HostCallCompletion<'call, BigInt>, HostCallError>
where
    Profile: GleamStdlibHostProfile,
{
    let size = call.external_payload(dict).storage.len;
    Ok(call.return_value(BigInt::from(size)))
}

pub(super) fn do_has_key<'call, Profile>(
    mut call: HostCall<'call, Profile, DictProvider<Profile>, bool>,
    key: HostValue<'call, Key>,
    dict: HostExternal<'call, Dict>,
) -> Result<HostCallCompletion<'call, bool>, HostCallError>
where
    Profile: GleamStdlibHostProfile,
{
    let key_hash = call.source_hash::<Key>(key);
    let payload = call.external_payload(dict);
    let found = matching_entry(&mut call, &payload, key_hash, key).is_some();
    Ok(call.return_value(found))
}

pub(super) fn new<'call, Profile>(
    mut call: HostCall<'call, Profile, DictProvider<Profile>, Dict>,
) -> Result<HostCallCompletion<'call, Dict>, HostCallError>
where
    Profile: GleamStdlibHostProfile,
{
    let dict = call.create_external(DictPayload {
        storage: DictStorage::default(),
    });
    Ok(call.return_value(dict))
}

pub(super) fn get<'call, Profile>(
    mut call: HostCall<'call, Profile, DictProvider<Profile>, GetResult>,
    dict: HostExternal<'call, GetDict>,
    key: HostValue<'call, GetKey>,
) -> Result<HostCallCompletion<'call, GetResult>, HostCallError>
where
    Profile: GleamStdlibHostProfile,
{
    let key_hash = call.source_hash::<GetKey>(key);
    let payload = call.external_payload(dict);
    let Some(index) = matching_entry(&mut call, &payload, key_hash, key) else {
        return Ok(call.return_custom::<GetError>(((), ())));
    };
    let value = payload.restore_argument::<Profile, DictProvider<Profile>, GetResult, ItemIndex>(
        &mut call,
        |payload| &payload.storage.buckets[&key_hash][index].value,
    );
    Ok(call.return_custom::<GetOk>((value, ())))
}

pub(super) fn do_insert<'call, Profile>(
    mut call: HostCall<'call, Profile, DictProvider<Profile>, Dict>,
    key: HostValue<'call, Key>,
    value: HostValue<'call, Item>,
    dict: HostExternal<'call, Dict>,
) -> Result<HostCallCompletion<'call, Dict>, HostCallError>
where
    Profile: GleamStdlibHostProfile,
{
    let key_hash = call.source_hash::<Key>(key);
    let payload = call.external_payload(dict);
    let index = matching_entry(&mut call, &payload, key_hash, key);
    let storage = payload.storage.clone();
    let dict = call.create_external_with(move |builder| DictPayload {
        storage: storage.with_entry(key_hash, index, create_entry(builder, key_hash, key, value)),
    });
    Ok(call.return_value(dict))
}

pub(super) fn transient_insert<'call, Profile>(
    mut call: HostCall<'call, Profile, DictProvider<Profile>, TransientDict>,
    key: HostValue<'call, Key>,
    value: HostValue<'call, Item>,
    transient: HostExternal<'call, TransientDict>,
) -> Result<HostCallCompletion<'call, TransientDict>, HostCallError>
where
    Profile: GleamStdlibHostProfile,
{
    let key_hash = call.source_hash::<Key>(key);
    let payload = call.external_payload(transient);
    let index = matching_entry(&mut call, &payload, key_hash, key);
    let storage = payload.storage.clone();
    let transient = call.create_external_with(move |builder| TransientDictPayload {
        storage: storage.with_entry(key_hash, index, create_entry(builder, key_hash, key, value)),
    });
    Ok(call.return_value(transient))
}

pub(super) fn do_map_values<'call, Profile>(
    mut call: HostCall<'call, Profile, DictProvider<Profile>, MapOutputDict>,
    function: HostCallable<'call, MapFunctionArguments, MapOutput>,
    dict: HostExternal<'call, MapInputDict>,
) -> Result<HostCallCompletion<'call, MapOutputDict>, HostCallError>
where
    Profile: GleamStdlibHostProfile,
{
    let payload = call.external_payload(dict);
    let mut values = Vec::with_capacity(payload.storage.len);
    let coordinates = payload
        .storage
        .buckets
        .iter()
        .flat_map(|(key_hash, bucket)| (0..bucket.len()).map(move |index| (*key_hash, index)))
        .collect::<Vec<_>>();
    for (key_hash, index) in coordinates {
        let key = payload
            .restore_argument::<Profile, DictProvider<Profile>, MapOutputDict, KeyIndex>(
                &mut call,
                |payload| &payload.storage.buckets[&key_hash][index].key,
            );
        let value = payload
            .restore_argument::<Profile, DictProvider<Profile>, MapOutputDict, ItemIndex>(
                &mut call,
                |payload| &payload.storage.buckets[&key_hash][index].value,
            );
        let value = call.invoke(function, (key, (value, ())))?;
        values.push((key_hash, key, value));
    }
    let dict = call.create_external_with(move |builder| {
        let mut storage = DictStorage::default();
        for (key_hash, key, value) in values {
            storage =
                storage.with_entry(key_hash, None, create_entry(builder, key_hash, key, value));
        }
        DictPayload { storage }
    });
    Ok(call.return_value(dict))
}

pub(super) fn transient_delete<'call, Profile>(
    mut call: HostCall<'call, Profile, DictProvider<Profile>, TransientDict>,
    key: HostValue<'call, Key>,
    transient: HostExternal<'call, TransientDict>,
) -> Result<HostCallCompletion<'call, TransientDict>, HostCallError>
where
    Profile: GleamStdlibHostProfile,
{
    let key_hash = call.source_hash::<Key>(key);
    let payload = call.external_payload(transient);
    let Some(index) = matching_entry(&mut call, &payload, key_hash, key) else {
        return Ok(call.return_value(transient));
    };
    let storage = payload.storage.without_entry(key_hash, index);
    let transient = call.create_external(TransientDictPayload { storage });
    Ok(call.return_value(transient))
}

pub(super) fn do_fold<'call, Profile>(
    mut call: HostCall<'call, Profile, DictProvider<Profile>, FoldAccumulator>,
    function: HostCallable<'call, FoldFunctionArguments, FoldAccumulator>,
    mut accumulator: HostValue<'call, FoldAccumulator>,
    dict: HostExternal<'call, FoldDict>,
) -> Result<HostCallCompletion<'call, FoldAccumulator>, HostCallError>
where
    Profile: GleamStdlibHostProfile,
{
    let payload = call.external_payload(dict);
    let coordinates = payload
        .storage
        .buckets
        .iter()
        .flat_map(|(key_hash, bucket)| (0..bucket.len()).map(move |index| (*key_hash, index)))
        .collect::<Vec<_>>();
    for (key_hash, index) in coordinates {
        let key = payload
            .restore_argument::<Profile, DictProvider<Profile>, FoldAccumulator, KeyIndex>(
                &mut call,
                |payload| &payload.storage.buckets[&key_hash][index].key,
            );
        let value = payload
            .restore_argument::<Profile, DictProvider<Profile>, FoldAccumulator, ItemIndex>(
                &mut call,
                |payload| &payload.storage.buckets[&key_hash][index].value,
            );
        accumulator = call.invoke(function, (key, (value, (accumulator, ()))))?;
    }
    Ok(call.return_value(accumulator))
}

pub(super) fn transient_update_with<'call, Profile>(
    mut call: HostCall<'call, Profile, DictProvider<Profile>, TransientDict>,
    key: HostValue<'call, Key>,
    function: HostCallable<'call, UpdateFunctionArguments, Item>,
    init: HostValue<'call, Item>,
    transient: HostExternal<'call, TransientDict>,
) -> Result<HostCallCompletion<'call, TransientDict>, HostCallError>
where
    Profile: GleamStdlibHostProfile,
{
    let key_hash = call.source_hash::<Key>(key);
    let payload = call.external_payload(transient);
    let index = matching_entry(&mut call, &payload, key_hash, key);
    let value = match index {
        Some(index) => {
            let current = payload
                .restore_argument::<Profile, DictProvider<Profile>, TransientDict, ItemIndex>(
                    &mut call,
                    |payload| &payload.storage.buckets[&key_hash][index].value,
                );
            call.invoke(function, (current, ()))?
        }
        None => init,
    };
    let storage = payload.storage.clone();
    let transient = call.create_external_with(move |builder| TransientDictPayload {
        storage: storage.with_entry(key_hash, index, create_entry(builder, key_hash, key, value)),
    });
    Ok(call.return_value(transient))
}

#[cfg(test)]
mod tests {
    use super::super::host_provider;
    use super::DictProvider;
    use crate::gleam_stdlib::{GleamStdlibProfile, GleamStdlibRunState};
    use crate::{
        HostFailure, HostModule, HostProvider, HostProviderSet, HostedExecution, ModuleSource,
        PackageSource, compile_typed_host_program, plan_host_program,
    };
    use ecow::EcoString;
    use num_bigint::BigInt;

    const DICT_DECLARATIONS: &str = r#"
pub type Dict(key, value)

type TransientDict(key, value)

@external(erlang, "gleam_stdlib", "identity")
fn to_transient(dict: Dict(key, value)) -> TransientDict(key, value)

@external(erlang, "gleam_stdlib", "identity")
fn from_transient(transient: TransientDict(key, value)) -> Dict(key, value)

@external(erlang, "maps", "size")
fn size(dict: Dict(key, value)) -> Int

@external(erlang, "maps", "is_key")
fn do_has_key(key: key, dict: Dict(key, value)) -> Bool

@external(erlang, "maps", "new")
fn new() -> Dict(key, value)

@external(erlang, "gleam_stdlib", "map_get")
fn get(dict: Dict(key, value), key: key) -> Result(value, Nil)

@external(erlang, "maps", "put")
fn do_insert(
  key: key,
  value: value,
  dict: Dict(key, value),
) -> Dict(key, value)

@external(erlang, "maps", "put")
fn transient_insert(
  key: key,
  value: value,
  dict: TransientDict(key, value),
) -> TransientDict(key, value)

@external(erlang, "maps", "map")
fn do_map_values(
  function: fn(key, value) -> mapped,
  dict: Dict(key, value),
) -> Dict(key, mapped)

@external(erlang, "maps", "remove")
fn transient_delete(
  key: key,
  dict: TransientDict(key, value),
) -> TransientDict(key, value)

@external(erlang, "maps", "fold")
fn do_fold(
  function: fn(key, value, accumulator) -> accumulator,
  initial: accumulator,
  dict: Dict(key, value),
) -> accumulator

@external(erlang, "maps", "update_with")
fn transient_update_with(
  key: key,
  function: fn(value) -> value,
  initial: value,
  dict: TransientDict(key, value),
) -> TransientDict(key, value)
"#;

    fn execution(
        source: &str,
        modules: impl IntoIterator<Item = HostModule<GleamStdlibProfile>>,
    ) -> HostedExecution<GleamStdlibProfile> {
        let source = format!("{DICT_DECLARATIONS}\n{source}");
        let providers = vec![
            host_provider::<GleamStdlibProfile>().expect("official dict provider should register"),
        ];
        let hosts = HostProviderSet::with_providers(modules, providers)
            .expect("test host modules should be unique");
        let typed = compile_typed_host_program(
            "gleam_stdlib",
            "gleam/dict",
            [PackageSource::new(
                "gleam_stdlib",
                Vec::<EcoString>::new(),
                [ModuleSource::new(
                    "gleam/dict",
                    "src/gleam/dict.gleam",
                    source,
                )],
            )],
            hosts,
        )
        .expect("synthetic dict source should compile");
        let plan = plan_host_program(typed).expect("synthetic dict source should plan");
        HostedExecution::try_from_module_plan(plan).expect("synthetic dict execution should seal")
    }

    #[test]
    fn provider_projects_the_complete_run_state() {
        let mut state = GleamStdlibRunState::from_seed([0; 32]);
        let projected =
            <DictProvider<GleamStdlibProfile> as HostProvider<GleamStdlibProfile>>::project(
                &mut state,
            );

        assert!(std::ptr::eq(projected, &state));
    }

    #[test]
    fn executes_every_dict_provider_with_persistent_aliases_and_typed_keys() {
        let float = HostModule::<GleamStdlibProfile>::new_for_profile("gleam_stdlib", "host/float")
            .expect("float module should be valid")
            .with_function("nan", || f64::NAN)
            .expect("NaN function should be valid");
        let source = r#"
import host/float

type Tag {
  Tag(Int)
}

fn increment(value: Int) -> Int {
  value + 1
}

pub fn main() {
  let empty = new()
  assert size(empty) == 0

  let first = do_insert("b", 2, empty)
  let alias = first
  let second = do_insert("a", 1, first)
  let replaced = do_insert("a", 3, second)
  assert size(replaced) == 2
  assert do_has_key("a", replaced)
  assert !do_has_key("missing", replaced)
  assert get(replaced, "a") == Ok(3)
  assert get(replaced, "missing") == Error(Nil)
  assert alias == do_insert("b", 2, new())

  let transient = to_transient(replaced)
  let inserted = transient_insert("c", 4, transient)
  let updated = transient_update_with("a", increment, 0, inserted)
  let updated = transient_update_with("d", increment, 5, updated)
  let unchanged = transient_delete("missing", updated)
  let final = from_transient(transient_delete("b", unchanged))

  let transient_equal = to_transient(do_insert("same", 1, new()))
  assert transient_equal == to_transient(do_insert("same", 1, new()))
  assert transient_equal != to_transient(new())

  assert final
    == do_insert("a", 4, do_insert("c", 4, do_insert("d", 5, new())))
  assert do_map_values(fn(_, value) { value * 2 }, final)
    == do_insert("a", 8, do_insert("c", 8, do_insert("d", 10, new())))
  assert do_fold(fn(_, value, total) { total + value }, 0, final) == 13

  assert do_has_key(1, do_insert(1, Nil, new()))
  assert do_has_key(1.5, do_insert(1.5, Nil, new()))
  assert do_has_key("key", do_insert("key", Nil, new()))
  assert do_has_key(<<1, 2>>, do_insert(<<1, 2>>, Nil, new()))
  let assert <<codepoint:utf8_codepoint>> = <<65>>
  assert do_has_key(codepoint, do_insert(codepoint, Nil, new()))
  assert do_has_key(True, do_insert(True, Nil, new()))
  assert do_has_key(Nil, do_insert(Nil, Nil, new()))
  assert do_has_key(#(1, True), do_insert(#(1, True), Nil, new()))
  assert do_has_key([1, 2], do_insert([1, 2], Nil, new()))
  assert do_has_key(Tag(1), do_insert(Tag(1), Nil, new()))
  assert do_has_key(increment, do_insert(increment, Nil, new()))

  let nan = float.nan()
  let nan_keys = do_insert(nan, 2, do_insert(nan, 1, new()))
  assert size(nan_keys) == 2
  assert !do_has_key(nan, nan_keys)
  assert get(nan_keys, nan) == Error(Nil)
  assert size(from_transient(transient_insert(nan, 3, to_transient(nan_keys)))) == 3

  let inner = do_insert(1, "one", do_insert(2, "two", new()))
  let equal_inner = do_insert(2, "two", do_insert(1, "one", new()))
  assert inner == equal_inner
  assert get(do_insert(inner, "nested", new()), equal_inner) == Ok("nested")

  final
}
"#;
        let execution = execution(source, [float]);
        let actual = execution
            .run_main(
                &mut GleamStdlibRunState::from_seed([0; 32]),
                &mut Vec::new(),
            )
            .expect("dict operations should run");

        assert_eq!(
            actual.inspect().to_string(),
            r#"dict.from_list([#("a", 4), #("c", 4), #("d", 5)])"#,
        );
    }

    #[test]
    fn preserves_nested_host_failure_identity_during_dict_callbacks() {
        let failure =
            HostModule::<GleamStdlibProfile>::new_for_profile("gleam_stdlib", "host/failure")
                .expect("failure module should be valid")
                .with_fallible_function(
                    "reject",
                    |_: EcoString, _: BigInt| -> Result<BigInt, HostFailure> {
                        Err(HostFailure::new("value is unavailable"))
                    },
                )
                .expect("failure function should be valid");
        let source = r#"
import host/failure

pub fn main() {
  let values = do_insert("a", 1, do_insert("b", 2, new()))
  do_map_values(failure.reject, values)
}
"#;
        let execution = execution(source, [failure]);
        let error = execution
            .run_main(
                &mut GleamStdlibRunState::from_seed([0; 32]),
                &mut Vec::new(),
            )
            .expect_err("nested host callback should fail");
        assert_eq!(
            error.to_string(),
            "host function gleam_stdlib::host/failure.reject failed: value is unavailable",
        );
    }

    #[test]
    fn preserves_nested_host_failure_identity_during_dict_folds() {
        let failure =
            HostModule::<GleamStdlibProfile>::new_for_profile("gleam_stdlib", "host/failure")
                .expect("failure module should be valid")
                .with_fallible_function(
                    "reject",
                    |_: EcoString, _: BigInt, _: BigInt| -> Result<BigInt, HostFailure> {
                        Err(HostFailure::new("fold is unavailable"))
                    },
                )
                .expect("failure function should be valid");
        let source = r#"
import host/failure

pub fn main() {
  do_fold(failure.reject, 0, do_insert("a", 1, new()))
}
"#;
        let execution = execution(source, [failure]);
        let error = execution
            .run_main(
                &mut GleamStdlibRunState::from_seed([0; 32]),
                &mut Vec::new(),
            )
            .expect_err("nested fold callback should fail");

        assert_eq!(
            error.to_string(),
            "host function gleam_stdlib::host/failure.reject failed: fold is unavailable",
        );
    }

    #[test]
    fn preserves_nested_host_failure_identity_during_dict_updates() {
        let failure =
            HostModule::<GleamStdlibProfile>::new_for_profile("gleam_stdlib", "host/failure")
                .expect("failure module should be valid")
                .with_fallible_function("reject", |_: BigInt| -> Result<BigInt, HostFailure> {
                    Err(HostFailure::new("update is unavailable"))
                })
                .expect("failure function should be valid");
        let source = r#"
import host/failure

pub fn main() {
  let transient = to_transient(do_insert("a", 1, new()))
  let _ = transient_update_with("a", failure.reject, 0, transient)
  Nil
}
"#;
        let execution = execution(source, [failure]);
        let error = execution
            .run_main(
                &mut GleamStdlibRunState::from_seed([0; 32]),
                &mut Vec::new(),
            )
            .expect_err("nested update callback should fail");

        assert_eq!(
            error.to_string(),
            "host function gleam_stdlib::host/failure.reject failed: update is unavailable",
        );
    }

    #[test]
    fn preserves_source_panic_identity_during_dict_callbacks() {
        let source = r#"
fn reject(_: String, value: Int) -> Int {
  let assert True = False
  value
}

pub fn main() {
  do_map_values(reject, do_insert("a", 1, new()))
}
"#;
        let execution = execution(source, Vec::<HostModule<GleamStdlibProfile>>::new());
        let error = execution
            .run_main(
                &mut GleamStdlibRunState::from_seed([0; 32]),
                &mut Vec::new(),
            )
            .expect_err("nested source callback should panic");
        assert_eq!(
            error.to_string(),
            "let_assert: Pattern match failed, no pattern matched the value.",
        );
    }

    #[test]
    fn explains_only_first_use_dict_host_targets() {
        let source = r#"
pub fn main() {
  size(new())
}
"#;
        let execution = execution(source, Vec::<HostModule<GleamStdlibProfile>>::new());

        assert_eq!(
            execution.explain().to_string().trim(),
            r#"
module gleam/dict
main int#0

function int#0
  entry b0 params=[] captures=[]
  block b0 params=[]
    %external#0:shape#0(external_type#0) = external.call external#0 args=[]
    tail int#1 args=[%external#0]

function int#1
  host gleam_stdlib::gleam/dict.size signature=fn(external_type#0) -> Int

function external#0
  host gleam_stdlib::gleam/dict.new signature=fn() -> external_type#0
"#
            .trim(),
        );
    }
}
