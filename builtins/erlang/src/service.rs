//! Call-scoped access to the process service and its original source identities.

mod current;
mod name;
mod native;
mod pid;
mod processes;
mod reference;
pub mod selectors;
mod subject;
pub mod types;
mod values;
pub use crate::process::mailbox::{Receive, RecordReceive};
pub use current::{CurrentProcess, with_current_process};
pub use name::Name;
pub use native::{cancel_timer, demonitor};
pub use pid::Pid;
pub use processes::Processes;
pub(crate) use processes::{monitor_reference, schedule};
pub use reference::Reference;
pub use subject::Subject;
pub use values::{charlist_string, native_rules, new_reference, pid_value};
pub(crate) use values::{fresh_name, subject_parts};

use crate::GleamErlangHostProfile;
use geam_core::host::{HostCall, HostCallError, HostProvider, HostType};
use geam_core::provider::{
    Call, ProviderActiveCall, ProviderFactoryBindings, ProviderValue, ProviderValueContext, Value,
};

/// Process operations available to ordinary macro-authored provider calls.
///
/// Lookup, registration and message delivery use the shared domain and do not
/// require a source process. Current-process and receive operations do.
pub trait ProcessCall {
    type Profile: GleamErlangHostProfile;

    fn current_process(&mut self) -> Result<Pid, HostCallError>;

    fn send<Message, Host: HostType>(
        &mut self,
        target: &Pid,
        message: Value<Message, ProviderValueContext<Host>>,
    );

    fn is_alive(&mut self, target: &Pid) -> bool;

    fn named<Message>(&mut self, name: &Name<Message>) -> Option<Pid>;

    fn register<Message>(&mut self, target: &Pid, name: &Name<Message>) -> bool;

    fn unregister<Message>(&mut self, name: &Name<Message>) -> bool;

    fn new_reference(&mut self) -> Reference;

    fn new_name<Message>(&mut self, prefix: &str) -> Result<Name<Message>, HostCallError>;

    fn send_subject<Message: ProviderValue>(
        &mut self,
        subject: Subject<Message>,
        message: Value<Message, ProviderValueContext<Message::Host>>,
    ) -> bool;

    fn receive_subject<Message: ProviderValue>(
        &mut self,
        subject: Subject<Message>,
        timeout: Option<std::time::Duration>,
    ) -> Result<Receive<Self::Profile>, HostCallError>;

    /// Receives the payload of a tagged pair, using this call's host clock.
    fn receive_tagged<Tag, Host: HostType>(
        &mut self,
        tag: Value<Tag, ProviderValueContext<Host>>,
        timeout: Option<std::time::Duration>,
    ) -> Result<Receive<Self::Profile>, HostCallError>;

    /// Receives the oldest message or signal. `None` waits without a deadline.
    fn receive_any(
        &mut self,
        timeout: Option<std::time::Duration>,
    ) -> Result<Receive<Self::Profile>, HostCallError>;
}

impl<'call, Profile, Provider, Return, Bindings> ProcessCall
    for Call<Provider::State, ProviderActiveCall<'call, Profile, Provider, Return, Bindings>>
