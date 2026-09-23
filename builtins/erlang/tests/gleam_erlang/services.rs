use geam_core::execution::{
    ExecutionMetadata, ExecutionServices, ExecutionUnit, ExecutionUnitId, HostExecutionState,
    UnitExit,
};
use geam_core::{
    HostComponentProfile, HostExecutionService, HostProfile, HostProviderComponent,
    HostProviderComponentRegistration, HostProviderSet, HostServiceProfile, HostedExecution,
    compile_typed_host_project, plan_host_program,
};
use geam_erlang::{Configuration, ErlangExecution, GleamErlangHostProfile};
use geam_stdlib::{GleamStdlibRunState, GleamStdlibStores, IoOutput};
use std::sync::{Arc, Mutex};

struct Tickets;
struct TicketConfiguration {
    first: usize,
    events: Arc<Mutex<Vec<String>>>,
}

#[derive(Default)]
struct TicketService {
    next: usize,
    events: Arc<Mutex<Vec<String>>>,
}

impl HostProviderComponent for Tickets {
    const ID: &'static str = "tickets";
    type Stores = ();
    type RunState = TicketConfiguration;
}
impl HostExecutionService for Tickets {
    type State = TicketService;
    fn initialize_service(configuration: &mut TicketConfiguration) -> TicketService {
        configuration.events.lock().unwrap().push("created".into());
        TicketService {
            next: configuration.first,
            events: Arc::clone(&configuration.events),
        }
    }
}
impl HostExecutionState for TicketService {
    fn initialize(&mut self, _: ExecutionMetadata<'_>) {
        self.events.lock().unwrap().push("initialized".into());
    }
    fn started(&mut self, _: ExecutionUnit) {
        self.events.lock().unwrap().push("started".into());
    }
    fn finished(&mut self, _: ExecutionUnitId, exit: &UnitExit) {
        self.events
            .lock()
            .unwrap()
            .push(format!("finished {exit:?}"));
    }
    fn close(&mut self) {
        self.events.lock().unwrap().push("closed".into());
    }
}

trait ServiceProfile: GleamErlangHostProfile + HostServiceProfile<Tickets> {}
impl<Profile: GleamErlangHostProfile + HostServiceProfile<Tickets>> ServiceProfile for Profile {}

mod consumer_a {
    use super::{ServiceProfile, Tickets};
    #[geam_macros::provider(package = "geam_erlang_test", modules = [first], crate_path = geam_core)]
    pub struct Component;

    #[geam_macros::module(path = "service_consumer_a", profile = super::ServiceProfile, component = super::Component, crate_path = geam_core)]
    mod first {
        use geam_core::provider::{BigInt, Call, HostResult, Value};
        use geam_erlang::service::{self, ProcessCall};

        #[geam_macros::function(profile = Profile)]
        fn ticket(#[geam_macros::call] call: &mut Call<()>) -> BigInt {
            let tickets = call.service::<super::Tickets>();
            let next = tickets.next;
            tickets.next += 1;
            next.into()
        }

        #[geam_macros::function(profile = Profile)]
        fn current(#[geam_macros::call] call: &mut Call<()>) -> HostResult<service::Pid> {
            call.current_process()
        }

        #[geam_macros::function(profile = Profile)]
        fn alive(#[geam_macros::call] call: &mut Call<()>, pid: service::Pid) -> bool {
            call.is_alive(&pid)
        }

        #[geam_macros::function(profile = Profile)]
        fn send<Message>(
            #[geam_macros::call] call: &mut Call<()>,
            pid: service::Pid,
            message: Value<Message>,
        ) -> () {
            call.send(&pid, message)
        }

        #[geam_macros::function(profile = Profile)]
        fn reference(#[geam_macros::call] call: &mut Call<()>) -> service::Reference {
            call.new_reference()
        }

        #[geam_macros::function(await, profile = Profile)]
        async fn receive<Tag>(
            #[geam_macros::call] call: &mut Call<()>,
            tag: Value<Tag>,
        ) -> HostResult<Result<geam_stdlib::Dynamic, ()>> {
            let receive = call
                .with_call(move |call| call.receive_tagged(tag, Some(std::time::Duration::ZERO)))
                .await??;
            Ok(receive
                .wait_in(call)
                .await?
                .map(geam_stdlib::Dynamic::from_native)
                .ok_or(()))
        }

        #[geam_macros::function(await, profile = Profile)]
        async fn receive_any(
            #[geam_macros::call] call: &mut Call<()>,
        ) -> HostResult<Result<geam_stdlib::Dynamic, ()>> {
            let receive = call
                .with_call(|call| call.receive_any(Some(std::time::Duration::ZERO)))
                .await??;
            Ok(receive
                .wait_in(call)
                .await?
                .map(geam_stdlib::Dynamic::from_native)
                .ok_or(()))
        }

