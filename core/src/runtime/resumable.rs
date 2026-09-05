use crate::runtime::graph::RuntimeGraphState;
use crate::runtime::transfer::TransferListStorage;
use crate::runtime::{EchoOutput, EchoSink, EvaluatedBitArray, EvaluatedValue, TransferValues};
use ecow::EcoString;
use num_bigint::BigInt;

#[cfg(test)]
use std::sync::{Arc, Mutex};

pub(in crate::runtime) use function::{
    ResumableFuture, run_bit_array, run_bit_array_list, run_bool, run_bool_list, run_core_function,
    run_custom, run_custom_list, run_external, run_external_function_function, run_external_list,
    run_float, run_float_list, run_function_list, run_int, run_int_list, run_list_list, run_never,
    run_never_value, run_nil, run_nil_list, run_parameter_list, run_parameter_list_list,
    run_string, run_string_list, run_tuple, run_tuple_list, run_utf_codepoint,
    run_utf_codepoint_list,
};

pub(crate) use crate::AsyncExecutionError as TransferExecutionError;
pub(crate) use callback::{AsyncHostCallbackRequest, ResumableCallback};
pub(in crate::runtime) use error::TransferExecutionResult;
pub(in crate::runtime) use host::drive_async_host;

pub(crate) struct TransferInputs(crate::runtime::graph::ProfiledRetainedValues<TransferValues>);

#[cfg(test)]
type EchoRecords = Arc<Mutex<Vec<(Option<EcoString>, String)>>>;

#[cfg(test)]
#[derive(Default)]
pub(in crate::runtime) struct RecordedEcho(pub(in crate::runtime) EchoRecords);

#[cfg(test)]
impl EchoSink for RecordedEcho {
    fn emit(&mut self, output: EchoOutput) {
        self.0.lock().expect("echo observation lock").push((
            output.message().cloned(),
            output.value().inspect().to_string(),
        ));
    }
}

impl TransferInputs {
    pub(crate) fn empty() -> Self {
        Self(crate::runtime::graph::ProfiledRetainedValues::empty())
    }

    pub(crate) fn push_int(&mut self, value: BigInt) {
        self.0.push_evaluated(EvaluatedValue::Int(value));
    }

    pub(crate) fn push_float(&mut self, value: f64) {
        self.0.push_evaluated(EvaluatedValue::Float(value));
    }

    pub(crate) fn push_string(&mut self, value: EcoString) {
        self.0.push_evaluated(EvaluatedValue::String(value));
    }

    pub(crate) fn push_bit_array(&mut self, value: crate::BitArrayValue) {
        self.0
            .push_evaluated(EvaluatedValue::BitArray(EvaluatedBitArray::from_value(
                value,
            )));
    }

    pub(crate) fn push_utf_codepoint(&mut self, value: char) {
        self.0.push_evaluated(EvaluatedValue::UtfCodepoint(value));
    }

    pub(crate) fn push_bool(&mut self, value: bool) {
        self.0.push_evaluated(EvaluatedValue::Bool(value));
    }

    pub(crate) fn push_nil(&mut self) {
        self.0.push_evaluated(EvaluatedValue::Nil);
    }

    pub(crate) fn push_input(&mut self, input: crate::runtime::EmbeddingInput<TransferValues>) {
        input.retain(&mut self.0);
    }

    pub(in crate::runtime) fn into_retained(
        self,
    ) -> crate::runtime::graph::ProfiledRetainedValues<TransferValues> {
        self.0
    }
}

macro_rules! run_embedded_scalar {
    ($name:ident, $run:ident, $function:ty, $return:ty, $map:expr) => {
        pub(crate) async fn $name<Profile>(
            plan: &crate::plan::execution::AsyncHostedExecution<Profile>,
            function: $function,
            inputs: TransferInputs,
            stores: &mut Profile::ExternalStores,
            host: &mut Profile::RunState,
            echo: &mut (dyn EchoSink + Send),
        ) -> Result<$return, crate::AsyncExecutionError>
        where
            Profile: crate::HostProfile,
            Profile::RunState: Send,
            Profile::ExternalStores: Send,
        {
            let mut state = ResumableState::<Profile>::new(host, stores, echo);
            $run(
                plan,
                &mut state,
                function,
                crate::runtime::error::HostCallOrigin::Entry,
                inputs.into_retained(),
            )
            .await
            .map($map)
        }
    };
}

