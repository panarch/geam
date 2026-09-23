mod schema;

use self::schema::{
    ChildResult, ChildSpec, ChildSpecSchema, Flag, Flags, FlagsSchema, Handle as HandleType,
    HandleSchema, Local, Message, MessageSchema, Property, Start, SupervisorName, Timeout,
    TimeoutSchema,
};
use crate::child::{Restart, RestartBudget, Shutdown, property};
use crate::schema::{StartError, StartResult};
use crate::{A, B, Call, Component, Four, One, OtpProfile, Three, Two};
use geam::execution::{ExecutionUnit, ExecutionUnitId};
use geam::gleam_erlang::service::{
    CurrentProcess, Pid as ProcessId, Processes, native_rules, new_reference, pid_value,
    with_current_process,
};
use geam::gleam_erlang::{Atom, AtomSchema, Name, Pid, Reference};
use geam::gleam_stdlib::provider_support::{
    Dynamic, DynamicSchema, GleamError, GleamOk, GleamResult,
};
use geam::host::native::{NativeCall, NativeRules};
use geam::host::{
    HostCallCompletion, HostCallContinuation, HostCallError, HostCallableSchema, HostCaptures,
    HostComponentProfile, HostConstructions, HostCreatedFunction, HostCustom, HostExecutionContext,
    HostExecutionError, HostExternal, HostExternalBinding, HostExternalEquality,
    HostExternalHashing, HostExternalInspection, HostExternalStorage, HostExternalStore,
    HostFunctionType, HostList, HostListType, HostOwnedCallable, HostOwnedCompletion, HostProfile,
    HostProviderModule, HostRegistrationError, HostReturns, HostTuple, HostTupleType, HostType,
    HostTypeAt, HostTypeIndex0, HostTypeIndexNext, HostTypeListEnd, HostTypeSequence,
};
use geam::provider::advanced::NativeValue;
use geam::provider::{BigInt, EcoString, Value};
use std::collections::HashMap;
use std::sync::Arc;

pub struct Child<Profile: HostProfile> {
    properties: NativeValue,
    restart: Restart,
    shutdown: Shutdown,
    start: HostOwnedCallable<Profile, Component<Profile>, One<A>, StartResult<B>, HostTypeListEnd>,
}

pub struct Handle {
    source: NativeValue,
    destination: Destination,
}

#[derive(Clone)]
enum Destination {
    Pid(ExecutionUnit),
    Name(EcoString),
}

pub struct ChildStorage;
impl<Profile: OtpProfile> HostExternalBinding<Profile, ChildSpecSchema> for Component<Profile> {
    type Storage = ChildStorage;
}
impl<Profile: OtpProfile> HostExternalStorage<Profile, ChildSpecSchema> for ChildStorage {
    type Payload = Arc<Child<Profile>>;
    fn store(stores: &Profile::ExternalStores) -> &HostExternalStore<Self::Payload> {
        &<Profile as HostComponentProfile<Component<Profile>>>::component_stores(stores)
            .factory_children
    }
    fn source_equal(
        context: &HostExternalEquality<'_>,
        left: &Self::Payload,
        right: &Self::Payload,
    ) -> bool {
        left.properties.source_equal(context, &right.properties)
    }
    fn source_hash(context: &HostExternalHashing<'_>, value: &Self::Payload) -> u64 {
        value.properties.source_hash(context)
    }
    fn inspect(context: &HostExternalInspection<'_>, value: &Self::Payload) -> EcoString {
        value.properties.inspect(context)
    }
    fn native_view(value: &Self::Payload) -> Option<NativeValue> {
        Some(value.properties.clone())
    }
}

pub struct HandleStorage;
impl<Profile: OtpProfile> HostExternalBinding<Profile, HandleSchema> for Component<Profile> {
    type Storage = HandleStorage;
}
impl<Profile: OtpProfile> HostExternalStorage<Profile, HandleSchema> for HandleStorage {
    type Payload = Handle;
    fn store(stores: &Profile::ExternalStores) -> &HostExternalStore<Self::Payload> {
        &<Profile as HostComponentProfile<Component<Profile>>>::component_stores(stores)
            .factory_handles
    }
    fn source_equal(
        context: &HostExternalEquality<'_>,
        left: &Self::Payload,
        right: &Self::Payload,
    ) -> bool {
        left.source.source_equal(context, &right.source)
    }
    fn source_hash(context: &HostExternalHashing<'_>, value: &Self::Payload) -> u64 {
        value.source.source_hash(context)
    }
    fn inspect(context: &HostExternalInspection<'_>, value: &Self::Payload) -> EcoString {
        value.source.inspect(context)
    }
    fn native_view(value: &Self::Payload) -> Option<NativeValue> {
        Some(value.source.clone())
    }
}

type Arguments = HostTupleType<Two<Flags, HostListType<ChildSpec>>>;
type Captures = Four<Flags, ChildSpec, Pid, Reference>;
type StartTypes = Four<Pid, Reference, Dynamic, HostCreatedFunction<SupervisorBody>>;
type Count = HostTupleType<Two<Atom, BigInt>>;
type CountTypes = Four<Pid, Reference, Atom, Count>;
type RequestTypes<Return> = Three<Pid, Reference, Return>;
type Index1 = HostTypeIndexNext<HostTypeIndex0>;
type Index2 = HostTypeIndexNext<Index1>;
type Index3 = HostTypeIndexNext<Index2>;

struct SupervisorBody;
impl HostCallableSchema for SupervisorBody {
    const PACKAGE: &'static str = "gleam_otp";
    const MODULE: &'static str = "gleam/otp/factory_supervisor";
    const NAME: &'static str = "supervisor_loop";
    type Arguments = HostTypeListEnd;
    type Return = ();
    type Captures = Captures;
    type Constructions = One<Pid>;
    type Completion = HostReturns;
}

