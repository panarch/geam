mod constant;
mod function;
mod graph;
mod host;
mod local;
mod specialization;
mod value_type;

use super::type_::{CustomTypeTable, ExternalTypeTable, ListTypeTable, ValueShapeTable};
use super::{ExecutionProgram, ExecutionProgramCommon};
use crate::plan::execution::function::ExecutionProfile;
use crate::plan::{ModulePlan, ValueShape};
use specialization::{
    RepresentationContext, SpecializationKey, SpecializedCustomConstructor,
    SpecializedCustomValueShape, SpecializedFunctionShape, SpecializedTypeSubstitution,
    SpecializedValueShape,
};
use std::collections::{HashMap, HashSet, VecDeque};
use std::convert::Infallible;

struct FunctionTemplates {
    templates: Vec<Vec<crate::plan::FunctionTemplate>>,
}

struct ProgramConstantTemplates {
    modules: Vec<crate::plan::ConstantTemplates>,
}

impl ProgramConstantTemplates {
    fn get(&self, module: crate::plan::ModuleId) -> &crate::plan::ConstantTemplates {
        &self.modules[module.index()]
    }
}

struct SpecializationState {
    constant_templates: ProgramConstantTemplates,
    representations: RepresentationContext,
    erased_specializations: HashSet<SpecializationKey>,
}

struct LoweredExecution<Profile: ExecutionProfile> {
    constants: super::constant::ProfiledConstantTable<Profile::Graph>,
    functions: super::function::FunctionTables<Profile>,
    list_types: ListTypeTable,
    custom_types: CustomTypeTable,
    external_types: ExternalTypeTable,
    value_shapes: ValueShapeTable,
}

#[derive(Debug, PartialEq, Eq)]
enum SpecializationOutcome<T> {
    Complete(T),
    RequiresErasure(HashSet<SpecializationKey>),
}

type LoweringCompletion<Execution> = (
    ProgramConstantTemplates,
    RepresentationContext,
    SpecializationOutcome<Box<Execution>>,
);
type PlainLoweredExecution = LoweredExecution<Infallible>;

pub(super) use host::lower_hosted;

pub(super) fn lower(module_plan: ModulePlan) -> ExecutionProgram<Infallible> {
    let parts = module_plan.into_parts();
    let root = parts.root;
    let entry = parts.entry;
    let mut module_contexts = Vec::with_capacity(parts.modules.len());
    let mut module_templates = Vec::with_capacity(parts.modules.len());
    let mut constant_templates = Vec::with_capacity(parts.modules.len());
    let mut custom_types = Vec::new();

    for module in parts.modules {
        let parts = module.into_parts();
        module_contexts.push(super::ExecutionModuleContext::new(
            parts.module,
            parts.source_context,
        ));
        custom_types.extend(parts.custom_types);
        constant_templates.push(parts.constants);
        let mut templates = parts.functions;
        templates.extend(parts.anonymous_functions);
        templates.sort_by_key(|template| template.id().index());
        module_templates.push(templates);
    }

    let templates = FunctionTemplates::new(module_templates);
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

    // Function indices remain provisional until a pass produces no new erasures.
    let (main, lowered) = resolve_specialization_fixed_point(initial, |state| {
        let SpecializationState {
            constant_templates,
            representations,
            erased_specializations,
        } = state;
        let main_value_shape = specialization::SpecializedValueShape::instantiate(
            &main_return_shape,
            main_key.substitution(),
        );
        let main_return_shape = representations.inhabitation(&main_value_shape);
        let mut context = LoweringContext::new(
            templates.entry_templates(),
            representations,
            constant_templates,
            main_key.clone(),
            erased_specializations,
        );

        let main = context.reserve_main(main_key.clone(), main_return_shape);

        while let Some(key) = context.pending.pop_front() {
            context.begin(&key);
            function::lower_specialized(templates.get(key.template()), &key, &mut context);
        }

        let (constant_templates, representations, lowered) = context.finish();
        let outcome = SpecializationOutcome::from_representability(
            graph::seal_plain_runtime_function_id(main),
            main_key.clone(),
        )
        .zip_with(lowered, |main, lowered| (main, lowered));
        let erased_specializations = outcome.erased_specializations();
        outcome.into_fixed_point(SpecializationState {
            constant_templates,
            representations,
            erased_specializations,
        })
    });

    ExecutionProgram {
        common: ExecutionProgramCommon {
            root,
            modules: module_contexts.into_boxed_slice(),
            main,
            constants: lowered.constants,
            list_types: lowered.list_types,
            custom_types: lowered.custom_types,
            external_types: lowered.external_types,
            value_shapes: lowered.value_shapes,
        },
        functions: lowered.functions,
    }
}

impl FunctionTemplates {
    fn new(templates: Vec<Vec<crate::plan::FunctionTemplate>>) -> Self {
        Self { templates }
    }

    fn get(&self, id: crate::plan::FunctionTemplateId) -> &crate::plan::FunctionTemplate {
        &self.templates[id.module().index()][id.index()]
    }

    fn entry_templates(
        &self,
    ) -> HashMap<crate::plan::FunctionTemplateId, local::FunctionEntryTemplate> {
        self.templates
            .iter()
            .flatten()
            .map(|template| (template.id(), local::FunctionEntryTemplate::new(template)))
            .collect()
    }
}

#[derive(Debug, PartialEq, Eq)]
enum FixedPointStep<State, Output> {
    Complete(Output),
    Continue(State),
}

#[derive(Clone)]
struct ProvisionalSpecialization {
    index: usize,
    parameters: Box<[specialization::StoredValueShape]>,
}

impl<T> SpecializationOutcome<T> {
    fn from_representability(
        value: specialization::Representability<T>,
        owner: SpecializationKey,
    ) -> Self {
        match value {
            specialization::Representability::Inhabited(value) => Self::Complete(value),
            specialization::Representability::Uninhabited => {
                Self::RequiresErasure(HashSet::from([owner]))
            }
        }
    }

    fn complete_unless_erased(value: T, erased: HashSet<SpecializationKey>) -> Self {
        if erased.is_empty() {
            Self::Complete(value)
        } else {
            Self::RequiresErasure(erased)
        }
    }

