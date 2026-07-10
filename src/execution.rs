mod function;
mod lowering;
mod table;

use self::function::ExecutableFunction;
use self::table::FunctionTables;
use crate::plan::{
    BoolFunctionFunctionId, BoolFunctionId, BoolFunctionReturn, BoolListFunctionId, BoolListReturn,
    BoolReturn, FloatFunctionFunctionId, FloatFunctionId, FloatFunctionReturn, FloatListFunctionId,
    FloatListReturn, FloatReturn, FunctionFunctionFunctionId, FunctionFunctionReturn,
    FunctionListFunctionId, FunctionListReturn, IntFunctionFunctionId, IntFunctionId,
    IntFunctionReturn, IntListFunctionId, IntListReturn, IntReturn, ListFunctionFunctionId,
    ListFunctionReturn, ListListFunctionId, ListListReturn, ModulePlan, NilFunctionFunctionId,
    NilFunctionId, NilFunctionReturn, NilListFunctionId, NilListReturn, NilReturn,
    RuntimeFunctionId, SourceContext, StringFunctionFunctionId, StringFunctionId,
    StringFunctionReturn, StringListFunctionId, StringListReturn, StringReturn,
    TupleFunctionFunctionId, TupleFunctionId, TupleFunctionReturn, TupleListFunctionId,
    TupleListReturn, TupleReturn,
};
use ecow::EcoString;

pub struct ExecutionPlan {
    module: EcoString,
    source_context: Option<SourceContext>,
    main: RuntimeFunctionId,
    functions: FunctionTables,
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

    pub(crate) fn main_runtime(&self) -> RuntimeFunctionId {
        self.main.clone()
    }

    pub(crate) fn int_function(&self, id: IntFunctionId) -> &ExecutableFunction<IntReturn> {
        self.functions.int_function(id)
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

    pub(crate) fn string_list_function(
        &self,
        id: StringListFunctionId,
    ) -> &ExecutableFunction<StringListReturn> {
        self.functions.string_list_function(id)
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

    pub(crate) fn function_function_function(
        &self,
        id: FunctionFunctionFunctionId,
    ) -> &ExecutableFunction<FunctionFunctionReturn> {
        self.functions.function_function_function(id)
    }
}
