use super::{MailboxMatch, Sleep, next_selected, start, timed_selected};
use crate::GleamErlangHostProfile;
use crate::execution::Scan;
use geam_core::execution::ExecutionUnitId;
use geam_core::host::native::NativeValues;
use geam_core::host::{
    HostCall, HostCallError, HostExecutionContext, HostExecutionError, HostProfile, HostProvider,
    HostType, HostTypeSequence,
};
use geam_core::provider::advanced::{NativeKind, NativeValue};
use std::marker::PhantomData;
use std::time::Instant;

/// A selective receive whose record fields are projected before consumption.
///
/// The projection runs once per matching tag during the bounded mailbox scan.
/// Returning `None` leaves the message queued. The selected output is returned
/// unchanged; retained native fields preserve their original source types.
pub struct RecordReceive<Profile: GleamErlangHostProfile, Output> {
    pid: ExecutionUnitId,
    filter: RecordFields<Output>,
    scan: Scan,
    timeout: Option<Sleep>,
    profile: PhantomData<fn() -> Profile>,
}

struct RecordFields<Output> {
    tag: NativeValue,
    project: fn(&NativeValue) -> Option<Output>,
}

impl<Output> Clone for RecordFields<Output> {
    fn clone(&self) -> Self {
        Self {
            tag: self.tag.clone(),
            project: self.project,
        }
    }
}

impl<Profile: HostProfile, Output: Send + 'static> MailboxMatch<Profile> for RecordFields<Output> {
    type Output = Output;

    fn select(
        &self,
        values: NativeValues<'_>,
        value: NativeValue,
    ) -> Result<Option<Output>, HostCallError> {
        Ok(match (value.kind(), value.index(0)) {
            (NativeKind::Tuple, Some(tag)) if values.equal(&self.tag, &tag) => {
                (self.project)(&value)
            }
            _ => None,
        })
    }
}

impl<Profile: GleamErlangHostProfile, Output: Send + 'static> RecordReceive<Profile, Output> {
    pub(crate) fn new<Provider: HostProvider<Profile>, Return: HostType>(
        call: &mut HostCall<'_, Profile, Provider, Return>,
        pid: ExecutionUnitId,
        tag: NativeValue,
        project: fn(&NativeValue) -> Option<Output>,
        deadline: Option<Instant>,
    ) -> Result<Self, HostCallError> {
        let scan = Profile::erlang_execution(call.execution_state())
            .mailbox(pid)
            .ok_or_else(|| geam_core::HostFailure::new("process mailbox is closed"))?
            .scan(0);
        Ok(Self {
            pid,
            filter: RecordFields { tag, project },
            scan,
            timeout: deadline.map(|deadline| call.clock().sleep_until(deadline)),
            profile: PhantomData,
        })
    }

    /// Returns projected fields, an elapsed deadline, or an execution failure.
    pub async fn wait<Provider: HostProvider<Profile>, Targets: HostTypeSequence>(
        self,
        context: &HostExecutionContext<'_, Profile, Provider, Targets>,
    ) -> Result<Option<Output>, HostExecutionError> {
        let first = start(context, self.pid, self.filter.clone(), self.scan).await?;
        match self.timeout {
            Some(timeout) => timed_selected(context, self.pid, self.filter, first, timeout).await,
            None => next_selected(context, self.pid, self.filter, first)
                .await
                .map(Some),
        }
    }

    /// Waits without a deadline, discarding any timeout supplied at preparation.
    pub async fn wait_forever<Provider: HostProvider<Profile>, Targets: HostTypeSequence>(
        self,
        context: &HostExecutionContext<'_, Profile, Provider, Targets>,
    ) -> Result<Output, HostExecutionError> {
        drop(self.timeout);
        let first = start(context, self.pid, self.filter.clone(), self.scan).await?;
        next_selected(context, self.pid, self.filter, first).await
    }
}

#[cfg(test)]
mod tests {
    use super::super::record_fields as fields;
    use crate::service::{CurrentProcess, Processes};
    use crate::{Component, GleamErlangProfile};
    use geam_core::host::{
        HostCall, HostCallContinuation, HostCallError, HostCallable, HostConstructions,
        HostFunctionType, HostOwnedCompletion, HostProviderModule, HostTypeListEnd,
    };
    use geam_core::provider::advanced::NativeValue;
    use geam_core::{ModuleSource, PackageSource};
    use num_bigint::BigInt;

