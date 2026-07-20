use super::{
    BoolExpr, CustomFieldAccess, DirectCall, FloatExpr, FunctionCall, IntExpr, PanicExpr,
    StringFunctionExpr, StringListExpr, TupleExpr,
};
use crate::plan::execution::{ConstantId, Step, StringFunctionId, StringLocalId};
use ecow::EcoString;
use num_bigint::BigInt;

pub struct StringExpr {
    kind: StringExprKind,
}

pub(crate) enum StringExprKind {
    Value(EcoString),
    Constant(ConstantId<StringExpr>),
    LocalGet {
        local: StringLocalId,
    },
    Call(DirectCall<StringFunctionId>),
    FunctionCall(FunctionCall<StringFunctionExpr>),
    TupleIndex {
        tuple: Box<TupleExpr>,
        index: usize,
    },
    CustomField(CustomFieldAccess),
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

    pub(in crate::plan::execution) fn into_kind(self) -> StringExprKind {
        self.kind
    }

    pub(crate) fn kind(&self) -> &StringExprKind {
        &self.kind
    }
}
