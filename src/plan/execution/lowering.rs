mod constant;
mod graph;
mod local;
mod specialization;
mod table;
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
    functions: table::FunctionTableBuilder,
    entry_templates: HashMap<crate::plan::FunctionTemplateId, local::FunctionEntryTemplate>,
    next_function_indices: HashMap<table::FunctionTableFamily, usize>,
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

#[derive(Clone)]
enum SpecializedFunctionLocal {
    Generic(super::GenericFunctionLocal),
    Never(super::NeverFunctionLocal),
    Int {
        local: super::IntFunctionLocalId,
        type_: super::FunctionType,
    },
    Float {
        local: super::FloatFunctionLocalId,
        type_: super::FunctionType,
    },
    String {
        local: super::StringFunctionLocalId,
        type_: super::FunctionType,
    },
    BitArray {
        local: super::BitArrayFunctionLocalId,
        type_: super::FunctionType,
    },
    UtfCodepoint {
        local: super::UtfCodepointFunctionLocalId,
        type_: super::FunctionType,
    },
    Custom(super::CustomFunctionLocal),
    Bool {
        local: super::BoolFunctionLocalId,
        type_: super::FunctionType,
    },
    Nil {
        local: super::NilFunctionLocalId,
        type_: super::FunctionType,
    },
    Tuple {
        local: super::TupleFunctionLocalId,
        type_: super::FunctionType,
    },
    List(super::ListFunctionLocal),
    Function(super::FunctionFunctionLocal),
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
            functions: table::FunctionTableBuilder::default(),
            entry_templates,
            next_function_indices: HashMap::new(),
            provisional_specializations: HashMap::new(),
            erased_specializations,
            pending: VecDeque::new(),
            substitution: SpecializedTypeSubstitution::empty(),
        }
    }

    fn lower_constant<ModuleValue, DraftValue, Value>(
        &mut self,
        instantiation: crate::plan::ConstantInstantiation,
        materialize: impl FnOnce(&crate::plan::ConstantTemplates) -> ModuleValue,
        lower: impl Copy
        + Fn(
            &ModuleValue,
            graph::DraftCursor,
            &mut graph::DraftGraph,
            &mut Self,
        ) -> specialization::Representability<graph::DraftFlow<DraftValue>>,
    ) -> specialization::Representability<super::ConstantId<Value>>
    where
        DraftValue: graph::DraftGraphValue + graph::FreezeGraphValue<Frozen = Value>,
        Value: super::ConstantValue,
    {
        let outer = self.substitution.to_module_substitution();
        let key = instantiation.substitute(&outer);
        if let Some(id) = self.constants.get(&key) {
            return specialization::Representability::Inhabited(id);
        }

        let value = materialize(&self.constant_templates);
        graph::lower_constant_graph(&value, self, lower)
            .map(|program| self.constants.insert(key, program))
    }

    fn int_constant(
        &mut self,
        reference: &crate::plan::ConstantIntReference,
    ) -> specialization::Representability<super::ConstantId<super::IntLocalId>> {
        let instantiation = reference.instantiation().clone();
        self.lower_constant(
            crate::plan::ConstantInstantiation::from_int(instantiation.clone()),
            |templates| templates.materialize_int(&instantiation),
            graph::int_expr,
        )
    }

    fn string_constant(
        &mut self,
        reference: &crate::plan::ConstantStringReference,
    ) -> specialization::Representability<super::ConstantId<super::StringLocalId>> {
        let instantiation = reference.instantiation().clone();
        self.lower_constant(
            crate::plan::ConstantInstantiation::from_string(instantiation.clone()),
            |templates| templates.materialize_string(&instantiation),
            graph::string_expr,
        )
    }

    fn bit_array_constant(
        &mut self,
        reference: &crate::plan::ConstantBitArrayReference,
    ) -> specialization::Representability<super::ConstantId<super::BitArrayLocalId>> {
        let instantiation = reference.instantiation().clone();
        self.lower_constant(
            crate::plan::ConstantInstantiation::from_bit_array(instantiation.clone()),
            |templates| templates.materialize_bit_array(&instantiation),
            graph::bit_array_expr,
        )
    }

    fn custom_constant(
        &mut self,
        reference: &crate::plan::ConstantCustomReference,
    ) -> specialization::Representability<super::ConstantId<super::CustomLocal>> {
        let instantiation = reference.instantiation().clone();
        self.lower_constant(
            crate::plan::ConstantInstantiation::from_custom(instantiation.clone()),
            |templates| templates.materialize_custom(&instantiation),
            graph::custom_expr,
        )
    }

    fn float_constant(
        &mut self,
        reference: &crate::plan::ConstantFloatReference,
    ) -> specialization::Representability<super::ConstantId<super::FloatLocalId>> {
        let instantiation = reference.instantiation().clone();
        self.lower_constant(
            crate::plan::ConstantInstantiation::from_float(instantiation.clone()),
            |templates| templates.materialize_float(&instantiation),
            graph::float_expr,
        )
    }

    fn bool_constant(
        &mut self,
        reference: &crate::plan::ConstantBoolReference,
    ) -> specialization::Representability<super::ConstantId<super::BoolLocalId>> {
        let instantiation = reference.instantiation().clone();
        self.lower_constant(
            crate::plan::ConstantInstantiation::from_bool(instantiation.clone()),
            |templates| templates.materialize_bool(&instantiation),
            graph::bool_expr,
        )
    }

    fn nil_constant(
        &mut self,
        reference: &crate::plan::ConstantNilReference,
    ) -> specialization::Representability<super::ConstantId<super::NilLocalId>> {
        let instantiation = reference.instantiation().clone();
        self.lower_constant(
            crate::plan::ConstantInstantiation::from_nil(instantiation.clone()),
            |templates| templates.materialize_nil(&instantiation),
            graph::nil_expr,
        )
    }

    fn tuple_constant(
        &mut self,
        reference: &crate::plan::ConstantTupleReference,
    ) -> specialization::Representability<super::ConstantId<super::TupleLocalId>> {
        let instantiation = reference.instantiation().clone();
        self.lower_constant(
            crate::plan::ConstantInstantiation::from_tuple(instantiation.clone()),
            |templates| templates.materialize_tuple(&instantiation),
            graph::tuple_expr,
        )
    }

    fn int_list_constant(
        &mut self,
        reference: &crate::plan::ConstantIntListInstantiation,
    ) -> specialization::Representability<super::ConstantId<super::IntListLocalId>> {
        let instantiation = reference.clone();
        self.lower_constant(
            crate::plan::ConstantInstantiation::from_list(
                crate::plan::ConstantListInstantiation::Int(instantiation.clone()),
            ),
            |templates| templates.materialize_int_list(&instantiation),
            graph::int_list_expr,
        )
    }

    fn string_list_constant(
        &mut self,
        reference: &crate::plan::ConstantStringListInstantiation,
    ) -> specialization::Representability<super::ConstantId<super::StringListLocalId>> {
        let instantiation = reference.clone();
        self.lower_constant(
            crate::plan::ConstantInstantiation::from_list(
                crate::plan::ConstantListInstantiation::String(instantiation.clone()),
            ),
            |templates| templates.materialize_string_list(&instantiation),
            graph::string_list_expr,
        )
    }

    fn bit_array_list_constant(
        &mut self,
        reference: &crate::plan::ConstantBitArrayListInstantiation,
    ) -> specialization::Representability<super::ConstantId<super::BitArrayListLocalId>> {
        let instantiation = reference.clone();
        self.lower_constant(
            crate::plan::ConstantInstantiation::from_list(
                crate::plan::ConstantListInstantiation::BitArray(instantiation.clone()),
            ),
            |templates| templates.materialize_bit_array_list(&instantiation),
            graph::bit_array_list_expr,
        )
    }

    fn utf_codepoint_list_constant(
        &mut self,
        reference: &crate::plan::ConstantUtfCodepointListInstantiation,
    ) -> specialization::Representability<super::ConstantId<super::UtfCodepointListLocalId>> {
        let instantiation = reference.clone();
        self.lower_constant(
            crate::plan::ConstantInstantiation::from_list(
                crate::plan::ConstantListInstantiation::UtfCodepoint(instantiation.clone()),
            ),
            |templates| templates.materialize_utf_codepoint_list(&instantiation),
            graph::utf_codepoint_list_expr,
        )
    }

    fn custom_list_constant(
        &mut self,
        reference: &crate::plan::ConstantCustomListInstantiation,
    ) -> specialization::Representability<super::ConstantId<super::CustomListLocalId>> {
        let instantiation = reference.clone();
        self.lower_constant(
            crate::plan::ConstantInstantiation::from_list(
                crate::plan::ConstantListInstantiation::Custom(instantiation.clone()),
            ),
            |templates| templates.materialize_custom_list(&instantiation),
            graph::custom_list_expr,
        )
    }

    fn float_list_constant(
        &mut self,
        reference: &crate::plan::ConstantFloatListInstantiation,
    ) -> specialization::Representability<super::ConstantId<super::FloatListLocalId>> {
        let instantiation = reference.clone();
        self.lower_constant(
            crate::plan::ConstantInstantiation::from_list(
                crate::plan::ConstantListInstantiation::Float(instantiation.clone()),
            ),
            |templates| templates.materialize_float_list(&instantiation),
            graph::float_list_expr,
        )
    }

    fn bool_list_constant(
        &mut self,
        reference: &crate::plan::ConstantBoolListInstantiation,
    ) -> specialization::Representability<super::ConstantId<super::BoolListLocalId>> {
        let instantiation = reference.clone();
        self.lower_constant(
            crate::plan::ConstantInstantiation::from_list(
                crate::plan::ConstantListInstantiation::Bool(instantiation.clone()),
            ),
            |templates| templates.materialize_bool_list(&instantiation),
            graph::bool_list_expr,
        )
    }

    fn nil_list_constant(
        &mut self,
        reference: &crate::plan::ConstantNilListInstantiation,
    ) -> specialization::Representability<super::ConstantId<super::NilListLocalId>> {
        let instantiation = reference.clone();
        self.lower_constant(
            crate::plan::ConstantInstantiation::from_list(
                crate::plan::ConstantListInstantiation::Nil(instantiation.clone()),
            ),
            |templates| templates.materialize_nil_list(&instantiation),
            graph::nil_list_expr,
        )
    }

    fn tuple_list_constant(
        &mut self,
        reference: &crate::plan::ConstantTupleListInstantiation,
    ) -> specialization::Representability<super::ConstantId<super::TupleListLocalId>> {
        let instantiation = reference.clone();
        self.lower_constant(
            crate::plan::ConstantInstantiation::from_list(
                crate::plan::ConstantListInstantiation::Tuple(instantiation.clone()),
            ),
            |templates| templates.materialize_tuple_list(&instantiation),
            graph::tuple_list_expr,
        )
    }

    fn parameter_list_list_constant(
        &mut self,
        reference: &crate::plan::ConstantParameterListListInstantiation,
        _parameter: crate::plan::TypeParameterId,
    ) -> specialization::Representability<super::ConstantId<super::ParameterListListLocalId>> {
        let instantiation = reference.clone();
        self.lower_constant(
            crate::plan::ConstantInstantiation::from_list(
                crate::plan::ConstantListInstantiation::ParameterList(instantiation.clone()),
            ),
            |templates| templates.materialize_parameter_list_list(&instantiation),
            |expression, cursor, graph, context| {
                graph::parameter_list_list_expr(expression, cursor, graph, context).map(|flow| {
                    flow.map(|value| graph::DraftParameterListList::new(value.into_list()))
                })
            },
        )
    }

    fn list_list_constant(
        &mut self,
        reference: &crate::plan::ConstantListListInstantiation,
    ) -> specialization::Representability<super::ConstantId<super::ListListLocalId>> {
        let instantiation = reference.clone();
        self.lower_constant(
            crate::plan::ConstantInstantiation::from_list(
                crate::plan::ConstantListInstantiation::List(instantiation.clone()),
            ),
            |templates| templates.materialize_list_list(&instantiation),
            graph::list_list_expr,
        )
    }

    fn function_list_constant(
        &mut self,
        reference: &crate::plan::ConstantFunctionListInstantiation,
    ) -> specialization::Representability<super::ConstantId<super::FunctionListLocalId>> {
        let instantiation = reference.clone();
        self.lower_constant(
            crate::plan::ConstantInstantiation::from_list(
                crate::plan::ConstantListInstantiation::Function(instantiation.clone()),
            ),
            |templates| templates.materialize_function_list(&instantiation),
            graph::function_list_expr,
        )
    }

    fn lower_generic_list_constant<DraftValue, Value>(
        &mut self,
        reference: &crate::plan::ConstantGenericListInstantiation,
        wrap: impl Copy + Fn(graph::DraftList) -> DraftValue,
    ) -> specialization::Representability<super::ConstantId<Value>>
    where
        DraftValue: graph::DraftGraphValue + graph::FreezeGraphValue<Frozen = Value>,
        Value: super::ConstantValue,
    {
        let instantiation = reference.clone();
        self.lower_constant(
            crate::plan::ConstantInstantiation::from_list(
                crate::plan::ConstantListInstantiation::Generic(instantiation.clone()),
            ),
            |templates| templates.materialize_generic_list(&instantiation),
            |expression, cursor, graph, context| {
                graph::generic_list_expr(expression, cursor, graph, context)
                    .map(|flow| flow.map(wrap))
            },
        )
    }

    fn generic_parameter_list_constant(
        &mut self,
        reference: &crate::plan::ConstantGenericListInstantiation,
        _parameter: crate::plan::TypeParameterId,
    ) -> specialization::Representability<super::ConstantId<super::ParameterListLocalId>> {
        self.lower_generic_list_constant(reference, graph::DraftParameterList::new)
    }

    fn generic_int_list_constant(
        &mut self,
        reference: &crate::plan::ConstantGenericListInstantiation,
    ) -> specialization::Representability<super::ConstantId<super::IntListLocalId>> {
        self.lower_generic_list_constant(reference, graph::DraftIntList::new)
    }

    fn generic_string_list_constant(
        &mut self,
        reference: &crate::plan::ConstantGenericListInstantiation,
    ) -> specialization::Representability<super::ConstantId<super::StringListLocalId>> {
        self.lower_generic_list_constant(reference, graph::DraftStringList::new)
    }

    fn generic_bit_array_list_constant(
        &mut self,
        reference: &crate::plan::ConstantGenericListInstantiation,
    ) -> specialization::Representability<super::ConstantId<super::BitArrayListLocalId>> {
        self.lower_generic_list_constant(reference, graph::DraftBitArrayList::new)
    }

    fn generic_utf_codepoint_list_constant(
        &mut self,
        reference: &crate::plan::ConstantGenericListInstantiation,
    ) -> specialization::Representability<super::ConstantId<super::UtfCodepointListLocalId>> {
        self.lower_generic_list_constant(reference, graph::DraftUtfCodepointList::new)
    }

    fn generic_custom_list_constant(
        &mut self,
        reference: &crate::plan::ConstantGenericListInstantiation,
        _shape: &SpecializedCustomValueShape,
    ) -> specialization::Representability<super::ConstantId<super::CustomListLocalId>> {
        self.lower_generic_list_constant(reference, graph::DraftCustomList::new)
    }

    fn generic_float_list_constant(
        &mut self,
        reference: &crate::plan::ConstantGenericListInstantiation,
    ) -> specialization::Representability<super::ConstantId<super::FloatListLocalId>> {
        self.lower_generic_list_constant(reference, graph::DraftFloatList::new)
    }

    fn generic_bool_list_constant(
        &mut self,
        reference: &crate::plan::ConstantGenericListInstantiation,
    ) -> specialization::Representability<super::ConstantId<super::BoolListLocalId>> {
        self.lower_generic_list_constant(reference, graph::DraftBoolList::new)
    }

    fn generic_nil_list_constant(
        &mut self,
        reference: &crate::plan::ConstantGenericListInstantiation,
    ) -> specialization::Representability<super::ConstantId<super::NilListLocalId>> {
        self.lower_generic_list_constant(reference, graph::DraftNilList::new)
    }

    fn generic_tuple_list_constant(
        &mut self,
        reference: &crate::plan::ConstantGenericListInstantiation,
        _elements: &[SpecializedValueShape],
    ) -> specialization::Representability<super::ConstantId<super::TupleListLocalId>> {
        self.lower_generic_list_constant(reference, graph::DraftTupleList::new)
    }

    fn generic_parameter_list_list_constant(
        &mut self,
        reference: &crate::plan::ConstantGenericListInstantiation,
        _parameter: crate::plan::TypeParameterId,
    ) -> specialization::Representability<super::ConstantId<super::ParameterListListLocalId>> {
        self.lower_generic_list_constant(reference, graph::DraftParameterListList::new)
    }

    fn generic_list_list_constant(
        &mut self,
        reference: &crate::plan::ConstantGenericListInstantiation,
        _item_shape: &specialization::StoredValueShape,
    ) -> specialization::Representability<super::ConstantId<super::ListListLocalId>> {
        self.lower_generic_list_constant(reference, graph::DraftListList::new)
    }

    fn generic_function_list_constant(
        &mut self,
        reference: &crate::plan::ConstantGenericListInstantiation,
        _shape: &SpecializedFunctionShape,
    ) -> specialization::Representability<super::ConstantId<super::FunctionListLocalId>> {
        self.lower_generic_list_constant(reference, graph::DraftFunctionList::new)
    }

    fn parameter_list_list_as_stored_constant(
        &mut self,
        reference: &crate::plan::ConstantParameterListListInstantiation,
        _item_shape: &specialization::StoredValueShape,
    ) -> specialization::Representability<super::ConstantId<super::ListListLocalId>> {
        let instantiation = reference.clone();
        self.lower_constant(
            crate::plan::ConstantInstantiation::from_list(
                crate::plan::ConstantListInstantiation::ParameterList(instantiation.clone()),
            ),
            |templates| templates.materialize_parameter_list_list(&instantiation),
            |expression, cursor, graph, context| {
                graph::parameter_list_list_expr(expression, cursor, graph, context)
                    .map(|flow| flow.map(|value| graph::DraftListList::new(value.into_list())))
            },
        )
    }

    fn lower_function_constant<ModuleValue, DraftValue>(
        &mut self,
        instantiation: crate::plan::ConstantInstantiation,
        materialize: impl FnOnce(&crate::plan::ConstantTemplates) -> ModuleValue,
        lower: impl Copy
        + Fn(
            &ModuleValue,
            graph::DraftCursor,
            &mut graph::DraftGraph,
            &mut Self,
        ) -> specialization::Representability<graph::DraftFlow<DraftValue>>,
    ) -> specialization::Representability<super::ConstantId<super::FunctionLocal>>
    where
        DraftValue: graph::DraftFunctionValue,
    {
        let outer = self.substitution.to_module_substitution();
        let key = instantiation.substitute(&outer);
        if let Some(id) = self.constants.get(&key) {
            return specialization::Representability::Inhabited(id);
        }

        let value = materialize(&self.constant_templates);
        graph::lower_constant_graph(&value, self, |expression, cursor, graph, context| {
            lower(expression, cursor, graph, context)
                .map(|flow| flow.map(graph::DraftFunctionValue::into_function))
        })
        .map(|program| self.constants.insert(key, program))
    }

    fn generic_function_constant(
        &mut self,
        reference: &crate::plan::ConstantGenericFunctionInstantiation,
        shape: &SpecializedFunctionShape,
    ) -> specialization::Representability<super::ConstantId<super::FunctionLocal>> {
        let instantiation = reference.clone();
        self.lower_function_constant(
            crate::plan::ConstantInstantiation::from_function(
                crate::plan::ConstantFunctionInstantiation::Generic(instantiation.clone()),
            ),
            |templates| templates.materialize_generic_function(&instantiation),
            |expression, cursor, graph, context| {
                graph::symbolic_generic_function_expr(expression, shape, cursor, graph, context)
            },
        )
    }

    fn generic_never_function_constant(
        &mut self,
        reference: &crate::plan::ConstantGenericFunctionInstantiation,
        shape: &SpecializedFunctionShape,
    ) -> specialization::Representability<super::ConstantId<super::FunctionLocal>> {
        let instantiation = reference.clone();
        self.lower_function_constant(
            crate::plan::ConstantInstantiation::from_function(
                crate::plan::ConstantFunctionInstantiation::Generic(instantiation.clone()),
            ),
            |templates| templates.materialize_generic_function(&instantiation),
            |expression, cursor, graph, context| {
                graph::generic_never_function_expr(expression, shape, cursor, graph, context)
            },
        )
    }

    fn custom_never_function_constant(
        &mut self,
        reference: &crate::plan::ConstantCustomFunctionInstantiation,
    ) -> specialization::Representability<super::ConstantId<super::FunctionLocal>> {
        let instantiation = reference.clone();
        self.lower_function_constant(
            crate::plan::ConstantInstantiation::from_function(
                crate::plan::ConstantFunctionInstantiation::Custom(instantiation.clone()),
            ),
            |templates| templates.materialize_custom_function(&instantiation),
            graph::custom_never_function_expr,
        )
    }

    fn tuple_never_function_constant(
        &mut self,
        reference: &crate::plan::ConstantTupleFunctionInstantiation,
    ) -> specialization::Representability<super::ConstantId<super::FunctionLocal>> {
        let instantiation = reference.clone();
        self.lower_function_constant(
            crate::plan::ConstantInstantiation::from_function(
                crate::plan::ConstantFunctionInstantiation::Tuple(instantiation.clone()),
            ),
            |templates| templates.materialize_tuple_function(&instantiation),
            graph::tuple_never_function_expr,
        )
    }

    fn generic_int_function_constant(
        &mut self,
        reference: &crate::plan::ConstantGenericFunctionInstantiation,
        shape: &SpecializedFunctionShape,
    ) -> specialization::Representability<super::ConstantId<super::FunctionLocal>> {
        self.generic_typed_function_constant(reference, shape, graph::generic_int_function_expr)
    }

    fn generic_float_function_constant(
        &mut self,
        reference: &crate::plan::ConstantGenericFunctionInstantiation,
        shape: &SpecializedFunctionShape,
    ) -> specialization::Representability<super::ConstantId<super::FunctionLocal>> {
        self.generic_typed_function_constant(reference, shape, graph::generic_float_function_expr)
    }

    fn generic_string_function_constant(
        &mut self,
        reference: &crate::plan::ConstantGenericFunctionInstantiation,
        shape: &SpecializedFunctionShape,
    ) -> specialization::Representability<super::ConstantId<super::FunctionLocal>> {
        self.generic_typed_function_constant(reference, shape, graph::generic_string_function_expr)
    }

    fn generic_bit_array_function_constant(
        &mut self,
        reference: &crate::plan::ConstantGenericFunctionInstantiation,
        shape: &SpecializedFunctionShape,
    ) -> specialization::Representability<super::ConstantId<super::FunctionLocal>> {
        self.generic_typed_function_constant(
            reference,
            shape,
            graph::generic_bit_array_function_expr,
        )
    }

    fn generic_utf_codepoint_function_constant(
        &mut self,
        reference: &crate::plan::ConstantGenericFunctionInstantiation,
        shape: &SpecializedFunctionShape,
    ) -> specialization::Representability<super::ConstantId<super::FunctionLocal>> {
        self.generic_typed_function_constant(
            reference,
            shape,
            graph::generic_utf_codepoint_function_expr,
        )
    }

    fn generic_bool_function_constant(
        &mut self,
        reference: &crate::plan::ConstantGenericFunctionInstantiation,
        shape: &SpecializedFunctionShape,
    ) -> specialization::Representability<super::ConstantId<super::FunctionLocal>> {
        self.generic_typed_function_constant(reference, shape, graph::generic_bool_function_expr)
    }

    fn generic_nil_function_constant(
        &mut self,
        reference: &crate::plan::ConstantGenericFunctionInstantiation,
        shape: &SpecializedFunctionShape,
    ) -> specialization::Representability<super::ConstantId<super::FunctionLocal>> {
        self.generic_typed_function_constant(reference, shape, graph::generic_nil_function_expr)
    }

    fn generic_tuple_function_constant(
        &mut self,
        reference: &crate::plan::ConstantGenericFunctionInstantiation,
        shape: &SpecializedFunctionShape,
    ) -> specialization::Representability<super::ConstantId<super::FunctionLocal>> {
        self.generic_typed_function_constant(reference, shape, graph::generic_tuple_function_expr)
    }

    fn generic_typed_function_constant<DraftValue>(
        &mut self,
        reference: &crate::plan::ConstantGenericFunctionInstantiation,
        shape: &SpecializedFunctionShape,
        lower: impl Copy
        + Fn(
            &crate::plan::GenericFunctionExpr,
            &SpecializedFunctionShape,
            graph::DraftCursor,
            &mut graph::DraftGraph,
            &mut Self,
        ) -> specialization::Representability<graph::DraftFlow<DraftValue>>,
    ) -> specialization::Representability<super::ConstantId<super::FunctionLocal>>
    where
        DraftValue: graph::DraftFunctionValue,
    {
        let instantiation = reference.clone();
        self.lower_function_constant(
            crate::plan::ConstantInstantiation::from_function(
                crate::plan::ConstantFunctionInstantiation::Generic(instantiation.clone()),
            ),
            |templates| templates.materialize_generic_function(&instantiation),
            |expression, cursor, graph, context| lower(expression, shape, cursor, graph, context),
        )
    }

    fn generic_custom_function_constant(
        &mut self,
        reference: &crate::plan::ConstantGenericFunctionInstantiation,
        return_shape: &SpecializedCustomValueShape,
        shape: &SpecializedFunctionShape,
    ) -> specialization::Representability<super::ConstantId<super::FunctionLocal>> {
        let instantiation = reference.clone();
        self.lower_function_constant(
            crate::plan::ConstantInstantiation::from_function(
                crate::plan::ConstantFunctionInstantiation::Generic(instantiation.clone()),
            ),
            |templates| templates.materialize_generic_function(&instantiation),
            |expression, cursor, graph, context| {
                graph::generic_custom_function_expr(
                    expression,
                    return_shape,
                    shape,
                    cursor,
                    graph,
                    context,
                )
            },
        )
    }

    fn generic_list_function_constant(
        &mut self,
        reference: &crate::plan::ConstantGenericFunctionInstantiation,
        item_shape: &SpecializedValueShape,
        shape: &SpecializedFunctionShape,
    ) -> specialization::Representability<super::ConstantId<super::FunctionLocal>> {
        let instantiation = reference.clone();
        self.lower_function_constant(
            crate::plan::ConstantInstantiation::from_function(
                crate::plan::ConstantFunctionInstantiation::Generic(instantiation.clone()),
            ),
            |templates| templates.materialize_generic_function(&instantiation),
            |expression, cursor, graph, context| {
                graph::generic_list_function_expr(
                    expression, item_shape, shape, cursor, graph, context,
                )
            },
        )
    }

    fn generic_function_function_constant(
        &mut self,
        reference: &crate::plan::ConstantGenericFunctionInstantiation,
        return_shape: &SpecializedFunctionShape,
        shape: &SpecializedFunctionShape,
    ) -> specialization::Representability<super::ConstantId<super::FunctionLocal>> {
        let instantiation = reference.clone();
        self.lower_function_constant(
            crate::plan::ConstantInstantiation::from_function(
                crate::plan::ConstantFunctionInstantiation::Generic(instantiation.clone()),
            ),
            |templates| templates.materialize_generic_function(&instantiation),
            |expression, cursor, graph, context| {
                graph::generic_function_function_expr(
                    expression,
                    return_shape,
                    shape,
                    cursor,
                    graph,
                    context,
                )
            },
        )
    }

    fn symbolic_custom_function_constant(
        &mut self,
        reference: &crate::plan::ConstantCustomFunctionInstantiation,
        shape: &SpecializedFunctionShape,
    ) -> specialization::Representability<super::ConstantId<super::FunctionLocal>> {
        let instantiation = reference.clone();
        self.lower_function_constant(
            crate::plan::ConstantInstantiation::from_function(
                crate::plan::ConstantFunctionInstantiation::Custom(instantiation.clone()),
            ),
            |templates| templates.materialize_custom_function(&instantiation),
            |expression, cursor, graph, context| {
                graph::symbolic_custom_function_expr_kind(
                    expression.kind(),
                    shape,
                    cursor,
                    graph,
                    context,
                )
            },
        )
    }

    fn symbolic_list_function_constant(
        &mut self,
        reference: &crate::plan::ConstantListFunctionInstantiation,
        shape: &SpecializedFunctionShape,
    ) -> specialization::Representability<super::ConstantId<super::FunctionLocal>> {
        let instantiation = reference.clone();
        self.lower_function_constant(
            crate::plan::ConstantInstantiation::from_function(
                crate::plan::ConstantFunctionInstantiation::List(instantiation.clone()),
            ),
            |templates| templates.materialize_list_function(&instantiation),
            |expression, cursor, graph, context| {
                graph::symbolic_list_function_expr(expression, shape, cursor, graph, context)
            },
        )
    }

    fn symbolic_function_function_constant(
        &mut self,
        reference: &crate::plan::ConstantFunctionFunctionInstantiation,
        shape: &SpecializedFunctionShape,
    ) -> specialization::Representability<super::ConstantId<super::FunctionLocal>> {
        let instantiation = reference.clone();
        self.lower_function_constant(
            crate::plan::ConstantInstantiation::from_function(
                crate::plan::ConstantFunctionInstantiation::Function(instantiation.clone()),
            ),
            |templates| templates.materialize_function_function(&instantiation),
            |expression, cursor, graph, context| {
                graph::symbolic_function_function_expr_kind(
                    expression.kind(),
                    shape,
                    cursor,
                    graph,
                    context,
                )
            },
        )
    }

    fn symbolic_int_function_constant(
        &mut self,
        reference: &crate::plan::ConstantIntFunctionInstantiation,
        shape: &SpecializedFunctionShape,
    ) -> specialization::Representability<super::ConstantId<super::FunctionLocal>> {
        let instantiation = reference.clone();
        self.lower_function_constant(
            crate::plan::ConstantInstantiation::from_function(
                crate::plan::ConstantFunctionInstantiation::Int(instantiation.clone()),
            ),
            |templates| templates.materialize_int_function(&instantiation),
            |expression, cursor, graph, context| {
                graph::symbolic_int_function_expr(expression, shape, cursor, graph, context)
            },
        )
    }

    fn symbolic_float_function_constant(
        &mut self,
        reference: &crate::plan::ConstantFloatFunctionInstantiation,
        shape: &SpecializedFunctionShape,
    ) -> specialization::Representability<super::ConstantId<super::FunctionLocal>> {
        let instantiation = reference.clone();
        self.lower_function_constant(
            crate::plan::ConstantInstantiation::from_function(
                crate::plan::ConstantFunctionInstantiation::Float(instantiation.clone()),
            ),
            |templates| templates.materialize_float_function(&instantiation),
            |expression, cursor, graph, context| {
                graph::symbolic_float_function_expr(expression, shape, cursor, graph, context)
            },
        )
    }

    fn symbolic_string_function_constant(
        &mut self,
        reference: &crate::plan::ConstantStringFunctionInstantiation,
        shape: &SpecializedFunctionShape,
    ) -> specialization::Representability<super::ConstantId<super::FunctionLocal>> {
        let instantiation = reference.clone();
        self.lower_function_constant(
            crate::plan::ConstantInstantiation::from_function(
                crate::plan::ConstantFunctionInstantiation::String(instantiation.clone()),
            ),
            |templates| templates.materialize_string_function(&instantiation),
            |expression, cursor, graph, context| {
                graph::symbolic_string_function_expr(expression, shape, cursor, graph, context)
            },
        )
    }

    fn symbolic_bit_array_function_constant(
        &mut self,
        reference: &crate::plan::ConstantBitArrayFunctionInstantiation,
        shape: &SpecializedFunctionShape,
    ) -> specialization::Representability<super::ConstantId<super::FunctionLocal>> {
        let instantiation = reference.clone();
        self.lower_function_constant(
            crate::plan::ConstantInstantiation::from_function(
                crate::plan::ConstantFunctionInstantiation::BitArray(instantiation.clone()),
            ),
            |templates| templates.materialize_bit_array_function(&instantiation),
            |expression, cursor, graph, context| {
                graph::symbolic_bit_array_function_expr(expression, shape, cursor, graph, context)
            },
        )
    }

    fn symbolic_utf_codepoint_function_constant(
        &mut self,
        reference: &crate::plan::ConstantUtfCodepointFunctionInstantiation,
        shape: &SpecializedFunctionShape,
    ) -> specialization::Representability<super::ConstantId<super::FunctionLocal>> {
        let instantiation = reference.clone();
        self.lower_function_constant(
            crate::plan::ConstantInstantiation::from_function(
                crate::plan::ConstantFunctionInstantiation::UtfCodepoint(instantiation.clone()),
            ),
            |templates| templates.materialize_utf_codepoint_function(&instantiation),
            |expression, cursor, graph, context| {
                graph::symbolic_utf_codepoint_function_expr(
                    expression, shape, cursor, graph, context,
                )
            },
        )
    }

    fn symbolic_bool_function_constant(
        &mut self,
        reference: &crate::plan::ConstantBoolFunctionInstantiation,
        shape: &SpecializedFunctionShape,
    ) -> specialization::Representability<super::ConstantId<super::FunctionLocal>> {
        let instantiation = reference.clone();
        self.lower_function_constant(
            crate::plan::ConstantInstantiation::from_function(
                crate::plan::ConstantFunctionInstantiation::Bool(instantiation.clone()),
            ),
            |templates| templates.materialize_bool_function(&instantiation),
            |expression, cursor, graph, context| {
                graph::symbolic_bool_function_expr(expression, shape, cursor, graph, context)
            },
        )
    }

    fn symbolic_nil_function_constant(
        &mut self,
        reference: &crate::plan::ConstantNilFunctionInstantiation,
        shape: &SpecializedFunctionShape,
    ) -> specialization::Representability<super::ConstantId<super::FunctionLocal>> {
        let instantiation = reference.clone();
        self.lower_function_constant(
            crate::plan::ConstantInstantiation::from_function(
                crate::plan::ConstantFunctionInstantiation::Nil(instantiation.clone()),
            ),
            |templates| templates.materialize_nil_function(&instantiation),
            |expression, cursor, graph, context| {
                graph::symbolic_nil_function_expr(expression, shape, cursor, graph, context)
            },
        )
    }

    fn symbolic_tuple_function_constant(
        &mut self,
        reference: &crate::plan::ConstantTupleFunctionInstantiation,
        shape: &SpecializedFunctionShape,
    ) -> specialization::Representability<super::ConstantId<super::FunctionLocal>> {
        let instantiation = reference.clone();
        self.lower_function_constant(
            crate::plan::ConstantInstantiation::from_function(
                crate::plan::ConstantFunctionInstantiation::Tuple(instantiation.clone()),
            ),
            |templates| templates.materialize_tuple_function(&instantiation),
            |expression, cursor, graph, context| {
                graph::symbolic_tuple_function_expr(expression, shape, cursor, graph, context)
            },
        )
    }

    fn int_function_constant(
        &mut self,
        reference: &crate::plan::ConstantIntFunctionInstantiation,
    ) -> specialization::Representability<super::ConstantId<super::FunctionLocal>> {
        let instantiation = reference.clone();
        self.lower_function_constant(
            crate::plan::ConstantInstantiation::from_function(
                crate::plan::ConstantFunctionInstantiation::Int(instantiation.clone()),
            ),
            |templates| templates.materialize_int_function(&instantiation),
            graph::int_function_expr,
        )
    }

    fn float_function_constant(
        &mut self,
        reference: &crate::plan::ConstantFloatFunctionInstantiation,
    ) -> specialization::Representability<super::ConstantId<super::FunctionLocal>> {
        let instantiation = reference.clone();
        self.lower_function_constant(
            crate::plan::ConstantInstantiation::from_function(
                crate::plan::ConstantFunctionInstantiation::Float(instantiation.clone()),
            ),
            |templates| templates.materialize_float_function(&instantiation),
            graph::float_function_expr,
        )
    }

    fn string_function_constant(
        &mut self,
        reference: &crate::plan::ConstantStringFunctionInstantiation,
    ) -> specialization::Representability<super::ConstantId<super::FunctionLocal>> {
        let instantiation = reference.clone();
        self.lower_function_constant(
            crate::plan::ConstantInstantiation::from_function(
                crate::plan::ConstantFunctionInstantiation::String(instantiation.clone()),
            ),
            |templates| templates.materialize_string_function(&instantiation),
            graph::string_function_expr,
        )
    }

    fn bit_array_function_constant(
        &mut self,
        reference: &crate::plan::ConstantBitArrayFunctionInstantiation,
    ) -> specialization::Representability<super::ConstantId<super::FunctionLocal>> {
        let instantiation = reference.clone();
        self.lower_function_constant(
            crate::plan::ConstantInstantiation::from_function(
                crate::plan::ConstantFunctionInstantiation::BitArray(instantiation.clone()),
            ),
            |templates| templates.materialize_bit_array_function(&instantiation),
            graph::bit_array_function_expr,
        )
    }

    fn utf_codepoint_function_constant(
        &mut self,
        reference: &crate::plan::ConstantUtfCodepointFunctionInstantiation,
    ) -> specialization::Representability<super::ConstantId<super::FunctionLocal>> {
        let instantiation = reference.clone();
        self.lower_function_constant(
            crate::plan::ConstantInstantiation::from_function(
                crate::plan::ConstantFunctionInstantiation::UtfCodepoint(instantiation.clone()),
            ),
            |templates| templates.materialize_utf_codepoint_function(&instantiation),
            graph::utf_codepoint_function_expr,
        )
    }

    fn custom_function_constant(
        &mut self,
        reference: &crate::plan::ConstantCustomFunctionInstantiation,
    ) -> specialization::Representability<super::ConstantId<super::FunctionLocal>> {
        let instantiation = reference.clone();
        self.lower_function_constant(
            crate::plan::ConstantInstantiation::from_function(
                crate::plan::ConstantFunctionInstantiation::Custom(instantiation.clone()),
            ),
            |templates| templates.materialize_custom_function(&instantiation),
            graph::custom_function_expr,
        )
    }

    fn bool_function_constant(
        &mut self,
        reference: &crate::plan::ConstantBoolFunctionInstantiation,
    ) -> specialization::Representability<super::ConstantId<super::FunctionLocal>> {
        let instantiation = reference.clone();
        self.lower_function_constant(
            crate::plan::ConstantInstantiation::from_function(
                crate::plan::ConstantFunctionInstantiation::Bool(instantiation.clone()),
            ),
            |templates| templates.materialize_bool_function(&instantiation),
            graph::bool_function_expr,
        )
    }

    fn nil_function_constant(
        &mut self,
        reference: &crate::plan::ConstantNilFunctionInstantiation,
    ) -> specialization::Representability<super::ConstantId<super::FunctionLocal>> {
        let instantiation = reference.clone();
        self.lower_function_constant(
            crate::plan::ConstantInstantiation::from_function(
                crate::plan::ConstantFunctionInstantiation::Nil(instantiation.clone()),
            ),
            |templates| templates.materialize_nil_function(&instantiation),
            graph::nil_function_expr,
        )
    }

    fn tuple_function_constant(
        &mut self,
        reference: &crate::plan::ConstantTupleFunctionInstantiation,
    ) -> specialization::Representability<super::ConstantId<super::FunctionLocal>> {
        let instantiation = reference.clone();
        self.lower_function_constant(
            crate::plan::ConstantInstantiation::from_function(
                crate::plan::ConstantFunctionInstantiation::Tuple(instantiation.clone()),
            ),
            |templates| templates.materialize_tuple_function(&instantiation),
            graph::tuple_function_expr,
        )
    }

    fn list_function_constant(
        &mut self,
        reference: &crate::plan::ConstantListFunctionInstantiation,
    ) -> specialization::Representability<super::ConstantId<super::FunctionLocal>> {
        let instantiation = reference.clone();
        self.lower_function_constant(
            crate::plan::ConstantInstantiation::from_function(
                crate::plan::ConstantFunctionInstantiation::List(instantiation.clone()),
            ),
            |templates| templates.materialize_list_function(&instantiation),
            graph::list_function_expr,
        )
    }

    fn function_function_constant(
        &mut self,
        reference: &crate::plan::ConstantFunctionFunctionInstantiation,
    ) -> specialization::Representability<super::ConstantId<super::FunctionLocal>> {
        let instantiation = reference.clone();
        self.lower_function_constant(
            crate::plan::ConstantInstantiation::from_function(
                crate::plan::ConstantFunctionInstantiation::Function(instantiation.clone()),
            ),
            |templates| templates.materialize_function_function(&instantiation),
            graph::function_function_expr,
        )
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
                let specialization =
                    self.reserve_provisional_specialization(key, table::FunctionTableFamily::Never);
                super::RuntimeFunctionId::Never(super::NeverFunctionId(specialization.index))
            }
            specialization::ValueInhabitation::Inhabited(return_shape) => {
                let family =
                    table::stored_function_table_family(&return_shape, &self.representations);
                let specialization = self.reserve_provisional_specialization(key, family);
                table::function_id(
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
        family: table::FunctionTableFamily,
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
        family: table::FunctionTableFamily,
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
        family: table::FunctionTableFamily,
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
        self.provisional_specialization(key, table::FunctionTableFamily::Never)
            .map(|specialization| super::NeverFunctionId(specialization.index))
    }

    fn next_function_index(&mut self, family: table::FunctionTableFamily) -> usize {
        let next = self.next_function_indices.entry(family).or_default();
        let index = *next;
        *next += 1;
        index
    }

    fn reserve_function_id<Function>(
        &mut self,
        function: &crate::plan::FunctionInstantiation,
        family: table::FunctionTableFamily,
        lower: impl FnOnce(usize, &mut Self) -> Function,
    ) -> specialization::Representability<Function> {
        self.reserve_index_for(function, family)
            .map(|index| lower(index, self))
    }

    fn int_function_id(
        &mut self,
        function: &crate::plan::FunctionInstantiation,
    ) -> specialization::Representability<super::IntFunctionId> {
        self.reserve_function_id(function, table::FunctionTableFamily::Int, |index, _| {
            super::IntFunctionId(index)
        })
    }

    fn float_function_id(
        &mut self,
        function: &crate::plan::FunctionInstantiation,
    ) -> specialization::Representability<super::FloatFunctionId> {
        self.reserve_function_id(function, table::FunctionTableFamily::Float, |index, _| {
            super::FloatFunctionId(index)
        })
    }

    fn string_function_id(
        &mut self,
        function: &crate::plan::FunctionInstantiation,
    ) -> specialization::Representability<super::StringFunctionId> {
        self.reserve_function_id(function, table::FunctionTableFamily::String, |index, _| {
            super::StringFunctionId(index)
        })
    }

    fn bit_array_function_id(
        &mut self,
        function: &crate::plan::FunctionInstantiation,
    ) -> specialization::Representability<super::BitArrayFunctionId> {
        self.reserve_function_id(
            function,
            table::FunctionTableFamily::BitArray,
            |index, _| super::BitArrayFunctionId(index),
        )
    }

    fn utf_codepoint_function_id(
        &mut self,
        function: &crate::plan::FunctionInstantiation,
    ) -> specialization::Representability<super::UtfCodepointFunctionId> {
        self.reserve_function_id(
            function,
            table::FunctionTableFamily::UtfCodepoint,
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
            table::FunctionTableFamily::Custom,
            |index, context| {
                super::CustomFunctionId::new(index, context.types.custom_value_shape(shape))
            },
        )
    }

    fn bool_function_id(
        &mut self,
        function: &crate::plan::FunctionInstantiation,
    ) -> specialization::Representability<super::BoolFunctionId> {
        self.reserve_function_id(function, table::FunctionTableFamily::Bool, |index, _| {
            super::BoolFunctionId(index)
        })
    }

    fn nil_function_id(
        &mut self,
        function: &crate::plan::FunctionInstantiation,
    ) -> specialization::Representability<super::NilFunctionId> {
        self.reserve_function_id(function, table::FunctionTableFamily::Nil, |index, _| {
            super::NilFunctionId(index)
        })
    }

    fn tuple_function_id(
        &mut self,
        function: &crate::plan::FunctionInstantiation,
    ) -> specialization::Representability<super::TupleFunctionId> {
        self.reserve_function_id(function, table::FunctionTableFamily::Tuple, |index, _| {
            super::TupleFunctionId(index)
        })
    }

    fn list_function_id(
        &mut self,
        function: &crate::plan::FunctionInstantiation,
        item: &SpecializedValueShape,
    ) -> specialization::Representability<super::ListFunctionId> {
        self.reserve_function_id(
            function,
            table::list_function_table_family(item),
            |index, context| table::list_function_id(item, index, &mut context.types),
        )
    }

    fn int_list_function_id(
        &mut self,
        function: &crate::plan::FunctionInstantiation,
    ) -> specialization::Representability<super::IntListFunctionId> {
        self.reserve_function_id(
            function,
            table::FunctionTableFamily::IntList,
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
            table::FunctionTableFamily::ParameterList,
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
            table::FunctionTableFamily::StringList,
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
            table::FunctionTableFamily::BitArrayList,
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
            table::FunctionTableFamily::UtfCodepointList,
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
            table::FunctionTableFamily::CustomList,
            |index, _| super::CustomListFunctionId::new(index, type_id),
        )
    }

    fn float_list_function_id(
        &mut self,
        function: &crate::plan::FunctionInstantiation,
    ) -> specialization::Representability<super::FloatListFunctionId> {
        self.reserve_function_id(
            function,
            table::FunctionTableFamily::FloatList,
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
            table::FunctionTableFamily::BoolList,
            |index, context| super::BoolListFunctionId::new(index, context.types.bool_list_type()),
        )
    }

    fn nil_list_function_id(
        &mut self,
        function: &crate::plan::FunctionInstantiation,
    ) -> specialization::Representability<super::NilListFunctionId> {
        self.reserve_function_id(
            function,
            table::FunctionTableFamily::NilList,
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
            table::FunctionTableFamily::TupleList,
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
            table::FunctionTableFamily::ListList,
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
            table::FunctionTableFamily::ParameterListList,
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
            table::FunctionTableFamily::FunctionList,
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
            table::function_function_table_family(return_, &self.representations),
            |index, context| {
                table::function_function_id(
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
            table::FunctionTableFamily::IntFunction,
            |index, _| super::IntFunctionFunctionId(index),
        )
    }

    fn float_function_function_id(
        &mut self,
        function: &crate::plan::FunctionInstantiation,
    ) -> specialization::Representability<super::FloatFunctionFunctionId> {
        self.reserve_function_id(
            function,
            table::FunctionTableFamily::FloatFunction,
            |index, _| super::FloatFunctionFunctionId(index),
        )
    }

    fn string_function_function_id(
        &mut self,
        function: &crate::plan::FunctionInstantiation,
    ) -> specialization::Representability<super::StringFunctionFunctionId> {
        self.reserve_function_id(
            function,
            table::FunctionTableFamily::StringFunction,
            |index, _| super::StringFunctionFunctionId(index),
        )
    }

    fn bit_array_function_function_id(
        &mut self,
        function: &crate::plan::FunctionInstantiation,
    ) -> specialization::Representability<super::BitArrayFunctionFunctionId> {
        self.reserve_function_id(
            function,
            table::FunctionTableFamily::BitArrayFunction,
            |index, _| super::BitArrayFunctionFunctionId(index),
        )
    }

    fn utf_codepoint_function_function_id(
        &mut self,
        function: &crate::plan::FunctionInstantiation,
    ) -> specialization::Representability<super::UtfCodepointFunctionFunctionId> {
        self.reserve_function_id(
            function,
            table::FunctionTableFamily::UtfCodepointFunction,
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
            table::FunctionTableFamily::CustomFunction,
            |index, _| super::CustomFunctionFunctionId::new(index, type_),
        )
    }

    fn bool_function_function_id(
        &mut self,
        function: &crate::plan::FunctionInstantiation,
    ) -> specialization::Representability<super::BoolFunctionFunctionId> {
        self.reserve_function_id(
            function,
            table::FunctionTableFamily::BoolFunction,
            |index, _| super::BoolFunctionFunctionId(index),
        )
    }

    fn nil_function_function_id(
        &mut self,
        function: &crate::plan::FunctionInstantiation,
    ) -> specialization::Representability<super::NilFunctionFunctionId> {
        self.reserve_function_id(
            function,
            table::FunctionTableFamily::NilFunction,
            |index, _| super::NilFunctionFunctionId(index),
        )
    }

    fn tuple_function_function_id(
        &mut self,
        function: &crate::plan::FunctionInstantiation,
    ) -> specialization::Representability<super::TupleFunctionFunctionId> {
        self.reserve_function_id(
            function,
            table::FunctionTableFamily::TupleFunction,
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
            table::list_function_function_table_family(item),
            |index, context| {
                table::list_function_function_id(type_, item, index, &mut context.types)
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
            table::FunctionTableFamily::FunctionFunction,
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
            table::FunctionTableFamily::GenericFunction,
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
            table::FunctionTableFamily::NeverFunction,
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
            table::lower_specialized(templates.get(key.template()), &key, &mut context);
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
                .provisional_specialization(seeded.clone(), super::table::FunctionTableFamily::Int,)
                .map(|specialization| specialization.index),
            Representability::Inhabited(0),
        );
        assert_eq!(context.provisional_specializations[&seeded].index, 0);
        assert_eq!(context.pending, VecDeque::from([seeded]));

        context.erased_specializations.insert(erased.clone());
        assert!(!context.provisional_specializations.contains_key(&erased));
        assert_eq!(
            context
                .provisional_specialization(erased, super::table::FunctionTableFamily::Never)
                .map(|_| ()),
            Representability::Uninhabited,
        );
    }
}
