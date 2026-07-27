use super::HostFunctionTemplate;
use crate::plan::{ModuleId, PlannedModule};
use ecow::EcoString;

pub struct HostedPlannedModule {
    kind: HostedPlannedModuleKind,
}

pub(crate) enum HostedPlannedModuleKind {
    Source(Box<PlannedModule>),
    Host(PlannedHostModule),
}

pub struct PlannedHostModule {
    id: ModuleId,
    package: EcoString,
    module: EcoString,
    functions: Vec<HostFunctionTemplate>,
}

impl HostedPlannedModule {
    pub(crate) fn from_source(module: PlannedModule) -> Self {
        Self {
            kind: HostedPlannedModuleKind::Source(Box::new(module)),
        }
    }

    pub(crate) fn from_host(module: PlannedHostModule) -> Self {
        Self {
            kind: HostedPlannedModuleKind::Host(module),
        }
    }

    pub fn id(&self) -> ModuleId {
        match &self.kind {
            HostedPlannedModuleKind::Source(module) => module.id(),
            HostedPlannedModuleKind::Host(module) => module.id(),
        }
    }

    pub fn package(&self) -> &EcoString {
        match &self.kind {
            HostedPlannedModuleKind::Source(module) => module.package(),
            HostedPlannedModuleKind::Host(module) => module.package(),
        }
    }

    pub fn module(&self) -> &EcoString {
        match &self.kind {
            HostedPlannedModuleKind::Source(module) => module.module(),
            HostedPlannedModuleKind::Host(module) => module.module(),
        }
    }

    pub fn source(&self) -> Option<&PlannedModule> {
        match &self.kind {
            HostedPlannedModuleKind::Source(module) => Some(module),
            HostedPlannedModuleKind::Host(_) => None,
        }
    }

    pub fn host(&self) -> Option<&PlannedHostModule> {
        match &self.kind {
            HostedPlannedModuleKind::Source(_) => None,
            HostedPlannedModuleKind::Host(module) => Some(module),
        }
    }

    pub(crate) fn into_kind(self) -> HostedPlannedModuleKind {
        self.kind
    }
}

impl PlannedHostModule {
    pub(crate) fn new(
        id: ModuleId,
        package: EcoString,
        module: EcoString,
        functions: Vec<HostFunctionTemplate>,
    ) -> Self {
        Self {
            id,
            package,
            module,
            functions,
        }
    }

    pub fn id(&self) -> ModuleId {
        self.id
    }

    pub fn package(&self) -> &EcoString {
        &self.package
    }

    pub fn module(&self) -> &EcoString {
        &self.module
    }

    pub fn functions(&self) -> &[HostFunctionTemplate] {
        &self.functions
    }

    pub(crate) fn into_parts(self) -> (ModuleId, EcoString, EcoString, Vec<HostFunctionTemplate>) {
        (self.id, self.package, self.module, self.functions)
    }
}
