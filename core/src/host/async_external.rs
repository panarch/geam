mod context;

pub use context::{
    AsyncHostExternalEquality, AsyncHostExternalHashing, AsyncHostExternalInspection,
};
pub(crate) use context::{
    TransferExternalEquality, TransferExternalHashing, TransferExternalInspection,
};

use super::{HostExternalSchema, HostExternalType, HostProfile, HostProvider, HostTypeListEnd};
use crate::runtime::{
    EvaluatedExternalValue, TransferExternalPayloadLease, TransferExternalStore,
    TransferStoredRuntimeValue, TransferValues,
};
use ecow::EcoString;
use std::marker::PhantomData;

/// Module-owned storage for immutable external payloads used by async embedding.
///
/// Payloads need `Send`, not `Sync`. Each short payload view is serialized; no
/// reference into the store can escape that view or remain live across `.await`.
pub struct AsyncHostExternalStore<Payload> {
    inner: TransferExternalStore<Payload>,
}

/// Source semantics and storage for an external type in resumable embedding.
///
/// Equal payloads must have equal source hashes. These hashes are runtime
/// indexes, not a stable serialization format.
pub trait AsyncHostExternalStorage<Profile, Schema>: Send + Sync + 'static
where
    Profile: HostProfile,
    Schema: HostExternalSchema,
{
    type Payload: Send + 'static;

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

/// A typed external argument retained for the duration of one async host call.
///
/// The argument cannot escape its invocation:
///
/// ```compile_fail
/// use geam_core::AsyncHostExternal;
/// fn escape<'call, Schema>(
///     value: AsyncHostExternal<'call, Schema>,
/// ) -> AsyncHostExternal<'static, Schema> {
///     value
/// }
/// ```
pub struct AsyncHostExternal<'call, Schema, Arguments = HostTypeListEnd> {
    value: EvaluatedExternalValue<TransferValues>,
    lifetime: PhantomData<&'call mut ()>,
    schema: PhantomData<fn() -> HostExternalType<Schema, Arguments>>,
}

/// An owned, typed source value kept inside an async external payload.
///
/// It retains its complete runtime graph after the originating call ends.
/// Creation belongs to [`AsyncHostExternalPayloadBuilder`]; the value cannot
/// be cloned out of a payload into provider state.
///
/// ```compile_fail
/// use geam_core::AsyncHostStoredValue;
/// fn require_clone<T: Clone>() {}
/// require_clone::<AsyncHostStoredValue<bool>>();
/// ```
pub struct AsyncHostStoredValue<Type> {
    value: TransferStoredRuntimeValue,
    marker: PhantomData<fn() -> Type>,
}

/// The active-call builder for values retained by a new async external payload.
pub struct AsyncHostExternalPayloadBuilder<'call> {
    lifetime: PhantomData<&'call mut ()>,
}

/// An owned external return bound to the async host invocation that created it.
pub struct AsyncHostExternalReturn<'call, Schema, Arguments = HostTypeListEnd> {
    lease: TransferExternalPayloadLease,
    lifetime: PhantomData<&'call mut ()>,
    schema: PhantomData<fn() -> HostExternalType<Schema, Arguments>>,
}

impl<Payload> Default for AsyncHostExternalStore<Payload> {
    fn default() -> Self {
        Self {
            inner: TransferExternalStore::default(),
        }
    }
}

impl<Payload: Send + 'static> AsyncHostExternalStore<Payload> {
    pub(super) fn insert<Profile, Schema, Storage>(
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

    pub(super) fn with_view<Output>(
        &self,
        lease: &TransferExternalPayloadLease,
        view: impl FnOnce(&Payload) -> Output,
    ) -> Output {
        self.inner.with_view(lease, view)
    }
}

impl<'call, Schema, Arguments> AsyncHostExternal<'call, Schema, Arguments> {
    pub(super) fn new(value: EvaluatedExternalValue<TransferValues>) -> Self {
        Self {
            value,
            lifetime: PhantomData,
            schema: PhantomData,
        }
    }

    pub(super) fn lease(&self) -> &TransferExternalPayloadLease {
        self.value.lease()
    }

