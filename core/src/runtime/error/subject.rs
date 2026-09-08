use crate::plan::execution::runtime::{OwnedRuntimeValueMetadata, RuntimeExecutionPlan};
use crate::runtime::{
    EvaluatedValue, LocalValues, RuntimeValueProfile, TransferListStorage, TransferValues, Value,
};
use std::fmt;

/// The retained failed value of an async `let assert`.
///
/// It owns the value's lifetime without borrowing the execution or requiring
/// thread-local storage. Inspecting a diagnostic does not consume that lifetime.
#[derive(Clone)]
pub struct AsyncPanicValue {
    subject: EvaluatedValue<TransferValues>,
    metadata: OwnedRuntimeValueMetadata,
    lists: TransferListStorage,
}

impl AsyncPanicValue {
    pub(in crate::runtime) fn new(
        plan: &impl RuntimeExecutionPlan,
        lists: &TransferListStorage,
        subject: EvaluatedValue<TransferValues>,
    ) -> Self {
        Self {
            subject,
            metadata: plan.value_metadata().to_owned(),
            lists: lists.clone(),
        }
    }

    /// Materializes an independent local diagnostic view of the failed value.
    ///
    /// This is explicit diagnostic inspection, not the async call's ordinary
    /// value conversion. The returned [`Value`] need not be `Send`.
    pub fn to_value(&self) -> Value {
        crate::runtime::materialize::value(
            self.metadata.as_borrowed(),
            &self.lists,
            self.subject.clone(),
        )
    }

    pub(super) fn into_value(self) -> Value {
        crate::runtime::materialize::value(self.metadata.as_borrowed(), &self.lists, self.subject)
    }
}

pub(in crate::runtime) trait PanicSubjectProfile: RuntimeValueProfile {
    fn panic_subject(
        plan: &impl RuntimeExecutionPlan,
        lists: &Self::ListStorage,
        value: EvaluatedValue<Self>,
    ) -> Self::PanicSubject;
}

impl PanicSubjectProfile for LocalValues {
    fn panic_subject(
        plan: &impl RuntimeExecutionPlan,
        lists: &Self::ListStorage,
        value: EvaluatedValue<Self>,
    ) -> Value {
        crate::runtime::materialize::value(plan.value_metadata(), lists, value)
    }
}

impl PanicSubjectProfile for TransferValues {
    fn panic_subject(
        plan: &impl RuntimeExecutionPlan,
        lists: &Self::ListStorage,
        value: EvaluatedValue<Self>,
    ) -> AsyncPanicValue {
        AsyncPanicValue::new(plan, lists, value)
    }
}

impl fmt::Debug for AsyncPanicValue {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.to_value(), formatter)
    }
}

impl PartialEq for AsyncPanicValue {
    fn eq(&self, other: &Self) -> bool {
        self.to_value() == other.to_value()
    }
}

#[cfg(test)]
mod tests {
    use super::{AsyncPanicValue, PanicSubjectProfile};
    use crate::plan::{FunctionType, ValueType};
    use crate::runtime::{EvaluatedValue, TransferListStorage, TransferValues, Value};
    use crate::{
        AsyncExecutionError, BitArraySegmentPanicReason, ExecutionError, HostCallSite, HostError,
        HostFailure, InvariantError, PanicKind, PanicSite, SourceContext, SourceSpan,
    };
    use miette::Diagnostic;

    fn subject(value: i64) -> AsyncPanicValue {
        let plan = crate::runtime::plan_src("pub fn main() { Nil }");
        <TransferValues as PanicSubjectProfile>::panic_subject(
            &plan,
            &TransferListStorage::default(),
            EvaluatedValue::Int(value.into()),
        )
    }

    #[test]
    fn inspection_keeps_the_transferable_subject_owned_and_can_materialize_a_local_view() {
        let value = subject(2);
        let copy = value.clone();
        assert_eq!(value, copy);
        assert_ne!(value, subject(3));
        assert_eq!(format!("{value:?}"), "Int(2)");
        assert_eq!(value.to_value(), Value::Int(2.into()));
        drop(value);
        let local = std::thread::spawn(move || {
            assert_eq!(copy.to_value(), Value::Int(2.into()));
            copy
        })
        .join()
        .expect("transferable retained subject");
        assert_eq!(local.into_value(), Value::Int(2.into()));
    }

