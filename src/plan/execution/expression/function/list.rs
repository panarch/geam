use crate::plan::execution::FunctionType;
use crate::plan::execution::{
    BoolExpr, CustomFieldAccess, FloatExpr, FunctionFunctionExpr, FunctionListExpr,
    FunctionReference, IntExpr, ListFunctionFunctionId, ListFunctionId, ListFunctionLocal,
    PanicExpr, Step, StringExpr, TupleExpr,
};
use ecow::EcoString;
use num_bigint::BigInt;

pub struct ListFunctionExpr {
    kind: ListFunctionExprKind,
}

pub(crate) enum ListFunctionExprKind {
    Constant(crate::plan::execution::ConstantId<ListFunctionExpr>),
    Reference(FunctionReference<ListFunctionId>),
    Closure(crate::plan::execution::ClosureTemplate<ListFunctionId>),
    LocalGet {
        local: ListFunctionLocal,
    },
    Call(crate::plan::execution::DirectCall<ListFunctionFunctionId>),
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

    pub(in crate::plan::execution) fn into_kind(self) -> ListFunctionExprKind {
        self.kind
    }

    pub(crate) fn kind(&self) -> &ListFunctionExprKind {
        &self.kind
    }
}
