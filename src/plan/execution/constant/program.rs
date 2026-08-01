use super::super::function::{ExecutionGraphProfile, FunctionLabelSource, HostedExecutionGraph};
use super::super::graph::{BlockGraphExitId, ProfiledBlockGraph};
use crate::plan::execution::explain::{Explain, ExplainContext};
use crate::plan::execution::graph::{BlockGraphExitExplanation, LocalLabel};

pub(crate) struct ProfiledConstantProgram<Return, Graph: ExecutionGraphProfile> {
    block_graph: ProfiledBlockGraph<Graph>,
    returns: Box<[Return]>,
}

pub(crate) type ConstantProgram<Return> = ProfiledConstantProgram<Return, HostedExecutionGraph>;

impl<Return, Graph: ExecutionGraphProfile> ProfiledConstantProgram<Return, Graph> {
    pub(in crate::plan::execution) fn from_parts(
        block_graph: ProfiledBlockGraph<Graph>,
        returns: Vec<Return>,
    ) -> Self {
        Self {
            block_graph,
            returns: returns.into_boxed_slice(),
        }
    }

    pub(crate) fn block_graph(&self) -> &ProfiledBlockGraph<Graph> {
        &self.block_graph
    }

    pub(crate) fn return_(&self, id: BlockGraphExitId) -> &Return {
        &self.returns[id.index()]
    }

    pub(in crate::plan::execution) fn into_parts(
        self,
    ) -> (ProfiledBlockGraph<Graph>, Box<[Return]>) {
        (self.block_graph, self.returns)
    }
}

impl<Return, Graph> Explain for ProfiledConstantProgram<Return, Graph>
where
    Return: LocalLabel,
    Graph: ExecutionGraphProfile,
    Graph::ExternalFunctionId: FunctionLabelSource,
    Graph::ExternalListFunctionId: FunctionLabelSource,
    Graph::ExternalFunctionFunctionId: FunctionLabelSource,
    Graph::ExternalListFunctionFunctionId: FunctionLabelSource,
    Graph::ExternalInstruction: Explain,
    Graph::ExternalListInstruction: Explain,
    Graph::ExternalFunctionInstruction: Explain,
{
    fn write_explanation(&self, context: &mut ExplainContext<'_, '_>) {
        self.block_graph()
            .write_explanation(context, &[], &[], self);
    }
}

impl<Return, Graph> BlockGraphExitExplanation for ProfiledConstantProgram<Return, Graph>
where
    Return: LocalLabel,
    Graph: ExecutionGraphProfile,
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
