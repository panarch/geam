use geam_core::embedding::{
    AsyncCallError, BigInt, FunctionDeclaration, List, ObservationError, WorkModuleBuilder,
    with_execution_scope,
};
use geam_core::frontend::{TransferHostedTypedProgram, compile_typed_transfer_host_program};
use geam_core::host::{
    AsyncHostComponentProfile, HostFutureStore, HostProfile,
    TransferHostProviderComponentRegistration, TransferHostProviderSet,
};
use geam_core::{
    AsyncExecutionError, EchoOutput, EchoSink, ModuleSource, PackageSource, PanicKind, PanicMessage,
};
use geam_runtime_api::FutureComponent;
use geam_runtime_api::embedding::FutureType;
use std::future::Future;
use std::pin::pin;
use std::task::{Context, Poll, Waker};

include!("support/future_families.rs");

struct Profile;
#[derive(Default)]
struct State {
    provider: BigInt,
    future: (),
}
#[derive(Default)]
struct HostStores {
    provider: AsyncStores,
    future: HostFutureStore,
}

impl HostProfile for Profile {
    type RunState = State;
    type ExternalStores = HostStores;
}
impl AsyncHostComponentProfile<Component> for Profile {
    fn component_async_stores(stores: &HostStores) -> &AsyncStores {
        &stores.provider
    }
    fn component_state(state: &mut State) -> &mut BigInt {
        &mut state.provider
    }
}
impl geam_core::host::HostWorkProfile for Profile {
    type Work = FutureComponent;
}
impl AsyncHostComponentProfile<FutureComponent> for Profile {
    fn component_async_stores(stores: &HostStores) -> &HostFutureStore {
        &stores.future
    }
    fn component_state(state: &mut State) -> &mut () {
        &mut state.future
    }
}

#[derive(Default)]
struct Echo(usize);
impl EchoSink for Echo {
    fn emit(&mut self, _output: EchoOutput) {
        self.0 += 1;
    }
}

fn program() -> TransferHostedTypedProgram<Profile> {
    compile(include_str!("fixtures/future_families/main.gleam"))
        .expect("explicit Future source linkage")
}

fn compile(source: &str) -> Result<TransferHostedTypedProgram<Profile>, geam_core::FrontendError> {
    let mut providers = FutureComponent::providers().expect("Future component");
    providers.extend(
        <Component as TransferHostProviderComponentRegistration<Profile>>::providers()
            .expect("macro-authored component"),
    );
    compile_typed_transfer_host_program(
        "application",
        "main",
        [
            PackageSource::new(
                "geam",
                Vec::<String>::new(),
                [ModuleSource::new(
                    "geam/future",
                    "src/geam/future.gleam",
                    include_str!("../../builtins/geam/gleam/src/geam/future.gleam"),
                )],
            ),
            PackageSource::new(
                "async_provider",
                ["gleam_stdlib", "geam"],
                [
                    ModuleSource::new(
                        "async_provider/declarations",
                        "src/async_provider/declarations.gleam",
                        "@external(erlang, \"declarations\", \"Token\")\npub type Token",
                    ),
                    ModuleSource::new(
                        "async_provider/native",
                        "src/async_provider/native.gleam",
                        include_str!("fixtures/future_families/native.gleam"),
                    ),
                ],
            ),
            PackageSource::new(
                "gleam_stdlib",
                Vec::<String>::new(),
                [ModuleSource::new(
                    "gleam/option",
                    "src/gleam/option.gleam",
                    "pub type Option(value) { Some(value) None }",
                )],
            ),
            PackageSource::new(
                "application",
                ["async_provider", "gleam_stdlib", "geam"],
                [ModuleSource::new("main", "src/main.gleam", source)],
            ),
        ],
        TransferHostProviderSet::new(providers).expect("provider set"),
    )
}

#[test]
fn an_ordinary_callback_cannot_hide_a_future_return_type() {
    let result = compile(
        r#"
import async_provider/native
pub fn invalid() {
  native.invoke_immediate(fn(value) { native.add_one(value) }, 1)
}
"#,
    );
    let Err(geam_core::FrontendError::Analyse { errors }) = result else {
        panic!("Future(Int) must fail the ordinary Int callback contract during type analysis");
    };
    assert_eq!(errors.len(), 1);
    let diagnostic = format!("{:?}", errors[0]);
    assert!(diagnostic.contains("CouldNotUnify"), "{diagnostic}");
    assert!(diagnostic.contains("Future"), "{diagnostic}");
}

#[test]
fn manual_payloads_retain_rich_values_and_work_across_native_suspension() {
    assert_work_checks("manual_retained");
    assert_work_checks("manual_retained_work");
}

fn assert_work_checks(entry: &str) {
    let (bindings, function) = WorkModuleBuilder::new(program())
        .expect("plan")
        .function(FunctionDeclaration::<(), FutureType<bool>>::new(entry))
        .expect("work entry");
    let mut module = bindings.seal().expect("typed source callbacks");
    let mut state = State::default();
    let mut echo = Echo::default();
    poll_ready(with_execution_scope(async |guard| {
        let mut scope = module.attach(guard, &mut state, &mut echo);
        let work = scope.call(&function, ()).expect("construct checks");
        let first = scope.observe(&work).await.expect("complete checks");
        assert!(first.read(|value| value));
        let again = scope
            .observe(&work)
            .await
            .expect("same completed operation");
        assert!(again.read(|value| value));
    }));
    assert_eq!(echo.0, 0);
}

