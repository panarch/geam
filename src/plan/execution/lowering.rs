mod constant;
mod expression;
mod frame;
mod id;
mod param;
mod pattern;
mod return_;
mod specialization;
mod step;
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
    functions: super::table::FunctionTables,
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
    frame_templates: HashMap<crate::plan::FunctionTemplateId, frame::LocalAllocationTemplate>,
    specialization_locals: HashMap<SpecializationKey, frame::LocalAllocationPlan>,
    current_specialization: SpecializationKey,
    next_function_indices: HashMap<table::FunctionTableFamily, usize>,
    provisional_specializations: HashMap<SpecializationKey, ProvisionalSpecialization>,
    erased_specializations: HashSet<SpecializationKey>,
    pending: VecDeque<SpecializationKey>,
    substitution: SpecializedTypeSubstitution,
    pending_return_divergence: Option<super::NeverExpr>,
}

struct StoredTargetLocal {
    index: usize,
    shape: specialization::StoredValueShape,
    substitution: SpecializedTypeSubstitution,
}

impl StoredTargetLocal {
    fn index(&self) -> usize {
        self.index
    }

    fn shape(&self) -> &specialization::StoredValueShape {
        &self.shape
    }

    fn substitution(&self) -> &SpecializedTypeSubstitution {
        &self.substitution
    }

    fn custom_shape(&self, shape: &crate::plan::CustomValueShape) -> SpecializedCustomValueShape {
        SpecializedCustomValueShape::instantiate(shape, &self.substitution)
    }

    fn function_shape(&self, shape: &crate::plan::FunctionShape) -> SpecializedFunctionShape {
        SpecializedFunctionShape::instantiate(shape, &self.substitution)
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
        main: SpecializationKey,
        representations: RepresentationContext,
        constant_templates: crate::plan::ConstantTemplates,
        erased_specializations: HashSet<SpecializationKey>,
    ) -> Self {
        let frame_templates = templates
            .templates
            .iter()
            .map(|template| (template.id(), frame::LocalAllocationTemplate::new(template)))
            .collect::<HashMap<_, _>>();
        let main_locals =
            frame_templates[&main.template()].specialize(main.substitution(), &representations);
        let mut specialization_locals = HashMap::new();
        specialization_locals.insert(main.clone(), main_locals);
        Self {
            constant_templates,
            constants: constant::ConstantLowering::default(),
            types: value_type::TypeInterner::new(),
            representations,
            functions: table::FunctionTableBuilder::default(),
            frame_templates,
            specialization_locals,
            current_specialization: main,
            next_function_indices: HashMap::new(),
            provisional_specializations: HashMap::new(),
            erased_specializations,
            pending: VecDeque::new(),
            substitution: SpecializedTypeSubstitution::empty(),
            pending_return_divergence: None,
        }
    }

    fn lower_constant<ModuleValue, ExecutionValue: super::ConstantExpression>(
        &mut self,
        instantiation: crate::plan::ConstantInstantiation,
        materialize: impl FnOnce(&crate::plan::ConstantTemplates) -> ModuleValue,
        lower: impl FnOnce(&ModuleValue, &mut Self) -> specialization::Representability<ExecutionValue>,
    ) -> specialization::Representability<super::ConstantId<ExecutionValue>> {
        let outer = self.substitution.to_module_substitution();
        let key = instantiation.substitute(&outer);
        if let Some(id) = self.constants.get(&key) {
            return specialization::Representability::Inhabited(id);
        }

        let value = materialize(&self.constant_templates);
        lower(&value, self).map(|value| self.constants.insert(key, value))
    }

    fn int_constant(
        &mut self,
        reference: &crate::plan::ConstantIntReference,
    ) -> specialization::Representability<super::ConstantId<super::IntExpr>> {
        let instantiation = reference.instantiation().clone();
        self.lower_constant(
            crate::plan::ConstantInstantiation::from_int(instantiation.clone()),
            |templates| templates.materialize_int(&instantiation),
            expression::int_expr,
        )
    }

    fn string_constant(
        &mut self,
        reference: &crate::plan::ConstantStringReference,
    ) -> specialization::Representability<super::ConstantId<super::StringExpr>> {
        let instantiation = reference.instantiation().clone();
        self.lower_constant(
            crate::plan::ConstantInstantiation::from_string(instantiation.clone()),
            |templates| templates.materialize_string(&instantiation),
            expression::string_expr,
        )
    }

    fn bit_array_constant(
        &mut self,
        reference: &crate::plan::ConstantBitArrayReference,
    ) -> specialization::Representability<super::ConstantId<super::BitArrayExpr>> {
        let instantiation = reference.instantiation().clone();
        self.lower_constant(
            crate::plan::ConstantInstantiation::from_bit_array(instantiation.clone()),
            |templates| templates.materialize_bit_array(&instantiation),
            expression::bit_array_expr,
        )
    }

    fn custom_constant(
        &mut self,
        reference: &crate::plan::ConstantCustomReference,
    ) -> specialization::Representability<super::ConstantId<super::CustomExpr>> {
        let instantiation = reference.instantiation().clone();
        self.lower_constant(
            crate::plan::ConstantInstantiation::from_custom(instantiation.clone()),
            |templates| templates.materialize_custom(&instantiation),
            expression::custom_expr,
        )
    }

    fn float_constant(
        &mut self,
        reference: &crate::plan::ConstantFloatReference,
    ) -> specialization::Representability<super::ConstantId<super::FloatExpr>> {
        let instantiation = reference.instantiation().clone();
        self.lower_constant(
            crate::plan::ConstantInstantiation::from_float(instantiation.clone()),
            |templates| templates.materialize_float(&instantiation),
            expression::float_expr,
        )
    }

    fn bool_constant(
        &mut self,
        reference: &crate::plan::ConstantBoolReference,
    ) -> specialization::Representability<super::ConstantId<super::BoolExpr>> {
        let instantiation = reference.instantiation().clone();
        self.lower_constant(
            crate::plan::ConstantInstantiation::from_bool(instantiation.clone()),
            |templates| templates.materialize_bool(&instantiation),
            expression::bool_expr,
        )
    }

    fn nil_constant(
        &mut self,
        reference: &crate::plan::ConstantNilReference,
    ) -> specialization::Representability<super::ConstantId<super::NilExpr>> {
        let instantiation = reference.instantiation().clone();
        self.lower_constant(
            crate::plan::ConstantInstantiation::from_nil(instantiation.clone()),
            |templates| templates.materialize_nil(&instantiation),
            expression::nil_expr,
        )
    }

    fn tuple_constant(
        &mut self,
        reference: &crate::plan::ConstantTupleReference,
    ) -> specialization::Representability<super::ConstantId<super::TupleExpr>> {
        let instantiation = reference.instantiation().clone();
        self.lower_constant(
            crate::plan::ConstantInstantiation::from_tuple(instantiation.clone()),
            |templates| templates.materialize_tuple(&instantiation),
            expression::tuple_expr,
        )
    }

    fn int_list_constant(
        &mut self,
        reference: &crate::plan::ConstantIntListInstantiation,
    ) -> specialization::Representability<super::ConstantId<super::IntListExpr>> {
        let instantiation = reference.clone();
        self.lower_constant(
            crate::plan::ConstantInstantiation::from_list(
                crate::plan::ConstantListInstantiation::Int(instantiation.clone()),
            ),
            |templates| templates.materialize_int_list(&instantiation),
            expression::int_list_expr,
        )
    }

    fn string_list_constant(
        &mut self,
        reference: &crate::plan::ConstantStringListInstantiation,
    ) -> specialization::Representability<super::ConstantId<super::StringListExpr>> {
        let instantiation = reference.clone();
        self.lower_constant(
            crate::plan::ConstantInstantiation::from_list(
                crate::plan::ConstantListInstantiation::String(instantiation.clone()),
            ),
            |templates| templates.materialize_string_list(&instantiation),
            expression::string_list_expr,
        )
    }

    fn bit_array_list_constant(
        &mut self,
        reference: &crate::plan::ConstantBitArrayListInstantiation,
    ) -> specialization::Representability<super::ConstantId<super::BitArrayListExpr>> {
        let instantiation = reference.clone();
        self.lower_constant(
            crate::plan::ConstantInstantiation::from_list(
                crate::plan::ConstantListInstantiation::BitArray(instantiation.clone()),
            ),
            |templates| templates.materialize_bit_array_list(&instantiation),
            expression::bit_array_list_expr,
        )
    }

    fn utf_codepoint_list_constant(
        &mut self,
        reference: &crate::plan::ConstantUtfCodepointListInstantiation,
    ) -> specialization::Representability<super::ConstantId<super::UtfCodepointListExpr>> {
        let instantiation = reference.clone();
        self.lower_constant(
            crate::plan::ConstantInstantiation::from_list(
                crate::plan::ConstantListInstantiation::UtfCodepoint(instantiation.clone()),
            ),
            |templates| templates.materialize_utf_codepoint_list(&instantiation),
            expression::utf_codepoint_list_expr,
        )
    }

