use crate::plan::CustomFieldAccess;
use crate::plan::{
    BoolExpr, CaptureArg, FloatExpr, FloatFunctionFunctionId, FloatFunctionId,
    FloatFunctionLocalId, FloatFunctionReference, FunctionFunctionExpr, FunctionListExpr,
    FunctionType, IntExpr, PanicExpr, ParamLocal, Step, StringExpr, TupleExpr,
};
use ecow::EcoString;
use num_bigint::BigInt;

#[derive(Debug, Clone, PartialEq)]
pub struct FloatFunctionExpr {
    type_: FunctionType,
    kind: FloatFunctionExprKind,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum FloatFunctionExprKind {
    Reference(FloatFunctionReference),
    Closure {
        runtime_id: FloatFunctionId,
        params: Vec<ParamLocal>,
        captures: Vec<CaptureArg>,
    },
    LocalGet {
        local: FloatFunctionLocalId,
        name: EcoString,
    },
    Call {
        function: FloatFunctionFunctionId,
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
    CustomField(CustomFieldAccess),
    ListIndex {
        list: Box<FunctionListExpr>,
        index: usize,
        type_: FunctionType,
    },
    Panic(PanicExpr),
    BoolCase {
        subject: Box<BoolExpr>,
        true_: Box<FloatFunctionExpr>,
        false_: Box<FloatFunctionExpr>,
    },
    IntCase {
        subject: Box<IntExpr>,
        clauses: Vec<(BigInt, FloatFunctionExpr)>,
        fallback: Box<FloatFunctionExpr>,
    },
    StringCase {
        subject: Box<StringExpr>,
        clauses: Vec<(EcoString, FloatFunctionExpr)>,
        fallback: Box<FloatFunctionExpr>,
    },
    FloatCase {
        subject: Box<FloatExpr>,
        clauses: Vec<(f64, FloatFunctionExpr)>,
        fallback: Box<FloatFunctionExpr>,
    },
    Block {
        steps: Vec<Step>,
        return_: Box<FloatFunctionExpr>,
    },
}

impl FloatFunctionExpr {
    pub(crate) fn reference(value: FloatFunctionReference) -> Self {
        let type_ = FunctionType::new(
            value.params().iter().map(ParamLocal::value_type).collect(),
            crate::plan::ValueType::Float,
        );
        Self {
            type_,
            kind: FloatFunctionExprKind::Reference(value),
        }
    }

    pub(crate) fn closure(
        runtime_id: FloatFunctionId,
        params: Vec<ParamLocal>,
        captures: Vec<CaptureArg>,
        type_: FunctionType,
    ) -> Self {
        Self {
            type_,
            kind: FloatFunctionExprKind::Closure {
                runtime_id,
                params,
                captures,
            },
        }
    }

    pub(crate) fn local_get(
        local: FloatFunctionLocalId,
        name: EcoString,
        type_: FunctionType,
    ) -> Self {
        Self {
            type_,
            kind: FloatFunctionExprKind::LocalGet { local, name },
        }
    }

    pub(crate) fn call(
        function: FloatFunctionFunctionId,
        args: Vec<crate::plan::CallArg>,
        type_: FunctionType,
    ) -> Self {
        Self {
            type_: type_.clone(),
            kind: FloatFunctionExprKind::Call {
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
    ) -> Self {
        Self {
            type_: type_.clone(),
            kind: FloatFunctionExprKind::FunctionCall {
                function: Box::new(function),
                args,
                type_,
            },
        }
    }

    pub(crate) fn tuple_index(tuple: TupleExpr, index: usize, type_: FunctionType) -> Self {
        Self {
            type_: type_.clone(),
            kind: FloatFunctionExprKind::TupleIndex {
                tuple: Box::new(tuple),
                index,
                type_,
            },
        }
    }

    pub(crate) fn custom_field(access: CustomFieldAccess, type_: FunctionType) -> Self {
        Self {
            type_,
            kind: FloatFunctionExprKind::CustomField(access),
        }
    }

    pub(crate) fn list_index(
        list: impl Into<FunctionListExpr>,
        index: usize,
        type_: FunctionType,
    ) -> Self {
        Self {
            type_: type_.clone(),
            kind: FloatFunctionExprKind::ListIndex {
                list: Box::new(list.into()),
                index,
                type_,
            },
        }
    }

    pub(crate) fn panic(panic: PanicExpr, type_: FunctionType) -> Self {
        Self {
            type_,
            kind: FloatFunctionExprKind::Panic(panic),
        }
    }

    pub(crate) fn bool_case(
        subject: BoolExpr,
        true_: FloatFunctionExpr,
        false_: FloatFunctionExpr,
    ) -> Self {
        Self {
            type_: true_.type_.clone(),
            kind: FloatFunctionExprKind::BoolCase {
                subject: Box::new(subject),
                true_: Box::new(true_),
                false_: Box::new(false_),
            },
        }
    }

    pub(crate) fn int_case(
        subject: IntExpr,
        clauses: Vec<(BigInt, FloatFunctionExpr)>,
        fallback: FloatFunctionExpr,
    ) -> Self {
        Self {
            type_: fallback.type_.clone(),
            kind: FloatFunctionExprKind::IntCase {
                subject: Box::new(subject),
                clauses,
                fallback: Box::new(fallback),
            },
        }
    }

    pub(crate) fn string_case(
        subject: StringExpr,
        clauses: Vec<(EcoString, FloatFunctionExpr)>,
        fallback: FloatFunctionExpr,
    ) -> Self {
        Self {
            type_: fallback.type_.clone(),
            kind: FloatFunctionExprKind::StringCase {
                subject: Box::new(subject),
                clauses,
                fallback: Box::new(fallback),
            },
        }
    }

    pub(crate) fn float_case(
        subject: FloatExpr,
        clauses: Vec<(f64, FloatFunctionExpr)>,
        fallback: FloatFunctionExpr,
    ) -> Self {
        Self {
            type_: fallback.type_.clone(),
            kind: FloatFunctionExprKind::FloatCase {
                subject: Box::new(subject),
                clauses,
                fallback: Box::new(fallback),
            },
        }
    }

    pub(crate) fn block(steps: Vec<Step>, return_: FloatFunctionExpr) -> Self {
        Self {
            type_: return_.type_.clone(),
            kind: FloatFunctionExprKind::Block {
                steps,
                return_: Box::new(return_),
            },
        }
    }

    pub fn type_(&self) -> &FunctionType {
        &self.type_
    }

    pub(crate) fn kind(&self) -> &FloatFunctionExprKind {
        &self.kind
    }

    pub(crate) fn into_parts(self) -> (FunctionType, FloatFunctionExprKind) {
        (self.type_, self.kind)
    }
}

#[cfg(test)]
mod tests {
    use super::{FloatFunctionExpr, FloatFunctionExprKind};
    use crate::plan::{
        BoolExpr, Expr, FloatExpr, FloatFunctionFunctionId, FloatFunctionId, FloatFunctionLocalId,
        FloatFunctionReference, FloatLocalId, FunctionFunctionExpr, FunctionFunctionId,
        FunctionFunctionReference, FunctionType, IntExpr, ParamLocal, Step, StringExpr, ValueType,
    };

    #[test]
    fn float_function_expr_kind_accessors() {
        assert_eq!(
            float_function_type(),
            FunctionType::new(vec![ValueType::Float], ValueType::Float),
        );
        assert_eq!(
            float_function_value().kind(),
            &FloatFunctionExprKind::Reference(FloatFunctionReference::new(
                FloatFunctionId(0),
                vec![ParamLocal::float(FloatLocalId(0))],
            )),
        );
        assert_eq!(
            FloatFunctionExpr::closure(
                FloatFunctionId(0),
                vec![ParamLocal::float(FloatLocalId(0))],
                Vec::new(),
                float_function_type(),
            )
            .kind(),
            &FloatFunctionExprKind::Closure {
                runtime_id: FloatFunctionId(0),
                params: vec![ParamLocal::float(FloatLocalId(0))],
                captures: Vec::new(),
            },
        );
        assert_eq!(
            FloatFunctionExpr::local_get(
                FloatFunctionLocalId(0),
                "f".into(),
                float_function_type(),
            )
            .kind(),
            &FloatFunctionExprKind::LocalGet {
                local: FloatFunctionLocalId(0),
                name: "f".into(),
            },
        );
        assert_eq!(
            FloatFunctionExpr::call(
                FloatFunctionFunctionId(0),
                Vec::new(),
                float_function_type()
            )
            .kind(),
            &FloatFunctionExprKind::Call {
                function: FloatFunctionFunctionId(0),
                args: Vec::new(),
                type_: float_function_type(),
            },
        );
        assert_eq!(
            FloatFunctionExpr::function_call(
                function_function_value(),
                Vec::new(),
                float_function_type(),
            )
            .kind(),
            &FloatFunctionExprKind::FunctionCall {
                function: Box::new(function_function_value()),
                args: Vec::new(),
                type_: float_function_type(),
            },
        );
        assert_eq!(
            FloatFunctionExpr::tuple_index(tuple_expr(), 0, float_function_type()).kind(),
            &FloatFunctionExprKind::TupleIndex {
                tuple: Box::new(tuple_expr()),
                index: 0,
                type_: float_function_type(),
            },
        );
        assert_eq!(
            FloatFunctionExpr::bool_case(
                BoolExpr::value(true),
                float_function_value(),
                float_function_value(),
            )
            .kind(),
            &FloatFunctionExprKind::BoolCase {
                subject: Box::new(BoolExpr::value(true)),
                true_: Box::new(float_function_value()),
                false_: Box::new(float_function_value()),
            },
        );
        assert_eq!(
            FloatFunctionExpr::int_case(
                IntExpr::value(1.into()),
                vec![(1.into(), float_function_value())],
                float_function_value(),
            )
            .kind(),
            &FloatFunctionExprKind::IntCase {
                subject: Box::new(IntExpr::value(1.into())),
                clauses: vec![(1.into(), float_function_value())],
                fallback: Box::new(float_function_value()),
            },
        );
        assert_eq!(
            FloatFunctionExpr::string_case(
                StringExpr::value("one".into()),
                vec![("one".into(), float_function_value())],
                float_function_value(),
            )
            .kind(),
            &FloatFunctionExprKind::StringCase {
                subject: Box::new(StringExpr::value("one".into())),
                clauses: vec![("one".into(), float_function_value())],
                fallback: Box::new(float_function_value()),
            },
        );
        assert_eq!(
            FloatFunctionExpr::float_case(
                FloatExpr::value(1.0),
                vec![(1.0, float_function_value())],
                float_function_value(),
            )
            .kind(),
            &FloatFunctionExprKind::FloatCase {
                subject: Box::new(FloatExpr::value(1.0)),
                clauses: vec![(1.0, float_function_value())],
                fallback: Box::new(float_function_value()),
            },
        );
        assert_eq!(
            FloatFunctionExpr::block(
                vec![Step::evaluate(Expr::float(FloatExpr::value(1.0)))],
                float_function_value(),
            )
            .kind(),
            &FloatFunctionExprKind::Block {
                steps: vec![Step::evaluate(Expr::float(FloatExpr::value(1.0)))],
                return_: Box::new(float_function_value()),
            },
        );
    }

    #[test]
    fn float_function_expr_type() {
        assert_eq!(float_function_value().type_(), &float_function_type());
        assert_eq!(
            FloatFunctionExpr::bool_case(
                BoolExpr::value(true),
                float_function_value(),
                float_function_value(),
            )
            .type_(),
            &float_function_type(),
        );
        assert_eq!(
            FloatFunctionExpr::float_case(
                FloatExpr::value(1.0),
                vec![(1.0, float_function_value())],
                float_function_value(),
            )
            .type_(),
            &float_function_type(),
        );
        assert_eq!(
            FloatFunctionExpr::block(Vec::new(), float_function_value()).type_(),
            &float_function_type(),
        );
    }

    fn float_function_value() -> FloatFunctionExpr {
        FloatFunctionExpr::reference(FloatFunctionReference::new(
            FloatFunctionId(0),
            vec![ParamLocal::float(FloatLocalId(0))],
        ))
    }

    fn float_function_type() -> FunctionType {
        FunctionType::new(vec![ValueType::Float], ValueType::Float)
    }

    fn function_function_value() -> FunctionFunctionExpr {
        FunctionFunctionExpr::reference(
            FunctionFunctionReference::new(
                FunctionFunctionId::Float(FloatFunctionFunctionId(0)),
                Vec::new(),
            ),
            float_function_type(),
        )
    }

    fn tuple_expr() -> crate::plan::TupleExpr {
        crate::plan::TupleExpr::value(
            vec![Expr::function(crate::plan::FunctionExpr::float(
                float_function_value(),
            ))],
            vec![ValueType::Function(Box::new(float_function_type()))],
        )
    }
}
