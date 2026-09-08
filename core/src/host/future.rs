mod context;
mod native;
mod value;

pub use context::{HostFutureCallable, HostFutureContext, HostFutureError, SharedExecutionError};
pub use native::HostFutureCompletion;
pub use value::HostFutureValue;

use super::{
    AsyncHostComponentProfile, AsyncHostExternalStorage, AsyncHostExternalStore,
    AsyncHostProviderComponent, HostCallCompletion, HostCallable, HostExternal, HostExternalSchema,
    HostExternalType, HostList, HostListType, HostProvider, HostType, HostTypeDescriptor,
    HostTypeList, HostTypeListEnd, TransferHostCall,
};
use crate::runtime::work::execution::SourceWork;
use std::hash::{DefaultHasher, Hash, Hasher};

/// Statically selects the nominal work representation and its typed storage.
pub trait HostWorkProfile:
    super::HostProfile + AsyncHostComponentProfile<Self::Work> + Sized
{
    type Work: HostWorkRepresentation<Self>;
}

/// The component owns the work schema, storage binding, and store projection.
pub trait HostWorkRepresentation<Profile: super::HostProfile>: AsyncHostProviderComponent {
    type Schema: HostExternalSchema;
    type Storage: AsyncHostExternalStorage<Profile, Self::Schema, Payload = HostFuturePayload>;

    fn store(stores: &Profile::ExternalStores) -> &HostFutureStore;
}

pub type HostWorkSchema<Profile> =
    <<Profile as HostWorkProfile>::Work as HostWorkRepresentation<Profile>>::Schema;
pub type HostWorkStorage<Profile> =
    <<Profile as HostWorkProfile>::Work as HostWorkRepresentation<Profile>>::Storage;

pub(crate) fn work_store<Profile: HostWorkProfile>(
    stores: &Profile::ExternalStores,
) -> &HostFutureStore {
    Profile::Work::store(stores)
}

/// Storage for a component's typed representation of runtime-owned work.
#[derive(Default)]
pub struct HostFutureStore {
    values: AsyncHostExternalStore<HostFuturePayload>,
}

/// The statically typed Future return used by provider registration.
pub type HostFutureType<Output, Schema> =
    HostExternalType<Schema, HostTypeList<Output, HostTypeListEnd>>;

type ProfileFutureType<Profile, Output> = HostFutureType<Output, HostWorkSchema<Profile>>;

/// An opaque shared operation stored by the selected work component.
pub struct HostFuturePayload {
    work: SourceWork,
}

impl HostFutureStore {
    /// Storage projected by a downstream work representation's external adapter.
    pub fn values(&self) -> &AsyncHostExternalStore<HostFuturePayload> {
        &self.values
    }

    pub(crate) fn work(&self, lease: &crate::runtime::TransferExternalPayloadLease) -> SourceWork {
        self.values.with_view(lease, |value| value.work.clone())
    }

    pub(crate) fn clone_handle(&self) -> Self {
        Self {
            values: self.values.clone_handle(),
        }
    }
}

impl HostFuturePayload {
    pub fn same_operation(&self, other: &Self) -> bool {
        self.work.identity() == other.work.identity()
    }

    pub fn operation_hash(&self) -> u64 {
        let mut hash = DefaultHasher::new();
        self.work.identity().hash(&mut hash);
        hash.finish()
    }
}

impl<'call, Profile, Provider, Output>
    TransferHostCall<'call, Profile, Provider, ProfileFutureType<Profile, Output>>
where
    Profile: HostWorkProfile,
    Provider: HostProvider<Profile>,
    Output: HostType,
{
    /// Returns an already-completed source Future without starting an executor.
    pub fn return_ready_future(
        self,
        value: Output::Value<'call>,
    ) -> HostCallCompletion<'call, ProfileFutureType<Profile, Output>> {
        let value = self.retain_value::<Output>(value);
        let work = self.runtime.work().ready(value);
        self.return_work(work)
    }

    /// Retains an exact source callback to transform one shared completion.
    pub fn return_mapped_future<Input: HostType>(
        self,
        input: HostExternal<'call, ProfileFutureType<Profile, Input>>,
        callback: HostCallable<'call, HostTypeList<Input, HostTypeListEnd>, Output>,
    ) -> HostCallCompletion<'call, ProfileFutureType<Profile, Output>> {
        let input = self.runtime.external_lease(input.token);
        let input = crate::host::work_store::<Profile>(self.runtime.external_stores()).work(&input);
        let callback = self.runtime.callable(callback.token);
        let work = self
            .runtime
            .work()
            .map(input, callback, self.runtime.origin());
        self.return_work(work)
    }

    fn return_work(
        self,
        work: SourceWork,
    ) -> HostCallCompletion<'call, ProfileFutureType<Profile, Output>> {
        let lease = crate::host::work_store::<Profile>(self.runtime.external_stores())
            .values
            .insert::<Profile, crate::host::HostWorkSchema<Profile>, crate::host::HostWorkStorage<Profile>>(HostFuturePayload { work });
        let value = HostExternal::new(self.runtime.build_external(
            &HostTypeDescriptor::of::<ProfileFutureType<Profile, Output>>(),
            lease,
        ));
        self.return_value(value)
    }

    pub fn return_flattened_future(
        self,
        input: HostExternal<'call, ProfileFutureType<Profile, ProfileFutureType<Profile, Output>>>,
    ) -> HostCallCompletion<'call, ProfileFutureType<Profile, Output>> {
        let input = self.runtime.external_lease(input.token);
        let input = crate::host::work_store::<Profile>(self.runtime.external_stores()).work(&input);
        let work =
            self.runtime
                .work()
                .flatten(input, self.runtime.codec_scope(), self.runtime.origin());
        self.return_work(work)
    }
}

impl<'call, Profile, Provider, Output>
    TransferHostCall<'call, Profile, Provider, ProfileFutureType<Profile, HostListType<Output>>>
where
    Profile: HostWorkProfile,
    Provider: HostProvider<Profile>,
    Output: HostType,
{
    pub fn return_all_future(
        self,
        inputs: HostList<'call, ProfileFutureType<Profile, Output>>,
    ) -> HostCallCompletion<'call, ProfileFutureType<Profile, HostListType<Output>>> {
        let inputs = self.retain_list_value(inputs);
        let store =
            crate::host::work_store::<Profile>(self.runtime.external_stores()).clone_handle();
        let work = self.runtime.work().all(
            inputs,
            store,
            HostTypeDescriptor::of::<HostListType<Output>>(),
            self.runtime.codec_scope(),
            self.runtime.origin(),
        );
        self.return_work(work)
    }
}
