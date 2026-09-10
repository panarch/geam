use super::transfer_fixture::{ENTRY, ObservedEcho, TransferFixture, observed_project};
use geam_builtin::FutureComponent;
use geam_core::host::{HostComponentProfile, HostFutureStore};
use geam_core::{EchoOutput, HostProfile, HostProviderSet, Value};
use geam_stdlib::{
    Component, GleamStdlibHostProfile, GleamStdlibRunState, GleamStdlibStores, IoOutput,
};

pub(super) struct Profile;

pub(super) struct RunState {
    pub(super) stdlib: GleamStdlibRunState,
    pub(super) work: (),
}

#[derive(Default)]
pub(super) struct Stores {
    stdlib: GleamStdlibStores,
    work: HostFutureStore,
}

impl HostProfile for Profile {
    type RunState = RunState;
    type ExternalStores = Stores;
    type ExecutionState = ();
}

impl GleamStdlibHostProfile for Profile {
    type Io = Vec<IoOutput>;
}

impl HostComponentProfile<Component> for Profile {
    fn component_stores(stores: &Stores) -> &GleamStdlibStores {
        &stores.stdlib
    }

    fn component_state(state: &mut RunState) -> &mut GleamStdlibRunState {
        &mut state.stdlib
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
    TransferFixture::new(
        observed_project(
            &super::project_root(),
            root_module,
            HostProviderSet::from_providers(
                geam_stdlib::host_providers::<Profile>().expect("stdlib transfer registration"),
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
