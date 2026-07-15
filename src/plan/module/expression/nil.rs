use super::{
    BoolExpr, CallArg, CustomFieldAccess, FloatExpr, IntExpr, NilFunctionExpr, NilListExpr,
    PanicExpr, StringExpr, TupleExpr,
};
use crate::plan::{NilFunctionId, NilLocalId, Step};
use ecow::EcoString;
use num_bigint::BigInt;

#[derive(Debug, Clone, PartialEq)]
pub struct NilExpr {
    kind: NilExprKind,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum NilExprKind {
    Value,
    LocalGet {
        local: NilLocalId,
        name: EcoString,
    },
    Call {
        function: NilFunctionId,
        args: Vec<CallArg>,
    },
    FunctionCall {
        function: Box<NilFunctionExpr>,
        args: Vec<CallArg>,
    },
    TupleIndex {
        tuple: Box<TupleExpr>,
        index: usize,
    },
    CustomField(CustomFieldAccess),
    ListIndex {
        list: Box<NilListExpr>,
        index: usize,
    },
    Panic(PanicExpr),
    BoolCase {
        subject: Box<BoolExpr>,
        true_: Box<NilExpr>,
        false_: Box<NilExpr>,
    },
    IntCase {
        subject: Box<IntExpr>,
        clauses: Vec<(BigInt, NilExpr)>,
        fallback: Box<NilExpr>,
    },
    StringCase {
        subject: Box<StringExpr>,
        clauses: Vec<(EcoString, NilExpr)>,
        fallback: Box<NilExpr>,
    },
    FloatCase {
        subject: Box<FloatExpr>,
        clauses: Vec<(f64, NilExpr)>,
        fallback: Box<NilExpr>,
    },
    Block {
        steps: Vec<Step>,
        return_: Box<NilExpr>,
    },
}

impl NilExpr {
    pub(crate) fn value() -> Self {
        Self {
            kind: NilExprKind::Value,
        }
    }

    pub(crate) fn local_get(local: NilLocalId, name: EcoString) -> Self {
        Self {
            kind: NilExprKind::LocalGet { local, name },
        }
    }

    pub(crate) fn call(function: NilFunctionId, args: Vec<CallArg>) -> Self {
        Self {
            kind: NilExprKind::Call { function, args },
        }
    }

    pub(crate) fn function_call(function: NilFunctionExpr, args: Vec<CallArg>) -> Self {
        Self {
            kind: NilExprKind::FunctionCall {
                function: Box::new(function),
                args,
            },
        }
    }

    pub(crate) fn tuple_index(tuple: TupleExpr, index: usize) -> Self {
        Self {
            kind: NilExprKind::TupleIndex {
                tuple: Box::new(tuple),
                index,
            },
        }
    }

    pub(crate) fn custom_field(access: CustomFieldAccess) -> Self {
        Self {
            kind: NilExprKind::CustomField(access),
        }
    }

    pub(crate) fn list_index(list: impl Into<NilListExpr>, index: usize) -> Self {
        Self {
            kind: NilExprKind::ListIndex {
                list: Box::new(list.into()),
                index,
            },
        }
    }

    pub(crate) fn panic(panic: PanicExpr) -> Self {
        Self {
            kind: NilExprKind::Panic(panic),
        }
    }

    pub(crate) fn bool_case(subject: BoolExpr, true_: NilExpr, false_: NilExpr) -> Self {
        Self {
            kind: NilExprKind::BoolCase {
                subject: Box::new(subject),
                true_: Box::new(true_),
                false_: Box::new(false_),
            },
        }
    }

    pub(crate) fn int_case(
        subject: IntExpr,
        clauses: Vec<(BigInt, NilExpr)>,
        fallback: NilExpr,
    ) -> Self {
        Self {
            kind: NilExprKind::IntCase {
                subject: Box::new(subject),
                clauses,
                fallback: Box::new(fallback),
            },
        }
    }

    pub(crate) fn string_case(
        subject: StringExpr,
        clauses: Vec<(EcoString, NilExpr)>,
        fallback: NilExpr,
    ) -> Self {
        Self {
            kind: NilExprKind::StringCase {
                subject: Box::new(subject),
                clauses,
                fallback: Box::new(fallback),
            },
        }
    }

