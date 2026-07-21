mod bit_array;
mod constant;
mod function;
mod graph;
mod instruction;
mod label;
mod pattern;
mod value;

use self::constant::write_constant_tables;
use self::function::write_function_tables;
use self::label::runtime_function_label;
use super::ExecutionPlan;
use std::fmt;

pub struct ExecutionPlanExplanation<'a> {
    plan: &'a ExecutionPlan,
}

impl<'a> ExecutionPlanExplanation<'a> {
    pub(super) fn new(plan: &'a ExecutionPlan) -> Self {
        Self { plan }
    }
}

impl fmt::Display for ExecutionPlanExplanation<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&render(self.plan))
    }
}

fn render(plan: &ExecutionPlan) -> String {
    let mut output = String::new();
    output.push_str("module ");
    output.push_str(&plan.module);
    output.push_str("\nmain ");
    runtime_function_label(&plan.main).push_to(&mut output);
    output.push('\n');
    write_function_tables(&mut output, plan, &plan.functions);
    write_constant_tables(&mut output, plan, &plan.constants);
    output
}

#[cfg(test)]
mod tests {
    use crate::{ExecutionPlan, ExecutionPlanExplanation, Value, run_main};

    #[test]
    fn formats_public_typed_block_graph_without_source_names_and_preserves_execution() {
        let source = include_str!("../../../tests/fixtures/explain/return_topology.gleam");
        let plan = execution_plan(source);
        assert_eq!(run_main(&plan), Ok(Value::Int(40.into())));

        let explanation: ExecutionPlanExplanation<'_> = plan.explain();
        assert_eq!(explanation.to_string(), expected_explanation(source));
        assert!(!explanation.to_string().contains("choose"));
        assert_eq!(run_main(&plan), Ok(Value::Int(40.into())));
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
