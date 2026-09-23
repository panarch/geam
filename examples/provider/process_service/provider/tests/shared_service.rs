extern crate geam as geam_core;

#[path = "../../../../../tests/support/execution_host.rs"]
mod execution_fixture;

use geam::execution::ExecutionServices;
use geam::gleam_erlang::{Configuration, ErlangExecution, GleamErlangHostProfile};
use geam::gleam_stdlib::{GleamStdlibRunState, GleamStdlibStores, IoOutput, IoStream};
use geam::{
    HostComponentProfile, HostExecutionService, HostProfile, HostProviderComponent,
    HostProviderComponentRegistration, HostProviderSet, HostServiceProfile, HostedExecution, Value,
    compile_typed_host_project, plan_host_program,
};
use geam_example_process_service::Component;

struct Profile;

#[derive(Default)]
struct Stores {
    stdlib: GleamStdlibStores,
    erlang: geam::gleam_erlang::Stores<Profile>,
    provider: <Component as HostProviderComponent>::Stores,
}

struct State {
    stdlib: GleamStdlibRunState,
    erlang: Configuration,
    provider: (),
}

impl HostProfile for Profile {
    type RunState = State;
    type ExternalStores = Stores;
    type ExecutionState = ExecutionServices<ErlangExecution, ()>;

    fn initialize_execution(state: &mut State) -> Self::ExecutionState {
        ExecutionServices {
            first:
                <geam::gleam_erlang::Component<Profile> as HostExecutionService>::initialize_service(
                    &mut state.erlang,
                ),
            rest: (),
        }
    }
}

impl geam::gleam_stdlib::GleamStdlibHostProfile for Profile {
    type Io = Vec<IoOutput>;
}

impl HostComponentProfile<geam::gleam_stdlib::Component> for Profile {
    fn component_stores(stores: &Stores) -> &GleamStdlibStores {
        &stores.stdlib
    }
    fn component_state(state: &mut State) -> &mut GleamStdlibRunState {
        &mut state.stdlib
    }
}

impl HostComponentProfile<geam::gleam_erlang::Component<Profile>> for Profile {
    fn component_stores(stores: &Stores) -> &geam::gleam_erlang::Stores<Profile> {
        &stores.erlang
    }
    fn component_state(state: &mut State) -> &mut Configuration {
        &mut state.erlang
    }
}

impl HostComponentProfile<Component> for Profile {
    fn component_stores(stores: &Stores) -> &<Component as HostProviderComponent>::Stores {
        &stores.provider
    }
    fn component_state(state: &mut State) -> &mut () {
        &mut state.provider
    }
}

impl HostServiceProfile<geam::gleam_erlang::Component<Profile>> for Profile {
    fn service(state: &mut Self::ExecutionState) -> &mut ErlangExecution {
        &mut state.first
    }
}

impl GleamErlangHostProfile for Profile {
    fn erlang_execution(state: &mut Self::ExecutionState) -> &mut ErlangExecution {
        <Self as HostServiceProfile<geam::gleam_erlang::Component<Profile>>>::service(state)
    }
}

fn execution(entry: &str) -> (HostedExecution<Profile>, State) {
    let root = camino::Utf8Path::new(env!("CARGO_MANIFEST_DIR")).join("../project");
    let acquisition = std::process::Command::new("gleam")
        .args(["deps", "download"])
        .current_dir(&root)
        .output()
        .unwrap();
    assert!(
        acquisition.status.success(),
        "{}",
        String::from_utf8_lossy(&acquisition.stderr)
    );
    let mut providers = geam::gleam_stdlib::host_providers::<Profile>().unwrap();
    providers.extend(geam::gleam_erlang::host_providers::<Profile>().unwrap());
    providers
        .extend(<Component as HostProviderComponentRegistration<Profile>>::providers().unwrap());
    providers.push(geam::HostProviderModule::new("process_service_example", "process_service_invalid_reply").unwrap()
        .with_scoped_function::<geam::gleam_erlang::Component<Profile>, (geam::host::HostTypeParameter<0>,), geam::gleam_stdlib::provider_support::Dynamic, _>("erase", erase).unwrap());
    let typed = compile_typed_host_project(
        &root,
        entry,
        HostProviderSet::from_providers(providers).unwrap(),
    )
    .unwrap();
    let resources = typed.package_resources().clone();
    let execution =
        HostedExecution::try_from_module_plan(plan_host_program(typed).unwrap()).unwrap();
    let state = State {
        stdlib: GleamStdlibRunState::from_seed([0; 32]),
        erlang: Configuration { resources },
        provider: (),
    };
    (execution, state)
}

