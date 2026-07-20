use super::{
    BoolExpr, CustomFieldAccess, DirectCall, FloatExpr, FunctionCall, IntExpr, NilFunctionExpr,
    NilListExpr, PanicExpr, StringExpr, TupleExpr,
};
use crate::plan::execution::{ConstantId, NilFunctionId, NilLocalId, Step};
use ecow::EcoString;
use num_bigint::BigInt;

pub struct NilExpr {
    kind: NilExprKind,
}

pub(crate) enum NilExprKind {
    Value,
    Constant(ConstantId<NilExpr>),
    LocalGet {
        local: NilLocalId,
    },
    Call(DirectCall<NilFunctionId>),
    FunctionCall(FunctionCall<NilFunctionExpr>),
    TupleIndex {
        tuple: Box<TupleExpr>,
        index: usize,
    },
    CustomField(CustomFieldAccess),
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

    pub(in crate::plan::execution) fn into_kind(self) -> NilExprKind {
        self.kind
    }

    pub(crate) fn kind(&self) -> &NilExprKind {
        &self.kind
    }
}
