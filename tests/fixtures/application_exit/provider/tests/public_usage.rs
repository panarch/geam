use geam::embedding::{BigInt, FunctionDeclaration, HostedModuleBuilder};
use geam::execution::{ExecutionOutcome, ExitStatus, TokioHost};
use geam::{HostComponentProfile, HostProfile, HostProviderComponentRegistration, HostProviderSet};
use geam_application_exit_fixture::{Component, State, Stores};

struct Profile;

impl HostProfile for Profile {
    type RunState = State;
    type ExternalStores = Stores;
    type ExecutionState = ();
}

impl HostComponentProfile<Component> for Profile {
    fn component_state(state: &mut State) -> &mut State {
        state
    }
    fn component_stores(stores: &Stores) -> &Stores {
        stores
    }
}

#[test]
fn public_calls_exit_without_killing_the_host_and_new_scopes_preserve_real_state() {
    let project = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../project");
    let typed = geam::compile_typed_host_project(
        project.to_str().unwrap(),
        "application_exit_fixture",
        HostProviderSet::from_providers(
            <Component as HostProviderComponentRegistration<Profile>>::providers().unwrap(),
        )
        .unwrap(),
    )
    .unwrap();
    let (mut bindings, stop) = HostedModuleBuilder::new(typed)
        .unwrap()
        .function(FunctionDeclaration::<(BigInt,), ()>::new("stop"))
        .unwrap();
    let stop_async = bindings
        .function(FunctionDeclaration::<(BigInt,), ()>::new("stop_async"))
        .unwrap();
    let normal = bindings
        .function(FunctionDeclaration::<(), BigInt>::new("normal"))
        .unwrap();
    let ordinary_int = bindings
        .function(FunctionDeclaration::<(), BigInt>::new("ordinary_int"))
        .unwrap();
    let ordinary_error = bindings
        .function(FunctionDeclaration::<(), Result<BigInt, BigInt>>::new(
            "ordinary_error",
        ))
        .unwrap();
    let fail = bindings
        .function(FunctionDeclaration::<(), BigInt>::new("fail"))
        .unwrap();
    let mut module = bindings.seal().unwrap();
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_time()
        .build()
        .unwrap();
    let host = TokioHost::new(runtime.handle().clone());
    let mut state = State::default();
    let mut echo = Vec::new();
    for (function, expected, name) in [
        (&stop, "\"before\"", "exit"),
        (&stop_async, "\"before async\"", "exit_async"),
    ] {
        for code in [0, 7, 255] {
            let outcome = runtime
                .block_on(
                    module.with_execution(&host, &mut state, &mut echo, async |scope| {
                        scope.call(function, (BigInt::from(code),)).await
                    }),
                )
                .unwrap();
            assert_eq!(outcome, ExecutionOutcome::Exited(ExitStatus::new(code)));
            let output = echo.pop().unwrap();
            assert_eq!(output.value().inspect().to_string(), expected);
            assert!(echo.is_empty());
        }
        for code in [-1, 256] {
            let outcome = runtime
                .block_on(
                    module.with_execution(&host, &mut state, &mut echo, async |scope| {
                        scope.call(function, (BigInt::from(code),)).await
                    }),
                )
                .unwrap();
            let error = outcome.try_into_value().unwrap().unwrap_err();
            assert_eq!(
                error.to_string(),
                format!(
                    "host function application_exit_fixture::application_exit_fixture.{name} failed: application exit status must be between 0 and 255"
                )
            );
            assert_eq!(echo.len(), 1);
            echo.clear();
        }
    }
    assert_eq!(state.calls, 6);
    let outcome = runtime
        .block_on(
            module.with_execution(&host, &mut state, &mut echo, async |scope| {
                let normal = scope.call(&normal, ()).await.unwrap();
                let ordinary_int = scope.call(&ordinary_int, ()).await.unwrap();
                let ordinary_error = scope.call(&ordinary_error, ()).await.unwrap();
                let failure = scope.call(&fail, ()).await.unwrap_err();
                assert_eq!(failure.to_string(), "panic: source failure");
                (normal, ordinary_int, ordinary_error)
            }),
        )
        .unwrap();
    assert_eq!(
        outcome,
        ExecutionOutcome::Returned((BigInt::from(6), BigInt::from(7), Err(BigInt::from(7))))
    );
    assert_eq!(state.calls, 6);
    assert_eq!(echo.len(), 1);
    assert_eq!(echo[0].value().inspect().to_string(), "\"normal\"");
}
