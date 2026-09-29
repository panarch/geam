use super::{
    BitArrayListValueId, BoolListValueId, CustomListValueId, ExternalListValueId, FloatListValueId,
    FunctionListValueId, IntListValueId, ListListValueId, NilListValueId, ParameterListListValueId,
    StoredListValueId, StringListValueId, TupleListValueId, UtfCodepointListValueId,
};

// The source handle owns its typed payload for the entire borrowed read.
#[derive(Clone, Copy)]
pub(in crate::runtime) enum StoredListValueRef<'value> {
    Int(&'value IntListValueId),
    String(&'value StringListValueId),
    BitArray(&'value BitArrayListValueId),
    UtfCodepoint(&'value UtfCodepointListValueId),
    Custom(&'value CustomListValueId),
    External(&'value ExternalListValueId),
    Float(&'value FloatListValueId),
    Bool(&'value BoolListValueId),
    Nil(&'value NilListValueId),
    Tuple(&'value TupleListValueId),
    ParameterList(&'value ParameterListListValueId),
    List(&'value ListListValueId),
    Function(&'value FunctionListValueId),
}

impl<'value> From<&'value StoredListValueId> for StoredListValueRef<'value> {
    fn from(value: &'value StoredListValueId) -> Self {
        match value {
            StoredListValueId::Int(value) => Self::Int(value),
            StoredListValueId::String(value) => Self::String(value),
            StoredListValueId::BitArray(value) => Self::BitArray(value),
            StoredListValueId::UtfCodepoint(value) => Self::UtfCodepoint(value),
            StoredListValueId::Custom(value) => Self::Custom(value),
            StoredListValueId::External(value) => Self::External(value),
            StoredListValueId::Float(value) => Self::Float(value),
            StoredListValueId::Bool(value) => Self::Bool(value),
            StoredListValueId::Nil(value) => Self::Nil(value),
            StoredListValueId::Tuple(value) => Self::Tuple(value),
            StoredListValueId::ParameterList(value) => Self::ParameterList(value),
            StoredListValueId::List(value) => Self::List(value),
            StoredListValueId::Function(value) => Self::Function(value),
        }
    }
}