        #[geam_macros::function(await, profile = Profile)]
        async fn receive_forever(
            #[geam_macros::call] call: &mut Call<()>,
        ) -> HostResult<geam_stdlib::Dynamic> {
            let receive = call.with_call(|call| call.receive_any(None)).await??;
            receive
                .wait_forever_in(call)
                .await
                .map(geam_stdlib::Dynamic::from_native)
        }

        #[geam_macros::function(profile = Profile)]
        fn new_name<Message>(
            #[geam_macros::call] call: &mut Call<()>,
            prefix: geam_core::StringValue,
        ) -> HostResult<service::Name<Message>> {
            call.new_name(&prefix)
        }

        #[geam_macros::function(profile = Profile)]
        fn subject<Message>(subject: service::Subject<Message>) -> service::Subject<Message> {
            subject
        }

        #[geam_macros::function(profile = Profile)]
        fn subjects<Message>(
            subjects: geam_core::provider::List<service::Subject<Message>>,
        ) -> Vec<service::Subject<Message>> {
            vec![subjects.get(1).unwrap(), subjects.get(0).unwrap()]
        }

        #[geam_macros::function(profile = Profile)]
        fn send_subject<Message>(
            #[geam_macros::call] call: &mut Call<()>,
            subject: service::Subject<Message>,
            message: Value<Message>,
        ) -> bool {
            call.send_subject(subject, message)
        }

        #[geam_macros::function(await, profile = Profile)]
        async fn receive_subject<Message>(
            #[geam_macros::call] call: &mut Call<()>,
            subject: service::Subject<Message>,
        ) -> HostResult<Result<Value<Message>, ()>> {
            let receive = call
                .with_call(move |call| {
                    call.receive_subject(subject, Some(std::time::Duration::ZERO))
                })
                .await??;
            let value = receive.wait_in(call).await?;
            call.with_call(move |call| {
                value
                    .map(|value| {
                        call.restore_native::<Value<Message>>(&value)
                            .ok_or_else(|| {
                                geam_core::HostFailure::new(
                                    "subject message has the wrong source type",
                                )
                            })
                    })
                    .transpose()
                    .map(|value| value.ok_or(()))
            })
            .await?
            .map_err(Into::into)
        }

        #[geam_macros::function(profile = Profile)]
        fn named<Message>(
            #[geam_macros::call] call: &mut Call<()>,
            name: service::Name<Message>,
        ) -> Option<service::Pid> {
            call.named(&name)
        }

        #[geam_macros::function(profile = Profile)]
        fn register<Message>(
            #[geam_macros::call] call: &mut Call<()>,
            pid: service::Pid,
            name: service::Name<Message>,
        ) -> bool {
            call.register(&pid, &name)
        }

        #[geam_macros::function(profile = Profile)]
        fn unregister<Message>(
            #[geam_macros::call] call: &mut Call<()>,
            name: service::Name<Message>,
        ) -> bool {
            call.unregister(&name)
        }

        #[geam_macros::function(profile = Profile)]
        fn pids(values: geam_core::provider::List<service::Pid>) -> Vec<service::Pid> {
            vec![values.get(1).unwrap().clone(), values.get(0).unwrap()]
        }

        #[geam_macros::function(profile = Profile)]
        fn names<Message>(
            values: geam_core::provider::List<service::Name<Message>>,
        ) -> Vec<service::Name<Message>> {
            vec![values.get(1).unwrap().clone(), values.get(0).unwrap()]
        }

        #[geam_macros::function(profile = Profile)]
        fn references(
            values: geam_core::provider::List<service::Reference>,
        ) -> Vec<service::Reference> {
            vec![values.get(1).unwrap().clone(), values.get(0).unwrap()]
        }
    }
}
use consumer_a::Component as First;

mod consumer_b {
    use super::{ServiceProfile, Tickets};
    #[geam_macros::provider(package = "geam_erlang_test", modules = [second], crate_path = geam_core)]
    pub struct Component;

    #[geam_macros::module(path = "service_consumer_b", profile = super::ServiceProfile, component = super::Component, crate_path = geam_core)]
    mod second {
        use geam_core::provider::{BigInt, Call};
        use geam_erlang::service::{self, ProcessCall};

        #[geam_macros::function(profile = Profile)]
        fn ticket(#[geam_macros::call] call: &mut Call<()>) -> BigInt {
            let tickets = call.service::<super::Tickets>();
            let next = tickets.next;
            tickets.next += 1;
            next.into()
        }

        #[geam_macros::function(profile = Profile)]
        fn pid(pid: service::Pid) -> service::Pid {
            pid.clone()
        }

        #[geam_macros::function(await, profile = Profile)]
        async fn reference(reference: service::Reference) -> service::Reference {
            reference.clone()
        }

        #[geam_macros::function(await, profile = Profile)]
        async fn subject<Message>(subject: service::Subject<Message>) -> service::Subject<Message> {
            subject
        }

