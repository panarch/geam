use crate::ExecutionError;
use thiserror::Error;

/// A failure while calling a previously bound function.
#[derive(Debug, Error, Clone, PartialEq)]
pub enum CallError<Subject = crate::Value> {
    #[error("the function belongs to a different embedding module")]
    ForeignFunction,
    #[error("the retained value belongs to a different embedding module")]
    ForeignValue,
    #[error(transparent)]
    Execution(#[from] ExecutionError<Subject>),
}

/// A bound-call failure that preserves worker-transferable assertion values.
pub type AsyncCallError = CallError<crate::AsyncPanicValue>;

impl AsyncCallError {
    /// Converts the diagnostic to the local error representation on request.
    pub fn into_local(self) -> CallError {
        match self {
            Self::ForeignFunction => CallError::ForeignFunction,
            Self::ForeignValue => CallError::ForeignValue,
            Self::Execution(error) => CallError::Execution(error.into_local()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{AsyncCallError, CallError};
    use crate::{ExecutionError, PanicKind, PanicSite, SourceSpan};

    #[test]
    fn displays_a_foreign_function_owner() {
        let error: CallError = CallError::ForeignFunction;

        assert_eq!(
            error.to_string(),
            "the function belongs to a different embedding module",
        );
        assert_eq!(error.clone(), error);

        let error: CallError = CallError::ForeignValue;
        assert_eq!(
            error.to_string(),
            "the retained value belongs to a different embedding module"
        );
        assert_eq!(error.clone(), error);
    }

    #[test]
    fn transparently_displays_a_source_execution_failure() {
        let execution: ExecutionError = ExecutionError::source_panic(
            None,
            PanicKind::Panic,
            Some("stopped".into()),
            PanicSite::new("library".into(), "explode".into(), SourceSpan::new(44, 62)),
        );
        let error = CallError::Execution(execution.clone());

        assert_eq!(error.to_string(), "panic: stopped");
        assert_eq!(error, CallError::from(execution));
    }

    #[test]
    fn transferable_call_errors_keep_the_same_ownership_and_execution_failures() {
        for (error, expected) in [
            (AsyncCallError::ForeignFunction, CallError::ForeignFunction),
            (AsyncCallError::ForeignValue, CallError::ForeignValue),
            (
                AsyncCallError::from(crate::AsyncExecutionError::source_panic(
                    None,
                    PanicKind::Panic,
                    Some("stopped".into()),
                    PanicSite::unknown(),
                )),
                CallError::Execution(ExecutionError::source_panic(
                    None,
                    PanicKind::Panic,
                    Some("stopped".into()),
                    PanicSite::unknown(),
                )),
            ),
        ] {
            assert_eq!(error.to_string(), expected.to_string());
            assert_eq!(error, error.clone());
            let transferred = std::thread::spawn(move || error)
                .join()
                .expect("error worker");
            assert_eq!(transferred.into_local(), expected);
        }
    }
}
