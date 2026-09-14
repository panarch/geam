use super::{
    BitArrayListFunctionId, BoolListFunctionId, CustomListFunctionId,
    ExecutionBitArrayListFunctionBody, ExecutionBoolListFunctionBody,
    ExecutionCustomListFunctionBody, ExecutionExternalListFunctionBody,
    ExecutionFloatListFunctionBody, ExecutionFunctionListFunctionBody,
    ExecutionIntListFunctionBody, ExecutionListListFunctionBody, ExecutionNilListFunctionBody,
    ExecutionParameterListFunctionBody, ExecutionParameterListListFunctionBody,
    ExecutionStringListFunctionBody, ExecutionTupleListFunctionBody,
    ExecutionUtfCodepointListFunctionBody, ExternalListFunctionId, FloatListFunctionId,
    FunctionListFunctionId, IntListFunctionId, ListListFunctionId, NilListFunctionId,
    ParameterListFunctionId, ParameterListListFunctionId, StringListFunctionId,
    TupleListFunctionId, UtfCodepointListFunctionId,
};
use crate::plan::execution::explain::{Explain, ExplainContext};
use crate::plan::execution::function::{ExecutionFunction, ExecutionProfile, write_table};
use crate::plan::execution::prepared::rust::{Emit, Rust};
use crate::plan::execution::storage::Table;
use std::convert::Infallible;

pub struct ListFunctionTables<Profile: ExecutionProfile> {
    pub parameter_list_functions: Table<(
        ParameterListFunctionId,
        ExecutionFunction<Profile, ExecutionParameterListFunctionBody<Profile>>,
    )>,
    pub int_list_functions: Table<(
        IntListFunctionId,
        ExecutionFunction<Profile, ExecutionIntListFunctionBody<Profile>>,
    )>,
    pub string_list_functions: Table<(
        StringListFunctionId,
        ExecutionFunction<Profile, ExecutionStringListFunctionBody<Profile>>,
    )>,
    pub bit_array_list_functions: Table<(
        BitArrayListFunctionId,
        ExecutionFunction<Profile, ExecutionBitArrayListFunctionBody<Profile>>,
    )>,
    pub utf_codepoint_list_functions: Table<(
        UtfCodepointListFunctionId,
        ExecutionFunction<Profile, ExecutionUtfCodepointListFunctionBody<Profile>>,
    )>,
    pub custom_list_functions: Table<(
        CustomListFunctionId,
        ExecutionFunction<Profile, ExecutionCustomListFunctionBody<Profile>>,
    )>,
    pub external_list_functions: Table<(
        ExternalListFunctionId,
        ExecutionFunction<Profile, ExecutionExternalListFunctionBody<Profile>>,
    )>,
    pub float_list_functions: Table<(
        FloatListFunctionId,
        ExecutionFunction<Profile, ExecutionFloatListFunctionBody<Profile>>,
    )>,
    pub bool_list_functions: Table<(
        BoolListFunctionId,
        ExecutionFunction<Profile, ExecutionBoolListFunctionBody<Profile>>,
    )>,
    pub nil_list_functions: Table<(
        NilListFunctionId,
        ExecutionFunction<Profile, ExecutionNilListFunctionBody<Profile>>,
    )>,
    pub tuple_list_functions: Table<(
        TupleListFunctionId,
        ExecutionFunction<Profile, ExecutionTupleListFunctionBody<Profile>>,
    )>,
    pub parameter_list_list_functions: Table<(
        ParameterListListFunctionId,
        ExecutionFunction<Profile, ExecutionParameterListListFunctionBody<Profile>>,
    )>,
    pub list_list_functions: Table<(
        ListListFunctionId,
        ExecutionFunction<Profile, ExecutionListListFunctionBody<Profile>>,
    )>,
    pub function_list_functions: Table<(
        FunctionListFunctionId,
        ExecutionFunction<Profile, ExecutionFunctionListFunctionBody<Profile>>,
    )>,
}

impl Explain for ListFunctionTables<Infallible> {
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
            "list.external",
            self.external_list_functions
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

impl<Profile: ExecutionProfile> Emit for ListFunctionTables<Profile>
where
    Table<(
        ParameterListFunctionId,
        ExecutionFunction<Profile, ExecutionParameterListFunctionBody<Profile>>,
    )>: Emit,
    Table<(
        IntListFunctionId,
        ExecutionFunction<Profile, ExecutionIntListFunctionBody<Profile>>,
    )>: Emit,
    Table<(
        StringListFunctionId,
        ExecutionFunction<Profile, ExecutionStringListFunctionBody<Profile>>,
    )>: Emit,
    Table<(
        BitArrayListFunctionId,
        ExecutionFunction<Profile, ExecutionBitArrayListFunctionBody<Profile>>,
    )>: Emit,
    Table<(
        UtfCodepointListFunctionId,
        ExecutionFunction<Profile, ExecutionUtfCodepointListFunctionBody<Profile>>,
    )>: Emit,
    Table<(
        CustomListFunctionId,
        ExecutionFunction<Profile, ExecutionCustomListFunctionBody<Profile>>,
    )>: Emit,
    Table<(
        ExternalListFunctionId,
        ExecutionFunction<Profile, ExecutionExternalListFunctionBody<Profile>>,
    )>: Emit,
    Table<(
        FloatListFunctionId,
        ExecutionFunction<Profile, ExecutionFloatListFunctionBody<Profile>>,
    )>: Emit,
    Table<(
        BoolListFunctionId,
        ExecutionFunction<Profile, ExecutionBoolListFunctionBody<Profile>>,
    )>: Emit,
    Table<(
        NilListFunctionId,
        ExecutionFunction<Profile, ExecutionNilListFunctionBody<Profile>>,
    )>: Emit,
    Table<(
        TupleListFunctionId,
        ExecutionFunction<Profile, ExecutionTupleListFunctionBody<Profile>>,
    )>: Emit,
    Table<(
        ParameterListListFunctionId,
        ExecutionFunction<Profile, ExecutionParameterListListFunctionBody<Profile>>,
    )>: Emit,
    Table<(
        ListListFunctionId,
        ExecutionFunction<Profile, ExecutionListListFunctionBody<Profile>>,
    )>: Emit,
    Table<(
        FunctionListFunctionId,
        ExecutionFunction<Profile, ExecutionFunctionListFunctionBody<Profile>>,
    )>: Emit,
{
    fn emit(&self, output: &mut Rust) {
        let Self {
            parameter_list_functions,
            int_list_functions,
            string_list_functions,
            bit_array_list_functions,
            utf_codepoint_list_functions,
            custom_list_functions,
            external_list_functions,
            float_list_functions,
            bool_list_functions,
            nil_list_functions,
            tuple_list_functions,
            parameter_list_list_functions,
            list_list_functions,
            function_list_functions,
        } = self;
        output.structure(
            "function::ListFunctionTables",
            &[
                ("parameter_list_functions", parameter_list_functions),
                ("int_list_functions", int_list_functions),
                ("string_list_functions", string_list_functions),
                ("bit_array_list_functions", bit_array_list_functions),
                ("utf_codepoint_list_functions", utf_codepoint_list_functions),
                ("custom_list_functions", custom_list_functions),
                ("external_list_functions", external_list_functions),
                ("float_list_functions", float_list_functions),
                ("bool_list_functions", bool_list_functions),
                ("nil_list_functions", nil_list_functions),
                ("tuple_list_functions", tuple_list_functions),
                (
                    "parameter_list_list_functions",
                    parameter_list_list_functions,
                ),
                ("list_list_functions", list_list_functions),
                ("function_list_functions", function_list_functions),
            ],
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
