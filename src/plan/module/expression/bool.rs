use super::{
    BoolFunctionExpr, BoolListExpr, CallArg, CustomExpr, Expr, FloatExpr, IntExpr, ListExpr,
    PanicExpr, StringExpr, TupleExpr,
};
use crate::plan::{AssertPattern, BitArrayExpr, BitArrayPattern};
use crate::plan::{BoolFunctionId, BoolLocalId, Step};
use ecow::EcoString;
use num_bigint::BigInt;

#[derive(Debug, Clone, PartialEq)]
pub struct BoolExpr {
    kind: BoolExprKind,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum BoolExprKind {
    Value(bool),
    LocalGet {
        local: BoolLocalId,
        name: EcoString,
    },
    Call {
        function: BoolFunctionId,
        args: Vec<CallArg>,
    },
    FunctionCall {
        function: Box<BoolFunctionExpr>,
        args: Vec<CallArg>,
    },
    TupleIndex {
        tuple: Box<TupleExpr>,
        index: usize,
    },
    ListIndex {
        list: Box<BoolListExpr>,
        index: usize,
    },
    Panic(PanicExpr),
    Not(Box<BoolExpr>),
    LtInt {
        left: Box<IntExpr>,
        right: Box<IntExpr>,
    },
    LtEqInt {
        left: Box<IntExpr>,
        right: Box<IntExpr>,
    },
    GtInt {
        left: Box<IntExpr>,
        right: Box<IntExpr>,
    },
    GtEqInt {
        left: Box<IntExpr>,
        right: Box<IntExpr>,
    },
    LtFloat {
        left: Box<FloatExpr>,
        right: Box<FloatExpr>,
    },
    LtEqFloat {
        left: Box<FloatExpr>,
        right: Box<FloatExpr>,
    },
    GtFloat {
        left: Box<FloatExpr>,
        right: Box<FloatExpr>,
    },
    GtEqFloat {
        left: Box<FloatExpr>,
        right: Box<FloatExpr>,
    },
    Equal {
        left: Box<Expr>,
        right: Box<Expr>,
    },
    NotEqual {
        left: Box<Expr>,
        right: Box<Expr>,
    },
    StringStartsWith {
        value: Box<StringExpr>,
        prefix: EcoString,
    },
    ListLengthEquals {
        value: Box<ListExpr>,
        length: usize,
    },
    ListLengthAtLeast {
        value: Box<ListExpr>,
        length: usize,
    },
    BitArrayMatches {
        value: Box<BitArrayExpr>,
        pattern: BitArrayPattern,
    },
    CustomMatches {
        value: Box<CustomExpr>,
        pattern: Box<AssertPattern>,
    },
    And {
        left: Box<BoolExpr>,
        right: Box<BoolExpr>,
    },
    Or {
        left: Box<BoolExpr>,
        right: Box<BoolExpr>,
    },
    BoolCase {
        subject: Box<BoolExpr>,
        true_: Box<BoolExpr>,
        false_: Box<BoolExpr>,
    },
    IntCase {
        subject: Box<IntExpr>,
        clauses: Vec<(BigInt, BoolExpr)>,
        fallback: Box<BoolExpr>,
    },
    StringCase {
        subject: Box<StringExpr>,
        clauses: Vec<(EcoString, BoolExpr)>,
        fallback: Box<BoolExpr>,
    },
    FloatCase {
        subject: Box<FloatExpr>,
        clauses: Vec<(f64, BoolExpr)>,
        fallback: Box<BoolExpr>,
    },
    Block {
        steps: Vec<Step>,
        return_: Box<BoolExpr>,
    },
}

impl BoolExpr {
    pub(crate) fn value(value: bool) -> Self {
        Self {
            kind: BoolExprKind::Value(value),
        }
    }

    pub(crate) fn local_get(local: BoolLocalId, name: EcoString) -> Self {
        Self {
            kind: BoolExprKind::LocalGet { local, name },
        }
    }

    pub(crate) fn call(function: BoolFunctionId, args: Vec<CallArg>) -> Self {
        Self {
            kind: BoolExprKind::Call { function, args },
        }
    }

    pub(crate) fn function_call(function: BoolFunctionExpr, args: Vec<CallArg>) -> Self {
        Self {
            kind: BoolExprKind::FunctionCall {
                function: Box::new(function),
                args,
            },
        }
    }

    pub(crate) fn tuple_index(tuple: TupleExpr, index: usize) -> Self {
        Self {
            kind: BoolExprKind::TupleIndex {
                tuple: Box::new(tuple),
                index,
            },
        }
    }

    pub(crate) fn list_index(list: impl Into<BoolListExpr>, index: usize) -> Self {
        Self {
            kind: BoolExprKind::ListIndex {
                list: Box::new(list.into()),
                index,
            },
        }
    }

