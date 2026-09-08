use super::transfer_fixture::{ENTRY, TransferFixture, observed_project};
use geam_core::host::{AsyncHostComponentProfile, HostFutureStore};
use geam_core::{HostProfile, TransferHostProviderSet, compile_typed_transfer_host_project};
use geam_json::{Component as JsonComponent, GleamJsonTransferStores};
use geam_runtime_api::FutureComponent;
use geam_stdlib::{
    Component as StdlibComponent, GleamStdlibHostProfile, GleamStdlibRunState,
    GleamStdlibTransferStores, IoOutput,
};

pub(super) struct Profile;

pub(super) struct RunState {
    pub(super) stdlib: GleamStdlibRunState,
    pub(super) json: (),
    pub(super) work: (),
}

#[derive(Default)]
pub(super) struct Stores {
    stdlib: GleamStdlibTransferStores,
    json: GleamJsonTransferStores,
    work: HostFutureStore,
}

impl HostProfile for Profile {
    type RunState = RunState;
    type ExternalStores = Stores;
}

impl GleamStdlibHostProfile for Profile {
    type Io = Vec<IoOutput>;
}

impl AsyncHostComponentProfile<StdlibComponent> for Profile {
    fn component_async_stores(stores: &Stores) -> &GleamStdlibTransferStores {
        &stores.stdlib
    }
    fn component_state(state: &mut RunState) -> &mut GleamStdlibRunState {
        &mut state.stdlib
    }
}

impl AsyncHostComponentProfile<JsonComponent> for Profile {
    fn component_async_stores(stores: &Stores) -> &GleamJsonTransferStores {
        &stores.json
    }
    fn component_state(state: &mut RunState) -> &mut () {
        &mut state.json
    }
}

impl geam_core::host::HostWorkProfile for Profile {
    type Work = FutureComponent;
}
impl AsyncHostComponentProfile<FutureComponent> for Profile {
    fn component_async_stores(stores: &Stores) -> &HostFutureStore {
        &stores.work
    }
    fn component_state(state: &mut RunState) -> &mut () {
        &mut state.work
    }
}

pub(super) fn fixture(root_module: &str) -> TransferFixture<Profile> {
    let mut providers =
        geam_stdlib::transfer_host_providers::<Profile>().expect("stdlib transfer registration");
    providers.extend(
        geam_json::transfer_host_providers::<Profile>().expect("JSON transfer registration"),
    );
    TransferFixture::new(
        observed_project(
            &super::project_root(),
            root_module,
            TransferHostProviderSet::new(providers).expect("JSON provider set"),
        ),
        ENTRY,
    )
}

#[test]
fn non_finite_json_preserves_the_host_failure_and_allows_the_next_call() {
    use ecow::EcoString;
    use geam_core::AsyncExecutionError;
    use geam_core::embedding::{
        AsyncCallError, FunctionDeclaration, WorkModuleBuilder, with_execution_scope,
    };
    use std::future::Future;
    use std::pin::pin;
    use std::task::{Context, Poll, Waker};

    let mut providers =
        geam_stdlib::transfer_host_providers::<Profile>().expect("stdlib transfer registration");
    providers.extend(
        geam_json::transfer_host_providers::<Profile>().expect("JSON transfer registration"),
    );
    let program = compile_typed_transfer_host_project(
        super::project_root(),
        "gleam_json_encode",
        TransferHostProviderSet::new(providers).expect("JSON provider set"),
    )
    .expect("official JSON source linkage");
    let (bindings, entry) = WorkModuleBuilder::new(program)
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
    let mut task = pin!(with_execution_scope(async |guard| {
        let mut scope = module.attach(guard, &mut state, &mut echo);
        for value in [f64::INFINITY, f64::NEG_INFINITY, f64::NAN] {
            let error = scope
                .call(&entry, (value,))
                .expect_err("non-finite JSON number");
            assert!(
                matches!(error, AsyncCallError::Execution(AsyncExecutionError::Host(ref error))
                if error.package() == "gleam_json"
                    && error.module() == "gleam/json"
                    && error.function() == "do_float"
                    && error.failure().message() == "JSON cannot encode a non-finite Float")
            );
        }
        assert_eq!(
            scope.call(&entry, (1.5,)).expect("later finite number"),
            "1.5"
        );
    }));
    assert!(matches!(
        task.as_mut().poll(&mut Context::from_waker(Waker::noop())),
        Poll::Ready(())
    ));
}
