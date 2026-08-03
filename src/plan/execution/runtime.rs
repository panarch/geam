use super::constant::{ConstantId, ConstantValue, ProfiledConstantProgram};
use super::function::{
    BitArrayFunctionFunctionId, BitArrayFunctionId, BitArrayListFunctionId, BoolFunctionFunctionId,
    BoolFunctionId, BoolListFunctionId, CustomFunctionFunctionId, CustomFunctionId,
    CustomListFunctionId, ExecutionFunction, ExecutionNeverFunction, ExecutionProfile,
    ExternalFunctionFunctionId, ExternalFunctionId, ExternalListFunctionFunctionId,
    ExternalListFunctionId, FloatFunctionFunctionId, FloatFunctionId, FloatListFunctionId,
    FunctionFunctionFunctionId, FunctionListFunctionId, GenericFunctionFunctionId,
    IntFunctionFunctionId, IntFunctionId, IntListFunctionId, ListListFunctionId,
    NeverFunctionFunctionId, NeverFunctionId, NilFunctionFunctionId, NilFunctionId,
    NilListFunctionId, ParameterListFunctionId, ParameterListListFunctionId,
    ProfiledListFunctionFunctionId, ProfiledRuntimeFunctionId, StringFunctionFunctionId,
    StringFunctionId, StringListFunctionId, TupleFunctionFunctionId, TupleFunctionId,
    TupleListFunctionId, UtfCodepointFunctionFunctionId, UtfCodepointFunctionId,
    UtfCodepointListFunctionId,
};
use super::function::{
    ExecutionBitArrayFunctionBody, ExecutionBitArrayFunctionFunctionBody,
    ExecutionBitArrayListFunctionBody, ExecutionBoolFunctionBody,
    ExecutionBoolFunctionFunctionBody, ExecutionBoolListFunctionBody,
    ExecutionCoreListFunctionFunctionBody, ExecutionCustomFunctionBody,
    ExecutionCustomFunctionFunctionBody, ExecutionCustomListFunctionBody,
    ExecutionExternalFunctionBody, ExecutionExternalFunctionFunctionBody,
    ExecutionExternalListFunctionBody, ExecutionExternalListFunctionFunctionBody,
    ExecutionFloatFunctionBody, ExecutionFloatFunctionFunctionBody, ExecutionFloatListFunctionBody,
    ExecutionFunctionFunctionFunctionBody, ExecutionFunctionListFunctionBody,
    ExecutionGenericFunctionFunctionBody, ExecutionIntFunctionBody,
    ExecutionIntFunctionFunctionBody, ExecutionIntListFunctionBody, ExecutionListListFunctionBody,
    ExecutionNeverFunctionFunctionBody, ExecutionNilFunctionBody, ExecutionNilFunctionFunctionBody,
    ExecutionNilListFunctionBody, ExecutionParameterListFunctionBody,
    ExecutionParameterListListFunctionBody, ExecutionStringFunctionBody,
    ExecutionStringFunctionFunctionBody, ExecutionStringListFunctionBody,
    ExecutionTupleFunctionBody, ExecutionTupleFunctionFunctionBody, ExecutionTupleListFunctionBody,
    ExecutionUtfCodepointFunctionBody, ExecutionUtfCodepointFunctionFunctionBody,
    ExecutionUtfCodepointListFunctionBody,
};
use super::type_::{
    CustomConstructorId, CustomTypeId, FunctionListTypeId, FunctionType, ListListTypeId,
    ListTypeId, TupleListTypeId, ValueShapeId, ValueType,
};
use super::{ExecutionPlan, ExecutionProgram, HostedExecution};
use crate::host::HostProfile;
use crate::plan::SourceContext;
use ecow::EcoString;
use std::convert::Infallible;

pub(crate) trait RuntimeExecutionPlan: Sized {
    type Profile: ExecutionProfile;
    type RunState;

    fn program(&self) -> &ExecutionProgram<Self::Profile>;

