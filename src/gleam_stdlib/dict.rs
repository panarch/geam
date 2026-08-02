use super::GleamStdlibHostProfile;
use crate::{
    HostCall, HostCallCompletion, HostCallError, HostCallable, HostCustomConstructorAt,
    HostCustomConstructorDefinition, HostCustomConstructorList, HostCustomConstructorListEnd,
    HostCustomField, HostCustomFieldList, HostCustomFieldListEnd, HostCustomIndex0,
    HostCustomIndexNext, HostCustomSchema, HostCustomType, HostCustomTypeArgument, HostExternal,
    HostExternalEquality, HostExternalHashing, HostExternalInspection, HostExternalPayloadBuilder,
    HostExternalPayloadView, HostExternalSchema, HostExternalStorage, HostExternalStore,
    HostExternalType, HostFunctionType, HostProfile, HostProvider, HostProviderModule,
    HostRegistrationError, HostStoredType, HostStoredValue, HostType, HostTypeAt, HostTypeIndex0,
    HostTypeIndexNext, HostTypeList, HostTypeListEnd, HostTypeParameter, HostTypeSequence,
    HostValue,
};
use ecow::EcoString;
use im::{HashMap, Vector};
use num_bigint::BigInt;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;
use std::rc::Rc;

#[derive(Default)]
pub(super) struct Stores {
    dicts: HostExternalStore<DictPayload>,
    transients: HostExternalStore<TransientDictPayload>,
}

struct DictSchema;

struct TransientDictSchema;

struct DictProvider<Profile>(PhantomData<Profile>);

struct DictPayload {
    storage: DictStorage,
}

struct TransientDictPayload {
    storage: DictStorage,
}

#[derive(Clone, Default)]
struct DictStorage {
    buckets: HashMap<u64, Vector<Rc<DictEntry>>>,
    len: usize,
}

struct DictEntry {
    key_hash: u64,
    key: HostStoredValue<StoredKey>,
    value: HostStoredValue<StoredValue>,
}

trait DictPayloadStorage {
    fn storage(&self) -> &DictStorage;
}

struct ResultSchema;

struct ResultOkField;

struct ResultOkDefinition;

struct ResultErrorField;

struct ResultErrorDefinition;

type KeyIndex = HostTypeIndex0;
type ItemIndex = HostTypeIndexNext<KeyIndex>;
type StoredKey = HostStoredType<KeyIndex>;
type StoredValue = HostStoredType<ItemIndex>;

// Function parameters follow the planner's return-first canonical order.
type Key = HostTypeParameter<0>;
type Item = HostTypeParameter<1>;
type DictArguments = HostTypeList<Key, HostTypeList<Item, HostTypeListEnd>>;
type Dict = HostExternalType<DictSchema, DictArguments>;
type TransientDict = HostExternalType<TransientDictSchema, DictArguments>;
type UpdateFunctionArguments = HostTypeList<Item, HostTypeListEnd>;
type UpdateFunction = HostFunctionType<UpdateFunctionArguments, Item>;

type GetItem = HostTypeParameter<0>;
type GetKey = HostTypeParameter<1>;
type GetDictArguments = HostTypeList<GetKey, HostTypeList<GetItem, HostTypeListEnd>>;
type GetDict = HostExternalType<DictSchema, GetDictArguments>;
type GetResultArguments = HostTypeList<GetItem, HostTypeList<(), HostTypeListEnd>>;
type GetResult = HostCustomType<ResultSchema, GetResultArguments>;
type GetOk = HostCustomConstructorAt<GetResult, HostCustomIndex0, ResultOkDefinition>;
type GetError = HostCustomConstructorAt<
    GetResult,
    HostCustomIndexNext<HostCustomIndex0>,
    ResultErrorDefinition,
>;

type MapKey = HostTypeParameter<0>;
type MapOutput = HostTypeParameter<1>;
type MapInput = HostTypeParameter<2>;
type MapInputDictArguments = HostTypeList<MapKey, HostTypeList<MapInput, HostTypeListEnd>>;
type MapInputDict = HostExternalType<DictSchema, MapInputDictArguments>;
type MapOutputDictArguments = HostTypeList<MapKey, HostTypeList<MapOutput, HostTypeListEnd>>;
type MapOutputDict = HostExternalType<DictSchema, MapOutputDictArguments>;
type MapFunctionArguments = HostTypeList<MapKey, HostTypeList<MapInput, HostTypeListEnd>>;
type MapFunction = HostFunctionType<MapFunctionArguments, MapOutput>;

