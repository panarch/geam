mod context;

pub use context::{
    AsyncHostExternalEquality, AsyncHostExternalHashing, AsyncHostExternalInspection,
};
pub(crate) use context::{
    TransferExternalEquality, TransferExternalHashing, TransferExternalInspection,
};

use super::{HostExternalSchema, HostProfile, HostProvider};
use crate::runtime::{TransferExternalPayloadLease, TransferExternalStore};
use ecow::EcoString;

/// Module-owned storage for immutable external payloads used by async embedding.
///
/// Payloads need `Send`, not `Sync`. Each short payload view is serialized; no
/// reference into the store can escape that view or remain live across `.await`.
pub struct AsyncHostExternalStore<Payload> {
    inner: TransferExternalStore<Payload>,
}

/// Source semantics and storage for an external type in transferable embedding.
///
/// Equal payloads must have equal source hashes. These hashes are runtime
/// indexes, not a stable serialization format.
pub trait AsyncHostExternalStorage<Profile, Schema>: Send + Sync + 'static
where
    Profile: HostProfile,
    Schema: HostExternalSchema,
{
    type Payload: 'static;

    fn store(stores: &Profile::ExternalStores) -> &AsyncHostExternalStore<Self::Payload>;
    fn source_equal(
        context: &AsyncHostExternalEquality<'_>,
        left: &Self::Payload,
        right: &Self::Payload,
    ) -> bool;
    fn source_hash(context: &AsyncHostExternalHashing<'_>, value: &Self::Payload) -> u64;
    fn inspect(context: &AsyncHostExternalInspection<'_>, value: &Self::Payload) -> EcoString;
}

/// Selects one typed async store for a source-declared external schema.
pub trait AsyncHostExternalBinding<Profile, Schema>: HostProvider<Profile>
where
    Profile: HostProfile,
    Schema: HostExternalSchema,
{
    type Storage: AsyncHostExternalStorage<Profile, Schema>;
}

impl<Payload> Default for AsyncHostExternalStore<Payload> {
    fn default() -> Self {
        Self {
            inner: TransferExternalStore::default(),
        }
    }
}

impl<Payload: Send + 'static> AsyncHostExternalStore<Payload> {
    pub(crate) fn clone_handle(&self) -> Self {
        Self {
            inner: self.inner.clone_handle(),
        }
    }

    pub(crate) fn insert<Profile, Schema, Storage>(
        &self,
        payload: Payload,
    ) -> TransferExternalPayloadLease
    where
        Profile: HostProfile,
        Schema: HostExternalSchema,
        Storage: AsyncHostExternalStorage<Profile, Schema, Payload = Payload>,
    {
        self.inner.insert(
            payload,
            |context, left, right| {
                Storage::source_equal(&AsyncHostExternalEquality(context), left, right)
            },
            |context, value| Storage::source_hash(&AsyncHostExternalHashing(context), value),
            |context, value| Storage::inspect(&AsyncHostExternalInspection(context), value),
        )
    }

    pub(crate) fn with_view<Output>(
        &self,
        lease: &TransferExternalPayloadLease,
        view: impl FnOnce(&Payload) -> Output,
    ) -> Output {
        self.inner.with_view(lease, view)
    }

    pub(crate) fn view(
        &self,
        lease: &TransferExternalPayloadLease,
    ) -> crate::runtime::TransferExternalPayloadView<Payload> {
        self.inner.view(lease)
    }
}

#[cfg(test)]
mod tests {
    use super::{
        AsyncHostExternalBinding, AsyncHostExternalEquality, AsyncHostExternalHashing,
        AsyncHostExternalInspection, AsyncHostExternalStorage, AsyncHostExternalStore,
        HostExternalSchema, HostProfile, HostProvider,
    };
    use crate::embedding::{FunctionDeclaration, WorkModuleBuilder, with_execution_scope};
    use crate::frontend::compile_typed_transfer_host_program;
    use crate::host::{
        AsyncHostComponentProfile, HostConstructions, HostFutureCompletion, HostFutureStore,
        TransferHostCall, TransferHostProviderModule, TransferHostProviderSet,
    };
    use crate::runtime::{StoredRuntimeValue, TransferValues};
    use crate::work_fixture::WorkType;
    use crate::work_fixture::{WorkComponent, WorkHostType};
    use crate::{
        AsyncHostCallError, HostCallCompletion, HostExternal, HostExternalType, HostTypeListEnd,
        ModuleSource, PackageSource,
    };
    use ecow::EcoString;
    use futures_util::FutureExt;
    use num_bigint::BigInt;
    use std::cell::Cell;
    use std::future::{Future, poll_fn};
    use std::sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    };
    use std::task::{Context, Poll, Waker};

    pub(super) struct Profile;
    pub(super) struct Provider;
    pub(super) struct Counter;
    pub(super) struct Envelope;
    pub(super) struct CounterStorage;
    pub(super) struct EnvelopeStorage;

    #[derive(Default)]
    pub(super) struct Echo(pub(super) Vec<String>);

    impl crate::EchoSink for Echo {
        fn emit(&mut self, output: crate::EchoOutput) {
            self.0.push(output.to_string());
        }
    }

    type State = Arc<AtomicUsize>;

    pub(super) struct CounterPayload {
        value: Cell<usize>,
        drops: Arc<AtomicUsize>,
    }

    impl Drop for CounterPayload {
        fn drop(&mut self) {
            self.drops.fetch_add(1, Ordering::SeqCst);
        }
    }

    #[derive(Default)]
    pub(super) struct Stores {
        counters: AsyncHostExternalStore<CounterPayload>,
        envelopes: AsyncHostExternalStore<StoredRuntimeValue<TransferValues>>,
        futures: HostFutureStore,
        // The aggregate, as well as its payload, must work without Sync.
        _exclusive: std::marker::PhantomData<Cell<()>>,
    }

    impl HostProfile for Profile {
        type RunState = (State, ());
        type ExternalStores = Stores;
    }

    impl crate::host::HostWorkProfile for Profile {
        type Work = crate::work_fixture::WorkComponent;
    }
    impl AsyncHostComponentProfile<WorkComponent> for Profile {
        fn component_async_stores(stores: &Stores) -> &HostFutureStore {
            &stores.futures
        }
        fn component_state(state: &mut (State, ())) -> &mut () {
            &mut state.1
        }
    }

    impl HostProvider<Profile> for Provider {
        type State = State;
        fn project(state: &mut (State, ())) -> &mut Self::State {
            &mut state.0
        }
    }

    impl HostExternalSchema for Counter {
        const PACKAGE: &'static str = "application";
        const MODULE: &'static str = "library";
        const NAME: &'static str = "Counter";
        const PARAMETER_COUNT: usize = 0;
    }

    impl HostExternalSchema for Envelope {
        const PACKAGE: &'static str = "application";
        const MODULE: &'static str = "library";
        const NAME: &'static str = "Envelope";
        const PARAMETER_COUNT: usize = 0;
    }

    impl AsyncHostExternalBinding<Profile, Counter> for Provider {
        type Storage = CounterStorage;
    }
    impl AsyncHostExternalBinding<Profile, Envelope> for Provider {
        type Storage = EnvelopeStorage;
    }

    impl AsyncHostExternalStorage<Profile, Counter> for CounterStorage {
        type Payload = CounterPayload;
        fn store(stores: &Stores) -> &AsyncHostExternalStore<Self::Payload> {
            &stores.counters
        }
        fn source_equal(
            _: &AsyncHostExternalEquality<'_>,
            left: &CounterPayload,
            right: &CounterPayload,
        ) -> bool {
            left.value.get() == right.value.get()
        }
        fn source_hash(_: &AsyncHostExternalHashing<'_>, value: &CounterPayload) -> u64 {
            value.value.get() as u64
        }
        fn inspect(_: &AsyncHostExternalInspection<'_>, value: &CounterPayload) -> EcoString {
            format!("Counter({})", value.value.get()).into()
        }
    }

    impl AsyncHostExternalStorage<Profile, Envelope> for EnvelopeStorage {
        type Payload = StoredRuntimeValue<TransferValues>;
        fn store(stores: &Stores) -> &AsyncHostExternalStore<Self::Payload> {
            &stores.envelopes
        }
        fn source_equal(
            context: &AsyncHostExternalEquality<'_>,
            left: &Self::Payload,
            right: &Self::Payload,
        ) -> bool {
            context.provider_stored_values_equal(left, right)
        }
        fn source_hash(context: &AsyncHostExternalHashing<'_>, value: &Self::Payload) -> u64 {
            context.provider_stored_value_hash(value)
        }
        fn inspect(context: &AsyncHostExternalInspection<'_>, value: &Self::Payload) -> EcoString {
            format!("Envelope({})", context.provider_inspect_stored_value(value)).into()
        }
    }

    pub(super) fn make_counter(
        mut call: TransferHostCall<'_, Profile, Provider, HostExternalType<Counter>>,
    ) -> Result<HostCallCompletion<'_, HostExternalType<Counter>>, AsyncHostCallError> {
        let drops = Arc::clone(call.state());
        let value = call.create_external_with_binding::<Provider>(CounterPayload {
            value: Cell::new(41),
            drops,
        });
        Ok(call.return_value(value))
    }

    pub(super) fn program(
        source: &str,
        provider: TransferHostProviderModule<Profile>,
    ) -> crate::frontend::TransferHostedTypedProgram<Profile> {
        let mut providers = WorkComponent::providers().expect("Future registration");
        providers.push(provider);
        compile_typed_transfer_host_program(
            "application",
            "library",
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
                    "application",
                    ["work_fixture"],
                    [ModuleSource::new("library", "src/library.gleam", source)],
                ),
            ],
            TransferHostProviderSet::new(providers).expect("provider set"),
        )
        .expect("external source")
    }

    #[test]
    fn direct_calls_create_read_and_retain_send_only_external_payloads() {
        fn read<'call>(
            call: TransferHostCall<'call, Profile, Provider, BigInt>,
            value: HostExternal<'call, HostExternalType<Counter>>,
        ) -> Result<HostCallCompletion<'call, BigInt>, AsyncHostCallError> {
            let value = {
                let payload = call.external_payload::<Counter, HostTypeListEnd>(value);
                BigInt::from(payload.value.get())
            };
            Ok(call.return_value(value))
        }
        fn wrap<'call>(
            mut call: TransferHostCall<'call, Profile, Provider, HostExternalType<Envelope>>,
            value: HostExternal<'call, HostExternalType<Counter>>,
        ) -> Result<HostCallCompletion<'call, HostExternalType<Envelope>>, AsyncHostCallError>
        {
            let payload = call.retain_value::<HostExternalType<Counter>>(value);
            let value = call.create_external_with_binding::<Provider>(payload);
            Ok(call.return_value(value))
        }
        fn hash<'call>(
            call: TransferHostCall<'call, Profile, Provider, BigInt>,
            value: HostExternal<'call, HostExternalType<Envelope>>,
        ) -> Result<HostCallCompletion<'call, BigInt>, AsyncHostCallError> {
            let hash = call.source_hash::<HostExternalType<Envelope>>(value);
            Ok(call.return_value(hash.into()))
        }
        let provider = TransferHostProviderModule::new_for_profile("application", "library")
            .expect("identity")
            .with_external_type::<Provider, Counter>().expect("counter schema")
            .with_external_type::<Provider, Envelope>().expect("envelope schema")
            .with_scoped_function::<Provider, (), HostExternalType<Counter>, _>("make", make_counter).expect("constructor")
            .with_scoped_function::<Provider, (HostExternalType<Counter>,), BigInt, _>("read", read).expect("read")
            .with_scoped_function::<Provider, (HostExternalType<Counter>,), HostExternalType<Envelope>, _>("wrap", wrap).expect("wrap")
            .with_scoped_function::<Provider, (HostExternalType<Envelope>,), BigInt, _>("hash", hash).expect("hash");
        let source = r#"
