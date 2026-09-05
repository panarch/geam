use super::{
    GateFuture, IntCallback, IntCallbackArguments, ManualGate, OrderedEcho, PendingOnce,
    poll_woken_to_ready, require_send,
};
use crate::embedding::{
    AsyncCallError as CallError, AsyncHostedModule, AsyncHostedModuleBuilder, BigInt, Function,
    FunctionDeclaration,
};
use crate::host::{
    AsyncHostCall, AsyncHostCallError, AsyncHostCallable, AsyncHostFuture, AsyncHostModule,
    AsyncHostProviderModule, AsyncHostProviderSet, HostProfile, HostProvider,
};
use crate::{
    ExecutionError, ModuleSource, PackageSource, PanicKind, compile_typed_async_host_program,
};
use std::cell::Cell;
use std::future::{Future, ready};
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::task::{Context, Poll, Waker};

struct RepeatProfile;
struct RepeatProvider;

struct RepeatState {
    gates: [Arc<ManualGate>; 3],
    callback_count: Cell<usize>,
    events: Vec<&'static str>,
}

impl RepeatState {
    fn new(gates: &[Arc<ManualGate>; 3]) -> Self {
        Self {
            gates: gates.clone(),
            callback_count: Cell::new(0),
            events: Vec::new(),
        }
    }
}

impl HostProfile for RepeatProfile {
    type RunState = RepeatState;
    type ExternalStores = Cell<()>;
}

impl HostProvider<RepeatProfile> for RepeatProvider {
    type State = RepeatState;

    fn project(state: &mut RepeatState) -> &mut Self::State {
        state
    }
}

fn suspend<'call>(
    mut call: AsyncHostCall<'call, RepeatProfile, RepeatProvider, BigInt>,
    value: BigInt,
) -> AsyncHostFuture<'call, BigInt> {
    AsyncHostFuture::new(async move {
        let gate = call
            .with_state(|state| {
                let index = state.callback_count.get();
                state.callback_count.set(index + 1);
                state.events.push("callback started");
                Arc::clone(&state.gates[2 * (index % 2)])
            })
            .await;
        let result = GateFuture {
            gate,
            value: ready(value + 1),
        }
        .await;
        call.with_state(|state| state.events.push("callback resumed"))
            .await;
        result
    })
}

fn invoke_twice<'call>(
    mut call: AsyncHostCall<'call, RepeatProfile, RepeatProvider, BigInt>,
    callback: AsyncHostCallable<'call, RepeatProfile, IntCallbackArguments, BigInt>,
    value: BigInt,
) -> AsyncHostFuture<'call, Result<BigInt, AsyncHostCallError>> {
    AsyncHostFuture::new(async move {
        let first = call.invoke(&callback, (value, ())).await?;
        let gate = call
            .with_state(|state| {
                state.events.push("between");
                Arc::clone(&state.gates[1])
            })
            .await;
        let first = GateFuture {
            gate,
            value: ready(first),
        }
        .await;
        let second = call.invoke(&callback, (first, ()));
        drop(callback);
        let second = second.await?;
        call.with_state(|state| state.events.push("completed"))
            .await;
        Ok(second)
    })
}

fn repeated_module() -> (
    AsyncHostedModule<RepeatProfile>,
    Function<(BigInt,), BigInt>,
) {
    let host = AsyncHostProviderModule::<RepeatProfile>::new("application", "library")
        .expect("repeat host module")
        .with_scoped_async_function::<RepeatProvider, (BigInt,), BigInt, _>("suspend", suspend)
        .expect("nested suspension")
        .with_fallible_scoped_async_function::<RepeatProvider, (IntCallback, BigInt), BigInt, _>(
            "invoke",
            invoke_twice,
        )
        .expect("repeat callback host");
    let providers =
        AsyncHostProviderSet::with_providers(Vec::<AsyncHostModule<RepeatProfile>>::new(), [host])
            .expect("repeat providers");
    let program = compile_typed_async_host_program(
        "application",
        "library",
        [PackageSource::new(
            "application",
            Vec::<String>::new(),
            [ModuleSource::new(
                "library",
                "src/library.gleam",
                r#"
@external(erlang, "native", "suspend")
fn suspend(value: Int) -> Int

@external(erlang, "native", "invoke")
fn invoke(callback: fn(Int) -> Int, value: Int) -> Int

pub fn run(value: Int) -> Int {
  let offset = 3
  let callback = fn(value) {
    echo value as "before"
    let resumed = suspend(value)
    echo resumed as "after"
    assert resumed > 0 as "positive callback"
    resumed + offset
  }
  let result = invoke(callback, value)
  echo result as "outer"
  result
}
"#,
            )],
        )],
        providers,
    )
    .expect("repeat source");
    let (bindings, function) = AsyncHostedModuleBuilder::new(program)
        .expect("repeat plan")
        .function(FunctionDeclaration::<(BigInt,), BigInt>::new("run"))
        .expect("repeat entry");
    (bindings.seal(), function)
}

fn echo_trace(echo: &OrderedEcho) -> Vec<(String, String)> {
    echo.outputs
        .lock()
        .expect("echo observations")
        .iter()
        .map(|output| {
            assert_eq!(output.path.as_deref(), Some("src/library.gleam"));
            (
                output.message.as_ref().expect("label").to_string(),
                output.value.clone(),
            )
        })
        .collect()
}