    #[test]
    fn transferable_errors_preserve_source_diagnostics_and_every_failure_domain() {
        let source = SourceContext::new("src/main.gleam", "pub fn main() { let assert 1 = 2 }");
        let site = PanicSite::new("main".into(), "main".into(), SourceSpan::new(16, 32));
        let assertion = AsyncExecutionError::let_assert_panic(
            Some(&source),
            Some("expected one".into()),
            site.clone(),
            subject(2),
            SourceSpan::new(27, 28),
        );
        assert_eq!(assertion.to_string(), "let_assert: expected one");
        assert_eq!(
            assertion.code().expect("code").to_string(),
            "geam::let_assert"
        );
        assert_eq!(
            assertion.help().expect("help").to_string(),
            "failed value: Int(2)"
        );
        assert!(assertion.source_code().is_some());
        let labels = assertion.labels().expect("labels").collect::<Vec<_>>();
        assert_eq!(
            labels
                .iter()
                .map(|label| (label.label(), label.offset(), label.len()))
                .collect::<Vec<_>>(),
            [
                (Some("let assert in main.main"), 16, 16),
                (Some("pattern"), 27, 1),
            ]
        );
        assert_eq!(assertion, assertion.clone());
        assert!(format!("{assertion:?}").contains("value: Int(2)"));
        let transferred = std::thread::spawn(move || assertion)
            .join()
            .expect("error worker");
        assert_eq!(
            transferred.into_local(),
            ExecutionError::let_assert_panic(
                Some(&source),
                Some("expected one".into()),
                site.clone(),
                Value::Int(2.into()),
                SourceSpan::new(27, 28),
            )
        );

        for context in [None, Some(&source)] {
            let error = AsyncExecutionError::source_panic(
                context,
                PanicKind::Todo,
                Some("unfinished".into()),
                site.clone(),
            );
            assert_eq!(error.code().expect("code").to_string(), "geam::todo");
            assert!(error.help().is_none());
            assert_eq!(error.source_code().is_some(), context.is_some());
            assert_eq!(
                error.labels().map(|labels| labels.count()),
                context.map(|_| 1)
            );
            assert_eq!(
                error.into_local(),
                ExecutionError::source_panic(
                    context,
                    PanicKind::Todo,
                    Some("unfinished".into()),
                    site.clone()
                )
            );
        }

        for (reason, help) in [
            (
                BitArraySegmentPanicReason::InvalidFloatSize { bit_size: 8.into() },
                "float segments must be 16, 32, or 64 bits; evaluated size was 8 bits",
            ),
            (
                BitArraySegmentPanicReason::InsufficientBits {
                    requested: 8,
                    available: 4,
                },
                "sized bits segment requested 8 bits, but the value contains 4 bits",
            ),
            (
                BitArraySegmentPanicReason::SizeOutOfRange { bit_size: 1.into() },
                "BitArray segment size 1 exceeds the supported host range",
            ),
        ] {
            let error = AsyncExecutionError::bit_array_segment_panic(
                Some(&source),
                reason.clone(),
                site.clone(),
            );
            assert_eq!(error.help().expect("segment help").to_string(), help);
            assert_eq!(error.labels().expect("segment labels").count(), 1);
            assert_eq!(
                error.into_local(),
                ExecutionError::bit_array_segment_panic(Some(&source), reason, site.clone())
            );
        }

        let invariant = InvariantError::ListIndexOutOfBounds {
            item_type: ValueType::Int,
            index: 1,
            length: 1,
        };
        let error = AsyncExecutionError::from(invariant.clone());
        assert_eq!(
            error.code().expect("code").to_string(),
            "geam::list_index_out_of_bounds"
        );
        assert!(error.help().is_none());
        assert!(error.source_code().is_none());
        assert!(error.labels().is_none());
        assert_eq!(error.to_string(), invariant.to_string());
        assert_eq!(error.into_local(), ExecutionError::Invariant(invariant));

        let host = HostError::new(
            "application".into(),
            "native".into(),
            "fail".into(),
            FunctionType::new(vec![ValueType::Int], ValueType::Int),
            HostFailure::new("stopped"),
            HostCallSite::new("main".into(), "main".into(), SourceSpan::new(16, 32)),
            Some(&source),
        );
        let error = AsyncExecutionError::Host(Box::new(host.clone()));
        assert_eq!(
            error.code().expect("code").to_string(),
            "geam::host_function"
        );
        assert!(error.help().is_none());
        assert!(error.source_code().is_some());
        assert_eq!(error.labels().expect("host labels").count(), 1);
        assert_eq!(error.to_string(), host.to_string());
        assert_eq!(error.into_local(), ExecutionError::Host(Box::new(host)));
    }
}
