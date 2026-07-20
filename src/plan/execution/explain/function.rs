use super::super::function::ExecutableFunction;
use super::super::table::FunctionTables;
use super::label::FunctionLabel;
use super::return_graph::{ExplainedReturn, write_graph};

pub(super) fn write_function_tables(output: &mut String, functions: &FunctionTables) {
    write_table(output, "never", &functions.never_functions);
    write_table(output, "int", &functions.int_functions);
    write_table(output, "float", &functions.float_functions);
    write_table(output, "string", &functions.string_functions);
    write_table(output, "bit_array", &functions.bit_array_functions);
    write_table(output, "utf_codepoint", &functions.utf_codepoint_functions);
    write_table(output, "custom", &functions.custom_functions);
    write_table(output, "bool", &functions.bool_functions);
    write_table(output, "nil", &functions.nil_functions);
    write_table(output, "tuple", &functions.tuple_functions);

    write_table(
        output,
        "list.parameter",
        functions
            .parameter_list_functions
            .iter()
            .map(|(_, function)| function),
    );
    write_table(
        output,
        "list.int",
        functions
            .int_list_functions
            .iter()
            .map(|(_, function)| function),
    );
    write_table(
        output,
        "list.string",
        functions
            .string_list_functions
            .iter()
            .map(|(_, function)| function),
    );
    write_table(
        output,
        "list.bit_array",
        functions
            .bit_array_list_functions
            .iter()
            .map(|(_, function)| function),
    );
    write_table(
        output,
        "list.utf_codepoint",
        functions
            .utf_codepoint_list_functions
            .iter()
            .map(|(_, function)| function),
    );
    write_table(
        output,
        "list.custom",
        functions
            .custom_list_functions
            .iter()
            .map(|(_, function)| function),
    );
    write_table(
        output,
        "list.float",
        functions
            .float_list_functions
            .iter()
            .map(|(_, function)| function),
    );
    write_table(
        output,
        "list.bool",
        functions
            .bool_list_functions
            .iter()
            .map(|(_, function)| function),
    );
    write_table(
        output,
        "list.nil",
        functions
            .nil_list_functions
            .iter()
            .map(|(_, function)| function),
    );
    write_table(
        output,
        "list.tuple",
        functions
            .tuple_list_functions
            .iter()
            .map(|(_, function)| function),
    );
    write_table(
        output,
        "list.parameter_list",
        functions
            .parameter_list_list_functions
            .iter()
            .map(|(_, function)| function),
    );
    write_table(
        output,
        "list.list",
        functions
            .list_list_functions
            .iter()
            .map(|(_, function)| function),
    );
    write_table(
        output,
        "list.function",
        functions
            .function_list_functions
            .iter()
            .map(|(_, function)| function),
    );

    write_table(output, "function.int", &functions.int_function_functions);
    write_table(
        output,
        "function.float",
        &functions.float_function_functions,
    );
    write_table(
        output,
        "function.string",
        &functions.string_function_functions,
    );
    write_table(
        output,
        "function.bit_array",
        &functions.bit_array_function_functions,
    );
    write_table(
        output,
        "function.utf_codepoint",
        &functions.utf_codepoint_function_functions,
    );
    write_table(
        output,
        "function.custom",
        &functions.custom_function_functions,
    );
    write_table(output, "function.bool", &functions.bool_function_functions);
    write_table(output, "function.nil", &functions.nil_function_functions);
    write_table(
        output,
        "function.tuple",
        &functions.tuple_function_functions,
    );
    write_table(
        output,
        "function.generic",
        &functions.generic_function_functions,
    );
    write_table(
        output,
        "function.never",
        &functions.never_function_functions,
    );

    write_table(
        output,
        "function.list.parameter",
        &functions.parameter_list_function_functions,
    );
    write_table(
        output,
        "function.list.parameter_list",
        &functions.parameter_list_list_function_functions,
    );
    write_table(
        output,
        "function.list.int",
        &functions.int_list_function_functions,
    );
    write_table(
        output,
        "function.list.string",
        &functions.string_list_function_functions,
    );
    write_table(
        output,
        "function.list.bit_array",
        &functions.bit_array_list_function_functions,
    );
    write_table(
        output,
        "function.list.utf_codepoint",
        &functions.utf_codepoint_list_function_functions,
    );
    write_table(
        output,
        "function.list.custom",
        &functions.custom_list_function_functions,
    );
    write_table(
        output,
        "function.list.float",
        &functions.float_list_function_functions,
    );
    write_table(
        output,
        "function.list.bool",
        &functions.bool_list_function_functions,
    );
    write_table(
        output,
        "function.list.nil",
        &functions.nil_list_function_functions,
    );
    write_table(
        output,
        "function.list.tuple",
        &functions.tuple_list_function_functions,
    );
    write_table(
        output,
        "function.list.list",
        &functions.list_list_function_functions,
    );
    write_table(
        output,
        "function.list.function",
        &functions.function_list_function_functions,
    );
    write_table(
        output,
        "function.function",
        &functions.function_function_functions,
    );
}

fn write_table<'a, Return, Functions>(
    output: &mut String,
    family: &'static str,
    functions: Functions,
) where
    Return: ExplainedReturn + 'a,
    Functions: IntoIterator<Item = &'a ExecutableFunction<Return>>,
{
    for (index, function) in functions.into_iter().enumerate() {
        output.push_str("\nfunction ");
        FunctionLabel::new(family, index).push_to(output);
        output.push_str("\n  entry steps=");
        output.push_str(&function.steps().len().to_string());
        output.push('\n');
        write_graph(output, function.return_(), family);
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
