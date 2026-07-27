mod function;
mod table;
use crate::host::HostProfile;
use std::marker::PhantomData;

pub(crate) use function::{
    HostBoolFunctionId, HostIntFunctionId, HostedBoolFunction, HostedFunction,
    HostedFunctionMetadata, HostedIntFunction,
};
pub(crate) use table::HostFunctionTables;

pub(crate) struct HostedExecutionHost<Profile: HostProfile>(PhantomData<Profile>);
