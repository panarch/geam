use ecow::EcoString;
use geam_core::host::{
    AsyncHostComponentProfile, AsyncHostExternalBinding, AsyncHostExternalEquality,
    AsyncHostExternalHashing, AsyncHostExternalInspection, AsyncHostExternalStorage,
    AsyncHostExternalStore, AsyncHostProviderComponent, HostExternalSchema, HostFuturePayload,
    HostFutureStore, HostProvider, HostProviderComponent, HostWorkRepresentation,
};

pub struct WorkComponent;
pub struct WorkSchema;
pub struct WorkStorage;

impl HostProviderComponent for WorkComponent {
    const ID: &'static str = "work_fixture";
    type Stores = HostFutureStore;
    type RunState = ();
}

impl AsyncHostProviderComponent for WorkComponent {
    type AsyncStores = HostFutureStore;
}

impl<Profile: AsyncHostComponentProfile<Self>> HostProvider<Profile> for WorkComponent {
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

impl<Profile> AsyncHostExternalBinding<Profile, WorkSchema> for WorkComponent
where
    Profile: AsyncHostComponentProfile<Self>,
{
    type Storage = WorkStorage;
}

impl<Profile: AsyncHostComponentProfile<WorkComponent>> HostWorkRepresentation<Profile>
    for WorkComponent
{
    type Schema = WorkSchema;
    type Storage = WorkStorage;
    fn store(stores: &Profile::ExternalStores) -> &HostFutureStore {
        Profile::component_async_stores(stores)
    }
}

impl<Profile: AsyncHostComponentProfile<WorkComponent>>
    AsyncHostExternalStorage<Profile, WorkSchema> for WorkStorage
{
    type Payload = HostFuturePayload;

    fn store(stores: &Profile::ExternalStores) -> &AsyncHostExternalStore<Self::Payload> {
        Profile::component_async_stores(stores).values()
    }

    fn source_equal(
        _: &AsyncHostExternalEquality<'_>,
        left: &Self::Payload,
        right: &Self::Payload,
    ) -> bool {
        left.same_operation(right)
    }

    fn source_hash(_: &AsyncHostExternalHashing<'_>, value: &Self::Payload) -> u64 {
        value.operation_hash()
    }

    fn inspect(_: &AsyncHostExternalInspection<'_>, _: &Self::Payload) -> EcoString {
        "Work(...)".into()
    }
}
