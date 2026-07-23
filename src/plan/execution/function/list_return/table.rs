use super::{
    BitArrayListFunctionId, BitArrayListReturn, BoolListFunctionId, BoolListReturn,
    CustomListFunctionId, CustomListReturn, FloatListFunctionId, FloatListReturn,
    FunctionListFunctionId, FunctionListReturn, IntListFunctionId, IntListReturn,
    ListListFunctionId, ListListReturn, NilListFunctionId, NilListReturn, ParameterListFunctionId,
    ParameterListListFunctionId, ParameterListListReturn, ParameterListReturn,
    StringListFunctionId, StringListReturn, TupleListFunctionId, TupleListReturn,
    UtfCodepointListFunctionId, UtfCodepointListReturn,
};
use crate::plan::execution::ExecutableFunction;

pub(in crate::plan::execution) struct ListFunctionTables {
    pub(in crate::plan::execution) parameter_list_functions: Vec<(
        ParameterListFunctionId,
        ExecutableFunction<ParameterListReturn>,
    )>,
    pub(in crate::plan::execution) int_list_functions:
        Vec<(IntListFunctionId, ExecutableFunction<IntListReturn>)>,
    pub(in crate::plan::execution) string_list_functions:
        Vec<(StringListFunctionId, ExecutableFunction<StringListReturn>)>,
    pub(in crate::plan::execution) bit_array_list_functions: Vec<(
        BitArrayListFunctionId,
        ExecutableFunction<BitArrayListReturn>,
    )>,
    pub(in crate::plan::execution) utf_codepoint_list_functions: Vec<(
        UtfCodepointListFunctionId,
        ExecutableFunction<UtfCodepointListReturn>,
    )>,
    pub(in crate::plan::execution) custom_list_functions:
        Vec<(CustomListFunctionId, ExecutableFunction<CustomListReturn>)>,
    pub(in crate::plan::execution) float_list_functions:
        Vec<(FloatListFunctionId, ExecutableFunction<FloatListReturn>)>,
    pub(in crate::plan::execution) bool_list_functions:
        Vec<(BoolListFunctionId, ExecutableFunction<BoolListReturn>)>,
    pub(in crate::plan::execution) nil_list_functions:
        Vec<(NilListFunctionId, ExecutableFunction<NilListReturn>)>,
    pub(in crate::plan::execution) tuple_list_functions:
        Vec<(TupleListFunctionId, ExecutableFunction<TupleListReturn>)>,
    pub(in crate::plan::execution) parameter_list_list_functions: Vec<(
        ParameterListListFunctionId,
        ExecutableFunction<ParameterListListReturn>,
    )>,
    pub(in crate::plan::execution) list_list_functions:
        Vec<(ListListFunctionId, ExecutableFunction<ListListReturn>)>,
    pub(in crate::plan::execution) function_list_functions: Vec<(
        FunctionListFunctionId,
        ExecutableFunction<FunctionListReturn>,
    )>,
}
