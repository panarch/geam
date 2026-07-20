use crate::plan::execution::{
    BoolExpr, CaptureArg, ConstantId, CustomFieldAccess, DirectCall, FloatExpr, FunctionCall,
    FunctionFunctionExpr, FunctionListExpr, GenericCallableId, GenericFunctionFunctionId,
    GenericFunctionLocal, GenericFunctionType, IntExpr, PanicExpr, Step, StringExpr, TupleExpr,
};
use ecow::EcoString;
use num_bigint::BigInt;

pub struct GenericFunctionExpr {
    type_: GenericFunctionType,
    kind: GenericFunctionExprKind,
}

pub(crate) enum GenericFunctionExprKind {
    Constant(ConstantId<GenericFunctionExpr>),
    Reference {
        target: GenericCallableId,
    },
    Constructor {
        target: GenericCallableId,
    },
    Closure {
        target: GenericCallableId,
        captures: Vec<CaptureArg>,
    },
    LocalGet {
        local: GenericFunctionLocal,
    },
    Call(DirectCall<GenericFunctionFunctionId>),
    FunctionCall(FunctionCall<FunctionFunctionExpr>),
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
        true_: Box<GenericFunctionExprKind>,
        false_: Box<GenericFunctionExprKind>,
    },
    IntCase {
        subject: Box<IntExpr>,
        clauses: Vec<(BigInt, GenericFunctionExprKind)>,
        fallback: Box<GenericFunctionExprKind>,
    },
    StringCase {
        subject: Box<StringExpr>,
        clauses: Vec<(EcoString, GenericFunctionExprKind)>,
        fallback: Box<GenericFunctionExprKind>,
    },
    FloatCase {
        subject: Box<FloatExpr>,
        clauses: Vec<(f64, GenericFunctionExprKind)>,
        fallback: Box<GenericFunctionExprKind>,
    },
    Block {
        steps: Vec<Step>,
        return_: Box<GenericFunctionExprKind>,
    },
}

impl GenericFunctionExpr {
    pub(in crate::plan::execution) fn from_parts(
        type_: GenericFunctionType,
        kind: GenericFunctionExprKind,
    ) -> Self {
        Self { type_, kind }
    }

    pub(crate) fn generic_function_type(&self) -> &GenericFunctionType {
        &self.type_
    }

    pub(crate) fn kind(&self) -> &GenericFunctionExprKind {
        &self.kind
    }

    pub(in crate::plan::execution) fn into_kind(self) -> GenericFunctionExprKind {
        self.kind
    }
}