    fn zip_with<U, V>(
        self,
        other: SpecializationOutcome<U>,
        map: impl FnOnce(T, U) -> V,
    ) -> SpecializationOutcome<V> {
        match (self, other) {
            (Self::Complete(left), SpecializationOutcome::Complete(right)) => {
                SpecializationOutcome::Complete(map(left, right))
            }
            (Self::RequiresErasure(mut left), SpecializationOutcome::RequiresErasure(right)) => {
                left.extend(right);
                SpecializationOutcome::RequiresErasure(left)
            }
            (Self::RequiresErasure(erased), SpecializationOutcome::Complete(_))
            | (Self::Complete(_), SpecializationOutcome::RequiresErasure(erased)) => {
                SpecializationOutcome::RequiresErasure(erased)
            }
        }
    }

    fn map<U>(self, map: impl FnOnce(T) -> U) -> SpecializationOutcome<U> {
        match self {
            Self::Complete(value) => SpecializationOutcome::Complete(map(value)),
            Self::RequiresErasure(erased) => SpecializationOutcome::RequiresErasure(erased),
        }
    }

    fn include_prior_erasure(self, mut prior: HashSet<SpecializationKey>) -> Self {
        match self {
            Self::Complete(value) => Self::Complete(value),
            Self::RequiresErasure(erased) => {
                prior.extend(erased);
                Self::RequiresErasure(prior)
            }
        }
    }

    fn erased_specializations(&self) -> HashSet<SpecializationKey> {
        match self {
            Self::Complete(_) => HashSet::new(),
            Self::RequiresErasure(erased) => erased.clone(),
        }
    }

    fn into_fixed_point<State>(self, continue_with: State) -> FixedPointStep<State, T> {
        match self {
            Self::Complete(value) => FixedPointStep::Complete(value),
            Self::RequiresErasure(_) => FixedPointStep::Continue(continue_with),
        }
    }
}

fn resolve_specialization_fixed_point<State, Output>(
    mut state: State,
    mut lower: impl FnMut(State) -> FixedPointStep<State, Output>,
) -> Output {
    loop {
        match lower(state) {
            FixedPointStep::Complete(output) => return output,
            FixedPointStep::Continue(next) => state = next,
        }
    }
}

fn try_resolve_specialization_fixed_point<State, Output, Error>(
    mut state: State,
    mut lower: impl FnMut(State) -> Result<FixedPointStep<State, Output>, Error>,
) -> Result<Output, Error> {
    loop {
        match lower(state)? {
            FixedPointStep::Complete(output) => return Ok(output),
            FixedPointStep::Continue(next) => state = next,
        }
    }
}

struct LoweringContext {
    constant_templates: ProgramConstantTemplates,
    constants: constant::ConstantLowering,
    types: value_type::TypeInterner,
    representations: RepresentationContext,
    functions: function::FunctionTableBuilder,
    entry_templates: HashMap<crate::plan::FunctionTemplateId, local::FunctionEntryTemplate>,
    next_function_indices: HashMap<function::FunctionTableFamily, usize>,
    provisional_specializations: HashMap<SpecializationKey, ProvisionalSpecialization>,
    erased_specializations: HashSet<SpecializationKey>,
    pending: VecDeque<SpecializationKey>,
    substitution: SpecializedTypeSubstitution,
    current_specialization: SpecializationKey,
}

struct StoredTargetLocal {
    index: usize,
    shape: specialization::StoredValueShape,
}

impl StoredTargetLocal {
    fn index(&self) -> usize {
        self.index
    }

    fn shape(&self) -> &specialization::StoredValueShape {
        &self.shape
    }
}

impl LoweringContext {
    fn new(
        entry_templates: HashMap<crate::plan::FunctionTemplateId, local::FunctionEntryTemplate>,
        representations: RepresentationContext,
        constant_templates: ProgramConstantTemplates,
        current_specialization: SpecializationKey,
        erased_specializations: HashSet<SpecializationKey>,
    ) -> Self {
        Self {
            constant_templates,
            constants: constant::ConstantLowering::default(),
            types: value_type::TypeInterner::new(),
            representations,
            functions: function::FunctionTableBuilder::default(),
            entry_templates,
            next_function_indices: HashMap::new(),
            provisional_specializations: HashMap::new(),
            erased_specializations,
            pending: VecDeque::new(),
            substitution: SpecializedTypeSubstitution::empty(),
            current_specialization,
        }
    }

    fn target_capture_local(
        &mut self,
        instantiation: &crate::plan::FunctionInstantiation,
        position: crate::plan::CapturePosition,
        source_shape: specialization::StoredValueShape,
    ) -> StoredTargetLocal {
        let (specialization, _) =
            SpecializationKey::from_instantiation(instantiation, &self.substitution);
        let (index, shape) = self.entry_templates[&specialization.template()].capture_target(
            position,
            source_shape,
            specialization.substitution(),
            &self.representations,
        );
        StoredTargetLocal { index, shape }
    }

    fn concrete_parameter(&self, parameter: crate::plan::TypeParameterId) -> SpecializedValueShape {
        SpecializedValueShape::instantiate(&ValueShape::Parameter(parameter), &self.substitution)
    }

    fn function_shape(&mut self, shape: crate::plan::FunctionShape) -> super::type_::FunctionShape {
        self.types
            .function_shape(&SpecializedFunctionShape::instantiate(
                &shape,
                &self.substitution,
            ))
    }

    fn custom_function_type(
        &mut self,
        type_: crate::plan::CustomFunctionType,
    ) -> super::type_::CustomFunctionType {
        let substitution = self.substitution.clone();
        self.custom_function_type_with_substitution(&type_, &substitution)
    }

    fn generic_function_type(
        &mut self,
        shape: &SpecializedFunctionShape,
    ) -> super::type_::GenericFunctionType {
        self.types.generic_function_type(shape)
    }

    fn specialized_custom_function_type(
        &mut self,
        arguments: &[SpecializedValueShape],
        return_: &SpecializedCustomValueShape,
    ) -> super::type_::CustomFunctionType {
        self.types.custom_function_type(arguments, return_)
    }

    fn specialized_external_function_type(
        &mut self,
        arguments: &[SpecializedValueShape],
        return_: &specialization::SpecializedExternalValueShape,
    ) -> super::type_::ExternalFunctionType {
        self.types.external_function_type(arguments, return_)
    }

