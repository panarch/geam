//! Typed registrations for creating and composing producer-owned selectors.
//!
//! These functions retain source callbacks and use the same persistent selector
//! representation as the builtin. A provider can register them under its own
//! source module without defining another external binding or selector engine.

use crate::process::schema::Down;
use crate::schema::{Selector, SelectorSchema};
use crate::selector::{Entry, Handler, Selector as SelectorValue};
use crate::{Component, GleamErlangHostProfile};
use geam_core::host::native::NativeCall;
use geam_core::host::{
    HostCall, HostCallCompletion, HostCallError, HostCallable, HostExternal, HostProvider,
    HostType, HostTypeIndex0, HostTypeList, HostTypeListEnd,
};
use geam_core::provider::advanced::NativeValue;
use std::sync::Arc;

type One<T> = HostTypeList<T, HostTypeListEnd>;
type Call<'call, Profile, Provider, Return> = HostCall<'call, Profile, Provider, Return>;
type Native<'call, Profile, Provider, Return, Target> =
    NativeCall<'call, Profile, Provider, Return, One<Target>>;

pub fn new_selector<
    'call,
    Profile: GleamErlangHostProfile,
    Provider: HostProvider<Profile>,
    A: HostType,
>(
    mut call: Call<'call, Profile, Provider, Selector<A>>,
) -> Result<HostCallCompletion<'call, Selector<A>>, HostCallError> {
    let value = call.create_external_with_binding::<Component<Profile>>(SelectorValue::default());
    Ok(call.return_value(value))
}

pub fn insert<
    'call,
    Profile: GleamErlangHostProfile,
    Provider: HostProvider<Profile>,
    A: HostType,
    B: HostType,
    C: HostType,
>(
    mut call: NativeCall<'call, Profile, Provider, Selector<A>, HostTypeList<C, One<Down>>>,
    selector: HostExternal<'call, Selector<A>>,
    key: B::Value<'call>,
    callback: HostCallable<'call, One<C>, A>,
) -> Result<HostCallCompletion<'call, Selector<A>>, HostCallError> {
    let mut selector = call
        .call()
        .external_payload_with::<Component<Profile>, SelectorSchema, One<A>>(selector)
        .clone();
    let key = call.source::<B>(key);
    let hash = call.call().native_hash(&key);
    let callback = call.owned_callable::<HostTypeIndex0, A>(callback);
    let entry = Arc::new(Entry {
        key,
        handler: Arc::new(Handler {
            native: callback.native_value().clone(),
            callback,
            mappings: im::OrdMap::new(),
        }),
    });
    let bucket = selector.entries.entry(hash).or_default();
    if let Some(index) = bucket
        .iter()
        .position(|previous| call.call().native_equal(&previous.key, &entry.key))
    {
        bucket.set(index, entry);
    } else {
        bucket.push_back(entry);
        selector.len += 1;
    }
    let value = call
        .call()
        .create_external_with_binding::<Component<Profile>>(selector);
    Ok(call.finish(value))
}

pub fn remove<
    'call,
    Profile: GleamErlangHostProfile,
    Provider: HostProvider<Profile>,
    A: HostType,
    B: HostType,
>(
    mut call: Call<'call, Profile, Provider, Selector<A>>,
    selector: HostExternal<'call, Selector<A>>,
    key: B::Value<'call>,
) -> Result<HostCallCompletion<'call, Selector<A>>, HostCallError> {
    let mut selector = call
        .external_payload_with::<Component<Profile>, SelectorSchema, One<A>>(selector)
        .clone();
    let key = call.native_value::<B>(key);
    let hash = call.native_hash(&key);
    if let Some(bucket) = selector.entries.get_mut(&hash) {
        if let Some(index) = bucket
            .iter()
            .position(|previous| call.native_equal(&previous.key, &key))
        {
            bucket.remove(index);
            selector.len -= 1;
        }
        if bucket.is_empty() {
            selector.entries.remove(&hash);
        }
    }
    let value = call.create_external_with_binding::<Component<Profile>>(selector);
    Ok(call.return_value(value))
}

pub fn merge_selector<
    'call,
    Profile: GleamErlangHostProfile,
    Provider: HostProvider<Profile>,
    A: HostType,
>(
    mut call: Call<'call, Profile, Provider, Selector<A>>,
    left: HostExternal<'call, Selector<A>>,
    right: HostExternal<'call, Selector<A>>,
) -> Result<HostCallCompletion<'call, Selector<A>>, HostCallError> {
    let mut left = call
        .external_payload_with::<Component<Profile>, SelectorSchema, One<A>>(left)
        .clone();
    let right = call
        .external_payload_with::<Component<Profile>, SelectorSchema, One<A>>(right)
        .clone();
    for (hash, entries) in &right.entries {
        let bucket = left.entries.entry(*hash).or_default();
        for entry in entries {
            if let Some(index) = bucket
                .iter()
                .position(|previous| call.native_equal(&previous.key, &entry.key))
            {
                bucket.set(index, Arc::clone(entry));
            } else {
                bucket.push_back(Arc::clone(entry));
                left.len += 1;
            }
        }
    }
    let value = call.create_external_with_binding::<Component<Profile>>(left);
    Ok(call.return_value(value))
}

pub fn map_selector<
    'call,
    Profile: GleamErlangHostProfile,
    Provider: HostProvider<Profile>,
    A: HostType,
    B: HostType,
>(
    mut call: Native<'call, Profile, Provider, Selector<A>, B>,
    selector: HostExternal<'call, Selector<B>>,
    callback: HostCallable<'call, One<B>, A>,
) -> Result<HostCallCompletion<'call, Selector<A>>, HostCallError> {
    let source = call
        .call()
        .external_payload_with::<Component<Profile>, SelectorSchema, One<B>>(selector)
        .clone();
    let callback = call.owned_callable::<HostTypeIndex0, A>(callback);
    let entries = source
        .entries
        .iter()
        .map(|(hash, entries)| {
            let mapped = entries
                .iter()
                .map(|entry| {
                    let mut mappings = entry.handler.mappings.clone();
                    mappings.insert(mappings.len(), callback.clone());
                    Arc::new(Entry {
                        key: entry.key.clone(),
                        handler: Arc::new(Handler {
                            native: NativeValue::unary_closure(
                                "gleam_erlang/gleam/erlang/process:map_selector/handler",
                                [
                                    callback.native_value().clone(),
                                    entry.handler.native.clone(),
                                ],
                            ),
                            callback: entry.handler.callback.clone(),
                            mappings,
                        }),
                    })
                })
                .collect();
            (*hash, mapped)
        })
        .collect();
    let value = call
        .call()
        .create_external_with_binding::<Component<Profile>>(SelectorValue {
            entries,
            len: source.len,
        });
    Ok(call.finish(value))
}
