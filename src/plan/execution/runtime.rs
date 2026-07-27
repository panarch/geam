use super::constant::{ConstantId, ConstantProgram, ConstantValue};
use super::function::{
    BitArrayFunctionBody, BitArrayFunctionFunctionBody, BitArrayFunctionFunctionId,
    BitArrayFunctionId, BitArrayListFunctionBody, BitArrayListFunctionId, BoolFunctionBody,
    BoolFunctionFunctionBody, BoolFunctionFunctionId, BoolFunctionId, BoolListFunctionBody,
    BoolListFunctionId, CustomFunctionBody, CustomFunctionFunctionBody, CustomFunctionFunctionId,
    CustomFunctionId, CustomListFunctionBody, CustomListFunctionId, ExecutableFunction,
    ExecutionFunction, ExecutionProfile, FloatFunctionBody, FloatFunctionFunctionBody,
    FloatFunctionFunctionId, FloatFunctionId, FloatListFunctionBody, FloatListFunctionId,
    FunctionFunctionFunctionBody, FunctionFunctionFunctionId, FunctionListFunctionBody,
    FunctionListFunctionId, GenericFunctionFunctionBody, GenericFunctionFunctionId,
    IntFunctionBody, IntFunctionFunctionBody, IntFunctionFunctionId, IntFunctionId,
    IntListFunctionBody, IntListFunctionId, ListFunctionFunctionBody, ListFunctionFunctionId,
    ListListFunctionBody, ListListFunctionId, NeverFunctionBody, NeverFunctionFunctionBody,
    NeverFunctionFunctionId, NeverFunctionId, NilFunctionBody, NilFunctionFunctionBody,
    NilFunctionFunctionId, NilFunctionId, NilListFunctionBody, NilListFunctionId,
    ParameterListFunctionBody, ParameterListFunctionId, ParameterListListFunctionBody,
    ParameterListListFunctionId, RuntimeFunctionId, StringFunctionBody, StringFunctionFunctionBody,
    StringFunctionFunctionId, StringFunctionId, StringListFunctionBody, StringListFunctionId,
    TupleFunctionBody, TupleFunctionFunctionBody, TupleFunctionFunctionId, TupleFunctionId,
    TupleListFunctionBody, TupleListFunctionId, UtfCodepointFunctionBody,
    UtfCodepointFunctionFunctionBody, UtfCodepointFunctionFunctionId, UtfCodepointFunctionId,
    UtfCodepointListFunctionBody, UtfCodepointListFunctionId, graph_function,
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

    fn int_function(&self, id: IntFunctionId)
    -> &ExecutionFunction<Self::Profile, IntFunctionBody>;
    fn bool_function(
        &self,
        id: BoolFunctionId,
    ) -> &ExecutionFunction<Self::Profile, BoolFunctionBody>;

    fn source_context_for(&self, module: &EcoString) -> Option<&SourceContext> {
        self.program()
            .common
            .modules
            .iter()
            .find(|context| &context.module == module)
            .and_then(|context| context.source_context.as_ref())
    }

    fn main_runtime(&self) -> RuntimeFunctionId {
        self.program().common.main.clone()
    }

    fn constant<Return: ConstantValue>(&self, id: ConstantId<Return>) -> &ConstantProgram<Return> {
        self.program().common.constants.get(id)
    }

    fn list_value_type(&self, id: ListTypeId) -> crate::plan::ValueType {
        self.program()
            .common
            .list_types
            .list_value_type(id, &self.program().common.custom_types)
    }

    fn tuple_list_item_type(&self, id: TupleListTypeId) -> Vec<crate::plan::ValueType> {
        self.program()
            .common
            .list_types
            .tuple_item_type(id, &self.program().common.custom_types)
    }

    fn nested_list_item_type(&self, id: ListListTypeId) -> crate::plan::ValueType {
        self.program()
            .common
            .list_types
            .nested_list_item_type(id, &self.program().common.custom_types)
    }

    fn function_list_item_type(&self, id: FunctionListTypeId) -> crate::plan::FunctionType {
        self.program()
            .common
            .list_types
            .function_item_type(id, &self.program().common.custom_types)
    }

    fn value_type(&self, type_: &ValueType) -> crate::plan::ValueType {
        self.program()
            .common
            .list_types
            .value_type(type_, &self.program().common.custom_types)
    }

    fn function_type(&self, type_: &FunctionType) -> crate::plan::FunctionType {
        self.program()
            .common
            .list_types
            .function_type(type_, &self.program().common.custom_types)
    }

    fn custom_value_type(&self, id: CustomTypeId) -> crate::plan::CustomType {
        self.program().common.custom_types.value_type(id)
    }

    fn shape_value_type(&self, id: ValueShapeId) -> ValueType {
        self.program().common.value_shapes.value_type(id).clone()
    }

    fn custom_constructor(
        &self,
        id: CustomConstructorId,
    ) -> &super::type_::CustomConstructorDescriptor {
        self.program().common.custom_types.constructor(id)
    }

    fn never_function(&self, id: NeverFunctionId) -> &ExecutableFunction<NeverFunctionBody> {
        graph_function::<Self::Profile, _>(self.program().functions.never_function(id))
    }

    fn float_function(
        &self,
        id: FloatFunctionId,
    ) -> &ExecutionFunction<Self::Profile, FloatFunctionBody> {
        self.program().functions.float_function(id)
    }

    fn string_function(
        &self,
        id: StringFunctionId,
    ) -> &ExecutionFunction<Self::Profile, StringFunctionBody> {
        self.program().functions.string_function(id)
    }

    fn bit_array_function(
        &self,
        id: BitArrayFunctionId,
    ) -> &ExecutionFunction<Self::Profile, BitArrayFunctionBody> {
        self.program().functions.bit_array_function(id)
    }

    fn utf_codepoint_function(
        &self,
        id: UtfCodepointFunctionId,
    ) -> &ExecutionFunction<Self::Profile, UtfCodepointFunctionBody> {
        self.program().functions.utf_codepoint_function(id)
    }

    fn custom_function(&self, id: CustomFunctionId) -> &ExecutableFunction<CustomFunctionBody> {
        graph_function::<Self::Profile, _>(self.program().functions.custom_function(id))
    }

    fn nil_function(
        &self,
        id: NilFunctionId,
    ) -> &ExecutionFunction<Self::Profile, NilFunctionBody> {
        self.program().functions.nil_function(id)
    }

    fn tuple_function(&self, id: TupleFunctionId) -> &ExecutableFunction<TupleFunctionBody> {
        graph_function::<Self::Profile, _>(self.program().functions.tuple_function(id))
    }

    fn parameter_list_function(
        &self,
        id: ParameterListFunctionId,
    ) -> &ExecutableFunction<ParameterListFunctionBody> {
        graph_function::<Self::Profile, _>(self.program().functions.parameter_list_function(id))
    }

    fn parameter_list_list_function(
        &self,
        id: ParameterListListFunctionId,
    ) -> &ExecutableFunction<ParameterListListFunctionBody> {
        graph_function::<Self::Profile, _>(
            self.program().functions.parameter_list_list_function(id),
        )
    }

    fn int_list_function(&self, id: IntListFunctionId) -> &ExecutableFunction<IntListFunctionBody> {
        graph_function::<Self::Profile, _>(self.program().functions.int_list_function(id))
    }

    fn string_list_function(
        &self,
        id: StringListFunctionId,
    ) -> &ExecutableFunction<StringListFunctionBody> {
        graph_function::<Self::Profile, _>(self.program().functions.string_list_function(id))
    }

    fn bit_array_list_function(
        &self,
        id: BitArrayListFunctionId,
    ) -> &ExecutableFunction<BitArrayListFunctionBody> {
        graph_function::<Self::Profile, _>(self.program().functions.bit_array_list_function(id))
    }

    fn utf_codepoint_list_function(
        &self,
        id: UtfCodepointListFunctionId,
    ) -> &ExecutableFunction<UtfCodepointListFunctionBody> {
        graph_function::<Self::Profile, _>(self.program().functions.utf_codepoint_list_function(id))
    }

    fn custom_list_function(
        &self,
        id: CustomListFunctionId,
    ) -> &ExecutableFunction<CustomListFunctionBody> {
        graph_function::<Self::Profile, _>(self.program().functions.custom_list_function(id))
    }

    fn float_list_function(
        &self,
        id: FloatListFunctionId,
    ) -> &ExecutableFunction<FloatListFunctionBody> {
        graph_function::<Self::Profile, _>(self.program().functions.float_list_function(id))
    }

    fn bool_list_function(
        &self,
        id: BoolListFunctionId,
    ) -> &ExecutableFunction<BoolListFunctionBody> {
        graph_function::<Self::Profile, _>(self.program().functions.bool_list_function(id))
    }

    fn nil_list_function(&self, id: NilListFunctionId) -> &ExecutableFunction<NilListFunctionBody> {
        graph_function::<Self::Profile, _>(self.program().functions.nil_list_function(id))
    }

    fn tuple_list_function(
        &self,
        id: TupleListFunctionId,
    ) -> &ExecutableFunction<TupleListFunctionBody> {
        graph_function::<Self::Profile, _>(self.program().functions.tuple_list_function(id))
    }

    fn list_list_function(
        &self,
        id: ListListFunctionId,
    ) -> &ExecutableFunction<ListListFunctionBody> {
        graph_function::<Self::Profile, _>(self.program().functions.list_list_function(id))
    }

    fn function_list_function(
        &self,
        id: FunctionListFunctionId,
    ) -> &ExecutableFunction<FunctionListFunctionBody> {
        graph_function::<Self::Profile, _>(self.program().functions.function_list_function(id))
    }

    fn int_function_function(
        &self,
        id: IntFunctionFunctionId,
    ) -> &ExecutableFunction<IntFunctionFunctionBody> {
        graph_function::<Self::Profile, _>(self.program().functions.int_function_function(id))
    }

    fn float_function_function(
        &self,
        id: FloatFunctionFunctionId,
    ) -> &ExecutableFunction<FloatFunctionFunctionBody> {
        graph_function::<Self::Profile, _>(self.program().functions.float_function_function(id))
    }

    fn string_function_function(
        &self,
        id: StringFunctionFunctionId,
    ) -> &ExecutableFunction<StringFunctionFunctionBody> {
        graph_function::<Self::Profile, _>(self.program().functions.string_function_function(id))
    }

    fn bit_array_function_function(
        &self,
        id: BitArrayFunctionFunctionId,
    ) -> &ExecutableFunction<BitArrayFunctionFunctionBody> {
        graph_function::<Self::Profile, _>(self.program().functions.bit_array_function_function(id))
    }

    fn utf_codepoint_function_function(
        &self,
        id: UtfCodepointFunctionFunctionId,
    ) -> &ExecutableFunction<UtfCodepointFunctionFunctionBody> {
        graph_function::<Self::Profile, _>(
            self.program().functions.utf_codepoint_function_function(id),
        )
    }

    fn custom_function_function(
        &self,
        id: &CustomFunctionFunctionId,
    ) -> &ExecutableFunction<CustomFunctionFunctionBody> {
        graph_function::<Self::Profile, _>(self.program().functions.custom_function_function(id))
    }

    fn generic_function_function(
        &self,
        id: &GenericFunctionFunctionId,
    ) -> &ExecutableFunction<GenericFunctionFunctionBody> {
        graph_function::<Self::Profile, _>(self.program().functions.generic_function_function(id))
    }

    fn never_function_function(
        &self,
        id: &NeverFunctionFunctionId,
    ) -> &ExecutableFunction<NeverFunctionFunctionBody> {
        graph_function::<Self::Profile, _>(self.program().functions.never_function_function(id))
    }

    fn bool_function_function(
        &self,
        id: BoolFunctionFunctionId,
    ) -> &ExecutableFunction<BoolFunctionFunctionBody> {
        graph_function::<Self::Profile, _>(self.program().functions.bool_function_function(id))
    }

    fn nil_function_function(
        &self,
        id: NilFunctionFunctionId,
    ) -> &ExecutableFunction<NilFunctionFunctionBody> {
        graph_function::<Self::Profile, _>(self.program().functions.nil_function_function(id))
    }

    fn tuple_function_function(
        &self,
        id: TupleFunctionFunctionId,
    ) -> &ExecutableFunction<TupleFunctionFunctionBody> {
        graph_function::<Self::Profile, _>(self.program().functions.tuple_function_function(id))
    }

    fn list_function_function(
        &self,
        id: &ListFunctionFunctionId,
    ) -> &ExecutableFunction<ListFunctionFunctionBody> {
        graph_function::<Self::Profile, _>(self.program().functions.list_function_function(id))
    }

    fn function_function_function(
        &self,
        id: &FunctionFunctionFunctionId,
    ) -> &ExecutableFunction<FunctionFunctionFunctionBody> {
        graph_function::<Self::Profile, _>(self.program().functions.function_function_function(id))
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
    ) -> &ExecutionFunction<Self::Profile, IntFunctionBody> {
        self.program.functions.int_function(id)
    }

    fn bool_function(
        &self,
        id: BoolFunctionId,
    ) -> &ExecutionFunction<Self::Profile, BoolFunctionBody> {
        self.program.functions.bool_function(id)
    }
}

impl<Profile: HostProfile> RuntimeExecutionPlan for HostedExecution<Profile> {
    type Profile = super::host::HostedExecutionProfile<Profile>;
    type RunState = Profile::RunState;

    fn program(&self) -> &ExecutionProgram<Self::Profile> {
        &self.program
    }

    fn int_function(
        &self,
        id: IntFunctionId,
    ) -> &ExecutionFunction<Self::Profile, IntFunctionBody> {
        self.program.functions.int_function(id)
    }

    fn bool_function(
        &self,
        id: BoolFunctionId,
    ) -> &ExecutionFunction<Self::Profile, BoolFunctionBody> {
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
