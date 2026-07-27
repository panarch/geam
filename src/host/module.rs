use super::{HostFunction, HostFunctionDefinition, HostFunctionSchema, HostRegistrationError};
use ecow::EcoString;
use gleam_core::analyse::name::check_name_case;
use gleam_core::ast::SrcSpan;
use gleam_core::parse::lexer::string_to_keyword;
use gleam_core::type_::PRELUDE_MODULE_NAME;
use gleam_core::type_::error::Named;
use std::collections::BTreeMap;

pub struct HostModule {
    package: EcoString,
    module: EcoString,
    functions: Vec<HostFunctionDefinition>,
}

pub struct HostModules {
    modules: Vec<HostModule>,
}

pub(crate) struct RegisteredHostModule {
    pub(crate) package: EcoString,
    pub(crate) module: EcoString,
    pub(crate) functions: Vec<HostFunctionDefinition>,
}

impl HostModule {
    pub fn new(
        package: impl Into<EcoString>,
        module: impl Into<EcoString>,
    ) -> Result<Self, HostRegistrationError> {
        Self::from_names(package.into(), module.into())
    }

    fn from_names(package: EcoString, module: EcoString) -> Result<Self, HostRegistrationError> {
        if !valid_module_name(&module) {
            return Err(HostRegistrationError::InvalidModuleName { module });
        }

        Ok(Self {
            package,
            module,
            functions: Vec::new(),
        })
    }

    pub fn with_function<Arguments, Return, Function>(
        self,
        name: impl Into<EcoString>,
        function: Function,
    ) -> Result<Self, HostRegistrationError>
    where
        Function: HostFunction<Arguments, Return>,
    {
        self.with_definition(HostFunctionDefinition::new(name.into(), function))
    }

    fn with_definition(
        mut self,
        function: HostFunctionDefinition,
    ) -> Result<Self, HostRegistrationError> {
        let name = function.schema().name().clone();
        if string_to_keyword(&name).is_some()
            || check_name_case(SrcSpan::new(0, 0), &name, Named::Function).is_err()
        {
            return Err(HostRegistrationError::InvalidFunctionName {
                module: self.module,
                function: name,
            });
        }
        if self
            .functions
            .iter()
            .any(|function| function.schema().name() == &name)
        {
            return Err(HostRegistrationError::DuplicateFunction {
                module: self.module,
                function: name,
            });
        }

        self.functions.push(function);
        Ok(self)
    }

    pub fn package(&self) -> &EcoString {
        &self.package
    }

    pub fn module(&self) -> &EcoString {
        &self.module
    }

    pub fn functions(&self) -> impl ExactSizeIterator<Item = &HostFunctionSchema> {
        self.functions.iter().map(HostFunctionDefinition::schema)
    }

    fn into_registered(self) -> RegisteredHostModule {
        RegisteredHostModule {
            package: self.package,
            module: self.module,
            functions: self.functions,
        }
    }
}

impl HostModules {
    pub fn new(
        modules: impl IntoIterator<Item = HostModule>,
    ) -> Result<Self, HostRegistrationError> {
        Self::from_modules(modules.into_iter().collect())
    }

    fn from_modules(modules: Vec<HostModule>) -> Result<Self, HostRegistrationError> {
        let mut identities = BTreeMap::new();
        for module in &modules {
            if let Some(first_package) =
                identities.insert(module.module.clone(), module.package.clone())
            {
                return Err(HostRegistrationError::DuplicateModule {
                    module: module.module.clone(),
                    first_package,
                    second_package: module.package.clone(),
                });
            }
        }
        Ok(Self { modules })
    }

    pub fn modules(&self) -> impl ExactSizeIterator<Item = &HostModule> {
        self.modules.iter()
    }

    pub(crate) fn into_registered(self) -> Vec<RegisteredHostModule> {
        self.modules
            .into_iter()
            .map(HostModule::into_registered)
            .collect()
    }
}

impl RegisteredHostModule {
    pub(crate) fn package(&self) -> &EcoString {
        &self.package
    }

    pub(crate) fn module(&self) -> &EcoString {
        &self.module
    }