    pub(super) fn source_hash(&self, lists: &crate::runtime::TransferListStorage) -> u64 {
        use crate::runtime::RuntimeValueProfile;
        TransferValues::external_value_source_hash(lists, &self.value)
    }
}

impl<'call> AsyncHostExternalPayloadBuilder<'call> {
    pub(super) fn new() -> Self {
        Self {
            lifetime: PhantomData,
        }
    }

    /// Retains one external value in the payload being constructed.
    pub fn store_external<Schema, Arguments>(
        &mut self,
        value: &AsyncHostExternal<'call, Schema, Arguments>,
    ) -> AsyncHostStoredValue<HostExternalType<Schema, Arguments>> {
        AsyncHostStoredValue {
            value: TransferStoredRuntimeValue::external(value.value.clone()),
            marker: PhantomData,
        }
    }
}

impl<'call, Schema, Arguments> AsyncHostExternalReturn<'call, Schema, Arguments> {
    pub(super) fn new(lease: TransferExternalPayloadLease) -> Self {
        Self {
            lease,
            lifetime: PhantomData,
            schema: PhantomData,
        }
    }

    pub(super) fn into_lease(self) -> TransferExternalPayloadLease {
        self.lease
    }
}

#[cfg(test)]
mod tests {
    use super::{
        AsyncHostExternal, AsyncHostExternalBinding, AsyncHostExternalEquality,
        AsyncHostExternalHashing, AsyncHostExternalInspection, AsyncHostExternalReturn,
        AsyncHostExternalStorage, AsyncHostExternalStore, AsyncHostStoredValue, HostExternalSchema,
        HostExternalType, HostProfile, HostProvider,
    };
    use crate::embedding::{AsyncHostedModuleBuilder, FunctionDeclaration};
    use crate::{
        AsyncHostCall, AsyncHostFuture, AsyncHostModule, AsyncHostProviderModule,
        AsyncHostProviderSet, ModuleSource, PackageSource, compile_typed_async_host_program,
    };
    use ecow::EcoString;
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
        envelopes: AsyncHostExternalStore<AsyncHostStoredValue<HostExternalType<Counter>>>,
        // The aggregate, as well as its payload, must work without Sync.
        _exclusive: std::marker::PhantomData<Cell<()>>,
    }

    impl HostProfile for Profile {
        type RunState = State;
        type ExternalStores = Stores;
    }

    impl HostProvider<Profile> for Provider {
        type State = State;
        fn project(state: &mut Self::State) -> &mut Self::State {
            state
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
        type Payload = AsyncHostStoredValue<HostExternalType<Counter>>;
        fn store(stores: &Stores) -> &AsyncHostExternalStore<Self::Payload> {
            &stores.envelopes
        }
        fn source_equal(
            context: &AsyncHostExternalEquality<'_>,
            left: &Self::Payload,
            right: &Self::Payload,
        ) -> bool {
            context.stored_values_equal(left, right)
        }
        fn source_hash(context: &AsyncHostExternalHashing<'_>, value: &Self::Payload) -> u64 {
            context.stored_value_hash(value)
        }
        fn inspect(context: &AsyncHostExternalInspection<'_>, value: &Self::Payload) -> EcoString {
            format!("Envelope({})", context.inspect_stored_value(value)).into()
        }
    }

    pub(super) fn make_counter(
        mut call: AsyncHostCall<'_, Profile, Provider, HostExternalType<Counter>>,
    ) -> AsyncHostFuture<'_, AsyncHostExternalReturn<'_, Counter>> {
        AsyncHostFuture::new(async move {
            let drops = call.with_state(|state| Arc::clone(state)).await;
            call.return_external(|_| CounterPayload {
                value: Cell::new(41),
                drops,
            })
            .await
        })
    }

    #[test]
    fn public_async_hosts_create_read_and_retain_send_only_external_payloads() {
        fn read<'call>(
            mut call: AsyncHostCall<'call, Profile, Provider, BigInt>,
            value: AsyncHostExternal<'call, Counter>,
        ) -> AsyncHostFuture<'call, BigInt> {
            AsyncHostFuture::new(async move {
                let mut pending = true;
                poll_fn(|context| {
                    if std::mem::take(&mut pending) {
                        context.waker().wake_by_ref();
                        Poll::Pending
                    } else {
                        Poll::Ready(())
                    }
                })
                .await;
                call.with_external(&value, |payload| BigInt::from(payload.value.get()))
                    .await
            })
        }

        fn wrap<'call>(
            mut call: AsyncHostCall<'call, Profile, Provider, HostExternalType<Envelope>>,
            value: AsyncHostExternal<'call, Counter>,
        ) -> AsyncHostFuture<'call, AsyncHostExternalReturn<'call, Envelope>> {
            AsyncHostFuture::new(async move {
                call.return_external(|payload| payload.store_external(&value))
                    .await
            })
        }

        fn hash<'call>(
            mut call: AsyncHostCall<'call, Profile, Provider, BigInt>,
            value: AsyncHostExternal<'call, Envelope>,
        ) -> AsyncHostFuture<'call, BigInt> {
            AsyncHostFuture::new(async move { call.source_hash(&value).into() })
        }

        let provider = AsyncHostProviderModule::<Profile>::new("application", "library")
            .expect("provider identity")
            .with_external_type::<Provider, Counter>().expect("counter schema")
            .with_external_type::<Provider, Envelope>().expect("envelope schema")
            .with_scoped_async_function::<Provider, (), HostExternalType<Counter>, _>("make", make_counter).expect("counter constructor")
            .with_scoped_async_function::<Provider, (HostExternalType<Counter>,), BigInt, _>("read", read).expect("counter reader")
            .with_scoped_async_function::<Provider, (HostExternalType<Counter>,), HostExternalType<Envelope>, _>("wrap", wrap).expect("envelope constructor")
            .with_scoped_async_function::<Provider, (HostExternalType<Envelope>,), BigInt, _>("hash", hash).expect("source hashing");
        let source = r#"