pub(super) fn provider<Profile: OtpProfile>()
-> Result<HostProviderModule<Profile>, HostRegistrationError> {
    HostProviderModule::new("gleam_otp", "gleam/otp/factory_supervisor")
        .and_then(|module| module.with_external_type::<Component<Profile>, ChildSpecSchema>())
        .and_then(|module| module.with_external_type::<Component<Profile>, FlagsSchema>())
        .and_then(|module| module.with_external_type::<Component<Profile>, TimeoutSchema>())
        .and_then(|module| module.with_external_type::<Component<Profile>, HandleSchema>())
        .and_then(|module| module.with_external_type::<Component<Profile>, MessageSchema>())
        .and_then(|module| module.with_resumable_callable::<Component<Profile>, SupervisorBody, (), _>(supervisor_body::<Profile>))
        .and_then(|module| module.with_scoped_function::<Component<Profile>, (Pid,), HandleType, _>("pid_to_supervisor_handle", pid_handle::<Profile>))
        .and_then(|module| module.with_scoped_function::<Component<Profile>, (Name<Message<A, B>>,), HandleType, _>("name_to_supervisor_handle", name_handle::<Profile>))
        .and_then(|module| module.with_scoped_function_and_constructions::<Component<Profile>, (HostListType<Property<A, B>>,), ChildSpec, HostTypeListEnd, _>("make_erlang_child_spec", make_child::<Profile>))
        .and_then(|module| module.with_scoped_function::<Component<Profile>, (HostListType<Flag<A>>,), Flags, _>("make_erlang_start_flags", make_flags::<Profile>))
        .and_then(|module| module.with_scoped_function::<Component<Profile>, (BigInt,), Timeout, _>("make_timeout", make_timeout::<Profile>))
        .and_then(|module| module.with_resumable_function::<Component<Profile>, (Atom, Arguments), GleamResult<Pid, Dynamic>, StartTypes, _>("unnamed_start", unnamed_start::<Profile>))
        .and_then(|module| module.with_resumable_function::<Component<Profile>, (SupervisorName<A, B>, Atom, Arguments), GleamResult<Pid, Dynamic>, StartTypes, _>("named_start", named_start::<Profile>))
        .and_then(|module| module.with_native_function::<Component<Profile>, (Dynamic,), StartError, One<StartError>, _>("convert_erlang_start_error", native_rules(NativeRules::default()), start_error::<Profile>))
        .and_then(|module| module.with_resumable_native_function::<Component<Profile>, (HandleType, HostListType<B>), ChildResult<A>, RequestTypes<ChildResult<A>>, _>("erlang_start_child", native_rules(NativeRules::default()), start_child::<Profile>))
        .and_then(|module| module.with_resumable_function::<Component<Profile>, (HandleType,), HostListType<Count>, CountTypes, _>("erlang_count_children", count_children::<Profile>))
}

fn pid_handle<'call, Profile: OtpProfile>(
    mut call: Call<'call, Profile, HandleType>,
    pid: HostExternal<'call, Pid>,
) -> Result<HostCallCompletion<'call, HandleType>, HostCallError> {
    let destination = Destination::Pid(Processes::new(&mut call).pid(pid));
    let source = call.native_value::<Pid>(pid);
    let handle = call.create_external(Handle {
        source,
        destination,
    });
    Ok(call.return_value(handle))
}

fn name_handle<'call, Profile: OtpProfile>(
    mut call: Call<'call, Profile, HandleType>,
    name: HostExternal<'call, Name<Message<A, B>>>,
) -> Result<HostCallCompletion<'call, HandleType>, HostCallError> {
    let destination = Destination::Name(Processes::new(&mut call).name(name));
    let source = call.native_value::<Name<Message<A, B>>>(name);
    let handle = call.create_external(Handle {
        source,
        destination,
    });
    Ok(call.return_value(handle))
}

fn make_child<'call, Profile: OtpProfile>(
    mut call: Call<'call, Profile, ChildSpec>,
    constructions: HostConstructions<'call, HostTypeListEnd>,
    properties: HostList<'call, Property<A, B>>,
) -> Result<HostCallCompletion<'call, ChildSpec>, HostCallError> {
    let mut callback = None;
    let mut index = 0;
    while let Some(property) = call.list_item::<Property<A, B>>(properties, index) {
        if let Some((mfa, ())) = call.custom_fields::<Start<A, B>>(property) {
            let (_, (_, (callbacks, ()))) = call.tuple_values(mfa);
            callback = call.list_item::<HostFunctionType<One<A>, StartResult<B>>>(callbacks, 0);
        }
        index += 1;
    }
    let callback = callback
        .ok_or_else(|| geam::HostFailure::new("child specification has no start callback"))?;
    let start = call.owned_callable(callback, &constructions);
    let properties = call.native_value::<HostListType<Property<A, B>>>(properties);
    let child = Child {
        restart: Restart::from_properties(&properties)?,
        shutdown: Shutdown::from_properties(&properties)?,
        properties,
        start,
    };
    let child = call.create_external(Arc::new(child));
    Ok(call.return_value(child))
}

fn make_flags<'call, Profile: OtpProfile>(
    mut call: Call<'call, Profile, Flags>,
    flags: HostList<'call, Flag<A>>,
) -> Result<HostCallCompletion<'call, Flags>, HostCallError> {
    let value = call.native_value::<HostListType<Flag<A>>>(flags);
    let value = call.create_external(value);
    Ok(call.return_value(value))
}
fn make_timeout<'call, Profile: OtpProfile>(
    mut call: Call<'call, Profile, Timeout>,
    amount: BigInt,
) -> Result<HostCallCompletion<'call, Timeout>, HostCallError> {
    let value = if amount < BigInt::from(0) {
        NativeValue::symbol("infinity")
    } else {
        call.native_value::<BigInt>(amount)
    };
    let value = call.create_external(value);
    Ok(call.return_value(value))
}
fn start_error<'call, Profile: OtpProfile>(
    mut call: NativeCall<'call, Profile, Component<Profile>, StartError, One<StartError>>,
    error: HostExternal<'call, Dynamic>,
) -> Result<HostCallCompletion<'call, StartError>, HostCallError> {
    let error = call.call().external_payload_with::<geam::gleam_erlang::Component<Profile>, DynamicSchema, HostTypeListEnd>(error).native_value().clone();
    let error = call
        .convert::<HostTypeIndex0>(&error)
        .ok_or_else(|| geam::HostFailure::new("invalid supervisor start error"))?;
    Ok(call.finish(error))
}

fn unnamed_start<'call, Profile: OtpProfile>(
    call: Call<'call, Profile, GleamResult<Pid, Dynamic>>,
    constructions: HostConstructions<'call, StartTypes>,
    _: HostExternal<'call, Atom>,
    arguments: HostTuple<'call, Two<Flags, HostListType<ChildSpec>>>,
) -> Result<HostCallContinuation<'call, GleamResult<Pid, Dynamic>>, HostCallError> {
    start(call, constructions, arguments, None)
}

