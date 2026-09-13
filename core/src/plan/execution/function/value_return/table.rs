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
use crate::plan::execution::prepared::rust::{Emit, Rust};
use crate::plan::execution::storage::Table;
use std::convert::Infallible;

pub struct ValueFunctionTables<Profile: ExecutionProfile> {
    pub never_functions: Table<ExecutionNeverFunction<Profile>>,
    pub int_functions: Table<ExecutionFunction<Profile, ExecutionIntFunctionBody<Profile>>>,
    pub float_functions: Table<ExecutionFunction<Profile, ExecutionFloatFunctionBody<Profile>>>,
    pub string_functions: Table<ExecutionFunction<Profile, ExecutionStringFunctionBody<Profile>>>,
    pub bit_array_functions:
        Table<ExecutionFunction<Profile, ExecutionBitArrayFunctionBody<Profile>>>,
    pub utf_codepoint_functions:
        Table<ExecutionFunction<Profile, ExecutionUtfCodepointFunctionBody<Profile>>>,
    pub custom_functions: Table<ExecutionFunction<Profile, ExecutionCustomFunctionBody<Profile>>>,
    pub external_functions:
        Table<ExecutionFunction<Profile, ExecutionExternalFunctionBody<Profile>>>,
    pub bool_functions: Table<ExecutionFunction<Profile, ExecutionBoolFunctionBody<Profile>>>,
    pub nil_functions: Table<ExecutionFunction<Profile, ExecutionNilFunctionBody<Profile>>>,
    pub tuple_functions: Table<ExecutionFunction<Profile, ExecutionTupleFunctionBody<Profile>>>,
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

impl<Profile: ExecutionProfile> Emit for ValueFunctionTables<Profile>
where
    Table<ExecutionNeverFunction<Profile>>: Emit,
    Table<ExecutionFunction<Profile, ExecutionIntFunctionBody<Profile>>>: Emit,
    Table<ExecutionFunction<Profile, ExecutionFloatFunctionBody<Profile>>>: Emit,
    Table<ExecutionFunction<Profile, ExecutionStringFunctionBody<Profile>>>: Emit,
    Table<ExecutionFunction<Profile, ExecutionBitArrayFunctionBody<Profile>>>: Emit,
    Table<ExecutionFunction<Profile, ExecutionUtfCodepointFunctionBody<Profile>>>: Emit,
    Table<ExecutionFunction<Profile, ExecutionCustomFunctionBody<Profile>>>: Emit,
    Table<ExecutionFunction<Profile, ExecutionExternalFunctionBody<Profile>>>: Emit,
    Table<ExecutionFunction<Profile, ExecutionBoolFunctionBody<Profile>>>: Emit,
    Table<ExecutionFunction<Profile, ExecutionNilFunctionBody<Profile>>>: Emit,
    Table<ExecutionFunction<Profile, ExecutionTupleFunctionBody<Profile>>>: Emit,
{
    fn emit(&self, output: &mut Rust) {
        let Self {
            never_functions,
            int_functions,
            float_functions,
            string_functions,
            bit_array_functions,
            utf_codepoint_functions,
            custom_functions,
            external_functions,
            bool_functions,
            nil_functions,
            tuple_functions,
        } = self;
        output.structure(
            "function::ValueFunctionTables",
            &[
                ("never_functions", never_functions),
                ("int_functions", int_functions),
                ("float_functions", float_functions),
                ("string_functions", string_functions),
                ("bit_array_functions", bit_array_functions),
                ("utf_codepoint_functions", utf_codepoint_functions),
                ("custom_functions", custom_functions),
                ("external_functions", external_functions),
                ("bool_functions", bool_functions),
                ("nil_functions", nil_functions),
                ("tuple_functions", tuple_functions),
            ],
        );
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