pub type Counter
pub type Envelope
@external(erlang, "native", "make")
fn make() -> Counter
@external(erlang, "native", "read")
fn read(value: Counter) -> Int
@external(erlang, "native", "wrap")
fn wrap(value: Counter) -> Envelope
@external(erlang, "native", "hash")
fn hash(value: Envelope) -> Int
fn make_graph() -> Counter { let constructor = make constructor() }
fn make_tail() -> Counter { make_graph() }
fn constructor() -> fn() -> Counter { make_tail }
pub fn run() {
  let constructor = constructor()
  let first = constructor()
  let retained = fn() { first }
  let values = [first, retained()]
  let wrapped = wrap(first)
  let result = read(first)
  echo wrapped
  let equal = wrap(make())
  #(result, values == [first, first] && wrapped == equal && hash(wrapped) == hash(equal))
}
"#;
        let (bindings, run) = WorkModuleBuilder::new(program(source, provider))
            .expect("plan")
            .function(FunctionDeclaration::<(), (BigInt, bool)>::new("run"))
            .expect("entry");
        let mut module = bindings.seal().expect("seal");
        let drops = Arc::new(AtomicUsize::new(0));
        let mut state = (Arc::clone(&drops), ());
        assert!(std::ptr::eq(
            <WorkComponent as HostProvider<Profile>>::project(&mut state),
            &state.1,
        ));
        let mut echo = Echo::default();
        with_execution_scope(async |guard| {
            let mut scope = module.attach(guard, &mut state, &mut echo);
            assert_eq!(
                scope.call(&run, ()).expect("source call"),
                (41.into(), true)
            );
        })
        .now_or_never()
        .expect("direct calls");
        assert!(echo.0[0].ends_with("Envelope(Counter(41))"));
        assert_eq!(drops.load(Ordering::SeqCst), 2);
        drop(module);
        assert_eq!(drops.load(Ordering::SeqCst), 2);
    }

    #[test]
    fn owned_external_input_survives_pending_and_drops_with_the_work_not_its_waiter() {
        fn read_later<'call>(
            call: TransferHostCall<'call, Profile, Provider, WorkHostType<BigInt>>,
            constructions: HostConstructions<'call, HostTypeListEnd>,
            value: HostExternal<'call, HostExternalType<Counter>>,
        ) -> Result<HostCallCompletion<'call, WorkHostType<BigInt>>, AsyncHostCallError> {
            let value = call
                .provider_transfer_external_item_with::<Provider, Counter, HostTypeListEnd>(value);
            Ok(call.return_future(constructions, move |_| {
                Box::pin(async move {
                    let mut pending = true;
                    poll_fn(|cx| {
                        if std::mem::take(&mut pending) {
                            cx.waker().wake_by_ref();
                            Poll::Pending
                        } else {
                            Poll::Ready(())
                        }
                    })
                    .await;
                    let value = value.with(|payload| BigInt::from(payload.value.get()));
                    Ok(HostFutureCompletion::new(move |call, _| {
                        Ok(call.return_value(value))
                    }))
                })
            }))
        }
        let provider = TransferHostProviderModule::new_for_profile("application", "library").expect("identity")
            .with_external_type::<Provider, Counter>().expect("counter schema")
            .with_scoped_function::<Provider, (), HostExternalType<Counter>, _>("make", make_counter).expect("constructor")
            .with_scoped_function_and_constructions::<Provider, (HostExternalType<Counter>,), WorkHostType<BigInt>, HostTypeListEnd, _>("read_later", read_later).expect("work");
        let source = r#"
