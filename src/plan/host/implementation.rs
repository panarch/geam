use crate::host::{
    HostFunctionImplementation as RegisteredHostFunctionImplementation, HostProfile,
};
use crate::plan::FunctionTemplateId;
use std::sync::Arc;

pub(crate) struct HostImplementationBinding<Profile: HostProfile> {
    template: FunctionTemplateId,
    constructions: crate::host::HostFunctionConstructions,
    implementation: Arc<RegisteredHostFunctionImplementation<Profile>>,
}

impl<Profile: HostProfile> HostImplementationBinding<Profile> {
    pub(crate) fn new(
        template: FunctionTemplateId,
        constructions: crate::host::HostFunctionConstructions,
        implementation: Arc<RegisteredHostFunctionImplementation<Profile>>,
    ) -> Self {
        Self {
            template,
            constructions,
            implementation,
        }
    }

    pub(crate) fn into_parts(
        self,
    ) -> (
        FunctionTemplateId,
        crate::host::HostFunctionConstructions,
        Arc<RegisteredHostFunctionImplementation<Profile>>,
    ) {
        (self.template, self.constructions, self.implementation)
    }
}
