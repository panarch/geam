use ecow::EcoString;
use num_bigint::BigInt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LocalId(pub usize);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModulePlan {
    pub module: EcoString,
    pub functions: Vec<FunctionPlan>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FunctionPlan {
    pub name: EcoString,
    pub params: Vec<Param>,
    pub steps: Vec<Step>,
    pub return_: Expr,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Param {
    pub local: LocalId,
    pub name: EcoString,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Step {
    Let {
        local: LocalId,
        name: EcoString,
        value: Expr,
    },
    Evaluate(Expr),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expr {
    Value(Value),
    LocalGet {
        local: LocalId,
        name: EcoString,
    },
    Call {
        function: FunctionRef,
        args: Vec<Expr>,
    },
    BinOp {
        op: BinOp,
        left: Box<Expr>,
        right: Box<Expr>,
    },
    NegateInt(Box<Expr>),
    NegateBool(Box<Expr>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FunctionRef {
    Local(EcoString),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinOp {
    AddInt,
    SubInt,
    MultInt,
    Eq,
    NotEq,
    Concatenate,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value {
    Int(BigInt),
    String(EcoString),
    Bool(bool),
    Nil,
}

impl Value {
    pub fn kind(&self) -> &'static str {
        match self {
            Self::Int(_) => "Int",
            Self::String(_) => "String",
            Self::Bool(_) => "Bool",
            Self::Nil => "Nil",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Value;
    use num_bigint::BigInt;

    #[test]
    fn value_kind() {
        assert_eq!(Value::Int(BigInt::from(1)).kind(), "Int");
        assert_eq!(Value::String("geam".into()).kind(), "String");
        assert_eq!(Value::Bool(true).kind(), "Bool");
        assert_eq!(Value::Nil.kind(), "Nil");
    }
}
