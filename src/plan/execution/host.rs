mod function;
mod table;
use crate::host::HostProfile;
use std::marker::PhantomData;

pub(crate) use function::{
    HostBitArrayFunctionId, HostBoolFunctionId, HostFloatFunctionId, HostIntFunctionId,
    HostNilFunctionId, HostStringFunctionId, HostUtfCodepointFunctionId, HostedBitArrayFunction,
    HostedBoolFunction, HostedFloatFunction, HostedFunction, HostedFunctionMetadata,
    HostedIntFunction, HostedNilFunction, HostedStringFunction, HostedUtfCodepointFunction,
};
pub(crate) use table::HostFunctionTables;

pub(crate) struct HostedExecutionProfile<Profile: HostProfile>(PhantomData<Profile>);
