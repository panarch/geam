use crate::plan::FunctionType;
use crate::plan::execution::{
    BoolExpr, ClosureTemplate, FloatExpr, FunctionFunctionExpr, FunctionListExpr,
    FunctionReference, IntExpr, PanicExpr, Step, StringExpr, TupleExpr, TupleFunctionFunctionId,
    TupleFunctionId, TupleFunctionLocalId,
};
use ecow::EcoString;
use num_bigint::BigInt;

pub struct TupleFunctionExpr {
    type_: FunctionType,
    kind: TupleFunctionExprKind,
}

pub(crate) enum TupleFunctionExprKind {
    Reference(FunctionReference<TupleFunctionId>),
    Closure(ClosureTemplate<TupleFunctionId>),
    LocalGet {
        local: TupleFunctionLocalId,
    },
    Call {
        function: TupleFunctionFunctionId,
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
        true_: Box<TupleFunctionExpr>,
        false_: Box<TupleFunctionExpr>,
    },
    IntCase {
        subject: Box<IntExpr>,
        clauses: Vec<(BigInt, TupleFunctionExpr)>,
        fallback: Box<TupleFunctionExpr>,
    },
    StringCase {
        subject: Box<StringExpr>,
        clauses: Vec<(EcoString, TupleFunctionExpr)>,
        fallback: Box<TupleFunctionExpr>,
    },
    FloatCase {
        subject: Box<FloatExpr>,
        clauses: Vec<(f64, TupleFunctionExpr)>,
        fallback: Box<TupleFunctionExpr>,
    },
    Block {
        steps: Vec<Step>,
        return_: Box<TupleFunctionExpr>,
    },
}

impl TupleFunctionExpr {
    pub(in crate::plan::execution) fn from_parts(
        type_: FunctionType,
        kind: TupleFunctionExprKind,
    ) -> Self {
        Self { type_, kind }
    }

    pub fn type_(&self) -> &FunctionType {
        &self.type_
    }

    pub(crate) fn kind(&self) -> &TupleFunctionExprKind {
        &self.kind
    }
}