    pub(crate) fn functions(&self) -> impl ExactSizeIterator<Item = &HostFunctionSchema> {
        self.functions.iter().map(HostFunctionDefinition::schema)
    }

    pub(crate) fn into_parts(self) -> (EcoString, EcoString, Vec<HostFunctionDefinition>) {
        (self.package, self.module, self.functions)
    }
}

fn valid_module_name(module: &EcoString) -> bool {
    module != PRELUDE_MODULE_NAME
        && !module.is_empty()
        && module.split('/').all(|segment| {
            !segment.is_empty()
                && string_to_keyword(segment).is_none()
                && check_name_case(
                    SrcSpan::new(0, 0),
                    &EcoString::from(segment),
                    Named::Function,
                )
                .is_ok()
        })
}

#[cfg(test)]
mod tests {
    use super::{HostModule, HostModules};
    use crate::host::HostRegistrationError;
    use crate::plan::ValueType;
    use num_bigint::BigInt;

    #[test]
    fn host_modules_expose_registered_schemas() {
        let module = HostModule::new("host_support", "host/math")
            .expect("module should be valid")
            .with_function("add", <BigInt as std::ops::Add>::add)
            .expect("function should be valid");
        let modules = HostModules::new([module]).expect("module should be unique");
        let module = modules.modules().next().expect("module should exist");
        let function = module.functions().next().expect("function should exist");

        assert_eq!(module.package(), "host_support");
        assert_eq!(module.module(), "host/math");
        assert_eq!(function.name(), "add");
        assert_eq!(
            function.type_().argument_types(),
            [ValueType::Int, ValueType::Int],
        );
        assert_eq!(function.type_().return_(), &ValueType::Int);
    }

    #[test]
    fn rejects_invalid_module_and_function_names() {
        assert_eq!(
            HostModule::new("host_support", "").err(),
            Some(HostRegistrationError::InvalidModuleName { module: "".into() }),
        );
        assert_eq!(
            HostModule::new("host_support", "gleam").err(),
            Some(HostRegistrationError::InvalidModuleName {
                module: "gleam".into(),
            }),
        );
        assert_eq!(
            HostModule::new("host_support", "host//math").err(),
            Some(HostRegistrationError::InvalidModuleName {
                module: "host//math".into(),
            }),
        );
        assert_eq!(
            HostModule::new("host_support", "host/fn").err(),
            Some(HostRegistrationError::InvalidModuleName {
                module: "host/fn".into(),
            }),
        );
        assert_eq!(
            HostModule::new("host_support", "host/Math").err(),
            Some(HostRegistrationError::InvalidModuleName {
                module: "host/Math".into(),
            }),
        );
        assert_eq!(
            HostModule::new("host_support", "host/math")
                .expect("module should be valid")
                .with_function("Add", <BigInt as std::ops::Add>::add)
                .err(),
            Some(HostRegistrationError::InvalidFunctionName {
                module: "host/math".into(),
                function: "Add".into(),
            }),
        );
        assert_eq!(
            HostModule::new("host_support", "host/math")
                .expect("module should be valid")
                .with_function("fn", <BigInt as std::ops::Add>::add)
                .err(),
            Some(HostRegistrationError::InvalidFunctionName {
                module: "host/math".into(),
                function: "fn".into(),
            }),
        );
    }

    #[test]
    fn rejects_duplicate_functions_and_modules() {
        let module = HostModule::new("host_support", "host/math")
            .expect("module should be valid")
            .with_function("add", <BigInt as std::ops::Add>::add)
            .expect("function should be valid");
        assert_eq!(
            module
                .with_function("add", <BigInt as std::ops::Add>::add)
                .err(),
            Some(HostRegistrationError::DuplicateFunction {
                module: "host/math".into(),
                function: "add".into(),
            }),
        );

        let first = HostModule::new("first", "host/math").expect("first module should be valid");
        let second = HostModule::new("second", "host/math").expect("second module should be valid");
        assert_eq!(
            HostModules::new([first, second]).err(),
            Some(HostRegistrationError::DuplicateModule {
                module: "host/math".into(),
                first_package: "first".into(),
                second_package: "second".into(),
            }),
        );
    }
}
