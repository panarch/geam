use crate::plan::{
    BoolExpr, CaptureArg, FloatExpr, FunctionFunctionExpr, FunctionListExpr, FunctionType, IntExpr,
    ListFunctionFunctionId, ListFunctionId, ListFunctionLocal, ListFunctionValue, PanicExpr,
    ParamLocal, Step, StringExpr, TupleExpr, ValueType,
};
use ecow::EcoString;
use num_bigint::BigInt;

#[derive(Debug, Clone, PartialEq)]
pub struct ListFunctionExpr {
    type_: FunctionType,
    item_type: ValueType,
    kind: ListFunctionExprKind,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum ListFunctionExprKind {
    Value(ListFunctionValue),
    Closure {
        runtime_id: ListFunctionId,
        params: Vec<ParamLocal>,
        captures: Vec<CaptureArg>,
    },
    LocalGet {
        local: ListFunctionLocal,
        name: EcoString,
    },
    Call {
        function: ListFunctionFunctionId,
        args: Vec<crate::plan::CallArg>,
        type_: FunctionType,
    },
    FunctionCall {
        function: Box<FunctionFunctionExpr>,
        args: Vec<crate::plan::CallArg>,
        type_: FunctionType,
    },
    TupleIndex {
        tuple: Box<TupleExpr>,
        index: usize,
        type_: FunctionType,
    },
    ListIndex {
        list: Box<FunctionListExpr>,
        index: usize,
        type_: FunctionType,
    },
    Panic(PanicExpr),
    BoolCase {
        subject: Box<BoolExpr>,
        true_: Box<ListFunctionExpr>,
        false_: Box<ListFunctionExpr>,
    },
    IntCase {
        subject: Box<IntExpr>,
        clauses: Vec<(BigInt, ListFunctionExpr)>,
        fallback: Box<ListFunctionExpr>,
    },
    StringCase {
        subject: Box<StringExpr>,
        clauses: Vec<(EcoString, ListFunctionExpr)>,
        fallback: Box<ListFunctionExpr>,
    },
    FloatCase {
        subject: Box<FloatExpr>,
        clauses: Vec<(f64, ListFunctionExpr)>,
        fallback: Box<ListFunctionExpr>,
    },
    Block {
        steps: Vec<Step>,
        return_: Box<ListFunctionExpr>,
    },
}

impl ListFunctionExpr {
    pub(crate) fn value(value: ListFunctionValue) -> Self {
        let item_type = value.runtime_id().item_type();
        Self {
            type_: value.type_(),
            item_type,
            kind: ListFunctionExprKind::Value(value),
        }
    }

    pub(crate) fn closure(
        runtime_id: ListFunctionId,
        params: Vec<ParamLocal>,
        captures: Vec<CaptureArg>,
    ) -> Self {
        let item_type = runtime_id.item_type();
        let type_ =
            FunctionType::from_params(&params, ValueType::List(Box::new(item_type.clone())));
        Self {
            type_,
            item_type,
            kind: ListFunctionExprKind::Closure {
                runtime_id,
                params,
                captures,
            },
        }
    }

    pub(crate) fn local_get(local: ListFunctionLocal, name: EcoString) -> Self {
        Self {
            type_: local.type_().clone(),
            item_type: local.item_type(),
            kind: ListFunctionExprKind::LocalGet { local, name },
        }
    }

    pub(crate) fn call(function: ListFunctionFunctionId, args: Vec<crate::plan::CallArg>) -> Self {
        let type_ = function.type_().clone();
        let item_type = function.item_type();
        Self {
            type_: type_.clone(),
            item_type,
            kind: ListFunctionExprKind::Call {
                function,
                args,
                type_,
            },
        }
    }

    pub(crate) fn function_call(
        function: FunctionFunctionExpr,
        args: Vec<crate::plan::CallArg>,
        type_: FunctionType,
        item_type: ValueType,
    ) -> Self {
        Self {
            type_: type_.clone(),
            item_type,
            kind: ListFunctionExprKind::FunctionCall {
                function: Box::new(function),
                args,
                type_,
            },
        }
    }

