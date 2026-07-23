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
