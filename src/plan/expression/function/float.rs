use crate::plan::{
    BoolExpr, CaptureArg, FloatExpr, FloatFunctionFunctionId, FloatFunctionId,
    FloatFunctionLocalId, FloatFunctionValue, FunctionFunctionExpr, FunctionType, IntExpr,
    ParamLocal, Step, StringExpr,
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
    Value(FloatFunctionValue),
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
    pub(crate) fn value(value: FloatFunctionValue) -> Self {
        Self {
            type_: value.type_(),
            kind: FloatFunctionExprKind::Value(value),
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
}

#[cfg(test)]
mod tests {
    use super::{FloatFunctionExpr, FloatFunctionExprKind};
    use crate::plan::{
        BoolExpr, Expr, FloatExpr, FloatFunctionFunctionId, FloatFunctionId, FloatFunctionLocalId,
        FloatFunctionValue, FloatLocalId, FunctionFunctionExpr, FunctionFunctionId,
        FunctionFunctionValue, FunctionType, ParamLocal, Step, ValueType,
    };

    #[test]
    fn float_function_expr_kind_accessors() {
        assert_eq!(
            float_function_type(),
            FunctionType::new(vec![ValueType::Float], ValueType::Float),
        );
        assert!(matches!(
            FloatFunctionExpr::local_get(
                FloatFunctionLocalId(0),
                "f".into(),
                float_function_type(),
            )
            .kind(),
            FloatFunctionExprKind::LocalGet { .. }
        ));
        assert!(matches!(
            FloatFunctionExpr::call(
                FloatFunctionFunctionId(0),
                Vec::new(),
                float_function_type()
            )
            .kind(),
            FloatFunctionExprKind::Call { .. }
        ));
        assert!(matches!(
            FloatFunctionExpr::function_call(
                function_function_value(),
                Vec::new(),
                float_function_type(),
            )
            .kind(),
            FloatFunctionExprKind::FunctionCall { .. }
        ));
        assert!(matches!(
            FloatFunctionExpr::float_case(
                FloatExpr::value(1.0),
                vec![(1.0, float_function_value())],
                float_function_value(),
            )
            .kind(),
            FloatFunctionExprKind::FloatCase { .. }
        ));
        assert!(matches!(
            FloatFunctionExpr::block(
                vec![Step::evaluate(Expr::float(FloatExpr::value(1.0)))],
                float_function_value(),
            )
            .kind(),
            FloatFunctionExprKind::Block { .. }
        ));
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
        FloatFunctionExpr::value(FloatFunctionValue::new(
            FloatFunctionId(0),
            vec![ParamLocal::float(FloatLocalId(0))],
        ))
    }

    fn float_function_type() -> FunctionType {
        FunctionType::new(vec![ValueType::Float], ValueType::Float)
    }

    fn function_function_value() -> FunctionFunctionExpr {
        FunctionFunctionExpr::value(FunctionFunctionValue::new(
            FunctionFunctionId::Float(FloatFunctionFunctionId(0)),
            Vec::new(),
            float_function_type(),
        ))
    }
}
