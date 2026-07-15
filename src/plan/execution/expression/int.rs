use super::{
    BoolExpr, CallArg, CustomFieldAccess, FloatExpr, IntFunctionExpr, IntListExpr, PanicExpr,
    StringExpr, TupleExpr,
};
use crate::plan::execution::{IntFunctionId, IntLocalId, Step};
use ecow::EcoString;
use num_bigint::BigInt;

pub struct IntExpr {
    kind: IntExprKind,
}

pub(crate) enum IntExprKind {
    Value(BigInt),
    LocalGet {
        local: IntLocalId,
    },
    Call {
        function: IntFunctionId,
        args: Vec<CallArg>,
    },
    FunctionCall {
        function: Box<IntFunctionExpr>,
        args: Vec<CallArg>,
    },
    TupleIndex {
        tuple: Box<TupleExpr>,
        index: usize,
    },
    CustomField(CustomFieldAccess),
    ListIndex {
        list: Box<IntListExpr>,
        index: usize,
    },
    Panic(PanicExpr),
    Add {
        left: Box<IntExpr>,
        right: Box<IntExpr>,
    },
    Sub {
        left: Box<IntExpr>,
        right: Box<IntExpr>,
    },
    Mult {
        left: Box<IntExpr>,
        right: Box<IntExpr>,
    },
    Div {
        left: Box<IntExpr>,
        right: Box<IntExpr>,
    },
    Remainder {
        left: Box<IntExpr>,
        right: Box<IntExpr>,
    },
    Negate(Box<IntExpr>),
    BoolCase {
        subject: Box<BoolExpr>,
        true_: Box<IntExpr>,
        false_: Box<IntExpr>,
    },
    IntCase {
        subject: Box<IntExpr>,
        clauses: Vec<(BigInt, IntExpr)>,
        fallback: Box<IntExpr>,
    },
    StringCase {
        subject: Box<StringExpr>,
        clauses: Vec<(EcoString, IntExpr)>,
        fallback: Box<IntExpr>,
    },
    FloatCase {
        subject: Box<FloatExpr>,
        clauses: Vec<(f64, IntExpr)>,
        fallback: Box<IntExpr>,
    },
    Block {
        steps: Vec<Step>,
        return_: Box<IntExpr>,
    },
}

impl IntExpr {
    pub(in crate::plan::execution) fn from_kind(kind: IntExprKind) -> Self {
        Self { kind }
    }

    pub(crate) fn kind(&self) -> &IntExprKind {
        &self.kind
    }
}