where
    Profile: GleamErlangHostProfile,
    Provider: HostProvider<Profile>,
    Return: HostType,
    Bindings: ProviderFactoryBindings,
{
    type Profile = Profile;

    fn current_process(&mut self) -> Result<Pid, HostCallError> {
        Processes::new(self.host_call()).current().map(Pid::new)
    }

    fn send<Message, Host: HostType>(
        &mut self,
        target: &Pid,
        message: Value<Message, ProviderValueContext<Host>>,
    ) {
        let call = self.host_call();
        let target = target.execution_unit();
        let message = message.into_host(call);
        let message = call.native_value::<Host>(message);
        Processes::new(call).send(&target, message);
    }

    fn is_alive(&mut self, target: &Pid) -> bool {
        let call = self.host_call();
        let target = target.execution_unit();
        Processes::new(call).is_alive(&target)
    }

    fn named<Message>(&mut self, name: &Name<Message>) -> Option<Pid> {
        let mut processes = Processes::new(self.host_call());
        name.with_name(|name| processes.named(name)).map(Pid::new)
    }

    fn register<Message>(&mut self, target: &Pid, name: &Name<Message>) -> bool {
        let mut processes = Processes::new(self.host_call());
        name.with_name(|name| processes.register(&target.execution_unit(), name.into()))
    }

    fn unregister<Message>(&mut self, name: &Name<Message>) -> bool {
        let mut processes = Processes::new(self.host_call());
        name.with_name(|name| processes.unregister(name))
    }

    fn new_reference(&mut self) -> Reference {
        Reference::new()
    }

    fn new_name<Message>(&mut self, prefix: &str) -> Result<Name<Message>, HostCallError> {
        fresh_name(self.host_call(), prefix).map(Name::new)
    }

    fn send_subject<Message: ProviderValue>(
        &mut self,
        subject: Subject<Message>,
        message: Value<Message, ProviderValueContext<Message::Host>>,
    ) -> bool {
        let call = self.host_call();
        let subject = subject.into_host(call);
        let message = message.into_host(call);
        let message = call.native_value::<Message::Host>(message);
        Processes::new(call).send_subject(subject, message)
    }

    fn receive_subject<Message: ProviderValue>(
        &mut self,
        subject: Subject<Message>,
        timeout: Option<std::time::Duration>,
    ) -> Result<Receive<Profile>, HostCallError> {
        let call = self.host_call();
        let subject = subject.into_host(call);
        let deadline = receive_timeout(call, timeout)?;
        Processes::new(call).receive_subject(subject, deadline)
    }

    fn receive_tagged<Tag, Host: HostType>(
        &mut self,
        tag: Value<Tag, ProviderValueContext<Host>>,
        timeout: Option<std::time::Duration>,
    ) -> Result<Receive<Profile>, HostCallError> {
        let call = self.host_call();
        let tag = tag.into_host(call);
        let tag = call.native_value::<Host>(tag);
        let deadline = receive_timeout(call, timeout)?;
        Processes::new(call).receive(tag, deadline)
    }

    fn receive_any(
        &mut self,
        timeout: Option<std::time::Duration>,
    ) -> Result<Receive<Profile>, HostCallError> {
        let call = self.host_call();
        let deadline = receive_timeout(call, timeout)?;
        Processes::new(call).receive_any(deadline)
    }
}

fn receive_timeout<Profile, Provider, Return>(
    call: &HostCall<'_, Profile, Provider, Return>,
    timeout: Option<std::time::Duration>,
) -> Result<Option<std::time::Instant>, HostCallError>
where
    Profile: GleamErlangHostProfile,
    Provider: HostProvider<Profile>,
    Return: HostType,
{
    timeout
        .map(|timeout| {
            call.clock().now().checked_add(timeout).ok_or_else(|| {
                geam_core::HostFailure::new("timeout exceeds the host clock range").into()
            })
        })
        .transpose()
}

#[cfg(test)]
mod tests {
    use super::ProcessCall;
    use crate::{Component, GleamErlangProfile};
    use geam_core::host::{
        HostCall, HostCallContinuation, HostCallError, HostConstructions, HostOwnedCompletion,
        HostProviderModule, HostTypeListEnd,
    };
    use geam_core::provider::{Call, Value};
    use geam_core::{ModuleSource, PackageSource};
    use num_bigint::BigInt;
    use std::time::Duration;

    #[test]
    fn authoring_calls_share_the_mailbox_and_reject_unrepresentable_deadlines() {
        let provider = HostProviderModule::new("application", "main")
            .unwrap()
            .with_resumable_function::<Component<GleamErlangProfile>, (), BigInt, HostTypeListEnd, _>("check", check)
            .unwrap();
        let result = crate::test_support::run_main(
            [PackageSource::new(
                "application",
                Vec::<String>::new(),
                [ModuleSource::new(
                    "main",
                    "main.gleam",
                    r#"
@external(erlang, "host", "check") fn check() -> Int
pub fn main() { check() }
"#,
                )],
            )],
            [provider],
        );
        assert_eq!(result, geam_core::Value::Int(42.into()));
    }

