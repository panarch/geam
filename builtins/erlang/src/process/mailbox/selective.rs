use super::{MailboxMatch, Sleep, next_selected, start, timed_selected};
use crate::GleamErlangHostProfile;
use crate::execution::Scan;
use geam_core::execution::ExecutionUnitId;
use geam_core::host::native::NativeValues;
use geam_core::host::{
    HostCall, HostCallError, HostExecutionContext, HostExecutionError, HostProfile, HostProvider,
    HostType, HostTypeSequence,
};
use geam_core::provider::advanced::NativeValue;
use geam_core::provider::{Call, ProviderExecutionCall, ProviderFactoryBindings};
use std::future::Future;
use std::marker::PhantomData;
use std::time::Instant;

/// A receive selected by an owned, caller-defined matcher.
///
/// The matcher borrows each candidate and the producer's native comparison
/// context. `None` leaves that candidate at its original queue position;
/// `Some(output)` removes the first matching message and returns the owned
/// output. One rule can recognize multiple tags without grouping their order.
///
/// Captured state must be `Send + 'static`, but need not be `Clone` or `Sync`.
/// Matching is synchronous and should be bounded and read-only. The matcher
/// receives no host effects or source callback capability.
pub struct SelectiveReceive<Profile: GleamErlangHostProfile, Matcher, Output> {
    pid: ExecutionUnitId,
    filter: MatchWith<Matcher>,
    scan: Scan,
    timeout: Option<Sleep>,
    profile: PhantomData<fn() -> (Profile, Output)>,
}

struct MatchWith<Matcher>(Matcher);

impl<Profile, Matcher, Output> SelectiveReceive<Profile, Matcher, Output>
where
    Profile: GleamErlangHostProfile,
    Matcher: for<'values, 'candidate> Fn(
            NativeValues<'values>,
            &'candidate NativeValue,
        ) -> Option<Output>
        + Send
        + 'static,
    Output: Send + 'static,
{
    /// Waits through the ordinary async provider call's original endpoint.
    pub fn wait_in<'request, 'run, Provider, Observation, Bindings>(
        self,
        call: &'request Call<
            Provider::State,
            ProviderExecutionCall<'run, Profile, Provider, Observation, Bindings>,
        >,
    ) -> impl Future<Output = Result<Option<Output>, HostExecutionError>> + Send + 'request
    where
        Provider: HostProvider<Profile>,
        Bindings: ProviderFactoryBindings,
        'run: 'request,
    {
        self.wait(call.execution_context())
    }

    /// Waits through an ordinary async call, discarding the prepared deadline.
    pub fn wait_forever_in<'request, 'run, Provider, Observation, Bindings>(
        self,
        call: &'request Call<
            Provider::State,
            ProviderExecutionCall<'run, Profile, Provider, Observation, Bindings>,
        >,
    ) -> impl Future<Output = Result<Output, HostExecutionError>> + Send + 'request
    where
        Provider: HostProvider<Profile>,
        Bindings: ProviderFactoryBindings,
        'run: 'request,
    {
        self.wait_forever(call.execution_context())
    }

    /// Returns the selected output, `None` for timeout, or an execution failure.
    /// The queued snapshot is scanned before observing even an elapsed deadline.
    /// Dropping the receive releases its matcher without consuming another message.
    pub async fn wait<Provider: HostProvider<Profile>, Targets: HostTypeSequence>(
        self,
        context: &HostExecutionContext<'_, Profile, Provider, Targets>,
    ) -> Result<Option<Output>, HostExecutionError> {
        let (filter, first) = start(context, self.pid, self.filter, self.scan).await?;
        match self.timeout {
            Some(timeout) => timed_selected(context, self.pid, filter, first, timeout).await,
            None => next_selected(context, self.pid, filter, first)
                .await
                .map(Some),
        }
    }

    /// Waits without a deadline through the original execution context.
    /// Any timeout supplied at preparation is discarded.
    pub async fn wait_forever<Provider: HostProvider<Profile>, Targets: HostTypeSequence>(
        self,
        context: &HostExecutionContext<'_, Profile, Provider, Targets>,
    ) -> Result<Output, HostExecutionError> {
        drop(self.timeout);
        let (filter, first) = start(context, self.pid, self.filter, self.scan).await?;
        next_selected(context, self.pid, filter, first).await
    }

    pub(crate) fn new<Provider: HostProvider<Profile>, Return: HostType>(
        call: &mut HostCall<'_, Profile, Provider, Return>,
        pid: ExecutionUnitId,
        matcher: Matcher,
        deadline: Option<Instant>,
    ) -> Result<Self, HostCallError> {
        let scan = Profile::erlang_execution(call.execution_state())
            .mailbox(pid)
            .map(|mailbox| mailbox.scan(0))
            .ok_or_else(|| geam_core::HostFailure::new("process mailbox is closed").into());
        scan.map(|scan| Self {
            pid,
            filter: MatchWith(matcher),
            scan,
            timeout: deadline.map(|deadline| call.clock().sleep_until(deadline)),
            profile: PhantomData,
        })
    }
}

