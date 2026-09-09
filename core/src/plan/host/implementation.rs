use crate::plan::FunctionTemplateId;
use std::sync::Arc;

pub(crate) struct ProfiledHostImplementationBinding<Implementation> {
    template: FunctionTemplateId,
    constructions: crate::host::RegisteredHostConstructions,
    implementation: Arc<Implementation>,
}

pub(crate) type HostImplementationBinding<Profile> =
    ProfiledHostImplementationBinding<crate::host::HostFunctionImplementation<Profile>>;

impl<Implementation> ProfiledHostImplementationBinding<Implementation> {
    pub(crate) fn new(
        template: FunctionTemplateId,
        constructions: crate::host::RegisteredHostConstructions,
        implementation: Arc<Implementation>,
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
        crate::host::RegisteredHostConstructions,
        Arc<Implementation>,
    ) {
        (self.template, self.constructions, self.implementation)
    }
}
