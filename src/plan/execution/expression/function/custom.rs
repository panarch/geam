use crate::plan::execution::{
    BoolExpr, ClosureTemplate, ConstantId, CustomConstructorId, CustomFieldAccess,
    CustomFunctionFunctionId, CustomFunctionId, CustomFunctionLocal, CustomFunctionType,
    DirectCall, FloatExpr, FunctionCall, FunctionFunctionExpr, FunctionListExpr, FunctionReference,
    IntExpr, PanicExpr, Step, StringExpr, TupleExpr,
};
use ecow::EcoString;
use num_bigint::BigInt;

pub struct CustomFunctionExpr {
    type_: CustomFunctionType,
    kind: CustomFunctionExprKind,
}

pub(crate) enum CustomFunctionExprKind {
    Constant(ConstantId<CustomFunctionExpr>),
    Constructor(CustomConstructorId),
    Reference(FunctionReference<CustomFunctionId>),
    Closure(ClosureTemplate<CustomFunctionId>),
    LocalGet {
        local: CustomFunctionLocal,
    },
    Call(DirectCall<CustomFunctionFunctionId>),
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
        true_: Box<CustomFunctionExprKind>,
        false_: Box<CustomFunctionExprKind>,
    },
    IntCase {
        subject: Box<IntExpr>,
        clauses: Vec<(BigInt, CustomFunctionExprKind)>,
        fallback: Box<CustomFunctionExprKind>,
    },
    StringCase {
        subject: Box<StringExpr>,
        clauses: Vec<(EcoString, CustomFunctionExprKind)>,
        fallback: Box<CustomFunctionExprKind>,
    },
    FloatCase {
        subject: Box<FloatExpr>,
        clauses: Vec<(f64, CustomFunctionExprKind)>,
        fallback: Box<CustomFunctionExprKind>,
    },
    Block {
        steps: Vec<Step>,
        return_: Box<CustomFunctionExprKind>,
    },
}

impl CustomFunctionExpr {
    pub(in crate::plan::execution) fn from_parts(
        type_: CustomFunctionType,
        kind: CustomFunctionExprKind,
    ) -> Self {
        Self { type_, kind }
    }

    pub(crate) fn custom_function_type(&self) -> &CustomFunctionType {
        &self.type_
    }

    pub(crate) fn kind(&self) -> &CustomFunctionExprKind {
        &self.kind
    }
}
