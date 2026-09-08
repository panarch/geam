use super::storage::{DictEntry, DictPayload, DictStorage};
use super::{DictOf, DictSchema};
use crate::dynamic::Dynamic;
use crate::{
    Component, GleamStdlibRunState, HostCall, HostConstruction, HostExternal, HostProvider,
    HostType, HostTypeIndex0, HostTypeIndexNext, HostTypeList, HostTypeListEnd,
};
use geam_core::provider::{Call, Callback, HostResult, Value};
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
        BigInt, Call, Callback, DictEntry, DictPayload, DictStorage, GleamStdlibRunState,
        HostResult, Value,
    };

    #[geam_macros::external(
        name = "Dict",
        parameters = [Key, Item],
        input = DictInput,
        payload = DictPayload,
        manual,
        context = Context,
    )]
    pub struct DictValue<Key, Item>;

    #[geam_macros::external(
        name = "TransientDict",
        parameters = [Key, Item],
        input = TransientDictInput,
        payload = DictPayload,
        manual,
        context = Context,
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
        let key_hash = call.source_hash(&key);
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
        let key_hash = call.source_hash(&key);
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
        let key_hash = call.source_hash(&key);
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
        let key_hash = call.source_hash(&key);
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

    #[geam_macros::function(profile = Profile)]
    fn do_map_values<Key, Mapped, Item>(
        #[geam_macros::call] call: &mut Call<GleamStdlibRunState<Profile::Io>>,
        function: Callback<fn(Value<Key>, Value<Item>) -> Value<Mapped>>,
        dict: DictInput<Key, Item>,
    ) -> HostResult<DictValue<Key, Mapped>> {
        let coordinates = dict.payload().coordinates();
        let mut buckets = im::HashMap::new();
        for (key_hash, index) in coordinates {
            let key = call.restore(
                dict.stored_key(|payload| payload.storage.buckets[&key_hash][index].key.as_ref()),
            );
            let value =
                call.restore(dict.stored_item(|payload| {
                    payload.storage.buckets[&key_hash][index].value.as_ref()
                }));
            let value = call.invoke(function, (key, value))?;
            let entry = dict.payload().storage.buckets[&key_hash][index]
                .with_value(call.store(value).into_retained());
            buckets
                .entry(key_hash)
                .or_insert_with(im::Vector::new)
                .push_back(entry);
        }
        Ok(DictValue::from_payload(DictPayload {
            storage: DictStorage {
                buckets,
                len: dict.payload().storage.len,
            },
        }))
    }

    #[geam_macros::function(profile = Profile)]
    fn transient_delete<Key, Item>(
        #[geam_macros::call] call: &mut Call<GleamStdlibRunState<Profile::Io>>,
        key: Value<Key>,
        transient: TransientDictInput<Key, Item>,
    ) -> TransientDictValue<Key, Item> {
        let key_hash = call.source_hash(&key);
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

    #[geam_macros::function(profile = Profile)]
    fn do_fold<Accumulator, Key, Item>(
        #[geam_macros::call] call: &mut Call<GleamStdlibRunState<Profile::Io>>,
        function: Callback<fn(Value<Key>, Value<Item>, Value<Accumulator>) -> Value<Accumulator>>,
        mut accumulator: Value<Accumulator>,
        dict: DictInput<Key, Item>,
    ) -> HostResult<Value<Accumulator>> {
        for (key_hash, index) in dict.payload().coordinates() {
            let key = call.restore(
                dict.stored_key(|payload| payload.storage.buckets[&key_hash][index].key.as_ref()),
            );
            let value =
                call.restore(dict.stored_item(|payload| {
                    payload.storage.buckets[&key_hash][index].value.as_ref()
                }));
            accumulator = call.invoke(function, (key, value, accumulator))?;
        }
        Ok(accumulator)
    }

    #[geam_macros::function(profile = Profile)]
    fn transient_update_with<Key, Item>(
        #[geam_macros::call] call: &mut Call<GleamStdlibRunState<Profile::Io>>,
        key: Value<Key>,
        function: Callback<fn(Value<Item>) -> Value<Item>>,
        initial: Value<Item>,
        transient: TransientDictInput<Key, Item>,
    ) -> HostResult<TransientDictValue<Key, Item>> {
        let key_hash = call.source_hash(&key);
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
        let value = match index {
            Some(index) => {
                let value = call.restore(transient.stored_item(|payload| {
                    payload.storage.buckets[&key_hash][index].value.as_ref()
                }));
                call.invoke(function, (value,))?
            }
            None => initial,
        };
        let storage = transient.payload().storage.clone();
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

pub fn create_dynamic_dict<'call, Profile, Provider, Return>(
    call: &mut HostCall<'call, Profile, Provider, Return>,
    construction: HostConstruction<'call, DictOf<Dynamic, Dynamic>>,
    entries: impl IntoIterator<Item = (HostExternal<'call, Dynamic>, HostExternal<'call, Dynamic>)>,
) -> HostExternal<'call, DictOf<Dynamic, Dynamic>>
where
    Profile: crate::GleamStdlibLocalProfile,
    Provider: HostProvider<Profile>,
    Return: HostType,
{
    let mut buckets = HashMap::new();
    for (key, value) in entries {
        let key_hash = call.source_hash::<Dynamic>(key);
        insert_first(&mut buckets, key_hash, key, value, |stored, candidate| {
            call.equal::<Dynamic>(*stored, *candidate)
        });
    }

    call.construct_retained_external_with_binding::<
        provider::__GeamProvider,
        DictSchema,
        HostTypeList<Dynamic, HostTypeList<Dynamic, HostTypeListEnd>>,
    >(construction, move |builder| {
        let len = buckets.values().map(Vec::len).sum();
        let buckets = buckets
            .into_iter()
            .map(|(key_hash, entries)| {
                let entries = entries
                    .into_iter()
                    .map(|(key, value)| {
                        DictEntry::new(
                            key_hash,
                            geam_core::__macro_support::retain_argument::<
                                _,
                                _,
                                DictPayload,
                                KeyIndex,
                            >(builder, key),
                            geam_core::__macro_support::retain_argument::<
                                _,
                                _,
                                DictPayload,
                                ItemIndex,
                            >(builder, value),
                        )
                    })
                    .collect();
                (key_hash, entries)
            })
            .collect();
        DictPayload {
            storage: DictStorage { buckets, len },
        }
    })
}

pub(super) fn host_provider<Profile>()
-> Result<crate::HostProviderModule<Profile>, crate::HostRegistrationError>
where
    Profile: crate::GleamStdlibLocalProfile,
{
    provider::__geam_module::<Profile>()
}

pub fn create_transfer_dynamic_dict<'call, Profile, Provider, Return>(
    call: &mut geam_core::host::TransferHostCall<'call, Profile, Provider, Return>,
    construction: HostConstruction<'call, DictOf<Dynamic, Dynamic>>,
    entries: impl IntoIterator<Item = (HostExternal<'call, Dynamic>, HostExternal<'call, Dynamic>)>,
) -> HostExternal<'call, DictOf<Dynamic, Dynamic>>
where
    Profile: crate::GleamStdlibTransferProfile,
    Profile::RunState: Send,
    Provider: HostProvider<Profile>,
    Return: HostType,
{
    let mut buckets = HashMap::new();
    for (key, value) in entries {
        let key_hash = call.source_hash::<Dynamic>(key);
        insert_first(&mut buckets, key_hash, key, value, |stored, candidate| {
            call.equal::<Dynamic>(*stored, *candidate)
        });
    }
    let len = buckets.values().map(Vec::len).sum();
    let buckets = buckets
        .into_iter()
        .map(|(key_hash, entries)| {
            let entries = entries
                .into_iter()
                .map(|(key, value)| {
                    DictEntry::new(
                        key_hash,
                        geam_core::__macro_support::retain_transfer_argument::<
                            _,
                            _,
                            _,
                            _,
                            _,
                            DictPayload,
                            KeyIndex,
                        >(call, &construction, key),
                        geam_core::__macro_support::retain_transfer_argument::<
                            _,
                            _,
                            _,
                            _,
                            _,
                            DictPayload,
                            ItemIndex,
                        >(call, &construction, value),
                    )
                })
                .collect();
            (key_hash, entries)
        })
        .collect();
    call.construct_external_with_binding::<provider::__GeamAsyncProvider, DictSchema, HostTypeList<Dynamic, HostTypeList<Dynamic, HostTypeListEnd>>>(
        construction, DictPayload { storage: DictStorage { buckets, len } },
    )
}

