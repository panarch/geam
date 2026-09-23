use crate::schema::{
    DebugOption, DebugState, DebugStateSchema, DoNotLeak, DoNotLeakSchema, StatusInfo,
};
use crate::{A, Call, Component, Four, One, OtpProfile, Two};
use geam::execution::ExecutionUnit;
use geam::gleam_erlang::service::{
    CurrentProcess, Processes, new_reference, pid_value, with_current_process,
};
use geam::gleam_erlang::{Pid, Reference};
use geam::gleam_stdlib::provider_support::Dynamic;
use geam::host::native::{NativeCall, NativeRules};
use geam::host::{
    HostCallCompletion, HostCallContinuation, HostCallError, HostCallableSchema, HostCaptures,
    HostConstructions, HostCreatedFunction, HostCustom, HostExternal, HostFunctionType, HostList,
    HostListType, HostOwnedCompletion, HostProviderModule, HostRegistrationError, HostReturns,
    HostType, HostTypeAt, HostTypeIndex0, HostTypeIndexNext, HostTypeList, HostTypeListEnd,
    HostTypeSequence, HostValue,
};
use geam::provider::advanced::NativeValue;

type Captures = Two<Pid, Reference>;
type RequestTypes<Reply> =
    HostTypeList<Pid, HostTypeList<Reference, One<HostCreatedFunction<Reply>>>>;
type Index1 = HostTypeIndexNext<HostTypeIndex0>;
type Index2 = HostTypeIndexNext<Index1>;
type Index3 = HostTypeIndexNext<Index2>;
type StatusTypes = Four<Pid, Reference, HostCreatedFunction<StatusReply>, StatusInfo>;

trait Reply: HostCallableSchema<Captures = Captures, Return = ()> {
    const MESSAGE: &'static str;
}

struct StateReply;
impl HostCallableSchema for StateReply {
    const PACKAGE: &'static str = "gleam_otp";
    const MODULE: &'static str = "gleam/otp/system";
    const NAME: &'static str = "state_reply";
    type Arguments = One<Dynamic>;
    type Return = ();
    type Captures = Captures;
    type Constructions = HostTypeListEnd;
    type Completion = HostReturns;
}
impl Reply for StateReply {
    const MESSAGE: &'static str = "get_state";
}

struct StatusReply;
impl HostCallableSchema for StatusReply {
    const PACKAGE: &'static str = "otp_service_fixture";
    const MODULE: &'static str = "otp_service_support";
    const NAME: &'static str = "status_reply";
    type Arguments = One<StatusInfo>;
    type Return = ();
    type Captures = Captures;
    type Constructions = HostTypeListEnd;
    type Completion = HostReturns;
}
impl Reply for StatusReply {
    const MESSAGE: &'static str = "get_status";
}

struct SuspendReply;
impl HostCallableSchema for SuspendReply {
    const PACKAGE: &'static str = "gleam_otp";
    const MODULE: &'static str = "gleam/otp/system";
    const NAME: &'static str = "suspend_reply";
    type Arguments = HostTypeListEnd;
    type Return = ();
    type Captures = Captures;
    type Constructions = HostTypeListEnd;
    type Completion = HostReturns;
}
impl Reply for SuspendReply {
    const MESSAGE: &'static str = "suspend";
}

struct ResumeReply;
impl HostCallableSchema for ResumeReply {
    const PACKAGE: &'static str = "gleam_otp";
    const MODULE: &'static str = "gleam/otp/system";
    const NAME: &'static str = "resume_reply";
    type Arguments = HostTypeListEnd;
    type Return = ();
    type Captures = Captures;
    type Constructions = HostTypeListEnd;
    type Completion = HostReturns;
}
impl Reply for ResumeReply {
    const MESSAGE: &'static str = "resume";
}

