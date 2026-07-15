use super::{
    BoolExpr, CallArg, CustomFieldAccess, FloatExpr, IntExpr, PanicExpr, StringExpr, TupleExpr,
    UtfCodepointFunctionExpr, UtfCodepointListExpr,
};
use crate::plan::{Step, UtfCodepointFunctionId, UtfCodepointLocalId};
use ecow::EcoString;
use num_bigint::BigInt;

#[derive(Debug, Clone, PartialEq)]
pub struct UtfCodepointExpr {
    kind: UtfCodepointExprKind,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum UtfCodepointExprKind {
    LocalGet {
        local: UtfCodepointLocalId,
        name: EcoString,
    },
    Call {
        function: UtfCodepointFunctionId,
        args: Vec<CallArg>,
    },
    FunctionCall {
        function: Box<UtfCodepointFunctionExpr>,
        args: Vec<CallArg>,
    },
    TupleIndex {
        tuple: Box<TupleExpr>,
        index: usize,
    },
    CustomField(CustomFieldAccess),
    ListIndex {
        list: Box<UtfCodepointListExpr>,
        index: usize,
    },
    Panic(PanicExpr),
    BoolCase {
        subject: Box<BoolExpr>,
        true_: Box<UtfCodepointExpr>,
        false_: Box<UtfCodepointExpr>,
    },
    IntCase {
        subject: Box<IntExpr>,
        clauses: Vec<(BigInt, UtfCodepointExpr)>,
        fallback: Box<UtfCodepointExpr>,
    },
    StringCase {
        subject: Box<StringExpr>,
        clauses: Vec<(EcoString, UtfCodepointExpr)>,
        fallback: Box<UtfCodepointExpr>,
    },
    FloatCase {
        subject: Box<FloatExpr>,
        clauses: Vec<(f64, UtfCodepointExpr)>,
        fallback: Box<UtfCodepointExpr>,
    },
    Block {
        steps: Vec<Step>,
        return_: Box<UtfCodepointExpr>,
    },
}

impl UtfCodepointExpr {
    pub(crate) fn local_get(local: UtfCodepointLocalId, name: EcoString) -> Self {
        Self::new(UtfCodepointExprKind::LocalGet { local, name })
    }

    pub(crate) fn call(function: UtfCodepointFunctionId, args: Vec<CallArg>) -> Self {
        Self::new(UtfCodepointExprKind::Call { function, args })
    }

    pub(crate) fn function_call(function: UtfCodepointFunctionExpr, args: Vec<CallArg>) -> Self {
        Self::new(UtfCodepointExprKind::FunctionCall {
            function: Box::new(function),
            args,
        })
    }

    pub(crate) fn tuple_index(tuple: TupleExpr, index: usize) -> Self {
        Self::new(UtfCodepointExprKind::TupleIndex {
            tuple: Box::new(tuple),
            index,
        })
    }

    pub(crate) fn custom_field(access: CustomFieldAccess) -> Self {
        Self::new(UtfCodepointExprKind::CustomField(access))
    }

    pub(crate) fn list_index(list: UtfCodepointListExpr, index: usize) -> Self {
        Self::new(UtfCodepointExprKind::ListIndex {
            list: Box::new(list),
            index,
        })
    }

    pub(crate) fn panic(panic: PanicExpr) -> Self {
        Self::new(UtfCodepointExprKind::Panic(panic))
    }

    pub(crate) fn bool_case(subject: BoolExpr, true_: Self, false_: Self) -> Self {
        Self::new(UtfCodepointExprKind::BoolCase {
            subject: Box::new(subject),
            true_: Box::new(true_),
            false_: Box::new(false_),
        })
    }

    pub(crate) fn int_case(subject: IntExpr, clauses: Vec<(BigInt, Self)>, fallback: Self) -> Self {
        Self::new(UtfCodepointExprKind::IntCase {
            subject: Box::new(subject),
            clauses,
            fallback: Box::new(fallback),
        })
    }

    pub(crate) fn string_case(
        subject: StringExpr,
        clauses: Vec<(EcoString, Self)>,
        fallback: Self,
    ) -> Self {
        Self::new(UtfCodepointExprKind::StringCase {
            subject: Box::new(subject),
            clauses,
            fallback: Box::new(fallback),
        })
    }

    pub(crate) fn float_case(
        subject: FloatExpr,
        clauses: Vec<(f64, Self)>,
        fallback: Self,
    ) -> Self {
        Self::new(UtfCodepointExprKind::FloatCase {
            subject: Box::new(subject),
            clauses,
            fallback: Box::new(fallback),
        })
    }

    pub(crate) fn block(steps: Vec<Step>, return_: Self) -> Self {
        Self::new(UtfCodepointExprKind::Block {
            steps,
            return_: Box::new(return_),
        })
    }

    pub(crate) fn kind(&self) -> &UtfCodepointExprKind {
        &self.kind
    }

