use super::BlockId;
use crate::plan::execution::ParamLocal;

pub(crate) struct Edge {
    target: BlockId,
    args: Box<[ParamLocal]>,
}

pub(crate) struct MatchEdge {
    target: BlockId,
    args: Box<[MatchEdgeArgument]>,
}

pub(crate) enum MatchEdgeArgument {
    Binding(usize),
    Value(ParamLocal),
}

impl Edge {
    pub(in crate::plan::execution) fn new(target: BlockId, args: Vec<ParamLocal>) -> Self {
        Self {
            target,
            args: args.into_boxed_slice(),
        }
    }

    pub(crate) fn target(&self) -> BlockId {
        self.target
    }

    pub(crate) fn args(&self) -> &[ParamLocal] {
        &self.args
    }
}

impl MatchEdge {
    pub(in crate::plan::execution) fn new(target: BlockId, args: Vec<MatchEdgeArgument>) -> Self {
        Self {
            target,
            args: args.into_boxed_slice(),
        }
    }

    pub(crate) fn target(&self) -> BlockId {
        self.target
    }

    pub(crate) fn args(&self) -> &[MatchEdgeArgument] {
        &self.args
    }
}
