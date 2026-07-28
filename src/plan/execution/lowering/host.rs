use super::function;
use super::local;
use super::specialization::{
    RepresentationContext, SpecializationKey, SpecializedFunctionShape, SpecializedValueShape,
    StoredValueShape, ValueInhabitation,
};
use super::{
    LoweredExecution, LoweringCompletion, LoweringContext, ProgramConstantTemplates,
    SpecializationState, try_resolve_specialization_fixed_point,
};
use crate::host::{
    HostFunctionImplementation as RegisteredHostFunctionImplementation, HostProfile,
};
use crate::plan::execution::function as execution_function;
use crate::plan::execution::graph as execution_graph;
use crate::plan::execution::graph::ParamLocal;
use crate::plan::execution::host::{
    HostCallParameter, HostFunctionId, HostFunctionTables, HostNeverFunctionId,
    HostSpecializationError, HostedExecutionProfile, HostedFunction, HostedFunctionTarget,
    HostedNeverFunction, HostedValueFunction,
};
use crate::plan::execution::{ExecutionModuleContext, ExecutionProgram, ExecutionProgramCommon};
use crate::plan::{
    FunctionTemplate, FunctionTemplateId, FunctionTemplateSignature,
    HostFunctionTemplate as PlannedHostFunctionTemplate, HostImplementationBinding,
    HostedFunctionTemplate as PlannedHostedFunctionTemplate, HostedModulePlan,
    HostedModulePlanParts,
};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

type HostedLoweredExecution<Profile> = LoweredExecution<HostedExecutionProfile<Profile>>;

struct HostedFunctionTemplates {
    templates: Vec<Vec<HostedFunctionTemplate>>,
}

enum HostedFunctionTemplate {
    Source(Box<FunctionTemplate>),
    Host(Box<PlannedHostFunctionTemplate>),
}

struct RegisteredHostFunctions<Profile: HostProfile> {
    functions: HashMap<FunctionTemplateId, Arc<RegisteredHostFunctionImplementation<Profile>>>,
}

struct LoweredHostFunctions<Profile: HostProfile> {
    value_functions: Vec<HostedValueFunction<Profile>>,
    never_functions: Vec<HostedNeverFunction<Profile>>,
    additional: function::AdditionalFunctions<HostedExecutionProfile<Profile>>,
}

pub(in crate::plan::execution) fn lower_hosted<Profile: HostProfile>(
    module_plan: HostedModulePlan<Profile>,
) -> Result<
    (
        ExecutionProgram<HostedExecutionProfile<Profile>>,
        HostFunctionTables<Profile>,
    ),
    HostSpecializationError,
