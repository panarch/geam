#[path = "../../tests/support/execution_host.rs"]
mod execution_fixture;
#[path = "support/work_fixture.rs"]
mod work_fixture;
use crate::work_fixture::WorkComponent;
use crate::work_fixture::WorkType;
use geam_core::embedding::{BigInt, EcoString, FunctionDeclaration, HostedModuleBuilder, List};
use geam_core::frontend::compile_typed_host_program;
use geam_core::host::{HostComponentProfile, HostFutureStore, HostProfile, HostProviderSet};
use geam_core::{ModuleSource, PackageSource};
use std::future::Future as _;
use std::task::Poll;

struct Profile;

#[derive(Default)]
struct Echo(Vec<String>);
impl geam_core::EchoSink for Echo {
    fn emit(&mut self, output: geam_core::EchoOutput) {
        self.0.push(output.to_string());
    }
}

impl HostProfile for Profile {
    type RunState = ();
    type ExternalStores = HostFutureStore;
}

impl geam_core::host::HostWorkProfile for Profile {
    type Work = crate::work_fixture::WorkComponent;
}
impl HostComponentProfile<WorkComponent> for Profile {
    fn component_stores(stores: &HostFutureStore) -> &HostFutureStore {
        stores
    }
    fn component_state(state: &mut ()) -> &mut () {
        state
    }
}

struct DirectProvider;

impl geam_core::HostProvider<Profile> for DirectProvider {
    type State = ();
    fn project(state: &mut ()) -> &mut () {
        state
    }
}

type IntCallbackArguments = geam_core::HostTypeList<BigInt, geam_core::HostTypeListEnd>;
type IntCallback = geam_core::host::HostFunctionType<IntCallbackArguments, BigInt>;
type IntCallbackTupleElements = geam_core::HostTypeList<IntCallback, geam_core::HostTypeListEnd>;
type IntCallbackTuple = geam_core::host::HostTupleType<IntCallbackTupleElements>;

fn invoke_nested_callback<'call>(
    mut call: geam_core::host::HostCall<'call, Profile, DirectProvider, BigInt>,
    constructions: geam_core::HostConstructions<'call, geam_core::HostTypeListEnd>,
    callback: geam_core::host::HostTuple<'call, IntCallbackTupleElements>,
) -> Result<geam_core::HostCallContinuation<'call, BigInt>, geam_core::HostCallError> {
    let (callback, ()) = call.tuple_values(callback);
    let callback = call.owned_callable(callback, &constructions);
    Ok(call.resume(constructions, move |context| {
        Box::pin(async move {
            let returned = callback
                .invoke(
                    &context,
                    |_, _| (BigInt::from(41), ()),
                    |_, _, value| Ok(value),
                )
                .await?;
            Ok(geam_core::HostOwnedCompletion::new(move |call, _| {
                Ok(call.return_value(returned))
            }))
        })
    }))
}

