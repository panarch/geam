use super::{
    ExecutionBitArrayFunctionBody, ExecutionBoolFunctionBody, ExecutionCustomFunctionBody,
    ExecutionExternalFunctionBody, ExecutionFloatFunctionBody, ExecutionIntFunctionBody,
    ExecutionNilFunctionBody, ExecutionStringFunctionBody, ExecutionTupleFunctionBody,
    ExecutionUtfCodepointFunctionBody,
};
use crate::plan::execution::explain::{Explain, ExplainContext};
use crate::plan::execution::function::{
    ExecutionFunction, ExecutionNeverFunction, ExecutionProfile, write_table,
};
use std::convert::Infallible;

pub(in crate::plan::execution) struct ValueFunctionTables<Profile: ExecutionProfile> {
    pub(in crate::plan::execution) never_functions: Vec<ExecutionNeverFunction<Profile>>,
    pub(in crate::plan::execution) int_functions:
        Vec<ExecutionFunction<Profile, ExecutionIntFunctionBody<Profile>>>,
    pub(in crate::plan::execution) float_functions:
        Vec<ExecutionFunction<Profile, ExecutionFloatFunctionBody<Profile>>>,
    pub(in crate::plan::execution) string_functions:
        Vec<ExecutionFunction<Profile, ExecutionStringFunctionBody<Profile>>>,
    pub(in crate::plan::execution) bit_array_functions:
        Vec<ExecutionFunction<Profile, ExecutionBitArrayFunctionBody<Profile>>>,
    pub(in crate::plan::execution) utf_codepoint_functions:
        Vec<ExecutionFunction<Profile, ExecutionUtfCodepointFunctionBody<Profile>>>,
    pub(in crate::plan::execution) custom_functions:
        Vec<ExecutionFunction<Profile, ExecutionCustomFunctionBody<Profile>>>,
    pub(in crate::plan::execution) external_functions:
        Vec<ExecutionFunction<Profile, ExecutionExternalFunctionBody<Profile>>>,
    pub(in crate::plan::execution) bool_functions:
        Vec<ExecutionFunction<Profile, ExecutionBoolFunctionBody<Profile>>>,
    pub(in crate::plan::execution) nil_functions:
        Vec<ExecutionFunction<Profile, ExecutionNilFunctionBody<Profile>>>,
    pub(in crate::plan::execution) tuple_functions:
        Vec<ExecutionFunction<Profile, ExecutionTupleFunctionBody<Profile>>>,
}

impl Explain for ValueFunctionTables<Infallible> {
    fn write_explanation(&self, context: &mut ExplainContext<'_, '_>) {
        write_table(context, "never", &self.never_functions);
        write_table(context, "int", &self.int_functions);
        write_table(context, "float", &self.float_functions);
        write_table(context, "string", &self.string_functions);
        write_table(context, "bit_array", &self.bit_array_functions);
        write_table(context, "utf_codepoint", &self.utf_codepoint_functions);
        write_table(context, "custom", &self.custom_functions);
        write_table(context, "external", &self.external_functions);
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
            context.write(&plan.program.functions.value_returns);
        });
    }
}
