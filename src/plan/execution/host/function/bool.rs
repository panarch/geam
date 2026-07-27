use super::HostedFunction;
use crate::host::{HostBoolFunction, HostCallArguments, HostCallError, HostProfile};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct HostBoolFunctionId(usize);

pub(crate) type HostedBoolFunction<Profile> = HostedFunction<HostBoolFunction<Profile>>;

impl HostBoolFunctionId {
    pub(in crate::plan::execution) fn new(index: usize) -> Self {
        Self(index)
    }

    pub(crate) fn index(self) -> usize {
        self.0
    }
}

impl<Profile: HostProfile> HostedBoolFunction<Profile> {
    pub(crate) fn call(
        &self,
        state: &mut Profile::RunState,
        arguments: &dyn HostCallArguments,
    ) -> Result<bool, HostCallError> {
        self.implementation.call(state, arguments)
    }
}
