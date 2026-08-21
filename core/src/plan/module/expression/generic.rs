use super::{
    BoolExpr, CallArg, CustomFieldAccess, FloatExpr, GenericFunctionExpr, GenericListExpr, IntExpr,
    PanicExpr, StringExpr, TupleExpr,
};
use crate::plan::{FunctionInstantiation, GenericLocal, HostCallSite, Step, TypeParameterId};
use ecow::EcoString;
use num_bigint::BigInt;

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct GenericExpr {
    parameter: TypeParameterId,
    kind: GenericExprKind,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum GenericExprKind {
    LocalGet {
        local: GenericLocal,
        name: EcoString,
    },
    Call {
        function: FunctionInstantiation,
        args: Vec<CallArg>,
        site: HostCallSite,
    },
    FunctionCall {
        function: Box<GenericFunctionExpr>,
        args: Vec<CallArg>,
        site: HostCallSite,
    },
    TupleIndex {
        tuple: Box<TupleExpr>,
        index: usize,
    },
    CustomField(CustomFieldAccess),
    ListIndex {
        list: Box<GenericListExpr>,
        index: usize,
    },
    Panic(PanicExpr),
    BoolCase {
        subject: Box<BoolExpr>,
        true_: Box<GenericExpr>,
        false_: Box<GenericExpr>,
    },
    IntCase {
        subject: Box<IntExpr>,
        clauses: Vec<(BigInt, GenericExpr)>,
        fallback: Box<GenericExpr>,
    },
    StringCase {
        subject: Box<StringExpr>,
        clauses: Vec<(EcoString, GenericExpr)>,
        fallback: Box<GenericExpr>,
    },
    FloatCase {
        subject: Box<FloatExpr>,
        clauses: Vec<(f64, GenericExpr)>,
        fallback: Box<GenericExpr>,
    },
    Block {
        steps: Vec<Step>,
        return_: Box<GenericExpr>,
    },
}

impl GenericExpr {
    pub(crate) fn local_get(local: GenericLocal, name: EcoString) -> Self {
        Self {
            parameter: local.parameter(),
            kind: GenericExprKind::LocalGet { local, name },
        }
    }

    #[cfg(test)]
    pub(crate) fn call(
        parameter: TypeParameterId,
        function: FunctionInstantiation,
        args: Vec<CallArg>,
    ) -> Self {
        Self::call_at(parameter, function, args, HostCallSite::unknown())
    }

    pub(crate) fn call_at(
        parameter: TypeParameterId,
        function: FunctionInstantiation,
        args: Vec<CallArg>,
        site: HostCallSite,
    ) -> Self {
        Self {
            parameter,
            kind: GenericExprKind::Call {
                function,
                args,
                site,
            },
        }
    }

    #[cfg(test)]
    pub(crate) fn function_call(function: GenericFunctionExpr, args: Vec<CallArg>) -> Self {
        Self::function_call_at(function, args, HostCallSite::unknown())
    }

    pub(crate) fn function_call_at(
        function: GenericFunctionExpr,
        args: Vec<CallArg>,
        site: HostCallSite,
    ) -> Self {
        let parameter = function.return_parameter();
        Self {
            parameter,
            kind: GenericExprKind::FunctionCall {
                function: Box::new(function),
                args,
                site,
            },
        }
    }

    pub(crate) fn tuple_index(parameter: TypeParameterId, tuple: TupleExpr, index: usize) -> Self {
        Self {
            parameter,
            kind: GenericExprKind::TupleIndex {
                tuple: Box::new(tuple),
                index,
            },
        }
    }

    pub(crate) fn custom_field(parameter: TypeParameterId, access: CustomFieldAccess) -> Self {
        Self {
            parameter,
            kind: GenericExprKind::CustomField(access),
        }
    }

    pub(crate) fn list_index(list: GenericListExpr, index: usize) -> Self {
        let parameter = list.item().parameter();
        Self {
            parameter,
            kind: GenericExprKind::ListIndex {
                list: Box::new(list),
                index,
            },
        }
    }

    pub(crate) fn panic(parameter: TypeParameterId, panic: PanicExpr) -> Self {
        Self {
            parameter,
            kind: GenericExprKind::Panic(panic),
        }
    }

    pub(crate) fn bool_case(subject: BoolExpr, true_: Self, false_: Self) -> Option<Self> {
        (true_.parameter == false_.parameter).then(|| Self {
            parameter: true_.parameter,
            kind: GenericExprKind::BoolCase {
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
        let parameter = fallback.parameter;
        clauses
            .iter()
            .all(|(_, branch)| branch.parameter == parameter)
            .then(|| Self {
                parameter,
                kind: GenericExprKind::IntCase {
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
        let parameter = fallback.parameter;
        clauses
            .iter()
            .all(|(_, branch)| branch.parameter == parameter)
            .then(|| Self {
                parameter,
                kind: GenericExprKind::StringCase {
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
        let parameter = fallback.parameter;
        clauses
            .iter()
            .all(|(_, branch)| branch.parameter == parameter)
            .then(|| Self {
                parameter,
                kind: GenericExprKind::FloatCase {
                    subject: Box::new(subject),
                    clauses,
                    fallback: Box::new(fallback),
                },
            })
    }

    pub(crate) fn block(steps: Vec<Step>, return_: Self) -> Self {
        Self {
            parameter: return_.parameter,
            kind: GenericExprKind::Block {
                steps,
                return_: Box::new(return_),
            },
        }
    }

    pub(crate) fn parameter(&self) -> TypeParameterId {
        self.parameter
    }

    pub(crate) fn kind(&self) -> &GenericExprKind {
        &self.kind
    }
}