fn erase<'call>(
    mut call: geam::host::HostCall<
        'call,
        Profile,
        geam::gleam_erlang::Component<Profile>,
        geam::gleam_stdlib::provider_support::Dynamic,
    >,
    value: geam::host::HostValue<'call, geam::host::HostTypeParameter<0>>,
) -> Result<
    geam::host::HostCallCompletion<'call, geam::gleam_stdlib::provider_support::Dynamic>,
    geam::host::HostCallError,
> {
    let value = call.native_value::<geam::host::HostTypeParameter<0>>(value);
    let value = call.create_external(geam::gleam_stdlib::Dynamic::from_native(value));
    Ok(call.return_value(value))
}

#[test]
fn original_gleam_process_and_macro_provider_share_identity_and_mailbox() {
    let (mut execution, mut state) = execution("process_service_example");
    let host = execution_fixture::TestHost::default();
    for _ in 0..2 {
        state.stdlib = GleamStdlibRunState::from_seed([0; 32]);
        let mut echo = Vec::new();
        let mut driver = Box::pin(execution.run_main(&host, &mut state, &mut echo));
        assert!(host.poll(driver.as_mut()).is_pending());
        host.advance(std::time::Duration::from_millis(5));
        let result = host.poll(driver.as_mut());
        drop(driver);
        assert_eq!(
            result.map(Result::unwrap),
            std::task::Poll::Ready(Value::Nil),
            "{:?}",
            state.stdlib.io_outputs()
        );
        assert!(echo.is_empty());
        assert_eq!(
            state
                .stdlib
                .io_outputs()
                .iter()
                .map(|output| (output.stream(), output.text().to_string()))
                .collect::<Vec<_>>(),
            [
                (
                    IoStream::Stdout,
                    "named service replied: 42, 17\n".to_owned()
                ),
                (
                    IoStream::Stdout,
                    "request timeout and unavailable name handled\n".to_owned()
                ),
                (
                    IoStream::Stdout,
                    "worker stopped and name released\n".to_owned()
                ),
            ]
        );
    }
}

#[test]
fn rejects_a_native_reply_that_violates_the_source_specialization() {
    let (mut execution, mut state) = execution("process_service_invalid_reply");
    let host = execution_fixture::TestHost::default();
    let mut echo = Vec::new();
    let mut run = Box::pin(execution.run_main(&host, &mut state, &mut echo));
    assert_eq!(
        host.poll(run.as_mut()).map(Result::unwrap),
        std::task::Poll::Ready(Value::Nil)
    );
}

#[test]
fn request_propagates_a_deadline_outside_the_hosts_clock_range() {
    use geam::execution::{ExecutionHost, RunError};
    use std::time::Duration;

    let (mut execution, mut state) = execution("process_service_clock_range");
    let host = execution_fixture::TestHost::default();
    for bit in (0..64).rev() {
        let step = Duration::from_secs(1 << bit);
        if host.now().checked_add(step).is_some() {
            host.advance(step);
        }
    }
    let mut echo = Vec::new();
    let mut run = Box::pin(execution.run_main(&host, &mut state, &mut echo));
    let result = host.poll(run.as_mut());
    let failure = match result {
        std::task::Poll::Ready(Err(RunError::Execution(geam::ExecutionError::Host(error)))) => {
            Some((
                error.function().to_string(),
                error.failure().message().to_string(),
            ))
        }
        _ => None,
    };
    assert_eq!(
        failure,
        Some((
            "exchange".into(),
            "timeout exceeds the host clock range".into(),
        ))
    );
}
