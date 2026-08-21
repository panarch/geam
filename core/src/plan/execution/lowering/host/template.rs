use super::super::local;
use crate::plan::{
    FunctionTemplate, FunctionTemplateId, FunctionTemplateSignature, HostFunctionTemplate,
    HostedFunctionTemplate,
};
use std::collections::HashMap;

pub(super) struct HostTemplateCatalog {
    templates: Vec<Vec<HostLoweringTemplate>>,
}

pub(super) enum HostLoweringTemplate {
    Gleam(Box<FunctionTemplate>),
    Host(Box<HostFunctionTemplate>),
}

impl HostTemplateCatalog {
    pub(super) fn new() -> Self {
        Self {
            templates: Vec::new(),
        }
    }

    pub(super) fn push_module(
        &mut self,
        functions: Vec<HostedFunctionTemplate>,
        anonymous_functions: Vec<FunctionTemplate>,
    ) {
        let mut templates = functions
            .into_iter()
            .map(|template| match template {
                HostedFunctionTemplate::GleamBody(template) => {
                    HostLoweringTemplate::Gleam(template)
                }
                HostedFunctionTemplate::HostTemplate(template) => {
                    HostLoweringTemplate::Host(template)
                }
            })
            .collect::<Vec<_>>();
        templates.extend(
            anonymous_functions
                .into_iter()
                .map(|template| HostLoweringTemplate::Gleam(Box::new(template))),
        );
        templates.sort_by_key(HostLoweringTemplate::index);
        self.templates.push(templates);
    }

    pub(super) fn get(&self, id: FunctionTemplateId) -> &HostLoweringTemplate {
        &self.templates[id.module().index()][id.index()]
    }

    pub(super) fn entry_templates(
        &self,
    ) -> HashMap<FunctionTemplateId, local::FunctionEntryTemplate> {
        self.templates
            .iter()
            .flatten()
            .map(|template| {
                (
                    template.id(),
                    match template {
                        HostLoweringTemplate::Gleam(template) => {
                            local::FunctionEntryTemplate::new(template)
                        }
                        HostLoweringTemplate::Host(template) => {
                            local::FunctionEntryTemplate::from_shapes(
                                template.signature().shape().argument_shapes().to_vec(),
                            )
                        }
                    },
                )
            })
            .collect()
    }
}

impl HostLoweringTemplate {
    pub(super) fn signature(&self) -> &FunctionTemplateSignature {
        match self {
            Self::Gleam(template) => template.signature(),
            Self::Host(template) => template.signature(),
        }
    }

    fn id(&self) -> FunctionTemplateId {
        self.signature().id()
    }

    fn index(&self) -> usize {
        self.id().index()
    }
}

#[cfg(test)]
mod tests {
    use super::{HostLoweringTemplate, HostTemplateCatalog};
    use crate::frontend::{ModuleSource, PackageSource, compile_typed_host_program};
    use crate::host::{HostModule, HostProviderSet};
    use crate::plan::FunctionTemplateId;
    use crate::planner::plan_host_program;

    #[test]
    fn catalogs_host_named_and_anonymous_templates_by_module_local_id() {
        let hosts = HostProviderSet::new([HostModule::new("host_support", "host/predicate")
            .expect("host module should be valid")
            .with_function("negate", <bool as std::ops::Not>::not)
            .expect("host function should be valid")])
        .expect("host modules should be unique");
        let source = r#"
import host/predicate

fn apply(function: fn(Bool) -> Bool, value: Bool) {
  function(value)
}

pub fn main() {
  apply(fn(value) { predicate.negate(value) }, True)
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
        .expect("host source should compile");
        let plan = plan_host_program(typed).expect("host source should plan");
        let parts = plan.into_parts();
        let root = parts.root;
        let host = parts
            .modules
            .iter()
            .find(|module| module.package() == "host_support")
            .expect("host package should be planned")
            .id();
        let mut catalog = HostTemplateCatalog::new();

        for module in parts.modules {
            let parts = module.into_parts();
            catalog.push_module(parts.functions, parts.anonymous_functions);
        }

        assert_eq!(
            [
                FunctionTemplateId::in_module(host, 0),
                FunctionTemplateId::in_module(root, 0),
                FunctionTemplateId::in_module(root, 1),
                FunctionTemplateId::in_module(root, 2),
            ]
            .map(|id| match catalog.get(id) {
                HostLoweringTemplate::Gleam(function) => ("gleam", function.name().as_str()),
                HostLoweringTemplate::Host(function) => ("host", function.name().as_str()),
            }),
            [
                ("host", "negate"),
                ("gleam", "main"),
                ("gleam", "apply"),
                ("gleam", "<anonymous:0>"),
            ],
        );
        assert_eq!(catalog.entry_templates().len(), 4);
    }
}