    fn custom_list_constant(
        &mut self,
        reference: &crate::plan::ConstantCustomListInstantiation,
    ) -> specialization::Representability<super::ConstantId<super::CustomListExpr>> {
        let instantiation = reference.clone();
        self.lower_constant(
            crate::plan::ConstantInstantiation::from_list(
                crate::plan::ConstantListInstantiation::Custom(instantiation.clone()),
            ),
            |templates| templates.materialize_custom_list(&instantiation),
            expression::custom_list_expr,
        )
    }

    fn float_list_constant(
        &mut self,
        reference: &crate::plan::ConstantFloatListInstantiation,
    ) -> specialization::Representability<super::ConstantId<super::FloatListExpr>> {
        let instantiation = reference.clone();
        self.lower_constant(
            crate::plan::ConstantInstantiation::from_list(
                crate::plan::ConstantListInstantiation::Float(instantiation.clone()),
            ),
            |templates| templates.materialize_float_list(&instantiation),
            expression::float_list_expr,
        )
    }

    fn bool_list_constant(
        &mut self,
        reference: &crate::plan::ConstantBoolListInstantiation,
    ) -> specialization::Representability<super::ConstantId<super::BoolListExpr>> {
        let instantiation = reference.clone();
        self.lower_constant(
            crate::plan::ConstantInstantiation::from_list(
                crate::plan::ConstantListInstantiation::Bool(instantiation.clone()),
            ),
            |templates| templates.materialize_bool_list(&instantiation),
            expression::bool_list_expr,
        )
    }

    fn nil_list_constant(
        &mut self,
        reference: &crate::plan::ConstantNilListInstantiation,
    ) -> specialization::Representability<super::ConstantId<super::NilListExpr>> {
        let instantiation = reference.clone();
        self.lower_constant(
            crate::plan::ConstantInstantiation::from_list(
                crate::plan::ConstantListInstantiation::Nil(instantiation.clone()),
            ),
            |templates| templates.materialize_nil_list(&instantiation),
            expression::nil_list_expr,
        )
    }

    fn tuple_list_constant(
        &mut self,
        reference: &crate::plan::ConstantTupleListInstantiation,
    ) -> specialization::Representability<super::ConstantId<super::TupleListExpr>> {
        let instantiation = reference.clone();
        self.lower_constant(
            crate::plan::ConstantInstantiation::from_list(
                crate::plan::ConstantListInstantiation::Tuple(instantiation.clone()),
            ),
            |templates| templates.materialize_tuple_list(&instantiation),
            expression::tuple_list_expr,
        )
    }

    fn parameter_list_list_constant(
        &mut self,
        reference: &crate::plan::ConstantParameterListListInstantiation,
        parameter: crate::plan::TypeParameterId,
    ) -> specialization::Representability<super::ConstantId<super::ParameterListListExpr>> {
        let instantiation = reference.clone();
        self.lower_constant(
            crate::plan::ConstantInstantiation::from_list(
                crate::plan::ConstantListInstantiation::ParameterList(instantiation.clone()),
            ),
            |templates| templates.materialize_parameter_list_list(&instantiation),
            |expression, context| {
                expression::unresolved_parameter_list_list_expr(expression, parameter, context)
            },
        )
    }

    fn list_list_constant(
        &mut self,
        reference: &crate::plan::ConstantListListInstantiation,
    ) -> specialization::Representability<super::ConstantId<super::ListListExpr>> {
        let instantiation = reference.clone();
        self.lower_constant(
            crate::plan::ConstantInstantiation::from_list(
                crate::plan::ConstantListInstantiation::List(instantiation.clone()),
            ),
            |templates| templates.materialize_list_list(&instantiation),
            expression::list_list_expr,
        )
    }

    fn function_list_constant(
        &mut self,
        reference: &crate::plan::ConstantFunctionListInstantiation,
    ) -> specialization::Representability<super::ConstantId<super::FunctionListExpr>> {
        let instantiation = reference.clone();
        self.lower_constant(
            crate::plan::ConstantInstantiation::from_list(
                crate::plan::ConstantListInstantiation::Function(instantiation.clone()),
            ),
            |templates| templates.materialize_function_list(&instantiation),
            expression::function_list_expr,
        )
    }

    fn lower_generic_list_constant<ExecutionValue: super::ConstantExpression>(
        &mut self,
        reference: &crate::plan::ConstantGenericListInstantiation,
        lower: impl FnOnce(
            &crate::plan::module::GenericListExpr,
            &mut Self,
        ) -> specialization::Representability<ExecutionValue>,
    ) -> specialization::Representability<super::ConstantId<ExecutionValue>> {
        let instantiation = reference.clone();
        self.lower_constant(
            crate::plan::ConstantInstantiation::from_list(
                crate::plan::ConstantListInstantiation::Generic(instantiation.clone()),
            ),
            |templates| templates.materialize_generic_list(&instantiation),
            lower,
        )
    }

    fn generic_parameter_list_constant(
        &mut self,
        reference: &crate::plan::ConstantGenericListInstantiation,
        parameter: crate::plan::TypeParameterId,
    ) -> specialization::Representability<super::ConstantId<super::ParameterListExpr>> {
        self.lower_generic_list_constant(reference, |expression, context| {
            expression::parameter_list_expr(expression, parameter, context)
        })
    }

    fn generic_int_list_constant(
        &mut self,
        reference: &crate::plan::ConstantGenericListInstantiation,
    ) -> specialization::Representability<super::ConstantId<super::IntListExpr>> {
        self.lower_generic_list_constant(reference, expression::generic_int_list_expr)
    }

    fn generic_string_list_constant(
        &mut self,
        reference: &crate::plan::ConstantGenericListInstantiation,
    ) -> specialization::Representability<super::ConstantId<super::StringListExpr>> {
        self.lower_generic_list_constant(reference, expression::generic_string_list_expr)
    }

    fn generic_bit_array_list_constant(
        &mut self,
        reference: &crate::plan::ConstantGenericListInstantiation,
    ) -> specialization::Representability<super::ConstantId<super::BitArrayListExpr>> {
        self.lower_generic_list_constant(reference, expression::generic_bit_array_list_expr)
    }

    fn generic_utf_codepoint_list_constant(
        &mut self,
        reference: &crate::plan::ConstantGenericListInstantiation,
    ) -> specialization::Representability<super::ConstantId<super::UtfCodepointListExpr>> {
        self.lower_generic_list_constant(reference, expression::generic_utf_codepoint_list_expr)
    }

    fn generic_custom_list_constant(
        &mut self,
        reference: &crate::plan::ConstantGenericListInstantiation,
        shape: &SpecializedCustomValueShape,
    ) -> specialization::Representability<super::ConstantId<super::CustomListExpr>> {
        self.lower_generic_list_constant(reference, |expression, context| {
            expression::generic_custom_list_expr(expression, shape, context)
        })
    }

    fn generic_float_list_constant(
        &mut self,
        reference: &crate::plan::ConstantGenericListInstantiation,
    ) -> specialization::Representability<super::ConstantId<super::FloatListExpr>> {
        self.lower_generic_list_constant(reference, expression::generic_float_list_expr)
    }

    fn generic_bool_list_constant(
        &mut self,
        reference: &crate::plan::ConstantGenericListInstantiation,
    ) -> specialization::Representability<super::ConstantId<super::BoolListExpr>> {
        self.lower_generic_list_constant(reference, expression::generic_bool_list_expr)
    }

    fn generic_nil_list_constant(
        &mut self,
        reference: &crate::plan::ConstantGenericListInstantiation,
    ) -> specialization::Representability<super::ConstantId<super::NilListExpr>> {
        self.lower_generic_list_constant(reference, expression::generic_nil_list_expr)
    }

    fn generic_tuple_list_constant(
        &mut self,
        reference: &crate::plan::ConstantGenericListInstantiation,
        elements: &[SpecializedValueShape],
    ) -> specialization::Representability<super::ConstantId<super::TupleListExpr>> {
        self.lower_generic_list_constant(reference, |expression, context| {
            expression::generic_tuple_list_expr(expression, elements, context)
        })
    }

    fn generic_parameter_list_list_constant(
        &mut self,
        reference: &crate::plan::ConstantGenericListInstantiation,
        parameter: crate::plan::TypeParameterId,
    ) -> specialization::Representability<super::ConstantId<super::ParameterListListExpr>> {
        self.lower_generic_list_constant(reference, |expression, context| {
            expression::generic_parameter_list_list_expr(expression, parameter, context)
        })
    }

    fn generic_list_list_constant(
        &mut self,
        reference: &crate::plan::ConstantGenericListInstantiation,
        item_shape: &specialization::StoredValueShape,
    ) -> specialization::Representability<super::ConstantId<super::ListListExpr>> {
        self.lower_generic_list_constant(reference, |expression, context| {
            expression::generic_stored_nested_list_expr(expression, item_shape, context)
        })
    }

    fn generic_function_list_constant(
        &mut self,
        reference: &crate::plan::ConstantGenericListInstantiation,
        shape: &SpecializedFunctionShape,
    ) -> specialization::Representability<super::ConstantId<super::FunctionListExpr>> {
        self.lower_generic_list_constant(reference, |expression, context| {
            expression::generic_function_list_expr(expression, shape, context)
        })
    }

