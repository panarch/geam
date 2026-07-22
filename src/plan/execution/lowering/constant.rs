use super::specialization::{
    SpecializedCustomValueShape, SpecializedFunctionShape, SpecializedValueShape,
};
use super::{LoweringContext, graph, specialization};
use crate::plan::execution::{self, ConstantId, ConstantProgram, ConstantTable, ConstantValue};
use crate::plan::module::ConstantInstantiation;
use std::collections::HashMap;

#[derive(Default)]
pub(super) struct ConstantLowering {
    indices: HashMap<ConstantInstantiation, usize>,
    table: ConstantTable,
}

impl ConstantLowering {
    fn get<Value>(&self, key: &ConstantInstantiation) -> Option<ConstantId<Value>> {
        self.indices.get(key).copied().map(ConstantId::new)
    }

    fn insert<Value: ConstantValue>(
        &mut self,
        key: ConstantInstantiation,
        program: ConstantProgram<Value>,
    ) -> ConstantId<Value> {
        let id = self.table.push(program);
        self.indices.insert(key, id.index());
        id
    }

    pub(super) fn finish(self) -> ConstantTable {
        self.table
    }
}

impl LoweringContext {
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
    ) -> specialization::Representability<execution::ConstantId<Value>>
    where
        DraftValue: graph::DraftGraphValue + graph::FreezeGraphValue<Frozen = Value>,
        Value: execution::ConstantValue,
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

    pub(super) fn int_constant(
        &mut self,
        reference: &crate::plan::ConstantIntReference,
    ) -> specialization::Representability<execution::ConstantId<execution::IntLocalId>> {
        let instantiation = reference.instantiation().clone();
        self.lower_constant(
            crate::plan::ConstantInstantiation::from_int(instantiation.clone()),
            |templates| templates.materialize_int(&instantiation),
            graph::int_expr,
        )
    }

    pub(super) fn string_constant(
        &mut self,
        reference: &crate::plan::ConstantStringReference,
    ) -> specialization::Representability<execution::ConstantId<execution::StringLocalId>> {
        let instantiation = reference.instantiation().clone();
        self.lower_constant(
            crate::plan::ConstantInstantiation::from_string(instantiation.clone()),
            |templates| templates.materialize_string(&instantiation),
            graph::string_expr,
        )
    }

    pub(super) fn bit_array_constant(
        &mut self,
        reference: &crate::plan::ConstantBitArrayReference,
    ) -> specialization::Representability<execution::ConstantId<execution::BitArrayLocalId>> {
        let instantiation = reference.instantiation().clone();
        self.lower_constant(
            crate::plan::ConstantInstantiation::from_bit_array(instantiation.clone()),
            |templates| templates.materialize_bit_array(&instantiation),
            graph::bit_array_expr,
        )
    }

    pub(super) fn custom_constant(
        &mut self,
        reference: &crate::plan::ConstantCustomReference,
    ) -> specialization::Representability<execution::ConstantId<execution::CustomLocal>> {
        let instantiation = reference.instantiation().clone();
        self.lower_constant(
            crate::plan::ConstantInstantiation::from_custom(instantiation.clone()),
            |templates| templates.materialize_custom(&instantiation),
            graph::custom_expr,
        )
    }

    pub(super) fn float_constant(
        &mut self,
        reference: &crate::plan::ConstantFloatReference,
    ) -> specialization::Representability<execution::ConstantId<execution::FloatLocalId>> {
        let instantiation = reference.instantiation().clone();
        self.lower_constant(
            crate::plan::ConstantInstantiation::from_float(instantiation.clone()),
            |templates| templates.materialize_float(&instantiation),
            graph::float_expr,
        )
    }

    pub(super) fn bool_constant(
        &mut self,
        reference: &crate::plan::ConstantBoolReference,
    ) -> specialization::Representability<execution::ConstantId<execution::BoolLocalId>> {
        let instantiation = reference.instantiation().clone();
        self.lower_constant(
            crate::plan::ConstantInstantiation::from_bool(instantiation.clone()),
            |templates| templates.materialize_bool(&instantiation),
            graph::bool_expr,
        )
    }

    pub(super) fn nil_constant(
        &mut self,
        reference: &crate::plan::ConstantNilReference,
    ) -> specialization::Representability<execution::ConstantId<execution::NilLocalId>> {
        let instantiation = reference.instantiation().clone();
        self.lower_constant(
            crate::plan::ConstantInstantiation::from_nil(instantiation.clone()),
            |templates| templates.materialize_nil(&instantiation),
            graph::nil_expr,
        )
    }

    pub(super) fn tuple_constant(
        &mut self,
        reference: &crate::plan::ConstantTupleReference,
    ) -> specialization::Representability<execution::ConstantId<execution::TupleLocalId>> {
        let instantiation = reference.instantiation().clone();
        self.lower_constant(
            crate::plan::ConstantInstantiation::from_tuple(instantiation.clone()),
            |templates| templates.materialize_tuple(&instantiation),
            graph::tuple_expr,
        )
    }

    pub(super) fn int_list_constant(
        &mut self,
        reference: &crate::plan::ConstantIntListInstantiation,
    ) -> specialization::Representability<execution::ConstantId<execution::IntListLocalId>> {
        let instantiation = reference.clone();
        self.lower_constant(
            crate::plan::ConstantInstantiation::from_list(
                crate::plan::ConstantListInstantiation::Int(instantiation.clone()),
            ),
            |templates| templates.materialize_int_list(&instantiation),
            graph::int_list_expr,
        )
    }

    pub(super) fn string_list_constant(
        &mut self,
        reference: &crate::plan::ConstantStringListInstantiation,
    ) -> specialization::Representability<execution::ConstantId<execution::StringListLocalId>> {
        let instantiation = reference.clone();
        self.lower_constant(
            crate::plan::ConstantInstantiation::from_list(
                crate::plan::ConstantListInstantiation::String(instantiation.clone()),
            ),
            |templates| templates.materialize_string_list(&instantiation),
            graph::string_list_expr,
        )
    }

    pub(super) fn bit_array_list_constant(
        &mut self,
        reference: &crate::plan::ConstantBitArrayListInstantiation,
    ) -> specialization::Representability<execution::ConstantId<execution::BitArrayListLocalId>>
    {
        let instantiation = reference.clone();
        self.lower_constant(
            crate::plan::ConstantInstantiation::from_list(
                crate::plan::ConstantListInstantiation::BitArray(instantiation.clone()),
            ),
            |templates| templates.materialize_bit_array_list(&instantiation),
            graph::bit_array_list_expr,
        )
    }

    pub(super) fn utf_codepoint_list_constant(
        &mut self,
        reference: &crate::plan::ConstantUtfCodepointListInstantiation,
    ) -> specialization::Representability<execution::ConstantId<execution::UtfCodepointListLocalId>>
    {
        let instantiation = reference.clone();
        self.lower_constant(
            crate::plan::ConstantInstantiation::from_list(
                crate::plan::ConstantListInstantiation::UtfCodepoint(instantiation.clone()),
            ),
            |templates| templates.materialize_utf_codepoint_list(&instantiation),
            graph::utf_codepoint_list_expr,
        )
    }

    pub(super) fn custom_list_constant(
        &mut self,
        reference: &crate::plan::ConstantCustomListInstantiation,
    ) -> specialization::Representability<execution::ConstantId<execution::CustomListLocalId>> {
        let instantiation = reference.clone();
        self.lower_constant(
            crate::plan::ConstantInstantiation::from_list(
                crate::plan::ConstantListInstantiation::Custom(instantiation.clone()),
            ),
            |templates| templates.materialize_custom_list(&instantiation),
            graph::custom_list_expr,
        )
    }

    pub(super) fn float_list_constant(
        &mut self,
        reference: &crate::plan::ConstantFloatListInstantiation,
    ) -> specialization::Representability<execution::ConstantId<execution::FloatListLocalId>> {
        let instantiation = reference.clone();
        self.lower_constant(
            crate::plan::ConstantInstantiation::from_list(
                crate::plan::ConstantListInstantiation::Float(instantiation.clone()),
            ),
            |templates| templates.materialize_float_list(&instantiation),
            graph::float_list_expr,
        )
    }

    pub(super) fn bool_list_constant(
        &mut self,
        reference: &crate::plan::ConstantBoolListInstantiation,
    ) -> specialization::Representability<execution::ConstantId<execution::BoolListLocalId>> {
        let instantiation = reference.clone();
        self.lower_constant(
            crate::plan::ConstantInstantiation::from_list(
                crate::plan::ConstantListInstantiation::Bool(instantiation.clone()),
            ),
            |templates| templates.materialize_bool_list(&instantiation),
            graph::bool_list_expr,
        )
    }

    pub(super) fn nil_list_constant(
        &mut self,
        reference: &crate::plan::ConstantNilListInstantiation,
    ) -> specialization::Representability<execution::ConstantId<execution::NilListLocalId>> {
        let instantiation = reference.clone();
        self.lower_constant(
            crate::plan::ConstantInstantiation::from_list(
                crate::plan::ConstantListInstantiation::Nil(instantiation.clone()),
            ),
            |templates| templates.materialize_nil_list(&instantiation),
            graph::nil_list_expr,
        )
    }

    pub(super) fn tuple_list_constant(
        &mut self,
        reference: &crate::plan::ConstantTupleListInstantiation,
    ) -> specialization::Representability<execution::ConstantId<execution::TupleListLocalId>> {
        let instantiation = reference.clone();
        self.lower_constant(
            crate::plan::ConstantInstantiation::from_list(
                crate::plan::ConstantListInstantiation::Tuple(instantiation.clone()),
            ),
            |templates| templates.materialize_tuple_list(&instantiation),
            graph::tuple_list_expr,
        )
    }

    pub(super) fn parameter_list_list_constant(
        &mut self,
        reference: &crate::plan::ConstantParameterListListInstantiation,
        _parameter: crate::plan::TypeParameterId,
    ) -> specialization::Representability<execution::ConstantId<execution::ParameterListListLocalId>>
    {
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

    pub(super) fn list_list_constant(
        &mut self,
        reference: &crate::plan::ConstantListListInstantiation,
    ) -> specialization::Representability<execution::ConstantId<execution::ListListLocalId>> {
        let instantiation = reference.clone();
        self.lower_constant(
            crate::plan::ConstantInstantiation::from_list(
                crate::plan::ConstantListInstantiation::List(instantiation.clone()),
            ),
            |templates| templates.materialize_list_list(&instantiation),
            graph::list_list_expr,
        )
    }

    pub(super) fn function_list_constant(
        &mut self,
        reference: &crate::plan::ConstantFunctionListInstantiation,
    ) -> specialization::Representability<execution::ConstantId<execution::FunctionListLocalId>>
    {
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
    ) -> specialization::Representability<execution::ConstantId<Value>>
    where
        DraftValue: graph::DraftGraphValue + graph::FreezeGraphValue<Frozen = Value>,
        Value: execution::ConstantValue,
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

    pub(super) fn generic_parameter_list_constant(
        &mut self,
        reference: &crate::plan::ConstantGenericListInstantiation,
        _parameter: crate::plan::TypeParameterId,
    ) -> specialization::Representability<execution::ConstantId<execution::ParameterListLocalId>>
    {
        self.lower_generic_list_constant(reference, graph::DraftParameterList::new)
    }

    pub(super) fn generic_int_list_constant(
        &mut self,
        reference: &crate::plan::ConstantGenericListInstantiation,
    ) -> specialization::Representability<execution::ConstantId<execution::IntListLocalId>> {
        self.lower_generic_list_constant(reference, graph::DraftIntList::new)
    }

    pub(super) fn generic_string_list_constant(
        &mut self,
        reference: &crate::plan::ConstantGenericListInstantiation,
    ) -> specialization::Representability<execution::ConstantId<execution::StringListLocalId>> {
        self.lower_generic_list_constant(reference, graph::DraftStringList::new)
    }

    pub(super) fn generic_bit_array_list_constant(
        &mut self,
        reference: &crate::plan::ConstantGenericListInstantiation,
    ) -> specialization::Representability<execution::ConstantId<execution::BitArrayListLocalId>>
    {
        self.lower_generic_list_constant(reference, graph::DraftBitArrayList::new)
    }

    pub(super) fn generic_utf_codepoint_list_constant(
        &mut self,
        reference: &crate::plan::ConstantGenericListInstantiation,
    ) -> specialization::Representability<execution::ConstantId<execution::UtfCodepointListLocalId>>
    {
        self.lower_generic_list_constant(reference, graph::DraftUtfCodepointList::new)
    }

    pub(super) fn generic_custom_list_constant(
        &mut self,
        reference: &crate::plan::ConstantGenericListInstantiation,
        _shape: &SpecializedCustomValueShape,
    ) -> specialization::Representability<execution::ConstantId<execution::CustomListLocalId>> {
        self.lower_generic_list_constant(reference, graph::DraftCustomList::new)
    }

    pub(super) fn generic_float_list_constant(
        &mut self,
        reference: &crate::plan::ConstantGenericListInstantiation,
    ) -> specialization::Representability<execution::ConstantId<execution::FloatListLocalId>> {
        self.lower_generic_list_constant(reference, graph::DraftFloatList::new)
    }

    pub(super) fn generic_bool_list_constant(
        &mut self,
        reference: &crate::plan::ConstantGenericListInstantiation,
    ) -> specialization::Representability<execution::ConstantId<execution::BoolListLocalId>> {
        self.lower_generic_list_constant(reference, graph::DraftBoolList::new)
    }

    pub(super) fn generic_nil_list_constant(
        &mut self,
        reference: &crate::plan::ConstantGenericListInstantiation,
    ) -> specialization::Representability<execution::ConstantId<execution::NilListLocalId>> {
        self.lower_generic_list_constant(reference, graph::DraftNilList::new)
    }

    pub(super) fn generic_tuple_list_constant(
        &mut self,
        reference: &crate::plan::ConstantGenericListInstantiation,
        _elements: &[SpecializedValueShape],
    ) -> specialization::Representability<execution::ConstantId<execution::TupleListLocalId>> {
        self.lower_generic_list_constant(reference, graph::DraftTupleList::new)
    }

    pub(super) fn generic_parameter_list_list_constant(
        &mut self,
        reference: &crate::plan::ConstantGenericListInstantiation,
        _parameter: crate::plan::TypeParameterId,
    ) -> specialization::Representability<execution::ConstantId<execution::ParameterListListLocalId>>
    {
        self.lower_generic_list_constant(reference, graph::DraftParameterListList::new)
    }

    pub(super) fn generic_list_list_constant(
        &mut self,
        reference: &crate::plan::ConstantGenericListInstantiation,
        _item_shape: &specialization::StoredValueShape,
    ) -> specialization::Representability<execution::ConstantId<execution::ListListLocalId>> {
        self.lower_generic_list_constant(reference, graph::DraftListList::new)
    }

    pub(super) fn generic_function_list_constant(
        &mut self,
        reference: &crate::plan::ConstantGenericListInstantiation,
        _shape: &SpecializedFunctionShape,
    ) -> specialization::Representability<execution::ConstantId<execution::FunctionListLocalId>>
    {
        self.lower_generic_list_constant(reference, graph::DraftFunctionList::new)
    }

    pub(super) fn parameter_list_list_as_stored_constant(
        &mut self,
        reference: &crate::plan::ConstantParameterListListInstantiation,
        _item_shape: &specialization::StoredValueShape,
    ) -> specialization::Representability<execution::ConstantId<execution::ListListLocalId>> {
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
    ) -> specialization::Representability<execution::ConstantId<execution::FunctionLocal>>
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

    pub(super) fn generic_function_constant(
        &mut self,
        reference: &crate::plan::ConstantGenericFunctionInstantiation,
        shape: &SpecializedFunctionShape,
    ) -> specialization::Representability<execution::ConstantId<execution::FunctionLocal>> {
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

    pub(super) fn generic_never_function_constant(
        &mut self,
        reference: &crate::plan::ConstantGenericFunctionInstantiation,
        shape: &SpecializedFunctionShape,
    ) -> specialization::Representability<execution::ConstantId<execution::FunctionLocal>> {
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

    pub(super) fn custom_never_function_constant(
        &mut self,
        reference: &crate::plan::ConstantCustomFunctionInstantiation,
    ) -> specialization::Representability<execution::ConstantId<execution::FunctionLocal>> {
        let instantiation = reference.clone();
        self.lower_function_constant(
            crate::plan::ConstantInstantiation::from_function(
                crate::plan::ConstantFunctionInstantiation::Custom(instantiation.clone()),
            ),
            |templates| templates.materialize_custom_function(&instantiation),
            graph::custom_never_function_expr,
        )
    }

    pub(super) fn tuple_never_function_constant(
        &mut self,
        reference: &crate::plan::ConstantTupleFunctionInstantiation,
    ) -> specialization::Representability<execution::ConstantId<execution::FunctionLocal>> {
        let instantiation = reference.clone();
        self.lower_function_constant(
            crate::plan::ConstantInstantiation::from_function(
                crate::plan::ConstantFunctionInstantiation::Tuple(instantiation.clone()),
            ),
            |templates| templates.materialize_tuple_function(&instantiation),
            graph::tuple_never_function_expr,
        )
    }

    pub(super) fn generic_int_function_constant(
        &mut self,
        reference: &crate::plan::ConstantGenericFunctionInstantiation,
        shape: &SpecializedFunctionShape,
    ) -> specialization::Representability<execution::ConstantId<execution::FunctionLocal>> {
        self.generic_typed_function_constant(reference, shape, graph::generic_int_function_expr)
    }

    pub(super) fn generic_float_function_constant(
        &mut self,
        reference: &crate::plan::ConstantGenericFunctionInstantiation,
        shape: &SpecializedFunctionShape,
    ) -> specialization::Representability<execution::ConstantId<execution::FunctionLocal>> {
        self.generic_typed_function_constant(reference, shape, graph::generic_float_function_expr)
    }

    pub(super) fn generic_string_function_constant(
        &mut self,
        reference: &crate::plan::ConstantGenericFunctionInstantiation,
        shape: &SpecializedFunctionShape,
    ) -> specialization::Representability<execution::ConstantId<execution::FunctionLocal>> {
        self.generic_typed_function_constant(reference, shape, graph::generic_string_function_expr)
    }

    pub(super) fn generic_bit_array_function_constant(
        &mut self,
        reference: &crate::plan::ConstantGenericFunctionInstantiation,
        shape: &SpecializedFunctionShape,
    ) -> specialization::Representability<execution::ConstantId<execution::FunctionLocal>> {
        self.generic_typed_function_constant(
            reference,
            shape,
            graph::generic_bit_array_function_expr,
        )
    }

    pub(super) fn generic_utf_codepoint_function_constant(
        &mut self,
        reference: &crate::plan::ConstantGenericFunctionInstantiation,
        shape: &SpecializedFunctionShape,
    ) -> specialization::Representability<execution::ConstantId<execution::FunctionLocal>> {
        self.generic_typed_function_constant(
            reference,
            shape,
            graph::generic_utf_codepoint_function_expr,
        )
    }

    pub(super) fn generic_bool_function_constant(
        &mut self,
        reference: &crate::plan::ConstantGenericFunctionInstantiation,
        shape: &SpecializedFunctionShape,
    ) -> specialization::Representability<execution::ConstantId<execution::FunctionLocal>> {
        self.generic_typed_function_constant(reference, shape, graph::generic_bool_function_expr)
    }

    pub(super) fn generic_nil_function_constant(
        &mut self,
        reference: &crate::plan::ConstantGenericFunctionInstantiation,
        shape: &SpecializedFunctionShape,
    ) -> specialization::Representability<execution::ConstantId<execution::FunctionLocal>> {
        self.generic_typed_function_constant(reference, shape, graph::generic_nil_function_expr)
    }

    pub(super) fn generic_tuple_function_constant(
        &mut self,
        reference: &crate::plan::ConstantGenericFunctionInstantiation,
        shape: &SpecializedFunctionShape,
    ) -> specialization::Representability<execution::ConstantId<execution::FunctionLocal>> {
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
    ) -> specialization::Representability<execution::ConstantId<execution::FunctionLocal>>
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

    pub(super) fn generic_custom_function_constant(
        &mut self,
        reference: &crate::plan::ConstantGenericFunctionInstantiation,
        return_shape: &SpecializedCustomValueShape,
        shape: &SpecializedFunctionShape,
    ) -> specialization::Representability<execution::ConstantId<execution::FunctionLocal>> {
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

    pub(super) fn generic_list_function_constant(
        &mut self,
        reference: &crate::plan::ConstantGenericFunctionInstantiation,
        item_shape: &SpecializedValueShape,
        shape: &SpecializedFunctionShape,
    ) -> specialization::Representability<execution::ConstantId<execution::FunctionLocal>> {
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

    pub(super) fn generic_function_function_constant(
        &mut self,
        reference: &crate::plan::ConstantGenericFunctionInstantiation,
        return_shape: &SpecializedFunctionShape,
        shape: &SpecializedFunctionShape,
    ) -> specialization::Representability<execution::ConstantId<execution::FunctionLocal>> {
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

    pub(super) fn symbolic_custom_function_constant(
        &mut self,
        reference: &crate::plan::ConstantCustomFunctionInstantiation,
        shape: &SpecializedFunctionShape,
    ) -> specialization::Representability<execution::ConstantId<execution::FunctionLocal>> {
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

    pub(super) fn symbolic_list_function_constant(
        &mut self,
        reference: &crate::plan::ConstantListFunctionInstantiation,
        shape: &SpecializedFunctionShape,
    ) -> specialization::Representability<execution::ConstantId<execution::FunctionLocal>> {
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

    pub(super) fn symbolic_function_function_constant(
        &mut self,
        reference: &crate::plan::ConstantFunctionFunctionInstantiation,
        shape: &SpecializedFunctionShape,
    ) -> specialization::Representability<execution::ConstantId<execution::FunctionLocal>> {
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

    pub(super) fn symbolic_int_function_constant(
        &mut self,
        reference: &crate::plan::ConstantIntFunctionInstantiation,
        shape: &SpecializedFunctionShape,
    ) -> specialization::Representability<execution::ConstantId<execution::FunctionLocal>> {
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

    pub(super) fn symbolic_float_function_constant(
        &mut self,
        reference: &crate::plan::ConstantFloatFunctionInstantiation,
        shape: &SpecializedFunctionShape,
    ) -> specialization::Representability<execution::ConstantId<execution::FunctionLocal>> {
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

    pub(super) fn symbolic_string_function_constant(
        &mut self,
        reference: &crate::plan::ConstantStringFunctionInstantiation,
        shape: &SpecializedFunctionShape,
    ) -> specialization::Representability<execution::ConstantId<execution::FunctionLocal>> {
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

    pub(super) fn symbolic_bit_array_function_constant(
        &mut self,
        reference: &crate::plan::ConstantBitArrayFunctionInstantiation,
        shape: &SpecializedFunctionShape,
    ) -> specialization::Representability<execution::ConstantId<execution::FunctionLocal>> {
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

    pub(super) fn symbolic_utf_codepoint_function_constant(
        &mut self,
        reference: &crate::plan::ConstantUtfCodepointFunctionInstantiation,
        shape: &SpecializedFunctionShape,
    ) -> specialization::Representability<execution::ConstantId<execution::FunctionLocal>> {
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

    pub(super) fn symbolic_bool_function_constant(
        &mut self,
        reference: &crate::plan::ConstantBoolFunctionInstantiation,
        shape: &SpecializedFunctionShape,
    ) -> specialization::Representability<execution::ConstantId<execution::FunctionLocal>> {
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

    pub(super) fn symbolic_nil_function_constant(
        &mut self,
        reference: &crate::plan::ConstantNilFunctionInstantiation,
        shape: &SpecializedFunctionShape,
    ) -> specialization::Representability<execution::ConstantId<execution::FunctionLocal>> {
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

    pub(super) fn symbolic_tuple_function_constant(
        &mut self,
        reference: &crate::plan::ConstantTupleFunctionInstantiation,
        shape: &SpecializedFunctionShape,
    ) -> specialization::Representability<execution::ConstantId<execution::FunctionLocal>> {
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

    pub(super) fn int_function_constant(
        &mut self,
        reference: &crate::plan::ConstantIntFunctionInstantiation,
    ) -> specialization::Representability<execution::ConstantId<execution::FunctionLocal>> {
        let instantiation = reference.clone();
        self.lower_function_constant(
            crate::plan::ConstantInstantiation::from_function(
                crate::plan::ConstantFunctionInstantiation::Int(instantiation.clone()),
            ),
            |templates| templates.materialize_int_function(&instantiation),
            graph::int_function_expr,
        )
    }

    pub(super) fn float_function_constant(
        &mut self,
        reference: &crate::plan::ConstantFloatFunctionInstantiation,
    ) -> specialization::Representability<execution::ConstantId<execution::FunctionLocal>> {
        let instantiation = reference.clone();
        self.lower_function_constant(
            crate::plan::ConstantInstantiation::from_function(
                crate::plan::ConstantFunctionInstantiation::Float(instantiation.clone()),
            ),
            |templates| templates.materialize_float_function(&instantiation),
            graph::float_function_expr,
        )
    }

    pub(super) fn string_function_constant(
        &mut self,
        reference: &crate::plan::ConstantStringFunctionInstantiation,
    ) -> specialization::Representability<execution::ConstantId<execution::FunctionLocal>> {
        let instantiation = reference.clone();
        self.lower_function_constant(
            crate::plan::ConstantInstantiation::from_function(
                crate::plan::ConstantFunctionInstantiation::String(instantiation.clone()),
            ),
            |templates| templates.materialize_string_function(&instantiation),
            graph::string_function_expr,
        )
    }

    pub(super) fn bit_array_function_constant(
        &mut self,
        reference: &crate::plan::ConstantBitArrayFunctionInstantiation,
    ) -> specialization::Representability<execution::ConstantId<execution::FunctionLocal>> {
        let instantiation = reference.clone();
        self.lower_function_constant(
            crate::plan::ConstantInstantiation::from_function(
                crate::plan::ConstantFunctionInstantiation::BitArray(instantiation.clone()),
            ),
            |templates| templates.materialize_bit_array_function(&instantiation),
            graph::bit_array_function_expr,
        )
    }

    pub(super) fn utf_codepoint_function_constant(
        &mut self,
        reference: &crate::plan::ConstantUtfCodepointFunctionInstantiation,
    ) -> specialization::Representability<execution::ConstantId<execution::FunctionLocal>> {
        let instantiation = reference.clone();
        self.lower_function_constant(
            crate::plan::ConstantInstantiation::from_function(
                crate::plan::ConstantFunctionInstantiation::UtfCodepoint(instantiation.clone()),
            ),
            |templates| templates.materialize_utf_codepoint_function(&instantiation),
            graph::utf_codepoint_function_expr,
        )
    }

    pub(super) fn custom_function_constant(
        &mut self,
        reference: &crate::plan::ConstantCustomFunctionInstantiation,
    ) -> specialization::Representability<execution::ConstantId<execution::FunctionLocal>> {
        let instantiation = reference.clone();
        self.lower_function_constant(
            crate::plan::ConstantInstantiation::from_function(
                crate::plan::ConstantFunctionInstantiation::Custom(instantiation.clone()),
            ),
            |templates| templates.materialize_custom_function(&instantiation),
            graph::custom_function_expr,
        )
    }

    pub(super) fn bool_function_constant(
        &mut self,
        reference: &crate::plan::ConstantBoolFunctionInstantiation,
    ) -> specialization::Representability<execution::ConstantId<execution::FunctionLocal>> {
        let instantiation = reference.clone();
        self.lower_function_constant(
            crate::plan::ConstantInstantiation::from_function(
                crate::plan::ConstantFunctionInstantiation::Bool(instantiation.clone()),
            ),
            |templates| templates.materialize_bool_function(&instantiation),
            graph::bool_function_expr,
        )
    }

    pub(super) fn nil_function_constant(
        &mut self,
        reference: &crate::plan::ConstantNilFunctionInstantiation,
    ) -> specialization::Representability<execution::ConstantId<execution::FunctionLocal>> {
        let instantiation = reference.clone();
        self.lower_function_constant(
            crate::plan::ConstantInstantiation::from_function(
                crate::plan::ConstantFunctionInstantiation::Nil(instantiation.clone()),
            ),
            |templates| templates.materialize_nil_function(&instantiation),
            graph::nil_function_expr,
        )
    }

    pub(super) fn tuple_function_constant(
        &mut self,
        reference: &crate::plan::ConstantTupleFunctionInstantiation,
    ) -> specialization::Representability<execution::ConstantId<execution::FunctionLocal>> {
        let instantiation = reference.clone();
        self.lower_function_constant(
            crate::plan::ConstantInstantiation::from_function(
                crate::plan::ConstantFunctionInstantiation::Tuple(instantiation.clone()),
            ),
            |templates| templates.materialize_tuple_function(&instantiation),
            graph::tuple_function_expr,
        )
    }

    pub(super) fn list_function_constant(
        &mut self,
        reference: &crate::plan::ConstantListFunctionInstantiation,
    ) -> specialization::Representability<execution::ConstantId<execution::FunctionLocal>> {
        let instantiation = reference.clone();
        self.lower_function_constant(
            crate::plan::ConstantInstantiation::from_function(
                crate::plan::ConstantFunctionInstantiation::List(instantiation.clone()),
            ),
            |templates| templates.materialize_list_function(&instantiation),
            graph::list_function_expr,
        )
    }

    pub(super) fn function_function_constant(
        &mut self,
        reference: &crate::plan::ConstantFunctionFunctionInstantiation,
    ) -> specialization::Representability<execution::ConstantId<execution::FunctionLocal>> {
        let instantiation = reference.clone();
        self.lower_function_constant(
            crate::plan::ConstantInstantiation::from_function(
                crate::plan::ConstantFunctionInstantiation::Function(instantiation.clone()),
            ),
            |templates| templates.materialize_function_function(&instantiation),
            graph::function_function_expr,
        )
    }
}
