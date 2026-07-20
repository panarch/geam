mod bit_array;
mod bool;
mod custom;
mod float;
mod generic;
mod int;
mod list;
mod never;
mod nil;
mod returning_function;
mod string;
mod tuple;
mod typed;
mod utf_codepoint;

pub use self::{
    bit_array::BitArrayFunctionExpr, bool::BoolFunctionExpr, custom::CustomFunctionExpr,
    float::FloatFunctionExpr, generic::GenericFunctionExpr, int::IntFunctionExpr,
    list::ListFunctionExpr, never::NeverFunctionExpr, nil::NilFunctionExpr,
    returning_function::FunctionFunctionExpr, string::StringFunctionExpr, tuple::TupleFunctionExpr,
    utf_codepoint::UtfCodepointFunctionExpr,
};
pub(crate) use self::{
    bit_array::BitArrayFunctionExprKind, bool::BoolFunctionExprKind,
    custom::CustomFunctionExprKind, float::FloatFunctionExprKind, generic::GenericFunctionExprKind,
    int::IntFunctionExprKind, list::ListFunctionExprKind, never::NeverFunctionExprKind,
    nil::NilFunctionExprKind, returning_function::FunctionFunctionExprKind,
    string::StringFunctionExprKind, tuple::TupleFunctionExprKind, typed::TypedFunctionExpr,
    utf_codepoint::UtfCodepointFunctionExprKind,
};

pub struct FunctionExpr {
    shape: super::super::FunctionShape,
    kind: FunctionExprKind,
}

pub(crate) enum FunctionExprKind {
    Generic(GenericFunctionExpr),
    Never(NeverFunctionExpr),
    Int(IntFunctionExpr),
    String(StringFunctionExpr),
    BitArray(BitArrayFunctionExpr),
    UtfCodepoint(UtfCodepointFunctionExpr),
    Custom(CustomFunctionExpr),
    Float(FloatFunctionExpr),
    Bool(BoolFunctionExpr),
    Nil(NilFunctionExpr),
    Tuple(TupleFunctionExpr),
    List(ListFunctionExpr),
    Function(FunctionFunctionExpr),
}

impl FunctionExpr {
    pub(in crate::plan::execution) fn from_parts(
        shape: super::super::FunctionShape,
        kind: FunctionExprKind,
    ) -> Self {
        Self { shape, kind }
    }

    pub(crate) fn shape(&self) -> &super::super::FunctionShape {
        &self.shape
    }

    pub(crate) fn kind(&self) -> &FunctionExprKind {
        &self.kind
    }
}