    pub(crate) fn tuple_index(
        tuple: TupleExpr,
        index: usize,
        type_: FunctionType,
        item_type: ValueType,
    ) -> Self {
        Self {
            type_: type_.clone(),
            item_type,
            kind: ListFunctionExprKind::TupleIndex {
                tuple: Box::new(tuple),
                index,
                type_,
            },
        }
    }

    pub(crate) fn list_index(
        list: impl Into<FunctionListExpr>,
        index: usize,
        type_: FunctionType,
        item_type: ValueType,
    ) -> Self {
        Self {
            type_: type_.clone(),
            item_type,
            kind: ListFunctionExprKind::ListIndex {
                list: Box::new(list.into()),
                index,
                type_,
            },
        }
    }

    pub(crate) fn panic(panic: PanicExpr, type_: FunctionType, item_type: ValueType) -> Self {
        Self {
            type_,
            item_type,
            kind: ListFunctionExprKind::Panic(panic),
        }
    }

    pub(crate) fn bool_case(
        subject: BoolExpr,
        true_: ListFunctionExpr,
        false_: ListFunctionExpr,
    ) -> Self {
        Self {
            type_: true_.type_.clone(),
            item_type: true_.item_type.clone(),
            kind: ListFunctionExprKind::BoolCase {
                subject: Box::new(subject),
                true_: Box::new(true_),
                false_: Box::new(false_),
            },
        }
    }

    pub(crate) fn int_case(
        subject: IntExpr,
        clauses: Vec<(BigInt, ListFunctionExpr)>,
        fallback: ListFunctionExpr,
    ) -> Self {
        Self {
            type_: fallback.type_.clone(),
            item_type: fallback.item_type.clone(),
            kind: ListFunctionExprKind::IntCase {
                subject: Box::new(subject),
                clauses,
                fallback: Box::new(fallback),
            },
        }
    }

    pub(crate) fn string_case(
        subject: StringExpr,
        clauses: Vec<(EcoString, ListFunctionExpr)>,
        fallback: ListFunctionExpr,
    ) -> Self {
        Self {
            type_: fallback.type_.clone(),
            item_type: fallback.item_type.clone(),
            kind: ListFunctionExprKind::StringCase {
                subject: Box::new(subject),
                clauses,
                fallback: Box::new(fallback),
            },
        }
    }

    pub(crate) fn float_case(
        subject: FloatExpr,
        clauses: Vec<(f64, ListFunctionExpr)>,
        fallback: ListFunctionExpr,
    ) -> Self {
        Self {
            type_: fallback.type_.clone(),
            item_type: fallback.item_type.clone(),
            kind: ListFunctionExprKind::FloatCase {
                subject: Box::new(subject),
                clauses,
                fallback: Box::new(fallback),
            },
        }
    }

    pub(crate) fn block(steps: Vec<Step>, return_: ListFunctionExpr) -> Self {
        Self {
            type_: return_.type_.clone(),
            item_type: return_.item_type.clone(),
            kind: ListFunctionExprKind::Block {
                steps,
                return_: Box::new(return_),
            },
        }
    }

    pub fn type_(&self) -> &FunctionType {
        &self.type_
    }

    pub(crate) fn return_item_type(&self) -> ValueType {
        self.item_type.clone()
    }

    pub(crate) fn kind(&self) -> &ListFunctionExprKind {
        &self.kind
    }
}

#[cfg(test)]
mod tests {
    use super::{ListFunctionExpr, ListFunctionExprKind};
    use crate::plan::{
        BoolExpr, Expr, FunctionFunctionExpr, FunctionFunctionId, FunctionFunctionValue,
        FunctionType, IntExpr, ListExpr, ListFunctionFunctionId, ListFunctionId, ListFunctionValue,
        ParamLocal, Step, TupleExpr, ValueType,
    };

