use super::schema::{Down, NamedSubject, OrdinarySubject, Subject};
use super::{A, B, C, Call, Native, One, receive_deadline};
use crate::execution::{Message, Reason, Scan};
use crate::schema::Selector;
use crate::selector::{Entry, Handler, Selector as SelectorValue};
use crate::{Component, GleamErlangHostProfile};
use futures_channel::oneshot;
use futures_util::future::{Either, select as select_future};
use geam_core::execution::ExecutionUnitId;
use geam_core::host::native::{NativeCall, NativeValues};
use geam_core::host::{
    HostCallCompletion, HostCallContinuation, HostCallError, HostCallable, HostCustom,
    HostExecutionContext, HostExecutionError, HostExternal, HostProfile, HostType, HostTypeIndex0,
    HostTypeList, HostTypeSequence, HostValue,
};
use geam_core::provider::advanced::{NativeKind, NativeValue};
use geam_stdlib::provider_support::GleamResult;
use num_bigint::BigInt;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

enum Filter<Profile: HostProfile> {
    Subject(NativeValue),
    Selector(SelectorValue<Profile>),
}

impl<Profile: HostProfile> Clone for Filter<Profile> {
    fn clone(&self) -> Self {
        match self {
            Self::Subject(tag) => Self::Subject(tag.clone()),
            Self::Selector(selector) => Self::Selector(selector.clone()),
        }
    }
}

struct Selected<Profile: HostProfile> {
    input: NativeValue,
    handler: Option<Arc<Handler<Profile>>>,
}

enum Next<Profile: HostProfile> {
    Selected(Selected<Profile>),
    Scanning(Scan),
    Waiting(oneshot::Receiver<()>, u64),
}

enum Cursor {
    After(u64),
    Resume(Scan),
}

type Sleep = Pin<Box<dyn Future<Output = ()> + Send>>;

pub(super) fn receive<'call, Profile: GleamErlangHostProfile>(
    mut call: Native<'call, Profile, GleamResult<A, ()>, GleamResult<A, ()>>,
    subject: HostCustom<'call, Subject<A>>,
    delay: BigInt,
) -> Result<HostCallContinuation<'call, GleamResult<A, ()>>, HostCallError> {
    let filter = Filter::Subject(subject_tag(call.call(), subject));
    receive_filtered(call, filter, delay)
}

pub(super) fn receive_forever<'call, Profile: GleamErlangHostProfile>(
    mut call: Native<'call, Profile, A, A>,
    subject: HostCustom<'call, Subject<A>>,
) -> Result<HostCallContinuation<'call, A>, HostCallError> {
    let filter = Filter::Subject(subject_tag(call.call(), subject));
    receive_filtered_forever(call, filter)
}

fn subject_tag<'call, Profile: GleamErlangHostProfile, Return: HostType>(
    call: &mut Call<'call, Profile, Return>,
    subject: HostCustom<'call, Subject<A>>,
) -> NativeValue {
    if let Some((_, (tag, ()))) = call.custom_fields::<OrdinarySubject<A>>(subject) {
        call.external_payload::<geam_stdlib::provider_support::DynamicSchema, geam_core::host::HostTypeListEnd>(tag).native_value().clone()
    } else {
        let (name, ()) = call.provider_remaining_custom_fields::<NamedSubject<A>>(subject);
        NativeValue::symbol(call.external_payload(name).clone())
    }
}

pub(super) fn select<'call, Profile: GleamErlangHostProfile>(
    mut call: Native<'call, Profile, GleamResult<A, ()>, GleamResult<A, ()>>,
    selector: HostExternal<'call, Selector<A>>,
    delay: BigInt,
) -> Result<HostCallContinuation<'call, GleamResult<A, ()>>, HostCallError> {
    let filter = Filter::Selector(call.call().external_payload(selector).clone());
    receive_filtered(call, filter, delay)
}

pub(super) fn select_forever<'call, Profile: GleamErlangHostProfile>(
    mut call: Native<'call, Profile, A, A>,
    selector: HostExternal<'call, Selector<A>>,
) -> Result<HostCallContinuation<'call, A>, HostCallError> {
    let filter = Filter::Selector(call.call().external_payload(selector).clone());
    receive_filtered_forever(call, filter)
}

fn receive_filtered<'call, Profile: GleamErlangHostProfile>(
    call: Native<'call, Profile, GleamResult<A, ()>, GleamResult<A, ()>>,
    filter: Filter<Profile>,
    delay: BigInt,
) -> Result<HostCallContinuation<'call, GleamResult<A, ()>>, HostCallError> {
    call.with_execution_unit(|mut call, unit| {
        let pid = unit.id();
        let deadline = receive_deadline(call.call().clock().now(), delay)?;
        let timeout = call.call().clock().sleep_until(deadline);
        let first = check(call.call(), pid, &filter, Cursor::After(0))?;
        Ok(call.resume::<HostTypeIndex0>(move |context| {
            Box::pin(async move { timed(&context, pid, filter, first, timeout).await })
        }))
    })
}

fn receive_filtered_forever<'call, Profile: GleamErlangHostProfile>(
    call: Native<'call, Profile, A, A>,
    filter: Filter<Profile>,
) -> Result<HostCallContinuation<'call, A>, HostCallError> {
    call.with_execution_unit(|mut call, unit| {
        let pid = unit.id();
        let first = check(call.call(), pid, &filter, Cursor::After(0))?;
        Ok(call.resume::<HostTypeIndex0>(move |context| {
            Box::pin(async move { next(&context, pid, filter, first).await })
        }))
    })
}

async fn timed<Profile: GleamErlangHostProfile, Targets: HostTypeSequence>(
    context: &HostExecutionContext<'_, Profile, Component<Profile>, Targets>,
    pid: ExecutionUnitId,
    filter: Filter<Profile>,
    mut state: Next<Profile>,
    mut timeout: Sleep,
) -> Result<NativeValue, HostExecutionError> {
    loop {
        match state {
            Next::Selected(selected) => {
                let value = invoke(context, selected).await?;
                return Ok(NativeValue::tuple([NativeValue::symbol("ok"), value]));
            }
            Next::Scanning(scan) => {
                let filter = filter.clone();
                state = context
                    .with_call(move |mut call| check(&mut call, pid, &filter, Cursor::Resume(scan)))
                    .await??;
            }
            Next::Waiting(waiter, after) => {
                // Finish the queued snapshot before considering its timeout,
                // including receive-after-zero. Selected callback time is separate.
                match select_future(timeout.as_mut(), waiter).await {
                    Either::Left(_) => {
                        return Ok(NativeValue::tuple([
                            NativeValue::symbol("error"),
                            NativeValue::symbol("nil"),
                        ]));
                    }
                    Either::Right((ready, _)) => {
                        ready.map_err(|_| HostExecutionError::Cancelled)?;
                        let filter = filter.clone();
                        state = context
                            .with_call(move |mut call| {
                                check(&mut call, pid, &filter, Cursor::After(after))
                            })
                            .await??;
                    }
                }
            }
        }
    }
}

