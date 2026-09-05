mod async_table;
mod error;
mod function;
mod table;

pub(crate) use async_table::{
    AsyncHostFunctionIndex, AsyncHostFunctionTableBuilder, AsyncHostFunctionTables,
    ImmediateHostFunctionIndex, ResumableHostCallback, async_host_failure, call_resumable_never,
};
pub(crate) use function::HostConstructionTypes;
pub(in crate::plan::execution) use function::HostedFunctionParameters;
pub(crate) use function::{
    HostCallParameter, HostFunctionId, HostNeverFunctionId, HostedFunction, HostedFunctionMetadata,
    HostedFunctionTarget, HostedNeverFunction, HostedValueFunction, ResumableHostedFunctionTarget,
};
pub(crate) use table::HostFunctionTables;

pub(crate) struct HostedExecutionProfile;
pub(crate) struct AsyncHostedExecutionProfile;
pub use error::{HostSpecializationError, HostSpecializationErrorReason};
