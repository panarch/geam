mod constant;
mod function;
mod graph;
mod local;
mod specialization;
mod value_type;

use super::ExecutionPlan;
use super::custom_type::CustomTypeTable;
use super::value_shape::ValueShapeTable;
use super::value_type::ListTypeTable;
use crate::plan::{ModulePlan, ValueShape};
use specialization::{
    RepresentationContext, SpecializationKey, SpecializedCustomConstructor,
    SpecializedCustomValueShape, SpecializedFunctionShape, SpecializedTypeSubstitution,
    SpecializedValueShape,
};
use std::collections::{HashMap, HashSet, VecDeque};

struct FunctionTemplates {
    templates: Vec<crate::plan::FunctionTemplate>,
    main: crate::plan::FunctionTemplateId,
}

struct SpecializationState {
    constant_templates: crate::plan::ConstantTemplates,
    representations: RepresentationContext,
    erased_specializations: HashSet<SpecializationKey>,
}

struct LoweredExecution {
    constants: super::constant::ConstantTable,
    functions: super::FunctionTables,
    list_types: ListTypeTable,
    custom_types: CustomTypeTable,
    value_shapes: ValueShapeTable,
}

enum SpecializationOutcome<T> {
    Complete(T),
    RequiresErasure(HashSet<SpecializationKey>),
}

#[derive(Debug, PartialEq, Eq)]
enum FixedPointStep<State, Output> {
    Complete(Output),
    Continue(State),
}

#[derive(Clone)]
struct ProvisionalSpecialization {
    index: usize,
}

