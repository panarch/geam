mod custom;
mod function;

use super::super::{AnonymousFunctions, ModuleRole, function_table};
use super::declaration::HostedModuleDeclaration;
use crate::host::RegisteredHostImplementationId;
use crate::plan::{HostFunctionTemplate, ModuleId, SourceContext};
use crate::planner::context::FunctionInfo;
use crate::planner::error::PlanError;
use ecow::EcoString;
use gleam_core::ast::TypedFunction;
use std::collections::{HashMap, HashSet};

pub(super) use custom::validate_host_custom_schemas;

pub(super) struct LinkedModule {
    pub(super) id: ModuleId,
    pub(super) package: EcoString,
    pub(super) module_name: EcoString,
    pub(super) source_context: Option<SourceContext>,
    pub(super) custom_types: Vec<crate::plan::CustomTypeDefinition>,
    pub(super) functions_by_name: HashMap<EcoString, FunctionInfo>,
    pub(super) functions: Vec<LinkedFunction>,
    pub(super) executable_externals: HashSet<EcoString>,
    pub(super) constants: Vec<gleam_core::ast::TypedModuleConstant>,
    pub(super) anonymous_functions: AnonymousFunctions,
}

pub(super) enum LinkedFunction {
    Gleam {
        info: FunctionInfo,
        function: TypedFunction,
    },
    ExternalFallback {
        name: EcoString,
        info: FunctionInfo,
        function: TypedFunction,
    },
    Host {
        template: HostFunctionTemplate,
        implementation: RegisteredHostImplementationId,
    },
}

pub(super) fn link_hosted_modules(
    root: ModuleId,
    declarations: Vec<HostedModuleDeclaration>,
) -> Result<Vec<LinkedModule>, PlanError> {
    declarations
        .into_iter()
        .map(|declaration| link_hosted_module(root, declaration))
        .collect()
}

fn link_hosted_module(
    root: ModuleId,
    declaration: HostedModuleDeclaration,
) -> Result<LinkedModule, PlanError> {
    match declaration {
        HostedModuleDeclaration::Source {
            id,
            package,
            module_name,
            source_context,
            custom_types,
            functions,
            constants,
            providers,
        } => {
            let role = if id == root {
                ModuleRole::Root
            } else {
                ModuleRole::Dependency
            };
            function_table(id, &functions, role).and_then(|table| {
                function::link_source_functions(
                    package.clone(),
                    module_name.clone(),
                    table.functions,
                    providers,
                )
                .map(|(functions, executable_externals)| LinkedModule {
                    id,
                    package,
                    module_name,
                    source_context,
                    custom_types,
                    functions_by_name: table.by_name,
                    functions,
                    executable_externals,
                    constants,
                    anonymous_functions: table.anonymous_functions,
                })
            })
        }
        HostedModuleDeclaration::Host {
            id,
            package,
            module_name,
            functions,
        } => Ok(function::link_source_less_module(
            id,
            package,
            module_name,
            functions,
        )),
    }
}
