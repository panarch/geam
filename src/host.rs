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
    HostBoolArgumentSlot, HostBoolFunction, HostCallArguments, HostFunctionDefinition,
    HostFunctionImplementation, HostIntArgumentSlot, HostIntFunction, HostParameter, HostValueType,
};
pub(crate) use module::{
    RegisteredHostFunction, RegisteredHostImplementationId, RegisteredHostImplementations,
    RegisteredHostModule, RegisteredHostProviderModule,
};
