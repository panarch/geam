use super::{Receive, pid_value};
use crate::execution::{Destination, Message, Reason, ReferenceId};
use crate::{
    Component, GleamErlangHostProfile, Name as HostName, NameSchema, Pid as HostPid, PidSchema,
};
use ecow::EcoString;
use geam_core::execution::ExecutionUnit;
use geam_core::host::{
    HostCall, HostCallError, HostConstruction, HostCustom, HostExternal, HostProvider, HostType,
    HostTypeList, HostTypeListEnd,
};
use geam_core::provider::advanced::NativeValue;

/// A bounded borrow of the execution domain's shared process service.
///
/// Pids retain only logical identities. They do not keep a process or execution
/// domain alive. All operations use the same routing and names as `gleam_erlang`.
/// Domain operations also work in an owned native Future, without a source
/// invocation. Operations relative to the current process validate that identity.
pub struct Processes<'borrow, 'call, Profile, Provider, Return>
where
    Profile: GleamErlangHostProfile,
    Provider: HostProvider<Profile>,
    Return: HostType,
{
    call: &'borrow mut HostCall<'call, Profile, Provider, Return>,
}

impl<'borrow, 'call, Profile, Provider, Return> Processes<'borrow, 'call, Profile, Provider, Return>
where
    Profile: GleamErlangHostProfile,
    Provider: HostProvider<Profile>,
    Return: HostType,
{
    /// Borrows the service without requiring a current source process.
    pub fn new(call: &'borrow mut HostCall<'call, Profile, Provider, Return>) -> Self {
        Self { call }
    }

    pub fn current(&self) -> Result<ExecutionUnit, HostCallError> {
        self.call.require_execution_unit()
    }

    /// Reads the producer-owned Pid binding without decoding or copying a payload.
    pub fn pid(&self, value: HostExternal<'call, HostPid>) -> ExecutionUnit {
        self.call
            .external_payload_with::<Component<Profile>, PidSchema, HostTypeListEnd>(value)
            .clone()
    }

    pub fn name<A: HostType>(&self, value: HostExternal<'call, HostName<A>>) -> EcoString {
        self.call.external_payload_with::<Component<Profile>, NameSchema, geam_core::HostTypeList<A, HostTypeListEnd>>(value).clone()
    }

    pub fn is_alive(&mut self, pid: &ExecutionUnit) -> bool {
        Profile::erlang_execution(self.call.execution_state()).alive(pid.id())
    }

    pub fn named(&mut self, name: &str) -> Option<ExecutionUnit> {
        Profile::erlang_execution(self.call.execution_state()).named(name)
    }

    pub fn register(&mut self, pid: &ExecutionUnit, name: EcoString) -> bool {
        Profile::erlang_execution(self.call.execution_state()).register(pid.id(), name)
    }

    pub fn unregister(&mut self, name: &str) -> bool {
        Profile::erlang_execution(self.call.execution_state()).unregister(name)
    }

    /// Links this unit to an existing source Pid, preserving normal exit signalling.
    pub fn link(
        &mut self,
        target: HostExternal<'call, HostPid>,
        construction: HostConstruction<'call, HostPid>,
    ) -> Result<bool, HostCallError> {
        let current = self.current()?;
        Ok(link(self.call, &current, target, construction))
    }

    /// Watches the target using the same monitor identity and DOWN queue as Gleam.
    pub fn monitor(
        &mut self,
        target: HostExternal<'call, HostPid>,
        monitor: HostConstruction<'call, super::types::Monitor>,
        reference: HostConstruction<'call, crate::Reference>,
    ) -> Result<HostExternal<'call, super::types::Monitor>, HostCallError> {
        let current = self.current()?;
        let reference = monitor_reference(self.call, current.id(), target, reference);
        Ok(self.call.construct_external_with_binding::<Component<Profile>, super::types::MonitorSchema, HostTypeListEnd>(monitor, reference))
    }

    /// Delivers an exit signal from the current process. `kill` is untrappable;
    /// other reasons retain the original native value in exit and DOWN messages.
    pub fn send_exit(
        &mut self,
        target: &ExecutionUnit,
        reason: NativeValue,
        construction: HostConstruction<'call, HostPid>,
    ) -> Result<(), HostCallError> {
        let current = self.current()?;
        send_exit(self.call, &current, target, reason, construction);
        Ok(())
    }

    pub fn unlink(&mut self, target: &ExecutionUnit) -> Result<(), HostCallError> {
        let current = self.current()?;
        Profile::erlang_execution(self.call.execution_state()).unlink(current.id(), target.id());
        Ok(())
    }

    pub fn trap_exits(&mut self, trapping: bool) -> Result<(), HostCallError> {
        let current = self.current()?;
        Profile::erlang_execution(self.call.execution_state()).trap_exits(current.id(), trapping);
        Ok(())
    }

    pub fn kill(&mut self, target: &ExecutionUnit) {
        Profile::erlang_execution(self.call.execution_state())
            .terminate(target.id(), Reason::Killed);
    }

    /// Sends an existing native view; terminated or foreign-domain targets receive nothing.
    pub fn send(&mut self, pid: &ExecutionUnit, value: NativeValue) {
        Profile::erlang_execution(self.call.execution_state())
            .send(pid.id(), Message::Source(value));
    }

    /// Sends a tagged payload to the subject's current destination.
    /// A missing name or inactive process returns `false` without enqueueing.
    pub fn send_subject<A: HostType>(
        &mut self,
        subject: HostCustom<'call, super::types::Subject<A>>,
        message: NativeValue,
    ) -> bool {
        send_subject(self.call, subject, message)
    }

    /// Schedules a subject message using the host clock. Named destinations are
    /// resolved at delivery, so registering the name later is supported.
    pub fn send_after<A: HostType>(
        &mut self,
        subject: HostCustom<'call, super::types::Subject<A>>,
        message: NativeValue,
        delay: std::time::Duration,
        timer: HostConstruction<'call, super::types::Timer>,
        reference: HostConstruction<'call, crate::Reference>,
    ) -> Result<HostExternal<'call, super::types::Timer>, HostCallError> {
        let (destination, tag) = super::values::subject_parts(self.call, subject);
        let reference = schedule(
            self.call,
            reference,
            delay,
            destination,
            NativeValue::tuple([tag, message]),
        )?;
        Ok(self.call.construct_external_with_binding::<Component<Profile>, super::types::TimerSchema, HostTypeListEnd>(timer, reference))
    }

    /// Receives a subject payload in its owning process. Named subjects use
    /// this process's mailbox, like the source `process.receive` operation.
    pub fn receive_subject<A: HostType>(
        &mut self,
        subject: HostCustom<'call, super::types::Subject<A>>,
        deadline: Option<std::time::Instant>,
    ) -> Result<Receive<Profile>, HostCallError> {
        let current = self.current()?;
        let (destination, tag) = super::values::subject_parts(self.call, subject);
        if matches!(destination, Destination::Pid(pid) if pid != current.id()) {
            return Err(geam_core::HostFailure::new("subject belongs to another process").into());
        }
        Receive::tagged(self.call, current.id(), tag, deadline)
    }

    /// Uses the producer's persistent selector and retained source callbacks.
    pub fn receive_selector<A: HostType>(
        &mut self,
        selector: HostExternal<'call, super::types::Selector<A>>,
        deadline: Option<std::time::Instant>,
    ) -> Result<Receive<Profile>, HostCallError> {
        let selector = self.call.external_payload_with::<Component<Profile>, super::types::SelectorSchema, HostTypeList<A, HostTypeListEnd>>(selector).clone();
        let current = self.current()?;
        Receive::selector(self.call, current.id(), selector, deadline)
    }

    /// Receives the payload from a two-element message tagged with `tag`.
    /// Unmatched messages retain their original order in this unit's mailbox.
    pub fn receive(
        &mut self,
        tag: NativeValue,
        deadline: Option<std::time::Instant>,
    ) -> Result<Receive<Profile>, HostCallError> {
        let current = self.current()?;
        Receive::tagged(self.call, current.id(), tag, deadline)
    }

    /// Receives the oldest queued message, including link and monitor signals.
    pub fn receive_any(
        &mut self,
        deadline: Option<std::time::Instant>,
    ) -> Result<Receive<Profile>, HostCallError> {
        let current = self.current()?;
        Receive::any(self.call, current.id(), deadline)
    }

    /// Receives a complete tuple with a matching tag and number of fields.
    pub fn receive_record(
        &mut self,
        tag: NativeValue,
        arity: usize,
        deadline: Option<std::time::Instant>,
    ) -> Result<Receive<Profile>, HostCallError> {
        let current = self.current()?;
        Receive::record(self.call, current.id(), tag, arity, deadline)
    }
}

