use super::storage::{DictEntry, DictPayload, DictStorage};
use super::{DictOf, DictSchema};
use crate::dynamic::Dynamic;
use crate::{
    Component, GleamStdlibHostProfile, GleamStdlibRunState, HostComponentProfile, HostConstruction,
    HostExternal, HostProvider, HostType, HostTypeIndex0, HostTypeIndexNext, HostTypeList,
    HostTypeListEnd,
};
use geam_core::__macro_support::retain_constructed_argument;
use geam_core::host::HostCall;
use geam_core::provider::{Call, Callback, Value};
use num_bigint::BigInt;
use std::collections::HashMap;

#[geam_macros::module(
    path = "gleam/dict",
    crate_path = geam_core,
    profile = crate::GleamStdlibHostProfile,
    component = crate::Component<Profile::Io>,
    stores = dict,
)]
pub(super) mod provider {
    use super::{
        BigInt, Call, Callback, DictEntry, DictPayload, DictStorage, GleamStdlibRunState, Value,
    };
    use geam_core::provider::HostResult;

    #[geam_macros::external(
        name = "Dict",
        parameters = [Key, Item],
        input = DictInput,
        payload = DictPayload,
        manual,
    )]
    pub struct DictValue<Key, Item>;

    #[geam_macros::external(
        name = "TransientDict",
        parameters = [Key, Item],
        input = TransientDictInput,
        payload = DictPayload,
        manual,
    )]
    pub(super) struct TransientDictValue<Key, Item>;

    #[geam_macros::function]
    fn to_transient<Key, Item>(dict: DictInput<Key, Item>) -> TransientDictValue<Key, Item> {
        TransientDictValue::from_payload(DictPayload {
            storage: dict.payload().storage.clone(),
        })
    }

    #[geam_macros::function]
    fn from_transient<Key, Item>(transient: TransientDictInput<Key, Item>) -> DictValue<Key, Item> {
        DictValue::from_payload(DictPayload {
            storage: transient.payload().storage.clone(),
        })
    }

    #[geam_macros::function]
    fn size<Key, Item>(dict: DictInput<Key, Item>) -> BigInt {
        dict.payload().storage.len.into()
    }

    #[geam_macros::function(profile = Profile)]
    fn do_has_key<Key, Item>(
        #[geam_macros::call] call: &mut Call<GleamStdlibRunState<Profile::Io>>,
        key: Value<Key>,
        dict: DictInput<Key, Item>,
    ) -> bool {
        let key_hash = call.native_source_hash(&key);
        dict.payload()
            .storage
            .matching_index(key_hash, &mut |index| {
                let candidate =
                    call.restore(dict.stored_key(|payload| {
                        payload.storage.buckets[&key_hash][index].key.as_ref()
                    }));
                call.equal(&candidate, &key)
            })
            .is_some()
    }

    #[geam_macros::function]
    fn new<Key, Item>() -> DictValue<Key, Item> {
        DictValue::from_payload(DictPayload {
            storage: DictStorage::default(),
        })
    }

    #[geam_macros::function(profile = Profile)]
    fn get<Item, Key>(
        #[geam_macros::call] call: &mut Call<GleamStdlibRunState<Profile::Io>>,
        dict: DictInput<Key, Item>,
        key: Value<Key>,
    ) -> Result<Value<Item>, ()> {
        let key_hash = call.native_source_hash(&key);
        let Some(index) =
            dict.payload()
                .storage
                .matching_index(key_hash, &mut |index| {
                    let candidate = call.restore(dict.stored_key(|payload| {
                        payload.storage.buckets[&key_hash][index].key.as_ref()
                    }));
                    call.equal(&candidate, &key)
                })
        else {
            return Err(());
        };
        Ok(call.restore(
            dict.stored_item(|payload| payload.storage.buckets[&key_hash][index].value.as_ref()),
        ))
    }

    #[geam_macros::function(profile = Profile)]
    fn do_insert<Key, Item>(
        #[geam_macros::call] call: &mut Call<GleamStdlibRunState<Profile::Io>>,
        key: Value<Key>,
        value: Value<Item>,
        dict: DictInput<Key, Item>,
    ) -> DictValue<Key, Item> {
        let key_hash = call.native_source_hash(&key);
        let index =
            dict.payload()
                .storage
                .matching_index(key_hash, &mut |index| {
                    let candidate = call.restore(dict.stored_key(|payload| {
                        payload.storage.buckets[&key_hash][index].key.as_ref()
                    }));
                    call.equal(&candidate, &key)
                });
        let storage = dict.payload().storage.clone();
        let entry = DictEntry::new(
            key_hash,
            call.store(key).into_retained(),
            call.store(value).into_retained(),
        );
        DictValue::from_payload(DictPayload {
            storage: storage.with_entry(key_hash, index, entry),
        })
    }

    #[geam_macros::function(profile = Profile)]
    fn transient_insert<Key, Item>(
        #[geam_macros::call] call: &mut Call<GleamStdlibRunState<Profile::Io>>,
        key: Value<Key>,
        value: Value<Item>,
        transient: TransientDictInput<Key, Item>,
    ) -> TransientDictValue<Key, Item> {
        let key_hash = call.native_source_hash(&key);
        let index = transient
            .payload()
            .storage
            .matching_index(key_hash, &mut |index| {
                let candidate =
                    call.restore(transient.stored_key(|payload| {
                        payload.storage.buckets[&key_hash][index].key.as_ref()
                    }));
                call.equal(&candidate, &key)
            });
        let storage = transient.payload().storage.clone();
        let entry = DictEntry::new(
            key_hash,
            call.store(key).into_retained(),
            call.store(value).into_retained(),
        );
        TransientDictValue::from_payload(DictPayload {
            storage: storage.with_entry(key_hash, index, entry),
        })
    }

    #[geam_macros::function(profile = Profile, await)]
    async fn do_map_values<Key, Mapped, Item>(
        #[geam_macros::call] call: &mut Call<GleamStdlibRunState<Profile::Io>>,
        function: Callback<fn(Value<Key>, Value<Item>) -> Value<Mapped>>,
        dict: DictInput<Key, Item>,
    ) -> HostResult<DictValue<Key, Mapped>> {
        let coordinates = dict.with_payload(DictPayload::coordinates);
        let mut buckets = im::HashMap::new();
        for (key_hash, index) in coordinates {
            let key = call.restore(
                dict.stored_key(|payload| payload.storage.buckets[&key_hash][index].key.as_ref()),
            );
            let value =
                call.restore(dict.stored_item(|payload| {
                    payload.storage.buckets[&key_hash][index].value.as_ref()
                }));
            let value = call.invoke(&function, (key, value)).await?;
            let value = call.store(value).into_retained();
            let entry = dict.with_payload(|payload| {
                payload.storage.buckets[&key_hash][index].with_value(value)
            });
            buckets
                .entry(key_hash)
                .or_insert_with(imbl::Vector::new)
                .push_back(entry);
        }
        Ok(DictValue::from_payload(DictPayload {
            storage: DictStorage {
                buckets,
                len: dict.with_payload(|payload| payload.storage.len),
            },
        }))
    }

    #[geam_macros::function(profile = Profile)]
    fn transient_delete<Key, Item>(
        #[geam_macros::call] call: &mut Call<GleamStdlibRunState<Profile::Io>>,
        key: Value<Key>,
        transient: TransientDictInput<Key, Item>,
    ) -> TransientDictValue<Key, Item> {
        let key_hash = call.native_source_hash(&key);
        let Some(index) = transient
            .payload()
            .storage
            .matching_index(key_hash, &mut |index| {
                let candidate =
                    call.restore(transient.stored_key(|payload| {
                        payload.storage.buckets[&key_hash][index].key.as_ref()
                    }));
                call.equal(&candidate, &key)
            })
        else {
            return transient.into_value();
        };
        TransientDictValue::from_payload(DictPayload {
            storage: transient.payload().storage.without_entry(key_hash, index),
        })
    }

    #[geam_macros::function(profile = Profile, await)]
    async fn do_fold<Accumulator, Key, Item>(
        #[geam_macros::call] call: &mut Call<GleamStdlibRunState<Profile::Io>>,
        function: Callback<fn(Value<Key>, Value<Item>, Value<Accumulator>) -> Value<Accumulator>>,
        mut accumulator: Value<Accumulator>,
        dict: DictInput<Key, Item>,
    ) -> HostResult<Value<Accumulator>> {
        for (key_hash, index) in dict.with_payload(DictPayload::coordinates) {
            let key = call.restore(
                dict.stored_key(|payload| payload.storage.buckets[&key_hash][index].key.as_ref()),
            );
            let value =
                call.restore(dict.stored_item(|payload| {
                    payload.storage.buckets[&key_hash][index].value.as_ref()
                }));
            accumulator = call.invoke(&function, (key, value, accumulator)).await?;
        }
        Ok(accumulator)
    }

    #[geam_macros::function(profile = Profile, await)]
    async fn transient_update_with<Key, Item>(
        #[geam_macros::call] call: &mut Call<GleamStdlibRunState<Profile::Io>>,
        key: Value<Key>,
        function: Callback<fn(Value<Item>) -> Value<Item>>,
        initial: Value<Item>,
        transient: TransientDictInput<Key, Item>,
    ) -> HostResult<TransientDictValue<Key, Item>> {
        let (mut key, key_hash) = call
            .with_call(move |call| {
                let hash = call.native_source_hash(&key);
                (key, hash)
            })
            .await?;
        let candidates = transient.with_payload(|payload| {
            payload
                .storage
                .buckets
                .get(&key_hash)
                .map_or(0, imbl::Vector::len)
        });
        let mut index = None;
        for position in 0..candidates {
            let candidate =
                call.restore(transient.stored_key(|payload| {
                    payload.storage.buckets[&key_hash][position].key.as_ref()
                }));
            let (equal, returned_key) = call
                .with_call(move |call| (call.equal(&candidate, &key), key))
                .await?;
            key = returned_key;
            if equal {
                index = Some(position);
                break;
            }
        }
        let value = match index {
            Some(index) => {
                let value = call.restore(transient.stored_item(|payload| {
                    payload.storage.buckets[&key_hash][index].value.as_ref()
                }));
                call.invoke(&function, (value,)).await?
            }
            None => initial,
        };
        let storage = transient.with_payload(|payload| payload.storage.clone());
        let entry = DictEntry::new(
            key_hash,
            call.store(key).into_retained(),
            call.store(value).into_retained(),
        );
        Ok(TransientDictValue::from_payload(DictPayload {
            storage: storage.with_entry(key_hash, index, entry),
        }))
    }
}

