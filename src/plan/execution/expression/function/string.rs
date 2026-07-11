use crate::plan::execution::FunctionType;
use crate::plan::execution::{
    BoolExpr, ClosureTemplate, FloatExpr, FunctionFunctionExpr, FunctionListExpr,
    FunctionReference, IntExpr, PanicExpr, Step, StringExpr, StringFunctionFunctionId,
    StringFunctionId, StringFunctionLocalId, TupleExpr,
};
use ecow::EcoString;
use num_bigint::BigInt;

pub struct StringFunctionExpr {
    kind: StringFunctionExprKind,
}

pub(crate) enum StringFunctionExprKind {
    Reference(FunctionReference<StringFunctionId>),
    Closure(ClosureTemplate<StringFunctionId>),
    LocalGet {
        local: StringFunctionLocalId,
    },
    Call {
        function: StringFunctionFunctionId,
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
        true_: Box<StringFunctionExpr>,
        false_: Box<StringFunctionExpr>,
    },
    IntCase {
        subject: Box<IntExpr>,
        clauses: Vec<(BigInt, StringFunctionExpr)>,
        fallback: Box<StringFunctionExpr>,
    },
    StringCase {
        subject: Box<StringExpr>,
        clauses: Vec<(EcoString, StringFunctionExpr)>,
        fallback: Box<StringFunctionExpr>,
    },
    FloatCase {
        subject: Box<FloatExpr>,
        clauses: Vec<(f64, StringFunctionExpr)>,
        fallback: Box<StringFunctionExpr>,
    },
    Block {
        steps: Vec<Step>,
        return_: Box<StringFunctionExpr>,
    },
}

impl StringFunctionExpr {
    pub(in crate::plan::execution) fn from_kind(kind: StringFunctionExprKind) -> Self {
        Self { kind }
    }

    pub(crate) fn kind(&self) -> &StringFunctionExprKind {
        &self.kind
    }
}
