use super::super::graph::{BlockGraph, BlockGraphExitId};
use crate::plan::execution::explain::{Explain, ExplainContext};
use crate::plan::execution::graph::{BlockGraphExitExplanation, LocalLabel};

pub(crate) struct ConstantProgram<Return> {
    block_graph: BlockGraph,
    returns: Box<[Return]>,
}

impl<Return> ConstantProgram<Return> {
    pub(in crate::plan::execution) fn from_parts(
        block_graph: BlockGraph,
        returns: Vec<Return>,
    ) -> Self {
        Self {
            block_graph,
            returns: returns.into_boxed_slice(),
        }
    }

    pub(crate) fn block_graph(&self) -> &BlockGraph {
        &self.block_graph
    }

    pub(crate) fn return_(&self, id: BlockGraphExitId) -> &Return {
        &self.returns[id.index()]
    }
}

impl<Return> Explain for ConstantProgram<Return>
where
    Return: LocalLabel,
{
    fn write_explanation(&self, context: &mut ExplainContext<'_, '_>) {
        self.block_graph()
            .write_explanation(context, &[], &[], self);
    }
}

impl<Return> BlockGraphExitExplanation for ConstantProgram<Return>
where
    Return: LocalLabel,
{
    fn write_exit(&self, context: &mut ExplainContext<'_, '_>, exit: BlockGraphExitId) {
        context.push_str("return ");
        context.write(self.return_(exit));
    }
}

#[cfg(test)]
mod explain_tests {
    use crate::plan::execution::constant::ConstantId;
    use crate::plan::execution::explain;
    use crate::plan::execution::graph::{BlockGraphExitExplanation, BlockGraphExitId, IntLocalId};

    #[test]
    fn writes_reusable_constant_graph_program() {
        let source = r#"
const one = 1
pub fn main() { one }
"#;
        let expected = concat!(
            "  entry b0 params=[] captures=[]\n",
            "  block b0 params=[]\n",
            "    %int#0:shape#0(Int) = int.value 1\n",
            "    return %int#0\n",
        );

        assert_explanation(source, expected);
    }

    #[test]
    fn writes_constant_block_graph_exit() {
        let source = r#"
const one = 1
pub fn main() { one }
"#;
        let expected = "return %int#0";

        explain::assert_rendered(source, expected, |plan, output| {
            let program = plan.constant(ConstantId::<IntLocalId>::new(0));
            let mut context = explain::ExplainContext::new(plan, output);
            program.write_exit(&mut context, BlockGraphExitId::new(0));
        });
    }

    fn assert_explanation(source: &str, expected: &str) {
        explain::assert_rendered(source, expected, |plan, output| {
            let program = plan.constant(ConstantId::<IntLocalId>::new(0));
            let mut context = explain::ExplainContext::new(plan, output);
            context.write(program);
        });
    }
}