    pub(crate) fn float_case(
        subject: FloatExpr,
        clauses: Vec<(f64, NilExpr)>,
        fallback: NilExpr,
    ) -> Self {
        Self {
            kind: NilExprKind::FloatCase {
                subject: Box::new(subject),
                clauses,
                fallback: Box::new(fallback),
            },
        }
    }

    pub(crate) fn block(steps: Vec<Step>, return_: NilExpr) -> Self {
        Self {
            kind: NilExprKind::Block {
                steps,
                return_: Box::new(return_),
            },
        }
    }

    pub(crate) fn kind(&self) -> &NilExprKind {
        &self.kind
    }

    pub(crate) fn into_kind(self) -> NilExprKind {
        self.kind
    }
}

#[cfg(test)]
mod tests {
    use super::{NilExpr, NilExprKind};
    use crate::plan::{
        BoolExpr, Expr, IntExpr, NilFunctionId, NilFunctionReference, NilLocalId, Step, TupleExpr,
        ValueType,
    };
    use num_bigint::BigInt;

    #[test]
    fn nil_expr_kind_accessors() {
        assert_eq!(NilExpr::value().kind(), &NilExprKind::Value);
        assert_eq!(
            NilExpr::local_get(NilLocalId(0), "unit".into()).kind(),
            &NilExprKind::LocalGet {
                local: NilLocalId(0),
                name: "unit".into(),
            },
        );
        assert_eq!(
            NilExpr::call(NilFunctionId(0), Vec::new()).kind(),
            &NilExprKind::Call {
                function: NilFunctionId(0),
                args: Vec::new(),
            },
        );
        assert_eq!(
            NilExpr::function_call(function_expr(), Vec::new()).kind(),
            &NilExprKind::FunctionCall {
                function: Box::new(function_expr()),
                args: Vec::new(),
            },
        );
        assert_eq!(
            NilExpr::tuple_index(tuple_expr(), 0).kind(),
            &NilExprKind::TupleIndex {
                tuple: Box::new(tuple_expr()),
                index: 0,
            },
        );
        assert_eq!(
            NilExpr::bool_case(BoolExpr::value(true), NilExpr::value(), NilExpr::value()).kind(),
            &NilExprKind::BoolCase {
                subject: Box::new(BoolExpr::value(true)),
                true_: Box::new(NilExpr::value()),
                false_: Box::new(NilExpr::value()),
            },
        );
        assert_eq!(
            NilExpr::int_case(
                IntExpr::value(1.into()),
                vec![(1.into(), NilExpr::value())],
                NilExpr::value()
            )
            .kind(),
            &NilExprKind::IntCase {
                subject: Box::new(IntExpr::value(1.into())),
                clauses: vec![(BigInt::from(1), NilExpr::value())],
                fallback: Box::new(NilExpr::value()),
            },
        );
        assert_eq!(
            NilExpr::string_case(
                crate::plan::StringExpr::value("a".into()),
                vec![("a".into(), NilExpr::value())],
                NilExpr::value()
            )
            .kind(),
            &NilExprKind::StringCase {
                subject: Box::new(crate::plan::StringExpr::value("a".into())),
                clauses: vec![("a".into(), NilExpr::value())],
                fallback: Box::new(NilExpr::value()),
            },
        );
        assert_eq!(
            NilExpr::float_case(
                crate::plan::FloatExpr::value(1.0),
                vec![(1.0, NilExpr::value())],
                NilExpr::value()
            )
            .kind(),
            &NilExprKind::FloatCase {
                subject: Box::new(crate::plan::FloatExpr::value(1.0)),
                clauses: vec![(1.0, NilExpr::value())],
                fallback: Box::new(NilExpr::value()),
            },
        );
        assert_eq!(
            NilExpr::block(
                vec![Step::evaluate(Expr::nil(NilExpr::value()))],
                NilExpr::value(),
            )
            .kind(),
            &NilExprKind::Block {
                steps: vec![Step::evaluate(Expr::nil(NilExpr::value()))],
                return_: Box::new(NilExpr::value()),
            },
        );
    }

    fn function_expr() -> crate::plan::NilFunctionExpr {
        crate::plan::NilFunctionExpr::reference(NilFunctionReference::new(
            NilFunctionId(0),
            Vec::new(),
        ))
    }

    fn tuple_expr() -> TupleExpr {
        TupleExpr::value(vec![Expr::nil(NilExpr::value())], vec![ValueType::Nil])
    }
}
