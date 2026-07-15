mod arg;
mod bit_array;
mod bool;
mod custom;
mod custom_field;
mod float;
mod function;
mod int;
mod list;
mod nil;
mod panic;
mod string;
mod tuple;
mod utf_codepoint;

pub use self::{
    arg::CallArg,
    bit_array::BitArrayExpr,
    bool::BoolExpr,
    custom::CustomExpr,
    float::FloatExpr,
    function::{
        BitArrayFunctionExpr, BoolFunctionExpr, CustomFunctionExpr, FloatFunctionExpr,
        FunctionExpr, FunctionFunctionExpr, IntFunctionExpr, ListFunctionExpr, NilFunctionExpr,
        StringFunctionExpr, TupleFunctionExpr, UtfCodepointFunctionExpr,
    },
    int::IntExpr,
    nil::NilExpr,
    string::StringExpr,
    tuple::TupleExpr,
    utf_codepoint::UtfCodepointExpr,
};
pub(crate) use self::{
    arg::{CallArgKind, CaptureArg, CaptureArgKind},
    bit_array::{BitArrayExprKind, BitArraySegment, Endianness, FloatBitSize, StringEncoding},
    bool::BoolExprKind,
    custom::{CustomConstruction, CustomExprKind, CustomFunctionCall},
    custom_field::CustomFieldAccess,
    float::FloatExprKind,
    function::{
        BitArrayFunctionExprKind, BoolFunctionExprKind, CustomFunctionExprKind,
        FloatFunctionExprKind, FunctionExprKind, FunctionFunctionExprKind, IntFunctionExprKind,
        ListFunctionExprKind, NilFunctionExprKind, StringFunctionExprKind, TupleFunctionExprKind,
        UtfCodepointFunctionExprKind,
    },
    int::IntExprKind,
    list::{
        BitArrayListExpr, BitArrayListItem, BoolListExpr, BoolListItem, CustomListExpr,
        CustomListItem, FloatListExpr, FloatListItem, FunctionListExpr, FunctionListItem,
        IntListExpr, IntListItem, ListExpr, ListIndexSource, ListItem, ListListExpr, ListListItem,
        ListLocalExpr, NilListExpr, NilListItem, StringListExpr, StringListItem, TupleListExpr,
        TupleListItem, TypedListExpr, TypedListExprKind, UtfCodepointListExpr,
        UtfCodepointListItem,
    },
    nil::NilExprKind,
    panic::{PanicExpr, PanicExprKind},
    string::StringExprKind,
    tuple::TupleExprKind,
    utf_codepoint::UtfCodepointExprKind,
};

pub struct Expr {
    kind: ExprKind,
}

pub(crate) enum ExprKind {
    Int(IntExpr),
    String(StringExpr),
    BitArray(BitArrayExpr),
    UtfCodepoint(UtfCodepointExpr),
    Custom(CustomExpr),
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
