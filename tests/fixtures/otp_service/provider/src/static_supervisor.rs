use crate::child::{Restart, RestartBudget, Shutdown, property};
use crate::schema::{
    StartError, StartResult, StaticChildSpec, StaticChildSpecSchema, StaticFlag, StaticFlags,
    StaticFlagsSchema, StaticProperty, StaticStart, StaticTimeout, StaticTimeoutSchema,
};
use crate::{A, Call, Component, Four, One, OtpProfile, Two};
use geam::execution::ExecutionUnit;
use geam::gleam_erlang::service::{
    CurrentProcess, Pid as ProcessId, Processes, new_reference, pid_value, with_current_process,
};
use geam::gleam_erlang::{Atom, Pid, Reference};
use geam::gleam_stdlib::provider_support::{
    Dynamic, DynamicSchema, GleamError, GleamOk, GleamResult,
};
use geam::host::native::{NativeCall, NativeRules};
use geam::host::{
    HostCallCompletion, HostCallContinuation, HostCallError, HostCallableSchema, HostCaptures,
    HostComponentProfile, HostConstructions, HostCreatedFunction, HostExecutionContext,
    HostExecutionError, HostExternal, HostExternalBinding, HostExternalEquality,
    HostExternalHashing, HostExternalInspection, HostExternalStorage, HostExternalStore,
    HostFunctionType, HostList, HostListType, HostOwnedCallable, HostOwnedCompletion, HostProfile,
    HostProviderModule, HostRegistrationError, HostReturns, HostTuple, HostTupleType,
    HostTypeIndex0, HostTypeIndexNext, HostTypeListEnd,
};
use geam::provider::BigInt;
use geam::provider::advanced::NativeValue;
use std::sync::Arc;

pub struct Child<Profile: HostProfile> {
    properties: NativeValue,
    restart: Restart,
    shutdown: Shutdown,
    start: HostOwnedCallable<
        Profile,
        Component<Profile>,
        HostTypeListEnd,
        StartResult<A>,
        HostTypeListEnd,
    >,
}

pub struct ChildStorage;
impl<Profile: OtpProfile> HostExternalBinding<Profile, StaticChildSpecSchema>
    for Component<Profile>
{
    type Storage = ChildStorage;
}
impl<Profile: OtpProfile> HostExternalStorage<Profile, StaticChildSpecSchema> for ChildStorage {
    type Payload = Arc<Child<Profile>>;
    fn store(stores: &Profile::ExternalStores) -> &HostExternalStore<Self::Payload> {
        &<Profile as HostComponentProfile<Component<Profile>>>::component_stores(stores)
            .static_children
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
    fn inspect(
        context: &HostExternalInspection<'_>,
        value: &Self::Payload,
    ) -> geam::provider::EcoString {
        value.properties.inspect(context)
    }
    fn native_view(value: &Self::Payload) -> Option<NativeValue> {
        Some(value.properties.clone())
    }
}

type Arguments = HostTupleType<Two<StaticFlags, HostListType<StaticChildSpec>>>;
type Captures = Four<StaticFlags, HostListType<StaticChildSpec>, Pid, Reference>;
type StartTypes = Four<Pid, Reference, Dynamic, HostCreatedFunction<SupervisorBody>>;
type Index1 = HostTypeIndexNext<HostTypeIndex0>;
type Index2 = HostTypeIndexNext<Index1>;
type Index3 = HostTypeIndexNext<Index2>;

struct SupervisorBody;
impl HostCallableSchema for SupervisorBody {
    const PACKAGE: &'static str = "gleam_otp";
    const MODULE: &'static str = "gleam/otp/static_supervisor";
    const NAME: &'static str = "supervisor_loop";
    type Arguments = HostTypeListEnd;
    type Return = ();
    type Captures = Captures;
    type Constructions = One<Pid>;
    type Completion = HostReturns;
}

pub(super) fn provider<Profile: OtpProfile>()
-> Result<HostProviderModule<Profile>, HostRegistrationError> {
    HostProviderModule::new("gleam_otp", "gleam/otp/static_supervisor")
        .and_then(|module| module.with_external_type::<Component<Profile>, StaticChildSpecSchema>())
        .and_then(|module| module.with_external_type::<Component<Profile>, StaticFlagsSchema>())
        .and_then(|module| module.with_external_type::<Component<Profile>, StaticTimeoutSchema>())
        .and_then(|module| module.with_resumable_callable::<Component<Profile>, SupervisorBody, (), _>(supervisor_body::<Profile>))
        .and_then(|module| module.with_scoped_function_and_constructions::<Component<Profile>, (HostListType<StaticProperty<A>>,), StaticChildSpec, HostTypeListEnd, _>("make_erlang_child_spec", make_child::<Profile>))
        .and_then(|module| module.with_scoped_function::<Component<Profile>, (HostListType<StaticFlag<A>>,), StaticFlags, _>("make_erlang_start_flags", make_flags::<Profile>))
        .and_then(|module| module.with_scoped_function::<Component<Profile>, (BigInt,), StaticTimeout, _>("make_timeout", make_timeout::<Profile>))
        .and_then(|module| module.with_resumable_function::<Component<Profile>, (Atom, Arguments), GleamResult<Pid, Dynamic>, StartTypes, _>("erlang_start_link", start::<Profile>))
        .and_then(|module| module.with_native_function::<Component<Profile>, (Dynamic,), StartError, One<StartError>, _>("convert_erlang_start_error", geam::gleam_erlang::service::native_rules(NativeRules::default()), start_error::<Profile>))
}

fn make_child<'call, Profile: OtpProfile>(
    mut call: Call<'call, Profile, StaticChildSpec>,
    constructions: HostConstructions<'call, HostTypeListEnd>,
    properties: HostList<'call, StaticProperty<A>>,
) -> Result<HostCallCompletion<'call, StaticChildSpec>, HostCallError> {
    let mut callback = None;
    let mut index = 0;
    while let Some(property) = call.list_item::<StaticProperty<A>>(properties, index) {
        if let Some((mfa, ())) = call.custom_fields::<StaticStart<A>>(property) {
            let (_, (_, (callbacks, ()))) = call.tuple_values(mfa);
            callback =
                call.list_item::<HostFunctionType<HostTypeListEnd, StartResult<A>>>(callbacks, 0);
        }
        index += 1;
    }
    let callback = callback
        .ok_or_else(|| geam::HostFailure::new("child specification has no start callback"))?;
    let callback = call.owned_callable(callback, &constructions);
    let properties = call.native_value::<HostListType<StaticProperty<A>>>(properties);
    let child = Child {
        restart: Restart::from_properties(&properties)?,
        shutdown: Shutdown::from_properties(&properties)?,
        properties,
        start: callback,
    };
    let value = call.create_external(Arc::new(child));
    Ok(call.return_value(value))
}