pub(super) fn link<'call, Profile, Provider, Return>(
    call: &mut HostCall<'call, Profile, Provider, Return>,
    current: &ExecutionUnit,
    target: HostExternal<'call, HostPid>,
    construction: HostConstruction<'call, HostPid>,
) -> bool
where
    Profile: GleamErlangHostProfile,
    Provider: HostProvider<Profile>,
    Return: HostType,
{
    let target_id = Processes::new(call).pid(target).id();
    let target = call.native_value::<HostPid>(target);
    let source = pid_value(call, construction, current.clone());
    let source = call.native_value::<HostPid>(source);
    Profile::erlang_execution(call.execution_state()).link(current.id(), target_id, source, target)
}

pub(super) fn send_exit<'call, Profile, Provider, Return>(
    call: &mut HostCall<'call, Profile, Provider, Return>,
    current: &ExecutionUnit,
    target: &ExecutionUnit,
    reason: NativeValue,
    construction: HostConstruction<'call, HostPid>,
) where
    Profile: GleamErlangHostProfile,
    Provider: HostProvider<Profile>,
    Return: HostType,
{
    let source = pid_value(call, construction, current.clone());
    let source = call.native_value::<HostPid>(source);
    let runtime = Profile::erlang_execution(call.execution_state());
    match reason.as_symbol().as_deref() {
        Some("kill") => runtime.terminate(target.id(), Reason::Killed),
        Some("normal") => runtime.signal(
            target.id(),
            source,
            Reason::Normal,
            current.id() == target.id(),
        ),
        _ => runtime.signal(
            target.id(),
            source,
            Reason::Native(reason),
            current.id() == target.id(),
        ),
    }
}

