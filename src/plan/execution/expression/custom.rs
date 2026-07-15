use super::{
    BoolExpr, CallArg, CustomFieldAccess, CustomFunctionExpr, CustomListExpr, FloatExpr, IntExpr,
    PanicExpr, StringExpr, TupleExpr,
};
use crate::plan::execution::{
    CustomConstructorId, CustomFunctionId, CustomLocal, CustomTypeId, Step,
};
use ecow::EcoString;
use num_bigint::BigInt;

pub struct CustomExpr {
    type_id: CustomTypeId,
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

pub(crate) struct CustomCallArguments {
    values: Box<[CallArg]>,
}

pub(crate) struct CustomFunctionCall {
    function: Box<CustomFunctionExpr>,
    arguments: CustomCallArguments,
}

pub(crate) enum CustomExprKind {
    Constructor(CustomConstruction),
    LocalGet {
        local: CustomLocal,
    },
    Call {
        function: CustomFunctionId,
        args: Vec<CallArg>,
    },
    FunctionCall(CustomFunctionCall),
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

impl CustomFunctionCall {
    pub(in crate::plan::execution) fn from_parts(
        function: CustomFunctionExpr,
        arguments: Box<[CallArg]>,
    ) -> Self {
        Self {
            function: Box::new(function),
            arguments: CustomCallArguments { values: arguments },
        }
    }

    pub(crate) fn function(&self) -> &CustomFunctionExpr {
        &self.function
    }

    pub(crate) fn arguments(&self) -> &[CallArg] {
        &self.arguments.values
    }
}
