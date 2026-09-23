use super::EvaluatedCapture;
use super::drain::DrainQueue;
use std::fmt;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};

#[derive(Clone)]
pub(crate) struct CaptureStorage {
    releases: Arc<DrainQueue<Vec<EvaluatedCapture>>>,
    domain: ExecutionDomain,
}

#[derive(Clone, Default)]
pub(in crate::runtime) struct Captures {
    lease: Option<Arc<CaptureLease>>,
    domain: Option<ExecutionDomain>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::runtime) struct ExecutionDomain(u64);

struct CaptureLease {
    values: Vec<EvaluatedCapture>,
    storage: CaptureStorage,
}

impl Default for CaptureStorage {
    fn default() -> Self {
        Self {
            releases: Arc::new(DrainQueue::new()),
            domain: ExecutionDomain::new(),
        }
    }
}

impl CaptureStorage {
    pub(in crate::runtime) fn for_execution(&self) -> Self {
        Self {
            releases: Arc::clone(&self.releases),
            domain: ExecutionDomain::new(),
        }
    }

    pub(in crate::runtime) fn domain(&self) -> ExecutionDomain {
        self.domain
    }

    pub(in crate::runtime) fn capture(&self, values: Vec<EvaluatedCapture>) -> Captures {
        Captures {
            lease: if values.is_empty() {
                None
            } else {
                Some(Arc::new(CaptureLease {
                    values,
                    storage: self.clone(),
                }))
            },
            domain: Some(self.domain),
        }
    }
}

impl Captures {
    pub(in crate::runtime) fn domain(&self) -> Option<ExecutionDomain> {
        self.domain
    }

    pub(in crate::runtime) fn values(&self) -> &[EvaluatedCapture] {
        self.lease.as_ref().map_or(&[], |lease| &lease.values)
    }
}

// This is storage-handle identity, not Gleam function identity or value equality.
impl PartialEq for Captures {
    fn eq(&self, other: &Self) -> bool {
        self.domain == other.domain
            && match (&self.lease, &other.lease) {
                (None, None) => true,
                (Some(left), Some(right)) => Arc::ptr_eq(left, right),
                _ => false,
            }
    }
}

impl fmt::Debug for Captures {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("Captures")
            .field("lease", &self.lease.as_ref().map(Arc::as_ptr))
            .field("domain", &self.domain)
            .finish()
    }
}

impl ExecutionDomain {
    fn new() -> Self {
        static NEXT: AtomicU64 = AtomicU64::new(0);
        Self(NEXT.fetch_add(1, Ordering::Relaxed))
    }
}

impl Drop for CaptureLease {
    fn drop(&mut self) {
        // Descendants enqueue into this same drain without growing the drop stack.
        self.storage
            .releases
            .deliver([std::mem::take(&mut self.values)], drop);
    }
}

#[cfg(test)]
mod tests {
    use super::{CaptureStorage, Captures};
    use crate::plan::execution::function::{IntFunctionFunctionId, ProfiledFunctionFunctionId};
    use crate::plan::execution::graph::IntLocalId;
    use crate::runtime::state::RuntimeState;
    use crate::runtime::{EvaluatedCapture, HostCallOrigin, RetainedValues};
    use std::sync::Arc;

