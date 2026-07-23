use super::{
    BitArrayReturn, BoolReturn, CustomReturn, FloatReturn, IntReturn, NeverReturn, NilReturn,
    StringReturn, TupleReturn, UtfCodepointReturn,
};
use crate::plan::execution::ExecutableFunction;
use crate::plan::execution::explain::{Explain, ExplainContext};
use crate::plan::execution::function::write_table;

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

impl Explain for ValueFunctionTables {
    fn write_explanation(&self, context: &mut ExplainContext<'_, '_>) {
        write_table(context, "never", &self.never_functions);
        write_table(context, "int", &self.int_functions);
        write_table(context, "float", &self.float_functions);
        write_table(context, "string", &self.string_functions);
        write_table(context, "bit_array", &self.bit_array_functions);
        write_table(context, "utf_codepoint", &self.utf_codepoint_functions);
        write_table(context, "custom", &self.custom_functions);
        write_table(context, "bool", &self.bool_functions);
        write_table(context, "nil", &self.nil_functions);
        write_table(context, "tuple", &self.tuple_functions);
    }
}

#[cfg(test)]
mod explain_tests {
    use crate::plan::execution::explain;

    #[test]
    fn writes_value_return_families_in_storage_order() {
        let source = r#"
fn truth() { True }
pub fn main() {
  let _ = truth()
  1
}
"#;
        let expected = concat!(
            "\nfunction int#0\n",
            "  entry b0 params=[] captures=[]\n",
            "  block b0 params=[]\n",
            "    %bool#0:shape#0(Bool) = bool.call bool#0 args=[]\n",
            "    %int#0:shape#1(Int) = int.value 1\n",
            "    return %int#0\n",
            "\nfunction bool#0\n",
            "  entry b0 params=[] captures=[]\n",
            "  block b0 params=[]\n",
            "    %bool#0:shape#0(Bool) = bool.value True\n",
            "    return %bool#0\n",
        );

        assert_explanation(source, expected);
    }

    fn assert_explanation(source: &str, expected: &str) {
        explain::assert_rendered(source, expected, |plan, output| {
            let mut context = explain::ExplainContext::new(plan, output);
            context.write(&plan.functions.value_returns);
        });
    }
}
