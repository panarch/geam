mod diagnostic;
mod panic;

use crate::plan::execution::FunctionReturnFamily;
use crate::plan::{PanicSite, SourceContext, SourceSpan, ValueType};
use crate::runtime::Value;
use ecow::EcoString;

pub use self::panic::{Panic, PanicDetails, PanicKind, PanicMessage};

pub(crate) type ExecutionResult<T> = Result<T, ExecutionError>;

#[derive(Debug, Clone, PartialEq, thiserror::Error)]
pub enum ExecutionError {
    #[error("{0}")]
    Panic(Panic),
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
    #[error("list index out of bounds for {item_type:?} list (index {index}, length {length})")]
    ListIndexOutOfBounds {
        item_type: ValueType,
        index: usize,
        length: usize,
    },
}

impl ExecutionError {
    pub(crate) fn source_panic(
        source_context: Option<&SourceContext>,
        kind: PanicKind,
        message: Option<EcoString>,
        site: PanicSite,
    ) -> Self {
        Self::Panic(Panic::new(
            kind,
            PanicMessage::from_optional_explicit(message),
            site,
            source_context,
            None,
        ))
    }

    pub(crate) fn let_assert_panic(
        source_context: Option<&SourceContext>,
        message: Option<EcoString>,
        site: PanicSite,
        value: Value,
        pattern_span: SourceSpan,
    ) -> Self {
        Self::Panic(Panic::new(
            PanicKind::LetAssert,
            PanicMessage::from_optional_explicit(message),
            site,
            source_context,
            Some(PanicDetails::LetAssert {
                value,
                pattern_span,
            }),
        ))
    }

    pub(crate) fn function_return_family_mismatch(
        expected: FunctionReturnFamily,
        actual: FunctionReturnFamily,
    ) -> Self {
        Self::FunctionReturnFamilyMismatch { expected, actual }
    }

    pub(crate) fn tuple_index_family_mismatch(expected: ValueType, actual: ValueType) -> Self {
        Self::TupleIndexFamilyMismatch { expected, actual }
    }

    pub(crate) fn list_index_out_of_bounds(
        item_type: ValueType,
        index: usize,
        length: usize,
    ) -> Self {
        Self::ListIndexOutOfBounds {
            item_type,
            index,
            length,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ExecutionError;
    use crate::plan::ValueType;
    use crate::plan::execution::FunctionReturnFamily;

    #[test]
    fn function_return_family_mismatch_display() {
        let error = ExecutionError::function_return_family_mismatch(
            FunctionReturnFamily::Int,
            FunctionReturnFamily::String,
        );

        assert_eq!(
            error.to_string(),
            "function return family mismatch (expected Int, got String)",
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
            "tuple index family mismatch (expected Tuple([Int]), got String)",
        );
    }

    #[test]
    fn list_index_out_of_bounds_display() {
        let error = ExecutionError::list_index_out_of_bounds(ValueType::Int, 1, 1);

        assert_eq!(
            error.to_string(),
            "list index out of bounds for Int list (index 1, length 1)",
        );
    }
}
