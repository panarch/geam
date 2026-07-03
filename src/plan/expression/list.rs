use super::{BoolExpr, CallArg, FloatExpr, IntExpr, ListFunctionExpr, StringExpr, TupleExpr};
use crate::plan::{ListFunctionId, ListLocalId, Step, ValueType};
use ecow::EcoString;
use num_bigint::BigInt;

#[derive(Debug, Clone, PartialEq)]
pub struct ListExpr {
    element_type: Box<ValueType>,
    kind: ListExprKind,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum ListExprKind {
    Value(Vec<super::Expr>),
    Spread {
        elements: Vec<super::Expr>,
        tail: Box<ListExpr>,
    },
    LocalGet {
        local: ListLocalId,
        name: EcoString,
    },
    Call {
        function: ListFunctionId,
        args: Vec<CallArg>,
    },
    FunctionCall {
        function: Box<ListFunctionExpr>,
        args: Vec<CallArg>,
    },
    TupleIndex {
        tuple: Box<TupleExpr>,
        index: usize,
    },
    BoolCase {
        subject: Box<BoolExpr>,
        true_: Box<ListExpr>,
        false_: Box<ListExpr>,
    },
    IntCase {
        subject: Box<IntExpr>,
        clauses: Vec<(BigInt, ListExpr)>,
        fallback: Box<ListExpr>,
    },
    StringCase {
        subject: Box<StringExpr>,
        clauses: Vec<(EcoString, ListExpr)>,
        fallback: Box<ListExpr>,
    },
    FloatCase {
        subject: Box<FloatExpr>,
        clauses: Vec<(f64, ListExpr)>,
        fallback: Box<ListExpr>,
    },
    Block {
        steps: Vec<Step>,
        return_: Box<ListExpr>,
    },
}

impl ListExpr {
    pub(crate) fn value(elements: Vec<super::Expr>, element_type: ValueType) -> Self {
        Self {
            element_type: Box::new(element_type),
            kind: ListExprKind::Value(elements),
        }
    }

    pub(crate) fn spread(
        elements: Vec<super::Expr>,
        tail: ListExpr,
        element_type: ValueType,
    ) -> Self {
        Self {
            element_type: Box::new(element_type),
            kind: ListExprKind::Spread {
                elements,
                tail: Box::new(tail),
            },
        }
    }

    pub(crate) fn local_get(local: ListLocalId, name: EcoString, element_type: ValueType) -> Self {
        Self {
            element_type: Box::new(element_type),
            kind: ListExprKind::LocalGet { local, name },
        }
    }

    pub(crate) fn call(
        function: ListFunctionId,
        args: Vec<CallArg>,
        element_type: ValueType,
    ) -> Self {
        Self {
            element_type: Box::new(element_type),
            kind: ListExprKind::Call { function, args },
        }
    }

    pub(crate) fn function_call(
        function: ListFunctionExpr,
        args: Vec<CallArg>,
        element_type: ValueType,
    ) -> Self {
        Self {
            element_type: Box::new(element_type),
            kind: ListExprKind::FunctionCall {
                function: Box::new(function),
                args,
            },
        }
    }

    pub(crate) fn tuple_index(tuple: TupleExpr, index: usize, element_type: ValueType) -> Self {
        Self {
            element_type: Box::new(element_type),
            kind: ListExprKind::TupleIndex {
                tuple: Box::new(tuple),
                index,
            },
        }
    }

    pub(crate) fn bool_case(subject: BoolExpr, true_: ListExpr, false_: ListExpr) -> Self {
        Self {
            element_type: true_.element_type.clone(),
            kind: ListExprKind::BoolCase {
                subject: Box::new(subject),
                true_: Box::new(true_),
                false_: Box::new(false_),
            },
        }
    }

    pub(crate) fn int_case(
        subject: IntExpr,
        clauses: Vec<(BigInt, ListExpr)>,
        fallback: ListExpr,
    ) -> Self {
        Self {
            element_type: fallback.element_type.clone(),
            kind: ListExprKind::IntCase {
                subject: Box::new(subject),
                clauses,
                fallback: Box::new(fallback),
            },
        }
    }

    pub(crate) fn string_case(
        subject: StringExpr,
        clauses: Vec<(EcoString, ListExpr)>,
        fallback: ListExpr,
    ) -> Self {
        Self {
            element_type: fallback.element_type.clone(),
            kind: ListExprKind::StringCase {
                subject: Box::new(subject),
                clauses,
                fallback: Box::new(fallback),
            },
        }
    }

    pub(crate) fn float_case(
        subject: FloatExpr,
        clauses: Vec<(f64, ListExpr)>,
        fallback: ListExpr,
    ) -> Self {
        Self {
            element_type: fallback.element_type.clone(),
            kind: ListExprKind::FloatCase {
                subject: Box::new(subject),
                clauses,
                fallback: Box::new(fallback),
            },
        }
    }

