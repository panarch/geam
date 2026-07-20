use super::{
    BoolExpr, CallArg, CustomFieldAccess, FloatExpr, IntFunctionExpr, IntListExpr, PanicExpr,
    StringExpr, TupleExpr,
};
use crate::plan::{ConstantIntReference, FunctionInstantiation, IntLocalId, Step};
use ecow::EcoString;
use num_bigint::BigInt;

#[derive(Debug, Clone, PartialEq)]
pub struct IntExpr {
    kind: IntExprKind,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum IntExprKind {
    Value(BigInt),
    Constant(ConstantIntReference),
    LocalGet {
        local: IntLocalId,
        name: EcoString,
    },
    Call {
        function: FunctionInstantiation,
        args: Vec<CallArg>,
    },
    FunctionCall {
        function: Box<IntFunctionExpr>,
        args: Vec<CallArg>,
    },
    TupleIndex {
        tuple: Box<TupleExpr>,
        index: usize,
    },
    CustomField(CustomFieldAccess),
    ListIndex {
        list: Box<IntListExpr>,
        index: usize,
    },
    Panic(PanicExpr),
    Add {
        left: Box<IntExpr>,
        right: Box<IntExpr>,
    },
    Sub {
        left: Box<IntExpr>,
        right: Box<IntExpr>,
    },
    Mult {
        left: Box<IntExpr>,
        right: Box<IntExpr>,
    },
    Div {
        left: Box<IntExpr>,
        right: Box<IntExpr>,
    },
    Remainder {
        left: Box<IntExpr>,
        right: Box<IntExpr>,
    },
    Negate(Box<IntExpr>),
    BoolCase {
        subject: Box<BoolExpr>,
        true_: Box<IntExpr>,
        false_: Box<IntExpr>,
    },
    IntCase {
        subject: Box<IntExpr>,
        clauses: Vec<(BigInt, IntExpr)>,
        fallback: Box<IntExpr>,
    },
    StringCase {
        subject: Box<StringExpr>,
        clauses: Vec<(EcoString, IntExpr)>,
        fallback: Box<IntExpr>,
    },
    FloatCase {
        subject: Box<FloatExpr>,
        clauses: Vec<(f64, IntExpr)>,
        fallback: Box<IntExpr>,
    },
    Block {
        steps: Vec<Step>,
        return_: Box<IntExpr>,
    },
}

impl IntExpr {
    pub(crate) fn value(value: BigInt) -> Self {
        Self {
            kind: IntExprKind::Value(value),
        }
    }

    pub(in crate::plan::module) fn constant(reference: ConstantIntReference) -> Self {
        Self {
            kind: IntExprKind::Constant(reference),
        }
    }

    pub(crate) fn local_get(local: IntLocalId, name: EcoString) -> Self {
        Self {
            kind: IntExprKind::LocalGet { local, name },
        }
    }

    pub(crate) fn call(function: FunctionInstantiation, args: Vec<CallArg>) -> Self {
        Self {
            kind: IntExprKind::Call { function, args },
        }
    }

    pub(crate) fn function_call(function: IntFunctionExpr, args: Vec<CallArg>) -> Self {
        Self {
            kind: IntExprKind::FunctionCall {
                function: Box::new(function),
                args,
            },
        }
    }

    pub(crate) fn tuple_index(tuple: TupleExpr, index: usize) -> Self {
        Self {
            kind: IntExprKind::TupleIndex {
                tuple: Box::new(tuple),
                index,
            },
        }
    }

    pub(crate) fn custom_field(access: CustomFieldAccess) -> Self {
        Self {
            kind: IntExprKind::CustomField(access),
        }
    }

    pub(crate) fn list_index(list: impl Into<IntListExpr>, index: usize) -> Self {
        Self {
            kind: IntExprKind::ListIndex {
                list: Box::new(list.into()),
                index,
            },
        }
    }

    pub(crate) fn panic(panic: PanicExpr) -> Self {
        Self {
            kind: IntExprKind::Panic(panic),
        }
    }

    pub(crate) fn add(left: IntExpr, right: IntExpr) -> Self {
        Self {
            kind: IntExprKind::Add {
                left: Box::new(left),
                right: Box::new(right),
            },
        }
    }

