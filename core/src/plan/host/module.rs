use super::HostFunctionTemplate;
use crate::plan::{
    ConstantTemplates, CustomTypeDefinition, ExternalTypeDefinition, FunctionTemplate, ModuleId,
    SourceContext,
};
use ecow::EcoString;

pub struct HostedPlannedModule {
    id: ModuleId,
    package: EcoString,
    module: EcoString,
    source_context: Option<SourceContext>,
    custom_types: Vec<CustomTypeDefinition>,
    external_types: Vec<ExternalTypeDefinition>,
    constants: ConstantTemplates,
    functions: Vec<HostedFunctionTemplate>,
    anonymous_functions: Vec<FunctionTemplate>,
}

pub enum HostedFunctionTemplate {
    GleamBody(Box<FunctionTemplate>),
    HostTemplate(Box<HostFunctionTemplate>),
}

pub(crate) struct HostedPlannedModuleParts {
    pub(crate) id: ModuleId,
    pub(crate) package: EcoString,
    pub(crate) module: EcoString,
    pub(crate) source_context: Option<SourceContext>,
    pub(crate) custom_types: Vec<CustomTypeDefinition>,
    pub(crate) external_types: Vec<ExternalTypeDefinition>,
    pub(crate) constants: ConstantTemplates,
    pub(crate) functions: Vec<HostedFunctionTemplate>,
    pub(crate) anonymous_functions: Vec<FunctionTemplate>,
}

impl HostedPlannedModule {
    pub(crate) fn new(parts: HostedPlannedModuleParts) -> Self {
        Self {
            id: parts.id,
            package: parts.package,
            module: parts.module,
            source_context: parts.source_context,
            custom_types: parts.custom_types,
            external_types: parts.external_types,
            constants: parts.constants,
            functions: parts.functions,
            anonymous_functions: parts.anonymous_functions,
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

    pub fn source_context(&self) -> Option<&SourceContext> {
        self.source_context.as_ref()
    }

    pub fn functions(&self) -> &[HostedFunctionTemplate] {
        &self.functions
    }

    pub fn external_types(&self) -> &[ExternalTypeDefinition] {
        &self.external_types
    }

    pub(crate) fn into_parts(self) -> HostedPlannedModuleParts {
        HostedPlannedModuleParts {
            id: self.id,
            package: self.package,
            module: self.module,
            source_context: self.source_context,
            custom_types: self.custom_types,
            external_types: self.external_types,
            constants: self.constants,
            functions: self.functions,
            anonymous_functions: self.anonymous_functions,
        }
    }
}

impl HostedFunctionTemplate {
    pub fn gleam_body(&self) -> Option<&FunctionTemplate> {
        match self {
            Self::GleamBody(function) => Some(function.as_ref()),
            Self::HostTemplate(_) => None,
        }
    }

    pub fn host_template(&self) -> Option<&HostFunctionTemplate> {
        match self {
            Self::GleamBody(_) => None,
            Self::HostTemplate(function) => Some(function.as_ref()),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::frontend::{ModuleSource, PackageSource, compile_typed_host_program};
    use crate::host::{HostModule, HostProviderSet};
    use crate::planner::plan_host_program;
    use num_bigint::BigInt;

    #[test]
    fn function_templates_expose_exactly_one_body_owner() {
        let hosts = HostProviderSet::new([HostModule::new("host_support", "host/math")
            .expect("host module should be valid")
            .with_function("identity", |value: BigInt| value)
            .expect("host function should be valid")])
        .expect("host modules should be unique");
        let source = r#"
import host/math

pub fn main() {
  math.identity(1)
}
"#;
        let typed = compile_typed_host_program(
            "application",
            "main",
            [PackageSource::new(
                "application",
                ["host_support"],
                [ModuleSource::new("main", "main.gleam", source)],
            )],
            hosts,
        )
        .expect("host program should compile");
        let plan = plan_host_program(typed).expect("host program should plan");
        let host = &plan.modules()[0].functions()[0];
        let source = &plan.modules()[1].functions()[0];

        assert_eq!(
            host.host_template()
                .expect("source-less function should own a host template")
                .name(),
            "identity",
        );
        assert!(host.gleam_body().is_none());
        assert_eq!(
            source
                .gleam_body()
                .expect("source function should own a Gleam body")
                .name(),
            "main",
        );
        assert!(source.host_template().is_none());
    }
}