> {
    let HostedModulePlanParts {
        root,
        entry,
        modules,
        implementation_bindings,
    } = module_plan.into_parts();
    let implementations = RegisteredHostFunctions::new(implementation_bindings);
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
                    HostedFunctionTemplate::Host(template)
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

    let (main, lowered, host_functions) =
        try_resolve_specialization_fixed_point(initial, |state| {
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
                        let parameters = context.specialization_parameters(&key).to_vec();
                        let implementation = Arc::clone(&implementations.functions[&template.id()]);
                        let return_ = context.representations.inhabitation(shape.return_());
                        match implementation.as_ref() {
                            RegisteredHostFunctionImplementation::Value(implementation) => {
                                let ValueInhabitation::Inhabited(return_) = return_ else {
                                    return Err(HostSpecializationError::new(
                                        template.package().clone(),
                                        template.site().module().clone(),
                                        template.site().function().clone(),
                                        shape.to_module_shape().type_(),
                                    ));
                                };
                                let parameters =
                                    host_parameters(&parameters, template.layout(), &mut context);
                                let type_ = context.lower_concrete_function_type(&shape);
                                let host_index = host_functions.value_functions.len();
                                host_functions.value_functions.push(HostedFunction::new(
                                    template.package().clone(),
                                    template.site().clone(),
                                    shape.to_module_shape().type_(),
                                    parameters.entry,
                                    parameters.call,
                                    type_,
                                    implementation.clone(),
                                ));
                                lower_host_return(
                                    index,
                                    &key,
                                    return_,
                                    HostSpecialization::Value(host_index),
                                    &mut host_functions.additional,
                                    &mut context,
                                );
                            }
                            RegisteredHostFunctionImplementation::Never(implementation) => {
                                let parameters =
                                    host_parameters(&parameters, template.layout(), &mut context);
                                let type_ = context.lower_concrete_function_type(&shape);
                                let host_index = host_functions.never_functions.len();
                                host_functions.never_functions.push(HostedFunction::new(
                                    template.package().clone(),
                                    template.site().clone(),
                                    shape.to_module_shape().type_(),
                                    parameters.entry,
                                    parameters.call,
                                    type_,
                                    implementation.clone(),
                                ));
                                match return_ {
                                    ValueInhabitation::Inhabited(return_) => lower_host_return(
                                        index,
                                        &key,
                                        return_,
                                        HostSpecialization::Never(host_index),
                                        &mut host_functions.additional,
                                        &mut context,
                                    ),
                                    ValueInhabitation::Uninhabited(_) => {
                                        host_functions.additional.never.push((
                                            index,
                                            lowered_never_host_target(&key, host_index),
                                        ));
                                    }
                                }
                            }
                        }
                    }
                }
            }

            let LoweredHostFunctions {
                value_functions,
                never_functions,
                additional,
            } = host_functions;
            let (constant_templates, representations, outcome) = context.finish_hosted(additional);
            let erased_specializations = outcome.erased_specializations();
            Ok(outcome
                .map(|lowered| {
                    (
                        main,
                        lowered,
                        HostFunctionTables::new(
                            value_functions.into_boxed_slice(),
                            never_functions.into_boxed_slice(),
                        ),
                    )
                })
                .into_fixed_point(SpecializationState {
                    constant_templates,
                    representations,
                    erased_specializations,
                }))
        })?;

    Ok((
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
    ))
}

#[derive(Clone, Copy)]
enum HostSpecialization {
    Value(usize),
    Never(usize),
}

fn host_parameters(
    shapes: &[StoredValueShape],
    layout: &[crate::host::HostParameter],
    context: &mut LoweringContext,
) -> HostParameters {
    let mut prefix = local::ParameterPrefix::default();
    let parameters = shapes
        .iter()
        .enumerate()
        .map(|(position, shape)| {
            let (index, stored) = prefix.allocate_stored(shape.clone(), &context.representations);
            let entry = local::stored_value_local_at(&stored, index, context);
            let generic = matches!(layout[position], crate::host::HostParameter::Value(_));
            let call = host_call_parameter(&stored, index, generic, context);
            (entry, call)
        })
        .collect::<Vec<_>>();
    let (entry, call) = parameters.into_iter().unzip::<_, _, Vec<_>, Vec<_>>();
    HostParameters {
        entry: entry.into_boxed_slice(),
        call: call.into_boxed_slice(),
    }
}

struct HostParameters {
    entry: Box<[ParamLocal]>,
    call: Box<[HostCallParameter]>,
}