fn named_start<'call, Profile: OtpProfile>(
    mut call: Call<'call, Profile, GleamResult<Pid, Dynamic>>,
    constructions: HostConstructions<'call, StartTypes>,
    name: HostCustom<'call, SupervisorName<A, B>>,
    _: HostExternal<'call, Atom>,
    arguments: HostTuple<'call, Two<Flags, HostListType<ChildSpec>>>,
) -> Result<HostCallContinuation<'call, GleamResult<Pid, Dynamic>>, HostCallError> {
    let (name, ()) = call.provider_remaining_custom_fields::<Local<A, B>>(name);
    let name = Processes::new(&mut call).name(name);
    start(call, constructions, arguments, Some(name))
}

fn start<'call, Profile: OtpProfile>(
    call: Call<'call, Profile, GleamResult<Pid, Dynamic>>,
    constructions: HostConstructions<'call, StartTypes>,
    arguments: HostTuple<'call, Two<Flags, HostListType<ChildSpec>>>,
    name: Option<EcoString>,
) -> Result<HostCallContinuation<'call, GleamResult<Pid, Dynamic>>, HostCallError> {
    CurrentProcess::with(call, |mut process| {
        let current = process.current().clone();
        let call = process.call();
        let (flags, (children, ())) = call.tuple_values(arguments);
        let child = match (
            call.list_len(children),
            call.list_item::<ChildSpec>(children, 0),
        ) {
            (1, Some(child)) => child,
            _ => return Err(geam::HostFailure::new("factory requires one child template").into()),
        };
        let parent = current;
        let parent = pid_value(call, constructions.at::<HostTypeIndex0>(), parent);
        let reference = new_reference(call, constructions.at::<Index1>());
        let tag = call.native_value::<Reference>(reference);
        let body = call.construct_function::<SupervisorBody>(
            constructions.at::<Index3>(),
            (flags, (child, (parent, (reference, ())))),
        );
        let supervisor = call.spawn(body);
        if let Some(name) = name {
            let mut processes = Processes::new(call);
            if !processes.register(&supervisor, name) {
                processes.kill(&supervisor);
                return Ok(process.into_call().resume(constructions, |_| Box::pin(async {
                Ok(HostOwnedCompletion::new(|mut call, constructions| {
                    let reason = NativeValue::tuple([NativeValue::symbol("init_failed"), call.native_values().string("supervisor name is already registered".into())]);
                    let reason = call.construct_external_with_binding::<geam::gleam_erlang::Component<Profile>, DynamicSchema, HostTypeListEnd>(constructions.at::<Index2>(), geam::gleam_stdlib::Dynamic::from_native(reason));
                    Ok(call.return_custom::<GleamError<Pid, Dynamic>>((reason, ())))
                }))
            })));
            }
        }
        let target = pid_value(
            call,
            constructions.at::<HostTypeIndex0>(),
            supervisor.clone(),
        );
        process.link(target, constructions.at::<HostTypeIndex0>());
        process.resume_receive(constructions, tag, None, move |receive, context| {
            Box::pin(async move {
                receive.wait_forever(&context).await?;
                Ok(HostOwnedCompletion::new(move |mut call, constructions| {
                    let pid =
                        pid_value(&mut call, constructions.at::<HostTypeIndex0>(), supervisor);
                    Ok(call.return_custom::<GleamOk<Pid, Dynamic>>((pid, ())))
                }))
            })
        })
    })
}

fn start_child<'call, Profile: OtpProfile>(
    mut call: NativeCall<
        'call,
        Profile,
        Component<Profile>,
        ChildResult<A>,
        RequestTypes<ChildResult<A>>,
    >,
    handle: HostExternal<'call, HandleType>,
    arguments: HostList<'call, B>,
) -> Result<HostCallContinuation<'call, ChildResult<A>>, HostCallError> {
    let destination = call.call().external_payload(handle).destination.clone();
    let argument = call
        .call()
        .list_item::<B>(arguments, 0)
        .ok_or_else(|| geam::HostFailure::new("factory child argument is missing"))?;
    let argument = call.source::<B>(argument);
    Ok(call.resume::<Index2>(move |context| {
        Box::pin(async move {
            let receive = with_current_process(&context, move |mut process, constructions| {
                let tag = request(&mut process, &constructions, destination, "start", argument)?;
                process.receive(tag, None)
            })
            .await?;
            receive.wait_forever(&context).await
        })
    }))
}

fn count_children<'call, Profile: OtpProfile>(
    call: Call<'call, Profile, HostListType<Count>>,
    constructions: HostConstructions<'call, CountTypes>,
    handle: HostExternal<'call, HandleType>,
) -> Result<HostCallContinuation<'call, HostListType<Count>>, HostCallError> {
    let destination = call.external_payload(handle).destination.clone();
    CurrentProcess::with(call, |mut process| {
        let tag = request(
            &mut process,
            &constructions,
            destination,
            "count",
            NativeValue::symbol("nil"),
        )?;
        process.resume_receive(constructions, tag, None, move |receive, context| Box::pin(async move {
        let count = receive.wait_forever(&context).await?.as_int().ok_or_else(|| geam::HostFailure::new("factory count response is not an integer"))?;
        Ok(HostOwnedCompletion::new(move |mut call, constructions| {
            let active = call.construct_external_with_binding::<geam::gleam_erlang::Component<Profile>, AtomSchema, HostTypeListEnd>(constructions.at::<Index2>(), NativeValue::symbol("active"));
            let count = call.construct_tuple(constructions.at::<Index3>(), (active, (count, ())));
            Ok(call.return_list([count]))
        }))
    }))
    })
}

fn request<'call, Profile: OtpProfile, Return: HostType, Targets>(
    process: &mut CurrentProcess<'call, Profile, Component<Profile>, Return>,
    constructions: &HostConstructions<'call, Targets>,
    destination: Destination,
    operation: &'static str,
    argument: NativeValue,
) -> Result<NativeValue, HostCallError>
where
    Targets: HostTypeSequence
        + HostTypeAt<HostTypeIndex0, Type = Pid>
        + HostTypeAt<Index1, Type = Reference>,
{
    let mut processes = process.processes();
    let target = match destination {
        Destination::Pid(pid) => pid,
        Destination::Name(name) => processes
            .named(&name)
            .ok_or_else(|| geam::HostFailure::new("factory supervisor name is not registered"))?,
    };
    if !processes.is_alive(&target) {
        return Err(geam::HostFailure::new("factory supervisor has exited").into());
    }
    let requester = process.current().clone();
    let call = process.call();
    let requester = pid_value(call, constructions.at::<HostTypeIndex0>(), requester);
    let requester = call.native_value::<Pid>(requester);
    let reference = new_reference(call, constructions.at::<Index1>());
    let tag = call.native_value::<Reference>(reference);
    let mut processes = Processes::new(call);
    processes.send(
        &target,
        NativeValue::tuple([
            NativeValue::symbol("factory"),
            NativeValue::symbol(operation),
            argument,
            requester,
            tag.clone(),
        ]),
    );
    Ok(tag)
}

