mod bool;
mod float;
mod int;
mod nil;
mod returning_function;
mod string;
mod tuple;

use crate::plan::{FunctionType, FunctionValue, FunctionValueKind};

pub use self::{
    bool::BoolFunctionExpr, float::FloatFunctionExpr, int::IntFunctionExpr, nil::NilFunctionExpr,
    returning_function::FunctionFunctionExpr, string::StringFunctionExpr, tuple::TupleFunctionExpr,
};
pub(crate) use self::{
    bool::BoolFunctionExprKind, float::FloatFunctionExprKind, int::IntFunctionExprKind,
    nil::NilFunctionExprKind, returning_function::FunctionFunctionExprKind,
    string::StringFunctionExprKind, tuple::TupleFunctionExprKind,
};

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionExpr {
    kind: FunctionExprKind,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum FunctionExprKind {
    Int(IntFunctionExpr),
    String(StringFunctionExpr),
    Float(FloatFunctionExpr),
    Bool(BoolFunctionExpr),
    Nil(NilFunctionExpr),
    Tuple(TupleFunctionExpr),
    Function(FunctionFunctionExpr),
}

impl FunctionExpr {
    pub(crate) fn value(value: FunctionValue) -> Self {
        match value.kind() {
            FunctionValueKind::Int(value) => Self::int(IntFunctionExpr::value(value.clone())),
            FunctionValueKind::String(value) => {
                Self::string(StringFunctionExpr::value(value.clone()))
            }
            FunctionValueKind::Float(value) => Self::float(FloatFunctionExpr::value(value.clone())),
            FunctionValueKind::Bool(value) => Self::bool(BoolFunctionExpr::value(value.clone())),
            FunctionValueKind::Nil(value) => Self::nil(NilFunctionExpr::value(value.clone())),
            FunctionValueKind::Tuple(value) => Self::tuple(TupleFunctionExpr::value(value.clone())),
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

    pub(crate) fn float(expression: FloatFunctionExpr) -> Self {
        Self {
            kind: FunctionExprKind::Float(expression),
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

    pub(crate) fn tuple(expression: TupleFunctionExpr) -> Self {
        Self {
            kind: FunctionExprKind::Tuple(expression),
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
            FunctionExprKind::Float(expression) => expression.type_(),
            FunctionExprKind::Bool(expression) => expression.type_(),
            FunctionExprKind::Nil(expression) => expression.type_(),
            FunctionExprKind::Tuple(expression) => expression.type_(),
            FunctionExprKind::Function(expression) => expression.type_(),
        }
    }

    pub(crate) fn kind(&self) -> &FunctionExprKind {
        &self.kind
    }

    pub(crate) fn into_kind(self) -> FunctionExprKind {
        self.kind
    }

    pub(crate) fn into_int(self) -> Option<IntFunctionExpr> {
        match self.kind {
            FunctionExprKind::Int(expression) => Some(expression),
            _ => None,
        }
    }

    pub(crate) fn into_string(self) -> Option<StringFunctionExpr> {
        match self.kind {
            FunctionExprKind::String(expression) => Some(expression),
            _ => None,
        }
    }

    pub(crate) fn into_float(self) -> Option<FloatFunctionExpr> {
        match self.kind {
            FunctionExprKind::Float(expression) => Some(expression),
            _ => None,
        }
    }

    pub(crate) fn into_bool(self) -> Option<BoolFunctionExpr> {
        match self.kind {
            FunctionExprKind::Bool(expression) => Some(expression),
            _ => None,
        }
    }

    pub(crate) fn into_nil(self) -> Option<NilFunctionExpr> {
        match self.kind {
            FunctionExprKind::Nil(expression) => Some(expression),
            _ => None,
        }
    }

    pub(crate) fn into_tuple(self) -> Option<TupleFunctionExpr> {
        match self.kind {
            FunctionExprKind::Tuple(expression) => Some(expression),
            _ => None,
        }
    }

    pub(crate) fn into_function(self) -> Option<FunctionFunctionExpr> {
        match self.kind {
            FunctionExprKind::Function(expression) => Some(expression),
            _ => None,
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

impl From<FloatFunctionExpr> for FunctionExpr {
    fn from(expression: FloatFunctionExpr) -> Self {
        Self::float(expression)
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

impl From<TupleFunctionExpr> for FunctionExpr {
    fn from(expression: TupleFunctionExpr) -> Self {
        Self::tuple(expression)
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
        BoolFunctionExpr, FloatFunctionExpr, FunctionExpr, FunctionExprKind, FunctionFunctionExpr,
        IntFunctionExpr, NilFunctionExpr, StringFunctionExpr, TupleFunctionExpr,
    };
    use crate::plan::{
        BoolFunctionId, BoolFunctionValue, FloatFunctionId, FloatFunctionValue, FunctionFunctionId,
        FunctionFunctionValue, FunctionType, FunctionValue, IntFunctionFunctionId, IntFunctionId,
        IntFunctionValue, NilFunctionId, NilFunctionValue, ParamLocal, RuntimeFunctionId,
        StringFunctionId, StringFunctionValue, TupleFunctionId, TupleFunctionValue, ValueType,
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
            FunctionExpr::float(float_function_value()).kind(),
            FunctionExprKind::Float(_)
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
            FunctionExpr::tuple(tuple_function_value()).kind(),
            FunctionExprKind::Tuple(_)
        ));
        assert!(matches!(
            FunctionExpr::function(function_function_value()).kind(),
            FunctionExprKind::Function(_)
        ));
    }

    #[test]
    fn function_expr_typed_conversions() {
        assert!(FunctionExpr::int(int_function_value()).into_int().is_some());
        assert!(
            FunctionExpr::string(string_function_value())
                .into_string()
                .is_some()
        );
        assert!(
            FunctionExpr::float(float_function_value())
                .into_float()
                .is_some()
        );
        assert!(
            FunctionExpr::bool(bool_function_value())
                .into_bool()
                .is_some()
        );
        assert!(FunctionExpr::nil(nil_function_value()).into_nil().is_some());
        assert!(
            FunctionExpr::tuple(tuple_function_value())
                .into_tuple()
                .is_some()
        );

        assert!(
            FunctionExpr::int(int_function_value())
                .into_string()
                .is_none()
        );
        assert!(
            FunctionExpr::int(int_function_value())
                .into_float()
                .is_none()
        );
        assert!(
            FunctionExpr::int(int_function_value())
                .into_bool()
                .is_none()
        );
        assert!(FunctionExpr::int(int_function_value()).into_nil().is_none());
        assert!(
            FunctionExpr::int(int_function_value())
                .into_tuple()
                .is_none()
        );

        assert!(matches!(
            FunctionExpr::from(int_function_value()).kind(),
            FunctionExprKind::Int(_),
        ));
        assert!(matches!(
            FunctionExpr::from(string_function_value()).kind(),
            FunctionExprKind::String(_),
        ));
        assert!(matches!(
            FunctionExpr::from(float_function_value()).kind(),
            FunctionExprKind::Float(_),
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
            FunctionExpr::from(tuple_function_value()).kind(),
            FunctionExprKind::Tuple(_),
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

    fn float_function_value() -> FloatFunctionExpr {
        FloatFunctionExpr::value(FloatFunctionValue::new(
            FloatFunctionId(0),
            vec![ParamLocal::float(crate::plan::FloatLocalId(0))],
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

    fn tuple_function_value() -> TupleFunctionExpr {
        TupleFunctionExpr::value(TupleFunctionValue::new(
            TupleFunctionId(0),
            vec![ParamLocal::tuple(
                crate::plan::TupleLocalId(0),
                vec![ValueType::Int],
            )],
            vec![ValueType::Int],
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