    fn value_metadata(&self) -> RuntimeValueMetadata<'_> {
        RuntimeValueMetadata::new(&self.program().common)
    }

    fn int_function(
        &self,
        id: IntFunctionId,
    ) -> &ExecutionFunction<Self::Profile, ExecutionIntFunctionBody<Self::Profile>>;
    fn bool_function(
        &self,
        id: BoolFunctionId,
    ) -> &ExecutionFunction<Self::Profile, ExecutionBoolFunctionBody<Self::Profile>>;

    fn source_context_for(&self, module: &EcoString) -> Option<&SourceContext> {
        self.program()
            .common
            .modules
            .iter()
            .find(|context| &context.module == module)
            .and_then(|context| context.source_context.as_ref())
    }

    fn main_runtime(
        &self,
    ) -> ProfiledRuntimeFunctionId<<Self::Profile as ExecutionProfile>::Graph> {
        self.program().common.main.clone()
    }

    fn constant<Return: ConstantValue>(
        &self,
        id: ConstantId<Return>,
    ) -> &ProfiledConstantProgram<Return, <Self::Profile as ExecutionProfile>::Graph> {
        self.program().common.constants.get(id)
    }

    fn list_storage_type(&self, id: ListTypeId) -> super::type_::ListStorageTypeId {
        self.program().common.list_types.storage_type(id)
    }

    fn custom_constructor_id(
        &self,
        type_id: CustomTypeId,
        constructor_index: usize,
    ) -> CustomConstructorId {
        self.program()
            .common
            .custom_types
            .constructor_id_for_type(type_id, constructor_index)
    }

    fn value_type(&self, type_: &ValueType) -> crate::plan::ValueType {
        self.program().common.list_types.value_type(
            type_,
            &self.program().common.custom_types,
            &self.program().common.external_types,
        )
    }

    fn custom_value_type(&self, id: CustomTypeId) -> crate::plan::CustomType {
        self.value_metadata().custom_value_type(id)
    }

    fn shape_value_type(&self, id: ValueShapeId) -> ValueType {
        self.program().common.value_shapes.value_type(id).clone()
    }

    fn custom_constructor(
        &self,
        id: CustomConstructorId,
    ) -> &super::type_::CustomConstructorDescriptor {
        self.value_metadata().custom_constructor(id)
    }

    fn never_function(&self, id: NeverFunctionId) -> &ExecutionNeverFunction<Self::Profile> {
        self.program().functions.never_function(id)
    }

    fn float_function(
        &self,
        id: FloatFunctionId,
    ) -> &ExecutionFunction<Self::Profile, ExecutionFloatFunctionBody<Self::Profile>> {
        self.program().functions.float_function(id)
    }

    fn string_function(
        &self,
        id: StringFunctionId,
    ) -> &ExecutionFunction<Self::Profile, ExecutionStringFunctionBody<Self::Profile>> {
        self.program().functions.string_function(id)
    }

    fn bit_array_function(
        &self,
        id: BitArrayFunctionId,
    ) -> &ExecutionFunction<Self::Profile, ExecutionBitArrayFunctionBody<Self::Profile>> {
        self.program().functions.bit_array_function(id)
    }

    fn utf_codepoint_function(
        &self,
        id: UtfCodepointFunctionId,
    ) -> &ExecutionFunction<Self::Profile, ExecutionUtfCodepointFunctionBody<Self::Profile>> {
        self.program().functions.utf_codepoint_function(id)
    }

    fn custom_function(
        &self,
        id: CustomFunctionId,
    ) -> &ExecutionFunction<Self::Profile, ExecutionCustomFunctionBody<Self::Profile>> {
        self.program().functions.custom_function(id)
    }

    fn external_function(
        &self,
        id: ExternalFunctionId,
    ) -> &ExecutionFunction<Self::Profile, ExecutionExternalFunctionBody<Self::Profile>> {
        self.program().functions.external_function(id)
    }

    fn nil_function(
        &self,
        id: NilFunctionId,
    ) -> &ExecutionFunction<Self::Profile, ExecutionNilFunctionBody<Self::Profile>> {
        self.program().functions.nil_function(id)
    }

    fn tuple_function(
        &self,
        id: TupleFunctionId,
    ) -> &ExecutionFunction<Self::Profile, ExecutionTupleFunctionBody<Self::Profile>> {
        self.program().functions.tuple_function(id)
    }

    fn parameter_list_function(
        &self,
        id: ParameterListFunctionId,
    ) -> &ExecutionFunction<Self::Profile, ExecutionParameterListFunctionBody<Self::Profile>> {
        self.program().functions.parameter_list_function(id)
    }

    fn parameter_list_list_function(
        &self,
        id: ParameterListListFunctionId,
    ) -> &ExecutionFunction<Self::Profile, ExecutionParameterListListFunctionBody<Self::Profile>>
    {
        self.program().functions.parameter_list_list_function(id)
    }

    fn int_list_function(
        &self,
        id: IntListFunctionId,
    ) -> &ExecutionFunction<Self::Profile, ExecutionIntListFunctionBody<Self::Profile>> {
        self.program().functions.int_list_function(id)
    }

    fn string_list_function(
        &self,
        id: StringListFunctionId,
    ) -> &ExecutionFunction<Self::Profile, ExecutionStringListFunctionBody<Self::Profile>> {
        self.program().functions.string_list_function(id)
    }

    fn bit_array_list_function(
        &self,
        id: BitArrayListFunctionId,
    ) -> &ExecutionFunction<Self::Profile, ExecutionBitArrayListFunctionBody<Self::Profile>> {
        self.program().functions.bit_array_list_function(id)
    }

    fn utf_codepoint_list_function(
        &self,
        id: UtfCodepointListFunctionId,
    ) -> &ExecutionFunction<Self::Profile, ExecutionUtfCodepointListFunctionBody<Self::Profile>>
    {
        self.program().functions.utf_codepoint_list_function(id)
    }

    fn custom_list_function(
        &self,
        id: CustomListFunctionId,
    ) -> &ExecutionFunction<Self::Profile, ExecutionCustomListFunctionBody<Self::Profile>> {
        self.program().functions.custom_list_function(id)
    }

    fn external_list_function(
        &self,
        id: ExternalListFunctionId,
    ) -> &ExecutionFunction<Self::Profile, ExecutionExternalListFunctionBody<Self::Profile>> {
        self.program().functions.external_list_function(id)
    }

    fn float_list_function(
        &self,
        id: FloatListFunctionId,
    ) -> &ExecutionFunction<Self::Profile, ExecutionFloatListFunctionBody<Self::Profile>> {
        self.program().functions.float_list_function(id)
    }

    fn bool_list_function(
        &self,
        id: BoolListFunctionId,
    ) -> &ExecutionFunction<Self::Profile, ExecutionBoolListFunctionBody<Self::Profile>> {
        self.program().functions.bool_list_function(id)
    }

    fn nil_list_function(
        &self,
        id: NilListFunctionId,
    ) -> &ExecutionFunction<Self::Profile, ExecutionNilListFunctionBody<Self::Profile>> {
        self.program().functions.nil_list_function(id)
    }

    fn tuple_list_function(
        &self,
        id: TupleListFunctionId,
    ) -> &ExecutionFunction<Self::Profile, ExecutionTupleListFunctionBody<Self::Profile>> {
        self.program().functions.tuple_list_function(id)
    }

    fn list_list_function(
        &self,
        id: ListListFunctionId,
    ) -> &ExecutionFunction<Self::Profile, ExecutionListListFunctionBody<Self::Profile>> {
        self.program().functions.list_list_function(id)
    }

    fn function_list_function(
        &self,
        id: FunctionListFunctionId,
    ) -> &ExecutionFunction<Self::Profile, ExecutionFunctionListFunctionBody<Self::Profile>> {
        self.program().functions.function_list_function(id)
    }

    fn int_function_function(
        &self,
        id: IntFunctionFunctionId,
    ) -> &ExecutionFunction<Self::Profile, ExecutionIntFunctionFunctionBody<Self::Profile>> {
        self.program().functions.int_function_function(id)
    }

    fn float_function_function(
        &self,
        id: FloatFunctionFunctionId,
    ) -> &ExecutionFunction<Self::Profile, ExecutionFloatFunctionFunctionBody<Self::Profile>> {
        self.program().functions.float_function_function(id)
    }

    fn string_function_function(
        &self,
        id: StringFunctionFunctionId,
    ) -> &ExecutionFunction<Self::Profile, ExecutionStringFunctionFunctionBody<Self::Profile>> {
        self.program().functions.string_function_function(id)
    }

    fn bit_array_function_function(
        &self,
        id: BitArrayFunctionFunctionId,
    ) -> &ExecutionFunction<Self::Profile, ExecutionBitArrayFunctionFunctionBody<Self::Profile>>
    {
        self.program().functions.bit_array_function_function(id)
    }

    fn utf_codepoint_function_function(
        &self,
        id: UtfCodepointFunctionFunctionId,
    ) -> &ExecutionFunction<Self::Profile, ExecutionUtfCodepointFunctionFunctionBody<Self::Profile>>
    {
        self.program().functions.utf_codepoint_function_function(id)
    }

    fn custom_function_function(
        &self,
        id: &CustomFunctionFunctionId,
    ) -> &ExecutionFunction<Self::Profile, ExecutionCustomFunctionFunctionBody<Self::Profile>> {
        self.program().functions.custom_function_function(id)
    }

    fn external_function_function(
        &self,
        id: &ExternalFunctionFunctionId,
    ) -> &ExecutionFunction<Self::Profile, ExecutionExternalFunctionFunctionBody<Self::Profile>>
    {
        self.program().functions.external_function_function(id)
    }

    fn generic_function_function(
        &self,
        id: &GenericFunctionFunctionId,
    ) -> &ExecutionFunction<Self::Profile, ExecutionGenericFunctionFunctionBody<Self::Profile>>
    {
        self.program().functions.generic_function_function(id)
    }

    fn never_function_function(
        &self,
        id: &NeverFunctionFunctionId,
    ) -> &ExecutionFunction<Self::Profile, ExecutionNeverFunctionFunctionBody<Self::Profile>> {
        self.program().functions.never_function_function(id)
    }

    fn bool_function_function(
        &self,
        id: BoolFunctionFunctionId,
    ) -> &ExecutionFunction<Self::Profile, ExecutionBoolFunctionFunctionBody<Self::Profile>> {
        self.program().functions.bool_function_function(id)
    }

    fn nil_function_function(
        &self,
        id: NilFunctionFunctionId,
    ) -> &ExecutionFunction<Self::Profile, ExecutionNilFunctionFunctionBody<Self::Profile>> {
        self.program().functions.nil_function_function(id)
    }

    fn tuple_function_function(
        &self,
        id: TupleFunctionFunctionId,
    ) -> &ExecutionFunction<Self::Profile, ExecutionTupleFunctionFunctionBody<Self::Profile>> {
        self.program().functions.tuple_function_function(id)
    }

    fn core_list_function_function(
        &self,
        id: &ProfiledListFunctionFunctionId<Infallible>,
    ) -> &ExecutionFunction<Self::Profile, ExecutionCoreListFunctionFunctionBody<Self::Profile>>
    {
        self.program().functions.core_list_function_function(id)
    }

    fn external_list_function_function(
        &self,
        id: ExternalListFunctionFunctionId,
    ) -> &ExecutionFunction<Self::Profile, ExecutionExternalListFunctionFunctionBody<Self::Profile>>
    {
        self.program().functions.external_list_function_function(id)
    }

    fn function_function_function(
        &self,
        id: &FunctionFunctionFunctionId,
    ) -> &ExecutionFunction<Self::Profile, ExecutionFunctionFunctionFunctionBody<Self::Profile>>
    {
        self.program().functions.function_function_function(id)
    }
}

