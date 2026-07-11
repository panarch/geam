use crate::plan::execution::FunctionType;
use crate::plan::execution::{
    BoolExpr, BoolFunctionFunctionId, BoolFunctionId, BoolFunctionLocalId, ClosureTemplate,
    FloatExpr, FunctionFunctionExpr, FunctionListExpr, FunctionReference, IntExpr, PanicExpr, Step,
    StringExpr, TupleExpr,
};
use ecow::EcoString;
use num_bigint::BigInt;

pub struct BoolFunctionExpr {
    kind: BoolFunctionExprKind,
}

pub(crate) enum BoolFunctionExprKind {
    Reference(FunctionReference<BoolFunctionId>),
    Closure(ClosureTemplate<BoolFunctionId>),
    LocalGet {
        local: BoolFunctionLocalId,
    },
    Call {
        function: BoolFunctionFunctionId,
        args: Vec<crate::plan::execution::CallArg>,
    },
    FunctionCall {
        function: Box<FunctionFunctionExpr>,
        args: Vec<crate::plan::execution::CallArg>,
    },
    TupleIndex {
        tuple: Box<TupleExpr>,
        index: usize,
        type_: FunctionType,
    },
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

    pub(crate) fn kind(&self) -> &BoolFunctionExprKind {
        &self.kind
    }
}
