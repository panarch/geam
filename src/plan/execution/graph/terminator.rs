use super::super::{
    BoolLocalId, FloatLocalId, IntLocalId, NeverFunctionId, NeverFunctionLocal, ParamLocal,
    StringLocalId,
};
use crate::plan::{PanicSite, SourceSpan};
use ecow::EcoString;
use num_bigint::BigInt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct BlockId(usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct GraphExitId(usize);

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum SourceStopKind {
    Panic,
    Todo,
    Assert,
    EmptyFunction,
    EmptyBlock,
    IncompleteUse,
}

pub(crate) enum NeverCallTarget {
    Direct(NeverFunctionId),
    Value(NeverFunctionLocal),
}

pub(crate) enum Terminator {
    Jump(Edge),
    BoolBranch {
        subject: BoolLocalId,
        true_: Edge,
        false_: Edge,
    },
    IntSwitch {
        subject: IntLocalId,
        clauses: Box<[(BigInt, Edge)]>,
        fallback: Edge,
    },
    FloatSwitch {
        subject: FloatLocalId,
        clauses: Box<[(f64, Edge)]>,
        fallback: Edge,
    },
    StringSwitch {
        subject: StringLocalId,
        clauses: Box<[(EcoString, Edge)]>,
        fallback: Edge,
    },
    Match {
        subject: ParamLocal,
        pattern: super::MatchPattern,
        success: MatchEdge,
        failure: Edge,
    },
    Exit(GraphExitId),
    SourceStop {
        kind: SourceStopKind,
        message: Option<StringLocalId>,
        site: PanicSite,
    },
    LetAssertPanic {
        subject: ParamLocal,
        message: Option<StringLocalId>,
        site: PanicSite,
        pattern_span: SourceSpan,
    },
    NeverCall {
        function: NeverCallTarget,
        args: Box<[ParamLocal]>,
    },
}

impl BlockId {
    pub(in crate::plan::execution) fn new(index: usize) -> Self {
        Self(index)
    }

    pub(crate) fn index(self) -> usize {
        self.0
    }
}

impl GraphExitId {
    pub(in crate::plan::execution) fn new(index: usize) -> Self {
        Self(index)
    }

    pub(crate) fn index(self) -> usize {
        self.0
    }
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