impl<Profile, Matcher, Output> MailboxMatch<Profile> for MatchWith<Matcher>
where
    Profile: HostProfile,
    Matcher: for<'values, 'candidate> Fn(
            NativeValues<'values>,
            &'candidate NativeValue,
        ) -> Option<Output>
        + Send
        + 'static,
    Output: Send + 'static,
{
    type Output = Output;

    fn select(
        &self,
        values: NativeValues<'_>,
        value: NativeValue,
    ) -> Result<Option<Output>, HostCallError> {
        Ok((self.0)(values, &value))
    }
}

#[cfg(test)]
mod tests {
    use crate::execution_fixture::TestHost;
    use crate::process::{Native, One};
    use crate::service::{CurrentProcess, Processes};
    use crate::test_support::PollGate;
    use crate::{Component, Configuration, GleamErlangProfile, GleamErlangRunState};
    use geam_core::embedding::{CallError, FunctionDeclaration, HostedModuleBuilder};
    use geam_core::host::HostTypeIndex0;
    use geam_core::host::native::NativeRules;
    use geam_core::host::native::NativeValues;
    use geam_core::host::{
        HostCall, HostCallContinuation, HostCallError, HostConstructions, HostOwnedCompletion,
        HostProviderModule, HostProviderSet, HostTypeListEnd,
    };
    use geam_core::provider::Call as ProviderCall;
    use geam_core::provider::advanced::NativeValue;
    use geam_core::{ModuleSource, PackageSource, compile_typed_host_program};
    use num_bigint::BigInt;
    use std::cell::Cell;
    use std::future::{Future, poll_fn};
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::{Arc, Mutex};
    use std::task::Poll;
    use std::time::Duration;

    // Deliberately neither Clone nor Sync. Reading it keeps the entire owned
    // capture in the matcher instead of testing only a disjoint Arc field.
    struct Capture {
        calls: Cell<usize>,
        alive: Arc<()>,
        visits: Arc<AtomicUsize>,
    }

    struct OwnedMatch {
        value: NativeValue,
        alive: Arc<()>,
        observation: Cell<usize>,
    }

    impl Capture {
        fn visit(&self) {
            self.calls.set(self.calls.get() + 1);
            self.visits.fetch_add(1, Ordering::SeqCst);
            assert_eq!(Arc::strong_count(&self.alive), 1);
        }
    }

    fn match_identity(
        capture: Capture,
        wanted: NativeValue,
    ) -> impl Fn(NativeValues<'_>, &NativeValue) -> Option<OwnedMatch> + Send {
        move |values, message| {
            capture.visit();
            let (Some(tag), Some(identity), Some(length)) =
                (message.index(0), message.index(1), message.len())
            else {
                return None;
            };
            let tag = tag.as_symbol()?;
            (matches!((tag.as_str(), length), ("tcp", 3) | ("closed", 2))
                && values.equal(&wanted, &identity))
            .then(|| OwnedMatch {
                value: message.clone(),
                alive: Arc::clone(&capture.alive),
                observation: Cell::new(0),
            })
        }
    }

