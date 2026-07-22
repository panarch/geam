use super::super::{
    BoolLocalId, FloatLocalId, IntLocalId, NeverFunctionId, NeverFunctionLocal, ParamLocal,
    StringLocalId,
};
use super::{Edge, GraphExitId, MatchEdge};
use crate::plan::{PanicSite, SourceSpan};
use ecow::EcoString;
use num_bigint::BigInt;

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