        #[geam_macros::function(await, profile = Profile)]
        async fn name<Message>(name: service::Name<Message>) -> service::Name<Message> {
            name.clone()
        }

        #[geam_macros::function(profile = Profile)]
        fn named<Message>(
            #[geam_macros::call] call: &mut Call<()>,
            name: service::Name<Message>,
        ) -> Option<service::Pid> {
            call.named(&name)
        }
    }
}
use consumer_b::Component as Second;

struct Profile;
#[derive(Default)]
struct Stores {
    stdlib: GleamStdlibStores,
    erlang: geam_erlang::Stores<Profile>,
    first: <First as HostProviderComponent>::Stores,
    second: <Second as HostProviderComponent>::Stores,
    tickets: (),
}
struct State {
    stdlib: GleamStdlibRunState,
    erlang: Configuration,
    first: (),
    second: (),
    tickets: TicketConfiguration,
}

impl HostProfile for Profile {
    type RunState = State;
    type ExternalStores = Stores;
    type ExecutionState = ExecutionServices<ErlangExecution, TicketService>;
    fn initialize_execution(state: &mut State) -> Self::ExecutionState {
        ExecutionServices {
            first: <geam_erlang::Component<Profile> as HostExecutionService>::initialize_service(
                &mut state.erlang,
            ),
            rest: Tickets::initialize_service(&mut state.tickets),
        }
    }
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
impl HostComponentProfile<geam_erlang::Component<Profile>> for Profile {
    fn component_stores(stores: &Stores) -> &geam_erlang::Stores<Profile> {
        &stores.erlang
    }
    fn component_state(state: &mut State) -> &mut Configuration {
        &mut state.erlang
    }
}
impl HostComponentProfile<First> for Profile {
    fn component_stores(stores: &Stores) -> &<First as HostProviderComponent>::Stores {
        &stores.first
    }
    fn component_state(state: &mut State) -> &mut () {
        &mut state.first
    }
}
impl HostComponentProfile<Second> for Profile {
    fn component_stores(stores: &Stores) -> &<Second as HostProviderComponent>::Stores {
        &stores.second
    }
    fn component_state(state: &mut State) -> &mut () {
        &mut state.second
    }
}
impl HostComponentProfile<Tickets> for Profile {
    fn component_stores(stores: &Stores) -> &() {
        &stores.tickets
    }
    fn component_state(state: &mut State) -> &mut TicketConfiguration {
        &mut state.tickets
    }
}
impl HostServiceProfile<Tickets> for Profile {
    fn service(state: &mut Self::ExecutionState) -> &mut TicketService {
        &mut state.rest
    }
}
impl HostServiceProfile<geam_erlang::Component<Profile>> for Profile {
    fn service(state: &mut Self::ExecutionState) -> &mut ErlangExecution {
        &mut state.first
    }
}
impl GleamErlangHostProfile for Profile {
    fn erlang_execution(state: &mut Self::ExecutionState) -> &mut ErlangExecution {
        <Self as HostServiceProfile<geam_erlang::Component<Self>>>::service(state)
    }
}

#[test]
fn two_macro_consumers_share_both_producer_services_and_preserve_exact_values() {
    let root = super::project_root();
    let mut providers = geam_stdlib::host_providers::<Profile>().unwrap();
    providers.extend(geam_erlang::host_providers::<Profile>().unwrap());
    providers.extend(<First as HostProviderComponentRegistration<Profile>>::providers().unwrap());
    providers.extend(<Second as HostProviderComponentRegistration<Profile>>::providers().unwrap());
    let typed = compile_typed_host_project(
        &root,
        "shared_services",
        HostProviderSet::from_providers(providers).unwrap(),
    )
    .unwrap();
    let resources = typed.package_resources().clone();
    let events = Arc::new(Mutex::new(Vec::new()));
    let mut execution =
        HostedExecution::try_from_module_plan(plan_host_program(typed).unwrap()).unwrap();
    assert!(events.lock().unwrap().is_empty());
    let host = super::execution_fixture::TestHost::default();
    for _ in 0..2 {
        let mut state = State {
            stdlib: GleamStdlibRunState::from_seed([0; 32]),
            erlang: Configuration {
                resources: resources.clone(),
            },
            first: (),
            second: (),
            tickets: TicketConfiguration {
                first: 40,
                events: Arc::clone(&events),
            },
        };
        let mut echo = Vec::new();
        let result = host
            .block_on(execution.run_main(&host, &mut state, &mut echo))
            .unwrap();
        assert_eq!(result.inspect().to_string(), "#(40, 41, 42)");
        assert!(echo.is_empty());
        assert_eq!(
            *events.lock().unwrap(),
            [
                "created",
                "initialized",
                "started",
                "started",
                "finished Cancelled",
                "finished Completed",
                "closed"
            ]
        );
        events.lock().unwrap().clear();
    }
}
