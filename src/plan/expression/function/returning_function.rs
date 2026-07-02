use crate::plan::{
    BoolExpr, CaptureArg, FunctionFunctionFunctionId, FunctionFunctionId, FunctionFunctionLocalId,
    FunctionFunctionValue, FunctionType, IntExpr, ParamLocal, Step, StringExpr,
};
use ecow::EcoString;
use num_bigint::BigInt;

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionFunctionExpr {
    type_: FunctionType,
    kind: FunctionFunctionExprKind,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum FunctionFunctionExprKind {
    Value(FunctionFunctionValue),
    Closure {
        runtime_id: FunctionFunctionId,
        params: Vec<ParamLocal>,
        captures: Vec<CaptureArg>,
        return_type: FunctionType,
    },
    LocalGet {
        local: FunctionFunctionLocalId,
        name: EcoString,
        type_: FunctionType,
    },
    Call {
        function: FunctionFunctionFunctionId,
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
        true_: Box<FunctionFunctionExpr>,
        false_: Box<FunctionFunctionExpr>,
    },
    IntCase {
        subject: Box<IntExpr>,
        clauses: Vec<(BigInt, FunctionFunctionExpr)>,
        fallback: Box<FunctionFunctionExpr>,
    },
    StringCase {
        subject: Box<StringExpr>,
        clauses: Vec<(EcoString, FunctionFunctionExpr)>,
        fallback: Box<FunctionFunctionExpr>,
    },
    Block {
        steps: Vec<Step>,
        return_: Box<FunctionFunctionExpr>,
    },
}

impl FunctionFunctionExpr {
    pub(crate) fn value(value: FunctionFunctionValue) -> Self {
        Self {
            type_: value.type_(),
            kind: FunctionFunctionExprKind::Value(value),
        }
    }

    pub(crate) fn closure(
        runtime_id: FunctionFunctionId,
        params: Vec<ParamLocal>,
        captures: Vec<CaptureArg>,
        type_: FunctionType,
        return_type: FunctionType,
    ) -> Self {
        Self {
            type_,
            kind: FunctionFunctionExprKind::Closure {
                runtime_id,
                params,
                captures,
                return_type,
            },
        }
    }

    pub(crate) fn local_get(
        local: FunctionFunctionLocalId,
        name: EcoString,
        type_: FunctionType,
    ) -> Self {
        Self {
            type_: type_.clone(),
            kind: FunctionFunctionExprKind::LocalGet { local, name, type_ },
        }
    }

    pub(crate) fn call(
        function: FunctionFunctionFunctionId,
        args: Vec<crate::plan::CallArg>,
        type_: FunctionType,
    ) -> Self {
        Self {
            type_: type_.clone(),
            kind: FunctionFunctionExprKind::Call {
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
            kind: FunctionFunctionExprKind::FunctionCall {
                function: Box::new(function),
                args,
                type_,
            },
        }
    }

    pub(crate) fn bool_case(
        subject: BoolExpr,
        true_: FunctionFunctionExpr,
        false_: FunctionFunctionExpr,
    ) -> Self {
        Self {
            type_: true_.type_.clone(),
            kind: FunctionFunctionExprKind::BoolCase {
                subject: Box::new(subject),
                true_: Box::new(true_),
                false_: Box::new(false_),
            },
        }
    }

    pub(crate) fn int_case(
        subject: IntExpr,
        clauses: Vec<(BigInt, FunctionFunctionExpr)>,
        fallback: FunctionFunctionExpr,
    ) -> Self {
        Self {
            type_: fallback.type_.clone(),
            kind: FunctionFunctionExprKind::IntCase {
                subject: Box::new(subject),
                clauses,
                fallback: Box::new(fallback),
            },
        }
    }

    pub(crate) fn string_case(
        subject: StringExpr,
        clauses: Vec<(EcoString, FunctionFunctionExpr)>,
        fallback: FunctionFunctionExpr,
    ) -> Self {
        Self {
            type_: fallback.type_.clone(),
            kind: FunctionFunctionExprKind::StringCase {
                subject: Box::new(subject),
                clauses,
                fallback: Box::new(fallback),
            },
        }
    }

    pub(crate) fn block(steps: Vec<Step>, return_: FunctionFunctionExpr) -> Self {
        Self {
            type_: return_.type_.clone(),
            kind: FunctionFunctionExprKind::Block {
                steps,
                return_: Box::new(return_),
            },
        }
    }

    pub fn type_(&self) -> &FunctionType {
        &self.type_
    }

    pub(crate) fn kind(&self) -> &FunctionFunctionExprKind {
        &self.kind
    }
}

#[cfg(test)]
mod tests {
    use super::{FunctionFunctionExpr, FunctionFunctionExprKind};
    use crate::plan::{
        BoolExpr, Expr, FunctionFunctionId, FunctionFunctionLocalId, FunctionFunctionValue,
        FunctionType, IntExpr, IntFunctionFunctionId, ParamLocal, Step, ValueType,
    };

    #[test]
    fn function_function_expr_kind_accessors() {
        assert!(matches!(
            FunctionFunctionExpr::local_get(
                FunctionFunctionLocalId(0),
                "f".into(),
                function_type(),
            )
            .kind(),
            FunctionFunctionExprKind::LocalGet { .. },
        ));
        assert!(matches!(
            FunctionFunctionExpr::call(
                crate::plan::FunctionFunctionFunctionId(0),
                Vec::new(),
                function_type(),
            )
            .kind(),
            FunctionFunctionExprKind::Call { .. },
        ));
        assert!(matches!(
            FunctionFunctionExpr::function_call(function_value(), Vec::new(), function_type())
                .kind(),
            FunctionFunctionExprKind::FunctionCall { .. },
        ));
        assert!(matches!(
            FunctionFunctionExpr::int_case(
                IntExpr::value(1.into()),
                vec![(1.into(), function_value())],
                function_value(),
            )
            .kind(),
            FunctionFunctionExprKind::IntCase { .. },
        ));
        assert!(matches!(
            FunctionFunctionExpr::block(
                vec![Step::evaluate(Expr::int(IntExpr::value(1.into())))],
                function_value(),
            )
            .kind(),
            FunctionFunctionExprKind::Block { .. },
        ));
    }

    #[test]
    fn function_function_expr_type() {
        assert_eq!(function_value().type_(), &function_type());
        assert_eq!(
            FunctionFunctionExpr::bool_case(
                BoolExpr::value(true),
                function_value(),
                function_value(),
            )
            .type_(),
            &function_type(),
        );
    }

    fn function_value() -> FunctionFunctionExpr {
        FunctionFunctionExpr::value(FunctionFunctionValue::new(
            FunctionFunctionId::Int(IntFunctionFunctionId(0)),
            vec![ParamLocal::int(crate::plan::IntLocalId(0))],
            FunctionType::new(vec![ValueType::Int], ValueType::Int),
        ))
    }

    fn function_type() -> FunctionType {
        FunctionType::new(
            vec![ValueType::Int],
            ValueType::Function(Box::new(FunctionType::new(
                vec![ValueType::Int],
                ValueType::Int,
            ))),
        )
    }
}
