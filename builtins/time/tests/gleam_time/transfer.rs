use super::ScriptedSource;
use super::transfer_fixture::{ENTRY, TransferFixture, observed_project};
use geam_builtin::FutureComponent;
use geam_core::host::{HostComponentProfile, HostFutureStore};
use geam_core::{HostProfile, HostProviderSet};
use geam_stdlib::{
    Component as StdlibComponent, GleamStdlibHostProfile, GleamStdlibRunState, GleamStdlibStores,
    IoOutput,
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
    stdlib: GleamStdlibStores,
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

impl HostComponentProfile<StdlibComponent> for Profile {
    fn component_stores(stores: &Stores) -> &GleamStdlibStores {
        &stores.stdlib
    }
    fn component_state(state: &mut RunState) -> &mut GleamStdlibRunState {
        &mut state.stdlib
    }
}

impl HostComponentProfile<TimeComponent<ScriptedSource>> for Profile {
    fn component_stores(stores: &Stores) -> &() {
        &stores.time
    }
    fn component_state(state: &mut RunState) -> &mut ScriptedSource {
        &mut state.source
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
    providers.extend(geam_time::host_providers::<Profile>().expect("Time transfer registration"));
    TransferFixture::new(
        observed_project(
            &super::project_root(),
            root_module,
            HostProviderSet::from_providers(providers).expect("Time provider set"),
        ),
        ENTRY,
    )
}