async fn next<Profile: GleamErlangHostProfile, Targets: HostTypeSequence>(
    context: &HostExecutionContext<'_, Profile, Component<Profile>, Targets>,
    pid: ExecutionUnitId,
    filter: Filter<Profile>,
    mut state: Next<Profile>,
) -> Result<NativeValue, HostExecutionError> {
    loop {
        match state {
            Next::Selected(selected) => return invoke(context, selected).await,
            Next::Scanning(scan) => {
                let filter = filter.clone();
                state = context
                    .with_call(move |mut call| check(&mut call, pid, &filter, Cursor::Resume(scan)))
                    .await??;
            }
            Next::Waiting(waiter, after) => {
                waiter.await.map_err(|_| HostExecutionError::Cancelled)?;
                let filter = filter.clone();
                state = context
                    .with_call(move |mut call| check(&mut call, pid, &filter, Cursor::After(after)))
                    .await??;
            }
        }
    }
}

async fn invoke<Profile: GleamErlangHostProfile, Targets: HostTypeSequence>(
    context: &HostExecutionContext<'_, Profile, Component<Profile>, Targets>,
    selected: Selected<Profile>,
) -> Result<NativeValue, HostExecutionError> {
    let Some(handler) = selected.handler else {
        return Ok(selected.input);
    };
    let mut value = handler.callback.invoke(context, selected.input).await?;
    for mapping in handler.mappings.values() {
        value = mapping.invoke(context, value).await?;
    }
    Ok(value)
}

fn check<Profile: GleamErlangHostProfile, Return: HostType>(
    call: &mut Call<'_, Profile, Return>,
    pid: ExecutionUnitId,
    filter: &Filter<Profile>,
    cursor: Cursor,
) -> Result<Next<Profile>, HostCallError> {
    call.with_native_values(|state, values| {
        let mailbox = Profile::erlang_execution(state)
            .mailbox(pid)
            .ok_or_else(|| geam_core::HostFailure::new("process mailbox is closed"))?;
        let mut scan = match cursor {
            Cursor::After(after) => mailbox.scan(after),
            Cursor::Resume(scan) => scan,
        };
        for _ in 0..64 {
            let next = mailbox.next(&mut scan);
            let Some((position, message)) = next else {
                let (waiter, after) = mailbox.wait(scan);
                return Ok(Next::Waiting(waiter, after));
            };
            let value = message_value(values, &message);
            let selected = match filter {
                Filter::Subject(tag) => {
                    match (value.kind(), value.index(0), value.index(1), value.len()) {
                        (NativeKind::Tuple, Some(received_tag), Some(input), Some(2))
                            if values.equal(tag, &received_tag) =>
                        {
                            Some(Selected {
                                input,
                                handler: None,
                            })
                        }
                        _ => None,
                    }
                }
                Filter::Selector(selector) => choose(values, selector, value)?,
            };
            if let Some(selected) = selected {
                mailbox.remove(position);
                return Ok(Next::Selected(selected));
            }
        }
        Ok(Next::Scanning(scan))
    })
}

fn choose<Profile: HostProfile>(
    values: NativeValues<'_>,
    selector: &SelectorValue<Profile>,
    value: NativeValue,
) -> Result<Option<Selected<Profile>>, HostCallError> {
    if let (NativeKind::Tuple, Some(tag), Some(arity)) = (value.kind(), value.index(0), value.len())
    {
        if let (Some("DOWN"), 5, Some(reference)) =
            (tag.as_symbol().as_deref(), arity, value.index(1))
            && let Some(handler) = find(values, selector, &reference)
        {
            let input = down_message(&value)?;
            return Ok(Some(Selected {
                input,
                handler: Some(handler),
            }));
        }
        let key = NativeValue::tuple([tag, values.integer(arity.into())]);
        if let Some(handler) = find(values, selector, &key) {
            return Ok(Some(Selected {
                input: value,
                handler: Some(handler),
            }));
        }
    }
    Ok(
        find(values, selector, &NativeValue::symbol("anything")).map(|handler| Selected {
            input: value,
            handler: Some(handler),
        }),
    )
}

fn find<Profile: HostProfile>(
    values: NativeValues<'_>,
    selector: &SelectorValue<Profile>,
    key: &NativeValue,
) -> Option<Arc<Handler<Profile>>> {
    selector
        .entries
        .get(&values.hash(key))?
        .iter()
        .find(|entry| values.equal(&entry.key, key))
        .map(|entry| Arc::clone(&entry.handler))
}

fn message_value(values: NativeValues<'_>, message: &Message) -> NativeValue {
    match message {
        Message::Source(value) => value.clone(),
        Message::Down {
            reference,
            pid,
            reason,
            ..
        } => NativeValue::tuple([
            NativeValue::symbol("DOWN"),
            reference.clone(),
            NativeValue::symbol("process"),
            pid.clone(),
            reason_value(values, reason),
        ]),
        Message::Exit { pid, reason } => NativeValue::tuple([
            NativeValue::symbol("EXIT"),
            pid.clone(),
            reason_value(values, reason),
        ]),
    }
}

fn reason_value(values: NativeValues<'_>, reason: &Reason) -> NativeValue {
    match reason {
        Reason::Normal => NativeValue::symbol("normal"),
        Reason::Killed => NativeValue::symbol("killed"),
        Reason::Native(value) => value.clone(),
        Reason::Failure(error) => NativeValue::tuple([
            NativeValue::symbol("geam_execution_error"),
            values.string(error.to_string().into()),
        ]),
    }
}

pub(super) fn exit_reason(value: NativeValue) -> NativeValue {
    match value.as_symbol().as_deref() {
        Some("normal") | Some("killed") => value,
        _ => NativeValue::tuple([NativeValue::symbol("abnormal"), value]),
    }
}

pub(super) fn down_message(value: &NativeValue) -> Result<NativeValue, HostCallError> {
    let kind = value.index(2).and_then(|kind| kind.as_symbol());
    let (
        NativeKind::Tuple,
        Some("DOWN"),
        Some(reference),
        Some(kind @ ("process" | "port")),
        Some(pid),
        Some(reason),
        Some(5),
    ) = (
        value.kind(),
        value.index(0).and_then(|tag| tag.as_symbol()).as_deref(),
        value.index(1),
        kind.as_deref(),
        value.index(3),
        value.index(4),
        value.len(),
    )
    else {
        return Err(geam_core::HostFailure::new("expected a DOWN message").into());
    };
    let tag = if kind == "process" {
        "process_down"
    } else {
        "port_down"
    };
    Ok(NativeValue::tuple([
        NativeValue::symbol(tag),
        reference,
        pid,
        exit_reason(reason),
    ]))
}

