use super::{
    BoolExpr, CallArg, CustomFunctionExpr, CustomListExpr, FloatExpr, IntExpr, PanicExpr,
    StringExpr, TupleExpr,
};
use crate::plan::{CustomConstructor, CustomFunctionId, CustomLocalId, CustomType, Step};
use ecow::EcoString;
use num_bigint::BigInt;

#[derive(Debug, Clone, PartialEq)]
pub struct CustomExpr {
    type_: CustomType,
    kind: CustomExprKind,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum CustomExprKind {
    Constructor {
        constructor: CustomConstructor,
        arguments: Vec<super::Expr>,
    },
    LocalGet {
        local: CustomLocalId,
        name: EcoString,
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
    pub(crate) fn constructor(constructor: CustomConstructor, arguments: Vec<super::Expr>) -> Self {
        Self::new(
            constructor.type_().clone(),
            CustomExprKind::Constructor {
                constructor,
                arguments,
            },
        )
    }

    pub(crate) fn local_get(local: CustomLocalId, name: EcoString, type_: CustomType) -> Self {
        Self::new(type_, CustomExprKind::LocalGet { local, name })
    }

    pub(crate) fn call(function: CustomFunctionId, args: Vec<CallArg>, type_: CustomType) -> Self {
        Self::new(type_, CustomExprKind::Call { function, args })
    }

    pub(crate) fn function_call(
        function: CustomFunctionExpr,
        args: Vec<CallArg>,
        type_: CustomType,
    ) -> Self {
        Self::new(
            type_,
            CustomExprKind::FunctionCall {
                function: Box::new(function),
                args,
            },
        )
    }

    pub(crate) fn tuple_index(tuple: TupleExpr, index: usize, type_: CustomType) -> Self {
        Self::new(
            type_,
            CustomExprKind::TupleIndex {
                tuple: Box::new(tuple),
                index,
            },
        )
    }

    pub(crate) fn list_index(list: CustomListExpr, index: usize, type_: CustomType) -> Self {
        Self::new(
            type_,
            CustomExprKind::ListIndex {
                list: Box::new(list),
                index,
            },
        )
    }

    pub(crate) fn panic(panic: PanicExpr, type_: CustomType) -> Self {
        Self::new(type_, CustomExprKind::Panic(panic))
    }

    pub(crate) fn bool_case(subject: BoolExpr, true_: Self, false_: Self) -> Self {
        Self::new(
            true_.type_.clone(),
            CustomExprKind::BoolCase {
                subject: Box::new(subject),
                true_: Box::new(true_),
                false_: Box::new(false_),
            },
        )
    }

    pub(crate) fn int_case(subject: IntExpr, clauses: Vec<(BigInt, Self)>, fallback: Self) -> Self {
        Self::new(
            fallback.type_.clone(),
            CustomExprKind::IntCase {
                subject: Box::new(subject),
                clauses,
                fallback: Box::new(fallback),
            },
        )
    }

    pub(crate) fn string_case(
        subject: StringExpr,
        clauses: Vec<(EcoString, Self)>,
        fallback: Self,
    ) -> Self {
        Self::new(
            fallback.type_.clone(),
            CustomExprKind::StringCase {
                subject: Box::new(subject),
                clauses,
                fallback: Box::new(fallback),
            },
        )
    }

    pub(crate) fn float_case(
        subject: FloatExpr,
        clauses: Vec<(f64, Self)>,
        fallback: Self,
    ) -> Self {
        Self::new(
            fallback.type_.clone(),
            CustomExprKind::FloatCase {
                subject: Box::new(subject),
                clauses,
                fallback: Box::new(fallback),
            },
        )
    }

    pub(crate) fn block(steps: Vec<Step>, return_: Self) -> Self {
        Self::new(
            return_.type_.clone(),
            CustomExprKind::Block {
                steps,
                return_: Box::new(return_),
            },
        )
    }

    pub fn type_(&self) -> &CustomType {
        &self.type_
    }
    pub(crate) fn kind(&self) -> &CustomExprKind {
        &self.kind
    }
    pub(crate) fn into_parts(self) -> (CustomType, CustomExprKind) {
        (self.type_, self.kind)
    }

    fn new(type_: CustomType, kind: CustomExprKind) -> Self {
        Self { type_, kind }
    }
}
