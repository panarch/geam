//! Original pinned stdlib source consumes the public native view rule.
use camino::Utf8Path;
use geam::embedding::{FunctionDeclaration, HostedModuleBuilder};
use geam::execution::TokioHost;
use geam::gleam_stdlib::{
    Component as StdlibComponent, GleamStdlibHostProfile, GleamStdlibRunState, GleamStdlibStores,
    IoOutput,
};
use geam::{
    HostComponentProfile, HostProfile, HostProvider, HostProviderComponentRegistration,
    HostProviderSet, compile_typed_host_project,
};
use geam_native_function_views_fixture::Component;
use std::{fs, process::Command};

struct Profile;
#[derive(Default)]
struct Stores {
    stdlib: GleamStdlibStores,
    provider: (),
}
struct State {
    stdlib: GleamStdlibRunState,
    provider: (),
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
    fn component_stores(stores: &Stores) -> &() {
        &stores.provider
    }
    fn component_state(state: &mut State) -> &mut () {
        &mut state.provider
    }
}

#[test]
fn original_dynamic_source_calls_views_and_remains_callable_after_conversion_failures() {
    let project = Utf8Path::new(env!("CARGO_MANIFEST_DIR")).join("../project");
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
    let mut providers = geam::gleam_stdlib::host_providers::<Profile>().unwrap();
    providers
        .extend(<Component as HostProviderComponentRegistration<Profile>>::providers().unwrap());
    let typed = compile_typed_host_project(
        &project,
        "native_function_views_fixture",
        HostProviderSet::from_providers(providers).unwrap(),
    )
    .unwrap();
    let (mut bindings, main) = HostedModuleBuilder::new(typed)
        .unwrap()
        .function(FunctionDeclaration::<(), ()>::new("main"))
        .unwrap();
    let invalid_input = bindings
        .function(FunctionDeclaration::<(), bool>::new("invalid_input"))
        .unwrap();
    let invalid_result = bindings
        .function(FunctionDeclaration::<(), geam::embedding::BigInt>::new(
            "invalid_result",
        ))
        .unwrap();
    let invalid_arity = bindings
        .function(FunctionDeclaration::<(), geam::embedding::BigInt>::new(
            "invalid_arity",
        ))
        .unwrap();
    let mut module = bindings.seal().unwrap();
    let executor = tokio::runtime::Builder::new_current_thread()
        .build()
        .unwrap();
    let host = TokioHost::new(executor.handle().clone());
    let mut state = State {
        stdlib: GleamStdlibRunState::from_seed([0; 32]),
        provider: (),
    };
    let expected = &raw mut state.provider;
    assert!(std::ptr::eq(
        <Component as HostProvider<Profile>>::project(&mut state),
        expected
    ));
    let stores = Stores::default();
    assert!(std::ptr::eq(
        <Profile as HostComponentProfile<Component>>::component_stores(&stores),
        &stores.provider
    ));
    let mut echo = Vec::new();
    for _ in 0..2 {
        executor
            .block_on(
                module.with_execution(&host, &mut state, &mut echo, async |scope| {
                    assert!(
                        scope
                            .call(&invalid_input, ())
                            .await
                            .unwrap_err()
                            .to_string()
                            .contains("input does not match its source signature")
                    );
                    assert!(
                        scope
                            .call(&invalid_result, ())
                            .await
                            .unwrap_err()
                            .to_string()
                            .contains("result does not match its target signature")
                    );
                    assert!(
                        scope
                            .call(&invalid_arity, ())
                            .await
                            .unwrap_err()
                            .to_string()
                            .contains("checked identity cannot convert target")
                    );
                    scope.call(&main, ()).await.unwrap();
                }),
            )
            .unwrap()
            .try_into_value()
            .unwrap();
    }
    assert!(echo.is_empty());
    let outputs = state.stdlib.take_io_outputs();
    assert_eq!(outputs.len(), 2);
    for output in outputs {
        assert_eq!(output.text(), "native function views: 42\n");
    }
}