    pub(crate) fn block(steps: Vec<Step>, return_: ListExpr) -> Self {
        Self {
            element_type: return_.element_type.clone(),
            kind: ListExprKind::Block {
                steps,
                return_: Box::new(return_),
            },
        }
    }

    pub fn element_type(&self) -> &ValueType {
        &self.element_type
    }

    pub(crate) fn kind(&self) -> &ListExprKind {
        &self.kind
    }
}

#[cfg(test)]
mod tests {
    use super::{ListExpr, ListExprKind};
    use crate::plan::{
        BoolExpr, Expr, FunctionType, IntExpr, ListFunctionExpr, ListFunctionId, ListFunctionValue,
        ListLocalId, ParamLocal, Step, ValueType,
    };

    #[test]
    fn list_expr_kind_accessors() {
        assert!(matches!(list_value().kind(), ListExprKind::Value(_)));
        assert!(matches!(
            ListExpr::spread(
                vec![Expr::int(IntExpr::value(0.into()))],
                list_value(),
                element_type()
            )
            .kind(),
            ListExprKind::Spread { .. }
        ));
        assert!(matches!(
            ListExpr::local_get(ListLocalId(0), "values".into(), element_type()).kind(),
            ListExprKind::LocalGet { .. }
        ));
        assert!(matches!(
            ListExpr::call(ListFunctionId(0), Vec::new(), element_type()).kind(),
            ListExprKind::Call { .. }
        ));
        assert!(matches!(
            ListExpr::function_call(list_function_expr(), Vec::new(), element_type()).kind(),
            ListExprKind::FunctionCall { .. }
        ));
        assert!(matches!(
            ListExpr::tuple_index(tuple_expr(), 0, element_type()).kind(),
            ListExprKind::TupleIndex { .. }
        ));
        assert!(matches!(
            ListExpr::bool_case(BoolExpr::value(true), list_value(), list_value()).kind(),
            ListExprKind::BoolCase { .. }
        ));
        assert!(matches!(
            ListExpr::int_case(
                IntExpr::value(1.into()),
                vec![(1.into(), list_value())],
                list_value(),
            )
            .kind(),
            ListExprKind::IntCase { .. }
        ));
        assert!(matches!(
            ListExpr::string_case(
                crate::plan::StringExpr::value("one".into()),
                vec![("one".into(), list_value())],
                list_value(),
            )
            .kind(),
            ListExprKind::StringCase { .. }
        ));
        assert!(matches!(
            ListExpr::float_case(
                crate::plan::FloatExpr::value(1.0),
                vec![(1.0, list_value())],
                list_value(),
            )
            .kind(),
            ListExprKind::FloatCase { .. }
        ));
        assert!(matches!(
            ListExpr::block(
                vec![Step::evaluate(Expr::int(IntExpr::value(1.into())))],
                list_value(),
            )
            .kind(),
            ListExprKind::Block { .. }
        ));
    }

    #[test]
    fn list_expr_element_type() {
        assert_eq!(list_value().element_type(), &element_type());
        assert_eq!(
            ListExpr::spread(
                vec![Expr::int(IntExpr::value(0.into()))],
                list_value(),
                element_type()
            )
            .element_type(),
            &element_type(),
        );
        assert_eq!(
            ListExpr::bool_case(BoolExpr::value(true), list_value(), list_value()).element_type(),
            &element_type(),
        );
        assert_eq!(
            ListExpr::int_case(
                IntExpr::value(1.into()),
                vec![(1.into(), list_value())],
                list_value(),
            )
            .element_type(),
            &element_type(),
        );
        assert_eq!(
            ListExpr::string_case(
                crate::plan::StringExpr::value("one".into()),
                vec![("one".into(), list_value())],
                list_value(),
            )
            .element_type(),
            &element_type(),
        );
        assert_eq!(
            ListExpr::float_case(
                crate::plan::FloatExpr::value(1.0),
                vec![(1.0, list_value())],
                list_value(),
            )
            .element_type(),
            &element_type(),
        );
        assert_eq!(
            ListExpr::block(Vec::new(), list_value()).element_type(),
            &element_type(),
        );
    }

    fn list_value() -> ListExpr {
        ListExpr::value(vec![Expr::int(IntExpr::value(1.into()))], element_type())
    }

    fn element_type() -> ValueType {
        ValueType::Int
    }

    fn list_function_expr() -> ListFunctionExpr {
        ListFunctionExpr::value(ListFunctionValue::new(
            ListFunctionId(0),
            vec![ParamLocal::int(crate::plan::IntLocalId(0))],
            element_type(),
        ))
    }

    fn tuple_expr() -> crate::plan::TupleExpr {
        crate::plan::TupleExpr::value(
            vec![Expr::list(list_value())],
            vec![ValueType::List(Box::new(element_type()))],
        )
    }

    #[test]
    fn list_function_value_type_fixture() {
        assert_eq!(
            list_function_expr().type_(),
            &FunctionType::new(
                vec![ValueType::Int],
                ValueType::List(Box::new(element_type())),
            ),
        );
    }
}
