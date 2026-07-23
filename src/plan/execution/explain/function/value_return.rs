use super::super::super::{ExecutionPlan, FunctionTables};
use super::table::write_table;

pub(super) fn write_value_return_tables(
    output: &mut String,
    plan: &ExecutionPlan,
    functions: &FunctionTables,
) {
    write_table(output, plan, "never", &functions.never_functions);
    write_table(output, plan, "int", &functions.int_functions);
    write_table(output, plan, "float", &functions.float_functions);
    write_table(output, plan, "string", &functions.string_functions);
    write_table(output, plan, "bit_array", &functions.bit_array_functions);
    write_table(
        output,
        plan,
        "utf_codepoint",
        &functions.utf_codepoint_functions,
    );
    write_table(output, plan, "custom", &functions.custom_functions);
    write_table(output, plan, "bool", &functions.bool_functions);
    write_table(output, plan, "nil", &functions.nil_functions);
    write_table(output, plan, "tuple", &functions.tuple_functions);
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
        let typed = crate::compile_typed_module("main", "main.gleam", source)
            .expect("source should compile");
        let module_plan = crate::plan_module(typed).expect("source should plan");
        let plan = crate::ExecutionPlan::from_module_plan(module_plan);
        let mut output = String::new();

        super::write_value_return_tables(&mut output, &plan, &plan.functions);

        assert_eq!(
            output,
            concat!(
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
            ),
        );
    }
}
