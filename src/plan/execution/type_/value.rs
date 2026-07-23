use super::{CustomTypeId, FunctionType, ListTypeId};
use crate::plan;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) enum ValueType {
    Parameter(plan::TypeParameterId),
    Int,
    Float,
    String,
    BitArray,
    UtfCodepoint,
    Bool,
    Nil,
    Tuple(Vec<ValueType>),
    List(ListTypeId),
    Function(Box<FunctionType>),
    Custom(CustomTypeId),
}
