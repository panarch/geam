use super::{
    BitArrayFunctionReturn, BoolFunctionReturn, CustomFunctionReturn, FloatFunctionReturn,
    FunctionFunctionReturn, GenericFunctionReturn, IntFunctionReturn, ListFunctionReturn,
    NeverFunctionReturn, NilFunctionReturn, StringFunctionReturn, TupleFunctionReturn,
    UtfCodepointFunctionReturn,
};
use crate::plan::execution::ExecutableFunction;
use crate::plan::execution::explain::{Explain, ExplainContext};
use crate::plan::execution::function::write_table;

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

impl Explain for FunctionFunctionTables {
    fn write_explanation(&self, context: &mut ExplainContext<'_, '_>) {
        write_table(context, "function.int", &self.int_function_functions);
        write_table(context, "function.float", &self.float_function_functions);
        write_table(context, "function.string", &self.string_function_functions);
        write_table(
            context,
            "function.bit_array",
            &self.bit_array_function_functions,
        );
        write_table(
            context,
            "function.utf_codepoint",
            &self.utf_codepoint_function_functions,
        );
        write_table(context, "function.custom", &self.custom_function_functions);
        write_table(context, "function.bool", &self.bool_function_functions);
        write_table(context, "function.nil", &self.nil_function_functions);
        write_table(context, "function.tuple", &self.tuple_function_functions);
        write_table(
            context,
            "function.generic",
            &self.generic_function_functions,
        );
        write_table(context, "function.never", &self.never_function_functions);
        write_table(
            context,
            "function.list.parameter",
            &self.parameter_list_function_functions,
        );
        write_table(
            context,
            "function.list.parameter_list",
            &self.parameter_list_list_function_functions,
        );
        write_table(
            context,
            "function.list.int",
            &self.int_list_function_functions,
        );
        write_table(
            context,
            "function.list.string",
            &self.string_list_function_functions,
        );
        write_table(
            context,
            "function.list.bit_array",
            &self.bit_array_list_function_functions,
        );
        write_table(
            context,
            "function.list.utf_codepoint",
            &self.utf_codepoint_list_function_functions,
        );
        write_table(
            context,
            "function.list.custom",
            &self.custom_list_function_functions,
        );
        write_table(
            context,
            "function.list.float",
            &self.float_list_function_functions,
        );
        write_table(
            context,
            "function.list.bool",
            &self.bool_list_function_functions,
        );
        write_table(
            context,
            "function.list.nil",
            &self.nil_list_function_functions,
        );
        write_table(
            context,
            "function.list.tuple",
            &self.tuple_list_function_functions,
        );
        write_table(
            context,
            "function.list.list",
            &self.list_list_function_functions,
        );
        write_table(
            context,
            "function.list.function",
            &self.function_list_function_functions,
        );
        write_table(
            context,
            "function.function",
            &self.function_function_functions,
        );
    }
}

#[cfg(test)]
mod explain_tests {
    use crate::plan::execution::explain;

    #[test]
    fn writes_function_return_families_in_storage_order() {
        let source = r#"
fn int_function() -> fn() -> Int { fn() { 1 } }

pub fn main() -> fn() -> Bool {
  let _ = int_function()
  fn() { True }
}
"#;
        let expected = concat!(
            "\nfunction function.int#0\n",
            "  entry b0 params=[] captures=[]\n",
            "  block b0 params=[]\n",
            "    %function.int#0:shape#1(fn() -> Int) = function[Int] closure ",
            "target=int#0 captures=[]\n",
            "    return %function.int#0\n",
            "\nfunction function.bool#0\n",
            "  entry b0 params=[] captures=[]\n",
            "  block b0 params=[]\n",
            "    %function.int#0:shape#1(fn() -> Int) = function[Int] call ",
            "function.int#0 args=[]\n",
            "    %function.bool#0:shape#3(fn() -> Bool) = function[Bool] closure ",
            "target=bool#0 captures=[]\n",
            "    return %function.bool#0\n",
        );

        assert_explanation(source, expected);
    }

    fn assert_explanation(source: &str, expected: &str) {
        explain::assert_rendered(source, expected, |plan, output| {
            let mut context = explain::ExplainContext::new(plan, output);
            context.write(&plan.functions.function_returns);
        });
    }
}
