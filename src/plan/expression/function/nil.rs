use crate::plan::{
    BoolExpr, FunctionFunctionExpr, FunctionType, IntExpr, NilFunctionFunctionId,
    NilFunctionLocalId, NilFunctionValue, Step,
};
use ecow::EcoString;
use num_bigint::BigInt;

#[derive(Debug, Clone, PartialEq)]
pub struct NilFunctionExpr {
    type_: FunctionType,
    kind: NilFunctionExprKind,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum NilFunctionExprKind {
    Value(NilFunctionValue),
    LocalGet {
        local: NilFunctionLocalId,
        name: EcoString,
    },
    Call {
        function: NilFunctionFunctionId,
        args: Vec<crate::plan::CallArg>,
        type_: FunctionType,
    },
    FunctionCall {
        function: Box<FunctionFunctionExpr>,
        args: Vec<crate::plan::FunctionCallArg>,
        type_: FunctionType,
    },
    BoolCase {
        subject: Box<BoolExpr>,
        true_: Box<NilFunctionExpr>,
        false_: Box<NilFunctionExpr>,
    },
    IntCase {
        subject: Box<IntExpr>,
        clauses: Vec<(BigInt, NilFunctionExpr)>,
        fallback: Box<NilFunctionExpr>,
    },
    Block {
        steps: Vec<Step>,
        return_: Box<NilFunctionExpr>,
    },
}

impl NilFunctionExpr {
    pub(crate) fn value(value: NilFunctionValue) -> Self {
        Self {
            type_: value.type_(),
            kind: NilFunctionExprKind::Value(value),
        }
    }

    pub(crate) fn local_get(
        local: NilFunctionLocalId,
        name: EcoString,
        type_: FunctionType,
    ) -> Self {
        Self {
            type_,
            kind: NilFunctionExprKind::LocalGet { local, name },
        }
    }

    pub(crate) fn call(
        function: NilFunctionFunctionId,
        args: Vec<crate::plan::CallArg>,
        type_: FunctionType,
    ) -> Self {
        Self {
            type_: type_.clone(),
            kind: NilFunctionExprKind::Call {
                function,
                args,
                type_,
            },
        }
    }

    pub(crate) fn function_call(
        function: FunctionFunctionExpr,
        args: Vec<crate::plan::FunctionCallArg>,
        type_: FunctionType,
    ) -> Self {
        Self {
            type_: type_.clone(),
            kind: NilFunctionExprKind::FunctionCall {
                function: Box::new(function),
                args,
                type_,
            },
        }
    }

    pub(crate) fn bool_case(
        subject: BoolExpr,
        true_: NilFunctionExpr,
        false_: NilFunctionExpr,
    ) -> Self {
        Self {
            type_: true_.type_.clone(),
            kind: NilFunctionExprKind::BoolCase {
                subject: Box::new(subject),
                true_: Box::new(true_),
                false_: Box::new(false_),
            },
        }
    }

    pub(crate) fn int_case(
        subject: IntExpr,
        clauses: Vec<(BigInt, NilFunctionExpr)>,
        fallback: NilFunctionExpr,
    ) -> Self {
        Self {
            type_: fallback.type_.clone(),
            kind: NilFunctionExprKind::IntCase {
                subject: Box::new(subject),
                clauses,
                fallback: Box::new(fallback),
            },
        }
    }

    pub(crate) fn block(steps: Vec<Step>, return_: NilFunctionExpr) -> Self {
        Self {
            type_: return_.type_.clone(),
            kind: NilFunctionExprKind::Block {
                steps,
                return_: Box::new(return_),
            },
        }
    }

    pub fn type_(&self) -> &FunctionType {
        &self.type_
    }

    pub(crate) fn kind(&self) -> &NilFunctionExprKind {
        &self.kind
    }
}

#[cfg(test)]
mod tests {
    use super::{NilFunctionExpr, NilFunctionExprKind};
    use crate::plan::{
        BoolExpr, Expr, FunctionFunctionExpr, FunctionFunctionId, FunctionFunctionValue,
        FunctionType, IntExpr, NilFunctionFunctionId, NilFunctionId, NilFunctionLocalId,
        NilFunctionValue, NilLocalId, ParamLocal, Step, ValueType,
    };

    #[test]
    fn nil_function_expr_kind_accessors() {
        assert!(matches!(
            NilFunctionExpr::local_get(NilFunctionLocalId(0), "f".into(), function_type()).kind(),
            NilFunctionExprKind::LocalGet { .. },
        ));
        assert!(matches!(
            NilFunctionExpr::call(NilFunctionFunctionId(0), Vec::new(), function_type()).kind(),
            NilFunctionExprKind::Call { .. },
        ));
        assert!(matches!(
            NilFunctionExpr::function_call(function_function_value(), Vec::new(), function_type())
                .kind(),
            NilFunctionExprKind::FunctionCall { .. },
        ));
        assert!(matches!(
            NilFunctionExpr::bool_case(BoolExpr::value(true), function_value(), function_value(),)
                .kind(),
            NilFunctionExprKind::BoolCase { .. },
        ));
        assert!(matches!(
            NilFunctionExpr::int_case(
                IntExpr::value(1.into()),
                vec![(1.into(), function_value())],
                function_value(),
            )
            .kind(),
            NilFunctionExprKind::IntCase { .. },
        ));
        assert!(matches!(
            NilFunctionExpr::block(
                vec![Step::evaluate(Expr::int(IntExpr::value(1.into())))],
                function_value(),
            )
            .kind(),
            NilFunctionExprKind::Block { .. },
        ));
    }

    #[test]
    fn nil_function_expr_type() {
        assert_eq!(function_value().type_(), &function_type());
    }

    fn function_value() -> NilFunctionExpr {
        NilFunctionExpr::value(NilFunctionValue::new(
            NilFunctionId(0),
            vec![ParamLocal::nil(NilLocalId(0))],
        ))
    }

    fn function_type() -> FunctionType {
        FunctionType::new(vec![ValueType::Nil], ValueType::Nil)
    }

    fn function_function_value() -> FunctionFunctionExpr {
        FunctionFunctionExpr::value(FunctionFunctionValue::new(
            FunctionFunctionId::Nil(NilFunctionFunctionId(0)),
            Vec::new(),
            function_type(),
        ))
    }
}
