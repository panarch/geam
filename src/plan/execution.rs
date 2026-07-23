pub(crate) mod constant;
mod explain;
pub(crate) mod function;
pub(crate) mod graph;
mod lowering;
pub(crate) mod type_;

pub use explain::ExecutionPlanExplanation;
pub(crate) use graph::{
    BitArrayBindingPattern, BitArrayBitsSize, BitArrayEvaluatedSize, BitArrayFunctionLocalId,
    BitArrayInstruction, BitArrayListFunctionLocalId, BitArrayListLocalId, BitArrayLocalId,
    BitArrayPattern, BitArrayPatternSegment, BitArrayPatternSize, BitArrayPatternSizeExpr,
    BitArrayPatternValue, BitArraySegment, BitArrayStringPattern, BlockGraph, BlockGraphExitId,
    BlockId, BoolFunctionLocalId, BoolInstruction, BoolListFunctionLocalId, BoolListLocalId,
    BoolLocalId, CustomFunctionLocal, CustomFunctionLocalId, CustomInstruction,
    CustomListFunctionLocalId, CustomListLocalId, CustomLocal, CustomLocalId, Edge, Endianness,
    FloatBitSize, FloatFunctionLocalId, FloatInstruction, FloatListFunctionLocalId,
    FloatListLocalId, FloatLocalId, FunctionCapture, FunctionFunctionLocal,
    FunctionFunctionLocalId, FunctionInstruction, FunctionInstructionKind,
    FunctionListFunctionLocalId, FunctionListLocalId, FunctionLocal, FunctionTarget,
    GenericFunctionLocal, GenericFunctionLocalId, Instruction, InstructionKind, IntFunctionLocalId,
    IntInstruction, IntListFunctionLocalId, IntListLocalId, IntLocalId, ListFunctionLocal,
    ListInstruction, ListListFunctionLocalId, ListListLocalId, ListLocal, MatchEdge,
    MatchEdgeArgument, MatchIntBindingId, MatchPattern, MatchPatternBinding, MatchPatternListTail,
    NeverCallTarget, NeverFunctionLocal, NeverFunctionLocalId, NilFunctionLocalId, NilInstruction,
    NilListFunctionLocalId, NilListLocalId, NilLocalId, ParamLocal, ParamSlot,
    ParameterListFunctionLocalId, ParameterListInstruction, ParameterListListFunctionLocalId,
    ParameterListListLocalId, ParameterListLocalId, Signedness, SourceStopKind, StoredListLocal,
    StringEncoding, StringFunctionLocalId, StringInstruction, StringListFunctionLocalId,
    StringListLocalId, StringLocalId, Terminator, TupleFunctionLocalId, TupleInstruction,
    TupleListFunctionLocalId, TupleListLocalId, TupleLocalId, TypedListInstruction,
    UtfCodepointFunctionLocalId, UtfCodepointInstruction, UtfCodepointListFunctionLocalId,
    UtfCodepointListLocalId, UtfCodepointLocalId,
};
#[cfg(test)]
pub(crate) use type_::ValueShapeDescriptor;
pub(crate) use type_::{
    BitArrayListTypeId, BoolListTypeId, CustomConstructorId, CustomConstructorRefinement,
    CustomFunctionType, CustomListTypeId, CustomTypeId, CustomValueShape, CustomValueShapeId,
    FloatListTypeId, FunctionFunctionType, FunctionListTypeId, FunctionShape, FunctionType,
    GenericFunctionType, IntListTypeId, ListListTypeId, ListStorageTypeId, ListTypeId,
    NilListTypeId, ParameterListListTypeId, ParameterListTypeId, StringListTypeId, TupleListTypeId,
    UtfCodepointListTypeId, ValueShapeId, ValueType,
};

