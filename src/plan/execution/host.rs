mod function;
mod table;

pub(crate) use function::{
    HostBoolFunctionId, HostIntFunctionId, HostedBoolFunction, HostedFunction, HostedIntFunction,
};
pub(crate) use table::HostFunctionTables;

pub(crate) struct HostedExecutionHost;
