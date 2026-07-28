use crate::host::{
    HostFunctionImplementation as RegisteredHostFunctionImplementation, HostProfile,
};
use crate::plan::FunctionTemplateId;
use std::sync::Arc;

pub(crate) struct HostImplementationBinding<Profile: HostProfile> {
    template: FunctionTemplateId,
    implementation: Arc<RegisteredHostFunctionImplementation<Profile>>,
}

impl<Profile: HostProfile> HostImplementationBinding<Profile> {
    pub(crate) fn new(
        template: FunctionTemplateId,
        implementation: Arc<RegisteredHostFunctionImplementation<Profile>>,
    ) -> Self {
        Self {
            template,
            implementation,
        }
    }

    pub(crate) fn into_parts(
        self,
    ) -> (
        FunctionTemplateId,
        Arc<RegisteredHostFunctionImplementation<Profile>>,
    ) {
        (self.template, self.implementation)
    }
}