pub(super) fn send_subject<'call, Profile, Provider, Return, A>(
    call: &mut HostCall<'call, Profile, Provider, Return>,
    subject: HostCustom<'call, super::types::Subject<A>>,
    message: NativeValue,
) -> bool
where
    Profile: GleamErlangHostProfile,
    Provider: HostProvider<Profile>,
    Return: HostType,
    A: HostType,
{
    let (destination, tag) = super::values::subject_parts(call, subject);
    let runtime = Profile::erlang_execution(call.execution_state());
    let target = match destination {
        Destination::Pid(pid) => pid,
        Destination::Name(name) => match runtime.named(&name) {
            Some(unit) => unit.id(),
            None => return false,
        },
    };
    if !runtime.alive(target) {
        return false;
    }
    runtime.send(target, Message::Source(NativeValue::tuple([tag, message])));
    true
}

pub(crate) fn monitor_reference<'call, Profile, Provider, Return>(
    call: &mut HostCall<'call, Profile, Provider, Return>,
    owner: geam_core::execution::ExecutionUnitId,
    target: HostExternal<'call, HostPid>,
    construction: HostConstruction<'call, crate::Reference>,
) -> NativeValue
where
    Profile: GleamErlangHostProfile,
    Provider: HostProvider<Profile>,
    Return: HostType,
{
    let target_id = call
        .external_payload_with::<Component<Profile>, PidSchema, HostTypeListEnd>(target)
        .id();
    let target = call.native_value::<HostPid>(target);
    let id = ReferenceId::new();
    let reference = call.construct_external_with_binding::<Component<Profile>, crate::ReferenceSchema, HostTypeListEnd>(construction, crate::reference::Payload::Identity(id));
    let reference = call.native_value::<crate::Reference>(reference);
    Profile::erlang_execution(call.execution_state()).monitor(
        owner,
        target_id,
        id,
        reference.clone(),
        target,
    );
    reference
}

pub(crate) fn schedule<'call, Profile, Provider, Return>(
    call: &mut HostCall<'call, Profile, Provider, Return>,
    construction: HostConstruction<'call, crate::Reference>,
    delay: std::time::Duration,
    destination: Destination,
    message: NativeValue,
) -> Result<NativeValue, HostCallError>
where
    Profile: GleamErlangHostProfile,
    Provider: HostProvider<Profile>,
    Return: HostType,
{
    let deadline = call
        .clock()
        .now()
        .checked_add(delay)
        .ok_or_else(|| geam_core::HostFailure::new("timeout exceeds the host clock range"))?;
    let id = ReferenceId::new();
    let reference = call.construct_external_with_binding::<Component<Profile>, crate::ReferenceSchema, HostTypeListEnd>(construction, crate::reference::Payload::Identity(id));
    let reference = call.native_value::<crate::Reference>(reference);
    Profile::erlang_execution(call.execution_state()).schedule(id, deadline, destination, message);
    Ok(reference)
}