type FoldAccumulator = HostTypeParameter<0>;
type FoldKey = HostTypeParameter<1>;
type FoldValue = HostTypeParameter<2>;
type FoldDictArguments = HostTypeList<FoldKey, HostTypeList<FoldValue, HostTypeListEnd>>;
type FoldDict = HostExternalType<DictSchema, FoldDictArguments>;
type FoldFunctionArguments =
    HostTypeList<FoldKey, HostTypeList<FoldValue, HostTypeList<FoldAccumulator, HostTypeListEnd>>>;
type FoldFunction = HostFunctionType<FoldFunctionArguments, FoldAccumulator>;

impl HostExternalSchema for DictSchema {
    const PACKAGE: &'static str = "gleam_stdlib";
    const MODULE: &'static str = "gleam/dict";
    const NAME: &'static str = "Dict";
    const PARAMETER_COUNT: usize = 2;
}

impl HostExternalSchema for TransientDictSchema {
    const PACKAGE: &'static str = "gleam_stdlib";
    const MODULE: &'static str = "gleam/dict";
    const NAME: &'static str = "TransientDict";
    const PARAMETER_COUNT: usize = 2;
}

impl<Profile> HostProvider<Profile> for DictProvider<Profile>
where
    Profile: GleamStdlibHostProfile,
{
    type State = Profile::RunState;

    fn project(state: &mut Profile::RunState) -> &mut Self::State {
        state
    }
}

impl<Profile> HostExternalStorage<DictSchema> for Profile
where
    Profile: GleamStdlibHostProfile,
{
    type Payload = DictPayload;

    fn store(stores: &Self::ExternalStores) -> &HostExternalStore<Self::Payload> {
        &Profile::gleam_stdlib_stores(stores).dict.dicts
    }

    fn source_equal(
        context: &HostExternalEquality<'_>,
        left: &Self::Payload,
        right: &Self::Payload,
    ) -> bool {
        storage_equal(context, &left.storage, &right.storage)
    }

    fn source_hash(context: &HostExternalHashing<'_>, value: &Self::Payload) -> u64 {
        storage_hash(context, &value.storage)
    }

    fn inspect(context: &HostExternalInspection<'_>, value: &Self::Payload) -> EcoString {
        inspect_storage(context, &value.storage)
    }
}

impl<Profile> HostExternalStorage<TransientDictSchema> for Profile
where
    Profile: GleamStdlibHostProfile,
{
    type Payload = TransientDictPayload;

    fn store(stores: &Self::ExternalStores) -> &HostExternalStore<Self::Payload> {
        &Profile::gleam_stdlib_stores(stores).dict.transients
    }

    fn source_equal(
        context: &HostExternalEquality<'_>,
        left: &Self::Payload,
        right: &Self::Payload,
    ) -> bool {
        storage_equal(context, &left.storage, &right.storage)
    }

    fn source_hash(context: &HostExternalHashing<'_>, value: &Self::Payload) -> u64 {
        storage_hash(context, &value.storage)
    }

    fn inspect(context: &HostExternalInspection<'_>, value: &Self::Payload) -> EcoString {
        inspect_storage(context, &value.storage)
    }
}

impl DictPayloadStorage for DictPayload {
    fn storage(&self) -> &DictStorage {
        &self.storage
    }
}

impl DictPayloadStorage for TransientDictPayload {
    fn storage(&self) -> &DictStorage {
        &self.storage
    }
}

impl HostCustomField for ResultOkField {
    const LABEL: Option<&'static str> = None;

    type Type = HostCustomTypeArgument<HostTypeIndex0>;
}

impl HostCustomConstructorDefinition for ResultOkDefinition {
    const NAME: &'static str = "Ok";

    type Fields = HostCustomFieldList<ResultOkField, HostCustomFieldListEnd>;
}

impl HostCustomField for ResultErrorField {
    const LABEL: Option<&'static str> = None;

    type Type = HostCustomTypeArgument<HostTypeIndexNext<HostTypeIndex0>>;
}

impl HostCustomConstructorDefinition for ResultErrorDefinition {
    const NAME: &'static str = "Error";

    type Fields = HostCustomFieldList<ResultErrorField, HostCustomFieldListEnd>;
}

impl HostCustomSchema for ResultSchema {
    const PACKAGE: &'static str = "";
    const MODULE: &'static str = "gleam";
    const NAME: &'static str = "Result";
    const PARAMETER_COUNT: usize = 2;

