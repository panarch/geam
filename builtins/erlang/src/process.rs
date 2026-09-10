mod identity;
mod mailbox;
mod schema;

use crate::execution::{Destination, Message, Reason, ReferenceId};
use crate::schema::{
    AtomSchema, DoNotLeak, DoNotLeakSchema, Monitor, MonitorSchema, Name, NameSchema, Pid,
    PidSchema, PortSchema, Reference, ReferenceSchema, Selector, SelectorSchema, Timer,
    TimerSchema,
};
use crate::{Component, GleamErlangHostProfile, reference};
use ecow::EcoString;
use futures_util::future::BoxFuture;
use geam_core::host::native::{NativeCall, NativeRules};
use geam_core::host::{
    HostCall, HostCallCompletion, HostCallContinuation, HostCallError, HostCallable,
    HostConstructions, HostCustom, HostExecutionContext, HostExecutionError, HostExternal,
    HostFunctionType, HostOwnedCompletion, HostProviderModule, HostRegistrationError, HostTuple,
    HostTupleType, HostType, HostTypeIndex0, HostTypeList, HostTypeListEnd, HostTypeParameter,
    HostValue,
};
use geam_core::provider::advanced::NativeValue;
use geam_stdlib::provider_support::{Dynamic, DynamicSchema, GleamError, GleamOk, GleamResult};
use num_bigint::{BigInt, Sign};
use schema::{Down, ExitReason, KillFlag, ProcessFlag, Subject};
use std::time::{Duration, Instant};

type A = HostTypeParameter<0>;
type B = HostTypeParameter<1>;
type C = HostTypeParameter<2>;
type One<T> = HostTypeList<T, HostTypeListEnd>;
type Pair<L, R> = HostTupleType<HostTypeList<L, One<R>>>;
type Unary<T, R> = HostFunctionType<One<T>, R>;
type Call<'call, Profile, Return> = HostCall<'call, Profile, Component<Profile>, Return>;
type Native<'call, Profile, Return, Target> =
    NativeCall<'call, Profile, Component<Profile>, Return, One<Target>>;

pub(crate) fn host_provider<Profile: GleamErlangHostProfile>()
-> Result<HostProviderModule<Profile>, HostRegistrationError> {
    HostProviderModule::new("gleam_erlang", "gleam/erlang/process")
        .and_then(|module| module.with_external_type::<Component<Profile>, PidSchema>())
        .and_then(|module| module.with_external_type::<Component<Profile>, NameSchema>())
        .and_then(|module| module.with_external_type::<Component<Profile>, SelectorSchema>())
        .and_then(|module| module.with_external_type::<Component<Profile>, MonitorSchema>())
        .and_then(|module| module.with_external_type::<Component<Profile>, TimerSchema>())
        .and_then(|module| module.with_external_type::<Component<Profile>, DoNotLeakSchema>())
        .and_then(|module| module.with_scoped_function::<Component<Profile>, (), Pid, _>("self", current::<Profile>))
        .and_then(|module| module.with_scoped_function::<Component<Profile>, (HostFunctionType<HostTypeListEnd, A>,), Pid, _>("spawn", spawn::<Profile>))
        .and_then(|module| module.with_scoped_function::<Component<Profile>, (HostFunctionType<HostTypeListEnd, A>,), Pid, _>("spawn_unlinked", spawn_unlinked::<Profile>))
        .and_then(|module| module.with_scoped_function::<Component<Profile>, (EcoString,), Name<A>, _>("new_name", new_name::<Profile>))
        .and_then(|module| module.with_scoped_function::<Component<Profile>, (Pid, A), DoNotLeak, _>("raw_send", raw_send::<Profile>))
        .and_then(|module| module.with_resumable_native_function::<Component<Profile>, (Subject<A>, BigInt), GleamResult<A, ()>, One<GleamResult<A, ()>>, _>("perform_receive", native_rules(), mailbox::receive::<Profile>))
        .and_then(|module| module.with_resumable_native_function::<Component<Profile>, (Subject<A>,), A, One<A>, _>("receive_forever", native_rules(), mailbox::receive_forever::<Profile>))
        .and_then(|module| module.with_scoped_function::<Component<Profile>, (), Selector<A>, _>("new_selector", mailbox::new_selector::<Profile>))
        .and_then(|module| module.with_resumable_native_function::<Component<Profile>, (Selector<A>, BigInt), GleamResult<A, ()>, One<GleamResult<A, ()>>, _>("selector_receive", native_rules(), mailbox::select::<Profile>))
        .and_then(|module| module.with_resumable_native_function::<Component<Profile>, (Selector<A>,), A, One<A>, _>("selector_receive_forever", native_rules(), mailbox::select_forever::<Profile>))
        .and_then(|module| module.with_native_function::<Component<Profile>, (Selector<B>, Unary<B, A>), Selector<A>, One<B>, _>("map_selector", native_rules(), mailbox::map_selector::<Profile>))
        .and_then(|module| module.with_scoped_function::<Component<Profile>, (Selector<A>, Selector<A>), Selector<A>, _>("merge_selector", mailbox::merge_selector::<Profile>))
        .and_then(|module| module.with_native_function::<Component<Profile>, (Selector<A>, B, Unary<C, A>), Selector<A>, HostTypeList<C, One<Down>>, _>("insert_selector_handler", native_rules(), mailbox::insert::<Profile>))
        .and_then(|module| module.with_scoped_function::<Component<Profile>, (Selector<A>, B), Selector<A>, _>("remove_selector_handler", mailbox::remove::<Profile>))
        .and_then(|module| module.with_scoped_function::<Component<Profile>, (), (), _>("flush_messages", flush::<Profile>))
        .and_then(|module| module.with_resumable_function::<Component<Profile>, (BigInt,), (), HostTypeListEnd, _>("sleep", sleep::<Profile>))
        .and_then(|module| module.with_resumable_function::<Component<Profile>, (), (), HostTypeListEnd, _>("sleep_forever", sleep_forever::<Profile>))
        .and_then(|module| module.with_scoped_function::<Component<Profile>, (Pid,), bool, _>("is_alive", alive::<Profile>))
        .and_then(|module| module.with_scoped_function_and_constructions::<Component<Profile>, (ProcessFlag, Pid), Monitor, One<Reference>, _>("erlang_monitor_process", monitor::<Profile>))
        .and_then(|module| module.with_native_function::<Component<Profile>, (Dynamic,), Down, One<Down>, _>("cast_down_message", native_rules(), cast_down::<Profile>))
        .and_then(|module| module.with_scoped_function::<Component<Profile>, (Dynamic,), ExitReason, _>("cast_exit_reason", cast_reason::<Profile>))
        .and_then(|module| module.with_native_function::<Component<Profile>, (Monitor,), DoNotLeak, One<Reference>, _>("erlang_demonitor_process", native_rules(), demonitor::<Profile>))
        .and_then(|module| module.with_scoped_function_and_constructions::<Component<Profile>, (Pid,), bool, One<Pid>, _>("link", link::<Profile>))
        .and_then(|module| module.with_scoped_function::<Component<Profile>, (Pid,), bool, _>("erlang_unlink", unlink::<Profile>))
        .and_then(|module| module.with_scoped_function_and_constructions::<Component<Profile>, (BigInt, Pid, Pair<Dynamic, A>), Timer, One<Reference>, _>("pid_send_after", pid_send_after::<Profile>))
        .and_then(|module| module.with_scoped_function_and_constructions::<Component<Profile>, (BigInt, Name<A>, Pair<Name<A>, A>), Timer, One<Reference>, _>("name_send_after", name_send_after::<Profile>))
        .and_then(|module| module.with_scoped_function::<Component<Profile>, (Reference,), Dynamic, _>("reference_to_dynamic", reference_to_dynamic::<Profile>))
        .and_then(|module| module.with_native_function::<Component<Profile>, (Timer,), Dynamic, One<Reference>, _>("erlang_cancel_timer", native_rules(), cancel_timer::<Profile>))
        .and_then(|module| module.with_scoped_function::<Component<Profile>, (Pid, KillFlag), bool, _>("erlang_kill", kill::<Profile>))
        .and_then(|module| module.with_scoped_function_and_constructions::<Component<Profile>, (Pid, A), bool, One<Pid>, _>("erlang_send_exit", send_exit::<Profile>))
        .and_then(|module| module.with_scoped_function::<Component<Profile>, (bool,), (), _>("trap_exits", trap_exits::<Profile>))
        .and_then(|module| module.with_scoped_function::<Component<Profile>, (Pid, Name<A>), GleamResult<(), ()>, _>("register", register::<Profile>))
        .and_then(|module| module.with_scoped_function::<Component<Profile>, (Name<A>,), GleamResult<(), ()>, _>("unregister", unregister::<Profile>))
        .and_then(|module| module.with_scoped_function_and_constructions::<Component<Profile>, (Name<A>,), GleamResult<Pid, ()>, One<Pid>, _>("named", named::<Profile>))
}

