mod error;
mod external;
mod failure;
mod function;
mod module;
mod profile;
mod type_;
mod value;

pub use error::HostRegistrationError;
pub(crate) use external::ExternalPayloadLease;
pub use external::{
    HostExternalEquality, HostExternalHashing, HostExternalInspection, HostExternalPayloadBuilder,
    HostExternalPayloadView, HostExternalSchema, HostExternalStorage, HostExternalStore,
    HostExternalType, HostExternalTypeSchema, HostStoredDynamic, HostStoredType, HostStoredValue,
};
pub(crate) use failure::HostCallErrorKind;
pub use failure::{HostCallError, HostFailure};
pub use function::{
    FallibleHostFunction, HostFunction, HostFunctionSchema, ScopedDivergingHostFunction,
    ScopedHostFunction,
};
pub use module::{HostModule, HostProviderModule, HostProviderSet};
pub use profile::{HostCall, HostProfile, HostProvider, StatelessHostProfile};
pub use type_::{
    HostCustomConstructor, HostCustomConstructorAt, HostCustomConstructorDefinition,
    HostCustomConstructorList, HostCustomConstructorListEnd, HostCustomConstructorSchema,
    HostCustomField, HostCustomFieldList, HostCustomFieldListEnd, HostCustomFieldSchema,
    HostCustomIndex0, HostCustomIndexNext, HostCustomSchema, HostCustomType, HostCustomTypeSchema,
    HostFunctionType, HostListType, HostSchemaType, HostTupleType, HostType, HostTypeAt,
    HostTypeIndex0, HostTypeIndexNext, HostTypeList, HostTypeListEnd, HostTypeParameter,
    HostTypeSequence,
};
pub use value::{
    HostCallCompletion, HostCallable, HostCustom, HostExternal, HostList, HostTuple, HostValue,
};

#[cfg(test)]
pub(crate) use external::{ExternalTestProfile, ExternalTestRunState, ExternalTestStores};
pub(crate) use function::{
    HostBitArrayArgumentSlot, HostBoolArgumentSlot, HostCallArguments, HostCustomArgumentSlot,
    HostExternalArgumentSlot, HostFloatArgumentSlot, HostFunctionArgumentSlot,
    HostFunctionDefinition, HostFunctionImplementation, HostIntArgumentSlot, HostListArgumentSlot,
    HostNeverFunction, HostNilArgumentSlot, HostParameter, HostStringArgumentSlot,
    HostTupleArgumentSlot, HostUtfCodepointArgumentSlot, HostValueArgumentSlot, HostValueFunction,
};
#[cfg(test)]
pub(crate) use function::{expect_never_implementation, expect_value_implementation};
pub(crate) use module::{
    RegisteredHostFunction, RegisteredHostImplementationId, RegisteredHostImplementations,
    RegisteredHostModule, RegisteredHostProviderModule,
};
pub(crate) use profile::HostCallRuntime;
#[cfg(test)]
pub(crate) use profile::test;
pub(crate) use type_::{HostAbiType, HostAbiTypeSequence, HostTypeDescriptor};
pub(crate) use value::{
    HostCustomToken, HostExternalToken, HostFunctionToken, HostListToken, HostScopedValue,
    HostTupleToken, HostValueFamily, HostValueToken,
};
