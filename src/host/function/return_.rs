mod bool;
mod int;

use super::HostValueType;
use super::argument::HostCallArguments;
use crate::host::{HostCallError, HostProfile};

pub(crate) use bool::HostBoolFunction;
pub(crate) use int::HostIntFunction;

pub(super) type HostCallback<Profile, Return> = dyn Fn(
        &mut <Profile as HostProfile>::RunState,
        &dyn HostCallArguments,
    ) -> Result<Return, HostCallError>
    + Send
    + Sync;

pub(crate) enum HostFunctionImplementation<Profile: HostProfile> {
    Int(HostIntFunction<Profile>),
    Bool(HostBoolFunction<Profile>),
}

impl<Profile: HostProfile> Clone for HostFunctionImplementation<Profile> {
    fn clone(&self) -> Self {
        match self {
            Self::Int(function) => Self::Int(function.clone()),
            Self::Bool(function) => Self::Bool(function.clone()),
        }
    }
}

pub(super) trait HostReturn: Sized {
    fn type_() -> HostValueType;

    fn implementation<Profile: HostProfile>(
        function: impl Fn(&mut Profile::RunState, &dyn HostCallArguments) -> Result<Self, HostCallError>
        + Send
        + Sync
        + 'static,
    ) -> HostFunctionImplementation<Profile>;
}
