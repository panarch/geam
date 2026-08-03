mod calendar;
mod source;
mod timestamp;

pub use self::source::SystemTimeSource;
use crate::gleam_stdlib::{
    GleamStdlibHostProfile, GleamStdlibProfile, GleamStdlibRunState, GleamStdlibStores, IoOutput,
};
use crate::{HostProfile, HostProvider, HostProviderModule, HostRegistrationError};
use std::marker::PhantomData;
use std::time::SystemTime;

/// A caller-owned source for official Gleam wall-clock operations.
pub trait TimeSource: 'static {
    /// Returns the current wall-clock time.
    fn system_time(&mut self) -> Result<SystemTime, crate::HostFailure>;

    /// Returns the current local offset from UTC in seconds.
    fn local_offset_seconds(&mut self) -> Result<i32, crate::HostFailure>;
}

/// A host profile that exposes caller-owned state for the official Gleam Time package.
pub trait GleamTimeHostProfile: GleamStdlibHostProfile {
    /// The concrete caller-owned wall-clock source.
    type Source: TimeSource;

    /// Projects the wall-clock source from this profile's run state.
    fn gleam_time_source(state: &mut Self::RunState) -> &mut Self::Source;
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
    type ExternalStores = GleamStdlibStores;
}

impl<Source> GleamStdlibHostProfile for GleamTimeProfile<Source>
where
    Source: TimeSource,
{
    type Io = Vec<IoOutput>;

    fn gleam_stdlib_stores(stores: &Self::ExternalStores) -> &GleamStdlibStores {
        stores
    }

    fn gleam_stdlib_run_state(state: &mut Self::RunState) -> &mut GleamStdlibRunState {
        &mut state.stdlib
    }

    fn gleam_stdlib_io(state: &mut Self::RunState) -> &mut Self::Io {
        GleamStdlibProfile::gleam_stdlib_io(&mut state.stdlib)
    }
}

impl<Source> GleamTimeHostProfile for GleamTimeProfile<Source>
where
    Source: TimeSource,
{
    type Source = Source;

    fn gleam_time_source(state: &mut Self::RunState) -> &mut Self::Source {
        &mut state.source
    }
}

/// Registers the Rust providers for the official Gleam Time package.
pub fn host_providers<Profile>() -> Result<Vec<HostProviderModule<Profile>>, HostRegistrationError>
where
    Profile: GleamTimeHostProfile,
{
    [
        calendar::host_provider::<Profile>,
        timestamp::host_provider::<Profile>,
    ]
    .into_iter()
    .map(|register| register())
    .collect()
}

pub(super) struct TimeProvider<Profile>(PhantomData<Profile>);

impl<Profile> HostProvider<Profile> for TimeProvider<Profile>
where
    Profile: GleamTimeHostProfile,
{
    type State = Profile::Source;

    fn project(state: &mut Profile::RunState) -> &mut Self::State {
        Profile::gleam_time_source(state)
    }
}

#[cfg(test)]
mod tests;
