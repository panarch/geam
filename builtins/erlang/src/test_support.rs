use crate::execution_fixture::TestHost;
use geam_core::embedding::{FunctionDeclaration, HostedModuleBuilder};
use geam_core::execution::ExecutionUnit;
use geam_core::host::{
    HostCall, HostCallContinuation, HostCallError, HostConstructions, HostProfile, HostProvider,
    HostProviderModule, HostProviderSet, HostTypeListEnd,
};
use geam_core::{ModuleSource, PackageSource, compile_typed_host_program};

/// Stops inside one native poll while the owner drives real cancellation.
pub(crate) struct PollGate {
    events: std::sync::mpsc::Sender<PollEvent>,
    resume: std::sync::Mutex<std::sync::mpsc::Receiver<()>>,
}

pub(crate) struct PollDriver {
    events: std::sync::mpsc::Sender<PollEvent>,
    received: std::sync::mpsc::Receiver<PollEvent>,
    resume: std::sync::mpsc::Sender<()>,
}

enum PollEvent {
    TurnFinished,
    Paused(ExecutionUnit),
}

impl PollGate {
    pub(crate) fn new() -> (std::sync::Arc<Self>, PollDriver) {
        let (events, received) = std::sync::mpsc::channel();
        let (resume, resumed) = std::sync::mpsc::channel();
        (
            std::sync::Arc::new(Self {
                events: events.clone(),
                resume: std::sync::Mutex::new(resumed),
            }),
            PollDriver {
                events,
                received,
                resume,
            },
        )
    }

    pub(crate) async fn observe<Output>(
        &self,
        operation: impl std::future::Future<
            Output = Result<Output, geam_core::host::HostExecutionError>,
        >,
        unit: ExecutionUnit,
    ) -> Result<Output, geam_core::host::HostExecutionError> {
        let mut operation = std::pin::pin!(operation);
        std::future::poll_fn(|cx| {
            assert!(operation.as_mut().poll(cx).is_pending());
            self.events.send(PollEvent::Paused(unit.clone())).unwrap();
            self.resume
                .lock()
                .unwrap()
                .recv_timeout(std::time::Duration::from_secs(10))
                .unwrap();
            let completed = operation.as_mut().poll(cx);
            assert!(completed.is_ready());
            completed.map(|result| {
                assert_eq!(
                    result.as_ref().err().map(ToString::to_string),
                    Some("the native operation was cancelled".into())
                );
                result
            })
        })
        .await
    }
}

impl PollDriver {
    pub(crate) fn cancel(
        self,
        host: &TestHost,
        mut execution: std::pin::Pin<&mut impl std::future::Future>,
    ) {
        std::thread::scope(|threads| {
            let (turn, turns) = std::sync::mpsc::channel();
            let worker = threads.spawn(move || {
                for () in turns {
                    host.step();
                    self.events.send(PollEvent::TurnFinished).unwrap();
                }
            });
            let mut cx = std::task::Context::from_waker(std::task::Waker::noop());
            let mut startup_turns = 0;
            let unit = loop {
                startup_turns += 1;
                assert!(
                    startup_turns <= 64,
                    "native poll did not reach its checkpoint"
                );
                assert!(execution.as_mut().poll(&mut cx).is_pending());
                turn.send(()).unwrap();
                match self
                    .received
                    .recv_timeout(std::time::Duration::from_secs(10))
                    .unwrap()
                {
                    PollEvent::TurnFinished => {}
                    PollEvent::Paused(unit) => break unit,
                }
            };
            assert!(unit.cancel());
            // The worker remains inside its poll while the driver closes effect admission.
            assert!(execution.as_mut().poll(&mut cx).is_pending());
            self.resume.send(()).unwrap();
            assert_eq!(
                std::mem::discriminant(
                    &self
                        .received
                        .recv_timeout(std::time::Duration::from_secs(10))
                        .unwrap()
                ),
                std::mem::discriminant(&PollEvent::TurnFinished)
            );
            drop(turn);
            worker.join().unwrap();
        });
    }
}

struct Capture;

impl HostProfile for Capture {
    type RunState = futures_channel::mpsc::UnboundedSender<ExecutionUnit>;
    type ExternalStores = ();
    type ExecutionState = ();
}

impl HostProvider<Capture> for Capture {
    type State = futures_channel::mpsc::UnboundedSender<ExecutionUnit>;

    fn project(state: &mut Self::State) -> &mut Self::State {
        state
    }
}

fn hold<'call>(
    mut call: HostCall<'call, Capture, Capture, ()>,
    constructions: HostConstructions<'call, HostTypeListEnd>,
) -> Result<HostCallContinuation<'call, ()>, HostCallError> {
    let unit = call.execution_unit().unwrap();
    call.state().unbounded_send(unit).unwrap();
    Ok(call.resume(constructions, |_| Box::pin(std::future::pending())))
}

