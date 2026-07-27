use super::HostedFunction;
use crate::host::{HostCallArguments, HostCallError, HostNilFunction, HostProfile};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct HostNilFunctionId(usize);

pub(crate) type HostedNilFunction<Profile> = HostedFunction<HostNilFunction<Profile>>;

impl HostNilFunctionId {
    pub(in crate::plan::execution) fn new(index: usize) -> Self {
        Self(index)
    }

    pub(crate) fn index(self) -> usize {
        self.0
    }
}

impl<Profile: HostProfile> HostedNilFunction<Profile> {
    pub(crate) fn call(
        &self,
        state: &mut Profile::RunState,
        arguments: &dyn HostCallArguments,
    ) -> Result<(), HostCallError> {
        self.implementation.call(state, arguments)
    }
}
