mod error;
mod failure;
mod function;
mod module;
mod profile;

pub use error::HostRegistrationError;
pub use failure::{HostCallError, HostFailure};
pub use function::{FallibleHostFunction, HostFunction, HostFunctionSchema, ScopedHostFunction};
pub use module::{HostModule, HostProviderModule, HostProviderSet};
pub use profile::{HostCall, HostProfile, HostProvider, StatelessHostProfile};

pub(crate) use failure::HostCallErrorKind;
pub(crate) use function::{
    HostBitArrayArgumentSlot, HostBitArrayFunction, HostBoolArgumentSlot, HostBoolFunction,
    HostCallArguments, HostFloatArgumentSlot, HostFloatFunction, HostFunctionDefinition,
    HostFunctionImplementation, HostIntArgumentSlot, HostIntFunction, HostNeverFunction,
    HostNilArgumentSlot, HostNilFunction, HostParameter, HostStringArgumentSlot,
    HostStringFunction, HostUtfCodepointArgumentSlot, HostUtfCodepointFunction,
    HostValueFunctionImplementation, HostValueType,
};
pub(crate) use module::{
    RegisteredHostFunction, RegisteredHostImplementationId, RegisteredHostImplementations,
    RegisteredHostModule, RegisteredHostProviderModule,
};
