mod item;
mod local;
mod typed;

pub(crate) use self::{
    item::{
        BitArrayListItem, BoolListItem, FloatListItem, FunctionListItem, IntListItem, ListItem,
        ListListItem, NilListItem, StringListItem, TupleListItem, UtfCodepointListItem,
    },
    local::ListLocalExpr,
    typed::{ListIndexSource, TypedListExpr, TypedListExprKind},
};

pub(crate) enum ListExpr {
    Int(IntListExpr),
    String(StringListExpr),
    BitArray(BitArrayListExpr),
    UtfCodepoint(UtfCodepointListExpr),
    Float(FloatListExpr),
    Bool(BoolListExpr),
    Nil(NilListExpr),
    Tuple(TupleListExpr),
    List(ListListExpr),
    Function(FunctionListExpr),
}

pub(crate) type IntListExpr = TypedListExpr<IntListItem>;
pub(crate) type StringListExpr = TypedListExpr<StringListItem>;
pub(crate) type BitArrayListExpr = TypedListExpr<BitArrayListItem>;
pub(crate) type UtfCodepointListExpr = TypedListExpr<UtfCodepointListItem>;
pub(crate) type FloatListExpr = TypedListExpr<FloatListItem>;
pub(crate) type BoolListExpr = TypedListExpr<BoolListItem>;
pub(crate) type NilListExpr = TypedListExpr<NilListItem>;
pub(crate) type TupleListExpr = TypedListExpr<TupleListItem>;
pub(crate) type ListListExpr = TypedListExpr<ListListItem>;
pub(crate) type FunctionListExpr = TypedListExpr<FunctionListItem>;
