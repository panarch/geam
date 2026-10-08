mod callable;
mod component;
mod construction;
mod declaration;
mod error;
mod execution;
mod external;
mod failure;
mod function;
mod future;
mod lifetime;
mod module;
pub mod native;
mod profile;
mod shared_custom;
mod type_;
mod value;

pub use callable::{HostCallableSchema, HostCaptures, ResumableHostCallable, ScopedHostCallable};
pub(crate) use callable::{RegisteredCallableConstruction, RegisteredHostCallable};
pub use component::{
    HostComponentProfile, HostExecutionService, HostProviderComponent,
    HostProviderComponentInitialization, HostProviderComponentRegistration,
    HostProviderConfiguration, HostProviderConfigurationValue, HostProviderInitializationError,
    HostServiceProfile,
};
pub use construction::{HostConstruction, HostConstructions, HostRestoration};
pub use declaration::{
    HostCompletion, HostDeclarations, HostDiverges, HostFunctionDeclaration, HostModuleDeclaration,
    HostProviderModuleDeclaration, HostReturns,
};
pub use error::HostRegistrationError;
pub(crate) use execution::CallableRetention;
pub use execution::{
    HostCallContinuation, HostExecutionContext, HostExecutionError, HostNeverContinuation,
    HostOwnedCallable, HostOwnedCompletion, HostOwnedFunctionValue, SharedExecutionError,
};
pub(crate) use external::{ExternalPayloadLease, ExternalPayloadView, HostStoredValueFamily};
pub use external::{
    HostExternalBinding, HostExternalEquality, HostExternalHashing, HostExternalInspection,
    HostExternalPayloadBuilder, HostExternalPayloadView, HostExternalSchema, HostExternalStorage,
    HostExternalStore, HostExternalType, HostExternalTypeSchema, HostStoredDynamic, HostStoredType,
    HostStoredValue,
};
pub(crate) use external::{RetainedValueEquality, RetainedValueHashing, RetainedValueInspection};
pub(crate) use failure::HostCallErrorKind;
pub use failure::{HostCallError, HostFailure};
pub use function::{
    FallibleHostFunction, HostFunction, HostFunctionSchema, ResumableHostFunction,
    ScopedConstructingHostFunction, ScopedDivergingHostFunction, ScopedHostFunction,
};
pub(crate) use future::{FutureRetention, work_store};
pub use future::{
    HostFutureContext, HostFuturePayload, HostFutureStore, HostFutureType, HostFutureValue,
    HostWorkProfile, HostWorkRepresentation, HostWorkSchema, HostWorkStorage,
};
pub use lifetime::HostValueLifetime;
pub use module::{HostModule, HostProviderModule, HostProviderSet};
pub use profile::{HostCall, HostProfile, HostProvider, StatelessHostProfile};
pub use type_::{
    HostCreatedFunction, HostCustomAccess, HostCustomConstructor, HostCustomConstructorAt,
    HostCustomConstructorDefinition, HostCustomConstructorList, HostCustomConstructorListEnd,
    HostCustomConstructorSchema, HostCustomField, HostCustomFieldList, HostCustomFieldListEnd,
    HostCustomFieldSchema, HostCustomIndex0, HostCustomIndexNext, HostCustomSchema, HostCustomType,
    HostCustomTypeArgument, HostCustomTypeSchema, HostFunctionType, HostFunctionValueType,
    HostListType, HostNominalCustomField, HostRestoredType, HostRetainedCustomSchema,
    HostRetainedCustomType, HostSchemaType, HostTupleType, HostType, HostTypeAt, HostTypeIndex0,
    HostTypeIndexNext, HostTypeList, HostTypeListEnd, HostTypeParameter, HostTypeSequence,
};
#[doc(hidden)]
pub use type_::{
    HostCustomConstructorBranch, HostCustomConstructorLeaf, HostCustomIndexHere,
    HostCustomIndexLeft, HostCustomIndexRight, HostRetainedType, HostRetainedValue,
};
pub use value::{
    HostCallCompletion, HostCallable, HostCustom, HostExternal, HostFunctionValue, HostList,
    HostTuple, HostValue,
};

#[cfg(test)]
pub(crate) use external::{ExternalTestProfile, ExternalTestRunState, ExternalTestStores};
#[cfg(test)]
pub(crate) use function::CallArguments;
pub(crate) use function::HostCallReturn;
#[cfg(test)]
pub(crate) use function::HostParameterLayout;
pub(crate) use function::RegisteredHostConstructions;
pub(crate) use function::{
    HostBitArrayArgumentSlot, HostBoolArgumentSlot, HostCallArguments, HostCustomArgumentSlot,
    HostExternalArgumentSlot, HostFloatArgumentSlot, HostFunctionArgumentSlot, HostFunctionBinding,
    HostFunctionDefinition, HostFunctionImplementation, HostIntArgumentSlot, HostListArgumentSlot,
    HostNativeViewFactory, HostNeverFunction, HostNilArgumentSlot, HostParameter,
    HostRetainedCallback, HostStringArgumentSlot, HostTupleArgumentSlot,
    HostUtfCodepointArgumentSlot, HostValueArgumentSlot, HostValueFunction,
};
pub(crate) use function::{NativeViewBinding, NativeViewImplementation};
#[cfg(test)]
pub(crate) use function::{
    expect_immediate_call, expect_never_implementation, expect_value_implementation,
};
pub(crate) use module::{
    RegisteredHostBindings, RegisteredHostFunction, RegisteredHostImplementationId,
    RegisteredHostImplementations, RegisteredHostModule, RegisteredHostProviderModule,
};
pub(crate) use profile::HostCodecScope;
#[cfg(test)]
pub(crate) use profile::test;
pub(crate) use profile::{HostCallRuntime, HostTokenRuntime};
pub(crate) use type_::retained::RetainedAbi;
pub(crate) use type_::{
    HostAbiType, HostAbiTypeSequence, HostOpaqueFunctionType, HostTypeDescriptor,
    construction_callable_count,
};
pub(crate) use value::{
    HostCustomToken, HostExternalToken, HostFunctionToken, HostFunctionValueToken, HostListToken,
    HostScopedValue, HostTupleToken, HostValueFamily, HostValueToken,
};