    fn specialized_function_function_type(
        &mut self,
        arguments: &[SpecializedValueShape],
        return_: &SpecializedFunctionShape,
    ) -> super::type_::FunctionFunctionType {
        self.types.function_function_type(arguments, return_)
    }

    fn function_representation(
        &self,
        shape: &SpecializedFunctionShape,
    ) -> specialization::FunctionRepresentation {
        shape.representation(&self.representations)
    }

    fn function_arguments_representation(
        &self,
        shape: &SpecializedFunctionShape,
    ) -> specialization::FunctionArgumentsRepresentation {
        shape.arguments_representation(&self.representations)
    }

    fn generic_callable_id(
        &mut self,
        function: &crate::plan::FunctionInstantiation,
    ) -> super::function::GenericCallableId {
        let (key, _) = SpecializationKey::from_instantiation(function, &self.substitution);
        super::function::GenericCallableId::function(
            key.template().index(),
            key.substitution()
                .arguments()
                .iter()
                .map(|shape| self.types.value_shape(shape))
                .collect(),
        )
    }

    fn generic_constructor_callable_id(
        &mut self,
        constructor: crate::plan::CustomConstructor,
    ) -> super::function::GenericCallableId {
        super::function::GenericCallableId::constructor(self.custom_constructor(constructor))
    }

    fn custom_function_type_with_substitution(
        &mut self,
        type_: &crate::plan::CustomFunctionType,
        substitution: &SpecializedTypeSubstitution,
    ) -> super::type_::CustomFunctionType {
        let arguments = type_
            .argument_shapes()
            .iter()
            .map(|shape| SpecializedValueShape::instantiate(shape, substitution))
            .collect::<Vec<_>>();
        let return_ = SpecializedCustomValueShape::instantiate(type_.return_(), substitution);
        self.types.custom_function_type(&arguments, &return_)
    }

    fn int_list_type(&mut self) -> super::type_::IntListTypeId {
        self.types.int_list_type()
    }

    fn string_list_type(&mut self) -> super::type_::StringListTypeId {
        self.types.string_list_type()
    }

    fn bit_array_list_type(&mut self) -> super::type_::BitArrayListTypeId {
        self.types.bit_array_list_type()
    }

    fn utf_codepoint_list_type(&mut self) -> super::type_::UtfCodepointListTypeId {
        self.types.utf_codepoint_list_type()
    }

    fn custom_constructor(
        &mut self,
        constructor: crate::plan::CustomConstructor,
    ) -> super::type_::CustomConstructorId {
        self.types
            .custom_constructor(SpecializedCustomConstructor::instantiate(
                constructor,
                &self.substitution,
            ))
    }

    fn custom_list_type(
        &mut self,
        item: crate::plan::CustomType,
    ) -> super::type_::CustomListTypeId {
        let shape = SpecializedCustomValueShape::instantiate(
            &crate::plan::CustomValueShape::any(item),
            &self.substitution,
        );
        self.types.custom_list_type(&shape)
    }

    fn specialized_custom_list_type(
        &mut self,
        item: &SpecializedCustomValueShape,
    ) -> super::type_::CustomListTypeId {
        self.types.custom_list_type(item)
    }

    fn specialized_external_list_type(
        &mut self,
        item: &specialization::SpecializedExternalValueShape,
    ) -> super::type_::ExternalListTypeId {
        self.types.external_list_type(item)
    }

    fn external_list_type(
        &mut self,
        item: crate::plan::ExternalType,
    ) -> super::type_::ExternalListTypeId {
        let shape = specialization::SpecializedExternalValueShape::instantiate(
            &crate::plan::ExternalValueShape::any(item),
            &self.substitution,
        );
        self.types.external_list_type(&shape)
    }

    fn float_list_type(&mut self) -> super::type_::FloatListTypeId {
        self.types.float_list_type()
    }

    fn bool_list_type(&mut self) -> super::type_::BoolListTypeId {
        self.types.bool_list_type()
    }

    fn nil_list_type(&mut self) -> super::type_::NilListTypeId {
        self.types.nil_list_type()
    }

    fn tuple_list_type(
        &mut self,
        item: Vec<crate::plan::ValueType>,
    ) -> super::type_::TupleListTypeId {
        let item = item
            .into_iter()
            .map(|type_| {
                SpecializedValueShape::instantiate(
                    &ValueShape::from_value_type(type_),
                    &self.substitution,
                )
            })
            .collect::<Vec<_>>();
        self.types.tuple_list_type(&item)
    }

    fn specialized_tuple_list_type(
        &mut self,
        item: &[SpecializedValueShape],
    ) -> super::type_::TupleListTypeId {
        self.types.tuple_list_type(item)
    }

    fn specialized_list_list_type(
        &mut self,
        item: &SpecializedValueShape,
    ) -> value_type::NestedListTypeId {
        self.types.list_list_type(item)
    }

    fn parameter_list_type(
        &mut self,
        parameter: crate::plan::TypeParameterId,
    ) -> super::type_::ParameterListTypeId {
        self.types.parameter_list_type(parameter)
    }

    fn parameter_list_list_type(
        &mut self,
        parameter: crate::plan::TypeParameterId,
    ) -> super::type_::ParameterListListTypeId {
        self.types.parameter_list_list_type(parameter)
    }

    fn stored_list_list_type(
        &mut self,
        item: &crate::plan::ValueStorageShape,
    ) -> super::type_::ListListTypeId {
        let item = specialization::StoredValueShape::instantiate(item, &self.substitution);
        self.types.stored_list_list_type(&item)
    }

    fn specialized_stored_list_list_type(
        &mut self,
        item: &specialization::StoredValueShape,
    ) -> super::type_::ListListTypeId {
        self.types.stored_list_list_type(item)
    }

    fn function_list_type(
        &mut self,
        item: crate::plan::FunctionType,
    ) -> super::type_::FunctionListTypeId {
        let shape = SpecializedFunctionShape::instantiate(
            &crate::plan::FunctionShape::from_function_type(item),
            &self.substitution,
        );
        self.types.function_list_type(&shape)
    }

    fn specialized_function_list_type(
        &mut self,
        item: &SpecializedFunctionShape,
    ) -> super::type_::FunctionListTypeId {
        self.types.function_list_type(item)
    }

