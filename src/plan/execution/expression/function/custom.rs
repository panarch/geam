use crate::plan::execution::{
    BoolExpr, ClosureTemplate, CustomConstructorId, CustomFunctionFunctionId, CustomFunctionId,
    CustomFunctionLocalId, FloatExpr, FunctionFunctionExpr, FunctionListExpr, FunctionReference,
    FunctionType, IntExpr, PanicExpr, Step, StringExpr, TupleExpr,
};
use ecow::EcoString;
use num_bigint::BigInt;

pub struct CustomFunctionExpr {
    type_: FunctionType,
    kind: CustomFunctionExprKind,
}

pub(crate) enum CustomFunctionExprKind {
    Constructor(CustomConstructorId),
    Reference(FunctionReference<CustomFunctionId>),
    Closure(ClosureTemplate<CustomFunctionId>),
    LocalGet {
        local: CustomFunctionLocalId,
    },
    Call {
        function: CustomFunctionFunctionId,
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
        true_: Box<CustomFunctionExpr>,
        false_: Box<CustomFunctionExpr>,
    },
    IntCase {
        subject: Box<IntExpr>,
        clauses: Vec<(BigInt, CustomFunctionExpr)>,
        fallback: Box<CustomFunctionExpr>,
    },
    StringCase {
        subject: Box<StringExpr>,
        clauses: Vec<(EcoString, CustomFunctionExpr)>,
        fallback: Box<CustomFunctionExpr>,
    },
    FloatCase {
        subject: Box<FloatExpr>,
        clauses: Vec<(f64, CustomFunctionExpr)>,
        fallback: Box<CustomFunctionExpr>,
    },
    Block {
        steps: Vec<Step>,
        return_: Box<CustomFunctionExpr>,
    },
}

impl CustomFunctionExpr {
    pub(in crate::plan::execution) fn from_parts(
        type_: FunctionType,
        kind: CustomFunctionExprKind,
    ) -> Self {
        Self { type_, kind }
    }

    pub(crate) fn type_(&self) -> &FunctionType {
        &self.type_
    }

    pub(crate) fn kind(&self) -> &CustomFunctionExprKind {
        &self.kind
    }
}
