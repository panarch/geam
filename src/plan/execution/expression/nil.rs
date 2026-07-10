use super::{
    BoolExpr, CallArg, FloatExpr, IntExpr, NilFunctionExpr, NilListExpr, PanicExpr, StringExpr,
    TupleExpr,
};
use crate::plan::execution::{NilFunctionId, NilLocalId, Step};
use ecow::EcoString;
use num_bigint::BigInt;

pub struct NilExpr {
    kind: NilExprKind,
}

pub(crate) enum NilExprKind {
    Value,
    LocalGet {
        local: NilLocalId,
    },
    Call {
        function: NilFunctionId,
        args: Vec<CallArg>,
    },
    FunctionCall {
        function: Box<NilFunctionExpr>,
        args: Vec<CallArg>,
    },
    TupleIndex {
        tuple: Box<TupleExpr>,
        index: usize,
    },
    ListIndex {
        list: Box<NilListExpr>,
        index: usize,
    },
    Panic(PanicExpr),
    BoolCase {
        subject: Box<BoolExpr>,
        true_: Box<NilExpr>,
        false_: Box<NilExpr>,
    },
    IntCase {
        subject: Box<IntExpr>,
        clauses: Vec<(BigInt, NilExpr)>,
        fallback: Box<NilExpr>,
    },
    StringCase {
        subject: Box<StringExpr>,
        clauses: Vec<(EcoString, NilExpr)>,
        fallback: Box<NilExpr>,
    },
    FloatCase {
        subject: Box<FloatExpr>,
        clauses: Vec<(f64, NilExpr)>,
        fallback: Box<NilExpr>,
    },
    Block {
        steps: Vec<Step>,
        return_: Box<NilExpr>,
    },
}

impl NilExpr {
    pub(in crate::plan::execution) fn from_kind(kind: NilExprKind) -> Self {
        Self { kind }
    }

    pub(crate) fn kind(&self) -> &NilExprKind {
        &self.kind
    }
}
