use super::ScriptedSource;
use super::transfer_fixture::{ENTRY, TransferFixture, observed_project};
use geam_core::host::{AsyncHostComponentProfile, HostFutureStore};
use geam_core::{HostProfile, TransferHostProviderSet};
use geam_runtime_api::FutureComponent;
use geam_stdlib::{
    Component as StdlibComponent, GleamStdlibHostProfile, GleamStdlibRunState,
    GleamStdlibTransferStores, IoOutput,
};
use geam_time::{Component as TimeComponent, GleamTimeHostProfile};

pub(super) struct Profile;

pub(super) struct RunState {
    pub(super) stdlib: GleamStdlibRunState,
    pub(super) source: ScriptedSource,
    pub(super) work: (),
}

#[derive(Default)]
pub(super) struct Stores {
    stdlib: GleamStdlibTransferStores,
    time: (),
    work: HostFutureStore,
}

impl HostProfile for Profile {
    type RunState = RunState;
    type ExternalStores = Stores;
}

impl GleamStdlibHostProfile for Profile {
    type Io = Vec<IoOutput>;
}
impl GleamTimeHostProfile for Profile {
    type Source = ScriptedSource;
}

impl AsyncHostComponentProfile<StdlibComponent> for Profile {
    fn component_async_stores(stores: &Stores) -> &GleamStdlibTransferStores {
        &stores.stdlib
    }
    fn component_state(state: &mut RunState) -> &mut GleamStdlibRunState {
        &mut state.stdlib
    }
}

impl AsyncHostComponentProfile<TimeComponent<ScriptedSource>> for Profile {
    fn component_async_stores(stores: &Stores) -> &() {
        &stores.time
    }
    fn component_state(state: &mut RunState) -> &mut ScriptedSource {
        &mut state.source
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
        geam_time::transfer_host_providers::<Profile>().expect("Time transfer registration"),
    );
    TransferFixture::new(
        observed_project(
            &super::project_root(),
            root_module,
            TransferHostProviderSet::new(providers).expect("Time provider set"),
        ),
        ENTRY,
    )
}
