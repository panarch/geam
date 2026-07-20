mod arg;
mod bit_array;
mod bool;
mod custom;
mod custom_field;
mod float;
mod function;
mod int;
mod list;
mod never;
mod nil;
mod panic;
mod string;
mod tuple;
mod utf_codepoint;

pub(crate) use self::function::TypedFunctionExpr;
pub use self::{
    arg::CallArg,
    bit_array::BitArrayExpr,
    bool::BoolExpr,
    custom::CustomExpr,
    float::FloatExpr,
    function::{
        BitArrayFunctionExpr, BoolFunctionExpr, CustomFunctionExpr, FloatFunctionExpr,
        FunctionExpr, FunctionFunctionExpr, GenericFunctionExpr, IntFunctionExpr, ListFunctionExpr,
        NeverFunctionExpr, NilFunctionExpr, StringFunctionExpr, TupleFunctionExpr,
        UtfCodepointFunctionExpr,
    },
    int::IntExpr,
    nil::NilExpr,
    string::StringExpr,
    tuple::TupleExpr,
    utf_codepoint::UtfCodepointExpr,
};
pub(crate) use self::{
    arg::{CallArgKind, CaptureArg, CaptureArgKind, DirectCall, FunctionCall},
    bit_array::{
        BitArrayBitsSize, BitArrayEvaluatedSize, BitArrayExprKind, BitArraySegment, Endianness,
        FloatBitSize, StringEncoding,
    },
    bool::BoolExprKind,
    custom::{CustomConstruction, CustomExprKind, CustomLocalExpr},
    custom_field::CustomFieldAccess,
    float::FloatExprKind,
    function::{
        BitArrayFunctionExprKind, BoolFunctionExprKind, CustomFunctionExprKind,
        FloatFunctionExprKind, FunctionExprKind, FunctionFunctionExprKind, GenericFunctionExprKind,
        IntFunctionExprKind, ListFunctionExprKind, NeverFunctionExprKind, NilFunctionExprKind,
        StringFunctionExprKind, TupleFunctionExprKind, UtfCodepointFunctionExprKind,
    },
    int::IntExprKind,
    list::{
        BitArrayListExpr, BitArrayListItem, BoolListExpr, BoolListItem, CustomListExpr,
        CustomListItem, FloatListExpr, FloatListItem, FunctionListExpr, FunctionListItem,
        IntListExpr, IntListItem, ListExpr, ListIndexSource, ListItem, ListListExpr, ListListItem,
        ListLocalExpr, NilListExpr, NilListItem, ParameterListExpr, ParameterListExprKind,
        ParameterListIndexSource, ParameterListItem, ParameterListListExpr, ParameterListListItem,
        StoredListExpr, StringListExpr, StringListItem, TupleListExpr, TupleListItem,
        TypedListExpr, TypedListExprKind, UtfCodepointListExpr, UtfCodepointListItem,
    },
    never::{NeverExpr, NeverExprKind},
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
    Never(NeverExpr),
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

    pub(in crate::plan::execution) fn into_kind(self) -> ExprKind {
        self.kind
    }
}
