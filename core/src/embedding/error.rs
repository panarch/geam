use crate::ExecutionError;
use thiserror::Error;

/// A failure while calling a previously bound function.
#[derive(Debug, Error, Clone, PartialEq)]
pub enum CallError {
    #[error("the function belongs to a different embedding module")]
    ForeignFunction,
    #[error(transparent)]
    Execution(#[from] ExecutionError),
}

#[cfg(test)]
mod tests {
    use super::CallError;
    use crate::{ExecutionError, PanicKind, PanicSite, SourceSpan};

    #[test]
    fn displays_a_foreign_function_owner() {
        let error = CallError::ForeignFunction;

        assert_eq!(
            error.to_string(),
            "the function belongs to a different embedding module",
        );
        assert_eq!(error.clone(), error);
    }

    #[test]
    fn transparently_displays_a_source_execution_failure() {
        let execution = ExecutionError::source_panic(
            None,
            PanicKind::Panic,
            Some("stopped".into()),
            PanicSite::new("library".into(), "explode".into(), SourceSpan::new(44, 62)),
        );
        let error = CallError::Execution(execution.clone());

        assert_eq!(error.to_string(), "panic: stopped");
        assert_eq!(error, CallError::from(execution));
    }
}