pub(crate) fn port_provider<Profile: GleamErlangHostProfile>()
-> Result<HostProviderModule<Profile>, HostRegistrationError> {
    HostProviderModule::new("gleam_erlang", "gleam/erlang/port")
        .and_then(|module| module.with_external_type::<Component<Profile>, PortSchema>())
}

fn native_rules<Profile: GleamErlangHostProfile, Return: HostType>()
-> NativeRules<Profile, Component<Profile>, Return> {
    NativeRules::default()
        .external::<AtomSchema, HostTypeListEnd>(|call, construction, value| {
            Some(call.construct_external(construction, value))
        })
        .external::<ReferenceSchema, HostTypeListEnd>(|call, construction, value| {
            Some(call.construct_external(construction, reference::Payload::View(value)))
        })
        .external::<DynamicSchema, HostTypeListEnd>(|call, construction, value| {
            Some(call.construct_external(construction, geam_stdlib::Dynamic::from_native(value)))
        })
        .external::<MonitorSchema, HostTypeListEnd>(|call, construction, value| {
            Some(call.construct_external(construction, value))
        })
}

fn current<'call, Profile: GleamErlangHostProfile>(
    call: Call<'call, Profile, Pid>,
) -> Result<HostCallCompletion<'call, Pid>, HostCallError> {
    call.with_execution_unit(|mut call, unit| {
        let pid = call.create_external(unit);
        Ok(call.return_value(pid))
    })
}

fn spawn_unlinked<'call, Profile: GleamErlangHostProfile>(
    mut call: Call<'call, Profile, Pid>,
    callback: HostCallable<'call, HostTypeListEnd, A>,
) -> Result<HostCallCompletion<'call, Pid>, HostCallError> {
    let child = call.spawn(callback);
    let pid = call.create_external(child);
    Ok(call.return_value(pid))
}

fn spawn<'call, Profile: GleamErlangHostProfile>(
    call: Call<'call, Profile, Pid>,
    callback: HostCallable<'call, HostTypeListEnd, A>,
) -> Result<HostCallCompletion<'call, Pid>, HostCallError> {
    call.with_execution_unit(|mut call, parent| {
        let child = call.spawn(callback);
        let from = call.create_external(parent.clone());
        let to = call.create_external(child.clone());
        let from_value = call.native_value::<Pid>(from);
        let to_value = call.native_value::<Pid>(to);
        Profile::erlang_execution(call.execution_state()).link(
            parent.id(),
            child.id(),
            from_value,
            to_value,
        );
        Ok(call.return_value(to))
    })
}