pub(super) fn new_selector<'call, Profile: GleamErlangHostProfile>(
    mut call: Call<'call, Profile, Selector<A>>,
) -> Result<HostCallCompletion<'call, Selector<A>>, HostCallError> {
    let value = call.create_external(SelectorValue::default());
    Ok(call.return_value(value))
}

pub(super) fn insert<'call, Profile: GleamErlangHostProfile>(
    mut call: NativeCall<
        'call,
        Profile,
        Component<Profile>,
        Selector<A>,
        HostTypeList<C, One<Down>>,
    >,
    selector: HostExternal<'call, Selector<A>>,
    key: HostValue<'call, B>,
    callback: HostCallable<'call, One<C>, A>,
) -> Result<HostCallCompletion<'call, Selector<A>>, HostCallError> {
    let mut selector = call.call().external_payload(selector).clone();
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
    let value = call.call().create_external(selector);
    Ok(call.finish(value))
}

pub(super) fn remove<'call, Profile: GleamErlangHostProfile>(
    mut call: Call<'call, Profile, Selector<A>>,
    selector: HostExternal<'call, Selector<A>>,
    key: HostValue<'call, B>,
) -> Result<HostCallCompletion<'call, Selector<A>>, HostCallError> {
    let mut selector = call.external_payload(selector).clone();
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
    let value = call.create_external(selector);
    Ok(call.return_value(value))
}

pub(super) fn merge_selector<'call, Profile: GleamErlangHostProfile>(
    mut call: Call<'call, Profile, Selector<A>>,
    left: HostExternal<'call, Selector<A>>,
    right: HostExternal<'call, Selector<A>>,
) -> Result<HostCallCompletion<'call, Selector<A>>, HostCallError> {
    let mut left = call.external_payload(left).clone();
    let right = call.external_payload(right).clone();
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
    let value = call.create_external(left);
    Ok(call.return_value(value))
}

pub(super) fn map_selector<'call, Profile: GleamErlangHostProfile>(
    mut call: Native<'call, Profile, Selector<A>, B>,
    selector: HostExternal<'call, Selector<B>>,
    callback: HostCallable<'call, One<B>, A>,
) -> Result<HostCallCompletion<'call, Selector<A>>, HostCallError> {
    let source = call.call().external_payload(selector).clone();
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
    let value = call.call().create_external(SelectorValue {
        entries,
        len: source.len,
    });
    Ok(call.finish(value))
}

#[cfg(test)]
mod tests {
    use super::{A, B, C, Call, Down, Native, One, Selector, Subject, find};
    use crate::{Component, GleamErlangProfile, GleamErlangRunState, GleamErlangStores};
    use ecow::EcoString;
    use geam_core::host::{
        HostCallCompletion, HostCallContinuation, HostCallError, HostCustom, HostExternal,
        HostExternalBinding, HostExternalEquality, HostExternalHashing, HostExternalInspection,
        HostExternalSchema, HostExternalStorage, HostExternalStore, HostExternalType,
        HostFunctionType, HostProviderModule, HostProviderSet, HostTypeList,
    };
    use geam_core::provider::advanced::NativeValue;
    use geam_core::{
        HostedExecution, ModuleSource, PackageSource, compile_typed_host_program, plan_host_program,
    };
    use geam_stdlib::provider_support::GleamResult;
    use num_bigint::BigInt;

    struct KeySchema;
    struct KeyStorage;
    type Key = HostExternalType<KeySchema>;

    impl HostExternalSchema for KeySchema {
        const PACKAGE: &'static str = "application";
        const MODULE: &'static str = "main";
        const NAME: &'static str = "Key";
        const PARAMETER_COUNT: usize = 0;
    }

    impl HostExternalBinding<GleamErlangProfile, KeySchema> for Component<GleamErlangProfile> {
        type Storage = KeyStorage;
    }

    impl HostExternalStorage<GleamErlangProfile, KeySchema> for KeyStorage {
        type Payload = EcoString;
        fn store(stores: &GleamErlangStores) -> &HostExternalStore<EcoString> {
            &stores.erlang.names
        }
        fn source_equal(_: &HostExternalEquality<'_>, left: &EcoString, right: &EcoString) -> bool {
            left == right
        }
        fn source_hash(_: &HostExternalHashing<'_>, _: &EcoString) -> u64 {
            0
        }
        fn inspect(_: &HostExternalInspection<'_>, value: &EcoString) -> EcoString {
            format!("Key(\"{value}\")").into()
        }
    }

