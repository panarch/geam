use super::function;
use super::local;
use super::specialization::{
    RepresentationContext, SpecializationKey, SpecializedFunctionShape, SpecializedValueShape,
};
use super::{
    LoweredExecution, LoweringCompletion, LoweringContext, ProgramConstantTemplates,
    SpecializationState, resolve_specialization_fixed_point,
};
use crate::host::HostFunctionImplementation as RegisteredHostFunctionImplementation;
use crate::plan::execution::function::{BoolFunctionBody, IntFunctionBody, ValueFunctionEntry};
use crate::plan::execution::graph::{
    BoolLocalId as ExecutionBoolLocalId, IntLocalId as ExecutionIntLocalId, ParamLocal,
};
use crate::plan::execution::host::{
    HostBoolFunctionId, HostFunctionTables, HostIntFunctionId, HostedBoolFunction,
    HostedExecutionHost, HostedIntFunction,
};
use crate::plan::execution::{ExecutionModuleContext, ExecutionProgram, ExecutionProgramCommon};
use crate::plan::{
    ConstantTemplates, FunctionTemplate, FunctionTemplateId, FunctionTemplateSignature,
    HostFunctionTemplate as PlannedHostFunctionTemplate, HostParameter, HostReturnFamily,
    HostedModulePlan, HostedModulePlanParts, HostedPlannedModuleKind,
};
use std::collections::{HashMap, HashSet};

type HostedIntFunctionEntry = ValueFunctionEntry<IntFunctionBody, HostIntFunctionId>;
type HostedBoolFunctionEntry = ValueFunctionEntry<BoolFunctionBody, HostBoolFunctionId>;
type LoweredHostedIntFunction = function::LoweredSpecialization<HostedIntFunctionEntry>;
type LoweredHostedBoolFunction = function::LoweredSpecialization<HostedBoolFunctionEntry>;
type HostedLoweredExecution = LoweredExecution<HostedIntFunctionEntry, HostedBoolFunctionEntry>;

struct HostedFunctionTemplates {
    templates: Vec<Vec<HostedFunctionTemplate>>,
}

enum HostedFunctionTemplate {
    Source(Box<FunctionTemplate>),
    Host(PlannedHostFunctionTemplate),
}

