mod custom;
mod function;

use super::super::{AnonymousFunctions, ModuleRole};
use super::declaration::HostedModuleDeclaration;
use crate::host::{HostFunctionConstructions, RegisteredHostImplementationId};
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
    pub(super) external_types: Vec<crate::plan::ExternalTypeDefinition>,
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
        constructions: HostFunctionConstructions,
        implementation: RegisteredHostImplementationId,
    },
}

pub(super) fn link_hosted_modules(
    root: ModuleId,
    declarations: Vec<HostedModuleDeclaration>,
) -> Result<Vec<LinkedModule>, PlanError> {
    let external_types = declarations
        .iter()
        .flat_map(|declaration| match declaration {
            HostedModuleDeclaration::Source { external_types, .. } => external_types.as_slice(),
            HostedModuleDeclaration::Host { .. } => &[],
        })
        .map(|definition| definition.name().clone())
        .collect::<HashSet<_>>();
    declarations
        .into_iter()
        .map(|declaration| link_hosted_module(root, declaration, &external_types))
        .collect()
}

fn link_hosted_module(
    root: ModuleId,
    declaration: HostedModuleDeclaration,
    external_types: &HashSet<crate::plan::ExternalTypeName>,
) -> Result<LinkedModule, PlanError> {
    match declaration {
        HostedModuleDeclaration::Source {
            id,
            package,
            module_name,
            source_context,
            custom_types,
            external_types: module_external_types,
            functions,
            constants,
            providers,
        } => {
            let role = if id == root {
                ModuleRole::Root
            } else {
                ModuleRole::Dependency
            };
            let functions = function::select_erlang_hosted_functions(functions, &providers);
            super::super::function_table_with_external_types(id, &functions, role, external_types)
                .and_then(|table| {
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
                        external_types: module_external_types,
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