struct Running {
    unit: ExecutionUnit,
    argument: NativeValue,
}

fn supervisor_body<'call, Profile: OtpProfile>(
    call: Call<'call, Profile, ()>,
    captures: HostCaptures<'call, Captures>,
    constructions: HostConstructions<'call, One<Pid>>,
) -> Result<HostCallContinuation<'call, ()>, HostCallError> {
    CurrentProcess::with(call, |mut process| {
        let call = process.call();
        let (flags, (child, (parent, (reference, ())))) = call.captures(captures);
        let flags = call.external_payload(flags).clone();
        validate_flags(&flags)?;
        let mut budget = RestartBudget::from_flags(&flags)?;
        let child = Arc::clone(&call.external_payload(child));
        let tag = call.native_value::<Reference>(reference);
        let parent = Processes::new(call).pid(parent);
        process.trap_exits(true);
        process.processes().send(
            &parent,
            NativeValue::tuple([tag, NativeValue::symbol("ready")]),
        );
        Ok(process.into_call().resume(constructions, move |context| {
            Box::pin(async move {
                let mut children = HashMap::<ExecutionUnitId, Running>::new();
                let mut reason = NativeValue::symbol("shutdown");
                loop {
                    let receive =
                        with_current_process(&context, |mut process, _| process.receive_any(None))
                            .await?;
                    let message = receive.wait_forever(&context).await?;
                    match incoming(message)? {
                        Incoming::Exit {
                            pid,
                            reason: child_reason,
                        } => {
                            let exited = restore_pid(&context, pid).await?;
                            if exited.id() == parent.id() {
                                reason = child_reason;
                                break;
                            }
                            if let Some(running) = children.remove(&exited.id())
                                && child.restart.after_exit(&child_reason)
                            {
                                let now = context.with_call(|call| call.clock().now()).await?;
                                if !budget.allow(now) {
                                    break;
                                }
                                match run_child(&context, &child, running.argument).await? {
                                    Ok((_, running)) => {
                                        children.insert(running.unit.id(), running);
                                    }
                                    Err(_) => break,
                                }
                            }
                        }
                        Incoming::Request {
                            operation,
                            requester,
                            tag,
                        } => {
                            let requester = restore_pid(&context, requester).await?;
                            let result = match operation {
                                Operation::Start(argument) => {
                                    match run_child(&context, &child, argument).await? {
                                        Ok((result, running)) => {
                                            children.insert(running.unit.id(), running);
                                            result
                                        }
                                        Err(error) => NativeValue::tuple([
                                            NativeValue::symbol("error"),
                                            error,
                                        ]),
                                    }
                                }
                                Operation::Count => {
                                    let count = children.len();
                                    context
                                        .with_call(move |call| {
                                            call.native_values().integer(count.into())
                                        })
                                        .await?
                                }
                            };
                            context
                                .with_call(move |mut call| {
                                    Processes::new(&mut call)
                                        .send(&requester, NativeValue::tuple([tag, result]));
                                })
                                .await?;
                        }
                    }
                }
                for running in children.into_values() {
                    crate::child::stop(&context, &running.unit, child.shutdown).await?;
                }
                Ok(HostOwnedCompletion::new(move |call, constructions| {
                    CurrentProcess::with(call, |mut process| {
                        let current = process.current().clone();
                        process.send_exit(&current, reason, constructions.at::<HostTypeIndex0>());
                        Ok(process.into_call().return_value(()))
                    })
                }))
            })
        }))
    })
}

fn validate_flags(flags: &NativeValue) -> Result<(), HostCallError> {
    if property(flags, "strategy")
        .and_then(|value| value.as_symbol())
        .as_deref()
        != Some("simple_one_for_one")
    {
        return Err(geam::HostFailure::new("factory requires SimpleOneForOne").into());
    }
    Ok(())
}

enum Incoming {
    Exit {
        pid: NativeValue,
        reason: NativeValue,
    },
    Request {
        operation: Operation,
        requester: NativeValue,
        tag: NativeValue,
    },
}
enum Operation {
    Start(NativeValue),
    Count,
}

fn incoming(message: NativeValue) -> Result<Incoming, geam::HostFailure> {
    match message.index(0).and_then(|tag| tag.as_symbol()).as_deref() {
        Some("EXIT") => Ok(Incoming::Exit {
            pid: field(&message, 1)?,
            reason: field(&message, 2)?,
        }),
        Some("factory") => {
            let operation = field(&message, 1)?;
            let argument = field(&message, 2)?;
            let requester = field(&message, 3)?;
            let tag = field(&message, 4)?;
            let operation = match operation.as_symbol().as_deref() {
                Some("start") => Operation::Start(argument),
                Some("count") => Operation::Count,
                _ => return Err(geam::HostFailure::new("unknown factory operation")),
            };
            Ok(Incoming::Request {
                operation,
                requester,
                tag,
            })
        }
        _ => Err(geam::HostFailure::new("unexpected factory message")),
    }
}

fn field(value: &NativeValue, index: usize) -> Result<NativeValue, geam::HostFailure> {
    value
        .index(index)
        .ok_or_else(|| geam::HostFailure::new("factory message is missing a field"))
}

async fn restore_pid<Profile: OtpProfile>(
    context: &HostExecutionContext<'_, Profile, Component<Profile>, One<Pid>>,
    pid: NativeValue,
) -> Result<ExecutionUnit, HostExecutionError> {
    context
        .with_call(move |call| {
            let mut call = geam::provider::Call::from_host_call(call);
            call.restore_native::<ProcessId>(&pid)
                .map(|pid| pid.execution_unit())
                .ok_or_else(|| geam::HostFailure::new("factory message has no source Pid"))
        })
        .await?
        .map_err(Into::into)
}

