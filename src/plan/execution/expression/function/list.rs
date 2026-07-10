use crate::plan::FunctionType;
use crate::plan::execution::{
    BoolExpr, ClosureTemplate, FloatExpr, FunctionFunctionExpr, FunctionListExpr,
    FunctionReference, IntExpr, ListFunctionFunctionId, ListFunctionId, ListFunctionLocal,
    PanicExpr, Step, StringExpr, TupleExpr,
};
use ecow::EcoString;
use num_bigint::BigInt;

pub struct ListFunctionExpr {
    kind: ListFunctionExprKind,
}

pub(crate) enum ListFunctionExprKind {
    Reference(FunctionReference<ListFunctionId>),
    Closure(ClosureTemplate<ListFunctionId>),
    LocalGet {
        local: ListFunctionLocal,
    },
    Call {
        function: ListFunctionFunctionId,
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
        true_: Box<ListFunctionExpr>,
        false_: Box<ListFunctionExpr>,
    },
    IntCase {
        subject: Box<IntExpr>,
        clauses: Vec<(BigInt, ListFunctionExpr)>,
        fallback: Box<ListFunctionExpr>,
    },
    StringCase {
        subject: Box<StringExpr>,
        clauses: Vec<(EcoString, ListFunctionExpr)>,
        fallback: Box<ListFunctionExpr>,
    },
    FloatCase {
        subject: Box<FloatExpr>,
        clauses: Vec<(f64, ListFunctionExpr)>,
        fallback: Box<ListFunctionExpr>,
    },
    Block {
        steps: Vec<Step>,
        return_: Box<ListFunctionExpr>,
    },
}

impl ListFunctionExpr {
    pub(in crate::plan::execution) fn from_kind(kind: ListFunctionExprKind) -> Self {
        Self { kind }
    }

    pub(crate) fn kind(&self) -> &ListFunctionExprKind {
        &self.kind
    }
}
