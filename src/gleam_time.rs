mod calendar;
mod source;
mod timestamp;

pub use self::source::SystemTimeSource;
use crate::gleam_stdlib::{
    Component as GleamStdlibComponent, GleamStdlibHostProfile, GleamStdlibRunState,
    GleamStdlibStores, IoOutput,
};
use crate::{
    HostComponentProfile, HostProfile, HostProvider, HostProviderComponent,
    HostProviderComponentRegistration, HostProviderModule, HostRegistrationError,
};
use std::marker::PhantomData;
use std::time::SystemTime;

/// A caller-owned source for official Gleam wall-clock operations.
pub trait TimeSource: 'static {
    /// Returns the current wall-clock time.
    fn system_time(&mut self) -> Result<SystemTime, crate::HostFailure>;

    /// Returns the current local offset from UTC in seconds.
    fn local_offset_seconds(&mut self) -> Result<i32, crate::HostFailure>;
}

/// A host profile that composes the official Gleam Time and standard-library components.
pub trait GleamTimeHostProfile:
    GleamStdlibHostProfile + HostComponentProfile<Component<Self::Source>>
{
    /// The concrete caller-owned wall-clock source.
    type Source: TimeSource;
}

/// The statically composed provider component for the official Gleam Time package.
#[derive(Debug, Clone, Copy)]
pub struct Component<Source = SystemTimeSource>(PhantomData<fn() -> Source>);

impl<Source> HostProviderComponent for Component<Source>
where
    Source: TimeSource,
{
    const ID: &'static str = "gleam_time";
    type Stores = ();
    type RunState = Source;
}

/// External stores for the default combined standard-library and Time profile.
#[derive(Default)]
pub struct GleamTimeProfileStores {
    stdlib: GleamStdlibStores,
    time: (),
}

/// Caller-owned run state for the default Gleam Time profile.
pub struct GleamTimeRunState<Source = SystemTimeSource> {
    stdlib: GleamStdlibRunState,
    source: Source,
}

/// The default profile for composing official standard-library and Time providers.
#[derive(Debug, Clone, Copy)]
pub struct GleamTimeProfile<Source = SystemTimeSource>(PhantomData<fn() -> Source>);

impl<Source> GleamTimeRunState<Source> {
    /// Combines caller-owned standard-library state and a wall-clock source.
    pub fn new(stdlib: GleamStdlibRunState, source: Source) -> Self {
        Self { stdlib, source }
    }

    /// Returns the standard-library state.
    pub fn stdlib(&self) -> &GleamStdlibRunState {
        &self.stdlib
    }

    /// Returns mutable standard-library state.
    pub fn stdlib_mut(&mut self) -> &mut GleamStdlibRunState {
        &mut self.stdlib
    }

    /// Returns the wall-clock source.
    pub fn source(&self) -> &Source {
        &self.source
    }

    /// Returns the mutable wall-clock source.
    pub fn source_mut(&mut self) -> &mut Source {
        &mut self.source
    }
}

impl<Source> HostProfile for GleamTimeProfile<Source>
where
    Source: TimeSource,
{
    type RunState = GleamTimeRunState<Source>;
    type ExternalStores = GleamTimeProfileStores;
}

impl<Source> HostComponentProfile<GleamStdlibComponent> for GleamTimeProfile<Source>
where
    Source: TimeSource,
{
    fn component_stores(stores: &Self::ExternalStores) -> &GleamStdlibStores {
        &stores.stdlib
    }

    fn component_state(state: &mut Self::RunState) -> &mut GleamStdlibRunState {
        &mut state.stdlib
    }
}

impl<Source> HostComponentProfile<Component<Source>> for GleamTimeProfile<Source>
where
    Source: TimeSource,
{
    fn component_stores(stores: &Self::ExternalStores) -> &() {
        &stores.time
    }

    fn component_state(state: &mut Self::RunState) -> &mut Source {
        &mut state.source
    }
}