#[derive(Clone, Copy)]
pub(crate) struct RuntimeValueMetadata<'plan> {
    list_types: &'plan super::type_::ListTypeTable,
    custom_types: &'plan super::type_::CustomTypeTable,
    external_types: &'plan super::type_::ExternalTypeTable,
}

impl<'plan> RuntimeValueMetadata<'plan> {
    fn new<Graph: super::function::ExecutionGraphProfile>(
        common: &'plan super::ExecutionProgramCommon<Graph>,
    ) -> Self {
        Self {
            list_types: &common.list_types,
            custom_types: &common.custom_types,
            external_types: &common.external_types,
        }
    }

    pub(crate) fn tuple_list_item_type(&self, id: TupleListTypeId) -> Vec<crate::plan::ValueType> {
        self.list_types
            .tuple_item_type(id, self.custom_types, self.external_types)
    }

    pub(crate) fn nested_list_item_type(&self, id: ListListTypeId) -> crate::plan::ValueType {
        self.list_types
            .nested_list_item_type(id, self.custom_types, self.external_types)
    }

    pub(crate) fn function_list_item_type(
        &self,
        id: FunctionListTypeId,
    ) -> crate::plan::FunctionType {
        self.list_types
            .function_item_type(id, self.custom_types, self.external_types)
    }

