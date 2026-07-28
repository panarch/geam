mod error;
mod failure;
mod function;
mod module;
mod profile;
mod type_;
mod value;

pub use error::HostRegistrationError;
pub use failure::{HostCallError, HostFailure};
pub use function::{FallibleHostFunction, HostFunction, HostFunctionSchema, ScopedHostFunction};
pub use module::{HostModule, HostProviderModule, HostProviderSet};
pub use profile::{HostCall, HostProfile, HostProvider, StatelessHostProfile};
pub use type_::{
    HostCustomConstructor, HostCustomConstructorAt, HostCustomConstructorDefinition,
    HostCustomConstructorList, HostCustomConstructorListEnd, HostCustomConstructorSchema,
    HostCustomField, HostCustomFieldList, HostCustomFieldListEnd, HostCustomFieldSchema,
    HostCustomIndex0, HostCustomIndexNext, HostCustomSchema, HostCustomType, HostCustomTypeSchema,
    HostListType, HostSchemaType, HostTupleType, HostType, HostTypeList, HostTypeListEnd,
    HostTypeParameter, HostTypeSequence,
};
pub use value::{HostCallCompletion, HostCustom, HostList, HostTuple, HostValue};

#[cfg(test)]
pub(crate) use function::expect_value_implementation;
pub(crate) use function::{
    HostBitArrayArgumentSlot, HostBoolArgumentSlot, HostCallArguments, HostCustomArgumentSlot,
    HostFloatArgumentSlot, HostFunctionDefinition, HostFunctionImplementation, HostIntArgumentSlot,
    HostListArgumentSlot, HostNeverFunction, HostNilArgumentSlot, HostParameter,
    HostStringArgumentSlot, HostTupleArgumentSlot, HostUtfCodepointArgumentSlot,
    HostValueArgumentSlot, HostValueFunction,
};
pub(crate) use module::{
    RegisteredHostFunction, RegisteredHostImplementationId, RegisteredHostImplementations,
    RegisteredHostModule, RegisteredHostProviderModule,
};
pub(crate) use profile::HostCallRuntime;
#[cfg(test)]
pub(crate) use profile::test;
pub(crate) use type_::{HostAbiType, HostAbiTypeSequence, HostTypeDescriptor};
pub(crate) use value::{
    HostCustomToken, HostListToken, HostScopedValue, HostTupleToken, HostValueFamily,
    HostValueToken,
};
