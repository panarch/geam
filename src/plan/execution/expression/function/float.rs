use crate::plan::FunctionType;
use crate::plan::execution::{
    BoolExpr, ClosureTemplate, FloatExpr, FloatFunctionFunctionId, FloatFunctionId,
    FloatFunctionLocalId, FunctionFunctionExpr, FunctionListExpr, FunctionReference, IntExpr,
    PanicExpr, Step, StringExpr, TupleExpr,
};
use ecow::EcoString;
use num_bigint::BigInt;

pub struct FloatFunctionExpr {
    kind: FloatFunctionExprKind,
}

pub(crate) enum FloatFunctionExprKind {
    Reference(FunctionReference<FloatFunctionId>),
    Closure(ClosureTemplate<FloatFunctionId>),
    LocalGet {
        local: FloatFunctionLocalId,
    },
    Call {
        function: FloatFunctionFunctionId,
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
        true_: Box<FloatFunctionExpr>,
        false_: Box<FloatFunctionExpr>,
    },
    IntCase {
        subject: Box<IntExpr>,
        clauses: Vec<(BigInt, FloatFunctionExpr)>,
        fallback: Box<FloatFunctionExpr>,
    },
    StringCase {
        subject: Box<StringExpr>,
        clauses: Vec<(EcoString, FloatFunctionExpr)>,
        fallback: Box<FloatFunctionExpr>,
    },
    FloatCase {
        subject: Box<FloatExpr>,
        clauses: Vec<(f64, FloatFunctionExpr)>,
        fallback: Box<FloatFunctionExpr>,
    },
    Block {
        steps: Vec<Step>,
        return_: Box<FloatFunctionExpr>,
    },
}

impl FloatFunctionExpr {
    pub(in crate::plan::execution) fn from_kind(kind: FloatFunctionExprKind) -> Self {
        Self { kind }
    }

    pub(crate) fn kind(&self) -> &FloatFunctionExprKind {
        &self.kind
    }
}