pub(crate) use self::constant::{ConstantId, ConstantProgram, ConstantTable, ConstantValue};
use self::function::FunctionTables;
pub(crate) use self::function::{
    BitArrayFunctionBody, BitArrayFunctionFunctionBody, BitArrayFunctionFunctionId,
    BitArrayFunctionId, BitArrayListFunctionBody, BitArrayListFunctionFunctionId,
    BitArrayListFunctionId, BoolFunctionBody, BoolFunctionFunctionBody, BoolFunctionFunctionId,
    BoolFunctionId, BoolListFunctionBody, BoolListFunctionFunctionId, BoolListFunctionId,
    CustomFunctionBody, CustomFunctionFunctionBody, CustomFunctionFunctionId, CustomFunctionId,
    CustomListFunctionBody, CustomListFunctionFunctionId, CustomListFunctionId, ExecutableFunction,
    FloatFunctionBody, FloatFunctionFunctionBody, FloatFunctionFunctionId, FloatFunctionId,
    FloatListFunctionBody, FloatListFunctionFunctionId, FloatListFunctionId, FunctionBody,
    FunctionEntry, FunctionExit, FunctionFunctionFunctionBody, FunctionFunctionFunctionId,
    FunctionFunctionId, FunctionListFunctionBody, FunctionListFunctionFunctionId,
    FunctionListFunctionId, FunctionReturnFamily, GenericCallableId, GenericFunctionFunctionBody,
    GenericFunctionFunctionId, IntFunctionBody, IntFunctionFunctionBody, IntFunctionFunctionId,
    IntFunctionId, IntListFunctionBody, IntListFunctionFunctionId, IntListFunctionId,
    ListFunctionFunctionBody, ListFunctionFunctionId, ListFunctionId, ListListFunctionBody,
    ListListFunctionFunctionId, ListListFunctionId, NeverFunctionBody, NeverFunctionFunctionBody,
    NeverFunctionFunctionId, NeverFunctionId, NilFunctionBody, NilFunctionFunctionBody,
    NilFunctionFunctionId, NilFunctionId, NilListFunctionBody, NilListFunctionFunctionId,
    NilListFunctionId, ParameterListFunctionBody, ParameterListFunctionFunctionId,
    ParameterListFunctionId, ParameterListListFunctionBody, ParameterListListFunctionFunctionId,
    ParameterListListFunctionId, RuntimeFunctionId, StringFunctionBody, StringFunctionFunctionBody,
    StringFunctionFunctionId, StringFunctionId, StringListFunctionBody,
    StringListFunctionFunctionId, StringListFunctionId, TupleFunctionBody,
    TupleFunctionFunctionBody, TupleFunctionFunctionId, TupleFunctionId, TupleListFunctionBody,
    TupleListFunctionFunctionId, TupleListFunctionId, TypedFunctionBody, UtfCodepointFunctionBody,
    UtfCodepointFunctionFunctionBody, UtfCodepointFunctionFunctionId, UtfCodepointFunctionId,
    UtfCodepointListFunctionBody, UtfCodepointListFunctionFunctionId, UtfCodepointListFunctionId,
};
use self::type_::{CustomTypeTable, ListTypeTable, ValueShapeTable};
use crate::plan::{ModulePlan, SourceContext};
use ecow::EcoString;

pub struct ExecutionPlan {
    module: EcoString,
    source_context: Option<SourceContext>,
    main: RuntimeFunctionId,
    constants: ConstantTable,
    functions: FunctionTables,
    list_types: ListTypeTable,
    custom_types: CustomTypeTable,
    value_shapes: ValueShapeTable,
}

impl explain::Explain for ExecutionPlan {
    fn write_explanation(&self, context: &mut explain::ExplainContext<'_, '_>) {
        context.push_str("module ");
        context.push_str(&self.module);
        context.push_str("\nmain ");
        function::runtime_function_label(&self.main).write(context.output());
        context.push('\n');
        context.write(&self.functions);
        context.write(&self.constants);
    }
}

impl ExecutionPlan {
    pub fn from_module_plan(module_plan: ModulePlan) -> Self {
        lowering::lower(module_plan)
    }

    pub fn module(&self) -> &EcoString {
        &self.module
    }

    pub fn source_context(&self) -> Option<&SourceContext> {
        self.source_context.as_ref()
    }