    pub(crate) fn function_type(&self, type_: &FunctionType) -> crate::plan::FunctionType {
        self.list_types
            .function_type(type_, self.custom_types, self.external_types)
    }

    pub(crate) fn list_value_type(&self, id: ListTypeId) -> crate::plan::ValueType {
        self.list_types
            .list_value_type(id, self.custom_types, self.external_types)
    }

    pub(crate) fn custom_value_type(&self, id: CustomTypeId) -> crate::plan::CustomType {
        self.custom_types.value_type(id)
    }

    pub(crate) fn external_value_type(
        &self,
        id: super::type_::ExternalTypeId,
    ) -> crate::plan::ExternalType {
        self.external_types.value_type(id)
    }

    pub(crate) fn custom_constructor(
        &self,
        id: CustomConstructorId,
    ) -> &'plan super::type_::CustomConstructorDescriptor {
        self.custom_types.constructor(id)
    }
}

impl RuntimeExecutionPlan for ExecutionPlan {
    type Profile = Infallible;
    type RunState = ();

    fn program(&self) -> &ExecutionProgram<Self::Profile> {
        &self.program
    }

    fn int_function(
        &self,
        id: IntFunctionId,
    ) -> &ExecutionFunction<Self::Profile, ExecutionIntFunctionBody<Self::Profile>> {
        self.program.functions.int_function(id)
    }