#[test]
fn direct_scalar_generic_and_lazy_values_keep_their_ordinary_call_path() {
    let (bindings, function) = WorkModuleBuilder::new(program())
        .expect("plan")
        .function(FunctionDeclaration::<(), bool>::new("direct_families"))
        .expect("ordinary entry");
    let mut module = bindings.seal().expect("sealed");
    let mut state = State::default();
    let mut echo = Echo::default();
    poll_ready(with_execution_scope(async |guard| {
        let mut scope = module.attach(guard, &mut state, &mut echo);
        assert!(scope.call(&function, ()).expect("direct checks"));
    }));
    assert_eq!(echo.0, 0);
}

#[test]
fn native_work_keeps_zero_to_seven_arguments_and_every_scalar_completion() {
    assert_work_checks("arity_families");
}

#[test]
fn generic_owned_values_keep_every_existing_source_family() {
    assert_work_checks("owned_generic_families");
}
#[test]
fn delayed_generic_callbacks_keep_specialization_and_returned_function_targets() {
    assert_work_checks("callback_generic_families");
}
#[test]
fn typed_callbacks_keep_construction_permissions_and_drop_unobserved_requests() {
    assert_work_checks("callback_typed_families");
}
#[test]
fn owned_collections_decode_lazily_and_construct_typed_outputs() {
    assert_work_checks("owned_collections");
}
#[test]
fn external_and_stored_values_keep_original_identity_after_native_suspension() {
    assert_work_checks("owned_externals");
}

#[test]
fn direct_calls_and_pending_work_share_one_caller_owned_state() {
    let (mut bindings, direct) = WorkModuleBuilder::new(program())
        .expect("plan")
        .function(FunctionDeclaration::<(BigInt,), BigInt>::new("direct"))
        .expect("direct");
    let awaited = bindings
        .function(FunctionDeclaration::<(BigInt,), FutureType<BigInt>>::new(
            "awaited",
        ))
        .expect("work");
    let stateful = bindings
        .function(FunctionDeclaration::<(BigInt,), FutureType<BigInt>>::new(
            "stateful",
        ))
        .expect("state work");
    let stateful_direct = bindings
        .function(FunctionDeclaration::<(BigInt,), (BigInt, BigInt)>::new(
            "stateful_direct",
        ))
        .expect("state direct");
    let mut module = bindings.seal().expect("sealed");
    let mut state = State {
        provider: 10.into(),
        future: (),
    };
    let mut echo = Echo::default();
    let mut task = pin!(with_execution_scope(async |guard| {
        let mut scope = module.attach(guard, &mut state, &mut echo);
        assert_eq!(
            scope.call(&direct, (21.into(),)).expect("direct result"),
            BigInt::from(42)
        );
        assert_eq!(
            scope
                .call(&stateful_direct, (2.into(),))
                .expect("state access"),
            (BigInt::from(10), BigInt::from(12))
        );
        let value = scope.call(&awaited, (41.into(),)).expect("construct");
        let value = scope.observe(&value).await.expect("native wait");
        assert_eq!(value.read(Clone::clone), BigInt::from(42));
        let value = scope
            .call(&stateful, (5.into(),))
            .expect("state construction");
        // Construction has not applied the queued mutation.
        assert_eq!(
            scope
                .call(&stateful_direct, (0.into(),))
                .expect("before driving"),
            (BigInt::from(12), BigInt::from(12))
        );
        let first = scope.observe(&value).await.expect("state request");
        let second = scope.observe(&value).await.expect("cached state result");
        assert_eq!(first.read(Clone::clone), BigInt::from(17));
        first.read(|a| second.read(|b| assert!(std::ptr::eq(a, b))));
        assert_eq!(
            scope
                .call(&stateful_direct, (0.into(),))
                .expect("after driving"),
            (BigInt::from(17), BigInt::from(17))
        );
    }));
    assert!(
        task.as_mut()
            .poll(&mut Context::from_waker(Waker::noop()))
            .is_pending()
    );
    poll_ready(task);
}

#[test]
fn retained_list_results_pass_back_to_native_work_without_materialization() {
    let (mut bindings, direct) = WorkModuleBuilder::new(program())
        .expect("plan")
        .function(FunctionDeclaration::<(List<BigInt>,), List<BigInt>>::new(
            "list_direct",
        ))
        .expect("direct List");
    let work = bindings
        .function(FunctionDeclaration::<
            (List<BigInt>,),
            FutureType<List<BigInt>>,
        >::new("list_awaited"))
        .expect("work List");
    let mut module = bindings.seal().expect("sealed");
    let mut state = State::default();
    let mut echo = Echo::default();
    poll_ready(with_execution_scope(async |guard| {
        let mut scope = module.attach(guard, &mut state, &mut echo);
        let list = scope
            .call(&direct, (vec![BigInt::from(1), BigInt::from(2)],))
            .expect("list");
        let work = scope.call(&work, (&list,)).expect("retain input");
        let result = scope.observe(&work).await.expect("list completion");
        result.read(|result| {
            assert_eq!(result.len(), 2);
            for index in 0..2 {
                list.read_item(index, |left| {
                    result.read_item(index, |right| {
                        assert_eq!(left, right);
                        assert!(std::ptr::eq(left, right));
                    })
                })
                .expect("retained item");
            }
        });
    }));
}

