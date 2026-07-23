use super::super::graph::{BlockGraph, BlockGraphExitId};
use crate::plan::execution::explain::{Explain, ExplainContext};
use crate::plan::execution::graph::{ExplainLocal, write_graph};

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
    Return: ExplainLocal,
{
    fn write_explanation(&self, context: &mut ExplainContext<'_, '_>) {
        write_graph(
            context,
            self.block_graph(),
            &[],
            &[],
            &mut |context, exit| {
                context.push_str("return ");
                context.write(self.return_(exit));
            },
        );
    }
}

#[cfg(test)]
mod explain_tests {
    use crate::plan::execution::constant::ConstantId;
    use crate::plan::execution::explain;
    use crate::plan::execution::graph::IntLocalId;

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

    fn assert_explanation(source: &str, expected: &str) {
        explain::assert_rendered(source, expected, |plan, output| {
            let program = plan.constant(ConstantId::<IntLocalId>::new(0));
            let mut context = explain::ExplainContext::new(plan, output);
            context.write(program);
        });
    }
}