impl<Source> GleamStdlibHostProfile for GleamTimeProfile<Source>
where
    Source: TimeSource,
{
    type Io = Vec<IoOutput>;
}

impl<Source> GleamTimeHostProfile for GleamTimeProfile<Source>
where
    Source: TimeSource,
{
    type Source = Source;
}

/// Registers the Rust providers for the official Gleam Time package.
pub fn host_providers<Profile>() -> Result<Vec<HostProviderModule<Profile>>, HostRegistrationError>
where
    Profile: GleamTimeHostProfile,
{
    <Component<Profile::Source> as HostProviderComponentRegistration<Profile>>::providers()
}

impl<Profile, Source> HostProviderComponentRegistration<Profile> for Component<Source>
where
    Profile: GleamTimeHostProfile<Source = Source>,
    Source: TimeSource,
{
    fn providers() -> Result<Vec<HostProviderModule<Profile>>, HostRegistrationError> {
        [
            calendar::host_provider::<Profile>,
            timestamp::host_provider::<Profile>,
        ]
        .into_iter()
        .map(|register| register())
        .collect()
    }
}

fn time_state<Profile>(state: &mut Profile::RunState) -> &mut Profile::Source
where
    Profile: GleamTimeHostProfile,
{
    <Profile as HostComponentProfile<Component<Profile::Source>>>::component_state(state)
}

pub(super) struct TimeProvider<Profile>(PhantomData<Profile>);

impl<Profile> HostProvider<Profile> for TimeProvider<Profile>
where
    Profile: GleamTimeHostProfile,
{
    type State = Profile::Source;

    fn project(state: &mut Profile::RunState) -> &mut Self::State {
        time_state::<Profile>(state)
    }
}

#[cfg(test)]
mod effects;
#[cfg(test)]
mod test_support;

#[cfg(test)]
mod tests {
    use super::test_support::ScriptedSource;
    use super::{
        Component, GleamTimeProfile, GleamTimeProfileStores, GleamTimeRunState, TimeProvider,
        host_providers,
    };
    use crate::gleam_stdlib::{
        Component as GleamStdlibComponent, GleamStdlibHostProfile, GleamStdlibRunState,
        GleamStdlibStores, IoOutput,
    };
    use crate::{
        HostComponentProfile, HostProfile, HostProvider, HostProviderComponent,
        HostProviderComponentRegistration,
    };

    struct CustomProfile;

    #[derive(Default)]
    struct CustomStores {
        stdlib: GleamStdlibStores,
        time: (),
    }

    struct CustomRunState {
        stdlib: GleamStdlibRunState,
        source: ScriptedSource,
    }

    impl HostProfile for CustomProfile {
        type RunState = CustomRunState;
        type ExternalStores = CustomStores;
    }

    impl HostComponentProfile<GleamStdlibComponent> for CustomProfile {
        fn component_stores(stores: &Self::ExternalStores) -> &GleamStdlibStores {
            &stores.stdlib
        }

        fn component_state(state: &mut Self::RunState) -> &mut GleamStdlibRunState {
            &mut state.stdlib
        }
    }

    impl GleamStdlibHostProfile for CustomProfile {
        type Io = Vec<IoOutput>;
    }

    impl HostComponentProfile<Component<ScriptedSource>> for CustomProfile {
        fn component_stores(stores: &Self::ExternalStores) -> &() {
            &stores.time
        }

        fn component_state(state: &mut Self::RunState) -> &mut ScriptedSource {
            &mut state.source
        }
    }

    impl super::GleamTimeHostProfile for CustomProfile {
        type Source = ScriptedSource;
    }

