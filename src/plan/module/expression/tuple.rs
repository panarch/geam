use super::{
    BoolExpr, CallArg, FloatExpr, IntExpr, PanicExpr, StringExpr, TupleFunctionExpr, TupleListExpr,
};
use crate::plan::{Step, TupleFunctionId, TupleLocalId, ValueType};
use ecow::EcoString;
use num_bigint::BigInt;

#[derive(Debug, Clone, PartialEq)]
pub struct TupleExpr {
    type_: Vec<ValueType>,
    kind: TupleExprKind,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum TupleExprKind {
    Value(Vec<super::Expr>),
    LocalGet {
        local: TupleLocalId,
        name: EcoString,
    },
    Call {
        function: TupleFunctionId,
        args: Vec<CallArg>,
    },
    FunctionCall {
        function: Box<TupleFunctionExpr>,
        args: Vec<CallArg>,
    },
    TupleIndex {
        tuple: Box<TupleExpr>,
        index: usize,
    },
    ListIndex {
        list: Box<TupleListExpr>,
        index: usize,
    },
    Panic(PanicExpr),
    BoolCase {
        subject: Box<BoolExpr>,
        true_: Box<TupleExpr>,
        false_: Box<TupleExpr>,
    },
    IntCase {
        subject: Box<IntExpr>,
        clauses: Vec<(BigInt, TupleExpr)>,
        fallback: Box<TupleExpr>,
    },
    StringCase {
        subject: Box<StringExpr>,
        clauses: Vec<(EcoString, TupleExpr)>,
        fallback: Box<TupleExpr>,
    },
    FloatCase {
        subject: Box<FloatExpr>,
        clauses: Vec<(f64, TupleExpr)>,
        fallback: Box<TupleExpr>,
    },
    Block {
        steps: Vec<Step>,
        return_: Box<TupleExpr>,
    },
}

impl TupleExpr {
    pub(crate) fn value(elements: Vec<super::Expr>, type_: Vec<ValueType>) -> Self {
        Self {
            type_,
            kind: TupleExprKind::Value(elements),
        }
    }

    pub(crate) fn local_get(local: TupleLocalId, name: EcoString, type_: Vec<ValueType>) -> Self {
        Self {
            type_,
            kind: TupleExprKind::LocalGet { local, name },
        }
    }

    pub(crate) fn call(
        function: TupleFunctionId,
        args: Vec<CallArg>,
        type_: Vec<ValueType>,
    ) -> Self {
        Self {
            type_,
            kind: TupleExprKind::Call { function, args },
        }
    }

    pub(crate) fn function_call(
        function: TupleFunctionExpr,
        args: Vec<CallArg>,
        type_: Vec<ValueType>,
    ) -> Self {
        Self {
            type_,
            kind: TupleExprKind::FunctionCall {
                function: Box::new(function),
                args,
            },
        }
    }

    pub(crate) fn tuple_index(tuple: TupleExpr, index: usize, type_: Vec<ValueType>) -> Self {
        Self {
            type_,
            kind: TupleExprKind::TupleIndex {
                tuple: Box::new(tuple),
                index,
            },
        }
    }

    pub(crate) fn list_index(
        list: impl Into<TupleListExpr>,
        index: usize,
        type_: Vec<ValueType>,
    ) -> Self {
        Self {
            type_,
            kind: TupleExprKind::ListIndex {
                list: Box::new(list.into()),
                index,
            },
        }
    }

    pub(crate) fn panic(panic: PanicExpr, type_: Vec<ValueType>) -> Self {
        Self {
            type_,
            kind: TupleExprKind::Panic(panic),
        }
    }

    pub(crate) fn bool_case(subject: BoolExpr, true_: TupleExpr, false_: TupleExpr) -> Self {
        Self {
            type_: true_.type_.clone(),
            kind: TupleExprKind::BoolCase {
                subject: Box::new(subject),
                true_: Box::new(true_),
                false_: Box::new(false_),
            },
        }
    }

    pub(crate) fn int_case(
        subject: IntExpr,
        clauses: Vec<(BigInt, TupleExpr)>,
        fallback: TupleExpr,
    ) -> Self {
        Self {
            type_: fallback.type_.clone(),
            kind: TupleExprKind::IntCase {
                subject: Box::new(subject),
                clauses,
                fallback: Box::new(fallback),
            },
        }
    }

    pub(crate) fn string_case(
        subject: StringExpr,
        clauses: Vec<(EcoString, TupleExpr)>,
        fallback: TupleExpr,
    ) -> Self {
        Self {
            type_: fallback.type_.clone(),
            kind: TupleExprKind::StringCase {
                subject: Box::new(subject),
                clauses,
                fallback: Box::new(fallback),
            },
        }
    }

    pub(crate) fn float_case(
        subject: FloatExpr,
        clauses: Vec<(f64, TupleExpr)>,
        fallback: TupleExpr,
    ) -> Self {
        Self {
            type_: fallback.type_.clone(),
            kind: TupleExprKind::FloatCase {
                subject: Box::new(subject),
                clauses,
                fallback: Box::new(fallback),
            },
        }
    }

    pub(crate) fn block(steps: Vec<Step>, return_: TupleExpr) -> Self {
        Self {
            type_: return_.type_.clone(),
            kind: TupleExprKind::Block {
                steps,
                return_: Box::new(return_),
            },
        }
    }

    pub fn type_(&self) -> &[ValueType] {
        &self.type_
    }