    fn parameter_list_list_as_stored_constant(
        &mut self,
        reference: &crate::plan::ConstantParameterListListInstantiation,
        item_shape: &specialization::StoredValueShape,
    ) -> specialization::Representability<super::ConstantId<super::ListListExpr>> {
        let instantiation = reference.clone();
        self.lower_constant(
            crate::plan::ConstantInstantiation::from_list(
                crate::plan::ConstantListInstantiation::ParameterList(instantiation.clone()),
            ),
            |templates| templates.materialize_parameter_list_list(&instantiation),
            |expression, context| {
                expression::concrete_parameter_list_list_expr(expression, item_shape, context)
            },
        )
    }

    fn generic_function_constant(
        &mut self,
        reference: &crate::plan::ConstantGenericFunctionInstantiation,
    ) -> specialization::Representability<super::ConstantId<super::GenericFunctionExpr>> {
        let instantiation = reference.clone();
        self.lower_constant(
            crate::plan::ConstantInstantiation::from_function(
                crate::plan::ConstantFunctionInstantiation::Generic(instantiation.clone()),
            ),
            |templates| templates.materialize_generic_function(&instantiation),
            expression::generic_symbolic_function_expr,
        )
    }

    fn generic_never_function_constant(
        &mut self,
        reference: &crate::plan::ConstantGenericFunctionInstantiation,
    ) -> specialization::Representability<super::ConstantId<super::NeverFunctionExpr>> {
        self.lower_generic_function_constant(reference, expression::generic_never_function_expr)
    }

    fn custom_never_function_constant(
        &mut self,
        reference: &crate::plan::ConstantCustomFunctionInstantiation,
    ) -> specialization::Representability<super::ConstantId<super::NeverFunctionExpr>> {
        let instantiation = reference.clone();
        self.lower_constant(
            crate::plan::ConstantInstantiation::from_function(
                crate::plan::ConstantFunctionInstantiation::Custom(instantiation.clone()),
            ),
            |templates| templates.materialize_custom_function(&instantiation),
            expression::custom_never_function_expr,
        )
    }

    fn tuple_never_function_constant(
        &mut self,
        reference: &crate::plan::ConstantTupleFunctionInstantiation,
    ) -> specialization::Representability<super::ConstantId<super::NeverFunctionExpr>> {
        let instantiation = reference.clone();
        self.lower_constant(
            crate::plan::ConstantInstantiation::from_function(
                crate::plan::ConstantFunctionInstantiation::Tuple(instantiation.clone()),
            ),
            |templates| templates.materialize_tuple_function(&instantiation),
            expression::tuple_never_function_expr,
        )
    }

    fn lower_generic_function_constant<ExecutionValue: super::ConstantExpression>(
        &mut self,
        reference: &crate::plan::ConstantGenericFunctionInstantiation,
        lower: impl FnOnce(
            &crate::plan::GenericFunctionExpr,
            &mut Self,
        ) -> specialization::Representability<ExecutionValue>,
    ) -> specialization::Representability<super::ConstantId<ExecutionValue>> {
        let instantiation = reference.clone();
        self.lower_constant(
            crate::plan::ConstantInstantiation::from_function(
                crate::plan::ConstantFunctionInstantiation::Generic(instantiation.clone()),
            ),
            |templates| templates.materialize_generic_function(&instantiation),
            lower,
        )
    }

    fn generic_int_function_constant(
        &mut self,
        reference: &crate::plan::ConstantGenericFunctionInstantiation,
    ) -> specialization::Representability<super::ConstantId<super::IntFunctionExpr>> {
        self.lower_generic_function_constant(reference, expression::generic_int_function_expr)
    }

    fn generic_string_function_constant(
        &mut self,
        reference: &crate::plan::ConstantGenericFunctionInstantiation,
    ) -> specialization::Representability<super::ConstantId<super::StringFunctionExpr>> {
        self.lower_generic_function_constant(reference, expression::generic_string_function_expr)
    }

    fn generic_bit_array_function_constant(
        &mut self,
        reference: &crate::plan::ConstantGenericFunctionInstantiation,
    ) -> specialization::Representability<super::ConstantId<super::BitArrayFunctionExpr>> {
        self.lower_generic_function_constant(reference, expression::generic_bit_array_function_expr)
    }

    fn generic_utf_codepoint_function_constant(
        &mut self,
        reference: &crate::plan::ConstantGenericFunctionInstantiation,
    ) -> specialization::Representability<super::ConstantId<super::UtfCodepointFunctionExpr>> {
        self.lower_generic_function_constant(
            reference,
            expression::generic_utf_codepoint_function_expr,
        )
    }

    fn generic_float_function_constant(
        &mut self,
        reference: &crate::plan::ConstantGenericFunctionInstantiation,
    ) -> specialization::Representability<super::ConstantId<super::FloatFunctionExpr>> {
        self.lower_generic_function_constant(reference, expression::generic_float_function_expr)
    }

    fn generic_bool_function_constant(
        &mut self,
        reference: &crate::plan::ConstantGenericFunctionInstantiation,
    ) -> specialization::Representability<super::ConstantId<super::BoolFunctionExpr>> {
        self.lower_generic_function_constant(reference, expression::generic_bool_function_expr)
    }

    fn generic_nil_function_constant(
        &mut self,
        reference: &crate::plan::ConstantGenericFunctionInstantiation,
    ) -> specialization::Representability<super::ConstantId<super::NilFunctionExpr>> {
        self.lower_generic_function_constant(reference, expression::generic_nil_function_expr)
    }

    fn generic_tuple_function_constant(
        &mut self,
        reference: &crate::plan::ConstantGenericFunctionInstantiation,
    ) -> specialization::Representability<super::ConstantId<super::TupleFunctionExpr>> {
        self.lower_generic_function_constant(reference, expression::generic_tuple_function_expr)
    }

    fn generic_custom_function_constant(
        &mut self,
        reference: &crate::plan::ConstantGenericFunctionInstantiation,
        return_shape: &SpecializedCustomValueShape,
    ) -> specialization::Representability<super::ConstantId<super::CustomFunctionExpr>> {
        self.lower_generic_function_constant(reference, |expression, context| {
            expression::generic_custom_function_expr(expression, return_shape, context)
        })
    }

    fn symbolic_custom_function_constant(
        &mut self,
        reference: &crate::plan::ConstantCustomFunctionInstantiation,
        shape: &SpecializedFunctionShape,
    ) -> specialization::Representability<super::ConstantId<super::GenericFunctionExpr>> {
        let instantiation = reference.clone();
        self.lower_constant(
            crate::plan::ConstantInstantiation::from_function(
                crate::plan::ConstantFunctionInstantiation::Custom(instantiation.clone()),
            ),
            |templates| templates.materialize_custom_function(&instantiation),
            |expression, context| {
                expression::symbolic_custom_function_expr(expression, shape, context)
            },
        )
    }

    fn symbolic_list_function_constant(
        &mut self,
        reference: &crate::plan::ConstantListFunctionInstantiation,
        shape: &SpecializedFunctionShape,
    ) -> specialization::Representability<super::ConstantId<super::GenericFunctionExpr>> {
        let instantiation = reference.clone();
        self.lower_constant(
            crate::plan::ConstantInstantiation::from_function(
                crate::plan::ConstantFunctionInstantiation::List(instantiation.clone()),
            ),
            |templates| templates.materialize_list_function(&instantiation),
            |expression, context| {
                expression::symbolic_list_function_expr(expression, shape, context)
            },
        )
    }

    fn symbolic_int_function_constant(
        &mut self,
        reference: &crate::plan::ConstantIntFunctionInstantiation,
        shape: &SpecializedFunctionShape,
    ) -> specialization::Representability<super::ConstantId<super::GenericFunctionExpr>> {
        let instantiation = reference.clone();
        self.lower_constant(
            crate::plan::ConstantInstantiation::from_function(
                crate::plan::ConstantFunctionInstantiation::Int(instantiation.clone()),
            ),
            |templates| templates.materialize_int_function(&instantiation),
            |expression, context| {
                expression::symbolic_int_function_expr(expression, shape, context)
            },
        )
    }

    fn symbolic_float_function_constant(
        &mut self,
        reference: &crate::plan::ConstantFloatFunctionInstantiation,
        shape: &SpecializedFunctionShape,
    ) -> specialization::Representability<super::ConstantId<super::GenericFunctionExpr>> {
        let instantiation = reference.clone();
        self.lower_constant(
            crate::plan::ConstantInstantiation::from_function(
                crate::plan::ConstantFunctionInstantiation::Float(instantiation.clone()),
            ),
            |templates| templates.materialize_float_function(&instantiation),
            |expression, context| {
                expression::symbolic_float_function_expr(expression, shape, context)
            },
        )
    }

    fn symbolic_string_function_constant(
        &mut self,
        reference: &crate::plan::ConstantStringFunctionInstantiation,
        shape: &SpecializedFunctionShape,
    ) -> specialization::Representability<super::ConstantId<super::GenericFunctionExpr>> {
        let instantiation = reference.clone();
        self.lower_constant(
            crate::plan::ConstantInstantiation::from_function(
                crate::plan::ConstantFunctionInstantiation::String(instantiation.clone()),
            ),
            |templates| templates.materialize_string_function(&instantiation),
            |expression, context| {
                expression::symbolic_string_function_expr(expression, shape, context)
            },
        )
    }

