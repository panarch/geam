use super::HostedFunction;
use crate::host::{HostCallArguments, HostCallError, HostFloatFunction, HostProfile};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct HostFloatFunctionId(usize);

pub(crate) type HostedFloatFunction<Profile> = HostedFunction<HostFloatFunction<Profile>>;

impl HostFloatFunctionId {
    pub(in crate::plan::execution) fn new(index: usize) -> Self {
        Self(index)
    }

    pub(crate) fn index(self) -> usize {
        self.0
    }
}

impl<Profile: HostProfile> HostedFloatFunction<Profile> {
    pub(crate) fn call(
        &self,
        state: &mut Profile::RunState,
        arguments: &dyn HostCallArguments,
    ) -> Result<f64, HostCallError> {
        self.implementation.call(state, arguments)
    }
}