    pub(crate) fn kind(&self) -> &TupleExprKind {
        &self.kind
    }

    pub(crate) fn into_parts(self) -> (Vec<ValueType>, TupleExprKind) {
        (self.type_, self.kind)
    }
}

#[cfg(test)]
mod tests {
    use super::{TupleExpr, TupleExprKind};
    use crate::plan::{
        BoolExpr, Expr, FunctionType, IntExpr, Step, TupleFunctionExpr, TupleFunctionId,
        TupleFunctionReference, TupleLocalId, ValueType,
    };

    #[test]
    fn tuple_expr_kind_accessors() {
        assert_eq!(
            tuple_value().kind(),
            &TupleExprKind::Value(vec![Expr::int(IntExpr::value(1.into()))]),
        );
        assert_eq!(
            TupleExpr::local_get(TupleLocalId(0), "pair".into(), tuple_type()).kind(),
            &TupleExprKind::LocalGet {
                local: TupleLocalId(0),
                name: "pair".into(),
            },
        );
        assert_eq!(
            TupleExpr::call(TupleFunctionId(0), Vec::new(), tuple_type()).kind(),
            &TupleExprKind::Call {
                function: TupleFunctionId(0),
                args: Vec::new(),
            },
        );
        assert_eq!(
            TupleExpr::function_call(tuple_function_expr(), Vec::new(), tuple_type()).kind(),
            &TupleExprKind::FunctionCall {
                function: Box::new(tuple_function_expr()),
                args: Vec::new(),
            },
        );
        assert_eq!(
            TupleExpr::tuple_index(tuple_value(), 0, tuple_type()).kind(),
            &TupleExprKind::TupleIndex {
                tuple: Box::new(tuple_value()),
                index: 0,
            },
        );
        assert_eq!(
            TupleExpr::bool_case(BoolExpr::value(true), tuple_value(), tuple_value()).kind(),
            &TupleExprKind::BoolCase {
                subject: Box::new(BoolExpr::value(true)),
                true_: Box::new(tuple_value()),
                false_: Box::new(tuple_value()),
            },
        );
        assert_eq!(
            TupleExpr::int_case(
                IntExpr::value(1.into()),
                vec![(1.into(), tuple_value())],
                tuple_value(),
            )
            .kind(),
            &TupleExprKind::IntCase {
                subject: Box::new(IntExpr::value(1.into())),
                clauses: vec![(1.into(), tuple_value())],
                fallback: Box::new(tuple_value()),
            },
        );
        assert_eq!(
            TupleExpr::string_case(
                crate::plan::StringExpr::value("one".into()),
                vec![("one".into(), tuple_value())],
                tuple_value(),
            )
            .kind(),
            &TupleExprKind::StringCase {
                subject: Box::new(crate::plan::StringExpr::value("one".into())),
                clauses: vec![("one".into(), tuple_value())],
                fallback: Box::new(tuple_value()),
            },
        );
        assert_eq!(
            TupleExpr::float_case(
                crate::plan::FloatExpr::value(1.0),
                vec![(1.0, tuple_value())],
                tuple_value(),
            )
            .kind(),
            &TupleExprKind::FloatCase {
                subject: Box::new(crate::plan::FloatExpr::value(1.0)),
                clauses: vec![(1.0, tuple_value())],
                fallback: Box::new(tuple_value()),
            },
        );
        assert_eq!(
            TupleExpr::block(
                vec![Step::evaluate(Expr::int(IntExpr::value(1.into())))],
                tuple_value(),
            )
            .kind(),
            &TupleExprKind::Block {
                steps: vec![Step::evaluate(Expr::int(IntExpr::value(1.into())))],
                return_: Box::new(tuple_value()),
            },
        );
    }

    #[test]
    fn tuple_expr_type() {
        assert_eq!(tuple_value().type_(), tuple_type());
        assert_eq!(
            TupleExpr::bool_case(BoolExpr::value(true), tuple_value(), tuple_value()).type_(),
            tuple_type(),
        );
        assert_eq!(
            TupleExpr::int_case(
                IntExpr::value(1.into()),
                vec![(1.into(), tuple_value())],
                tuple_value(),
            )
            .type_(),
            tuple_type(),
        );
        assert_eq!(
            TupleExpr::string_case(
                crate::plan::StringExpr::value("one".into()),
                vec![("one".into(), tuple_value())],
                tuple_value(),
            )
            .type_(),
            tuple_type(),
        );
        assert_eq!(
            TupleExpr::float_case(
                crate::plan::FloatExpr::value(1.0),
                vec![(1.0, tuple_value())],
                tuple_value(),
            )
            .type_(),
            tuple_type(),
        );
        assert_eq!(
            TupleExpr::block(Vec::new(), tuple_value()).type_(),
            tuple_type(),
        );
    }

    fn tuple_value() -> TupleExpr {
        TupleExpr::value(
            vec![Expr::int(IntExpr::value(1.into()))],
            vec![ValueType::Int],
        )
    }

    fn tuple_type() -> Vec<ValueType> {
        vec![ValueType::Int]
    }

    fn tuple_function_expr() -> TupleFunctionExpr {
        TupleFunctionExpr::reference(
            TupleFunctionReference::new(TupleFunctionId(0), Vec::new()),
            tuple_type(),
        )
    }

    #[test]
    fn tuple_function_value_type_fixture() {
        assert_eq!(
            tuple_function_expr().type_(),
            &FunctionType::new(Vec::new(), ValueType::Tuple(tuple_type())),
        );
    }
}
