use crate::plan::execution::CustomFieldAccess;
use crate::plan::execution::FunctionType;
use crate::plan::execution::{
    BitArrayFunctionFunctionId, BitArrayFunctionId, BitArrayFunctionLocalId, BoolExpr,
    ClosureTemplate, FloatExpr, FunctionFunctionExpr, FunctionListExpr, FunctionReference, IntExpr,
    PanicExpr, Step, StringExpr, TupleExpr,
};
use ecow::EcoString;
use num_bigint::BigInt;

pub struct BitArrayFunctionExpr {
    kind: BitArrayFunctionExprKind,
}

pub(crate) enum BitArrayFunctionExprKind {
    Reference(FunctionReference<BitArrayFunctionId>),
    Closure(ClosureTemplate<BitArrayFunctionId>),
    LocalGet {
        local: BitArrayFunctionLocalId,
    },
    Call {
        function: BitArrayFunctionFunctionId,
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
    CustomField(CustomFieldAccess),
    ListIndex {
        list: Box<FunctionListExpr>,
        index: usize,
        type_: FunctionType,
    },
    Panic(PanicExpr),
    BoolCase {
        subject: Box<BoolExpr>,
        true_: Box<BitArrayFunctionExpr>,
        false_: Box<BitArrayFunctionExpr>,
    },
    IntCase {
        subject: Box<IntExpr>,
        clauses: Vec<(BigInt, BitArrayFunctionExpr)>,
        fallback: Box<BitArrayFunctionExpr>,
    },
    StringCase {
        subject: Box<StringExpr>,
        clauses: Vec<(EcoString, BitArrayFunctionExpr)>,
        fallback: Box<BitArrayFunctionExpr>,
    },
    FloatCase {
        subject: Box<FloatExpr>,
        clauses: Vec<(f64, BitArrayFunctionExpr)>,
        fallback: Box<BitArrayFunctionExpr>,
    },
    Block {
        steps: Vec<Step>,
        return_: Box<BitArrayFunctionExpr>,
    },
}

impl BitArrayFunctionExpr {
    pub(in crate::plan::execution) fn from_kind(kind: BitArrayFunctionExprKind) -> Self {
        Self { kind }
    }

    pub(crate) fn kind(&self) -> &BitArrayFunctionExprKind {
        &self.kind
    }
}
