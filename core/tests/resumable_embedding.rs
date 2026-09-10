#![cfg(feature = "tokio")]

use futures_channel::{mpsc, oneshot};
use futures_util::{StreamExt, future};
use geam_core::embedding::{
    BigInt, Function, FunctionDeclaration, HostedModule, HostedModuleBuilder,
};
use geam_core::execution::TokioHost;
use geam_core::host::{
    HostCall, HostCallContinuation, HostCallError, HostCallable, HostConstructions,
    HostFunctionType, HostOwnedCompletion, HostProfile, HostProvider, HostProviderModule,
    HostProviderSet, HostTypeList, HostTypeListEnd,
};
use geam_core::{HostFailure, ModuleSource, PackageSource};
use std::cell::Cell;
use std::collections::VecDeque;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

struct Profile;
struct Provider;
struct State {
    folds: Cell<usize>,
    waits: Cell<usize>,
    gates: VecDeque<oneshot::Receiver<()>>,
    started: mpsc::UnboundedSender<usize>,
    destroyed: Arc<AtomicUsize>,
}

struct NativeLifetime(Arc<AtomicUsize>);
impl Drop for NativeLifetime {
    fn drop(&mut self) {
        self.0.fetch_add(1, Ordering::SeqCst);
    }
}

impl HostProfile for Profile {
    type RunState = State;
    type ExternalStores = Cell<()>;
    type ExecutionState = ();
}

impl HostProvider<Profile> for Provider {
    type State = State;
    fn project(state: &mut State) -> &mut State {
        state
    }
}

type Arguments = HostTypeList<BigInt, HostTypeListEnd>;
type Callback = HostFunctionType<Arguments, BigInt>;

fn fold<'call>(
    mut call: HostCall<'call, Profile, Provider, BigInt>,
    constructions: HostConstructions<'call, HostTypeListEnd>,
    callback: HostCallable<'call, Arguments, BigInt>,
    initial: BigInt,
) -> Result<HostCallContinuation<'call, BigInt>, HostCallError> {
    let state = call.state();
    state.folds.set(state.folds.get() + 1);
    let lifetime = NativeLifetime(Arc::clone(&state.destroyed));
    let callback = call.owned_callable(callback, &constructions);
    Ok(call.resume(constructions, move |context| {
        Box::pin(async move {
            let _lifetime = lifetime;
            let mut total = initial;
            for _ in 0..3 {
                total = callback
                    .invoke(&context, move |_, _| (total, ()), |_, _, value| Ok(value))
                    .await?;
            }
            Ok(HostOwnedCompletion::new(move |call, _| {
                Ok(call.return_value(total))
            }))
        })
    }))
}

fn wait<'call>(
    call: HostCall<'call, Profile, Provider, BigInt>,
    constructions: HostConstructions<'call, HostTypeListEnd>,
    value: BigInt,
) -> Result<HostCallContinuation<'call, BigInt>, HostCallError> {
    Ok(call.resume(constructions, move |context| {
        Box::pin(async move {
            let (gate, _lifetime) = context
                .with_state(|state| {
                    let index = state.waits.get();
                    state.waits.set(index + 1);
                    state.started.unbounded_send(index).unwrap();
                    (
                        state.gates.pop_front().unwrap(),
                        NativeLifetime(Arc::clone(&state.destroyed)),
                    )
                })
                .await?;
            gate.await
                .map_err(|_| HostFailure::new("native gate closed"))?;
            Ok(HostOwnedCompletion::new(move |call, _| {
                Ok(call.return_value(value + 1))
            }))
        })
    }))
}

