mod context;
mod dynamic;
mod store;
mod stored;

use crate::host::{HostProfile, HostProvider, HostTypeListEnd};
use ecow::EcoString;
use std::marker::PhantomData;

pub(crate) use crate::provider_support::HostStoredValueFamily;
pub(crate) use context::{RetainedValueEquality, RetainedValueHashing, RetainedValueInspection};
pub use dynamic::HostStoredDynamic;
pub use store::HostExternalStore;
pub(crate) use store::{ExternalPayloadLease, ExternalPayloadView};
pub use stored::{
    HostExternalPayloadBuilder, HostExternalPayloadView, HostStoredType, HostStoredValue,
};

/// A source-declared external Gleam type linked to Rust storage.
pub trait HostExternalSchema: Send + Sync + 'static {
    const PACKAGE: &'static str;
    const MODULE: &'static str;
    const NAME: &'static str;
    const PARAMETER_COUNT: usize;
}

/// Provider-owned storage and Gleam source semantics for one external schema.
///
/// Payloads are immutable after creation. Values that compare equal through
/// [`HostExternalStorage::source_equal`] must return the same
/// [`HostExternalStorage::source_hash`]. Hash collisions are allowed and are
/// resolved through source equality. Source hashes are runtime indexes, not
/// stable serialized values.
pub trait HostExternalStorage<Profile, Schema>: Send + Sync + 'static
where
    Profile: HostProfile,
    Schema: HostExternalSchema,
{
    type Payload: Send + 'static;

    /// Projects the typed payload store from the final profile's external stores.
    fn store(stores: &Profile::ExternalStores) -> &HostExternalStore<Self::Payload>;

    /// Compares two payloads using Gleam source equality.
    fn source_equal(
        context: &HostExternalEquality<'_>,
        left: &Self::Payload,
        right: &Self::Payload,
    ) -> bool;

    /// Hashes a payload consistently with [`HostExternalStorage::source_equal`].
    fn source_hash(context: &HostExternalHashing<'_>, value: &Self::Payload) -> u64;

    /// Produces the payload's canonical source-oriented inspection.
    fn inspect(context: &HostExternalInspection<'_>, value: &Self::Payload) -> EcoString;

    /// Returns a declared native view without borrowing the payload.
    fn native_view(_value: &Self::Payload) -> Option<crate::provider::advanced::NativeValue> {
        None
    }
}

/// Selects a provider-owned external storage adapter for one source schema.
pub trait HostExternalBinding<Profile, Schema>: HostProvider<Profile>
where
    Profile: HostProfile,
    Schema: HostExternalSchema,
{
    type Storage: HostExternalStorage<Profile, Schema>;
}

/// Gleam source equality for values retained by an external payload.
pub struct HostExternalEquality<'context>(
    pub(crate) &'context crate::host::RetainedValueEquality<'context>,
);

/// Gleam source hashing for values retained by an external payload.
pub struct HostExternalHashing<'context>(
    pub(crate) &'context crate::host::RetainedValueHashing<'context>,
);

/// Canonical inspection for values retained by an external payload.
pub struct HostExternalInspection<'context>(
    pub(crate) &'context crate::host::RetainedValueInspection<'context>,
);

/// A source-declared external type and its concrete type arguments.
pub struct HostExternalType<Schema, Arguments = HostTypeListEnd>(PhantomData<(Schema, Arguments)>);

impl<'context> HostExternalEquality<'context> {
    /// Compares two retained values with their exact sealed host type.
    pub fn stored_values_equal<Type>(
        &self,
        left: &HostStoredValue<Type>,
        right: &HostStoredValue<Type>,
    ) -> bool {
        self.provider_stored_values_equal(&left.value, &right.value)
    }

    /// Compares two existentially retained values using their specialized shapes.
    pub fn dynamic_values_equal(
        &self,
        left: &HostStoredDynamic,
        right: &HostStoredDynamic,
    ) -> bool {
        self.provider_stored_values_equal(left.runtime_value(), right.runtime_value())
    }
}

impl<'context> HostExternalHashing<'context> {
    /// Hashes a retained value with its exact sealed host type.
    pub fn stored_value_hash<Type>(&self, value: &HostStoredValue<Type>) -> u64 {
        self.provider_stored_value_hash(&value.value)
    }

