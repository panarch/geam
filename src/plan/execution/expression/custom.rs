use super::{
    BoolExpr, CallArg, CustomFieldAccess, CustomFunctionExpr, CustomListExpr, FloatExpr, IntExpr,
    PanicExpr, StringExpr, TupleExpr,
};
use crate::plan::execution::{
    CustomConstructorId, CustomFunctionId, CustomLocalId, CustomTypeId, Step,
};
use ecow::EcoString;
use num_bigint::BigInt;

pub struct CustomExpr {
    type_id: CustomTypeId,
    kind: CustomExprKind,
}

pub(crate) enum CustomExprKind {
    Constructor {
        constructor: CustomConstructorId,
        arguments: Vec<super::Expr>,
    },
    LocalGet {
        local: CustomLocalId,
    },
    Call {
        function: CustomFunctionId,
        args: Vec<CallArg>,
    },
    FunctionCall {
        function: Box<CustomFunctionExpr>,
        args: Vec<CallArg>,
    },
    TupleIndex {
        tuple: Box<TupleExpr>,
        index: usize,
    },
    CustomField(CustomFieldAccess),
    ListIndex {
        list: Box<CustomListExpr>,
        index: usize,
    },
    Panic(PanicExpr),
    BoolCase {
        subject: Box<BoolExpr>,
        true_: Box<CustomExpr>,
        false_: Box<CustomExpr>,
    },
    IntCase {
        subject: Box<IntExpr>,
        clauses: Vec<(BigInt, CustomExpr)>,
        fallback: Box<CustomExpr>,
    },
    StringCase {
        subject: Box<StringExpr>,
        clauses: Vec<(EcoString, CustomExpr)>,
        fallback: Box<CustomExpr>,
    },
    FloatCase {
        subject: Box<FloatExpr>,
        clauses: Vec<(f64, CustomExpr)>,
        fallback: Box<CustomExpr>,
    },
    Block {
        steps: Vec<Step>,
        return_: Box<CustomExpr>,
    },
}

impl CustomExpr {
    pub(in crate::plan::execution) fn from_parts(
        type_id: CustomTypeId,
        kind: CustomExprKind,
    ) -> Self {
        Self { type_id, kind }
    }

    pub(crate) fn type_id(&self) -> CustomTypeId {
        self.type_id
    }

    pub(crate) fn kind(&self) -> &CustomExprKind {
        &self.kind
    }
}