@external(erlang, "native", "Counter")
pub type Counter
@external(erlang, "native", "Envelope")
pub type Envelope
@external(erlang, "native", "make")
fn make() -> Counter
@external(erlang, "native", "read")
fn read(value: Counter) -> Int
@external(erlang, "native", "wrap")
fn wrap(value: Counter) -> Envelope
@external(erlang, "native", "hash")
fn hash(value: Envelope) -> Int

fn make_graph() -> Counter {
  let constructor = make
  constructor()
}
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
        let hosts = AsyncHostProviderSet::with_providers([], [provider]).expect("source provider");
        let program = compile_typed_async_host_program(
            "application",
            "library",
            [PackageSource::new(
                "application",
                Vec::<String>::new(),
                [ModuleSource::new("library", "src/library.gleam", source)],
            )],
            hosts,
        )
        .expect("public external source");
        let (bindings, function) = AsyncHostedModuleBuilder::new(program)
            .expect("external plan")
            .function(FunctionDeclaration::<(), (BigInt, bool)>::new("run"))
            .expect("run binding");
        let mut module = bindings.seal();
        let drops = Arc::new(AtomicUsize::new(0));
        let mut state = Arc::clone(&drops);
        let mut echo = Echo::default();
        fn require_send<T: Send>(_: &T) {}
        require_send(&module);
        let mut call = Box::pin(module.call_async(&function, (), &mut state, &mut echo));
        require_send(&call);
        let mut context = Context::from_waker(Waker::noop());
        assert!(call.as_mut().poll(&mut context).is_pending());
        assert_eq!(drops.load(Ordering::SeqCst), 0);
        drop(call);
        assert_eq!(drops.load(Ordering::SeqCst), 1);
        assert!(echo.0.is_empty());
        let mut call = Box::pin(module.call_async(&function, (), &mut state, &mut echo));
        assert!(call.as_mut().poll(&mut context).is_pending());
        assert_eq!(
            call.as_mut()
                .poll(&mut context)
                .map(|result| result.expect("external completion")),
            Poll::Ready((41.into(), true))
        );
        drop(call);
        assert!(echo.0[0].ends_with("Envelope(Counter(41))"));
        assert_eq!(drops.load(Ordering::SeqCst), 3);
        drop(module);
        assert_eq!(drops.load(Ordering::SeqCst), 3);
        drop(state);
        drop(echo);
        assert_eq!(drops.load(Ordering::SeqCst), 3);
    }

    #[test]
    fn discarding_an_unpolled_return_request_releases_its_payload_without_insertion() {
        fn abandon<'call>(
            mut call: AsyncHostCall<'call, Profile, Provider, HostExternalType<Counter>>,
        ) -> AsyncHostFuture<'call, AsyncHostExternalReturn<'call, Counter>> {
            AsyncHostFuture::new(async move {
                let drops = call.with_state(|state| Arc::clone(state)).await;
                let request = call.return_external(|_| CounterPayload {
                    value: Cell::new(7),
                    drops: Arc::clone(&drops),
                });
                drop(request);
                let mut pending = true;
                poll_fn(|context| {
                    if std::mem::take(&mut pending) {
                        context.waker().wake_by_ref();
                        Poll::Pending
                    } else {
                        Poll::Ready(())
                    }
                })
                .await;
                call.return_external(|_| CounterPayload {
                    value: Cell::new(7),
                    drops,
                })
                .await
            })
        }

        let provider = AsyncHostProviderModule::<Profile>::new("application", "library")
            .expect("provider identity")
            .with_external_type::<Provider, Counter>()
            .expect("counter schema")
            .with_scoped_async_function::<Provider, (), HostExternalType<Counter>, _>(
                "make", abandon,
            )
            .expect("abandoned payload constructor");
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
pub type Counter
@external(erlang, "native", "make")
fn make() -> Counter
pub fn run() -> Int {
  let _value = make()
  7
}
"#,
                )],
            )],
            AsyncHostProviderSet::with_providers([], [provider]).expect("async host set"),
        )
        .expect("abandoned constructor source");
        let (bindings, function) = AsyncHostedModuleBuilder::new(program)
            .expect("abandoned constructor plan")
            .function(FunctionDeclaration::<(), BigInt>::new("run"))
            .expect("run binding");
        let mut module = bindings.seal();
        let drops = Arc::new(AtomicUsize::new(0));
        let mut state = Arc::clone(&drops);
        let mut echo = Echo::default();
        let mut context = Context::from_waker(Waker::noop());

        let mut call = Box::pin(module.call_async(&function, (), &mut state, &mut echo));
        assert_eq!(call.as_mut().poll(&mut context), Poll::Pending);
        assert_eq!(drops.load(Ordering::SeqCst), 1);
        drop(call);
        assert_eq!(drops.load(Ordering::SeqCst), 1);

        let mut call = Box::pin(module.call_async(&function, (), &mut state, &mut echo));
        assert_eq!(call.as_mut().poll(&mut context), Poll::Pending);
        assert_eq!(drops.load(Ordering::SeqCst), 2);
        assert_eq!(call.as_mut().poll(&mut context), Poll::Ready(Ok(7.into())));
        drop(call);
        assert_eq!(drops.load(Ordering::SeqCst), 3);
        drop(module);
        assert_eq!(drops.load(Ordering::SeqCst), 3);
        assert!(echo.0.is_empty());
    }

    #[test]
    fn external_returns_compose_with_every_owned_and_scoped_scalar_family() {
        fn identity<'call, Value: Send + 'static>(
            _: AsyncHostCall<'call, Profile, Provider, Value>,
            value: Value,
        ) -> AsyncHostFuture<'call, Value> {
            AsyncHostFuture::new(async move { value })
        }

        let provider = AsyncHostProviderModule::<Profile>::new("application", "library")
            .expect("provider identity")
            .with_external_type::<Provider, Counter>()
            .expect("counter schema")
            .with_scoped_async_function::<Provider, (), HostExternalType<Counter>, _>(
                "make",
                make_counter,
            )
            .expect("counter constructor");
        macro_rules! scalar {
            ($provider:expr, $owned:literal, $scoped:literal, $type:ty) => {
                $provider
                    .with_async_function($owned, std::future::ready::<$type>)
                    .expect("owned scalar alongside an external return")
                    .with_scoped_async_function::<Provider, ($type,), $type, _>(
                        $scoped,
                        identity::<$type>,
                    )
                    .expect("scoped scalar alongside an external return")
            };
        }
        let provider = scalar!(provider, "owned_int", "scoped_int", BigInt);
        let provider = scalar!(provider, "owned_float", "scoped_float", f64);
        let provider = scalar!(provider, "owned_string", "scoped_string", EcoString);
        let provider = scalar!(provider, "owned_bits", "scoped_bits", crate::BitArrayValue);
        let provider = scalar!(provider, "owned_codepoint", "scoped_codepoint", char);
        let provider = scalar!(provider, "owned_bool", "scoped_bool", bool);
        let provider = scalar!(provider, "owned_nil", "scoped_nil", ());
        let source = r#"
