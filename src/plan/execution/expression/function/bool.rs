use crate::plan::execution::CustomFieldAccess;
use crate::plan::execution::FunctionType;
use crate::plan::execution::{
    BoolExpr, BoolFunctionFunctionId, BoolFunctionId, BoolFunctionLocalId, FloatExpr,
    FunctionFunctionExpr, FunctionListExpr, FunctionReference, IntExpr, PanicExpr, Step,
    StringExpr, TupleExpr,
};
use ecow::EcoString;
use num_bigint::BigInt;

pub struct BoolFunctionExpr {
    kind: BoolFunctionExprKind,
}

pub(crate) enum BoolFunctionExprKind {
    Constant(crate::plan::execution::ConstantId<BoolFunctionExpr>),
    Reference(FunctionReference<BoolFunctionId>),
    Closure(crate::plan::execution::ClosureTemplate<BoolFunctionId>),
    LocalGet {
        local: BoolFunctionLocalId,
    },
    Call(crate::plan::execution::DirectCall<BoolFunctionFunctionId>),
    FunctionCall(crate::plan::execution::FunctionCall<FunctionFunctionExpr>),
    TupleIndex {
        tuple: Box<TupleExpr>,
        index: usize,
        type_: FunctionType,
    },
    CustomField(CustomFieldAccess),
    ListIndex {
        list: Box<FunctionListExpr>,
        index: usize,
        type_: FunctionType,
    },
    Panic(PanicExpr),
    BoolCase {
        subject: Box<BoolExpr>,
        true_: Box<BoolFunctionExpr>,
        false_: Box<BoolFunctionExpr>,
    },
    IntCase {
        subject: Box<IntExpr>,
        clauses: Vec<(BigInt, BoolFunctionExpr)>,
        fallback: Box<BoolFunctionExpr>,
    },
    StringCase {
        subject: Box<StringExpr>,
        clauses: Vec<(EcoString, BoolFunctionExpr)>,
        fallback: Box<BoolFunctionExpr>,
    },
    FloatCase {
        subject: Box<FloatExpr>,
        clauses: Vec<(f64, BoolFunctionExpr)>,
        fallback: Box<BoolFunctionExpr>,
    },
    Block {
        steps: Vec<Step>,
        return_: Box<BoolFunctionExpr>,
    },
}

impl BoolFunctionExpr {
    pub(in crate::plan::execution) fn from_kind(kind: BoolFunctionExprKind) -> Self {
        Self { kind }
    }

    pub(in crate::plan::execution) fn into_kind(self) -> BoolFunctionExprKind {
        self.kind
    }

    pub(crate) fn kind(&self) -> &BoolFunctionExprKind {
        &self.kind
    }
}