    fn reserve_main(
        &mut self,
        key: SpecializationKey,
        return_: specialization::ValueInhabitation,
    ) -> super::function::RuntimeFunctionId {
        match return_ {
            specialization::ValueInhabitation::Uninhabited(_) => {
                let specialization = self.reserve_provisional_specialization(
                    key,
                    function::FunctionTableFamily::Never,
                    Box::new([]),
                );
                super::function::RuntimeFunctionId::Core(
                    super::function::CoreRuntimeFunctionId::Never(
                        super::function::NeverFunctionId(specialization.index),
                    ),
                )
            }
            specialization::ValueInhabitation::Inhabited(return_shape) => {
                let family =
                    function::stored_function_table_family(&return_shape, &self.representations);
                let specialization =
                    self.reserve_provisional_specialization(key, family, Box::new([]));
                function::function_id(
                    &return_shape,
                    specialization.index,
                    &mut self.types,
                    &self.representations,
                )
            }
        }
    }

    fn provisional_specialization(
        &mut self,
        key: SpecializationKey,
        family: function::FunctionTableFamily,
    ) -> specialization::Representability<ProvisionalSpecialization> {
        if self.erased_specializations.contains(&key) {
            specialization::Representability::Uninhabited
        } else {
            let parameters = self.entry_templates[&key.template()]
                .stored_parameters(key.substitution(), &self.representations);
            parameters
                .map(|parameters| self.reserve_provisional_specialization(key, family, parameters))
        }
    }

    fn reserve_provisional_specialization(
        &mut self,
        key: SpecializationKey,
        family: function::FunctionTableFamily,
        parameters: Box<[specialization::StoredValueShape]>,
    ) -> ProvisionalSpecialization {
        match self.provisional_specializations.get(&key) {
            Some(specialization) => specialization.clone(),
            None => {
                let index = self.next_function_index(family);
                let specialization = ProvisionalSpecialization { index, parameters };
                self.provisional_specializations
                    .insert(key.clone(), specialization.clone());
                self.pending.push_back(key);
                specialization
            }
        }
    }

    fn specialization_parameters(
        &self,
        key: &SpecializationKey,
    ) -> &[specialization::StoredValueShape] {
        &self.provisional_specializations[key].parameters
    }

    fn reserve_index_for(
        &mut self,
        instantiation: &crate::plan::FunctionInstantiation,
        family: function::FunctionTableFamily,
    ) -> specialization::Representability<usize> {
        let (key, _) = SpecializationKey::from_instantiation(instantiation, &self.substitution);
        self.provisional_specialization(key, family)
            .map(|specialization| specialization.index)
    }

    fn never_function_id(
        &mut self,
        function: &crate::plan::FunctionInstantiation,
    ) -> specialization::Representability<super::function::NeverFunctionId> {
        let (key, _) = SpecializationKey::from_instantiation(function, &self.substitution);
        self.provisional_specialization(key, function::FunctionTableFamily::Never)
            .map(|specialization| super::function::NeverFunctionId(specialization.index))
    }

    fn next_function_index(&mut self, family: function::FunctionTableFamily) -> usize {
        let next = self.next_function_indices.entry(family).or_default();
        let index = *next;
        *next += 1;
        index
    }

    fn reserve_function_id<Function>(
        &mut self,
        function: &crate::plan::FunctionInstantiation,
        family: function::FunctionTableFamily,
        lower: impl FnOnce(usize, &mut Self) -> Function,
    ) -> specialization::Representability<Function> {
        self.reserve_index_for(function, family)
            .map(|index| lower(index, self))
    }

    fn int_function_id(
        &mut self,
        function: &crate::plan::FunctionInstantiation,
    ) -> specialization::Representability<super::function::IntFunctionId> {
        self.reserve_function_id(function, function::FunctionTableFamily::Int, |index, _| {
            super::function::IntFunctionId(index)
        })
    }

    fn float_function_id(
        &mut self,
        function: &crate::plan::FunctionInstantiation,
    ) -> specialization::Representability<super::function::FloatFunctionId> {
        self.reserve_function_id(
            function,
            function::FunctionTableFamily::Float,
            |index, _| super::function::FloatFunctionId(index),
        )
    }

    fn string_function_id(
        &mut self,
        function: &crate::plan::FunctionInstantiation,
    ) -> specialization::Representability<super::function::StringFunctionId> {
        self.reserve_function_id(
            function,
            function::FunctionTableFamily::String,
            |index, _| super::function::StringFunctionId(index),
        )
    }

    fn bit_array_function_id(
        &mut self,
        function: &crate::plan::FunctionInstantiation,
    ) -> specialization::Representability<super::function::BitArrayFunctionId> {
        self.reserve_function_id(
            function,
            function::FunctionTableFamily::BitArray,
            |index, _| super::function::BitArrayFunctionId(index),
        )
    }

    fn utf_codepoint_function_id(
        &mut self,
        function: &crate::plan::FunctionInstantiation,
    ) -> specialization::Representability<super::function::UtfCodepointFunctionId> {
        self.reserve_function_id(
            function,
            function::FunctionTableFamily::UtfCodepoint,
            |index, _| super::function::UtfCodepointFunctionId(index),
        )
    }

    fn custom_function_id(
        &mut self,
        function: &crate::plan::FunctionInstantiation,
        shape: &SpecializedCustomValueShape,
    ) -> specialization::Representability<super::function::CustomFunctionId> {
        self.reserve_function_id(
            function,
            function::FunctionTableFamily::Custom,
            |index, context| {
                super::function::CustomFunctionId::new(
                    index,
                    context.types.custom_value_shape(shape),
                )
            },
        )
    }

    fn external_function_id(
        &mut self,
        function: &crate::plan::FunctionInstantiation,
        shape: &specialization::SpecializedExternalValueShape,
    ) -> specialization::Representability<super::function::ExternalFunctionId> {
        self.reserve_function_id(
            function,
            function::FunctionTableFamily::External,
            |index, context| {
                super::function::ExternalFunctionId::new(index, context.types.external_type(shape))
            },
        )
    }

    fn bool_function_id(
        &mut self,
        function: &crate::plan::FunctionInstantiation,
    ) -> specialization::Representability<super::function::BoolFunctionId> {
        self.reserve_function_id(function, function::FunctionTableFamily::Bool, |index, _| {
            super::function::BoolFunctionId(index)
        })
    }

