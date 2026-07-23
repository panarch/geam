use super::super::graph::{Graph, GraphExitId};

pub(crate) struct ConstantProgram<Value> {
    graph: Graph,
    returns: Box<[Value]>,
}

impl<Value> ConstantProgram<Value> {
    pub(in crate::plan::execution) fn from_parts(graph: Graph, returns: Vec<Value>) -> Self {
        Self {
            graph,
            returns: returns.into_boxed_slice(),
        }
    }

    pub(crate) fn graph(&self) -> &Graph {
        &self.graph
    }

    pub(crate) fn return_(&self, id: GraphExitId) -> &Value {
        &self.returns[id.index()]
    }
}

use crate::plan::execution::explain::{Explain, ExplainContext};
use crate::plan::execution::graph::{ExplainLocal, write_graph};

impl<Value> Explain for ConstantProgram<Value>
where
    Value: ExplainLocal,
{
    fn write_explanation(&self, context: &mut ExplainContext<'_, '_>) {
        write_graph(context, self.graph(), &[], &[], &mut |context, exit| {
            context.push_str("return ");
            context.write(self.return_(exit));
        });
    }
}

#[cfg(test)]
mod explain_tests {
    use crate::plan::execution::{ConstantId, IntLocalId, explain};

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