    #[test]
    fn empty_storage_has_no_lease_and_nonempty_copies_keep_the_same_owner() {
        let storage = CaptureStorage::default();
        let empty = storage.capture(Vec::new());
        assert!(empty.lease.is_none());
        assert_ne!(empty, Captures::default());
        assert_eq!(empty.values(), &[]);
        assert_eq!(
            format!("{empty:?}"),
            format!(
                "Captures {{ lease: None, domain: Some({:?}) }}",
                storage.domain()
            ),
        );
        assert_eq!(Arc::strong_count(&storage.releases), 1);

        let first = storage.capture(vec![EvaluatedCapture::int(IntLocalId(0), 42.into())]);
        let alias = first.clone();
        let second = storage.capture(vec![EvaluatedCapture::int(IntLocalId(0), 42.into())]);
        let other_storage = CaptureStorage::default();
        let other = other_storage.capture(vec![EvaluatedCapture::int(IntLocalId(0), 42.into())]);
        assert_eq!(first, alias);
        assert_ne!(first, second);
        assert_ne!(first, other);
        assert_ne!(first, empty);
        assert_ne!(empty, first);
        assert_eq!(
            first.values(),
            &[EvaluatedCapture::int(IntLocalId(0), 42.into())]
        );
        assert!(std::ptr::eq(first.values(), alias.values()));
        let lease = first.lease.as_ref().expect("nonempty capture lease");
        assert!(Arc::ptr_eq(&lease.storage.releases, &storage.releases));
        assert_eq!(
            format!("{first:?}"),
            format!(
                "Captures {{ lease: Some({:p}), domain: Some({:?}) }}",
                Arc::as_ptr(lease),
                storage.domain(),
            ),
        );
        assert_eq!(Arc::strong_count(&storage.releases), 3);
        let weak = Arc::downgrade(lease);
        drop(first);
        assert!(weak.upgrade().is_some());
        drop(alias);
        assert!(weak.upgrade().is_none());
        assert_eq!(Arc::strong_count(&storage.releases), 2);
        drop(second);
        assert_eq!(Arc::strong_count(&storage.releases), 1);
    }

    #[test]
    fn capture_storage_identity_keeps_its_execution_even_without_a_lease() {
        let storage = CaptureStorage::default();
        let captures = storage.capture(Vec::new());
        let alias = captures.clone();
        let same_execution = storage.capture(Vec::new());
        let other_execution = storage.for_execution().capture(Vec::new());
        assert_eq!(captures.domain(), Some(storage.domain()));
        assert_eq!(captures.values(), &[]);
        assert!(captures.lease.is_none());
        assert_eq!(captures, alias);
        assert_eq!(captures, same_execution);
        assert_ne!(captures, other_execution);
        assert_ne!(captures, Captures::default());
        assert_eq!(
            format!("{captures:?}"),
            format!(
                "Captures {{ lease: None, domain: Some(ExecutionDomain({})) }}",
                storage.domain().0,
            ),
        );
        assert_eq!(Captures::default().domain(), None);
        assert_eq!(
            format!("{:?}", Captures::default()),
            "Captures { lease: None, domain: None }"
        );
    }

