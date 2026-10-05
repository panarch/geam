//! Original source, ordinary provider authoring, and caller-owned embedding.
extern crate geam as geam_core;

#[path = "../../../../support/execution_host.rs"]
mod execution_fixture;

use camino::Utf8Path;
use futures_channel::oneshot;
use geam::compile_typed_host_project;
use geam::embedding::{Function, FunctionDeclaration, HostedModule, HostedModuleBuilder};
use geam::gleam_stdlib::{
    Component as StdlibComponent, GleamStdlibHostProfile, GleamStdlibRunState, GleamStdlibStores,
    IoOutput,
};
use geam::host::{
    HostComponentProfile, HostProfile, HostProviderComponentRegistration, HostProviderSet,
};
use geam::provider::BitArrayValue;
use geam_bytes_tree_service_fixture::{Component, RunState, Stores as ProviderStores};
use std::sync::{Arc, OnceLock};
use std::{fs, process::Command};

struct Profile;

#[derive(Default)]
struct Stores {
    stdlib: GleamStdlibStores,
    provider: ProviderStores,
}

struct State {
    stdlib: GleamStdlibRunState,
    provider: RunState,
}

impl HostProfile for Profile {
    type RunState = State;
    type ExternalStores = Stores;
    type ExecutionState = ();
}

impl GleamStdlibHostProfile for Profile {
    type Io = Vec<IoOutput>;
}

impl HostComponentProfile<StdlibComponent> for Profile {
    fn component_stores(stores: &Stores) -> &GleamStdlibStores {
        &stores.stdlib
    }

    fn component_state(state: &mut State) -> &mut GleamStdlibRunState {
        &mut state.stdlib
    }
}

impl HostComponentProfile<Component> for Profile {
    fn component_stores(stores: &Stores) -> &ProviderStores {
        &stores.provider
    }

    fn component_state(state: &mut State) -> &mut RunState {
        &mut state.provider
    }
}

struct Fixture {
    module: HostedModule<Profile>,
    verify: Function<(), BitArrayValue>,
    capture: Function<(), ()>,
    capture_generated: Function<(), ()>,
    paused: Function<(), (BitArrayValue, BitArrayValue)>,
}

fn fixture() -> (Fixture, State) {
    let project = Utf8Path::new(env!("CARGO_MANIFEST_DIR")).join("../project");
    static SOURCE: OnceLock<()> = OnceLock::new();
    SOURCE.get_or_init(|| {
        let manifest = fs::read(project.join("manifest.toml")).unwrap();
        let acquisition = Command::new("gleam")
            .args(["deps", "download"])
            .current_dir(&project)
            .output()
            .unwrap();
        assert!(
            acquisition.status.success(),
            "{}",
            String::from_utf8_lossy(&acquisition.stderr)
        );
        assert_eq!(fs::read(project.join("manifest.toml")).unwrap(), manifest);
    });
    let mut providers = geam::gleam_stdlib::host_providers::<Profile>().unwrap();
    providers
        .extend(<Component as HostProviderComponentRegistration<Profile>>::providers().unwrap());
    let typed = compile_typed_host_project(
        &project,
        "bytes_tree_service_fixture",
        HostProviderSet::from_providers(providers).unwrap(),
    )
    .unwrap();
    let (mut bindings, verify) = HostedModuleBuilder::new(typed)
        .unwrap()
        .function(FunctionDeclaration::new("verify"))
        .unwrap();
    let capture = bindings
        .function(FunctionDeclaration::new("capture"))
        .unwrap();
    let capture_generated = bindings
        .function(FunctionDeclaration::new("capture_generated"))
        .unwrap();
    let paused = bindings
        .function(FunctionDeclaration::new("paused"))
        .unwrap();
    (
        Fixture {
            module: bindings.seal().unwrap(),
            verify,
            capture,
            capture_generated,
            paused,
        },
        State {
            stdlib: GleamStdlibRunState::from_seed([0; 32]),
            provider: RunState::default(),
        },
    )
}

#[test]
fn native_generated_tree_remains_readable_after_its_execution_and_host_are_dropped() {
    let (
        Fixture {
            mut module,
            capture_generated,
            ..
        },
        mut state,
    ) = fixture();
    let host = execution_fixture::TestHost::default();
    let mut echo = Vec::new();
    for _ in 0..2 {
        host.block_on(
            module.with_execution(&host, &mut state, &mut echo, async |scope| {
                scope.call(&capture_generated, ()).await
            }),
        )
        .unwrap()
        .try_into_value()
        .unwrap()
        .unwrap();
    }
    drop(module);
    let input = state.provider.retained.take().unwrap();
    drop(state);
    drop(host);
    let bytes = input.to_bit_array();
    drop(input);
    assert_eq!(bytes.bytes(), [0, 255, 128, 42]);
    assert_eq!(bytes.bit_len(), 32);
    assert!(echo.is_empty());
}

