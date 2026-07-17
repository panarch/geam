use crate::plan::{
    BoolExpr, CaptureArg, CustomFieldAccess, FloatExpr, FunctionFunctionExpr,
    FunctionInstantiation, FunctionListExpr, FunctionShape, GenericFunctionLocal,
    GenericFunctionReference, IntExpr, PanicExpr, ParamSlot, Step, StringExpr, TupleExpr,
    TypeParameterId,
};
use ecow::EcoString;
use num_bigint::BigInt;

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct GenericFunctionExpr {
    type_: crate::plan::GenericFunctionType,
    kind: GenericFunctionExprKind,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum GenericFunctionExprKind {
    Reference(GenericFunctionReference),
    Closure {
        function: FunctionInstantiation,
        params: Vec<ParamSlot>,
        captures: Vec<CaptureArg>,
    },
    LocalGet {
        local: GenericFunctionLocal,
        name: EcoString,
    },
    Call {
        function: FunctionInstantiation,
        args: Vec<crate::plan::CallArg>,
    },
    FunctionCall {
        function: Box<FunctionFunctionExpr>,
        args: Vec<crate::plan::CallArg>,
    },
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
        true_: Box<GenericFunctionExpr>,
        false_: Box<GenericFunctionExpr>,
    },
    IntCase {
        subject: Box<IntExpr>,
        clauses: Vec<(BigInt, GenericFunctionExpr)>,
        fallback: Box<GenericFunctionExpr>,
    },
    StringCase {
        subject: Box<StringExpr>,
        clauses: Vec<(EcoString, GenericFunctionExpr)>,
        fallback: Box<GenericFunctionExpr>,
    },
    FloatCase {
        subject: Box<FloatExpr>,
        clauses: Vec<(f64, GenericFunctionExpr)>,
        fallback: Box<GenericFunctionExpr>,
    },
    Block {
        steps: Vec<Step>,
        return_: Box<GenericFunctionExpr>,
    },
}

impl GenericFunctionExpr {
    pub(crate) fn reference(
        reference: GenericFunctionReference,
        type_: crate::plan::GenericFunctionType,
    ) -> Self {
        Self {
            type_,
            kind: GenericFunctionExprKind::Reference(reference),
        }
    }

    pub(crate) fn closure(
        function: FunctionInstantiation,
        params: Vec<ParamSlot>,
        captures: Vec<CaptureArg>,
        type_: crate::plan::GenericFunctionType,
    ) -> Self {
        Self {
            type_,
            kind: GenericFunctionExprKind::Closure {
                function,
                params,
                captures,
            },
        }
    }

    pub(crate) fn local_get(local: GenericFunctionLocal, name: EcoString) -> Self {
        Self {
            type_: local.type_().clone(),
            kind: GenericFunctionExprKind::LocalGet { local, name },
        }
    }

    pub(crate) fn call(
        function: FunctionInstantiation,
        args: Vec<crate::plan::CallArg>,
        type_: crate::plan::GenericFunctionType,
    ) -> Self {
        Self {
            type_,
            kind: GenericFunctionExprKind::Call { function, args },
        }
    }

    pub(crate) fn function_call(
        function: FunctionFunctionExpr,
        args: Vec<crate::plan::CallArg>,
        type_: crate::plan::GenericFunctionType,
    ) -> Self {
        Self {
            type_,
            kind: GenericFunctionExprKind::FunctionCall {
                function: Box::new(function),
                args,
            },
        }
    }

    pub(crate) fn tuple_index(
        tuple: TupleExpr,
        index: usize,
        type_: crate::plan::GenericFunctionType,
    ) -> Self {
        Self {
            type_,
            kind: GenericFunctionExprKind::TupleIndex {
                tuple: Box::new(tuple),
                index,
            },
        }
    }

    pub(crate) fn custom_field(
        access: CustomFieldAccess,
        type_: crate::plan::GenericFunctionType,
    ) -> Self {
        Self {
            type_,
            kind: GenericFunctionExprKind::CustomField(access),
        }
    }

    pub(crate) fn list_index(
        list: FunctionListExpr,
        index: usize,
        type_: crate::plan::GenericFunctionType,
    ) -> Self {
        Self {
            type_,
            kind: GenericFunctionExprKind::ListIndex {
                list: Box::new(list),
                index,
            },
        }
    }

    pub(crate) fn panic(panic: PanicExpr, type_: crate::plan::GenericFunctionType) -> Self {
        Self {
            type_,
            kind: GenericFunctionExprKind::Panic(panic),
        }
    }

    pub(crate) fn bool_case(subject: BoolExpr, true_: Self, false_: Self) -> Option<Self> {
        (true_.type_ == false_.type_).then(|| Self {
            type_: true_.type_.clone(),
            kind: GenericFunctionExprKind::BoolCase {
                subject: Box::new(subject),
                true_: Box::new(true_),
                false_: Box::new(false_),
            },
        })
    }

    pub(crate) fn int_case(
        subject: IntExpr,
        clauses: Vec<(BigInt, Self)>,
        fallback: Self,
    ) -> Option<Self> {
        let type_ = fallback.type_.clone();
        clauses
            .iter()
            .all(|(_, branch)| branch.type_ == type_)
            .then(|| Self {
                type_,
                kind: GenericFunctionExprKind::IntCase {
                    subject: Box::new(subject),
                    clauses,
                    fallback: Box::new(fallback),
                },
            })
    }

    pub(crate) fn string_case(
        subject: StringExpr,
        clauses: Vec<(EcoString, Self)>,
        fallback: Self,
    ) -> Option<Self> {
        let type_ = fallback.type_.clone();
        clauses
            .iter()
            .all(|(_, branch)| branch.type_ == type_)
            .then(|| Self {
                type_,
                kind: GenericFunctionExprKind::StringCase {
                    subject: Box::new(subject),
                    clauses,
                    fallback: Box::new(fallback),
                },
            })
    }

    pub(crate) fn float_case(
        subject: FloatExpr,
        clauses: Vec<(f64, Self)>,
        fallback: Self,
    ) -> Option<Self> {
        let type_ = fallback.type_.clone();
        clauses
            .iter()
            .all(|(_, branch)| branch.type_ == type_)
            .then(|| Self {
                type_,
                kind: GenericFunctionExprKind::FloatCase {
                    subject: Box::new(subject),
                    clauses,
                    fallback: Box::new(fallback),
                },
            })
    }

    pub(crate) fn block(steps: Vec<Step>, return_: Self) -> Self {
        Self {
            type_: return_.type_.clone(),
            kind: GenericFunctionExprKind::Block {
                steps,
                return_: Box::new(return_),
            },
        }
    }

    pub(crate) fn shape(&self) -> FunctionShape {
        self.type_.shape()
    }

    pub(crate) fn return_parameter(&self) -> TypeParameterId {
        self.type_.return_parameter()
    }

    pub(crate) fn type_(&self) -> &crate::plan::GenericFunctionType {
        &self.type_
    }

    pub(crate) fn kind(&self) -> &GenericFunctionExprKind {
        &self.kind
    }

    pub(crate) fn with_type(mut self, type_: crate::plan::GenericFunctionType) -> Self {
        self.type_ = type_;
        self
    }
}
