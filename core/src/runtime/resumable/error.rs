use crate::plan::execution::runtime::{OwnedRuntimeValueMetadata, RuntimeExecutionPlan};
use crate::runtime::{
    EvaluatedValue, HostCallOrigin, InvariantError, TransferValues, transfer::TransferListStorage,
};
use crate::{
    BitArraySegmentPanicReason, ExecutionError, HostFailure, PanicKind, SourceContext, SourceSpan,
};
use ecow::EcoString;
use std::fmt::{self, Display, Formatter};

pub(in crate::runtime) type TransferExecutionResult<Value> = Result<Value, TransferExecutionError>;

pub(crate) struct TransferExecutionError(TransferExecutionErrorKind);

enum TransferExecutionErrorKind {
    Invariant(Box<InvariantError>),
    Host(Box<crate::HostError>),
    SourcePanic(Box<ResumableSourcePanic>),
    LetAssertPanic(Box<RetainedAssertionPanic>),
    BitArraySegmentPanic(Box<ResumableBitArrayPanic>),
}

struct ResumableBitArrayPanic {
    source: Option<SourceContext>,
    reason: BitArraySegmentPanicReason,
    site: crate::PanicSite,
}

struct ResumableSourcePanic {
    source: Option<SourceContext>,
    kind: PanicKind,
    message: Option<EcoString>,
    site: crate::PanicSite,
}

struct RetainedAssertionPanic {
    source: Option<SourceContext>,
    message: Option<EcoString>,
    site: crate::PanicSite,
    subject: EvaluatedValue<TransferValues>,
    pattern_span: SourceSpan,
    metadata: OwnedRuntimeValueMetadata,
    lists: TransferListStorage,
}

impl TransferExecutionError {
    pub(in crate::runtime) fn source_panic(
        source: Option<&SourceContext>,
        kind: PanicKind,
        message: Option<EcoString>,
        site: crate::PanicSite,
    ) -> Self {
        Self(TransferExecutionErrorKind::SourcePanic(Box::new(
            ResumableSourcePanic {
                source: source.cloned(),
                kind,
                message,
                site,
            },
        )))
    }

    pub(in crate::runtime) fn let_assert_panic<Plan>(
        plan: &Plan,
        lists: &TransferListStorage,
        source: Option<&SourceContext>,
        message: Option<EcoString>,
        site: crate::PanicSite,
        subject: EvaluatedValue<TransferValues>,
        pattern_span: SourceSpan,
    ) -> Self
    where
        Plan: RuntimeExecutionPlan,
    {
        Self(TransferExecutionErrorKind::LetAssertPanic(Box::new(
            RetainedAssertionPanic {
                source: source.cloned(),
                message,
                site,
                subject,
                pattern_span,
                metadata: plan.value_metadata().to_owned(),
                lists: lists.clone(),
            },
        )))
    }

    pub(in crate::runtime) fn bit_array_segment_panic(
        source: Option<&SourceContext>,
        reason: BitArraySegmentPanicReason,
        site: crate::PanicSite,
    ) -> Self {
        Self(TransferExecutionErrorKind::BitArraySegmentPanic(Box::new(
            ResumableBitArrayPanic {
                source: source.cloned(),
                reason,
                site,
            },
        )))
    }

    pub(in crate::runtime) fn host_failure(
        plan: &impl RuntimeExecutionPlan,
        origin: HostCallOrigin,
        function: &crate::plan::execution::host::HostedFunctionMetadata,
        failure: HostFailure,
    ) -> Self {
        let error = match origin.into_source_site(function.site()) {
            Ok(site) => crate::HostError::new(
                function.package().clone(),
                function.module().clone(),
                function.name().clone(),
                function.signature().clone(),
                failure,
                site.clone(),
                plan.source_context_for(site.module()),
            ),
            Err(caller) => crate::HostError::new_from_host(
                function.package().clone(),
                function.module().clone(),
                function.name().clone(),
                function.signature().clone(),
                failure,
                caller,
            ),
        };
        Self(TransferExecutionErrorKind::Host(Box::new(error)))
    }

    pub(crate) fn into_execution(self) -> ExecutionError {
        match self.0 {
            TransferExecutionErrorKind::Invariant(error) => ExecutionError::Invariant(*error),
            TransferExecutionErrorKind::Host(error) => ExecutionError::Host(error),
            TransferExecutionErrorKind::SourcePanic(panic) => {
                let ResumableSourcePanic {
                    source,
                    kind,
                    message,
                    site,
                } = *panic;
                ExecutionError::source_panic(source.as_ref(), kind, message, site)
            }
            TransferExecutionErrorKind::LetAssertPanic(panic) => {
                let RetainedAssertionPanic {
                    source,
                    message,
                    site,
                    subject,
                    pattern_span,
                    metadata,
                    lists,
                } = *panic;
                let subject =
                    crate::runtime::materialize::value(metadata.as_borrowed(), &lists, subject);
                ExecutionError::let_assert_panic(
                    source.as_ref(),
                    message,
                    site,
                    subject,
                    pattern_span,
                )
            }
            TransferExecutionErrorKind::BitArraySegmentPanic(panic) => {
                let ResumableBitArrayPanic {
                    source,
                    reason,
                    site,
                } = *panic;
                ExecutionError::bit_array_segment_panic(source.as_ref(), reason, site)
            }
        }
    }