    fn nil_function_id(
        &mut self,
        function: &crate::plan::FunctionInstantiation,
    ) -> specialization::Representability<super::function::NilFunctionId> {
        self.reserve_function_id(function, function::FunctionTableFamily::Nil, |index, _| {
            super::function::NilFunctionId(index)
        })
    }

    fn tuple_function_id(
        &mut self,
        function: &crate::plan::FunctionInstantiation,
    ) -> specialization::Representability<super::function::TupleFunctionId> {
        self.reserve_function_id(
            function,
            function::FunctionTableFamily::Tuple,
            |index, _| super::function::TupleFunctionId(index),
        )
    }

    fn list_function_id(
        &mut self,
        function: &crate::plan::FunctionInstantiation,
        item: &SpecializedValueShape,
    ) -> specialization::Representability<super::function::RuntimeListFunctionId> {
        self.reserve_function_id(
            function,
            function::list_function_table_family(item),
            |index, context| function::list_function_id(item, index, &mut context.types),
        )
    }

    fn int_list_function_id(
        &mut self,
        function: &crate::plan::FunctionInstantiation,
    ) -> specialization::Representability<super::function::IntListFunctionId> {
        self.reserve_function_id(
            function,
            function::FunctionTableFamily::IntList,
            |index, context| {
                super::function::IntListFunctionId::new(index, context.types.int_list_type())
            },
        )
    }

    fn parameter_list_function_id(
        &mut self,
        function: &crate::plan::FunctionInstantiation,
        parameter: crate::plan::TypeParameterId,
    ) -> specialization::Representability<super::function::ParameterListFunctionId> {
        self.reserve_function_id(
            function,
            function::FunctionTableFamily::ParameterList,
            |index, context| {
                super::function::ParameterListFunctionId::new(
                    index,
                    context.types.parameter_list_type(parameter),
                )
            },
        )
    }

    fn string_list_function_id(
        &mut self,
        function: &crate::plan::FunctionInstantiation,
    ) -> specialization::Representability<super::function::StringListFunctionId> {
        self.reserve_function_id(
            function,
            function::FunctionTableFamily::StringList,
            |index, context| {
                super::function::StringListFunctionId::new(index, context.types.string_list_type())
            },
        )
    }

    fn bit_array_list_function_id(
        &mut self,
        function: &crate::plan::FunctionInstantiation,
    ) -> specialization::Representability<super::function::BitArrayListFunctionId> {
        self.reserve_function_id(
            function,
            function::FunctionTableFamily::BitArrayList,
            |index, context| {
                super::function::BitArrayListFunctionId::new(
                    index,
                    context.types.bit_array_list_type(),
                )
            },
        )
    }

    fn utf_codepoint_list_function_id(
        &mut self,
        function: &crate::plan::FunctionInstantiation,
    ) -> specialization::Representability<super::function::UtfCodepointListFunctionId> {
        self.reserve_function_id(
            function,
            function::FunctionTableFamily::UtfCodepointList,
            |index, context| {
                super::function::UtfCodepointListFunctionId::new(
                    index,
                    context.types.utf_codepoint_list_type(),
                )
            },
        )
    }

    fn custom_list_function_id(
        &mut self,
        function: &crate::plan::FunctionInstantiation,
        type_id: super::type_::CustomListTypeId,
    ) -> specialization::Representability<super::function::CustomListFunctionId> {
        self.reserve_function_id(
            function,
            function::FunctionTableFamily::CustomList,
            |index, _| super::function::CustomListFunctionId::new(index, type_id),
        )
    }

    fn external_list_function_id(
        &mut self,
        function: &crate::plan::FunctionInstantiation,
        type_id: super::type_::ExternalListTypeId,
    ) -> specialization::Representability<super::function::ExternalListFunctionId> {
        self.reserve_function_id(
            function,
            function::FunctionTableFamily::ExternalList,
            |index, _| super::function::ExternalListFunctionId::new(index, type_id),
        )
    }

    fn float_list_function_id(
        &mut self,
        function: &crate::plan::FunctionInstantiation,
    ) -> specialization::Representability<super::function::FloatListFunctionId> {
        self.reserve_function_id(
            function,
            function::FunctionTableFamily::FloatList,
            |index, context| {
                super::function::FloatListFunctionId::new(index, context.types.float_list_type())
            },
        )
    }

    fn bool_list_function_id(
        &mut self,
        function: &crate::plan::FunctionInstantiation,
    ) -> specialization::Representability<super::function::BoolListFunctionId> {
        self.reserve_function_id(
            function,
            function::FunctionTableFamily::BoolList,
            |index, context| {
                super::function::BoolListFunctionId::new(index, context.types.bool_list_type())
            },
        )
    }

    fn nil_list_function_id(
        &mut self,
        function: &crate::plan::FunctionInstantiation,
    ) -> specialization::Representability<super::function::NilListFunctionId> {
        self.reserve_function_id(
            function,
            function::FunctionTableFamily::NilList,
            |index, context| {
                super::function::NilListFunctionId::new(index, context.types.nil_list_type())
            },
        )
    }

    fn tuple_list_function_id(
        &mut self,
        function: &crate::plan::FunctionInstantiation,
        type_id: super::type_::TupleListTypeId,
    ) -> specialization::Representability<super::function::TupleListFunctionId> {
        self.reserve_function_id(
            function,
            function::FunctionTableFamily::TupleList,
            |index, _| super::function::TupleListFunctionId::new(index, type_id),
        )
    }

    fn list_list_function_id(
        &mut self,
        function: &crate::plan::FunctionInstantiation,
        type_id: super::type_::ListListTypeId,
    ) -> specialization::Representability<super::function::ListListFunctionId> {
        self.reserve_function_id(
            function,
            function::FunctionTableFamily::ListList,
            |index, _| super::function::ListListFunctionId::new(index, type_id),
        )
    }

    fn parameter_list_list_function_id(
        &mut self,
        function: &crate::plan::FunctionInstantiation,
        type_id: super::type_::ParameterListListTypeId,
    ) -> specialization::Representability<super::function::ParameterListListFunctionId> {
        self.reserve_function_id(
            function,
            function::FunctionTableFamily::ParameterListList,
            |index, _| super::function::ParameterListListFunctionId::new(index, type_id),
        )
    }