run_embedded_scalar!(
    run_embedded_int,
    run_int,
    crate::plan::execution::function::IntFunctionId,
    BigInt,
    std::convert::identity
);
run_embedded_scalar!(
    run_embedded_float,
    run_float,
    crate::plan::execution::function::FloatFunctionId,
    f64,
    std::convert::identity
);
run_embedded_scalar!(
    run_embedded_string,
    run_string,
    crate::plan::execution::function::StringFunctionId,
    EcoString,
    std::convert::identity
);
run_embedded_scalar!(
    run_embedded_bit_array,
    run_bit_array,
    crate::plan::execution::function::BitArrayFunctionId,
    crate::BitArrayValue,
    EvaluatedBitArray::into_value
);
run_embedded_scalar!(
    run_embedded_utf_codepoint,
    run_utf_codepoint,
    crate::plan::execution::function::UtfCodepointFunctionId,
    char,
    std::convert::identity
);
run_embedded_scalar!(
    run_embedded_bool,
    run_bool,
    crate::plan::execution::function::BoolFunctionId,
    bool,
    std::convert::identity
);
run_embedded_scalar!(
    run_embedded_nil,
    run_nil,
    crate::plan::execution::function::NilFunctionId,
    (),
    std::convert::identity
);

pub(crate) async fn run_embedded_custom<Profile>(
    plan: &crate::plan::execution::AsyncHostedExecution<Profile>,
    function: crate::plan::execution::function::CustomFunctionId,
    inputs: TransferInputs,
    stores: &mut Profile::ExternalStores,
    host: &mut Profile::RunState,
    echo: &mut (dyn EchoSink + Send),
) -> Result<crate::runtime::EmbeddingOutput<TransferValues>, crate::AsyncExecutionError>
where
    Profile: crate::HostProfile,
    Profile::RunState: Send,
    Profile::ExternalStores: Send,
{
    let mut state = ResumableState::<Profile>::new(host, stores, echo);
    run_custom(
        plan,
        &mut state,
        function,
        crate::runtime::error::HostCallOrigin::Entry,
        inputs.into_retained(),
    )
    .await
    .map(crate::runtime::EmbeddingOutput::from_custom)
}

pub(crate) async fn run_embedded_tuple<Profile>(
    plan: &crate::plan::execution::AsyncHostedExecution<Profile>,
    function: crate::plan::execution::function::TupleFunctionId,
    inputs: TransferInputs,
    stores: &mut Profile::ExternalStores,
    host: &mut Profile::RunState,
    echo: &mut (dyn EchoSink + Send),
) -> Result<crate::runtime::EmbeddingOutput<TransferValues>, crate::AsyncExecutionError>
where
    Profile: crate::HostProfile,
    Profile::RunState: Send,
    Profile::ExternalStores: Send,
{
    let mut state = ResumableState::<Profile>::new(host, stores, echo);
    run_tuple(
        plan,
        &mut state,
        function,
        crate::runtime::error::HostCallOrigin::Entry,
        inputs.into_retained(),
    )
    .await
    .map(crate::runtime::EmbeddingOutput::from_tuple)
}

