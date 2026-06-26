use crate::plan::{
    BoolExpr, FunctionType, IntExpr, Step, StringFunctionLocalId, StringFunctionValue,
};
use ecow::EcoString;
use num_bigint::BigInt;

#[derive(Debug, Clone, PartialEq)]
pub struct StringFunctionExpr {
    type_: FunctionType,
    kind: StringFunctionExprKind,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum StringFunctionExprKind {
    Value(StringFunctionValue),
    LocalGet {
        local: StringFunctionLocalId,
        name: EcoString,
    },
    BoolCase {
        subject: Box<BoolExpr>,
        true_: Box<StringFunctionExpr>,
        false_: Box<StringFunctionExpr>,
    },
    IntCase {
        subject: Box<IntExpr>,
        clauses: Vec<(BigInt, StringFunctionExpr)>,
        fallback: Box<StringFunctionExpr>,
    },
    Block {
        steps: Vec<Step>,
        return_: Box<StringFunctionExpr>,
    },
}

impl StringFunctionExpr {
    pub(crate) fn value(value: StringFunctionValue) -> Self {
        Self {
            type_: value.type_(),
            kind: StringFunctionExprKind::Value(value),
        }
    }

    pub(crate) fn local_get(
        local: StringFunctionLocalId,
        name: EcoString,
        type_: FunctionType,
    ) -> Self {
        Self {
            type_,
            kind: StringFunctionExprKind::LocalGet { local, name },
        }
    }

    pub(crate) fn bool_case(
        subject: BoolExpr,
        true_: StringFunctionExpr,
        false_: StringFunctionExpr,
    ) -> Self {
        Self {
            type_: true_.type_.clone(),
            kind: StringFunctionExprKind::BoolCase {
                subject: Box::new(subject),
                true_: Box::new(true_),
                false_: Box::new(false_),
            },
        }
    }

    pub(crate) fn int_case(
        subject: IntExpr,
        clauses: Vec<(BigInt, StringFunctionExpr)>,
        fallback: StringFunctionExpr,
    ) -> Self {
        Self {
            type_: fallback.type_.clone(),
            kind: StringFunctionExprKind::IntCase {
                subject: Box::new(subject),
                clauses,
                fallback: Box::new(fallback),
            },
        }
    }

    pub(crate) fn block(steps: Vec<Step>, return_: StringFunctionExpr) -> Self {
        Self {
            type_: return_.type_.clone(),
            kind: StringFunctionExprKind::Block {
                steps,
                return_: Box::new(return_),
            },
        }
    }

    pub fn type_(&self) -> &FunctionType {
        &self.type_
    }

    pub(crate) fn kind(&self) -> &StringFunctionExprKind {
        &self.kind
    }
}

#[cfg(test)]
mod tests {
    use super::{StringFunctionExpr, StringFunctionExprKind};
    use crate::plan::{
        BoolExpr, Expr, FunctionType, IntExpr, ParamLocal, Step, StringFunctionId,
        StringFunctionLocalId, StringFunctionValue, StringLocalId, ValueType,
    };

    #[test]
    fn string_function_expr_kind_accessors() {
        assert!(matches!(
            StringFunctionExpr::local_get(StringFunctionLocalId(0), "f".into(), function_type())
                .kind(),
            StringFunctionExprKind::LocalGet { .. },
        ));
        assert!(matches!(
            StringFunctionExpr::bool_case(
                BoolExpr::value(true),
                function_value(),
                function_value(),
            )
            .kind(),
            StringFunctionExprKind::BoolCase { .. },
        ));
        assert!(matches!(
            StringFunctionExpr::int_case(
                IntExpr::value(1.into()),
                vec![(1.into(), function_value())],
                function_value(),
            )
            .kind(),
            StringFunctionExprKind::IntCase { .. },
        ));
        assert!(matches!(
            StringFunctionExpr::block(
                vec![Step::evaluate(Expr::int(IntExpr::value(1.into())))],
                function_value(),
            )
            .kind(),
            StringFunctionExprKind::Block { .. },
        ));
    }

    #[test]
    fn string_function_expr_type() {
        assert_eq!(function_value().type_(), &function_type());
    }

    fn function_value() -> StringFunctionExpr {
        StringFunctionExpr::value(StringFunctionValue::new(
            StringFunctionId(0),
            vec![ParamLocal::string(StringLocalId(0))],
        ))
    }

    fn function_type() -> FunctionType {
        FunctionType::new(vec![ValueType::String], ValueType::String)
    }
}
