use camino::Utf8Path;
use geam::builtin::FutureComponent;
use geam::embedding::{BigInt, FunctionDeclaration, FutureType, HostedModule, HostedModuleBuilder};
use geam::gleam_json::{Component as JsonComponent, GleamJsonStores};
use geam::gleam_stdlib::{
    Component as StdlibComponent, GleamStdlibHostProfile, GleamStdlibRunState, GleamStdlibStores,
    IoOutput, IoStream,
};
use geam::gleam_time::{Component as TimeComponent, GleamTimeHostProfile, TimeSource};
use geam::host::{
    HostComponentProfile, HostFutureStore, HostProviderComponentRegistration, HostProviderSet,
};
use geam::{EchoOutput, EchoSink, HostFailure, HostProfile};
use std::cell::Cell;
use std::task::Poll;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

#[path = "support/execution_host.rs"]
mod execution_fixture;
#[path = "support/workspace_dependencies.rs"]
mod workspace_dependencies;

#[derive(Default)]
pub struct NativeState {
    gate: Option<futures_channel::oneshot::Receiver<()>>,
    starts: Cell<usize>,
}

#[geam::provider(package = "geam_future_builtins_test", state = NativeState, modules = [native])]
pub struct Component;

#[geam::module(path = "future_builtins/native")]
mod native {
    use super::NativeState;
    use geam::provider::{BigInt, Call, Callback};

    #[geam::function]
    async fn apply(
        #[geam::call] call: &mut Call<NativeState>,
        callback: Callback<fn(BigInt) -> BigInt>,
    ) -> geam::provider::HostResult<BigInt> {
        let gate = call
            .with_state(|state| {
                state.starts.set(state.starts.get() + 1);
                state.gate.take().expect("one native construction")
            })
            .await?;
        gate.await.expect("caller releases the gate");
        let first = call.invoke(&callback, (20.into(),)).await?;
        let second = call.invoke(&callback, (21.into(),)).await?;
        Ok(first + second)
    }
}

struct Profile;
struct State {
    stdlib: GleamStdlibRunState,
    clock: Clock,
    native: NativeState,
    work: (),
    json: (),
}
#[derive(Default)]
struct HostStores {
    stdlib: GleamStdlibStores,
    json: GleamJsonStores,
    native: Stores,
    work: HostFutureStore,
    time: (),
}
struct Clock(Cell<u64>);
impl TimeSource for Clock {
    fn system_time(&mut self) -> Result<SystemTime, HostFailure> {
        let seconds = self.0.get();
        self.0.set(seconds + 1);
        Ok(UNIX_EPOCH + Duration::from_secs(seconds))
    }
    fn local_offset_seconds(&mut self) -> Result<i32, HostFailure> {
        Ok(0)
    }
}
impl HostProfile for Profile {
    type RunState = State;
    type ExternalStores = HostStores;
}
impl GleamStdlibHostProfile for Profile {
    type Io = Vec<IoOutput>;
}
impl GleamTimeHostProfile for Profile {
    type Source = Clock;
}
impl HostComponentProfile<StdlibComponent> for Profile {
    fn component_stores(stores: &HostStores) -> &GleamStdlibStores {
        &stores.stdlib
    }
    fn component_state(state: &mut State) -> &mut GleamStdlibRunState {
        &mut state.stdlib
    }
}
impl HostComponentProfile<JsonComponent> for Profile {
    fn component_stores(stores: &HostStores) -> &GleamJsonStores {
        &stores.json
    }
    fn component_state(state: &mut State) -> &mut () {
        &mut state.json
    }
}
impl HostComponentProfile<TimeComponent<Clock>> for Profile {
    fn component_stores(stores: &HostStores) -> &() {
        &stores.time
    }
    fn component_state(state: &mut State) -> &mut Clock {
        &mut state.clock
    }
}
impl HostComponentProfile<Component> for Profile {
    fn component_stores(stores: &HostStores) -> &Stores {
        &stores.native
    }
    fn component_state(state: &mut State) -> &mut NativeState {
        &mut state.native
    }
}
impl geam_core::host::HostWorkProfile for Profile {
    type Work = FutureComponent;
}
impl HostComponentProfile<FutureComponent> for Profile {
    fn component_stores(stores: &HostStores) -> &HostFutureStore {
        &stores.work
    }
    fn component_state(state: &mut State) -> &mut () {
        &mut state.work
    }
}

#[derive(Default)]
struct Echo(Vec<String>);
impl EchoSink for Echo {
    fn emit(&mut self, output: EchoOutput) {
        self.0.push(output.value().inspect().to_string());
    }
}

struct Fixture {
    module: HostedModule<Profile>,
    work: geam::embedding::Function<(), FutureType<BigInt>>,
    later: geam::embedding::Function<(), BigInt>,
}