const EXPECTED: &[u8] = &[
    0, 255, 128, 237, 149, 156, 0, 101, 204, 129, 240, 159, 153, 130, 1, 2,
];

#[test]
fn original_source_and_retained_input_remain_readable_after_execution_closure() {
    let (
        Fixture {
            mut module,
            verify,
            capture,
            paused,
            ..
        },
        mut state,
    ) = fixture();
    let host = execution_fixture::TestHost::default();
    let mut echo = Vec::new();
    let mut results = Vec::new();
    for _ in 0..2 {
        let bytes = host
            .block_on(
                module.with_execution(&host, &mut state, &mut echo, async |scope| {
                    scope.call(&verify, ()).await
                }),
            )
            .unwrap()
            .try_into_value()
            .unwrap()
            .unwrap();
        results.push(bytes);
        let pair = host
            .block_on(
                module.with_execution(&host, &mut state, &mut echo, async |scope| {
                    scope.call(&paused, ()).await
                }),
            )
            .unwrap()
            .try_into_value()
            .unwrap()
            .unwrap();
        results.extend([pair.0, pair.1]);
    }
    host.block_on(
        module.with_execution(&host, &mut state, &mut echo, async |scope| {
            scope.call(&capture, ()).await
        }),
    )
    .unwrap()
    .try_into_value()
    .unwrap()
    .unwrap();
    drop(module);
    let input = state.provider.retained.take().unwrap();
    drop(state);
    results.push(input.to_bit_array());
    drop(input);
    for bytes in results {
        assert_eq!(bytes.bytes(), EXPECTED);
        assert_eq!(bytes.bit_len(), EXPECTED.len() * 8);
    }
    assert!(echo.is_empty());
}

#[test]
fn pending_native_input_resumes_on_another_worker_and_releases_after_completion() {
    let (
        Fixture {
            mut module, paused, ..
        },
        mut state,
    ) = fixture();
    let host = execution_fixture::TestHost::default();
    let (send, receive) = oneshot::channel();
    state.provider.gate = Some(receive);
    let observed = Arc::clone(&state.provider.pending);
    let mut echo = Vec::new();
    let mut task = Box::pin(
        module.with_execution(&host, &mut state, &mut echo, async |scope| {
            scope.call(&paused, ()).await
        }),
    );
    assert!(host.poll(task.as_mut()).is_pending());
    {
        let input = observed.lock().unwrap().upgrade().unwrap();
        assert_eq!(input.lock().unwrap().to_bit_array().bytes(), EXPECTED);
    }
    send.send(()).unwrap();
    let result = std::thread::scope(|workers| {
        workers
            .spawn(|| host.block_on(task.as_mut()))
            .join()
            .unwrap()
    })
    .unwrap()
    .try_into_value()
    .unwrap()
    .unwrap();
    drop(task);
    assert!(observed.lock().unwrap().upgrade().is_none());
    drop(module);
    drop(state);
    assert_eq!(result.0.bytes(), EXPECTED);
    assert_eq!(result.1.bytes(), EXPECTED);
    assert!(echo.is_empty());
}

#[test]
fn cancelling_pending_native_work_releases_the_input_and_allows_another_call() {
    let (
        Fixture {
            mut module,
            paused,
            verify,
            ..
        },
        mut state,
    ) = fixture();
    let host = execution_fixture::TestHost::default();
    let (send, receive) = oneshot::channel();
    state.provider.gate = Some(receive);
    let observed = Arc::clone(&state.provider.pending);
    let mut echo = Vec::new();
    let mut task = Box::pin(
        module.with_execution(&host, &mut state, &mut echo, async |scope| {
            scope.call(&paused, ()).await
        }),
    );
    assert!(host.poll(task.as_mut()).is_pending());
    assert!(observed.lock().unwrap().upgrade().is_some());
    drop(task);
    host.step();
    assert_eq!(send.send(()), Err(()));
    assert!(observed.lock().unwrap().upgrade().is_none());
    let output = host
        .block_on(
            module.with_execution(&host, &mut state, &mut echo, async |scope| {
                scope.call(&verify, ()).await
            }),
        )
        .unwrap()
        .try_into_value()
        .unwrap()
        .unwrap();
    assert_eq!(output.bytes(), EXPECTED);
    assert!(echo.is_empty());
}