    fn function_list_function_id(
        &mut self,
        function: &crate::plan::FunctionInstantiation,
        type_id: super::type_::FunctionListTypeId,
    ) -> specialization::Representability<super::function::FunctionListFunctionId> {
        self.reserve_function_id(
            function,
            function::FunctionTableFamily::FunctionList,
            |index, _| super::function::FunctionListFunctionId::new(index, type_id),
        )
    }

    fn function_function_id(
        &mut self,
        function: &crate::plan::FunctionInstantiation,
        return_: &SpecializedFunctionShape,
    ) -> specialization::Representability<super::function::FunctionFunctionId> {
        self.reserve_function_id(
            function,
            function::function_function_table_family(return_, &self.representations),
            |index, context| {
                function::function_function_id(
                    return_,
                    index,
                    &mut context.types,
                    &context.representations,
                )
            },
        )
    }

    fn int_function_function_id(
        &mut self,
        function: &crate::plan::FunctionInstantiation,
    ) -> specialization::Representability<super::function::IntFunctionFunctionId> {
        self.reserve_function_id(
            function,
            function::FunctionTableFamily::IntFunction,
            |index, _| super::function::IntFunctionFunctionId(index),
        )
    }

    fn float_function_function_id(
        &mut self,
        function: &crate::plan::FunctionInstantiation,
    ) -> specialization::Representability<super::function::FloatFunctionFunctionId> {
        self.reserve_function_id(
            function,
            function::FunctionTableFamily::FloatFunction,
            |index, _| super::function::FloatFunctionFunctionId(index),
        )
    }

    fn string_function_function_id(
        &mut self,
        function: &crate::plan::FunctionInstantiation,
    ) -> specialization::Representability<super::function::StringFunctionFunctionId> {
        self.reserve_function_id(
            function,
            function::FunctionTableFamily::StringFunction,
            |index, _| super::function::StringFunctionFunctionId(index),
        )
    }

    fn bit_array_function_function_id(
        &mut self,
        function: &crate::plan::FunctionInstantiation,
    ) -> specialization::Representability<super::function::BitArrayFunctionFunctionId> {
        self.reserve_function_id(
            function,
            function::FunctionTableFamily::BitArrayFunction,
            |index, _| super::function::BitArrayFunctionFunctionId(index),
        )
    }

    fn utf_codepoint_function_function_id(
        &mut self,
        function: &crate::plan::FunctionInstantiation,
    ) -> specialization::Representability<super::function::UtfCodepointFunctionFunctionId> {
        self.reserve_function_id(
            function,
            function::FunctionTableFamily::UtfCodepointFunction,
            |index, _| super::function::UtfCodepointFunctionFunctionId(index),
        )
    }

    fn custom_function_function_id(
        &mut self,
        function: &crate::plan::FunctionInstantiation,
        type_: super::type_::CustomFunctionType,
    ) -> specialization::Representability<super::function::CustomFunctionFunctionId> {
        self.reserve_function_id(
            function,
            function::FunctionTableFamily::CustomFunction,
            |index, _| super::function::CustomFunctionFunctionId::new(index, type_),
        )
    }

    fn external_function_function_id(
        &mut self,
        function: &crate::plan::FunctionInstantiation,
        type_: super::type_::ExternalFunctionType,
    ) -> specialization::Representability<super::function::ExternalFunctionFunctionId> {
        self.reserve_function_id(
            function,
            function::FunctionTableFamily::ExternalFunction,
            |index, _| super::function::ExternalFunctionFunctionId::new(index, type_),
        )
    }

    fn bool_function_function_id(
        &mut self,
        function: &crate::plan::FunctionInstantiation,
    ) -> specialization::Representability<super::function::BoolFunctionFunctionId> {
        self.reserve_function_id(
            function,
            function::FunctionTableFamily::BoolFunction,
            |index, _| super::function::BoolFunctionFunctionId(index),
        )
    }

    fn nil_function_function_id(
        &mut self,
        function: &crate::plan::FunctionInstantiation,
    ) -> specialization::Representability<super::function::NilFunctionFunctionId> {
        self.reserve_function_id(
            function,
            function::FunctionTableFamily::NilFunction,
            |index, _| super::function::NilFunctionFunctionId(index),
        )
    }

    fn tuple_function_function_id(
        &mut self,
        function: &crate::plan::FunctionInstantiation,
    ) -> specialization::Representability<super::function::TupleFunctionFunctionId> {
        self.reserve_function_id(
            function,
            function::FunctionTableFamily::TupleFunction,
            |index, _| super::function::TupleFunctionFunctionId(index),
        )
    }

    fn list_function_function_id(
        &mut self,
        function: &crate::plan::FunctionInstantiation,
        type_: &SpecializedFunctionShape,
        item: &SpecializedValueShape,
    ) -> specialization::Representability<super::function::ListFunctionFunctionId> {
        let signature = self.list_function_function_signature(type_, item);
        self.reserve_function_id(function, signature.table_family(), |index, _| {
            signature.hosted_id(index)
        })
    }

    fn list_function_function_signature(
        &mut self,
        function: &SpecializedFunctionShape,
        item: &SpecializedValueShape,
    ) -> function::ListFunctionFunctionSignature {
        function::list_function_function_signature(function, item, &mut self.types)
    }

    fn core_list_function_function_id(
        &mut self,
        function: &crate::plan::FunctionInstantiation,
        signature: &function::CoreListFunctionFunctionSignature,
    ) -> specialization::Representability<super::function::ProfiledListFunctionFunctionId<Infallible>>
    {
        self.reserve_function_id(function, signature.table_family(), |index, _| {
            signature.profiled_id(index)
        })
    }

    fn external_list_function_function_id(
        &mut self,
        function: &crate::plan::FunctionInstantiation,
        signature: &function::ExternalListFunctionFunctionSignature,
    ) -> specialization::Representability<super::function::ExternalListFunctionFunctionId> {
        self.reserve_function_id(
            function,
            function::FunctionTableFamily::ExternalListFunction,
            |index, _| signature.id(index),
        )
    }

