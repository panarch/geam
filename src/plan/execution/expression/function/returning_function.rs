use crate::plan::execution::CustomFieldAccess;
use crate::plan::execution::{
    BoolExpr, FloatExpr, FunctionFunctionFunctionId, FunctionFunctionId, FunctionFunctionLocal,
    FunctionFunctionType, FunctionListExpr, FunctionReference, IntExpr, PanicExpr, Step,
    StringExpr, TupleExpr,
};
use ecow::EcoString;
use num_bigint::BigInt;

pub struct FunctionFunctionExpr {
    type_: FunctionFunctionType,
    kind: FunctionFunctionExprKind,
}

pub(crate) enum FunctionFunctionExprKind {
    Constant(crate::plan::execution::ConstantId<FunctionFunctionExpr>),
    Reference(FunctionReference<FunctionFunctionId>),
    Closure(crate::plan::execution::ClosureTemplate<FunctionFunctionId>),
    LocalGet {
        local: FunctionFunctionLocal,
    },
    Call(crate::plan::execution::DirectCall<FunctionFunctionFunctionId>),
    FunctionCall(crate::plan::execution::FunctionCall<FunctionFunctionExpr>),
    TupleIndex {
        tuple: Box<TupleExpr>,
        index: usize,
    },
    CustomField(CustomFieldAccess),
    ListIndex {
        list: Box<FunctionListExpr>,
        index: usize,
    },
    Panic(PanicExpr),
    BoolCase {
        subject: Box<BoolExpr>,
        true_: Box<FunctionFunctionExprKind>,
        false_: Box<FunctionFunctionExprKind>,
    },
    IntCase {
        subject: Box<IntExpr>,
        clauses: Vec<(BigInt, FunctionFunctionExprKind)>,
        fallback: Box<FunctionFunctionExprKind>,
    },
    StringCase {
        subject: Box<StringExpr>,
        clauses: Vec<(EcoString, FunctionFunctionExprKind)>,
        fallback: Box<FunctionFunctionExprKind>,
    },
    FloatCase {
        subject: Box<FloatExpr>,
        clauses: Vec<(f64, FunctionFunctionExprKind)>,
        fallback: Box<FunctionFunctionExprKind>,
    },
    Block {
        steps: Vec<Step>,
        return_: Box<FunctionFunctionExprKind>,
    },
}

impl FunctionFunctionExpr {
    pub(in crate::plan::execution) fn from_parts(
        type_: FunctionFunctionType,
        kind: FunctionFunctionExprKind,
    ) -> Self {
        Self { type_, kind }
    }

    pub(crate) fn function_function_type(&self) -> &FunctionFunctionType {
        &self.type_
    }

    pub(crate) fn kind(&self) -> &FunctionFunctionExprKind {
        &self.kind
    }
}