#[cfg(test)]
mod tests {
    use super::Processes;
    use crate::service::types::{
        Monitor, MonitorSchema, NamedSubject, OrdinarySubject, Subject, SubjectSchema, Timer,
        TimerSchema,
    };
    use crate::{
        Component, GleamErlangProfile, Name, NameSchema, Pid, PidSchema, Reference, ReferenceSchema,
    };
    use geam_core::host::{
        HostCall, HostCallContinuation, HostCallError, HostCallable, HostConstructions,
        HostFunctionType, HostOwnedCompletion, HostProviderModule, HostTypeIndex0,
        HostTypeIndexNext, HostTypeList, HostTypeListEnd,
    };
    use geam_core::provider::advanced::NativeValue;
    use geam_core::{ModuleSource, PackageSource};
    use geam_stdlib::provider_support::{Dynamic, DynamicSchema};
    use num_bigint::BigInt;
    use std::time::Duration;

    type One<T> = HostTypeList<T, HostTypeListEnd>;
    type Two = HostTypeIndexNext<HostTypeIndex0>;
    type Three = HostTypeIndexNext<Two>;
    type Four = HostTypeIndexNext<Three>;
    type Five = HostTypeIndexNext<Four>;
    type Six = HostTypeIndexNext<Five>;
    type Seven = HostTypeIndexNext<Six>;
    type Constructions = HostTypeList<
        Pid,
        HostTypeList<
            Name<BigInt>,
            HostTypeList<
                Subject<BigInt>,
                HostTypeList<Dynamic, HostTypeList<Monitor, HostTypeList<Reference, One<Timer>>>>,
            >,
        >,
    >;

