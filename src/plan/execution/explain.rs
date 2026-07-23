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
fn with_execution_plan<Result>(
    source: &str,
    inspect: impl FnOnce(&ExecutionPlan) -> Result,
) -> Result {
    let typed =
        crate::compile_typed_module("main", "main.gleam", source).expect("source should compile");
    let module_plan = crate::plan_module(typed).expect("source should plan");
    let plan = ExecutionPlan::from_module_plan(module_plan);
    inspect(&plan)
}

#[cfg(test)]
fn assert_rendered(source: &str, expected: &str, render: impl FnOnce(&ExecutionPlan, &mut String)) {
    with_execution_plan(source, |plan| {
        let mut actual = String::new();
        render(plan, &mut actual);
        assert_eq!(actual, expected, "source:\n{source}");
    });
}

#[cfg(test)]
fn assert_written(expected: &str, write: impl FnOnce(&mut String)) {
    let mut actual = String::new();
    write(&mut actual);
    assert_eq!(actual, expected);
}

#[cfg(test)]
mod tests {
    use crate::ExecutionPlanExplanation;

    #[test]
    fn formats_the_public_execution_plan_facade() {
        super::assert_rendered(
            "pub fn main() { 1 }",
            concat!(
                "module main\n",
                "main int#0\n",
                "\n",
                "function int#0\n",
                "  entry b0 params=[] captures=[]\n",
                "  block b0 params=[]\n",
                "    %int#0:shape#0(Int) = int.value 1\n",
                "    return %int#0\n",
            ),
            |plan, output| {
                let explanation: ExecutionPlanExplanation<'_> = plan.explain();
                output.push_str(&explanation.to_string());
            },
        );
    }
}