    fn symbolic_bit_array_function_constant(
        &mut self,
        reference: &crate::plan::ConstantBitArrayFunctionInstantiation,
        shape: &SpecializedFunctionShape,
    ) -> specialization::Representability<super::ConstantId<super::GenericFunctionExpr>> {
        let instantiation = reference.clone();
        self.lower_constant(
            crate::plan::ConstantInstantiation::from_function(
                crate::plan::ConstantFunctionInstantiation::BitArray(instantiation.clone()),
            ),
            |templates| templates.materialize_bit_array_function(&instantiation),
            |expression, context| {
                expression::symbolic_bit_array_function_expr(expression, shape, context)
            },
        )
    }

    fn symbolic_utf_codepoint_function_constant(
        &mut self,
        reference: &crate::plan::ConstantUtfCodepointFunctionInstantiation,
        shape: &SpecializedFunctionShape,
    ) -> specialization::Representability<super::ConstantId<super::GenericFunctionExpr>> {
        let instantiation = reference.clone();
        self.lower_constant(
            crate::plan::ConstantInstantiation::from_function(
                crate::plan::ConstantFunctionInstantiation::UtfCodepoint(instantiation.clone()),
            ),
            |templates| templates.materialize_utf_codepoint_function(&instantiation),
            |expression, context| {
                expression::symbolic_utf_codepoint_function_expr(expression, shape, context)
            },
        )
    }

    fn symbolic_bool_function_constant(
        &mut self,
        reference: &crate::plan::ConstantBoolFunctionInstantiation,
        shape: &SpecializedFunctionShape,
    ) -> specialization::Representability<super::ConstantId<super::GenericFunctionExpr>> {
        let instantiation = reference.clone();
        self.lower_constant(
            crate::plan::ConstantInstantiation::from_function(
                crate::plan::ConstantFunctionInstantiation::Bool(instantiation.clone()),
            ),
            |templates| templates.materialize_bool_function(&instantiation),
            |expression, context| {
                expression::symbolic_bool_function_expr(expression, shape, context)
            },
        )
    }

    fn symbolic_nil_function_constant(
        &mut self,
        reference: &crate::plan::ConstantNilFunctionInstantiation,
        shape: &SpecializedFunctionShape,
    ) -> specialization::Representability<super::ConstantId<super::GenericFunctionExpr>> {
        let instantiation = reference.clone();
        self.lower_constant(
            crate::plan::ConstantInstantiation::from_function(
                crate::plan::ConstantFunctionInstantiation::Nil(instantiation.clone()),
            ),
            |templates| templates.materialize_nil_function(&instantiation),
            |expression, context| {
                expression::symbolic_nil_function_expr(expression, shape, context)
            },
        )
    }

    fn symbolic_tuple_function_constant(
        &mut self,
        reference: &crate::plan::ConstantTupleFunctionInstantiation,
        shape: &SpecializedFunctionShape,
    ) -> specialization::Representability<super::ConstantId<super::GenericFunctionExpr>> {
        let instantiation = reference.clone();
        self.lower_constant(
            crate::plan::ConstantInstantiation::from_function(
                crate::plan::ConstantFunctionInstantiation::Tuple(instantiation.clone()),
            ),
            |templates| templates.materialize_tuple_function(&instantiation),
            |expression, context| {
                expression::symbolic_tuple_function_expr(expression, shape, context)
            },
        )
    }

    fn symbolic_function_function_constant(
        &mut self,
        reference: &crate::plan::ConstantFunctionFunctionInstantiation,
        shape: &SpecializedFunctionShape,
    ) -> specialization::Representability<super::ConstantId<super::GenericFunctionExpr>> {
        let instantiation = reference.clone();
        self.lower_constant(
            crate::plan::ConstantInstantiation::from_function(
                crate::plan::ConstantFunctionInstantiation::Function(instantiation.clone()),
            ),
            |templates| templates.materialize_function_function(&instantiation),
            |expression, context| {
                expression::symbolic_function_function_expr(expression, shape, context)
            },
        )
    }

    fn generic_list_function_constant(
        &mut self,
        reference: &crate::plan::ConstantGenericFunctionInstantiation,
        item_shape: &SpecializedValueShape,
    ) -> specialization::Representability<super::ConstantId<super::ListFunctionExpr>> {
        self.lower_generic_function_constant(reference, |expression, context| {
            expression::generic_list_function_expr(expression, item_shape, context)
        })
    }

    fn generic_function_function_constant(
        &mut self,
        reference: &crate::plan::ConstantGenericFunctionInstantiation,
        return_shape: &SpecializedFunctionShape,
    ) -> specialization::Representability<super::ConstantId<super::FunctionFunctionExpr>> {
        self.lower_generic_function_constant(reference, |expression, context| {
            expression::generic_function_function_expr(expression, return_shape, context)
        })
    }

    fn int_function_constant(
        &mut self,
        reference: &crate::plan::ConstantIntFunctionInstantiation,
    ) -> specialization::Representability<super::ConstantId<super::IntFunctionExpr>> {
        let instantiation = reference.clone();
        self.lower_constant(
            crate::plan::ConstantInstantiation::from_function(
                crate::plan::ConstantFunctionInstantiation::Int(instantiation.clone()),
            ),
            |templates| templates.materialize_int_function(&instantiation),
            expression::int_function_expr,
        )
    }

    fn string_function_constant(
        &mut self,
        reference: &crate::plan::ConstantStringFunctionInstantiation,
    ) -> specialization::Representability<super::ConstantId<super::StringFunctionExpr>> {
        let instantiation = reference.clone();
        self.lower_constant(
            crate::plan::ConstantInstantiation::from_function(
                crate::plan::ConstantFunctionInstantiation::String(instantiation.clone()),
            ),
            |templates| templates.materialize_string_function(&instantiation),
            expression::string_function_expr,
        )
    }

    fn bit_array_function_constant(
        &mut self,
        reference: &crate::plan::ConstantBitArrayFunctionInstantiation,
    ) -> specialization::Representability<super::ConstantId<super::BitArrayFunctionExpr>> {
        let instantiation = reference.clone();
        self.lower_constant(
            crate::plan::ConstantInstantiation::from_function(
                crate::plan::ConstantFunctionInstantiation::BitArray(instantiation.clone()),
            ),
            |templates| templates.materialize_bit_array_function(&instantiation),
            expression::bit_array_function_expr,
        )
    }

    fn utf_codepoint_function_constant(
        &mut self,
        reference: &crate::plan::ConstantUtfCodepointFunctionInstantiation,
    ) -> specialization::Representability<super::ConstantId<super::UtfCodepointFunctionExpr>> {
        let instantiation = reference.clone();
        self.lower_constant(
            crate::plan::ConstantInstantiation::from_function(
                crate::plan::ConstantFunctionInstantiation::UtfCodepoint(instantiation.clone()),
            ),
            |templates| templates.materialize_utf_codepoint_function(&instantiation),
            expression::utf_codepoint_function_expr,
        )
    }

    fn custom_function_constant(
        &mut self,
        reference: &crate::plan::ConstantCustomFunctionInstantiation,
    ) -> specialization::Representability<super::ConstantId<super::CustomFunctionExpr>> {
        let instantiation = reference.clone();
        self.lower_constant(
            crate::plan::ConstantInstantiation::from_function(
                crate::plan::ConstantFunctionInstantiation::Custom(instantiation.clone()),
            ),
            |templates| templates.materialize_custom_function(&instantiation),
            expression::custom_function_expr,
        )
    }

    fn float_function_constant(
        &mut self,
        reference: &crate::plan::ConstantFloatFunctionInstantiation,
    ) -> specialization::Representability<super::ConstantId<super::FloatFunctionExpr>> {
        let instantiation = reference.clone();
        self.lower_constant(
            crate::plan::ConstantInstantiation::from_function(
                crate::plan::ConstantFunctionInstantiation::Float(instantiation.clone()),
            ),
            |templates| templates.materialize_float_function(&instantiation),
            expression::float_function_expr,
        )
    }

    fn bool_function_constant(
        &mut self,
        reference: &crate::plan::ConstantBoolFunctionInstantiation,
    ) -> specialization::Representability<super::ConstantId<super::BoolFunctionExpr>> {
        let instantiation = reference.clone();
        self.lower_constant(
            crate::plan::ConstantInstantiation::from_function(
                crate::plan::ConstantFunctionInstantiation::Bool(instantiation.clone()),
            ),
            |templates| templates.materialize_bool_function(&instantiation),
            expression::bool_function_expr,
        )
    }

    fn nil_function_constant(
        &mut self,
        reference: &crate::plan::ConstantNilFunctionInstantiation,
    ) -> specialization::Representability<super::ConstantId<super::NilFunctionExpr>> {
        let instantiation = reference.clone();
        self.lower_constant(
            crate::plan::ConstantInstantiation::from_function(
                crate::plan::ConstantFunctionInstantiation::Nil(instantiation.clone()),
            ),
            |templates| templates.materialize_nil_function(&instantiation),
            expression::nil_function_expr,
        )
    }

