use super::transfer_fixture::{ENTRY, TransferFixture, observed_project};
use geam_builtin::FutureComponent;
use geam_core::host::{HostComponentProfile, HostFutureStore};
use geam_core::{HostProfile, HostProviderSet, compile_typed_host_project};
use geam_json::{Component as JsonComponent, GleamJsonStores};
use geam_stdlib::{
    Component as StdlibComponent, GleamStdlibHostProfile, GleamStdlibRunState, GleamStdlibStores,
    IoOutput,
};

pub(super) struct Profile;

pub(super) struct RunState {
    pub(super) stdlib: GleamStdlibRunState,
    pub(super) json: (),
    pub(super) work: (),
}

#[derive(Default)]
pub(super) struct Stores {
    stdlib: GleamStdlibStores,
    json: GleamJsonStores,
    work: HostFutureStore,
}

impl HostProfile for Profile {
    type RunState = RunState;
    type ExternalStores = Stores;
}

impl GleamStdlibHostProfile for Profile {
    type Io = Vec<IoOutput>;
}

impl HostComponentProfile<StdlibComponent> for Profile {
    fn component_stores(stores: &Stores) -> &GleamStdlibStores {
        &stores.stdlib
    }
    fn component_state(state: &mut RunState) -> &mut GleamStdlibRunState {
        &mut state.stdlib
    }
}

impl HostComponentProfile<JsonComponent> for Profile {
    fn component_stores(stores: &Stores) -> &GleamJsonStores {
        &stores.json
    }
    fn component_state(state: &mut RunState) -> &mut () {
        &mut state.json
    }
}

impl geam_core::host::HostWorkProfile for Profile {
    type Work = FutureComponent;
}
impl HostComponentProfile<FutureComponent> for Profile {
    fn component_stores(stores: &Stores) -> &HostFutureStore {
        &stores.work
    }
    fn component_state(state: &mut RunState) -> &mut () {
        &mut state.work
    }
}

pub(super) fn fixture(root_module: &str) -> TransferFixture<Profile> {
    let mut providers =
        geam_stdlib::host_providers::<Profile>().expect("stdlib transfer registration");
    providers.extend(geam_json::host_providers::<Profile>().expect("JSON transfer registration"));
    TransferFixture::new(
        observed_project(
            &super::project_root(),
            root_module,
            HostProviderSet::from_providers(providers).expect("JSON provider set"),
        ),
        ENTRY,
    )
}

#[test]
fn non_finite_json_preserves_the_host_failure_and_allows_the_next_call() {
    let execution_host = crate::execution_fixture::TestHost::default();

    use ecow::EcoString;
    use geam_core::ExecutionError;
    use geam_core::embedding::{CallError, FunctionDeclaration, HostedModuleBuilder};
    use std::pin::pin;
    use std::task::Poll;

    let mut providers =
        geam_stdlib::host_providers::<Profile>().expect("stdlib transfer registration");
    providers.extend(geam_json::host_providers::<Profile>().expect("JSON transfer registration"));
    let program = compile_typed_host_project(
        super::project_root(),
        "gleam_json_encode",
        HostProviderSet::from_providers(providers).expect("JSON provider set"),
    )
    .expect("official JSON source linkage");
    let (bindings, entry) = HostedModuleBuilder::new(program)
        .expect("JSON plan")
        .function(FunctionDeclaration::<(f64,), EcoString>::new(
            "encode_number",
        ))
        .expect("typed probe");
    let mut module = bindings.seal().expect("JSON seal");
    let mut state = RunState {
        stdlib: GleamStdlibRunState::from_seed([0; 32]),
        json: (),
        work: (),
    };
    let mut echo = super::transfer_fixture::ObservedEcho::default();
    let mut task =
        pin!(
            module.with_execution(&execution_host, &mut state, &mut echo, async |scope| {
                for value in [f64::INFINITY, f64::NEG_INFINITY, f64::NAN] {
                    let error = scope
                        .call(&entry, (value,))
                        .await
                        .expect_err("non-finite JSON number");
                    assert!(
                        matches!(error, CallError::Execution(ExecutionError::Host(ref error))
                if error.package() == "gleam_json"
                    && error.module() == "gleam/json"
                    && error.function() == "do_float"
                    && error.failure().message() == "JSON cannot encode a non-finite Float")
                    );
                }
                assert_eq!(
                    scope
                        .call(&entry, (1.5,))
                        .await
                        .expect("later finite number"),
                    "1.5"
                );
            })
        );
    assert!(matches!(
        execution_host.poll(task.as_mut()),
        Poll::Ready(Ok(()))
    ));
}