fn make_flags<'call, Profile: OtpProfile>(
    mut call: Call<'call, Profile, StaticFlags>,
    flags: HostList<'call, StaticFlag<A>>,
) -> Result<HostCallCompletion<'call, StaticFlags>, HostCallError> {
    let value = call.native_value::<HostListType<StaticFlag<A>>>(flags);
    let value = call.create_external(value);
    Ok(call.return_value(value))
}
fn make_timeout<'call, Profile: OtpProfile>(
    mut call: Call<'call, Profile, StaticTimeout>,
    amount: BigInt,
) -> Result<HostCallCompletion<'call, StaticTimeout>, HostCallError> {
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

fn start<'call, Profile: OtpProfile>(
    call: Call<'call, Profile, GleamResult<Pid, Dynamic>>,
    constructions: HostConstructions<'call, StartTypes>,
    _: HostExternal<'call, Atom>,
    arguments: HostTuple<'call, Two<StaticFlags, HostListType<StaticChildSpec>>>,
) -> Result<HostCallContinuation<'call, GleamResult<Pid, Dynamic>>, HostCallError> {
    CurrentProcess::with(call, |mut process| {
        let current = process.current().clone();
        let call = process.call();
        let (flags, (children, ())) = call.tuple_values(arguments);
        let current = pid_value(call, constructions.at::<HostTypeIndex0>(), current);
        let reference = new_reference(call, constructions.at::<Index1>());
        let tag = call.native_value::<Reference>(reference);
        let callback = call.construct_function::<SupervisorBody>(
            constructions.at::<Index3>(),
            (flags, (children, (current, (reference, ())))),
        );
        let child = call.spawn(callback);
        let target = pid_value(call, constructions.at::<HostTypeIndex0>(), child.clone());
        process.link(target, constructions.at::<HostTypeIndex0>());
        process.resume_receive(constructions, tag, None, move |receive, context| Box::pin(async move {
        let result = receive.wait_forever(&context).await?;
        Ok(HostOwnedCompletion::new(move |mut call, constructions| {
            match start_response(result) {
                Ok(()) => {
                    let pid = pid_value(&mut call, constructions.at::<HostTypeIndex0>(), child);
                    Ok(call.return_custom::<GleamOk<Pid, Dynamic>>((pid, ())))
                }
                Err(error) => {
                    let error = call.construct_external_with_binding::<geam::gleam_erlang::Component<Profile>, DynamicSchema, HostTypeListEnd>(constructions.at::<Index2>(), geam::gleam_stdlib::Dynamic::from_native(error));
                    Ok(call.return_custom::<GleamError<Pid, Dynamic>>((error, ())))
                }
            }
        }))
    }))
    })
}