pub(super) fn provider<Profile: OtpProfile>()
-> Result<HostProviderModule<Profile>, HostRegistrationError> {
    HostProviderModule::new("gleam_otp", "gleam/otp/system")
        .and_then(|module| module.with_external_type::<Component<Profile>, DebugStateSchema>())
        .and_then(|module| module.with_external_type::<Component<Profile>, DoNotLeakSchema>())
        .and_then(|module| module.with_callable::<Component<Profile>, StateReply, (Dynamic,), _>(state_reply::<Profile>))
        .and_then(|module| module.with_callable::<Component<Profile>, SuspendReply, (), _>(empty_reply::<Profile>))
        .and_then(|module| module.with_callable::<Component<Profile>, ResumeReply, (), _>(empty_reply::<Profile>))
        .and_then(|module| module.with_scoped_function::<Component<Profile>, (HostListType<DebugOption>,), DebugState, _>("debug_state", debug_state::<Profile>))
        .and_then(|module| module.with_resumable_function::<Component<Profile>, (Pid,), Dynamic, RequestTypes<StateReply>, _>("get_state", get_state::<Profile>))
        .and_then(|module| module.with_resumable_function::<Component<Profile>, (Pid,), DoNotLeak, RequestTypes<SuspendReply>, _>("erl_suspend", acknowledge::<Profile, SuspendReply>))
        .and_then(|module| module.with_resumable_function::<Component<Profile>, (Pid,), DoNotLeak, RequestTypes<ResumeReply>, _>("erl_resume", acknowledge::<Profile, ResumeReply>))
}

pub(super) fn support_provider<Profile: OtpProfile>()
-> Result<HostProviderModule<Profile>, HostRegistrationError> {
    HostProviderModule::new("otp_service_fixture", "otp_service_support")
        .and_then(|module| module.with_scoped_function::<Component<Profile>, (Pid, A), (), _>(
            "send_message",
            send_message::<Profile>,
        ))
        .and_then(|module| module.with_callable::<Component<Profile>, StatusReply, (StatusInfo,), _>(
            status_reply::<Profile>,
        ))
        .and_then(|module| module.with_resumable_native_function::<Component<Profile>, (Pid,), StatusInfo, StatusTypes, _>(
            "get_status",
            geam::gleam_erlang::service::native_rules(NativeRules::default()),
            get_status::<Profile>,
        ))
}

fn send_message<'call, Profile: OtpProfile>(
    mut call: Call<'call, Profile, ()>,
    target: HostExternal<'call, Pid>,
    message: HostValue<'call, A>,
) -> Result<HostCallCompletion<'call, ()>, HostCallError> {
    let message = call.native_value::<A>(message);
    let mut processes = Processes::new(&mut call);
    let target = processes.pid(target);
    processes.send(&target, message);
    Ok(call.return_value(()))
}

fn get_status<'call, Profile: OtpProfile>(
    mut call: NativeCall<'call, Profile, Component<Profile>, StatusInfo, StatusTypes>,
    target: HostExternal<'call, Pid>,
) -> Result<HostCallContinuation<'call, StatusInfo>, HostCallError> {
    let target = Processes::new(call.call()).pid(target);
    Ok(call.resume::<Index3>(move |context| {
        Box::pin(async move {
            let receive = with_current_process(&context, move |mut process, constructions| {
                let tag = begin::<_, _, StatusReply, _>(&mut process, &constructions, target);
                process.receive(tag, None)
            })
            .await?;
            receive.wait_forever(&context).await
        })
    }))
}

fn debug_state<'call, Profile: OtpProfile>(
    mut call: Call<'call, Profile, DebugState>,
    options: HostList<'call, DebugOption>,
) -> Result<HostCallCompletion<'call, DebugState>, HostCallError> {
    // Preserve the supplied NoDebug options as the actor's actual debug state.
    let options = call.native_value::<HostListType<DebugOption>>(options);
    let state = call.create_external(options);
    Ok(call.return_value(state))
}

fn begin<'call, Profile: OtpProfile, Return: HostType, Response: Reply, Targets>(
    process: &mut CurrentProcess<'call, Profile, Component<Profile>, Return>,
    constructions: &HostConstructions<'call, Targets>,
    target: ExecutionUnit,
) -> NativeValue
where
    Targets: HostTypeSequence
        + HostTypeAt<HostTypeIndex0, Type = Pid>
        + HostTypeAt<Index1, Type = Reference>
        + HostTypeAt<Index2, Type = HostCreatedFunction<Response>>,
{
    let requester = process.current().clone();
    let call = process.call();
    let requester = pid_value(call, constructions.at::<HostTypeIndex0>(), requester);
    let reference = new_reference(call, constructions.at::<Index1>());
    let tag = call.native_value::<Reference>(reference);
    let callback = call
        .construct_function::<Response>(constructions.at::<Index2>(), (requester, (reference, ())));
    let callback = call.native_value::<HostFunctionType<Response::Arguments, ()>>(callback);
    let message = NativeValue::tuple([
        NativeValue::symbol("system"),
        NativeValue::tuple([NativeValue::symbol(Response::MESSAGE), callback]),
        NativeValue::symbol("nil"),
    ]);
    let mut processes = Processes::new(call);
    processes.send(&target, message);
    tag
}

