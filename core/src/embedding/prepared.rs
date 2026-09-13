use super::{
    Arguments, BindingError, EmbeddingValue, Function, FunctionDeclaration, HostedModule, Module,
    ReturnValue,
};
use crate::plan::execution::prepared::{
    AdmittedHostedModule, AdmittedModule, HostedModuleArtifact, ModuleArtifact, PreparedError,
};
use std::collections::HashSet;
use std::convert::Infallible;
use std::sync::Arc;

/// Selects typed handles from an admitted, already prepared plain program.
///
/// The artifact contains a non-empty selection made during preparation. Loading
/// validates that program without reading sources or evaluating application code.
pub struct PreparedModuleBindings {
    program: AdmittedModule<'static, Infallible>,
    selected: HashSet<ecow::EcoString>,
    owner: Arc<()>,
}

/// Selects typed handles from a prepared program linked to fresh Rust providers.
pub struct PreparedHostedModuleBindings<Profile: crate::HostProfile> {
    program: AdmittedHostedModule<Profile>,
    selected: HashSet<ecow::EcoString>,
    owner: Arc<()>,
}

impl HostedModuleArtifact {
    /// Checks static data and provider contracts without initializing run state.
    pub fn load<Profile: crate::HostProfile>(
        &'static self,
        providers: crate::HostProviderSet<Profile>,
    ) -> Result<PreparedHostedModuleBindings<Profile>, PreparedError> {
        Ok(PreparedHostedModuleBindings {
            program: self.admit(providers)?,
            selected: HashSet::new(),
            owner: Arc::new(()),
        })
    }
}

impl ModuleArtifact<Infallible> {
    /// Validates this static program and creates an independent binding owner.
    pub fn load(&'static self) -> Result<PreparedModuleBindings, PreparedError> {
        Ok(PreparedModuleBindings {
            program: self.admit()?,
            selected: HashSet::new(),
            owner: Arc::new(()),
        })
    }
}

impl PreparedModuleBindings {
    /// Checks a Rust declaration against a function selected during preparation.
    #[allow(private_bounds)]
    pub fn function<ArgumentsType: Arguments, Return: ReturnValue>(
        &mut self,
        declaration: FunctionDeclaration<ArgumentsType, Return>,
    ) -> Result<Function<ArgumentsType, Return>, PreparedError> {
        let name = declaration.into_name();
        if self.selected.contains(&name) {
            return Err(BindingError::DuplicateFunction { name }.into());
        }
        let mut standard = ArgumentsType::standard_variants();
        standard.extend(Return::standard_variants());
        let slot = self.program.select(
            name.clone(),
            crate::plan::FunctionType::new(ArgumentsType::value_types(), Return::value_type()),
            &ArgumentsType::input_variants(),
            &ArgumentsType::input_lists(),
            &standard,
        )?;
        let function = Function::new(name.clone(), slot, &self.owner);
        self.selected.insert(name);
        Ok(function)
    }

    /// Creates the callable module without planning or rebuilding its static graph.
    pub fn seal(self) -> Module {
        let (execution, entries) = self.program.into_execution();
        Module::from_parts(execution, entries, self.owner)
    }
}

impl<Profile: crate::HostProfile> PreparedHostedModuleBindings<Profile> {
    /// Checks a Rust declaration against a function selected during preparation.
    #[allow(private_bounds)]
    pub fn function<ArgumentsType: Arguments, Return: EmbeddingValue>(
        &mut self,
        declaration: FunctionDeclaration<ArgumentsType, Return>,
    ) -> Result<Function<ArgumentsType, Return>, PreparedError> {
        let name = declaration.into_name();
        if self.selected.contains(&name) {
            return Err(BindingError::DuplicateFunction { name }.into());
        }
        let mut standard = ArgumentsType::standard_variants();
        standard.extend(Return::standard_variants());
        let slot = self.program.program.select(
            name.clone(),
            crate::plan::FunctionType::new(ArgumentsType::value_types(), Return::value_type()),
            &ArgumentsType::input_variants(),
            &ArgumentsType::input_lists(),
            &standard,
        )?;
        let function = Function::new(name.clone(), slot, &self.owner);
        self.selected.insert(name);
        Ok(function)
    }

    /// Creates fresh runtime stores while borrowing the validated static program.
    pub fn seal(self) -> HostedModule<Profile> {
        let (execution, entries) = self.program.into_execution();
        HostedModule {
            execution,
            entries,
            owner: self.owner,
        }
    }
}
