use super::function::ExecutableFunction;
use super::{
    BoolFunctionFunctionId, BoolFunctionId, BoolFunctionReturn, BoolListFunctionId, BoolListReturn,
    BoolReturn, FloatFunctionFunctionId, FloatFunctionId, FloatFunctionReturn, FloatListFunctionId,
    FloatListReturn, FloatReturn, FunctionFunctionFunctionId, FunctionFunctionReturn,
    FunctionListFunctionId, FunctionListReturn, IntFunctionFunctionId, IntFunctionId,
    IntFunctionReturn, IntListFunctionId, IntListReturn, IntReturn, ListFunctionFunctionId,
    ListFunctionReturn, ListListFunctionId, ListListReturn, NilFunctionFunctionId, NilFunctionId,
    NilFunctionReturn, NilListFunctionId, NilListReturn, NilReturn, StringFunctionFunctionId,
    StringFunctionId, StringFunctionReturn, StringListFunctionId, StringListReturn, StringReturn,
    TupleFunctionFunctionId, TupleFunctionId, TupleFunctionReturn, TupleListFunctionId,
    TupleListReturn, TupleReturn,
};

pub(super) struct FunctionTables {
    pub(super) int_functions: Vec<ExecutableFunction<IntReturn>>,
    pub(super) float_functions: Vec<ExecutableFunction<FloatReturn>>,
    pub(super) string_functions: Vec<ExecutableFunction<StringReturn>>,
    pub(super) bool_functions: Vec<ExecutableFunction<BoolReturn>>,
    pub(super) nil_functions: Vec<ExecutableFunction<NilReturn>>,
    pub(super) tuple_functions: Vec<ExecutableFunction<TupleReturn>>,
    pub(super) int_list_functions: Vec<ExecutableFunction<IntListReturn>>,
    pub(super) string_list_functions: Vec<ExecutableFunction<StringListReturn>>,
    pub(super) float_list_functions: Vec<ExecutableFunction<FloatListReturn>>,
    pub(super) bool_list_functions: Vec<ExecutableFunction<BoolListReturn>>,
    pub(super) nil_list_functions: Vec<ExecutableFunction<NilListReturn>>,
    pub(super) tuple_list_functions: Vec<ExecutableFunction<TupleListReturn>>,
    pub(super) list_list_functions: Vec<ExecutableFunction<ListListReturn>>,
    pub(super) function_list_functions: Vec<ExecutableFunction<FunctionListReturn>>,
    pub(super) int_function_functions: Vec<ExecutableFunction<IntFunctionReturn>>,
    pub(super) float_function_functions: Vec<ExecutableFunction<FloatFunctionReturn>>,
    pub(super) string_function_functions: Vec<ExecutableFunction<StringFunctionReturn>>,
    pub(super) bool_function_functions: Vec<ExecutableFunction<BoolFunctionReturn>>,
    pub(super) nil_function_functions: Vec<ExecutableFunction<NilFunctionReturn>>,
    pub(super) tuple_function_functions: Vec<ExecutableFunction<TupleFunctionReturn>>,
    pub(super) int_list_function_functions: Vec<ExecutableFunction<ListFunctionReturn>>,
    pub(super) string_list_function_functions: Vec<ExecutableFunction<ListFunctionReturn>>,
    pub(super) float_list_function_functions: Vec<ExecutableFunction<ListFunctionReturn>>,
    pub(super) bool_list_function_functions: Vec<ExecutableFunction<ListFunctionReturn>>,
    pub(super) nil_list_function_functions: Vec<ExecutableFunction<ListFunctionReturn>>,
    pub(super) tuple_list_function_functions: Vec<ExecutableFunction<ListFunctionReturn>>,
    pub(super) list_list_function_functions: Vec<ExecutableFunction<ListFunctionReturn>>,
    pub(super) function_list_function_functions: Vec<ExecutableFunction<ListFunctionReturn>>,
    pub(super) function_function_functions: Vec<ExecutableFunction<FunctionFunctionReturn>>,
}

impl FunctionTables {
    pub(super) fn int_function(&self, id: IntFunctionId) -> &ExecutableFunction<IntReturn> {
        &self.int_functions[id.0]
    }

    pub(super) fn float_function(&self, id: FloatFunctionId) -> &ExecutableFunction<FloatReturn> {
        &self.float_functions[id.0]
    }

    pub(super) fn string_function(
        &self,
        id: StringFunctionId,
    ) -> &ExecutableFunction<StringReturn> {
        &self.string_functions[id.0]
    }

