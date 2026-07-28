use super::function;
use super::local;
use super::specialization::{
    FunctionRepresentation, RepresentationContext, SpecializationKey, SpecializedFunctionShape,
    SpecializedValueShape,
};
use super::{
    LoweredExecution, LoweringCompletion, LoweringContext, ProgramConstantTemplates,
    SpecializationState, resolve_specialization_fixed_point,
};
use crate::host::{HostFunctionImplementation, HostProfile, HostValueFunctionImplementation};
use crate::plan::execution::function as execution_function;
use crate::plan::execution::graph as execution_graph;
use crate::plan::execution::graph::ParamLocal;
use crate::plan::execution::host::{
    HostBitArrayFunctionId, HostBoolFunctionId, HostFloatFunctionId, HostFunctionTables,
    HostIntFunctionId, HostNeverFunctionId, HostNilFunctionId, HostStringFunctionId,
    HostUtfCodepointFunctionId, HostValueFunctionTables, HostedBitArrayFunction,
    HostedBoolFunction, HostedExecutionProfile, HostedFloatFunction, HostedFunction,
    HostedFunctionTarget, HostedIntFunction, HostedNeverFunction, HostedNilFunction,
    HostedStringFunction, HostedUtfCodepointFunction,
};
use crate::plan::execution::{ExecutionModuleContext, ExecutionProgram, ExecutionProgramCommon};
use crate::plan::{
    FunctionTemplate, FunctionTemplateId, FunctionTemplateSignature,
    HostFunctionTemplate as PlannedHostFunctionTemplate, HostParameter,
    HostedFunctionTemplate as PlannedHostedFunctionTemplate, HostedModulePlan,
    HostedModulePlanParts,
};
use std::collections::{HashMap, HashSet};

type HostedLoweredExecution<Profile> = LoweredExecution<HostedExecutionProfile<Profile>>;

struct HostedFunctionTemplates {
    templates: Vec<Vec<HostedFunctionTemplate>>,
}

enum HostedFunctionTemplate {
    Source(Box<FunctionTemplate>),
    Host(Box<PlannedHostFunctionTemplate>),
}

struct RegisteredHostFunctions<Profile: HostProfile> {
    functions: HashMap<FunctionTemplateId, HostFunctionImplementation<Profile>>,
}

struct LoweredHostFunctions<Profile: HostProfile> {
    int: Vec<HostedIntFunction<Profile>>,
    float: Vec<HostedFloatFunction<Profile>>,
    string: Vec<HostedStringFunction<Profile>>,
    bit_array: Vec<HostedBitArrayFunction<Profile>>,
    utf_codepoint: Vec<HostedUtfCodepointFunction<Profile>>,
    bool: Vec<HostedBoolFunction<Profile>>,
    nil: Vec<HostedNilFunction<Profile>>,
    never: Vec<HostedNeverFunction<Profile>>,
    additional: function::AdditionalFunctions<HostedExecutionProfile<Profile>>,
}

