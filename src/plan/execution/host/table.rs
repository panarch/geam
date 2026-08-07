use super::{HostFunctionId, HostNeverFunctionId, HostedNeverFunction, HostedValueFunction};
use crate::host::HostProfile;
use crate::plan::execution::function::ExecutionFunctionBody;

pub(crate) struct HostFunctionTables<Profile: HostProfile> {
    value_functions: Box<[HostedValueFunction<Profile>]>,
    never_functions: Box<[HostedNeverFunction<Profile>]>,
}

impl<Profile: HostProfile> HostFunctionTables<Profile> {
    pub(in crate::plan::execution) fn new(
        value_functions: Box<[HostedValueFunction<Profile>]>,
        never_functions: Box<[HostedNeverFunction<Profile>]>,
    ) -> Self {
        Self {
            value_functions,
            never_functions,
        }
    }

    pub(crate) fn value<Body: ExecutionFunctionBody>(
        &self,
        id: &HostFunctionId<Body>,
    ) -> &HostedValueFunction<Profile> {
        &self.value_functions[id.index()]
    }

    pub(crate) fn never(&self, id: HostNeverFunctionId) -> &HostedNeverFunction<Profile> {
        &self.never_functions[id.index()]
    }

    #[cfg(test)]
    pub(in crate::plan::execution) fn value_functions(&self) -> &[HostedValueFunction<Profile>] {
        &self.value_functions
    }

    #[cfg(test)]
    pub(in crate::plan::execution) fn never_functions(&self) -> &[HostedNeverFunction<Profile>] {
        &self.never_functions
    }
}