    pub(crate) fn into_kind(self) -> UtfCodepointExprKind {
        self.kind
    }

    fn new(kind: UtfCodepointExprKind) -> Self {
        Self { kind }
    }
}

#[cfg(test)]
mod tests {
    use super::{UtfCodepointExpr, UtfCodepointExprKind};
    use crate::plan::{
        BoolExpr, Expr, IntExpr, PanicExpr, PanicSite, ParamLocal, Step, StringExpr, TupleExpr,
        TupleLocalId, UtfCodepointFunctionExpr, UtfCodepointFunctionId,
        UtfCodepointFunctionReference, UtfCodepointListExpr, UtfCodepointListItem,
        UtfCodepointListLocalId, UtfCodepointLocalId, ValueType,
    };

    #[test]
    fn utf_codepoint_expr_kind_accessors() {
        assert_eq!(
            value().kind(),
            &UtfCodepointExprKind::LocalGet {
                local: UtfCodepointLocalId(0),
                name: "value".into(),
            },
        );
        assert_eq!(
            UtfCodepointExpr::call(UtfCodepointFunctionId(0), Vec::new()).kind(),
            &UtfCodepointExprKind::Call {
                function: UtfCodepointFunctionId(0),
                args: Vec::new(),
            },
        );
        assert_eq!(
            UtfCodepointExpr::function_call(function(), Vec::new()).kind(),
            &UtfCodepointExprKind::FunctionCall {
                function: Box::new(function()),
                args: Vec::new(),
            },
        );
        assert_eq!(
            UtfCodepointExpr::tuple_index(tuple(), 0).kind(),
            &UtfCodepointExprKind::TupleIndex {
                tuple: Box::new(tuple()),
                index: 0,
            },
        );
        assert_eq!(
            UtfCodepointExpr::list_index(list(), 0).kind(),
            &UtfCodepointExprKind::ListIndex {
                list: Box::new(list()),
                index: 0,
            },
        );
        let panic = PanicExpr::panic_at(None, PanicSite::unknown());
        assert_eq!(
            UtfCodepointExpr::panic(panic.clone()).kind(),
            &UtfCodepointExprKind::Panic(panic),
        );
        assert_eq!(
            UtfCodepointExpr::bool_case(BoolExpr::value(true), value(), value()).kind(),
            &UtfCodepointExprKind::BoolCase {
                subject: Box::new(BoolExpr::value(true)),
                true_: Box::new(value()),
                false_: Box::new(value()),
            },
        );
        assert_eq!(
            UtfCodepointExpr::int_case(
                IntExpr::value(1.into()),
                vec![(1.into(), value())],
                value(),
            )
            .kind(),
            &UtfCodepointExprKind::IntCase {
                subject: Box::new(IntExpr::value(1.into())),
                clauses: vec![(1.into(), value())],
                fallback: Box::new(value()),
            },
        );
        assert_eq!(
            UtfCodepointExpr::string_case(
                StringExpr::value("one".into()),
                vec![("one".into(), value())],
                value(),
            )
            .kind(),
            &UtfCodepointExprKind::StringCase {
                subject: Box::new(StringExpr::value("one".into())),
                clauses: vec![("one".into(), value())],
                fallback: Box::new(value()),
            },
        );
        assert_eq!(
            UtfCodepointExpr::float_case(
                crate::plan::FloatExpr::value(1.0),
                vec![(1.0, value())],
                value(),
            )
            .kind(),
            &UtfCodepointExprKind::FloatCase {
                subject: Box::new(crate::plan::FloatExpr::value(1.0)),
                clauses: vec![(1.0, value())],
                fallback: Box::new(value()),
            },
        );
        assert_eq!(
            UtfCodepointExpr::block(
                vec![Step::evaluate(Expr::int(IntExpr::value(1.into())))],
                value(),
            )
            .kind(),
            &UtfCodepointExprKind::Block {
                steps: vec![Step::evaluate(Expr::int(IntExpr::value(1.into())))],
                return_: Box::new(value()),
            },
        );
    }

    fn value() -> UtfCodepointExpr {
        UtfCodepointExpr::local_get(UtfCodepointLocalId(0), "value".into())
    }

    fn function() -> UtfCodepointFunctionExpr {
        UtfCodepointFunctionExpr::reference(UtfCodepointFunctionReference::new(
            UtfCodepointFunctionId(0),
            vec![ParamLocal::utf_codepoint(UtfCodepointLocalId(0))],
        ))
    }

    fn tuple() -> TupleExpr {
        TupleExpr::local_get(
            TupleLocalId(0),
            "pair".into(),
            vec![ValueType::UtfCodepoint],
        )
    }

    fn list() -> UtfCodepointListExpr {
        UtfCodepointListExpr::local_get(
            UtfCodepointListItem,
            UtfCodepointListLocalId(0),
            "values".into(),
        )
    }
}