    #[test]
    fn owned_matchers_preserve_queue_order_snapshots_and_project_before_consumption() {
        let provider = HostProviderModule::new("application", "main").unwrap()
            .with_resumable_function::<Component<GleamErlangProfile>, (BigInt,), (), HostTypeListEnd, _>("check", check).unwrap();
        let result = crate::test_support::run_main(
            [PackageSource::new(
                "application",
                Vec::<String>::new(),
                [ModuleSource::new(
                    "main",
                    "main.gleam",
                    r#"
@external(erlang, "host", "check") fn check(mode: Int) -> Nil
pub fn main() {
  check(0) // queued match, unbounded wait
  check(1) // queued match before zero deadline
  check(2) // forever discards zero deadline
  check(3) // full rejected snapshot before timeout
  check(4) // snapshot arrival and two unbounded wakes
  check(5) // snapshot arrival and two timed wakes
  Nil
}
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
    ) -> Result<HostCallContinuation<'call, ()>, HostCallError> {
        CurrentProcess::with(call, |mut process| {
            let current = process.current().clone();
            let values = process.call().native_values();
            let wanted = values.integer(11.into());
            let other = values.integer(12.into());
            let a = NativeValue::tuple([
                NativeValue::symbol("tcp"),
                wanted.clone(),
                NativeValue::symbol("a"),
            ]);
            let b =
                NativeValue::tuple([NativeValue::symbol("tcp"), other, NativeValue::symbol("b")]);
            let closed = NativeValue::tuple([NativeValue::symbol("closed"), wanted.clone()]);
            let rejected = (0..70)
                .map(|index| values.integer(index.into()))
                .chain([
                    b,
                    NativeValue::symbol("user"),
                    NativeValue::tuple([]),
                    NativeValue::tuple([NativeValue::symbol("tcp")]),
                    NativeValue::tuple([values.integer(0.into()), wanted.clone()]),
                    NativeValue::tuple([NativeValue::symbol("other"), wanted.clone()]),
                ])
                .collect::<Vec<_>>();
            let rejected_len = rejected.len();
            for message in &rejected {
                process.processes().send(&current, message.clone());
            }
            if mode < BigInt::from(3) {
                process.processes().send(&current, a.clone());
                process.processes().send(&current, closed.clone());
            }
            let alive = Arc::new(());
            let weak = Arc::downgrade(&alive);
            let visits = Arc::new(AtomicUsize::new(0));
            let capture = Capture {
                calls: Cell::new(0),
                alive,
                visits: Arc::clone(&visits),
            };
            let matcher = match_identity(capture, wanted.clone());
            let deadline = if mode == BigInt::from(5) {
                Some(process.call().clock().now() + Duration::from_secs(1))
            } else {
                (mode != BigInt::from(0) && mode != BigInt::from(4))
                    .then(|| process.call().clock().now())
            };
            let receive = process.receive_with(matcher, deadline).unwrap();
            // Preparing and dropping a receive never calls its matcher or removes a candidate.
            let unstarted_alive = Arc::new(());
            let unstarted_weak = Arc::downgrade(&unstarted_alive);
            let unstarted_visits = Arc::new(AtomicUsize::new(0));
            let unstarted_capture = Capture {
                calls: Cell::new(0),
                alive: unstarted_alive,
                visits: Arc::clone(&unstarted_visits),
            };
            let unstarted = process
                .receive_with(match_identity(unstarted_capture, wanted), None)
                .unwrap();
            drop(unstarted);
            assert!(unstarted_weak.upgrade().is_none());
            assert_eq!(unstarted_visits.load(Ordering::SeqCst), 0);
            Ok(process.into_call().resume(constructions, move |context| {
                Box::pin(async move {
                    let authoring = ProviderCall::from_execution_context(context);
                    let context = authoring.execution_context();
                    let selected = if mode == BigInt::from(2) {
                        Some(receive.wait_forever_in(&authoring).await.unwrap())
                    } else if mode >= BigInt::from(4) {
                        let mut waiting = Box::pin(receive.wait(context));
                        poll_fn(|cx| {
                            assert!(waiting.as_mut().poll(cx).is_pending());
                            Poll::Ready(())
                        })
                        .await;
                        // This request runs after the initial bounded scan. The
                        // remaining original snapshot must still be visited once.
                        let target = current.clone();
                        let observed = Arc::clone(&visits);
                        context
                            .with_call(move |mut call| {
                                assert_eq!(observed.load(Ordering::SeqCst), 64);
                                Processes::new(&mut call)
                                    .send(&target, NativeValue::symbol("new rejection"));
                            })
                            .await
                            .unwrap();
                        poll_fn(|cx| {
                            assert!(waiting.as_mut().poll(cx).is_pending());
                            Poll::Ready(())
                        })
                        .await;
                        let observed = Arc::clone(&visits);
                        context
                            .with_call(move |_| {
                                assert_eq!(observed.load(Ordering::SeqCst), rejected_len)
                            })
                            .await
                            .unwrap();
                        // The first wake scans only the new rejection, then waits
                        // again. A second wake delivers the match without rescanning.
                        poll_fn(|cx| {
                            assert!(waiting.as_mut().poll(cx).is_pending());
                            Poll::Ready(())
                        })
                        .await;
                        let send_a = a.clone();
                        let send_closed = closed.clone();
                        let observed = Arc::clone(&visits);
                        context
                            .with_call(move |mut call| {
                                assert_eq!(observed.load(Ordering::SeqCst), rejected_len + 1);
                                let mut processes = Processes::new(&mut call);
                                processes.send(&current, send_a);
                                processes.send(&current, send_closed);
                            })
                            .await
                            .unwrap();
                        waiting.await.unwrap()
                    } else {
                        receive.wait_in(&authoring).await.unwrap()
                    };
                    assert_eq!(weak.strong_count(), usize::from(selected.is_some()));
                    let expected_visits = rejected.len()
                        + usize::from(mode != BigInt::from(3))
                        + usize::from(mode >= BigInt::from(4));
                    assert_eq!(visits.load(Ordering::SeqCst), expected_visits);
                    if mode == BigInt::from(3) {
                        assert!(selected.is_none());
                    } else {
                        let actual = selected.unwrap();
                        let alias = Arc::clone(&actual.alive);
                        context
                            .with_call(move |call| {
                                assert_eq!(actual.observation.get(), 0);
                                assert!(call.native_values().equal(&actual.value, &a));
                                drop(actual);
                            })
                            .await
                            .unwrap();
                        assert_eq!(weak.strong_count(), 1);
                        drop(alias);
                        assert!(weak.upgrade().is_none());
                        // A new receive can select a different tag while retaining FIFO.
                        let next = context
                            .with_call(|mut call| {
                                Processes::new(&mut call)
                                    .receive_with(
                                        |_, message| match (
                                            message.index(0).and_then(|tag| tag.as_symbol()),
                                            message.index(1),
                                        ) {
                                            (Some(tag), Some(identity)) if tag == "closed" => {
                                                Some(identity)
                                            }
                                            _ => None,
                                        },
                                        None,
                                    )
                                    .unwrap()
                            })
                            .await
                            .unwrap();
                        let actual = next.wait_forever(context).await.unwrap();
                        context
                            .with_call(move |call| {
                                assert!(
                                    call.native_values()
                                        .equal(&actual, &closed.index(1).unwrap())
                                )
                            })
                            .await
                            .unwrap();
                    }
                    let mut expected = rejected;
                    if mode >= BigInt::from(4) {
                        expected.push(NativeValue::symbol("new rejection"));
                    }
                    for expected in expected {
                        let next = context
                            .with_call(|mut call| {
                                Processes::new(&mut call).receive_any(None).unwrap()
                            })
                            .await
                            .unwrap();
                        let actual = next.wait_forever(context).await.unwrap();
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

    #[test]
    fn owned_matchers_release_on_completion_cancellation_and_domain_drop() {
        // 0: first request; 1: unfinished snapshot; 2: waiting mailbox woken
        // before recheck; 3: pending receive at domain drop. Both deadline and
        // unbounded loops carry owned state. Phase 4 is their successful
        // completion control with the same matcher and native return type.
        for phase in 0..5 {
            for timed in [false, true] {
                let (gate, driver) = PollGate::new();
                let alive = Arc::new(());
                let weak = Arc::downgrade(&alive);
                let slot = Arc::new(Mutex::new(Some(alive)));
                let owned = Arc::clone(&slot);
                let provider = HostProviderModule::new("application", "main").unwrap()
                    .with_resumable_native_function::<Component<GleamErlangProfile>, (), (), One<()>, _>("wait", NativeRules::default(), move |call| {
                        cancel_wait(call, phase, timed, Arc::clone(&owned), Arc::clone(&gate))
                    }).unwrap();
                let typed = compile_typed_host_program(
                    "application",
                    "main",
                    [PackageSource::new(
                        "application",
                        Vec::<String>::new(),
                        [ModuleSource::new(
                            "main",
                            "main.gleam",
                            r#"
@external(erlang, "host", "wait") fn wait() -> Nil
pub fn main() { wait() }
"#,
                        )],
                    )],
                    HostProviderSet::from_providers([provider]).unwrap(),
                )
                .unwrap();
                let (builder, main) = HostedModuleBuilder::<GleamErlangProfile>::new(typed)
                    .unwrap()
                    .function(FunctionDeclaration::<(), ()>::new("main"))
                    .unwrap();
                let mut module = builder.seal().unwrap();
                let host = TestHost::default();
                let mut state = GleamErlangRunState {
                    stdlib: geam_stdlib::GleamStdlibRunState::from_seed([0; 32]),
                    erlang: Configuration::default(),
                };
                let mut echo = Vec::new();
                let mut execution = Box::pin(module.with_execution(
                    &host,
                    &mut state,
                    &mut echo,
                    async |scope| scope.call(&main, ()).await,
                ));
                if phase == 4 {
                    assert_eq!(host.block_on(execution.as_mut()).unwrap(), Ok(()));
                    drop(execution);
                    drop(driver);
                } else if phase == 3 {
                    assert!(host.poll(execution.as_mut()).is_pending());
                    assert_eq!(weak.strong_count(), 1);
                    drop(execution);
                    drop(driver);
                } else {
                    driver.cancel(&host, execution.as_mut());
                    assert_eq!(
                        host.block_on(execution.as_mut()).unwrap(),
                        Err(CallError::Cancelled)
                    );
                    drop(execution);
                }
                host.step();
                assert!(slot.lock().unwrap().is_none());
                assert!(weak.upgrade().is_none());
                assert!(echo.is_empty());
            }
        }
    }
    fn cancel_wait<'call>(
        mut native: Native<'call, GleamErlangProfile, (), ()>,
        phase: u8,
        timed: bool,
        owned: Arc<Mutex<Option<Arc<()>>>>,
        gate: Arc<PollGate>,
    ) -> Result<HostCallContinuation<'call, ()>, HostCallError> {
        let call = native.call();
        let unit = Processes::new(call).current().unwrap();
        let alive = owned.lock().unwrap().take().unwrap();
        let capture = Capture {
            calls: Cell::new(0),
            alive,
            visits: Arc::default(),
        };
        let matcher = move |_: NativeValues<'_>, message: &NativeValue| -> Option<NativeValue> {
            capture.visit();
            message
                .as_symbol()
                .filter(|symbol| symbol.as_str() == "finish")
                .map(|_| NativeValue::symbol("nil"))
        };
        if phase == 1 {
            for _ in 0..65 {
                Processes::new(call).send(&unit, NativeValue::symbol("skip"));
            }
        }
        if phase == 4 {
            Processes::new(call).send(&unit, NativeValue::symbol("finish"));
        }
        let deadline = timed.then(|| call.clock().now() + Duration::from_secs(1));
        let receive = Processes::new(call)
            .receive_with(matcher, deadline)
            .unwrap();

        Ok(native.resume::<HostTypeIndex0>(move |context| {
            Box::pin(async move {
                if phase >= 3 {
                    return receive.wait_forever(&context).await;
                }
                let error = if phase == 0 {
                    gate.observe(receive.wait(&context), unit)
                        .await
                        .err()
                        .unwrap()
                } else {
                    let (filter, first) =
                        super::start(&context, receive.pid, receive.filter, receive.scan)
                            .await
                            .unwrap();
                    if phase == 2 {
                        let target = unit.clone();
                        context
                            .with_call(move |mut call| {
                                Processes::new(&mut call)
                                    .send(&target, NativeValue::symbol("arrival"))
                            })
                            .await
                            .unwrap();
                    }
                    if let Some(timeout) = receive.timeout {
                        gate.observe(
                            super::timed_selected(&context, unit.id(), filter, first, timeout),
                            unit,
                        )
                        .await
                        .err()
                        .unwrap()
                    } else {
                        gate.observe(
                            super::next_selected(&context, unit.id(), filter, first),
                            unit,
                        )
                        .await
                        .err()
                        .unwrap()
                    }
                };
                Err(error)
            })
        }))
    }
}
