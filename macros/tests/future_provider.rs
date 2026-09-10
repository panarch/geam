#[path = "../../tests/support/execution_host.rs"]
mod execution_fixture;

use geam_builtin::FutureComponent;
use geam_builtin::embedding::FutureType;
use geam_core::embedding::{BigInt, FunctionDeclaration, HostedModuleBuilder};
use geam_core::frontend::compile_typed_host_program;
use geam_core::host::{
    HostComponentProfile, HostFutureStore, HostProfile, HostProviderComponentRegistration,
    HostProviderSet,
};
use geam_core::{ModuleSource, PackageSource};
use std::task::Poll;

#[derive(Default)]
pub struct State {
    gate: Option<futures_channel::oneshot::Receiver<BigInt>>,
    starts: std::cell::Cell<usize>,
}

#[geam_macros::provider(package = "future_provider", state = State, modules = [native], crate_path = geam_core)]
pub struct Component;

#[geam_macros::module(path = "future_provider/native", crate_path = geam_core)]
mod native {
    use super::State;
    use geam_core::provider::{BigInt, Call, Callback};

    #[geam_macros::function]
    fn double(value: BigInt) -> BigInt {
        value * 2
    }

    #[geam_macros::function]
    async fn fetch(
        #[geam_macros::call] call: &mut Call<State>,
        value: BigInt,
    ) -> geam_core::provider::HostResult<BigInt> {
        let gate = call
            .with_state(|state| {
                state.starts.set(state.starts.get() + 1);
                state.gate.take().expect("one operation")
            })
            .await?;
        Ok(value + gate.await.expect("host gate"))
    }

    #[geam_macros::function]
    async fn apply(
        #[geam_macros::call] call: &mut Call<State>,
        callback: Callback<fn(BigInt) -> BigInt>,
        value: BigInt,
    ) -> geam_core::provider::HostResult<BigInt> {
        let first = call.invoke(&callback, (value,)).await?;
        call.invoke(&callback, (first,)).await
    }
}

struct Profile;
#[derive(Default)]
struct HostState {
    provider: State,
    future: (),
}
#[derive(Default)]
struct HostStores {
    provider: Stores,
    future: HostFutureStore,
}
impl HostProfile for Profile {
    type RunState = HostState;
    type ExternalStores = HostStores;
    type ExecutionState = ();
}
impl HostComponentProfile<Component> for Profile {
    fn component_stores(stores: &HostStores) -> &Stores {
        &stores.provider
    }
    fn component_state(state: &mut HostState) -> &mut State {
        &mut state.provider
    }
}
impl geam_core::host::HostWorkProfile for Profile {
    type Work = FutureComponent;
}
impl HostComponentProfile<FutureComponent> for Profile {
    fn component_stores(stores: &HostStores) -> &HostFutureStore {
        &stores.future
    }
    fn component_state(state: &mut HostState) -> &mut () {
        &mut state.future
    }
}

#[derive(Default)]
struct Echo(Vec<String>);
impl geam_core::EchoSink for Echo {
    fn emit(&mut self, value: geam_core::EchoOutput) {
        self.0.push(value.to_string());
    }
}

#[test]
fn async_declarations_construct_source_work_and_only_the_rust_owner_drives_it() {
    let execution_host = crate::execution_fixture::TestHost::default();

    let source = r#"
import geam/future
@external(erlang, "native", "double")
pub fn double(value: Int) -> Int
@external(erlang, "native", "fetch")
fn fetch(value: Int) -> future.Future(Int)
@external(erlang, "native", "apply")
fn apply(callback: fn(Int) -> Int, value: Int) -> future.Future(Int)
pub fn work(value: Int) {
  use received <- future.then(fetch(value))
  apply(fn(value) { echo value value + 1 }, received)
}
"#;
    let mut providers = FutureComponent::providers().expect("Future component");
    providers.extend(
        <Component as HostProviderComponentRegistration<Profile>>::providers()
            .expect("macro component"),
    );
    let typed = compile_typed_host_program(
        "future_provider",
        "future_provider/native",
        [
            PackageSource::new(
                "geam",
                Vec::<String>::new(),
                [ModuleSource::new(
                    "geam/future",
                    "src/geam/future.gleam",
                    include_str!("../../builtins/geam/gleam/src/geam/future.gleam"),
                )],
            ),
            PackageSource::new(
                "future_provider",
                ["geam"],
                [ModuleSource::new(
                    "future_provider/native",
                    "src/future_provider/native.gleam",
                    source,
                )],
            ),
        ],
        HostProviderSet::from_providers(providers).expect("provider set"),
    )
    .expect("source linkage");
    let (mut builder, double) = HostedModuleBuilder::new(typed)
        .expect("plan")
        .function(FunctionDeclaration::<(BigInt,), BigInt>::new("double"))
        .expect("direct entry");
    let work = builder
        .function(FunctionDeclaration::<(BigInt,), FutureType<BigInt>>::new(
            "work",
        ))
        .expect("work entry");
    let mut module = builder.seal().expect("one execution");
    let (send, receive) = futures_channel::oneshot::channel();
    let mut state = HostState {
        provider: State {
            gate: Some(receive),
            starts: std::cell::Cell::new(0),
        },
        future: (),
    };
    let mut echo = Echo::default();
    let mut task =
        Box::pin(
            module.with_execution(&execution_host, &mut state, &mut echo, async |scope| {
                assert_eq!(
                    scope
                        .call(&double, (21.into(),))
                        .await
                        .expect("ordinary call"),
                    BigInt::from(42)
                );
                let value = scope
                    .call(&work, (10.into(),))
                    .await
                    .expect("construct work");
                let first = scope.observe(&value).await.expect("first observation");
                let second = scope.observe(&value).await.expect("shared observation");
                first.read(|a| second.read(|b| assert!(std::ptr::eq(a, b))));
                first
            }),
        );
    assert!(execution_host.poll(task.as_mut()).is_pending());
    send.send(10.into()).expect("pending native operation");
    let Poll::Ready(result) = execution_host.poll(task.as_mut()) else {
        panic!("released work completes")
    };
    assert_eq!(
        result.expect("execution completed").read(Clone::clone),
        BigInt::from(22)
    );
    drop(task);
    assert_eq!(state.provider.starts.get(), 1);
    assert_eq!(
        echo.0,
        [
            "src/future_provider/native.gleam:11\n20",
            "src/future_provider/native.gleam:11\n21"
        ]
    );
}