#[test]
fn one_captured_callback_is_reusable_across_pending_reentry_and_worker_movement() {
    let (mut module, function) = repeated_module();
    let gates = std::array::from_fn(|_| Arc::new(ManualGate::default()));
    let mut state = RepeatState::new(&gates);
    let mut echo = OrderedEcho::default();
    let mut call = Box::pin(require_send(module.call_async(
        &function,
        (5.into(),),
        &mut state,
        &mut echo,
    )));
    for gate in &gates {
        std::thread::scope(|scope| {
            scope
                .spawn(|| {
                    assert_eq!(
                        call.as_mut().poll(&mut Context::from_waker(Waker::noop())),
                        Poll::Pending
                    );
                })
                .join()
                .expect("poll worker");
        });
        assert_eq!(gate.polls.load(Ordering::SeqCst), 1);
        gate.release();
    }
    let result = std::thread::scope(|scope| {
        scope
            .spawn(|| poll_woken_to_ready(call.as_mut()))
            .join()
            .expect("completion worker")
    });
    assert_eq!(result, Ok(13.into()));
    drop(call);
    assert_eq!(state.callback_count.get(), 2);
    assert_eq!(
        state.events,
        [
            "callback started",
            "callback resumed",
            "between",
            "callback started",
            "callback resumed",
            "completed",
        ]
    );
    assert_eq!(
        gates
            .each_ref()
            .map(|gate| gate.drops.load(Ordering::SeqCst)),
        [1, 1, 1]
    );
    assert_eq!(
        echo_trace(&echo),
        [
            ("before".into(), "5".into()),
            ("after".into(), "6".into()),
            ("before".into(), "9".into()),
            ("after".into(), "10".into()),
            ("outer".into(), "13".into()),
        ]
    );
    assert_eq!(
        poll_woken_to_ready(module.call_async(&function, (5.into(),), &mut state, &mut echo,)),
        Ok(13.into())
    );
    assert_eq!(state.callback_count.get(), 4);
    assert_eq!(
        gates
            .each_ref()
            .map(|gate| gate.drops.load(Ordering::SeqCst)),
        [2, 2, 2]
    );
}

#[test]
fn cancellation_before_during_and_between_repeated_callbacks_never_replays_work() {
    let (mut module, function) = repeated_module();
    for completed_gates in 0..=3 {
        let gates = std::array::from_fn(|_| Arc::new(ManualGate::default()));
        let mut state = RepeatState::new(&gates);
        let mut echo = OrderedEcho::default();
        let mut call = Box::pin(module.call_async(&function, (5.into(),), &mut state, &mut echo));
        for index in 0..completed_gates {
            if index > 0 {
                gates[index - 1].release();
            }
            assert_eq!(
                call.as_mut().poll(&mut Context::from_waker(Waker::noop())),
                Poll::Pending
            );
        }
        drop(call);
        for (index, gate) in gates.iter().enumerate() {
            assert_eq!(
                gate.drops.load(Ordering::SeqCst),
                usize::from(index < completed_gates)
            );
            gate.release();
        }
        let expected_events: &[&str] = match completed_gates {
            0 => &[],
            1 => &["callback started"],
            2 => &["callback started", "callback resumed", "between"],
            3 => &[
                "callback started",
                "callback resumed",
                "between",
                "callback started",
            ],
            _ => unreachable!(),
        };
        assert_eq!(state.events, expected_events);
        let expected_echoes = &[
            ("before".into(), "5".into()),
            ("after".into(), "6".into()),
            ("before".into(), "9".into()),
        ][..completed_gates];
        assert_eq!(echo_trace(&echo), expected_echoes);
        let mut next_state = RepeatState::new(&gates);
        assert_eq!(
            poll_woken_to_ready(module.call_async(
                &function,
                (5.into(),),
                &mut next_state,
                &mut echo,
            )),
            Ok(13.into())
        );
        assert_eq!(next_state.callback_count.get(), 2);
        for (index, gate) in gates.iter().enumerate() {
            assert_eq!(
                gate.drops.load(Ordering::SeqCst),
                1 + usize::from(index < completed_gates)
            );
        }
    }
}

#[test]
fn public_completion_remains_send_when_retained_across_another_await() {
    let (mut module, function) = repeated_module();
    for input in [5, -2] {
        let gates = std::array::from_fn(|_| Arc::new(ManualGate::default()));
        for gate in &gates {
            gate.release();
        }
        let mut state = RepeatState::new(&gates);
        let mut echo = OrderedEcho::default();
        let polls = Arc::new(AtomicUsize::new(0));
        let future = require_send(async {
            let completion = module
                .call_async(&function, (input.into(),), &mut state, &mut echo)
                .await;
            PendingOnce {
                polls: Arc::clone(&polls),
                pending_returned: false,
                value: ready(()),
            }
            .await;
            completion
        });
        let mut future = Box::pin(future);
        std::thread::scope(|scope| {
            scope
                .spawn(|| {
                    assert_eq!(
                        future
                            .as_mut()
                            .poll(&mut Context::from_waker(Waker::noop())),
                        Poll::Pending
                    );
                })
                .join()
                .expect("first worker");
        });
        assert_eq!(polls.load(Ordering::SeqCst), 1);
        let completion = std::thread::scope(|scope| {
            scope
                .spawn(|| poll_woken_to_ready(future.as_mut()))
                .join()
                .expect("second worker")
        });
        drop(future);
        assert_eq!(polls.load(Ordering::SeqCst), 2);
        if input == 5 {
            assert_eq!(completion, Ok(13.into()));
        } else {
            let Err(CallError::Execution(ExecutionError::Panic(panic))) = completion else {
                panic!("the callback assertion must survive worker transfer");
            };
            assert_eq!(panic.kind(), PanicKind::Assert);
            assert_eq!(panic.to_string(), "assert: positive callback");
            assert_eq!(panic.site().module(), "library");
            assert_eq!(state.events, ["callback started", "callback resumed"]);
            assert_eq!(
                echo_trace(&echo),
                [
                    ("before".into(), "-2".into()),
                    ("after".into(), "-1".into())
                ]
            );
        }
    }
}