pub(super) fn transfer_host_provider<Profile>()
-> Result<geam_core::TransferHostProviderModule<Profile>, crate::HostRegistrationError>
where
    Profile: crate::GleamStdlibTransferProfile,
    Profile::RunState: Send,
{
    provider::__geam_transfer_module::<Profile>()
}

pub(super) fn insert_first<Key, Value>(
    buckets: &mut HashMap<u64, Vec<(Key, Value)>>,
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
        use crate::dict::function::provider::__GeamAsyncProvider as DictProvider;
        use crate::dict::storage::DictEntry;
        use crate::dict::{DictOf, DictSchema};
        use crate::{
            Component, GleamStdlibHostProfile, GleamStdlibRunState, GleamStdlibTransferStores,
            IoOutput,
        };
        use ecow::EcoString;
        use geam_core::embedding::{FunctionDeclaration, WorkModuleBuilder, with_execution_scope};
        use geam_core::frontend::compile_typed_transfer_host_program;
        use geam_core::host::{
            AsyncHostComponentProfile, AsyncHostExternalBinding, HostFutureStore, TransferHostCall,
        };
        use geam_core::provider::advanced::ProviderTransferRetainedContext as TransferRetainedContext;
        use geam_core::{
            HostCallCompletion, HostExternal, HostProfile, HostProvider, ModuleSource,
            PackageSource, TransferHostProviderSet,
        };
        use geam_runtime_api::FutureComponent;
        use num_bigint::BigInt;
        use std::future::Future;
        use std::pin::pin;
        use std::sync::{Arc, Weak};
        use std::task::{Context, Poll, Waker};

        struct Profile;
        #[derive(Default)]
        struct Stores {
            stdlib: GleamStdlibTransferStores,
            work: HostFutureStore,
        }
        struct State {
            stdlib: GleamStdlibRunState,
            work: (),
            entries: Vec<Weak<DictEntry<TransferRetainedContext>>>,
        }
        struct Observer;
        #[derive(Default)]
        struct Echo(Vec<String>);
        impl geam_core::EchoSink for Echo {
            fn emit(&mut self, output: geam_core::EchoOutput) {
                self.0.push(output.value().inspect().to_string());
            }
        }
        type Dict = DictOf<EcoString, geam_core::HostListType<BigInt>>;

        impl HostProfile for Profile {
            type RunState = State;
            type ExternalStores = Stores;
        }
        impl GleamStdlibHostProfile for Profile {
            type Io = Vec<IoOutput>;
        }
        impl AsyncHostComponentProfile<Component> for Profile {
            fn component_async_stores(stores: &Stores) -> &GleamStdlibTransferStores {
                &stores.stdlib
            }
            fn component_state(state: &mut State) -> &mut GleamStdlibRunState {
                &mut state.stdlib
            }
        }
        impl geam_core::host::HostWorkProfile for Profile {
            type Work = FutureComponent;
        }
        impl AsyncHostComponentProfile<FutureComponent> for Profile {
            fn component_async_stores(stores: &Stores) -> &HostFutureStore {
                &stores.work
            }
            fn component_state(state: &mut State) -> &mut () {
                &mut state.work
            }
        }
        impl HostProvider<Profile> for Observer {
            type State = Vec<Weak<DictEntry<TransferRetainedContext>>>;
            fn project(state: &mut State) -> &mut Self::State {
                &mut state.entries
            }
        }
        impl AsyncHostExternalBinding<Profile, DictSchema> for Observer {
            type Storage = <DictProvider as AsyncHostExternalBinding<Profile, DictSchema>>::Storage;
        }

        fn observe<'call>(
            mut call: TransferHostCall<'call, Profile, Observer, ()>,
            before: HostExternal<'call, Dict>,
            after: HostExternal<'call, Dict>,
            mapped: bool,
        ) -> Result<HostCallCompletion<'call, ()>, geam_core::AsyncHostCallError> {
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

