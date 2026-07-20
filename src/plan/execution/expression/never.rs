use super::{
    BoolExpr, CallArg, Expr, FloatExpr, FunctionExpr, IntExpr, NeverFunctionExpr, PanicExpr,
    StringExpr,
};
use crate::plan::execution::{AssertSubject, NeverFunctionId, Step};
use crate::plan::{PanicSite, SourceSpan};
use ecow::EcoString;
use num_bigint::BigInt;

pub(crate) struct NeverExpr {
    kind: NeverExprKind,
}

pub(crate) enum NeverExprKind {
    Call {
        function: NeverFunctionId,
        args: Vec<CallArg>,
    },
    Arguments {
        prefix: Vec<CallArg>,
        diverging: Box<NeverExpr>,
    },
    FunctionCall {
        function: Box<NeverFunctionExpr>,
        args: Vec<CallArg>,
    },
    FunctionArguments {
        function: Box<FunctionExpr>,
        prefix: Vec<CallArg>,
        diverging: Box<NeverExpr>,
    },
    Values {
        prefix: Vec<Expr>,
        diverging: Box<NeverExpr>,
    },
    LetAssert {
        subject: AssertSubject,
        message: Option<Box<StringExpr>>,
        site: PanicSite,
        pattern_span: SourceSpan,
    },
    Panic(PanicExpr),
    BoolCase {
        subject: Box<BoolExpr>,
        true_: Box<NeverExpr>,
        false_: Box<NeverExpr>,
    },
    IntCase {
        subject: Box<IntExpr>,
        clauses: Vec<(BigInt, NeverExpr)>,
        fallback: Box<NeverExpr>,
    },
    StringCase {
        subject: Box<StringExpr>,
        clauses: Vec<(EcoString, NeverExpr)>,
        fallback: Box<NeverExpr>,
    },
    FloatCase {
        subject: Box<FloatExpr>,
        clauses: Vec<(f64, NeverExpr)>,
        fallback: Box<NeverExpr>,
    },
    Block {
        steps: Vec<Step>,
        return_: Box<NeverExpr>,
    },
}

impl NeverExpr {
    pub(in crate::plan::execution) fn from_kind(kind: NeverExprKind) -> Self {
        Self { kind }
    }

    pub(crate) fn kind(&self) -> &NeverExprKind {
        &self.kind
    }

    pub(in crate::plan::execution) fn into_kind(self) -> NeverExprKind {
        self.kind
    }
}
