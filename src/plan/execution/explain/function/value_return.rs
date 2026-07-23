use super::super::super::{ExecutionPlan, FunctionTables};
use super::table::write_table;

pub(super) fn write_value_return_tables(
    output: &mut String,
    plan: &ExecutionPlan,
    functions: &FunctionTables,
) {
    write_table(
        output,
        plan,
        "never",
        &functions.value_returns.never_functions,
    );
    write_table(output, plan, "int", &functions.value_returns.int_functions);
    write_table(
        output,
        plan,
        "float",
        &functions.value_returns.float_functions,
    );
    write_table(
        output,
        plan,
        "string",
        &functions.value_returns.string_functions,
    );
    write_table(
        output,
        plan,
        "bit_array",
        &functions.value_returns.bit_array_functions,
    );
    write_table(
        output,
        plan,
        "utf_codepoint",
        &functions.value_returns.utf_codepoint_functions,
    );
    write_table(
        output,
        plan,
        "custom",
        &functions.value_returns.custom_functions,
    );
    write_table(
        output,
        plan,
        "bool",
        &functions.value_returns.bool_functions,
    );
    write_table(output, plan, "nil", &functions.value_returns.nil_functions);
    write_table(
        output,
        plan,
        "tuple",
        &functions.value_returns.tuple_functions,
    );
}

#[cfg(test)]
mod tests {
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
        super::super::super::assert_rendered(source, expected, |plan, output| {
            super::write_value_return_tables(output, plan, &plan.functions);
        });
    }
}