import fixture/work as future
pub type Counter
@external(erlang, "native", "make")
fn make() -> Counter
@external(erlang, "native", "read_later")
fn read_later(value: Counter) -> future.Work(Int)
pub fn run() { read_later(make()) }
"#;
        let (bindings, run) = WorkModuleBuilder::new(program(source, provider))
            .expect("plan")
            .function(FunctionDeclaration::<(), WorkType<BigInt>>::new("run"))
            .expect("entry");
        let mut module = bindings.seal().expect("seal");
        let drops = Arc::new(AtomicUsize::new(0));
        let mut state = (Arc::clone(&drops), ());
        let mut echo = Echo::default();
        let mut task = Box::pin(with_execution_scope(async |guard| {
            let mut scope = module.attach(guard, &mut state, &mut echo);
            let abandoned = scope.call(&run, ()).expect("unpolled work");
            assert_eq!(drops.load(Ordering::SeqCst), 0);
            drop(abandoned);
            assert_eq!(drops.load(Ordering::SeqCst), 1);
            let work = scope.call(&run, ()).expect("work");
            {
                let mut observer = Box::pin(scope.observe(&work));
                assert!(
                    observer
                        .as_mut()
                        .poll(&mut Context::from_waker(Waker::noop()))
                        .is_pending()
                );
            }
            assert_eq!(drops.load(Ordering::SeqCst), 1);
            let result = scope
                .observe(&work)
                .await
                .expect("redrive existing operation");
            assert_eq!(result.read(Clone::clone), BigInt::from(41));
            assert_eq!(drops.load(Ordering::SeqCst), 2);
            let cancelled = scope.call(&run, ()).expect("cancelled work");
            {
                let mut observer = Box::pin(scope.observe(&cancelled));
                assert!(
                    observer
                        .as_mut()
                        .poll(&mut Context::from_waker(Waker::noop()))
                        .is_pending()
                );
            }
            drop(cancelled);
            assert_eq!(drops.load(Ordering::SeqCst), 3);
        }));
        fn require_send<T: Send>(_: &T) {}
        require_send(&task);
        std::thread::scope(|threads| {
            threads
                .spawn(|| {
                    assert!(
                        task.as_mut()
                            .poll(&mut Context::from_waker(Waker::noop()))
                            .is_ready()
                    );
                })
                .join()
                .expect("work and non-Sync payload move together");
        });
        drop(task);
        assert!(echo.0.is_empty());
        assert_eq!(drops.load(Ordering::SeqCst), 3);
    }

    #[test]
    fn external_returns_compose_with_every_direct_scalar_family() {
        let provider = TransferHostProviderModule::new_for_profile("application", "library")
            .expect("identity")
            .with_external_type::<Provider, Counter>()
            .expect("schema")
            .with_scoped_function::<Provider, (), HostExternalType<Counter>, _>(
                "make",
                make_counter,
            )
            .expect("constructor");
        macro_rules! scalar {
            ($provider:expr, $name:literal, $ty:ty) => {{
                fn identity<'call>(
                    call: TransferHostCall<'call, Profile, Provider, $ty>,
                    value: $ty,
                ) -> Result<HostCallCompletion<'call, $ty>, AsyncHostCallError> {
                    Ok(call.return_value(value))
                }
                $provider
                    .with_scoped_function::<Provider, ($ty,), $ty, _>($name, identity)
                    .expect("scalar")
            }};
        }
        let provider = scalar!(provider, "int", BigInt);
        let provider = scalar!(provider, "float", f64);
        let provider = scalar!(provider, "string", EcoString);
        let provider = scalar!(provider, "bits", crate::BitArrayValue);
        let provider = scalar!(provider, "codepoint", char);
        let provider = scalar!(provider, "bool", bool);
        let provider = scalar!(provider, "nil", ());
        let source = r#"
