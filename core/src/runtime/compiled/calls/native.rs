use super::{CallExecution, CallProgress, CallValues};
use crate::StringValue;
use crate::plan::HostCallSite;
use crate::plan::execution::function::StringFunctionId;
use crate::runtime::captures::ExecutionDomain;
use crate::runtime::compiled::bit_array::BitArrayValues;
use crate::runtime::compiled::numeric::NumericValues;
use crate::runtime::compiled::string::StringValues;

/// Only groups that own a synchronous String request implement this phase.
/// Delivery moves the same engine, without a per-call boxed resume closure.
pub trait StringNativeExecution: CallExecution {
    fn resume_native(self: Box<Self>, value: StringValue) -> Box<dyn CallExecution>;
}

pub struct StringNativeRequest {
    pub function: StringFunctionId,
    pub site: HostCallSite,
    pub arguments: Box<CallValues>,
    pub root_tail: bool,
    pub execution: Box<dyn StringNativeExecution>,
}

pub(in crate::runtime) enum GeneratedNativePhase {
    Progress(CallProgress),
    Invoke {
        request: StringNativeRequest,
        before: usize,
    },
    Deliver {
        value: StringValue,
        execution: Box<dyn StringNativeExecution>,
        charge: bool,
    },
}

/// Scratch moves to the service once, and returns to its original worker.
/// The frame and typed destination never enter this owned service payload.
pub(in crate::runtime) struct GeneratedNativeState {
    pub phase: GeneratedNativePhase,
    pub numeric: NumericValues,
    pub strings: Option<Box<StringValues>>,
    pub bit_arrays: Option<Box<BitArrayValues>>,
    pub root_tail_entry: bool,
    pub domain: ExecutionDomain,
    pub prepaid_completion: bool,
}