fn new_name<'call, Profile: GleamErlangHostProfile>(
    mut call: Call<'call, Profile, Name<A>>,
    prefix: EcoString,
) -> Result<HostCallCompletion<'call, Name<A>>, HostCallError> {
    let name = format!("{prefix}${}", ReferenceId::new().number()).into();
    let name = Profile::erlang_execution(call.execution_state()).intern(name)?;
    let name = call.create_external(name);
    Ok(call.return_value(name))
}

fn raw_send<'call, Profile: GleamErlangHostProfile>(
    mut call: Call<'call, Profile, DoNotLeak>,
    pid: HostExternal<'call, Pid>,
    value: HostValue<'call, A>,
) -> Result<HostCallCompletion<'call, DoNotLeak>, HostCallError> {
    let pid = call.external_payload(pid).id();
    let value = call.native_value::<A>(value);
    Profile::erlang_execution(call.execution_state()).send(pid, Message::Source(value.clone()));
    let value = call.create_external(value);
    Ok(call.return_value(value))
}

fn flush<'call, Profile: GleamErlangHostProfile>(
    call: Call<'call, Profile, ()>,
) -> Result<HostCallCompletion<'call, ()>, HostCallError> {
    call.with_execution_unit(|mut call, unit| {
        if let Some(mailbox) = Profile::erlang_execution(call.execution_state()).mailbox(unit.id())
        {
            mailbox.clear();
        }
        Ok(call.return_value(()))
    })
}

fn sleep<'call, Profile: GleamErlangHostProfile>(
    call: Call<'call, Profile, ()>,
    constructions: HostConstructions<'call, HostTypeListEnd>,
    mut delay: BigInt,
) -> Result<HostCallContinuation<'call, ()>, HostCallError> {
    if delay.sign() == Sign::Minus {
        return Err(geam_core::HostFailure::new("sleep timeout must be non-negative").into());
    }
    let wait = sleep_part(&call, &mut delay)?;
    Ok(call.resume(constructions, move |context| {
        Box::pin(sleep_chunks(context, delay, wait))
    }))
}

async fn sleep_chunks<Profile: GleamErlangHostProfile>(
    context: HostExecutionContext<'_, Profile, Component<Profile>, HostTypeListEnd>,
    mut remaining: BigInt,
    mut wait: BoxFuture<'static, ()>,
) -> Result<HostOwnedCompletion<Profile, Component<Profile>, (), HostTypeListEnd>, HostExecutionError>
{
    loop {
        wait.await;
        if remaining == BigInt::ZERO {
            break;
        }
        (remaining, wait) = context
            .with_call(move |call| sleep_part(&call, &mut remaining).map(|wait| (remaining, wait)))
            .await??;
    }
    Ok(HostOwnedCompletion::new(
        |call, _| Ok(call.return_value(())),
    ))
}

fn sleep_part<Profile: GleamErlangHostProfile>(
    call: &Call<'_, Profile, ()>,
    remaining: &mut BigInt,
) -> Result<BoxFuture<'static, ()>, HostCallError> {
    // Like OTP timer:sleep, retain large delays and wait in finite chunks.
    let millis = u32::try_from(&*remaining).unwrap_or(u32::MAX);
    *remaining -= millis;
    let deadline = deadline(call.clock().now(), millis.into())?;
    Ok(call.clock().sleep_until(deadline))
}

fn deadline(now: Instant, millis: u64) -> Result<Instant, HostCallError> {
    now.checked_add(Duration::from_millis(millis))
        .ok_or_else(|| geam_core::HostFailure::new("timeout exceeds the host clock range").into())
}

fn receive_deadline(now: Instant, delay: BigInt) -> Result<Instant, HostCallError> {
    let millis = u32::try_from(&delay).map_err(|_| {
        geam_core::HostFailure::new("receive timeout must be between 0 and 4294967295 milliseconds")
    })?;
    deadline(now, millis.into())
}

fn sleep_forever<'call, Profile: GleamErlangHostProfile>(
    call: Call<'call, Profile, ()>,
    constructions: HostConstructions<'call, HostTypeListEnd>,
) -> Result<HostCallContinuation<'call, ()>, HostCallError> {
    Ok(call.resume(constructions, |_| Box::pin(std::future::pending())))
}

fn alive<'call, Profile: GleamErlangHostProfile>(
    mut call: Call<'call, Profile, bool>,
    pid: HostExternal<'call, Pid>,
) -> Result<HostCallCompletion<'call, bool>, HostCallError> {
    let pid = call.external_payload(pid).id();
    let alive = Profile::erlang_execution(call.execution_state()).alive(pid);
    Ok(call.return_value(alive))
}

fn monitor<'call, Profile: GleamErlangHostProfile>(
    call: Call<'call, Profile, Monitor>,
    constructions: HostConstructions<'call, One<Reference>>,
    _flag: HostCustom<'call, ProcessFlag>,
    target: HostExternal<'call, Pid>,
) -> Result<HostCallCompletion<'call, Monitor>, HostCallError> {
    call.with_execution_unit(|mut call, owner| {
        let target_id = call.external_payload(target).id();
        let target = call.native_value::<Pid>(target);
        let id = ReferenceId::new();
        let reference = call.construct_external(
            constructions.at::<HostTypeIndex0>(),
            reference::Payload::Identity(id),
        );
        let reference = call.native_value::<Reference>(reference);
        Profile::erlang_execution(call.execution_state()).monitor(
            owner.id(),
            target_id,
            id,
            reference.clone(),
            target,
        );
        let monitor = call.create_external(reference);
        Ok(call.return_value(monitor))
    })
}

