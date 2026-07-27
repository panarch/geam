use super::HostedFunction;
use crate::host::{HostCallArguments, HostCallError, HostProfile, HostUtfCodepointFunction};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct HostUtfCodepointFunctionId(usize);

pub(crate) type HostedUtfCodepointFunction<Profile> =
    HostedFunction<HostUtfCodepointFunction<Profile>>;

impl HostUtfCodepointFunctionId {
    pub(in crate::plan::execution) fn new(index: usize) -> Self {
        Self(index)
    }

    pub(crate) fn index(self) -> usize {
        self.0
    }
}

impl<Profile: HostProfile> HostedUtfCodepointFunction<Profile> {
    pub(crate) fn call(
        &self,
        state: &mut Profile::RunState,
        arguments: &dyn HostCallArguments,
    ) -> Result<char, HostCallError> {
        self.implementation.call(state, arguments)
    }
}
