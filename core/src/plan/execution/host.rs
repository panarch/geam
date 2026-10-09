pub(in crate::plan::execution) mod construction;
mod error;
pub(in crate::plan::execution) mod function;
pub(in crate::plan::execution) mod native;
pub(in crate::plan::execution) mod registration;
mod synchronous;
mod table;

pub(crate) use function::HostConstructionTypes;
pub(in crate::plan::execution) use function::HostTypeArgument;
pub(in crate::plan::execution) use function::HostedFunctionParameters;
pub(crate) use function::{
    HostCallParameter, HostCallableConstruction, HostCallableEntry, HostFunctionCompletion,
    HostFunctionId, HostNeverFunctionId, HostedFunction, HostedFunctionMetadata,
    HostedFunctionTarget, HostedNeverFunction, HostedValueFunction,
};
pub(crate) use native::{
    HostNativeView, NativeConstructor, NativeConversion, NativeConversionId, NativeConversionKind,
    NativeConversions, NativeCustomView, NativeFunctionView,
};
pub(in crate::plan::execution) use registration::{CallableRegistration, RegistrationContract};
pub(crate) use synchronous::{SynchronousStringBinding, SynchronousStringFunctions};
pub(crate) use table::{HostBindingTables, HostFunctionTables};

pub struct HostedExecutionProfile;

pub use error::{HostSpecializationError, HostSpecializationErrorReason};
