use crate::host::{
    HostFunctionImplementation as RegisteredHostFunctionImplementation, HostProfile,
};
use crate::plan::FunctionTemplateId;

pub(crate) struct HostFunctionImplementation<Profile: HostProfile> {
    template: FunctionTemplateId,
    implementation: RegisteredHostFunctionImplementation<Profile>,
}

impl<Profile: HostProfile> HostFunctionImplementation<Profile> {
    pub(crate) fn new(
        template: FunctionTemplateId,
        implementation: RegisteredHostFunctionImplementation<Profile>,
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
        RegisteredHostFunctionImplementation<Profile>,
    ) {
        (self.template, self.implementation)
    }
}
