use ecow::EcoString;
use num_bigint::BigInt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ValueType {
    Int,
    String,
    Bool,
    Nil,
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