/// Supplies real, pending source executions; no fabricated identity or lifecycle.
pub(crate) fn with_units(count: usize, test: impl FnOnce(&[ExecutionUnit])) {
    let provider = HostProviderModule::new("application", "main")
        .unwrap()
        .with_resumable_function::<Capture, (), (), HostTypeListEnd, _>("hold", hold)
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
                "@external(erlang, \"host\", \"hold\") fn hold() -> Nil\n\
                 pub fn wait() { hold() }",
            )],
        )],
        HostProviderSet::from_providers([provider]).unwrap(),
    )
    .unwrap();
    let (builder, wait) = HostedModuleBuilder::<Capture>::new(typed)
        .unwrap()
        .function(FunctionDeclaration::<(), ()>::new("wait"))
        .unwrap();
    let mut module = builder.seal().unwrap();
    let host = TestHost::default();
    let (mut state, mut units) = futures_channel::mpsc::unbounded();
    let mut echo = Vec::new();
    let mut execution =
        Box::pin(
            module.with_execution(&host, &mut state, &mut echo, async |scope| {
                futures_util::future::join_all((0..count).map(|_| scope.call(&wait, ()))).await
            }),
        );
    assert!(host.poll(execution.as_mut()).is_pending());
    let units = (0..count)
        .map(|_| units.try_recv().unwrap())
        .collect::<Vec<_>>();
    assert!(units.iter().all(ExecutionUnit::is_active));
    test(&units);
    for unit in &units {
        unit.cancel();
    }
    let results = host.block_on(execution.as_mut()).unwrap();
    assert_eq!(results.len(), count);
    assert!(
        results
            .into_iter()
            .all(|result| matches!(result, Err(geam_core::embedding::CallError::Cancelled)))
    );
    drop(execution);
    assert!(units.iter().all(|unit| !unit.is_active()));
    assert!(echo.is_empty());
}

mod contexts {
    use ecow::EcoString;
    use geam_core::host::{
        HostCall, HostCallCompletion, HostCallError, HostExternal, HostExternalBinding,
        HostExternalEquality, HostExternalHashing, HostExternalInspection, HostExternalSchema,
        HostExternalStorage, HostExternalStore, HostExternalType, HostProfile, HostProvider,
        HostProviderModule, HostProviderSet,
    };
    use geam_core::{
        HostedExecution, ModuleSource, PackageSource, compile_typed_host_program, plan_host_program,
    };

    pub(super) struct Assertions {
        pub(super) equal: Box<dyn Fn(&HostExternalEquality<'_>) + Send>,
        pub(super) hash: Box<dyn Fn(&HostExternalHashing<'_>) + Send>,
        pub(super) inspect: Box<dyn Fn(&HostExternalInspection<'_>) + Send>,
    }

    struct Profile;
    struct Probe;
    type Value = HostExternalType<Probe>;

    impl HostProfile for Profile {
        type RunState = Option<Assertions>;
        type ExternalStores = HostExternalStore<Assertions>;
        type ExecutionState = ();
    }

    impl HostProvider<Profile> for Probe {
        type State = Option<Assertions>;
        fn project(state: &mut Self::State) -> &mut Self::State {
            state
        }
    }

    impl HostExternalSchema for Probe {
        const PACKAGE: &'static str = "application";
        const MODULE: &'static str = "main";
        const NAME: &'static str = "Probe";
        const PARAMETER_COUNT: usize = 0;
    }

    impl HostExternalBinding<Profile, Probe> for Probe {
        type Storage = Probe;
    }

    impl HostExternalStorage<Profile, Probe> for Probe {
        type Payload = Assertions;
        fn store(stores: &HostExternalStore<Assertions>) -> &HostExternalStore<Assertions> {
            stores
        }
        fn source_equal(
            context: &HostExternalEquality<'_>,
            left: &Assertions,
            _: &Assertions,
        ) -> bool {
            (left.equal)(context);
            true
        }
        fn source_hash(context: &HostExternalHashing<'_>, value: &Assertions) -> u64 {
            (value.hash)(context);
            42
        }
        fn inspect(context: &HostExternalInspection<'_>, value: &Assertions) -> EcoString {
            (value.inspect)(context);
            "Probe".into()
        }
    }

    fn make<'call>(
        mut call: HostCall<'call, Profile, Probe, Value>,
    ) -> Result<HostCallCompletion<'call, Value>, HostCallError> {
        let assertions = call.state().take().unwrap();
        let value = call.create_external(assertions);
        Ok(call.return_value(value))
    }

    fn check<'call>(
        call: HostCall<'call, Profile, Probe, ()>,
        value: HostExternal<'call, Value>,
    ) -> Result<HostCallCompletion<'call, ()>, HostCallError> {
        assert!(call.equal::<Value>(value, value));
        let _ = call.source_hash::<Value>(value);
        assert_eq!(call.inspect::<Value>(value), "Probe");
        Ok(call.return_value(()))
    }

    pub(super) fn run(assertions: Assertions) {
        let provider = HostProviderModule::<Profile>::new("application", "main")
            .unwrap()
            .with_external_type::<Probe, Probe>()
            .unwrap()
            .with_scoped_function::<Probe, (), Value, _>("make", make)
            .unwrap()
            .with_scoped_function::<Probe, (Value,), (), _>("check", check)
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
pub type Probe
@external(erlang, "host", "make") fn make() -> Probe
@external(erlang, "host", "check") fn check(value: Probe) -> Nil
pub fn main() { check(make()) }
"#,
                )],
            )],
            HostProviderSet::from_providers([provider]).unwrap(),
        )
        .unwrap();
        let plan = plan_host_program(typed).unwrap();
        let mut execution = HostedExecution::try_from_module_plan(plan).unwrap();
        let host = crate::execution_fixture::TestHost::default();
        let mut state = Some(assertions);
        let mut echo = Vec::new();
        assert_eq!(
            host.block_on(execution.run_main(&host, &mut state, &mut echo))
                .unwrap(),
            geam_core::Value::Nil
        );
        assert!(state.is_none());
        assert!(echo.is_empty());
    }
}

/// Obtains the ordinary operation contexts through real external storage calls.
pub(crate) fn with_contexts(
    equal: impl Fn(&geam_core::host::HostExternalEquality<'_>) + Send + 'static,
    hash: impl Fn(&geam_core::host::HostExternalHashing<'_>) + Send + 'static,
    inspect: impl Fn(&geam_core::host::HostExternalInspection<'_>) + Send + 'static,
) {
    contexts::run(contexts::Assertions {
        equal: Box::new(equal),
        hash: Box::new(hash),
        inspect: Box::new(inspect),
    });
}
