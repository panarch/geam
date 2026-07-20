use super::{
    BoolExpr, CustomFieldAccess, DirectCall, FloatExpr, FunctionCall, IntExpr, PanicExpr,
    StringExpr, TupleExpr, UtfCodepointFunctionExpr, UtfCodepointListExpr,
};
use crate::plan::execution::{Step, UtfCodepointFunctionId, UtfCodepointLocalId};
use ecow::EcoString;
use num_bigint::BigInt;

pub struct UtfCodepointExpr {
    kind: UtfCodepointExprKind,
}

pub(crate) enum UtfCodepointExprKind {
    LocalGet {
        local: UtfCodepointLocalId,
    },
    Call(DirectCall<UtfCodepointFunctionId>),
    FunctionCall(FunctionCall<UtfCodepointFunctionExpr>),
    TupleIndex {
        tuple: Box<TupleExpr>,
        index: usize,
    },
    CustomField(CustomFieldAccess),
    ListIndex {
        list: Box<UtfCodepointListExpr>,
        index: usize,
    },
    Panic(PanicExpr),
    BoolCase {
        subject: Box<BoolExpr>,
        true_: Box<UtfCodepointExpr>,
        false_: Box<UtfCodepointExpr>,
    },
    IntCase {
        subject: Box<IntExpr>,
        clauses: Vec<(BigInt, UtfCodepointExpr)>,
        fallback: Box<UtfCodepointExpr>,
    },
    StringCase {
        subject: Box<StringExpr>,
        clauses: Vec<(EcoString, UtfCodepointExpr)>,
        fallback: Box<UtfCodepointExpr>,
    },
    FloatCase {
        subject: Box<FloatExpr>,
        clauses: Vec<(f64, UtfCodepointExpr)>,
        fallback: Box<UtfCodepointExpr>,
    },
    Block {
        steps: Vec<Step>,
        return_: Box<UtfCodepointExpr>,
    },
}

impl UtfCodepointExpr {
    pub(in crate::plan::execution) fn from_kind(kind: UtfCodepointExprKind) -> Self {
        Self { kind }
    }

    pub(in crate::plan::execution) fn into_kind(self) -> UtfCodepointExprKind {
        self.kind
    }

    pub(crate) fn kind(&self) -> &UtfCodepointExprKind {
        &self.kind
    }
}
