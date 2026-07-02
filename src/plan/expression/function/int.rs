use crate::plan::{
    BoolExpr, CaptureArg, FunctionFunctionExpr, FunctionType, IntExpr, IntFunctionFunctionId,
    IntFunctionId, IntFunctionLocalId, IntFunctionValue, ParamLocal, Step, StringExpr,
};
use ecow::EcoString;
use num_bigint::BigInt;

#[derive(Debug, Clone, PartialEq)]
pub struct IntFunctionExpr {
    type_: FunctionType,
    kind: IntFunctionExprKind,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum IntFunctionExprKind {
    Value(IntFunctionValue),
    Closure {
        runtime_id: IntFunctionId,
        params: Vec<ParamLocal>,
        captures: Vec<CaptureArg>,
    },
    LocalGet {
        local: IntFunctionLocalId,
        name: EcoString,
    },
    Call {
        function: IntFunctionFunctionId,
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
        true_: Box<IntFunctionExpr>,
        false_: Box<IntFunctionExpr>,
    },
    IntCase {
        subject: Box<IntExpr>,
        clauses: Vec<(BigInt, IntFunctionExpr)>,
        fallback: Box<IntFunctionExpr>,
    },
    StringCase {
        subject: Box<StringExpr>,
        clauses: Vec<(EcoString, IntFunctionExpr)>,
        fallback: Box<IntFunctionExpr>,
    },
    Block {
        steps: Vec<Step>,
        return_: Box<IntFunctionExpr>,
    },
}

impl IntFunctionExpr {
    pub(crate) fn value(value: IntFunctionValue) -> Self {
        Self {
            type_: value.type_(),
            kind: IntFunctionExprKind::Value(value),
        }
    }

    pub(crate) fn closure(
        runtime_id: IntFunctionId,
        params: Vec<ParamLocal>,
        captures: Vec<CaptureArg>,
        type_: FunctionType,
    ) -> Self {
        Self {
            type_,
            kind: IntFunctionExprKind::Closure {
                runtime_id,
                params,
                captures,
            },
        }
    }

    pub(crate) fn local_get(
        local: IntFunctionLocalId,
        name: EcoString,
        type_: FunctionType,
    ) -> Self {
        Self {
            type_,
            kind: IntFunctionExprKind::LocalGet { local, name },
        }
    }

    pub(crate) fn call(
        function: IntFunctionFunctionId,
        args: Vec<crate::plan::CallArg>,
        type_: FunctionType,
    ) -> Self {
        Self {
            type_: type_.clone(),
            kind: IntFunctionExprKind::Call {
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
            kind: IntFunctionExprKind::FunctionCall {
                function: Box::new(function),
                args,
                type_,
            },
        }
    }

    pub(crate) fn bool_case(
        subject: BoolExpr,
        true_: IntFunctionExpr,
        false_: IntFunctionExpr,
    ) -> Self {
        Self {
            type_: true_.type_.clone(),
            kind: IntFunctionExprKind::BoolCase {
                subject: Box::new(subject),
                true_: Box::new(true_),
                false_: Box::new(false_),
            },
        }
    }

    pub(crate) fn int_case(
        subject: IntExpr,
        clauses: Vec<(BigInt, IntFunctionExpr)>,
        fallback: IntFunctionExpr,
    ) -> Self {
        Self {
            type_: fallback.type_.clone(),
            kind: IntFunctionExprKind::IntCase {
                subject: Box::new(subject),
                clauses,
                fallback: Box::new(fallback),
            },
        }
    }

    pub(crate) fn string_case(
        subject: StringExpr,
        clauses: Vec<(EcoString, IntFunctionExpr)>,
        fallback: IntFunctionExpr,
    ) -> Self {
        Self {
            type_: fallback.type_.clone(),
            kind: IntFunctionExprKind::StringCase {
                subject: Box::new(subject),
                clauses,
                fallback: Box::new(fallback),
            },
        }
    }

    pub(crate) fn block(steps: Vec<Step>, return_: IntFunctionExpr) -> Self {
        Self {
            type_: return_.type_.clone(),
            kind: IntFunctionExprKind::Block {
                steps,
                return_: Box::new(return_),
            },
        }
    }

    pub fn type_(&self) -> &FunctionType {
        &self.type_
    }

    pub(crate) fn kind(&self) -> &IntFunctionExprKind {
        &self.kind
    }
}

#[cfg(test)]
mod tests {
    use super::{IntFunctionExpr, IntFunctionExprKind};
    use crate::plan::{
        BoolExpr, Expr, FunctionFunctionExpr, FunctionFunctionId, FunctionFunctionValue,
        FunctionType, IntExpr, IntFunctionFunctionId, IntFunctionId, IntFunctionLocalId,
        IntFunctionValue, IntLocalId, ParamLocal, Step, ValueType,
    };

    #[test]
    fn int_function_expr_kind_accessors() {
        assert_eq!(
            int_function_type(),
            FunctionType::new(vec![ValueType::Int], ValueType::Int),
        );
        assert!(matches!(
            IntFunctionExpr::local_get(IntFunctionLocalId(0), "f".into(), int_function_type(),)
                .kind(),
            IntFunctionExprKind::LocalGet { .. }
        ));
        assert!(matches!(
            IntFunctionExpr::call(IntFunctionFunctionId(0), Vec::new(), int_function_type()).kind(),
            IntFunctionExprKind::Call { .. }
        ));
        assert!(matches!(
            IntFunctionExpr::function_call(
                function_function_value(),
                Vec::new(),
                int_function_type(),
            )
            .kind(),
            IntFunctionExprKind::FunctionCall { .. }
        ));
        assert!(matches!(
            IntFunctionExpr::int_case(
                IntExpr::value(1.into()),
                vec![(1.into(), int_function_value())],
                int_function_value(),
            )
            .kind(),
            IntFunctionExprKind::IntCase { .. }
        ));
        assert!(matches!(
            IntFunctionExpr::block(
                vec![Step::evaluate(Expr::int(IntExpr::value(1.into())))],
                int_function_value(),
            )
            .kind(),
            IntFunctionExprKind::Block { .. }
        ));
    }

    #[test]
    fn int_function_expr_type() {
        assert_eq!(int_function_value().type_(), &int_function_type());
        assert_eq!(
            IntFunctionExpr::bool_case(
                BoolExpr::value(true),
                int_function_value(),
                int_function_value(),
            )
            .type_(),
            &int_function_type(),
        );
        assert_eq!(
            IntFunctionExpr::int_case(
                IntExpr::value(1.into()),
                vec![(1.into(), int_function_value())],
                int_function_value(),
            )
            .type_(),
            &int_function_type(),
        );
        assert_eq!(
            IntFunctionExpr::block(Vec::new(), int_function_value()).type_(),
            &int_function_type(),
        );
    }

    fn int_function_value() -> IntFunctionExpr {
        IntFunctionExpr::value(IntFunctionValue::new(
            IntFunctionId(0),
            vec![ParamLocal::int(IntLocalId(0))],
        ))
    }

    fn int_function_type() -> FunctionType {
        FunctionType::new(vec![ValueType::Int], ValueType::Int)
    }

    fn function_function_value() -> FunctionFunctionExpr {
        FunctionFunctionExpr::value(FunctionFunctionValue::new(
            FunctionFunctionId::Int(IntFunctionFunctionId(0)),
            Vec::new(),
            int_function_type(),
        ))
    }
}