type KeyIndex = HostTypeIndex0;
type ItemIndex = HostTypeIndexNext<KeyIndex>;

pub(super) fn host_provider<Profile>()
-> Result<crate::HostProviderModule<Profile>, crate::HostRegistrationError>
where
    Profile: crate::GleamStdlibProviderProfile,
{
    provider::__geam_module::<Profile>()
}

/// Constructs the original Gleam Dict from typed key/item pairs.
///
/// Register an exact [`DictOf<Key, Item>`] construction for the calling native
/// function and pass its token here. Any generic key/item parameters must be
/// bound by that function's signature. Intermediate values, such as Lists, use
/// their own construction tokens; existing typed values need no extra tokens.
/// The function's return type need not itself be a Dict.
///
/// Keys use Gleam source equality and hashing. The last pair for an equal key
/// wins, matching `gleam/dict.from_list`; different keys with the same hash stay
/// distinct. A Dict has no iteration-order guarantee.
///
/// The iterator is consumed once. Unique entries are retained by the immutable
/// payload, so input containers may be dropped and later persistent updates do
/// not mutate this value. The returned handle is call-scoped; returning or
/// retaining it follows the ordinary typed host lifetime rules, including any
/// execution-scoped values nested inside it.
pub fn dict_from_entries<'call, Profile, Provider, Return, Key, Item>(
    call: &mut HostCall<'call, Profile, Provider, Return>,
    construction: HostConstruction<'call, DictOf<Key, Item>>,
    entries: impl IntoIterator<Item = (Key::Value<'call>, Item::Value<'call>)>,
) -> HostExternal<'call, DictOf<Key, Item>>
where
    Profile: GleamStdlibHostProfile + HostComponentProfile<Component<Profile::Io>>,
    Provider: HostProvider<Profile>,
    Return: HostType,
    Key: HostType,
    Item: HostType,
{
    let mut buckets = EntryBuckets::<Key::Value<'call>, Item::Value<'call>>::new();
    for (key, item) in entries {
        let key_hash = call.native_source_hash::<Key>(key.clone());
        let bucket = buckets.entry(key_hash).or_default();
        if let Some(stored) = bucket
            .iter_mut()
            .find(|(stored, _)| call.equal::<Key>(stored.clone(), key.clone()))
        {
            *stored = (key, item);
        } else {
            bucket.push((key, item));
        }
    }
    finish_dict(call, construction, buckets)
}

