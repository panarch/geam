use super::{Block, BlockId, Graph};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct GraphExitId(usize);

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

impl GraphExitId {
    pub(in crate::plan::execution) fn new(index: usize) -> Self {
        Self(index)
    }

    pub(crate) fn index(self) -> usize {
        self.0
    }
}

impl<Return, TailCall> FunctionGraph<Return, TailCall> {
    pub(in crate::plan::execution) fn from_parts(
        entry: BlockId,
        blocks: Vec<Block>,
        exits: Vec<FunctionGraphExit<Return, TailCall>>,
    ) -> Self {
        Self {
            graph: Graph {
                entry,
                blocks: blocks.into_boxed_slice(),
            },
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

    pub(in crate::plan::execution) fn into_parts(
        self,
    ) -> (Graph, Box<[FunctionGraphExit<Return, TailCall>]>) {
        (self.graph, self.exits)
    }
}