#[test]
fn direct_failures_keep_the_provider_or_source_origin() {
    let (mut bindings, direct) = WorkModuleBuilder::new(program())
        .expect("plan")
        .function(FunctionDeclaration::<(), BigInt>::new("direct_failure"))
        .expect("direct failure");
    let nil = bindings
        .function(FunctionDeclaration::<(), ()>::new("nil_failure"))
        .expect("Nil");
    let panic = bindings
        .function(FunctionDeclaration::<(), BigInt>::new(
            "callback_direct_panic",
        ))
        .expect("panic");
    let mut module = bindings.seal().expect("sealed");
    let mut state = State::default();
    let mut echo = Echo::default();
    poll_ready(with_execution_scope(async |guard| {
        let mut scope = module.attach(guard, &mut state, &mut echo);
        let AsyncCallError::Execution(AsyncExecutionError::Host(error)) =
            scope.call(&direct, ()).expect_err("direct failure")
        else {
            panic!("host origin");
        };
        assert_eq!(error.package(), "async_provider");
        assert_eq!(error.module(), "async_provider/native");
        assert_eq!(error.function(), "fail_direct");
        assert_eq!(error.failure().message(), "immediate provider failed");
        let AsyncCallError::Execution(AsyncExecutionError::Host(error)) =
            scope.call(&nil, ()).expect_err("Nil failure")
        else {
            panic!("Nil host origin");
        };
        assert_eq!(error.function(), "fail_nil");
        assert_eq!(error.failure().message(), "immediate Nil provider failed");
        let AsyncCallError::Execution(AsyncExecutionError::Panic(error)) =
            scope.call(&panic, ()).expect_err("callback panic")
        else {
            panic!("source origin");
        };
        assert_eq!(error.kind(), PanicKind::Panic);
        assert_eq!(error.site().module(), "main");
        assert_eq!(error.site().function(), "panic_immediately");
        assert_eq!(
            error.message(),
            &PanicMessage::Explicit("immediate callback panic".into())
        );
    }));
}

#[test]
fn future_and_composed_provider_failures_share_the_original_failure() {
    for entry in [
        "future_failure",
        "composed_failure",
        "callback_future_failure",
    ] {
        assert_work_failure(entry, |error| {
            let AsyncExecutionError::Host(error) = error else {
                panic!("host origin");
            };
            assert_eq!(error.package(), "async_provider");
            assert_eq!(error.module(), "async_provider/native");
            assert_eq!(error.function(), "fail_async");
            assert_eq!(error.failure().message(), "async provider failed");
        });
    }
}

#[test]
fn delayed_source_panics_keep_their_function_and_never_return_family() {
    for (entry, function, message) in [
        (
            "callback_panic",
            "panicking_callback",
            "async callback panic",
        ),
        (
            "callback_never",
            "never_callback",
            "async never callback panic",
        ),
    ] {
        assert_work_failure(entry, |error| {
            let AsyncExecutionError::Panic(error) = error else {
                panic!("source origin");
            };
            assert_eq!(error.kind(), PanicKind::Panic);
            assert_eq!(error.site().module(), "main");
            assert_eq!(error.site().function(), function);
            assert_eq!(error.message(), &PanicMessage::Explicit(message.into()));
        });
    }
}

fn assert_work_failure(entry: &str, inspect: impl Fn(&AsyncExecutionError)) {
    let (bindings, function) = WorkModuleBuilder::new(program())
        .expect("plan")
        .function(FunctionDeclaration::<(), FutureType<BigInt>>::new(entry))
        .expect("work entry");
    let mut module = bindings.seal().expect("sealed");
    let mut state = State::default();
    let mut echo = Echo::default();
    poll_ready(with_execution_scope(async |guard| {
        let mut scope = module.attach(guard, &mut state, &mut echo);
        let work = scope.call(&function, ()).expect("construct failure");
        let Err(ObservationError::Execution(first)) = scope.observe(&work).await else {
            panic!("original execution error");
        };
        let Err(ObservationError::Execution(second)) = scope.observe(&work).await else {
            panic!("cached execution error");
        };
        first.read(|a| {
            inspect(a);
            second.read(|b| assert!(std::ptr::eq(a, b)));
        });
    }));
    assert_eq!(echo.0, 0);
}

fn poll_ready<Output>(future: impl Future<Output = Output>) -> Output {
    let mut future = pin!(future);
    for _ in 0..128 {
        if let Poll::Ready(value) = future
            .as_mut()
            .poll(&mut Context::from_waker(Waker::noop()))
        {
            return value;
        }
    }
    panic!("controlled work did not finish")
}
