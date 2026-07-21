use super::super::ExecutionPlan;
use super::super::function::ExecutableFunction;
use super::super::table::FunctionTables;
use super::graph::ExplainedGraph;
use super::label::FunctionLabel;

pub(super) fn write_function_tables(
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

    write_table(
        output,
        plan,
        "function.int",
        &functions.int_function_functions,
    );
    write_table(
        output,
        plan,
        "function.float",
        &functions.float_function_functions,
    );
    write_table(
        output,
        plan,
        "function.string",
        &functions.string_function_functions,
    );
    write_table(
        output,
        plan,
        "function.bit_array",
        &functions.bit_array_function_functions,
    );
    write_table(
        output,
        plan,
        "function.utf_codepoint",
        &functions.utf_codepoint_function_functions,
    );
    write_table(
        output,
        plan,
        "function.custom",
        &functions.custom_function_functions,
    );
    write_table(
        output,
        plan,
        "function.bool",
        &functions.bool_function_functions,
    );
    write_table(
        output,
        plan,
        "function.nil",
        &functions.nil_function_functions,
    );
    write_table(
        output,
        plan,
        "function.tuple",
        &functions.tuple_function_functions,
    );
    write_table(
        output,
        plan,
        "function.generic",
        &functions.generic_function_functions,
    );
    write_table(
        output,
        plan,
        "function.never",
        &functions.never_function_functions,
    );

    write_table(
        output,
        plan,
        "function.list.parameter",
        &functions.parameter_list_function_functions,
    );
    write_table(
        output,
        plan,
        "function.list.parameter_list",
        &functions.parameter_list_list_function_functions,
    );
    write_table(
        output,
        plan,
        "function.list.int",
        &functions.int_list_function_functions,
    );
    write_table(
        output,
        plan,
        "function.list.string",
        &functions.string_list_function_functions,
    );
    write_table(
        output,
        plan,
        "function.list.bit_array",
        &functions.bit_array_list_function_functions,
    );
    write_table(
        output,
        plan,
        "function.list.utf_codepoint",
        &functions.utf_codepoint_list_function_functions,
    );
    write_table(
        output,
        plan,
        "function.list.custom",
        &functions.custom_list_function_functions,
    );
    write_table(
        output,
        plan,
        "function.list.float",
        &functions.float_list_function_functions,
    );
    write_table(
        output,
        plan,
        "function.list.bool",
        &functions.bool_list_function_functions,
    );
    write_table(
        output,
        plan,
        "function.list.nil",
        &functions.nil_list_function_functions,
    );
    write_table(
        output,
        plan,
        "function.list.tuple",
        &functions.tuple_list_function_functions,
    );
    write_table(
        output,
        plan,
        "function.list.list",
        &functions.list_list_function_functions,
    );
    write_table(
        output,
        plan,
        "function.list.function",
        &functions.function_list_function_functions,
    );
    write_table(
        output,
        plan,
        "function.function",
        &functions.function_function_functions,
    );
}

fn write_table<'a, Return, Functions>(
    output: &mut String,
    plan: &ExecutionPlan,
    family: &'static str,
    functions: Functions,
) where
    Return: ExplainedGraph + 'a,
    Functions: IntoIterator<Item = &'a ExecutableFunction<Return>>,
{
    for (index, function) in functions.into_iter().enumerate() {
        output.push_str("\nfunction ");
        FunctionLabel::new(family, index).push_to(output);
        output.push('\n');
        let graph = function.graph();
        graph.write_complete(
            output,
            plan,
            family,
            graph.entry_params(function.entry()),
            graph.entry_captures(function.entry()),
        );
    }
}

#[cfg(test)]
mod tests {
    use crate::ExecutionPlan;

    #[test]
    fn formats_every_function_table_group_in_storage_order() {
        let fixtures = [
            include_str!("../../../../tests/fixtures/explain/value_return_tables.gleam"),
            include_str!("../../../../tests/fixtures/explain/list_return_tables.gleam"),
            include_str!("../../../../tests/fixtures/explain/function_return_tables.gleam"),
            include_str!("../../../../tests/fixtures/explain/list_returning_function_tables.gleam"),
            include_str!("../../../../tests/fixtures/explain/return_table_group_order.gleam"),
        ];

        for source in fixtures {
            assert_eq!(
                execution_plan(source).explain().to_string(),
                expected_explanation(source),
            );
        }
    }

    fn expected_explanation(source: &str) -> String {
        let (_, comments) = source
            .split_once("\n// geam:explain\n")
            .expect("explain fixture should contain an expected output block");
        let mut expected = String::new();

        for line in comments.lines() {
            let comment = line
                .strip_prefix("//")
                .expect("expected output lines should be comments");
            expected.push_str(comment.strip_prefix(' ').unwrap_or(comment));
            expected.push('\n');
        }

        expected
    }

    fn execution_plan(source: &str) -> ExecutionPlan {
        let typed = crate::compile_typed_module("main", "main.gleam", source)
            .expect("source should compile");
        let module_plan = crate::plan_module(typed).expect("source should plan");
        ExecutionPlan::from_module_plan(module_plan)
    }
}
