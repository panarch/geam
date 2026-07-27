use crate::plan::ValueType;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum HostValueType {
    Int,
    Float,
    String,
    BitArray,
    UtfCodepoint,
    Bool,
    Nil,
}

impl HostValueType {
    pub(crate) fn value_type(self) -> ValueType {
        match self {
            Self::Int => ValueType::Int,
            Self::Float => ValueType::Float,
            Self::String => ValueType::String,
            Self::BitArray => ValueType::BitArray,
            Self::UtfCodepoint => ValueType::UtfCodepoint,
            Self::Bool => ValueType::Bool,
            Self::Nil => ValueType::Nil,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::HostValueType;
    use crate::plan::ValueType;

    #[test]
    fn maps_host_value_types_to_plan_value_types() {
        assert_eq!(HostValueType::Int.value_type(), ValueType::Int);
        assert_eq!(HostValueType::Float.value_type(), ValueType::Float);
        assert_eq!(HostValueType::String.value_type(), ValueType::String);
        assert_eq!(HostValueType::BitArray.value_type(), ValueType::BitArray);
        assert_eq!(
            HostValueType::UtfCodepoint.value_type(),
            ValueType::UtfCodepoint,
        );
        assert_eq!(HostValueType::Bool.value_type(), ValueType::Bool);
        assert_eq!(HostValueType::Nil.value_type(), ValueType::Nil);
    }
}
