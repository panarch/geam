use crate::plan::execution::{
    BoolExpr, CustomFieldAccess, FloatExpr, FunctionFunctionExpr, FunctionListExpr,
    FunctionReference, GenericFunctionType, IntExpr, NeverFunctionFunctionId, NeverFunctionId,
    NeverFunctionLocal, PanicExpr, Step, StringExpr, TupleExpr,
};
use ecow::EcoString;
use num_bigint::BigInt;

pub struct NeverFunctionExpr {
    type_: GenericFunctionType,
    kind: NeverFunctionExprKind,
}

pub(crate) enum NeverFunctionExprKind {
    Constant(crate::plan::execution::ConstantId<NeverFunctionExpr>),
    Reference(FunctionReference<NeverFunctionId>),
    Closure(crate::plan::execution::ClosureTemplate<NeverFunctionId>),
    LocalGet {
        local: NeverFunctionLocal,
    },
    Call(crate::plan::execution::DirectCall<NeverFunctionFunctionId>),
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
        true_: Box<NeverFunctionExprKind>,
        false_: Box<NeverFunctionExprKind>,
    },
    IntCase {
        subject: Box<IntExpr>,
        clauses: Vec<(BigInt, NeverFunctionExprKind)>,
        fallback: Box<NeverFunctionExprKind>,
    },
    StringCase {
        subject: Box<StringExpr>,
        clauses: Vec<(EcoString, NeverFunctionExprKind)>,
        fallback: Box<NeverFunctionExprKind>,
    },
    FloatCase {
        subject: Box<FloatExpr>,
        clauses: Vec<(f64, NeverFunctionExprKind)>,
        fallback: Box<NeverFunctionExprKind>,
    },
    Block {
        steps: Vec<Step>,
        return_: Box<NeverFunctionExprKind>,
    },
}

impl NeverFunctionExpr {
    pub(in crate::plan::execution) fn from_parts(
        type_: GenericFunctionType,
        kind: NeverFunctionExprKind,
    ) -> Self {
        Self { type_, kind }
    }

    pub(crate) fn type_(&self) -> &GenericFunctionType {
        &self.type_
    }

    pub(crate) fn kind(&self) -> &NeverFunctionExprKind {
        &self.kind
    }

    pub(in crate::plan::execution) fn into_kind(self) -> NeverFunctionExprKind {
        self.kind
    }
}