#[test]
fn direct_transfer_calls_decode_callbacks_nested_in_compound_arguments() {
    let execution_host = crate::execution_fixture::TestHost::default();

    let provider = geam_core::host::HostProviderModule::new("app", "app")
        .expect("native module")
        .with_resumable_function::<DirectProvider, (IntCallbackTuple,), BigInt, geam_core::HostTypeListEnd, _>(
            "invoke_nested",
            invoke_nested_callback,
        )
        .expect("nested callback registration");
    let typed = compile_typed_host_program(
        "app",
        "app",
        [PackageSource::new(
            "app",
            Vec::<String>::new(),
            [ModuleSource::new(
                "app",
                "src/app.gleam",
                r#"
@external(erlang, "native", "invoke_nested")
fn invoke_nested(callback: #(fn(Int) -> Int)) -> Int
fn increment(value: Int) -> Int { value + 1 }
pub fn run() -> Int { invoke_nested(#(increment)) }
pub fn captured(offset: Int) -> Int {
  invoke_nested(#(fn(value) { value + offset }))
}
"#,
            )],
        )],
        HostProviderSet::from_providers([provider]).expect("providers"),
    )
    .expect("source");
    let (mut bindings, run) = HostedModuleBuilder::new(typed)
        .expect("plan")
        .function(FunctionDeclaration::<(), BigInt>::new("run"))
        .expect("entry");
    let captured = bindings
        .function(FunctionDeclaration::<(BigInt,), BigInt>::new("captured"))
        .expect("captured entry");
    let mut module = bindings.seal().expect("sealed callbacks");
    let mut echo = Echo::default();
    let mut state = ();
    let mut task =
        Box::pin(
            module.with_execution(&execution_host, &mut state, &mut echo, async |scope| {
                assert_eq!(
                    scope.call(&run, ()).await.expect("nested function"),
                    BigInt::from(42)
                );
                assert_eq!(
                    scope
                        .call(&captured, (2.into(),))
                        .await
                        .expect("captured function"),
                    BigInt::from(43)
                );
            }),
        );
    assert!(execution_host.poll(task.as_mut()).is_ready());
}

#[test]
fn independent_scopes_can_drive_their_own_work_at_the_same_time() {
    let execution_host = crate::execution_fixture::TestHost::default();

    let (left, left_work) = HostedModuleBuilder::new(program())
        .expect("left plan")
        .function(FunctionDeclaration::<(), WorkType<BigInt>>::new("delayed"))
        .expect("left entry");
    let (right, right_work) = HostedModuleBuilder::new(program())
        .expect("right plan")
        .function(FunctionDeclaration::<(), WorkType<BigInt>>::new("delayed"))
        .expect("right entry");
    let mut left = left.seal().expect("left execution");
    let mut right = right.seal().expect("right execution");
    let mut left_state = ();
    let mut right_state = ();
    let mut left_echo = Echo::default();
    let mut right_echo = Echo::default();
    let mut task = Box::pin(left.with_execution(
        &execution_host,
        &mut left_state,
        &mut left_echo,
        async |left| {
            let left_work = left.call(&left_work, ()).await.expect("left work");
            right
                .with_execution(
                    &execution_host,
                    &mut right_state,
                    &mut right_echo,
                    async |right| {
                        let right_work = right.call(&right_work, ()).await.expect("right work");
                        let left = left.observe(&left_work);
                        let right = right.observe(&right_work);
                        let (left, right) = futures_util::future::join(left, right).await;
                        let left = left.expect("left completion");
                        let right = right.expect("right completion");
                        assert_eq!(left.read(Clone::clone), BigInt::from(42));
                        assert_eq!(right.read(Clone::clone), BigInt::from(42));
                        left.read(|a| right.read(|b| assert!(!std::ptr::eq(a, b))));
                    },
                )
                .await
                .expect("right execution");
        },
    ));
    assert!(execution_host.poll(task.as_mut()).is_ready());
}

fn program() -> geam_core::frontend::HostedTypedProgram<Profile> {
    compile_typed_host_program(
        "app",
        "app",
        [
            PackageSource::new(
                "work_fixture",
                Vec::<String>::new(),
                [ModuleSource::new(
                    "fixture/work",
                    "src/fixture/work.gleam",
                    crate::work_fixture::WorkComponent::SOURCE,
                )],
            ),
            PackageSource::new(
                "app",
                ["work_fixture"],
                [ModuleSource::new(
                    "app",
                    "src/app.gleam",
                    r#"
import fixture/work as future
pub fn double(value: Int) { value * 2 }
pub fn delayed() {
  use value <- future.map(future.ready(21))
  double(value)
}
pub fn nested() -> future.Work(Result(#(String, List(future.Work(Int))), Nil)) {
  future.ready(Ok(#("values", [future.ready(40), future.ready(42)])))
}
pub fn keep(value: future.Work(Int)) { value }
pub fn keep_list(values: List(future.Work(Int))) { values }
pub fn packet_work() -> future.Work(Result(List(Int), String)) {
  future.ready(Ok([1, 2]))
}
pub fn packet(value: #(future.Work(Result(List(Int), String)), Result(List(Int), String))) { value }
"#,
                )],
            ),
        ],
        HostProviderSet::from_providers(
            WorkComponent::providers::<Profile>().expect("Future registration"),
        )
        .expect("provider set"),
    )
    .expect("source linkage")
}

#[test]
fn public_calls_preserve_direct_results_and_recursively_scoped_shared_work() {
    let execution_host = crate::execution_fixture::TestHost::default();

    let (mut bindings, double) = HostedModuleBuilder::new(program())
        .expect("planned library")
        .function(FunctionDeclaration::<(BigInt,), BigInt>::new("double"))
        .expect("direct entry");
    let delayed = bindings
        .function(FunctionDeclaration::<(), WorkType<BigInt>>::new("delayed"))
        .expect("Future entry");
    type Nested = WorkType<Result<(EcoString, List<WorkType<BigInt>>), ()>>;
    let nested = bindings
        .function(FunctionDeclaration::<(), Nested>::new("nested"))
        .expect("nested entry");
    let mut module = bindings.seal().expect("sealed execution");
    let mut state = ();
    let mut echo = Echo::default();
    let mut run =
        Box::pin(
            module.with_execution(&execution_host, &mut state, &mut echo, async |scope| {
                assert_eq!(
                    scope
                        .call(&double, (21.into(),))
                        .await
                        .expect("direct result"),
                    BigInt::from(42)
                );
                let work = scope.call(&delayed, ()).await.expect("work construction");
                let completed = scope.observe(&work).await.expect("completion");
                let alias = scope
                    .observe(&work)
                    .await
                    .expect("repeated observation")
                    .clone();
                completed.read(|first| alias.read(|second| assert!(std::ptr::eq(first, second))));
                let nested = scope.call(&nested, ()).await.expect("nested construction");
                let completed_outer = scope.observe(&nested).await.expect("outer completion");
                let inner = completed_outer.read(|value| {
                    let (label, list) = value.expect("source Ok");
                    assert_eq!(label, "values");
                    assert_eq!(list.len(), 2);
                    list.read_item(1, |work| work).expect("second work")
                });
                let inner = scope.observe(&inner).await.expect("inner completion");
                assert_eq!(inner.read(Clone::clone), BigInt::from(42));
                completed
            }),
        );
    let Poll::Ready(Ok(completed)) = execution_host.poll(run.as_mut()) else {
        panic!("source-only ready work finishes while its host polls");
    };
    drop(run);
    assert_eq!(completed.read(Clone::clone), BigInt::from(42));
}

#[test]
fn public_future_inputs_retain_identity_and_do_not_construct_their_completion_type() {
    let execution_host = crate::execution_fixture::TestHost::default();

    let (mut bindings, delayed) = HostedModuleBuilder::new(program())
        .expect("planned library")
        .function(FunctionDeclaration::<(), WorkType<BigInt>>::new("delayed"))
        .expect("work entry");
    let keep = bindings
        .function(FunctionDeclaration::<(WorkType<BigInt>,), WorkType<BigInt>>::new("keep"))
        .expect("work identity");
    let keep_list = bindings
        .function(FunctionDeclaration::<
            (List<WorkType<BigInt>>,),
            List<WorkType<BigInt>>,
        >::new("keep_list"))
        .expect("work list");
    type ResultList = Result<List<BigInt>, EcoString>;
    let packet_work = bindings
        .function(FunctionDeclaration::<(), WorkType<ResultList>>::new(
            "packet_work",
        ))
        .expect("work with compound completion");
    let packet = bindings
        .function(FunctionDeclaration::<
            ((WorkType<ResultList>, ResultList),),
            (WorkType<ResultList>, ResultList),
        >::new("packet"))
        .expect("compound input");
    let mut module = bindings.seal().expect("sealed execution");
    let mut state = ();
    let mut echo = Echo::default();
    let mut run =
        Box::pin(
            module.with_execution(&execution_host, &mut state, &mut echo, async |scope| {
                let work = scope.call(&delayed, ()).await.expect("construction");
                let same = scope
                    .call(&keep, (&work,))
                    .await
                    .expect("borrowed work input");
                let same = scope
                    .call(&keep, (same.clone(),))
                    .await
                    .expect("owned work input");
                let borrowed = scope
                    .call(&keep_list, (vec![&work, &same],))
                    .await
                    .expect("borrowed work list");
                assert_eq!(borrowed.len(), 2);
                let completed = scope.observe(&work).await.expect("first completion");
                let alias = scope.observe(&same).await.expect("same completion");
                completed.read(|a| alias.read(|b| assert!(std::ptr::eq(a, b))));
                let list = scope
                    .call(&keep_list, (vec![work, same],))
                    .await
                    .expect("fresh list input");
                let list = scope
                    .call(&keep_list, (&list,))
                    .await
                    .expect("same-owner retained list input");
                let first = list.read_item(0, |value| value).expect("first work");
                let second = list.read_item(1, |value| value).expect("second work");
                let first = scope.observe(&first).await.expect("first list completion");
                let second = scope
                    .observe(&second)
                    .await
                    .expect("second list completion");
                first.read(|a| second.read(|b| assert!(std::ptr::eq(a, b))));
                let work = scope.call(&packet_work, ()).await.expect("packet work");
                for input in [Ok(vec![BigInt::from(3)]), Err(EcoString::from("message"))] {
                    let (work, result) = scope
                        .call(&packet, ((&work, input.clone()),))
                        .await
                        .expect("packet input");
                    let actual =
                        result.map(|list| list.read_item(0, Clone::clone).expect("list item"));
                    assert_eq!(actual, input.map(|mut list| list.remove(0)));
                    let result = scope.observe(&work).await.expect("packet completion");
                    result.read(|result| {
                        assert_eq!(
                            result.expect("Ok list").read_item(1, Clone::clone),
                            Some(BigInt::from(2))
                        )
                    });
                }
            }),
        );
    assert!(matches!(
        execution_host.poll(run.as_mut()),
        Poll::Ready(Ok(()))
    ));
}

struct NativeProfile;
struct NativeProvider;

struct NativeState {
    gate: Option<futures_channel::oneshot::Receiver<Result<BigInt, geam_core::HostFailure>>>,
    started: Option<futures_channel::oneshot::Sender<()>>,
    touches: std::cell::Cell<usize>,
    unit: (),
}

impl HostProfile for NativeProfile {
    type RunState = NativeState;
    type ExternalStores = HostFutureStore;
}

impl geam_core::HostProvider<NativeProfile> for NativeProvider {
    type State = NativeState;
    fn project(state: &mut NativeState) -> &mut NativeState {
        state
    }
}

impl geam_core::host::HostWorkProfile for NativeProfile {
    type Work = crate::work_fixture::WorkComponent;
}
impl HostComponentProfile<WorkComponent> for NativeProfile {
    fn component_stores(stores: &HostFutureStore) -> &HostFutureStore {
        stores
    }
    fn component_state(state: &mut NativeState) -> &mut () {
        &mut state.unit
    }
}

fn native_work<'call>(
    mut call: geam_core::host::HostCall<
        'call,
        NativeProfile,
        NativeProvider,
        crate::work_fixture::WorkHostType<BigInt>,
    >,
    constructions: geam_core::HostConstructions<'call, geam_core::HostTypeListEnd>,
) -> Result<
    geam_core::HostCallCompletion<'call, crate::work_fixture::WorkHostType<BigInt>>,
    geam_core::HostCallError,
> {
    let gate = call.state().gate.take().expect("one native construction");
    let started = call.state().started.take();
    Ok(call.return_future(constructions, move |context| {
        Box::pin(async move {
            context
                .with_state(|state| state.touches.set(state.touches.get() + 1))
                .await?;
            if let Some(started) = started {
                started.send(()).expect("active host checkpoint");
            }
            let value = gate.await.expect("host controlled gate")?;
            context
                .with_state(|state| state.touches.set(state.touches.get() + 1))
                .await?;
            Ok(geam_core::host::HostOwnedCompletion::new(move |call, _| {
                Ok(call.return_value(value))
            }))
        })
    }))
}

fn await_future_callback<'call>(
    call: geam_core::host::HostCall<
        'call,
        NativeProfile,
        NativeProvider,
        crate::work_fixture::WorkHostType<BigInt>,
    >,
    constructions: geam_core::HostConstructions<'call, geam_core::HostTypeListEnd>,
    callback: geam_core::host::HostCallable<
        'call,
        IntCallbackArguments,
        crate::work_fixture::WorkHostType<BigInt>,
    >,
) -> Result<
    geam_core::HostCallCompletion<'call, crate::work_fixture::WorkHostType<BigInt>>,
    geam_core::HostCallError,
> {
    let callback = call.owned_callable(callback, &constructions);
    Ok(call.return_future(constructions, move |context| {
        Box::pin(async move {
            let work = callback
                .invoke(
                    context.execution(),
                    |_, _| (BigInt::from(1), ()),
                    |call, _, value| Ok(call.future_value(value)),
                )
                .await?;
            context
                .with_state(|state| {
                    assert_eq!(
                        state.touches.get(),
                        0,
                        "receiving a Future does not poll it"
                    );
                    assert!(
                        state.gate.is_none(),
                        "the callback constructed the native work"
                    );
                })
                .await?;
            let first = work.observe(&context, |_, value| Ok(value)).await;
            let second = work.clone().observe(&context, |_, value| Ok(value)).await;
            match (&first, &second) {
                (Ok(first), Ok(second)) => assert_eq!(first, second),
                (
                    Err(geam_core::host::HostExecutionError::Execution(first)),
                    Err(geam_core::host::HostExecutionError::Execution(second)),
                ) => {
                    first.read(|a| second.read(|b| assert!(std::ptr::eq(a, b))));
                }
                _ => panic!("repeated native observations share one completion"),
            }
            let value = first?;
            Ok(geam_core::host::HostOwnedCompletion::new(move |call, _| {
                Ok(call.return_value(value))
            }))
        })
    }))
}

#[test]
fn native_code_receives_and_explicitly_drives_a_future_valued_gleam_callback() {
    let execution_host = crate::execution_fixture::TestHost::default();

    use crate::work_fixture::WorkHostType;
    use geam_core::host::{HostFunctionType, HostProviderModule};
    for (native_succeeds, source_succeeds) in [(true, true), (false, true), (true, false)] {
        let native = HostProviderModule::new("app", "app").expect("native module")
            .with_scoped_function_and_constructions::<NativeProvider, (), WorkHostType<BigInt>, geam_core::HostTypeListEnd, _>("fetch", native_work).expect("fetch registration")
            .with_scoped_function_and_constructions::<NativeProvider, (HostFunctionType<IntCallbackArguments, WorkHostType<BigInt>>,), WorkHostType<BigInt>, geam_core::HostTypeListEnd, _>("await_callback", await_future_callback).expect("callback registration");
        let mut providers =
            WorkComponent::providers::<NativeProfile>().expect("Future registration");
        providers.push(native);
        let typed = compile_typed_host_program(
            "app",
            "app",
            [
                PackageSource::new(
                    "work_fixture",
                    Vec::<String>::new(),
                    [ModuleSource::new(
                        "fixture/work",
                        "src/fixture/work.gleam",
                        crate::work_fixture::WorkComponent::SOURCE,
                    )],
                ),
                PackageSource::new(
                    "app",
                    ["work_fixture"],
                    [ModuleSource::new(
                        "app",
                        "src/app.gleam",
                        r#"
import fixture/work as future
@external(erlang, "native", "fetch")
fn fetch() -> future.Work(Int)
@external(erlang, "native", "await_callback")
fn await_callback(callback: fn(Int) -> future.Work(Int)) -> future.Work(Int)
fn checked(value: Int, succeeds: Bool) -> Int {
  let assert True = succeeds
  echo value
  value
}
pub fn work(succeeds: Bool) -> future.Work(Int) {
  await_callback(fn(offset) {
    use value <- future.map(fetch())
    checked(value + offset, succeeds)
  })
}
"#,
                    )],
                ),
            ],
            HostProviderSet::from_providers(providers).expect("providers"),
        )
        .expect("source");
        let (bindings, work) = HostedModuleBuilder::new(typed)
            .expect("plan")
            .function(FunctionDeclaration::<(bool,), WorkType<BigInt>>::new(
                "work",
            ))
            .expect("entry");
        let mut module = bindings.seal().expect("sealed work");
        let (send, receive) = futures_channel::oneshot::channel();
        let mut state = NativeState {
            gate: Some(receive),
            started: None,
            touches: std::cell::Cell::new(0),
            unit: (),
        };
        let mut echo = Echo::default();
        let mut task = Box::pin(module.with_execution(
            &execution_host,
            &mut state,
            &mut echo,
            async |scope| {
                let work = scope
                    .call(&work, (source_succeeds,))
                    .await
                    .expect("constructed work");
                scope.observe(&work).await
            },
        ));
        assert!(execution_host.poll(task.as_mut()).is_pending());
        send.send(if native_succeeds {
            Ok(41.into())
        } else {
            Err(geam_core::HostFailure::new("fetch failed"))
        })
        .expect("active dependency");
        let Poll::Ready(Ok(result)) = execution_host.poll(task.as_mut()) else {
            panic!("the explicit dependency must resume");
        };
        drop(task);
        if native_succeeds && source_succeeds {
            assert_eq!(
                result.expect("completion").read(Clone::clone),
                BigInt::from(42)
            );
            assert_eq!(echo.0.len(), 1);
        } else {
            let Err(geam_core::embedding::ObservationError::Execution(error)) = result else {
                panic!("original execution failure");
            };
            error.read(|error| match error {
                geam_core::ExecutionError::Host(error) => {
                    assert!(!native_succeeds);
                    assert_eq!(error.function(), "fetch");
                    assert!(error.to_string().contains("fetch failed"));
                }
                geam_core::ExecutionError::Panic(error) => {
                    assert!(!source_succeeds);
                    assert_eq!(error.site().function(), "checked");
                    assert_eq!(error.kind(), geam_core::PanicKind::LetAssert);
                }
                _ => panic!("source and provider failures keep their origin"),
            });
            assert!(echo.0.is_empty());
        }
        assert_eq!(state.touches.get(), if native_succeeds { 2 } else { 1 });
    }
}

#[test]
fn public_pending_work_and_shared_completion_transfer_with_borrowed_send_only_state() {
    let execution_host = crate::execution_fixture::TestHost::default();

    use crate::work_fixture::WorkHostType;
    use geam_core::host::HostProviderModule;
    for succeeds in [true, false] {
        let native = HostProviderModule::new("app", "app").expect("native module")
            .with_scoped_function_and_constructions::<NativeProvider, (), WorkHostType<BigInt>, geam_core::HostTypeListEnd, _>("fetch", native_work).expect("native registration");
        let mut providers =
            WorkComponent::providers::<NativeProfile>().expect("Future registration");
        providers.push(native);
        let typed = compile_typed_host_program(
            "app",
            "app",
            [
                PackageSource::new(
                    "work_fixture",
                    Vec::<String>::new(),
                    [ModuleSource::new(
                        "fixture/work",
                        "src/fixture/work.gleam",
                        crate::work_fixture::WorkComponent::SOURCE,
                    )],
                ),
                PackageSource::new(
                    "app",
                    ["work_fixture"],
                    [ModuleSource::new(
                        "app",
                        "src/app.gleam",
                        r#"
import fixture/work as future
@external(erlang, "native", "fetch")
fn fetch() -> future.Work(Int)
pub fn work() {
  use value <- future.map(fetch())
  echo value
  value + 1
}

"#,
                    )],
                ),
            ],
            HostProviderSet::from_providers(providers).expect("providers"),
        )
        .expect("native source");
        let (bindings, work) = HostedModuleBuilder::new(typed)
            .expect("native plan")
            .function(FunctionDeclaration::<(), WorkType<BigInt>>::new("work"))
            .expect("native entry");
        let mut module = bindings.seal().expect("native seal");
        let (send, receive) = futures_channel::oneshot::channel();
        let mut state = NativeState {
            gate: Some(receive),
            started: None,
            touches: std::cell::Cell::new(0),
            unit: (),
        };
        let mut echo = Echo::default();
        let mut task = Box::pin(module.with_execution(
            &execution_host,
            &mut state,
            &mut echo,
            async |scope| {
                let work = scope.call(&work, ()).await.expect("native construction");
                let first = scope.observe(&work).await;
                let second = scope.observe(&work).await;
                match (&first, &second) {
                    (Ok(first), Ok(second)) => {
                        first.read(|a| second.read(|b| assert!(std::ptr::eq(a, b))))
                    }
                    (
                        Err(geam_core::embedding::ObservationError::Execution(first)),
                        Err(geam_core::embedding::ObservationError::Execution(second)),
                    ) => first.read(|a| second.read(|b| assert!(std::ptr::eq(a, b)))),
                    _ => panic!("the same operation preserves one completion"),
                }
                first
            },
        ));
        let driver = &execution_host;
        let completion = std::thread::scope(|threads| {
            task = threads
                .spawn(move || {
                    assert!(driver.poll(task.as_mut()).is_pending());
                    task
                })
                .join()
                .expect("first worker");
            send.send(if succeeds {
                Ok(41.into())
            } else {
                Err(geam_core::HostFailure::new("native failure"))
            })
            .expect("live work");
            threads
                .spawn(move || {
                    let result = driver.poll(task.as_mut());
                    let Poll::Ready(Ok(result)) = result else {
                        panic!("released gate completes");
                    };
                    result
                })
                .join()
                .expect("second worker")
        });
        if succeeds {
            assert_eq!(
                completion.expect("success").read(Clone::clone),
                BigInt::from(42)
            );
            assert_eq!(state.touches.get(), 2);
            assert_eq!(echo.0.len(), 1);
        } else {
            let error = match completion {
                Err(error) => error,
                Ok(_) => panic!("native failure expected"),
            };
            assert!(error.to_string().contains("native failure"));
            assert_eq!(state.touches.get(), 1);
            assert!(echo.0.is_empty());
        }
    }
}

#[test]
fn dropping_work_or_its_execution_releases_native_inputs_without_late_source_execution() {
    let execution_host = crate::execution_fixture::TestHost::default();

    use crate::work_fixture::WorkHostType;
    use geam_core::host::HostProviderModule;
    let native = HostProviderModule::new("app", "app")
        .expect("native module")
        .with_scoped_function_and_constructions::<NativeProvider, (), WorkHostType<BigInt>, geam_core::HostTypeListEnd, _>("fetch", native_work)
        .expect("native registration");
    let mut providers = WorkComponent::providers::<NativeProfile>().expect("Future registration");
    providers.push(native);
    let typed = compile_typed_host_program(
        "app",
        "app",
        [
            PackageSource::new(
                "work_fixture",
                Vec::<String>::new(),
                [ModuleSource::new(
                    "fixture/work",
                    "src/fixture/work.gleam",
                    crate::work_fixture::WorkComponent::SOURCE,
                )],
            ),
            PackageSource::new(
                "app",
                ["work_fixture"],
                [ModuleSource::new(
                    "app",
                    "src/app.gleam",
                    r#"
import fixture/work as future
@external(erlang, "native", "fetch")
fn fetch() -> future.Work(Int)
pub fn work() {
  use value <- future.map(fetch())
  echo value
  value + 1
}
"#,
                )],
            ),
        ],
        HostProviderSet::from_providers(providers).expect("providers"),
    )
    .expect("source");
    let (bindings, work) = HostedModuleBuilder::new(typed)
        .expect("plan")
        .function(FunctionDeclaration::<(), WorkType<BigInt>>::new("work"))
        .expect("entry");
    let mut module = bindings.seal().expect("sealed execution");
    for poll_work in [false, true] {
        for drop_work in [false, true] {
            let (send, receive) = futures_channel::oneshot::channel();
            let (keep_scope, scope_wait) = futures_channel::oneshot::channel::<()>();
            let (started, mut native_started) = futures_channel::oneshot::channel();
            let mut state = NativeState {
                gate: Some(receive),
                started: Some(started),
                touches: std::cell::Cell::new(0),
                unit: (),
            };
            let mut echo = Echo::default();
            let mut task = Box::pin(module.with_execution(
                &execution_host,
                &mut state,
                &mut echo,
                async |scope| {
                    let work = scope.call(&work, ()).await.expect("native construction");
                    if poll_work {
                        let mut waiter = Box::pin(scope.observe(&work));
                        std::future::poll_fn(|cx| {
                            assert!(waiter.as_mut().poll(cx).is_pending());
                            std::pin::Pin::new(&mut native_started).poll(cx)
                        })
                        .await
                        .expect("native work reached the gate");
                        drop(waiter);
                    }
                    let retained = if drop_work {
                        drop(work);
                        None
                    } else {
                        Some(work)
                    };
                    scope_wait.await.expect("host checkpoint");
                    drop(retained);
                },
            ));
            assert!(execution_host.poll(task.as_mut()).is_pending());
            assert_eq!(
                send.is_canceled(),
                drop_work,
                "dropping a waiter does not cancel retained work"
            );
            drop(task);
            assert!(
                send.is_canceled(),
                "scope shutdown releases pending inputs even when work was retained"
            );
            assert!(
                send.send(Ok(41.into())).is_err(),
                "late completion cannot restart the operation"
            );
            assert!(keep_scope.is_canceled());
            assert_eq!(state.touches.get(), usize::from(poll_work));
            assert!(echo.0.is_empty());
        }
    }
}