pub fn create_dynamic_dict<'call, Profile, Provider, Return>(
    call: &mut HostCall<'call, Profile, Provider, Return>,
    construction: HostConstruction<'call, DictOf<Dynamic, Dynamic>>,
    entries: impl IntoIterator<Item = (HostExternal<'call, Dynamic>, HostExternal<'call, Dynamic>)>,
) -> HostExternal<'call, DictOf<Dynamic, Dynamic>>
where
    Profile: crate::GleamStdlibProviderProfile,
    Profile::RunState: Send,
    Provider: HostProvider<Profile>,
    Return: HostType,
{
    create_dynamic_dict_with(call, construction, entries, |_, entry| entry)
}

pub(super) fn create_dynamic_dict_with<'call, Profile, Provider, Return, Entry>(
    call: &mut HostCall<'call, Profile, Provider, Return>,
    construction: HostConstruction<'call, DictOf<Dynamic, Dynamic>>,
    entries: impl IntoIterator<Item = Entry>,
    mut convert: impl FnMut(
        &mut HostCall<'call, Profile, Provider, Return>,
        Entry,
    ) -> (HostExternal<'call, Dynamic>, HostExternal<'call, Dynamic>),
) -> HostExternal<'call, DictOf<Dynamic, Dynamic>>
where
    Profile: crate::GleamStdlibProviderProfile,
    Profile::RunState: Send,
    Provider: HostProvider<Profile>,
    Return: HostType,
{
    let mut buckets = EntryBuckets::new();
    for entry in entries {
        let (key, value) = convert(call, entry);
        let key_hash = call.native_source_hash::<Dynamic>(key);
        insert_first(&mut buckets, key_hash, key, value, |stored, candidate| {
            call.equal::<Dynamic>(*stored, *candidate)
        });
    }
    finish_dict(call, construction, buckets)
}

type EntryBuckets<Key, Item> = HashMap<u64, Vec<(Key, Item)>>;

fn finish_dict<'call, Profile, Provider, Return, Key, Item>(
    call: &mut HostCall<'call, Profile, Provider, Return>,
    construction: HostConstruction<'call, DictOf<Key, Item>>,
    buckets: EntryBuckets<Key::Value<'call>, Item::Value<'call>>,
) -> HostExternal<'call, DictOf<Key, Item>>
where
    Profile: crate::GleamStdlibProviderProfile,
    Provider: HostProvider<Profile>,
    Return: HostType,
    Key: HostType,
    Item: HostType,
{
    let len = buckets.values().map(Vec::len).sum();
    let buckets = buckets
        .into_iter()
        .map(|(key_hash, entries)| {
            let entries = entries
                .into_iter()
                .map(|(key, value)| {
                    DictEntry::new(
                        key_hash,
                        retain_constructed_argument::<_, _, _, _, _, DictPayload, KeyIndex>(
                            call,
                            &construction,
                            key,
                        ),
                        retain_constructed_argument::<_, _, _, _, _, DictPayload, ItemIndex>(
                            call,
                            &construction,
                            value,
                        ),
                    )
                })
                .collect();
            (key_hash, entries)
        })
        .collect();
    call.construct_external_with_binding::<provider::__GeamProvider, DictSchema, HostTypeList<Key, HostTypeList<Item, HostTypeListEnd>>>(
        construction, DictPayload { storage: DictStorage { buckets, len } },
    )
}

pub(super) fn insert_first<Key, Value>(
    buckets: &mut EntryBuckets<Key, Value>,
    key_hash: u64,
    key: Key,
    value: Value,
    mut equal: impl FnMut(&Key, &Key) -> bool,
) {
    let bucket = buckets.entry(key_hash).or_default();
    if !bucket.iter().any(|(stored, _)| equal(stored, &key)) {
        bucket.push((key, value));
    }
}

