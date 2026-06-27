mod bool;
mod int;
mod nil;
mod returning_function;
mod string;

use crate::plan::{FunctionType, FunctionValue, FunctionValueKind};

pub use self::{
    bool::BoolFunctionExpr, int::IntFunctionExpr, nil::NilFunctionExpr,
    returning_function::FunctionFunctionExpr, string::StringFunctionExpr,
};
pub(crate) use self::{
    bool::BoolFunctionExprKind, int::IntFunctionExprKind, nil::NilFunctionExprKind,
    returning_function::FunctionFunctionExprKind, string::StringFunctionExprKind,
};

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionExpr {
    kind: FunctionExprKind,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum FunctionExprKind {
    Int(IntFunctionExpr),
    String(StringFunctionExpr),
    Bool(BoolFunctionExpr),
    Nil(NilFunctionExpr),
    Function(FunctionFunctionExpr),
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
            FunctionValueKind::Function(value) => {
                Self::function(FunctionFunctionExpr::value(value.clone()))
            }
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

    pub(crate) fn function(expression: FunctionFunctionExpr) -> Self {
        Self {
            kind: FunctionExprKind::Function(expression),
        }
    }

    pub fn type_(&self) -> &FunctionType {
        match &self.kind {
            FunctionExprKind::Int(expression) => expression.type_(),
            FunctionExprKind::String(expression) => expression.type_(),
            FunctionExprKind::Bool(expression) => expression.type_(),
            FunctionExprKind::Nil(expression) => expression.type_(),
            FunctionExprKind::Function(expression) => expression.type_(),
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

    pub(crate) fn into_function(self) -> Result<FunctionFunctionExpr, Self> {
        match self.kind {
            FunctionExprKind::Function(expression) => Ok(expression),
            kind => Err(Self { kind }),
        }
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

impl From<FunctionFunctionExpr> for FunctionExpr {
    fn from(expression: FunctionFunctionExpr) -> Self {
        Self::function(expression)
    }
}

#[cfg(test)]
mod tests {
    use super::{
        BoolFunctionExpr, FunctionExpr, FunctionExprKind, FunctionFunctionExpr, IntFunctionExpr,
        NilFunctionExpr, StringFunctionExpr,
    };
    use crate::plan::{
        BoolFunctionId, BoolFunctionValue, FunctionFunctionId, FunctionFunctionValue, FunctionType,
        FunctionValue, IntFunctionFunctionId, IntFunctionId, IntFunctionValue, NilFunctionId,
        NilFunctionValue, ParamLocal, RuntimeFunctionId, StringFunctionId, StringFunctionValue,
        ValueType,
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
        assert!(matches!(
            FunctionExpr::function(function_function_value()).kind(),
            FunctionExprKind::Function(_)
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
        assert!(matches!(
            FunctionExpr::from(function_function_value()).kind(),
            FunctionExprKind::Function(_),
        ));
    }

    fn function_value() -> FunctionValue {
        FunctionValue::new(
            RuntimeFunctionId::Int(IntFunctionId(0)),
            vec![ParamLocal::int(crate::plan::IntLocalId(0))],
        )
    }

    fn int_function_value() -> IntFunctionExpr {
        IntFunctionExpr::value(IntFunctionValue::new(
            IntFunctionId(0),
            vec![ParamLocal::int(crate::plan::IntLocalId(0))],
        ))
    }

    fn string_function_value() -> StringFunctionExpr {
        StringFunctionExpr::value(StringFunctionValue::new(
            StringFunctionId(0),
            vec![ParamLocal::string(crate::plan::StringLocalId(0))],
        ))
    }

    fn bool_function_value() -> BoolFunctionExpr {
        BoolFunctionExpr::value(BoolFunctionValue::new(
            BoolFunctionId(0),
            vec![ParamLocal::bool(crate::plan::BoolLocalId(0))],
        ))
    }

    fn nil_function_value() -> NilFunctionExpr {
        NilFunctionExpr::value(NilFunctionValue::new(
            NilFunctionId(0),
            vec![ParamLocal::nil(crate::plan::NilLocalId(0))],
        ))
    }

    fn function_function_value() -> FunctionFunctionExpr {
        FunctionFunctionExpr::value(FunctionFunctionValue::new(
            FunctionFunctionId::Int(IntFunctionFunctionId(0)),
            Vec::new(),
            FunctionType::new(vec![ValueType::Int], ValueType::Int),
        ))
    }
}