pub(crate) async fn run_embedded_list<Profile>(
    plan: &crate::plan::execution::AsyncHostedExecution<Profile>,
    function: &crate::plan::execution::function::LibraryListFunctionId,
    inputs: TransferInputs,
    stores: &mut Profile::ExternalStores,
    host: &mut Profile::RunState,
    echo: &mut (dyn EchoSink + Send),
) -> Result<crate::runtime::EmbeddingOutput<TransferValues>, crate::AsyncExecutionError>
where
    Profile: crate::HostProfile,
    Profile::RunState: Send,
    Profile::ExternalStores: Send,
{
    use crate::plan::execution::function::LibraryListFunctionId;
    use crate::runtime::state::list::ListValueId;

    let mut state = ResumableState::<Profile>::new(host, stores, echo);
    let origin = crate::runtime::error::HostCallOrigin::Entry;
    let inputs = inputs.into_retained();
    let value: ListValueId<TransferValues> = match *function {
        LibraryListFunctionId::Int(function) => {
            run_int_list(plan, &mut state, function, origin, inputs)
                .await
                .map(ListValueId::Int)
        }
        LibraryListFunctionId::String(function) => {
            run_string_list(plan, &mut state, function, origin, inputs)
                .await
                .map(ListValueId::String)
        }
        LibraryListFunctionId::BitArray(function) => {
            run_bit_array_list(plan, &mut state, function, origin, inputs)
                .await
                .map(ListValueId::BitArray)
        }
        LibraryListFunctionId::UtfCodepoint(function) => {
            run_utf_codepoint_list(plan, &mut state, function, origin, inputs)
                .await
                .map(ListValueId::UtfCodepoint)
        }
        LibraryListFunctionId::Custom(function) => {
            run_custom_list(plan, &mut state, function, origin, inputs)
                .await
                .map(ListValueId::Custom)
        }
        LibraryListFunctionId::Float(function) => {
            run_float_list(plan, &mut state, function, origin, inputs)
                .await
                .map(ListValueId::Float)
        }
        LibraryListFunctionId::Bool(function) => {
            run_bool_list(plan, &mut state, function, origin, inputs)
                .await
                .map(ListValueId::Bool)
        }
        LibraryListFunctionId::Nil(function) => {
            run_nil_list(plan, &mut state, function, origin, inputs)
                .await
                .map(ListValueId::Nil)
        }
        LibraryListFunctionId::Tuple(function) => {
            run_tuple_list(plan, &mut state, function, origin, inputs)
                .await
                .map(ListValueId::Tuple)
        }
        LibraryListFunctionId::List(function) => {
            run_list_list(plan, &mut state, function, origin, inputs)
                .await
                .map(ListValueId::List)
        }
    }?;
    Ok(crate::runtime::EmbeddingOutput::from_value(value.into()))
}

pub(crate) struct ResumableState<'call, Profile: crate::HostProfile> {
    host: &'call mut Profile::RunState,
    stores: &'call mut Profile::ExternalStores,
    echo: &'call mut (dyn EchoSink + Send),
    lists: TransferListStorage,
}

impl<'call, Profile: crate::HostProfile> ResumableState<'call, Profile> {
    pub(in crate::runtime) fn new(
        host: &'call mut Profile::RunState,
        stores: &'call mut Profile::ExternalStores,
        echo: &'call mut (dyn EchoSink + Send),
    ) -> Self {
        Self {
            host,
            stores,
            echo,
            lists: TransferListStorage::default(),
        }
    }

    pub(in crate::runtime) fn host(&mut self) -> &mut Profile::RunState {
        self.host
    }

    pub(in crate::runtime) fn lists_mut(&mut self) -> &mut TransferListStorage {
        &mut self.lists
    }
}

impl<Profile: crate::HostProfile> RuntimeGraphState<TransferValues>
    for ResumableState<'_, Profile>
{
    type Error = TransferExecutionError;

    fn lists(&self) -> &TransferListStorage {
        &self.lists
    }

    fn lists_mut(&mut self) -> &mut TransferListStorage {
        &mut self.lists
    }

    fn emit_echo(&mut self, output: EchoOutput) {
        self.echo.emit(output);
    }

    fn source_panic(
        &self,
        source: Option<&crate::plan::SourceContext>,
        kind: crate::PanicKind,
        message: Option<EcoString>,
        site: crate::PanicSite,
    ) -> Self::Error {
        TransferExecutionError::source_panic(source, kind, message, site)
    }

    fn let_assert_panic<Plan>(
        &self,
        plan: &Plan,
        source: Option<&crate::plan::SourceContext>,
        message: Option<EcoString>,
        site: crate::PanicSite,
        subject: EvaluatedValue<TransferValues>,
        pattern_span: crate::SourceSpan,
    ) -> Self::Error
    where
        Plan: crate::plan::execution::runtime::RuntimeExecutionPlan,
    {
        TransferExecutionError::let_assert_panic(
            source,
            message,
            site,
            crate::AsyncPanicValue::new(plan, &self.lists, subject),
            pattern_span,
        )
    }

    fn bit_array_segment_panic(
        &self,
        source: Option<&crate::plan::SourceContext>,
        reason: crate::BitArraySegmentPanicReason,
        site: crate::PanicSite,
    ) -> Self::Error {
        TransferExecutionError::bit_array_segment_panic(source, reason, site)
    }
}

mod callback;
mod error;
mod function;
mod host;