    fn function_function_function_id(
        &mut self,
        function: &crate::plan::FunctionInstantiation,
        type_: super::type_::FunctionFunctionType,
    ) -> specialization::Representability<super::function::FunctionFunctionFunctionId> {
        self.reserve_function_id(
            function,
            function::FunctionTableFamily::FunctionFunction,
            |index, _| super::function::FunctionFunctionFunctionId::new(index, type_),
        )
    }

    fn generic_function_function_id(
        &mut self,
        function: &crate::plan::FunctionInstantiation,
        type_: super::type_::GenericFunctionType,
    ) -> specialization::Representability<super::function::GenericFunctionFunctionId> {
        self.reserve_function_id(
            function,
            function::FunctionTableFamily::GenericFunction,
            |index, _| super::function::GenericFunctionFunctionId::new(index, type_),
        )
    }

    fn never_function_function_id(
        &mut self,
        function: &crate::plan::FunctionInstantiation,
        type_: super::type_::GenericFunctionType,
    ) -> specialization::Representability<super::function::NeverFunctionFunctionId> {
        self.reserve_function_id(
            function,
            function::FunctionTableFamily::NeverFunction,
            |index, _| super::function::NeverFunctionFunctionId::new(index, type_),
        )
    }

    fn concrete_value_shape(&self, shape: &ValueShape) -> SpecializedValueShape {
        SpecializedValueShape::instantiate(shape, &self.substitution)
    }

    fn lower_concrete_value_type(
        &mut self,
        shape: &SpecializedValueShape,
    ) -> super::type_::ValueType {
        self.types.value_type(shape)
    }

    fn lower_concrete_custom_shape(
        &mut self,
        shape: &SpecializedCustomValueShape,
    ) -> super::type_::CustomValueShape {
        self.types.custom_value_shape(shape)
    }

    fn lower_concrete_external_type(
        &mut self,
        shape: &specialization::SpecializedExternalValueShape,
    ) -> super::type_::ExternalTypeId {
        self.types.external_type(shape)
    }

    fn lower_concrete_function_type(
        &mut self,
        shape: &SpecializedFunctionShape,
    ) -> super::type_::FunctionType {
        self.types.function_type(shape)
    }

    fn lower_concrete_function_shape(
        &mut self,
        shape: &SpecializedFunctionShape,
    ) -> super::type_::FunctionShape {
        self.types.function_shape(shape)
    }

    fn concrete_custom_value_shape(
        &self,
        shape: &crate::plan::CustomValueShape,
    ) -> SpecializedCustomValueShape {
        SpecializedCustomValueShape::instantiate(shape, &self.substitution)
    }

    fn concrete_external_value_shape(
        &self,
        shape: &crate::plan::ExternalValueShape,
    ) -> specialization::SpecializedExternalValueShape {
        specialization::SpecializedExternalValueShape::instantiate(shape, &self.substitution)
    }

    fn concrete_function_shape(
        &self,
        shape: &crate::plan::FunctionShape,
    ) -> SpecializedFunctionShape {
        SpecializedFunctionShape::instantiate(shape, &self.substitution)
    }

    fn begin(&mut self, key: &SpecializationKey) {
        self.substitution = key.substitution().clone();
        self.current_specialization = key.clone();
    }

    fn specialization_index(&self, key: &SpecializationKey) -> usize {
        self.provisional_specializations[key].index
    }

    fn finish(self) -> LoweringCompletion<PlainLoweredExecution> {
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
            .finish()
            .zip_with(constants.finish_plain(), |functions, constants| {
                let (list_types, custom_types, external_types, value_shapes) = types.into_tables();
                Box::new(LoweredExecution {
                    constants,
                    functions: *functions,
                    list_types,
                    custom_types,
                    external_types,
                    value_shapes,
                })
            })
            .include_prior_erasure(erased_specializations);
        (constant_templates, representations, outcome)
    }
}

#[cfg(test)]
pub(super) mod test_support {
    use super::specialization::RepresentationContext;
    use super::{FunctionTemplates, LoweringContext, ProgramConstantTemplates};
    use crate::plan::TypeParameterId;
    use std::collections::HashSet;