    pub(crate) fn sub(left: IntExpr, right: IntExpr) -> Self {
        Self {
            kind: IntExprKind::Sub {
                left: Box::new(left),
                right: Box::new(right),
            },
        }
    }

    pub(crate) fn mult(left: IntExpr, right: IntExpr) -> Self {
        Self {
            kind: IntExprKind::Mult {
                left: Box::new(left),
                right: Box::new(right),
            },
        }
    }

    pub(crate) fn div(left: IntExpr, right: IntExpr) -> Self {
        Self {
            kind: IntExprKind::Div {
                left: Box::new(left),
                right: Box::new(right),
            },
        }
    }

    pub(crate) fn remainder(left: IntExpr, right: IntExpr) -> Self {
        Self {
            kind: IntExprKind::Remainder {
                left: Box::new(left),
                right: Box::new(right),
            },
        }
    }

    pub(crate) fn negate(value: IntExpr) -> Self {
        Self {
            kind: IntExprKind::Negate(Box::new(value)),
        }
    }

    pub(crate) fn bool_case(subject: BoolExpr, true_: IntExpr, false_: IntExpr) -> Self {
        Self {
            kind: IntExprKind::BoolCase {
                subject: Box::new(subject),
                true_: Box::new(true_),
                false_: Box::new(false_),
            },
        }
    }

    pub(crate) fn int_case(
        subject: IntExpr,
        clauses: Vec<(BigInt, IntExpr)>,
        fallback: IntExpr,
    ) -> Self {
        Self {
            kind: IntExprKind::IntCase {
                subject: Box::new(subject),
                clauses,
                fallback: Box::new(fallback),
            },
        }
    }

    pub(crate) fn string_case(
        subject: StringExpr,
        clauses: Vec<(EcoString, IntExpr)>,
        fallback: IntExpr,
    ) -> Self {
        Self {
            kind: IntExprKind::StringCase {
                subject: Box::new(subject),
                clauses,
                fallback: Box::new(fallback),
            },
        }
    }

    pub(crate) fn float_case(
        subject: FloatExpr,
        clauses: Vec<(f64, IntExpr)>,
        fallback: IntExpr,
    ) -> Self {
        Self {
            kind: IntExprKind::FloatCase {
                subject: Box::new(subject),
                clauses,
                fallback: Box::new(fallback),
            },
        }
    }

    pub(crate) fn block(steps: Vec<Step>, return_: IntExpr) -> Self {
        Self {
            kind: IntExprKind::Block {
                steps,
                return_: Box::new(return_),
            },
        }
    }

    pub(crate) fn kind(&self) -> &IntExprKind {
        &self.kind
    }
}

#[cfg(test)]
mod tests {
    use super::{IntExpr, IntExprKind};
    use crate::plan::{
        BoolExpr, Expr, FunctionInstantiation, FunctionShape, IntFunctionReference, IntLocalId,
        Step, TupleExpr, ValueShape, ValueType, monomorphic_function_instantiation,
    };
    use num_bigint::BigInt;