// Only this supervisor owns the fresh reply reference. Its startup payload is
// Nil for success or the child's exact StartError, whose constructors cannot be
// Nil. Keep the failure value intact instead of adding and parsing an envelope.
fn start_response(response: NativeValue) -> Result<(), NativeValue> {
    if response.as_symbol().as_deref() == Some("nil") {
        Ok(())
    } else {
        Err(response)
    }
}

struct Running<Profile: HostProfile> {
    specification: Arc<Child<Profile>>,
    unit: ExecutionUnit,
}

fn supervisor_body<'call, Profile: OtpProfile>(
    call: Call<'call, Profile, ()>,
    captures: HostCaptures<'call, Captures>,
    constructions: HostConstructions<'call, One<Pid>>,
) -> Result<HostCallContinuation<'call, ()>, HostCallError> {
    CurrentProcess::with(call, |mut process| {
        let call = process.call();
        let (flags, (children, (parent, (reference, ())))) = call.captures(captures);
        let flags = call.external_payload(flags).clone();
        validate_flags(&flags)?;
        let mut budget = RestartBudget::from_flags(&flags)?;
        let mut specifications = Vec::new();
        let mut index = 0;
        while let Some(child) = call.list_item::<StaticChildSpec>(children, index) {
            specifications.push(Arc::clone(&call.external_payload(child)));
            index += 1;
        }
        let tag = call.native_value::<Reference>(reference);
        let parent = Processes::new(call).pid(parent);
        process.trap_exits(true);
        Ok(process.into_call().resume(constructions, move |context| {
            Box::pin(async move {
                let mut children = Vec::new();
                for specification in specifications {
                    match run_child(&context, specification).await? {
                        Ok(child) => children.push(child),
                        Err(error) => {
                            stop_children(&context, &children).await?;
                            respond(&context, parent, tag, error).await?;
                            return Ok(HostOwnedCompletion::new(|call, _| {
                                Ok(call.return_value(()))
                            }));
                        }
                    }
                }
                respond(&context, parent.clone(), tag, NativeValue::symbol("nil")).await?;
                let mut reason = NativeValue::symbol("shutdown");
                loop {
                    let receive = with_current_process(&context, |mut process, _| {
                        process.receive_record_with(NativeValue::symbol("EXIT"), exit_record, None)
                    })
                    .await?;
                    let (pid, child_reason) = receive.wait_forever(&context).await?;
                    let exited = restore_pid(&context, pid).await?;
                    if exited.id() == parent.id() {
                        reason = child_reason;
                        break;
                    }
                    if let Some(index) = children
                        .iter()
                        .position(|child| child.unit.id() == exited.id())
                    {
                        let child = children.remove(index);
                        if child.specification.restart.after_exit(&child_reason) {
                            let now = context.with_call(|call| call.clock().now()).await?;
                            if !budget.allow(now) {
                                break;
                            }
                            match run_child(&context, child.specification).await? {
                                Ok(child) => children.insert(index, child),
                                Err(_) => break,
                            }
                        }
                    }
                }
                stop_children(&context, &children).await?;
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

fn exit_record(message: &NativeValue) -> Option<(NativeValue, NativeValue)> {
    match (message.index(1), message.index(2), message.len()) {
        (Some(pid), Some(reason), Some(3)) => Some((pid, reason)),
        _ => None,
    }
}

fn validate_flags(flags: &NativeValue) -> Result<(), HostCallError> {
    if property(flags, "strategy")
        .and_then(|value| value.as_symbol())
        .as_deref()
        != Some("one_for_one")
        || property(flags, "auto_shutdown")
            .and_then(|value| value.as_symbol())
            .as_deref()
            != Some("never")
    {
        return Err(
            geam::HostFailure::new("fixture supports OneForOne with Never auto shutdown").into(),
        );
    }
    Ok(())
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
                .ok_or_else(|| geam::HostFailure::new("exit message has no source Pid"))
        })
        .await?
        .map_err(Into::into)
}
async fn run_child<Profile: OtpProfile>(
    context: &HostExecutionContext<'_, Profile, Component<Profile>, One<Pid>>,
    specification: Arc<Child<Profile>>,
) -> Result<Result<Running<Profile>, NativeValue>, HostExecutionError> {
    let result = specification
        .start
        .invoke(
            context,
            |_, _| (),
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
    Ok(Ok(Running {
        specification,
        unit,
    }))
}
async fn respond<Profile: OtpProfile>(
    context: &HostExecutionContext<'_, Profile, Component<Profile>, One<Pid>>,
    parent: ExecutionUnit,
    tag: NativeValue,
    result: NativeValue,
) -> Result<(), HostExecutionError> {
    context
        .with_call(move |mut call| {
            Processes::new(&mut call).send(&parent, NativeValue::tuple([tag, result]));
        })
        .await
}
async fn stop_children<Profile: OtpProfile>(
    context: &HostExecutionContext<'_, Profile, Component<Profile>, One<Pid>>,
    children: &[Running<Profile>],
) -> Result<(), HostExecutionError> {
    for child in children.iter().rev() {
        crate::child::stop(context, &child.unit, child.specification.shutdown).await?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{Child, StaticChildSpecSchema};
    use crate::child::{Restart, Shutdown};
    use crate::schema::StartResult;
    use crate::test_support::storage::{
        Probe, ProbeValue, callback_packages, callback_providers, check,
    };
    use crate::test_support::{Profile, run_source};
    use crate::{A, Call, Component};
    use geam::host::{
        HostCallCompletion, HostCallError, HostCallable, HostConstructions, HostFunctionType,
        HostProviderModule, HostTypeListEnd,
    };
    use geam::provider::{BigInt, StringValue};
    use std::sync::Arc;

    #[test]
    fn native_start_error_roundtrips_declared_variants_and_rejects_invalid_input() {
        use geam::gleam_stdlib::provider_support::Dynamic;
        let provider = HostProviderModule::new("application", "main").unwrap()
            .with_scoped_function::<geam::gleam_erlang::Component<Profile>, (A,), Dynamic, _>("erase", crate::test_support::erase_native).unwrap()
            .with_native_function::<Component<Profile>, (Dynamic,), super::StartError, crate::One<super::StartError>, _>("convert", geam::gleam_erlang::service::native_rules(geam::host::native::NativeRules::default()), super::start_error::<Profile>).unwrap();
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
    fn child_spec_native_boundary_requires_a_callback_and_complete_policies() {
        use geam::{ModuleSource, PackageSource};
        for (properties, expected) in [
            ("[]", Some("child specification has no start callback")),
            (
                "[Start(#(atom.create(\"m\"), atom.create(\"f\"), []))]",
                Some("child specification has no start callback"),
            ),
            (
                "[Start(mfa)]",
                Some("child specification requires a restart policy"),
            ),
            (
                "[Start(mfa), Restart(supervision.Permanent)]",
                Some("child specification requires a shutdown policy"),
            ),
            (
                "[Start(mfa), Restart(supervision.Permanent), Shutdown(make_timeout(5))]",
                None,
            ),
        ] {
            let source = format!(
                r#"
import gleam/erlang/atom.{{type Atom}}
import gleam/otp/actor
import gleam/otp/supervision
pub type ErlangChildSpec
pub type Timeout
pub type ErlangChildSpecProperty(data) {{
  Id(Int)
  Start(#(Atom, Atom, List(fn() -> actor.StartResult(data))))
  Restart(supervision.Restart)
  Significant(Bool)
  Type(Atom)
  Shutdown(Timeout)
}}
@external(erlang, "maps", "from_list")
fn make_erlang_child_spec(properties: List(ErlangChildSpecProperty(data))) -> ErlangChildSpec
@external(erlang, "fixture", "make_timeout") fn make_timeout(amount: Int) -> Timeout
pub fn probe() {{
  let mfa = #(atom.create("m"), atom.create("f"), [fn() -> actor.StartResult(Nil) {{ Error(actor.InitTimeout) }}])
  let properties: List(ErlangChildSpecProperty(Nil)) = {properties}
  let _ = make_erlang_child_spec(properties)
  Nil
}}
"#
            );
            let packages = callback_packages(
                "import gleam/otp/static_supervisor\npub fn main() { static_supervisor.probe() }",
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
                            "gleam/otp/static_supervisor",
                            "static_supervisor.gleam",
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
            let native = HostProviderModule::new("gleam_otp", "gleam/otp/static_supervisor").unwrap()
                .with_external_type::<Component<Profile>, StaticChildSpecSchema>().unwrap()
                .with_external_type::<Component<Profile>, crate::schema::StaticTimeoutSchema>().unwrap()
                .with_scoped_function_and_constructions::<Component<Profile>, (geam::host::HostListType<crate::schema::StaticProperty<A>>,), super::StaticChildSpec, HostTypeListEnd, _>("make_erlang_child_spec", super::make_child::<Profile>).unwrap()
                .with_scoped_function::<Component<Profile>, (BigInt,), super::StaticTimeout, _>("make_timeout", super::make_timeout::<Profile>).unwrap();
            let mut providers = callback_providers(native);
            providers.extend(
                geam::gleam_erlang::host_providers::<Profile>()
                    .unwrap()
                    .into_iter()
                    .filter(|module| module.module() == "gleam/erlang/atom"),
            );
            let result = run_source(packages, providers);
            if let Some(expected) = expected {
                assert_eq!(
                    crate::test_support::host_failure(result).as_deref(),
                    Some(expected)
                );
            } else {
                assert_eq!(result.unwrap(), geam::Value::Nil);
            }
        }
    }

    #[test]
    fn static_native_flags_and_start_responses_reject_unsupported_inputs() {
        use geam::provider::advanced::NativeValue;
        let flags = |strategy, shutdown| {
            NativeValue::tuple([
                NativeValue::tuple([
                    NativeValue::symbol("strategy"),
                    NativeValue::symbol(strategy),
                ]),
                NativeValue::tuple([
                    NativeValue::symbol("auto_shutdown"),
                    NativeValue::symbol(shutdown),
                ]),
            ])
        };
        assert!(super::validate_flags(&flags("one_for_one", "never")).is_ok());
        for rejected in [
            flags("one_for_all", "never"),
            flags("one_for_one", "any_significant"),
            NativeValue::tuple([]),
        ] {
            assert!(super::validate_flags(&rejected).is_err());
        }
        assert!(super::start_response(NativeValue::symbol("nil")).is_ok());
        let error = super::start_response(NativeValue::symbol("init_timeout"))
            .err()
            .unwrap();
        assert_eq!(error.as_symbol().as_deref(), Some("init_timeout"));
        let error = super::start_response(NativeValue::tuple([
            NativeValue::symbol("init_exited"),
            NativeValue::symbol("killed"),
        ]))
        .err()
        .unwrap();
        assert_eq!(
            error.index(0).unwrap().as_symbol().as_deref(),
            Some("init_exited")
        );
        assert_eq!(
            error.index(1).unwrap().as_symbol().as_deref(),
            Some("killed")
        );
    }

    #[test]
    fn child_storage_uses_properties_for_equality_hash_inspection_and_native_view() {
        let provider = HostProviderModule::new("application", "main").unwrap()
            .with_external_type::<Component<Profile>, Probe<StaticChildSpecSchema>>().unwrap()
            .with_scoped_function_and_constructions::<Component<Profile>, (HostFunctionType<HostTypeListEnd, StartResult<A>>, BigInt), ProbeValue<StaticChildSpecSchema>, HostTypeListEnd, _>("make", make).unwrap()
            .with_scoped_function::<Component<Profile>, (ProbeValue<StaticChildSpecSchema>, ProbeValue<StaticChildSpecSchema>, ProbeValue<StaticChildSpecSchema>, StringValue), (), _>("check", check::<StaticChildSpecSchema>).unwrap();
        assert_eq!(run_source(callback_packages(r#"
import gleam/otp/actor
pub type Probe
@external(erlang, "fixture", "make") fn make(start: fn() -> actor.StartResult(a), property: Int) -> Probe
@external(erlang, "fixture", "check") fn check(a: Probe, b: Probe, c: Probe, expected: String) -> Nil
fn start() -> actor.StartResult(Nil) { Error(actor.InitTimeout) }
pub fn main() { check(make(start, 1), make(start, 1), make(start, 2), "1") }
"#), callback_providers(provider)).unwrap(), geam::Value::Nil);
    }

    fn make<'call>(
        mut call: Call<'call, Profile, ProbeValue<StaticChildSpecSchema>>,
        constructions: HostConstructions<'call, HostTypeListEnd>,
        start: HostCallable<'call, HostTypeListEnd, StartResult<A>>,
        property: BigInt,
    ) -> Result<HostCallCompletion<'call, ProbeValue<StaticChildSpecSchema>>, HostCallError> {
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
}