pub(in crate::plan::execution) fn lower_hosted<Profile: HostProfile>(
    module_plan: HostedModulePlan<Profile>,
) -> (
    ExecutionProgram<HostedExecutionProfile<Profile>>,
    HostFunctionTables<Profile>,
) {
    let HostedModulePlanParts {
        root,
        entry,
        modules,
        implementations,
    } = module_plan.into_parts();
    let implementations = RegisteredHostFunctions::new(implementations);
    let mut module_contexts = Vec::with_capacity(modules.len());
    let mut module_templates = Vec::with_capacity(modules.len());
    let mut constant_templates = Vec::with_capacity(modules.len());
    let mut custom_types = Vec::new();

    for module in modules {
        let parts = module.into_parts();
        module_contexts.push(ExecutionModuleContext::new(
            parts.module,
            parts.source_context,
        ));
        custom_types.extend(parts.custom_types);
        constant_templates.push(parts.constants);
        let mut templates = parts
            .functions
            .into_iter()
            .map(|template| match template {
                PlannedHostedFunctionTemplate::GleamBody(template) => {
                    HostedFunctionTemplate::Source(template)
                }
                PlannedHostedFunctionTemplate::HostTemplate(template) => {
                    HostedFunctionTemplate::Host(Box::new(template))
                }
            })
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

    let (main, lowered, host_functions) = resolve_specialization_fixed_point(initial, |state| {
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
        let mut host_functions = LoweredHostFunctions::new();

        while let Some(key) = context.pending.pop_front() {
            context.begin(&key);
            match templates.get(key.template()) {
                HostedFunctionTemplate::Source(template) => {
                    function::lower_specialized(template, &key, &mut context);
                }
                HostedFunctionTemplate::Host(template) => {
                    let index = context.specialization_index(&key);
                    let shape = SpecializedFunctionShape::instantiate(
                        template.signature().shape(),
                        key.substitution(),
                    );
                    let parameters = template.parameters().iter().map(host_parameter).collect();
                    let signature = shape.to_module_shape().type_();
                    let type_ = context.lower_concrete_function_type(&shape);
                    match implementations.functions[&template.id()].clone() {
                        HostFunctionImplementation::Value(
                            HostValueFunctionImplementation::Int(implementation),
                        ) => {
                            let target = HostIntFunctionId::new(host_functions.int.len());
                            host_functions.int.push(HostedFunction::new(
                                template.package().clone(),
                                template.site().clone(),
                                signature,
                                parameters,
                                type_,
                                implementation,
                            ));
                            host_functions.additional.int.push((
                                index,
                                function::lowered_host_function(
                                    &key,
                                    HostedFunctionTarget::value(target),
                                ),
                            ));
                        }
                        HostFunctionImplementation::Value(
                            HostValueFunctionImplementation::Float(implementation),
                        ) => {
                            let target = HostFloatFunctionId::new(host_functions.float.len());
                            host_functions.float.push(HostedFunction::new(
                                template.package().clone(),
                                template.site().clone(),
                                signature,
                                parameters,
                                type_,
                                implementation,
                            ));
                            host_functions.additional.float.push((
                                index,
                                function::lowered_host_function(
                                    &key,
                                    HostedFunctionTarget::value(target),
                                ),
                            ));
                        }
                        HostFunctionImplementation::Value(
                            HostValueFunctionImplementation::String(implementation),
                        ) => {
                            let target = HostStringFunctionId::new(host_functions.string.len());
                            host_functions.string.push(HostedFunction::new(
                                template.package().clone(),
                                template.site().clone(),
                                signature,
                                parameters,
                                type_,
                                implementation,
                            ));
                            host_functions.additional.string.push((
                                index,
                                function::lowered_host_function(
                                    &key,
                                    HostedFunctionTarget::value(target),
                                ),
                            ));
                        }
                        HostFunctionImplementation::Value(
                            HostValueFunctionImplementation::BitArray(implementation),
                        ) => {
                            let target =
                                HostBitArrayFunctionId::new(host_functions.bit_array.len());
                            host_functions.bit_array.push(HostedFunction::new(
                                template.package().clone(),
                                template.site().clone(),
                                signature,
                                parameters,
                                type_,
                                implementation,
                            ));
                            host_functions.additional.bit_array.push((
                                index,
                                function::lowered_host_function(
                                    &key,
                                    HostedFunctionTarget::value(target),
                                ),
                            ));
                        }
                        HostFunctionImplementation::Value(
                            HostValueFunctionImplementation::UtfCodepoint(implementation),
                        ) => {
                            let target =
                                HostUtfCodepointFunctionId::new(host_functions.utf_codepoint.len());
                            host_functions.utf_codepoint.push(HostedFunction::new(
                                template.package().clone(),
                                template.site().clone(),
                                signature,
                                parameters,
                                type_,
                                implementation,
                            ));
                            host_functions.additional.utf_codepoint.push((
                                index,
                                function::lowered_host_function(
                                    &key,
                                    HostedFunctionTarget::value(target),
                                ),
                            ));
                        }
                        HostFunctionImplementation::Value(
                            HostValueFunctionImplementation::Bool(implementation),
                        ) => {
                            let target = HostBoolFunctionId::new(host_functions.bool.len());
                            host_functions.bool.push(HostedFunction::new(
                                template.package().clone(),
                                template.site().clone(),
                                signature,
                                parameters,
                                type_,
                                implementation,
                            ));
                            host_functions.additional.bool.push((
                                index,
                                function::lowered_host_function(
                                    &key,
                                    HostedFunctionTarget::value(target),
                                ),
                            ));
                        }
                        HostFunctionImplementation::Value(
                            HostValueFunctionImplementation::Nil(implementation),
                        ) => {
                            let target = HostNilFunctionId::new(host_functions.nil.len());
                            host_functions.nil.push(HostedFunction::new(
                                template.package().clone(),
                                template.site().clone(),
                                signature,
                                parameters,
                                type_,
                                implementation,
                            ));
                            host_functions.additional.nil.push((
                                index,
                                function::lowered_host_function(
                                    &key,
                                    HostedFunctionTarget::value(target),
                                ),
                            ));
                        }
                        HostFunctionImplementation::Never(implementation) => {
                            let target = HostNeverFunctionId::new(host_functions.never.len());
                            host_functions.never.push(HostedNeverFunction::new(
                                template.package().clone(),
                                template.site().clone(),
                                signature,
                                parameters,
                                type_,
                                implementation,
                            ));
                            match shape.representation(&context.representations) {
                                FunctionRepresentation::Executable(return_) => {
                                    let function = function::function_id(
                                        &return_,
                                        index,
                                        &mut context.types,
                                        &context.representations,
                                    );
                                    host_functions
                                        .additional
                                        .push_never_host_function(index, &key, function, target);
                                }
                                FunctionRepresentation::Never(_)
                                | FunctionRepresentation::Symbolic => {
                                    host_functions.additional.push_never_host_function(
                                        index,
                                        &key,
                                        execution_function::RuntimeFunctionId::Never(
                                            execution_function::NeverFunctionId(index),
                                        ),
                                        target,
                                    );
                                }
                            }
                        }
                    }
                }
            }
        }

        let LoweredHostFunctions {
            int,
            float,
            string,
            bit_array,
            utf_codepoint,
            bool,
            nil,
            never,
            additional,
        } = host_functions;
        let (constant_templates, representations, outcome) = context.finish_hosted(additional);
        let erased_specializations = outcome.erased_specializations();
        outcome
            .map(|lowered| {
                (
                    main,
                    lowered,
                    HostFunctionTables::new(
                        HostValueFunctionTables::new(
                            int.into_boxed_slice(),
                            float.into_boxed_slice(),
                            string.into_boxed_slice(),
                            bit_array.into_boxed_slice(),
                            utf_codepoint.into_boxed_slice(),
                            bool.into_boxed_slice(),
                            nil.into_boxed_slice(),
                        ),
                        never.into_boxed_slice(),
                    ),
                )
            })
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
        host_functions,
    )
}

fn host_parameter(parameter: &HostParameter) -> ParamLocal {
    match parameter {
        HostParameter::Int(local) => ParamLocal::Int(execution_graph::IntLocalId(local.0)),
        HostParameter::Float(local) => ParamLocal::Float(execution_graph::FloatLocalId(local.0)),
        HostParameter::String(local) => ParamLocal::String(execution_graph::StringLocalId(local.0)),
        HostParameter::BitArray(local) => {
            ParamLocal::BitArray(execution_graph::BitArrayLocalId(local.0))
        }
        HostParameter::UtfCodepoint(local) => {
            ParamLocal::UtfCodepoint(execution_graph::UtfCodepointLocalId(local.0))
        }
        HostParameter::Bool(local) => ParamLocal::Bool(execution_graph::BoolLocalId(local.0)),
        HostParameter::Nil(local) => ParamLocal::Nil(execution_graph::NilLocalId(local.0)),
    }
}

impl<Profile: HostProfile> RegisteredHostFunctions<Profile> {
    fn new(implementations: Vec<crate::plan::HostFunctionImplementation<Profile>>) -> Self {
        let mut functions = HashMap::new();
        for implementation in implementations {
            let (template, implementation) = implementation.into_parts();
            functions.insert(template, implementation);
        }
        Self { functions }
    }
}

impl<Profile: HostProfile> LoweredHostFunctions<Profile> {
    fn new() -> Self {
        Self {
            int: Vec::new(),
            float: Vec::new(),
            string: Vec::new(),
            bit_array: Vec::new(),
            utf_codepoint: Vec::new(),
            bool: Vec::new(),
            nil: Vec::new(),
            never: Vec::new(),
            additional: function::AdditionalFunctions::empty(),
        }
    }
}

impl LoweringContext {
    fn finish_hosted<Profile: HostProfile>(
        self,
        additional: function::AdditionalFunctions<HostedExecutionProfile<Profile>>,
    ) -> LoweringCompletion<HostedLoweredExecution<Profile>> {
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
            .finish_hosted(additional)
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