impl<T> SpecializationOutcome<T> {
    fn complete_unless_erased(value: T, erased: HashSet<SpecializationKey>) -> Self {
        if erased.is_empty() {
            Self::Complete(value)
        } else {
            Self::RequiresErasure(erased)
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

struct LoweringContext {
    constant_templates: crate::plan::ConstantTemplates,
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
        templates: &FunctionTemplates,
        representations: RepresentationContext,
        constant_templates: crate::plan::ConstantTemplates,
        erased_specializations: HashSet<SpecializationKey>,
    ) -> Self {
        let entry_templates = templates
            .templates
            .iter()
            .map(|template| (template.id(), local::FunctionEntryTemplate::new(template)))
            .collect::<HashMap<_, _>>();
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

    fn function_shape(&mut self, shape: crate::plan::FunctionShape) -> super::FunctionShape {
        self.types
            .function_shape(&SpecializedFunctionShape::instantiate(
                &shape,
                &self.substitution,
            ))
    }

    fn custom_function_type(
        &mut self,
        type_: crate::plan::CustomFunctionType,
    ) -> super::CustomFunctionType {
        let substitution = self.substitution.clone();
        self.custom_function_type_with_substitution(&type_, &substitution)
    }

    fn generic_function_type(
        &mut self,
        shape: &SpecializedFunctionShape,
    ) -> super::GenericFunctionType {
        self.types.generic_function_type(shape)
    }

    fn specialized_custom_function_type(
        &mut self,
        arguments: &[SpecializedValueShape],
        return_: &SpecializedCustomValueShape,
    ) -> super::CustomFunctionType {
        self.types.custom_function_type(arguments, return_)
    }

    fn specialized_function_function_type(
        &mut self,
        arguments: &[SpecializedValueShape],
        return_: &SpecializedFunctionShape,
    ) -> super::FunctionFunctionType {
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
    ) -> super::GenericCallableId {
        let (key, _) = SpecializationKey::from_instantiation(function, &self.substitution);
        super::GenericCallableId::function(
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
    ) -> super::GenericCallableId {
        super::GenericCallableId::constructor(self.custom_constructor(constructor))
    }

    fn custom_function_type_with_substitution(
        &mut self,
        type_: &crate::plan::CustomFunctionType,
        substitution: &SpecializedTypeSubstitution,
    ) -> super::CustomFunctionType {
        let arguments = type_
            .argument_shapes()
            .iter()
            .map(|shape| SpecializedValueShape::instantiate(shape, substitution))
            .collect::<Vec<_>>();
        let return_ = SpecializedCustomValueShape::instantiate(type_.return_(), substitution);
        self.types.custom_function_type(&arguments, &return_)
    }

    fn int_list_type(&mut self) -> super::IntListTypeId {
        self.types.int_list_type()
    }

    fn string_list_type(&mut self) -> super::StringListTypeId {
        self.types.string_list_type()
    }

    fn bit_array_list_type(&mut self) -> super::BitArrayListTypeId {
        self.types.bit_array_list_type()
    }

    fn utf_codepoint_list_type(&mut self) -> super::UtfCodepointListTypeId {
        self.types.utf_codepoint_list_type()
    }

    fn custom_constructor(
        &mut self,
        constructor: crate::plan::CustomConstructor,
    ) -> super::CustomConstructorId {
        self.types
            .custom_constructor(SpecializedCustomConstructor::instantiate(
                constructor,
                &self.substitution,
            ))
    }

    fn custom_list_type(&mut self, item: crate::plan::CustomType) -> super::CustomListTypeId {
        let shape = SpecializedCustomValueShape::instantiate(
            &crate::plan::CustomValueShape::any(item),
            &self.substitution,
        );
        self.types.custom_list_type(&shape)
    }

    fn specialized_custom_list_type(
        &mut self,
        item: &SpecializedCustomValueShape,
    ) -> super::CustomListTypeId {
        self.types.custom_list_type(item)
    }

    fn float_list_type(&mut self) -> super::FloatListTypeId {
        self.types.float_list_type()
    }

    fn bool_list_type(&mut self) -> super::BoolListTypeId {
        self.types.bool_list_type()
    }

    fn nil_list_type(&mut self) -> super::NilListTypeId {
        self.types.nil_list_type()
    }

    fn tuple_list_type(&mut self, item: Vec<crate::plan::ValueType>) -> super::TupleListTypeId {
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
    ) -> super::TupleListTypeId {
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
    ) -> super::ParameterListTypeId {
        self.types.parameter_list_type(parameter)
    }

    fn parameter_list_list_type(
        &mut self,
        parameter: crate::plan::TypeParameterId,
    ) -> super::ParameterListListTypeId {
        self.types.parameter_list_list_type(parameter)
    }

    fn stored_list_list_type(
        &mut self,
        item: &crate::plan::ValueStorageShape,
    ) -> super::ListListTypeId {
        let item = specialization::StoredValueShape::instantiate(item, &self.substitution);
        self.types.stored_list_list_type(&item)
    }

    fn specialized_stored_list_list_type(
        &mut self,
        item: &specialization::StoredValueShape,
    ) -> super::ListListTypeId {
        self.types.stored_list_list_type(item)
    }

    fn function_list_type(&mut self, item: crate::plan::FunctionType) -> super::FunctionListTypeId {
        let shape = SpecializedFunctionShape::instantiate(
            &crate::plan::FunctionShape::from_function_type(item),
            &self.substitution,
        );
        self.types.function_list_type(&shape)
    }

    fn specialized_function_list_type(
        &mut self,
        item: &SpecializedFunctionShape,
    ) -> super::FunctionListTypeId {
        self.types.function_list_type(item)
    }

    fn reserve_main(
        &mut self,
        key: SpecializationKey,
        return_: specialization::ValueInhabitation,
    ) -> super::RuntimeFunctionId {
        match return_ {
            specialization::ValueInhabitation::Uninhabited(_) => {
                let specialization = self
                    .reserve_provisional_specialization(key, function::FunctionTableFamily::Never);
                super::RuntimeFunctionId::Never(super::NeverFunctionId(specialization.index))
            }
            specialization::ValueInhabitation::Inhabited(return_shape) => {
                let family =
                    function::stored_function_table_family(&return_shape, &self.representations);
                let specialization = self.reserve_provisional_specialization(key, family);
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
            specialization::Representability::Inhabited(
                self.reserve_provisional_specialization(key, family),
            )
        }
    }

    fn reserve_provisional_specialization(
        &mut self,
        key: SpecializationKey,
        family: function::FunctionTableFamily,
    ) -> ProvisionalSpecialization {
        match self.provisional_specializations.get(&key) {
            Some(specialization) => specialization.clone(),
            None => {
                let index = self.next_function_index(family);
                let specialization = ProvisionalSpecialization { index };
                self.provisional_specializations
                    .insert(key.clone(), specialization.clone());
                self.pending.push_back(key);
                specialization
            }
        }
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
    ) -> specialization::Representability<super::NeverFunctionId> {
        let (key, _) = SpecializationKey::from_instantiation(function, &self.substitution);
        self.provisional_specialization(key, function::FunctionTableFamily::Never)
            .map(|specialization| super::NeverFunctionId(specialization.index))
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
    ) -> specialization::Representability<super::IntFunctionId> {
        self.reserve_function_id(function, function::FunctionTableFamily::Int, |index, _| {
            super::IntFunctionId(index)
        })
    }

    fn float_function_id(
        &mut self,
        function: &crate::plan::FunctionInstantiation,
    ) -> specialization::Representability<super::FloatFunctionId> {
        self.reserve_function_id(
            function,
            function::FunctionTableFamily::Float,
            |index, _| super::FloatFunctionId(index),
        )
    }

    fn string_function_id(
        &mut self,
        function: &crate::plan::FunctionInstantiation,
    ) -> specialization::Representability<super::StringFunctionId> {
        self.reserve_function_id(
            function,
            function::FunctionTableFamily::String,
            |index, _| super::StringFunctionId(index),
        )
    }

    fn bit_array_function_id(
        &mut self,
        function: &crate::plan::FunctionInstantiation,
    ) -> specialization::Representability<super::BitArrayFunctionId> {
        self.reserve_function_id(
            function,
            function::FunctionTableFamily::BitArray,
            |index, _| super::BitArrayFunctionId(index),
        )
    }

    fn utf_codepoint_function_id(
        &mut self,
        function: &crate::plan::FunctionInstantiation,
    ) -> specialization::Representability<super::UtfCodepointFunctionId> {
        self.reserve_function_id(
            function,
            function::FunctionTableFamily::UtfCodepoint,
            |index, _| super::UtfCodepointFunctionId(index),
        )
    }

    fn custom_function_id(
        &mut self,
        function: &crate::plan::FunctionInstantiation,
        shape: &SpecializedCustomValueShape,
    ) -> specialization::Representability<super::CustomFunctionId> {
        self.reserve_function_id(
            function,
            function::FunctionTableFamily::Custom,
            |index, context| {
                super::CustomFunctionId::new(index, context.types.custom_value_shape(shape))
            },
        )
    }

    fn bool_function_id(
        &mut self,
        function: &crate::plan::FunctionInstantiation,
    ) -> specialization::Representability<super::BoolFunctionId> {
        self.reserve_function_id(function, function::FunctionTableFamily::Bool, |index, _| {
            super::BoolFunctionId(index)
        })
    }

    fn nil_function_id(
        &mut self,
        function: &crate::plan::FunctionInstantiation,
    ) -> specialization::Representability<super::NilFunctionId> {
        self.reserve_function_id(function, function::FunctionTableFamily::Nil, |index, _| {
            super::NilFunctionId(index)
        })
    }

    fn tuple_function_id(
        &mut self,
        function: &crate::plan::FunctionInstantiation,
    ) -> specialization::Representability<super::TupleFunctionId> {
        self.reserve_function_id(
            function,
            function::FunctionTableFamily::Tuple,
            |index, _| super::TupleFunctionId(index),
        )
    }

    fn list_function_id(
        &mut self,
        function: &crate::plan::FunctionInstantiation,
        item: &SpecializedValueShape,
    ) -> specialization::Representability<super::ListFunctionId> {
        self.reserve_function_id(
            function,
            function::list_function_table_family(item),
            |index, context| function::list_function_id(item, index, &mut context.types),
        )
    }

    fn int_list_function_id(
        &mut self,
        function: &crate::plan::FunctionInstantiation,
    ) -> specialization::Representability<super::IntListFunctionId> {
        self.reserve_function_id(
            function,
            function::FunctionTableFamily::IntList,
            |index, context| super::IntListFunctionId::new(index, context.types.int_list_type()),
        )
    }

    fn parameter_list_function_id(
        &mut self,
        function: &crate::plan::FunctionInstantiation,
        parameter: crate::plan::TypeParameterId,
    ) -> specialization::Representability<super::ParameterListFunctionId> {
        self.reserve_function_id(
            function,
            function::FunctionTableFamily::ParameterList,
            |index, context| {
                super::ParameterListFunctionId::new(
                    index,
                    context.types.parameter_list_type(parameter),
                )
            },
        )
    }

    fn string_list_function_id(
        &mut self,
        function: &crate::plan::FunctionInstantiation,
    ) -> specialization::Representability<super::StringListFunctionId> {
        self.reserve_function_id(
            function,
            function::FunctionTableFamily::StringList,
            |index, context| {
                super::StringListFunctionId::new(index, context.types.string_list_type())
            },
        )
    }

    fn bit_array_list_function_id(
        &mut self,
        function: &crate::plan::FunctionInstantiation,
    ) -> specialization::Representability<super::BitArrayListFunctionId> {
        self.reserve_function_id(
            function,
            function::FunctionTableFamily::BitArrayList,
            |index, context| {
                super::BitArrayListFunctionId::new(index, context.types.bit_array_list_type())
            },
        )
    }

    fn utf_codepoint_list_function_id(
        &mut self,
        function: &crate::plan::FunctionInstantiation,
    ) -> specialization::Representability<super::UtfCodepointListFunctionId> {
        self.reserve_function_id(
            function,
            function::FunctionTableFamily::UtfCodepointList,
            |index, context| {
                super::UtfCodepointListFunctionId::new(
                    index,
                    context.types.utf_codepoint_list_type(),
                )
            },
        )
    }

    fn custom_list_function_id(
        &mut self,
        function: &crate::plan::FunctionInstantiation,
        type_id: super::CustomListTypeId,
    ) -> specialization::Representability<super::CustomListFunctionId> {
        self.reserve_function_id(
            function,
            function::FunctionTableFamily::CustomList,
            |index, _| super::CustomListFunctionId::new(index, type_id),
        )
    }

    fn float_list_function_id(
        &mut self,
        function: &crate::plan::FunctionInstantiation,
    ) -> specialization::Representability<super::FloatListFunctionId> {
        self.reserve_function_id(
            function,
            function::FunctionTableFamily::FloatList,
            |index, context| {
                super::FloatListFunctionId::new(index, context.types.float_list_type())
            },
        )
    }

    fn bool_list_function_id(
        &mut self,
        function: &crate::plan::FunctionInstantiation,
    ) -> specialization::Representability<super::BoolListFunctionId> {
        self.reserve_function_id(
            function,
            function::FunctionTableFamily::BoolList,
            |index, context| super::BoolListFunctionId::new(index, context.types.bool_list_type()),
        )
    }

    fn nil_list_function_id(
        &mut self,
        function: &crate::plan::FunctionInstantiation,
    ) -> specialization::Representability<super::NilListFunctionId> {
        self.reserve_function_id(
            function,
            function::FunctionTableFamily::NilList,
            |index, context| super::NilListFunctionId::new(index, context.types.nil_list_type()),
        )
    }

    fn tuple_list_function_id(
        &mut self,
        function: &crate::plan::FunctionInstantiation,
        type_id: super::TupleListTypeId,
    ) -> specialization::Representability<super::TupleListFunctionId> {
        self.reserve_function_id(
            function,
            function::FunctionTableFamily::TupleList,
            |index, _| super::TupleListFunctionId::new(index, type_id),
        )
    }

    fn list_list_function_id(
        &mut self,
        function: &crate::plan::FunctionInstantiation,
        type_id: super::ListListTypeId,
    ) -> specialization::Representability<super::ListListFunctionId> {
        self.reserve_function_id(
            function,
            function::FunctionTableFamily::ListList,
            |index, _| super::ListListFunctionId::new(index, type_id),
        )
    }

    fn parameter_list_list_function_id(
        &mut self,
        function: &crate::plan::FunctionInstantiation,
        type_id: super::ParameterListListTypeId,
    ) -> specialization::Representability<super::ParameterListListFunctionId> {
        self.reserve_function_id(
            function,
            function::FunctionTableFamily::ParameterListList,
            |index, _| super::ParameterListListFunctionId::new(index, type_id),
        )
    }

    fn function_list_function_id(
        &mut self,
        function: &crate::plan::FunctionInstantiation,
        type_id: super::FunctionListTypeId,
    ) -> specialization::Representability<super::FunctionListFunctionId> {
        self.reserve_function_id(
            function,
            function::FunctionTableFamily::FunctionList,
            |index, _| super::FunctionListFunctionId::new(index, type_id),
        )
    }

    fn function_function_id(
        &mut self,
        function: &crate::plan::FunctionInstantiation,
        return_: &SpecializedFunctionShape,
    ) -> specialization::Representability<super::FunctionFunctionId> {
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
    ) -> specialization::Representability<super::IntFunctionFunctionId> {
        self.reserve_function_id(
            function,
            function::FunctionTableFamily::IntFunction,
            |index, _| super::IntFunctionFunctionId(index),
        )
    }

    fn float_function_function_id(
        &mut self,
        function: &crate::plan::FunctionInstantiation,
    ) -> specialization::Representability<super::FloatFunctionFunctionId> {
        self.reserve_function_id(
            function,
            function::FunctionTableFamily::FloatFunction,
            |index, _| super::FloatFunctionFunctionId(index),
        )
    }

    fn string_function_function_id(
        &mut self,
        function: &crate::plan::FunctionInstantiation,
    ) -> specialization::Representability<super::StringFunctionFunctionId> {
        self.reserve_function_id(
            function,
            function::FunctionTableFamily::StringFunction,
            |index, _| super::StringFunctionFunctionId(index),
        )
    }

    fn bit_array_function_function_id(
        &mut self,
        function: &crate::plan::FunctionInstantiation,
    ) -> specialization::Representability<super::BitArrayFunctionFunctionId> {
        self.reserve_function_id(
            function,
            function::FunctionTableFamily::BitArrayFunction,
            |index, _| super::BitArrayFunctionFunctionId(index),
        )
    }

    fn utf_codepoint_function_function_id(
        &mut self,
        function: &crate::plan::FunctionInstantiation,
    ) -> specialization::Representability<super::UtfCodepointFunctionFunctionId> {
        self.reserve_function_id(
            function,
            function::FunctionTableFamily::UtfCodepointFunction,
            |index, _| super::UtfCodepointFunctionFunctionId(index),
        )
    }

    fn custom_function_function_id(
        &mut self,
        function: &crate::plan::FunctionInstantiation,
        type_: super::CustomFunctionType,
    ) -> specialization::Representability<super::CustomFunctionFunctionId> {
        self.reserve_function_id(
            function,
            function::FunctionTableFamily::CustomFunction,
            |index, _| super::CustomFunctionFunctionId::new(index, type_),
        )
    }

    fn bool_function_function_id(
        &mut self,
        function: &crate::plan::FunctionInstantiation,
    ) -> specialization::Representability<super::BoolFunctionFunctionId> {
        self.reserve_function_id(
            function,
            function::FunctionTableFamily::BoolFunction,
            |index, _| super::BoolFunctionFunctionId(index),
        )
    }

    fn nil_function_function_id(
        &mut self,
        function: &crate::plan::FunctionInstantiation,
    ) -> specialization::Representability<super::NilFunctionFunctionId> {
        self.reserve_function_id(
            function,
            function::FunctionTableFamily::NilFunction,
            |index, _| super::NilFunctionFunctionId(index),
        )
    }

    fn tuple_function_function_id(
        &mut self,
        function: &crate::plan::FunctionInstantiation,
    ) -> specialization::Representability<super::TupleFunctionFunctionId> {
        self.reserve_function_id(
            function,
            function::FunctionTableFamily::TupleFunction,
            |index, _| super::TupleFunctionFunctionId(index),
        )
    }

    fn list_function_function_id(
        &mut self,
        function: &crate::plan::FunctionInstantiation,
        type_: &SpecializedFunctionShape,
        item: &SpecializedValueShape,
    ) -> specialization::Representability<super::ListFunctionFunctionId> {
        self.reserve_function_id(
            function,
            function::list_function_function_table_family(item),
            |index, context| {
                function::list_function_function_id(type_, item, index, &mut context.types)
            },
        )
    }

    fn function_function_function_id(
        &mut self,
        function: &crate::plan::FunctionInstantiation,
        type_: super::FunctionFunctionType,
    ) -> specialization::Representability<super::FunctionFunctionFunctionId> {
        self.reserve_function_id(
            function,
            function::FunctionTableFamily::FunctionFunction,
            |index, _| super::FunctionFunctionFunctionId::new(index, type_),
        )
    }

    fn generic_function_function_id(
        &mut self,
        function: &crate::plan::FunctionInstantiation,
        type_: super::GenericFunctionType,
    ) -> specialization::Representability<super::GenericFunctionFunctionId> {
        self.reserve_function_id(
            function,
            function::FunctionTableFamily::GenericFunction,
            |index, _| super::GenericFunctionFunctionId::new(index, type_),
        )
    }

    fn never_function_function_id(
        &mut self,
        function: &crate::plan::FunctionInstantiation,
        type_: super::GenericFunctionType,
    ) -> specialization::Representability<super::NeverFunctionFunctionId> {
        self.reserve_function_id(
            function,
            function::FunctionTableFamily::NeverFunction,
            |index, _| super::NeverFunctionFunctionId::new(index, type_),
        )
    }

    fn concrete_value_shape(&self, shape: &ValueShape) -> SpecializedValueShape {
        SpecializedValueShape::instantiate(shape, &self.substitution)
    }

    fn lower_concrete_value_type(&mut self, shape: &SpecializedValueShape) -> super::ValueType {
        self.types.value_type(shape)
    }

    fn lower_concrete_custom_shape(
        &mut self,
        shape: &SpecializedCustomValueShape,
    ) -> super::CustomValueShape {
        self.types.custom_value_shape(shape)
    }

    fn lower_concrete_function_type(
        &mut self,
        shape: &SpecializedFunctionShape,
    ) -> super::FunctionType {
        self.types.function_type(shape)
    }

    fn lower_concrete_function_shape(
        &mut self,
        shape: &SpecializedFunctionShape,
    ) -> super::FunctionShape {
        self.types.function_shape(shape)
    }

    fn concrete_custom_value_shape(
        &self,
        shape: &crate::plan::CustomValueShape,
    ) -> SpecializedCustomValueShape {
        SpecializedCustomValueShape::instantiate(shape, &self.substitution)
    }

    fn concrete_function_shape(
        &self,
        shape: &crate::plan::FunctionShape,
    ) -> SpecializedFunctionShape {
        SpecializedFunctionShape::instantiate(shape, &self.substitution)
    }

    fn begin(&mut self, key: &SpecializationKey) {
        self.substitution = key.substitution().clone();
    }

    fn specialization_index(&self, key: &SpecializationKey) -> usize {
        self.provisional_specializations[key].index
    }

    fn finish(
        self,
    ) -> (
        crate::plan::ConstantTemplates,
        RepresentationContext,
        SpecializationOutcome<Box<LoweredExecution>>,
    ) {
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

pub(super) fn lower(module_plan: ModulePlan) -> ExecutionPlan {
    let parts = module_plan.into_parts();
    let main_return_shape = parts.main.signature().shape().return_shape().clone();
    let templates = FunctionTemplates::new(parts.main, parts.functions, parts.anonymous_functions);
    let main_key = SpecializationKey::monomorphic(templates.main);
    let initial = SpecializationState {
        constant_templates: parts.constants,
        representations: RepresentationContext::new(parts.custom_types),
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
            &templates,
            representations,
            constant_templates,
            erased_specializations,
        );

        let main = context.reserve_main(main_key.clone(), main_return_shape);

        while let Some(key) = context.pending.pop_front() {
            context.begin(&key);
            function::lower_specialized(templates.get(key.template()), &key, &mut context);
        }

        let (constant_templates, representations, outcome) = context.finish();
        let erased_specializations = outcome.erased_specializations();
        outcome
            .map(|lowered| (main, lowered))
            .into_fixed_point(SpecializationState {
                constant_templates,
                representations,
                erased_specializations,
            })
    });

    ExecutionPlan {
        module: parts.module,
        source_context: parts.source_context,
        main,
        constants: lowered.constants,
        functions: lowered.functions,
        list_types: lowered.list_types,
        custom_types: lowered.custom_types,
        value_shapes: lowered.value_shapes,
    }
}

impl FunctionTemplates {
    fn new(
        main: crate::plan::FunctionTemplate,
        functions: Vec<crate::plan::FunctionTemplate>,
        anonymous_functions: Vec<crate::plan::FunctionTemplate>,
    ) -> Self {
        let main_id = main.id();
        let mut templates = Vec::with_capacity(1 + functions.len() + anonymous_functions.len());
        templates.push(main);
        templates.extend(functions);
        templates.extend(anonymous_functions);
        templates.sort_by_key(|template| template.id().index());
        Self {
            templates,
            main: main_id,
        }
    }

    fn get(&self, id: crate::plan::FunctionTemplateId) -> &crate::plan::FunctionTemplate {
        &self.templates[id.index()]
    }
}

#[cfg(test)]
pub(super) mod test_support {
    use super::specialization::RepresentationContext;
    use super::{FunctionTemplates, LoweringContext};
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
        let templates = FunctionTemplates::new(main, vec![capture_target], Vec::new());

        LoweringContext::new(
            &templates,
            RepresentationContext::new(custom_types),
            crate::plan::ConstantTemplates::from_entries(Vec::new()),
            HashSet::new(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::specialization::{Representability, SpecializationKey};
    use super::{FixedPointStep, SpecializationOutcome, resolve_specialization_fixed_point};

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
}