fn cast_down<'call, Profile: GleamErlangHostProfile>(
    mut call: Native<'call, Profile, Down, Down>,
    value: HostExternal<'call, Dynamic>,
) -> Result<HostCallCompletion<'call, Down>, HostCallError> {
    let value = call.call().external_payload(value).native_value().clone();
    let value = mailbox::down_message(&value)?;
    let value = call
        .convert::<HostTypeIndex0>(&value)
        .ok_or_else(|| geam_core::HostFailure::new("DOWN message has invalid fields"))?;
    Ok(call.finish(value))
}

fn cast_reason<'call, Profile: GleamErlangHostProfile>(
    call: Call<'call, Profile, ExitReason>,
    value: HostExternal<'call, Dynamic>,
) -> Result<HostCallCompletion<'call, ExitReason>, HostCallError> {
    let reason = call.external_payload(value).native_value().as_symbol();
    match reason.as_deref() {
        Some("normal") => Ok(call.return_custom::<schema::NormalReason>(())),
        Some("killed") => Ok(call.return_custom::<schema::KilledReason>(())),
        _ => Ok(call.return_custom::<schema::AbnormalReason>((value, ()))),
    }
}

fn demonitor<'call, Profile: GleamErlangHostProfile>(
    call: Native<'call, Profile, DoNotLeak, Reference>,
    monitor: HostExternal<'call, Monitor>,
) -> Result<HostCallCompletion<'call, DoNotLeak>, HostCallError> {
    call.with_execution_unit(|mut call, owner| {
        let reference = call.call().external_payload(monitor).clone();
        let id = reference_id(&mut call, &reference)?;
        Profile::erlang_execution(call.call().execution_state()).demonitor(owner.id(), id);
        let value = call.call().create_external(NativeValue::symbol("true"));
        Ok(call.finish(value))
    })
}

fn reference_id<Profile: GleamErlangHostProfile, Return: HostType>(
    call: &mut Native<'_, Profile, Return, Reference>,
    value: &NativeValue,
) -> Result<ReferenceId, HostCallError> {
    call.convert::<HostTypeIndex0>(value)
        .and_then(
            |reference| match &*call.call().external_payload(reference) {
                reference::Payload::Identity(id) => Some(*id),
                reference::Payload::View(_) => None,
            },
        )
        .ok_or_else(|| geam_core::HostFailure::new("operation requires a reference").into())
}

fn link<'call, Profile: GleamErlangHostProfile>(
    call: Call<'call, Profile, bool>,
    constructions: HostConstructions<'call, One<Pid>>,
    target: HostExternal<'call, Pid>,
) -> Result<HostCallCompletion<'call, bool>, HostCallError> {
    call.with_execution_unit(|mut call, from| {
        let to = call.external_payload(target).id();
        let target = call.native_value::<Pid>(target);
        let source = call.construct_external(constructions.at::<HostTypeIndex0>(), from.clone());
        let source = call.native_value::<Pid>(source);
        let value =
            Profile::erlang_execution(call.execution_state()).link(from.id(), to, source, target);
        Ok(call.return_value(value))
    })
}

fn unlink<'call, Profile: GleamErlangHostProfile>(
    call: Call<'call, Profile, bool>,
    target: HostExternal<'call, Pid>,
) -> Result<HostCallCompletion<'call, bool>, HostCallError> {
    call.with_execution_unit(|mut call, from| {
        let target = call.external_payload(target).id();
        Profile::erlang_execution(call.execution_state()).unlink(from.id(), target);
        Ok(call.return_value(true))
    })
}

fn pid_send_after<'call, Profile: GleamErlangHostProfile>(
    mut call: Call<'call, Profile, Timer>,
    constructions: HostConstructions<'call, One<Reference>>,
    delay: BigInt,
    target: HostExternal<'call, Pid>,
    message: HostTuple<'call, HostTypeList<Dynamic, One<A>>>,
) -> Result<HostCallCompletion<'call, Timer>, HostCallError> {
    let destination = Destination::Pid(call.external_payload(target).id());
    let message = call.native_value::<Pair<Dynamic, A>>(message);
    send_after(&mut call, &constructions, delay, destination, message)
        .map(|timer| call.return_value(timer))
}

fn name_send_after<'call, Profile: GleamErlangHostProfile>(
    mut call: Call<'call, Profile, Timer>,
    constructions: HostConstructions<'call, One<Reference>>,
    delay: BigInt,
    target: HostExternal<'call, Name<A>>,
    message: HostTuple<'call, HostTypeList<Name<A>, One<A>>>,
) -> Result<HostCallCompletion<'call, Timer>, HostCallError> {
    let name = call.external_payload(target).clone();
    let message = call.native_value::<Pair<Name<A>, A>>(message);
    send_after(
        &mut call,
        &constructions,
        delay,
        Destination::Name(name),
        message,
    )
    .map(|timer| call.return_value(timer))
}

fn send_after<'call, Profile: GleamErlangHostProfile>(
    call: &mut Call<'call, Profile, Timer>,
    constructions: &HostConstructions<'call, One<Reference>>,
    delay: BigInt,
    destination: Destination,
    message: NativeValue,
) -> Result<HostExternal<'call, Timer>, HostCallError> {
    let millis = u64::try_from(&delay).map_err(|_| {
        geam_core::HostFailure::new("timer timeout must fit an unsigned 64-bit millisecond count")
    })?;
    let deadline = deadline(call.clock().now(), millis)?;
    let id = ReferenceId::new();
    let reference = call.construct_external(
        constructions.at::<HostTypeIndex0>(),
        reference::Payload::Identity(id),
    );
    let reference = call.native_value::<Reference>(reference);
    Profile::erlang_execution(call.execution_state()).schedule(id, deadline, destination, message);
    Ok(call.create_external(reference))
}

