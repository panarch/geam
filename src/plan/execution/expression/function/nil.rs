use crate::plan::execution::CustomFieldAccess;
use crate::plan::execution::FunctionType;
use crate::plan::execution::{
    BoolExpr, ClosureTemplate, ConstantId, DirectCall, FloatExpr, FunctionCall,
    FunctionFunctionExpr, FunctionListExpr, FunctionReference, IntExpr, NilFunctionFunctionId,
    NilFunctionId, NilFunctionLocalId, PanicExpr, Step, StringExpr, TupleExpr,
};
use ecow::EcoString;
use num_bigint::BigInt;

pub struct NilFunctionExpr {
    kind: NilFunctionExprKind,
}

pub(crate) enum NilFunctionExprKind {
    Constant(ConstantId<NilFunctionExpr>),
    Reference(FunctionReference<NilFunctionId>),
    Closure(ClosureTemplate<NilFunctionId>),
    LocalGet {
        local: NilFunctionLocalId,
    },
    Call(DirectCall<NilFunctionFunctionId>),
    FunctionCall(FunctionCall<FunctionFunctionExpr>),
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
        true_: Box<NilFunctionExpr>,
        false_: Box<NilFunctionExpr>,
    },
    IntCase {
        subject: Box<IntExpr>,
        clauses: Vec<(BigInt, NilFunctionExpr)>,
        fallback: Box<NilFunctionExpr>,
    },
    StringCase {
        subject: Box<StringExpr>,
        clauses: Vec<(EcoString, NilFunctionExpr)>,
        fallback: Box<NilFunctionExpr>,
    },
    FloatCase {
        subject: Box<FloatExpr>,
        clauses: Vec<(f64, NilFunctionExpr)>,
        fallback: Box<NilFunctionExpr>,
    },
    Block {
        steps: Vec<Step>,
        return_: Box<NilFunctionExpr>,
    },
}

impl NilFunctionExpr {
    pub(in crate::plan::execution) fn from_kind(kind: NilFunctionExprKind) -> Self {
        Self { kind }
    }

    pub(in crate::plan::execution) fn into_kind(self) -> NilFunctionExprKind {
        self.kind
    }

    pub(crate) fn kind(&self) -> &NilFunctionExprKind {
        &self.kind
    }
}
