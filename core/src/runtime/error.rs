mod diagnostic;
mod host;
mod invariant;
mod panic;

use crate::plan::{PanicSite, SourceContext, SourceSpan};
use crate::runtime::Value;
use ecow::EcoString;

pub(crate) use self::host::HostCallOrigin;
pub use self::host::{HostError, HostLocation, HostOrigin};
pub use self::invariant::InvariantError;
pub use self::panic::{BitArraySegmentPanicReason, Panic, PanicDetails, PanicKind, PanicMessage};

#[derive(Debug, Clone, PartialEq, thiserror::Error)]
pub enum ExecutionError {
    #[error("{0}")]
    Panic(Panic),
    #[error("{0}")]
    Invariant(InvariantError),
    #[error("{0}")]
    Host(Box<HostError>),
}

pub(crate) type ExecutionResult<T> = Result<T, ExecutionError>;

impl ExecutionError {
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
        let error = ExecutionError::Invariant(invariant);

        assert_eq!(
            error.to_string(),
            "function return family mismatch (expected Int, got String)",
        );
    }
}
