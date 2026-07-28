use crate::plan::{TypeParameterId, ValueType};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum HostValueType {
    Parameter(TypeParameterId),
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
            Self::Parameter(parameter) => ValueType::Parameter(parameter),
            Self::Int => ValueType::Int,
            Self::Float => ValueType::Float,
            Self::String => ValueType::String,
            Self::BitArray => ValueType::BitArray,
            Self::UtfCodepoint => ValueType::UtfCodepoint,
            Self::Bool => ValueType::Bool,
            Self::Nil => ValueType::Nil,
        }
    }

    pub(crate) fn type_parameter_count(self) -> usize {
        match self {
            Self::Parameter(parameter) => parameter.index() + 1,
            Self::Int
            | Self::Float
            | Self::String
            | Self::BitArray
            | Self::UtfCodepoint
            | Self::Bool
            | Self::Nil => 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::HostValueType;
    use crate::plan::ValueType;

    #[test]
    fn maps_host_value_types_to_plan_value_types() {
        assert_eq!(
            HostValueType::Parameter(crate::plan::TypeParameterId(0)).value_type(),
            ValueType::Parameter(crate::plan::TypeParameterId(0)),
        );
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

    #[test]
    fn counts_only_explicit_host_type_parameters() {
        assert_eq!(
            HostValueType::Parameter(crate::plan::TypeParameterId(0)).type_parameter_count(),
            1,
        );
        assert_eq!(
            HostValueType::Parameter(crate::plan::TypeParameterId(2)).type_parameter_count(),
            3,
        );
        assert_eq!(HostValueType::Int.type_parameter_count(), 0);
        assert_eq!(HostValueType::Float.type_parameter_count(), 0);
        assert_eq!(HostValueType::String.type_parameter_count(), 0);
        assert_eq!(HostValueType::BitArray.type_parameter_count(), 0);
        assert_eq!(HostValueType::UtfCodepoint.type_parameter_count(), 0);
        assert_eq!(HostValueType::Bool.type_parameter_count(), 0);
        assert_eq!(HostValueType::Nil.type_parameter_count(), 0);
    }
}
