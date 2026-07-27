mod bit_array;
mod bool;
mod float;
mod int;
mod nil;
mod string;
mod utf_codepoint;

use super::HostValueType;
use super::argument::HostCallArguments;
use crate::host::{HostCallError, HostProfile};

pub(crate) use bit_array::HostBitArrayFunction;
pub(crate) use bool::HostBoolFunction;
pub(crate) use float::HostFloatFunction;
pub(crate) use int::HostIntFunction;
pub(crate) use nil::HostNilFunction;
pub(crate) use string::HostStringFunction;
pub(crate) use utf_codepoint::HostUtfCodepointFunction;

pub(super) type HostCallback<Profile, Return> = dyn Fn(
        &mut <Profile as HostProfile>::RunState,
        &dyn HostCallArguments,
    ) -> Result<Return, HostCallError>
    + Send
    + Sync;

pub(crate) enum HostFunctionImplementation<Profile: HostProfile> {
    Int(HostIntFunction<Profile>),
    Float(HostFloatFunction<Profile>),
    String(HostStringFunction<Profile>),
    BitArray(HostBitArrayFunction<Profile>),
    UtfCodepoint(HostUtfCodepointFunction<Profile>),
    Bool(HostBoolFunction<Profile>),
    Nil(HostNilFunction<Profile>),
}

impl<Profile: HostProfile> Clone for HostFunctionImplementation<Profile> {
    fn clone(&self) -> Self {
        match self {
            Self::Int(function) => Self::Int(function.clone()),
            Self::Float(function) => Self::Float(function.clone()),
            Self::String(function) => Self::String(function.clone()),
            Self::BitArray(function) => Self::BitArray(function.clone()),
            Self::UtfCodepoint(function) => Self::UtfCodepoint(function.clone()),
            Self::Bool(function) => Self::Bool(function.clone()),
            Self::Nil(function) => Self::Nil(function.clone()),
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