    pub fn explain(&self) -> ExecutionPlanExplanation<'_> {
        ExecutionPlanExplanation::new(self)
    }

    pub(crate) fn main_runtime(&self) -> RuntimeFunctionId {
        self.main.clone()
    }

    pub(crate) fn constant<Value: ConstantValue>(
        &self,
        id: ConstantId<Value>,
    ) -> &ConstantProgram<Value> {
        self.constants.get(id)
    }

    pub(crate) fn list_value_type(&self, id: ListTypeId) -> crate::plan::ValueType {
        self.list_types.list_value_type(id, &self.custom_types)
    }

    #[cfg(test)]
    pub(crate) fn list_storage_type(&self, id: ListTypeId) -> ListStorageTypeId {
        self.list_types.storage_type(id)
    }

    pub(crate) fn tuple_list_item_type(&self, id: TupleListTypeId) -> Vec<crate::plan::ValueType> {
        self.list_types.tuple_item_type(id, &self.custom_types)
    }

    pub(crate) fn nested_list_item_type(&self, id: ListListTypeId) -> crate::plan::ValueType {
        self.list_types
            .nested_list_item_type(id, &self.custom_types)
    }

    pub(crate) fn function_list_item_type(
        &self,
        id: FunctionListTypeId,
    ) -> crate::plan::FunctionType {
        self.list_types.function_item_type(id, &self.custom_types)
    }

    pub(crate) fn value_type(&self, type_: &ValueType) -> crate::plan::ValueType {
        self.list_types.value_type(type_, &self.custom_types)
    }

    pub(crate) fn function_type(&self, type_: &FunctionType) -> crate::plan::FunctionType {
        self.list_types.function_type(type_, &self.custom_types)
    }

    pub(crate) fn custom_value_type(&self, id: CustomTypeId) -> crate::plan::CustomType {
        self.custom_types.value_type(id)
    }

    #[cfg(test)]
    pub(crate) fn custom_shape_refinement(
        &self,
        shape: &CustomValueShape,
    ) -> CustomConstructorRefinement {
        self.value_shapes.custom(shape.shape_id()).constructor()
    }

    #[cfg(test)]
    pub(crate) fn custom_shape_value_type(
        &self,
        shape: &CustomValueShape,
    ) -> crate::plan::CustomType {
        self.custom_shape_type(shape.shape_id())
    }

    #[cfg(test)]
    fn custom_shape_type(&self, id: CustomValueShapeId) -> crate::plan::CustomType {
        let shape = self.value_shapes.custom(id);
        let nominal = self.custom_types.value_type(shape.type_id());
        crate::plan::CustomType::new(
            nominal.type_name().clone(),
            shape
                .arguments()
                .iter()
                .map(|argument| self.value_shape_type(*argument))
                .collect(),
        )
    }

    #[cfg(test)]
    fn value_shape_type(&self, id: ValueShapeId) -> crate::plan::ValueType {
        match self.value_shapes.get(id) {
            ValueShapeDescriptor::Parameter(parameter) => {
                crate::plan::ValueType::Parameter(*parameter)
            }
            ValueShapeDescriptor::Int => crate::plan::ValueType::Int,
            ValueShapeDescriptor::Float => crate::plan::ValueType::Float,
            ValueShapeDescriptor::String => crate::plan::ValueType::String,
            ValueShapeDescriptor::BitArray => crate::plan::ValueType::BitArray,
            ValueShapeDescriptor::UtfCodepoint => crate::plan::ValueType::UtfCodepoint,
            ValueShapeDescriptor::Bool => crate::plan::ValueType::Bool,
            ValueShapeDescriptor::Nil => crate::plan::ValueType::Nil,
            ValueShapeDescriptor::Tuple(elements) => crate::plan::ValueType::Tuple(
                elements
                    .iter()
                    .map(|element| self.value_shape_type(*element))
                    .collect(),
            ),
            ValueShapeDescriptor::List(item) => {
                crate::plan::ValueType::List(Box::new(self.value_shape_type(*item)))
            }
            ValueShapeDescriptor::Function { arguments, return_ } => {
                crate::plan::ValueType::Function(Box::new(crate::plan::FunctionType::new(
                    arguments
                        .iter()
                        .map(|argument| self.value_shape_type(*argument))
                        .collect(),
                    self.value_shape_type(*return_),
                )))
            }
            ValueShapeDescriptor::Custom(custom) => {
                crate::plan::ValueType::Custom(self.custom_shape_type(*custom))
            }
        }
    }

    pub(crate) fn shape_value_type(&self, id: ValueShapeId) -> ValueType {
        self.value_shapes.value_type(id).clone()
    }

    pub(crate) fn custom_constructor(
        &self,
        id: CustomConstructorId,
    ) -> &type_::CustomConstructorDescriptor {
        self.custom_types.constructor(id)
    }

    #[cfg(test)]
    pub(crate) fn custom_constructor_id(
        &self,
        type_index: usize,
        constructor_index: usize,
    ) -> CustomConstructorId {
        self.custom_types
            .constructor_id(type_index, constructor_index)
    }

    pub(crate) fn int_function(&self, id: IntFunctionId) -> &ExecutableFunction<IntFunctionBody> {
        self.functions.int_function(id)
    }

    pub(crate) fn never_function(
        &self,
        id: NeverFunctionId,
    ) -> &ExecutableFunction<NeverFunctionBody> {
        self.functions.never_function(id)
    }

    pub(crate) fn float_function(
        &self,
        id: FloatFunctionId,
    ) -> &ExecutableFunction<FloatFunctionBody> {
        self.functions.float_function(id)
    }

    pub(crate) fn string_function(
        &self,
        id: StringFunctionId,
    ) -> &ExecutableFunction<StringFunctionBody> {
        self.functions.string_function(id)
    }

    pub(crate) fn bit_array_function(
        &self,
        id: BitArrayFunctionId,
    ) -> &ExecutableFunction<BitArrayFunctionBody> {
        self.functions.bit_array_function(id)
    }

    pub(crate) fn utf_codepoint_function(
        &self,
        id: UtfCodepointFunctionId,
    ) -> &ExecutableFunction<UtfCodepointFunctionBody> {
        self.functions.utf_codepoint_function(id)
    }

    pub(crate) fn custom_function(
        &self,
        id: CustomFunctionId,
    ) -> &ExecutableFunction<CustomFunctionBody> {
        self.functions.custom_function(id)
    }

    #[cfg(test)]
    pub(crate) fn custom_function_id(&self, index: usize) -> CustomFunctionId {
        self.functions.custom_function_id(index)
    }

    pub(crate) fn bool_function(
        &self,
        id: BoolFunctionId,
    ) -> &ExecutableFunction<BoolFunctionBody> {
        self.functions.bool_function(id)
    }

    pub(crate) fn nil_function(&self, id: NilFunctionId) -> &ExecutableFunction<NilFunctionBody> {
        self.functions.nil_function(id)
    }

    pub(crate) fn tuple_function(
        &self,
        id: TupleFunctionId,
    ) -> &ExecutableFunction<TupleFunctionBody> {
        self.functions.tuple_function(id)
    }

    pub(crate) fn int_list_function(
        &self,
        id: IntListFunctionId,
    ) -> &ExecutableFunction<IntListFunctionBody> {
        self.functions.int_list_function(id)
    }

    pub(crate) fn parameter_list_function(
        &self,
        id: ParameterListFunctionId,
    ) -> &ExecutableFunction<ParameterListFunctionBody> {
        self.functions.parameter_list_function(id)
    }

    pub(crate) fn parameter_list_list_function(
        &self,
        id: ParameterListListFunctionId,
    ) -> &ExecutableFunction<ParameterListListFunctionBody> {
        self.functions.parameter_list_list_function(id)
    }

    #[cfg(test)]
    pub(crate) fn parameter_list_function_id(&self, index: usize) -> ParameterListFunctionId {
        self.functions.parameter_list_function_id(index)
    }

    #[cfg(test)]
    pub(crate) fn parameter_list_list_function_id(
        &self,
        index: usize,
    ) -> ParameterListListFunctionId {
        self.functions.parameter_list_list_function_id(index)
    }

    #[cfg(test)]
    pub(crate) fn int_list_function_id(&self, index: usize) -> IntListFunctionId {
        self.functions.int_list_function_id(index)
    }

    #[cfg(test)]
    pub(crate) fn string_list_function_id(&self, index: usize) -> StringListFunctionId {
        self.functions.string_list_function_id(index)
    }

    #[cfg(test)]
    pub(crate) fn bit_array_list_function_id(&self, index: usize) -> BitArrayListFunctionId {
        self.functions.bit_array_list_function_id(index)
    }

    #[cfg(test)]
    pub(crate) fn utf_codepoint_list_function_id(
        &self,
        index: usize,
    ) -> UtfCodepointListFunctionId {
        self.functions.utf_codepoint_list_function_id(index)
    }

    #[cfg(test)]
    pub(crate) fn custom_list_function_id(&self, index: usize) -> CustomListFunctionId {
        self.functions.custom_list_function_id(index)
    }

    #[cfg(test)]
    pub(crate) fn float_list_function_id(&self, index: usize) -> FloatListFunctionId {
        self.functions.float_list_function_id(index)
    }

    #[cfg(test)]
    pub(crate) fn bool_list_function_id(&self, index: usize) -> BoolListFunctionId {
        self.functions.bool_list_function_id(index)
    }

    #[cfg(test)]
    pub(crate) fn nil_list_function_id(&self, index: usize) -> NilListFunctionId {
        self.functions.nil_list_function_id(index)
    }

    #[cfg(test)]
    pub(crate) fn tuple_list_function_id(&self, index: usize) -> TupleListFunctionId {
        self.functions.tuple_list_function_id(index)
    }

    #[cfg(test)]
    pub(crate) fn list_list_function_id(&self, index: usize) -> ListListFunctionId {
        self.functions.list_list_function_id(index)
    }

    #[cfg(test)]
    pub(crate) fn function_list_function_id(&self, index: usize) -> FunctionListFunctionId {
        self.functions.function_list_function_id(index)
    }

    pub(crate) fn string_list_function(
        &self,
        id: StringListFunctionId,
    ) -> &ExecutableFunction<StringListFunctionBody> {
        self.functions.string_list_function(id)
    }

    pub(crate) fn bit_array_list_function(
        &self,
        id: BitArrayListFunctionId,
    ) -> &ExecutableFunction<BitArrayListFunctionBody> {
        self.functions.bit_array_list_function(id)
    }

    pub(crate) fn utf_codepoint_list_function(
        &self,
        id: UtfCodepointListFunctionId,
    ) -> &ExecutableFunction<UtfCodepointListFunctionBody> {
        self.functions.utf_codepoint_list_function(id)
    }

    pub(crate) fn custom_list_function(
        &self,
        id: CustomListFunctionId,
    ) -> &ExecutableFunction<CustomListFunctionBody> {
        self.functions.custom_list_function(id)
    }

    pub(crate) fn float_list_function(
        &self,
        id: FloatListFunctionId,
    ) -> &ExecutableFunction<FloatListFunctionBody> {
        self.functions.float_list_function(id)
    }

    pub(crate) fn bool_list_function(
        &self,
        id: BoolListFunctionId,
    ) -> &ExecutableFunction<BoolListFunctionBody> {
        self.functions.bool_list_function(id)
    }

    pub(crate) fn nil_list_function(
        &self,
        id: NilListFunctionId,
    ) -> &ExecutableFunction<NilListFunctionBody> {
        self.functions.nil_list_function(id)
    }

    pub(crate) fn tuple_list_function(
        &self,
        id: TupleListFunctionId,
    ) -> &ExecutableFunction<TupleListFunctionBody> {
        self.functions.tuple_list_function(id)
    }

    pub(crate) fn list_list_function(
        &self,
        id: ListListFunctionId,
    ) -> &ExecutableFunction<ListListFunctionBody> {
        self.functions.list_list_function(id)
    }

    pub(crate) fn function_list_function(
        &self,
        id: FunctionListFunctionId,
    ) -> &ExecutableFunction<FunctionListFunctionBody> {
        self.functions.function_list_function(id)
    }

    pub(crate) fn int_function_function(
        &self,
        id: IntFunctionFunctionId,
    ) -> &ExecutableFunction<IntFunctionFunctionBody> {
        self.functions.int_function_function(id)
    }

    pub(crate) fn float_function_function(
        &self,
        id: FloatFunctionFunctionId,
    ) -> &ExecutableFunction<FloatFunctionFunctionBody> {
        self.functions.float_function_function(id)
    }

    pub(crate) fn string_function_function(
        &self,
        id: StringFunctionFunctionId,
    ) -> &ExecutableFunction<StringFunctionFunctionBody> {
        self.functions.string_function_function(id)
    }

    pub(crate) fn bit_array_function_function(
        &self,
        id: BitArrayFunctionFunctionId,
    ) -> &ExecutableFunction<BitArrayFunctionFunctionBody> {
        self.functions.bit_array_function_function(id)
    }

    pub(crate) fn utf_codepoint_function_function(
        &self,
        id: UtfCodepointFunctionFunctionId,
    ) -> &ExecutableFunction<UtfCodepointFunctionFunctionBody> {
        self.functions.utf_codepoint_function_function(id)
    }

    pub(crate) fn custom_function_function(
        &self,
        id: &CustomFunctionFunctionId,
    ) -> &ExecutableFunction<CustomFunctionFunctionBody> {
        self.functions.custom_function_function(id)
    }

    pub(crate) fn generic_function_function(
        &self,
        id: &GenericFunctionFunctionId,
    ) -> &ExecutableFunction<GenericFunctionFunctionBody> {
        self.functions.generic_function_function(id)
    }

    pub(crate) fn never_function_function(
        &self,
        id: &NeverFunctionFunctionId,
    ) -> &ExecutableFunction<NeverFunctionFunctionBody> {
        self.functions.never_function_function(id)
    }

    pub(crate) fn bool_function_function(
        &self,
        id: BoolFunctionFunctionId,
    ) -> &ExecutableFunction<BoolFunctionFunctionBody> {
        self.functions.bool_function_function(id)
    }

    pub(crate) fn nil_function_function(
        &self,
        id: NilFunctionFunctionId,
    ) -> &ExecutableFunction<NilFunctionFunctionBody> {
        self.functions.nil_function_function(id)
    }

    pub(crate) fn tuple_function_function(
        &self,
        id: TupleFunctionFunctionId,
    ) -> &ExecutableFunction<TupleFunctionFunctionBody> {
        self.functions.tuple_function_function(id)
    }

    pub(crate) fn list_function_function(
        &self,
        id: &ListFunctionFunctionId,
    ) -> &ExecutableFunction<ListFunctionFunctionBody> {
        self.functions.list_function_function(id)
    }

    pub(crate) fn function_function_function(
        &self,
        id: &FunctionFunctionFunctionId,
    ) -> &ExecutableFunction<FunctionFunctionFunctionBody> {
        self.functions.function_function_function(id)
    }

    #[cfg(test)]
    pub(crate) fn function_function_function_id(&self, index: usize) -> FunctionFunctionFunctionId {
        self.functions.function_function_function_id(index)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn lowering_preserves_public_module_and_source_context() {
        let source = "pub fn main() { 1 }";
        let typed = crate::compile_typed_module("sample", "sample.gleam", source)
            .expect("source should compile");
        let context = crate::SourceContext::new("sample.gleam", source);
        let module =
            crate::plan_module_with_source(typed, context.clone()).expect("source should plan");
        let execution = super::ExecutionPlan::from_module_plan(module);

        assert_eq!(execution.module(), "sample");
        assert_eq!(execution.source_context(), Some(&context));
    }
}