pub fn run() -> Nil {
  let original = new() |> do_insert("first", [1, 2], _) |> do_insert("second", [3], _)
  let updated = do_insert("first", [4, 5], original)
  echo size(updated)
  observe(original, updated, False)
  observe(original, do_map_values(fn(_, value) { value }, original), True)
}
"#
            );
            let dict = super::super::transfer_host_provider::<Profile>()
                .expect("dict registration")
                .with_scoped_function::<Observer, (Dict, Dict, bool), (), _>("observe", observe)
                .expect("observation registration");
            let program = compile_typed_transfer_host_program(
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
                TransferHostProviderSet::new([dict]).expect("provider set"),
            )
            .expect("dict source");
            let (bindings, entry) = WorkModuleBuilder::new(program)
                .expect("plan")
                .function(FunctionDeclaration::<(), ()>::new("run"))
                .expect("entry");
            let mut module = bindings.seal().expect("seal");
            let mut state = State {
                stdlib: GleamStdlibRunState::from_seed([0; 32]),
                work: (),
                entries: Vec::new(),
            };
            assert!(std::ptr::eq(
                <Profile as AsyncHostComponentProfile<Component>>::component_state(&mut state),
                &state.stdlib,
            ));
            assert!(std::ptr::eq(
                <Profile as AsyncHostComponentProfile<FutureComponent>>::component_state(
                    &mut state
                ),
                &state.work,
            ));
            let mut echo = Echo::default();
            {
                let mut task = pin!(with_execution_scope(async |guard| {
                    module
                        .attach(guard, &mut state, &mut echo)
                        .call(&entry, ())
                        .expect("direct update");
                }));
                assert_eq!(
                    task.as_mut().poll(&mut Context::from_waker(Waker::noop())),
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
    use super::{insert_first, provider::__GeamProvider as DictProvider};
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

    fn collision_execution(source: &str) -> HostedExecution<CollisionProfile> {
        const COLLISION_SOURCE: &str = r#"
pub type CollisionKey

@external(erlang, "host", "new")
pub fn new(value: Int) -> CollisionKey
"#;

        let source = format!("{DICT_DECLARATIONS}\n{source}");
        let providers = [
            host_provider::<CollisionProfile>().expect("official dict provider should register"),
            collision_provider(),
        ];
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
        let execution = execution(source, [float]);
        let mut echoes = Vec::new();
        let actual = execution
            .run_main(&mut GleamStdlibRunState::from_seed([0; 32]), &mut echoes)
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
        let execution = collision_execution(
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

        let actual = execution
            .run_main(&mut state, &mut Vec::new())
            .expect("colliding dict key should be removed without dropping its bucket peer");

        assert_eq!(
            actual.inspect().to_string(),
            "dict.from_list([#(CollisionKey(2), 20)])",
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
