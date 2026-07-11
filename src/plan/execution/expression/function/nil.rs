use crate::plan::execution::FunctionType;
use crate::plan::execution::{
    BoolExpr, ClosureTemplate, FloatExpr, FunctionFunctionExpr, FunctionListExpr,
    FunctionReference, IntExpr, NilFunctionFunctionId, NilFunctionId, NilFunctionLocalId,
    PanicExpr, Step, StringExpr, TupleExpr,
};
use ecow::EcoString;
use num_bigint::BigInt;

pub struct NilFunctionExpr {
    kind: NilFunctionExprKind,
}

pub(crate) enum NilFunctionExprKind {
    Reference(FunctionReference<NilFunctionId>),
    Closure(ClosureTemplate<NilFunctionId>),
    LocalGet {
        local: NilFunctionLocalId,
    },
    Call {
        function: NilFunctionFunctionId,
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

    pub(crate) fn kind(&self) -> &NilFunctionExprKind {
        &self.kind
    }
}