    #[test]
    fn int_expr_kind_accessors() {
        assert_eq!(
            IntExpr::value(1.into()).kind(),
            &IntExprKind::Value(BigInt::from(1)),
        );
        assert_eq!(
            IntExpr::local_get(IntLocalId(0), "value".into()).kind(),
            &IntExprKind::LocalGet {
                local: IntLocalId(0),
                name: "value".into(),
            },
        );
        assert_eq!(
            IntExpr::call(function_instantiation(), Vec::new()).kind(),
            &IntExprKind::Call {
                function: function_instantiation(),
                args: Vec::new(),
            },
        );
        assert_eq!(
            IntExpr::function_call(function_expr(), Vec::new()).kind(),
            &IntExprKind::FunctionCall {
                function: Box::new(function_expr()),
                args: Vec::new(),
            },
        );
        assert_eq!(
            IntExpr::tuple_index(tuple_expr(), 0).kind(),
            &IntExprKind::TupleIndex {
                tuple: Box::new(tuple_expr()),
                index: 0,
            },
        );
        assert_eq!(
            IntExpr::add(IntExpr::value(1.into()), IntExpr::value(2.into())).kind(),
            &IntExprKind::Add {
                left: Box::new(IntExpr::value(1.into())),
                right: Box::new(IntExpr::value(2.into())),
            },
        );
        assert_eq!(
            IntExpr::sub(IntExpr::value(1.into()), IntExpr::value(2.into())).kind(),
            &IntExprKind::Sub {
                left: Box::new(IntExpr::value(1.into())),
                right: Box::new(IntExpr::value(2.into())),
            },
        );
        assert_eq!(
            IntExpr::mult(IntExpr::value(1.into()), IntExpr::value(2.into())).kind(),
            &IntExprKind::Mult {
                left: Box::new(IntExpr::value(1.into())),
                right: Box::new(IntExpr::value(2.into())),
            },
        );
        assert_eq!(
            IntExpr::div(IntExpr::value(1.into()), IntExpr::value(2.into())).kind(),
            &IntExprKind::Div {
                left: Box::new(IntExpr::value(1.into())),
                right: Box::new(IntExpr::value(2.into())),
            },
        );
        assert_eq!(
            IntExpr::remainder(IntExpr::value(1.into()), IntExpr::value(2.into())).kind(),
            &IntExprKind::Remainder {
                left: Box::new(IntExpr::value(1.into())),
                right: Box::new(IntExpr::value(2.into())),
            },
        );
        assert_eq!(
            IntExpr::negate(IntExpr::value(1.into())).kind(),
            &IntExprKind::Negate(Box::new(IntExpr::value(1.into()))),
        );
        assert_eq!(
            IntExpr::bool_case(
                BoolExpr::value(true),
                IntExpr::value(1.into()),
                IntExpr::value(0.into())
            )
            .kind(),
            &IntExprKind::BoolCase {
                subject: Box::new(BoolExpr::value(true)),
                true_: Box::new(IntExpr::value(1.into())),
                false_: Box::new(IntExpr::value(0.into())),
            },
        );
        assert_eq!(
            IntExpr::int_case(
                IntExpr::value(1.into()),
                vec![(1.into(), IntExpr::value(10.into()))],
                IntExpr::value(0.into())
            )
            .kind(),
            &IntExprKind::IntCase {
                subject: Box::new(IntExpr::value(1.into())),
                clauses: vec![(BigInt::from(1), IntExpr::value(10.into()))],
                fallback: Box::new(IntExpr::value(0.into())),
            },
        );
        assert_eq!(
            IntExpr::string_case(
                crate::plan::StringExpr::value("a".into()),
                vec![("a".into(), IntExpr::value(10.into()))],
                IntExpr::value(0.into())
            )
            .kind(),
            &IntExprKind::StringCase {
                subject: Box::new(crate::plan::StringExpr::value("a".into())),
                clauses: vec![("a".into(), IntExpr::value(10.into()))],
                fallback: Box::new(IntExpr::value(0.into())),
            },
        );
        assert_eq!(
            IntExpr::float_case(
                crate::plan::FloatExpr::value(1.0),
                vec![(1.0, IntExpr::value(10.into()))],
                IntExpr::value(0.into())
            )
            .kind(),
            &IntExprKind::FloatCase {
                subject: Box::new(crate::plan::FloatExpr::value(1.0)),
                clauses: vec![(1.0, IntExpr::value(10.into()))],
                fallback: Box::new(IntExpr::value(0.into())),
            },
        );
        assert_eq!(
            IntExpr::block(
                vec![Step::evaluate(Expr::int(IntExpr::value(1.into())))],
                IntExpr::value(2.into()),
            )
            .kind(),
            &IntExprKind::Block {
                steps: vec![Step::evaluate(Expr::int(IntExpr::value(1.into())))],
                return_: Box::new(IntExpr::value(2.into())),
            },
        );
    }

    fn function_expr() -> crate::plan::IntFunctionExpr {
        crate::plan::IntFunctionExpr::reference(IntFunctionReference::new(
            function_instantiation(),
            Vec::new(),
        ))
    }

    fn function_instantiation() -> FunctionInstantiation {
        monomorphic_function_instantiation(0, FunctionShape::new(Vec::new(), ValueShape::Int))
    }

    fn tuple_expr() -> TupleExpr {
        TupleExpr::value(
            vec![Expr::int(IntExpr::value(1.into()))],
            vec![ValueType::Int],
        )
    }
}