    #[test]
    fn list_function_expr_kind_accessors() {
        assert_eq!(
            list_function_value().kind(),
            &ListFunctionExprKind::Value(ListFunctionValue::new(
                ListFunctionId::from_item_type(0, crate::plan::ValueType::Int),
                vec![ParamLocal::int(crate::plan::IntLocalId(0))]
            )),
        );
        assert_eq!(
            ListFunctionExpr::closure(
                ListFunctionId::from_item_type(0, crate::plan::ValueType::Int),
                vec![ParamLocal::int(crate::plan::IntLocalId(0))],
                Vec::new()
            )
            .kind(),
            &ListFunctionExprKind::Closure {
                runtime_id: ListFunctionId::from_item_type(0, crate::plan::ValueType::Int),
                params: vec![ParamLocal::int(crate::plan::IntLocalId(0))],
                captures: Vec::new(),
            },
        );
        assert_eq!(
            ListFunctionExpr::local_get(
                crate::plan::ListFunctionLocal::from_item_type(
                    0,
                    crate::plan::FunctionType::new(
                        Vec::new(),
                        crate::plan::ValueType::List(Box::new(crate::plan::ValueType::Int))
                    ),
                    crate::plan::ValueType::Int,
                ),
                "f".into()
            )
            .kind(),
            &ListFunctionExprKind::LocalGet {
                local: crate::plan::ListFunctionLocal::from_item_type(
                    0,
                    crate::plan::FunctionType::new(
                        Vec::new(),
                        crate::plan::ValueType::List(Box::new(crate::plan::ValueType::Int))
                    ),
                    crate::plan::ValueType::Int,
                ),
                name: "f".into(),
            },
        );
        assert_eq!(
            ListFunctionExpr::call(
                ListFunctionFunctionId::from_item_type(
                    0,
                    list_function_type(),
                    crate::plan::ValueType::Int
                ),
                Vec::new()
            )
            .kind(),
            &ListFunctionExprKind::Call {
                function: ListFunctionFunctionId::from_item_type(
                    0,
                    list_function_type(),
                    crate::plan::ValueType::Int
                ),
                args: Vec::new(),
                type_: list_function_type(),
            },
        );
        assert_eq!(
            ListFunctionExpr::function_call(
                function_function_value(),
                Vec::new(),
                list_function_type(),
                crate::plan::ValueType::Int,
            )
            .kind(),
            &ListFunctionExprKind::FunctionCall {
                function: Box::new(function_function_value()),
                args: Vec::new(),
                type_: list_function_type(),
            },
        );
        assert_eq!(
            ListFunctionExpr::tuple_index(
                tuple_expr(),
                0,
                list_function_type(),
                crate::plan::ValueType::Int,
            )
            .kind(),
            &ListFunctionExprKind::TupleIndex {
                tuple: Box::new(tuple_expr()),
                index: 0,
                type_: list_function_type(),
            },
        );
        assert_eq!(
            ListFunctionExpr::bool_case(
                BoolExpr::value(true),
                list_function_value(),
                list_function_value(),
            )
            .kind(),
            &ListFunctionExprKind::BoolCase {
                subject: Box::new(BoolExpr::value(true)),
                true_: Box::new(list_function_value()),
                false_: Box::new(list_function_value()),
            },
        );
        assert_eq!(
            ListFunctionExpr::int_case(
                IntExpr::value(1.into()),
                vec![(1.into(), list_function_value())],
                list_function_value(),
            )
            .kind(),
            &ListFunctionExprKind::IntCase {
                subject: Box::new(IntExpr::value(1.into())),
                clauses: vec![(1.into(), list_function_value())],
                fallback: Box::new(list_function_value()),
            },
        );
        assert_eq!(
            ListFunctionExpr::string_case(
                crate::plan::StringExpr::value("one".into()),
                vec![("one".into(), list_function_value())],
                list_function_value(),
            )
            .kind(),
            &ListFunctionExprKind::StringCase {
                subject: Box::new(crate::plan::StringExpr::value("one".into())),
                clauses: vec![("one".into(), list_function_value())],
                fallback: Box::new(list_function_value()),
            },
        );
        assert_eq!(
            ListFunctionExpr::float_case(
                crate::plan::FloatExpr::value(1.0),
                vec![(1.0, list_function_value())],
                list_function_value(),
            )
            .kind(),
            &ListFunctionExprKind::FloatCase {
                subject: Box::new(crate::plan::FloatExpr::value(1.0)),
                clauses: vec![(1.0, list_function_value())],
                fallback: Box::new(list_function_value()),
            },
        );
        assert_eq!(
            ListFunctionExpr::block(
                vec![Step::evaluate(Expr::int(IntExpr::value(1.into())))],
                list_function_value(),
            )
            .kind(),
            &ListFunctionExprKind::Block {
                steps: vec![Step::evaluate(Expr::int(IntExpr::value(1.into())))],
                return_: Box::new(list_function_value()),
            },
        );
    }