fn fixture() -> Fixture {
    let root =
        Utf8Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/projects/future_builtins");
    static PREPARED: std::sync::OnceLock<Result<(), String>> = std::sync::OnceLock::new();
    workspace_dependencies::prepare(
        &PREPARED,
        root.as_std_path(),
        "gleam",
        &["deps", "download"],
        "`gleam deps download`",
    );
    let mut providers = geam::gleam_stdlib::host_providers::<Profile>().expect("stdlib");
    providers.extend(geam::gleam_json::host_providers::<Profile>().expect("JSON"));
    providers.extend(geam::gleam_time::host_providers::<Profile>().expect("Time"));
    providers.extend(FutureComponent::providers::<Profile>().expect("Future"));
    providers.extend(
        <Component as HostProviderComponentRegistration<Profile>>::providers().expect("native"),
    );
    let program = geam::frontend::compile_typed_host_project(
        root,
        "future_builtins",
        HostProviderSet::from_providers(providers).expect("provider set"),
    )
    .expect("official source with explicit Future package");
    let (mut bindings, work) = HostedModuleBuilder::new(program)
        .expect("plan")
        .function(FunctionDeclaration::<(), FutureType<BigInt>>::new("work"))
        .expect("work entry");
    let later = bindings
        .function(FunctionDeclaration::<(), BigInt>::new("later"))
        .expect("direct entry");
    Fixture {
        module: bindings.seal().expect("seal"),
        work,
        later,
    }
}

#[test]
fn builtins_and_retained_values_survive_pending_and_repeated_native_callbacks() {
    let execution_host = crate::execution_fixture::TestHost::default();

    let Fixture {
        mut module,
        work,
        later,
    } = fixture();
    let (send, receive) = futures_channel::oneshot::channel();
    let mut state = State {
        stdlib: GleamStdlibRunState::from_seed([0; 32]),
        clock: Clock(Cell::new(100)),
        native: NativeState {
            gate: Some(receive),
            starts: Cell::new(0),
        },
        work: (),
        json: (),
    };
    let mut echo = Echo::default();
    let mut task =
        Box::pin(
            module.with_execution(&execution_host, &mut state, &mut echo, async |scope| {
                let work = scope.call(&work, ()).await.expect("construction");
                let result = scope.observe(&work).await.expect("completion");
                let shared = scope.observe(&work).await.expect("shared completion");
                result.read(|left| shared.read(|right| assert!(std::ptr::eq(left, right))));
                assert_eq!(
                    scope
                        .call(&later, ())
                        .await
                        .expect("direct call after work"),
                    BigInt::from(103)
                );
                result
            }),
        );
    assert!(execution_host.poll(task.as_mut()).is_pending());
    send.send(()).expect("release native work");
    let result = std::thread::scope(|threads| {
        threads
            .spawn(|| {
                let Poll::Ready(result) = execution_host.poll(task.as_mut()) else {
                    panic!("released work completes")
                };
                result
            })
            .join()
            .expect("different worker")
    });
    drop(task);
    assert_eq!(
        result.expect("controlled execution").read(Clone::clone),
        BigInt::from(82)
    );
    assert_eq!(state.native.starts.get(), 1);
    assert_eq!(state.clock.0.get(), 104);
    assert_eq!(echo.0, ["20", "21"]);
    assert_eq!(
        state
            .stdlib
            .io_outputs()
            .iter()
            .map(|output| (output.stream(), output.text().as_str()))
            .collect::<Vec<_>>(),
        [
            (IoStream::Stdout, "created\n"),
            (IoStream::Stdout, "callback\n"),
            (IoStream::Stdout, "callback\n"),
            (IoStream::Stdout, "later\n")
        ]
    );
}

#[test]
fn dropping_the_execution_cancels_pending_callbacks_without_replacing_builtin_state() {
    let execution_host = crate::execution_fixture::TestHost::default();

    let Fixture {
        mut module,
        work,
        later,
    } = fixture();
    let (send, receive) = futures_channel::oneshot::channel();
    let mut state = State {
        stdlib: GleamStdlibRunState::from_seed([0; 32]),
        clock: Clock(Cell::new(100)),
        native: NativeState {
            gate: Some(receive),
            starts: Cell::new(0),
        },
        work: (),
        json: (),
    };
    let mut echo = Echo::default();
    let mut task =
        Box::pin(
            module.with_execution(&execution_host, &mut state, &mut echo, async |scope| {
                let work = scope.call(&work, ()).await.expect("construction");
                scope.observe(&work).await
            }),
        );
    assert!(execution_host.poll(task.as_mut()).is_pending());
    drop(task);
    assert_eq!(
        send.send(()),
        Err(()),
        "the abandoned work released its receiver"
    );
    assert_eq!(state.native.starts.get(), 1);
    assert_eq!(state.clock.0.get(), 101);
    assert!(echo.0.is_empty());
    {
        let mut task = Box::pin(module.with_execution(
            &execution_host,
            &mut state,
            &mut echo,
            async |scope| {
                scope
                    .call(&later, ())
                    .await
                    .expect("direct call after cancellation")
            },
        ));
        assert_eq!(
            execution_host
                .poll(task.as_mut())
                .map(|result| result.expect("controlled execution")),
            Poll::Ready(BigInt::from(101))
        );
    }
    assert_eq!(state.clock.0.get(), 102);
    assert_eq!(
        state
            .stdlib
            .io_outputs()
            .iter()
            .map(IoOutput::text)
            .collect::<Vec<_>>(),
        ["created\n", "later\n"]
    );
}
