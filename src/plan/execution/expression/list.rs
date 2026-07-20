mod item;
mod local;
mod parameter;
mod typed;

pub(crate) use self::{
    item::{
        BitArrayListItem, BoolListItem, CustomListItem, FloatListItem, FunctionListItem,
        IntListItem, ListItem, ListListItem, NilListItem, ParameterListItem, ParameterListListItem,
        StringListItem, TupleListItem, UtfCodepointListItem,
    },
    local::ListLocalExpr,
    parameter::{ParameterListExpr, ParameterListExprKind, ParameterListIndexSource},
    typed::{ListIndexSource, TypedListExpr, TypedListExprKind},
};

pub(crate) enum ListExpr {
    Parameter(ParameterListExpr),
    ParameterList(ParameterListListExpr),
    Int(IntListExpr),
    String(StringListExpr),
    BitArray(BitArrayListExpr),
    UtfCodepoint(UtfCodepointListExpr),
    Custom(CustomListExpr),
    Float(FloatListExpr),
    Bool(BoolListExpr),
    Nil(NilListExpr),
    Tuple(TupleListExpr),
    List(ListListExpr),
    Function(FunctionListExpr),
}

pub(crate) enum StoredListExpr {
    ParameterList(ParameterListListExpr),
    Int(IntListExpr),
    String(StringListExpr),
    BitArray(BitArrayListExpr),
    UtfCodepoint(UtfCodepointListExpr),
    Custom(CustomListExpr),
    Float(FloatListExpr),
    Bool(BoolListExpr),
    Nil(NilListExpr),
    Tuple(TupleListExpr),
    List(ListListExpr),
    Function(FunctionListExpr),
}

pub(crate) type IntListExpr = TypedListExpr<IntListItem>;
pub(crate) type ParameterListListExpr = TypedListExpr<ParameterListListItem>;
pub(crate) type StringListExpr = TypedListExpr<StringListItem>;
pub(crate) type BitArrayListExpr = TypedListExpr<BitArrayListItem>;
pub(crate) type UtfCodepointListExpr = TypedListExpr<UtfCodepointListItem>;
pub(crate) type CustomListExpr = TypedListExpr<CustomListItem>;
pub(crate) type FloatListExpr = TypedListExpr<FloatListItem>;
pub(crate) type BoolListExpr = TypedListExpr<BoolListItem>;
pub(crate) type NilListExpr = TypedListExpr<NilListItem>;
pub(crate) type TupleListExpr = TypedListExpr<TupleListItem>;
pub(crate) type ListListExpr = TypedListExpr<ListListItem>;
pub(crate) type FunctionListExpr = TypedListExpr<FunctionListItem>;