    #[test]
    fn default_and_custom_profiles_project_owned_state_stores_source_and_io() {
        let default_stores = GleamTimeProfileStores::default();
        let custom_stores = CustomStores::default();
        let mut default_state = GleamTimeRunState::new(
            GleamStdlibRunState::from_seed([1; 32]),
            ScriptedSource::default(),
        );
        let mut custom_state = CustomRunState {
            stdlib: GleamStdlibRunState::from_seed([2; 32]),
            source: ScriptedSource::default(),
        };

        assert!(std::ptr::eq(
            <GleamTimeProfile<ScriptedSource> as HostComponentProfile<
                GleamStdlibComponent,
            >>::component_stores(&default_stores),
            &default_stores.stdlib,
        ));
        assert!(std::ptr::eq(
            <CustomProfile as HostComponentProfile<GleamStdlibComponent>>::component_stores(
                &custom_stores,
            ),
            &custom_stores.stdlib,
        ));
        let default_stdlib = default_state.stdlib() as *const GleamStdlibRunState;
        assert!(std::ptr::eq(
            <GleamTimeProfile<ScriptedSource> as HostComponentProfile<
                GleamStdlibComponent,
            >>::component_state(&mut default_state),
            default_stdlib,
        ));
        let custom_stdlib = &custom_state.stdlib as *const GleamStdlibRunState;
        assert!(std::ptr::eq(
            <CustomProfile as HostComponentProfile<GleamStdlibComponent>>::component_state(
                &mut custom_state,
            ),
            custom_stdlib,
        ));

        let default_source = default_state.source() as *const ScriptedSource;
        assert!(
            std::ptr::eq(
                <GleamTimeProfile<ScriptedSource> as HostComponentProfile<
                    Component<ScriptedSource>,
                >>::component_state(&mut default_state),
                default_source,
            )
        );
        let custom_source = &custom_state.source as *const ScriptedSource;
        assert!(std::ptr::eq(
            <CustomProfile as HostComponentProfile<Component<ScriptedSource>>>::component_state(
                &mut custom_state,
            ),
            custom_source,
        ));
        assert!(
            std::ptr::eq(
                <GleamTimeProfile<ScriptedSource> as HostComponentProfile<
                    Component<ScriptedSource>,
                >>::component_stores(&default_stores),
                &default_stores.time,
            )
        );
        assert!(std::ptr::eq(
            <CustomProfile as HostComponentProfile<Component<ScriptedSource>>>::component_stores(
                &custom_stores,
            ),
            &custom_stores.time,
        ));
        let provider_source = default_state.source() as *const ScriptedSource;
        assert!(std::ptr::eq(
            <TimeProvider<GleamTimeProfile<ScriptedSource>> as HostProvider<
                GleamTimeProfile<ScriptedSource>,
            >>::project(&mut default_state),
            provider_source,
        ));

        assert!(default_state.stdlib_mut().take_io_outputs().is_empty());
        assert!(default_state.stdlib().io_outputs().is_empty());
        default_state.source_mut().offsets.push_back(Ok(3600));

        assert!(default_state.stdlib().io_outputs().is_empty());
        assert_eq!(default_state.source().offsets.len(), 1);
    }

    #[test]
    fn registers_calendar_then_timestamp_without_external_types() {
        assert_eq!(<Component as HostProviderComponent>::ID, "gleam_time");
        let providers =
            <Component as HostProviderComponentRegistration<GleamTimeProfile>>::providers()
                .expect("Time component should register");
        let facade =
            host_providers::<GleamTimeProfile>().expect("official Time providers should register");
        assert_eq!(
            facade
                .iter()
                .map(|provider| provider.module().as_str())
                .collect::<Vec<_>>(),
            providers
                .iter()
                .map(|provider| provider.module().as_str())
                .collect::<Vec<_>>(),
        );

        assert_eq!(
            providers
                .iter()
                .map(|provider| (provider.package().as_str(), provider.module().as_str()))
                .collect::<Vec<_>>(),
            [
                ("gleam_time", "gleam/time/calendar"),
                ("gleam_time", "gleam/time/timestamp"),
            ],
        );
        assert!(
            providers
                .iter()
                .all(|provider| provider.external_types().count() == 0),
        );
    }
}
