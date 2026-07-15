use crate::plan::execution::CustomFieldAccess;
use crate::plan::execution::FunctionType;
use crate::plan::execution::{
    BoolExpr, ClosureTemplate, FloatExpr, FunctionFunctionExpr, FunctionListExpr,
    FunctionReference, IntExpr, PanicExpr, Step, StringExpr, TupleExpr,
    UtfCodepointFunctionFunctionId, UtfCodepointFunctionId, UtfCodepointFunctionLocalId,
};
use ecow::EcoString;
use num_bigint::BigInt;

pub struct UtfCodepointFunctionExpr {
    kind: UtfCodepointFunctionExprKind,
}

pub(crate) enum UtfCodepointFunctionExprKind {
    Reference(FunctionReference<UtfCodepointFunctionId>),
    Closure(ClosureTemplate<UtfCodepointFunctionId>),
    LocalGet {
        local: UtfCodepointFunctionLocalId,
    },
    Call {
        function: UtfCodepointFunctionFunctionId,
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
        true_: Box<UtfCodepointFunctionExpr>,
        false_: Box<UtfCodepointFunctionExpr>,
    },
    IntCase {
        subject: Box<IntExpr>,
        clauses: Vec<(BigInt, UtfCodepointFunctionExpr)>,
        fallback: Box<UtfCodepointFunctionExpr>,
    },
    StringCase {
        subject: Box<StringExpr>,
        clauses: Vec<(EcoString, UtfCodepointFunctionExpr)>,
        fallback: Box<UtfCodepointFunctionExpr>,
    },
    FloatCase {
        subject: Box<FloatExpr>,
        clauses: Vec<(f64, UtfCodepointFunctionExpr)>,
        fallback: Box<UtfCodepointFunctionExpr>,
    },
    Block {
        steps: Vec<Step>,
        return_: Box<UtfCodepointFunctionExpr>,
    },
}

impl UtfCodepointFunctionExpr {
    pub(in crate::plan::execution) fn from_kind(kind: UtfCodepointFunctionExprKind) -> Self {
        Self { kind }
    }

    pub(crate) fn kind(&self) -> &UtfCodepointFunctionExprKind {
        &self.kind
    }
}
