use super::super::function::{ExecutionGraphProfile, FunctionLabelSource, HostedExecutionGraph};
use super::super::graph::{BlockGraphExitId, ProfiledBlockGraph};
use crate::plan::execution::explain::{Explain, ExplainContext};
use crate::plan::execution::graph::{BlockGraphExitExplanation, LocalLabel};
use crate::plan::execution::prepared::rust::{Emit, Rust};
use crate::plan::execution::storage::Table;
use crate::plan::execution::type_::ValueShapeId;

pub struct ProfiledConstantProgram<Return: 'static, Graph: ExecutionGraphProfile> {
    pub block_graph: ProfiledBlockGraph<Graph>,
    pub returns: Table<Return>,
    pub shape: ValueShapeId,
}

pub(crate) type ConstantProgram<Return> = ProfiledConstantProgram<Return, HostedExecutionGraph>;

impl<Return, Graph: ExecutionGraphProfile> ProfiledConstantProgram<Return, Graph> {
    pub(in crate::plan::execution) fn from_parts(
        block_graph: ProfiledBlockGraph<Graph>,
        returns: Table<Return>,
        shape: ValueShapeId,
    ) -> Self {
        Self {
            block_graph,
            returns,
            shape,
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
    ) -> (ProfiledBlockGraph<Graph>, Table<Return>, ValueShapeId) {
        (self.block_graph, self.returns, self.shape)
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

impl<Return: 'static, Graph: ExecutionGraphProfile> Emit for ProfiledConstantProgram<Return, Graph>
where
    ProfiledBlockGraph<Graph>: Emit,
    Table<Return>: Emit,
{
    fn emit(&self, output: &mut Rust) {
        let Self {
            block_graph,
            returns,
            shape,
        } = self;
        output.structure(
            "constant::ProfiledConstantProgram",
            &[
                ("block_graph", block_graph),
                ("returns", returns),
                ("shape", shape),
            ],
        );
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
