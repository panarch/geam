use super::{
    BoolExpr, CustomFieldAccess, DirectCall, FloatExpr, FunctionCall, IntExpr, NeverExpr,
    PanicExpr, StringExpr, TupleFunctionExpr, TupleListExpr,
};
use crate::plan::execution::ValueType;
use crate::plan::execution::{ConstantId, Step, TupleFunctionId, TupleLocalId};
use ecow::EcoString;
use num_bigint::BigInt;

pub struct TupleExpr {
    type_: Vec<ValueType>,
    kind: TupleExprKind,
}

pub(crate) enum TupleExprKind {
    Never(NeverExpr),
    Value(Vec<super::Expr>),
    Constant(ConstantId<TupleExpr>),
    LocalGet {
        local: TupleLocalId,
    },
    Call(DirectCall<TupleFunctionId>),
    FunctionCall(FunctionCall<TupleFunctionExpr>),
    TupleIndex {
        tuple: Box<TupleExpr>,
        index: usize,
    },
    CustomField(CustomFieldAccess),
    ListIndex {
        list: Box<TupleListExpr>,
        index: usize,
    },
    Panic(PanicExpr),
    BoolCase {
        subject: Box<BoolExpr>,
        true_: Box<TupleExpr>,
        false_: Box<TupleExpr>,
    },
    IntCase {
        subject: Box<IntExpr>,
        clauses: Vec<(BigInt, TupleExpr)>,
        fallback: Box<TupleExpr>,
    },
    StringCase {
        subject: Box<StringExpr>,
        clauses: Vec<(EcoString, TupleExpr)>,
        fallback: Box<TupleExpr>,
    },
    FloatCase {
        subject: Box<FloatExpr>,
        clauses: Vec<(f64, TupleExpr)>,
        fallback: Box<TupleExpr>,
    },
    Block {
        steps: Vec<Step>,
        return_: Box<TupleExpr>,
    },
}

impl TupleExpr {
    pub(in crate::plan::execution) fn from_parts(
        type_: Vec<ValueType>,
        kind: TupleExprKind,
    ) -> Self {
        Self { type_, kind }
    }

    pub(in crate::plan::execution) fn into_kind(self) -> TupleExprKind {
        self.kind
    }

    pub(crate) fn type_(&self) -> &[ValueType] {
        &self.type_
    }

    pub(crate) fn kind(&self) -> &TupleExprKind {
        &self.kind
    }
}
