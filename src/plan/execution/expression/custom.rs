use super::{
    BoolExpr, CustomFieldAccess, CustomFunctionExpr, CustomListExpr, DirectCall, FloatExpr,
    FunctionCall, IntExpr, NeverExpr, PanicExpr, StringExpr, TupleExpr,
};
use crate::plan::execution::{
    ConstantId, CustomConstructorId, CustomFunctionId, CustomLocal, CustomTypeId, CustomValueShape,
    Step,
};
use ecow::EcoString;
use num_bigint::BigInt;

pub struct CustomExpr {
    shape: CustomValueShape,
    kind: CustomExprKind,
}

pub(crate) struct CustomLocalExpr {
    local: CustomLocal,
    value: CustomExpr,
}

pub(crate) struct CustomConstruction {
    constructor: CustomConstructorId,
    fields: Box<[super::Expr]>,
}

pub(crate) enum CustomExprKind {
    Never(NeverExpr),
    Constructor(CustomConstruction),
    Constant(ConstantId<CustomExpr>),
    LocalGet {
        local: CustomLocal,
    },
    Call(DirectCall<CustomFunctionId>),
    FunctionCall(FunctionCall<CustomFunctionExpr>),
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
        true_: Box<CustomExprKind>,
        false_: Box<CustomExprKind>,
    },
    IntCase {
        subject: Box<IntExpr>,
        clauses: Vec<(BigInt, CustomExprKind)>,
        fallback: Box<CustomExprKind>,
    },
    StringCase {
        subject: Box<StringExpr>,
        clauses: Vec<(EcoString, CustomExprKind)>,
        fallback: Box<CustomExprKind>,
    },
    FloatCase {
        subject: Box<FloatExpr>,
        clauses: Vec<(f64, CustomExprKind)>,
        fallback: Box<CustomExprKind>,
    },
    Block {
        steps: Vec<Step>,
        return_: Box<CustomExprKind>,
    },
}

impl CustomExpr {
    pub(in crate::plan::execution) fn from_parts(
        shape: CustomValueShape,
        kind: CustomExprKind,
    ) -> Self {
        Self { shape, kind }
    }

    pub(crate) fn type_id(&self) -> CustomTypeId {
        self.shape.type_id()
    }

    pub(crate) fn kind(&self) -> &CustomExprKind {
        &self.kind
    }
}

impl CustomLocalExpr {
    pub(in crate::plan::execution) fn new(local: CustomLocal, value: CustomExpr) -> Self {
        Self { local, value }
    }

    pub(crate) fn local(&self) -> CustomLocal {
        self.local
    }

    pub(crate) fn value(&self) -> &CustomExpr {
        &self.value
    }
}

impl CustomConstruction {
    pub(in crate::plan::execution) fn from_parts(
        constructor: CustomConstructorId,
        fields: Box<[super::Expr]>,
    ) -> Self {
        Self {
            constructor,
            fields,
        }
    }

    pub(crate) fn constructor(&self) -> CustomConstructorId {
        self.constructor
    }

    pub(crate) fn fields(&self) -> &[super::Expr] {
        &self.fields
    }
}