    pub(super) fn bool_function(&self, id: BoolFunctionId) -> &ExecutableFunction<BoolReturn> {
        &self.bool_functions[id.0]
    }

    pub(super) fn nil_function(&self, id: NilFunctionId) -> &ExecutableFunction<NilReturn> {
        &self.nil_functions[id.0]
    }

    pub(super) fn tuple_function(&self, id: TupleFunctionId) -> &ExecutableFunction<TupleReturn> {
        &self.tuple_functions[id.0]
    }

    pub(super) fn int_list_function(
        &self,
        id: IntListFunctionId,
    ) -> &ExecutableFunction<IntListReturn> {
        &self.int_list_functions[id.0]
    }

    pub(super) fn string_list_function(
        &self,
        id: StringListFunctionId,
    ) -> &ExecutableFunction<StringListReturn> {
        &self.string_list_functions[id.0]
    }

    pub(super) fn float_list_function(
        &self,
        id: FloatListFunctionId,
    ) -> &ExecutableFunction<FloatListReturn> {
        &self.float_list_functions[id.0]
    }

    pub(super) fn bool_list_function(
        &self,
        id: BoolListFunctionId,
    ) -> &ExecutableFunction<BoolListReturn> {
        &self.bool_list_functions[id.0]
    }

    pub(super) fn nil_list_function(
        &self,
        id: NilListFunctionId,
    ) -> &ExecutableFunction<NilListReturn> {
        &self.nil_list_functions[id.0]
    }

    pub(super) fn tuple_list_function(
        &self,
        id: TupleListFunctionId,
    ) -> &ExecutableFunction<TupleListReturn> {
        &self.tuple_list_functions[id.0]
    }

    pub(super) fn list_list_function(
        &self,
        id: ListListFunctionId,
    ) -> &ExecutableFunction<ListListReturn> {
        &self.list_list_functions[id.0]
    }

    pub(super) fn function_list_function(
        &self,
        id: FunctionListFunctionId,
    ) -> &ExecutableFunction<FunctionListReturn> {
        &self.function_list_functions[id.0]
    }

    pub(super) fn int_function_function(
        &self,
        id: IntFunctionFunctionId,
    ) -> &ExecutableFunction<IntFunctionReturn> {
        &self.int_function_functions[id.0]
    }

    pub(super) fn float_function_function(
        &self,
        id: FloatFunctionFunctionId,
    ) -> &ExecutableFunction<FloatFunctionReturn> {
        &self.float_function_functions[id.0]
    }

    pub(super) fn string_function_function(
        &self,
        id: StringFunctionFunctionId,
    ) -> &ExecutableFunction<StringFunctionReturn> {
        &self.string_function_functions[id.0]
    }

    pub(super) fn bool_function_function(
        &self,
        id: BoolFunctionFunctionId,
    ) -> &ExecutableFunction<BoolFunctionReturn> {
        &self.bool_function_functions[id.0]
    }

    pub(super) fn nil_function_function(
        &self,
        id: NilFunctionFunctionId,
    ) -> &ExecutableFunction<NilFunctionReturn> {
        &self.nil_function_functions[id.0]
    }

    pub(super) fn tuple_function_function(
        &self,
        id: TupleFunctionFunctionId,
    ) -> &ExecutableFunction<TupleFunctionReturn> {
        &self.tuple_function_functions[id.0]
    }

    pub(super) fn list_function_function(
        &self,
        id: &ListFunctionFunctionId,
    ) -> &ExecutableFunction<ListFunctionReturn> {
        match id {
            ListFunctionFunctionId::Int { id, .. } => &self.int_list_function_functions[id.0],
            ListFunctionFunctionId::String { id, .. } => &self.string_list_function_functions[id.0],
            ListFunctionFunctionId::Float { id, .. } => &self.float_list_function_functions[id.0],
            ListFunctionFunctionId::Bool { id, .. } => &self.bool_list_function_functions[id.0],
            ListFunctionFunctionId::Nil { id, .. } => &self.nil_list_function_functions[id.0],
            ListFunctionFunctionId::Tuple { id, .. } => &self.tuple_list_function_functions[id.0],
            ListFunctionFunctionId::List { id, .. } => &self.list_list_function_functions[id.0],
            ListFunctionFunctionId::Function { id, .. } => {
                &self.function_list_function_functions[id.0]
            }
        }
    }

    pub(super) fn function_function_function(
        &self,
        id: FunctionFunctionFunctionId,
    ) -> &ExecutableFunction<FunctionFunctionReturn> {
        &self.function_function_functions[id.0]
    }
}