    #[test]
    fn projection_is_part_of_selection_and_keeps_rejected_records_in_order() {
        let provider = HostProviderModule::new("application", "main").unwrap()
            .with_resumable_function::<Component<GleamErlangProfile>, (BigInt, HostFunctionType<HostTypeListEnd, ()>), (), HostTypeListEnd, _>("check", check).unwrap();
        let result = crate::test_support::run_main(
            [PackageSource::new(
                "application",
                Vec::<String>::new(),
                [ModuleSource::new(
                    "main",
                    "main.gleam",
                    r#"
@external(erlang, "host", "check") fn check(mode: Int, child: fn() -> Nil) -> Nil
fn child() { panic as "terminated child must not run" }
pub fn main() { check(0, child) check(1, child) check(2, child) check(3, child) Nil }
"#,
                )],
            )],
            [provider],
        );
        assert_eq!(result.inspect().to_string(), "Nil");
    }

    fn check<'call>(
        call: HostCall<'call, GleamErlangProfile, Component<GleamErlangProfile>, ()>,
        constructions: HostConstructions<'call, HostTypeListEnd>,
        mode: BigInt,
        child: HostCallable<'call, HostTypeListEnd, ()>,
    ) -> Result<HostCallContinuation<'call, ()>, HostCallError> {
        CurrentProcess::with(call, |mut process| {
            let current = process.current().clone();
            let child = process.call().spawn(child);
            process.processes().kill(&child);
            let closed = super::RecordReceive::new(
                process.call(),
                child.id(),
                NativeValue::symbol("reply"),
                fields,
                None,
            );
            assert_eq!(
                closed.err().unwrap().to_string(),
                "process mailbox is closed"
            );
            let mut rejected = (0..70)
                .map(|index| process.call().native_values().integer(index.into()))
                .collect::<Vec<_>>();
            rejected.extend([
                NativeValue::tuple([]),
                NativeValue::tuple([
                    NativeValue::symbol("other"),
                    NativeValue::symbol("one"),
                    NativeValue::symbol("two"),
                ]),
                NativeValue::tuple([NativeValue::symbol("reply")]),
                NativeValue::tuple([NativeValue::symbol("reply"), NativeValue::symbol("one")]),
                NativeValue::tuple([
                    NativeValue::symbol("reply"),
                    NativeValue::symbol("one"),
                    NativeValue::symbol("two"),
                    NativeValue::symbol("extra"),
                ]),
            ]);
            for message in &rejected {
                process.processes().send(&current, message.clone());
            }
            if mode != BigInt::from(3) {
                process.processes().send(
                    &current,
                    NativeValue::tuple([
                        NativeValue::symbol("reply"),
                        NativeValue::symbol("first"),
                        NativeValue::symbol("second"),
                    ]),
                );
            }
            let deadline = (mode != BigInt::from(1)).then(|| process.call().clock().now());
            let receive = process
                .receive_record_with(NativeValue::symbol("reply"), fields, deadline)
                .unwrap();
            Ok(process.into_call().resume(constructions, move |context| {
                Box::pin(async move {
                    let selected = if mode == BigInt::from(2) {
                        Some(receive.wait_forever(&context).await.unwrap())
                    } else {
                        receive.wait(&context).await.unwrap()
                    };
                    if mode == BigInt::from(3) {
                        assert!(selected.is_none());
                    } else {
                        let (first, second) = selected.unwrap();
                        assert_eq!(first.as_symbol().as_deref(), Some("first"));
                        assert_eq!(second.as_symbol().as_deref(), Some("second"));
                    }
                    for expected in rejected {
                        let receive = context
                            .with_call(|mut call| {
                                Processes::new(&mut call).receive_any(None).unwrap()
                            })
                            .await
                            .unwrap();
                        let actual = receive.wait_forever(&context).await.unwrap();
                        context
                            .with_call(move |call| {
                                assert!(call.native_values().equal(&actual, &expected))
                            })
                            .await
                            .unwrap();
                    }
                    Ok(HostOwnedCompletion::new(
                        |call, _| Ok(call.return_value(())),
                    ))
                })
            }))
        })
    }
}
