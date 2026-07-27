use super::{ExecutionPlan, HostedExecution};
use crate::plan::execution::type_::{ValueShapeId, ValueShapeTable, ValueType};
use std::fmt;

pub(in crate::plan::execution) trait Explain {
    fn write_explanation(&self, context: &mut ExplainContext<'_, '_>);
}

#[derive(Clone, Copy)]
pub(in crate::plan::execution) struct FunctionLabel {
    family: &'static str,
    index: usize,
}

impl FunctionLabel {
    pub(in crate::plan::execution) fn new(family: &'static str, index: usize) -> Self {
        Self { family, index }
    }

    pub(in crate::plan::execution) fn write(self, output: &mut String) {
        output.push_str(self.family);
        output.push('#');
        output.push_str(&self.index.to_string());
    }
}

pub(in crate::plan::execution) struct ExplainContext<'plan, 'output> {
    value_shapes: &'plan ValueShapeTable,
    output: &'output mut String,
}

impl<'plan, 'output> ExplainContext<'plan, 'output> {
    pub(in crate::plan::execution) fn new(
        plan: &'plan ExecutionPlan,
        output: &'output mut String,
    ) -> Self {
        Self {
            value_shapes: &plan.program.common.value_shapes,
            output,
        }
    }

    pub(in crate::plan::execution) fn new_hosted(
        execution: &'plan HostedExecution,
        output: &'output mut String,
    ) -> Self {
        Self {
            value_shapes: &execution.program.common.value_shapes,
            output,
        }
    }

    pub(in crate::plan::execution) fn shape_value_type(&self, id: ValueShapeId) -> ValueType {
        self.value_shapes.value_type(id).clone()
    }

    pub(in crate::plan::execution) fn output(&mut self) -> &mut String {
        self.output
    }

    pub(in crate::plan::execution) fn push(&mut self, character: char) {
        self.output.push(character);
    }

    pub(in crate::plan::execution) fn push_str(&mut self, text: &str) {
        self.output.push_str(text);
    }

    pub(in crate::plan::execution) fn write<Value>(&mut self, value: &Value)
    where
        Value: Explain + ?Sized,
    {
        value.write_explanation(self);
    }

    pub(in crate::plan::execution) fn write_list<Value>(
        &mut self,
        values: &[Value],
        mut write_value: impl FnMut(&mut Self, &Value),
    ) {
        self.output.push('[');
        for (index, value) in values.iter().enumerate() {
            if index > 0 {
                self.output.push_str(", ");
            }
            write_value(self, value);
        }
        self.output.push(']');
    }
}

pub struct ExecutionPlanExplanation<'a> {
    execution: ExplainedExecution<'a>,
}

enum ExplainedExecution<'a> {
    Plain(&'a ExecutionPlan),
    Hosted(&'a HostedExecution),
}

impl<'a> ExecutionPlanExplanation<'a> {
    pub(super) fn new(plan: &'a ExecutionPlan) -> Self {
        Self {
            execution: ExplainedExecution::Plain(plan),
        }
    }

    pub(super) fn new_hosted(execution: &'a HostedExecution) -> Self {
        Self {
            execution: ExplainedExecution::Hosted(execution),
        }
    }
}

impl fmt::Display for ExecutionPlanExplanation<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut output = String::new();
        match self.execution {
            ExplainedExecution::Plain(plan) => {
                let mut context = ExplainContext::new(plan, &mut output);
                context.write(plan);
            }
            ExplainedExecution::Hosted(execution) => {
                let mut context = ExplainContext::new_hosted(execution, &mut output);
                context.write(execution);
            }
        }
        formatter.write_str(&output)
    }
}

#[cfg(test)]
pub(in crate::plan::execution) fn with_execution_plan<Result>(
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
pub(in crate::plan::execution) fn assert_rendered(
    source: &str,
    expected: &str,
    render: impl FnOnce(&ExecutionPlan, &mut String),
) {
    with_execution_plan(source, |plan| {
        let mut actual = String::new();
        render(plan, &mut actual);
        assert_eq!(actual, expected, "source:\n{source}");
    });
}

#[cfg(test)]
pub(in crate::plan::execution) fn assert_written(expected: &str, write: impl FnOnce(&mut String)) {
    let mut actual = String::new();
    write(&mut actual);
    assert_eq!(actual, expected);
}

#[cfg(test)]
mod tests {
    use crate::ExecutionPlanExplanation;

    #[test]
    fn formats_the_public_execution_plan_facade() {
        let source = "pub fn main() { 1 }";
        let expected = concat!(
            "module main\n",
            "main int#0\n",
            "\n",
            "function int#0\n",
            "  entry b0 params=[] captures=[]\n",
            "  block b0 params=[]\n",
            "    %int#0:shape#0(Int) = int.value 1\n",
            "    return %int#0\n",
        );

        super::assert_rendered(source, expected, |plan, output| {
            let explanation: ExecutionPlanExplanation<'_> = plan.explain();
            output.push_str(&explanation.to_string());
        });
    }
}