    /// Hashes an existentially retained value and its specialized shape.
    pub fn dynamic_value_hash(&self, value: &HostStoredDynamic) -> u64 {
        self.provider_stored_value_hash(value.runtime_value())
    }
}

impl<'context> HostExternalInspection<'context> {
    /// Inspects a retained value with its exact sealed host type.
    pub fn inspect_stored_value<Type>(&self, value: &HostStoredValue<Type>) -> EcoString {
        self.provider_inspect_stored_value(&value.value)
    }

    /// Inspects an existentially retained value using its specialized shape.
    pub fn inspect_dynamic_value(&self, value: &HostStoredDynamic) -> EcoString {
        self.provider_inspect_stored_value(value.runtime_value())
    }
}

/// The source-facing identity of one registered external type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HostExternalTypeSchema {
    package: EcoString,
    module: EcoString,
    name: EcoString,
    parameter_count: usize,
}

impl HostExternalTypeSchema {
    pub fn of<Schema: HostExternalSchema>() -> Self {
        Self::new(
            Schema::PACKAGE,
            Schema::MODULE,
            Schema::NAME,
            Schema::PARAMETER_COUNT,
        )
    }

    pub fn new(
        package: impl Into<EcoString>,
        module: impl Into<EcoString>,
        name: impl Into<EcoString>,
        parameter_count: usize,
    ) -> Self {
        Self {
            package: package.into(),
            module: module.into(),
            name: name.into(),
            parameter_count,
        }
    }

    pub fn package(&self) -> &EcoString {
        &self.package
    }

    pub fn module(&self) -> &EcoString {
        &self.module
    }

    pub fn name(&self) -> &EcoString {
        &self.name
    }

    pub fn parameter_count(&self) -> usize {
        self.parameter_count
    }
}

