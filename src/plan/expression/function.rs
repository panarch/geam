use super::{BoolExpr, IntExpr};
use crate::plan::{
    BoolFunctionLocalId, BoolFunctionValue, FunctionType, FunctionValue, FunctionValueKind,
    IntFunctionLocalId, IntFunctionValue, NilFunctionLocalId, NilFunctionValue, Step,
    StringFunctionLocalId, StringFunctionValue,
};
use ecow::EcoString;
use num_bigint::BigInt;

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionExpr {
    kind: FunctionExprKind,
}

#[derive(Debug, Clone, PartialEq)]
pub struct IntFunctionExpr {
    type_: FunctionType,
    kind: IntFunctionExprKind,
}

#[derive(Debug, Clone, PartialEq)]
pub struct StringFunctionExpr {
    type_: FunctionType,
    kind: StringFunctionExprKind,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BoolFunctionExpr {
    type_: FunctionType,
    kind: BoolFunctionExprKind,
}

#[derive(Debug, Clone, PartialEq)]
pub struct NilFunctionExpr {
    type_: FunctionType,
    kind: NilFunctionExprKind,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum FunctionExprKind {
    Int(IntFunctionExpr),
    String(StringFunctionExpr),
    Bool(BoolFunctionExpr),
    Nil(NilFunctionExpr),
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum IntFunctionExprKind {
    Value(IntFunctionValue),
    LocalGet {
        local: IntFunctionLocalId,
        name: EcoString,
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
    Block {
        steps: Vec<Step>,
        return_: Box<IntFunctionExpr>,
    },
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

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum BoolFunctionExprKind {
    Value(BoolFunctionValue),
    LocalGet {
        local: BoolFunctionLocalId,
        name: EcoString,
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

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum NilFunctionExprKind {
    Value(NilFunctionValue),
    LocalGet {
        local: NilFunctionLocalId,
        name: EcoString,
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

impl FunctionExpr {
    pub(crate) fn value(value: FunctionValue) -> Self {
        match value.kind() {
            FunctionValueKind::Int(value) => Self::int(IntFunctionExpr::value(value.clone())),
            FunctionValueKind::String(value) => {
                Self::string(StringFunctionExpr::value(value.clone()))
            }
            FunctionValueKind::Bool(value) => Self::bool(BoolFunctionExpr::value(value.clone())),
            FunctionValueKind::Nil(value) => Self::nil(NilFunctionExpr::value(value.clone())),
        }
    }

    pub(crate) fn int(expression: IntFunctionExpr) -> Self {
        Self {
            kind: FunctionExprKind::Int(expression),
        }
    }

    pub(crate) fn string(expression: StringFunctionExpr) -> Self {
        Self {
            kind: FunctionExprKind::String(expression),
        }
    }

    pub(crate) fn bool(expression: BoolFunctionExpr) -> Self {
        Self {
            kind: FunctionExprKind::Bool(expression),
        }
    }

    pub(crate) fn nil(expression: NilFunctionExpr) -> Self {
        Self {
            kind: FunctionExprKind::Nil(expression),
        }
    }

    pub fn type_(&self) -> &FunctionType {
        match &self.kind {
            FunctionExprKind::Int(expression) => expression.type_(),
            FunctionExprKind::String(expression) => expression.type_(),
            FunctionExprKind::Bool(expression) => expression.type_(),
            FunctionExprKind::Nil(expression) => expression.type_(),
        }
    }

    pub(crate) fn kind(&self) -> &FunctionExprKind {
        &self.kind
    }

    pub(crate) fn into_kind(self) -> FunctionExprKind {
        self.kind
    }

    pub(crate) fn into_int(self) -> Result<IntFunctionExpr, Self> {
        match self.kind {
            FunctionExprKind::Int(expression) => Ok(expression),
            kind => Err(Self { kind }),
        }
    }

    pub(crate) fn into_string(self) -> Result<StringFunctionExpr, Self> {
        match self.kind {
            FunctionExprKind::String(expression) => Ok(expression),
            kind => Err(Self { kind }),
        }
    }

    pub(crate) fn into_bool(self) -> Result<BoolFunctionExpr, Self> {
        match self.kind {
            FunctionExprKind::Bool(expression) => Ok(expression),
            kind => Err(Self { kind }),
        }
    }

    pub(crate) fn into_nil(self) -> Result<NilFunctionExpr, Self> {
        match self.kind {
            FunctionExprKind::Nil(expression) => Ok(expression),
            kind => Err(Self { kind }),
        }
    }
}

impl IntFunctionExpr {
    pub(crate) fn value(value: IntFunctionValue) -> Self {
        Self {
            type_: value.type_(),
            kind: IntFunctionExprKind::Value(value),
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

impl BoolFunctionExpr {
    pub(crate) fn value(value: BoolFunctionValue) -> Self {
        Self {
            type_: value.type_(),
            kind: BoolFunctionExprKind::Value(value),
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

impl From<IntFunctionExpr> for FunctionExpr {
    fn from(expression: IntFunctionExpr) -> Self {
        Self::int(expression)
    }
}

impl From<StringFunctionExpr> for FunctionExpr {
    fn from(expression: StringFunctionExpr) -> Self {
        Self::string(expression)
    }
}

impl From<BoolFunctionExpr> for FunctionExpr {
    fn from(expression: BoolFunctionExpr) -> Self {
        Self::bool(expression)
    }
}

impl From<NilFunctionExpr> for FunctionExpr {
    fn from(expression: NilFunctionExpr) -> Self {
        Self::nil(expression)
    }
}

#[cfg(test)]
mod tests {
    use super::{
        BoolFunctionExpr, BoolFunctionExprKind, FunctionExpr, FunctionExprKind, IntFunctionExpr,
        IntFunctionExprKind, NilFunctionExpr, NilFunctionExprKind, StringFunctionExpr,
        StringFunctionExprKind,
    };
    use crate::plan::{
        BoolExpr, BoolFunctionLocalId, BoolFunctionValue, BoolLocalId, Expr, FunctionType,
        FunctionValue, IntExpr, IntFunctionId, IntFunctionLocalId, IntFunctionValue, IntLocalId,
        LocalId, NilFunctionLocalId, NilFunctionValue, RuntimeFunctionId, Step,
        StringFunctionLocalId, StringFunctionValue, ValueType,
    };

    #[test]
    fn function_expr_kind_accessors() {
        assert!(matches!(
            FunctionExpr::value(function_value()).kind(),
            FunctionExprKind::Int(_)
        ));
        assert!(matches!(
            FunctionExpr::int(int_function_value()).kind(),
            FunctionExprKind::Int(_)
        ));
        assert!(matches!(
            FunctionExpr::string(string_function_value()).kind(),
            FunctionExprKind::String(_)
        ));
        assert!(matches!(
            FunctionExpr::bool(bool_function_value()).kind(),
            FunctionExprKind::Bool(_)
        ));
        assert!(matches!(
            FunctionExpr::nil(nil_function_value()).kind(),
            FunctionExprKind::Nil(_)
        ));
    }

    #[test]
    fn function_expr_typed_conversions() {
        assert!(FunctionExpr::int(int_function_value()).into_int().is_ok());
        assert!(
            FunctionExpr::string(string_function_value())
                .into_string()
                .is_ok()
        );
        assert!(
            FunctionExpr::bool(bool_function_value())
                .into_bool()
                .is_ok()
        );
        assert!(FunctionExpr::nil(nil_function_value()).into_nil().is_ok());

        assert!(
            FunctionExpr::int(int_function_value())
                .into_string()
                .is_err()
        );
        assert!(FunctionExpr::int(int_function_value()).into_bool().is_err());
        assert!(FunctionExpr::int(int_function_value()).into_nil().is_err());

        assert!(matches!(
            FunctionExpr::from(int_function_value()).kind(),
            FunctionExprKind::Int(_),
        ));
        assert!(matches!(
            FunctionExpr::from(string_function_value()).kind(),
            FunctionExprKind::String(_),
        ));
        assert!(matches!(
            FunctionExpr::from(bool_function_value()).kind(),
            FunctionExprKind::Bool(_),
        ));
        assert!(matches!(
            FunctionExpr::from(nil_function_value()).kind(),
            FunctionExprKind::Nil(_),
        ));
    }

    #[test]
    fn int_function_expr_kind_accessors() {
        assert_eq!(
            int_function_type(),
            FunctionType::new(vec![crate::plan::FunctionArgumentType::Int], ValueType::Int),
        );
        assert!(matches!(
            IntFunctionExpr::local_get(IntFunctionLocalId(0), "f".into(), int_function_type(),)
                .kind(),
            IntFunctionExprKind::LocalGet { .. }
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
        assert!(matches!(
            IntFunctionExpr::bool_case(
                BoolExpr::value(true),
                int_function_value(),
                int_function_value(),
            )
            .kind(),
            IntFunctionExprKind::BoolCase { .. }
        ));
    }

    #[test]
    fn string_bool_nil_function_expr_kind_accessors() {
        assert!(matches!(
            StringFunctionExpr::local_get(
                StringFunctionLocalId(0),
                "f".into(),
                string_function_value().type_().clone(),
            )
            .kind(),
            StringFunctionExprKind::LocalGet { .. }
        ));
        assert!(matches!(
            StringFunctionExpr::bool_case(
                BoolExpr::value(true),
                string_function_value(),
                string_function_value(),
            )
            .kind(),
            StringFunctionExprKind::BoolCase { .. }
        ));
        assert!(matches!(
            StringFunctionExpr::int_case(
                IntExpr::value(1.into()),
                vec![(1.into(), string_function_value())],
                string_function_value(),
            )
            .kind(),
            StringFunctionExprKind::IntCase { .. }
        ));
        assert!(matches!(
            StringFunctionExpr::block(Vec::new(), string_function_value()).kind(),
            StringFunctionExprKind::Block { .. }
        ));
        assert!(matches!(
            BoolFunctionExpr::local_get(
                BoolFunctionLocalId(0),
                "f".into(),
                bool_function_value().type_().clone(),
            )
            .kind(),
            BoolFunctionExprKind::LocalGet { .. }
        ));
        assert!(matches!(
            BoolFunctionExpr::bool_case(
                BoolExpr::value(true),
                bool_function_value(),
                bool_function_value(),
            )
            .kind(),
            BoolFunctionExprKind::BoolCase { .. }
        ));
        assert!(matches!(
            BoolFunctionExpr::int_case(
                IntExpr::value(1.into()),
                vec![(1.into(), bool_function_value())],
                bool_function_value(),
            )
            .kind(),
            BoolFunctionExprKind::IntCase { .. }
        ));
        assert!(matches!(
            BoolFunctionExpr::block(Vec::new(), bool_function_value()).kind(),
            BoolFunctionExprKind::Block { .. }
        ));
        assert!(matches!(
            NilFunctionExpr::local_get(
                NilFunctionLocalId(0),
                "f".into(),
                nil_function_value().type_().clone(),
            )
            .kind(),
            NilFunctionExprKind::LocalGet { .. }
        ));
        assert!(matches!(
            NilFunctionExpr::bool_case(
                BoolExpr::value(true),
                nil_function_value(),
                nil_function_value(),
            )
            .kind(),
            NilFunctionExprKind::BoolCase { .. }
        ));
        assert!(matches!(
            NilFunctionExpr::int_case(
                IntExpr::value(1.into()),
                vec![(1.into(), nil_function_value())],
                nil_function_value(),
            )
            .kind(),
            NilFunctionExprKind::IntCase { .. }
        ));
        assert!(matches!(
            NilFunctionExpr::block(Vec::new(), nil_function_value()).kind(),
            NilFunctionExprKind::Block { .. }
        ));
    }

    fn function_value() -> FunctionValue {
        FunctionValue::new(
            RuntimeFunctionId::Int(IntFunctionId(0)),
            vec![LocalId::Int(IntLocalId(0))],
        )
    }

    fn int_function_value() -> IntFunctionExpr {
        IntFunctionExpr::value(IntFunctionValue::new(
            IntFunctionId(0),
            vec![LocalId::Int(IntLocalId(0))],
        ))
    }

    fn string_function_value() -> StringFunctionExpr {
        StringFunctionExpr::value(StringFunctionValue::new(
            crate::plan::StringFunctionId(0),
            vec![LocalId::String(crate::plan::StringLocalId(0))],
        ))
    }

    fn bool_function_value() -> BoolFunctionExpr {
        BoolFunctionExpr::value(BoolFunctionValue::new(
            crate::plan::BoolFunctionId(0),
            vec![LocalId::Bool(BoolLocalId(0))],
        ))
    }

    fn nil_function_value() -> NilFunctionExpr {
        NilFunctionExpr::value(NilFunctionValue::new(
            crate::plan::NilFunctionId(0),
            vec![LocalId::Nil(crate::plan::NilLocalId(0))],
        ))
    }

    fn int_function_type() -> FunctionType {
        FunctionType::new(vec![crate::plan::FunctionArgumentType::Int], ValueType::Int)
    }
}
