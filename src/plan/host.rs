use super::{
    FunctionShape, FunctionTemplateId, FunctionTemplateSignature, FunctionType, ModuleId,
    PlannedModule, TypeScheme, ValueShape, ValueType,
};
use crate::host::HostIntFunction;
use ecow::EcoString;

pub struct HostedModulePlan {
    root: ModuleId,
    entry: FunctionTemplateId,
    modules: Vec<HostedPlannedModule>,
    implementations: Vec<HostFunctionImplementation>,
}

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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HostFunctionTemplate {
    signature: FunctionTemplateSignature,
    package: EcoString,
    module: EcoString,
    name: EcoString,
    type_: FunctionType,
}

pub(crate) struct HostFunctionImplementation {
    template: FunctionTemplateId,
    function: HostIntFunction,
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
}

impl HostFunctionTemplate {
    pub(crate) fn int_binary(
        id: FunctionTemplateId,
        package: EcoString,
        module: EcoString,
        name: EcoString,
    ) -> Self {
        Self {
            signature: FunctionTemplateSignature::new(
                id,
                TypeScheme::new(0),
                FunctionShape::new(vec![ValueShape::Int, ValueShape::Int], ValueShape::Int),
            ),
            package,
            module,
            name,
            type_: FunctionType::new(vec![ValueType::Int, ValueType::Int], ValueType::Int),
        }
    }

    pub fn id(&self) -> FunctionTemplateId {
        self.signature.id()
    }

    pub fn package(&self) -> &EcoString {
        &self.package
    }

    pub fn module(&self) -> &EcoString {
        &self.module
    }

    pub fn name(&self) -> &EcoString {
        &self.name
    }

    pub fn scheme(&self) -> &TypeScheme {
        self.signature.scheme()
    }

    pub fn type_(&self) -> &FunctionType {
        &self.type_
    }

    pub(crate) fn signature(&self) -> &FunctionTemplateSignature {
        &self.signature
    }
}

impl HostFunctionImplementation {
    pub(crate) fn new(template: FunctionTemplateId, function: HostIntFunction) -> Self {
        Self { template, function }
    }

    pub(crate) fn into_parts(self) -> (FunctionTemplateId, HostIntFunction) {
        (self.template, self.function)
    }
}