    fn tuple_function_constant(
        &mut self,
        reference: &crate::plan::ConstantTupleFunctionInstantiation,
    ) -> specialization::Representability<super::ConstantId<super::TupleFunctionExpr>> {
        let instantiation = reference.clone();
        self.lower_constant(
            crate::plan::ConstantInstantiation::from_function(
                crate::plan::ConstantFunctionInstantiation::Tuple(instantiation.clone()),
            ),
            |templates| templates.materialize_tuple_function(&instantiation),
            expression::tuple_function_expr,
        )
    }

    fn list_function_constant(
        &mut self,
        reference: &crate::plan::ConstantListFunctionInstantiation,
    ) -> specialization::Representability<super::ConstantId<super::ListFunctionExpr>> {
        let instantiation = reference.clone();
        self.lower_constant(
            crate::plan::ConstantInstantiation::from_function(
                crate::plan::ConstantFunctionInstantiation::List(instantiation.clone()),
            ),
            |templates| templates.materialize_list_function(&instantiation),
            expression::list_function_expr,
        )
    }

    fn function_function_constant(
        &mut self,
        reference: &crate::plan::ConstantFunctionFunctionInstantiation,
    ) -> specialization::Representability<super::ConstantId<super::FunctionFunctionExpr>> {
        let instantiation = reference.clone();
        self.lower_constant(
            crate::plan::ConstantInstantiation::from_function(
                crate::plan::ConstantFunctionInstantiation::Function(instantiation.clone()),
            ),
            |templates| templates.materialize_function_function(&instantiation),
            expression::function_function_expr,
        )
    }

    fn generic_local_index(
        &self,
        local: crate::plan::GenericLocalId,
    ) -> specialization::Representability<usize> {
        specialization::Representability::Inhabited(
            self.local_index(frame::LocalKey::new(frame::LocalKind::Generic, local.0)),
        )
    }

    fn generic_list_local_index(&self, local: crate::plan::GenericListLocalId) -> usize {
        self.local_index(frame::LocalKey::new(frame::LocalKind::GenericList, local.0))
    }

    fn generic_function_local_index(&self, local: crate::plan::GenericFunctionLocalId) -> usize {
        self.local_index(frame::LocalKey::new(
            frame::LocalKind::GenericFunction,
            local.0,
        ))
    }

    fn local_index(&self, key: frame::LocalKey) -> usize {
        self.specialization_locals[&self.current_specialization].index(key)
    }

    fn mapped_local(&self, kind: frame::LocalKind, index: usize) -> usize {
        self.local_index(frame::LocalKey::new(kind, index))
    }

    fn stored_symbolic_target_local(
        &mut self,
        instantiation: &crate::plan::FunctionInstantiation,
        key: frame::LocalKey,
    ) -> StoredTargetLocal {
        let (specialization, _) =
            SpecializationKey::from_instantiation(instantiation, &self.substitution);
        self.reserve_locals(&specialization);
        let (index, shape) = self.specialization_locals[&specialization].stored_allocation(key);
        StoredTargetLocal {
            index,
            shape,
            substitution: specialization.substitution().clone(),
        }
    }

    fn current_stored_target(
        &self,
        index: usize,
        shape: specialization::StoredValueShape,
    ) -> StoredTargetLocal {
        StoredTargetLocal {
            index,
            shape,
            substitution: self.substitution.clone(),
        }
    }

    fn current_local_shapes(&self) -> Vec<specialization::StoredValueShape> {
        self.specialization_locals[&self.current_specialization]
            .stored_shapes()
            .to_vec()
    }

    fn concrete_parameter(&self, parameter: crate::plan::TypeParameterId) -> SpecializedValueShape {
        SpecializedValueShape::instantiate(&ValueShape::Parameter(parameter), &self.substitution)
    }

    fn value_type(&mut self, type_: crate::plan::ValueType) -> super::ValueType {
        let shape = SpecializedValueShape::instantiate(
            &ValueShape::from_value_type(type_),
            &self.substitution,
        );
        self.types.value_type(&shape)
    }

    fn custom_value_shape(
        &mut self,
        shape: crate::plan::CustomValueShape,
    ) -> super::CustomValueShape {
        let shape = SpecializedCustomValueShape::instantiate(&shape, &self.substitution);
        self.types.custom_value_shape(&shape)
    }

    fn value_shape(&mut self, shape: crate::plan::ValueShape) -> super::ValueShapeId {
        self.types.value_shape(&SpecializedValueShape::instantiate(
            &shape,
            &self.substitution,
        ))
    }

    fn function_shape(&mut self, shape: crate::plan::FunctionShape) -> super::FunctionShape {
        self.types
            .function_shape(&SpecializedFunctionShape::instantiate(
                &shape,
                &self.substitution,
            ))
    }

