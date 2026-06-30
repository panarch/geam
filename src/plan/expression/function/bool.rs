use crate::plan::{
    BoolExpr, BoolFunctionFunctionId, BoolFunctionId, BoolFunctionLocalId, BoolFunctionValue,
    CaptureArg, FunctionFunctionExpr, FunctionType, IntExpr, ParamLocal, Step,
};
use ecow::EcoString;
use num_bigint::BigInt;

#[derive(Debug, Clone, PartialEq)]
pub struct BoolFunctionExpr {
    type_: FunctionType,
    kind: BoolFunctionExprKind,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum BoolFunctionExprKind {
    Value(BoolFunctionValue),
    Closure {
        runtime_id: BoolFunctionId,
        params: Vec<ParamLocal>,
        captures: Vec<CaptureArg>,
    },
    LocalGet {
        local: BoolFunctionLocalId,
        name: EcoString,
    },
    Call {
        function: BoolFunctionFunctionId,
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
        true_: Box<BoolFunctionExpr>,
        false_: Box<BoolFunctionExpr>,
    },
    IntCase {
        subject: Box<IntExpr>,
        clauses: Vec<(BigInt, BoolFunctionExpr)>,
        fallback: Box<BoolFunctionExpr>,
    },
    Block {
        steps: Vec<Step>,
        return_: Box<BoolFunctionExpr>,
    },
}

impl BoolFunctionExpr {
    pub(crate) fn value(value: BoolFunctionValue) -> Self {
        Self {
            type_: value.type_(),
            kind: BoolFunctionExprKind::Value(value),
        }
    }

    pub(crate) fn closure(
        runtime_id: BoolFunctionId,
        params: Vec<ParamLocal>,
        captures: Vec<CaptureArg>,
        type_: FunctionType,
    ) -> Self {
        Self {
            type_,
            kind: BoolFunctionExprKind::Closure {
                runtime_id,
                params,
                captures,
            },
        }
    }

    pub(crate) fn local_get(
        local: BoolFunctionLocalId,
        name: EcoString,
        type_: FunctionType,
    ) -> Self {
        Self {
            type_,
            kind: BoolFunctionExprKind::LocalGet { local, name },
        }
    }

    pub(crate) fn call(
        function: BoolFunctionFunctionId,
        args: Vec<crate::plan::CallArg>,
        type_: FunctionType,
    ) -> Self {
        Self {
            type_: type_.clone(),
            kind: BoolFunctionExprKind::Call {
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
            kind: BoolFunctionExprKind::FunctionCall {
                function: Box::new(function),
                args,
                type_,
            },
        }
    }

    pub(crate) fn bool_case(
        subject: BoolExpr,
        true_: BoolFunctionExpr,
        false_: BoolFunctionExpr,
    ) -> Self {
        Self {
            type_: true_.type_.clone(),
            kind: BoolFunctionExprKind::BoolCase {
                subject: Box::new(subject),
                true_: Box::new(true_),
                false_: Box::new(false_),
            },
        }
    }

    pub(crate) fn int_case(
        subject: IntExpr,
        clauses: Vec<(BigInt, BoolFunctionExpr)>,
        fallback: BoolFunctionExpr,
    ) -> Self {
        Self {
            type_: fallback.type_.clone(),
            kind: BoolFunctionExprKind::IntCase {
                subject: Box::new(subject),
                clauses,
                fallback: Box::new(fallback),
            },
        }
    }

    pub(crate) fn block(steps: Vec<Step>, return_: BoolFunctionExpr) -> Self {
        Self {
            type_: return_.type_.clone(),
            kind: BoolFunctionExprKind::Block {
                steps,
                return_: Box::new(return_),
            },
        }
    }

    pub fn type_(&self) -> &FunctionType {
        &self.type_
    }

    pub(crate) fn kind(&self) -> &BoolFunctionExprKind {
        &self.kind
    }
}

#[cfg(test)]
mod tests {
    use super::{BoolFunctionExpr, BoolFunctionExprKind};
    use crate::plan::{
        BoolExpr, BoolFunctionFunctionId, BoolFunctionId, BoolFunctionLocalId, BoolFunctionValue,
        BoolLocalId, Expr, FunctionFunctionExpr, FunctionFunctionId, FunctionFunctionValue,
        FunctionType, IntExpr, ParamLocal, Step, ValueType,
    };

    #[test]
    fn bool_function_expr_kind_accessors() {
        assert!(matches!(
            BoolFunctionExpr::local_get(BoolFunctionLocalId(0), "f".into(), function_type()).kind(),
            BoolFunctionExprKind::LocalGet { .. },
        ));
        assert!(matches!(
            BoolFunctionExpr::call(BoolFunctionFunctionId(0), Vec::new(), function_type()).kind(),
            BoolFunctionExprKind::Call { .. },
        ));
        assert!(matches!(
            BoolFunctionExpr::function_call(function_function_value(), Vec::new(), function_type())
                .kind(),
            BoolFunctionExprKind::FunctionCall { .. },
        ));
        assert!(matches!(
            BoolFunctionExpr::bool_case(BoolExpr::value(true), function_value(), function_value(),)
                .kind(),
            BoolFunctionExprKind::BoolCase { .. },
        ));
        assert!(matches!(
            BoolFunctionExpr::int_case(
                IntExpr::value(1.into()),
                vec![(1.into(), function_value())],
                function_value(),
            )
            .kind(),
            BoolFunctionExprKind::IntCase { .. },
        ));
        assert!(matches!(
            BoolFunctionExpr::block(
                vec![Step::evaluate(Expr::int(IntExpr::value(1.into())))],
                function_value(),
            )
            .kind(),
            BoolFunctionExprKind::Block { .. },
        ));
    }

    #[test]
    fn bool_function_expr_type() {
        assert_eq!(function_value().type_(), &function_type());
    }

    fn function_value() -> BoolFunctionExpr {
        BoolFunctionExpr::value(BoolFunctionValue::new(
            BoolFunctionId(0),
            vec![ParamLocal::bool(BoolLocalId(0))],
        ))
    }

    fn function_type() -> FunctionType {
        FunctionType::new(vec![ValueType::Bool], ValueType::Bool)
    }

    fn function_function_value() -> FunctionFunctionExpr {
        FunctionFunctionExpr::value(FunctionFunctionValue::new(
            FunctionFunctionId::Bool(BoolFunctionFunctionId(0)),
            Vec::new(),
            function_type(),
        ))
    }
}