fn host_call_parameter(
    shape: &StoredValueShape,
    index: usize,
    generic: bool,
    context: &mut LoweringContext,
) -> HostCallParameter {
    match shape {
        StoredValueShape::Function(_) => {
            HostCallParameter::Value(local::stored_value_local_at(shape, index, context))
        }
        _ if generic => {
            HostCallParameter::Value(local::stored_value_local_at(shape, index, context))
        }
        StoredValueShape::Int => HostCallParameter::Int(execution_graph::IntLocalId(index)),
        StoredValueShape::Float => HostCallParameter::Float(execution_graph::FloatLocalId(index)),
        StoredValueShape::String => {
            HostCallParameter::String(execution_graph::StringLocalId(index))
        }
        StoredValueShape::BitArray => {
            HostCallParameter::BitArray(execution_graph::BitArrayLocalId(index))
        }
        StoredValueShape::UtfCodepoint => {
            HostCallParameter::UtfCodepoint(execution_graph::UtfCodepointLocalId(index))
        }
        StoredValueShape::Custom(shape) => {
            HostCallParameter::Custom(execution_graph::CustomLocal::new(
                execution_graph::CustomLocalId(index),
                context.lower_concrete_custom_shape(shape),
            ))
        }
        StoredValueShape::Bool => HostCallParameter::Bool(execution_graph::BoolLocalId(index)),
        StoredValueShape::Nil => HostCallParameter::Nil(execution_graph::NilLocalId(index)),
        StoredValueShape::Tuple(_) => {
            HostCallParameter::Tuple(local::stored_value_local_at(shape, index, context))
        }
        StoredValueShape::List(item) => {
            HostCallParameter::List(local::list_local_at(item, index, context))
        }
    }
}