    pub(crate) fn panic(panic: PanicExpr) -> Self {
        Self {
            kind: BoolExprKind::Panic(panic),
        }
    }

    pub(crate) fn not(value: BoolExpr) -> Self {
        Self {
            kind: BoolExprKind::Not(Box::new(value)),
        }
    }

    pub(crate) fn lt_int(left: IntExpr, right: IntExpr) -> Self {
        Self {
            kind: BoolExprKind::LtInt {
                left: Box::new(left),
                right: Box::new(right),
            },
        }
    }

    pub(crate) fn lte_int(left: IntExpr, right: IntExpr) -> Self {
        Self {
            kind: BoolExprKind::LtEqInt {
                left: Box::new(left),
                right: Box::new(right),
            },
        }
    }

    pub(crate) fn gt_int(left: IntExpr, right: IntExpr) -> Self {
        Self {
            kind: BoolExprKind::GtInt {
                left: Box::new(left),
                right: Box::new(right),
            },
        }
    }

    pub(crate) fn gte_int(left: IntExpr, right: IntExpr) -> Self {
        Self {
            kind: BoolExprKind::GtEqInt {
                left: Box::new(left),
                right: Box::new(right),
            },
        }
    }

    pub(crate) fn lt_float(left: FloatExpr, right: FloatExpr) -> Self {
        Self {
            kind: BoolExprKind::LtFloat {
                left: Box::new(left),
                right: Box::new(right),
            },
        }
    }

    pub(crate) fn lte_float(left: FloatExpr, right: FloatExpr) -> Self {
        Self {
            kind: BoolExprKind::LtEqFloat {
                left: Box::new(left),
                right: Box::new(right),
            },
        }
    }

    pub(crate) fn gt_float(left: FloatExpr, right: FloatExpr) -> Self {
        Self {
            kind: BoolExprKind::GtFloat {
                left: Box::new(left),
                right: Box::new(right),
            },
        }
    }

    pub(crate) fn gte_float(left: FloatExpr, right: FloatExpr) -> Self {
        Self {
            kind: BoolExprKind::GtEqFloat {
                left: Box::new(left),
                right: Box::new(right),
            },
        }
    }

    pub(crate) fn equal(left: Expr, right: Expr) -> Self {
        Self {
            kind: BoolExprKind::Equal {
                left: Box::new(left),
                right: Box::new(right),
            },
        }
    }

    pub(crate) fn not_equal(left: Expr, right: Expr) -> Self {
        Self {
            kind: BoolExprKind::NotEqual {
                left: Box::new(left),
                right: Box::new(right),
            },
        }
    }

    pub(crate) fn string_starts_with(value: StringExpr, prefix: EcoString) -> Self {
        Self {
            kind: BoolExprKind::StringStartsWith {
                value: Box::new(value),
                prefix,
            },
        }
    }

    pub(crate) fn list_length_equals(value: ListExpr, length: usize) -> Self {
        Self {
            kind: BoolExprKind::ListLengthEquals {
                value: Box::new(value),
                length,
            },
        }
    }

    pub(crate) fn list_length_at_least(value: ListExpr, length: usize) -> Self {
        Self {
            kind: BoolExprKind::ListLengthAtLeast {
                value: Box::new(value),
                length,
            },
        }
    }

    pub(crate) fn bit_array_matches(value: BitArrayExpr, pattern: BitArrayPattern) -> Self {
        Self {
            kind: BoolExprKind::BitArrayMatches {
                value: Box::new(value),
                pattern,
            },
        }
    }

    pub(crate) fn custom_matches(value: CustomExpr, pattern: AssertPattern) -> Self {
        Self {
            kind: BoolExprKind::CustomMatches {
                value: Box::new(value),
                pattern: Box::new(pattern),
            },
        }
    }

    pub(crate) fn and(left: BoolExpr, right: BoolExpr) -> Self {
        Self {
            kind: BoolExprKind::And {
                left: Box::new(left),
                right: Box::new(right),
            },
        }
    }

    pub(crate) fn or(left: BoolExpr, right: BoolExpr) -> Self {
        Self {
            kind: BoolExprKind::Or {
                left: Box::new(left),
                right: Box::new(right),
            },
        }
    }

    pub(crate) fn bool_case(subject: BoolExpr, true_: BoolExpr, false_: BoolExpr) -> Self {
        Self {
            kind: BoolExprKind::BoolCase {
                subject: Box::new(subject),
                true_: Box::new(true_),
                false_: Box::new(false_),
            },
        }
    }

    pub(crate) fn int_case(
        subject: IntExpr,
        clauses: Vec<(BigInt, BoolExpr)>,
        fallback: BoolExpr,
    ) -> Self {
        Self {
            kind: BoolExprKind::IntCase {
                subject: Box::new(subject),
                clauses,
                fallback: Box::new(fallback),
            },
        }
    }