    fn check<'call>(
        host: HostCall<'call, GleamErlangProfile, Component<GleamErlangProfile>, BigInt>,
        constructions: HostConstructions<'call, HostTypeListEnd>,
    ) -> Result<HostCallContinuation<'call, BigInt>, HostCallError> {
        let message = Value::<BigInt, geam_core::provider::ProviderValueContext<BigInt>>::from_host(
            &host,
            42.into(),
        );
        let tag = Value::<BigInt, geam_core::provider::ProviderValueContext<BigInt>>::from_host(
            &host,
            7.into(),
        );
        let mut call = Call::from_host_call(host);
        let current = call.current_process().unwrap();
        assert!(call.is_alive(&current));
        let name = call.new_name::<BigInt>("owner").unwrap();
        assert!(call.register(&current, &name));
        assert!(!call.register(&current, &name));
        assert_eq!(
            call.named(&name).unwrap().execution_unit().id(),
            current.execution_unit().id()
        );
        assert!(call.unregister(&name));
        assert!(call.named(&name).is_none());
        assert_eq!(
            call.new_name::<BigInt>(&"x".repeat(256))
                .err()
                .unwrap()
                .to_string(),
            "atom name exceeds 255 Unicode codepoints"
        );
        assert_eq!(
            call.receive_any(Some(Duration::MAX))
                .err()
                .unwrap()
                .to_string(),
            "timeout exceeds the host clock range"
        );
        assert_eq!(
            call.receive_tagged(tag, Some(Duration::MAX))
                .err()
                .unwrap()
                .to_string(),
            "timeout exceeds the host clock range"
        );
        call.send(&current, message);
        let host = call.host_call();
        let tag = host.native_value::<BigInt>(7.into());
        let payload = host.native_value::<BigInt>(8.into());
        super::Processes::new(host).send(
            &current.execution_unit(),
            geam_core::provider::advanced::NativeValue::tuple([tag, payload]),
        );
        let tag = Value::<BigInt, geam_core::provider::ProviderValueContext<BigInt>>::from_host(
            host,
            7.into(),
        );
        let selected = call.receive_tagged(tag, None).unwrap();
        let receive = call.receive_any(None).unwrap();
        Ok(call.into_host_call().resume(constructions, move |context| {
            Box::pin(async move {
                assert_eq!(
                    selected.wait_forever(&context).await.unwrap().as_int(),
                    Some(8.into())
                );
                let message = receive.wait(&context).await.unwrap().unwrap();
                assert_eq!(message.as_int(), Some(42.into()));
                Ok(HostOwnedCompletion::new(|call, _| {
                    Ok(call.return_value(42.into()))
                }))
            })
        }))
    }
}

#[cfg(test)]
#[path = "../../../core/tests/support/work_fixture.rs"]
mod work_fixture;

#[cfg(test)]
mod domain_tests {
    use super::types::{
        Monitor, MonitorSchema, OrdinarySubject, Selector, SelectorSchema, Subject, SubjectSchema,
    };
    use super::{CurrentProcess, Pid, ProcessCall, Processes, with_current_process};
    use crate::{Component, Configuration, ErlangExecution, GleamErlangHostProfile};
    use geam_core::embedding::{CallError, FunctionDeclaration, HostedModuleBuilder};
    use geam_core::host::{
        HostCall, HostCallCompletion, HostCallContinuation, HostCallError, HostCallable,
        HostComponentProfile, HostConstructions, HostFunctionType, HostFutureStore,
        HostOwnedCompletion, HostProfile, HostProvider, HostProviderModule, HostProviderSet,
        HostTypeIndex0, HostTypeIndexNext, HostTypeList, HostTypeListEnd, HostWorkProfile,
    };
    use geam_core::provider::advanced::NativeValue;
    use geam_core::provider::{Call, ProviderInputValue, ProviderValueContext, Value};
    use geam_core::{ModuleSource, PackageSource};
    use geam_stdlib::provider_support::{Dynamic, DynamicSchema};
    use geam_stdlib::{GleamStdlibRunState, GleamStdlibStores, IoOutput};
    use num_bigint::BigInt;