    pub(super) fn lowering_context(
        custom_types: Vec<crate::plan::CustomTypeDefinition>,
    ) -> LoweringContext {
        let main_id = crate::plan::FunctionTemplateId::new(0);
        let main = crate::plan::FunctionTemplate::new(
            main_id,
            "main".into(),
            Vec::new(),
            Vec::new(),
            crate::plan::ReturnExpr::int(
                crate::plan::IntFunctionId(0),
                crate::plan::IntExpr::value(0.into()),
            ),
        );
        let parameter = TypeParameterId(0);
        let capture_local =
            crate::plan::GenericLocal::new(crate::plan::GenericLocalId(0), parameter);
        let captured_local =
            crate::plan::GenericLocal::new(crate::plan::GenericLocalId(1), parameter);
        let capture_target_signature = crate::plan::FunctionTemplateSignature::new(
            crate::plan::FunctionTemplateId::new(1),
            crate::plan::TypeScheme::new(1),
            crate::plan::FunctionShape::new(
                vec![crate::plan::ValueShape::Parameter(parameter)],
                crate::plan::ValueShape::Int,
            ),
        );
        let capture_target = crate::plan::FunctionTemplate::from_signature(
            capture_target_signature,
            "capture_target".into(),
            vec![crate::plan::Param::named_shape(
                crate::plan::ParamLocal::generic(capture_local),
                "captured".into(),
                crate::plan::ValueShape::Parameter(parameter),
            )],
            vec![crate::plan::ParamSlot::new(
                crate::plan::ParamLocal::generic(captured_local),
                crate::plan::ValueShape::Parameter(parameter),
            )],
            Vec::new(),
            crate::plan::ReturnExpr::int(
                crate::plan::IntFunctionId(0),
                crate::plan::IntExpr::value(0.into()),
            ),
        );
        let templates = FunctionTemplates::new(vec![vec![main, capture_target]]);

        LoweringContext::new(
            templates.entry_templates(),
            RepresentationContext::new(custom_types),
            ProgramConstantTemplates {
                modules: vec![crate::plan::ConstantTemplates::from_entries(Vec::new())],
            },
            super::SpecializationKey::monomorphic(main_id),
            HashSet::new(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::specialization::{Representability, SpecializationKey};
    use super::{
        FixedPointStep, SpecializationOutcome, resolve_specialization_fixed_point,
        try_resolve_specialization_fixed_point,
    };

    use std::collections::{HashSet, VecDeque};

    #[test]
    fn specialization_fixed_point_discards_provisional_passes_before_completing() {
        let mut visited = Vec::new();

        let output = resolve_specialization_fixed_point(0, |state| {
            visited.push(state);
            if state < 2 {
                FixedPointStep::Continue(state + 1)
            } else {
                FixedPointStep::Complete(state)
            }
        });

        assert_eq!(visited, vec![0, 1, 2]);
        assert_eq!(output, 2);
    }

    #[test]
    fn fallible_specialization_fixed_point_retries_and_propagates_errors() {
        let mut visited = Vec::new();
        let mut lower = |state| match state {
            Ok(state) => {
                visited.push(state);
                if state < 2 {
                    Ok(FixedPointStep::Continue(Ok(state + 1)))
                } else {
                    Ok(FixedPointStep::Complete(state))
                }
            }
            Err(error) => Err(error),
        };

        let output = try_resolve_specialization_fixed_point(Ok(0), &mut lower);
        let error = try_resolve_specialization_fixed_point(Err("lowering failed"), &mut lower);

        assert_eq!(visited, vec![0, 1, 2]);
        assert_eq!(output, Ok(2));
        assert_eq!(error, Err("lowering failed"));
    }

    #[test]
    fn specialization_outcome_accumulates_only_provisional_erasure() {
        fn increment(value: usize) -> usize {
            value + 1
        }

        let prior = SpecializationKey::monomorphic(crate::plan::FunctionTemplateId::new(3));
        let erased = SpecializationKey::monomorphic(crate::plan::FunctionTemplateId::new(5));

        assert_eq!(
            SpecializationOutcome::<usize>::Complete(1).erased_specializations(),
            HashSet::new()
        );
        assert_eq!(
            SpecializationOutcome::<usize>::RequiresErasure(HashSet::from([erased.clone()]))
                .erased_specializations(),
            HashSet::from([erased.clone()])
        );

        let complete = SpecializationOutcome::complete_unless_erased(1_usize, HashSet::new())
            .map(increment)
            .include_prior_erasure(HashSet::from([prior.clone()]))
            .into_fixed_point(HashSet::<SpecializationKey>::new());
        let retried =
            SpecializationOutcome::complete_unless_erased(1_usize, HashSet::from([erased.clone()]))
                .map(increment)
                .include_prior_erasure(HashSet::from([prior.clone()]))
                .into_fixed_point(HashSet::from([prior.clone(), erased.clone()]));

        assert_eq!(complete, FixedPointStep::Complete(2));
        assert_eq!(
            retried,
            FixedPointStep::Continue(HashSet::from([prior, erased.clone()]))
        );

        let mut context = super::test_support::lowering_context(Vec::new());
        let seeded = SpecializationKey::monomorphic(crate::plan::FunctionTemplateId::new(0));
        assert_eq!(
            context
                .provisional_specialization(
                    seeded.clone(),
                    super::function::FunctionTableFamily::Int,
                )
                .map(|specialization| specialization.index),
            Representability::Inhabited(0),
        );
        assert_eq!(context.provisional_specializations[&seeded].index, 0);
        assert!(
            context.provisional_specializations[&seeded]
                .parameters
                .is_empty()
        );
        assert_eq!(context.pending, VecDeque::from([seeded]));

        context.erased_specializations.insert(erased.clone());
        assert!(!context.provisional_specializations.contains_key(&erased));
        assert_eq!(
            context
                .provisional_specialization(erased, super::function::FunctionTableFamily::Never)
                .map(|_| ()),
            Representability::Uninhabited,
        );
    }

    #[test]
    fn specialization_outcome_combines_representability() {
        let first = SpecializationKey::monomorphic(crate::plan::FunctionTemplateId::new(3));
        let second = SpecializationKey::monomorphic(crate::plan::FunctionTemplateId::new(5));

        assert_eq!(
            SpecializationOutcome::from_representability(
                Representability::Inhabited(2_usize),
                first.clone(),
            ),
            SpecializationOutcome::Complete(2),
        );
        assert_eq!(
            SpecializationOutcome::<usize>::from_representability(
                Representability::Uninhabited,
                first.clone(),
            ),
            SpecializationOutcome::RequiresErasure(HashSet::from([first.clone()])),
        );
        assert_eq!(
            SpecializationOutcome::Complete(2_usize).zip_with(
                SpecializationOutcome::Complete(3_usize),
                usize::wrapping_add,
            ),
            SpecializationOutcome::Complete(5),
        );
        assert_eq!(
            SpecializationOutcome::<usize>::RequiresErasure(HashSet::from([first.clone()]))
                .zip_with(
                    SpecializationOutcome::<usize>::RequiresErasure(HashSet::from(
                        [second.clone()],
                    )),
                    usize::wrapping_add,
                ),
            SpecializationOutcome::RequiresErasure(HashSet::from([first.clone(), second.clone(),])),
        );
        assert_eq!(
            SpecializationOutcome::<usize>::RequiresErasure(HashSet::from([first.clone()]))
                .zip_with(
                    SpecializationOutcome::Complete(3_usize),
                    usize::wrapping_add,
                ),
            SpecializationOutcome::RequiresErasure(HashSet::from([first])),
        );
        assert_eq!(
            SpecializationOutcome::Complete(2_usize).zip_with(
                SpecializationOutcome::<usize>::RequiresErasure(HashSet::from([second.clone()])),
                usize::wrapping_add,
            ),
            SpecializationOutcome::RequiresErasure(HashSet::from([second])),
        );
    }

    #[test]
    fn uninhabited_parameters_do_not_enter_the_specialization_queue() {
        let mut context = super::test_support::lowering_context(Vec::new());
        let generic = SpecializationKey::monomorphic(crate::plan::FunctionTemplateId::new(1));

        assert_eq!(
            context
                .provisional_specialization(
                    generic.clone(),
                    super::function::FunctionTableFamily::Int,
                )
                .map(|_| ()),
            Representability::Uninhabited,
        );
        assert!(!context.provisional_specializations.contains_key(&generic));
        assert!(context.pending.is_empty());
    }
}
