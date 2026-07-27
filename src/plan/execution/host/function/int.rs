use super::HostedFunction;
use crate::host::{HostCallArguments, HostCallError, HostIntFunction, HostProfile};
use num_bigint::BigInt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct HostIntFunctionId(usize);

pub(crate) type HostedIntFunction<Profile> = HostedFunction<HostIntFunction<Profile>>;

impl HostIntFunctionId {
    pub(in crate::plan::execution) fn new(index: usize) -> Self {
        Self(index)
    }

    pub(crate) fn index(self) -> usize {
        self.0
    }
}

impl<Profile: HostProfile> HostedIntFunction<Profile> {
    pub(crate) fn call(
        &self,
        state: &mut Profile::RunState,
        arguments: &dyn HostCallArguments,
    ) -> Result<BigInt, HostCallError> {
        self.implementation.call(state, arguments)
    }
}