    use super::work_fixture::{WorkComponent, WorkHostType, WorkType};

    type Index1 = HostTypeIndexNext<HostTypeIndex0>;
    type Index2 = HostTypeIndexNext<Index1>;
    type Index3 = HostTypeIndexNext<Index2>;
    type Index4 = HostTypeIndexNext<Index3>;
    type Index5 = HostTypeIndexNext<Index4>;
    type Constructions = HostTypeList<
        crate::Pid,
        HostTypeList<
            Monitor,
            HostTypeList<
                crate::Reference,
                HostTypeList<
                    Subject<BigInt>,
                    HostTypeList<Dynamic, HostTypeList<Selector<BigInt>, HostTypeListEnd>>,
                >,
            >,
        >,
    >;

    struct Profile;

    struct State {
        stdlib: GleamStdlibRunState,
        erlang: Configuration,
        work: (),
        received: Option<futures_channel::oneshot::Sender<BigInt>>,
        creator: Option<geam_core::execution::ExecutionUnit>,
        gate: Option<std::sync::Arc<crate::test_support::PollGate>>,
    }

    #[derive(Default)]
    struct Stores {
        stdlib: GleamStdlibStores,
        erlang: crate::Stores<Profile>,
        work: HostFutureStore,
    }

    impl HostProfile for Profile {
        type RunState = State;
        type ExternalStores = Stores;
        type ExecutionState = ErlangExecution;
    }

    impl geam_stdlib::GleamStdlibHostProfile for Profile {
        type Io = Vec<IoOutput>;
    }

    impl HostComponentProfile<geam_stdlib::Component> for Profile {
        fn component_stores(stores: &Stores) -> &GleamStdlibStores {
            &stores.stdlib
        }

        fn component_state(state: &mut State) -> &mut GleamStdlibRunState {
            &mut state.stdlib
        }
    }

    impl HostComponentProfile<Component<Self>> for Profile {
        fn component_stores(stores: &Stores) -> &crate::Stores<Self> {
            &stores.erlang
        }

        fn component_state(state: &mut State) -> &mut Configuration {
            &mut state.erlang
        }
    }

    impl GleamErlangHostProfile for Profile {
        fn erlang_execution(state: &mut ErlangExecution) -> &mut ErlangExecution {
            state
        }
    }

    impl HostComponentProfile<WorkComponent> for Profile {
        fn component_stores(stores: &Stores) -> &HostFutureStore {
            &stores.work
        }

        fn component_state(state: &mut State) -> &mut () {
            &mut state.work
        }
    }

    impl HostWorkProfile for Profile {
        type Work = WorkComponent;
    }

    impl HostProvider<Self> for Profile {
        type State = State;

        fn project(state: &mut State) -> &mut State {
            state
        }
    }

    #[test]
    fn manual_profile_projects_the_callers_component_states() {
        let mut state = State {
            stdlib: GleamStdlibRunState::from_seed([7; 32]),
            erlang: Configuration::default(),
            work: (),
            received: None,
            creator: None,
            gate: None,
        };
        let stdlib = &raw const state.stdlib;
        let work = &raw const state.work;
        assert!(std::ptr::eq(
            <Profile as HostComponentProfile<geam_stdlib::Component>>::component_state(&mut state),
            stdlib,
        ));
        assert!(std::ptr::eq(
            <Profile as HostComponentProfile<WorkComponent>>::component_state(&mut state),
            work,
        ));
        <Profile as HostComponentProfile<Component<Profile>>>::component_state(&mut state)
            .resources
            .insert("owner".into(), "owner/resources".into());
        assert_eq!(
            state.erlang.resources.into_iter().collect::<Vec<_>>(),
            [("owner".into(), "owner/resources".into())],
        );
        assert!(state.stdlib.io_outputs().is_empty());
    }