fn program() -> (HostedModule<Profile>, Function<(BigInt,), BigInt>) {
    let provider = HostProviderModule::new("application", "library")
        .unwrap()
        .with_resumable_function::<Provider, (Callback, BigInt), BigInt, HostTypeListEnd, _>(
            "fold", fold,
        )
        .unwrap()
        .with_resumable_function::<Provider, (BigInt,), BigInt, HostTypeListEnd, _>("wait", wait)
        .unwrap();
    let source = r#"
@external(erlang, "native", "fold")
fn fold(callback: fn(Int) -> Int, initial: Int) -> Int
@external(erlang, "native", "wait")
fn wait(value: Int) -> Int
fn nested(value: Int, offset: Int) {
  let value = wait(value + offset)
  echo value
  value
}
pub fn run(offset: Int) { fold(fn(value) { nested(value, offset) }, 40) }
"#;
    let typed = geam_core::compile_typed_host_program(
        "application",
        "library",
        [PackageSource::new(
            "application",
            Vec::<String>::new(),
            [ModuleSource::new("library", "src/library.gleam", source)],
        )],
        HostProviderSet::from_providers([provider]).unwrap(),
    )
    .unwrap();
    let (bindings, run) = HostedModuleBuilder::new(typed)
        .unwrap()
        .function(FunctionDeclaration::<(BigInt,), BigInt>::new("run"))
        .unwrap();
    (bindings.seal().unwrap(), run)
}

#[test]
fn ordinary_native_loop_resumes_each_callback_without_source_future_or_replay() {
    let (mut module, run) = program();
    let (started, mut events) = mpsc::unbounded();
    let (senders, gates): (Vec<_>, VecDeque<_>) = (0..3).map(|_| oneshot::channel()).unzip();
    let destroyed = Arc::new(AtomicUsize::new(0));
    let mut state = State {
        folds: Cell::new(0),
        waits: Cell::new(0),
        gates,
        started,
        destroyed: Arc::clone(&destroyed),
    };
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(2)
        .build()
        .unwrap();
    let host = TokioHost::new(runtime.handle().clone());
    let mut echo = Vec::new();
    let value = runtime
        .block_on(
            module.with_execution(&host, &mut state, &mut echo, async |scope| {
                let controller = async {
                    for (index, sender) in senders.into_iter().enumerate() {
                        assert_eq!(events.next().await, Some(index));
                        sender.send(()).unwrap();
                    }
                };
                let (value, ()) =
                    future::join(scope.call(&run, (BigInt::from(1),)), controller).await;
                value.unwrap()
            }),
        )
        .unwrap();
    assert_eq!(value, BigInt::from(46));
    assert_eq!(state.folds.get(), 1);
    assert_eq!(state.waits.get(), 3);
    assert_eq!(
        echo.iter()
            .map(|output| output.value().inspect().to_string())
            .collect::<Vec<_>>(),
        ["42", "44", "46"]
    );
    assert_eq!(destroyed.load(Ordering::SeqCst), 4);
}

#[test]
fn abandoning_an_ordinary_call_releases_its_native_loop_and_waiting_callback() {
    let (mut module, run) = program();
    let (started, mut events) = mpsc::unbounded();
    let (_sender, gate) = oneshot::channel();
    let destroyed = Arc::new(AtomicUsize::new(0));
    let mut state = State {
        folds: Cell::new(0),
        waits: Cell::new(0),
        gates: VecDeque::from([gate]),
        started,
        destroyed: Arc::clone(&destroyed),
    };
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(2)
        .build()
        .unwrap();
    let host = TokioHost::new(runtime.handle().clone());
    let mut echo = Vec::new();
    runtime
        .block_on(
            module.with_execution(&host, &mut state, &mut echo, async |scope| {
                let call = Box::pin(scope.call(&run, (BigInt::from(0),)));
                let (event, call) = match future::select(events.next(), call).await {
                    future::Either::Left(result) => result,
                    future::Either::Right(_) => panic!("the native gate has not been released"),
                };
                assert_eq!(event, Some(0));
                drop(call);
            }),
        )
        .unwrap();
    assert_eq!(state.folds.get(), 1);
    assert_eq!(state.waits.get(), 1);
    assert!(echo.is_empty());
    assert_eq!(destroyed.load(Ordering::SeqCst), 2);
}