async fn run_child<Profile: OtpProfile>(
    context: &HostExecutionContext<'_, Profile, Component<Profile>, One<Pid>>,
    child: &Child<Profile>,
    argument: NativeValue,
) -> Result<Result<(NativeValue, Running), NativeValue>, HostExecutionError> {
    let input = argument.clone();
    let result = child
        .start
        .try_invoke(
            context,
            move |call, _| {
                let mut call = geam::provider::Call::from_host_call(call);
                let value = call.restore_native::<Value<A>>(&input).ok_or_else(|| {
                    geam::HostFailure::new("factory argument does not match the retained callback")
                })?;
                Ok((value.into_host(call.host_call()), ()))
            },
            |call, _, result| Ok(crate::child::decode(call, result)),
        )
        .await?;
    let started = match result {
        Ok(started) => started,
        Err(error) => return Ok(Err(error)),
    };
    let unit = started.unit;
    let target = unit.clone();
    with_current_process(context, move |mut process, constructions| {
        let target = pid_value(process.call(), constructions.at::<HostTypeIndex0>(), target);
        process.link(target, constructions.at::<HostTypeIndex0>());
        Ok(())
    })
    .await?;
    Ok(Ok((
        NativeValue::tuple([NativeValue::symbol("ok"), started.pid, started.data]),
        Running { unit, argument },
    )))
}

#[cfg(test)]
mod tests {
    use super::schema::{ChildSpecSchema, FlagsSchema, HandleSchema, MessageSchema, TimeoutSchema};
    use super::{Child, Destination, Handle};
    use crate::child::{Restart, Shutdown};
    use crate::schema::StartResult;
    use crate::test_support::storage::{
        Probe, ProbeValue, callback_packages, callback_providers, check, native_protocol,
    };
    use crate::test_support::{Profile, run_source};
    use crate::{A, B, Call, Component, One};
    use geam::host::{
        HostCallCompletion, HostCallError, HostCallable, HostConstructions, HostFunctionType,
        HostProviderModule, HostTypeListEnd,
    };
    use geam::provider::{BigInt, StringValue};
    use std::sync::Arc;