    pub(crate) fn string_case(
        subject: StringExpr,
        clauses: Vec<(EcoString, BoolExpr)>,
        fallback: BoolExpr,
    ) -> Self {
        Self {
            kind: BoolExprKind::StringCase {
                subject: Box::new(subject),
                clauses,
                fallback: Box::new(fallback),
            },
        }
    }

    pub(crate) fn float_case(
        subject: FloatExpr,
        clauses: Vec<(f64, BoolExpr)>,
        fallback: BoolExpr,
    ) -> Self {
        Self {
            kind: BoolExprKind::FloatCase {
                subject: Box::new(subject),
                clauses,
                fallback: Box::new(fallback),
            },
        }
    }

    pub(crate) fn block(steps: Vec<Step>, return_: BoolExpr) -> Self {
        Self {
            kind: BoolExprKind::Block {
                steps,
                return_: Box::new(return_),
            },
        }
    }

    pub(crate) fn kind(&self) -> &BoolExprKind {
        &self.kind
    }

    pub(crate) fn into_kind(self) -> BoolExprKind {
        self.kind
    }
}

#[cfg(test)]
mod tests {
    use super::{BoolExpr, BoolExprKind};
    use crate::plan::{
        BoolFunctionId, BoolFunctionReference, BoolLocalId, Expr, FloatExpr, IntExpr, Step,
        StringExpr, TupleExpr, ValueType,
    };
    use num_bigint::BigInt;

