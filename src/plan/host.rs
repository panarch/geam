mod function;
mod implementation;
mod module;

use super::{FunctionTemplateId, ModuleId};

pub use function::HostFunctionTemplate;
pub(crate) use function::{HostParameter, HostReturnFamily};
pub(crate) use implementation::HostFunctionImplementation;
pub(crate) use module::HostedPlannedModuleKind;
pub use module::{HostedPlannedModule, PlannedHostModule};

pub struct HostedModulePlan {
    root: ModuleId,
    entry: FunctionTemplateId,
    modules: Vec<HostedPlannedModule>,
    implementations: Vec<HostFunctionImplementation>,
}

pub(crate) struct HostedModulePlanParts {
    pub(crate) root: ModuleId,
    pub(crate) entry: FunctionTemplateId,
    pub(crate) modules: Vec<HostedPlannedModule>,
    pub(crate) implementations: Vec<HostFunctionImplementation>,
}

impl HostedModulePlan {
    pub(crate) fn new(
        root: ModuleId,
        entry: FunctionTemplateId,
        modules: Vec<HostedPlannedModule>,
        implementations: Vec<HostFunctionImplementation>,
    ) -> Self {
        Self {
            root,
            entry,
            modules,
            implementations,
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

    pub(crate) fn into_parts(self) -> HostedModulePlanParts {
        HostedModulePlanParts {
            root: self.root,
            entry: self.entry,
            modules: self.modules,
            implementations: self.implementations,
        }
    }
}
