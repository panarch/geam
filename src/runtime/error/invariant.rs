use crate::plan::execution::FunctionReturnFamily;
use crate::plan::{CustomType, ValueType};
use ecow::EcoString;

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum InvariantError {
    #[error("function return family mismatch (expected {expected}, got {actual})")]
    FunctionReturnFamilyMismatch {
        expected: FunctionReturnFamily,
        actual: FunctionReturnFamily,
    },
    #[error("tuple index family mismatch (expected {expected:?}, got {actual:?})")]
    TupleIndexFamilyMismatch {
        expected: ValueType,
        actual: ValueType,
    },
    #[error(
        "custom field family mismatch in {custom_type:?}::{constructor} field {field_index} (expected {expected:?}, got {actual:?})"
    )]
    CustomFieldFamilyMismatch {
        custom_type: CustomType,
        constructor: EcoString,
        field_index: usize,
        expected: ValueType,
        actual: ValueType,
    },
    #[error("list index out of bounds for {item_type:?} list (index {index}, length {length})")]
    ListIndexOutOfBounds {
        item_type: ValueType,
        index: usize,
        length: usize,
    },
}

#[cfg(test)]
mod tests {
    use super::InvariantError;
    use crate::plan::execution::FunctionReturnFamily;
    use crate::plan::{CustomType, CustomTypeName, ValueType};

    #[test]
    fn function_return_family_mismatch_display() {
        let error = InvariantError::FunctionReturnFamilyMismatch {
            expected: FunctionReturnFamily::Int,
            actual: FunctionReturnFamily::String,
        };

        assert_eq!(
            error.to_string(),
            "function return family mismatch (expected Int, got String)",
        );
    }

    #[test]
    fn tuple_index_family_mismatch_display() {
        let error = InvariantError::TupleIndexFamilyMismatch {
            expected: ValueType::Tuple(vec![ValueType::Int]),
            actual: ValueType::String,
        };

        assert_eq!(
            error.to_string(),
            "tuple index family mismatch (expected Tuple([Int]), got String)",
        );
    }

    #[test]
    fn list_index_out_of_bounds_display() {
        let error = InvariantError::ListIndexOutOfBounds {
            item_type: ValueType::Int,
            index: 1,
            length: 1,
        };

        assert_eq!(
            error.to_string(),
            "list index out of bounds for Int list (index 1, length 1)",
        );
    }

    #[test]
    fn custom_field_family_mismatch_display() {
        let error = InvariantError::CustomFieldFamilyMismatch {
            custom_type: CustomType::new(
                CustomTypeName::new("app".into(), "main".into(), "Box".into()),
                vec![ValueType::Int],
            ),
            constructor: "Box".into(),
            field_index: 0,
            expected: ValueType::Int,
            actual: ValueType::String,
        };

        assert_eq!(
            error.to_string(),
            "custom field family mismatch in CustomType { name: CustomTypeName { package: \"app\", module: \"main\", name: \"Box\" }, arguments: [Int] }::Box field 0 (expected Int, got String)",
        );
    }
}
