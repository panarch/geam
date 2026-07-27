mod bool;
mod int;

use super::HostValueType;
use super::argument::HostCallArguments;

pub(crate) use bool::HostBoolFunction;
pub(crate) use int::HostIntFunction;

pub(super) type HostCallback<Return> = dyn Fn(&dyn HostCallArguments) -> Return + Send + Sync;

#[derive(Clone)]
pub(crate) enum HostFunctionImplementation {
    Int(HostIntFunction),
    Bool(HostBoolFunction),
}

pub(super) trait HostReturn: Sized {
    fn type_() -> HostValueType;

    fn implementation(
        function: impl Fn(&dyn HostCallArguments) -> Self + Send + Sync + 'static,
    ) -> HostFunctionImplementation;
}
