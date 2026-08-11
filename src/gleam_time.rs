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
mod tests;
