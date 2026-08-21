mod function;
mod implementation;
mod module;

use super::{FunctionTemplateId, ModuleId};
use crate::host::HostProfile;

pub use function::HostFunctionTemplate;
pub(crate) use implementation::HostImplementationBinding;
pub(crate) use module::HostedPlannedModuleParts;
pub use module::{HostedFunctionTemplate, HostedPlannedModule};

pub struct HostedModulePlan<Profile: HostProfile> {
    root: ModuleId,
    entry: FunctionTemplateId,
    modules: Vec<HostedPlannedModule>,
    implementation_bindings: Vec<HostImplementationBinding<Profile>>,
}

pub(crate) struct HostedModulePlanParts<Profile: HostProfile> {
    pub(crate) root: ModuleId,
    pub(crate) entry: FunctionTemplateId,
    pub(crate) modules: Vec<HostedPlannedModule>,
    pub(crate) implementation_bindings: Vec<HostImplementationBinding<Profile>>,
}

impl<Profile: HostProfile> HostedModulePlan<Profile> {
    pub(crate) fn new(
        root: ModuleId,
        entry: FunctionTemplateId,
        modules: Vec<HostedPlannedModule>,
        implementation_bindings: Vec<HostImplementationBinding<Profile>>,
    ) -> Self {
        Self {
            root,
            entry,
            modules,
            implementation_bindings,
        }
    }

    pub fn root(&self) -> ModuleId {
        self.root
    }

    pub fn entry(&self) -> FunctionTemplateId {
        self.entry
    }

    pub fn modules(&self) -> &[HostedPlannedModule] {
        &self.modules
    }

    pub(crate) fn into_parts(self) -> HostedModulePlanParts<Profile> {
        HostedModulePlanParts {
            root: self.root,
            entry: self.entry,
            modules: self.modules,
            implementation_bindings: self.implementation_bindings,
        }
    }
}