#[cfg(test)]
mod tests {
    use super::{RecordedEcho, TransferInputs, run_embedded_int};
    use crate::host::{
        AsyncHostModule, AsyncHostProviderModule, AsyncHostProviderSet, HostExternalSchema,
        HostFailure,
    };
    use crate::plan::execution::AsyncHostedExecution;
    use crate::plan::execution::runtime::RuntimeExecutionPlan;
    use crate::plan::execution::type_::ExternalTypeId;
    use crate::plan::{ExternalType, ExternalTypeName, LibraryEntry, LibraryValueType};
    use crate::runtime::transfer::{
        TransferExternalEquality, TransferExternalHashing, TransferExternalInspection,
        TransferExternalStore, TransferStoredRuntimeValue,
    };
    use crate::runtime::{EvaluatedExternalValue, EvaluatedValue, TransferValues};
    use crate::{ExecutionError, ModuleSource, PackageSource, compile_typed_async_host_program};
    use ecow::EcoString;
    use miette::Diagnostic;
    use num_bigint::BigInt;
    use std::convert::Infallible;
    use std::future::{Future, Ready, ready};
    use std::pin::Pin;
    use std::sync::Arc;
    use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
    use std::task::{Context, Poll, Wake, Waker};

    struct CounterSchema;

    impl HostExternalSchema for CounterSchema {
        const PACKAGE: &'static str = "application";
        const MODULE: &'static str = "library";
        const NAME: &'static str = "Counter";
        const PARAMETER_COUNT: usize = 0;
    }

    #[derive(Clone, Default)]
    struct PayloadObservations {
        equal_calls: Arc<AtomicUsize>,
        hash_calls: Arc<AtomicUsize>,
        inspect_calls: Arc<AtomicUsize>,
        drops: Arc<AtomicUsize>,
    }

    struct CounterPayload {
        value: usize,
        observations: PayloadObservations,
    }

    impl Drop for CounterPayload {
        fn drop(&mut self) {
            self.observations.drops.fetch_add(1, Ordering::SeqCst);
        }
    }

    fn counter_equal(
        context: &TransferExternalEquality<'_>,
        left: &CounterPayload,
        right: &CounterPayload,
    ) -> bool {
        left.observations.equal_calls.fetch_add(1, Ordering::SeqCst);
        context.stored_values_equal(
            &TransferStoredRuntimeValue::new(EvaluatedValue::Int(left.value.into())),
            &TransferStoredRuntimeValue::new(EvaluatedValue::Int(right.value.into())),
        )
    }

    fn counter_hash(context: &TransferExternalHashing<'_>, value: &CounterPayload) -> u64 {
        value.observations.hash_calls.fetch_add(1, Ordering::SeqCst);
        context.stored_value_hash(&TransferStoredRuntimeValue::new(EvaluatedValue::Int(
            value.value.into(),
        )))
    }

    fn counter_inspect(
        context: &TransferExternalInspection<'_>,
        value: &CounterPayload,
    ) -> EcoString {
        value
            .observations
            .inspect_calls
            .fetch_add(1, Ordering::SeqCst);
        format!(
            "Counter({})",
            context.inspect_stored_value(&TransferStoredRuntimeValue::new(EvaluatedValue::Int(
                value.value.into()
            )))
        )
        .into()
    }

    struct PendingOnce {
        value: Ready<BigInt>,
        pending_returned: bool,
    }

    impl Future for PendingOnce {
        type Output = BigInt;

        fn poll(mut self: Pin<&mut Self>, context: &mut Context<'_>) -> Poll<Self::Output> {
            if self.pending_returned {
                Pin::new(&mut self.value).poll(context)
            } else {
                self.pending_returned = true;
                context.waker().wake_by_ref();
                Poll::Pending
            }
        }
    }

    fn stop_external() -> Result<Infallible, HostFailure> {
        Err(HostFailure::new("external stop"))
    }

    struct WakeFlag(AtomicBool);

    struct CallbackProvider;

    impl crate::HostProvider<crate::StatelessHostProfile> for CallbackProvider {
        type State = ();

        fn project(state: &mut ()) -> &mut () {
            state
        }
    }

    type CallbackArguments = crate::HostTypeList<BigInt, crate::HostTypeListEnd>;