    fn bool_function(
        &self,
        id: BoolFunctionId,
    ) -> &ExecutionFunction<Self::Profile, ExecutionBoolFunctionBody<Self::Profile>> {
        self.program.functions.bool_function(id)
    }
}

impl<Profile: HostProfile> RuntimeExecutionPlan for HostedExecution<Profile> {
    type Profile = super::host::HostedExecutionProfile;
    type RunState = Profile::RunState;

    fn program(&self) -> &ExecutionProgram<Self::Profile> {
        &self.program
    }

    fn int_function(
        &self,
        id: IntFunctionId,
    ) -> &ExecutionFunction<Self::Profile, ExecutionIntFunctionBody<Self::Profile>> {
        self.program.functions.int_function(id)
    }

    fn bool_function(
        &self,
        id: BoolFunctionId,
    ) -> &ExecutionFunction<Self::Profile, ExecutionBoolFunctionBody<Self::Profile>> {
        self.program.functions.bool_function(id)
    }
}

#[cfg(test)]
mod tests {
    use super::RuntimeExecutionPlan;
    use crate::plan::execution::function::{BoolFunctionId, IntFunctionId};
    use crate::{compile_typed_module, plan_module};

    #[test]
    fn plain_execution_resolves_only_graph_int_functions() {
        let typed = compile_typed_module("main", "main.gleam", "pub fn main() { 1 }")
            .expect("source should compile");
        let plan = plan_module(typed).expect("source should plan");
        let execution = crate::ExecutionPlan::from_module_plan(plan);

        let function = RuntimeExecutionPlan::int_function(&execution, IntFunctionId(0));

        assert_eq!(function.body().block_graph().blocks().len(), 1);
    }

    #[test]
    fn plain_execution_resolves_only_graph_bool_functions() {
        let typed = compile_typed_module("main", "main.gleam", "pub fn main() { True }")
            .expect("source should compile");
        let plan = plan_module(typed).expect("source should plan");
        let execution = crate::ExecutionPlan::from_module_plan(plan);

        let function = RuntimeExecutionPlan::bool_function(&execution, BoolFunctionId(0));

        assert_eq!(function.body().block_graph().blocks().len(), 1);
    }
}
