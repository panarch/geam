mod error;
mod function;
mod table;

pub(in crate::plan::execution) use function::HostedFunctionParameters;
pub(crate) use function::{
    HostCallParameter, HostFunctionId, HostNeverFunctionId, HostedFunction, HostedFunctionMetadata,
    HostedFunctionTarget, HostedNeverFunction, HostedValueFunction,
};
pub(crate) use table::HostFunctionTables;

pub(crate) struct HostedExecutionProfile;
pub use error::{HostSpecializationError, HostSpecializationErrorReason};
