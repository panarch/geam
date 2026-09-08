use ecow::EcoString;
use geam_core::host::{
    HostComponentProfile, HostExternalBinding, HostExternalEquality, HostExternalHashing,
    HostExternalInspection, HostExternalSchema, HostExternalStorage, HostExternalStore,
    HostFuturePayload, HostFutureStore, HostProvider, HostProviderComponent,
    HostWorkRepresentation,
};

pub struct WorkComponent;
pub struct WorkSchema;
pub struct WorkStorage;

impl HostProviderComponent for WorkComponent {
    const ID: &'static str = "work_fixture";
    type Stores = HostFutureStore;
    type RunState = ();
}

impl<Profile: HostComponentProfile<Self>> HostProvider<Profile> for WorkComponent {
    type State = ();
    fn project(state: &mut Profile::RunState) -> &mut () {
        Profile::component_state(state)
    }
}

impl HostExternalSchema for WorkSchema {
    const PACKAGE: &'static str = "work_fixture";
    const MODULE: &'static str = "fixture/work";
    const NAME: &'static str = "Work";
    const PARAMETER_COUNT: usize = 1;
}

impl<Profile> HostExternalBinding<Profile, WorkSchema> for WorkComponent
where
    Profile: HostComponentProfile<Self>,
{
    type Storage = WorkStorage;
}

impl<Profile: HostComponentProfile<WorkComponent>> HostWorkRepresentation<Profile>
    for WorkComponent
{
    type Schema = WorkSchema;
    type Storage = WorkStorage;
    fn store(stores: &Profile::ExternalStores) -> &HostFutureStore {
        Profile::component_stores(stores)
    }
}

impl<Profile: HostComponentProfile<WorkComponent>> HostExternalStorage<Profile, WorkSchema>
    for WorkStorage
{
    type Payload = HostFuturePayload;

    fn store(stores: &Profile::ExternalStores) -> &HostExternalStore<Self::Payload> {
        Profile::component_stores(stores).values()
    }

    fn source_equal(
        _: &HostExternalEquality<'_>,
        left: &Self::Payload,
        right: &Self::Payload,
    ) -> bool {
        left.same_operation(right)
    }

    fn source_hash(_: &HostExternalHashing<'_>, value: &Self::Payload) -> u64 {
        value.operation_hash()
    }

    fn inspect(_: &HostExternalInspection<'_>, _: &Self::Payload) -> EcoString {
        "Work(...)".into()
    }
}
