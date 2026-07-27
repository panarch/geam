use super::HostedFunction;
use crate::BitArrayValue;
use crate::host::{HostBitArrayFunction, HostCallArguments, HostCallError, HostProfile};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct HostBitArrayFunctionId(usize);

pub(crate) type HostedBitArrayFunction<Profile> = HostedFunction<HostBitArrayFunction<Profile>>;

impl HostBitArrayFunctionId {
    pub(in crate::plan::execution) fn new(index: usize) -> Self {
        Self(index)
    }

    pub(crate) fn index(self) -> usize {
        self.0
    }
}

impl<Profile: HostProfile> HostedBitArrayFunction<Profile> {
    pub(crate) fn call(
        &self,
        state: &mut Profile::RunState,
        arguments: &dyn HostCallArguments,
    ) -> Result<BitArrayValue, HostCallError> {
        self.implementation.call(state, arguments)
    }
}
