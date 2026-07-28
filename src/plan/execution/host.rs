mod function;
mod table;
use crate::host::HostProfile;
use std::marker::PhantomData;

pub(crate) use function::{
    HostBitArrayFunctionId, HostBoolFunctionId, HostFloatFunctionId, HostIntFunctionId,
    HostNeverFunctionId, HostNilFunctionId, HostStringFunctionId, HostUtfCodepointFunctionId,
    HostedBitArrayFunction, HostedBoolFunction, HostedFloatFunction, HostedFunction,
    HostedFunctionMetadata, HostedFunctionTarget, HostedIntFunction, HostedNeverFunction,
    HostedNilFunction, HostedStringFunction, HostedUtfCodepointFunction,
};
pub(in crate::plan::execution) use table::HostValueFunctionTables;
pub(crate) use table::{HostFunctionTables, HostValueFunctionTarget};

pub(crate) struct HostedExecutionProfile<Profile: HostProfile>(PhantomData<Profile>);
