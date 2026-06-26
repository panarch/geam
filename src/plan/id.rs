#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocalId {
    Int(IntLocalId),
    String(StringLocalId),
    Bool(BoolLocalId),
    Nil(NilLocalId),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IntLocalId(pub(crate) usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StringLocalId(pub(crate) usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BoolLocalId(pub(crate) usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NilLocalId(pub(crate) usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct IntFunctionLocalId(pub(crate) usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct StringFunctionLocalId(pub(crate) usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BoolFunctionLocalId(pub(crate) usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NilFunctionLocalId(pub(crate) usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FunctionId(usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum RuntimeFunctionId {
    Int(IntFunctionId),
    String(StringFunctionId),
    Bool(BoolFunctionId),
    Nil(NilFunctionId),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IntFunctionId(pub(crate) usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StringFunctionId(pub(crate) usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BoolFunctionId(pub(crate) usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NilFunctionId(pub(crate) usize);

impl FunctionId {
    pub(crate) fn new(index: usize) -> Self {
        Self(index)
    }

    #[cfg(test)]
    pub(crate) fn index(self) -> usize {
        self.0
    }
}

impl LocalId {
    pub(crate) fn value_type(self) -> crate::plan::ValueType {
        match self {
            Self::Int(_) => crate::plan::ValueType::Int,
            Self::String(_) => crate::plan::ValueType::String,
            Self::Bool(_) => crate::plan::ValueType::Bool,
            Self::Nil(_) => crate::plan::ValueType::Nil,
        }
    }
}

impl RuntimeFunctionId {
    pub(crate) fn value_type(self) -> crate::plan::ValueType {
        match self {
            Self::Int(_) => crate::plan::ValueType::Int,
            Self::String(_) => crate::plan::ValueType::String,
            Self::Bool(_) => crate::plan::ValueType::Bool,
            Self::Nil(_) => crate::plan::ValueType::Nil,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        BoolFunctionId, BoolFunctionLocalId, BoolLocalId, FunctionId, IntFunctionId,
        IntFunctionLocalId, IntLocalId, LocalId, NilFunctionId, NilFunctionLocalId, NilLocalId,
        RuntimeFunctionId, StringFunctionId, StringFunctionLocalId, StringLocalId,
    };
    use crate::plan::ValueType;

    #[test]
    fn function_id_index() {
        assert_eq!(FunctionId::new(5).index(), 5);
    }

    #[test]
    fn local_id_value_type() {
        assert_eq!(LocalId::Int(IntLocalId(0)).value_type(), ValueType::Int);
        assert_eq!(
            LocalId::String(StringLocalId(0)).value_type(),
            ValueType::String
        );
        assert_eq!(LocalId::Bool(BoolLocalId(0)).value_type(), ValueType::Bool);
        assert_eq!(LocalId::Nil(NilLocalId(0)).value_type(), ValueType::Nil);
    }

    #[test]
    fn runtime_function_id_value_type() {
        assert_eq!(
            RuntimeFunctionId::Int(IntFunctionId(0)).value_type(),
            ValueType::Int
        );
        assert_eq!(
            RuntimeFunctionId::String(StringFunctionId(0)).value_type(),
            ValueType::String
        );
        assert_eq!(
            RuntimeFunctionId::Bool(BoolFunctionId(0)).value_type(),
            ValueType::Bool
        );
        assert_eq!(
            RuntimeFunctionId::Nil(NilFunctionId(0)).value_type(),
            ValueType::Nil
        );
    }

    #[test]
    fn function_local_id_debug_surface() {
        assert_eq!(
            format!("{:?}", IntFunctionLocalId(3)),
            "IntFunctionLocalId(3)"
        );
        assert_eq!(
            format!("{:?}", StringFunctionLocalId(3)),
            "StringFunctionLocalId(3)"
        );
        assert_eq!(
            format!("{:?}", BoolFunctionLocalId(3)),
            "BoolFunctionLocalId(3)"
        );
        assert_eq!(
            format!("{:?}", NilFunctionLocalId(3)),
            "NilFunctionLocalId(3)"
        );
    }
}
