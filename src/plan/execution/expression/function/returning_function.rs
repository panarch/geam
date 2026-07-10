use crate::plan::FunctionType;
use crate::plan::execution::{
    BoolExpr, ClosureTemplate, FloatExpr, FunctionFunctionFunctionId, FunctionFunctionId,
    FunctionFunctionLocalId, FunctionListExpr, FunctionReference, IntExpr, PanicExpr, Step,
    StringExpr, TupleExpr,
};
use ecow::EcoString;
use num_bigint::BigInt;

pub struct FunctionFunctionExpr {
    type_: FunctionType,
    kind: FunctionFunctionExprKind,
}

pub(crate) enum FunctionFunctionExprKind {
    Reference(FunctionReference<FunctionFunctionId>),
    Closure(ClosureTemplate<FunctionFunctionId>),
    LocalGet {
        local: FunctionFunctionLocalId,
    },
    Call {
        function: FunctionFunctionFunctionId,
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
        true_: Box<FunctionFunctionExpr>,
        false_: Box<FunctionFunctionExpr>,
    },
    IntCase {
        subject: Box<IntExpr>,
        clauses: Vec<(BigInt, FunctionFunctionExpr)>,
        fallback: Box<FunctionFunctionExpr>,
    },
    StringCase {
        subject: Box<StringExpr>,
        clauses: Vec<(EcoString, FunctionFunctionExpr)>,
        fallback: Box<FunctionFunctionExpr>,
    },
    FloatCase {
        subject: Box<FloatExpr>,
        clauses: Vec<(f64, FunctionFunctionExpr)>,
        fallback: Box<FunctionFunctionExpr>,
    },
    Block {
        steps: Vec<Step>,
        return_: Box<FunctionFunctionExpr>,
    },
}

impl FunctionFunctionExpr {
    pub(in crate::plan::execution) fn from_parts(
        type_: FunctionType,
        kind: FunctionFunctionExprKind,
    ) -> Self {
        Self { type_, kind }
    }

    pub fn type_(&self) -> &FunctionType {
        &self.type_
    }

    pub(crate) fn kind(&self) -> &FunctionFunctionExprKind {
        &self.kind
    }
}
