use ecow::EcoString;
use geam_core::host::{
    HostCall, HostCallCompletion, HostCallable, HostComponentProfile, HostConstructions,
    HostExternal, HostExternalBinding, HostExternalEquality, HostExternalHashing,
    HostExternalInspection, HostExternalSchema, HostExternalStorage, HostExternalStore,
    HostFunctionType, HostFuturePayload, HostFutureStore, HostList, HostListType, HostProvider,
    HostProviderComponent, HostProviderComponentRegistration, HostProviderModule,
    HostRegistrationError, HostTypeList, HostTypeListEnd, HostTypeParameter, HostWorkProfile,
    HostWorkRepresentation,
};

pub struct FutureComponent;
pub struct HostFutureSchema;
pub struct HostFutureStorage;
pub type HostFutureType<Value> = geam_core::host::HostFutureType<Value, HostFutureSchema>;

impl FutureComponent {
    pub fn providers<Profile>() -> Result<Vec<HostProviderModule<Profile>>, HostRegistrationError>
    where
        Profile: HostWorkProfile<Work = FutureComponent>,
    {
        <Self as HostProviderComponentRegistration<Profile>>::providers()
    }
}

impl HostProviderComponent for FutureComponent {
    const ID: &'static str = "geam";
    type Stores = HostFutureStore;
    type RunState = ();
}

impl<Profile: HostComponentProfile<Self>> HostProvider<Profile> for FutureComponent {
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

impl<Profile> HostExternalBinding<Profile, HostFutureSchema> for FutureComponent
where
    Profile: HostComponentProfile<Self>,
{
    type Storage = HostFutureStorage;
}

impl<Profile: HostComponentProfile<FutureComponent>> HostWorkRepresentation<Profile>
    for FutureComponent
{
    type Schema = HostFutureSchema;
    type Storage = HostFutureStorage;
    fn store(stores: &Profile::ExternalStores) -> &HostFutureStore {
        Profile::component_stores(stores)
    }
}

impl<Profile: HostComponentProfile<FutureComponent>> HostExternalStorage<Profile, HostFutureSchema>
    for HostFutureStorage
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
        "Future(...)".into()
    }
}

impl<Profile> HostProviderComponentRegistration<Profile> for FutureComponent
where
    Profile: HostWorkProfile<Work = FutureComponent>,
{
    fn providers() -> Result<Vec<HostProviderModule<Profile>>, HostRegistrationError> {
        type A = HostTypeParameter<1>;
        type B = HostTypeParameter<0>;
        HostProviderModule::new("geam", "geam/future")
            .and_then(|module| module.with_external_type::<Self, HostFutureSchema>())
            .and_then(|module| module.with_scoped_function::<Self, (B,), HostFutureType<B>, _>("ready", ready::<Profile>))
            .and_then(|module| module.with_scoped_function::<Self, (HostFutureType<A>, HostFunctionType<HostTypeList<A, HostTypeListEnd>, B>), HostFutureType<B>, _>("map", map::<Profile>))
            .and_then(|module| module.with_scoped_function::<Self, (HostFutureType<HostFutureType<B>>,), HostFutureType<B>, _>("flatten", flatten::<Profile>))
            .and_then(|module| module.with_scoped_function_and_constructions::<Self, (HostListType<HostFutureType<B>>,), HostFutureType<HostListType<B>>, HostTypeList<HostListType<B>, HostTypeListEnd>, _>("all", all::<Profile>))
            .map(|module| vec![module])
    }
}

fn ready<'call, Profile>(
    call: HostCall<'call, Profile, FutureComponent, HostFutureType<HostTypeParameter<0>>>,
    value: <HostTypeParameter<0> as geam_core::HostType>::Value<'call>,
) -> Result<HostCallCompletion<'call, HostFutureType<HostTypeParameter<0>>>, geam_core::HostCallError>
where
    Profile: HostWorkProfile<Work = FutureComponent>,
{
    Ok(call.return_ready_future(value))
}

fn map<'call, Profile>(
    call: HostCall<'call, Profile, FutureComponent, HostFutureType<HostTypeParameter<0>>>,
    input: HostExternal<'call, HostFutureType<HostTypeParameter<1>>>,
    callback: HostCallable<
        'call,
        HostTypeList<HostTypeParameter<1>, HostTypeListEnd>,
        HostTypeParameter<0>,
    >,
) -> Result<HostCallCompletion<'call, HostFutureType<HostTypeParameter<0>>>, geam_core::HostCallError>
where
    Profile: HostWorkProfile<Work = FutureComponent>,
{
    Ok(call.return_mapped_future::<HostTypeParameter<1>>(input, callback))
}

fn flatten<'call, Profile>(
    call: HostCall<'call, Profile, FutureComponent, HostFutureType<HostTypeParameter<0>>>,
    input: HostExternal<'call, HostFutureType<HostFutureType<HostTypeParameter<0>>>>,
) -> Result<HostCallCompletion<'call, HostFutureType<HostTypeParameter<0>>>, geam_core::HostCallError>
where
    Profile: HostWorkProfile<Work = FutureComponent>,
{
    Ok(call.return_flattened_future(input))
}

fn all<'call, Profile>(
    call: HostCall<
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
    geam_core::HostCallError,
>
where
    Profile: HostWorkProfile<Work = FutureComponent>,
{
    Ok(call.return_all_future(values))
}
