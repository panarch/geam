use super::{
    BoolExpr, CallArg, CustomFieldAccess, FloatExpr, IntExpr, PanicExpr, StringFunctionExpr,
    StringListExpr, TupleExpr,
};
use crate::plan::{Step, StringFunctionId, StringLocalId};
use ecow::EcoString;
use num_bigint::BigInt;

#[derive(Debug, Clone, PartialEq)]
pub struct StringExpr {
    kind: StringExprKind,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum StringExprKind {
    Value(EcoString),
    LocalGet {
        local: StringLocalId,
        name: EcoString,
    },
    Call {
        function: StringFunctionId,
        args: Vec<CallArg>,
    },
    FunctionCall {
        function: Box<StringFunctionExpr>,
        args: Vec<CallArg>,
    },
    TupleIndex {
        tuple: Box<TupleExpr>,
        index: usize,
    },
    CustomField(CustomFieldAccess),
    ListIndex {
        list: Box<StringListExpr>,
        index: usize,
    },
    Panic(PanicExpr),
    Concatenate {
        left: Box<StringExpr>,
        right: Box<StringExpr>,
    },
    DropPrefix {
        value: Box<StringExpr>,
        prefix: EcoString,
    },
    BoolCase {
        subject: Box<BoolExpr>,
        true_: Box<StringExpr>,
        false_: Box<StringExpr>,
    },
    IntCase {
        subject: Box<IntExpr>,
        clauses: Vec<(BigInt, StringExpr)>,
        fallback: Box<StringExpr>,
    },
    StringCase {
        subject: Box<StringExpr>,
        clauses: Vec<(EcoString, StringExpr)>,
        fallback: Box<StringExpr>,
    },
    FloatCase {
        subject: Box<FloatExpr>,
        clauses: Vec<(f64, StringExpr)>,
        fallback: Box<StringExpr>,
    },
    Block {
        steps: Vec<Step>,
        return_: Box<StringExpr>,
    },
}

impl StringExpr {
    pub(crate) fn value(value: EcoString) -> Self {
        Self {
            kind: StringExprKind::Value(value),
        }
    }

    pub(crate) fn local_get(local: StringLocalId, name: EcoString) -> Self {
        Self {
            kind: StringExprKind::LocalGet { local, name },
        }
    }

    pub(crate) fn call(function: StringFunctionId, args: Vec<CallArg>) -> Self {
        Self {
            kind: StringExprKind::Call { function, args },
        }
    }

    pub(crate) fn function_call(function: StringFunctionExpr, args: Vec<CallArg>) -> Self {
        Self {
            kind: StringExprKind::FunctionCall {
                function: Box::new(function),
                args,
            },
        }
    }

    pub(crate) fn tuple_index(tuple: TupleExpr, index: usize) -> Self {
        Self {
            kind: StringExprKind::TupleIndex {
                tuple: Box::new(tuple),
                index,
            },
        }
    }

    pub(crate) fn custom_field(access: CustomFieldAccess) -> Self {
        Self {
            kind: StringExprKind::CustomField(access),
        }
    }

    pub(crate) fn list_index(list: impl Into<StringListExpr>, index: usize) -> Self {
        Self {
            kind: StringExprKind::ListIndex {
                list: Box::new(list.into()),
                index,
            },
        }
    }

    pub(crate) fn panic(panic: PanicExpr) -> Self {
        Self {
            kind: StringExprKind::Panic(panic),
        }
    }

    pub(crate) fn concatenate(left: StringExpr, right: StringExpr) -> Self {
        Self {
            kind: StringExprKind::Concatenate {
                left: Box::new(left),
                right: Box::new(right),
            },
        }
    }

    pub(crate) fn drop_prefix(value: StringExpr, prefix: EcoString) -> Self {
        Self {
            kind: StringExprKind::DropPrefix {
                value: Box::new(value),
                prefix,
            },
        }
    }

    pub(crate) fn bool_case(subject: BoolExpr, true_: StringExpr, false_: StringExpr) -> Self {
        Self {
            kind: StringExprKind::BoolCase {
                subject: Box::new(subject),
                true_: Box::new(true_),
                false_: Box::new(false_),
            },
        }
    }

    pub(crate) fn int_case(
        subject: IntExpr,
        clauses: Vec<(BigInt, StringExpr)>,
        fallback: StringExpr,
    ) -> Self {
        Self {
            kind: StringExprKind::IntCase {
                subject: Box::new(subject),
                clauses,
                fallback: Box::new(fallback),
            },
        }
    }

    pub(crate) fn string_case(
        subject: StringExpr,
        clauses: Vec<(EcoString, StringExpr)>,
        fallback: StringExpr,
    ) -> Self {
        Self {
            kind: StringExprKind::StringCase {
                subject: Box::new(subject),
                clauses,
                fallback: Box::new(fallback),
            },
        }
    }

    pub(crate) fn float_case(
        subject: FloatExpr,
        clauses: Vec<(f64, StringExpr)>,
        fallback: StringExpr,
    ) -> Self {
        Self {
            kind: StringExprKind::FloatCase {
                subject: Box::new(subject),
                clauses,
                fallback: Box::new(fallback),
            },
        }
    }

