use super::constant::{ConstantId, ConstantProgram, ConstantValue};
use super::function::{
    BitArrayFunctionBody, BitArrayFunctionFunctionBody, BitArrayFunctionFunctionId,
    BitArrayFunctionId, BitArrayListFunctionBody, BitArrayListFunctionId, BoolFunctionBody,
    BoolFunctionFunctionBody, BoolFunctionFunctionId, BoolFunctionId, BoolListFunctionBody,
    BoolListFunctionId, CustomFunctionBody, CustomFunctionFunctionBody, CustomFunctionFunctionId,
    CustomFunctionId, CustomListFunctionBody, CustomListFunctionId, ExecutionFunction,
    ExecutionProfile, FloatFunctionBody, FloatFunctionFunctionBody, FloatFunctionFunctionId,
    FloatFunctionId, FloatListFunctionBody, FloatListFunctionId, FunctionFunctionFunctionBody,
    FunctionFunctionFunctionId, FunctionListFunctionBody, FunctionListFunctionId,
    GenericFunctionFunctionBody, GenericFunctionFunctionId, IntFunctionBody,
    IntFunctionFunctionBody, IntFunctionFunctionId, IntFunctionId, IntListFunctionBody,
    IntListFunctionId, ListFunctionFunctionBody, ListFunctionFunctionId, ListListFunctionBody,
    ListListFunctionId, NeverFunctionBody, NeverFunctionFunctionBody, NeverFunctionFunctionId,
    NeverFunctionId, NilFunctionBody, NilFunctionFunctionBody, NilFunctionFunctionId,
    NilFunctionId, NilListFunctionBody, NilListFunctionId, ParameterListFunctionBody,
    ParameterListFunctionId, ParameterListListFunctionBody, ParameterListListFunctionId,
    RuntimeFunctionId, StringFunctionBody, StringFunctionFunctionBody, StringFunctionFunctionId,
    StringFunctionId, StringListFunctionBody, StringListFunctionId, TupleFunctionBody,
    TupleFunctionFunctionBody, TupleFunctionFunctionId, TupleFunctionId, TupleListFunctionBody,
    TupleListFunctionId, UtfCodepointFunctionBody, UtfCodepointFunctionFunctionBody,
    UtfCodepointFunctionFunctionId, UtfCodepointFunctionId, UtfCodepointListFunctionBody,
    UtfCodepointListFunctionId,
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

    fn never_function(
        &self,
        id: NeverFunctionId,
    ) -> &ExecutionFunction<Self::Profile, NeverFunctionBody> {
        self.program().functions.never_function(id)
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

    fn custom_function(
        &self,
        id: CustomFunctionId,
    ) -> &ExecutionFunction<Self::Profile, CustomFunctionBody> {
        self.program().functions.custom_function(id)
    }

    fn nil_function(
        &self,
        id: NilFunctionId,
    ) -> &ExecutionFunction<Self::Profile, NilFunctionBody> {
        self.program().functions.nil_function(id)
    }

    fn tuple_function(
        &self,
        id: TupleFunctionId,
    ) -> &ExecutionFunction<Self::Profile, TupleFunctionBody> {
        self.program().functions.tuple_function(id)
    }

    fn parameter_list_function(
        &self,
        id: ParameterListFunctionId,
    ) -> &ExecutionFunction<Self::Profile, ParameterListFunctionBody> {
        self.program().functions.parameter_list_function(id)
    }

    fn parameter_list_list_function(
        &self,
        id: ParameterListListFunctionId,
    ) -> &ExecutionFunction<Self::Profile, ParameterListListFunctionBody> {
        self.program().functions.parameter_list_list_function(id)
    }

    fn int_list_function(
        &self,
        id: IntListFunctionId,
    ) -> &ExecutionFunction<Self::Profile, IntListFunctionBody> {
        self.program().functions.int_list_function(id)
    }

    fn string_list_function(
        &self,
        id: StringListFunctionId,
    ) -> &ExecutionFunction<Self::Profile, StringListFunctionBody> {
        self.program().functions.string_list_function(id)
    }

    fn bit_array_list_function(
        &self,
        id: BitArrayListFunctionId,
    ) -> &ExecutionFunction<Self::Profile, BitArrayListFunctionBody> {
        self.program().functions.bit_array_list_function(id)
    }

    fn utf_codepoint_list_function(
        &self,
        id: UtfCodepointListFunctionId,
    ) -> &ExecutionFunction<Self::Profile, UtfCodepointListFunctionBody> {
        self.program().functions.utf_codepoint_list_function(id)
    }

    fn custom_list_function(
        &self,
        id: CustomListFunctionId,
    ) -> &ExecutionFunction<Self::Profile, CustomListFunctionBody> {
        self.program().functions.custom_list_function(id)
    }

    fn float_list_function(
        &self,
        id: FloatListFunctionId,
    ) -> &ExecutionFunction<Self::Profile, FloatListFunctionBody> {
        self.program().functions.float_list_function(id)
    }

    fn bool_list_function(
        &self,
        id: BoolListFunctionId,
    ) -> &ExecutionFunction<Self::Profile, BoolListFunctionBody> {
        self.program().functions.bool_list_function(id)
    }

    fn nil_list_function(
        &self,
        id: NilListFunctionId,
    ) -> &ExecutionFunction<Self::Profile, NilListFunctionBody> {
        self.program().functions.nil_list_function(id)
    }

    fn tuple_list_function(
        &self,
        id: TupleListFunctionId,
    ) -> &ExecutionFunction<Self::Profile, TupleListFunctionBody> {
        self.program().functions.tuple_list_function(id)
    }

    fn list_list_function(
        &self,
        id: ListListFunctionId,
    ) -> &ExecutionFunction<Self::Profile, ListListFunctionBody> {
        self.program().functions.list_list_function(id)
    }

    fn function_list_function(
        &self,
        id: FunctionListFunctionId,
    ) -> &ExecutionFunction<Self::Profile, FunctionListFunctionBody> {
        self.program().functions.function_list_function(id)
    }

    fn int_function_function(
        &self,
        id: IntFunctionFunctionId,
    ) -> &ExecutionFunction<Self::Profile, IntFunctionFunctionBody> {
        self.program().functions.int_function_function(id)
    }

    fn float_function_function(
        &self,
        id: FloatFunctionFunctionId,
    ) -> &ExecutionFunction<Self::Profile, FloatFunctionFunctionBody> {
        self.program().functions.float_function_function(id)
    }

    fn string_function_function(
        &self,
        id: StringFunctionFunctionId,
    ) -> &ExecutionFunction<Self::Profile, StringFunctionFunctionBody> {
        self.program().functions.string_function_function(id)
    }

    fn bit_array_function_function(
        &self,
        id: BitArrayFunctionFunctionId,
    ) -> &ExecutionFunction<Self::Profile, BitArrayFunctionFunctionBody> {
        self.program().functions.bit_array_function_function(id)
    }

    fn utf_codepoint_function_function(
        &self,
        id: UtfCodepointFunctionFunctionId,
    ) -> &ExecutionFunction<Self::Profile, UtfCodepointFunctionFunctionBody> {
        self.program().functions.utf_codepoint_function_function(id)
    }

    fn custom_function_function(
        &self,
        id: &CustomFunctionFunctionId,
    ) -> &ExecutionFunction<Self::Profile, CustomFunctionFunctionBody> {
        self.program().functions.custom_function_function(id)
    }

    fn generic_function_function(
        &self,
        id: &GenericFunctionFunctionId,
    ) -> &ExecutionFunction<Self::Profile, GenericFunctionFunctionBody> {
        self.program().functions.generic_function_function(id)
    }

    fn never_function_function(
        &self,
        id: &NeverFunctionFunctionId,
    ) -> &ExecutionFunction<Self::Profile, NeverFunctionFunctionBody> {
        self.program().functions.never_function_function(id)
    }

    fn bool_function_function(
        &self,
        id: BoolFunctionFunctionId,
    ) -> &ExecutionFunction<Self::Profile, BoolFunctionFunctionBody> {
        self.program().functions.bool_function_function(id)
    }

    fn nil_function_function(
        &self,
        id: NilFunctionFunctionId,
    ) -> &ExecutionFunction<Self::Profile, NilFunctionFunctionBody> {
        self.program().functions.nil_function_function(id)
    }

    fn tuple_function_function(
        &self,
        id: TupleFunctionFunctionId,
    ) -> &ExecutionFunction<Self::Profile, TupleFunctionFunctionBody> {
        self.program().functions.tuple_function_function(id)
    }

    fn list_function_function(
        &self,
        id: &ListFunctionFunctionId,
    ) -> &ExecutionFunction<Self::Profile, ListFunctionFunctionBody> {
        self.program().functions.list_function_function(id)
    }

    fn function_function_function(
        &self,
        id: &FunctionFunctionFunctionId,
    ) -> &ExecutionFunction<Self::Profile, FunctionFunctionFunctionBody> {
        self.program().functions.function_function_function(id)
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
