use super::{
    BitArrayReturn, BoolReturn, CustomReturn, FloatReturn, IntReturn, NeverReturn, NilReturn,
    StringReturn, TupleReturn, UtfCodepointReturn,
};
use crate::plan::execution::ExecutableFunction;

pub(in crate::plan::execution) struct ValueFunctionTables {
    pub(in crate::plan::execution) never_functions: Vec<ExecutableFunction<NeverReturn>>,
    pub(in crate::plan::execution) int_functions: Vec<ExecutableFunction<IntReturn>>,
    pub(in crate::plan::execution) float_functions: Vec<ExecutableFunction<FloatReturn>>,
    pub(in crate::plan::execution) string_functions: Vec<ExecutableFunction<StringReturn>>,
    pub(in crate::plan::execution) bit_array_functions: Vec<ExecutableFunction<BitArrayReturn>>,
    pub(in crate::plan::execution) utf_codepoint_functions:
        Vec<ExecutableFunction<UtfCodepointReturn>>,
    pub(in crate::plan::execution) custom_functions: Vec<ExecutableFunction<CustomReturn>>,
    pub(in crate::plan::execution) bool_functions: Vec<ExecutableFunction<BoolReturn>>,
    pub(in crate::plan::execution) nil_functions: Vec<ExecutableFunction<NilReturn>>,
    pub(in crate::plan::execution) tuple_functions: Vec<ExecutableFunction<TupleReturn>>,
}
