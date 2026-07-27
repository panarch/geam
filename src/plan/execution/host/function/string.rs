use super::HostedFunction;
use crate::host::{HostCallArguments, HostCallError, HostProfile, HostStringFunction};
use ecow::EcoString;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct HostStringFunctionId(usize);

pub(crate) type HostedStringFunction<Profile> = HostedFunction<HostStringFunction<Profile>>;

impl HostStringFunctionId {
    pub(in crate::plan::execution) fn new(index: usize) -> Self {
        Self(index)
    }

    pub(crate) fn index(self) -> usize {
        self.0
    }
}

impl<Profile: HostProfile> HostedStringFunction<Profile> {
    pub(crate) fn call(
        &self,
        state: &mut Profile::RunState,
        arguments: &dyn HostCallArguments,
    ) -> Result<EcoString, HostCallError> {
        self.implementation.call(state, arguments)
    }
}
