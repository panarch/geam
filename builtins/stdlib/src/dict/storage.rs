use ecow::EcoString;
use geam_core::provider::advanced::{
    Equality, Hashing, Index0, Inspection, NativeMap, NativeMapEntry, NativeValue, Next, Retained,
    RetainedExternalPayload,
};
use im::{HashMap, Vector};

pub struct DictPayload {
    pub(super) storage: DictStorage,
}

pub(super) struct DictStorage {
    pub(super) buckets: HashMap<u64, Vector<std::sync::Arc<DictEntry>>>,
    pub(super) len: usize,
}

pub(super) struct DictEntry {
    pub(super) key_hash: u64,
    pub(super) key: std::sync::Arc<Retained<DictPayload, Index0>>,
    pub(super) value: std::sync::Arc<Retained<DictPayload, Next<Index0>>>,
}

impl DictEntry {
    pub(super) fn new(
        key_hash: u64,
        key: Retained<DictPayload, Index0>,
        value: Retained<DictPayload, Next<Index0>>,
    ) -> std::sync::Arc<Self> {
        std::sync::Arc::new(Self {
            key_hash,
            key: std::sync::Arc::new(key),
            value: std::sync::Arc::new(value),
        })
    }

    pub(super) fn with_value(
        &self,
        value: Retained<DictPayload, Next<Index0>>,
    ) -> std::sync::Arc<Self> {
        std::sync::Arc::new(Self {
            key_hash: self.key_hash,
            key: self.key.clone(),
            value: std::sync::Arc::new(value),
        })
    }
}

impl Clone for DictStorage {
    fn clone(&self) -> Self {
        Self {
            buckets: self.buckets.clone(),
            len: self.len,
        }
    }
}

impl Default for DictStorage {
    fn default() -> Self {
        Self {
            buckets: HashMap::new(),
            len: 0,
        }
    }
}

impl DictPayload {
    pub(crate) fn coordinates(&self) -> Vec<(u64, usize)> {
        self.storage
            .buckets
            .iter()
            .flat_map(|(key_hash, bucket)| (0..bucket.len()).map(move |index| (*key_hash, index)))
            .collect()
    }

    fn native_value(&self) -> NativeValue {
        NativeValue::map(NativeMap::new(
            self.storage.clone(),
            |storage| storage.len,
            DictStorage::native_entries,
            DictStorage::native_get,
        ))
    }
}

impl RetainedExternalPayload for DictPayload {
    fn source_equal(&self, context: &Equality<'_>, other: &Self) -> bool {
        self.native_value()
            .source_equal(context, &other.native_value())
    }

    fn source_hash(&self, context: &Hashing<'_>) -> u64 {
        self.native_value().source_hash(context)
    }

    fn inspect(&self, context: &Inspection<'_>) -> EcoString {
        self.native_value().inspect(context)
    }

    fn native_view(&self) -> Option<NativeValue> {
        Some(self.native_value())
    }
}

impl DictStorage {
    pub(super) fn with_entry(
        &self,
        key_hash: u64,
        index: Option<usize>,
        entry: std::sync::Arc<DictEntry>,
    ) -> Self {
        let mut bucket = self.buckets.get(&key_hash).cloned().unwrap_or_default();
        let len = match index {
            Some(index) => {
                bucket[index] = entry;
                self.len
            }
            None => {
                bucket.push_back(entry);
                self.len + 1
            }
        };
        let mut buckets = self.buckets.clone();
        buckets.insert(key_hash, bucket);
        Self { buckets, len }
    }

    pub(super) fn without_entry(&self, key_hash: u64, index: usize) -> Self {
        let mut bucket = self.buckets[&key_hash].clone();
        let removed = bucket.remove(index);
        drop(removed);
        let mut buckets = self.buckets.clone();
        if bucket.is_empty() {
            buckets.remove(&key_hash);
        } else {
            buckets.insert(key_hash, bucket);
        }
        Self {
            buckets,
            len: self.len - 1,
        }
    }

    pub(super) fn matching_index(
        &self,
        key_hash: u64,
        is_equal: &mut dyn FnMut(usize) -> bool,
    ) -> Option<usize> {
        let bucket = self.buckets.get(&key_hash)?;
        (0..bucket.len()).find(|index| is_equal(*index))
    }

