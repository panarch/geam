use super::{
    BitArrayListFunctionBody, BitArrayListFunctionId, BoolListFunctionBody, BoolListFunctionId,
    CustomListFunctionBody, CustomListFunctionId, FloatListFunctionBody, FloatListFunctionId,
    FunctionListFunctionBody, FunctionListFunctionId, IntListFunctionBody, IntListFunctionId,
    ListListFunctionBody, ListListFunctionId, NilListFunctionBody, NilListFunctionId,
    ParameterListFunctionBody, ParameterListFunctionId, ParameterListListFunctionBody,
    ParameterListListFunctionId, StringListFunctionBody, StringListFunctionId,
    TupleListFunctionBody, TupleListFunctionId, UtfCodepointListFunctionBody,
    UtfCodepointListFunctionId,
};
use crate::plan::execution::explain::{Explain, ExplainContext};
use crate::plan::execution::function::ExecutableFunction;
use crate::plan::execution::function::write_table;

pub(in crate::plan::execution) struct ListFunctionTables {
    pub(in crate::plan::execution) parameter_list_functions: Vec<(
        ParameterListFunctionId,
        ExecutableFunction<ParameterListFunctionBody>,
    )>,
    pub(in crate::plan::execution) int_list_functions:
        Vec<(IntListFunctionId, ExecutableFunction<IntListFunctionBody>)>,
    pub(in crate::plan::execution) string_list_functions: Vec<(
        StringListFunctionId,
        ExecutableFunction<StringListFunctionBody>,
    )>,
    pub(in crate::plan::execution) bit_array_list_functions: Vec<(
        BitArrayListFunctionId,
        ExecutableFunction<BitArrayListFunctionBody>,
    )>,
    pub(in crate::plan::execution) utf_codepoint_list_functions: Vec<(
        UtfCodepointListFunctionId,
        ExecutableFunction<UtfCodepointListFunctionBody>,
    )>,
    pub(in crate::plan::execution) custom_list_functions: Vec<(
        CustomListFunctionId,
        ExecutableFunction<CustomListFunctionBody>,
    )>,
    pub(in crate::plan::execution) float_list_functions: Vec<(
        FloatListFunctionId,
        ExecutableFunction<FloatListFunctionBody>,
    )>,
    pub(in crate::plan::execution) bool_list_functions:
        Vec<(BoolListFunctionId, ExecutableFunction<BoolListFunctionBody>)>,
    pub(in crate::plan::execution) nil_list_functions:
        Vec<(NilListFunctionId, ExecutableFunction<NilListFunctionBody>)>,
    pub(in crate::plan::execution) tuple_list_functions: Vec<(
        TupleListFunctionId,
        ExecutableFunction<TupleListFunctionBody>,
    )>,
    pub(in crate::plan::execution) parameter_list_list_functions: Vec<(
        ParameterListListFunctionId,
        ExecutableFunction<ParameterListListFunctionBody>,
    )>,
    pub(in crate::plan::execution) list_list_functions:
        Vec<(ListListFunctionId, ExecutableFunction<ListListFunctionBody>)>,
    pub(in crate::plan::execution) function_list_functions: Vec<(
        FunctionListFunctionId,
        ExecutableFunction<FunctionListFunctionBody>,
    )>,
}

impl Explain for ListFunctionTables {
    fn write_explanation(&self, context: &mut ExplainContext<'_, '_>) {
        write_table(
            context,
            "list.parameter",
            self.parameter_list_functions
                .iter()
                .map(|(_, function)| function),
        );
        write_table(
            context,
            "list.int",
            self.int_list_functions.iter().map(|(_, function)| function),
        );
        write_table(
            context,
            "list.string",
            self.string_list_functions
                .iter()
                .map(|(_, function)| function),
        );
        write_table(
            context,
            "list.bit_array",
            self.bit_array_list_functions
                .iter()
                .map(|(_, function)| function),
        );
        write_table(
            context,
            "list.utf_codepoint",
            self.utf_codepoint_list_functions
                .iter()
                .map(|(_, function)| function),
        );
        write_table(
            context,
            "list.custom",
            self.custom_list_functions
                .iter()
                .map(|(_, function)| function),
        );
        write_table(
            context,
            "list.float",
            self.float_list_functions
                .iter()
                .map(|(_, function)| function),
        );
        write_table(
            context,
            "list.bool",
            self.bool_list_functions
                .iter()
                .map(|(_, function)| function),
        );
        write_table(
            context,
            "list.nil",
            self.nil_list_functions.iter().map(|(_, function)| function),
        );
        write_table(
            context,
            "list.tuple",
            self.tuple_list_functions
                .iter()
                .map(|(_, function)| function),
        );
        write_table(
            context,
            "list.parameter_list",
            self.parameter_list_list_functions
                .iter()
                .map(|(_, function)| function),
        );
        write_table(
            context,
            "list.list",
            self.list_list_functions
                .iter()
                .map(|(_, function)| function),
        );
        write_table(
            context,
            "list.function",
            self.function_list_functions
                .iter()
                .map(|(_, function)| function),
        );
    }
}

#[cfg(test)]
mod explain_tests {
    use crate::plan::execution::explain;

    #[test]
    fn writes_list_return_families_in_storage_order() {
        let source = r#"
fn ints() -> List(Int) { [] }
pub fn main() -> List(String) {
  let _ = ints()
  []
}
"#;
        let expected = concat!(
            "\nfunction list.int#0\n",
            "  entry b0 params=[] captures=[]\n",
            "  block b0 params=[]\n",
            "    %list.int#0:shape#1(list_type#1) = list.int[type#1] value elements=[]\n",
            "    return %list.int#0\n",
            "\nfunction list.string#0\n",
            "  entry b0 params=[] captures=[]\n",
            "  block b0 params=[]\n",
            "    %list.int#0:shape#1(list_type#1) = list.int[type#1] call list.int#0 args=[]\n",
            "    %list.string#0:shape#3(list_type#0) = list.string[type#0] value elements=[]\n",
            "    return %list.string#0\n",
        );

        assert_explanation(source, expected);
    }

    fn assert_explanation(source: &str, expected: &str) {
        explain::assert_rendered(source, expected, |plan, output| {
            let mut context = explain::ExplainContext::new(plan, output);
            context.write(&plan.program.functions.list_returns);
        });
    }
}