    type Constructors = HostCustomConstructorList<
        ResultOkDefinition,
        HostCustomConstructorList<ResultErrorDefinition, HostCustomConstructorListEnd>,
    >;
}

pub(super) fn host_provider<Profile>() -> Result<HostProviderModule<Profile>, HostRegistrationError>
where
    Profile: GleamStdlibHostProfile,
{
    HostProviderModule::new("gleam_stdlib", "gleam/dict")
        .and_then(HostProviderModule::with_external_type::<DictSchema>)
        .and_then(HostProviderModule::with_external_type::<TransientDictSchema>)
        .and_then(|provider| {
            provider.with_scoped_function::<DictProvider<Profile>, (Dict,), TransientDict, _>(
                "to_transient",
                to_transient::<Profile>,
            )
        })
        .and_then(|provider| {
            provider.with_scoped_function::<DictProvider<Profile>, (TransientDict,), Dict, _>(
                "from_transient",
                from_transient::<Profile>,
            )
        })
        .and_then(|provider| {
            provider.with_scoped_function::<DictProvider<Profile>, (Dict,), BigInt, _>(
                "size",
                size::<Profile>,
            )
        })
        .and_then(|provider| {
            provider.with_scoped_function::<DictProvider<Profile>, (Key, Dict), bool, _>(
                "do_has_key",
                do_has_key::<Profile>,
            )
        })
        .and_then(|provider| {
            provider
                .with_scoped_function::<DictProvider<Profile>, (), Dict, _>("new", new::<Profile>)
        })
        .and_then(|provider| {
            provider.with_scoped_function::<DictProvider<Profile>, (GetDict, GetKey), GetResult, _>(
                "get",
                get::<Profile>,
            )
        })
        .and_then(|provider| {
            provider.with_scoped_function::<DictProvider<Profile>, (Key, Item, Dict), Dict, _>(
                "do_insert",
                do_insert::<Profile>,
            )
        })
        .and_then(|provider| {
            provider.with_scoped_function::<
                DictProvider<Profile>,
                (Key, Item, TransientDict),
                TransientDict,
                _,
            >("transient_insert", transient_insert::<Profile>)
        })
        .and_then(|provider| {
            provider.with_scoped_function::<
                DictProvider<Profile>,
                (MapFunction, MapInputDict),
                MapOutputDict,
                _,
            >("do_map_values", do_map_values::<Profile>)
        })
        .and_then(|provider| {
            provider.with_scoped_function::<
                DictProvider<Profile>,
                (Key, TransientDict),
                TransientDict,
                _,
            >("transient_delete", transient_delete::<Profile>)
        })
        .and_then(|provider| {
            provider.with_scoped_function::<
                DictProvider<Profile>,
                (FoldFunction, FoldAccumulator, FoldDict),
                FoldAccumulator,
                _,
            >("do_fold", do_fold::<Profile>)
        })
        .and_then(|provider| {
            provider.with_scoped_function::<
                DictProvider<Profile>,
                (Key, UpdateFunction, Item, TransientDict),
                TransientDict,
                _,
            >("transient_update_with", transient_update_with::<Profile>)
        })
}

fn storage_equal(
    context: &HostExternalEquality<'_>,
    left: &DictStorage,
    right: &DictStorage,
) -> bool {
    left.len == right.len
        && left.entries().all(|left| {
            right.buckets.get(&left.key_hash).is_some_and(|bucket| {
                bucket.iter().any(|right| {
                    context.stored_values_equal(&left.key, &right.key)
                        && context.stored_values_equal(&left.value, &right.value)
                })
            })
        })
}

fn storage_hash(context: &HostExternalHashing<'_>, storage: &DictStorage) -> u64 {
    let mut sum = 0_u64;
    let mut xor = 0_u64;
    for entry in storage.entries() {
        let mut hasher = DefaultHasher::new();
        entry.key_hash.hash(&mut hasher);
        context.stored_value_hash(&entry.value).hash(&mut hasher);
        let hash = hasher.finish();
        sum = sum.wrapping_add(hash);
        xor ^= hash.rotate_left(29);
    }

    let mut hasher = DefaultHasher::new();
    storage.len.hash(&mut hasher);
    sum.hash(&mut hasher);
    xor.hash(&mut hasher);
    hasher.finish()
}