    fn panic_display(kind: PanicKind, message: &Option<EcoString>) -> String {
        let message = crate::runtime::error::PanicMessage::from_optional_explicit(message.clone());
        format!("{}: {}", kind.code(), message.text(kind))
    }
}

impl From<InvariantError> for TransferExecutionError {
    fn from(error: InvariantError) -> Self {
        Self(TransferExecutionErrorKind::Invariant(Box::new(error)))
    }
}

impl Display for TransferExecutionError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        match &self.0 {
            TransferExecutionErrorKind::Invariant(error) => Display::fmt(error, formatter),
            TransferExecutionErrorKind::Host(error) => Display::fmt(error, formatter),
            TransferExecutionErrorKind::SourcePanic(panic) => {
                formatter.write_str(&Self::panic_display(panic.kind, &panic.message))
            }
            TransferExecutionErrorKind::LetAssertPanic(panic) => {
                formatter.write_str(&Self::panic_display(PanicKind::LetAssert, &panic.message))
            }
            TransferExecutionErrorKind::BitArraySegmentPanic(_) => {
                formatter.write_str(&Self::panic_display(PanicKind::BitArraySegment, &None))
            }
        }
    }
}

impl fmt::Debug for TransferExecutionError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        formatter
            .debug_tuple("TransferExecutionError")
            .field(&self.to_string())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::{TransferExecutionError, TransferExecutionErrorKind};
    use crate::plan::{FunctionType, HostCallSite, PanicSite, SourceSpan, ValueType};
    use crate::runtime::transfer::TransferListStorage;
    use crate::runtime::{EvaluatedValue, InvariantError};
    use crate::{BitArraySegmentPanicReason, ExecutionError, HostFailure, PanicKind, Value};

    fn assert_preserved(error: TransferExecutionError, expected: ExecutionError) {
        let display = error.to_string();
        assert_eq!(
            format!("{error:?}"),
            format!("TransferExecutionError({display:?})"),
        );

        let actual = error.into_execution();
        assert_eq!(actual.to_string(), display);
        assert_eq!(actual, expected);
    }

    #[test]
    fn transfer_execution_error_is_send_without_requiring_sync() {
        fn require_send<Value: Send>() {}

        require_send::<TransferExecutionError>();
    }

    #[test]
    fn transfer_errors_preserve_every_execution_error_variant() {
        let invariant = InvariantError::ListIndexOutOfBounds {
            item_type: ValueType::Int,
            index: 1,
            length: 1,
        };
        assert_preserved(
            invariant.clone().into(),
            ExecutionError::Invariant(invariant),
        );

        let host = crate::HostError::new(
            "application".into(),
            "native".into(),
            "fail".into(),
            FunctionType::new(vec![ValueType::Int], ValueType::Int),
            HostFailure::new("stopped"),
            HostCallSite::new("main".into(), "run".into(), SourceSpan::new(10, 17)),
            None,
        );
        assert_preserved(
            TransferExecutionError(TransferExecutionErrorKind::Host(Box::new(host.clone()))),
            ExecutionError::Host(Box::new(host)),
        );

        let source_site = PanicSite::new("main".into(), "run".into(), SourceSpan::new(20, 25));
        assert_preserved(
            TransferExecutionError::source_panic(
                None,
                PanicKind::Todo,
                Some("unfinished".into()),
                source_site.clone(),
            ),
            ExecutionError::source_panic(
                None,
                PanicKind::Todo,
                Some("unfinished".into()),
                source_site,
            ),
        );

        let plan = crate::runtime::plan_src("pub fn main() { Nil }");
        let lists = TransferListStorage::default();
        let let_assert_site = PanicSite::new("main".into(), "run".into(), SourceSpan::new(30, 40));
        let pattern_span = SourceSpan::new(34, 40);
        assert_preserved(
            TransferExecutionError::let_assert_panic(
                &plan,
                &lists,
                None,
                Some("expected one".into()),
                let_assert_site.clone(),
                EvaluatedValue::Int(2.into()),
                pattern_span,
            ),
            ExecutionError::let_assert_panic(
                None,
                Some("expected one".into()),
                let_assert_site,
                Value::Int(2.into()),
                pattern_span,
            ),
        );

        let segment_site = PanicSite::new("main".into(), "run".into(), SourceSpan::new(50, 60));
        let reason = BitArraySegmentPanicReason::InsufficientBits {
            requested: 8,
            available: 4,
        };
        assert_preserved(
            TransferExecutionError::bit_array_segment_panic(
                None,
                reason.clone(),
                segment_site.clone(),
            ),
            ExecutionError::bit_array_segment_panic(None, reason, segment_site),
        );
    }
}
