use crate::plan::LocalId;
use ecow::EcoString;
use thiserror::Error;

#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum RuntimeError {
    #[error("missing function: {name}")]
    MissingFunction { name: EcoString },

    #[error("function {name} expected {expected} arguments, got {got}")]
    ArityMismatch {
        name: EcoString,
        expected: usize,
        got: usize,
    },

    #[error("unbound local: {local:?}")]
    UnboundLocal { local: LocalId },

    #[error("expected {expected}, got {actual}")]
    TypeMismatch {
        expected: &'static str,
        actual: &'static str,
    },
}
