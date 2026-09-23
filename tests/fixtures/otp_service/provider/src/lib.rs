//! A test consumer of the public process and callable contracts against pinned OTP.
mod actor;
mod child;
mod factory_supervisor;
mod schema;
mod static_supervisor;
mod system;

#[cfg(test)]
mod test_support;

#[cfg(test)]
extern crate geam as geam_core;

use geam::gleam_erlang::GleamErlangHostProfile;
use geam::host::{
    HostCall, HostComponentProfile, HostExternalBinding, HostExternalEquality, HostExternalHashing,
    HostExternalInspection, HostExternalStorage, HostExternalStore, HostProfile, HostProvider,
    HostProviderComponent, HostProviderComponentInitialization, HostProviderComponentRegistration,
    HostProviderConfiguration, HostProviderInitializationError, HostProviderModule,
    HostRegistrationError, HostTypeList, HostTypeListEnd, HostTypeParameter,
};
use geam::provider::advanced::NativeValue;
use std::marker::PhantomData;

pub struct Component<Profile>(PhantomData<fn() -> Profile>);

pub struct Stores<Profile: HostProfile> {
    values: HostExternalStore<NativeValue>,
    static_children: HostExternalStore<std::sync::Arc<static_supervisor::Child<Profile>>>,
    factory_children: HostExternalStore<std::sync::Arc<factory_supervisor::Child<Profile>>>,
    factory_handles: HostExternalStore<factory_supervisor::Handle>,
    profile: PhantomData<fn() -> Profile>,
}

impl<Profile: HostProfile> Default for Stores<Profile> {
    fn default() -> Self {
        Self {
            values: HostExternalStore::default(),
            static_children: HostExternalStore::default(),
            factory_children: HostExternalStore::default(),
            factory_handles: HostExternalStore::default(),
            profile: PhantomData,
        }
    }
}

#[derive(Default)]
pub struct State {
    pub warnings: Vec<String>,
}

impl<Profile: HostProfile> HostProviderComponent for Component<Profile> {
    const ID: &'static str = "gleam_otp";
    type Stores = Stores<Profile>;
    type RunState = State;
}

impl<Profile: HostProfile> HostProviderComponentInitialization for Component<Profile> {
    fn initialize(_: &HostProviderConfiguration) -> Result<State, HostProviderInitializationError> {
        Ok(State::default())
    }
}

pub trait OtpProfile: GleamErlangHostProfile + HostComponentProfile<Component<Self>> {}
impl<Profile> OtpProfile for Profile where
    Profile: GleamErlangHostProfile + HostComponentProfile<Component<Self>>
{
}

impl<Profile: OtpProfile> HostProvider<Profile> for Component<Profile> {
    type State = State;
    fn project(state: &mut Profile::RunState) -> &mut State {
        <Profile as HostComponentProfile<Self>>::component_state(state)
    }
}

impl<Profile: OtpProfile> HostProviderComponentRegistration<Profile> for Component<Profile> {
    fn providers() -> Result<Vec<HostProviderModule<Profile>>, HostRegistrationError> {
        [
            actor::provider::<Profile>,
            system::provider::<Profile>,
            system::support_provider::<Profile>,
            static_supervisor::provider::<Profile>,
            factory_supervisor::provider::<Profile>,
        ]
        .into_iter()
        .map(|register| register())
        .collect()
    }
}

pub struct NativeStorage;
impl<Profile: OtpProfile, Schema: schema::NativeSchema> HostExternalBinding<Profile, Schema>
    for Component<Profile>
{
    type Storage = NativeStorage;
}
impl<Profile: OtpProfile, Schema: schema::NativeSchema> HostExternalStorage<Profile, Schema>
    for NativeStorage
{
    type Payload = NativeValue;
    fn store(stores: &Profile::ExternalStores) -> &HostExternalStore<NativeValue> {
        &<Profile as HostComponentProfile<Component<Profile>>>::component_stores(stores).values
    }
    fn source_equal(
        context: &HostExternalEquality<'_>,
        left: &NativeValue,
        right: &NativeValue,
    ) -> bool {
        left.source_equal(context, right)
    }
    fn source_hash(context: &HostExternalHashing<'_>, value: &NativeValue) -> u64 {
        value.source_hash(context)
    }
    fn inspect(
        context: &HostExternalInspection<'_>,
        value: &NativeValue,
    ) -> geam::provider::EcoString {
        value.inspect(context)
    }
    fn native_view(value: &NativeValue) -> Option<NativeValue> {
        Some(value.clone())
    }
}

type Call<'call, Profile, Return> = HostCall<'call, Profile, Component<Profile>, Return>;
type One<T> = HostTypeList<T, HostTypeListEnd>;
type Two<A, B> = HostTypeList<A, One<B>>;
type A = HostTypeParameter<0>;
type B = HostTypeParameter<1>;

type Three<A, B, C> = HostTypeList<A, Two<B, C>>;
type Four<A, B, C, D> = HostTypeList<A, Three<B, C, D>>;

#[cfg(test)]
mod native_storage_tests {
    use super::schema::{
        DebugStateSchema, DoNotLeakSchema, StaticFlagsSchema, StaticTimeoutSchema,
    };
    use super::test_support::storage::native_protocol;

    #[test]
    fn native_storage_preserves_each_static_and_system_payload_family() {
        native_protocol::<DebugStateSchema>(
            r#"
pub type Probe
pub type DebugOption { NoDebug }
@external(erlang, "fixture", "make") fn make(value: a) -> Probe
@external(erlang, "fixture", "check") fn check(a: Probe, b: Probe, c: Probe, expected: String) -> Nil
pub fn main() { check(make([NoDebug]), make([NoDebug]), make([]), "[NoDebug]") }
"#,
        );
        native_protocol::<DoNotLeakSchema>(
            r#"
pub type Probe
@external(erlang, "fixture", "make") fn make(value: a) -> Probe
@external(erlang, "fixture", "check") fn check(a: Probe, b: Probe, c: Probe, expected: String) -> Nil
pub fn main() { check(make(Nil), make(Nil), make(False), "Nil") }
"#,
        );
        native_protocol::<StaticFlagsSchema>(
            r#"
pub type Probe
pub type Flag { Intensity(Int) }
@external(erlang, "fixture", "make") fn make(value: a) -> Probe
@external(erlang, "fixture", "check") fn check(a: Probe, b: Probe, c: Probe, expected: String) -> Nil
pub fn main() { check(make([Intensity(1)]), make([Intensity(1)]), make([Intensity(2)]), "[Intensity(1)]") }
"#,
        );
        native_protocol::<StaticTimeoutSchema>(
            r#"
pub type Probe
@external(erlang, "fixture", "make") fn make(value: a) -> Probe
@external(erlang, "fixture", "check") fn check(a: Probe, b: Probe, c: Probe, expected: String) -> Nil
pub fn main() { check(make(5000), make(5000), make(0), "5000") }
"#,
        );
    }
}