    #[test]
    fn list_function_expr_type() {
        assert_eq!(list_function_value().type_(), &list_function_type());
        assert_eq!(
            ListFunctionExpr::closure(
                ListFunctionId::from_item_type(0, crate::plan::ValueType::Int),
                vec![ParamLocal::int(crate::plan::IntLocalId(0))],
                Vec::new()
            )
            .type_(),
            &list_function_type(),
        );
        assert_eq!(
            ListFunctionExpr::bool_case(
                BoolExpr::value(true),
                list_function_value(),
                list_function_value(),
            )
            .type_(),
            &list_function_type(),
        );
        assert_eq!(
            ListFunctionExpr::int_case(
                IntExpr::value(1.into()),
                vec![(1.into(), list_function_value())],
                list_function_value(),
            )
            .type_(),
            &list_function_type(),
        );
        assert_eq!(
            ListFunctionExpr::block(Vec::new(), list_function_value()).type_(),
            &list_function_type(),
        );
    }

    fn list_function_value() -> ListFunctionExpr {
        ListFunctionExpr::value(ListFunctionValue::new(
            ListFunctionId::from_item_type(0, crate::plan::ValueType::Int),
            vec![ParamLocal::int(crate::plan::IntLocalId(0))],
        ))
    }

    fn list_function_type() -> FunctionType {
        FunctionType::new(
            vec![ValueType::Int],
            ValueType::List(Box::new(element_type())),
        )
    }

    fn element_type() -> ValueType {
        ValueType::Int
    }

    fn tuple_expr() -> TupleExpr {
        TupleExpr::value(
            vec![Expr::function(crate::plan::FunctionExpr::list(
                list_function_value(),
            ))],
            vec![ValueType::Function(Box::new(list_function_type()))],
        )
    }

    fn function_function_value() -> FunctionFunctionExpr {
        FunctionFunctionExpr::value(FunctionFunctionValue::new(
            FunctionFunctionId::List(ListFunctionFunctionId::from_item_type(
                0,
                crate::plan::FunctionType::new(
                    Vec::new(),
                    crate::plan::ValueType::List(Box::new(crate::plan::ValueType::Int)),
                ),
                crate::plan::ValueType::Int,
            )),
            Vec::new(),
            list_function_type(),
        ))
    }

    fn list_expr() -> ListExpr {
        ListExpr::value(vec![Expr::int(IntExpr::value(1.into()))], element_type())
    }

    #[test]
    fn list_function_value_type_fixture() {
        assert_eq!(
            ListFunctionExpr::value(ListFunctionValue::new(
                ListFunctionId::from_item_type(0, crate::plan::ValueType::Int),
                Vec::new()
            ))
            .type_(),
            &FunctionType::new(Vec::new(), ValueType::List(Box::new(element_type()))),
        );
        assert_eq!(list_expr().element_type(), element_type(),);
    }
}