    #[test]
    fn owned_work_can_route_messages_after_its_source_process_has_finished() {
        let process = HostProviderModule::new("gleam_erlang", "gleam/erlang/process")
            .unwrap()
            .with_shared_custom_type::<SubjectSchema>()
            .unwrap()
            .with_external_type::<Component<Profile>, crate::PidSchema>()
            .unwrap()
            .with_external_type::<Component<Profile>, crate::NameSchema>()
            .unwrap()
            .with_external_type::<Component<Profile>, MonitorSchema>()
            .unwrap()
            .with_external_type::<Component<Profile>, SelectorSchema>()
            .unwrap();
        let reference = HostProviderModule::new("gleam_erlang", "gleam/erlang/reference")
            .unwrap()
            .with_external_type::<Component<Profile>, crate::ReferenceSchema>()
            .unwrap();
        let dynamic = HostProviderModule::new("gleam_stdlib", "gleam/dynamic")
            .unwrap()
            .with_external_type::<Component<Profile>, DynamicSchema>()
            .unwrap();
        let (gate, driver) = crate::test_support::PollGate::new();
        let provider = HostProviderModule::new("application", "main")
            .unwrap()
            .with_scoped_function_and_constructions::<Profile, (HostFunctionType<HostTypeListEnd, ()>,), WorkHostType<BigInt>, Constructions, _>("make", make)
            .unwrap()
            .with_resumable_function::<Profile, (), (), Constructions, _>("receive", receive)
            .unwrap()
            .with_resumable_function::<Profile, (bool,), (), Constructions, _>("checked", checked)
            .unwrap()
            .with_resumable_function::<Profile, (), (), Constructions, _>("cancel", cancelled).unwrap();
        let typed = geam_core::compile_typed_host_program(
            "application",
            "main",
            [
                PackageSource::new("gleam_stdlib", Vec::<String>::new(), [
                    ModuleSource::new("gleam/dynamic", "dynamic.gleam", "pub type Dynamic"),
                ]),
                PackageSource::new("gleam_erlang", ["gleam_stdlib"], [
                    ModuleSource::new("gleam/erlang/reference", "reference.gleam", "pub type Reference"),
                    ModuleSource::new("gleam/erlang/process", "process.gleam", r#"
    import gleam/dynamic.{type Dynamic}
    pub type Pid
    pub type Name(message)
    pub type Monitor
    pub type Selector(message)
    pub opaque type Subject(message) { Subject(owner: Pid, tag: Dynamic) NamedSubject(name: Name(message)) }
    "#),
                ]),
                PackageSource::new(
                    "work_fixture",
                    Vec::<String>::new(),
                    [ModuleSource::new("fixture/work", "work.gleam", WorkComponent::SOURCE)],
                ),
                PackageSource::new(
                    "application",
                    ["work_fixture", "gleam_erlang"],
                    [ModuleSource::new(
                        "main",
                        "main.gleam",
                        r#"
    import fixture/work
    @external(erlang, "host", "make") fn make(child: fn() -> Nil) -> work.Work(Int)
    @external(erlang, "host", "receive") fn receive() -> Nil
    @external(erlang, "host", "checked") fn checked(close: Bool) -> Nil
    @external(erlang, "host", "cancel") fn cancel() -> Nil
    pub fn main() { make(fn() { receive() }) }
    pub fn probe(close: Bool) { checked(close) }
    pub fn wait() { cancel() }
    "#,
                    )],
                ),
            ],
            HostProviderSet::from_providers(
                WorkComponent::providers::<Profile>().unwrap().into_iter().chain([provider, process, reference, dynamic]),
            )
            .unwrap(),
        )
        .unwrap();
        let (mut builder, main) = HostedModuleBuilder::new(typed)
            .unwrap()
            .function(FunctionDeclaration::<(), WorkType<BigInt>>::new("main"))
            .unwrap();
        let probe = builder
            .function(FunctionDeclaration::<(bool,), ()>::new("probe"))
            .unwrap();
        let wait = builder
            .function(FunctionDeclaration::<(), ()>::new("wait"))
            .unwrap();
        let mut module = builder.seal().unwrap();
        let host = crate::execution_fixture::TestHost::default();
        let mut state = State {
            stdlib: GleamStdlibRunState::from_seed([0; 32]),
            erlang: Configuration::default(),
            work: (),
            received: None,
            creator: None,
            gate: Some(gate),
        };
        let result = host
            .block_on(
                module.with_execution(&host, &mut state, &mut Vec::new(), async |scope| {
                    assert_eq!(scope.call(&probe, (false,)).await, Ok(()));
                    assert_eq!(scope.call(&probe, (true,)).await, Err(CallError::Cancelled));
                    let work = scope.call(&main, ()).await.unwrap();
                    scope.observe(&work).await.unwrap().read(Clone::clone)
                }),
            )
            .unwrap();
        assert_eq!(result, BigInt::from(85));
        assert!(state.received.is_none());
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
            Err(CallError::Cancelled)
        );
        drop(execution);
        assert!(echo.is_empty());
    }

