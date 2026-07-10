use super::{
    BoolExpr, CallArg, FloatExpr, IntExpr, PanicExpr, StringFunctionExpr, StringListExpr, TupleExpr,
};
use crate::plan::execution::{Step, StringFunctionId, StringLocalId};
use ecow::EcoString;
use num_bigint::BigInt;

pub struct StringExpr {
    kind: StringExprKind,
}

pub(crate) enum StringExprKind {
    Value(EcoString),
    LocalGet {
        local: StringLocalId,
    },
    Call {
        function: StringFunctionId,
        args: Vec<CallArg>,
    },
    FunctionCall {
        function: Box<StringFunctionExpr>,
        args: Vec<CallArg>,
    },
    TupleIndex {
        tuple: Box<TupleExpr>,
        index: usize,
    },
    ListIndex {
        list: Box<StringListExpr>,
        index: usize,
    },
    Panic(PanicExpr),
    Concatenate {
        left: Box<StringExpr>,
        right: Box<StringExpr>,
    },
    DropPrefix {
        value: Box<StringExpr>,
        prefix: EcoString,
    },
    BoolCase {
        subject: Box<BoolExpr>,
        true_: Box<StringExpr>,
        false_: Box<StringExpr>,
    },
    IntCase {
        subject: Box<IntExpr>,
        clauses: Vec<(BigInt, StringExpr)>,
        fallback: Box<StringExpr>,
    },
    StringCase {
        subject: Box<StringExpr>,
        clauses: Vec<(EcoString, StringExpr)>,
        fallback: Box<StringExpr>,
    },
    FloatCase {
        subject: Box<FloatExpr>,
        clauses: Vec<(f64, StringExpr)>,
        fallback: Box<StringExpr>,
    },
    Block {
        steps: Vec<Step>,
        return_: Box<StringExpr>,
    },
}

impl StringExpr {
    pub(in crate::plan::execution) fn from_kind(kind: StringExprKind) -> Self {
        Self { kind }
    }

    pub(crate) fn kind(&self) -> &StringExprKind {
        &self.kind
    }
}
