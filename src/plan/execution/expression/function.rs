mod bit_array;
mod bool;
mod float;
mod int;
mod list;
mod nil;
mod returning_function;
mod string;
mod tuple;

pub use self::{
    bit_array::BitArrayFunctionExpr, bool::BoolFunctionExpr, float::FloatFunctionExpr,
    int::IntFunctionExpr, list::ListFunctionExpr, nil::NilFunctionExpr,
    returning_function::FunctionFunctionExpr, string::StringFunctionExpr, tuple::TupleFunctionExpr,
};
pub(crate) use self::{
    bit_array::BitArrayFunctionExprKind, bool::BoolFunctionExprKind, float::FloatFunctionExprKind,
    int::IntFunctionExprKind, list::ListFunctionExprKind, nil::NilFunctionExprKind,
    returning_function::FunctionFunctionExprKind, string::StringFunctionExprKind,
    tuple::TupleFunctionExprKind,
};

pub struct FunctionExpr {
    kind: FunctionExprKind,
}

pub(crate) enum FunctionExprKind {
    Int(IntFunctionExpr),
    String(StringFunctionExpr),
    BitArray(BitArrayFunctionExpr),
    Float(FloatFunctionExpr),
    Bool(BoolFunctionExpr),
    Nil(NilFunctionExpr),
    Tuple(TupleFunctionExpr),
    List(ListFunctionExpr),
    Function(FunctionFunctionExpr),
}

impl FunctionExpr {
    pub(in crate::plan::execution) fn from_kind(kind: FunctionExprKind) -> Self {
        Self { kind }
    }

    pub(crate) fn kind(&self) -> &FunctionExprKind {
        &self.kind
    }
}
