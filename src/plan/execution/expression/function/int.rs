use crate::plan::execution::FunctionType;
use crate::plan::execution::{
    BoolExpr, ClosureTemplate, FloatExpr, FunctionFunctionExpr, FunctionListExpr,
    FunctionReference, IntExpr, IntFunctionFunctionId, IntFunctionId, IntFunctionLocalId,
    PanicExpr, Step, StringExpr, TupleExpr,
};
use ecow::EcoString;
use num_bigint::BigInt;

pub struct IntFunctionExpr {
    kind: IntFunctionExprKind,
}

pub(crate) enum IntFunctionExprKind {
    Reference(FunctionReference<IntFunctionId>),
    Closure(ClosureTemplate<IntFunctionId>),
    LocalGet {
        local: IntFunctionLocalId,
    },
    Call {
        function: IntFunctionFunctionId,
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
        true_: Box<IntFunctionExpr>,
        false_: Box<IntFunctionExpr>,
    },
    IntCase {
        subject: Box<IntExpr>,
        clauses: Vec<(BigInt, IntFunctionExpr)>,
        fallback: Box<IntFunctionExpr>,
    },
    StringCase {
        subject: Box<StringExpr>,
        clauses: Vec<(EcoString, IntFunctionExpr)>,
        fallback: Box<IntFunctionExpr>,
    },
    FloatCase {
        subject: Box<FloatExpr>,
        clauses: Vec<(f64, IntFunctionExpr)>,
        fallback: Box<IntFunctionExpr>,
    },
    Block {
        steps: Vec<Step>,
        return_: Box<IntFunctionExpr>,
    },
}

impl IntFunctionExpr {
    pub(in crate::plan::execution) fn from_kind(kind: IntFunctionExprKind) -> Self {
        Self { kind }
    }

    pub(crate) fn kind(&self) -> &IntFunctionExprKind {
        &self.kind
    }
}
