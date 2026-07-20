mod constant;
mod custom_type;
mod explain;
mod expression;
mod frame;
mod function;
mod id;
mod lowering;
mod param;
mod pattern;
mod reference;
mod return_graph;
mod step;
mod table;
mod value_shape;
mod value_type;

pub use explain::ExecutionPlanExplanation;
pub(crate) use expression::{
    BitArrayBitsSize, BitArrayEvaluatedSize, BitArrayExpr, BitArrayExprKind, BitArrayFunctionExpr,
    BitArrayFunctionExprKind, BitArrayListExpr, BitArrayListItem, BitArraySegment, BoolExpr,
    BoolExprKind, BoolFunctionExpr, BoolFunctionExprKind, BoolListExpr, BoolListItem, CallArg,
    CallArgKind, CaptureArg, CaptureArgKind, CustomConstruction, CustomExpr, CustomExprKind,
    CustomFieldAccess, CustomFunctionExpr, CustomFunctionExprKind, CustomListExpr, CustomListItem,
    CustomLocalExpr, DirectCall, Endianness, Expr, ExprKind, FloatBitSize, FloatExpr,
    FloatExprKind, FloatFunctionExpr, FloatFunctionExprKind, FloatListExpr, FloatListItem,
    FunctionCall, FunctionExpr, FunctionExprKind, FunctionFunctionExpr, FunctionFunctionExprKind,
    FunctionListExpr, FunctionListItem, GenericFunctionExpr, GenericFunctionExprKind, IntExpr,
    IntExprKind, IntFunctionExpr, IntFunctionExprKind, IntListExpr, IntListItem, ListExpr,
    ListFunctionExpr, ListFunctionExprKind, ListIndexSource, ListItem, ListListExpr, ListListItem,
    ListLocalExpr, NeverExpr, NeverExprKind, NeverFunctionExpr, NeverFunctionExprKind, NilExpr,
    NilExprKind, NilFunctionExpr, NilFunctionExprKind, NilListExpr, NilListItem, PanicExpr,
    PanicExprKind, ParameterListExpr, ParameterListExprKind, ParameterListIndexSource,
    ParameterListItem, ParameterListListExpr, ParameterListListItem, StoredListExpr,
    StringEncoding, StringExpr, StringExprKind, StringFunctionExpr, StringFunctionExprKind,
    StringListExpr, StringListItem, TupleExpr, TupleExprKind, TupleFunctionExpr,
    TupleFunctionExprKind, TupleListExpr, TupleListItem, TypedFunctionExpr, TypedListExpr,
    TypedListExprKind, UtfCodepointExpr, UtfCodepointExprKind, UtfCodepointFunctionExpr,
    UtfCodepointFunctionExprKind, UtfCodepointListExpr, UtfCodepointListItem,
};
pub(crate) use frame::FrameLayout;
pub(crate) use id::{
    BitArrayFunctionFunctionId, BitArrayFunctionId, BitArrayFunctionLocalId,
    BitArrayListFunctionFunctionId, BitArrayListFunctionId, BitArrayListFunctionLocalId,
    BitArrayListLocalId, BitArrayLocalId, BoolFunctionFunctionId, BoolFunctionId,
    BoolFunctionLocalId, BoolListFunctionFunctionId, BoolListFunctionId, BoolListFunctionLocalId,
    BoolListLocalId, BoolLocalId, CustomFunctionFunctionId, CustomFunctionId, CustomFunctionLocal,
    CustomFunctionLocalId, CustomListFunctionFunctionId, CustomListFunctionId,
    CustomListFunctionLocalId, CustomListLocalId, CustomLocal, CustomLocalId,
    FloatFunctionFunctionId, FloatFunctionId, FloatFunctionLocalId, FloatListFunctionFunctionId,
    FloatListFunctionId, FloatListFunctionLocalId, FloatListLocalId, FloatLocalId,
    FunctionFunctionFunctionId, FunctionFunctionId, FunctionFunctionLocal, FunctionFunctionLocalId,
    FunctionListFunctionFunctionId, FunctionListFunctionId, FunctionListFunctionLocalId,
    FunctionListLocalId, FunctionReturnFamily, GenericCallableId, GenericFunctionFunctionId,
    GenericFunctionLocal, GenericFunctionLocalId, IntFunctionFunctionId, IntFunctionId,
    IntFunctionLocalId, IntListFunctionFunctionId, IntListFunctionId, IntListFunctionLocalId,
    IntListLocalId, IntLocalId, ListFunctionFunctionId, ListFunctionId, ListFunctionLocal,
    ListListFunctionFunctionId, ListListFunctionId, ListListFunctionLocalId, ListListLocalId,
    ListLocal, NeverFunctionFunctionId, NeverFunctionId, NeverFunctionLocal, NeverFunctionLocalId,
    NilFunctionFunctionId, NilFunctionId, NilFunctionLocalId, NilListFunctionFunctionId,
    NilListFunctionId, NilListFunctionLocalId, NilListLocalId, NilLocalId,
    ParameterListFunctionFunctionId, ParameterListFunctionId, ParameterListFunctionLocalId,
    ParameterListListFunctionFunctionId, ParameterListListFunctionId,
    ParameterListListFunctionLocalId, ParameterListListLocalId, ParameterListLocalId,
    RuntimeFunctionId, StringFunctionFunctionId, StringFunctionId, StringFunctionLocalId,
    StringListFunctionFunctionId, StringListFunctionId, StringListFunctionLocalId,
    StringListLocalId, StringLocalId, TupleFunctionFunctionId, TupleFunctionId,
    TupleFunctionLocalId, TupleListFunctionFunctionId, TupleListFunctionId,
    TupleListFunctionLocalId, TupleListLocalId, TupleLocalId, UtfCodepointFunctionFunctionId,
    UtfCodepointFunctionId, UtfCodepointFunctionLocalId, UtfCodepointListFunctionFunctionId,
    UtfCodepointListFunctionId, UtfCodepointListFunctionLocalId, UtfCodepointListLocalId,
    UtfCodepointLocalId,
};
pub(crate) use param::{ParamLocal, ParamSlot};
pub(crate) use pattern::{
    BitArrayBindingPattern, BitArrayPattern, BitArrayPatternSegment, BitArrayPatternSize,
    BitArrayPatternSizeExpr, BitArrayPatternValue, BitArrayStringPattern, CustomBindingPattern,
    CustomPattern, PatternBinding, Signedness, TotalBindingPattern, TotalBindingPatternKind,
};
pub(crate) use reference::{ClosureTemplate, FunctionReference};
pub(crate) use return_graph::{
    BitArrayFunctionReturn, BitArrayListReturn, BitArrayReturn, BoolFunctionReturn, BoolListReturn,
    BoolReturn, CustomFunctionReturn, CustomListReturn, CustomReturn, FloatFunctionReturn,
    FloatListReturn, FloatReturn, FunctionFunctionReturn, FunctionListReturn,
    GenericFunctionReturn, IntFunctionReturn, IntListReturn, IntReturn, ListFunctionReturn,
    ListListReturn, NeverFunctionReturn, NeverReturn, NilFunctionReturn, NilListReturn, NilReturn,
    ParameterListListReturn, ParameterListReturn, ReturnBlock, ReturnExpressionId, ReturnGraph,
    ReturnTailCall, ReturnTailCallId, ReturnTarget, StringFunctionReturn, StringListReturn,
    StringReturn, TupleFunctionReturn, TupleListReturn, TupleReturn, TypedFunctionReturn,
    UtfCodepointFunctionReturn, UtfCodepointListReturn, UtfCodepointReturn,
};
pub(crate) use step::{
    AssertBinding, AssertPattern, AssertSubject, ListAssertPattern, ListAssertTail, Step, StepKind,
    StringAssertBinding,
};
#[cfg(test)]
pub(crate) use value_shape::ValueShapeDescriptor;
pub(crate) use value_shape::{
    CustomConstructorRefinement, CustomValueShape, CustomValueShapeId, FunctionShape, ValueShapeId,
};
pub(crate) use value_type::{
    BitArrayListTypeId, BoolListTypeId, CustomConstructorId, CustomFunctionType, CustomListTypeId,
    CustomTypeId, FloatListTypeId, FunctionFunctionType, FunctionListTypeId, FunctionType,
    GenericFunctionType, IntListTypeId, ListListTypeId, ListStorageTypeId, ListTypeId,
    NilListTypeId, ParameterListListTypeId, ParameterListTypeId, StringListTypeId, TupleListTypeId,
    UtfCodepointListTypeId, ValueType,
};