fn lower_host_return<Profile: HostProfile>(
    index: usize,
    key: &SpecializationKey,
    return_: StoredValueShape,
    specialization: HostSpecialization,
    functions: &mut function::AdditionalFunctions<HostedExecutionProfile<Profile>>,
    context: &mut LoweringContext,
) {
    use execution_function::ListFunctionId as L;

    match return_ {
        StoredValueShape::Int => {
            let return_ = execution_graph::IntLocalId(0);
            functions.int.push((
                index,
                lowered_host_target::<execution_function::IntFunctionBody>(
                    key,
                    specialization,
                    return_,
                ),
            ));
        }
        StoredValueShape::Float => {
            let return_ = execution_graph::FloatLocalId(0);
            functions.float.push((
                index,
                lowered_host_target::<execution_function::FloatFunctionBody>(
                    key,
                    specialization,
                    return_,
                ),
            ));
        }
        StoredValueShape::String => {
            let return_ = execution_graph::StringLocalId(0);
            functions.string.push((
                index,
                lowered_host_target::<execution_function::StringFunctionBody>(
                    key,
                    specialization,
                    return_,
                ),
            ));
        }
        StoredValueShape::BitArray => {
            let return_ = execution_graph::BitArrayLocalId(0);
            functions.bit_array.push((
                index,
                lowered_host_target::<execution_function::BitArrayFunctionBody>(
                    key,
                    specialization,
                    return_,
                ),
            ));
        }
        StoredValueShape::UtfCodepoint => {
            let return_ = execution_graph::UtfCodepointLocalId(0);
            functions.utf_codepoint.push((
                index,
                lowered_host_target::<execution_function::UtfCodepointFunctionBody>(
                    key,
                    specialization,
                    return_,
                ),
            ));
        }
        StoredValueShape::Custom(shape) => {
            let return_ = execution_graph::CustomLocal::new(
                execution_graph::CustomLocalId(0),
                context.lower_concrete_custom_shape(&shape),
            );
            functions.custom.push((
                index,
                lowered_host_target::<execution_function::CustomFunctionBody>(
                    key,
                    specialization,
                    return_,
                ),
            ));
        }
        StoredValueShape::Bool => {
            let return_ = execution_graph::BoolLocalId(0);
            functions.bool.push((
                index,
                lowered_host_target::<execution_function::BoolFunctionBody>(
                    key,
                    specialization,
                    return_,
                ),
            ));
        }
        StoredValueShape::Nil => {
            let return_ = execution_graph::NilLocalId(0);
            functions.nil.push((
                index,
                lowered_host_target::<execution_function::NilFunctionBody>(
                    key,
                    specialization,
                    return_,
                ),
            ));
        }
        StoredValueShape::Tuple(_) => {
            let return_ = execution_graph::TupleLocalId(0);
            functions.tuple.push((
                index,
                lowered_host_target::<execution_function::TupleFunctionBody>(
                    key,
                    specialization,
                    return_,
                ),
            ));
        }
        StoredValueShape::List(item) => {
            match function::list_function_id(&item, index, &mut context.types) {
                L::Parameter(id) => functions.parameter_list.push((
                    id,
                    lowered_host_target::<execution_function::ParameterListFunctionBody>(
                        key,
                        specialization,
                        execution_graph::ParameterListLocalId(0),
                    ),
                )),
                L::ParameterList(id) => functions.parameter_list_list.push((
                    id,
                    lowered_host_target::<execution_function::ParameterListListFunctionBody>(
                        key,
                        specialization,
                        execution_graph::ParameterListListLocalId(0),
                    ),
                )),
                L::Int(id) => functions.int_list.push((
                    id,
                    lowered_host_target::<execution_function::IntListFunctionBody>(
                        key,
                        specialization,
                        execution_graph::IntListLocalId(0),
                    ),
                )),
                L::String(id) => functions.string_list.push((
                    id,
                    lowered_host_target::<execution_function::StringListFunctionBody>(
                        key,
                        specialization,
                        execution_graph::StringListLocalId(0),
                    ),
                )),
                L::BitArray(id) => functions.bit_array_list.push((
                    id,
                    lowered_host_target::<execution_function::BitArrayListFunctionBody>(
                        key,
                        specialization,
                        execution_graph::BitArrayListLocalId(0),
                    ),
                )),
                L::UtfCodepoint(id) => functions.utf_codepoint_list.push((
                    id,
                    lowered_host_target::<execution_function::UtfCodepointListFunctionBody>(
                        key,
                        specialization,
                        execution_graph::UtfCodepointListLocalId(0),
                    ),
                )),
                L::Custom(id) => functions.custom_list.push((
                    id,
                    lowered_host_target::<execution_function::CustomListFunctionBody>(
                        key,
                        specialization,
                        execution_graph::CustomListLocalId(0),
                    ),
                )),
                L::Float(id) => functions.float_list.push((
                    id,
                    lowered_host_target::<execution_function::FloatListFunctionBody>(
                        key,
                        specialization,
                        execution_graph::FloatListLocalId(0),
                    ),
                )),
                L::Bool(id) => functions.bool_list.push((
                    id,
                    lowered_host_target::<execution_function::BoolListFunctionBody>(
                        key,
                        specialization,
                        execution_graph::BoolListLocalId(0),
                    ),
                )),
                L::Nil(id) => functions.nil_list.push((
                    id,
                    lowered_host_target::<execution_function::NilListFunctionBody>(
                        key,
                        specialization,
                        execution_graph::NilListLocalId(0),
                    ),
                )),
                L::Tuple(id) => functions.tuple_list.push((
                    id,
                    lowered_host_target::<execution_function::TupleListFunctionBody>(
                        key,
                        specialization,
                        execution_graph::TupleListLocalId(0),
                    ),
                )),
                L::List(id) => functions.list_list.push((
                    id,
                    lowered_host_target::<execution_function::ListListFunctionBody>(
                        key,
                        specialization,
                        execution_graph::ListListLocalId(0),
                    ),
                )),
                L::Function(id) => functions.function_list.push((
                    id,
                    lowered_host_target::<execution_function::FunctionListFunctionBody>(
                        key,
                        specialization,
                        execution_graph::FunctionListLocalId(0),
                    ),
                )),
            }
        }
        StoredValueShape::Function(function) => {
            lower_host_function_return(index, key, &function, specialization, functions, context)
        }
    }
}

