mod arg;
mod bit_array;
mod bool;
mod float;
mod function;
mod int;
mod list;
mod nil;
mod panic;
mod string;
mod tuple;

pub use self::{
    arg::CallArg,
    bit_array::BitArrayExpr,
    bool::BoolExpr,
    float::FloatExpr,
    function::{
        BitArrayFunctionExpr, BoolFunctionExpr, FloatFunctionExpr, FunctionExpr,
        FunctionFunctionExpr, IntFunctionExpr, ListFunctionExpr, NilFunctionExpr,
        StringFunctionExpr, TupleFunctionExpr,
    },
    int::IntExpr,
    nil::NilExpr,
    string::StringExpr,
    tuple::TupleExpr,
};
pub(crate) use self::{
    arg::{CallArgKind, CaptureArg, CaptureArgKind},
    bit_array::{BitArrayExprKind, BitArraySegment, Endianness, FloatBitSize, StringEncoding},
    bool::BoolExprKind,
    float::FloatExprKind,
    function::{
        BitArrayFunctionExprKind, BoolFunctionExprKind, FloatFunctionExprKind, FunctionExprKind,
        FunctionFunctionExprKind, IntFunctionExprKind, ListFunctionExprKind, NilFunctionExprKind,
        StringFunctionExprKind, TupleFunctionExprKind,
    },
    int::IntExprKind,
    list::{
        BitArrayListExpr, BitArrayListItem, BoolListExpr, BoolListItem, FloatListExpr,
        FloatListItem, FunctionListExpr, FunctionListItem, IntListExpr, IntListItem, ListExpr,
        ListIndexSource, ListItem, ListListExpr, ListListItem, ListLocalExpr, NilListExpr,
        NilListItem, StringListExpr, StringListItem, TupleListExpr, TupleListItem, TypedListExpr,
        TypedListExprKind,
    },
    nil::NilExprKind,
    panic::{PanicExpr, PanicExprKind},
    string::StringExprKind,
    tuple::TupleExprKind,
};

pub struct Expr {
    kind: ExprKind,
}

pub(crate) enum ExprKind {
    Int(IntExpr),
    String(StringExpr),
    BitArray(BitArrayExpr),
    Float(FloatExpr),
    Bool(BoolExpr),
    Nil(NilExpr),
    Tuple(TupleExpr),
    List(ListExpr),
    Function(FunctionExpr),
}

impl Expr {
    pub(in crate::plan::execution) fn from_kind(kind: ExprKind) -> Self {
        Self { kind }
    }

    pub(crate) fn kind(&self) -> &ExprKind {
        &self.kind
    }
}
