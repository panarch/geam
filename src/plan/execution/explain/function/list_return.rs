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
            .parameter_list_functions
            .iter()
            .map(|(_, function)| function),
    );
    write_table(
        output,
        plan,
        "list.int",
        functions
            .int_list_functions
            .iter()
            .map(|(_, function)| function),
    );
    write_table(
        output,
        plan,
        "list.string",
        functions
            .string_list_functions
            .iter()
            .map(|(_, function)| function),
    );
    write_table(
        output,
        plan,
        "list.bit_array",
        functions
            .bit_array_list_functions
            .iter()
            .map(|(_, function)| function),
    );
    write_table(
        output,
        plan,
        "list.utf_codepoint",
        functions
            .utf_codepoint_list_functions
            .iter()
            .map(|(_, function)| function),
    );
    write_table(
        output,
        plan,
        "list.custom",
        functions
            .custom_list_functions
            .iter()
            .map(|(_, function)| function),
    );
    write_table(
        output,
        plan,
        "list.float",
        functions
            .float_list_functions
            .iter()
            .map(|(_, function)| function),
    );
    write_table(
        output,
        plan,
        "list.bool",
        functions
            .bool_list_functions
            .iter()
            .map(|(_, function)| function),
    );
    write_table(
        output,
        plan,
        "list.nil",
        functions
            .nil_list_functions
            .iter()
            .map(|(_, function)| function),
    );
    write_table(
        output,
        plan,
        "list.tuple",
        functions
            .tuple_list_functions
            .iter()
            .map(|(_, function)| function),
    );
    write_table(
        output,
        plan,
        "list.parameter_list",
        functions
            .parameter_list_list_functions
            .iter()
            .map(|(_, function)| function),
    );
    write_table(
        output,
        plan,
        "list.list",
        functions
            .list_list_functions
            .iter()
            .map(|(_, function)| function),
    );
    write_table(
        output,
        plan,
        "list.function",
        functions
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
        let typed = crate::compile_typed_module("main", "main.gleam", source)
            .expect("source should compile");
        let module_plan = crate::plan_module(typed).expect("source should plan");
        let plan = crate::ExecutionPlan::from_module_plan(module_plan);
        let mut output = String::new();

        super::write_list_return_tables(&mut output, &plan, &plan.functions);

        assert_eq!(
            output,
            concat!(
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
            ),
        );
    }
}