#[cfg(test)]
mod tests {
    mod transfer {
        use super::DICT_DECLARATIONS;
        use crate::dict::function::provider::__GeamProvider as DictProvider;
        use crate::dict::storage::DictEntry;
        use crate::dict::{DictOf, DictSchema};
        use crate::{
            Component, GleamStdlibHostProfile, GleamStdlibRunState, GleamStdlibStores, IoOutput,
        };
        use geam_core::embedding::{FunctionDeclaration, HostedModuleBuilder};
        use geam_core::frontend::compile_typed_host_program;
        use geam_core::host::{HostCall, HostComponentProfile, HostExternalBinding};
        use geam_core::{
            HostCallCompletion, HostConstructions, HostExternal, HostList, HostListType,
            HostProfile, HostProvider, HostProviderSet, HostTypeIndex0, HostTypeList,
            HostTypeListEnd, ModuleSource, PackageSource, StringValue,
        };
        use num_bigint::BigInt;
        use std::pin::pin;
        use std::sync::{Arc, Weak};
        use std::task::Poll;

        struct Profile;
        #[derive(Default)]
        struct Stores {
            stdlib: GleamStdlibStores,
        }
        struct State {
            stdlib: GleamStdlibRunState,
            entries: Vec<Weak<DictEntry>>,
        }
        struct Observer;
        #[derive(Default)]
        struct Echo(Vec<String>);
        impl geam_core::EchoSink for Echo {
            fn emit(&mut self, output: geam_core::EchoOutput) {
                self.0.push(output.value().inspect().to_string());
            }
        }
        type Dict = DictOf<StringValue, HostListType<BigInt>>;
        type Constructions = HostTypeList<Dict, HostTypeListEnd>;

        impl HostProfile for Profile {
            type RunState = State;
            type ExternalStores = Stores;
            type ExecutionState = ();
        }
        impl GleamStdlibHostProfile for Profile {
            type Io = Vec<IoOutput>;
        }
        impl HostComponentProfile<Component> for Profile {
            fn component_stores(stores: &Stores) -> &GleamStdlibStores {
                &stores.stdlib
            }
            fn component_state(state: &mut State) -> &mut GleamStdlibRunState {
                &mut state.stdlib
            }
        }
        impl HostProvider<Profile> for Observer {
            type State = Vec<Weak<DictEntry>>;
            fn project(state: &mut State) -> &mut Self::State {
                &mut state.entries
            }
        }
        impl HostExternalBinding<Profile, DictSchema> for Observer {
            type Storage = <DictProvider as HostExternalBinding<Profile, DictSchema>>::Storage;
        }

        fn make<'call>(
            mut call: HostCall<'call, Profile, Observer, Dict>,
            constructions: HostConstructions<'call, Constructions>,
            first: HostList<'call, BigInt>,
            second: HostList<'call, BigInt>,
        ) -> Result<HostCallCompletion<'call, Dict>, geam_core::HostCallError> {
            let entries = vec![
                (StringValue::from("first"), first),
                (StringValue::from("second"), second),
            ];
            let dict = crate::service::dict_from_entries(
                &mut call,
                constructions.at::<HostTypeIndex0>(),
                entries,
            );
            Ok(call.return_value(dict))
        }

        fn observe<'call>(
            mut call: HostCall<'call, Profile, Observer, ()>,
            before: HostExternal<'call, Dict>,
            after: HostExternal<'call, Dict>,
            mapped: bool,
        ) -> Result<HostCallCompletion<'call, ()>, geam_core::HostCallError> {
            let before = call.external_payload(before);
            let after = call.external_payload(after);
            assert_eq!(before.storage.len, 2);
            assert_eq!(after.storage.len, 2);
            let mut shared_entries = 0;
            let mut changed_values = 0;
            for (hash, bucket) in &before.storage.buckets {
                for (index, entry) in bucket.iter().enumerate() {
                    let updated = &after.storage.buckets[hash][index];
                    if Arc::ptr_eq(entry, updated) {
                        shared_entries += 1;
                        assert!(Arc::ptr_eq(&entry.key, &updated.key));
                        assert!(Arc::ptr_eq(&entry.value, &updated.value));
                    } else {
                        changed_values += 1;
                        assert_eq!(Arc::ptr_eq(&entry.key, &updated.key), mapped);
                        assert!(!Arc::ptr_eq(&entry.value, &updated.value));
                    }
                    call.state()
                        .extend([Arc::downgrade(entry), Arc::downgrade(updated)]);
                }
            }
            assert_eq!(
                (shared_entries, changed_values),
                if mapped { (0, 2) } else { (1, 1) }
            );
            Ok(call.return_value(()))
        }

        #[test]
        fn persistent_transfer_updates_share_unchanged_entries_and_release_the_graph() {
            let source = format!(
                "{DICT_DECLARATIONS}\n{}",
                r#"
@external(erlang, "observer", "compare")
fn observe(before: Dict(String, List(Int)), after: Dict(String, List(Int)), mapped: Bool) -> Nil

@external(erlang, "observer", "make")
fn make(first: List(Int), second: List(Int)) -> Dict(String, List(Int))

pub fn run() -> Nil {
  let original = make([1, 2], [3])
  let updated = do_insert("first", [4, 5], original)
  echo size(updated)
  observe(original, updated, False)
  observe(original, do_map_values(fn(_, value) { value }, original), True)
}
"#
            );
            let dict = super::super::host_provider::<Profile>()
                .expect("dict registration")
                .with_scoped_function_and_constructions::<Observer, (HostListType<BigInt>, HostListType<BigInt>), Dict, Constructions, _>("make", make)
                .expect("construction registration")
                .with_scoped_function::<Observer, (Dict, Dict, bool), (), _>("observe", observe)
                .expect("observation registration");
            let program = compile_typed_host_program(
                "gleam_stdlib",
                "gleam/dict",
                [PackageSource::new(
                    "gleam_stdlib",
                    Vec::<String>::new(),
                    [ModuleSource::new(
                        "gleam/dict",
                        "src/gleam/dict.gleam",
                        source,
                    )],
                )],
                HostProviderSet::from_providers([dict]).expect("provider set"),
            )
            .expect("dict source");
            let (bindings, entry) = HostedModuleBuilder::new(program)
                .expect("plan")
                .function(FunctionDeclaration::<(), ()>::new("run"))
                .expect("entry");
            let mut module = bindings.seal().expect("seal");
            let mut state = State {
                stdlib: GleamStdlibRunState::from_seed([0; 32]),
                entries: Vec::new(),
            };
            assert!(std::ptr::eq(
                <Profile as HostComponentProfile<Component>>::component_state(&mut state),
                &state.stdlib,
            ));
            let mut echo = Echo::default();
            {
                let execution_host = crate::execution_fixture::TestHost::default();
                let mut task = pin!(module.with_execution(
                    &execution_host,
                    &mut state,
                    &mut echo,
                    async |scope| {
                        scope.call(&entry, ()).await.expect("direct update");
                    }
                ));
                assert_eq!(
                    execution_host
                        .poll(task.as_mut())
                        .map(|result| result.expect("controlled execution")),
                    Poll::Ready(())
                );
            }
            assert_eq!(state.entries.len(), 8);
            std::thread::spawn(move || drop(module))
                .join()
                .expect("owner drop on another worker");
            assert!(state.entries.iter().all(|entry| entry.upgrade().is_none()));
            assert_eq!(echo.0, ["2"]);
        }
    }

    use super::super::host_provider;
    use super::{dict_from_entries, insert_first, provider::__GeamProvider as DictProvider};
    use crate::dict::DictOf;
    use crate::{
        Component as GleamStdlibComponent, GleamStdlibHostProfile, GleamStdlibProfile,
        GleamStdlibRunState, GleamStdlibStores, IoOutput,
    };
    use crate::{
        HostCall, HostCallCompletion, HostCallError, HostComponentProfile, HostExternalBinding,
        HostExternalEquality, HostExternalHashing, HostExternalInspection, HostExternalSchema,
        HostExternalStorage, HostExternalStore, HostExternalType, HostFailure, HostModule,
        HostProfile, HostProvider, HostProviderModule, HostProviderSet, HostedExecution,
        ModuleSource, PackageSource, compile_typed_host_program, plan_host_program,
    };
    use ecow::EcoString;
    use geam_core::StringValue;
    use geam_core::host::{
        HostConstructions, HostList, HostListType, HostTupleType, HostType, HostTypeIndex0,
        HostTypeList, HostTypeListEnd, HostTypeParameter,
    };
    use num_bigint::BigInt;

    #[test]
    fn batch_staging_resolves_hash_collisions_and_preserves_the_first_duplicate() {
        fn equal(left: &&str, right: &&str) -> bool {
            left == right
        }

        let mut buckets = std::collections::HashMap::new();

        insert_first(&mut buckets, 7, "first", 1, equal);
        insert_first(&mut buckets, 7, "second", 2, equal);
        insert_first(&mut buckets, 7, "first", 3, equal);

        assert_eq!(buckets[&7], [("first", 1), ("second", 2)]);
    }

    const CONSTRUCTION_DECLARATIONS: &str = r#"