    #[test]
    fn native_start_boundaries_validate_templates_configuration_names_and_arguments() {
        use crate::test_support::{native_boundary, run_observed_source};
        use geam::execution::UnitExit;
        let actor = r#"
import gleam/erlang/process.{type Pid, type ExitReason}
pub type Started(a) { Started(pid: Pid, data: a) }
pub type StartError { InitTimeout InitFailed(String) InitExited(ExitReason) }
pub type StartResult(a) = Result(Started(a), StartError)
"#;
        let result2 = "pub type Result2(a, b, e) { Ok(a, b) Error(e) }";
        let declarations = r#"
import gleam/dynamic.{type Dynamic}
import gleam/erlang/atom.{type Atom}
import gleam/erlang/process.{type Pid, type Name}
import gleam/otp/actor
import gleam/otp/internal/result2.{type Result2}
import gleam/otp/supervision
pub type ErlangStartFlags
pub type ErlangChildSpec
pub type Timeout
pub type SupervisorHandle
pub type Message(argument, data)
pub type ErlangSupervisorName(argument, data) { Local(Name(Message(argument, data))) }
pub type Strategy { SimpleOneForOne }
pub type ErlangStartFlag(data) { Strategy(Strategy) Intensity(Int) Period(Int) }
pub type ErlangChildSpecProperty(argument, data) {
  Id(Int)
  Start(#(Atom, Atom, List(fn(argument) -> actor.StartResult(data))))
  Restart(supervision.Restart)
  Type(Atom)
  Shutdown(Timeout)
}
@external(erlang, "fixture", "make_erlang_start_flags") fn make_erlang_start_flags(flags: List(ErlangStartFlag(data))) -> ErlangStartFlags
@external(erlang, "fixture", "make_erlang_child_spec") fn make_erlang_child_spec(properties: List(ErlangChildSpecProperty(argument, data))) -> ErlangChildSpec
@external(erlang, "fixture", "make_timeout") fn make_timeout(amount: Int) -> Timeout
@external(erlang, "fixture", "unnamed_start") fn unnamed_start(module: Atom, args: #(ErlangStartFlags, List(ErlangChildSpec))) -> Result(Pid, Dynamic)
@external(erlang, "fixture", "named_start") fn named_start(name: ErlangSupervisorName(argument, data), module: Atom, args: #(ErlangStartFlags, List(ErlangChildSpec))) -> Result(Pid, Dynamic)
@external(erlang, "fixture", "name") fn name(value: String) -> Name(message)
@external(erlang, "fixture", "pid_to_supervisor_handle") fn pid_to_supervisor_handle(pid: Pid) -> SupervisorHandle
@external(erlang, "fixture", "erlang_start_child") fn erlang_start_child(handle: SupervisorHandle, arguments: List(argument)) -> Result2(Pid, data, actor.StartError)
"#;
        for (operation, expected, cancelled) in [
            (
                "let _ = unnamed_start(module, #(flags, []))",
                Some("factory requires one child template"),
                false,
            ),
            (
                "let _ = unnamed_start(module, #(flags, [child, child]))",
                Some("factory requires one child template"),
                false,
            ),
            (
                "let missing: List(ErlangStartFlag(Nil)) = []\nlet _ = unnamed_start(module, #(make_erlang_start_flags(missing), [child]))",
                Some("factory requires SimpleOneForOne"),
                true,
            ),
            (
                "let invalid: List(ErlangStartFlag(Nil)) = [Strategy(SimpleOneForOne), Intensity(-1), Period(5)]\nlet _ = unnamed_start(module, #(make_erlang_start_flags(invalid), [child]))",
                Some("restart intensity must be nonnegative"),
                true,
            ),
            (
                "let assert Ok(pid) = unnamed_start(module, #(flags, [child]))\nlet arguments: List(Nil) = []\nlet result: Result2(Pid, Nil, actor.StartError) = erlang_start_child(pid_to_supervisor_handle(pid), arguments)\nlet _ = result",
                Some("factory child argument is missing"),
                false,
            ),
            (
                "let assert Ok(pid) = unnamed_start(module, #(flags, [child]))\nlet result: Result2(Pid, Nil, actor.StartError) = erlang_start_child(pid_to_supervisor_handle(pid), [Nil])\nlet assert result2.Error(actor.InitTimeout) = result",
                None,
                false,
            ),
            (
                "let destination: ErlangSupervisorName(Nil, Nil) = Local(name(\"owner-factory\"))\nlet assert Ok(_) = named_start(destination, module, #(flags, [child]))\nlet assert Error(_) = named_start(destination, module, #(flags, [child]))",
                None,
                false,
            ),
        ] {
            let factory = format!(
                r#"{declarations}
pub fn probe() {{
  let module = atom.create("factory")
  let values: List(ErlangStartFlag(Nil)) = [Strategy(SimpleOneForOne), Intensity(1), Period(5)]
  let flags = make_erlang_start_flags(values)
  let callback = fn(_: Nil) -> actor.StartResult(Nil) {{ Error(actor.InitTimeout) }}
  let child = make_erlang_child_spec([Start(#(module, module, [callback])), Restart(supervision.Permanent), Shutdown(make_timeout(5))])
  {operation}
  Nil
}}
"#
            );
            let native = HostProviderModule::new("gleam_otp", "gleam/otp/factory_supervisor").unwrap()
                .with_external_type::<Component<Profile>, ChildSpecSchema>().unwrap()
                .with_external_type::<Component<Profile>, FlagsSchema>().unwrap()
                .with_external_type::<Component<Profile>, TimeoutSchema>().unwrap()
                .with_external_type::<Component<Profile>, HandleSchema>().unwrap()
                .with_external_type::<Component<Profile>, MessageSchema>().unwrap()
                .with_resumable_callable::<Component<Profile>, super::SupervisorBody, (), _>(super::supervisor_body::<Profile>).unwrap()
                .with_scoped_function_and_constructions::<Component<Profile>, (geam::host::HostListType<super::Property<A, B>>,), super::ChildSpec, HostTypeListEnd, _>("make_erlang_child_spec", super::make_child::<Profile>).unwrap()
                .with_scoped_function::<Component<Profile>, (geam::host::HostListType<super::Flag<A>>,), super::Flags, _>("make_erlang_start_flags", super::make_flags::<Profile>).unwrap()
                .with_scoped_function::<Component<Profile>, (BigInt,), super::Timeout, _>("make_timeout", super::make_timeout::<Profile>).unwrap()
                .with_resumable_function::<Component<Profile>, (super::Atom, super::Arguments), super::GleamResult<super::Pid, super::Dynamic>, super::StartTypes, _>("unnamed_start", super::unnamed_start::<Profile>).unwrap()
                .with_resumable_function::<Component<Profile>, (super::SupervisorName<A, B>, super::Atom, super::Arguments), super::GleamResult<super::Pid, super::Dynamic>, super::StartTypes, _>("named_start", super::named_start::<Profile>).unwrap()
                .with_scoped_function::<geam::gleam_erlang::Component<Profile>, (StringValue,), super::Name<A>, _>("name", source_name).unwrap()
                .with_scoped_function::<Component<Profile>, (super::Pid,), super::HandleType, _>("pid_to_supervisor_handle", super::pid_handle::<Profile>).unwrap()
                .with_resumable_native_function::<Component<Profile>, (super::HandleType, geam::host::HostListType<B>), super::ChildResult<A>, super::RequestTypes<super::ChildResult<A>>, _>("erlang_start_child", super::native_rules(geam::host::native::NativeRules::default()), super::start_child::<Profile>).unwrap();
            let mut providers = native_boundary::providers();
            providers.push(native);
            let (result, exits) = run_observed_source(
                native_boundary::packages(
                    "import gleam/otp/factory_supervisor\npub fn main() { factory_supervisor.probe() }",
                    &[
                        ("gleam/otp/actor", actor),
                        ("gleam/otp/factory_supervisor", &factory),
                        (
                            "gleam/otp/supervision",
                            "pub type Restart { Permanent Transient Temporary }",
                        ),
                        ("gleam/otp/internal/result2", result2),
                    ],
                ),
                providers,
            );
            if cancelled {
                assert_eq!(
                    std::mem::discriminant(&result.err().unwrap()),
                    std::mem::discriminant(&geam::execution::RunError::Cancelled)
                );
            } else if let Some(expected) = expected {
                assert_eq!(
                    crate::test_support::host_failure(result).as_deref(),
                    Some(expected)
                );
            } else {
                assert_eq!(result.unwrap(), geam::Value::Nil);
            }
            let failures = exits
                .iter()
                .filter_map(|exit| match exit {
                    UnitExit::Failed(geam::ExecutionError::Host(error)) => {
                        Some(error.failure().message())
                    }
                    _ => None,
                })
                .collect::<Vec<_>>();
            assert_eq!(failures, expected.into_iter().collect::<Vec<_>>());
        }
    }

    fn source_name<'call>(
        mut call: geam::HostCall<
            'call,
            Profile,
            geam::gleam_erlang::Component<Profile>,
            super::Name<A>,
        >,
        name: StringValue,
    ) -> Result<HostCallCompletion<'call, super::Name<A>>, HostCallError> {
        let name = call.create_external(name.as_str().into());
        Ok(call.return_value(name))
    }

    #[test]
    fn native_start_error_roundtrips_declared_variants_and_rejects_invalid_input() {
        use geam::gleam_stdlib::provider_support::Dynamic;
        let provider = HostProviderModule::new("application", "main").unwrap()
            .with_scoped_function::<geam::gleam_erlang::Component<Profile>, (A,), Dynamic, _>("erase", crate::test_support::erase_native).unwrap()
            .with_native_function::<Component<Profile>, (Dynamic,), super::StartError, One<super::StartError>, _>("convert", geam::gleam_erlang::service::native_rules(geam::host::native::NativeRules::default()), super::start_error::<Profile>).unwrap();
        let source = r#"
import gleam/dynamic.{type Dynamic}
import gleam/otp/actor
@external(erlang, "fixture", "erase") fn erase(value: a) -> Dynamic
@external(erlang, "fixture", "convert") fn convert(value: Dynamic) -> actor.StartError
pub fn main() {
  let assert actor.InitTimeout = convert(erase(actor.InitTimeout))
  let assert actor.InitFailed("rejected") = convert(erase(actor.InitFailed("rejected")))
  let _ = convert(erase("not a start error"))
  Nil
}
"#;
        let packages = callback_packages(source).into_iter().map(|package| {
            if package.package() == "application" {
                geam::PackageSource::new("application", ["gleam_otp", "gleam_stdlib"], package.modules().to_vec())
            } else if package.package() == "gleam_erlang" {
                geam::PackageSource::new("gleam_erlang", ["gleam_stdlib"], [
                    geam::ModuleSource::new("gleam/erlang/process", "process.gleam", "import gleam/dynamic.{type Dynamic}\npub type Pid\npub type Monitor\npub type ExitReason { Normal Killed Abnormal(reason: Dynamic) }"),
                    geam::ModuleSource::new("gleam/erlang/atom", "atom.gleam", "pub type Atom"),
                    geam::ModuleSource::new("gleam/erlang/reference", "reference.gleam", "pub type Reference"),
                ])
            } else {
                package
            }
        });
        let mut providers = callback_providers(provider);
        providers.retain(|module| module.module() != "gleam/erlang/process");
        providers.extend([
            HostProviderModule::new("gleam_erlang", "gleam/erlang/process").unwrap()
                .with_external_type::<geam::gleam_erlang::Component<Profile>, geam::gleam_erlang::PidSchema>().unwrap()
                .with_external_type::<geam::gleam_erlang::Component<Profile>, geam::gleam_erlang::service::types::MonitorSchema>().unwrap(),
            HostProviderModule::new("gleam_erlang", "gleam/erlang/atom").unwrap()
                .with_external_type::<geam::gleam_erlang::Component<Profile>, geam::gleam_erlang::AtomSchema>().unwrap(),
            HostProviderModule::new("gleam_erlang", "gleam/erlang/reference").unwrap()
                .with_external_type::<geam::gleam_erlang::Component<Profile>, geam::gleam_erlang::ReferenceSchema>().unwrap(),
        ]);
        assert_eq!(
            crate::test_support::host_failure(run_source(packages, providers)).as_deref(),
            Some("invalid supervisor start error")
        );
    }

    #[test]
    fn child_spec_native_boundary_rejects_missing_callback_and_policies() {
        use geam::{ModuleSource, PackageSource};
        for (properties, expected) in [
            ("[]", "child specification has no start callback"),
            (
                "[Start(#(atom.create(\"m\"), atom.create(\"f\"), []))]",
                "child specification has no start callback",
            ),
            (
                "[Start(mfa)]",
                "child specification requires a restart policy",
            ),
            (
                "[Start(mfa), Restart(supervision.Permanent)]",
                "child specification requires a shutdown policy",
            ),
        ] {
            let source = format!(
                r#"
import gleam/erlang/atom.{{type Atom}}
import gleam/otp/actor
import gleam/otp/supervision
pub type ErlangChildSpec
pub type Timeout
pub type ErlangChildSpecProperty(argument, data) {{
  Id(Int)
  Start(#(Atom, Atom, List(fn(argument) -> actor.StartResult(data))))
  Restart(supervision.Restart)
  Type(Atom)
  Shutdown(Timeout)
}}
@external(erlang, "maps", "from_list")
fn make_erlang_child_spec(properties: List(ErlangChildSpecProperty(argument, data))) -> ErlangChildSpec
pub fn probe() {{
  let mfa = #(atom.create("m"), atom.create("f"), [fn(_: Int) -> actor.StartResult(Nil) {{ Error(actor.InitTimeout) }}])
  let properties: List(ErlangChildSpecProperty(Int, Nil)) = {properties}
  let _ = make_erlang_child_spec(properties)
  Nil
}}
"#
            );
            let packages = callback_packages(
                "import gleam/otp/factory_supervisor\npub fn main() { factory_supervisor.probe() }",
            )
            .into_iter()
            .map(|package| {
                let mut modules = package.modules().to_vec();
                match package.package().as_str() {
                    "gleam_erlang" => modules.push(ModuleSource::new(
                        "gleam/erlang/atom",
                        "atom.gleam",
                        r#"
import gleam/dynamic.{type Dynamic}
pub type Atom
@external(erlang, "fixture", "create") pub fn create(name: String) -> Atom
@external(erlang, "fixture", "get") pub fn get(name: String) -> Result(Atom, Nil)
@external(erlang, "fixture", "to_string") pub fn to_string(atom: Atom) -> String
@external(erlang, "fixture", "to_dynamic") pub fn to_dynamic(atom: Atom) -> Dynamic
@external(erlang, "fixture", "cast_from_dynamic") pub fn cast_from_dynamic(value: Dynamic) -> Atom
@external(erlang, "fixture", "is_atom") pub fn is_atom(value: Dynamic) -> Bool
"#,
                    )),
                    "gleam_otp" => {
                        modules.push(ModuleSource::new(
                            "gleam/otp/supervision",
                            "supervision.gleam",
                            "pub type Restart { Permanent Transient Temporary }",
                        ));
                        modules.push(ModuleSource::new(
                            "gleam/otp/factory_supervisor",
                            "factory_supervisor.gleam",
                            source.clone(),
                        ));
                    }
                    _ => {}
                }
                PackageSource::new(
                    package.package().clone(),
                    package.direct_dependencies().iter().cloned(),
                    modules,
                )
            })
            .collect::<Vec<_>>();
            let native = HostProviderModule::new("gleam_otp", "gleam/otp/factory_supervisor").unwrap()
                .with_external_type::<Component<Profile>, ChildSpecSchema>().unwrap()
                .with_external_type::<Component<Profile>, TimeoutSchema>().unwrap()
                .with_scoped_function_and_constructions::<Component<Profile>, (geam::host::HostListType<super::Property<A, B>>,), super::ChildSpec, HostTypeListEnd, _>("make_erlang_child_spec", super::make_child::<Profile>).unwrap();
            let mut providers = callback_providers(native);
            providers.extend(
                geam::gleam_erlang::host_providers::<Profile>()
                    .unwrap()
                    .into_iter()
                    .filter(|module| module.module() == "gleam/erlang/atom"),
            );
            assert_eq!(
                crate::test_support::host_failure(run_source(packages, providers)).as_deref(),
                Some(expected)
            );
        }
    }

    #[test]
    fn factory_native_messages_validate_tag_operation_and_required_fields() {
        use geam::provider::advanced::NativeValue;
        let symbols = |values: &[&str]| {
            NativeValue::tuple(values.iter().map(|value| NativeValue::symbol(*value)))
        };
        assert_eq!(
            super::incoming(symbols(&[])).err().unwrap().to_string(),
            "unexpected factory message"
        );
        assert_eq!(
            super::incoming(symbols(&["factory", "unknown", "argument", "pid", "tag"]))
                .err()
                .unwrap()
                .to_string(),
            "unknown factory operation"
        );
        for prefix in 1..5 {
            assert_eq!(
                super::incoming(symbols(
                    &["factory", "start", "argument", "pid", "tag"][..prefix]
                ))
                .err()
                .unwrap()
                .to_string(),
                "factory message is missing a field"
            );
        }
        for fields in [&["EXIT"][..], &["EXIT", "pid"][..]] {
            assert_eq!(
                super::incoming(symbols(fields)).err().unwrap().to_string(),
                "factory message is missing a field"
            );
        }
        for (input, expected) in [
            (
                &["EXIT", "pid", "normal"][..],
                &["exit", "pid", "normal"][..],
            ),
            (
                &["factory", "start", "argument", "pid", "tag"][..],
                &["start", "argument", "pid", "tag"][..],
            ),
            (
                &["factory", "count", "argument", "pid", "tag"][..],
                &["count", "pid", "tag"][..],
            ),
        ] {
            let observed = match super::incoming(symbols(input)).unwrap() {
                super::Incoming::Exit { pid, reason } => vec![
                    "exit".into(),
                    pid.as_symbol().unwrap(),
                    reason.as_symbol().unwrap(),
                ],
                super::Incoming::Request {
                    operation,
                    requester,
                    tag,
                } => match operation {
                    super::Operation::Start(value) => vec![
                        "start".into(),
                        value.as_symbol().unwrap(),
                        requester.as_symbol().unwrap(),
                        tag.as_symbol().unwrap(),
                    ],
                    super::Operation::Count => vec![
                        "count".into(),
                        requester.as_symbol().unwrap(),
                        tag.as_symbol().unwrap(),
                    ],
                },
            };
            assert_eq!(observed, expected);
        }
        assert!(super::validate_flags(&symbols(&[])).is_err());
        let good = NativeValue::tuple([symbols(&["strategy", "simple_one_for_one"])]);
        assert!(super::validate_flags(&good).is_ok());
    }

    #[test]
    fn factory_native_storage_preserves_flags_timeout_and_message_families() {
        native_protocol::<FlagsSchema>(
            r##"
pub type Probe
pub type Flag { Intensity(Int) }
@external(erlang, "fixture", "make") fn make(value: a) -> Probe
@external(erlang, "fixture", "check") fn check(a: Probe, b: Probe, c: Probe, expected: String) -> Nil
pub fn main() { check(make([Intensity(1)]), make([Intensity(1)]), make([Intensity(2)]), "[Intensity(1)]") }
"##,
        );
        native_protocol::<TimeoutSchema>(
            r##"
pub type Probe
@external(erlang, "fixture", "make") fn make(value: a) -> Probe
@external(erlang, "fixture", "check") fn check(a: Probe, b: Probe, c: Probe, expected: String) -> Nil
pub fn main() { check(make(5000), make(5000), make(0), "5000") }
"##,
        );
        native_protocol::<MessageSchema>(
            r##"
pub type Probe
@external(erlang, "fixture", "make") fn make(value: a) -> Probe
@external(erlang, "fixture", "check") fn check(a: Probe, b: Probe, c: Probe, expected: String) -> Nil
pub fn main() { check(make(#("request", 1)), make(#("request", 1)), make(#("request", 2)), "#(\"request\", 1)") }
"##,
        );
    }

    #[test]
    fn factory_child_storage_preserves_its_property_protocol_and_retained_callback() {
        let provider = HostProviderModule::new("application", "main").unwrap()
            .with_external_type::<Component<Profile>, Probe<ChildSpecSchema>>().unwrap()
            .with_scoped_function_and_constructions::<Component<Profile>, (HostFunctionType<One<A>, StartResult<B>>, BigInt), ProbeValue<ChildSpecSchema>, HostTypeListEnd, _>("make", make_child).unwrap()
            .with_scoped_function::<Component<Profile>, (ProbeValue<ChildSpecSchema>, ProbeValue<ChildSpecSchema>, ProbeValue<ChildSpecSchema>, StringValue), (), _>("check", check::<ChildSpecSchema>).unwrap();
        assert_eq!(run_source(callback_packages(r##"
import gleam/otp/actor
pub type Probe
@external(erlang, "fixture", "make") fn make(start: fn(a) -> actor.StartResult(b), property: Int) -> Probe
@external(erlang, "fixture", "check") fn check(a: Probe, b: Probe, c: Probe, expected: String) -> Nil
fn start(_: Int) -> actor.StartResult(String) { Error(actor.InitFailed("rejected")) }
pub fn main() { check(make(start, 1), make(start, 1), make(start, 2), "1") }
"##), callback_providers(provider)).unwrap(), geam::Value::Nil);
    }

    fn make_child<'call>(
        mut call: Call<'call, Profile, ProbeValue<ChildSpecSchema>>,
        constructions: HostConstructions<'call, HostTypeListEnd>,
        start: HostCallable<'call, One<A>, StartResult<B>>,
        property: BigInt,
    ) -> Result<HostCallCompletion<'call, ProbeValue<ChildSpecSchema>>, HostCallError> {
        let start = call.owned_callable(start, &constructions);
        let properties = call.native_values().integer(property);
        let value = call.create_external(Arc::new(Child {
            properties,
            restart: Restart::Temporary,
            shutdown: Shutdown::Infinite,
            start,
        }));
        Ok(call.return_value(value))
    }

    #[test]
    fn handle_storage_uses_source_identity_for_each_destination_kind() {
        let provider = HostProviderModule::new("application", "main")
            .unwrap()
            .with_external_type::<Component<Profile>, Probe<HandleSchema>>()
            .unwrap()
            .with_scoped_function::<Component<Profile>, (BigInt, bool), ProbeValue<HandleSchema>, _>("make", make_handle)
            .unwrap()
            .with_scoped_function::<Component<Profile>, (
                ProbeValue<HandleSchema>,
                ProbeValue<HandleSchema>,
                ProbeValue<HandleSchema>,
                StringValue,
            ), (), _>("check", check::<HandleSchema>)
            .unwrap();
        assert_eq!(run_source([geam::PackageSource::new("application", Vec::<String>::new(), [geam::ModuleSource::new("main", "main.gleam", r##"
pub type Probe
@external(erlang, "fixture", "make") fn make(property: Int, pid: Bool) -> Probe
@external(erlang, "fixture", "check") fn check(a: Probe, b: Probe, c: Probe, expected: String) -> Nil
pub fn main() {
  check(make(1, False), make(1, False), make(2, False), "1")
  check(make(1, True), make(1, True), make(2, True), "1")
}
"##)])], [provider]).unwrap(), geam::Value::Nil);
    }
    fn make_handle<'call>(
        mut call: Call<'call, Profile, ProbeValue<HandleSchema>>,
        value: BigInt,
        pid: bool,
    ) -> Result<HostCallCompletion<'call, ProbeValue<HandleSchema>>, HostCallError> {
        let source = call.native_values().integer(value);
        let destination = if pid {
            Destination::Pid(
                call.execution_unit()
                    .expect("storage fixture runs in a source invocation"),
            )
        } else {
            Destination::Name("worker".into())
        };
        let value = call.create_external(Handle {
            source,
            destination,
        });
        Ok(call.return_value(value))
    }
}