pub type Counter
@external(erlang, "native", "make")
fn make() -> Counter
@external(erlang, "native", "int")
fn int(value: Int) -> Int
@external(erlang, "native", "float")
fn float(value: Float) -> Float
@external(erlang, "native", "string")
fn string(value: String) -> String
@external(erlang, "native", "bits")
fn bits(value: BitArray) -> BitArray
@external(erlang, "native", "codepoint")
fn codepoint(value: UtfCodepoint) -> UtfCodepoint
@external(erlang, "native", "bool")
fn bool(value: Bool) -> Bool
@external(erlang, "native", "nil")
fn nil(value: Nil) -> Nil
pub fn run(i: Int, f: Float, s: String, b: BitArray, c: UtfCodepoint, flag: Bool, n: Nil) {
  echo make()
  #(int(i), float(f), string(s), bits(b), codepoint(c), bool(flag), nil(n))
}
"#;
        type Scalars = (BigInt, f64, EcoString, crate::BitArrayValue, char, bool, ());
        let (bindings, run) = WorkModuleBuilder::new(program(source, provider))
            .expect("plan")
            .function(FunctionDeclaration::<Scalars, Scalars>::new("run"))
            .expect("entry");
        let mut module = bindings.seal().expect("seal");
        let values: Scalars = (
            7.into(),
            2.5,
            "scalar".into(),
            crate::BitArrayValue::from_bytes(vec![0x7f]),
            'x',
            true,
            (),
        );
        let mut state = (Arc::new(AtomicUsize::new(0)), ());
        let mut echo = Echo::default();
        with_execution_scope(async |guard| {
            let mut scope = module.attach(guard, &mut state, &mut echo);
            assert_eq!(
                scope.call(&run, values.clone()).expect("scalar returns"),
                values
            );
        })
        .now_or_never()
        .expect("immediate scalar call");
        assert_eq!(echo.0.len(), 1);
        assert!(echo.0[0].ends_with("Counter(41)"));
        assert_eq!(state.0.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn external_returns_preserve_provider_failures_and_source_panics() {
        fn bridge<'call>(
            mut call: TransferHostCall<'call, Profile, Provider, HostExternalType<Counter>>,
            callback: crate::HostCallable<'call, HostTypeListEnd, HostExternalType<Counter>>,
        ) -> Result<HostCallCompletion<'call, HostExternalType<Counter>>, AsyncHostCallError>
        {
            let value = call.invoke(callback, ())?;
            Ok(call.return_value(value))
        }
        fn fail(
            _: TransferHostCall<'_, Profile, Provider, HostExternalType<Counter>>,
        ) -> Result<HostCallCompletion<'_, HostExternalType<Counter>>, AsyncHostCallError> {
            Err(crate::HostFailure::new("counter unavailable").into())
        }
        fn stop(
            _: TransferHostCall<'_, Profile, Provider, HostExternalType<Counter>>,
        ) -> Result<std::convert::Infallible, AsyncHostCallError> {
            Err(crate::HostFailure::new("external construction stopped").into())
        }
        let provider = TransferHostProviderModule::new_for_profile("application", "library")
            .expect("identity")
            .with_external_type::<Provider, Counter>()
            .expect("schema")
            .with_scoped_function::<Provider, (), HostExternalType<Counter>, _>("make", make_counter)
            .expect("constructor")
            .with_scoped_function::<Provider, (crate::HostFunctionType<HostTypeListEnd, HostExternalType<Counter>>,), HostExternalType<Counter>, _>("bridge", bridge)
            .expect("external callback")
            .with_scoped_function::<Provider, (), HostExternalType<Counter>, _>("fail", fail)
            .expect("fallible")
            .with_scoped_diverging_function::<Provider, (), HostExternalType<Counter>, _>(
                "stop", stop,
            )
            .expect("diverging");
        let source = r#"