    #[test]
    fn bool_expr_kind_accessors() {
        assert_eq!(BoolExpr::value(true).kind(), &BoolExprKind::Value(true),);
        assert_eq!(
            BoolExpr::local_get(BoolLocalId(0), "flag".into()).kind(),
            &BoolExprKind::LocalGet {
                local: BoolLocalId(0),
                name: "flag".into(),
            },
        );
        assert_eq!(
            BoolExpr::call(BoolFunctionId(0), Vec::new()).kind(),
            &BoolExprKind::Call {
                function: BoolFunctionId(0),
                args: Vec::new(),
            },
        );
        assert_eq!(
            BoolExpr::function_call(function_expr(), Vec::new()).kind(),
            &BoolExprKind::FunctionCall {
                function: Box::new(function_expr()),
                args: Vec::new(),
            },
        );
        assert_eq!(
            BoolExpr::tuple_index(tuple_expr(), 0).kind(),
            &BoolExprKind::TupleIndex {
                tuple: Box::new(tuple_expr()),
                index: 0,
            },
        );
        assert_eq!(
            BoolExpr::not(BoolExpr::value(true)).kind(),
            &BoolExprKind::Not(Box::new(BoolExpr::value(true))),
        );
        assert_eq!(
            BoolExpr::lt_int(IntExpr::value(1.into()), IntExpr::value(2.into())).kind(),
            &BoolExprKind::LtInt {
                left: Box::new(IntExpr::value(1.into())),
                right: Box::new(IntExpr::value(2.into())),
            },
        );
        assert_eq!(
            BoolExpr::lte_int(IntExpr::value(1.into()), IntExpr::value(2.into())).kind(),
            &BoolExprKind::LtEqInt {
                left: Box::new(IntExpr::value(1.into())),
                right: Box::new(IntExpr::value(2.into())),
            },
        );
        assert_eq!(
            BoolExpr::gt_int(IntExpr::value(2.into()), IntExpr::value(1.into())).kind(),
            &BoolExprKind::GtInt {
                left: Box::new(IntExpr::value(2.into())),
                right: Box::new(IntExpr::value(1.into())),
            },
        );
        assert_eq!(
            BoolExpr::gte_int(IntExpr::value(2.into()), IntExpr::value(1.into())).kind(),
            &BoolExprKind::GtEqInt {
                left: Box::new(IntExpr::value(2.into())),
                right: Box::new(IntExpr::value(1.into())),
            },
        );
        assert_eq!(
            BoolExpr::lt_float(FloatExpr::value(1.0), FloatExpr::value(2.0)).kind(),
            &BoolExprKind::LtFloat {
                left: Box::new(FloatExpr::value(1.0)),
                right: Box::new(FloatExpr::value(2.0)),
            },
        );
        assert_eq!(
            BoolExpr::lte_float(FloatExpr::value(1.0), FloatExpr::value(2.0)).kind(),
            &BoolExprKind::LtEqFloat {
                left: Box::new(FloatExpr::value(1.0)),
                right: Box::new(FloatExpr::value(2.0)),
            },
        );
        assert_eq!(
            BoolExpr::gt_float(FloatExpr::value(2.0), FloatExpr::value(1.0)).kind(),
            &BoolExprKind::GtFloat {
                left: Box::new(FloatExpr::value(2.0)),
                right: Box::new(FloatExpr::value(1.0)),
            },
        );
        assert_eq!(
            BoolExpr::gte_float(FloatExpr::value(2.0), FloatExpr::value(1.0)).kind(),
            &BoolExprKind::GtEqFloat {
                left: Box::new(FloatExpr::value(2.0)),
                right: Box::new(FloatExpr::value(1.0)),
            },
        );
        assert_eq!(
            BoolExpr::equal(
                Expr::int(IntExpr::value(1.into())),
                Expr::int(IntExpr::value(1.into()))
            )
            .kind(),
            &BoolExprKind::Equal {
                left: Box::new(Expr::int(IntExpr::value(1.into()))),
                right: Box::new(Expr::int(IntExpr::value(1.into()))),
            },
        );
        assert_eq!(
            BoolExpr::not_equal(
                Expr::bool(BoolExpr::value(true)),
                Expr::bool(BoolExpr::value(false))
            )
            .kind(),
            &BoolExprKind::NotEqual {
                left: Box::new(Expr::bool(BoolExpr::value(true))),
                right: Box::new(Expr::bool(BoolExpr::value(false))),
            },
        );
        assert_eq!(
            BoolExpr::string_starts_with(StringExpr::value("geam".into()), "ge".into()).kind(),
            &BoolExprKind::StringStartsWith {
                value: Box::new(StringExpr::value("geam".into())),
                prefix: "ge".into(),
            },
        );
        assert_eq!(
            BoolExpr::and(BoolExpr::value(true), BoolExpr::value(false)).kind(),
            &BoolExprKind::And {
                left: Box::new(BoolExpr::value(true)),
                right: Box::new(BoolExpr::value(false)),
            },
        );
        assert_eq!(
            BoolExpr::or(BoolExpr::value(true), BoolExpr::value(false)).kind(),
            &BoolExprKind::Or {
                left: Box::new(BoolExpr::value(true)),
                right: Box::new(BoolExpr::value(false)),
            },
        );
        assert_eq!(
            BoolExpr::bool_case(
                BoolExpr::value(true),
                BoolExpr::value(true),
                BoolExpr::value(false)
            )
            .kind(),
            &BoolExprKind::BoolCase {
                subject: Box::new(BoolExpr::value(true)),
                true_: Box::new(BoolExpr::value(true)),
                false_: Box::new(BoolExpr::value(false)),
            },
        );
        assert_eq!(
            BoolExpr::int_case(
                IntExpr::value(1.into()),
                vec![(1.into(), BoolExpr::value(true))],
                BoolExpr::value(false)
            )
            .kind(),
            &BoolExprKind::IntCase {
                subject: Box::new(IntExpr::value(1.into())),
                clauses: vec![(BigInt::from(1), BoolExpr::value(true))],
                fallback: Box::new(BoolExpr::value(false)),
            },
        );
        assert_eq!(
            BoolExpr::string_case(
                crate::plan::StringExpr::value("a".into()),
                vec![("a".into(), BoolExpr::value(true))],
                BoolExpr::value(false)
            )
            .kind(),
            &BoolExprKind::StringCase {
                subject: Box::new(crate::plan::StringExpr::value("a".into())),
                clauses: vec![("a".into(), BoolExpr::value(true))],
                fallback: Box::new(BoolExpr::value(false)),
            },
        );
        assert_eq!(
            BoolExpr::float_case(
                FloatExpr::value(1.0),
                vec![(1.0, BoolExpr::value(true))],
                BoolExpr::value(false)
            )
            .kind(),
            &BoolExprKind::FloatCase {
                subject: Box::new(FloatExpr::value(1.0)),
                clauses: vec![(1.0, BoolExpr::value(true))],
                fallback: Box::new(BoolExpr::value(false)),
            },
        );
        assert_eq!(
            BoolExpr::block(
                vec![Step::evaluate(Expr::bool(BoolExpr::value(false)))],
                BoolExpr::value(true),
            )
            .kind(),
            &BoolExprKind::Block {
                steps: vec![Step::evaluate(Expr::bool(BoolExpr::value(false)))],
                return_: Box::new(BoolExpr::value(true)),
            },
        );
    }

    fn function_expr() -> crate::plan::BoolFunctionExpr {
        crate::plan::BoolFunctionExpr::reference(BoolFunctionReference::new(
            BoolFunctionId(0),
            Vec::new(),
        ))
    }

    fn tuple_expr() -> TupleExpr {
        TupleExpr::value(
            vec![Expr::bool(BoolExpr::value(true))],
            vec![ValueType::Bool],
        )
    }
}
