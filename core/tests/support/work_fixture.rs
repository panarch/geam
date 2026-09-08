// Source-backed adapters for the generic core work boundary.
#[path = "work_representation.rs"]
mod representation;
use geam_core::host::{
    HostCall, HostCallCompletion, HostCallable, HostConstructions, HostExternal, HostFunctionType,
    HostList, HostListType, HostProviderComponentRegistration, HostProviderModule,
    HostRegistrationError, HostTypeList, HostTypeListEnd, HostTypeParameter, HostWorkProfile,
};
pub use representation::{WorkComponent, WorkSchema};
pub type WorkHostType<Value> = geam_core::host::HostFutureType<Value, WorkSchema>;
pub type WorkType<Value> = geam_core::embedding::FutureType<Value, WorkSchema>;

impl WorkComponent {
    pub const SOURCE: &'static str = include_str!("work_fixture.gleam");
    pub fn providers<Profile>() -> Result<Vec<HostProviderModule<Profile>>, HostRegistrationError>
    where
        Profile: HostWorkProfile<Work = WorkComponent>,
    {
        <Self as HostProviderComponentRegistration<Profile>>::providers()
    }
}

impl<Profile> HostProviderComponentRegistration<Profile> for WorkComponent
where
    Profile: HostWorkProfile<Work = WorkComponent>,
{
    fn providers() -> Result<Vec<HostProviderModule<Profile>>, HostRegistrationError> {
        type A = HostTypeParameter<1>;
        type B = HostTypeParameter<0>;
        HostProviderModule::new("work_fixture", "fixture/work")
            .and_then(|module| module.with_external_type::<Self, WorkSchema>())
            .and_then(|module| module.with_scoped_function::<Self, (B,), WorkHostType<B>, _>("ready", ready::<Profile>))
            .and_then(|module| module.with_scoped_function::<Self, (WorkHostType<A>, HostFunctionType<HostTypeList<A, HostTypeListEnd>, B>), WorkHostType<B>, _>("map", map::<Profile>))
            .and_then(|module| module.with_scoped_function::<Self, (WorkHostType<WorkHostType<B>>,), WorkHostType<B>, _>("flatten", flatten::<Profile>))
            .and_then(|module| module.with_scoped_function_and_constructions::<Self, (HostListType<WorkHostType<B>>,), WorkHostType<HostListType<B>>, HostTypeList<HostListType<B>, HostTypeListEnd>, _>("all", all::<Profile>))
            .map(|module| vec![module])
    }
}

fn ready<'call, Profile>(
    call: HostCall<'call, Profile, WorkComponent, WorkHostType<HostTypeParameter<0>>>,
    value: <HostTypeParameter<0> as geam_core::HostType>::Value<'call>,
) -> Result<HostCallCompletion<'call, WorkHostType<HostTypeParameter<0>>>, geam_core::HostCallError>
where
    Profile: HostWorkProfile<Work = WorkComponent>,
{
    Ok(call.return_ready_future(value))
}

fn map<'call, Profile>(
    call: HostCall<'call, Profile, WorkComponent, WorkHostType<HostTypeParameter<0>>>,
    input: HostExternal<'call, WorkHostType<HostTypeParameter<1>>>,
    callback: HostCallable<
        'call,
        HostTypeList<HostTypeParameter<1>, HostTypeListEnd>,
        HostTypeParameter<0>,
    >,
) -> Result<HostCallCompletion<'call, WorkHostType<HostTypeParameter<0>>>, geam_core::HostCallError>
where
    Profile: HostWorkProfile<Work = WorkComponent>,
{
    Ok(call.return_mapped_future::<HostTypeParameter<1>>(input, callback))
}

fn flatten<'call, Profile>(
    call: HostCall<'call, Profile, WorkComponent, WorkHostType<HostTypeParameter<0>>>,
    input: HostExternal<'call, WorkHostType<WorkHostType<HostTypeParameter<0>>>>,
) -> Result<HostCallCompletion<'call, WorkHostType<HostTypeParameter<0>>>, geam_core::HostCallError>
where
    Profile: HostWorkProfile<Work = WorkComponent>,
{
    Ok(call.return_flattened_future(input))
}

fn all<'call, Profile>(
    call: HostCall<'call, Profile, WorkComponent, WorkHostType<HostListType<HostTypeParameter<0>>>>,
    _constructions: HostConstructions<
        'call,
        HostTypeList<HostListType<HostTypeParameter<0>>, HostTypeListEnd>,
    >,
    values: HostList<'call, WorkHostType<HostTypeParameter<0>>>,
) -> Result<
    HostCallCompletion<'call, WorkHostType<HostListType<HostTypeParameter<0>>>>,
    geam_core::HostCallError,
>
where
    Profile: HostWorkProfile<Work = WorkComponent>,
{
    Ok(call.return_all_future(values))
}