    #[test]
    fn source_capture_graphs_release_on_a_bounded_stack_after_their_initial_owners() {
        const CHILD: &str = "GEAM_CAPTURE_RELEASE_CHILD";
        if std::env::var_os(CHILD).is_none() {
            for timeout in [
                std::time::Duration::ZERO,
                std::time::Duration::from_secs(120),
            ] {
                let mut child = std::process::Command::new(std::env::current_exe().expect("test binary"))
                .args([
                    "--exact",
                    "runtime::captures::tests::source_capture_graphs_release_on_a_bounded_stack_after_their_initial_owners",
                    "--nocapture",
                ])
                .env(CHILD, "1")
                .spawn()
                .expect("isolated stack regression");
                let deadline = std::time::Instant::now() + timeout;
                let completed = loop {
                    if std::time::Instant::now() >= deadline {
                        child.kill().expect("stop an unfinished regression");
                        child.wait().expect("reap child");
                        break false;
                    }
                    if let Some(status) = child.try_wait().expect("child status") {
                        assert!(status.success(), "capture release child failed: {status}");
                        break true;
                    }
                    std::thread::sleep(std::time::Duration::from_millis(10));
                };
                assert_eq!(completed, !timeout.is_zero());
            }
            return;
        }

        for source in [
            r#"
fn chain(n, previous) {
  case n {
    0 -> previous
    _ -> chain(n - 1, fn(value) { previous(value) + 1 })
  }
}
pub fn main() { chain(20000, fn(value: Int) { value }) }
"#,
            r#"
fn chain(n, previous) {
  case n {
    0 -> previous
    _ -> {
      let pair = #(previous, previous)
      chain(n - 1, fn(value) { pair.0(value) + pair.1(value) })
    }
  }
}
pub fn main() { chain(20000, fn(value: Int) { value }) }
"#,
            r#"
pub type Boxed { Boxed(callback: fn(Int) -> Int) }
fn chain(n, previous) {
  case n {
    0 -> previous
    _ -> {
      let boxed = Boxed(previous)
      chain(n - 1, fn(value) { boxed.callback(value) + 1 })
    }
  }
}
pub fn main() { chain(20000, fn(value: Int) { value }) }
"#,
            r#"
fn chain(n, previous) {
  case n {
    0 -> previous
    _ -> {
      let callbacks = [previous, previous]
      chain(n - 1, fn(value) {
        case callbacks {
          [first, second] -> first(value) + second(value)
          _ -> value
        }
      })
    }
  }
}
pub fn main() { chain(20000, fn(value: Int) { value }) }
"#,
        ] {
            let plan = crate::runtime::plan_src(source);
            let storage = CaptureStorage::default();
            let weak = Arc::downgrade(&storage.releases);
            let mut echo = Vec::new();
            let mut state =
                RuntimeState::with_host_storage(&mut echo, (), Default::default(), storage.clone());
            let value = crate::runtime::function::run_core_function(
                &plan,
                &mut state,
                ProfiledFunctionFunctionId::Int(IntFunctionFunctionId(0)),
                HostCallOrigin::Entry,
                RetainedValues::empty(),
            )
            .expect("source builds a capture graph");
            assert_eq!(Arc::strong_count(&storage.releases), 20_002);
            let alias = value.clone();
            assert_eq!(Arc::strong_count(&storage.releases), 20_002);
            drop(state);
            drop(plan);
            drop(storage);
            drop(value);
            assert!(weak.upgrade().is_some());
            std::thread::Builder::new()
                .stack_size(256 * 1024)
                .spawn(move || drop(alias))
                .expect("bounded-stack final owner")
                .join()
                .expect("iterative release");
            assert!(weak.upgrade().is_none());
        }
    }

    #[test]
    fn shared_source_branches_release_only_their_dead_capture_nodes() {
        let plan = crate::runtime::plan_src(
            r#"
fn chain(n, previous) {
  case n {
    0 -> previous
    _ -> chain(n - 1, fn(value) { previous(value) + 1 })
  }
}
pub fn main() {
  let shared = chain(1000, fn(value: Int) { value })
  #(shared, fn(value) { shared(value) + 1 }, fn(value) { shared(value) + 2 })
}
"#,
        );
        let storage = CaptureStorage::default();
        let mut echo = Vec::new();
        let mut state =
            RuntimeState::with_host_storage(&mut echo, (), Default::default(), storage.clone());
        let mut values = crate::runtime::function::run_tuple(
            &plan,
            &mut state,
            crate::plan::execution::function::TupleFunctionId(0),
            HostCallOrigin::Entry,
            RetainedValues::empty(),
        )
        .expect("source builds a capture graph");
        assert_eq!(values.len(), 3);
        assert_eq!(Arc::strong_count(&storage.releases), 1004);
        drop(values.pop());
        assert_eq!(Arc::strong_count(&storage.releases), 1003);
        drop(values.pop());
        assert_eq!(Arc::strong_count(&storage.releases), 1002);
        drop(values);
        assert_eq!(Arc::strong_count(&storage.releases), 2);
        drop(state);
        assert!(echo.is_empty());
    }