fn lower_host_function_return<Profile: HostProfile>(
    index: usize,
    key: &SpecializationKey,
    function: &SpecializedFunctionShape,
    specialization: HostSpecialization,
    functions: &mut function::AdditionalFunctions<HostedExecutionProfile<Profile>>,
    context: &mut LoweringContext,
) {
    use local::SpecializedFunctionLocal as F;

    match local::function_local_at(function, 0, context) {
        F::Generic(return_) => functions.generic_function_functions.push((
            index,
            lowered_host_target::<execution_function::GenericFunctionFunctionBody>(
                key,
                specialization,
                return_,
            ),
        )),
        F::Never(return_) => functions.never_function_functions.push((
            index,
            lowered_host_target::<execution_function::NeverFunctionFunctionBody>(
                key,
                specialization,
                return_,
            ),
        )),
        F::Int { local: return_, .. } => functions.int_function_functions.push((
            index,
            lowered_host_target::<execution_function::IntFunctionFunctionBody>(
                key,
                specialization,
                return_,
            ),
        )),
        F::Float { local: return_, .. } => functions.float_function_functions.push((
            index,
            lowered_host_target::<execution_function::FloatFunctionFunctionBody>(
                key,
                specialization,
                return_,
            ),
        )),
        F::String { local: return_, .. } => functions.string_function_functions.push((
            index,
            lowered_host_target::<execution_function::StringFunctionFunctionBody>(
                key,
                specialization,
                return_,
            ),
        )),
        F::BitArray { local: return_, .. } => functions.bit_array_function_functions.push((
            index,
            lowered_host_target::<execution_function::BitArrayFunctionFunctionBody>(
                key,
                specialization,
                return_,
            ),
        )),
        F::UtfCodepoint { local: return_, .. } => {
            functions.utf_codepoint_function_functions.push((
                index,
                lowered_host_target::<execution_function::UtfCodepointFunctionFunctionBody>(
                    key,
                    specialization,
                    return_,
                ),
            ));
        }
        F::Custom(return_) => functions.custom_function_functions.push((
            index,
            lowered_host_target::<execution_function::CustomFunctionFunctionBody>(
                key,
                specialization,
                return_,
            ),
        )),
        F::Bool { local: return_, .. } => functions.bool_function_functions.push((
            index,
            lowered_host_target::<execution_function::BoolFunctionFunctionBody>(
                key,
                specialization,
                return_,
            ),
        )),
        F::Nil { local: return_, .. } => functions.nil_function_functions.push((
            index,
            lowered_host_target::<execution_function::NilFunctionFunctionBody>(
                key,
                specialization,
                return_,
            ),
        )),
        F::Tuple { local: return_, .. } => functions.tuple_function_functions.push((
            index,
            lowered_host_target::<execution_function::TupleFunctionFunctionBody>(
                key,
                specialization,
                return_,
            ),
        )),
        F::List(return_) => {
            use execution_graph::ListFunctionLocal as L;

            let lowered = lowered_host_target::<execution_function::ListFunctionFunctionBody>(
                key,
                specialization,
                return_.clone(),
            );
            match return_ {
                L::Parameter { .. } => functions
                    .parameter_list_function_functions
                    .push((index, lowered)),
                L::ParameterList { .. } => functions
                    .parameter_list_list_function_functions
                    .push((index, lowered)),
                L::Int { .. } => functions.int_list_function_functions.push((index, lowered)),
                L::String { .. } => functions
                    .string_list_function_functions
                    .push((index, lowered)),
                L::BitArray { .. } => functions
                    .bit_array_list_function_functions
                    .push((index, lowered)),
                L::UtfCodepoint { .. } => functions
                    .utf_codepoint_list_function_functions
                    .push((index, lowered)),
                L::Custom { .. } => functions
                    .custom_list_function_functions
                    .push((index, lowered)),
                L::Float { .. } => functions
                    .float_list_function_functions
                    .push((index, lowered)),
                L::Bool { .. } => functions
                    .bool_list_function_functions
                    .push((index, lowered)),
                L::Nil { .. } => functions.nil_list_function_functions.push((index, lowered)),
                L::Tuple { .. } => functions
                    .tuple_list_function_functions
                    .push((index, lowered)),
                L::List { .. } => functions
                    .list_list_function_functions
                    .push((index, lowered)),
                L::Function { .. } => functions
                    .function_list_function_functions
                    .push((index, lowered)),
            }
        }
        F::Function(return_) => functions.function_function_functions.push((
            index,
            lowered_host_target::<execution_function::FunctionFunctionFunctionBody>(
                key,
                specialization,
                return_,
            ),
        )),
    }
}