#[cfg(test)]
mod tests {
    use super::{
        HostExternalBinding, HostExternalEquality, HostExternalHashing, HostExternalInspection,
        HostExternalSchema, HostExternalStorage, HostExternalStore, HostProfile, HostProvider,
    };
    use super::{HostStoredDynamic, HostStoredValue};
    use crate::embedding::{FunctionDeclaration, HostedModuleBuilder};
    use crate::frontend::compile_typed_host_program;
    use crate::host::{
        HostCall, HostComponentProfile, HostConstructions, HostFutureStore, HostOwnedCompletion,
        HostProviderModule, HostProviderSet,
    };
    use crate::runtime::StoredRuntimeValue;
    use crate::work_fixture::WorkType;
    use crate::work_fixture::{WorkComponent, WorkHostType};
    use crate::{
        HostCallCompletion, HostCallError, HostExternal, HostExternalType, HostTypeListEnd,
        ModuleSource, PackageSource,
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

    #[test]
    fn external_semantics_contexts_delegate_typed_and_dynamic_values() {
        let typed = HostStoredValue::<BigInt>::new(crate::runtime::StoredRuntimeValue::test_int(
            BigInt::from(7),
        ));
        let other = HostStoredValue::<BigInt>::new(crate::runtime::StoredRuntimeValue::test_int(
            BigInt::from(8),
        ));
        let dynamic = HostStoredDynamic::new(crate::runtime::StoredRuntimeValue::test_int(
            BigInt::from(9),
        ));
        let other_dynamic = HostStoredDynamic::new(crate::runtime::StoredRuntimeValue::test_int(
            BigInt::from(10),
        ));
        let equal =
            |_: &crate::runtime::RetainedValueRef, _: &crate::runtime::RetainedValueRef| true;
        let source_hash = |_: &crate::runtime::RetainedValueRef| 17;
        let inspect = |_: &crate::runtime::RetainedValueRef| "Int".into();
        let raw_equality = crate::host::RetainedValueEquality::new(&equal);
        let equality = crate::host::HostExternalEquality(&raw_equality);
        let raw_hashing = crate::host::RetainedValueHashing::new(&source_hash);
        let hashing = crate::host::HostExternalHashing(&raw_hashing);
        let raw_inspection = crate::host::RetainedValueInspection::new(&inspect);
        let inspection = crate::host::HostExternalInspection(&raw_inspection);

        assert!(equality.stored_values_equal(&typed, &other));
        assert!(equality.dynamic_values_equal(&dynamic, &other_dynamic));
        assert_eq!(hashing.stored_value_hash(&typed), 17);
        assert_eq!(hashing.dynamic_value_hash(&dynamic), 17);
        assert_eq!(inspection.inspect_stored_value(&typed), "Int");
        assert_eq!(inspection.inspect_dynamic_value(&dynamic), "Int");
    }

    struct Profile;
    struct Provider;
    struct Counter;
    struct Envelope;
    struct CounterStorage;
    struct EnvelopeStorage;

    #[derive(Default)]
    struct Echo(Vec<String>);

    impl crate::EchoSink for Echo {
        fn emit(&mut self, output: crate::EchoOutput) {
            self.0.push(output.to_string());
        }
    }

    type State = Arc<AtomicUsize>;

    struct CounterPayload {
        value: Cell<usize>,
        drops: Arc<AtomicUsize>,
    }

    impl Drop for CounterPayload {
        fn drop(&mut self) {
            self.drops.fetch_add(1, Ordering::SeqCst);
        }
    }

    #[derive(Default)]
    struct Stores {
        counters: HostExternalStore<CounterPayload>,
        envelopes: HostExternalStore<StoredRuntimeValue>,
        futures: HostFutureStore,
        // The aggregate, as well as its payload, must work without Sync.
        _exclusive: std::marker::PhantomData<Cell<()>>,
    }

    impl HostProfile for Profile {
        type RunState = (State, ());
        type ExternalStores = Stores;
        type ExecutionState = ();
    }

    impl crate::host::HostWorkProfile for Profile {
        type Work = crate::work_fixture::WorkComponent;
    }
    impl HostComponentProfile<WorkComponent> for Profile {
        fn component_stores(stores: &Stores) -> &HostFutureStore {
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

    impl HostExternalBinding<Profile, Counter> for Provider {
        type Storage = CounterStorage;
    }
    impl HostExternalBinding<Profile, Envelope> for Provider {
        type Storage = EnvelopeStorage;
    }

    impl HostExternalStorage<Profile, Counter> for CounterStorage {
        type Payload = CounterPayload;
        fn store(stores: &Stores) -> &HostExternalStore<Self::Payload> {
            &stores.counters
        }
        fn source_equal(
            _: &HostExternalEquality<'_>,
            left: &CounterPayload,
            right: &CounterPayload,
        ) -> bool {
            left.value.get() == right.value.get()
        }
        fn source_hash(_: &HostExternalHashing<'_>, value: &CounterPayload) -> u64 {
            value.value.get() as u64
        }
        fn inspect(_: &HostExternalInspection<'_>, value: &CounterPayload) -> EcoString {
            format!("Counter({})", value.value.get()).into()
        }
    }

    impl HostExternalStorage<Profile, Envelope> for EnvelopeStorage {
        type Payload = StoredRuntimeValue;
        fn store(stores: &Stores) -> &HostExternalStore<Self::Payload> {
            &stores.envelopes
        }
        fn source_equal(
            context: &HostExternalEquality<'_>,
            left: &Self::Payload,
            right: &Self::Payload,
        ) -> bool {
            context.provider_stored_values_equal(left, right)
        }
        fn source_hash(context: &HostExternalHashing<'_>, value: &Self::Payload) -> u64 {
            context.provider_stored_value_hash(value)
        }
        fn inspect(context: &HostExternalInspection<'_>, value: &Self::Payload) -> EcoString {
            format!("Envelope({})", context.provider_inspect_stored_value(value)).into()
        }
    }

    fn make_counter(
        mut call: HostCall<'_, Profile, Provider, HostExternalType<Counter>>,
    ) -> Result<HostCallCompletion<'_, HostExternalType<Counter>>, HostCallError> {
        let drops = Arc::clone(call.state());
        let value = call.create_external_with_binding::<Provider>(CounterPayload {
            value: Cell::new(41),
            drops,
        });
        Ok(call.return_value(value))
    }

    fn program(
        source: &str,
        provider: HostProviderModule<Profile>,
    ) -> crate::frontend::HostedTypedProgram<Profile> {
        let mut providers = WorkComponent::providers().expect("Future registration");
        providers.push(provider);
        compile_typed_host_program(
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
            HostProviderSet::from_providers(providers).expect("provider set"),
        )
        .expect("external source")
    }

    #[test]
    fn direct_calls_create_read_and_retain_send_only_external_payloads() {
        let execution_host = crate::execution_fixture::TestHost::default();

        fn read<'call>(
            call: HostCall<'call, Profile, Provider, BigInt>,
            value: HostExternal<'call, HostExternalType<Counter>>,
        ) -> Result<HostCallCompletion<'call, BigInt>, HostCallError> {
            let value = {
                let payload = call.external_payload::<Counter, HostTypeListEnd>(value);
                BigInt::from((*payload).value.get())
            };
            Ok(call.return_value(value))
        }
        fn wrap<'call>(
            mut call: HostCall<'call, Profile, Provider, HostExternalType<Envelope>>,
            value: HostExternal<'call, HostExternalType<Counter>>,
        ) -> Result<HostCallCompletion<'call, HostExternalType<Envelope>>, HostCallError> {
            let payload = call.retain_value::<HostExternalType<Counter>>(value);
            let value = call.create_external_with_binding::<Provider>(payload);
            Ok(call.return_value(value))
        }
        fn hash<'call>(
            call: HostCall<'call, Profile, Provider, BigInt>,
            value: HostExternal<'call, HostExternalType<Envelope>>,
        ) -> Result<HostCallCompletion<'call, BigInt>, HostCallError> {
            let hash = call.source_hash::<HostExternalType<Envelope>>(value);
            Ok(call.return_value(hash.into()))
        }
        let provider = HostProviderModule::new("application", "library")
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
        let (bindings, run) = HostedModuleBuilder::new(program(source, provider))
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
        execution_host
            .block_on(module.with_execution(
                &execution_host,
                &mut state,
                &mut echo,
                async |scope| {
                    assert_eq!(
                        scope.call(&run, ()).await.expect("source call"),
                        (41.into(), true)
                    );
                },
            ))
            .expect("direct calls");
        assert!(echo.0[0].ends_with("Envelope(Counter(41))"));
        assert_eq!(drops.load(Ordering::SeqCst), 2);
        drop(module);
        assert_eq!(drops.load(Ordering::SeqCst), 2);
    }

    #[test]
    fn owned_external_input_survives_pending_and_drops_with_the_work_not_its_waiter() {
        let execution_host = crate::execution_fixture::TestHost::default();

        fn read_later<'call>(
            call: HostCall<'call, Profile, Provider, WorkHostType<BigInt>>,
            constructions: HostConstructions<'call, HostTypeListEnd>,
            value: HostExternal<'call, HostExternalType<Counter>>,
        ) -> Result<HostCallCompletion<'call, WorkHostType<BigInt>>, HostCallError> {
            let value =
                call.provider_external_item_with::<Provider, Counter, HostTypeListEnd>(value);
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
                    Ok(HostOwnedCompletion::new(move |call, _| {
                        Ok(call.return_value(value))
                    }))
                })
            }))
        }
        let provider = HostProviderModule::new("application", "library").expect("identity")
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
        let (bindings, run) = HostedModuleBuilder::new(program(source, provider))
            .expect("plan")
            .function(FunctionDeclaration::<(), WorkType<BigInt>>::new("run"))
            .expect("entry");
        let mut module = bindings.seal().expect("seal");
        let drops = Arc::new(AtomicUsize::new(0));
        let mut state = (Arc::clone(&drops), ());
        let mut echo = Echo::default();
        let mut task = Box::pin(module.with_execution(
            &execution_host,
            &mut state,
            &mut echo,
            async |scope| {
                let abandoned = scope.call(&run, ()).await.expect("unpolled work");
                assert_eq!(drops.load(Ordering::SeqCst), 0);
                drop(abandoned);
                assert_eq!(drops.load(Ordering::SeqCst), 1);
                let work = scope.call(&run, ()).await.expect("work");
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
                let cancelled = scope.call(&run, ()).await.expect("cancelled work");
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
            },
        ));
        fn require_send<T: Send>(_: &T) {}
        require_send(&task);
        std::thread::scope(|threads| {
            threads
                .spawn(|| {
                    assert!(execution_host.poll(task.as_mut()).is_ready());
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
        let execution_host = crate::execution_fixture::TestHost::default();

        let provider = HostProviderModule::new("application", "library")
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
                    call: HostCall<'call, Profile, Provider, $ty>,
                    value: $ty,
                ) -> Result<HostCallCompletion<'call, $ty>, HostCallError> {
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
        let (bindings, run) = HostedModuleBuilder::new(program(source, provider))
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
        execution_host
            .block_on(module.with_execution(
                &execution_host,
                &mut state,
                &mut echo,
                async |scope| {
                    assert_eq!(
                        scope
                            .call(&run, values.clone())
                            .await
                            .expect("scalar returns"),
                        values
                    );
                },
            ))
            .expect("immediate scalar call");
        assert_eq!(echo.0.len(), 1);
        assert!(echo.0[0].ends_with("Counter(41)"));
        assert_eq!(state.0.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn external_returns_preserve_provider_failures_and_source_panics() {
        let execution_host = crate::execution_fixture::TestHost::default();

        fn bridge<'call>(
            call: HostCall<'call, Profile, Provider, HostExternalType<Counter>>,
            constructions: crate::HostConstructions<'call, HostTypeListEnd>,
            callback: crate::HostCallable<'call, HostTypeListEnd, HostExternalType<Counter>>,
        ) -> Result<crate::HostCallContinuation<'call, HostExternalType<Counter>>, HostCallError>
        {
            type Owned = crate::provider::Value<
                HostExternalType<Counter>,
                crate::provider::ProviderValueContext<HostExternalType<Counter>>,
            >;
            let callback = call.owned_callable(callback, &constructions);
            Ok(call.resume(constructions, move |context| {
                Box::pin(async move {
                    let value = callback
                        .invoke(
                            &context,
                            |_, _| (),
                            |call, _, value| Ok(Owned::from_host(&call, value)),
                        )
                        .await?;
                    Ok(crate::HostOwnedCompletion::new(move |mut call, _| {
                        let value = value.into_host(&mut call);
                        Ok(call.return_value(value))
                    }))
                })
            }))
        }
        fn fail(
            _: HostCall<'_, Profile, Provider, HostExternalType<Counter>>,
        ) -> Result<HostCallCompletion<'_, HostExternalType<Counter>>, HostCallError> {
            Err(crate::HostFailure::new("counter unavailable").into())
        }
        fn stop(
            _: HostCall<'_, Profile, Provider, HostExternalType<Counter>>,
        ) -> Result<std::convert::Infallible, HostCallError> {
            Err(crate::HostFailure::new("external construction stopped").into())
        }
        let provider = HostProviderModule::new("application", "library")
            .expect("identity")
            .with_external_type::<Provider, Counter>()
            .expect("schema")
            .with_scoped_function::<Provider, (), HostExternalType<Counter>, _>("make", make_counter)
            .expect("constructor")
            .with_resumable_function::<Provider, (crate::HostFunctionType<HostTypeListEnd, HostExternalType<Counter>>,), HostExternalType<Counter>, HostTypeListEnd, _>("bridge", bridge)
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
        let (mut bindings, run) = HostedModuleBuilder::new(program(source, provider))
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
        execution_host.block_on(module.with_execution(&execution_host, &mut state, &mut echo, async |scope| {
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
                        .call(entry, ()).await
                        .expect_err("original failure")
                        .to_string(),
                    message
                );
            }
            assert_eq!(
                scope
                    .call(&nested, (true,)).await
                    .expect_err("nested source failure")
                    .to_string(),
                "panic: external callback stopped"
            );
            scope
                .call(&nested, (false,)).await
                .expect("external callback success");
        }))
        .expect("failures are direct");
        assert_eq!(echo.0.len(), 1);
        assert!(echo.0[0].ends_with("Counter(41)"));
        assert_eq!(state.0.load(Ordering::SeqCst), 1);
    }
}

#[cfg(test)]
pub(crate) struct ExternalTestProfile;

#[cfg(test)]
#[derive(Default)]
pub(crate) struct ExternalTestRunState {
    pub(crate) provider: (),
}

#[cfg(test)]
#[derive(Default)]
pub(crate) struct ExternalTestStores {
    pub(crate) units: HostExternalStore<()>,
    pub(crate) integers: HostExternalStore<num_bigint::BigInt>,
    pub(crate) indices: HostExternalStore<usize>,
}

#[cfg(test)]
impl HostProfile for ExternalTestProfile {
    type RunState = ExternalTestRunState;
    type ExternalStores = ExternalTestStores;
    type ExecutionState = ();
}