pub(in crate::plan::execution) fn lower_hosted(
    module_plan: HostedModulePlan,
) -> (ExecutionProgram<HostedExecutionHost>, HostFunctionTables) {
    let HostedModulePlanParts {
        root,
        entry,
        modules,
        implementations,
    } = module_plan.into_parts();
    let mut int_implementations = HashMap::new();
    let mut bool_implementations = HashMap::new();
    for implementation in implementations {
        let (template, implementation) = implementation.into_parts();
        match implementation {
            RegisteredHostFunctionImplementation::Int(function) => {
                int_implementations.insert(template, function);
            }
            RegisteredHostFunctionImplementation::Bool(function) => {
                bool_implementations.insert(template, function);
            }
        }
    }
    let mut module_contexts = Vec::with_capacity(modules.len());
    let mut module_templates = Vec::with_capacity(modules.len());
    let mut constant_templates = Vec::with_capacity(modules.len());
    let mut custom_types = Vec::new();

    for module in modules {
        match module.into_kind() {
            HostedPlannedModuleKind::Source(module) => {
                let parts = (*module).into_parts();
                module_contexts.push(ExecutionModuleContext::new(
                    parts.module,
                    parts.source_context,
                ));
                custom_types.extend(parts.custom_types);
                constant_templates.push(parts.constants);
                let mut templates = parts
                    .functions
                    .into_iter()
                    .map(|template| HostedFunctionTemplate::Source(Box::new(template)))
                    .collect::<Vec<_>>();
                templates.extend(
                    parts
                        .anonymous_functions
                        .into_iter()
                        .map(|template| HostedFunctionTemplate::Source(Box::new(template))),
                );
                templates.sort_by_key(HostedFunctionTemplate::index);
                module_templates.push(templates);
            }
            HostedPlannedModuleKind::Host(module) => {
                let (module_id, _, module_name, functions) = module.into_parts();
                module_contexts.push(ExecutionModuleContext::new(module_name, None));
                constant_templates.push(ConstantTemplates::from_module_entries(
                    module_id,
                    Vec::new(),
                ));
                let mut templates = functions
                    .into_iter()
                    .map(HostedFunctionTemplate::Host)
                    .collect::<Vec<_>>();
                templates.sort_by_key(HostedFunctionTemplate::index);
                module_templates.push(templates);
            }
        }
    }

    let templates = HostedFunctionTemplates {
        templates: module_templates,
    };
    let main_return_shape = templates
        .get(entry)
        .signature()
        .shape()
        .return_shape()
        .clone();
    let main_key = SpecializationKey::monomorphic(entry);
    let initial = SpecializationState {
        constant_templates: ProgramConstantTemplates {
            modules: constant_templates,
        },
        representations: RepresentationContext::new(custom_types),
        erased_specializations: HashSet::new(),
    };

    let (main, lowered, host_int_functions, host_bool_functions) =
        resolve_specialization_fixed_point(initial, |state| {
            let SpecializationState {
                constant_templates,
                representations,
                erased_specializations,
            } = state;
            let main_value_shape =
                SpecializedValueShape::instantiate(&main_return_shape, main_key.substitution());
            let main_return_shape = representations.inhabitation(&main_value_shape);
            let mut context = LoweringContext::new(
                templates.entry_templates(),
                representations,
                constant_templates,
                erased_specializations,
            );
            let main = context.reserve_main(main_key.clone(), main_return_shape);
            let mut host_int_functions = Vec::new();
            let mut host_bool_functions = Vec::new();
            let mut lowered_host_int_functions = Vec::new();
            let mut lowered_host_bool_functions = Vec::new();

            while let Some(key) = context.pending.pop_front() {
                context.begin(&key);
                match templates.get(key.template()) {
                    HostedFunctionTemplate::Source(template) => {
                        function::lower_specialized(template, &key, &mut context);
                    }
                    HostedFunctionTemplate::Host(template) => {
                        let index = context.specialization_index(&key);
                        let parameters = template
                            .parameters()
                            .iter()
                            .map(|parameter| match parameter {
                                HostParameter::Int(local) => {
                                    ParamLocal::Int(ExecutionIntLocalId(local.0))
                                }
                                HostParameter::Bool(local) => {
                                    ParamLocal::Bool(ExecutionBoolLocalId(local.0))
                                }
                            })
                            .collect();
                        let shape = SpecializedFunctionShape::instantiate(
                            template.signature().shape(),
                            key.substitution(),
                        );
                        let type_ = context.lower_concrete_function_type(&shape);
                        match template.return_family() {
                            HostReturnFamily::Int => {
                                let target = HostIntFunctionId::new(host_int_functions.len());
                                host_int_functions.push(HostedIntFunction::new(
                                    template.package().clone(),
                                    template.module().clone(),
                                    template.name().clone(),
                                    parameters,
                                    type_,
                                    int_implementations[&template.id()].clone(),
                                ));
                                lowered_host_int_functions.push((
                                    index,
                                    function::lowered_host_function::<IntFunctionBody, _>(
                                        &key, target,
                                    ),
                                ));
                            }
                            HostReturnFamily::Bool => {
                                let target = HostBoolFunctionId::new(host_bool_functions.len());
                                host_bool_functions.push(HostedBoolFunction::new(
                                    template.package().clone(),
                                    template.module().clone(),
                                    template.name().clone(),
                                    parameters,
                                    type_,
                                    bool_implementations[&template.id()].clone(),
                                ));
                                lowered_host_bool_functions.push((
                                    index,
                                    function::lowered_host_function::<BoolFunctionBody, _>(
                                        &key, target,
                                    ),
                                ));
                            }
                        }
                    }
                }
            }

            let (constant_templates, representations, outcome) =
                context.finish_hosted(lowered_host_int_functions, lowered_host_bool_functions);
            let erased_specializations = outcome.erased_specializations();
            outcome
                .map(|lowered| (main, lowered, host_int_functions, host_bool_functions))
                .into_fixed_point(SpecializationState {
                    constant_templates,
                    representations,
                    erased_specializations,
                })
        });

    (
        ExecutionProgram {
            common: ExecutionProgramCommon {
                root,
                modules: module_contexts.into_boxed_slice(),
                main,
                constants: lowered.constants,
                list_types: lowered.list_types,
                custom_types: lowered.custom_types,
                value_shapes: lowered.value_shapes,
            },
            functions: lowered.functions,
        },
        HostFunctionTables::new(
            host_int_functions.into_boxed_slice(),
            host_bool_functions.into_boxed_slice(),
        ),
    )
}

impl LoweringContext {
    fn finish_hosted(
        self,
        host_int_functions: Vec<(usize, LoweredHostedIntFunction)>,
        host_bool_functions: Vec<(usize, LoweredHostedBoolFunction)>,
    ) -> LoweringCompletion<HostedLoweredExecution> {
        let Self {
            constant_templates,
            constants,
            types,
            representations,
            functions,
            erased_specializations,
            ..
        } = self;
        let outcome = functions
            .finish_hosted(host_int_functions, host_bool_functions)
            .map(|functions| {
                let (list_types, custom_types, value_shapes) = types.into_tables();
                Box::new(LoweredExecution {
                    constants: constants.finish(),
                    functions: *functions,
                    list_types,
                    custom_types,
                    value_shapes,
                })
            })
            .include_prior_erasure(erased_specializations);
        (constant_templates, representations, outcome)
    }
}

impl HostedFunctionTemplates {
    fn get(&self, id: FunctionTemplateId) -> &HostedFunctionTemplate {
        &self.templates[id.module().index()][id.index()]
    }

    fn entry_templates(&self) -> HashMap<FunctionTemplateId, local::FunctionEntryTemplate> {
        self.templates
            .iter()
            .flatten()
            .map(|template| {
                (
                    template.id(),
                    match template {
                        HostedFunctionTemplate::Source(template) => {
                            local::FunctionEntryTemplate::new(template)
                        }
                        HostedFunctionTemplate::Host(template) => {
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

impl HostedFunctionTemplate {
    fn signature(&self) -> &FunctionTemplateSignature {
        match self {
            Self::Source(template) => template.signature(),
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