    fn repeat_callback<'call>(
        mut call: crate::AsyncHostCall<
            'call,
            crate::StatelessHostProfile,
            CallbackProvider,
            BigInt,
        >,
        callback: crate::AsyncHostCallable<
            'call,
            crate::StatelessHostProfile,
            CallbackArguments,
            BigInt,
        >,
        value: BigInt,
    ) -> crate::AsyncHostFuture<'call, Result<BigInt, crate::AsyncHostCallError>> {
        crate::AsyncHostFuture::new(async move {
            let first = call.invoke(&callback, (value, ())).await?;
            call.invoke(&callback, (first, ())).await
        })
    }

    impl Wake for WakeFlag {
        fn wake(self: Arc<Self>) {
            self.0.store(true, Ordering::SeqCst);
        }
    }

    fn external_execution() -> (
        AsyncHostedExecution<crate::StatelessHostProfile>,
        crate::plan::execution::function::IntFunctionId,
        crate::plan::execution::function::IntFunctionId,
        crate::plan::execution::function::IntFunctionId,
        crate::plan::execution::function::IntFunctionId,
        ExternalTypeId,
    ) {
        let control = AsyncHostModule::new("host_support", "host/control")
            .expect("source-less async host module")
            .with_fallible_function("stop", stop_external)
            .expect("non-returning external host function");
        let provider = AsyncHostProviderModule::new("application", "library")
            .expect("async provider module")
            .with_external_type_for_test::<CounterSchema>()
            .with_async_function("suspend", |value: BigInt| PendingOnce {
                value: ready(value),
                pending_returned: false,
            })
            .expect("async host function")
            .with_fallible_scoped_async_function::<
                CallbackProvider,
                (crate::HostFunctionType<CallbackArguments, BigInt>, BigInt),
                BigInt,
                _,
            >("repeat_callback", repeat_callback)
            .expect("repeat captured external callback");
        let providers =
            AsyncHostProviderSet::with_providers([control], [provider]).expect("async host set");
        let program = compile_typed_async_host_program(
            "application",
            "library",
            [PackageSource::new(
                "application",
                ["host_support"],
                [ModuleSource::new(
                    "library",
                    "src/library.gleam",
                    r#"
import host/control

@external(erlang, "native", "Counter")
pub type Counter

@external(erlang, "native", "suspend")
fn suspend(value: Int) -> Int

@external(erlang, "native", "repeat_callback")
fn repeat_callback(callback: fn(Int) -> Int, value: Int) -> Int

fn stop_counter() -> fn() -> Counter {
  control.stop
}

fn identity(counter: Counter) -> Counter {
  counter
}

fn tail_identity(counter: Counter) -> Counter {
  identity(counter)
}

fn singleton(counter: Counter) -> List(Counter) {
  [counter]
}

fn tail_singleton(counter: Counter) -> List(Counter) {
  singleton(counter)
}

pub type CounterListBox {
  CounterListBox(values: List(Counter))
}

fn maker(counter: Counter) -> fn() -> Counter {
  fn() { counter }
}

fn tail_maker(counter: Counter) -> fn() -> Counter {
  maker(counter)
}

fn list_maker(counter: Counter) -> fn() -> List(Counter) {
  fn() { [counter] }
}

fn tail_list_maker(counter: Counter) -> fn() -> List(Counter) {
  list_maker(counter)
}

fn maker_factory(counter: Counter) -> fn() -> fn() -> Counter {
  fn() { fn() { counter } }
}

fn tail_maker_factory(counter: Counter) -> fn() -> fn() -> Counter {
  maker_factory(counter)
}

fn list_maker_factory(counter: Counter) -> fn() -> fn() -> List(Counter) {
  fn() { fn() { [counter] } }
}

fn tail_list_maker_factory(counter: Counter) -> fn() -> fn() -> List(Counter) {
  list_maker_factory(counter)
}

pub fn retain(counter: Counter) -> Int {
  let projected = #(counter).0
  let retained = tail_identity(projected)
  let counters = tail_singleton(retained)
  let tuple_counters = #(counters).0
  let CounterListBox(field_counters) = CounterListBox(counters)
  let make_counter = tail_maker(retained)
  let make_counters = tail_list_maker(retained)
  let make_counter_factory = tail_maker_factory(retained)
  let make_counters_factory = tail_list_maker_factory(retained)
  echo retained as "before"
  let resumed = suspend(7)
  echo retained as "after"
  let from_function = make_counter()
  let from_list_function = make_counters()
  let from_function_function = make_counter_factory()()
  let from_list_function_function = make_counters_factory()()
  case #(
    counter == retained,
    retained == from_function,
    counters == from_list_function,
    counters == tuple_counters,
    counters == field_counters,
    retained == from_function_function,
    counters == from_list_function_function,
  ) {
    #(True, True, True, True, True, True, True) -> resumed
    _ -> 0
  }
}

pub fn stop_after_pending() -> Int {
  let _ = suspend(0)
  let stop = stop_counter()
  let _ = stop()
  0
}

fn reject_counter(counter: Counter, value: Int) -> Int {
  let value = suspend(value)
  let assert [] = [counter] as "expected no counter"
  value
}

pub fn reject(counter: Counter) -> Int {
  repeat_callback(fn(value) { reject_counter(counter, value) }, 0)
}

pub fn repeat(counter: Counter) -> Int {
  let callback = fn(value) {
    let value = suspend(value)
    case counter == counter {
      True -> value + 1
      False -> 0
    }
  }
  repeat_callback(callback, 0)
}
"#,
                )],
            )],
            providers,
        )
        .expect("external retention source");
        let plan = crate::planner::plan_async_host_library_program(program)
            .expect("external retention plan");
        let retain_template = plan
            .functions()
            .iter()
            .find(|function| function.name() == "retain")
            .expect("public retain function")
            .signature()
            .id();
        let stop_template = plan
            .functions()
            .iter()
            .find(|function| function.name() == "stop_after_pending")
            .expect("public external stop function")
            .signature()
            .id();
        let reject_template = plan
            .functions()
            .iter()
            .find(|function| function.name() == "reject")
            .expect("public assertion function")
            .signature()
            .id();
        let retain_entry = LibraryEntry::new(
            retain_template,
            LibraryValueType::Int,
            Vec::new(),
            Vec::new(),
        );
        let stop_entry =
            LibraryEntry::new(stop_template, LibraryValueType::Int, Vec::new(), Vec::new());
        let reject_entry = LibraryEntry::new(
            reject_template,
            LibraryValueType::Int,
            Vec::new(),
            Vec::new(),
        );
        let repeat_template = plan
            .functions()
            .iter()
            .find(|function| function.name() == "repeat")
            .expect("public repeat function")
            .signature()
            .id();
        let repeat_entry = LibraryEntry::new(
            repeat_template,
            LibraryValueType::Int,
            Vec::new(),
            Vec::new(),
        );
        let (execution, entries) = AsyncHostedExecution::from_library_plan(
            plan,
            retain_entry,
            vec![stop_entry, reject_entry, repeat_entry],
        );
        let retain_function = *entries.ints[0].function();
        let stop_function = *entries.ints[1].function();
        let reject_function = *entries.ints[2].function();
        let repeat_function = *entries.ints[3].function();
        let type_id = ExternalTypeId::new(0);

        assert_eq!(
            execution.value_metadata().external_value_type(type_id),
            ExternalType::new(
                ExternalTypeName::new("application".into(), "library".into(), "Counter".into()),
                Vec::new(),
            ),
        );

        (
            execution,
            retain_function,
            stop_function,
            reject_function,
            repeat_function,
            type_id,
        )
    }

    fn external_input(
        type_id: ExternalTypeId,
        observations: &PayloadObservations,
    ) -> TransferInputs {
        let store = TransferExternalStore::default();
        let lease = store.insert(
            CounterPayload {
                value: 7,
                observations: observations.clone(),
            },
            counter_equal,
            counter_hash,
            counter_inspect,
        );
        let external = EvaluatedExternalValue::<TransferValues>::new(type_id, lease);
        let stored_hash = |_: &TransferStoredRuntimeValue| 17;
        let hashing = TransferExternalHashing::new(&stored_hash);

        assert_eq!(external.source_hash(&hashing), 17);
        drop(store);

        let mut inputs = TransferInputs::empty();
        inputs.0.push_evaluated(EvaluatedValue::External(external));
        inputs
    }

    #[test]
    fn external_payload_survives_pending_and_releases_on_completion_or_cancellation() {
        let (execution, function, _, _, _, type_id) = external_execution();
        let wake = Arc::new(WakeFlag(AtomicBool::new(false)));
        let waker = Waker::from(Arc::clone(&wake));
        let mut context = Context::from_waker(&waker);

        let completed = PayloadObservations::default();
        let mut state = ();
        let mut stores = ();
        let mut echo = RecordedEcho::default();
        let observed = Arc::clone(&echo.0);
        let mut call = Box::pin(run_embedded_int(
            &execution,
            function,
            external_input(type_id, &completed),
            &mut stores,
            &mut state,
            &mut echo,
        ));

        assert_eq!(call.as_mut().poll(&mut context), Poll::Pending);
        assert!(wake.0.load(Ordering::SeqCst));
        assert_eq!(completed.drops.load(Ordering::SeqCst), 0);
        assert_eq!(completed.hash_calls.load(Ordering::SeqCst), 1);
        assert_eq!(completed.inspect_calls.load(Ordering::SeqCst), 1);
        assert_eq!(completed.equal_calls.load(Ordering::SeqCst), 0);
        assert_eq!(
            *observed.lock().expect("echo observation lock"),
            [(Some("before".into()), "Counter(7)".into())],
        );

        assert_eq!(call.as_mut().poll(&mut context), Poll::Ready(Ok(7.into())));
        assert_eq!(completed.equal_calls.load(Ordering::SeqCst), 7);
        assert_eq!(completed.inspect_calls.load(Ordering::SeqCst), 1);
        assert_eq!(
            *observed.lock().expect("echo observation lock"),
            [
                (Some("before".into()), "Counter(7)".into()),
                (Some("after".into()), "Counter(7)".into()),
            ],
        );
        drop(call);
        assert_eq!(completed.drops.load(Ordering::SeqCst), 1);

        let cancelled = PayloadObservations::default();
        let mut echo = RecordedEcho::default();
        let observed = Arc::clone(&echo.0);
        wake.0.store(false, Ordering::SeqCst);
        let mut call = Box::pin(run_embedded_int(
            &execution,
            function,
            external_input(type_id, &cancelled),
            &mut stores,
            &mut state,
            &mut echo,
        ));

        assert_eq!(call.as_mut().poll(&mut context), Poll::Pending);
        assert!(wake.0.load(Ordering::SeqCst));
        assert_eq!(cancelled.drops.load(Ordering::SeqCst), 0);
        assert_eq!(
            *observed.lock().expect("echo observation lock"),
            [(Some("before".into()), "Counter(7)".into())],
        );
        drop(call);
        assert_eq!(cancelled.drops.load(Ordering::SeqCst), 1);
        assert_eq!(cancelled.equal_calls.load(Ordering::SeqCst), 0);
        assert_eq!(cancelled.inspect_calls.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn non_returning_host_specialized_to_an_external_return_preserves_its_failure() {
        let (execution, _, function, _, _, _) = external_execution();
        let mut state = ();
        let mut stores = ();
        let mut echo = RecordedEcho::default();
        let wake = Arc::new(WakeFlag(AtomicBool::new(false)));
        let waker = Waker::from(wake);
        let mut context = Context::from_waker(&waker);
        let mut call = Box::pin(run_embedded_int(
            &execution,
            function,
            TransferInputs::empty(),
            &mut stores,
            &mut state,
            &mut echo,
        ));

        let mut pending = 0;
        let result = loop {
            match call.as_mut().poll(&mut context) {
                Poll::Ready(result) => break result,
                Poll::Pending => pending += 1,
            }
        };
        let error = result.expect_err("external stop should fail");
        assert_eq!(pending, 1);
        assert!(matches!(
            error,
            ExecutionError::Host(ref error)
                if error.failure() == &HostFailure::new("external stop")
        ));
    }

    #[test]
    fn an_assertion_error_owns_its_external_list_after_execution_and_worker_transfer() {
        let (execution, _, _, reject, _, type_id) = external_execution();
        let observations = PayloadObservations::default();
        let mut state = ();
        let mut stores = ();
        let mut echo = RecordedEcho::default();
        let mut call = Box::pin(run_embedded_int(
            &execution,
            reject,
            external_input(type_id, &observations),
            &mut stores,
            &mut state,
            &mut echo,
        ));
        let (result, pending) = std::thread::scope(|scope| {
            scope
                .spawn(|| {
                    let wake = Arc::new(WakeFlag(AtomicBool::new(false)));
                    let waker = Waker::from(Arc::clone(&wake));
                    let mut context = Context::from_waker(&waker);
                    let mut pending = 0;
                    loop {
                        match call.as_mut().poll(&mut context) {
                            Poll::Ready(result) => break (result, pending),
                            Poll::Pending => {
                                assert!(wake.0.swap(false, Ordering::SeqCst));
                                pending += 1;
                            }
                        }
                    }
                })
                .join()
                .expect("completion worker")
        });
        assert_eq!(pending, 1);
        drop(call);
        drop(execution);
        let error = result.expect_err("failed external list assertion");
        assert_eq!(observations.drops.load(Ordering::SeqCst), 0);
        assert_eq!(observations.inspect_calls.load(Ordering::SeqCst), 0);
        let copy = error.clone();
        assert_eq!(observations.inspect_calls.load(Ordering::SeqCst), 0);
        let error = std::thread::spawn(move || {
            assert_eq!(
                error.code().expect("assertion code").to_string(),
                "geam::let_assert"
            );
            assert_eq!(error.to_string(), "let_assert: expected no counter");
            assert!(error.source_code().is_some());
            assert_eq!(
                error
                    .labels()
                    .expect("assertion labels")
                    .map(|label| label.label().expect("label").to_owned())
                    .collect::<Vec<_>>(),
                ["let assert in library.reject_counter", "pattern"]
            );
            assert_eq!(
                error
                    .help()
                    .expect("retained assertion subject")
                    .to_string(),
                "failed value: List(application::library.Counter)([External(Counter(7))])"
            );
            error
        })
        .join()
        .expect("diagnostic worker");
        drop(error);
        assert_eq!(observations.drops.load(Ordering::SeqCst), 0);
        let local = copy.into_local();
        assert_eq!(
            local.help().expect("local assertion subject").to_string(),
            "failed value: List(application::library.Counter)([External(Counter(7))])"
        );
        assert_eq!(observations.drops.load(Ordering::SeqCst), 0);
        drop(local);
        assert_eq!(observations.drops.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn repeated_callbacks_release_one_captured_external_graph_at_completion_or_cancellation() {
        assert_eq!(
            <CallbackProvider as crate::HostProvider<crate::StatelessHostProfile>>::project(&mut ()),
            &mut (),
        );
        let (execution, _, _, _, repeat, type_id) = external_execution();
        let mut state = ();
        let mut stores = ();
        let mut echo = RecordedEcho::default();
        for polls_before_cancel in 0..=2 {
            let observations = PayloadObservations::default();
            let mut call = Box::pin(run_embedded_int(
                &execution,
                repeat,
                external_input(type_id, &observations),
                &mut stores,
                &mut state,
                &mut echo,
            ));
            for _ in 0..polls_before_cancel {
                assert_eq!(
                    call.as_mut().poll(&mut Context::from_waker(Waker::noop())),
                    Poll::Pending
                );
                assert_eq!(observations.drops.load(Ordering::SeqCst), 0);
            }
            drop(call);
            assert_eq!(observations.drops.load(Ordering::SeqCst), 1);
            assert_eq!(
                observations.equal_calls.load(Ordering::SeqCst),
                usize::from(polls_before_cancel == 2)
            );
            assert_eq!(observations.inspect_calls.load(Ordering::SeqCst), 0);
        }

        let observations = PayloadObservations::default();
        let mut call = Box::pin(run_embedded_int(
            &execution,
            repeat,
            external_input(type_id, &observations),
            &mut stores,
            &mut state,
            &mut echo,
        ));
        let mut context = Context::from_waker(Waker::noop());
        assert_eq!(call.as_mut().poll(&mut context), Poll::Pending);
        assert_eq!(observations.equal_calls.load(Ordering::SeqCst), 0);
        assert_eq!(call.as_mut().poll(&mut context), Poll::Pending);
        assert_eq!(observations.equal_calls.load(Ordering::SeqCst), 1);
        assert_eq!(observations.drops.load(Ordering::SeqCst), 0);
        assert_eq!(call.as_mut().poll(&mut context), Poll::Ready(Ok(2.into())));
        drop(call);
        assert_eq!(observations.equal_calls.load(Ordering::SeqCst), 2);
        assert_eq!(observations.inspect_calls.load(Ordering::SeqCst), 0);
        assert_eq!(observations.drops.load(Ordering::SeqCst), 1);
    }
}