    #[test]
    fn mixed_callbacks_release_once_across_threads_and_a_single_destructor_unwind() {
        use crate::host::{
            HostCall, HostCallCompletion, HostCallError, HostCallable, HostCallableSchema,
            HostCaptures, HostConstructions, HostCreatedFunction, HostExternal,
            HostExternalBinding, HostExternalEquality, HostExternalHashing, HostExternalInspection,
            HostExternalSchema, HostExternalStorage, HostExternalStore, HostExternalType,
            HostFunctionType, HostProfile, HostProvider, HostProviderModule, HostProviderSet,
            HostReturns, HostStoredValue, HostTypeIndex0, HostTypeList, HostTypeListEnd,
        };
        use num_bigint::BigInt;
        use std::cell::Cell;
        use std::sync::atomic::{AtomicUsize, Ordering};

        struct Profile;
        struct Provider;
        struct Holder;
        type Arguments = HostTypeList<BigInt, HostTypeListEnd>;
        type Callback = HostFunctionType<Arguments, BigInt>;
        type Held = HostExternalType<Holder>;
        struct State {
            created: usize,
            drops: Arc<AtomicUsize>,
            panic_at: Option<usize>,
        }
        struct Payload {
            callback: HostStoredValue<Callback>,
            drops: Arc<AtomicUsize>,
            panic_on_drop: bool,
            dropped: Cell<bool>,
        }
        impl Drop for Payload {
            fn drop(&mut self) {
                assert!(!self.dropped.replace(true));
                self.drops.fetch_add(1, Ordering::SeqCst);
                assert!(!self.panic_on_drop, "one native destructor unwinds");
            }
        }
        impl HostProfile for Profile {
            type RunState = State;
            type ExternalStores = HostExternalStore<Payload>;
            type ExecutionState = ();
        }
        impl HostProvider<Profile> for Provider {
            type State = State;
            fn project(state: &mut State) -> &mut State {
                state
            }
        }
        impl HostExternalSchema for Holder {
            const PACKAGE: &'static str = "application";
            const MODULE: &'static str = "library";
            const NAME: &'static str = "Holder";
            const PARAMETER_COUNT: usize = 0;
        }
        impl HostExternalBinding<Profile, Holder> for Provider {
            type Storage = Provider;
        }
        impl HostExternalStorage<Profile, Holder> for Provider {
            type Payload = Payload;
            fn store(stores: &HostExternalStore<Payload>) -> &HostExternalStore<Payload> {
                stores
            }
            fn source_equal(
                context: &HostExternalEquality<'_>,
                left: &Payload,
                right: &Payload,
            ) -> bool {
                context.stored_values_equal(&left.callback, &right.callback)
            }
            fn source_hash(context: &HostExternalHashing<'_>, value: &Payload) -> u64 {
                context.stored_value_hash(&value.callback)
            }
            fn inspect(context: &HostExternalInspection<'_>, value: &Payload) -> ecow::EcoString {
                context.inspect_stored_value(&value.callback)
            }
        }
        struct Wrapper;
        impl HostCallableSchema for Wrapper {
            const PACKAGE: &'static str = "application";
            const MODULE: &'static str = "library";
            const NAME: &'static str = "wrapper";
            type Arguments = Arguments;
            type Return = BigInt;
            type Captures = HostTypeList<Callback, HostTypeListEnd>;
            type Constructions = HostTypeListEnd;
            type Completion = HostReturns;
        }
        type Factory = HostTypeList<HostCreatedFunction<Wrapper>, HostTypeListEnd>;
        fn wrap<'call>(
            mut call: HostCall<'call, Profile, Provider, Callback>,
            constructions: HostConstructions<'call, Factory>,
            previous: HostCallable<'call, Arguments, BigInt>,
        ) -> Result<HostCallCompletion<'call, Callback>, HostCallError> {
            let callback =
                call.construct_function(constructions.at::<HostTypeIndex0>(), (previous, ()));
            Ok(call.return_value(callback))
        }
        fn wrapper<'call>(
            call: HostCall<'call, Profile, Provider, BigInt>,
            captures: HostCaptures<'call, HostTypeList<Callback, HostTypeListEnd>>,
            constructions: HostConstructions<'call, HostTypeListEnd>,
            value: BigInt,
        ) -> Result<crate::HostCallContinuation<'call, BigInt>, HostCallError> {
            let (callback, ()) = call.captures(captures);
            let callback = call.owned_callable(callback, &constructions);
            Ok(call.resume(constructions, move |context| {
                Box::pin(async move {
                    let result = callback
                        .invoke(&context, move |_, _| (value, ()), |_, _, value| Ok(value))
                        .await
                        .unwrap();
                    Ok(crate::HostOwnedCompletion::new(move |call, _| {
                        Ok(call.return_value(result))
                    }))
                })
            }))
        }

        fn hold<'call>(
            mut call: HostCall<'call, Profile, Provider, Held>,
            callback: HostCallable<'call, Arguments, BigInt>,
        ) -> Result<HostCallCompletion<'call, Held>, HostCallError> {
            let state = call.state();
            state.created += 1;
            let drops = Arc::clone(&state.drops);
            let panic_on_drop = state.panic_at == Some(state.created);
            let value = call.create_external_with(|builder| Payload {
                callback: builder.store::<Callback>(callback),
                drops,
                panic_on_drop,
                dropped: Cell::new(false),
            });
            Ok(call.return_value(value))
        }
        fn open<'call>(
            mut call: HostCall<'call, Profile, Provider, Callback>,
            value: HostExternal<'call, Held>,
        ) -> Result<HostCallCompletion<'call, Callback>, HostCallError> {
            let alias = value;
            assert!(call.equal::<Held>(value, alias));
            assert_eq!(
                call.source_hash::<Held>(value),
                call.source_hash::<Held>(alias)
            );
            assert_eq!(call.inspect::<Held>(value), "//fn(a) { ... }");
            let payload = call.external_payload(value);
            let callback = payload.restore(&mut call, |payload| &payload.callback);
            drop(payload);
            Ok(call.return_value(callback))
        }

        for (native_wrappers, source) in [
            (
                false,
                r#"
pub type Holder
@external(erlang, "native", "hold")
fn hold(callback: fn(Int) -> Int) -> Holder
@external(erlang, "native", "open")
fn open(value: Holder) -> fn(Int) -> Int
fn chain(n, previous) {
  case n {
    0 -> previous
    _ -> {
      let held = hold(previous)
      chain(n - 1, fn(value) { open(held)(value) + 1 })
    }
  }
}
pub fn main() {
  let check = chain(4, fn(value: Int) { value })
  let assert 5 = check(1)
  #(chain(2000, fn(value: Int) { value }))
}
"#,
            ),
            (
                false,
                r#"
pub type Holder
@external(erlang, "native", "hold")
fn hold(callback: fn(Int) -> Int) -> Holder
@external(erlang, "native", "open")
fn open(value: Holder) -> fn(Int) -> Int
fn chain(n, previous) {
  case n {
    0 -> previous
    _ -> {
      let held = [hold(previous)]
      chain(n - 1, fn(value) {
        let assert [payload] = held
        open(payload)(value) + 1
      })
    }
  }
}
pub fn main() {
  let check = chain(4, fn(value: Int) { value })
  let assert 5 = check(1)
  #(chain(2000, fn(value: Int) { value }))
}
"#,
            ),
            (
                true,
                r#"
pub type Holder
@external(erlang, "native", "hold")
fn hold(callback: fn(Int) -> Int) -> Holder
@external(erlang, "native", "open")
fn open(value: Holder) -> fn(Int) -> Int
@external(erlang, "native", "wrap")
fn wrap(callback: fn(Int) -> Int) -> fn(Int) -> Int
fn chain(n, previous) {
  case n {
    0 -> previous
    _ -> {
      let held = hold(previous)
      chain(n - 1, wrap(fn(value) { open(held)(value) + 1 }))
    }
  }
}
pub fn main() {
  let check = chain(4, fn(value: Int) { value })
  let assert 5 = check(1)
  #(chain(2000, fn(value: Int) { value }))
}
"#,
            ),
        ] {
            for panic_at in [None, Some(500)] {
                let provider = HostProviderModule::new("application", "library")
                    .unwrap()
                    .with_external_type::<Provider, Holder>()
                    .unwrap()
                    .with_scoped_function::<Provider, (Callback,), Held, _>("hold", hold)
                    .unwrap()
                    .with_scoped_function::<Provider, (Held,), Callback, _>("open", open)
                    .unwrap();
                let provider = if native_wrappers {
                    provider.with_resumable_callable::<Provider, Wrapper, (BigInt,), _>(wrapper).unwrap()
                        .with_scoped_function_and_constructions::<Provider, (Callback,), Callback, Factory, _>("wrap", wrap).unwrap()
                } else {
                    provider
                };
                let typed = crate::compile_typed_host_program(
                    "application",
                    "library",
                    [crate::PackageSource::new(
                        "application",
                        Vec::<String>::new(),
                        [crate::ModuleSource::new(
                            "library",
                            "src/library.gleam",
                            source,
                        )],
                    )],
                    HostProviderSet::<Profile>::from_providers([provider]).unwrap(),
                )
                .unwrap();
                let mut execution = crate::HostedExecution::try_from_module_plan(
                    crate::plan_host_program(typed).unwrap(),
                )
                .unwrap();
                let drops = Arc::new(AtomicUsize::new(0));
                let mut state = State {
                    created: 0,
                    drops: Arc::clone(&drops),
                    panic_at,
                };
                let host = crate::execution_fixture::TestHost::default();
                let mut echo = Vec::new();
                let (plan, stores, captures) = execution.parts_mut();
                let capture_owner = captures.clone();
                let weak = Arc::downgrade(&captures.releases);
                let domain = crate::runtime::execution::Domain::new(
                    Arc::clone(plan),
                    &host,
                    &mut state,
                    stores,
                    &mut echo,
                    captures.clone(),
                    crate::runtime::execution::Domain::<Profile>::DEFAULT_BUDGET,
                );
                let context = domain.context();
                let values = host
                    .block_on(domain.drive(context.call(
                        crate::plan::execution::function::TupleFunctionId(0),
                        HostCallOrigin::Entry,
                        RetainedValues::empty(),
                    )))
                    .expect("domain cleanup")
                    .expect("active entry")
                    .expect("native chain");
                drop(context);
                assert_eq!(state.created, 2004);
                assert_eq!(drops.load(Ordering::SeqCst), 4);
                assert_eq!(
                    Arc::strong_count(&capture_owner.releases),
                    2002 + 2000 * usize::from(native_wrappers)
                );
                drop(execution);
                drop(host);
                drop(state);
                let barrier = Arc::new(std::sync::Barrier::new(4));
                let mut workers = Vec::new();
                for value in [values.clone(), values.clone(), values.clone(), values] {
                    let barrier = Arc::clone(&barrier);
                    workers.push(
                        std::thread::Builder::new()
                            .stack_size(256 * 1024)
                            .spawn(move || {
                                barrier.wait();
                                std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                                    drop(value)
                                }))
                            })
                            .expect("bounded-stack native owner thread"),
                    );
                }
                let panics = workers
                    .into_iter()
                    .map(|worker| usize::from(worker.join().expect("native owner thread").is_err()))
                    .sum::<usize>();
                assert_eq!(panics, usize::from(panic_at.is_some()));
                assert_eq!(drops.load(Ordering::SeqCst), 2004);
                assert_eq!(Arc::strong_count(&capture_owner.releases), 1);
                let next =
                    capture_owner.capture(vec![EvaluatedCapture::int(IntLocalId(0), 42.into())]);
                drop(next);
                assert_eq!(Arc::strong_count(&capture_owner.releases), 1);
                drop(capture_owner);
                assert!(weak.upgrade().is_none());
                assert!(echo.is_empty());
            }
        }
    }
}
