use super::id::{BoolLocalId, FunctionId, IntLocalId, NilLocalId, StringLocalId};
use super::value::{Value, ValueType};
use ecow::EcoString;
use num_bigint::BigInt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expr {
    Int(IntExpr),
    String(StringExpr),
    Bool(BoolExpr),
    Nil(NilExpr),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IntExpr {
    Value(BigInt),
    LocalGet {
        local: IntLocalId,
        name: EcoString,
    },
    Call {
        function: FunctionId,
        args: Vec<Expr>,
    },
    Add {
        left: Box<IntExpr>,
        right: Box<IntExpr>,
    },
    Sub {
        left: Box<IntExpr>,
        right: Box<IntExpr>,
    },
    Mult {
        left: Box<IntExpr>,
        right: Box<IntExpr>,
    },
    Div {
        left: Box<IntExpr>,
        right: Box<IntExpr>,
    },
    Remainder {
        left: Box<IntExpr>,
        right: Box<IntExpr>,
    },
    Negate(Box<IntExpr>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StringExpr {
    Value(EcoString),
    LocalGet {
        local: StringLocalId,
        name: EcoString,
    },
    Call {
        function: FunctionId,
        args: Vec<Expr>,
    },
    Concatenate {
        left: Box<StringExpr>,
        right: Box<StringExpr>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BoolExpr {
    Value(bool),
    LocalGet {
        local: BoolLocalId,
        name: EcoString,
    },
    Call {
        function: FunctionId,
        args: Vec<Expr>,
    },
    Not(Box<BoolExpr>),
    LtInt {
        left: Box<IntExpr>,
        right: Box<IntExpr>,
    },
    LtEqInt {
        left: Box<IntExpr>,
        right: Box<IntExpr>,
    },
    GtInt {
        left: Box<IntExpr>,
        right: Box<IntExpr>,
    },
    GtEqInt {
        left: Box<IntExpr>,
        right: Box<IntExpr>,
    },
    Equal {
        left: Box<Expr>,
        right: Box<Expr>,
    },
    NotEqual {
        left: Box<Expr>,
        right: Box<Expr>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NilExpr {
    Value,
    LocalGet {
        local: NilLocalId,
        name: EcoString,
    },
    Call {
        function: FunctionId,
        args: Vec<Expr>,
    },
}

impl Expr {
    pub fn value_type(&self) -> ValueType {
        match self {
            Self::Int(_) => ValueType::Int,
            Self::String(_) => ValueType::String,
            Self::Bool(_) => ValueType::Bool,
            Self::Nil(_) => ValueType::Nil,
        }
    }
}

impl From<Value> for Expr {
    fn from(value: Value) -> Self {
        match value {
            Value::Int(value) => Self::Int(IntExpr::Value(value)),
            Value::String(value) => Self::String(StringExpr::Value(value)),
            Value::Bool(value) => Self::Bool(BoolExpr::Value(value)),
            Value::Nil => Self::Nil(NilExpr::Value),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{BoolExpr, Expr, IntExpr, NilExpr, StringExpr};
    use crate::plan::{Value, ValueType};
    use num_bigint::BigInt;

    #[test]
    fn expr_value_shapes() {
        assert_eq!(
            Expr::Int(IntExpr::Value(BigInt::from(1))),
            Expr::from(Value::Int(BigInt::from(1)))
        );
        assert_eq!(
            Expr::String(StringExpr::Value("geam".into())),
            Expr::from(Value::String("geam".into()))
        );
        assert_eq!(
            Expr::Bool(BoolExpr::Value(true)),
            Expr::from(Value::Bool(true))
        );
        assert_eq!(Expr::Nil(NilExpr::Value), Expr::from(Value::Nil));
    }

    #[test]
    fn expr_value_type() {
        assert_eq!(
            Expr::from(Value::Int(BigInt::from(1))).value_type(),
            ValueType::Int
        );
        assert_eq!(
            Expr::from(Value::String("geam".into())).value_type(),
            ValueType::String,
        );
        assert_eq!(Expr::from(Value::Bool(true)).value_type(), ValueType::Bool);
        assert_eq!(Expr::from(Value::Nil).value_type(), ValueType::Nil);
    }
}
