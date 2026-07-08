use super::{
    BoolExpr, CallArg, Expr, FloatExpr, FunctionExpr, IntExpr, ListFunctionExpr, NilExpr,
    PanicExpr, StringExpr, TupleExpr,
};
use crate::plan::{FunctionType, ListFunctionId, ListLocal, Step, ValueType};
use ecow::EcoString;
use num_bigint::BigInt;

#[derive(Debug, Clone, PartialEq)]
pub struct ListExpr {
    element_type: Box<ValueType>,
    kind: ListExprKind,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum ListExprKind {
    Value(ListElements),
    Spread {
        elements: ListElements,
        tail: Box<ListExpr>,
    },
    LocalGet {
        local: ListLocal,
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
    ListIndex {
        list: Box<ListExpr>,
        index: usize,
    },
    DropFirst {
        list: Box<ListExpr>,
        count: usize,
    },
    Panic(PanicExpr),
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

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum ListElements {
    Int(Vec<IntExpr>),
    String(Vec<StringExpr>),
    Float(Vec<FloatExpr>),
    Bool(Vec<BoolExpr>),
    Nil(Vec<NilExpr>),
    Tuple {
        item_type: Vec<ValueType>,
        values: Vec<TupleExpr>,
    },
    List {
        item_type: Box<ValueType>,
        values: Vec<ListExpr>,
    },
    Function {
        item_type: FunctionType,
        values: Vec<FunctionExpr>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ListElementTypeMismatch {
    pub(crate) expected: ValueType,
    pub(crate) actual: ValueType,
}

impl ListExpr {
    pub(crate) fn value(elements: Vec<Expr>, element_type: ValueType) -> Self {
        let elements = ListElements::from_exprs(element_type, elements)
            .expect("list expression elements must match declared item type");
        Self::from_elements(elements)
    }

    pub(crate) fn try_value(
        elements: Vec<Expr>,
        element_type: ValueType,
    ) -> Result<Self, ListElementTypeMismatch> {
        Ok(Self::from_elements(ListElements::from_exprs(
            element_type,
            elements,
        )?))
    }

    pub(crate) fn from_elements(elements: ListElements) -> Self {
        Self {
            element_type: Box::new(elements.item_type()),
            kind: ListExprKind::Value(elements),
        }
    }

    #[cfg(test)]
    pub(crate) fn spread(elements: Vec<Expr>, tail: ListExpr, element_type: ValueType) -> Self {
        let elements = ListElements::from_exprs(element_type, elements)
            .expect("list spread elements must match declared item type");
        Self::try_spread(elements, tail).expect("list spread tail must match prefix item type")
    }

    #[cfg(test)]
    pub(crate) fn try_spread(
        elements: ListElements,
        tail: ListExpr,
    ) -> Result<Self, ListElementTypeMismatch> {
        let expected = elements.item_type();
        let actual = tail.element_type().clone();
        if expected != actual {
            return Err(ListElementTypeMismatch { expected, actual });
        }

        Ok(Self::from_spread_elements(elements, tail))
    }

    pub(crate) fn from_spread_elements(elements: ListElements, tail: ListExpr) -> Self {
        Self {
            element_type: Box::new(elements.item_type()),
            kind: ListExprKind::Spread {
                elements,
                tail: Box::new(tail),
            },
        }
    }

    pub(crate) fn local_get(local: ListLocal, name: EcoString) -> Self {
        Self {
            element_type: Box::new(local.item_type()),
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

    pub(crate) fn list_index(list: ListExpr, index: usize, element_type: ValueType) -> Self {
        Self {
            element_type: Box::new(element_type),
            kind: ListExprKind::ListIndex {
                list: Box::new(list),
                index,
            },
        }
    }

    pub(crate) fn drop_first(list: ListExpr, count: usize) -> Self {
        Self {
            element_type: list.element_type.clone(),
            kind: ListExprKind::DropFirst {
                list: Box::new(list),
                count,
            },
        }
    }

    pub(crate) fn panic(panic: PanicExpr, element_type: ValueType) -> Self {
        Self {
            element_type: Box::new(element_type),
            kind: ListExprKind::Panic(panic),
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

impl ListElements {
    pub(crate) fn from_exprs(
        item_type: ValueType,
        values: Vec<Expr>,
    ) -> Result<Self, ListElementTypeMismatch> {
        match item_type {
            ValueType::Int => values
                .into_iter()
                .map(|value| match value {
                    Expr {
                        kind: super::ExprKind::Int(value),
                    } => Ok(value),
                    value => Err(ListElementTypeMismatch {
                        expected: ValueType::Int,
                        actual: value.value_type(),
                    }),
                })
                .collect::<Result<Vec<_>, _>>()
                .map(Self::Int),
            ValueType::String => values
                .into_iter()
                .map(|value| match value {
                    Expr {
                        kind: super::ExprKind::String(value),
                    } => Ok(value),
                    value => Err(ListElementTypeMismatch {
                        expected: ValueType::String,
                        actual: value.value_type(),
                    }),
                })
                .collect::<Result<Vec<_>, _>>()
                .map(Self::String),
            ValueType::Float => values
                .into_iter()
                .map(|value| match value {
                    Expr {
                        kind: super::ExprKind::Float(value),
                    } => Ok(value),
                    value => Err(ListElementTypeMismatch {
                        expected: ValueType::Float,
                        actual: value.value_type(),
                    }),
                })
                .collect::<Result<Vec<_>, _>>()
                .map(Self::Float),
            ValueType::Bool => values
                .into_iter()
                .map(|value| match value {
                    Expr {
                        kind: super::ExprKind::Bool(value),
                    } => Ok(value),
                    value => Err(ListElementTypeMismatch {
                        expected: ValueType::Bool,
                        actual: value.value_type(),
                    }),
                })
                .collect::<Result<Vec<_>, _>>()
                .map(Self::Bool),
            ValueType::Nil => values
                .into_iter()
                .map(|value| match value {
                    Expr {
                        kind: super::ExprKind::Nil(value),
                    } => Ok(value),
                    value => Err(ListElementTypeMismatch {
                        expected: ValueType::Nil,
                        actual: value.value_type(),
                    }),
                })
                .collect::<Result<Vec<_>, _>>()
                .map(Self::Nil),
            ValueType::Tuple(item_type) => values
                .into_iter()
                .map(|value| match value {
                    Expr {
                        kind: super::ExprKind::Tuple(value),
                    } if value.type_() == item_type.as_slice() => Ok(value),
                    value => Err(ListElementTypeMismatch {
                        expected: ValueType::Tuple(item_type.clone()),
                        actual: value.value_type(),
                    }),
                })
                .collect::<Result<Vec<_>, _>>()
                .map(|values| Self::Tuple { item_type, values }),
            ValueType::List(item_type) => values
                .into_iter()
                .map(|value| match value {
                    Expr {
                        kind: super::ExprKind::List(value),
                    } if value.element_type() == item_type.as_ref() => Ok(value),
                    value => Err(ListElementTypeMismatch {
                        expected: ValueType::List(item_type.clone()),
                        actual: value.value_type(),
                    }),
                })
                .collect::<Result<Vec<_>, _>>()
                .map(|values| Self::List { item_type, values }),
            ValueType::Function(item_type) => values
                .into_iter()
                .map(|value| match value {
                    Expr {
                        kind: super::ExprKind::Function(value),
                    } if value.type_() == item_type.as_ref() => Ok(value),
                    value => Err(ListElementTypeMismatch {
                        expected: ValueType::Function(item_type.clone()),
                        actual: value.value_type(),
                    }),
                })
                .collect::<Result<Vec<_>, _>>()
                .map(|values| Self::Function {
                    item_type: *item_type,
                    values,
                }),
        }
    }

    pub(crate) fn item_type(&self) -> ValueType {
        match self {
            Self::Int(_) => ValueType::Int,
            Self::String(_) => ValueType::String,
            Self::Float(_) => ValueType::Float,
            Self::Bool(_) => ValueType::Bool,
            Self::Nil(_) => ValueType::Nil,
            Self::Tuple { item_type, .. } => ValueType::Tuple(item_type.clone()),
            Self::List { item_type, .. } => ValueType::List(item_type.clone()),
            Self::Function { item_type, .. } => ValueType::Function(Box::new(item_type.clone())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{ListElementTypeMismatch, ListElements, ListExpr, ListExprKind};
    use crate::plan::{
        BoolExpr, Expr, FunctionExpr, FunctionType, IntExpr, IntListLocalId, ListFunctionExpr,
        ListFunctionId, ListFunctionValue, ListLocal, NilExpr, ParamLocal, Step, ValueType,
    };

    #[test]
    fn list_expr_kind_accessors() {
        assert_eq!(
            list_value().kind(),
            &ListExprKind::Value(ListElements::Int(vec![IntExpr::value(1.into())])),
        );
        assert_eq!(
            ListExpr::spread(
                vec![Expr::int(IntExpr::value(0.into()))],
                list_value(),
                element_type()
            )
            .kind(),
            &ListExprKind::Spread {
                elements: ListElements::Int(vec![IntExpr::value(0.into())]),
                tail: Box::new(list_value()),
            },
        );
        assert_eq!(
            ListExpr::local_get(ListLocal::int(IntListLocalId(0)), "values".into()).kind(),
            &ListExprKind::LocalGet {
                local: ListLocal::int(IntListLocalId(0)),
                name: "values".into(),
            },
        );
        assert_eq!(
            ListExpr::call(ListFunctionId(0), Vec::new(), element_type()).kind(),
            &ListExprKind::Call {
                function: ListFunctionId(0),
                args: Vec::new(),
            },
        );
        assert_eq!(
            ListExpr::function_call(list_function_expr(), Vec::new(), element_type()).kind(),
            &ListExprKind::FunctionCall {
                function: Box::new(list_function_expr()),
                args: Vec::new(),
            },
        );
        assert_eq!(
            ListExpr::tuple_index(tuple_expr(), 0, element_type()).kind(),
            &ListExprKind::TupleIndex {
                tuple: Box::new(tuple_expr()),
                index: 0,
            },
        );
        assert_eq!(
            ListExpr::bool_case(BoolExpr::value(true), list_value(), list_value()).kind(),
            &ListExprKind::BoolCase {
                subject: Box::new(BoolExpr::value(true)),
                true_: Box::new(list_value()),
                false_: Box::new(list_value()),
            },
        );
        assert_eq!(
            ListExpr::int_case(
                IntExpr::value(1.into()),
                vec![(1.into(), list_value())],
                list_value(),
            )
            .kind(),
            &ListExprKind::IntCase {
                subject: Box::new(IntExpr::value(1.into())),
                clauses: vec![(1.into(), list_value())],
                fallback: Box::new(list_value()),
            },
        );
        assert_eq!(
            ListExpr::string_case(
                crate::plan::StringExpr::value("one".into()),
                vec![("one".into(), list_value())],
                list_value(),
            )
            .kind(),
            &ListExprKind::StringCase {
                subject: Box::new(crate::plan::StringExpr::value("one".into())),
                clauses: vec![("one".into(), list_value())],
                fallback: Box::new(list_value()),
            },
        );
        assert_eq!(
            ListExpr::float_case(
                crate::plan::FloatExpr::value(1.0),
                vec![(1.0, list_value())],
                list_value(),
            )
            .kind(),
            &ListExprKind::FloatCase {
                subject: Box::new(crate::plan::FloatExpr::value(1.0)),
                clauses: vec![(1.0, list_value())],
                fallback: Box::new(list_value()),
            },
        );
        assert_eq!(
            ListExpr::block(
                vec![Step::evaluate(Expr::int(IntExpr::value(1.into())))],
                list_value(),
            )
            .kind(),
            &ListExprKind::Block {
                steps: vec![Step::evaluate(Expr::int(IntExpr::value(1.into())))],
                return_: Box::new(list_value()),
            },
        );
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

    #[test]
    fn list_expr_try_constructors_reject_wrong_item_family() {
        assert_eq!(
            ListExpr::try_value(
                vec![Expr::string(crate::plan::StringExpr::value("one".into()))],
                ValueType::Int,
            ),
            Err(ListElementTypeMismatch {
                expected: ValueType::Int,
                actual: ValueType::String,
            }),
        );
        assert_eq!(
            ListExpr::try_spread(
                ListElements::Int(vec![IntExpr::value(1.into())]),
                ListExpr::value(
                    vec![Expr::string(crate::plan::StringExpr::value("two".into()))],
                    ValueType::String,
                ),
            ),
            Err(ListElementTypeMismatch {
                expected: ValueType::Int,
                actual: ValueType::String,
            }),
        );
    }

    #[test]
    fn list_elements_reject_wrong_item_family() {
        assert_eq!(
            ListElements::from_exprs(
                ValueType::Int,
                vec![Expr::string(crate::plan::StringExpr::value("one".into()))],
            ),
            Err(ListElementTypeMismatch {
                expected: ValueType::Int,
                actual: ValueType::String,
            }),
        );
        assert_eq!(
            ListElements::from_exprs(ValueType::String, vec![Expr::int(IntExpr::value(1.into()))],),
            Err(ListElementTypeMismatch {
                expected: ValueType::String,
                actual: ValueType::Int,
            }),
        );
        assert_eq!(
            ListElements::from_exprs(ValueType::Float, vec![Expr::int(IntExpr::value(1.into()))],),
            Err(ListElementTypeMismatch {
                expected: ValueType::Float,
                actual: ValueType::Int,
            }),
        );
        assert_eq!(
            ListElements::from_exprs(ValueType::Bool, vec![Expr::int(IntExpr::value(1.into()))],),
            Err(ListElementTypeMismatch {
                expected: ValueType::Bool,
                actual: ValueType::Int,
            }),
        );
        assert_eq!(
            ListElements::from_exprs(ValueType::Nil, vec![Expr::int(IntExpr::value(1.into()))],),
            Err(ListElementTypeMismatch {
                expected: ValueType::Nil,
                actual: ValueType::Int,
            }),
        );
        assert_eq!(
            ListElements::from_exprs(
                ValueType::Tuple(vec![ValueType::Int]),
                vec![Expr::int(IntExpr::value(1.into()))],
            ),
            Err(ListElementTypeMismatch {
                expected: ValueType::Tuple(vec![ValueType::Int]),
                actual: ValueType::Int,
            }),
        );
        assert_eq!(
            ListElements::from_exprs(
                ValueType::List(Box::new(ValueType::Int)),
                vec![Expr::int(IntExpr::value(1.into()))],
            ),
            Err(ListElementTypeMismatch {
                expected: ValueType::List(Box::new(ValueType::Int)),
                actual: ValueType::Int,
            }),
        );
        assert_eq!(
            ListElements::from_exprs(
                ValueType::Function(Box::new(list_function_expr().type_().clone())),
                vec![Expr::int(IntExpr::value(1.into()))],
            ),
            Err(ListElementTypeMismatch {
                expected: ValueType::Function(Box::new(list_function_expr().type_().clone())),
                actual: ValueType::Int,
            }),
        );
    }

    #[test]
    fn list_elements_reject_nested_item_metadata_mismatch() {
        assert_eq!(
            ListElements::from_exprs(
                ValueType::Tuple(vec![ValueType::String]),
                vec![Expr::tuple(tuple_expr())],
            ),
            Err(ListElementTypeMismatch {
                expected: ValueType::Tuple(vec![ValueType::String]),
                actual: ValueType::Tuple(vec![ValueType::List(Box::new(element_type()))]),
            }),
        );
        assert_eq!(
            ListElements::from_exprs(
                ValueType::List(Box::new(ValueType::String)),
                vec![Expr::list(list_value())],
            ),
            Err(ListElementTypeMismatch {
                expected: ValueType::List(Box::new(ValueType::String)),
                actual: ValueType::List(Box::new(element_type())),
            }),
        );
        assert_eq!(
            ListElements::from_exprs(
                ValueType::Function(Box::new(FunctionType::new(
                    Vec::new(),
                    ValueType::List(Box::new(element_type())),
                ))),
                vec![Expr::function(FunctionExpr::list(list_function_expr()))],
            ),
            Err(ListElementTypeMismatch {
                expected: ValueType::Function(Box::new(FunctionType::new(
                    Vec::new(),
                    ValueType::List(Box::new(element_type())),
                ))),
                actual: ValueType::Function(Box::new(list_function_expr().type_().clone())),
            }),
        );
    }

    #[test]
    fn list_elements_preserve_family_specific_storage() {
        assert_eq!(
            ListElements::from_exprs(
                ValueType::String,
                vec![Expr::string(crate::plan::StringExpr::value("one".into()))],
            ),
            Ok(ListElements::String(vec![crate::plan::StringExpr::value(
                "one".into()
            )])),
        );
        assert_eq!(
            ListElements::from_exprs(
                ValueType::Float,
                vec![Expr::float(crate::plan::FloatExpr::value(1.5))],
            ),
            Ok(ListElements::Float(vec![crate::plan::FloatExpr::value(
                1.5
            )])),
        );
        assert_eq!(
            ListElements::from_exprs(ValueType::Bool, vec![Expr::bool(BoolExpr::value(true))],),
            Ok(ListElements::Bool(vec![BoolExpr::value(true)])),
        );
        assert_eq!(
            ListElements::from_exprs(ValueType::Nil, vec![Expr::nil(NilExpr::value())],),
            Ok(ListElements::Nil(vec![NilExpr::value()])),
        );
        assert_eq!(
            ListElements::from_exprs(
                ValueType::Tuple(vec![ValueType::List(Box::new(element_type()))]),
                vec![Expr::tuple(tuple_expr())],
            ),
            Ok(ListElements::Tuple {
                item_type: vec![ValueType::List(Box::new(element_type()))],
                values: vec![tuple_expr()],
            }),
        );
        assert_eq!(
            ListElements::from_exprs(
                ValueType::List(Box::new(element_type())),
                vec![Expr::list(list_value())],
            ),
            Ok(ListElements::List {
                item_type: Box::new(element_type()),
                values: vec![list_value()],
            }),
        );
        assert_eq!(
            ListElements::from_exprs(
                ValueType::Function(Box::new(list_function_expr().type_().clone())),
                vec![Expr::function(FunctionExpr::list(list_function_expr()))],
            ),
            Ok(ListElements::Function {
                item_type: list_function_expr().type_().clone(),
                values: vec![FunctionExpr::list(list_function_expr())],
            }),
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
