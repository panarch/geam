mod error;
mod function;
mod table;
use crate::host::HostProfile;
use std::marker::PhantomData;

pub(crate) use function::{
    HostCallParameter, HostFunctionId, HostNeverFunctionId, HostedFunction, HostedFunctionMetadata,
    HostedFunctionTarget, HostedNeverFunction, HostedValueFunction,
};
pub(crate) use table::HostFunctionTables;

pub(crate) struct HostedExecutionProfile<Profile: HostProfile>(PhantomData<Profile>);
pub use error::{HostSpecializationError, HostSpecializationErrorReason};