pub type Counter
@external(erlang, "native", "fail")
fn fail() -> Counter
@external(erlang, "native", "stop")
fn stop() -> Counter
pub fn run() { echo fail() Nil }
fn panic_counter() -> Counter { panic as "external graph stopped" }
pub fn source_panic() { echo panic_counter() Nil }
fn stopping_constructor() -> fn() -> Counter { stop }
pub fn stopped() { let constructor = stopping_constructor() echo constructor() Nil }
@external(erlang, "native", "make")
fn make() -> Counter
@external(erlang, "native", "bridge")
fn bridge(callback: fn() -> Counter) -> Counter
pub fn nested(fail: Bool) {
  echo bridge(fn() {
    case fail {
      True -> panic as "external callback stopped"
      False -> make()
    }
  })
  Nil
}
"#;
        let (mut bindings, run) = WorkModuleBuilder::new(program(source, provider))
            .expect("plan")
            .function(FunctionDeclaration::<(), ()>::new("run"))
            .expect("entry");
        let panic = bindings
            .function(FunctionDeclaration::<(), ()>::new("source_panic"))
            .expect("panic entry");
        let stop = bindings
            .function(FunctionDeclaration::<(), ()>::new("stopped"))
            .expect("stop entry");
        let nested = bindings
            .function(FunctionDeclaration::<(bool,), ()>::new("nested"))
            .expect("external callback entry");
        let mut module = bindings.seal().expect("seal");
        let mut state = (Arc::new(AtomicUsize::new(0)), ());
        let mut echo = Echo::default();
        with_execution_scope(async |guard| {
            let mut scope = module.attach(guard, &mut state, &mut echo);
            for (entry, message) in [
                (
                    &run,
                    "host function application::library.fail failed: counter unavailable",
                ),
                (&panic, "panic: external graph stopped"),
                (
                    &stop,
                    "host function application::library.stop failed: external construction stopped",
                ),
            ] {
                assert_eq!(
                    scope
                        .call(entry, ())
                        .expect_err("original failure")
                        .to_string(),
                    message
                );
            }
            assert_eq!(
                scope
                    .call(&nested, (true,))
                    .expect_err("nested source failure")
                    .to_string(),
                "panic: external callback stopped"
            );
            scope
                .call(&nested, (false,))
                .expect("external callback success");
        })
        .now_or_never()
        .expect("failures are direct");
        assert_eq!(echo.0.len(), 1);
        assert!(echo.0[0].ends_with("Counter(41)"));
        assert_eq!(state.0.load(Ordering::SeqCst), 1);
    }
}