    fn function_type(&mut self, type_: crate::plan::FunctionType) -> super::FunctionType {
        let shape = crate::plan::FunctionShape::from_function_type(type_);
        let shape = SpecializedFunctionShape::instantiate(&shape, &self.substitution);
        self.types.function_type(&shape)
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

    fn function_function_type(
        &mut self,
        type_: crate::plan::FunctionFunctionType,
    ) -> super::FunctionFunctionType {
        let substitution = self.substitution.clone();
        self.function_function_type_with_substitution(&type_, &substitution)
    }

    fn function_function_type_with_substitution(
        &mut self,
        type_: &crate::plan::FunctionFunctionType,
        substitution: &SpecializedTypeSubstitution,
    ) -> super::FunctionFunctionType {
        let arguments = type_
            .argument_shapes()
            .iter()
            .map(|shape| SpecializedValueShape::instantiate(shape, substitution))
            .collect::<Vec<_>>();
        let return_ = SpecializedFunctionShape::instantiate(type_.return_shape(), substitution);
        self.types.function_function_type(&arguments, &return_)
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

    fn list_list_type(&mut self, item: crate::plan::ValueType) -> value_type::NestedListTypeId {
        self.types
            .list_list_type(&SpecializedValueShape::instantiate(
                &ValueShape::from_value_type(item),
                &self.substitution,
            ))
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
        self.reserve_locals(&key);
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

    fn reserve_locals(&mut self, key: &SpecializationKey) {
        if !self.specialization_locals.contains_key(key) {
            let locals = self.frame_templates[&key.template()]
                .specialize(key.substitution(), &self.representations);
            self.specialization_locals.insert(key.clone(), locals);
        }
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

    fn stored_value_shape(
        &self,
        shape: &crate::plan::ValueStorageShape,
    ) -> specialization::StoredValueShape {
        specialization::StoredValueShape::instantiate(shape, &self.substitution)
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
        self.current_specialization = key.clone();
        self.pending_return_divergence = None;
    }

    fn set_return_divergence(&mut self, expression: super::NeverExpr) {
        self.pending_return_divergence = Some(expression);
    }

    fn take_return_divergence(&mut self) -> Option<super::NeverExpr> {
        self.pending_return_divergence.take()
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
            main_key.clone(),
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
mod tests {
    use super::super::{ExecutionPlan, IntFunctionId, RuntimeFunctionId};
    use super::specialization::{Representability, RepresentationContext, SpecializationKey};
    use super::{
        FixedPointStep, FunctionTemplates, LoweringContext, SpecializationOutcome,
        resolve_specialization_fixed_point,
    };
    use crate::plan::{SourceContext, TypeParameterId};
    use crate::{ListValue, Value, ValueType};
    use num_bigint::BigInt;
    use std::collections::{HashSet, VecDeque};

    fn lowering_context(custom_types: Vec<crate::plan::CustomTypeDefinition>) -> LoweringContext {
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
            Vec::new(),
            crate::plan::ReturnExpr::int(
                crate::plan::IntFunctionId(0),
                crate::plan::IntExpr::value(0.into()),
            ),
        );
        let templates = FunctionTemplates::new(main, vec![capture_target], Vec::new());

        LoweringContext::new(
            &templates,
            SpecializationKey::monomorphic(main_id),
            RepresentationContext::new(custom_types),
            crate::plan::ConstantTemplates::from_entries(Vec::new()),
            HashSet::new(),
        )
    }

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

        let mut context = lowering_context(Vec::new());
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

    #[test]
    fn provisional_uninhabited_expression_owners_erase_unstorable_values() {
        let parameter = TypeParameterId(0);
        let custom_name =
            crate::plan::CustomTypeName::new("geam".into(), "main".into(), "Boxed".into());
        let custom_definition = crate::plan::CustomTypeDefinition::new(
            custom_name.clone(),
            crate::plan::CustomTypePublicity::Private,
            false,
            vec![crate::plan::CustomTypeParameterId(0)],
            vec![crate::plan::CustomConstructorDefinition::new(
                "Boxed".into(),
                0,
                vec![crate::plan::CustomFieldDefinition::new(
                    None,
                    crate::plan::CustomTypeTemplate::Parameter(crate::plan::CustomTypeParameterId(
                        0,
                    )),
                )],
            )],
        );
        let mut context = lowering_context(vec![custom_definition]);
        let local = crate::plan::GenericExpr::local_get(
            crate::plan::GenericLocal::new(crate::plan::GenericLocalId(0), parameter),
            "value".into(),
        );
        let empty_list =
            crate::plan::ListExpr::try_value(Vec::new(), ValueType::Parameter(parameter))
                .expect("an empty generic list should preserve its item parameter")
                .into_generic()
                .expect("a parameter item type should create a generic list expression");
        let list_index = crate::plan::GenericExpr::list_index(empty_list, 0);

        let diverging_local =
            crate::plan::GenericLocal::new(crate::plan::GenericLocalId(1), parameter);
        let diverging_argument = crate::plan::CallArg::parametric(
            crate::plan::ParamSlot::from_local(crate::plan::ParamLocal::generic(diverging_local)),
            crate::plan::Expr::generic(crate::plan::GenericExpr::panic(
                parameter,
                crate::plan::PanicExpr::panic_at(None, crate::plan::PanicSite::unknown()),
            )),
        );
        let diverging_call = crate::plan::GenericExpr::call(
            parameter,
            crate::plan::monomorphic_function_instantiation(
                1,
                crate::plan::FunctionShape::new(
                    vec![crate::plan::ValueShape::Parameter(parameter)],
                    crate::plan::ValueShape::Parameter(parameter),
                ),
            ),
            vec![diverging_argument],
        );
        let erased_argument = crate::plan::CallArg::parametric(
            crate::plan::ParamSlot::from_local(crate::plan::ParamLocal::generic(diverging_local)),
            crate::plan::Expr::generic(local.clone()),
        );
        let erased_direct_call = crate::plan::GenericExpr::call(
            parameter,
            crate::plan::monomorphic_function_instantiation(
                1,
                crate::plan::FunctionShape::new(
                    vec![crate::plan::ValueShape::Parameter(parameter)],
                    crate::plan::ValueShape::Parameter(parameter),
                ),
            ),
            vec![erased_argument.clone()],
        );
        let erased_function_call = crate::plan::GenericExpr::function_call(
            crate::plan::GenericFunctionExpr::panic(
                crate::plan::PanicExpr::panic_at(None, crate::plan::PanicSite::unknown()),
                crate::plan::GenericFunctionType::new(
                    vec![crate::plan::ValueShape::Parameter(parameter)],
                    parameter,
                ),
            ),
            vec![erased_argument],
        );
        let parameter_list_function_call = crate::plan::GenericExpr::function_call(
            crate::plan::GenericFunctionExpr::panic(
                crate::plan::PanicExpr::panic_at(None, crate::plan::PanicSite::unknown()),
                crate::plan::GenericFunctionType::new(
                    vec![crate::plan::ValueShape::Parameter(parameter)],
                    parameter,
                ),
            ),
            vec![crate::plan::CallArg::parametric(
                crate::plan::ParamSlot::from_local(crate::plan::ParamLocal::generic(
                    crate::plan::GenericLocal::new(crate::plan::GenericLocalId(2), parameter),
                )),
                crate::plan::Expr::generic(crate::plan::GenericExpr::panic(
                    parameter,
                    crate::plan::PanicExpr::panic_at(None, crate::plan::PanicSite::unknown()),
                )),
            )],
        );
        let parameter_list_call = crate::plan::ListExpr::function_call(
            crate::plan::ListFunctionExpr::panic(
                crate::plan::PanicExpr::panic_at(None, crate::plan::PanicSite::unknown()),
                crate::plan::FunctionType::new(
                    vec![ValueType::Parameter(parameter)],
                    ValueType::List(Box::new(ValueType::Parameter(parameter))),
                ),
                ValueType::Parameter(parameter),
            ),
            vec![crate::plan::CallArg::parametric(
                crate::plan::ParamSlot::from_local(crate::plan::ParamLocal::generic(
                    crate::plan::GenericLocal::new(crate::plan::GenericLocalId(3), parameter),
                )),
                crate::plan::Expr::generic(crate::plan::GenericExpr::panic(
                    parameter,
                    crate::plan::PanicExpr::panic_at(None, crate::plan::PanicSite::unknown()),
                )),
            )],
        )
        .into_generic()
        .expect("a parameter item type should create a generic-list expression");
        let diverging_step = crate::plan::Step::evaluate(crate::plan::Expr::generic(
            crate::plan::GenericExpr::panic(
                parameter,
                crate::plan::PanicExpr::panic_at(None, crate::plan::PanicSite::unknown()),
            ),
        ));
        let diverging_block =
            crate::plan::GenericExpr::block(vec![diverging_step.clone()], local.clone());

        assert_eq!(
            super::expression::never_expr(&local, &mut context).map(|_| ()),
            Representability::Uninhabited,
        );
        assert_eq!(
            super::expression::never_expr(&list_index, &mut context).map(|_| ()),
            Representability::Uninhabited,
        );

        let tuple_projection = crate::plan::GenericExpr::tuple_index(
            parameter,
            crate::plan::TupleExpr::local_get(
                crate::plan::TupleLocalId(0),
                "tuple".into(),
                vec![ValueType::Parameter(parameter)],
            ),
            0,
        );
        assert_eq!(
            super::expression::never_expr(&tuple_projection, &mut context).map(|_| ()),
            Representability::Uninhabited,
        );

        let custom_shape = crate::plan::CustomValueShape::new(
            custom_name,
            vec![crate::plan::ValueShape::Parameter(parameter)],
            crate::plan::CustomConstructorRefinement::Exact(0),
        );
        let custom_source = crate::plan::CustomExpr::local_get(
            crate::plan::CustomLocal::from_shape(crate::plan::CustomLocalId(0), custom_shape),
            "boxed".into(),
        );
        let custom_projection = crate::plan::GenericExpr::custom_field(
            parameter,
            crate::plan::CustomFieldAccess::new(custom_source, 0, None),
        );
        assert_eq!(
            super::expression::never_expr(&custom_projection, &mut context).map(|_| ()),
            Representability::Uninhabited,
        );
        assert_eq!(
            super::expression::never_expr(&diverging_call, &mut context).map(|_| ()),
            Representability::Inhabited(()),
        );
        assert_eq!(
            super::expression::never_expr(&erased_direct_call, &mut context).map(|_| ()),
            Representability::Uninhabited,
        );
        assert_eq!(
            super::expression::never_expr(&erased_function_call, &mut context).map(|_| ()),
            Representability::Uninhabited,
        );
        assert_eq!(
            super::expression::parameter_list_value_expr(
                &parameter_list_function_call,
                parameter,
                &mut context,
            )
            .map(|_| ()),
            Representability::Inhabited(()),
        );
        assert_eq!(
            super::expression::parameter_list_expr(&parameter_list_call, parameter, &mut context)
                .map(|_| ()),
            Representability::Inhabited(()),
        );
        assert_eq!(
            super::expression::never_expr(&diverging_block, &mut context).map(|_| ()),
            Representability::Inhabited(()),
        );

        let tuple_local = crate::plan::TupleExpr::local_get(
            crate::plan::TupleLocalId(0),
            "tuple".into(),
            vec![ValueType::Parameter(parameter)],
        );
        let lowered =
            super::expression::tuple_inhabitation(&tuple_local, &context).and_then(|proof| {
                super::expression::tuple_never_expr(&tuple_local, &proof, &mut context)
            });
        assert_eq!(lowered.map(|_| ()), Representability::Uninhabited);

        let tuple_block = crate::plan::TupleExpr::block(vec![diverging_step], tuple_local);
        let lowered =
            super::expression::tuple_inhabitation(&tuple_block, &context).and_then(|proof| {
                super::expression::tuple_never_expr(&tuple_block, &proof, &mut context)
            });
        assert_eq!(lowered.map(|_| ()), Representability::Inhabited(()));

        let tuple_proof_owner = crate::plan::TupleExpr::local_get(
            crate::plan::TupleLocalId(1),
            "proof".into(),
            vec![ValueType::Parameter(parameter)],
        );
        let inhabited_tuple = crate::plan::TupleExpr::local_get(
            crate::plan::TupleLocalId(2),
            "inhabited".into(),
            vec![ValueType::Int],
        );
        assert_eq!(
            super::expression::tuple_inhabitation(&inhabited_tuple, &context).map(|_| ()),
            Representability::Uninhabited,
        );

        let tuple_list = crate::plan::ListExpr::panic(
            crate::plan::PanicExpr::panic_at(None, crate::plan::PanicSite::unknown()),
            ValueType::Tuple(vec![ValueType::Parameter(parameter)]),
        )
        .into_tuple()
        .expect("a tuple item type should create a tuple-list expression");
        let tuple_list_index = crate::plan::TupleExpr::list_index(
            tuple_list,
            0,
            vec![ValueType::Parameter(parameter)],
        );
        let lowered =
            super::expression::tuple_inhabitation(&tuple_proof_owner, &context).and_then(|proof| {
                super::expression::tuple_never_expr(&tuple_list_index, &proof, &mut context)
            });
        assert_eq!(lowered.map(|_| ()), Representability::Uninhabited);

        let inhabited_at_divergence = crate::plan::TupleExpr::value(
            vec![crate::plan::Expr::int(crate::plan::IntExpr::value(
                1.into(),
            ))],
            vec![ValueType::Parameter(parameter)],
        );
        let lowered =
            super::expression::tuple_inhabitation(&tuple_proof_owner, &context).and_then(|proof| {
                super::expression::tuple_never_expr(&inhabited_at_divergence, &proof, &mut context)
            });
        assert_eq!(lowered.map(|_| ()), Representability::Uninhabited);

        let prefix_proof_owner = crate::plan::TupleExpr::local_get(
            crate::plan::TupleLocalId(3),
            "prefix_proof".into(),
            vec![ValueType::Int, ValueType::Parameter(parameter)],
        );
        let erased_prefix = crate::plan::TupleExpr::value(
            vec![
                crate::plan::Expr::generic(local.clone()),
                crate::plan::Expr::generic(local.clone()),
            ],
            vec![ValueType::Int, ValueType::Parameter(parameter)],
        );
        let lowered = super::expression::tuple_inhabitation(&prefix_proof_owner, &context)
            .and_then(|proof| {
                super::expression::tuple_never_expr(&erased_prefix, &proof, &mut context)
            });
        assert_eq!(lowered.map(|_| ()), Representability::Uninhabited);

        let missing_divergence =
            crate::plan::TupleExpr::value(Vec::new(), vec![ValueType::Parameter(parameter)]);
        let lowered =
            super::expression::tuple_inhabitation(&tuple_proof_owner, &context).and_then(|proof| {
                super::expression::tuple_never_expr(&missing_divergence, &proof, &mut context)
            });
        assert_eq!(lowered.map(|_| ()), Representability::Uninhabited);

        let capture_target = crate::plan::FunctionTemplateSignature::new(
            crate::plan::FunctionTemplateId::new(1),
            crate::plan::TypeScheme::new(1),
            crate::plan::FunctionShape::new(
                vec![crate::plan::ValueShape::Parameter(parameter)],
                crate::plan::ValueShape::Int,
            ),
        )
        .try_instantiate(vec![crate::plan::ValueShape::Int])
        .expect("the capture target should accept one concrete type argument");
        let erased_capture_source = crate::plan::GenericExpr::call(
            parameter,
            crate::plan::monomorphic_function_instantiation(
                2,
                crate::plan::FunctionShape::new(
                    vec![crate::plan::ValueShape::Parameter(parameter)],
                    crate::plan::ValueShape::Parameter(parameter),
                ),
            ),
            vec![crate::plan::CallArg::parametric(
                crate::plan::ParamSlot::from_local(crate::plan::ParamLocal::generic(
                    crate::plan::GenericLocal::new(crate::plan::GenericLocalId(1), parameter),
                )),
                crate::plan::Expr::generic(local),
            )],
        );
        assert_eq!(
            super::expression::capture_args(
                &capture_target,
                &[crate::plan::CaptureArg::generic(
                    crate::plan::GenericLocal::new(crate::plan::GenericLocalId(0), parameter),
                    erased_capture_source,
                )],
                &mut context,
            )
            .map(|_| ()),
            Representability::Uninhabited,
        );
    }

    #[test]
    fn provisional_never_constructor_functions_are_erased_before_table_insertion() {
        let parameter = TypeParameterId(0);
        let name = crate::plan::CustomTypeName::new("geam".into(), "main".into(), "Boxed".into());
        let definition = crate::plan::CustomTypeDefinition::new(
            name.clone(),
            crate::plan::CustomTypePublicity::Private,
            false,
            vec![crate::plan::CustomTypeParameterId(0)],
            vec![crate::plan::CustomConstructorDefinition::new(
                "Boxed".into(),
                0,
                vec![crate::plan::CustomFieldDefinition::new(
                    None,
                    crate::plan::CustomTypeTemplate::Parameter(crate::plan::CustomTypeParameterId(
                        0,
                    )),
                )],
            )],
        );
        let constructor = crate::plan::CustomConstructor::new(
            crate::plan::CustomType::new(name.clone(), vec![ValueType::Parameter(parameter)]),
            "Boxed".into(),
            0,
            vec![crate::plan::CustomConstructorField::new(
                None,
                ValueType::Parameter(parameter),
            )],
        );
        let expression = crate::plan::CustomFunctionExpr::constructor(constructor);
        let mut context = lowering_context(vec![definition]);

        assert_eq!(
            super::expression::custom_never_function_expr(&expression, &mut context).map(|_| ()),
            Representability::Uninhabited,
        );

        let shape =
            context.concrete_function_shape(&crate::plan::FunctionShape::from_function_type(
                expression.custom_function_type().to_function_type(),
            ));
        let type_ = context.generic_function_type(&shape);
        assert_eq!(
            super::expression::custom_never_function_expr_kind(
                expression.kind(),
                &type_,
                &mut context,
            )
            .map(|_| ()),
            Representability::Uninhabited,
        );

        let custom_shape = expression.custom_function_type().return_().clone();
        let custom_local = crate::plan::CustomExpr::local_get(
            crate::plan::CustomLocal::from_shape(crate::plan::CustomLocalId(0), custom_shape),
            "custom".into(),
        );
        let lowered =
            super::expression::custom_inhabitation(&custom_local, &context).and_then(|proof| {
                super::expression::custom_never_expr(&custom_local, &proof, &mut context)
            });
        assert_eq!(lowered.map(|_| ()), Representability::Uninhabited);

        let inhabited_custom = crate::plan::CustomExpr::local_get(
            crate::plan::CustomLocal::from_shape(
                crate::plan::CustomLocalId(1),
                crate::plan::CustomValueShape::any(crate::plan::CustomType::new(
                    name,
                    vec![ValueType::Int],
                )),
            ),
            "inhabited_custom".into(),
        );
        assert_eq!(
            super::expression::custom_inhabitation(&inhabited_custom, &context).map(|_| ()),
            Representability::Uninhabited,
        );

        let custom_list = crate::plan::ListExpr::panic(
            crate::plan::PanicExpr::panic_at(None, crate::plan::PanicSite::unknown()),
            ValueType::Custom(custom_local.shape().type_().clone()),
        )
        .into_custom()
        .expect("a custom item type should create a custom-list expression");
        let custom_list_index =
            crate::plan::CustomExpr::list_index_shape(custom_list, 0, custom_local.shape().clone());
        let lowered =
            super::expression::custom_inhabitation(&custom_local, &context).and_then(|proof| {
                super::expression::custom_never_expr(&custom_list_index, &proof, &mut context)
            });
        assert_eq!(lowered.map(|_| ()), Representability::Uninhabited);

        let custom_block = crate::plan::CustomExpr::block(
            vec![crate::plan::Step::evaluate(crate::plan::Expr::generic(
                crate::plan::GenericExpr::panic(
                    parameter,
                    crate::plan::PanicExpr::panic_at(None, crate::plan::PanicSite::unknown()),
                ),
            ))],
            custom_local,
        );
        let lowered =
            super::expression::custom_inhabitation(&custom_block, &context).and_then(|proof| {
                super::expression::custom_never_expr(&custom_block, &proof, &mut context)
            });
        assert_eq!(lowered.map(|_| ()), Representability::Inhabited(()));

        let panic = || crate::plan::PanicExpr::panic_at(None, crate::plan::PanicSite::unknown());

        let returned_shape = crate::plan::FunctionShape::new(
            vec![crate::plan::ValueShape::Int],
            crate::plan::ValueShape::Custom(expression.custom_function_type().return_().clone()),
        );
        let function_function = crate::plan::FunctionFunctionExpr::panic(
            panic(),
            crate::plan::FunctionFunctionType::from_shapes(
                vec![crate::plan::ValueShape::Parameter(parameter)],
                returned_shape,
            ),
        );
        let argument_local =
            crate::plan::GenericLocal::new(crate::plan::GenericLocalId(0), parameter);
        let argument = crate::plan::CallArg::parametric(
            crate::plan::ParamSlot::from_local(crate::plan::ParamLocal::generic(argument_local)),
            crate::plan::Expr::generic(crate::plan::GenericExpr::panic(parameter, panic())),
        );
        let function_call =
            crate::plan::CustomFunctionExpr::try_function_call(function_function, vec![argument])
                .expect("the function-function argument count should match");
        assert_eq!(
            super::expression::custom_never_function_expr(&function_call, &mut context).map(|_| ()),
            Representability::Inhabited(()),
        );

        let tuple_projection = crate::plan::CustomFunctionExpr::tuple_index(
            crate::plan::TupleExpr::panic(
                panic(),
                vec![ValueType::Function(Box::new(
                    expression.custom_function_type().to_function_type(),
                ))],
            ),
            0,
            expression.custom_function_type().clone(),
        );
        assert_eq!(
            super::expression::custom_never_function_expr(&tuple_projection, &mut context)
                .map(|_| ()),
            Representability::Inhabited(()),
        );

        let function_list = crate::plan::ListExpr::panic(
            panic(),
            ValueType::Function(Box::new(
                expression.custom_function_type().to_function_type(),
            )),
        )
        .into_function()
        .expect("a function item type should create a function-list expression");
        let list_projection = crate::plan::CustomFunctionExpr::list_index(
            function_list,
            0,
            expression.custom_function_type().clone(),
        );
        assert_eq!(
            super::expression::custom_never_function_expr_kind(
                list_projection.kind(),
                &type_,
                &mut context,
            )
            .map(|_| ()),
            Representability::Inhabited(()),
        );
    }

    #[test]
    fn lowering_preserves_module_source_context_and_main_runtime() {
        let source = "pub fn main() { 1 }";
        let typed = crate::compile_typed_module("main", "src/main.gleam", source)
            .expect("source should compile");
        let module_plan =
            crate::plan_module_with_source(typed, SourceContext::new("src/main.gleam", source))
                .expect("source should plan");
        let plan = ExecutionPlan::from_module_plan(module_plan);

        assert_eq!(plan.module().as_str(), "main");
        let source_context = plan.source_context().expect("source should be preserved");
        assert_eq!(source_context.path(), "src/main.gleam");
        assert_eq!(source_context.source(), source);
        assert_eq!(
            plan.main_runtime(),
            RuntimeFunctionId::Int(IntFunctionId(0))
        );
    }

    #[test]
    fn lowering_reserves_locals_for_zero_argument_generic_specializations() {
        let source = r#"
type Box(value) {
  Box
}

fn make() -> Box(value) {
  Box
}

pub fn main() {
  let make_int: fn() -> Box(Int) = make
  case make_int() {
    Box -> 1
  }
}
"#;
        let typed = crate::compile_typed_module("main", "src/main.gleam", source)
            .expect("source should compile");
        let module_plan = crate::plan_module(typed).expect("source should plan");
        let plan = ExecutionPlan::from_module_plan(module_plan);

        assert_eq!(
            crate::run_main(&plan).expect("source should execute"),
            Value::Int(BigInt::from(1)),
        );
    }

    #[test]
    fn lowering_assigns_generic_specializations_by_first_use_and_deduplicates_them() {
        let source = r#"
fn first(value: value) -> value {
  let first_marker = "first"
  value
}

fn second(value: value) -> value {
  let second_marker = 2
  value
}

pub fn main() {
  #(second(1), first(2), second(3), first("four"))
}
"#;
        let typed = crate::compile_typed_module("main", "src/main.gleam", source)
            .expect("source should compile");
        let module_plan = crate::plan_module(typed).expect("source should plan");
        let plan = ExecutionPlan::from_module_plan(module_plan);

        assert_eq!(plan.functions.int_functions.len(), 2);
        assert_eq!(plan.functions.string_functions.len(), 1);
        assert_eq!(plan.functions.tuple_functions.len(), 1);
        assert_eq!(plan.int_function(IntFunctionId(0)).frame_layout().ints(), 2);
        assert_eq!(
            plan.int_function(IntFunctionId(0)).frame_layout().strings(),
            0,
        );
        assert_eq!(plan.int_function(IntFunctionId(1)).frame_layout().ints(), 1);
        assert_eq!(
            plan.int_function(IntFunctionId(1)).frame_layout().strings(),
            1,
        );
        assert_eq!(
            crate::run_main(&plan).expect("source should execute"),
            Value::Tuple(vec![
                Value::Int(BigInt::from(1)),
                Value::Int(BigInt::from(2)),
                Value::Int(BigInt::from(3)),
                Value::String("four".into()),
            ]),
        );
    }

    #[test]
    fn lowering_omits_unreachable_functions_and_their_constants() {
        let source = r#"
const failing = <<<<1>>:bits-size(16)>>

fn unused() {
  failing
}

pub fn main() {
  1
}
"#;
        let typed = crate::compile_typed_module("main", "src/main.gleam", source)
            .expect("source should compile");
        let module_plan = crate::plan_module(typed).expect("source should plan");
        let plan = ExecutionPlan::from_module_plan(module_plan);

        assert_eq!(plan.constant_count(), 0);
        assert_eq!(plan.functions.int_functions.len(), 1);
        assert_eq!(plan.functions.bit_array_functions.len(), 0);
    }

    #[test]
    fn lowering_executes_unresolved_list_calls_through_typed_tables() {
        let source = r#"
fn empty() -> List(value) { [] }
fn nested() -> List(List(value)) { [[]] }

pub fn main() {
  let first = empty()
  let second = nested()
  #(first, second)
}
"#;
        let typed = crate::compile_typed_module("main", "src/main.gleam", source)
            .expect("source should compile");
        let module_plan = crate::plan_module(typed).expect("source should plan");
        let plan = ExecutionPlan::from_module_plan(module_plan);

        assert_eq!(plan.functions.parameter_list_functions.len(), 1);
        assert_eq!(plan.functions.parameter_list_list_functions.len(), 1);
        assert_eq!(plan.functions.list_list_functions.len(), 0);

        let first_type = ValueType::Parameter(TypeParameterId(0));
        let second_type = ValueType::Parameter(TypeParameterId(1));
        assert_eq!(
            crate::run_main(&plan).expect("source should execute"),
            Value::Tuple(vec![
                Value::List(ListValue::empty(first_type)),
                Value::List(ListValue::from_evaluated_list(
                    second_type.clone(),
                    vec![ListValue::empty(second_type)],
                )),
            ]),
        );
    }

    #[test]
    fn lowering_executes_recursive_never_function_handoffs() {
        let source = include_str!(
            "../../../tests/fixtures/execution/functions/generic_recursive_never_function_handoffs.gleam"
        );
        let typed = crate::compile_typed_module("main", "src/main.gleam", source)
            .expect("source should compile");
        let module_plan = crate::plan_module(typed).expect("source should plan");
        let plan = ExecutionPlan::from_module_plan(module_plan);

        assert_eq!(
            crate::run_main(&plan).expect("source should execute"),
            Value::Tuple(vec![Value::Bool(true); 62]),
        );
    }

    #[test]
    fn lowering_executes_recursive_never_value_handoffs() {
        let source = include_str!(
            "../../../tests/fixtures/execution/functions/generic_recursive_never_value_handoffs.gleam"
        );
        let typed = crate::compile_typed_module("main", "src/main.gleam", source)
            .expect("source should compile");
        let module_plan = crate::plan_module(typed).expect("source should plan");
        let plan = ExecutionPlan::from_module_plan(module_plan);

        assert_eq!(
            crate::run_main(&plan).expect("source should execute"),
            Value::Tuple(vec![Value::Bool(true); 28]),
        );
    }

    #[test]
    fn lowering_executes_symbolic_function_handoffs_across_return_families() {
        let source = include_str!(
            "../../../tests/fixtures/execution/functions/generic_symbolic_handoffs.gleam"
        );
        let typed = crate::compile_typed_module("main", "src/main.gleam", source)
            .expect("source should compile");
        let module_plan = crate::plan_module(typed).expect("source should plan");
        let plan = ExecutionPlan::from_module_plan(module_plan);

        assert_eq!(
            crate::run_main(&plan).expect("source should execute"),
            Value::Tuple(vec![Value::Bool(true); 48]),
        );
    }

    #[test]
    fn lowering_executes_never_function_handoffs_across_expression_owners() {
        let source = include_str!(
            "../../../tests/fixtures/execution/functions/generic_never_function_handoffs.gleam"
        );
        let typed = crate::compile_typed_module("main", "src/main.gleam", source)
            .expect("source should compile");
        let module_plan = crate::plan_module(typed).expect("source should plan");
        let plan = ExecutionPlan::from_module_plan(module_plan);

        assert_eq!(
            crate::run_main(&plan).expect("source should execute"),
            Value::Tuple(vec![Value::Bool(true); 38]),
        );
    }

    #[test]
    fn lowering_specializes_generic_panic_branches_across_primitive_families() {
        let source = include_str!(
            "../../../tests/fixtures/execution_errors/functions/generic_symbolic_function_panic.gleam"
        );
        let typed = crate::compile_typed_module("main", "src/main.gleam", source)
            .expect("source should compile");
        let module_plan = crate::plan_module(typed).expect("source should plan");
        let plan = ExecutionPlan::from_module_plan(module_plan);

        assert_eq!(plan.functions.int_functions.len(), 2);
        assert_eq!(plan.functions.float_functions.len(), 2);
        assert_eq!(plan.functions.string_functions.len(), 2);
        assert_eq!(plan.functions.bit_array_functions.len(), 2);
        assert_eq!(plan.functions.utf_codepoint_functions.len(), 3);
        assert_eq!(plan.functions.bool_functions.len(), 2);
        assert_eq!(plan.functions.nil_functions.len(), 2);
        assert_eq!(plan.functions.int_function_functions.len(), 1);
        assert_eq!(plan.functions.float_function_functions.len(), 1);
        assert_eq!(plan.functions.string_function_functions.len(), 1);
        assert_eq!(plan.functions.bit_array_function_functions.len(), 1);
        assert_eq!(plan.functions.utf_codepoint_function_functions.len(), 1);
        assert_eq!(plan.functions.bool_function_functions.len(), 1);
        assert_eq!(plan.functions.nil_function_functions.len(), 1);
    }
}
