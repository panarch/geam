use super::{
    BoolExpr, CallArg, FloatFunctionExpr, FloatListExpr, IntExpr, PanicExpr, StringExpr, TupleExpr,
};
use crate::plan::execution::{FloatFunctionId, FloatLocalId, Step};
use ecow::EcoString;
use num_bigint::BigInt;

pub struct FloatExpr {
    kind: FloatExprKind,
}

pub(crate) enum FloatExprKind {
    Value(f64),
    LocalGet {
        local: FloatLocalId,
    },
    Call {
        function: FloatFunctionId,
        args: Vec<CallArg>,
    },
    FunctionCall {
        function: Box<FloatFunctionExpr>,
        args: Vec<CallArg>,
    },
    TupleIndex {
        tuple: Box<TupleExpr>,
        index: usize,
    },
    ListIndex {
        list: Box<FloatListExpr>,
        index: usize,
    },
    Panic(PanicExpr),
    Add {
        left: Box<FloatExpr>,
        right: Box<FloatExpr>,
    },
    Sub {
        left: Box<FloatExpr>,
        right: Box<FloatExpr>,
    },
    Mult {
        left: Box<FloatExpr>,
        right: Box<FloatExpr>,
    },
    Div {
        left: Box<FloatExpr>,
        right: Box<FloatExpr>,
    },
    BoolCase {
        subject: Box<BoolExpr>,
        true_: Box<FloatExpr>,
        false_: Box<FloatExpr>,
    },
    IntCase {
        subject: Box<IntExpr>,
        clauses: Vec<(BigInt, FloatExpr)>,
        fallback: Box<FloatExpr>,
    },
    StringCase {
        subject: Box<StringExpr>,
        clauses: Vec<(EcoString, FloatExpr)>,
        fallback: Box<FloatExpr>,
    },
    FloatCase {
        subject: Box<FloatExpr>,
        clauses: Vec<(f64, FloatExpr)>,
        fallback: Box<FloatExpr>,
    },
    Block {
        steps: Vec<Step>,
        return_: Box<FloatExpr>,
    },
}

impl FloatExpr {
    pub(in crate::plan::execution) fn from_kind(kind: FloatExprKind) -> Self {
        Self { kind }
    }

    pub(crate) fn kind(&self) -> &FloatExprKind {
        &self.kind
    }
}
