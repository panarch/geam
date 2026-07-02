use crate::plan::{FunctionReturnFamily, ValueType};

pub(crate) type ExecutionResult<T> = Result<T, ExecutionError>;

// Adding a new invariant kind is a design change, not a local runtime fix.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[error("execution invariant failed: {kind}")]
pub struct ExecutionError {
    kind: ExecutionErrorKind,
}

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
enum ExecutionErrorKind {
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
}

impl ExecutionError {
    pub(crate) fn function_return_family_mismatch(
        expected: FunctionReturnFamily,
        actual: FunctionReturnFamily,
    ) -> Self {
        Self {
            kind: ExecutionErrorKind::FunctionReturnFamilyMismatch { expected, actual },
        }
    }

    pub(crate) fn tuple_index_family_mismatch(expected: ValueType, actual: ValueType) -> Self {
        Self {
            kind: ExecutionErrorKind::TupleIndexFamilyMismatch { expected, actual },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ExecutionError;
    use crate::plan::{FunctionReturnFamily, ValueType};

    #[test]
    fn function_return_family_mismatch_display() {
        let error = ExecutionError::function_return_family_mismatch(
            FunctionReturnFamily::Int,
            FunctionReturnFamily::String,
        );

        assert_eq!(
            error.to_string(),
            "execution invariant failed: function return family mismatch (expected Int, got String)",
        );
    }

    #[test]
    fn tuple_index_family_mismatch_display() {
        let error = ExecutionError::tuple_index_family_mismatch(
            ValueType::Tuple(vec![ValueType::Int]),
            ValueType::String,
        );

        assert_eq!(
            error.to_string(),
            "execution invariant failed: tuple index family mismatch (expected Tuple([Int]), got String)",
        );
    }
}