@external(erlang, "native", "Counter")
pub type Counter
@external(erlang, "native", "make")
fn make() -> Counter
@external(erlang, "native", "owned_int")
fn owned_int(value: Int) -> Int
@external(erlang, "native", "scoped_int")
fn scoped_int(value: Int) -> Int
@external(erlang, "native", "owned_float")
fn owned_float(value: Float) -> Float
@external(erlang, "native", "scoped_float")
fn scoped_float(value: Float) -> Float
@external(erlang, "native", "owned_string")
fn owned_string(value: String) -> String
@external(erlang, "native", "scoped_string")
fn scoped_string(value: String) -> String
@external(erlang, "native", "owned_bits")
fn owned_bits(value: BitArray) -> BitArray
@external(erlang, "native", "scoped_bits")
fn scoped_bits(value: BitArray) -> BitArray
@external(erlang, "native", "owned_codepoint")
fn owned_codepoint(value: UtfCodepoint) -> UtfCodepoint
@external(erlang, "native", "scoped_codepoint")
fn scoped_codepoint(value: UtfCodepoint) -> UtfCodepoint
@external(erlang, "native", "owned_bool")
fn owned_bool(value: Bool) -> Bool
@external(erlang, "native", "scoped_bool")
fn scoped_bool(value: Bool) -> Bool
@external(erlang, "native", "owned_nil")
fn owned_nil(value: Nil) -> Nil
@external(erlang, "native", "scoped_nil")
fn scoped_nil(value: Nil) -> Nil