fn reference_to_dynamic<'call, Profile: GleamErlangHostProfile>(
    mut call: Call<'call, Profile, Dynamic>,
    reference: HostExternal<'call, Reference>,
) -> Result<HostCallCompletion<'call, Dynamic>, HostCallError> {
    let reference = call.native_value::<Reference>(reference);
    let value = call.create_external(geam_stdlib::Dynamic::from_native(reference));
    Ok(call.return_value(value))
}

fn cancel_timer<'call, Profile: GleamErlangHostProfile>(
    mut call: Native<'call, Profile, Dynamic, Reference>,
    timer: HostExternal<'call, Timer>,
) -> Result<HostCallCompletion<'call, Dynamic>, HostCallError> {
    let value = call.call().external_payload(timer).clone();
    let id = reference_id(&mut call, &value)?;
    let now = call.call().clock().now();
    let remaining = Profile::erlang_execution(call.call().execution_state()).cancel_timer(id, now);
    let value = match remaining {
        Some(millis) => call.call().native_value::<BigInt>(millis.into()),
        None => NativeValue::symbol("false"),
    };
    let value = call
        .call()
        .create_external(geam_stdlib::Dynamic::from_native(value));
    Ok(call.finish(value))
}

fn kill<'call, Profile: GleamErlangHostProfile>(
    mut call: Call<'call, Profile, bool>,
    target: HostExternal<'call, Pid>,
    _flag: HostCustom<'call, KillFlag>,
) -> Result<HostCallCompletion<'call, bool>, HostCallError> {
    let target = call.external_payload(target).id();
    Profile::erlang_execution(call.execution_state()).terminate(target, Reason::Killed);
    Ok(call.return_value(true))
}

fn send_exit<'call, Profile: GleamErlangHostProfile>(
    call: Call<'call, Profile, bool>,
    constructions: HostConstructions<'call, One<Pid>>,
    target: HostExternal<'call, Pid>,
    reason: HostValue<'call, A>,
) -> Result<HostCallCompletion<'call, bool>, HostCallError> {
    call.with_execution_unit(|mut call, from| {
        let target = call.external_payload(target).id();
        let reason = call.native_value::<A>(reason);
        let from_value =
            call.construct_external(constructions.at::<HostTypeIndex0>(), from.clone());
        let from_value = call.native_value::<Pid>(from_value);
        let runtime = Profile::erlang_execution(call.execution_state());
        match reason.as_symbol().as_deref() {
            Some("kill") => runtime.terminate(target, Reason::Killed),
            Some("normal") => {
                runtime.signal(target, from_value, Reason::Normal, from.id() == target)
            }
            _ => runtime.signal(
                target,
                from_value,
                Reason::Native(reason),
                from.id() == target,
            ),
        }
        Ok(call.return_value(true))
    })
}

fn trap_exits<'call, Profile: GleamErlangHostProfile>(
    call: Call<'call, Profile, ()>,
    enabled: bool,
) -> Result<HostCallCompletion<'call, ()>, HostCallError> {
    call.with_execution_unit(|mut call, unit| {
        Profile::erlang_execution(call.execution_state()).trap_exits(unit.id(), enabled);
        Ok(call.return_value(()))
    })
}

fn register<'call, Profile: GleamErlangHostProfile>(
    mut call: Call<'call, Profile, GleamResult<(), ()>>,
    target: HostExternal<'call, Pid>,
    name: HostExternal<'call, Name<A>>,
) -> Result<HostCallCompletion<'call, GleamResult<(), ()>>, HostCallError> {
    let target = call.external_payload(target).id();
    let name = call.external_payload(name).clone();
    if Profile::erlang_execution(call.execution_state()).register(target, name) {
        Ok(call.return_custom::<GleamOk<(), ()>>(((), ())))
    } else {
        Ok(call.return_custom::<GleamError<(), ()>>(((), ())))
    }
}

fn unregister<'call, Profile: GleamErlangHostProfile>(
    mut call: Call<'call, Profile, GleamResult<(), ()>>,
    name: HostExternal<'call, Name<A>>,
) -> Result<HostCallCompletion<'call, GleamResult<(), ()>>, HostCallError> {
    let name = call.external_payload(name).clone();
    if Profile::erlang_execution(call.execution_state()).unregister(&name) {
        Ok(call.return_custom::<GleamOk<(), ()>>(((), ())))
    } else {
        Ok(call.return_custom::<GleamError<(), ()>>(((), ())))
    }
}

fn named<'call, Profile: GleamErlangHostProfile>(
    mut call: Call<'call, Profile, GleamResult<Pid, ()>>,
    constructions: HostConstructions<'call, One<Pid>>,
    name: HostExternal<'call, Name<A>>,
) -> Result<HostCallCompletion<'call, GleamResult<Pid, ()>>, HostCallError> {
    let name = call.external_payload(name).clone();
    match Profile::erlang_execution(call.execution_state()).named(&name) {
        Some(pid) => {
            let pid = call.construct_external(constructions.at::<HostTypeIndex0>(), pid);
            Ok(call.return_custom::<GleamOk<Pid, ()>>((pid, ())))
        }
        None => Ok(call.return_custom::<GleamError<Pid, ()>>(((), ()))),
    }
}