fn get_state<'call, Profile: OtpProfile>(
    mut call: Call<'call, Profile, Dynamic>,
    constructions: HostConstructions<'call, RequestTypes<StateReply>>,
    target: HostExternal<'call, Pid>,
) -> Result<HostCallContinuation<'call, Dynamic>, HostCallError> {
    let target = Processes::new(&mut call).pid(target);
    CurrentProcess::with(call, |mut process| {
        let tag = begin::<_, _, StateReply, _>(&mut process, &constructions, target);
        process.resume_receive(constructions, tag, None, move |receive, context| {
            Box::pin(async move {
                let value = receive.wait_forever(&context).await?;
                Ok(HostOwnedCompletion::new(move |mut call, _| {
                    let value = call
                        .create_external_with_binding::<geam::gleam_erlang::Component<Profile>>(
                            geam::gleam_stdlib::Dynamic::from_native(value),
                        );
                    Ok(call.return_value(value))
                }))
            })
        })
    })
}

fn acknowledge<'call, Profile: OtpProfile, Response: Reply>(
    mut call: Call<'call, Profile, DoNotLeak>,
    constructions: HostConstructions<'call, RequestTypes<Response>>,
    target: HostExternal<'call, Pid>,
) -> Result<HostCallContinuation<'call, DoNotLeak>, HostCallError> {
    let target = Processes::new(&mut call).pid(target);
    CurrentProcess::with(call, |mut process| {
        let tag = begin::<_, _, Response, _>(&mut process, &constructions, target);
        process.resume_receive(constructions, tag, None, move |receive, context| {
            Box::pin(async move {
                let reply = receive.wait_forever(&context).await?;
                Ok(HostOwnedCompletion::new(move |mut call, _| {
                    let value = call.create_external(reply);
                    Ok(call.return_value(value))
                }))
            })
        })
    })
}

fn state_reply<'call, Profile: OtpProfile>(
    call: Call<'call, Profile, ()>,
    captures: HostCaptures<'call, Captures>,
    _: HostConstructions<'call, HostTypeListEnd>,
    state: HostExternal<'call, Dynamic>,
) -> Result<HostCallCompletion<'call, ()>, HostCallError> {
    let value = call.native_value::<Dynamic>(state);
    reply(call, captures, value)
}

fn status_reply<'call, Profile: OtpProfile>(
    call: Call<'call, Profile, ()>,
    captures: HostCaptures<'call, Captures>,
    _: HostConstructions<'call, HostTypeListEnd>,
    status: HostCustom<'call, StatusInfo>,
) -> Result<HostCallCompletion<'call, ()>, HostCallError> {
    let value = call.native_value::<StatusInfo>(status);
    reply(call, captures, value)
}

fn empty_reply<'call, Profile: OtpProfile>(
    call: Call<'call, Profile, ()>,
    captures: HostCaptures<'call, Captures>,
    _: HostConstructions<'call, HostTypeListEnd>,
) -> Result<HostCallCompletion<'call, ()>, HostCallError> {
    reply(call, captures, NativeValue::symbol("nil"))
}

fn reply<'call, Profile: OtpProfile>(
    mut call: Call<'call, Profile, ()>,
    captures: HostCaptures<'call, Captures>,
    value: NativeValue,
) -> Result<HostCallCompletion<'call, ()>, HostCallError> {
    let (target, (reference, ())) = call.captures(captures);
    let tag = call.native_value::<Reference>(reference);
    let mut processes = Processes::new(&mut call);
    let target = processes.pid(target);
    processes.send(&target, NativeValue::tuple([tag, value]));
    Ok(call.return_value(()))
}
