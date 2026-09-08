use ecow::EcoString;
use geam_core::host::{
    AsyncHostComponentProfile, AsyncHostExternalBinding, AsyncHostExternalEquality,
    AsyncHostExternalHashing, AsyncHostExternalInspection, AsyncHostExternalStorage,
    AsyncHostExternalStore, AsyncHostProviderComponent, HostCallCompletion, HostCallable,
    HostConstructions, HostExternal, HostExternalSchema, HostFunctionType, HostFuturePayload,
    HostFutureStore, HostList, HostListType, HostProvider, HostProviderComponent,
    HostRegistrationError, HostTypeList, HostTypeListEnd, HostTypeParameter, HostWorkProfile,
    HostWorkRepresentation, TransferHostCall, TransferHostProviderComponentRegistration,
    TransferHostProviderModule,
};

pub struct FutureComponent;
pub struct HostFutureSchema;
pub struct HostFutureStorage;
pub type HostFutureType<Value> = geam_core::host::HostFutureType<Value, HostFutureSchema>;

impl FutureComponent {
    pub fn providers<Profile>()
    -> Result<Vec<TransferHostProviderModule<Profile>>, HostRegistrationError>
    where
        Profile: HostWorkProfile<Work = FutureComponent>,
    {
        <Self as TransferHostProviderComponentRegistration<Profile>>::providers()
    }
}

impl HostProviderComponent for FutureComponent {
    const ID: &'static str = "geam";
    type Stores = HostFutureStore;
    type RunState = ();
}

impl AsyncHostProviderComponent for FutureComponent {
    type AsyncStores = HostFutureStore;
}

impl<Profile: AsyncHostComponentProfile<Self>> HostProvider<Profile> for FutureComponent {
    type State = ();
    fn project(state: &mut Profile::RunState) -> &mut () {
        Profile::component_state(state)
    }
}

impl HostExternalSchema for HostFutureSchema {
    const PACKAGE: &'static str = "geam";
    const MODULE: &'static str = "geam/future";
    const NAME: &'static str = "Future";
    const PARAMETER_COUNT: usize = 1;
}

impl<Profile> AsyncHostExternalBinding<Profile, HostFutureSchema> for FutureComponent
where
    Profile: AsyncHostComponentProfile<Self>,
{
    type Storage = HostFutureStorage;
}

impl<Profile: AsyncHostComponentProfile<FutureComponent>> HostWorkRepresentation<Profile>
    for FutureComponent
{
    type Schema = HostFutureSchema;
    type Storage = HostFutureStorage;
    fn store(stores: &Profile::ExternalStores) -> &HostFutureStore {
        Profile::component_async_stores(stores)
    }
}

impl<Profile: AsyncHostComponentProfile<FutureComponent>>
    AsyncHostExternalStorage<Profile, HostFutureSchema> for HostFutureStorage
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
        "Future(...)".into()
    }
}

impl<Profile> TransferHostProviderComponentRegistration<Profile> for FutureComponent
where
    Profile: HostWorkProfile<Work = FutureComponent>,
{
    fn providers() -> Result<Vec<TransferHostProviderModule<Profile>>, HostRegistrationError> {
        type A = HostTypeParameter<1>;
        type B = HostTypeParameter<0>;
        TransferHostProviderModule::new_for_profile("geam", "geam/future")
            .and_then(|module| module.with_external_type::<Self, HostFutureSchema>())
            .and_then(|module| module.with_scoped_function::<Self, (B,), HostFutureType<B>, _>("ready", ready::<Profile>))
            .and_then(|module| module.with_scoped_function::<Self, (HostFutureType<A>, HostFunctionType<HostTypeList<A, HostTypeListEnd>, B>), HostFutureType<B>, _>("map", map::<Profile>))
            .and_then(|module| module.with_scoped_function::<Self, (HostFutureType<HostFutureType<B>>,), HostFutureType<B>, _>("flatten", flatten::<Profile>))
            .and_then(|module| module.with_scoped_function_and_constructions::<Self, (HostListType<HostFutureType<B>>,), HostFutureType<HostListType<B>>, HostTypeList<HostListType<B>, HostTypeListEnd>, _>("all", all::<Profile>))
            .map(|module| vec![module])
    }
}

fn ready<'call, Profile>(
    call: TransferHostCall<'call, Profile, FutureComponent, HostFutureType<HostTypeParameter<0>>>,
    value: <HostTypeParameter<0> as geam_core::HostType>::Value<'call>,
) -> Result<
    HostCallCompletion<'call, HostFutureType<HostTypeParameter<0>>>,
    geam_core::AsyncHostCallError,
>
where
    Profile: HostWorkProfile<Work = FutureComponent>,
{
    Ok(call.return_ready_future(value))
}

fn map<'call, Profile>(
    call: TransferHostCall<'call, Profile, FutureComponent, HostFutureType<HostTypeParameter<0>>>,
    input: HostExternal<'call, HostFutureType<HostTypeParameter<1>>>,
    callback: HostCallable<
        'call,
        HostTypeList<HostTypeParameter<1>, HostTypeListEnd>,
        HostTypeParameter<0>,
    >,
) -> Result<
    HostCallCompletion<'call, HostFutureType<HostTypeParameter<0>>>,
    geam_core::AsyncHostCallError,
>
where
    Profile: HostWorkProfile<Work = FutureComponent>,
{
    Ok(call.return_mapped_future::<HostTypeParameter<1>>(input, callback))
}

fn flatten<'call, Profile>(
    call: TransferHostCall<'call, Profile, FutureComponent, HostFutureType<HostTypeParameter<0>>>,
    input: HostExternal<'call, HostFutureType<HostFutureType<HostTypeParameter<0>>>>,
) -> Result<
    HostCallCompletion<'call, HostFutureType<HostTypeParameter<0>>>,
    geam_core::AsyncHostCallError,
>
where
    Profile: HostWorkProfile<Work = FutureComponent>,
{
    Ok(call.return_flattened_future(input))
}

fn all<'call, Profile>(
    call: TransferHostCall<
        'call,
        Profile,
        FutureComponent,
        HostFutureType<HostListType<HostTypeParameter<0>>>,
    >,
    _constructions: HostConstructions<
        'call,
        HostTypeList<HostListType<HostTypeParameter<0>>, HostTypeListEnd>,
    >,
    values: HostList<'call, HostFutureType<HostTypeParameter<0>>>,
) -> Result<
    HostCallCompletion<'call, HostFutureType<HostListType<HostTypeParameter<0>>>>,
    geam_core::AsyncHostCallError,
>
where
    Profile: HostWorkProfile<Work = FutureComponent>,
{
    Ok(call.return_all_future(values))
}
