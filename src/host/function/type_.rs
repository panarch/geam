use crate::plan::ValueType;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum HostValueType {
    Int,
    Bool,
}

impl HostValueType {
    pub(crate) fn value_type(self) -> ValueType {
        match self {
            Self::Int => ValueType::Int,
            Self::Bool => ValueType::Bool,
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
        assert_eq!(HostValueType::Bool.value_type(), ValueType::Bool);
    }
}
