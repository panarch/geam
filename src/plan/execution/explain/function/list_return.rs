use super::super::super::{ExecutionPlan, FunctionTables};
use super::table::write_table;

pub(super) fn write_list_return_tables(
    output: &mut String,
    plan: &ExecutionPlan,
    functions: &FunctionTables,
) {
    write_table(
        output,
        plan,
        "list.parameter",
        functions
            .list_returns
            .parameter_list_functions
            .iter()
            .map(|(_, function)| function),
    );
    write_table(
        output,
        plan,
        "list.int",
        functions
            .list_returns
            .int_list_functions
            .iter()
            .map(|(_, function)| function),
    );
    write_table(
        output,
        plan,
        "list.string",
        functions
            .list_returns
            .string_list_functions
            .iter()
            .map(|(_, function)| function),
    );
    write_table(
        output,
        plan,
        "list.bit_array",
        functions
            .list_returns
            .bit_array_list_functions
            .iter()
            .map(|(_, function)| function),
    );
    write_table(
        output,
        plan,
        "list.utf_codepoint",
        functions
            .list_returns
            .utf_codepoint_list_functions
            .iter()
            .map(|(_, function)| function),
    );
    write_table(
        output,
        plan,
        "list.custom",
        functions
            .list_returns
            .custom_list_functions
            .iter()
            .map(|(_, function)| function),
    );
    write_table(
        output,
        plan,
        "list.float",
        functions
            .list_returns
            .float_list_functions
            .iter()
            .map(|(_, function)| function),
    );
    write_table(
        output,
        plan,
        "list.bool",
        functions
            .list_returns
            .bool_list_functions
            .iter()
            .map(|(_, function)| function),
    );
    write_table(
        output,
        plan,
        "list.nil",
        functions
            .list_returns
            .nil_list_functions
            .iter()
            .map(|(_, function)| function),
    );
    write_table(
        output,
        plan,
        "list.tuple",
        functions
            .list_returns
            .tuple_list_functions
            .iter()
            .map(|(_, function)| function),
    );
    write_table(
        output,
        plan,
        "list.parameter_list",
        functions
            .list_returns
            .parameter_list_list_functions
            .iter()
            .map(|(_, function)| function),
    );
    write_table(
        output,
        plan,
        "list.list",
        functions
            .list_returns
            .list_list_functions
            .iter()
            .map(|(_, function)| function),
    );
    write_table(
        output,
        plan,
        "list.function",
        functions
            .list_returns
            .function_list_functions
            .iter()
            .map(|(_, function)| function),
    );
}

#[cfg(test)]
mod tests {
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
        super::super::super::assert_rendered(source, expected, |plan, output| {
            super::write_list_return_tables(output, plan, &plan.functions);
        });
    }
}