    pub(crate) fn block(steps: Vec<Step>, return_: StringExpr) -> Self {
        Self {
            kind: StringExprKind::Block {
                steps,
                return_: Box::new(return_),
            },
        }
    }

    pub(crate) fn kind(&self) -> &StringExprKind {
        &self.kind
    }

    pub(crate) fn into_kind(self) -> StringExprKind {
        self.kind
    }
}

#[cfg(test)]
mod tests {
    use super::{StringExpr, StringExprKind};
    use crate::plan::{
        BoolExpr, Expr, IntExpr, Step, StringFunctionId, StringFunctionReference, StringLocalId,
        TupleExpr, ValueType,
    };
    use num_bigint::BigInt;

    #[test]
    fn string_expr_kind_accessors() {
        assert_eq!(
            StringExpr::value("geam".into()).kind(),
            &StringExprKind::Value("geam".into()),
        );
        assert_eq!(
            StringExpr::local_get(StringLocalId(0), "value".into()).kind(),
            &StringExprKind::LocalGet {
                local: StringLocalId(0),
                name: "value".into(),
            },
        );
        assert_eq!(
            StringExpr::call(StringFunctionId(0), Vec::new()).kind(),
            &StringExprKind::Call {
                function: StringFunctionId(0),
                args: Vec::new(),
            },
        );
        assert_eq!(
            StringExpr::function_call(function_expr(), Vec::new()).kind(),
            &StringExprKind::FunctionCall {
                function: Box::new(function_expr()),
                args: Vec::new(),
            },
        );
        assert_eq!(
            StringExpr::tuple_index(tuple_expr(), 0).kind(),
            &StringExprKind::TupleIndex {
                tuple: Box::new(tuple_expr()),
                index: 0,
            },
        );
        assert_eq!(
            StringExpr::concatenate(StringExpr::value("a".into()), StringExpr::value("b".into()))
                .kind(),
            &StringExprKind::Concatenate {
                left: Box::new(StringExpr::value("a".into())),
                right: Box::new(StringExpr::value("b".into())),
            },
        );
        assert_eq!(
            StringExpr::drop_prefix(StringExpr::value("hello".into()), "he".into()).kind(),
            &StringExprKind::DropPrefix {
                value: Box::new(StringExpr::value("hello".into())),
                prefix: "he".into(),
            },
        );
        assert_eq!(
            StringExpr::bool_case(
                BoolExpr::value(true),
                StringExpr::value("yes".into()),
                StringExpr::value("no".into())
            )
            .kind(),
            &StringExprKind::BoolCase {
                subject: Box::new(BoolExpr::value(true)),
                true_: Box::new(StringExpr::value("yes".into())),
                false_: Box::new(StringExpr::value("no".into())),
            },
        );
        assert_eq!(
            StringExpr::int_case(
                IntExpr::value(1.into()),
                vec![(1.into(), StringExpr::value("one".into()))],
                StringExpr::value("other".into())
            )
            .kind(),
            &StringExprKind::IntCase {
                subject: Box::new(IntExpr::value(1.into())),
                clauses: vec![(BigInt::from(1), StringExpr::value("one".into()))],
                fallback: Box::new(StringExpr::value("other".into())),
            },
        );
        assert_eq!(
            StringExpr::string_case(
                StringExpr::value("a".into()),
                vec![("a".into(), StringExpr::value("hit".into()))],
                StringExpr::value("miss".into())
            )
            .kind(),
            &StringExprKind::StringCase {
                subject: Box::new(StringExpr::value("a".into())),
                clauses: vec![("a".into(), StringExpr::value("hit".into()))],
                fallback: Box::new(StringExpr::value("miss".into())),
            },
        );
        assert_eq!(
            StringExpr::float_case(
                crate::plan::FloatExpr::value(1.0),
                vec![(1.0, StringExpr::value("hit".into()))],
                StringExpr::value("miss".into())
            )
            .kind(),
            &StringExprKind::FloatCase {
                subject: Box::new(crate::plan::FloatExpr::value(1.0)),
                clauses: vec![(1.0, StringExpr::value("hit".into()))],
                fallback: Box::new(StringExpr::value("miss".into())),
            },
        );
        assert_eq!(
            StringExpr::block(
                vec![Step::evaluate(Expr::string(StringExpr::value("a".into())))],
                StringExpr::value("b".into()),
            )
            .kind(),
            &StringExprKind::Block {
                steps: vec![Step::evaluate(Expr::string(StringExpr::value("a".into())))],
                return_: Box::new(StringExpr::value("b".into())),
            },
        );
    }

    fn function_expr() -> crate::plan::StringFunctionExpr {
        crate::plan::StringFunctionExpr::reference(StringFunctionReference::new(
            StringFunctionId(0),
            Vec::new(),
        ))
    }

    fn tuple_expr() -> TupleExpr {
        TupleExpr::value(
            vec![Expr::string(StringExpr::value("geam".into()))],
            vec![ValueType::String],
        )
    }
}