fn lowered_host_target<Body>(
    key: &SpecializationKey,
    specialization: HostSpecialization,
    return_: Body::Return,
) -> function::LoweredSpecialization<
    execution_function::ValueFunctionEntry<Body, HostedFunctionTarget<Body>>,
>
where
    Body: execution_function::ExecutionFunctionBody,
{
    match specialization {
        HostSpecialization::Value(index) => function::lowered_host_function(
            key,
            HostedFunctionTarget::value(HostFunctionId::<Body>::new(index, return_)),
        ),
        HostSpecialization::Never(index) => function::lowered_host_function(
            key,
            HostedFunctionTarget::never(HostNeverFunctionId::new(index)),
        ),
    }
}

fn lowered_never_host_target(
    key: &SpecializationKey,
    index: usize,
) -> function::LoweredSpecialization<
    execution_function::ValueFunctionEntry<
        execution_function::NeverFunctionBody,
        HostedFunctionTarget<execution_function::NeverFunctionBody>,
    >,
> {
    function::lowered_host_function(
        key,
        HostedFunctionTarget::never(HostNeverFunctionId::new(index)),
    )
}

impl<Profile: HostProfile> RegisteredHostFunctions<Profile> {
    fn new(implementation_bindings: Vec<HostImplementationBinding<Profile>>) -> Self {
        Self {
            functions: implementation_bindings
                .into_iter()
                .map(HostImplementationBinding::into_parts)
                .collect(),
        }
    }
}

impl<Profile: HostProfile> LoweredHostFunctions<Profile> {
    fn new() -> Self {
        Self {
            value_functions: Vec::new(),
            never_functions: Vec::new(),
            additional: function::AdditionalFunctions {
                never: Vec::new(),
                custom: Vec::new(),
                int: Vec::new(),
                float: Vec::new(),
                string: Vec::new(),
                bit_array: Vec::new(),
                utf_codepoint: Vec::new(),
                bool: Vec::new(),
                nil: Vec::new(),
                tuple: Vec::new(),
                parameter_list: Vec::new(),
                int_list: Vec::new(),
                string_list: Vec::new(),
                bit_array_list: Vec::new(),
                utf_codepoint_list: Vec::new(),
                custom_list: Vec::new(),
                float_list: Vec::new(),
                bool_list: Vec::new(),
                nil_list: Vec::new(),
                tuple_list: Vec::new(),
                parameter_list_list: Vec::new(),
                list_list: Vec::new(),
                function_list: Vec::new(),
                int_function_functions: Vec::new(),
                float_function_functions: Vec::new(),
                string_function_functions: Vec::new(),
                bit_array_function_functions: Vec::new(),
                utf_codepoint_function_functions: Vec::new(),
                custom_function_functions: Vec::new(),
                bool_function_functions: Vec::new(),
                nil_function_functions: Vec::new(),
                tuple_function_functions: Vec::new(),
                generic_function_functions: Vec::new(),
                never_function_functions: Vec::new(),
                parameter_list_function_functions: Vec::new(),
                parameter_list_list_function_functions: Vec::new(),
                int_list_function_functions: Vec::new(),
                string_list_function_functions: Vec::new(),
                bit_array_list_function_functions: Vec::new(),
                utf_codepoint_list_function_functions: Vec::new(),
                custom_list_function_functions: Vec::new(),
                float_list_function_functions: Vec::new(),
                bool_list_function_functions: Vec::new(),
                nil_list_function_functions: Vec::new(),
                tuple_list_function_functions: Vec::new(),
                list_list_function_functions: Vec::new(),
                function_list_function_functions: Vec::new(),
                function_function_functions: Vec::new(),
            },
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
