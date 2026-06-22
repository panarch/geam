use ecow::EcoString;
use num_bigint::BigInt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
