use crate::host::HostFunctionImplementation as RegisteredHostFunctionImplementation;
use crate::plan::FunctionTemplateId;

pub(crate) struct HostFunctionImplementation {
    template: FunctionTemplateId,
    implementation: RegisteredHostFunctionImplementation,
}

impl HostFunctionImplementation {
    pub(crate) fn new(
        template: FunctionTemplateId,
        implementation: RegisteredHostFunctionImplementation,
    ) -> Self {
        Self {
            template,
            implementation,
        }
    }

    pub(crate) fn into_parts(self) -> (FunctionTemplateId, RegisteredHostFunctionImplementation) {
        (self.template, self.implementation)
    }
}
