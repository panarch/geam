mod error;
mod function;
mod table;
mod transfer;

pub(crate) use function::HostConstructionTypes;
pub(in crate::plan::execution) use function::HostedFunctionParameters;
pub(crate) use function::{
    HostCallParameter, HostFunctionId, HostNeverFunctionId, HostedFunction, HostedFunctionMetadata,
    HostedFunctionTarget, HostedNeverFunction, HostedValueFunction,
};
pub(crate) use table::HostFunctionTables;
pub(crate) use transfer::{
    TransferHostFunctionTableBuilder, TransferHostFunctionTables, TransferHostedNeverFunction,
    TransferHostedValueFunction, async_host_failure, call_transfer_never, call_transfer_value,
};

pub(crate) struct HostedExecutionProfile;
pub(crate) struct TransferHostedExecutionProfile;
pub use error::{HostSpecializationError, HostSpecializationErrorReason};
