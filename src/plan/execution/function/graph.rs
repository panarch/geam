use crate::plan::execution::graph::{Block, BlockId, Graph, GraphExitId};

pub(crate) struct FunctionGraph<Return, TailCall> {
    graph: Graph,
    exits: Box<[FunctionGraphExit<Return, TailCall>]>,
}

pub(crate) enum FunctionGraphExit<Return, TailCall> {
    Return(Return),
    TailCall {
        function: TailCall,
        args: Box<[crate::plan::execution::ParamLocal]>,
    },
}

impl<Return, TailCall> FunctionGraph<Return, TailCall> {
    pub(in crate::plan::execution) fn from_parts(
        graph: Graph,
        exits: Vec<FunctionGraphExit<Return, TailCall>>,
    ) -> Self {
        Self {
            graph,
            exits: exits.into_boxed_slice(),
        }
    }

    pub(crate) fn entry(&self) -> BlockId {
        self.graph.entry()
    }

    #[cfg(test)]
    pub(crate) fn blocks(&self) -> &[Block] {
        self.graph.blocks()
    }

    pub(crate) fn block(&self, id: BlockId) -> &Block {
        self.graph.block(id)
    }

    pub(crate) fn graph(&self) -> &Graph {
        &self.graph
    }

    pub(crate) fn exit(&self, id: GraphExitId) -> &FunctionGraphExit<Return, TailCall> {
        &self.exits[id.index()]
    }
}