    fn native_entries(&self) -> impl Iterator<Item = NativeMapEntry> + Send + 'static + use<> {
        self.buckets.clone().into_iter().flat_map(|(_, bucket)| {
            bucket.into_iter().map(|entry| NativeMapEntry {
                key_hash: entry.key_hash,
                key: entry.key.native_view(),
                value: entry.value.native_view(),
            })
        })
    }

    fn native_get(
        &self,
        hash: u64,
        key: &NativeValue,
        equal: &dyn Fn(&NativeValue, &NativeValue) -> bool,
    ) -> Option<NativeValue> {
        self.buckets.get(&hash)?.iter().find_map(|entry| {
            equal(&entry.key.native_view(), key).then(|| entry.value.native_view())
        })
    }
}

#[cfg(test)]
mod tests {
    use super::{DictPayload, DictStorage};
    use crate::dict::{DictExternalStorage, DictOf, DictSchema};
    use crate::{
        Component, GleamStdlibHostProfile, GleamStdlibRunState, GleamStdlibStores, IoOutput,
    };
    use ecow::EcoString;
    use geam_core::provider::advanced::{Equality, Hashing, Inspection, RetainedExternalPayload};
    use geam_core::{
        HostCall, HostCallCompletion, HostCallError, HostComponentProfile, HostExternal,
        HostExternalBinding, HostExternalSchema, HostExternalStorage, HostExternalStore,
        HostExternalType, HostProfile, HostProvider, HostProviderModule, HostProviderSet,
        HostTypeList, HostTypeListEnd, HostedExecution, ModuleSource, PackageSource,
        compile_typed_host_program, plan_host_program,
    };
    use num_bigint::BigInt;

    struct Profile;
    struct Snapshot;
    struct SnapshotStorage;
    type SnapshotType = HostExternalType<Snapshot>;

    impl HostProfile for Profile {
        type RunState = GleamStdlibRunState;
        type ExternalStores = (GleamStdlibStores, HostExternalStore<DictPayload>);
        type ExecutionState = ();
    }

    impl HostComponentProfile<Component> for Profile {
        fn component_stores(stores: &Self::ExternalStores) -> &GleamStdlibStores {
            &stores.0
        }

        fn component_state(state: &mut Self::RunState) -> &mut GleamStdlibRunState {
            state
        }
    }

    impl GleamStdlibHostProfile for Profile {
        type Io = Vec<IoOutput>;
    }

    impl HostProvider<Profile> for Snapshot {
        type State = GleamStdlibRunState;

        fn project(state: &mut Self::State) -> &mut Self::State {
            state
        }
    }

    impl HostExternalSchema for Snapshot {
        const PACKAGE: &'static str = "application";
        const MODULE: &'static str = "main";
        const NAME: &'static str = "Snapshot";
        const PARAMETER_COUNT: usize = 0;
    }

    impl HostExternalBinding<Profile, Snapshot> for Snapshot {
        type Storage = SnapshotStorage;
    }

    impl HostExternalBinding<Profile, DictSchema> for Snapshot {
        type Storage = DictExternalStorage;
    }

    impl HostExternalStorage<Profile, Snapshot> for SnapshotStorage {
        type Payload = DictPayload;

        fn store(
            stores: &<Profile as HostProfile>::ExternalStores,
        ) -> &HostExternalStore<DictPayload> {
            &stores.1
        }

        fn source_equal(context: &Equality<'_>, left: &DictPayload, right: &DictPayload) -> bool {
            left.source_equal(context, right)
        }

        fn source_hash(context: &Hashing<'_>, value: &DictPayload) -> u64 {
            value.source_hash(context)
        }

        fn inspect(context: &Inspection<'_>, value: &DictPayload) -> EcoString {
            value.inspect(context)
        }
    }

