mod callable;
mod function;
mod implementation;
mod module;

use super::{FunctionTemplateId, ModuleId};
use crate::host::HostProfile;

pub(crate) use callable::instantiate_native_callable;
pub use function::HostFunctionTemplate;
pub(crate) use implementation::{HostImplementationBinding, ProfiledHostImplementationBinding};
pub(crate) use module::HostedPlannedModuleParts;
pub use module::{HostedFunctionTemplate, HostedPlannedModule};

pub struct HostedModulePlan<Profile: HostProfile> {
    root: ModuleId,
    entry: FunctionTemplateId,
    modules: Vec<HostedPlannedModule>,
    implementation_bindings: Vec<HostImplementationBinding<Profile>>,
}

pub(crate) struct ProfiledHostedLibraryModulePlan<Implementation> {
    root: ModuleId,
    modules: Vec<HostedPlannedModule>,
    implementation_bindings: Vec<implementation::ProfiledHostImplementationBinding<Implementation>>,
    callables: Vec<LibraryNativeCallable>,
}

pub(crate) struct LibraryNativeCallable {
    pub(crate) template: HostFunctionTemplate,
    pub(crate) target: super::FunctionInstantiation,
    pub(crate) declaration: crate::host::RegisteredCallableConstruction,
    pub(crate) signature: super::LibraryNativeSignature,
}

pub(crate) type HostedLibraryModulePlan<Profile> =
    ProfiledHostedLibraryModulePlan<crate::host::HostFunctionImplementation<Profile>>;

pub(crate) struct HostedModulePlanParts<Profile: HostProfile> {
    pub(crate) root: ModuleId,
    pub(crate) entry: FunctionTemplateId,
    pub(crate) modules: Vec<HostedPlannedModule>,
    pub(crate) implementation_bindings: Vec<HostImplementationBinding<Profile>>,
}

pub(crate) struct ProfiledHostedLibraryModulePlanParts<Implementation> {
    pub(crate) root: ModuleId,
    pub(crate) modules: Vec<HostedPlannedModule>,
    pub(crate) implementation_bindings:
        Vec<implementation::ProfiledHostImplementationBinding<Implementation>>,
    pub(crate) callables: Vec<LibraryNativeCallable>,
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

impl<Implementation> ProfiledHostedLibraryModulePlan<Implementation> {
    pub(crate) fn new(
        root: ModuleId,
        modules: Vec<HostedPlannedModule>,
        implementation_bindings: Vec<
            implementation::ProfiledHostImplementationBinding<Implementation>,
        >,
    ) -> Self {
        Self {
            root,
            modules,
            implementation_bindings,
            callables: Vec::new(),
        }
    }

    pub(crate) fn functions(&self) -> &[HostedFunctionTemplate] {
        self.modules[self.root.index()].functions()
    }

    pub(crate) fn custom_type(
        &self,
        name: &super::CustomTypeName,
    ) -> Option<&super::CustomTypeDefinition> {
        self.modules
            .iter()
            .flat_map(HostedPlannedModule::custom_types)
            .find(|definition| definition.name() == name)
    }

    pub(crate) fn into_parts(self) -> ProfiledHostedLibraryModulePlanParts<Implementation> {
        ProfiledHostedLibraryModulePlanParts {
            root: self.root,
            modules: self.modules,
            implementation_bindings: self.implementation_bindings,
            callables: self.callables,
        }
    }
}

impl<Value, Never> ProfiledHostedLibraryModulePlan<crate::host::HostFunctionBinding<Value, Never>> {
    pub(crate) fn callable(
        &mut self,
        declaration: crate::host::RegisteredCallableConstruction,
        signature: super::LibraryNativeSignature,
    ) -> Result<usize, crate::embedding::BindingError> {
        let identity = &declaration.identity;
        let failure = || crate::embedding::BindingError::NativeCallable {
            package: identity.package.clone(),
            module: identity.module.clone(),
            name: identity.name.clone(),
        };
        let template = self
            .modules
            .iter()
            .find(|module| {
                module.package() == &identity.package && module.module() == &identity.module
            })
            .and_then(|module| {
                module
                    .native_callables()
                    .iter()
                    .find(|template| template.name() == identity.name.as_str())
            })
            .ok_or_else(failure)?;
        let target = instantiate_native_callable(template, &declaration).ok_or_else(failure)?;
        let returns_value = matches!(
            declaration.completion,
            crate::host::HostFunctionBinding::Value(())
        );
        if !self.implementation_bindings.iter().any(|binding| {
            binding.template() == template.id() && binding.returns_value() == returns_value
        }) {
            return Err(failure());
        }
        let slot = self.callables.len();
        self.callables.push(LibraryNativeCallable {
            template: template.clone(),
            target,
            declaration,
            signature,
        });
        Ok(slot)
    }
}
