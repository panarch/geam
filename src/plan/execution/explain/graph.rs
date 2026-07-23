mod block;
mod edge;
mod exit;
mod terminator;

use self::block::write_block;
use self::exit::write_function_exit;
use self::terminator::write_terminator;
use super::super::constant::ConstantProgram;
use super::super::function::{FunctionEntry, FunctionGraph};
use super::super::graph::{Graph, GraphExitId};
use super::super::{
    CustomFunctionReturn, CustomReturn, ExecutionPlan, FunctionFunctionReturn, ParamSlot,
    TypedFunctionReturn,
};
use super::value::{ExplainLocal, write_slots};

pub(super) trait ExplainedGraph {
    fn entry_params<'a>(&'a self, entry: &FunctionEntry) -> &'a [ParamSlot];

    fn entry_captures<'a>(&'a self, entry: &FunctionEntry) -> &'a [ParamSlot];

    fn write_complete(
        &self,
        output: &mut String,
        plan: &ExecutionPlan,
        family: &'static str,
        entry_params: &[ParamSlot],
        entry_captures: &[ParamSlot],
    );
}

impl<Return, TailCall> ExplainedGraph for FunctionGraph<Return, TailCall>
where
    Return: ExplainLocal,
    TailCall: exit::TailFunctionIndex,
{
    fn entry_params<'a>(&'a self, entry: &FunctionEntry) -> &'a [ParamSlot] {
        entry.params(self)
    }

    fn entry_captures<'a>(&'a self, entry: &FunctionEntry) -> &'a [ParamSlot] {
        entry.captures(self)
    }

    fn write_complete(
        &self,
        output: &mut String,
        plan: &ExecutionPlan,
        family: &'static str,
        entry_params: &[ParamSlot],
        entry_captures: &[ParamSlot],
    ) {
        write_graph(
            output,
            plan,
            self.graph(),
            entry_params,
            entry_captures,
            &mut |output, exit| write_function_exit(output, self.exit(exit), family),
        );
    }
}

pub(super) fn write_constant_program<Value>(
    output: &mut String,
    plan: &ExecutionPlan,
    program: &ConstantProgram<Value>,
) where
    Value: ExplainLocal,
{
    write_graph(
        output,
        plan,
        program.graph(),
        &[],
        &[],
        &mut |output, exit| {
            output.push_str("return ");
            program.return_(exit).write_local(output);
        },
    );
}

fn write_graph(
    output: &mut String,
    plan: &ExecutionPlan,
    graph: &Graph,
    entry_params: &[ParamSlot],
    entry_captures: &[ParamSlot],
    write_exit: &mut dyn FnMut(&mut String, GraphExitId),
) {
    output.push_str("  entry b");
    output.push_str(&graph.entry().index().to_string());
    output.push_str(" params=");
    write_slots(output, plan, entry_params);
    output.push_str(" captures=");
    write_slots(output, plan, entry_captures);
    output.push('\n');

    for (index, block) in graph.blocks().iter().enumerate() {
        write_block(output, plan, index, block);
        output.push_str("    ");
        write_terminator(output, block.terminator(), write_exit);
        output.push('\n');
    }
}

impl<Body> ExplainedGraph for TypedFunctionReturn<Body>
where
    Body: ExplainedGraph,
{
    fn entry_params<'a>(&'a self, entry: &FunctionEntry) -> &'a [ParamSlot] {
        self.body().entry_params(entry)
    }

    fn entry_captures<'a>(&'a self, entry: &FunctionEntry) -> &'a [ParamSlot] {
        self.body().entry_captures(entry)
    }

    fn write_complete(
        &self,
        output: &mut String,
        plan: &ExecutionPlan,
        family: &'static str,
        entry_params: &[ParamSlot],
        entry_captures: &[ParamSlot],
    ) {
        self.body()
            .write_complete(output, plan, family, entry_params, entry_captures);
    }
}

impl ExplainedGraph for CustomReturn {
    fn entry_params<'a>(&'a self, entry: &FunctionEntry) -> &'a [ParamSlot] {
        self.body().entry_params(entry)
    }

    fn entry_captures<'a>(&'a self, entry: &FunctionEntry) -> &'a [ParamSlot] {
        self.body().entry_captures(entry)
    }

    fn write_complete(
        &self,
        output: &mut String,
        plan: &ExecutionPlan,
        family: &'static str,
        entry_params: &[ParamSlot],
        entry_captures: &[ParamSlot],
    ) {
        self.body()
            .write_complete(output, plan, family, entry_params, entry_captures);
    }
}

impl ExplainedGraph for CustomFunctionReturn {
    fn entry_params<'a>(&'a self, entry: &FunctionEntry) -> &'a [ParamSlot] {
        self.body().entry_params(entry)
    }

    fn entry_captures<'a>(&'a self, entry: &FunctionEntry) -> &'a [ParamSlot] {
        self.body().entry_captures(entry)
    }

    fn write_complete(
        &self,
        output: &mut String,
        plan: &ExecutionPlan,
        family: &'static str,
        entry_params: &[ParamSlot],
        entry_captures: &[ParamSlot],
    ) {
        self.body()
            .write_complete(output, plan, family, entry_params, entry_captures);
    }
}

impl ExplainedGraph for FunctionFunctionReturn {
    fn entry_params<'a>(&'a self, entry: &FunctionEntry) -> &'a [ParamSlot] {
        self.body().entry_params(entry)
    }

    fn entry_captures<'a>(&'a self, entry: &FunctionEntry) -> &'a [ParamSlot] {
        self.body().entry_captures(entry)
    }

    fn write_complete(
        &self,
        output: &mut String,
        plan: &ExecutionPlan,
        family: &'static str,
        entry_params: &[ParamSlot],
        entry_captures: &[ParamSlot],
    ) {
        self.body()
            .write_complete(output, plan, family, entry_params, entry_captures);
    }
}

#[cfg(test)]
mod tests {
    use super::ExplainedGraph;
    use crate::plan::execution::IntFunctionId;

    #[test]
    fn writes_complete_graph_entry_and_block_order() {
        let source = r#"
fn choose(flag: Bool) { case flag { True -> 1 False -> 0 } }
pub fn main() { choose(True) }
"#;
        let expected = concat!(
            "  entry b0 params=[%bool#0:shape#0(Bool)] captures=[]\n",
            "  block b0 params=[%bool#0:shape#0(Bool)]\n",
            "    branch %bool#0 true=b1() false=b2()\n",
            "  block b1 params=[]\n",
            "    %int#0:shape#1(Int) = int.value 1\n",
            "    return %int#0\n",
            "  block b2 params=[]\n",
            "    %int#0:shape#1(Int) = int.value 0\n",
            "    return %int#0\n",
        );

        assert_explanation(source, expected);
    }

    fn assert_explanation(source: &str, expected: &str) {
        super::super::assert_rendered(source, expected, |plan, output| {
            let function = plan.int_function(IntFunctionId(1));
            let graph = function.graph();
            graph.write_complete(
                output,
                plan,
                "int",
                graph.entry_params(function.entry()),
                graph.entry_captures(function.entry()),
            );
        });
    }
}