    #[test]
    fn subjects_monitors_and_timers_use_the_current_domain_and_original_mailboxes() {
        let process = HostProviderModule::new("gleam_erlang", "gleam/erlang/process")
            .unwrap()
            .with_shared_custom_type::<SubjectSchema>()
            .unwrap()
            .with_external_type::<Component<GleamErlangProfile>, PidSchema>()
            .unwrap()
            .with_external_type::<Component<GleamErlangProfile>, NameSchema>()
            .unwrap()
            .with_external_type::<Component<GleamErlangProfile>, MonitorSchema>()
            .unwrap()
            .with_external_type::<Component<GleamErlangProfile>, TimerSchema>()
            .unwrap();
        let reference = HostProviderModule::new("gleam_erlang", "gleam/erlang/reference")
            .unwrap()
            .with_external_type::<Component<GleamErlangProfile>, ReferenceSchema>()
            .unwrap();
        let dynamic = HostProviderModule::new("gleam_stdlib", "gleam/dynamic")
            .unwrap()
            .with_external_type::<Component<GleamErlangProfile>, DynamicSchema>()
            .unwrap();
        let consumer = HostProviderModule::new("application", "main")
            .unwrap()
            .with_resumable_function::<Component<GleamErlangProfile>, (HostFunctionType<HostTypeListEnd, ()>,), BigInt, Constructions, _>("exercise", exercise)
            .unwrap();
        let result = crate::test_support::run_main(
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
                        ModuleSource::new(
                            "gleam/erlang/reference",
                            "reference.gleam",
                            "pub type Reference",
                        ),
                        ModuleSource::new(
                            "gleam/erlang/process",
                            "process.gleam",
                            r#"
import gleam/dynamic.{type Dynamic}
pub type Pid
pub type Name(message)
pub type Monitor
pub type Timer
pub opaque type Subject(message) { Subject(owner: Pid, tag: Dynamic) NamedSubject(name: Name(message)) }
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
@external(erlang, "host", "exercise") fn exercise(child: fn() -> Nil) -> Int
pub fn main() { exercise(fn() { panic as "cancelled child must never run" }) }
"#,
                    )],
                ),
            ],
            [process, reference, dynamic, consumer],
        );
        assert_eq!(result, geam_core::Value::Int(42.into()));
    }

    fn exercise<'call>(
        mut call: HostCall<'call, GleamErlangProfile, Component<GleamErlangProfile>, BigInt>,
        constructions: HostConstructions<'call, Constructions>,
        callback: HostCallable<'call, HostTypeListEnd, ()>,
    ) -> Result<HostCallContinuation<'call, BigInt>, HostCallError> {
        let current = call.execution_unit().unwrap();
        let child = call.spawn(callback);
        let current_pid = crate::service::pid_value(
            &mut call,
            constructions.at::<HostTypeIndex0>(),
            current.clone(),
        );
        let child_pid = crate::service::pid_value(
            &mut call,
            constructions.at::<HostTypeIndex0>(),
            child.clone(),
        );
        let name = call.construct_external_with_binding::<Component<GleamErlangProfile>, NameSchema, One<BigInt>>(constructions.at::<Two>(), "worker".into());
        let message = call.native_value::<BigInt>(42.into());
        let tag = call.construct_external_with_binding::<Component<GleamErlangProfile>, DynamicSchema, HostTypeListEnd>(constructions.at::<Four>(), geam_stdlib::Dynamic::from_native(NativeValue::symbol("reply")));
        let ordinary = call.construct_custom::<OrdinarySubject<BigInt>>(
            constructions.at::<Three>(),
            (current_pid, (tag, ())),
        );
        let foreign_subject = call.construct_custom::<OrdinarySubject<BigInt>>(
            constructions.at::<Three>(),
            (child_pid, (tag, ())),
        );
        let named =
            call.construct_custom::<NamedSubject<BigInt>>(constructions.at::<Three>(), (name, ()));
        let subject =
            <crate::service::Subject<BigInt> as geam_core::provider::ProviderInputValue<
                GleamErlangProfile,
                Component<GleamErlangProfile>,
                BigInt,
            >>::from_host(&mut call, ordinary);
        let mut authoring = geam_core::provider::Call::from_host_call(call);
        assert_eq!(
            crate::service::ProcessCall::receive_subject(
                &mut authoring,
                subject,
                Some(Duration::MAX)
            )
            .err()
            .unwrap()
            .to_string(),
            "timeout exceeds the host clock range",
        );
        let subject =
            <crate::service::Subject<BigInt> as geam_core::provider::ProviderInputValue<
                GleamErlangProfile,
                Component<GleamErlangProfile>,
                BigInt,
            >>::from_host(authoring.host_call(), ordinary);
        let ordinary_receive =
            crate::service::ProcessCall::receive_subject(&mut authoring, subject, None).unwrap();
        let mut call = authoring.into_host_call();
        let mut processes = Processes::new(&mut call);
        assert!(processes.is_alive(&child));
        assert_eq!(processes.name(name), "worker");
        assert_eq!(processes.pid(current_pid).id(), current.id());
        assert_eq!(
            processes
                .receive_subject(foreign_subject, None)
                .err()
                .unwrap()
                .to_string(),
            "subject belongs to another process"
        );
        assert!(!processes.send_subject(named, message.clone()));
        assert!(processes.register(&child, "worker".into()));
        assert!(!processes.register(&current, "worker".into()));
        assert_eq!(processes.named("worker").unwrap().id(), child.id());
        assert!(processes.send_subject(named, message.clone()));
        assert!(processes.send_subject(ordinary, message.clone()));
        assert!(processes.unregister("worker"));
        assert!(!processes.unregister("worker"));
        assert!(processes.named("worker").is_none());
        // Names are resolved at delivery, after this timer has been created.
        let _timer = processes
            .send_after(
                named,
                message.clone(),
                Duration::ZERO,
                constructions.at::<Seven>(),
                constructions.at::<Six>(),
            )
            .unwrap();
        assert!(processes.register(&current, "worker".into()));
        let named_receive = processes.receive_subject(named, None).unwrap();
        assert_eq!(
            processes
                .send_after(
                    ordinary,
                    message.clone(),
                    Duration::MAX,
                    constructions.at::<Seven>(),
                    constructions.at::<Six>()
                )
                .err()
                .unwrap()
                .to_string(),
            "timeout exceeds the host clock range"
        );
        let _monitor = processes
            .monitor(
                child_pid,
                constructions.at::<Five>(),
                constructions.at::<Six>(),
            )
            .unwrap();
        assert!(
            processes
                .link(child_pid, constructions.at::<HostTypeIndex0>())
                .unwrap()
        );
        processes.trap_exits(true).unwrap();
        processes.unlink(&child).unwrap();
        processes.kill(&child);
        assert!(!processes.is_alive(&child));
        assert!(!processes.send_subject(foreign_subject, message));
        assert!(!processes.register(&child, "dead".into()));
        let down = processes
            .receive_record(NativeValue::symbol("DOWN"), 4, None)
            .unwrap();
        Ok(call.resume(constructions, move |context| {
            Box::pin(async move {
                let first = ordinary_receive.wait_forever(&context).await.unwrap();
                let second = named_receive.wait(&context).await.unwrap().unwrap();
                let down = down.wait(&context).await.unwrap().unwrap();
                assert_eq!(first.as_int(), Some(42.into()));
                assert_eq!(second.as_int(), Some(42.into()));
                assert_eq!(down.len(), Some(5));
                assert_eq!(down.index(0).unwrap().as_symbol().as_deref(), Some("DOWN"));
                assert_eq!(
                    down.index(2).unwrap().as_symbol().as_deref(),
                    Some("process")
                );
                assert_eq!(
                    down.index(4).unwrap().as_symbol().as_deref(),
                    Some("killed")
                );
                Ok(HostOwnedCompletion::new(|call, _| {
                    Ok(call.return_value(42.into()))
                }))
            })
        }))
    }
}