@external(erlang, "dict_construction", "from_entries")
fn from_entries(entries: List(#(key, item))) -> Dict(key, item)

@external(erlang, "dict_construction", "text_entries")
fn text_entries(entries: List(#(String, String))) -> Dict(String, String)

@external(erlang, "dict_construction", "integer_entries")
fn integer_entries(entries: List(#(Int, List(String)))) -> Dict(Int, List(String))

@external(erlang, "dict_construction", "compare")
fn compare(left: Dict(key, item), right: Dict(key, item)) -> Nil

"#;

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

    struct CollisionProfile;

    #[derive(Default)]
    struct CollisionStores {
        stdlib: GleamStdlibStores,
        keys: HostExternalStore<BigInt>,
    }

    struct CollisionRunState {
        stdlib: GleamStdlibRunState,
        keys: (),
    }

    struct CollisionProvider;
    struct CollisionSchema;
    struct CollisionStorage;

    type CollisionKey = HostExternalType<CollisionSchema>;

    impl HostProfile for CollisionProfile {
        type RunState = CollisionRunState;
        type ExternalStores = CollisionStores;
        type ExecutionState = ();
    }

    impl HostComponentProfile<GleamStdlibComponent> for CollisionProfile {
        fn component_stores(stores: &Self::ExternalStores) -> &GleamStdlibStores {
            &stores.stdlib
        }

        fn component_state(state: &mut Self::RunState) -> &mut GleamStdlibRunState {
            &mut state.stdlib
        }
    }

    impl GleamStdlibHostProfile for CollisionProfile {
        type Io = Vec<IoOutput>;
    }

    impl HostProvider<CollisionProfile> for CollisionProvider {
        type State = ();

        fn project(state: &mut CollisionRunState) -> &mut Self::State {
            &mut state.keys
        }
    }

    impl HostExternalSchema for CollisionSchema {
        const PACKAGE: &'static str = "gleam_stdlib";
        const MODULE: &'static str = "host/collision";
        const NAME: &'static str = "CollisionKey";
        const PARAMETER_COUNT: usize = 0;
    }

    impl HostExternalStorage<CollisionProfile, CollisionSchema> for CollisionStorage {
        type Payload = BigInt;

        fn store(stores: &CollisionStores) -> &HostExternalStore<Self::Payload> {
            &stores.keys
        }

        fn source_equal(
            _context: &HostExternalEquality<'_>,
            left: &Self::Payload,
            right: &Self::Payload,
        ) -> bool {
            left == right
        }

        fn source_hash(_context: &HostExternalHashing<'_>, _value: &Self::Payload) -> u64 {
            7
        }

        fn inspect(_context: &HostExternalInspection<'_>, value: &Self::Payload) -> EcoString {
            format!("CollisionKey({value})").into()
        }
    }

    impl HostExternalBinding<CollisionProfile, CollisionSchema> for CollisionProvider {
        type Storage = CollisionStorage;
    }

    fn collision_key<'call>(
        mut call: HostCall<'call, CollisionProfile, CollisionProvider, CollisionKey>,
        value: BigInt,
    ) -> Result<HostCallCompletion<'call, CollisionKey>, HostCallError> {
        let key = call.create_external(value);
        Ok(call.return_value(key))
    }

    fn collision_provider() -> HostProviderModule<CollisionProfile> {
        HostProviderModule::new("gleam_stdlib", "host/collision")
            .and_then(HostProviderModule::with_external_type::<CollisionProvider, CollisionSchema>)
            .and_then(|provider| {
                provider.with_scoped_function::<CollisionProvider, (BigInt,), CollisionKey, _>(
                    "new",
                    collision_key,
                )
            })
            .expect("collision-key provider should register")
    }

    fn collision_execution(
        source: &str,
        dict: HostProviderModule<CollisionProfile>,
    ) -> HostedExecution<CollisionProfile> {
        const COLLISION_SOURCE: &str = r#"
pub type CollisionKey

@external(erlang, "host", "new")
pub fn new(value: Int) -> CollisionKey
"#;

        let source = format!("{DICT_DECLARATIONS}\n{source}");
        let providers = [dict, collision_provider()];
        let hosts =
            HostProviderSet::with_providers(Vec::<HostModule<CollisionProfile>>::new(), providers)
                .expect("collision test providers should be unique");
        let typed = compile_typed_host_program(
            "gleam_stdlib",
            "gleam/dict",
            [PackageSource::new(
                "gleam_stdlib",
                Vec::<EcoString>::new(),
                [
                    ModuleSource::new(
                        "host/collision",
                        "src/host/collision.gleam",
                        COLLISION_SOURCE,
                    ),
                    ModuleSource::new("gleam/dict", "src/gleam/dict.gleam", source),
                ],
            )],
            hosts,
        )
        .expect("collision-backed dict source should compile");
        let plan = plan_host_program(typed).expect("collision-backed dict source should plan");
        HostedExecution::try_from_module_plan(plan)
            .expect("collision-backed dict execution should seal")
    }

    fn execution(
        source: &str,
        modules: impl IntoIterator<Item = HostModule<GleamStdlibProfile>>,
        dict: HostProviderModule<GleamStdlibProfile>,
    ) -> HostedExecution<GleamStdlibProfile> {
        let source = format!("{DICT_DECLARATIONS}\n{source}");
        let providers = vec![dict];
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

    type Pair<Key, Item> = HostTupleType<HostTypeList<Key, HostTypeList<Item, HostTypeListEnd>>>;
    type DictConstructions<Key, Item> = HostTypeList<DictOf<Key, Item>, HostTypeListEnd>;
    type GenericDict = DictOf<HostTypeParameter<0>, HostTypeParameter<1>>;

    fn construction_provider<Profile: crate::GleamStdlibProviderProfile>()
    -> HostProviderModule<Profile> {
        host_provider::<Profile>()
            .unwrap()
            .with_scoped_function_and_constructions::<
                DictProvider,
                (HostListType<Pair<HostTypeParameter<0>, HostTypeParameter<1>>>,),
                GenericDict,
                DictConstructions<HostTypeParameter<0>, HostTypeParameter<1>>,
                _,
            >("from_entries", construct_entries::<Profile, HostTypeParameter<0>, HostTypeParameter<1>>)
            .unwrap()
            .with_scoped_function_and_constructions::<
                DictProvider,
                (HostListType<Pair<StringValue, StringValue>>,),
                DictOf<StringValue, StringValue>,
                DictConstructions<StringValue, StringValue>,
                _,
            >("text_entries", construct_entries::<Profile, StringValue, StringValue>)
            .unwrap()
            .with_scoped_function_and_constructions::<
                DictProvider,
                (HostListType<Pair<BigInt, HostListType<StringValue>>>,),
                DictOf<BigInt, HostListType<StringValue>>,
                DictConstructions<BigInt, HostListType<StringValue>>,
                _,
            >("integer_entries", construct_entries::<Profile, BigInt, HostListType<StringValue>>)
            .unwrap()
            .with_scoped_function::<DictProvider, (GenericDict, GenericDict), (), _>("compare", compare_dicts::<Profile>)
            .unwrap()
    }

    fn construct_entries<'call, Profile, Key, Item>(
        mut call: HostCall<'call, Profile, DictProvider, DictOf<Key, Item>>,
        constructions: HostConstructions<'call, DictConstructions<Key, Item>>,
        entries: HostList<'call, Pair<Key, Item>>,
    ) -> Result<HostCallCompletion<'call, DictOf<Key, Item>>, HostCallError>
    where
        Profile: crate::GleamStdlibProviderProfile,
        Key: HostType,
        Item: HostType,
    {
        let entries = (0..call.list_len(entries))
            .map(|index| {
                let pair = call.list_item(entries, index).unwrap();
                let (key, (item, ())) = call.tuple_values(pair);
                (key, item)
            })
            .collect::<Vec<_>>();
        let value = dict_from_entries(&mut call, constructions.at::<HostTypeIndex0>(), entries);
        Ok(call.return_value(value))
    }

    fn compare_dicts<'call, Profile: crate::GleamStdlibProviderProfile>(
        call: HostCall<'call, Profile, DictProvider, ()>,
        left: crate::HostExternal<'call, GenericDict>,
        right: crate::HostExternal<'call, GenericDict>,
    ) -> Result<HostCallCompletion<'call, ()>, HostCallError> {
        assert!(call.equal::<GenericDict>(left, right));
        assert!(call.equal::<GenericDict>(right, left));
        assert_eq!(
            call.source_hash::<GenericDict>(left),
            call.source_hash::<GenericDict>(right)
        );
        let left = call.native_value::<GenericDict>(left);
        let right = call.native_value::<GenericDict>(right);
        assert_eq!(left.kind(), right.kind());
        assert!(call.native_equal(&left, &right));
        assert!(call.native_equal(&right, &left));
        assert_eq!(call.native_hash(&left), call.native_hash(&right));
        Ok(call.return_value(()))
    }

    #[test]
    fn typed_entries_preserve_duplicates_generic_values_and_builtin_semantics() {
        let source = r#"
pub fn main() {
  let empty = text_entries([])
  assert size(empty) == 0
  assert get(empty, "absent") == Error(Nil)
  compare(empty, new())
  let values = text_entries([
    #("a", "discarded"), #("a", "second"), #("b", ""),
    #("a", "last"), #("한국어\u{0}🙂", "é"),
  ])
  assert size(values) == 3
  assert get(values, "a") == Ok("last")
  assert get(values, "b") == Ok("")
  assert get(values, "한국어\u{0}🙂") == Ok("é")
  compare(values, do_insert("a", "last", do_insert("b", "", do_insert("한국어\u{0}🙂", "é", new()))))
  let numbers = integer_entries([#(1, ["old"]), #(9999999999999999999999999, []), #(1, ["one", "two"])])
  assert size(numbers) == 2
  assert get(numbers, 1) == Ok(["one", "two"])
  assert get(numbers, 9999999999999999999999999) == Ok([])
  compare(numbers, do_insert(1, ["one", "two"], do_insert(9999999999999999999999999, [], new())))
  let compound = from_entries([#(#([1, 2], "x"), numbers)])
  assert get(compound, #([1, 2], "x")) == Ok(numbers)
  let keyed = from_entries([#(numbers, "nested")])
  let equal_numbers = do_insert(9999999999999999999999999, [], do_insert(1, ["one", "two"], new()))
  assert get(keyed, equal_numbers) == Ok("nested")
  compare(compound, do_insert(#([1, 2], "x"), equal_numbers, new()))
  let changed = do_insert("a", "changed", values)
  let deleted = changed |> to_transient |> transient_delete("b", _) |> from_transient
  assert get(values, "a") == Ok("last")
  assert get(values, "b") == Ok("")
  assert get(deleted, "a") == Ok("changed")
  assert get(deleted, "b") == Error(Nil)
  text_entries([#("only", "old"), #("only", "retained")])
}
"#;
        let mut execution = execution(
            &format!("{CONSTRUCTION_DECLARATIONS}\n{source}"),
            [],
            construction_provider(),
        );
        let mut state = GleamStdlibRunState::from_seed([0; 32]);
        let mut echo = Vec::new();
        let first = crate::execution_fixture::run(&mut execution, &mut state, &mut echo).unwrap();
        let second = crate::execution_fixture::run(&mut execution, &mut state, &mut echo).unwrap();
        drop(execution);
        drop(state);
        for value in [first, second] {
            assert_eq!(
                value.inspect().to_string(),
                r#"dict.from_list([#("only", "retained")])"#
            );
        }
        assert!(echo.is_empty());
    }

    #[test]
    fn typed_entries_retain_the_last_equal_key_as_well_as_its_item() {
        let mut execution = execution(
            &format!(
                "{CONSTRUCTION_DECLARATIONS}\n{}",
                r#"
pub fn main() {
  let negative = from_entries([#(0.0, "first"), #(-0.0, "last")])
  let positive = from_entries([#(-0.0, "first"), #(0.0, "last")])
  assert size(negative) == 1
  assert size(positive) == 1
  assert get(negative, 0.0) == Ok("last")
  assert get(positive, -0.0) == Ok("last")
  #(negative, positive)
}
"#
            ),
            [],
            construction_provider(),
        );
        let value = crate::execution_fixture::run(
            &mut execution,
            &mut GleamStdlibRunState::from_seed([0; 32]),
            &mut Vec::new(),
        )
        .unwrap();
        assert_eq!(
            value.inspect().to_string(),
            r#"#(dict.from_list([#(-0.0, "last")]), dict.from_list([#(0.0, "last")]))"#
        );
    }

    #[test]
    fn typed_entries_resolve_real_source_hash_collisions_and_replace_the_last_pair() {
        let mut execution = collision_execution(
            &format!(
                "{CONSTRUCTION_DECLARATIONS}\n{}",
                r#"
import host/collision

pub fn main() {
  let a = collision.new(1)
  let b = collision.new(2)
  let values = from_entries([#(a, 1), #(b, 2), #(collision.new(1), 3)])
  assert size(values) == 2
  assert get(values, a) == Ok(3)
  assert get(values, b) == Ok(2)
  assert get(values, collision.new(3)) == Error(Nil)
  compare(values, do_insert(a, 3, do_insert(b, 2, new())))
  values
}
"#
            ),
            construction_provider(),
        );
        let mut state = CollisionRunState {
            stdlib: GleamStdlibRunState::from_seed([0; 32]),
            keys: (),
        };
        let value =
            crate::execution_fixture::run(&mut execution, &mut state, &mut Vec::new()).unwrap();
        assert_eq!(
            value.inspect().to_string(),
            "dict.from_list([#(CollisionKey(1), 3), #(CollisionKey(2), 2)])"
        );
    }

    #[test]
    fn provider_projects_the_complete_run_state() {
        let mut state = GleamStdlibRunState::from_seed([0; 32]);
        let projected = <DictProvider as HostProvider<GleamStdlibProfile>>::project(&mut state);

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
  let transient_keys = do_insert(transient_equal, "transient", new())
  assert get(transient_keys, transient_equal) == Ok("transient")
  echo transient_equal as "transient"

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
        let mut execution = execution(source, [float], host_provider().unwrap());
        let mut echoes = Vec::new();
        let actual = crate::execution_fixture::run(
            &mut execution,
            &mut GleamStdlibRunState::from_seed([0; 32]),
            &mut echoes,
        )
        .expect("dict operations should run");

        assert_eq!(
            actual.inspect().to_string(),
            r#"dict.from_list([#("a", 4), #("c", 4), #("d", 5)])"#,
        );
        assert_eq!(echoes.len(), 1);
        assert_eq!(
            echoes[0].value().inspect().to_string(),
            r#"dict.from_list([#("same", 1)])"#,
        );
    }

    #[test]
    fn collision_bucket_deletion_retains_other_entries_through_the_hosted_pipeline() {
        let mut execution = collision_execution(
            r#"
import host/collision

pub fn main() {
  let first = collision.new(1)
  let second = collision.new(2)
  let collided = do_insert(second, 20, do_insert(first, 10, new()))
  assert size(collided) == 2
  let transient = to_transient(collided)
  let remaining = from_transient(transient_delete(first, transient))
  assert remaining == do_insert(second, 20, new())
  remaining
}
"#,
            host_provider().unwrap(),
        );
        let mut state = CollisionRunState {
            stdlib: GleamStdlibRunState::from_seed([0; 32]),
            keys: (),
        };
        let expected_stdlib = &state.stdlib as *const GleamStdlibRunState;
        let projected_stdlib =
            <CollisionProfile as HostComponentProfile<GleamStdlibComponent>>::component_state(
                &mut state,
            ) as *mut GleamStdlibRunState;
        assert_eq!(projected_stdlib.cast_const(), expected_stdlib);
        let expected_keys = &state.keys as *const ();
        let projected_keys =
            <CollisionProvider as HostProvider<CollisionProfile>>::project(&mut state) as *mut ();
        assert_eq!(projected_keys.cast_const(), expected_keys);

        let actual = crate::execution_fixture::run(&mut execution, &mut state, &mut Vec::new())
            .expect("colliding dict key should be removed without dropping its bucket peer");

        assert_eq!(
            actual.inspect().to_string(),
            "dict.from_list([#(CollisionKey(2), 20)])",
        );
    }

    #[test]
    fn large_collision_bucket_preserves_aliases_across_updates_and_callbacks() {
        let mut execution = collision_execution(
            r#"
import host/collision

fn fill(next: Int, dict: Dict(collision.CollisionKey, Int)) {
  case next {
    384 -> dict
    _ -> fill(next + 1, do_insert(collision.new(next), next, dict))
  }
}

pub fn main() {
  let original = fill(0, new())
  let alias = original
  assert size(original) == 384
  assert do_fold(fn(_, value, total) { total + value }, 0, original) == 73536

  let replaced = do_insert(collision.new(192), 1192, original)
  assert size(replaced) == 384
  assert get(alias, collision.new(192)) == Ok(192)
  assert get(replaced, collision.new(192)) == Ok(1192)

  let transient = to_transient(replaced)
  let transient = transient_delete(collision.new(0), transient)
  let transient = transient_delete(collision.new(192), transient)
  let remaining = from_transient(transient_delete(collision.new(383), transient))
  assert size(remaining) == 381
  assert get(remaining, collision.new(0)) == Error(Nil)
  assert get(remaining, collision.new(192)) == Error(Nil)
  assert get(remaining, collision.new(383)) == Error(Nil)
  assert get(remaining, collision.new(1)) == Ok(1)
  assert get(remaining, collision.new(382)) == Ok(382)
  assert do_fold(fn(_, value, total) { total + value }, 0, remaining) == 72961

  let mapped = do_map_values(fn(_, value) { value * 2 }, remaining)
  assert size(mapped) == 381
  assert get(mapped, collision.new(100)) == Ok(200)
  assert do_fold(fn(_, value, total) { total + value }, 0, mapped) == 145922
  assert get(alias, collision.new(0)) == Ok(0)
  assert get(alias, collision.new(192)) == Ok(192)
  assert get(alias, collision.new(383)) == Ok(383)
  assert size(alias) == 384
  Nil
}
"#,
            host_provider().unwrap(),
        );
        let mut state = CollisionRunState {
            stdlib: GleamStdlibRunState::from_seed([0; 32]),
            keys: (),
        };
        let actual = crate::execution_fixture::run(&mut execution, &mut state, &mut Vec::new())
            .expect("large collision buckets should retain immutable versions");
        assert_eq!(actual.inspect().to_string(), "Nil");
    }

    #[test]
    fn preserves_nested_host_failure_identity_during_dict_callbacks() {
        let failure =
            HostModule::<GleamStdlibProfile>::new_for_profile("gleam_stdlib", "host/failure")
                .expect("failure module should be valid")
                .with_fallible_function(
                    "reject",
                    |_: geam_core::StringValue, _: BigInt| -> Result<BigInt, HostFailure> {
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
        let mut execution = execution(source, [failure], host_provider().unwrap());
        let error = crate::execution_fixture::run(
            &mut execution,
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
                    |_: geam_core::StringValue,
                     _: BigInt,
                     _: BigInt|
                     -> Result<BigInt, HostFailure> {
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
        let mut execution = execution(source, [failure], host_provider().unwrap());
        let error = crate::execution_fixture::run(
            &mut execution,
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
        let mut execution = execution(source, [failure], host_provider().unwrap());
        let error = crate::execution_fixture::run(
            &mut execution,
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
        let mut execution = execution(
            source,
            Vec::<HostModule<GleamStdlibProfile>>::new(),
            host_provider().unwrap(),
        );
        let error = crate::execution_fixture::run(
            &mut execution,
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
        let execution = execution(
            source,
            Vec::<HostModule<GleamStdlibProfile>>::new(),
            host_provider().unwrap(),
        );

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
