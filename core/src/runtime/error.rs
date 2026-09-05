mod diagnostic;
mod host;
mod invariant;
mod panic;
mod subject;

use crate::plan::{PanicSite, SourceContext, SourceSpan};
use crate::runtime::Value;
use ecow::EcoString;

pub(crate) use self::host::HostCallOrigin;
pub use self::host::{HostError, HostLocation, HostOrigin};
pub use self::invariant::InvariantError;
pub use self::panic::{BitArraySegmentPanicReason, Panic, PanicDetails, PanicKind, PanicMessage};
pub use subject::AsyncPanicValue;

/// An execution failure whose retained assertion value can move between workers.
pub type AsyncExecutionError = ExecutionError<AsyncPanicValue>;

#[derive(Debug, Clone, PartialEq, thiserror::Error)]
pub enum ExecutionError<Subject = Value> {
    #[error("{0}")]
    Panic(Panic<Subject>),
    #[error("{0}")]
    Invariant(InvariantError),
    #[error("{0}")]
    Host(Box<HostError>),
}

pub(crate) type ExecutionResult<T> = Result<T, ExecutionError>;

impl<Subject> From<InvariantError> for ExecutionError<Subject> {
    fn from(error: InvariantError) -> Self {
        Self::Invariant(error)
    }
}

impl<Subject> ExecutionError<Subject> {
    pub(crate) fn from_host_call(
        function: &crate::plan::execution::host::HostedFunctionMetadata,
        site: crate::plan::HostCallSite,
        source_context: Option<&SourceContext>,
        failure: crate::HostFailure,
    ) -> Self {
        Self::Host(Box::new(HostError::new(
            function.package().clone(),
            function.module().clone(),
            function.name().clone(),
            function.signature().clone(),
            failure,
            site,
            source_context,
        )))
    }

    pub(crate) fn from_host_origin(
        function: &crate::plan::execution::host::HostedFunctionMetadata,
        caller: HostOrigin,
        failure: crate::HostFailure,
    ) -> Self {
        Self::Host(Box::new(HostError::new_from_host(
            function.package().clone(),
            function.module().clone(),
            function.name().clone(),
            function.signature().clone(),
            failure,
            caller,
        )))
    }

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
        value: Subject,
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

    pub(crate) fn bit_array_segment_panic(
        source_context: Option<&SourceContext>,
        reason: BitArraySegmentPanicReason,
        site: PanicSite,
    ) -> Self {
        Self::Panic(Panic::new(
            PanicKind::BitArraySegment,
            PanicMessage::Default,
            site,
            source_context,
            Some(PanicDetails::BitArraySegment { reason }),
        ))
    }
}

impl AsyncExecutionError {
    /// Materializes a local diagnostic value when the caller needs the synchronous form.
    ///
    /// The returned error preserves the source, provider and panic details, but
    /// its general-purpose assertion value may no longer be `Send`.
    pub fn into_local(self) -> ExecutionError {
        match self {
            Self::Panic(panic) => ExecutionError::Panic(panic.into_local()),
            Self::Host(error) => ExecutionError::Host(error),
            Self::Invariant(error) => ExecutionError::Invariant(error),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{ExecutionError, InvariantError};
    use crate::plan::execution::function::FunctionReturnFamily;

    #[test]
    fn invariant_display_delegates_to_invariant_error() {
        let invariant = InvariantError::FunctionReturnFamilyMismatch {
            expected: FunctionReturnFamily::Int,
            actual: FunctionReturnFamily::String,
        };
        let error: ExecutionError = ExecutionError::Invariant(invariant);

        assert_eq!(
            error.to_string(),
            "function return family mismatch (expected Int, got String)",
        );
    }
}