#[cfg(test)]
mod tests {
    use super::{Call, One, native_rules, schema::Down};
    use crate::execution_fixture::TestHost;
    use crate::schema::{MonitorSchema, Pid, PidSchema, Reference, Timer, TimerSchema};
    use crate::{Component, Configuration, GleamErlangProfile, GleamErlangRunState};
    use geam_core::embedding::{FunctionDeclaration, HostedModuleBuilder};
    use geam_core::execution::ExecutionHost;
    use geam_core::host::{
        HostCallCompletion, HostCallContinuation, HostCallError, HostConstructions,
        HostProviderModule, HostProviderSet, HostTypeIndex0, HostTypeIndexNext, HostTypeList,
        HostTypeListEnd,
    };
    use geam_core::provider::advanced::NativeValue;
    use geam_core::{ModuleSource, PackageSource, compile_typed_host_program};
    use num_bigint::BigInt;
    use std::time::Duration;

    fn foreign_message<'call>(
        mut call: Call<'call, GleamErlangProfile, geam_stdlib::provider_support::Dynamic>,
        constructions: HostConstructions<'call, HostTypeList<Pid, One<Reference>>>,
        case: BigInt,
    ) -> Result<HostCallCompletion<'call, geam_stdlib::provider_support::Dynamic>, HostCallError>
    {
        let unit = call.execution_unit().unwrap();
        let pid = call.construct_external(constructions.at::<HostTypeIndex0>(), unit);
        let pid = call.native_value::<Pid>(pid);
        let reference = if case == BigInt::from(4) {
            let reference = call.construct_external(
                constructions.at::<HostTypeIndexNext<HostTypeIndex0>>(),
                crate::reference::Payload::Identity(crate::execution::ReferenceId::new()),
            );
            call.native_value::<Reference>(reference)
        } else {
            NativeValue::symbol("not_a_reference")
        };
        let message = if case == BigInt::ZERO {
            NativeValue::symbol("not_an_envelope")
        } else {
            NativeValue::tuple([
                NativeValue::symbol("DOWN"),
                reference,
                NativeValue::symbol(if case == BigInt::from(1) {
                    "unknown"
                } else {
                    "process"
                }),
                if case == BigInt::from(2) {
                    call.native_value::<BigInt>(42.into())
                } else {
                    pid
                },
                NativeValue::symbol("normal"),
            ])
        };
        let value = call.create_external(geam_stdlib::Dynamic::from_native(message));
        Ok(call.return_value(value))
    }

    fn foreign_timer<'call>(
        mut call: Call<'call, GleamErlangProfile, Timer>,
        constructions: HostConstructions<'call, One<Reference>>,
        delay: BigInt,
    ) -> Result<HostCallCompletion<'call, Timer>, HostCallError> {
        let timer = if delay.sign() == num_bigint::Sign::Minus {
            call.create_external(NativeValue::symbol("not_a_reference"))
        } else {
            super::send_after(
                &mut call,
                &constructions,
                delay,
                crate::execution::Destination::Name("late_bound".into()),
                NativeValue::symbol("message"),
            )?
        };
        Ok(call.return_value(timer))
    }

    #[test]
    fn foreign_envelopes_and_identity_views_fail_at_the_operation_that_requires_them() {
        use geam_stdlib::provider_support::{Dynamic, DynamicSchema};
        let provider = HostProviderModule::<GleamErlangProfile>::new(
            "gleam_erlang", "gleam/erlang/process",
        ).unwrap()
            .with_external_type::<Component<GleamErlangProfile>, PidSchema>().unwrap()
            .with_external_type::<Component<GleamErlangProfile>, MonitorSchema>().unwrap()
            .with_external_type::<Component<GleamErlangProfile>, TimerSchema>().unwrap()
            .with_external_type::<Component<GleamErlangProfile>, crate::schema::DoNotLeakSchema>().unwrap()
            .with_scoped_function_and_constructions::<Component<GleamErlangProfile>, (BigInt,), Dynamic, HostTypeList<Pid, One<Reference>>, _>(
                "foreign_message", foreign_message,
            ).unwrap()
            .with_scoped_function_and_constructions::<Component<GleamErlangProfile>, (BigInt,), Timer, One<Reference>, _>(
                "foreign_timer", foreign_timer,
            ).unwrap()
            .with_native_function::<Component<GleamErlangProfile>, (Dynamic,), Down, One<Down>, _>(
                "cast_down_message", native_rules(), super::cast_down,
            ).unwrap()
            .with_native_function::<Component<GleamErlangProfile>, (super::Monitor,), super::DoNotLeak, One<super::Reference>, _>(
                "erlang_demonitor_process", native_rules(), super::demonitor,
            ).unwrap()
            .with_native_function::<Component<GleamErlangProfile>, (Timer,), Dynamic, One<super::Reference>, _>(
                "erlang_cancel_timer", native_rules(), super::cancel_timer,
            ).unwrap();
        let typed = compile_typed_host_program(
            "gleam_erlang", "gleam/erlang/process",
            [
                PackageSource::new("gleam_stdlib", Vec::<String>::new(), [
                    ModuleSource::new("gleam/dynamic", "dynamic.gleam", "pub type Dynamic"),
                ]),
                PackageSource::new("gleam_erlang", ["gleam_stdlib"], [
                    ModuleSource::new("gleam/erlang/atom", "atom.gleam", "pub type Atom"),
                    ModuleSource::new("gleam/erlang/port", "port.gleam", "pub type Port"),
                    ModuleSource::new("gleam/erlang/reference", "reference.gleam", "pub type Reference"),
                    ModuleSource::new("gleam/erlang/process", "process.gleam", r#"
import gleam/dynamic.{type Dynamic}
import gleam/erlang/port.{type Port}
pub type Pid
pub type Monitor
pub type Timer
type DoNotLeak
pub type ExitReason { Normal Killed Abnormal(reason: Dynamic) }
pub type Down {
  ProcessDown(monitor: Monitor, pid: Pid, reason: ExitReason)
  PortDown(monitor: Monitor, port: Port, reason: ExitReason)
}
@external(erlang, "fixture", "foreign_message") fn foreign_message(kind: Int) -> Dynamic
@external(erlang, "fixture", "foreign_timer") fn foreign_timer(delay: Int) -> Timer
@external(erlang, "host", "cast_down_message") fn cast_down_message(value: Dynamic) -> Down
@external(erlang, "host", "demonitor") fn erlang_demonitor_process(monitor: Monitor) -> DoNotLeak
@external(erlang, "host", "cancel_timer") fn erlang_cancel_timer(timer: Timer) -> Dynamic
pub fn check(kind: Int) {
  let _ = cast_down_message(foreign_message(kind))
  Nil
}
pub fn check_monitor(kind: Int) {
  let assert ProcessDown(monitor, _, _) = cast_down_message(foreign_message(kind))
  erlang_demonitor_process(monitor)
  Nil
}
pub fn check_timer(delay: Int) {
  let timer = foreign_timer(delay)
  let remaining = erlang_cancel_timer(timer)
  let repeated = erlang_cancel_timer(timer)
  echo #(remaining, repeated)
  Nil
}
"#),
                ]),
            ],
            HostProviderSet::from_providers([
                provider,
                super::port_provider().unwrap(),
                HostProviderModule::new("gleam_erlang", "gleam/erlang/atom").unwrap()
                    .with_external_type::<Component<GleamErlangProfile>, crate::schema::AtomSchema>().unwrap(),
                HostProviderModule::new("gleam_erlang", "gleam/erlang/reference").unwrap()
                    .with_external_type::<Component<GleamErlangProfile>, crate::schema::ReferenceSchema>().unwrap(),
                HostProviderModule::new("gleam_stdlib", "gleam/dynamic").unwrap()
                    .with_external_type::<Component<GleamErlangProfile>, DynamicSchema>().unwrap(),
            ]).unwrap(),
        ).unwrap();
        let (mut builder, check) = HostedModuleBuilder::<GleamErlangProfile>::new(typed)
            .unwrap()
            .function(FunctionDeclaration::<(BigInt,), ()>::new("check"))
            .unwrap();
        let monitor = builder
            .function(FunctionDeclaration::<(BigInt,), ()>::new("check_monitor"))
            .unwrap();
        let timer = builder
            .function(FunctionDeclaration::<(BigInt,), ()>::new("check_timer"))
            .unwrap();
        let mut module = builder.seal().unwrap();
        let host = TestHost::default();
        let mut state = GleamErlangRunState {
            stdlib: geam_stdlib::GleamStdlibRunState::from_seed([0; 32]),
            erlang: Configuration::default(),
        };
        let mut echo = Vec::new();
        host.block_on(module.with_execution(&host, &mut state, &mut echo, async |scope| {
            for (case, expected) in [
                (0, "host function gleam_erlang::gleam/erlang/process.cast_down_message failed: expected a DOWN message"),
                (1, "host function gleam_erlang::gleam/erlang/process.cast_down_message failed: expected a DOWN message"),
                (2, "host function gleam_erlang::gleam/erlang/process.cast_down_message failed: DOWN message has invalid fields"),
            ] {
                assert_eq!(scope.call(&check, (case.into(),)).await.unwrap_err().to_string(), expected);
            }
            assert_eq!(scope.call(&check, (3.into(),)).await, Ok(()));
            assert_eq!(scope.call(&monitor, (4.into(),)).await, Ok(()));
            assert_eq!(scope.call(&monitor, (3.into(),)).await.unwrap_err().to_string(),
                "host function gleam_erlang::gleam/erlang/process.erlang_demonitor_process failed: operation requires a reference");
            assert_eq!(scope.call(&timer, ((-1).into(),)).await.unwrap_err().to_string(),
                "host function gleam_erlang::gleam/erlang/process.erlang_cancel_timer failed: operation requires a reference");
            assert_eq!(scope.call(&timer, (BigInt::from(u64::MAX) + 1,)).await.unwrap_err().to_string(),
                "host function gleam_erlang::gleam/erlang/process.foreign_timer failed: timer timeout must fit an unsigned 64-bit millisecond count");
            assert_eq!(scope.call(&timer, (10.into(),)).await, Ok(()));
            advance_to_clock_limit(&host, Duration::ZERO);
            assert_eq!(scope.call(&timer, (1.into(),)).await.unwrap_err().to_string(),
                "host function gleam_erlang::gleam/erlang/process.foreign_timer failed: timeout exceeds the host clock range");
        })).unwrap();
        assert_eq!(
            echo.iter()
                .map(|output| output.value().inspect().to_string())
                .collect::<Vec<_>>(),
            ["#(10, False)"]
        );
    }

    #[test]
    fn host_clock_exhaustion_is_reported_before_sleep_and_after_a_large_sleep_chunk() {
        let provider = HostProviderModule::new("application", "main")
            .unwrap()
            .with_resumable_function::<
                Component<GleamErlangProfile>,
                (BigInt,),
                (),
                HostTypeListEnd,
                _,
            >("sleep", super::sleep)
            .unwrap();
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
@external(erlang, "timer", "sleep") fn sleep(delay: Int) -> Nil
pub fn wait(delay: Int) { sleep(delay) }
"#,
                )],
            )],
            HostProviderSet::from_providers([provider]).unwrap(),
        )
        .unwrap();
        let (builder, wait) = HostedModuleBuilder::<GleamErlangProfile>::new(typed)
            .unwrap()
            .function(FunctionDeclaration::<(BigInt,), ()>::new("wait"))
            .unwrap();
        let mut module = builder.seal().unwrap();
        let mut state = GleamErlangRunState {
            stdlib: geam_stdlib::GleamStdlibRunState::from_seed([0; 32]),
            erlang: Configuration::default(),
        };
        let mut echo = Vec::new();
        let host = TestHost::default();
        host.block_on(module.with_execution(&host, &mut state, &mut echo, async |scope| {
            assert_eq!(scope.call(&wait, (BigInt::ZERO,)).await, Ok(()));
            assert_eq!(scope.call(&wait, ((-1).into(),)).await.unwrap_err().to_string(),
                "host function application::main.sleep failed: sleep timeout must be non-negative");
        })).unwrap();
        {
            let mut execution = Box::pin(module.with_execution(
                &host,
                &mut state,
                &mut echo,
                async |scope| scope.call(&wait, (BigInt::from(u32::MAX) + 1,)).await,
            ));
            assert!(host.poll(execution.as_mut()).is_pending());
            host.advance(Duration::from_millis(u32::MAX.into()));
            assert!(host.poll(execution.as_mut()).is_pending());
            host.advance(Duration::from_millis(1));
            assert_eq!(host.block_on(execution.as_mut()).unwrap(), Ok(()));
        }
        for before_chunk in [false, true] {
            let host = TestHost::default();
            let chunk = Duration::from_millis(u32::MAX.into());
            advance_to_clock_limit(&host, if before_chunk { chunk } else { Duration::ZERO });
            let delay = if before_chunk {
                BigInt::from(u32::MAX) + 1
            } else {
                BigInt::from(1)
            };
            let mut execution = Box::pin(module.with_execution(
                &host,
                &mut state,
                &mut echo,
                async |scope| scope.call(&wait, (delay,)).await,
            ));
            if before_chunk {
                assert!(host.poll(execution.as_mut()).is_pending());
                host.advance(chunk);
            }
            let error = host
                .block_on(std::future::poll_fn(|_| host.poll(execution.as_mut())))
                .unwrap()
                .unwrap_err();
            assert_eq!(
                error.to_string(),
                "host function application::main.sleep failed: timeout exceeds the host clock range"
            );
            drop(execution);
            assert_eq!(
                super::receive_deadline(host.now(), 1.into())
                    .unwrap_err()
                    .to_string(),
                "timeout exceeds the host clock range"
            );
        }
        assert!(echo.is_empty());
    }

    fn cancelled_sleep(
        clock: TestHost,
        gate: std::sync::Arc<crate::test_support::PollGate>,
    ) -> impl for<'call> Fn(
        Call<'call, GleamErlangProfile, ()>,
        HostConstructions<'call, HostTypeListEnd>,
        BigInt,
    ) -> Result<HostCallContinuation<'call, ()>, HostCallError> {
        move |call, constructions, mut delay| {
            let unit = call.execution_unit().unwrap();
            let wait = super::sleep_part(&call, &mut delay).unwrap();
            clock.advance(Duration::from_millis(u32::MAX.into()));
            let gate = std::sync::Arc::clone(&gate);
            Ok(call.resume(constructions, move |context| {
                Box::pin(async move {
                    gate.observe(super::sleep_chunks(context, delay, wait), unit)
                        .await
                })
            }))
        }
    }

    #[test]
    fn cancellation_during_the_next_sleep_chunk_rejects_the_queued_clock_request() {
        let host = TestHost::default();
        let clock = host.clone();
        let (gate, driver) = crate::test_support::PollGate::new();
        let provider = HostProviderModule::new("application", "main").unwrap()
            .with_resumable_function::<Component<GleamErlangProfile>, (BigInt,), (), HostTypeListEnd, _>(
                "sleep", cancelled_sleep(clock, gate),
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
@external(erlang, "fixture", "sleep") fn sleep(delay: Int) -> Nil
pub fn wait() { sleep(4294967296) echo "must not run after cancellation" Nil }
"#,
                )],
            )],
            HostProviderSet::from_providers([provider]).unwrap(),
        )
        .unwrap();
        let (builder, wait) = HostedModuleBuilder::<GleamErlangProfile>::new(typed)
            .unwrap()
            .function(FunctionDeclaration::<(), ()>::new("wait"))
            .unwrap();
        let mut module = builder.seal().unwrap();
        let mut state = GleamErlangRunState {
            stdlib: geam_stdlib::GleamStdlibRunState::from_seed([0; 32]),
            erlang: Configuration::default(),
        };
        let mut echo = Vec::new();
        let mut execution = Box::pin(module.with_execution(
            &host,
            &mut state,
            &mut echo,
            async |scope| scope.call(&wait, ()).await,
        ));
        driver.cancel(&host, execution.as_mut());
        assert_eq!(
            host.block_on(execution.as_mut()).unwrap(),
            Err(geam_core::embedding::CallError::Cancelled)
        );
        drop(execution);
        assert!(echo.is_empty());
    }

    fn advance_to_clock_limit(host: &TestHost, remaining: Duration) {
        // Locate the platform's Instant limit without changing process-global time.
        let nanos = |value: u128| {
            Duration::new(
                (value / 1_000_000_000) as u64,
                (value % 1_000_000_000) as u32,
            )
        };
        let (mut low, mut high) = (0, Duration::MAX.as_nanos());
        while low < high {
            let middle = low + (high - low).div_ceil(2);
            if host.now().checked_add(nanos(middle)).is_some() {
                low = middle;
            } else {
                high = middle - 1;
            }
        }
        host.advance(nanos(low) - remaining);
    }
}