pub fn run(i: Int, f: Float, s: String, b: BitArray, c: UtfCodepoint, flag: Bool, n: Nil) {
  echo make()
  #(
    #(owned_int(i), owned_float(f), owned_string(s), owned_bits(b),
      owned_codepoint(c), owned_bool(flag), owned_nil(n)),
    #(scoped_int(i), scoped_float(f), scoped_string(s), scoped_bits(b),
      scoped_codepoint(c), scoped_bool(flag), scoped_nil(n)),
  )
}
"#;
        let program = compile_typed_async_host_program(
            "application",
            "library",
            [PackageSource::new(
                "application",
                Vec::<String>::new(),
                [ModuleSource::new("library", "src/library.gleam", source)],
            )],
            AsyncHostProviderSet::with_providers([], [provider]).expect("mixed provider"),
        )
        .expect("mixed return source");
        type Scalars = (BigInt, f64, EcoString, crate::BitArrayValue, char, bool, ());
        let (bindings, function) = AsyncHostedModuleBuilder::new(program)
            .expect("mixed return plan")
            .function(FunctionDeclaration::<Scalars, (Scalars, Scalars)>::new(
                "run",
            ))
            .expect("mixed return binding");
        let mut module = bindings.seal();
        let values: Scalars = (
            7.into(),
            2.5,
            "scalar".into(),
            crate::BitArrayValue::from_bytes(vec![0x7f]),
            'x',
            true,
            (),
        );
        let mut state = Arc::new(AtomicUsize::new(0));
        let mut echo = Echo::default();
        let mut call =
            Box::pin(module.call_async(&function, values.clone(), &mut state, &mut echo));
        let mut context = Context::from_waker(Waker::noop());
        assert_eq!(
            call.as_mut()
                .poll(&mut context)
                .map(|result| result.expect("mixed scalar returns")),
            Poll::Ready((values.clone(), values)),
        );
        drop(call);
        assert_eq!(echo.0.len(), 1);
        assert!(echo.0[0].ends_with("Counter(41)"));
        assert_eq!(state.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn external_returns_preserve_provider_failures_and_source_panics() {
        fn stop() -> Result<std::convert::Infallible, crate::HostFailure> {
            Err(crate::HostFailure::new("external construction stopped"))
        }

        fn fail(
            _: AsyncHostCall<'_, Profile, Provider, HostExternalType<Counter>>,
        ) -> AsyncHostFuture<
            '_,
            Result<AsyncHostExternalReturn<'_, Counter>, crate::AsyncHostCallError>,
        > {
            AsyncHostFuture::new(async {
                Err(crate::HostFailure::new("counter unavailable").into())
            })
        }

        let provider = AsyncHostProviderModule::<Profile>::new("application", "library")
            .expect("provider identity")
            .with_external_type::<Provider, Counter>()
            .expect("counter schema")
            .with_fallible_scoped_async_function::<Provider, (), HostExternalType<Counter>, _>(
                "fail", fail,
            )
            .expect("fallible external constructor");
        let control = AsyncHostModule::<Profile>::new_for_profile("host_support", "host/control")
            .expect("control module")
            .with_fallible_function("stop", stop)
            .expect("non-returning external constructor");
        let source = r#"
import host/control

@external(erlang, "native", "Counter")
pub type Counter
@external(erlang, "native", "fail")
fn fail() -> Counter

pub fn run() -> Nil {
  echo fail()
  Nil
}

fn panic_counter() -> Counter {
  panic as "external graph stopped"
}

pub fn source_panic() -> Nil {
  echo panic_counter()
  Nil
}

fn stopping_constructor() -> fn() -> Counter { control.stop }

pub fn stopped() -> Nil {
  let constructor = stopping_constructor()
  echo constructor()
  Nil
}
"#;
        let program = compile_typed_async_host_program(
            "application",
            "library",
            [PackageSource::new(
                "application",
                ["host_support"],
                [ModuleSource::new("library", "src/library.gleam", source)],
            )],
            AsyncHostProviderSet::with_providers([control], [provider]).expect("source provider"),
        )
        .expect("fallible external source");
        let (mut bindings, function) = AsyncHostedModuleBuilder::new(program)
            .expect("external plan")
            .function(FunctionDeclaration::<(), ()>::new("run"))
            .expect("run binding");
        let source_panic = bindings
            .function(FunctionDeclaration::<(), ()>::new("source_panic"))
            .expect("source panic binding");
        let stopped = bindings
            .function(FunctionDeclaration::<(), ()>::new("stopped"))
            .expect("stopped external binding");
        let mut module = bindings.seal();
        let mut state = Arc::new(AtomicUsize::new(0));
        let mut echo = Echo::default();
        let mut call = Box::pin(module.call_async(&function, (), &mut state, &mut echo));
        let mut context = Context::from_waker(Waker::noop());
        assert_eq!(
            call.as_mut()
                .poll(&mut context)
                .map(|result| result.map_err(|error| error.to_string())),
            Poll::Ready(Err(
                "host function application::library.fail failed: counter unavailable".to_owned()
            )),
        );
        drop(call);
        let mut call = Box::pin(module.call_async(&source_panic, (), &mut state, &mut echo));
        assert_eq!(
            call.as_mut()
                .poll(&mut context)
                .map(|result| result.map_err(|error| error.to_string())),
            Poll::Ready(Err("panic: external graph stopped".to_owned())),
        );
        drop(call);
        let mut call = Box::pin(module.call_async(&stopped, (), &mut state, &mut echo));
        assert_eq!(
            call.as_mut()
                .poll(&mut context)
                .map(|result| result.map_err(|error| error.to_string())),
            Poll::Ready(Err(
                "host function host_support::host/control.stop failed: external construction stopped".to_owned()
            )),
        );
        drop(call);
        assert!(echo.0.is_empty());
        assert_eq!(state.load(Ordering::SeqCst), 0);
    }
}
