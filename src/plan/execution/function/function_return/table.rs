use super::{
    BitArrayFunctionReturn, BoolFunctionReturn, CustomFunctionReturn, FloatFunctionReturn,
    FunctionFunctionReturn, GenericFunctionReturn, IntFunctionReturn, ListFunctionReturn,
    NeverFunctionReturn, NilFunctionReturn, StringFunctionReturn, TupleFunctionReturn,
    UtfCodepointFunctionReturn,
};
use crate::plan::execution::ExecutableFunction;

pub(in crate::plan::execution) struct FunctionFunctionTables {
    pub(in crate::plan::execution) int_function_functions:
        Vec<ExecutableFunction<IntFunctionReturn>>,
    pub(in crate::plan::execution) float_function_functions:
        Vec<ExecutableFunction<FloatFunctionReturn>>,
    pub(in crate::plan::execution) string_function_functions:
        Vec<ExecutableFunction<StringFunctionReturn>>,
    pub(in crate::plan::execution) bit_array_function_functions:
        Vec<ExecutableFunction<BitArrayFunctionReturn>>,
    pub(in crate::plan::execution) utf_codepoint_function_functions:
        Vec<ExecutableFunction<UtfCodepointFunctionReturn>>,
    pub(in crate::plan::execution) custom_function_functions:
        Vec<ExecutableFunction<CustomFunctionReturn>>,
    pub(in crate::plan::execution) bool_function_functions:
        Vec<ExecutableFunction<BoolFunctionReturn>>,
    pub(in crate::plan::execution) nil_function_functions:
        Vec<ExecutableFunction<NilFunctionReturn>>,
    pub(in crate::plan::execution) tuple_function_functions:
        Vec<ExecutableFunction<TupleFunctionReturn>>,
    pub(in crate::plan::execution) generic_function_functions:
        Vec<ExecutableFunction<GenericFunctionReturn>>,
    pub(in crate::plan::execution) never_function_functions:
        Vec<ExecutableFunction<NeverFunctionReturn>>,
    pub(in crate::plan::execution) parameter_list_function_functions:
        Vec<ExecutableFunction<ListFunctionReturn>>,
    pub(in crate::plan::execution) parameter_list_list_function_functions:
        Vec<ExecutableFunction<ListFunctionReturn>>,
    pub(in crate::plan::execution) int_list_function_functions:
        Vec<ExecutableFunction<ListFunctionReturn>>,
    pub(in crate::plan::execution) string_list_function_functions:
        Vec<ExecutableFunction<ListFunctionReturn>>,
    pub(in crate::plan::execution) bit_array_list_function_functions:
        Vec<ExecutableFunction<ListFunctionReturn>>,
    pub(in crate::plan::execution) utf_codepoint_list_function_functions:
        Vec<ExecutableFunction<ListFunctionReturn>>,
    pub(in crate::plan::execution) custom_list_function_functions:
        Vec<ExecutableFunction<ListFunctionReturn>>,
    pub(in crate::plan::execution) float_list_function_functions:
        Vec<ExecutableFunction<ListFunctionReturn>>,
    pub(in crate::plan::execution) bool_list_function_functions:
        Vec<ExecutableFunction<ListFunctionReturn>>,
    pub(in crate::plan::execution) nil_list_function_functions:
        Vec<ExecutableFunction<ListFunctionReturn>>,
    pub(in crate::plan::execution) tuple_list_function_functions:
        Vec<ExecutableFunction<ListFunctionReturn>>,
    pub(in crate::plan::execution) list_list_function_functions:
        Vec<ExecutableFunction<ListFunctionReturn>>,
    pub(in crate::plan::execution) function_list_function_functions:
        Vec<ExecutableFunction<ListFunctionReturn>>,
    pub(in crate::plan::execution) function_function_functions:
        Vec<ExecutableFunction<FunctionFunctionReturn>>,
}
