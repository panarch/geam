use crate::plan::{FunctionReturnFamily, ValueType};
use ecow::EcoString;
use std::fmt;

pub(crate) type ExecutionResult<T> = Result<T, ExecutionError>;

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum ExecutionError {
    #[error("{0}")]
    Panic(PanicKind),
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PanicKind {
    Panic { message: Option<EcoString> },
    Todo { message: Option<EcoString> },
    LetAssert { message: Option<EcoString> },
    EmptyFunction,
    EmptyBlock,
    IncompleteUse,
}

impl fmt::Display for PanicKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PanicKind::Panic { message } => write_panic_message(f, "panic", message.as_deref()),
            PanicKind::Todo { message } => write_panic_message(f, "todo", message.as_deref()),
            PanicKind::LetAssert { message } => {
                write_panic_message(f, "let assert", message.as_deref())
            }
            PanicKind::EmptyFunction => f.write_str("empty function"),
            PanicKind::EmptyBlock => f.write_str("empty block"),
            PanicKind::IncompleteUse => f.write_str("incomplete use"),
        }
    }
}

impl std::error::Error for PanicKind {}

fn write_panic_message(
    f: &mut fmt::Formatter<'_>,
    kind: &str,
    message: Option<&str>,
) -> fmt::Result {
    match message {
        Some(message) => write!(f, "{kind} as {message:?}"),
        None => f.write_str(kind),
    }
}

impl ExecutionError {
    pub(crate) fn panic(kind: PanicKind) -> Self {
        Self::Panic(kind)
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
}

#[cfg(test)]
mod tests {
    use super::{ExecutionError, PanicKind};
    use crate::plan::{FunctionReturnFamily, ValueType};

    #[test]
    fn panic_display() {
        for (kind, expected) in [
            (PanicKind::Panic { message: None }, "panic"),
            (
                PanicKind::Panic {
                    message: Some("boom".into()),
                },
                "panic as \"boom\"",
            ),
            (PanicKind::Todo { message: None }, "todo"),
            (
                PanicKind::Todo {
                    message: Some("later".into()),
                },
                "todo as \"later\"",
            ),
            (PanicKind::LetAssert { message: None }, "let assert"),
            (
                PanicKind::LetAssert {
                    message: Some("not empty".into()),
                },
                "let assert as \"not empty\"",
            ),
            (PanicKind::EmptyFunction, "empty function"),
            (PanicKind::EmptyBlock, "empty block"),
            (PanicKind::IncompleteUse, "incomplete use"),
        ] {
            assert_eq!(ExecutionError::panic(kind).to_string(), expected);
        }
    }

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
}
