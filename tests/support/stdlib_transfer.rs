use super::transfer_fixture::{ENTRY, ObservedEcho, TransferFixture, observed_project};
use geam_core::host::{AsyncHostComponentProfile, HostFutureStore};
use geam_core::{EchoOutput, HostProfile, TransferHostProviderSet, Value};
use geam_runtime_api::FutureComponent;
use geam_stdlib::{
    Component, GleamStdlibHostProfile, GleamStdlibRunState, GleamStdlibTransferStores, IoOutput,
};

pub(super) struct Profile;

pub(super) struct RunState {
    pub(super) stdlib: GleamStdlibRunState,
    pub(super) work: (),
}

#[derive(Default)]
pub(super) struct Stores {
    stdlib: GleamStdlibTransferStores,
    work: HostFutureStore,
}

impl HostProfile for Profile {
    type RunState = RunState;
    type ExternalStores = Stores;
}

impl GleamStdlibHostProfile for Profile {
    type Io = Vec<IoOutput>;
}

impl AsyncHostComponentProfile<Component> for Profile {
    fn component_async_stores(stores: &Stores) -> &GleamStdlibTransferStores {
        &stores.stdlib
    }

    fn component_state(state: &mut RunState) -> &mut GleamStdlibRunState {
        &mut state.stdlib
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
    TransferFixture::new(
        observed_project(
            &super::project_root(),
            root_module,
            TransferHostProviderSet::new(
                geam_stdlib::transfer_host_providers::<Profile>()
                    .expect("stdlib transfer registration"),
            )
            .expect("stdlib provider set"),
        ),
        ENTRY,
    )
}

pub(super) fn assert_fixture(
    root_module: &str,
    state: GleamStdlibRunState,
    expected: &Value,
    expected_echo: &[EchoOutput],
) {
    let mut execution = fixture(root_module);
    let mut state = RunState {
        stdlib: state,
        work: (),
    };
    let mut echo = ObservedEcho::default();
    execution
        .run(&mut state, &mut echo)
        .expect("official transferable stdlib fixture");
    echo.assert_result(expected, expected_echo);
    assert!(
        state.stdlib.io_outputs().is_empty(),
        "non-IO fixture {root_module}"
    );
}