    fn key<'call>(
        mut call: Call<'call, GleamErlangProfile, Key>,
        text: EcoString,
    ) -> Result<HostCallCompletion<'call, Key>, HostCallError> {
        let key = call.create_external(text);
        Ok(call.return_value(key))
    }

    fn check_lookup<'call>(
        mut call: Call<'call, GleamErlangProfile, A>,
        selector: HostExternal<'call, Selector<A>>,
        one: HostExternal<'call, Key>,
        two: HostExternal<'call, Key>,
        missing: HostExternal<'call, Key>,
        returned: geam_core::host::HostValue<'call, A>,
    ) -> Result<HostCallCompletion<'call, A>, HostCallError> {
        assert_eq!(call.inspect::<Key>(one), "Key(\"one\")");
        assert_eq!(call.inspect::<Key>(two), "Key(\"two\")");
        assert!(!call.equal::<Key>(one, two));
        assert!(call.equal::<Key>(one, one));
        let one = call.native_value::<Key>(one);
        let two = call.native_value::<Key>(two);
        let missing = call.native_value::<Key>(missing);
        assert_eq!(call.native_hash(&one), call.native_hash(&two));
        assert_eq!(call.native_hash(&two), call.native_hash(&missing));
        let selector = call.external_payload(selector);
        assert_eq!(selector.len, 2);
        assert_eq!(selector.entries.len(), 1);
        assert!(find(call.native_values(), &selector, &one).is_some());
        assert!(find(call.native_values(), &selector, &two).is_some());
        assert!(find(call.native_values(), &selector, &missing).is_none());
        for message in [
            NativeValue::symbol("unselected"),
            NativeValue::tuple([]),
            NativeValue::tuple([one.clone()]),
            NativeValue::tuple([one.clone(), two.clone()]),
        ] {
            assert!(
                super::choose(call.native_values(), &selector, message)
                    .unwrap()
                    .is_none()
            );
        }
        let malformed_down = NativeValue::tuple([
            NativeValue::symbol("DOWN"),
            one.clone(),
            NativeValue::symbol("unknown_kind"),
            two.clone(),
            NativeValue::symbol("normal"),
        ]);
        assert_eq!(
            super::choose(call.native_values(), &selector, malformed_down.clone())
                .err()
                .unwrap()
                .to_string(),
            "expected a DOWN message"
        );
        let down = NativeValue::tuple([
            NativeValue::symbol("DOWN"),
            one,
            NativeValue::symbol("process"),
            two,
            NativeValue::symbol("normal"),
        ]);
        assert_eq!(
            super::choose(call.native_values(), &selector, down)
                .unwrap()
                .unwrap()
                .input
                .index(0)
                .unwrap()
                .as_symbol()
                .as_deref(),
            Some("process_down")
        );
        let original_handler = selector.entries.values().next().unwrap()[0].handler.clone();
        for (key, message) in [
            (
                NativeValue::tuple([
                    NativeValue::symbol("record"),
                    call.native_value::<BigInt>(2.into()),
                ]),
                NativeValue::tuple([
                    NativeValue::symbol("record"),
                    call.native_value::<BigInt>(42.into()),
                ]),
            ),
            (
                NativeValue::symbol("anything"),
                NativeValue::symbol("ordinary"),
            ),
        ] {
            let hash = call.native_hash(&key);
            let entry = std::sync::Arc::new(crate::selector::Entry {
                key,
                handler: original_handler.clone(),
            });
            let filter = crate::selector::Selector {
                entries: [(hash, [entry].into_iter().collect())]
                    .into_iter()
                    .collect(),
                len: 1,
            };
            let selected = super::choose(call.native_values(), &filter, message.clone())
                .unwrap()
                .unwrap();
            assert!(call.native_equal(&selected.input, &message));
        }
        let filter = super::Filter::Selector(selector.clone());
        let pid = call.execution_unit().unwrap().id();
        call.execution_state()
            .send(pid, crate::execution::Message::Source(malformed_down));
        let scan = call.execution_state().mailbox(pid).unwrap().scan(0);
        assert_eq!(
            super::check(&mut call, pid, &filter, super::Cursor::Resume(scan))
                .err()
                .unwrap()
                .to_string(),
            "expected a DOWN message"
        );
        Ok(call.return_value(returned))
    }

    #[test]
    fn down_envelopes_validate_shape_without_fabricating_typed_identities() {
        crate::test_support::with_contexts(
            |context| {
                // These fields are untrusted native data. This parser retains
                // them; the sealed callback converter checks their source types.
                let reference = NativeValue::tuple([NativeValue::symbol("reference_field")]);
                let identity = NativeValue::tuple([NativeValue::symbol("identity_field")]);
                for (kind, tag, reason, converted_reason) in [
                    (
                        "process",
                        "process_down",
                        "normal",
                        NativeValue::symbol("normal"),
                    ),
                    ("port", "port_down", "killed", NativeValue::symbol("killed")),
                    (
                        "process",
                        "process_down",
                        "failed",
                        NativeValue::tuple([
                            NativeValue::symbol("abnormal"),
                            NativeValue::symbol("failed"),
                        ]),
                    ),
                ] {
                    let fields = [
                        NativeValue::symbol("DOWN"),
                        reference.clone(),
                        NativeValue::symbol(kind),
                        identity.clone(),
                        NativeValue::symbol(reason),
                    ];
                    let message = NativeValue::tuple(fields.clone());
                    let result = super::down_message(&message).unwrap();
                    let expected = NativeValue::tuple([
                        NativeValue::symbol(tag),
                        reference.clone(),
                        identity.clone(),
                        converted_reason,
                    ]);
                    assert!(result.source_equal(context, &expected));
                    for count in 0..5 {
                        assert!(
                            super::down_message(&NativeValue::tuple(
                                fields[..count].iter().cloned()
                            ))
                            .is_err()
                        );
                    }
                    let extra = NativeValue::tuple(
                        fields.into_iter().chain([NativeValue::symbol("extra")]),
                    );
                    assert!(super::down_message(&extra).is_err());
                }
                for (tag, kind) in [("EXIT", "process"), ("DOWN", "unknown")] {
                    let message = NativeValue::tuple([
                        NativeValue::symbol(tag),
                        reference.clone(),
                        NativeValue::symbol(kind),
                        identity.clone(),
                        NativeValue::symbol("normal"),
                    ]);
                    assert!(super::down_message(&message).is_err());
                }
                assert!(super::down_message(&NativeValue::symbol("DOWN")).is_err());
            },
            |_| {},
            |_| {},
        );
    }

    #[test]
    fn selector_updates_and_native_lookup_distinguish_colliding_keys() {
        let selectors = HostProviderModule::<GleamErlangProfile>::new(
            "gleam_erlang", "gleam/erlang/process",
        ).unwrap()
            .with_external_type::<Component<GleamErlangProfile>, crate::schema::SelectorSchema>().unwrap()
            .with_external_type::<Component<GleamErlangProfile>, crate::schema::PidSchema>().unwrap()
            .with_external_type::<Component<GleamErlangProfile>, crate::schema::MonitorSchema>().unwrap()
            .with_scoped_function::<Component<GleamErlangProfile>, (), Selector<A>, _>(
                "new_selector", super::new_selector,
            ).unwrap()
            .with_native_function::<Component<GleamErlangProfile>,
                (Selector<A>, B, HostFunctionType<One<C>, A>), Selector<A>, HostTypeList<C, One<Down>>, _>(
                "insert_selector_handler", super::super::native_rules(), super::insert,
            ).unwrap()
            .with_scoped_function::<Component<GleamErlangProfile>, (Selector<A>, B), Selector<A>, _>(
                "remove_selector_handler", super::remove,
            ).unwrap()
            .with_scoped_function::<Component<GleamErlangProfile>, (Selector<A>, Selector<A>), Selector<A>, _>(
                "merge_selector", super::merge_selector,
            ).unwrap();
        let provider = HostProviderModule::<GleamErlangProfile>::new("application", "main")
            .unwrap()
            .with_external_type::<Component<GleamErlangProfile>, KeySchema>()
            .unwrap()
            .with_scoped_function::<Component<GleamErlangProfile>, (EcoString,), Key, _>(
                "key", key,
            )
            .unwrap()
            .with_scoped_function::<
                Component<GleamErlangProfile>,
                (Selector<A>, Key, Key, Key, A),
                A,
                _,
            >("check_lookup", check_lookup)
            .unwrap();
        let typed = compile_typed_host_program(
            "application",
            "main",
            [
                PackageSource::new(
                    "gleam_stdlib",
                    Vec::<String>::new(),
                    [ModuleSource::new(
                        "gleam/dynamic",
                        "dynamic.gleam",
                        "pub type Dynamic",
                    )],
                ),
                PackageSource::new(
                    "gleam_erlang",
                    ["gleam_stdlib"],
                    [
                        ModuleSource::new("gleam/erlang/port", "port.gleam", "pub type Port"),
                        ModuleSource::new("gleam/erlang/atom", "atom.gleam", "pub type Atom"),
                        ModuleSource::new("gleam/erlang/reference", "reference.gleam", "pub type Reference"),
                        ModuleSource::new(
                            "gleam/erlang/process",
                            "process.gleam",
                            r#"
import gleam/dynamic.{type Dynamic}
import gleam/erlang/port.{type Port}
pub type Pid
pub type Monitor
pub type Selector(a)
pub type ExitReason { Normal Killed Abnormal(reason: Dynamic) }
pub type Down {
  ProcessDown(monitor: Monitor, pid: Pid, reason: ExitReason)
  PortDown(monitor: Monitor, port: Port, reason: ExitReason)
}
@external(erlang, "host", "new_selector")
pub fn new_selector() -> Selector(a)
@external(erlang, "host", "insert_selector_handler")
pub fn insert_selector_handler(s: Selector(a), key: b, callback: fn(c) -> a) -> Selector(a)
@external(erlang, "host", "remove_selector_handler")
pub fn remove_selector_handler(s: Selector(a), key: b) -> Selector(a)
@external(erlang, "host", "merge_selector")
pub fn merge_selector(left: Selector(a), right: Selector(a)) -> Selector(a)
"#,
                        ),
                    ],
                ),
                PackageSource::new(
                    "application",
                    ["gleam_erlang"],
                    [ModuleSource::new(
                        "main",
                        "main.gleam",
                        r#"
import gleam/erlang/process as p
pub type Key
@external(erlang, "host", "key") fn key(text: String) -> Key
@external(erlang, "host", "check_lookup")
fn check_lookup(s: p.Selector(a), one: Key, two: Key, missing: Key, returned: a) -> a
fn first(value: Int) { value + 1 }
fn second(value: Int) { value * 2 }
pub fn main() {
  let one = key("one")
  let two = key("two")
  let three = key("three")
  let empty = p.new_selector()
  let left = p.insert_selector_handler(empty, one, first)
  let right = p.insert_selector_handler(empty, two, second)
  let both = p.insert_selector_handler(left, two, second)
  let reversed = p.insert_selector_handler(right, one, first)
  let different = p.insert_selector_handler(left, three, second)
  check_lookup(both, one, two, three, 42)
  #(both == reversed,
    both != different,
    left != p.insert_selector_handler(empty, "other", first),
    p.remove_selector_handler(empty, one) == empty,
    p.remove_selector_handler(both, three) == both,
    p.remove_selector_handler(both, one) == right,
    p.remove_selector_handler(left, one) == empty,
    p.merge_selector(left, right) == both,
    p.merge_selector(both, right) == both,
    p.insert_selector_handler(both, one, second)
      == p.merge_selector(both, p.insert_selector_handler(empty, one, second)),
    left != both)
}
"#,
                    )],
                ),
            ],
            HostProviderSet::from_providers([
                selectors,
                super::super::port_provider().unwrap(),
                HostProviderModule::new("gleam_erlang", "gleam/erlang/atom")
                    .unwrap()
                    .with_external_type::<Component<GleamErlangProfile>, crate::schema::AtomSchema>()
                    .unwrap(),
                HostProviderModule::new("gleam_erlang", "gleam/erlang/reference")
                    .unwrap()
                    .with_external_type::<Component<GleamErlangProfile>, crate::schema::ReferenceSchema>()
                    .unwrap(),
                HostProviderModule::new("gleam_stdlib", "gleam/dynamic")
                    .unwrap()
                    .with_external_type::<Component<GleamErlangProfile>, geam_stdlib::provider_support::DynamicSchema>()
                    .unwrap(),
                provider,
            ])
            .unwrap(),
        )
        .unwrap();
        let plan = plan_host_program(typed).unwrap();
        let mut execution = HostedExecution::try_from_module_plan(plan).unwrap();
        let host = crate::execution_fixture::TestHost::default();
        let mut state = GleamErlangRunState {
            stdlib: geam_stdlib::GleamStdlibRunState::from_seed([0; 32]),
            erlang: crate::Configuration::default(),
        };
        let mut echo = Vec::new();
        let result = host
            .block_on(execution.run_main(&host, &mut state, &mut echo))
            .unwrap();
        assert_eq!(
            result.inspect().to_string(),
            "#(True, True, True, True, True, True, True, True, True, True, True)"
        );
        assert!(echo.is_empty());
    }

    #[test]
    fn synchronous_native_code_cannot_reopen_its_terminated_mailbox() {
        use crate::schema::{MonitorSchema, Name, NameSchema, PidSchema, SelectorSchema};
        use geam_core::embedding::{CallError, FunctionDeclaration, HostedModuleBuilder};

        let process = HostProviderModule::<GleamErlangProfile>::new(
            "gleam_erlang", "gleam/erlang/process",
        ).unwrap()
            .with_external_type::<Component<GleamErlangProfile>, PidSchema>().unwrap()
            .with_external_type::<Component<GleamErlangProfile>, NameSchema>().unwrap()
            .with_external_type::<Component<GleamErlangProfile>, MonitorSchema>().unwrap()
            .with_external_type::<Component<GleamErlangProfile>, SelectorSchema>().unwrap()
            .with_scoped_function::<Component<GleamErlangProfile>, (EcoString,), Name<A>, _>(
                "new_name", super::super::new_name,
            ).unwrap()
            .with_scoped_function::<Component<GleamErlangProfile>, (), Selector<A>, _>(
                "new_selector", super::new_selector,
            ).unwrap()
            .with_resumable_native_function::<Component<GleamErlangProfile>, (Subject<A>, BigInt, bool), GleamResult<A, ()>, One<GleamResult<A, ()>>, _>(
                "late_receive", super::super::native_rules(), late_receive,
            ).unwrap()
            .with_resumable_native_function::<Component<GleamErlangProfile>, (Subject<A>, bool), A, One<A>, _>(
                "late_receive_forever", super::super::native_rules(), late_receive_forever,
            ).unwrap()
            .with_resumable_native_function::<Component<GleamErlangProfile>, (Selector<A>, BigInt), GleamResult<A, ()>, One<GleamResult<A, ()>>, _>(
                "late_select", super::super::native_rules(), late_select,
            ).unwrap()
            .with_resumable_native_function::<Component<GleamErlangProfile>, (Selector<A>,), A, One<A>, _>(
                "late_select_forever", super::super::native_rules(), late_select_forever,
            ).unwrap()
            .with_scoped_function::<Component<GleamErlangProfile>, (bool,), (), _>(
                "late_flush", late_flush,
            ).unwrap();
        let typed = compile_typed_host_program(
            "gleam_erlang", "gleam/erlang/process",
            [
                PackageSource::new("gleam_stdlib", Vec::<String>::new(), [
                    ModuleSource::new("gleam/dynamic", "dynamic.gleam", "pub type Dynamic"),
                ]),
                PackageSource::new("gleam_erlang", ["gleam_stdlib"], [
                    ModuleSource::new("gleam/erlang/atom", "atom.gleam", "pub type Atom"),
                    ModuleSource::new("gleam/erlang/reference", "reference.gleam", "pub type Reference"),
                    ModuleSource::new("gleam/erlang/process", "process.gleam", r#"
import gleam/dynamic.{type Dynamic}
pub type Pid
pub type Name(a)
pub type Selector(a)
pub type Monitor
pub opaque type Subject(a) {
  Subject(owner: Pid, tag: Dynamic)
  NamedSubject(name: Name(a))
}
@external(erlang, "host", "new_name") fn new_name(prefix: String) -> Name(a)
@external(erlang, "host", "new_selector") fn new_selector() -> Selector(a)
@external(erlang, "fixture", "late_receive")
fn late_receive(subject: Subject(a), delay: Int, cancel: Bool) -> Result(a, Nil)
@external(erlang, "fixture", "late_receive_forever")
fn late_receive_forever(subject: Subject(a), cancel: Bool) -> a
@external(erlang, "fixture", "late_select")
fn late_select(selector: Selector(a), delay: Int) -> Result(a, Nil)
@external(erlang, "fixture", "late_select_forever")
fn late_select_forever(selector: Selector(a)) -> a
@external(erlang, "fixture", "late_flush") fn late_flush(cancel: Bool) -> Nil
pub fn check(kind: Int, cancel: Bool) {
  case kind {
    0 -> { let assert Ok(42) = late_receive(NamedSubject(new_name("late")), 0, cancel) Nil }
    1 -> { let assert 42 = late_receive_forever(NamedSubject(new_name("late")), cancel) Nil }
    2 -> { let _: Result(Int, Nil) = late_select(new_selector(), 0) Nil }
    4 -> { let _: Result(Int, Nil) = late_receive(NamedSubject(new_name("late")), -1, cancel) Nil }
    5 -> late_flush(cancel)
    _ -> { let _: Int = late_select_forever(new_selector()) Nil }
  }
  echo "received"
  Nil
}
"#),
                ]),
            ],
            HostProviderSet::from_providers([
                process,
                HostProviderModule::new("gleam_erlang", "gleam/erlang/atom").unwrap()
                    .with_external_type::<Component<GleamErlangProfile>, crate::schema::AtomSchema>().unwrap(),
                HostProviderModule::new("gleam_erlang", "gleam/erlang/reference").unwrap()
                    .with_external_type::<Component<GleamErlangProfile>, crate::schema::ReferenceSchema>().unwrap(),
                HostProviderModule::new("gleam_stdlib", "gleam/dynamic").unwrap()
                    .with_external_type::<Component<GleamErlangProfile>, geam_stdlib::provider_support::DynamicSchema>().unwrap(),
            ]).unwrap(),
        ).unwrap();
        let (builder, check) = HostedModuleBuilder::<GleamErlangProfile>::new(typed)
            .unwrap()
            .function(FunctionDeclaration::<(BigInt, bool), ()>::new("check"))
            .unwrap();
        let mut module = builder.seal().unwrap();
        let host = crate::execution_fixture::TestHost::default();
        let mut state = GleamErlangRunState {
            stdlib: geam_stdlib::GleamStdlibRunState::from_seed([0; 32]),
            erlang: crate::Configuration::default(),
        };
        let mut echo = Vec::new();
        host.block_on(
            module.with_execution(&host, &mut state, &mut echo, async |scope| {
                for kind in [0, 1, 5] {
                    assert_eq!(scope.call(&check, (kind.into(), false)).await, Ok(()));
                }
                assert_eq!(scope.call(&check, (4.into(), false)).await.unwrap_err().to_string(),
                    "host function gleam_erlang::gleam/erlang/process.late_receive failed: receive timeout must be between 0 and 4294967295 milliseconds");
                for kind in [0, 1, 2, 3, 5] {
                    assert_eq!(
                        scope.call(&check, (kind.into(), true)).await,
                        Err(CallError::Cancelled)
                    );
                }
            }),
        )
        .unwrap();
        assert_eq!(
            echo.iter()
                .map(|output| output.value().inspect().to_string())
                .collect::<Vec<_>>(),
            ["\"received\"", "\"received\"", "\"received\""]
        );
    }

    fn late_receive<'call>(
        mut call: Native<'call, GleamErlangProfile, GleamResult<A, ()>, GleamResult<A, ()>>,
        subject: HostCustom<'call, Subject<A>>,
        delay: BigInt,
        cancel: bool,
    ) -> Result<HostCallContinuation<'call, GleamResult<A, ()>>, HostCallError> {
        let unit = call.call().execution_unit().unwrap();
        if !cancel {
            let (name, ()) = call
                .call()
                .custom_fields::<super::NamedSubject<A>>(subject)
                .unwrap();
            let tag = NativeValue::symbol(call.call().external_payload(name).clone());
            let message = NativeValue::tuple([tag, call.call().native_value::<BigInt>(42.into())]);
            call.call()
                .execution_state()
                .send(unit.id(), crate::execution::Message::Source(message));
            return super::receive(call, subject, delay);
        }
        call.call()
            .execution_state()
            .terminate(unit.id(), crate::execution::Reason::Killed);
        let result = super::receive(call, subject, delay);
        assert_eq!(
            result.as_ref().err().unwrap().to_string(),
            "process mailbox is closed"
        );
        result
    }

    fn late_flush<'call>(
        mut call: Call<'call, GleamErlangProfile, ()>,
        cancel: bool,
    ) -> Result<HostCallCompletion<'call, ()>, HostCallError> {
        if cancel {
            let unit = call.execution_unit().unwrap();
            call.execution_state()
                .terminate(unit.id(), crate::execution::Reason::Killed);
        }
        super::super::flush(call)
    }

    fn late_receive_forever<'call>(
        mut call: Native<'call, GleamErlangProfile, A, A>,
        subject: HostCustom<'call, Subject<A>>,
        cancel: bool,
    ) -> Result<HostCallContinuation<'call, A>, HostCallError> {
        let unit = call.call().execution_unit().unwrap();
        if !cancel {
            let (name, ()) = call
                .call()
                .custom_fields::<super::NamedSubject<A>>(subject)
                .unwrap();
            let tag = NativeValue::symbol(call.call().external_payload(name).clone());
            let message = NativeValue::tuple([tag, call.call().native_value::<BigInt>(42.into())]);
            call.call()
                .execution_state()
                .send(unit.id(), crate::execution::Message::Source(message));
            return super::receive_forever(call, subject);
        }
        let scan = call
            .call()
            .execution_state()
            .mailbox(unit.id())
            .unwrap()
            .scan(0);
        call.call()
            .execution_state()
            .terminate(unit.id(), crate::execution::Reason::Killed);
        assert_eq!(
            super::check(
                call.call(),
                unit.id(),
                &super::Filter::Selector(crate::selector::Selector::default()),
                super::Cursor::Resume(scan)
            )
            .err()
            .unwrap()
            .to_string(),
            "process mailbox is closed"
        );
        let result = super::receive_forever(call, subject);
        assert_eq!(
            result.as_ref().err().unwrap().to_string(),
            "process mailbox is closed"
        );
        result
    }

    fn late_select<'call>(
        mut call: Native<'call, GleamErlangProfile, GleamResult<A, ()>, GleamResult<A, ()>>,
        selector: HostExternal<'call, Selector<A>>,
        delay: BigInt,
    ) -> Result<HostCallContinuation<'call, GleamResult<A, ()>>, HostCallError> {
        let unit = call.call().execution_unit().unwrap();
        call.call()
            .execution_state()
            .terminate(unit.id(), crate::execution::Reason::Killed);
        let result = super::select(call, selector, delay);
        assert_eq!(
            result.as_ref().err().unwrap().to_string(),
            "process mailbox is closed"
        );
        result
    }

    fn late_select_forever<'call>(
        mut call: Native<'call, GleamErlangProfile, A, A>,
        selector: HostExternal<'call, Selector<A>>,
    ) -> Result<HostCallContinuation<'call, A>, HostCallError> {
        let unit = call.call().execution_unit().unwrap();
        call.call()
            .execution_state()
            .terminate(unit.id(), crate::execution::Reason::Killed);
        let result = super::select_forever(call, selector);
        assert_eq!(
            result.as_ref().err().unwrap().to_string(),
            "process mailbox is closed"
        );
        result
    }

    mod cancellation {
        use super::super::{A, Call, Filter, Native, Next, One};
        use crate::execution::Message;
        use crate::execution_fixture::TestHost;
        use crate::test_support::PollGate;
        use crate::{Component, Configuration, GleamErlangProfile, GleamErlangRunState};
        use geam_core::embedding::{CallError, FunctionDeclaration, HostedModuleBuilder};
        use geam_core::execution::ExecutionUnit;
        use geam_core::host::native::{NativeCallable, NativeRules};
        use geam_core::host::{
            HostCallContinuation, HostCallError, HostCallable, HostFunctionType,
            HostProviderModule, HostProviderSet, HostType, HostTypeIndex0, HostValue,
        };
        use geam_core::provider::advanced::NativeValue;
        use geam_core::{ModuleSource, PackageSource, compile_typed_host_program};
        use geam_stdlib::provider_support::GleamResult;
        use num_bigint::BigInt;
        use std::sync::Arc;
        use std::time::Duration;

        const WAITING: u8 = 0;
        const QUEUED_SCAN: u8 = 1;
        const QUEUED_RECHECK: u8 = 2;
        const IMMEDIATE: u8 = 3;
        const AFTER_BACKLOG: u8 = 4;
        const AFTER_WAKE: u8 = 5;
        const TIMEOUT: u8 = 6;
        const MALFORMED_SCAN: u8 = 7;
        const MALFORMED_WAKE: u8 = 8;
        const CALLBACK_FAILURE: u8 = 9;

        #[test]
        fn cancellation_during_native_poll_closes_wait_scan_and_woken_requests() {
            for entry in ["wait", "wait_timed"] {
                for phase in [WAITING, QUEUED_SCAN, QUEUED_RECHECK] {
                    let (gate, driver) = PollGate::new();
                    let forever_gate = Arc::clone(&gate);
                    let timed_gate = Arc::clone(&gate);
                    let provider = HostProviderModule::<GleamErlangProfile>::new("application", "main").unwrap()
                        .with_resumable_native_function::<Component<GleamErlangProfile>, (A, BigInt, HostFunctionType<One<A>, A>), A, One<A>, _>(
                            "native_wait", NativeRules::default(),
                            move |call, value, phase, callback| wait(call, value, phase, callback, Arc::clone(&forever_gate)),
                        ).unwrap()
                        .with_resumable_native_function::<Component<GleamErlangProfile>, (A, BigInt, HostFunctionType<One<GleamResult<A, ()>>, A>), GleamResult<A, ()>, One<GleamResult<A, ()>>, _>(
                            "native_wait_timed", NativeRules::default(),
                            move |call, value, phase, callback| wait_timed(call, value, phase, callback, Arc::clone(&timed_gate)),
                        ).unwrap();
                    let typed = compile_typed_host_program(
                        "application",
                        "main",
                        [PackageSource::new(
                            "application",
                            Vec::<String>::new(),
                            [ModuleSource::new(
                                "main",
                                "main.gleam",
                                r#"
@external(erlang, "fixture", "native_wait")
fn native_wait(value: a, phase: Int, callback: fn(a) -> a) -> a
@external(erlang, "fixture", "native_wait_timed")
fn native_wait_timed(value: a, phase: Int, callback: fn(Result(a, Nil)) -> a) -> Result(a, Nil)
pub fn wait(phase: Int) {
  echo native_wait(42, phase, fn(_) { panic as "selected callback failed" })
  Nil
}
pub fn wait_timed(phase: Int) {
  echo native_wait_timed(42, phase, fn(_) { panic as "selected callback failed" })
  Nil
}
"#,
                            )],
                        )],
                        HostProviderSet::from_providers([provider]).unwrap(),
                    )
                    .unwrap();
                    let (builder, function) = HostedModuleBuilder::<GleamErlangProfile>::new(typed)
                        .unwrap()
                        .function(FunctionDeclaration::<(BigInt,), ()>::new(entry))
                        .unwrap();
                    let mut module = builder.seal().unwrap();
                    let host = TestHost::default();
                    let mut state = GleamErlangRunState {
                        stdlib: geam_stdlib::GleamStdlibRunState::from_seed([0; 32]),
                        erlang: Configuration::default(),
                    };
                    let mut echo = Vec::new();
                    let mut successful = vec![IMMEDIATE, AFTER_BACKLOG, AFTER_WAKE];
                    if entry == "wait_timed" {
                        successful.push(TIMEOUT);
                    }
                    host.block_on(module.with_execution(
                        &host,
                        &mut state,
                        &mut echo,
                        async |scope| {
                            for ready in successful {
                                assert_eq!(scope.call(&function, (ready.into(),)).await, Ok(()));
                            }
                            for failed in [MALFORMED_SCAN, MALFORMED_WAKE, CALLBACK_FAILURE] {
                                let error = scope
                                    .call(&function, (failed.into(),))
                                    .await
                                    .unwrap_err()
                                    .to_string();
                                let expected = if failed == CALLBACK_FAILURE {
                                    "panic: selected callback failed".to_owned()
                                } else {
                                    format!("host function application::main.native_{entry} failed: expected a DOWN message")
                                };
                                assert_eq!(error, expected);
                            }
                        },
                    ))
                    .unwrap();
                    let expected = if entry == "wait" {
                        vec!["42", "42", "42"]
                    } else {
                        vec!["Ok(42)", "Ok(42)", "Ok(42)", "Error(Nil)"]
                    };
                    assert_eq!(
                        echo.iter()
                            .map(|output| output.value().inspect().to_string())
                            .collect::<Vec<_>>(),
                        expected
                    );
                    echo.clear();
                    let mut execution = Box::pin(module.with_execution(
                        &host,
                        &mut state,
                        &mut echo,
                        async |scope| scope.call(&function, (phase.into(),)).await,
                    ));
                    driver.cancel(&host, execution.as_mut());
                    assert_eq!(
                        host.block_on(execution.as_mut()).unwrap(),
                        Err(CallError::Cancelled)
                    );
                    drop(execution);
                    assert!(echo.is_empty());
                }
            }
        }

        fn wait<'call>(
            mut call: Native<'call, GleamErlangProfile, A, A>,
            value: HostValue<'call, A>,
            phase: BigInt,
            callback: HostCallable<'call, One<A>, A>,
            gate: Arc<PollGate>,
        ) -> Result<HostCallContinuation<'call, A>, HostCallError> {
            let message = call.source::<A>(value);
            let callback = call.owned_callable::<HostTypeIndex0, A>(callback);
            let cancel = phase < BigInt::from(IMMEDIATE);
            let (unit, filter, first) =
                waiting_mailbox(call.call(), message.clone(), phase, callback, message);
            Ok(call.resume::<HostTypeIndex0>(move |context| {
                Box::pin(async move {
                    let operation = super::super::next(&context, unit.id(), filter, first);
                    if cancel {
                        let error = gate.observe(operation, unit).await.err().unwrap();
                        Err(error)
                    } else {
                        operation.await
                    }
                })
            }))
        }

        fn wait_timed<'call>(
            mut call: Native<'call, GleamErlangProfile, GleamResult<A, ()>, GleamResult<A, ()>>,
            value: HostValue<'call, A>,
            phase: BigInt,
            callback: HostCallable<'call, One<GleamResult<A, ()>>, A>,
            gate: Arc<PollGate>,
        ) -> Result<HostCallContinuation<'call, GleamResult<A, ()>>, HostCallError> {
            let message = call.source::<A>(value);
            let callback = call.owned_callable::<HostTypeIndex0, A>(callback);
            let cancel = phase < BigInt::from(IMMEDIATE);
            let timeout_millis = u64::from(phase != BigInt::from(TIMEOUT));
            let input = NativeValue::tuple([NativeValue::symbol("ok"), message.clone()]);
            let (unit, filter, first) =
                waiting_mailbox(call.call(), message, phase, callback, input);
            let deadline = call.call().clock().now() + Duration::from_millis(timeout_millis);
            let timeout = call.call().clock().sleep_until(deadline);
            Ok(call.resume::<HostTypeIndex0>(move |context| {
                Box::pin(async move {
                    let operation =
                        super::super::timed(&context, unit.id(), filter, first, timeout);
                    if cancel {
                        gate.observe(operation, unit).await
                    } else {
                        operation.await
                    }
                })
            }))
        }

        fn waiting_mailbox<Return: HostType>(
            call: &mut Call<'_, GleamErlangProfile, Return>,
            message: NativeValue,
            phase: BigInt,
            callback: NativeCallable<GleamErlangProfile>,
            callback_input: NativeValue,
        ) -> (
            ExecutionUnit,
            Filter<GleamErlangProfile>,
            Next<GleamErlangProfile>,
        ) {
            let unit = call.execution_unit().unwrap();
            if [QUEUED_SCAN, AFTER_BACKLOG, MALFORMED_SCAN]
                .into_iter()
                .any(|candidate| phase == BigInt::from(candidate))
            {
                for _ in 0..65 {
                    call.execution_state()
                        .send(unit.id(), Message::Source(message.clone()));
                }
            }
            let filter = if phase >= BigInt::from(MALFORMED_SCAN) {
                let key = NativeValue::symbol(if phase == BigInt::from(CALLBACK_FAILURE) {
                    "anything"
                } else {
                    "reference"
                });
                let hash = call.native_hash(&key);
                let handler = Arc::new(crate::selector::Handler {
                    native: callback.native_value().clone(),
                    callback,
                    mappings: im::OrdMap::new(),
                });
                let entry = Arc::new(crate::selector::Entry { key, handler });
                Filter::Selector(crate::selector::Selector {
                    entries: [(hash, [entry].into_iter().collect())]
                        .into_iter()
                        .collect(),
                    len: 1,
                })
            } else {
                Filter::Subject(NativeValue::symbol("selected"))
            };
            let malformed = NativeValue::tuple([
                NativeValue::symbol("DOWN"),
                NativeValue::symbol("reference"),
                NativeValue::symbol("invalid_kind"),
                NativeValue::symbol("pid"),
                NativeValue::symbol("normal"),
            ]);
            if phase == BigInt::from(MALFORMED_SCAN) {
                call.execution_state()
                    .send(unit.id(), Message::Source(malformed.clone()));
            }
            if phase == BigInt::from(CALLBACK_FAILURE) {
                call.execution_state()
                    .send(unit.id(), Message::Source(callback_input));
            }
            let selected = NativeValue::tuple([NativeValue::symbol("selected"), message.clone()]);
            if phase == BigInt::from(IMMEDIATE) || phase == BigInt::from(AFTER_BACKLOG) {
                call.execution_state()
                    .send(unit.id(), Message::Source(selected.clone()));
            }
            let first =
                super::super::check(call, unit.id(), &filter, super::super::Cursor::After(0))
                    .unwrap();
            if phase == BigInt::from(QUEUED_RECHECK) {
                call.execution_state()
                    .send(unit.id(), Message::Source(message));
            }
            if phase == BigInt::from(AFTER_WAKE) {
                call.execution_state()
                    .send(unit.id(), Message::Source(selected));
            }
            if phase == BigInt::from(MALFORMED_WAKE) {
                call.execution_state()
                    .send(unit.id(), Message::Source(malformed));
            }
            (unit, filter, first)
        }
    }
}