use self::custom_type::CustomTypeTable;
use self::function::ExecutableFunction;
use self::table::FunctionTables;
use self::value_shape::ValueShapeTable;
use self::value_type::ListTypeTable;
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

    pub(crate) fn constant<Expression: ConstantExpression>(
        &self,
        id: ConstantId<Expression>,
    ) -> &Expression {
        self.constants.get(id)
    }

    #[cfg(test)]
    pub(crate) fn constant_count(&self) -> usize {
        self.constants.len()
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
    ) -> &custom_type::CustomConstructorDescriptor {
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

    pub(crate) fn int_function(&self, id: IntFunctionId) -> &ExecutableFunction<IntReturn> {
        self.functions.int_function(id)
    }

    pub(crate) fn never_function(&self, id: NeverFunctionId) -> &ExecutableFunction<NeverReturn> {
        self.functions.never_function(id)
    }

    pub(crate) fn float_function(&self, id: FloatFunctionId) -> &ExecutableFunction<FloatReturn> {
        self.functions.float_function(id)
    }

    pub(crate) fn string_function(
        &self,
        id: StringFunctionId,
    ) -> &ExecutableFunction<StringReturn> {
        self.functions.string_function(id)
    }

    pub(crate) fn bit_array_function(
        &self,
        id: BitArrayFunctionId,
    ) -> &ExecutableFunction<BitArrayReturn> {
        self.functions.bit_array_function(id)
    }

    pub(crate) fn utf_codepoint_function(
        &self,
        id: UtfCodepointFunctionId,
    ) -> &ExecutableFunction<UtfCodepointReturn> {
        self.functions.utf_codepoint_function(id)
    }

    pub(crate) fn custom_function(
        &self,
        id: CustomFunctionId,
    ) -> &ExecutableFunction<CustomReturn> {
        self.functions.custom_function(id)
    }

    #[cfg(test)]
    pub(crate) fn custom_function_id(&self, index: usize) -> CustomFunctionId {
        self.functions.custom_function_id(index)
    }

    pub(crate) fn bool_function(&self, id: BoolFunctionId) -> &ExecutableFunction<BoolReturn> {
        self.functions.bool_function(id)
    }

    pub(crate) fn nil_function(&self, id: NilFunctionId) -> &ExecutableFunction<NilReturn> {
        self.functions.nil_function(id)
    }

    pub(crate) fn tuple_function(&self, id: TupleFunctionId) -> &ExecutableFunction<TupleReturn> {
        self.functions.tuple_function(id)
    }

    pub(crate) fn int_list_function(
        &self,
        id: IntListFunctionId,
    ) -> &ExecutableFunction<IntListReturn> {
        self.functions.int_list_function(id)
    }

    pub(crate) fn parameter_list_function(
        &self,
        id: ParameterListFunctionId,
    ) -> &ExecutableFunction<ParameterListReturn> {
        self.functions.parameter_list_function(id)
    }

    pub(crate) fn parameter_list_list_function(
        &self,
        id: ParameterListListFunctionId,
    ) -> &ExecutableFunction<ParameterListListReturn> {
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
    ) -> &ExecutableFunction<StringListReturn> {
        self.functions.string_list_function(id)
    }

    pub(crate) fn bit_array_list_function(
        &self,
        id: BitArrayListFunctionId,
    ) -> &ExecutableFunction<BitArrayListReturn> {
        self.functions.bit_array_list_function(id)
    }

    pub(crate) fn utf_codepoint_list_function(
        &self,
        id: UtfCodepointListFunctionId,
    ) -> &ExecutableFunction<UtfCodepointListReturn> {
        self.functions.utf_codepoint_list_function(id)
    }

    pub(crate) fn custom_list_function(
        &self,
        id: CustomListFunctionId,
    ) -> &ExecutableFunction<CustomListReturn> {
        self.functions.custom_list_function(id)
    }

    pub(crate) fn float_list_function(
        &self,
        id: FloatListFunctionId,
    ) -> &ExecutableFunction<FloatListReturn> {
        self.functions.float_list_function(id)
    }

    pub(crate) fn bool_list_function(
        &self,
        id: BoolListFunctionId,
    ) -> &ExecutableFunction<BoolListReturn> {
        self.functions.bool_list_function(id)
    }

    pub(crate) fn nil_list_function(
        &self,
        id: NilListFunctionId,
    ) -> &ExecutableFunction<NilListReturn> {
        self.functions.nil_list_function(id)
    }

    pub(crate) fn tuple_list_function(
        &self,
        id: TupleListFunctionId,
    ) -> &ExecutableFunction<TupleListReturn> {
        self.functions.tuple_list_function(id)
    }

    pub(crate) fn list_list_function(
        &self,
        id: ListListFunctionId,
    ) -> &ExecutableFunction<ListListReturn> {
        self.functions.list_list_function(id)
    }

    pub(crate) fn function_list_function(
        &self,
        id: FunctionListFunctionId,
    ) -> &ExecutableFunction<FunctionListReturn> {
        self.functions.function_list_function(id)
    }

    pub(crate) fn int_function_function(
        &self,
        id: IntFunctionFunctionId,
    ) -> &ExecutableFunction<IntFunctionReturn> {
        self.functions.int_function_function(id)
    }

    pub(crate) fn float_function_function(
        &self,
        id: FloatFunctionFunctionId,
    ) -> &ExecutableFunction<FloatFunctionReturn> {
        self.functions.float_function_function(id)
    }

    pub(crate) fn string_function_function(
        &self,
        id: StringFunctionFunctionId,
    ) -> &ExecutableFunction<StringFunctionReturn> {
        self.functions.string_function_function(id)
    }

    pub(crate) fn bit_array_function_function(
        &self,
        id: BitArrayFunctionFunctionId,
    ) -> &ExecutableFunction<BitArrayFunctionReturn> {
        self.functions.bit_array_function_function(id)
    }

    pub(crate) fn utf_codepoint_function_function(
        &self,
        id: UtfCodepointFunctionFunctionId,
    ) -> &ExecutableFunction<UtfCodepointFunctionReturn> {
        self.functions.utf_codepoint_function_function(id)
    }

    pub(crate) fn custom_function_function(
        &self,
        id: &CustomFunctionFunctionId,
    ) -> &ExecutableFunction<CustomFunctionReturn> {
        self.functions.custom_function_function(id)
    }

    pub(crate) fn generic_function_function(
        &self,
        id: &GenericFunctionFunctionId,
    ) -> &ExecutableFunction<GenericFunctionReturn> {
        self.functions.generic_function_function(id)
    }

    pub(crate) fn never_function_function(
        &self,
        id: &NeverFunctionFunctionId,
    ) -> &ExecutableFunction<NeverFunctionReturn> {
        self.functions.never_function_function(id)
    }

    #[cfg(test)]
    pub(crate) fn custom_function_function_id(&self, index: usize) -> CustomFunctionFunctionId {
        self.functions.custom_function_function_id(index)
    }

    pub(crate) fn bool_function_function(
        &self,
        id: BoolFunctionFunctionId,
    ) -> &ExecutableFunction<BoolFunctionReturn> {
        self.functions.bool_function_function(id)
    }

    pub(crate) fn nil_function_function(
        &self,
        id: NilFunctionFunctionId,
    ) -> &ExecutableFunction<NilFunctionReturn> {
        self.functions.nil_function_function(id)
    }

    pub(crate) fn tuple_function_function(
        &self,
        id: TupleFunctionFunctionId,
    ) -> &ExecutableFunction<TupleFunctionReturn> {
        self.functions.tuple_function_function(id)
    }

    pub(crate) fn list_function_function(
        &self,
        id: &ListFunctionFunctionId,
    ) -> &ExecutableFunction<ListFunctionReturn> {
        self.functions.list_function_function(id)
    }

    #[cfg(test)]
    pub(crate) fn int_list_function_function(
        &self,
        id: IntListFunctionFunctionId,
    ) -> &ExecutableFunction<ListFunctionReturn> {
        self.functions.int_list_function_function(id)
    }

    pub(crate) fn function_function_function(
        &self,
        id: &FunctionFunctionFunctionId,
    ) -> &ExecutableFunction<FunctionFunctionReturn> {
        self.functions.function_function_function(id)
    }

    #[cfg(test)]
    pub(crate) fn function_function_function_id(&self, index: usize) -> FunctionFunctionFunctionId {
        self.functions.function_function_function_id(index)
    }
}
pub(crate) use constant::{ConstantExpression, ConstantId, ConstantTable};