    fn snapshot<'call>(
        mut call: HostCall<'call, Profile, Snapshot, SnapshotType>,
        dict: HostExternal<'call, DictOf<BigInt, EcoString>>,
    ) -> Result<HostCallCompletion<'call, SnapshotType>, HostCallError> {
        let view = call.provider_external_view_with::<
            Snapshot,
            DictSchema,
            HostTypeList<BigInt, HostTypeList<EcoString, HostTypeListEnd>>,
        >(dict);
        let storage: DictStorage = view.storage.clone();
        drop(view);
        let value = call.create_external(DictPayload { storage });
        Ok(call.return_value(value))
    }

    fn hash<'call>(
        call: HostCall<'call, Profile, Snapshot, BigInt>,
        value: HostExternal<'call, SnapshotType>,
    ) -> Result<HostCallCompletion<'call, BigInt>, HostCallError> {
        let hash = call.source_hash::<SnapshotType>(value).into();
        Ok(call.return_value(hash))
    }

    #[test]
    fn dict_payload_semantics_remain_coherent_inside_an_opaque_snapshot() {
        let snapshot = HostProviderModule::new("application", "main")
            .unwrap()
            .with_external_type::<Snapshot, Snapshot>()
            .unwrap()
            .with_scoped_function::<Snapshot, (DictOf<BigInt, EcoString>,), SnapshotType, _>(
                "snapshot", snapshot,
            )
            .unwrap()
            .with_scoped_function::<Snapshot, (SnapshotType,), BigInt, _>("hash", hash)
            .unwrap();
        let dict = crate::dict::host_provider::<Profile>().unwrap();
        let program = compile_typed_host_program(
            "application",
            "main",
            [
                PackageSource::new(
                    "gleam_stdlib",
                    Vec::<String>::new(),
                    [ModuleSource::new(
                        "gleam/dict",
                        "src/gleam/dict.gleam",
                        r#"
pub type Dict(key, value)
type TransientDict(key, value)
@external(erlang, "gleam_stdlib", "identity")
fn to_transient(dict: Dict(key, value)) -> TransientDict(key, value)
@external(erlang, "gleam_stdlib", "identity")
fn from_transient(dict: TransientDict(key, value)) -> Dict(key, value)
@external(erlang, "maps", "size")
fn size(dict: Dict(key, value)) -> Int
@external(erlang, "maps", "is_key")
fn do_has_key(key: key, dict: Dict(key, value)) -> Bool
@external(erlang, "maps", "new")
pub fn new() -> Dict(key, value)
@external(erlang, "gleam_stdlib", "map_get")
fn get(dict: Dict(key, value), key: key) -> Result(value, Nil)
@external(erlang, "maps", "put")
pub fn do_insert(key: key, value: value, dict: Dict(key, value)) -> Dict(key, value)
@external(erlang, "maps", "put")
fn transient_insert(key: key, value: value, dict: TransientDict(key, value)) -> TransientDict(key, value)
@external(erlang, "maps", "map")
fn do_map_values(function: fn(key, value) -> mapped, dict: Dict(key, value)) -> Dict(key, mapped)
@external(erlang, "maps", "remove")
fn transient_delete(key: key, dict: TransientDict(key, value)) -> TransientDict(key, value)
@external(erlang, "maps", "fold")
fn do_fold(function: fn(key, value, accumulator) -> accumulator, initial: accumulator, dict: Dict(key, value)) -> accumulator
@external(erlang, "maps", "update_with")
fn transient_update_with(key: key, function: fn(value) -> value, initial: value, dict: TransientDict(key, value)) -> TransientDict(key, value)
"#,
                    )],
                ),
                PackageSource::new(
                    "application",
                    ["gleam_stdlib"],
                    [ModuleSource::new(
                        "main",
                        "src/main.gleam",
                        r#"
import gleam/dict.{type Dict}
pub type Snapshot
@external(erlang, "native", "snapshot")
fn snapshot(value: Dict(Int, String)) -> Snapshot
@external(erlang, "native", "hash")
fn hash(value: Snapshot) -> Int
pub fn main() {
  let original = dict.new() |> dict.do_insert(1, "one", _) |> dict.do_insert(2, "two", _)
  let first = snapshot(original)
  let same = snapshot(dict.new() |> dict.do_insert(2, "two", _) |> dict.do_insert(1, "one", _))
  let changed = snapshot(dict.do_insert(1, "changed", original))
  #(first == same, first != changed, hash(first) == hash(same), first)
}
"#,
                    )],
                ),
            ],
            HostProviderSet::from_providers([dict, snapshot]).unwrap(),
        )
        .unwrap();
        let mut execution =
            HostedExecution::try_from_module_plan(plan_host_program(program).unwrap()).unwrap();
        let mut state = GleamStdlibRunState::from_seed([0; 32]);
        let state_address = std::ptr::from_mut(&mut state);
        assert!(std::ptr::eq(
            state_address,
            <Snapshot as HostProvider<Profile>>::project(&mut state),
        ));
        assert!(std::ptr::eq(
            state_address,
            <Profile as HostComponentProfile<Component>>::component_state(&mut state),
        ));
        let value =
            crate::execution_fixture::run(&mut execution, &mut state, &mut Vec::new()).unwrap();
        drop(execution);
        assert_eq!(
            value.inspect().to_string(),
            "#(True, True, True, dict.from_list([#(1, \"one\"), #(2, \"two\")]))"
        );
    }
}