fn inspect_storage(context: &HostExternalInspection<'_>, storage: &DictStorage) -> EcoString {
    let mut entries = storage
        .entries()
        .map(|entry| {
            format!(
                "#({}, {})",
                context.inspect_stored_value(&entry.key),
                context.inspect_stored_value(&entry.value),
            )
        })
        .collect::<Vec<_>>();
    entries.sort_unstable();
    format!("dict.from_list([{}])", entries.join(", ")).into()
}

impl DictStorage {
    fn entries(&self) -> impl Iterator<Item = &Rc<DictEntry>> {
        self.buckets.values().flat_map(Vector::iter)
    }

    fn with_entry(&self, key_hash: u64, index: Option<usize>, entry: Rc<DictEntry>) -> Self {
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

    fn without_entry(&self, key_hash: u64, index: usize) -> Self {
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

    fn matching_index(
        &self,
        key_hash: u64,
        is_equal: &mut dyn FnMut(usize) -> bool,
    ) -> Option<usize> {
        let bucket = self.buckets.get(&key_hash)?;
        (0..bucket.len()).find(|index| is_equal(*index))
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

fn to_transient<'call, Profile>(
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

fn from_transient<'call, Profile>(
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

fn size<'call, Profile>(
    call: HostCall<'call, Profile, DictProvider<Profile>, BigInt>,
    dict: HostExternal<'call, Dict>,
) -> Result<HostCallCompletion<'call, BigInt>, HostCallError>
where
    Profile: GleamStdlibHostProfile,
{
    let size = call.external_payload(dict).storage.len;
    Ok(call.return_value(BigInt::from(size)))
}

fn do_has_key<'call, Profile>(
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

fn new<'call, Profile>(
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

fn get<'call, Profile>(
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

fn do_insert<'call, Profile>(
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

fn transient_insert<'call, Profile>(
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

fn do_map_values<'call, Profile>(
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

fn transient_delete<'call, Profile>(
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

fn do_fold<'call, Profile>(
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

fn transient_update_with<'call, Profile>(
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
    use super::{
        DictEntry, DictProvider, DictStorage, StoredKey, StoredValue, host_provider,
        inspect_storage, storage_equal, storage_hash,
    };
    use crate::gleam_stdlib::GleamStdlibProfile;
    use crate::{
        HostExternalEquality, HostExternalHashing, HostExternalInspection, HostFailure, HostModule,
        HostProvider, HostProviderSet, HostedExecution, ModuleSource, PackageSource,
        compile_typed_host_program, plan_host_program,
    };
    use ecow::EcoString;
    use num_bigint::BigInt;
    use std::rc::Rc;

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

    fn entry(key_hash: u64, key: i64, value: i64) -> Rc<DictEntry> {
        Rc::new(DictEntry {
            key_hash,
            key: crate::HostStoredValue::<StoredKey>::new(
                crate::runtime::StoredRuntimeValue::test_int(BigInt::from(key)),
            ),
            value: crate::HostStoredValue::<StoredValue>::new(
                crate::runtime::StoredRuntimeValue::test_int(BigInt::from(value)),
            ),
        })
    }

    #[test]
    fn persistent_buckets_replace_remove_and_share_only_unchanged_entries() {
        let first = entry(7, 1, 10);
        let second = entry(7, 2, 20);
        let replacement = entry(7, 1, 30);
        let distinct = entry(11, 3, 40);

        let one = DictStorage::default().with_entry(7, None, Rc::clone(&first));
        let alias = one.clone();
        let collided =
            one.with_entry(7, None, Rc::clone(&second))
                .with_entry(11, None, Rc::clone(&distinct));
        let replaced = collided.with_entry(7, Some(0), Rc::clone(&replacement));
        let retained_collision = replaced.without_entry(7, 0);
        let removed_bucket = retained_collision.without_entry(7, 0);

        assert_eq!(alias.len, 1);
        assert!(Rc::ptr_eq(&alias.buckets[&7][0], &first));
        assert_eq!(collided.len, 3);
        assert!(Rc::ptr_eq(&collided.buckets[&7][0], &first));
        assert!(Rc::ptr_eq(&collided.buckets[&7][1], &second));
        assert!(Rc::ptr_eq(&collided.buckets[&11][0], &distinct));
        assert_eq!(replaced.len, 3);
        assert!(Rc::ptr_eq(&replaced.buckets[&7][0], &replacement));
        assert!(Rc::ptr_eq(&replaced.buckets[&7][1], &second));
        assert_eq!(retained_collision.len, 2);
        assert!(Rc::ptr_eq(&retained_collision.buckets[&7][0], &second));
        assert_eq!(removed_bucket.len, 1);
        assert!(!removed_bucket.buckets.contains_key(&7));
        assert!(Rc::ptr_eq(&removed_bucket.buckets[&11][0], &distinct));

        let mut visited = Vec::new();
        assert_eq!(
            collided.matching_index(7, &mut |index| {
                visited.push(index);
                index == 1
            }),
            Some(1),
        );
        assert_eq!(visited, [0, 1]);
        assert_eq!(collided.matching_index(7, &mut |_| false), None);
        assert_eq!(collided.matching_index(99, &mut |_| true), None);
    }

    #[test]
    fn provider_projects_the_complete_run_state() {
        let mut state = ();

        assert_eq!(
            *<DictProvider<GleamStdlibProfile> as HostProvider<GleamStdlibProfile>>::project(
                &mut state,
            ),
            (),
        );
    }

    #[test]
    fn source_semantics_resolve_collisions_and_ignore_bucket_iteration_order() {
        let first = entry(7, 1, 10);
        let equal_first = entry(7, 1, 10);
        let collision = entry(7, 2, 20);
        let equal_collision = entry(7, 2, 20);
        let different_value = entry(7, 2, 30);

        let left = DictStorage::default()
            .with_entry(7, None, Rc::clone(&first))
            .with_entry(7, None, Rc::clone(&collision));
        let equal = DictStorage::default()
            .with_entry(7, None, Rc::clone(&equal_collision))
            .with_entry(7, None, Rc::clone(&equal_first));
        let different = DictStorage::default()
            .with_entry(7, None, Rc::clone(&equal_first))
            .with_entry(7, None, Rc::clone(&different_value));

        let stored_equal = |left: &crate::runtime::StoredRuntimeValue,
                            right: &crate::runtime::StoredRuntimeValue| {
            std::ptr::eq(left, right)
                || std::ptr::eq(left, &first.key.value)
                    && std::ptr::eq(right, &equal_first.key.value)
                || std::ptr::eq(left, &first.value.value)
                    && std::ptr::eq(right, &equal_first.value.value)
                || std::ptr::eq(left, &collision.key.value)
                    && (std::ptr::eq(right, &equal_collision.key.value)
                        || std::ptr::eq(right, &different_value.key.value))
                || std::ptr::eq(left, &collision.value.value)
                    && std::ptr::eq(right, &equal_collision.value.value)
        };
        let stored_hash = |value: &crate::runtime::StoredRuntimeValue| {
            if std::ptr::eq(value, &first.value.value)
                || std::ptr::eq(value, &equal_first.value.value)
            {
                10
            } else if std::ptr::eq(value, &collision.value.value)
                || std::ptr::eq(value, &equal_collision.value.value)
            {
                20
            } else {
                30
            }
        };
        let inspect = |value: &crate::runtime::StoredRuntimeValue| {
            if std::ptr::eq(value, &first.key.value) {
                EcoString::from("z-key")
            } else if std::ptr::eq(value, &first.value.value) {
                EcoString::from("one")
            } else if std::ptr::eq(value, &collision.key.value) {
                EcoString::from("a-key")
            } else {
                EcoString::from("two")
            }
        };
        let equality = HostExternalEquality::new(&stored_equal);
        let hashing = HostExternalHashing::new(&stored_hash);
        let inspection = HostExternalInspection::new(&inspect);

        assert!(storage_equal(&equality, &left, &equal));
        assert!(!storage_equal(&equality, &left, &different));
        assert!(!storage_equal(&equality, &left, &DictStorage::default()));
        assert_eq!(
            storage_hash(&hashing, &left),
            storage_hash(&hashing, &equal)
        );
        assert_ne!(
            storage_hash(&hashing, &left),
            storage_hash(&hashing, &different),
        );
        assert_eq!(
            inspect_storage(&inspection, &left),
            "dict.from_list([#(a-key, two), #(z-key, one)])",
        );
        assert_eq!(
            inspect_storage(&inspection, &DictStorage::default()),
            "dict.from_list([])",
        );
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
            .run_main(&mut (), &mut Vec::new())
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
            .run_main(&mut (), &mut Vec::new())
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
            .run_main(&mut (), &mut Vec::new())
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
            .run_main(&mut (), &mut Vec::new())
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
            .run_main(&mut (), &mut Vec::new())
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
