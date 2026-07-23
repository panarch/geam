use super::super::super::ExecutionPlan;
use super::super::super::function::ExecutableFunction;
use super::super::graph::ExplainedGraph;
use super::super::label::FunctionLabel;

pub(super) fn write_table<'a, Return, Functions>(
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
    #[test]
    fn writes_one_function_table_with_its_exact_graph() {
        let source = "pub fn main() { 1 }";
        let expected = concat!(
            "\nfunction int#0\n",
            "  entry b0 params=[] captures=[]\n",
            "  block b0 params=[]\n",
            "    %int#0:shape#0(Int) = int.value 1\n",
            "    return %int#0\n",
        );

        assert_explanation(source, expected);
    }

    fn assert_explanation(source: &str, expected: &str) {
        super::super::super::assert_rendered(source, expected, |plan, output| {
            super::write_table(output, plan, "int", &plan.functions.int_functions);
        });
    }
}