    fn checked_identity<'call>(
        mut process: CurrentProcess<'call, Profile, Profile, ()>,
        _: HostConstructions<'call, Constructions>,
    ) -> Result<(), HostCallError> {
        assert!(process.current().is_active());
        assert!(process.processes().named("missing").is_none());
        let current = process.current().id();
        let call = process.into_call();
        assert_eq!(call.execution_unit().unwrap().id(), current);
        Ok(())
    }

    fn cancelled<'call>(
        mut call: HostCall<'call, Profile, Profile, ()>,
        constructions: HostConstructions<'call, Constructions>,
    ) -> Result<HostCallContinuation<'call, ()>, HostCallError> {
        let gate = call.state().gate.take().unwrap();
        let unit = call.execution_unit().unwrap();
        Ok(call.resume(constructions, move |context| {
            Box::pin(async move {
                let request = with_current_process(&context, checked_identity);
                Err(gate.observe(request, unit).await.err().unwrap())
            })
        }))
    }

    fn checked<'call>(
        call: HostCall<'call, Profile, Profile, ()>,
        constructions: HostConstructions<'call, Constructions>,
        close: bool,
    ) -> Result<HostCallContinuation<'call, ()>, HostCallError> {
        CurrentProcess::with(call, |mut process| {
            let current = process.current().clone();
            let pid = super::pid_value(
                process.call(),
                constructions.at::<HostTypeIndex0>(),
                current.clone(),
            );
            assert!(process.link(pid, constructions.at::<HostTypeIndex0>()));
            process.trap_exits(true);
            process.send_exit(
                &current,
                NativeValue::symbol("normal"),
                constructions.at::<HostTypeIndex0>(),
            );
            let tag = NativeValue::symbol("checked");
            process.processes().send(
                &current,
                NativeValue::tuple([tag.clone(), NativeValue::symbol("ready")]),
            );
            if close {
                process.send_exit(
                    &current,
                    NativeValue::symbol("kill"),
                    constructions.at::<HostTypeIndex0>(),
                );
                for error in [
                    process.receive_any(None).err().unwrap(),
                    process
                        .receive_record_with(NativeValue::symbol("EXIT"), exit_reason, None)
                        .err()
                        .unwrap(),
                    process
                        .receive_record(NativeValue::symbol("EXIT"), 2, None)
                        .err()
                        .unwrap(),
                ] {
                    assert_eq!(error.to_string(), "process mailbox is closed");
                }
            } else {
                // Preparing and discarding alternatives must not consume either message.
                drop(process.receive_any(None).unwrap());
                drop(
                    process
                        .receive_record(NativeValue::symbol("EXIT"), 2, None)
                        .unwrap(),
                );
            }
            process.resume_receive(constructions, tag, None, |receive, context| {
                Box::pin(async move {
                    assert_eq!(
                        receive
                            .wait_forever(&context)
                            .await
                            .unwrap()
                            .as_symbol()
                            .as_deref(),
                        Some("ready")
                    );
                    with_current_process(&context, checked_identity)
                        .await
                        .unwrap();
                    Ok(HostOwnedCompletion::new(
                        |call, _| Ok(call.return_value(())),
                    ))
                })
            })
        })
    }

    fn make<'call>(
        mut host: HostCall<'call, Profile, Profile, WorkHostType<BigInt>>,
        constructions: HostConstructions<'call, Constructions>,
        child: HostCallable<'call, HostTypeListEnd, ()>,
    ) -> Result<HostCallCompletion<'call, WorkHostType<BigInt>>, HostCallError> {
        let creator = host.require_execution_unit().unwrap();
        host.state().creator = Some(creator.clone());
        let target = Pid::new(host.spawn(child));
        let (sent, received) = futures_channel::oneshot::channel();
        host.state().received = Some(sent);
        Ok(host.return_future(constructions, move |context| {
            Box::pin(async move {
                assert!(!creator.is_active());
                assert_eq!(with_current_process(context.execution(), checked_identity).await.err().unwrap().to_string(), "native operation requires a source invocation");
                context.execution().with_constructions(move |mut host, constructions| {
                    assert!(host.execution_unit().is_none());
                    let pid = super::pid_value(&mut host, constructions.at::<HostTypeIndex0>(), target.execution_unit());
                    let tag = host.construct_external_with_binding::<Component<Profile>, DynamicSchema, HostTypeListEnd>(constructions.at::<Index4>(), geam_stdlib::Dynamic::from_native(NativeValue::symbol("reply")));
                    let subject = host.construct_custom::<OrdinarySubject<BigInt>>(constructions.at::<Index3>(), (pid, (tag, ())));
                    let selector = host.construct_external_with_binding::<Component<Profile>, SelectorSchema, HostTypeList<BigInt, HostTypeListEnd>>(constructions.at::<Index5>(), crate::selector::Selector::default());
                    let mut processes = Processes::new(&mut host);
                    let errors = [
                        processes.link(pid, constructions.at::<HostTypeIndex0>()).err().unwrap(),
                        processes.monitor(pid, constructions.at::<Index1>(), constructions.at::<Index2>()).err().unwrap(),
                        processes.send_exit(&target.execution_unit(), NativeValue::symbol("kill"), constructions.at::<HostTypeIndex0>()).err().unwrap(),
                        processes.unlink(&target.execution_unit()).err().unwrap(),
                        processes.trap_exits(true).err().unwrap(),
                        processes.receive_record(NativeValue::symbol("EXIT"), 2, None).err().unwrap(),
                        processes.receive_selector(selector, None).err().unwrap(),
                    ];
                    for error in errors {
                        assert_eq!(error.to_string(), "native operation requires a source invocation");
                    }
                    let send_subject = super::Subject::<BigInt>::from_host(&mut host, subject);
                    let receive_subject = super::Subject::<BigInt>::from_host(&mut host, subject);
                    let message = Value::<BigInt, ProviderValueContext<BigInt>>::from_host(&host, 42.into());
                    let tagged = Value::<BigInt, ProviderValueContext<BigInt>>::from_host(&host, 43.into());
                    let tag = Value::<BigInt, ProviderValueContext<BigInt>>::from_host(&host, 7.into());
                    let mut call = Call::from_host_call(host);
                    assert!(call.is_alive(&target));
                    let name = call.new_name::<BigInt>("work").unwrap();
                    assert!(call.named(&name).is_none());
                    assert!(call.register(&target, &name));
                    assert_eq!(call.named(&name).unwrap().execution_unit().id(), target.execution_unit().id());
                    assert!(call.unregister(&name));
                    assert!(call.named(&name).is_none());
                    call.send(&target, message);
                    assert!(call.send_subject(send_subject, tagged));
                    assert_eq!(call.current_process().err().unwrap().to_string(), "native operation requires a source invocation");
                    assert_eq!(call.receive_any(None).err().unwrap().to_string(), "native operation requires a source invocation");
                    assert_eq!(call.receive_tagged(tag, None).err().unwrap().to_string(), "native operation requires a source invocation");
                    assert_eq!(call.receive_subject(receive_subject, None).err().unwrap().to_string(), "native operation requires a source invocation");
                }).await.unwrap();
                let value = received.await.unwrap();
                Ok(HostOwnedCompletion::new(move |call, _| Ok(call.return_value(value))))
            })
        }))
    }

    fn receive<'call>(
        mut call: HostCall<'call, Profile, Profile, ()>,
        constructions: HostConstructions<'call, Constructions>,
    ) -> Result<HostCallContinuation<'call, ()>, HostCallError> {
        let creator = call.state().creator.take().unwrap();
        let receive = Processes::new(&mut call).receive_any(None).unwrap();
        Ok(call.resume(constructions, move |context| {
            Box::pin(async move {
                let message = receive.wait_forever(&context).await.unwrap();
                let value = message.as_int().unwrap();
                assert_eq!(value, BigInt::from(42));
                let (tagged, normal, shutdown) = context.with_constructions(move |mut call, constructions| {
                    assert!(!creator.is_active());
                    let current = Processes::new(&mut call).current().unwrap();
                    let pid = super::pid_value(&mut call, constructions.at::<HostTypeIndex0>(), current.clone());
                    let old_pid = super::pid_value(&mut call, constructions.at::<HostTypeIndex0>(), creator.clone());
                    let tag = call.construct_external_with_binding::<Component<Profile>, DynamicSchema, HostTypeListEnd>(constructions.at::<Index4>(), geam_stdlib::Dynamic::from_native(NativeValue::symbol("reply")));
                    let subject = call.construct_custom::<OrdinarySubject<BigInt>>(constructions.at::<Index3>(), (pid, (tag, ())));
                    let foreign = call.construct_custom::<OrdinarySubject<BigInt>>(constructions.at::<Index3>(), (old_pid, (tag, ())));
                    let selector = call.construct_external_with_binding::<Component<Profile>, SelectorSchema, HostTypeList<BigInt, HostTypeListEnd>>(constructions.at::<Index5>(), crate::selector::Selector::default());
                    let mut processes = Processes::new(&mut call);
                    assert!(processes.link(pid, constructions.at::<HostTypeIndex0>()).unwrap());
                    let _monitor = processes.monitor(pid, constructions.at::<Index1>(), constructions.at::<Index2>()).unwrap();
                    processes.unlink(&current).unwrap();
                    processes.trap_exits(true).unwrap();
                    processes.send_exit(&creator, NativeValue::symbol("kill"), constructions.at::<HostTypeIndex0>()).unwrap();
                    processes.send_exit(&current, NativeValue::symbol("normal"), constructions.at::<HostTypeIndex0>()).unwrap();
                    processes.send_exit(&current, NativeValue::symbol("shutdown"), constructions.at::<HostTypeIndex0>()).unwrap();
                    drop(processes.receive_selector(selector, None).unwrap());
                    // Preparing an alternative receive must leave the reply queued.
                    drop(processes.receive(NativeValue::symbol("reply"), None).unwrap());
                    assert_eq!(processes.receive_subject(foreign, None).err().unwrap().to_string(), "subject belongs to another process");
                    let tagged = processes.receive_subject(subject, None).unwrap();
                    let normal = processes.receive_record(NativeValue::symbol("EXIT"), 2, None).unwrap();
                    let shutdown = CurrentProcess::with(call, |mut process| {
                        process.receive_record_with(NativeValue::symbol("EXIT"), exit_reason, None)
                    }).unwrap();
                    (tagged, normal, shutdown)
                }).await.unwrap();
                let payload = tagged
                    .wait_forever(&context)
                    .await
                    .unwrap()
                    .as_int()
                    .unwrap();
                assert_eq!(payload, BigInt::from(43));
                let normal = normal.wait_forever(&context).await.unwrap();
                assert_eq!(normal.index(2).unwrap().as_symbol().as_deref(), Some("normal"));
                let shutdown = shutdown.wait_forever(&context).await.unwrap();
                assert_eq!(shutdown.as_symbol().as_deref(), Some("shutdown"));
                let value = value + payload;
                context
                    .with_state(move |state| state.received.take().unwrap().send(value).unwrap())
                    .await
                    .unwrap();
                Ok(HostOwnedCompletion::new(
                    |call, _| Ok(call.return_value(())),
                ))
            })
        }))
    }

    fn exit_reason(message: &NativeValue) -> Option<NativeValue> {
        message.index(2)
    }
}
