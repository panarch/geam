mod function;
mod schema;
mod storage;

pub(crate) use function::create_dynamic_dict;
pub(in crate::gleam_stdlib) use function::lookup;
pub(crate) use schema::{DictOf, DictSchema};
pub(crate) use storage::DictExternalStorage;
pub(super) use storage::Stores;

use self::function::{
    DictProvider, do_fold, do_has_key, do_insert, do_map_values, from_transient, get, new, size,
    to_transient, transient_delete, transient_insert, transient_update_with,
};
use self::schema::{
    Dict, FoldAccumulator, FoldDict, FoldFunction, GetDict, GetKey, GetResult, Item, Key,
    MapFunction, MapInputDict, MapOutputDict, TransientDict, TransientDictSchema, UpdateFunction,
};
use super::GleamStdlibHostProfile;
use crate::{HostProviderModule, HostRegistrationError};
use num_bigint::BigInt;

pub(super) fn host_provider<Profile>() -> Result<HostProviderModule<Profile>, HostRegistrationError>
where
    Profile: GleamStdlibHostProfile,
{
    HostProviderModule::new("gleam_stdlib", "gleam/dict")
        .and_then(HostProviderModule::with_external_type::<DictProvider<Profile>, DictSchema>)
        .and_then(
            HostProviderModule::with_external_type::<DictProvider<Profile>, TransientDictSchema>,
        )
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
