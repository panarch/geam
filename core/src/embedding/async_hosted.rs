use super::binding::{BindingBuilder, BindingParts, Bindings};
use super::input::{AsyncArgumentsInput, InputShape};
use super::value::{EmbeddingValue, OutputValue};
use super::{Arguments, BindingError, CallError, Function, FunctionDeclaration};
use crate::frontend::AsyncHostedTypedProgram;
use crate::host::HostProfile;
use crate::plan::AsyncHostedLibraryModulePlan;
use crate::plan::execution::{
    AsyncHostedExecution, LibraryFunctionEntries, LibraryInputConstructions,
};
use crate::runtime::TransferValues;
use crate::{EchoSink, PlanError};
use std::future::Future;
use std::sync::Arc;

/// Plans a resumable hosted Gleam project before selecting its first function.
pub struct AsyncHostedModuleBuilder<Profile: HostProfile> {
    inner: BindingBuilder<AsyncHostedLibraryModulePlan<Profile>>,
}

/// Collects typed function bindings before resumable hosted sealing.
pub struct AsyncHostedModuleBindings<Profile: HostProfile> {
    inner: Bindings<AsyncHostedLibraryModulePlan<Profile>>,
}

/// One sealed resumable hosted execution shared by all selected functions.
///
/// Each call borrows this owner mutably, allowing one active root call while
/// keeping all pending execution state inside the returned Future.
///
/// Two root calls cannot borrow the same module at once:
///
/// ```compile_fail
/// use geam_core::{EchoSink, StatelessHostProfile};
/// use geam_core::embedding::{AsyncHostedModule, Function};
/// fn overlapping(
///     module: &mut AsyncHostedModule<StatelessHostProfile>,
///     function: &Function<(), ()>,
///     first_state: &mut (),
///     second_state: &mut (),
///     first_echo: &mut (dyn EchoSink + Send),
///     second_echo: &mut (dyn EchoSink + Send),
/// ) {
///     let first = module.call_async(function, (), first_state, first_echo);
///     let second = module.call_async(function, (), second_state, second_echo);
///     drop((first, second));
/// }
/// ```
pub struct AsyncHostedModule<Profile: HostProfile> {
    execution: AsyncHostedExecution<Profile>,
    stores: Profile::ExternalStores,
    entries: LibraryFunctionEntries,
    owner: Arc<()>,
}

enum AsyncCallPreflightError {
    ForeignFunction,
    ForeignValue,
}

impl AsyncCallPreflightError {
    fn into_call_error(self) -> CallError {
        match self {
            Self::ForeignFunction => CallError::ForeignFunction,
            Self::ForeignValue => CallError::ForeignValue,
        }
    }
}

impl<Profile: HostProfile> AsyncHostedModuleBuilder<Profile> {
    /// Plans every source and async host body without requiring a `main` function.
    pub fn new(program: AsyncHostedTypedProgram<Profile>) -> Result<Self, PlanError> {
        let public_functions = program.root_public_functions().cloned().collect();
        crate::planner::plan_async_host_library_program(program).map(|plan| Self {
            inner: BindingBuilder::new(plan, public_functions),
        })
    }

    /// Selects the first function and creates a non-empty binding owner.
    #[allow(private_bounds)]
    pub fn function<ArgumentsType, Return>(
        self,
        declaration: FunctionDeclaration<ArgumentsType, Return>,
    ) -> Result<
        (
            AsyncHostedModuleBindings<Profile>,
            Function<ArgumentsType, Return>,
        ),
        BindingError,
    >
    where
        ArgumentsType: Arguments,
        Return: EmbeddingValue,
    {
        self.inner
            .function(declaration)
            .map(|(inner, function)| (AsyncHostedModuleBindings { inner }, function))
    }
}

impl<Profile: HostProfile> AsyncHostedModuleBindings<Profile> {
    /// Validates and selects another named function for the shared execution.
    #[allow(private_bounds)]
    pub fn function<ArgumentsType, Return>(
        &mut self,
        declaration: FunctionDeclaration<ArgumentsType, Return>,
    ) -> Result<Function<ArgumentsType, Return>, BindingError>
    where
        ArgumentsType: Arguments,
        Return: EmbeddingValue,
    {
        self.inner.function(declaration)
    }

    /// Seals the validated concrete entries into resumable execution tables.
    pub fn seal(self) -> AsyncHostedModule<Profile> {
        let BindingParts {
            plan,
            first,
            remaining,
            owner,
        } = self.inner.into_parts();
        let (execution, entries) = AsyncHostedExecution::from_library_plan(plan, first, remaining);
        AsyncHostedModule {
            execution,
            stores: Profile::ExternalStores::default(),
            entries,
            owner,
        }
    }
}

impl<Profile: HostProfile> AsyncHostedModule<Profile> {
    /// Calls a bound function through caller-owned async execution.
    ///
    /// The returned Future owns this call's runtime and continuation state. No
    /// Gleam, host-state, or Echo effect occurs until the Future is polled.
    /// Dropping it cancels the call and releases the exclusive module borrow;
    /// Geam does not run or select an executor.
    #[allow(private_bounds)]
    pub fn call_async<'call, ArgumentsType, Return, Input, Shape>(
        &'call mut self,
        function: &Function<ArgumentsType, Return, Shape>,
        arguments: Input,
        state: &'call mut Profile::RunState,
        echo: &'call mut (dyn EchoSink + Send),
    ) -> impl Future<Output = Result<Return, CallError>> + Send + 'call
    where
        ArgumentsType: AsyncArgumentsInput<Input>,
        Return: AsyncReturnValue + 'call,
        Shape: InputShape<Input>,
        Profile::RunState: Send,
        Profile::ExternalStores: Send,
    {
        let slot = function.slot;
        let inputs = if !Arc::ptr_eq(&self.owner, &function.owner) {
            Err(AsyncCallPreflightError::ForeignFunction)
        } else if !ArgumentsType::owners_match(&arguments, &self.owner) {
            Err(AsyncCallPreflightError::ForeignValue)
        } else {
            let constructions = Return::input_constructions(&self.entries, slot);
            Ok(ArgumentsType::into_inputs(arguments, constructions))
        };

        let module = self;
        async move {
            match inputs {
                Ok(inputs) => Return::call_async(module, slot, inputs, state, echo)
                    .await
                    .map_err(CallError::Execution),
                Err(error) => Err(error.into_call_error()),
            }
        }
    }
}

trait AsyncReturnValue: OutputValue<TransferValues> {
    fn input_constructions(
        entries: &LibraryFunctionEntries,
        slot: usize,
    ) -> &LibraryInputConstructions;

    fn call_async<Profile: HostProfile>(
        module: &mut AsyncHostedModule<Profile>,
        slot: usize,
        inputs: crate::runtime::TransferInputs,
        state: &mut Profile::RunState,
        echo: &mut (dyn EchoSink + Send),
    ) -> impl Future<Output = Result<Self, crate::ExecutionError>> + Send
    where
        Profile::RunState: Send,
        Profile::ExternalStores: Send;
}

macro_rules! async_scalar_return {
    ($type:ty, $entries:ident, $run:ident) => {
        impl AsyncReturnValue for $type {
            fn input_constructions(
                entries: &LibraryFunctionEntries,
                slot: usize,
            ) -> &LibraryInputConstructions {
                entries.$entries[slot].inputs()
            }

            async fn call_async<Profile: HostProfile>(
                module: &mut AsyncHostedModule<Profile>,
                slot: usize,
                inputs: crate::runtime::TransferInputs,
                state: &mut Profile::RunState,
                echo: &mut (dyn EchoSink + Send),
            ) -> Result<Self, crate::ExecutionError>
            where
                Profile::RunState: Send,
                Profile::ExternalStores: Send,
            {
                let entry = &module.entries.$entries[slot];
                crate::runtime::$run(
                    &module.execution,
                    *entry.function(),
                    inputs,
                    &mut module.stores,
                    state,
                    echo,
                )
                .await
            }
        }
    };
}

async_scalar_return!(super::BigInt, ints, run_resumable_embedded_int);
async_scalar_return!(f64, floats, run_resumable_embedded_float);
async_scalar_return!(super::EcoString, strings, run_resumable_embedded_string);
async_scalar_return!(
    super::BitArrayValue,
    bit_arrays,
    run_resumable_embedded_bit_array
);
async_scalar_return!(char, utf_codepoints, run_resumable_embedded_utf_codepoint);
async_scalar_return!(bool, bools, run_resumable_embedded_bool);
async_scalar_return!((), nils, run_resumable_embedded_nil);

macro_rules! async_tuple_return {
    ($($type:ident),+) => {
        impl<$($type),+> AsyncReturnValue for ($($type,)+)
        where
            $($type: OutputValue<TransferValues>,)+
        {
            fn input_constructions(
                entries: &LibraryFunctionEntries,
                slot: usize,
            ) -> &LibraryInputConstructions {
                entries.tuples[slot].inputs()
            }

            async fn call_async<Profile: HostProfile>(
                module: &mut AsyncHostedModule<Profile>,
                slot: usize,
                inputs: crate::runtime::TransferInputs,
                state: &mut Profile::RunState,
                echo: &mut (dyn EchoSink + Send),
            ) -> Result<Self, crate::ExecutionError>
            where
                Profile::RunState: Send,
                Profile::ExternalStores: Send,
            {
                let entry = &module.entries.tuples[slot];
                crate::runtime::run_resumable_embedded_tuple(
                    &module.execution,
                    *entry.function(),
                    inputs,
                    &mut module.stores,
                    state,
                    echo,
                )
                .await
                .map(|mut output| {
                    <Self as OutputValue<TransferValues>>::take(&mut output, &module.owner)
                })
            }
        }
    };
}

async_tuple_return!(A);
async_tuple_return!(A, B);
async_tuple_return!(A, B, C);
async_tuple_return!(A, B, C, D);
async_tuple_return!(A, B, C, D, E);
async_tuple_return!(A, B, C, D, E, F);
async_tuple_return!(A, B, C, D, E, F, G);

macro_rules! async_custom_return {
    ($container:ty, $($type:ident),+) => {
        impl<$($type),+> AsyncReturnValue for $container
        where
            $($type: OutputValue<TransferValues>,)+
        {
            fn input_constructions(
                entries: &LibraryFunctionEntries,
                slot: usize,
            ) -> &LibraryInputConstructions {
                entries.customs[slot].inputs()
            }

            async fn call_async<Profile: HostProfile>(
                module: &mut AsyncHostedModule<Profile>,
                slot: usize,
                inputs: crate::runtime::TransferInputs,
                state: &mut Profile::RunState,
                echo: &mut (dyn EchoSink + Send),
            ) -> Result<Self, crate::ExecutionError>
            where
                Profile::RunState: Send,
                Profile::ExternalStores: Send,
            {
                let entry = &module.entries.customs[slot];
                crate::runtime::run_resumable_embedded_custom(
                    &module.execution,
                    *entry.function(),
                    inputs,
                    &mut module.stores,
                    state,
                    echo,
                )
                .await
                .map(|mut output| {
                    <Self as OutputValue<TransferValues>>::take(&mut output, &module.owner)
                })
            }
        }
    };
}

async_custom_return!(Result<Success, Failure>, Success, Failure);
async_custom_return!(Option<Value>, Value);

impl<T> AsyncReturnValue for super::AsyncList<T>
where
    T: OutputValue<TransferValues>,
{
    fn input_constructions(
        entries: &LibraryFunctionEntries,
        slot: usize,
    ) -> &LibraryInputConstructions {
        entries.lists[slot].inputs()
    }

    async fn call_async<Profile: HostProfile>(
        module: &mut AsyncHostedModule<Profile>,
        slot: usize,
        inputs: crate::runtime::TransferInputs,
        state: &mut Profile::RunState,
        echo: &mut (dyn EchoSink + Send),
    ) -> Result<Self, crate::ExecutionError>
    where
        Profile::RunState: Send,
        Profile::ExternalStores: Send,
    {
        let entry = &module.entries.lists[slot];
        crate::runtime::run_resumable_embedded_list(
            &module.execution,
            entry.function(),
            inputs,
            &mut module.stores,
            state,
            echo,
        )
        .await
        .map(|mut output| <Self as OutputValue<TransferValues>>::take(&mut output, &module.owner))
    }
}

#[cfg(test)]
mod tests {
    use super::AsyncHostedModuleBuilder;
    use crate::embedding::{
        AsyncList, BigInt, BitArrayValue, CallError, EcoString, FunctionDeclaration,
    };
    use crate::host::{
        AsyncHostCall, AsyncHostCallError, AsyncHostCallable, AsyncHostFuture, AsyncHostModule,
        AsyncHostProviderModule, AsyncHostProviderSet, HostFailure, HostFunctionType, HostProfile,
        HostProvider, HostTypeList, HostTypeListEnd,
    };
    use crate::planner::UnsupportedBitArraySegmentReason;
    use crate::{
        EchoOutput, EchoSink, ExecutionError, ModuleSource, PackageSource, PanicKind, PanicSite,
        PlanError, SourceContext, SourceSpan, compile_typed_async_host_program,
    };
    use std::cell::Cell;
    use std::convert::Infallible;
    use std::future::{Future, Ready, ready};
    use std::pin::Pin;
    use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
    use std::sync::{Arc, Mutex};
    use std::task::{Context, Poll, Wake, Waker};

    struct PendingOnce<Value> {
        polls: Arc<AtomicUsize>,
        pending_returned: bool,
        value: Ready<Value>,
    }

    struct PendingForever {
        polls: Arc<AtomicUsize>,
        drops: Arc<AtomicUsize>,
    }

    impl<Value: Unpin> Future for PendingOnce<Value> {
        type Output = Value;

        fn poll(mut self: Pin<&mut Self>, context: &mut Context<'_>) -> Poll<Self::Output> {
            self.polls.fetch_add(1, Ordering::SeqCst);
            if !self.pending_returned {
                self.pending_returned = true;
                context.waker().wake_by_ref();
                Poll::Pending
            } else {
                Pin::new(&mut self.value).poll(context)
            }
        }
    }

    impl Future for PendingForever {
        type Output = BigInt;

        fn poll(self: Pin<&mut Self>, _context: &mut Context<'_>) -> Poll<Self::Output> {
            self.polls.fetch_add(1, Ordering::SeqCst);
            Poll::Pending
        }
    }

    impl Drop for PendingForever {
        fn drop(&mut self) {
            self.drops.fetch_add(1, Ordering::SeqCst);
        }
    }

    struct WakeFlag(AtomicBool);

    struct StatefulProfile;
    struct StatefulProvider;

    type IntCallbackArguments = HostTypeList<BigInt, HostTypeListEnd>;
    type IntCallback = HostFunctionType<IntCallbackArguments, BigInt>;
    type AllCallbackArguments = HostTypeList<
        BigInt,
        HostTypeList<
            f64,
            HostTypeList<
                EcoString,
                HostTypeList<
                    BitArrayValue,
                    HostTypeList<char, HostTypeList<bool, HostTypeList<(), HostTypeListEnd>>>,
                >,
            >,
        >,
    >;
    type AllIntCallback = HostFunctionType<AllCallbackArguments, BigInt>;
    type AllFloatCallback = HostFunctionType<AllCallbackArguments, f64>;
    type AllStringCallback = HostFunctionType<AllCallbackArguments, EcoString>;
    type AllBitArrayCallback = HostFunctionType<AllCallbackArguments, BitArrayValue>;
    type AllUtfCodepointCallback = HostFunctionType<AllCallbackArguments, char>;
    type AllBoolCallback = HostFunctionType<AllCallbackArguments, bool>;
    type AllNilCallback = HostFunctionType<AllCallbackArguments, ()>;

    struct StatefulRunState {
        capability: BigInt,
        gate: Arc<ManualGate>,
        total: Cell<usize>,
        events: Vec<&'static str>,
    }

    struct ManualGate {
        ready: AtomicBool,
        polls: AtomicUsize,
        drops: AtomicUsize,
        waker: Mutex<Waker>,
    }

    impl Default for ManualGate {
        fn default() -> Self {
            Self {
                ready: AtomicBool::new(false),
                polls: AtomicUsize::new(0),
                drops: AtomicUsize::new(0),
                waker: Mutex::new(Waker::noop().clone()),
            }
        }
    }

    struct GateFuture {
        gate: Arc<ManualGate>,
        value: Ready<BigInt>,
    }

    impl Future for GateFuture {
        type Output = BigInt;

        fn poll(mut self: Pin<&mut Self>, context: &mut Context<'_>) -> Poll<Self::Output> {
            self.gate.polls.fetch_add(1, Ordering::SeqCst);
            if self.gate.ready.load(Ordering::SeqCst) {
                Pin::new(&mut self.value).poll(context)
            } else {
                *self.gate.waker.lock().expect("gate waker lock") = context.waker().clone();
                Poll::Pending
            }
        }
    }

    impl Drop for GateFuture {
        fn drop(&mut self) {
            self.gate.drops.fetch_add(1, Ordering::SeqCst);
        }
    }

    impl ManualGate {
        fn release(&self) {
            self.ready.store(true, Ordering::SeqCst);
            self.waker.lock().expect("gate waker lock").wake_by_ref();
        }
    }

    impl HostProfile for StatefulProfile {
        type RunState = StatefulRunState;
        type ExternalStores = Cell<()>;
    }

    impl HostProvider<StatefulProfile> for StatefulProvider {
        type State = StatefulRunState;

        fn project(state: &mut StatefulRunState) -> &mut Self::State {
            state
        }
    }

    fn stateful_process<'call>(
        mut call: AsyncHostCall<'call, StatefulProfile, StatefulProvider, BigInt>,
        value: BigInt,
    ) -> AsyncHostFuture<'call, BigInt> {
        AsyncHostFuture::new(async move {
            let capability = call
                .with_state(|state| {
                    state.events.push("read");
                    state.capability.clone()
                })
                .await;
            let value = PendingOnce {
                polls: Arc::new(AtomicUsize::new(0)),
                pending_returned: false,
                value: ready(value + capability),
            }
            .await;
            call.with_state(move |state| {
                state.events.push("write");
                state.total.set(state.total.get() + 1);
                value
            })
            .await
        })
    }

    fn cancellable_stateful_process<'call>(
        mut call: AsyncHostCall<'call, StatefulProfile, StatefulProvider, BigInt>,
        value: BigInt,
    ) -> AsyncHostFuture<'call, BigInt> {
        AsyncHostFuture::new(async move {
            let gate = call
                .with_state(|state| {
                    state.events.push("read");
                    Arc::clone(&state.gate)
                })
                .await;
            let value = GateFuture {
                gate,
                value: ready(value),
            }
            .await;
            call.with_state(move |state| {
                state.events.push("write");
                state.total.set(state.total.get() + 1);
                value
            })
            .await
        })
    }

    fn fallible_stateful_process<'call>(
        mut call: AsyncHostCall<'call, StatefulProfile, StatefulProvider, BigInt>,
        value: BigInt,
    ) -> AsyncHostFuture<'call, Result<BigInt, AsyncHostCallError>> {
        AsyncHostFuture::new(async move {
            let capability = call
                .with_state(|state| {
                    state.events.push("read");
                    state.capability.clone()
                })
                .await;
            PendingOnce {
                polls: Arc::new(AtomicUsize::new(0)),
                pending_returned: false,
                value: ready(Err(HostFailure::new(format!(
                    "stopped at {}",
                    value + capability
                ))
                .into())),
            }
            .await
        })
    }

    fn invoke_int_callback<'call>(
        mut call: AsyncHostCall<'call, StatefulProfile, StatefulProvider, BigInt>,
        callback: AsyncHostCallable<'call, StatefulProfile, IntCallbackArguments, BigInt>,
        value: BigInt,
    ) -> AsyncHostFuture<'call, Result<BigInt, AsyncHostCallError>> {
        AsyncHostFuture::new(async move {
            let value = {
                let mut callback_call = Box::pin(call.invoke(callback, (value, ())));
                std::future::poll_fn(|context| {
                    let first = callback_call.as_mut().poll(context);
                    if first.is_pending() {
                        callback_call.as_mut().poll(context)
                    } else {
                        first
                    }
                })
                .await?
            };
            let value = call
                .with_state(move |state| {
                    state.events.push("callback returned");
                    state.total.set(state.total.get() + 1);
                    value + &state.capability
                })
                .await;
            Ok(value)
        })
    }

    fn cancel_queued_int_callback<'call>(
        mut call: AsyncHostCall<'call, StatefulProfile, StatefulProvider, BigInt>,
        callback: AsyncHostCallable<'call, StatefulProfile, IntCallbackArguments, BigInt>,
        value: BigInt,
    ) -> AsyncHostFuture<'call, BigInt> {
        AsyncHostFuture::new(async move {
            {
                let mut callback_call = Box::pin(call.invoke(callback, (value.clone(), ())));
                std::future::poll_fn(|context| {
                    let _ = callback_call.as_mut().poll(context);
                    Poll::Ready(())
                })
                .await;
            }

            PendingOnce {
                polls: Arc::new(AtomicUsize::new(0)),
                pending_returned: false,
                value: ready(value),
            }
            .await
        })
    }

    #[allow(clippy::too_many_arguments)]
    fn invoke_all_callbacks<'call>(
        mut call: AsyncHostCall<'call, StatefulProfile, StatefulProvider, BigInt>,
        int: AsyncHostCallable<'call, StatefulProfile, AllCallbackArguments, BigInt>,
        float: AsyncHostCallable<'call, StatefulProfile, AllCallbackArguments, f64>,
        string: AsyncHostCallable<'call, StatefulProfile, AllCallbackArguments, EcoString>,
        bit_array: AsyncHostCallable<'call, StatefulProfile, AllCallbackArguments, BitArrayValue>,
        codepoint: AsyncHostCallable<'call, StatefulProfile, AllCallbackArguments, char>,
        bool_: AsyncHostCallable<'call, StatefulProfile, AllCallbackArguments, bool>,
        nil: AsyncHostCallable<'call, StatefulProfile, AllCallbackArguments, ()>,
    ) -> AsyncHostFuture<'call, Result<BigInt, AsyncHostCallError>> {
        AsyncHostFuture::new(async move {
            let bits = BitArrayValue::try_from_parts(vec![0b1010_0000], 3)
                .expect("three bits should fit in one byte");
            let arguments = (
                BigInt::from(1),
                (
                    2.5,
                    (
                        EcoString::from("three"),
                        (bits.clone(), ('四', (true, ((), ())))),
                    ),
                ),
            );
            let int = call
                .invoke(int, arguments.clone())
                .await
                .expect("Int callback");
            let float = call
                .invoke(float, arguments.clone())
                .await
                .expect("Float callback");
            let string = call
                .invoke(string, arguments.clone())
                .await
                .expect("String callback");
            let returned_bits = call
                .invoke(bit_array, arguments.clone())
                .await
                .expect("BitArray callback");
            let codepoint = call
                .invoke(codepoint, arguments.clone())
                .await
                .expect("UtfCodepoint callback");
            let bool_ = call
                .invoke(bool_, arguments.clone())
                .await
                .expect("Bool callback");
            call.invoke(nil, arguments).await.expect("Nil callback");

            assert_eq!(int, BigInt::from(1));
            assert_eq!(float, 2.5);
            assert_eq!(string, "three");
            assert_eq!(returned_bits, bits);
            assert_eq!(codepoint, '四');
            assert!(bool_);
            Ok(int)
        })
    }

    fn checked_identity(value: BigInt) -> Result<BigInt, HostFailure> {
        Ok(value)
    }

    fn async_checked_identity(value: BigInt) -> Ready<Result<BigInt, HostFailure>> {
        ready(Ok(value))
    }

    fn scoped_identity<'call, Value: Send + 'static>(
        _call: AsyncHostCall<'call, StatefulProfile, StatefulProvider, Value>,
        value: Value,
    ) -> AsyncHostFuture<'call, Value> {
        AsyncHostFuture::new(ready(value))
    }

    fn stop_immediately(_value: BigInt) -> Result<Infallible, HostFailure> {
        Err(HostFailure::new("immediate stop"))
    }

    fn fail_immediately(_value: BigInt) -> Result<BigInt, HostFailure> {
        Err(HostFailure::new("immediate failure"))
    }

    impl Wake for WakeFlag {
        fn wake(self: Arc<Self>) {
            self.0.store(true, Ordering::SeqCst);
        }

        fn wake_by_ref(self: &Arc<Self>) {
            self.0.store(true, Ordering::SeqCst);
        }
    }

    #[derive(Default)]
    struct SendEcho {
        outputs: usize,
    }

    impl EchoSink for SendEcho {
        fn emit(&mut self, _output: EchoOutput) {
            self.outputs += 1;
        }
    }

    #[derive(Clone, Debug, PartialEq)]
    struct RecordedEcho {
        message: Option<EcoString>,
        value: String,
        path: Option<String>,
        line: Option<usize>,
    }

    #[derive(Clone, Default)]
    struct OrderedEcho {
        outputs: Arc<Mutex<Vec<RecordedEcho>>>,
    }

    impl EchoSink for OrderedEcho {
        fn emit(&mut self, output: EchoOutput) {
            self.outputs
                .lock()
                .expect("echo observation lock")
                .push(RecordedEcho {
                    message: output.message().cloned(),
                    value: output.value().inspect().to_string(),
                    path: output
                        .location()
                        .path()
                        .map(|path| path.as_str().to_owned()),
                    line: output.location().line(),
                });
        }
    }

    fn require_send<Value: Send>(value: Value) -> Value {
        value
    }

    fn poll_woken_to_ready<Output>(future: impl Future<Output = Output>) -> Output {
        let mut future = Box::pin(future);
        let wake = Arc::new(WakeFlag(AtomicBool::new(false)));
        let waker = Waker::from(Arc::clone(&wake));
        let mut context = Context::from_waker(&waker);
        loop {
            wake.0.store(false, Ordering::SeqCst);
            match future.as_mut().poll(&mut context) {
                Poll::Ready(output) => return output,
                Poll::Pending => assert!(
                    wake.0.load(Ordering::SeqCst),
                    "a controlled Pending host must wake before the next poll",
                ),
            }
        }
    }

    fn assert_immediate_stop<Output>(result: Result<Output, CallError>) {
        assert!(matches!(
            result,
            Err(CallError::Execution(ExecutionError::Host(error)))
                if error.package() == "host_support"
                    && error.module() == "host/control"
                    && error.function() == "stop"
                    && error.failure() == &HostFailure::new("immediate stop")
                    && error.location().path().map(|path| path.as_str())
                        == Some("src/library.gleam")
        ));
    }

    #[test]
    fn preserves_library_planning_failures() {
        let hosts = AsyncHostProviderSet::new(Vec::<AsyncHostModule>::new())
            .expect("empty async hosts should be valid");
        let program = compile_typed_async_host_program(
            "application",
            "library",
            [PackageSource::new(
                "application",
                Vec::<EcoString>::new(),
                [ModuleSource::new(
                    "library",
                    "src/library.gleam",
                    "pub fn unsupported() { <<1:native>> }",
                )],
            )],
            hosts,
        )
        .expect("native-endian source should type-check");
        let error = AsyncHostedModuleBuilder::new(program)
            .err()
            .expect("native-endian source should fail planning");

        assert_eq!(
            error,
            PlanError::UnsupportedBitArraySegment {
                reason: UnsupportedBitArraySegmentReason::NativeEndianness,
            },
        );
    }

    #[test]
    fn source_less_async_hosts_cover_each_registration_and_scalar_family() {
        type Scalars = (BigInt, f64, EcoString, BitArrayValue, char, bool, ());
        type Output = (Scalars, Scalars, Scalars, (BigInt, BigInt, BigInt, BigInt));

        let host =
            AsyncHostModule::<StatefulProfile>::new_for_profile("host_support", "host/values")
                .expect("source-less async host module")
                .with_function("immediate_int", std::convert::identity::<BigInt>)
                .expect("immediate function")
                .with_function("immediate_float", std::convert::identity::<f64>)
                .expect("immediate Float function")
                .with_function("immediate_string", std::convert::identity::<EcoString>)
                .expect("immediate String function")
                .with_function(
                    "immediate_bit_array",
                    std::convert::identity::<BitArrayValue>,
                )
                .expect("immediate BitArray function")
                .with_function("immediate_utf_codepoint", std::convert::identity::<char>)
                .expect("immediate UtfCodepoint function")
                .with_function("immediate_bool", std::convert::identity::<bool>)
                .expect("immediate Bool function")
                .with_function("immediate_nil", std::convert::identity::<()>)
                .expect("immediate Nil function")
                .with_fallible_function("checked_int", checked_identity)
                .expect("fallible immediate function")
                .with_fallible_function("stop", stop_immediately)
                .expect("diverging immediate function")
                .with_fallible_function("fail_int", fail_immediately)
                .expect("fallible immediate Int function")
                .with_async_function("int", ready::<BigInt>)
                .expect("async Int function")
                .with_async_function("float", ready::<f64>)
                .expect("async Float function")
                .with_async_function("string", ready::<EcoString>)
                .expect("async String function")
                .with_async_function("bit_array", ready::<BitArrayValue>)
                .expect("async BitArray function")
                .with_async_function("utf_codepoint", ready::<char>)
                .expect("async UtfCodepoint function")
                .with_async_function("bool", ready::<bool>)
                .expect("async Bool function")
                .with_async_function("nil", ready::<()>)
                .expect("async Nil function")
                .with_fallible_async_function("async_checked_int", async_checked_identity)
                .expect("fallible async function")
                .with_scoped_async_function::<StatefulProvider, (BigInt,), BigInt, _>(
                    "scoped_int",
                    stateful_process,
                )
                .expect("scoped async function")
                .with_scoped_async_function::<StatefulProvider, (f64,), f64, _>(
                    "scoped_float",
                    scoped_identity::<f64>,
                )
                .expect("scoped Float function")
                .with_scoped_async_function::<StatefulProvider, (EcoString,), EcoString, _>(
                    "scoped_string",
                    scoped_identity::<EcoString>,
                )
                .expect("scoped String function")
                .with_scoped_async_function::<StatefulProvider, (BitArrayValue,), BitArrayValue, _>(
                    "scoped_bit_array",
                    scoped_identity::<BitArrayValue>,
                )
                .expect("scoped BitArray function")
                .with_scoped_async_function::<StatefulProvider, (char,), char, _>(
                    "scoped_utf_codepoint",
                    scoped_identity::<char>,
                )
                .expect("scoped UtfCodepoint function")
                .with_scoped_async_function::<StatefulProvider, (bool,), bool, _>(
                    "scoped_bool",
                    scoped_identity::<bool>,
                )
                .expect("scoped Bool function")
                .with_scoped_async_function::<StatefulProvider, ((),), (), _>(
                    "scoped_nil",
                    scoped_identity::<()>,
                )
                .expect("scoped Nil function")
                .with_fallible_scoped_async_function::<StatefulProvider, (BigInt,), BigInt, _>(
                    "scoped_stop",
                    fallible_stateful_process,
                )
                .expect("fallible scoped async function");
        assert_eq!(host.package(), "host_support");
        assert_eq!(host.module(), "host/values");
        assert_eq!(host.functions().len(), 26);

        let providers = AsyncHostProviderSet::new([host]).expect("source-less async host set");
        assert_eq!(providers.modules().len(), 1);
        assert!(providers.providers().next().is_none());
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
import host/values

fn apply(function: fn(value) -> result, value: value) -> result {
  function(value)
}

pub fn all(
  int: Int,
  float: Float,
  string: String,
  bit_array: BitArray,
  codepoint: UtfCodepoint,
  bool: Bool,
  nil: Nil,
) {
  #(
    #(
      apply(values.int, int),
      apply(values.float, float),
      apply(values.string, string),
      apply(values.bit_array, bit_array),
      apply(values.utf_codepoint, codepoint),
      apply(values.bool, bool),
      apply(values.nil, nil),
    ),
    #(
      values.immediate_int(int),
      values.immediate_float(float),
      values.immediate_string(string),
      values.immediate_bit_array(bit_array),
      values.immediate_utf_codepoint(codepoint),
      values.immediate_bool(bool),
      values.immediate_nil(nil),
    ),
    #(
      int,
      apply(values.scoped_float, float),
      apply(values.scoped_string, string),
      apply(values.scoped_bit_array, bit_array),
      apply(values.scoped_utf_codepoint, codepoint),
      apply(values.scoped_bool, bool),
      apply(values.scoped_nil, nil),
    ),
    #(
      values.immediate_int(int),
      values.checked_int(int),
      values.async_checked_int(int),
      values.scoped_int(int),
    ),
  )
}

fn stopping_function() -> fn(Int) -> Int { values.stop }

pub fn stop(value: Int) -> Int {
  let stop = stopping_function()
  stop(value)
}

pub fn scoped_stop(value: Int) -> Int {
  values.scoped_stop(value)
}

pub fn fail_int(value: Int) -> Int {
  values.fail_int(value)
}
"#,
                )],
            )],
            providers,
        )
        .expect("source-less async host source");
        let (mut bindings, all) = AsyncHostedModuleBuilder::new(program)
            .expect("source-less async host plan")
            .function(FunctionDeclaration::<Scalars, Output>::new("all"))
            .expect("all binding");
        let stop = bindings
            .function(FunctionDeclaration::<(BigInt,), BigInt>::new("stop"))
            .expect("stop binding");
        let scoped_stop = bindings
            .function(FunctionDeclaration::<(BigInt,), BigInt>::new("scoped_stop"))
            .expect("scoped stop binding");
        let fail_int = bindings
            .function(FunctionDeclaration::<(BigInt,), BigInt>::new("fail_int"))
            .expect("fallible immediate Int binding");
        let mut module = bindings.seal();
        let bits = BitArrayValue::try_from_parts(vec![0b1010_0000], 3)
            .expect("three bits should fit in one byte");
        let inputs = (
            BigInt::from(7),
            2.5,
            EcoString::from("three"),
            bits,
            '四',
            true,
            (),
        );
        let mut state = StatefulRunState {
            capability: BigInt::from(5),
            gate: Arc::new(ManualGate::default()),
            total: Cell::new(0),
            events: Vec::new(),
        };
        let mut echo = SendEcho::default();

        let output = poll_woken_to_ready(require_send(module.call_async(
            &all,
            inputs.clone(),
            &mut state,
            &mut echo,
        )))
        .expect("all source-less async host functions");
        assert_eq!(output.0, inputs);
        assert_eq!(output.1, inputs);
        assert_eq!(output.2, inputs);
        assert_eq!(
            output.3,
            (
                BigInt::from(7),
                BigInt::from(7),
                BigInt::from(7),
                BigInt::from(12),
            ),
        );
        assert_eq!(state.events, ["read", "write"]);
        assert_eq!(state.total.get(), 1);

        let error = poll_woken_to_ready(module.call_async(
            &stop,
            (BigInt::from(1),),
            &mut state,
            &mut echo,
        ))
        .expect_err("immediate diverging host should fail");
        assert!(matches!(
            error,
            CallError::Execution(ExecutionError::Host(ref error))
                if error.failure() == &HostFailure::new("immediate stop")
        ));

        let error = poll_woken_to_ready(module.call_async(
            &scoped_stop,
            (BigInt::from(2),),
            &mut state,
            &mut echo,
        ))
        .expect_err("scoped async host should fail");
        assert!(matches!(
            error,
            CallError::Execution(ExecutionError::Host(ref error))
                if error.failure() == &HostFailure::new("stopped at 7")
        ));
        assert_eq!(state.events, ["read", "write", "read"]);
        assert_eq!(state.total.get(), 1);

        let error = poll_woken_to_ready(module.call_async(
            &fail_int,
            (BigInt::from(3),),
            &mut state,
            &mut echo,
        ))
        .expect_err("fallible immediate Int host should fail");
        assert!(matches!(
            error,
            CallError::Execution(ExecutionError::Host(ref error))
                if error.failure() == &HostFailure::new("immediate failure")
        ));
        assert_eq!(echo.outputs, 0);
    }

    #[test]
    fn specializes_non_returning_async_hosts_across_every_source_return_family() {
        let control = AsyncHostModule::new("host_support", "host/control")
            .expect("source-less async host module")
            .with_fallible_function("stop", stop_immediately)
            .expect("non-returning immediate function");
        let providers = AsyncHostProviderSet::new([control]).expect("source-less async host set");
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

pub type Never

fn stopping_function() -> fn(Int) -> value { control.stop }

pub fn int(value: Int) -> Int {
  let stop: fn(Int) -> Int = stopping_function()
  stop(value)
}

pub fn float(value: Int) -> Float {
  let stop: fn(Int) -> Float = stopping_function()
  stop(value)
}

pub fn string(value: Int) -> String {
  let stop: fn(Int) -> String = stopping_function()
  stop(value)
}

pub fn bit_array(value: Int) -> BitArray {
  let stop: fn(Int) -> BitArray = stopping_function()
  stop(value)
}

pub fn codepoint(value: Int) -> UtfCodepoint {
  let stop: fn(Int) -> UtfCodepoint = stopping_function()
  stop(value)
}

pub fn bool(value: Int) -> Bool {
  let stop: fn(Int) -> Bool = stopping_function()
  stop(value)
}

pub fn nil(value: Int) -> Nil {
  let stop: fn(Int) -> Nil = stopping_function()
  stop(value)
}

pub fn custom(value: Int) -> Result(Int, String) {
  let stop: fn(Int) -> Result(Int, String) = stopping_function()
  stop(value)
}

pub fn tuple(value: Int) -> #(Int, String) {
  let stop: fn(Int) -> #(Int, String) = stopping_function()
  stop(value)
}

pub fn list(value: Int) -> List(Int) {
  let stop: fn(Int) -> List(Int) = stopping_function()
  stop(value)
}

fn function(value: Int) -> fn(Int) -> Int {
  let stop: fn(Int) -> fn(Int) -> Int = stopping_function()
  stop(value)
}

pub fn through_function(value: Int) -> Int {
  function(value)(value)
}

fn never_function() -> fn(Int) -> Never { control.stop }

fn never(value: Int) -> Never {
  let stop = never_function()
  stop(value)
}
fn delegate_never(value: Int) -> Never { never(value) }
fn consume_never(_value: Never) -> Int { 0 }

pub fn through_never(value: Int) -> Int {
  consume_never(delegate_never(value))
}

fn source_stop(_value: Int) -> Never { panic as "source stopped" }
fn source_never_function() -> fn(Int) -> Never { source_stop }

pub fn through_source_never(value: Int) -> Int {
  let stop = source_never_function()
  consume_never(stop(value))
}
"#,
                )],
            )],
            providers,
        )
        .expect("non-returning return-family source");
        let (mut bindings, int) = AsyncHostedModuleBuilder::new(program)
            .expect("non-returning return-family plan")
            .function(FunctionDeclaration::<(BigInt,), BigInt>::new("int"))
            .expect("Int binding");
        let float = bindings
            .function(FunctionDeclaration::<(BigInt,), f64>::new("float"))
            .expect("Float binding");
        let string = bindings
            .function(FunctionDeclaration::<(BigInt,), EcoString>::new("string"))
            .expect("String binding");
        let bit_array = bindings
            .function(FunctionDeclaration::<(BigInt,), BitArrayValue>::new(
                "bit_array",
            ))
            .expect("BitArray binding");
        let codepoint = bindings
            .function(FunctionDeclaration::<(BigInt,), char>::new("codepoint"))
            .expect("UtfCodepoint binding");
        let bool_ = bindings
            .function(FunctionDeclaration::<(BigInt,), bool>::new("bool"))
            .expect("Bool binding");
        let nil = bindings
            .function(FunctionDeclaration::<(BigInt,), ()>::new("nil"))
            .expect("Nil binding");
        let custom = bindings
            .function(FunctionDeclaration::<(BigInt,), Result<BigInt, EcoString>>::new("custom"))
            .expect("custom binding");
        let tuple = bindings
            .function(FunctionDeclaration::<(BigInt,), (BigInt, EcoString)>::new(
                "tuple",
            ))
            .expect("tuple binding");
        let list = bindings
            .function(FunctionDeclaration::<(BigInt,), AsyncList<BigInt>>::new(
                "list",
            ))
            .expect("List binding");
        let through_function = bindings
            .function(FunctionDeclaration::<(BigInt,), BigInt>::new(
                "through_function",
            ))
            .expect("function-return binding");
        let through_never = bindings
            .function(FunctionDeclaration::<(BigInt,), BigInt>::new(
                "through_never",
            ))
            .expect("Never-return binding");
        let through_source_never = bindings
            .function(FunctionDeclaration::<(BigInt,), BigInt>::new(
                "through_source_never",
            ))
            .expect("source Never-return binding");
        let mut module = bindings.seal();
        let mut state = ();
        let mut echo = SendEcho::default();

        macro_rules! assert_stop {
            ($function:expr, $value:expr) => {
                assert_immediate_stop(poll_woken_to_ready(module.call_async(
                    $function,
                    (BigInt::from($value),),
                    &mut state,
                    &mut echo,
                )))
            };
        }

        assert_stop!(&int, 1);
        assert_stop!(&float, 2);
        assert_stop!(&string, 3);
        assert_stop!(&bit_array, 4);
        assert_stop!(&codepoint, 5);
        assert_stop!(&bool_, 6);
        assert_stop!(&nil, 7);
        assert_stop!(&custom, 8);
        assert_stop!(&tuple, 9);
        assert_stop!(&list, 10);
        assert_stop!(&through_function, 11);
        assert_stop!(&through_never, 12);
        assert_eq!(
            poll_woken_to_ready(module.call_async(
                &through_source_never,
                (BigInt::from(13),),
                &mut state,
                &mut echo,
            ))
            .map_err(|error| error.to_string()),
            Err("panic: source stopped".to_owned()),
        );
        assert_eq!(echo.outputs, 0);
    }

    #[test]
    fn callbacks_transfer_every_scalar_argument_and_return_family() {
        let host = AsyncHostProviderModule::<StatefulProfile>::new("application", "library")
            .expect("async host module")
            .with_fallible_scoped_async_function::<StatefulProvider, (
                AllIntCallback,
                AllFloatCallback,
                AllStringCallback,
                AllBitArrayCallback,
                AllUtfCodepointCallback,
                AllBoolCallback,
                AllNilCallback,
            ), BigInt, _>("invoke_all", invoke_all_callbacks)
            .expect("all-family callback host function");
        let providers = AsyncHostProviderSet::with_providers(
            Vec::<AsyncHostModule<StatefulProfile>>::new(),
            [host],
        )
        .expect("async host set");
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
@external(erlang, "native", "invoke_all")
fn invoke_all(
  int: fn(Int, Float, String, BitArray, UtfCodepoint, Bool, Nil) -> Int,
  float: fn(Int, Float, String, BitArray, UtfCodepoint, Bool, Nil) -> Float,
  string: fn(Int, Float, String, BitArray, UtfCodepoint, Bool, Nil) -> String,
  bit_array: fn(Int, Float, String, BitArray, UtfCodepoint, Bool, Nil) -> BitArray,
  codepoint: fn(Int, Float, String, BitArray, UtfCodepoint, Bool, Nil) -> UtfCodepoint,
  bool: fn(Int, Float, String, BitArray, UtfCodepoint, Bool, Nil) -> Bool,
  nil: fn(Int, Float, String, BitArray, UtfCodepoint, Bool, Nil) -> Nil,
) -> Int

fn int_value(int, _, _, _, _, _, _) { int }
fn float_value(_, float, _, _, _, _, _) { float }
fn string_value(_, _, string, _, _, _, _) { string }
fn bit_array_value(_, _, _, bit_array, _, _, _) { bit_array }
fn codepoint_value(_, _, _, _, codepoint, _, _) { codepoint }
fn bool_value(_, _, _, _, _, bool, _) { bool }
fn nil_value(_, _, _, _, _, _, nil) { nil }

pub fn run() {
  invoke_all(
    int_value,
    float_value,
    string_value,
    bit_array_value,
    codepoint_value,
    bool_value,
    nil_value,
  )
}
"#,
                )],
            )],
            providers,
        )
        .expect("all-family callback source");
        let (bindings, run) = AsyncHostedModuleBuilder::new(program)
            .expect("all-family callback plan")
            .function(FunctionDeclaration::<(), BigInt>::new("run"))
            .expect("all-family callback binding");
        let mut module = bindings.seal();
        let mut state = StatefulRunState {
            capability: BigInt::from(0),
            gate: Arc::new(ManualGate::default()),
            total: Cell::new(0),
            events: Vec::new(),
        };
        let mut echo = SendEcho::default();

        assert_eq!(
            poll_woken_to_ready(require_send(module.call_async(
                &run,
                (),
                &mut state,
                &mut echo,
            ))),
            Ok(BigInt::from(1)),
        );
        assert!(state.events.is_empty());
        assert_eq!(state.total.get(), 0);
        assert_eq!(echo.outputs, 0);
    }

    #[test]
    fn immediate_and_pending_hosts_share_one_exact_resumable_gleam_call() {
        let immediate_calls = Arc::new(AtomicUsize::new(0));
        let creations = Arc::new(AtomicUsize::new(0));
        let polls = Arc::new(AtomicUsize::new(0));
        let host = AsyncHostProviderModule::new("application", "library")
            .expect("async host module")
            .with_function("double", {
                let immediate_calls = Arc::clone(&immediate_calls);
                move |value: BigInt| {
                    immediate_calls.fetch_add(1, Ordering::SeqCst);
                    value * 2
                }
            })
            .expect("immediate host function")
            .with_async_function("add_one", {
                let creations = Arc::clone(&creations);
                let polls = Arc::clone(&polls);
                move |value: BigInt| {
                    creations.fetch_add(1, Ordering::SeqCst);
                    PendingOnce {
                        polls: Arc::clone(&polls),
                        pending_returned: false,
                        value: ready(value + 1),
                    }
                }
            })
            .expect("async host function");
        let providers = AsyncHostProviderSet::with_providers(Vec::<AsyncHostModule>::new(), [host])
            .expect("async host set");
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
@external(erlang, "native", "add_one")
fn add_one(value: Int) -> Int

@external(erlang, "native", "double")
fn double(value: Int) -> Int

fn make_adder(offset: Int) -> fn(Int) -> Int {
  fn(value) { value + offset }
}

pub fn calculate(value: Int) {
  echo value as "input"
  let resumed = add_one(double(value))
  make_adder(3)(double(resumed))
}
"#,
                )],
            )],
            providers,
        )
        .expect("async hosted source");
        let builder = AsyncHostedModuleBuilder::new(program).expect("async hosted plan");
        let (bindings, function) = builder
            .function(FunctionDeclaration::<(BigInt,), BigInt>::new("calculate"))
            .expect("calculate binding");
        let mut module = bindings.seal();
        let mut state = ();
        let mut echo = SendEcho::default();
        let mut call =
            Box::pin(module.call_async(&function, (BigInt::from(10),), &mut state, &mut echo));
        let wake = Arc::new(WakeFlag(AtomicBool::new(false)));
        let waker = Waker::from(Arc::clone(&wake));
        let mut context = Context::from_waker(&waker);

        assert_eq!(call.as_mut().poll(&mut context), Poll::Pending);
        assert!(wake.0.load(Ordering::SeqCst));
        assert_eq!(creations.load(Ordering::SeqCst), 1);
        assert_eq!(polls.load(Ordering::SeqCst), 1);
        assert_eq!(immediate_calls.load(Ordering::SeqCst), 1);

        assert_eq!(
            call.as_mut().poll(&mut context),
            Poll::Ready(Ok(BigInt::from(45))),
        );
        assert_eq!(creations.load(Ordering::SeqCst), 1);
        assert_eq!(polls.load(Ordering::SeqCst), 2);
        assert_eq!(immediate_calls.load(Ordering::SeqCst), 2);
        drop(call);
        assert_eq!(echo.outputs, 1);
    }

    #[test]
    fn nested_async_callback_resumes_rust_and_gleam_once_through_the_root_driver() {
        let nested_polls = Arc::new(AtomicUsize::new(0));
        let host = AsyncHostProviderModule::<StatefulProfile>::new("application", "library")
            .expect("async host module")
            .with_async_function("suspend", {
                let nested_polls = Arc::clone(&nested_polls);
                move |value: BigInt| PendingOnce {
                    polls: Arc::clone(&nested_polls),
                    pending_returned: false,
                    value: ready(value + 1),
                }
            })
            .expect("nested async function")
            .with_fallible_scoped_async_function::<
                StatefulProvider,
                (IntCallback, BigInt),
                BigInt,
                _,
            >("invoke", invoke_int_callback)
            .expect("callback host function");
        let providers = AsyncHostProviderSet::with_providers(
            Vec::<AsyncHostModule<StatefulProfile>>::new(),
            [host],
        )
        .expect("async host set");
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

fn callback(offset: Int) -> fn(Int) -> Int {
  fn(value) { suspend(value) * 2 + offset }
}

pub fn run(value: Int) -> Int {
  invoke(callback(3), value) + 4
}
"#,
                )],
            )],
            providers,
        )
        .expect("nested callback source");
        let (bindings, function) = AsyncHostedModuleBuilder::new(program)
            .expect("nested callback plan")
            .function(FunctionDeclaration::<(BigInt,), BigInt>::new("run"))
            .expect("nested callback binding");
        let mut module = bindings.seal();
        let mut state = StatefulRunState {
            capability: BigInt::from(10),
            gate: Arc::new(ManualGate::default()),
            total: Cell::new(0),
            events: Vec::new(),
        };
        let mut echo = SendEcho::default();
        let mut call = Box::pin(require_send(module.call_async(
            &function,
            (BigInt::from(5),),
            &mut state,
            &mut echo,
        )));
        let wake = Arc::new(WakeFlag(AtomicBool::new(false)));
        let waker = Waker::from(Arc::clone(&wake));
        let mut context = Context::from_waker(&waker);

        assert_eq!(call.as_mut().poll(&mut context), Poll::Pending);
        assert!(wake.0.swap(false, Ordering::SeqCst));
        assert_eq!(nested_polls.load(Ordering::SeqCst), 1);

        assert_eq!(
            call.as_mut().poll(&mut context),
            Poll::Ready(Ok(BigInt::from(29))),
        );
        drop(call);
        assert_eq!(nested_polls.load(Ordering::SeqCst), 2);
        assert_eq!(state.events, ["callback returned"]);
        assert_eq!(state.total.get(), 1);
        assert_eq!(echo.outputs, 0);
    }

    #[test]
    fn nested_async_callback_preserves_echo_order_across_both_suspension_levels() {
        let host = AsyncHostProviderModule::<StatefulProfile>::new("application", "library")
            .expect("async host module")
            .with_async_function("suspend", |value: BigInt| PendingOnce {
                polls: Arc::new(AtomicUsize::new(0)),
                pending_returned: false,
                value: ready(value + 1),
            })
            .expect("nested async function")
            .with_fallible_scoped_async_function::<
                StatefulProvider,
                (IntCallback, BigInt),
                BigInt,
                _,
            >("invoke", invoke_int_callback)
            .expect("callback host function");
        let providers = AsyncHostProviderSet::with_providers(
            Vec::<AsyncHostModule<StatefulProfile>>::new(),
            [host],
        )
        .expect("async host set");
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

fn callback(value: Int) -> Int {
  echo value as "callback before"
  let resumed = suspend(value)
  echo resumed as "callback after"
  resumed
}

pub fn run(value: Int) -> Int {
  echo value as "outer before"
  let returned = invoke(callback, value)
  echo returned as "outer after"
  returned
}
"#,
                )],
            )],
            providers,
        )
        .expect("nested callback source");
        let (bindings, function) = AsyncHostedModuleBuilder::new(program)
            .expect("nested callback plan")
            .function(FunctionDeclaration::<(BigInt,), BigInt>::new("run"))
            .expect("nested callback binding");
        let mut module = bindings.seal();
        let mut state = StatefulRunState {
            capability: BigInt::from(10),
            gate: Arc::new(ManualGate::default()),
            total: Cell::new(0),
            events: Vec::new(),
        };
        let mut echo = OrderedEcho::default();
        let observed = Arc::clone(&echo.outputs);
        let mut call =
            Box::pin(module.call_async(&function, (BigInt::from(5),), &mut state, &mut echo));
        let wake = Arc::new(WakeFlag(AtomicBool::new(false)));
        let waker = Waker::from(Arc::clone(&wake));
        let mut context = Context::from_waker(&waker);

        assert_eq!(call.as_mut().poll(&mut context), Poll::Pending);
        assert_eq!(
            *observed.lock().expect("echo observation lock"),
            vec![
                RecordedEcho {
                    message: Some("outer before".into()),
                    value: "5".into(),
                    path: Some("src/library.gleam".into()),
                    line: Some(16),
                },
                RecordedEcho {
                    message: Some("callback before".into()),
                    value: "5".into(),
                    path: Some("src/library.gleam".into()),
                    line: Some(9),
                },
            ],
        );

        assert_eq!(
            call.as_mut().poll(&mut context),
            Poll::Ready(Ok(BigInt::from(16))),
        );
        drop(call);
        assert_eq!(
            *observed.lock().expect("echo observation lock"),
            vec![
                RecordedEcho {
                    message: Some("outer before".into()),
                    value: "5".into(),
                    path: Some("src/library.gleam".into()),
                    line: Some(16),
                },
                RecordedEcho {
                    message: Some("callback before".into()),
                    value: "5".into(),
                    path: Some("src/library.gleam".into()),
                    line: Some(9),
                },
                RecordedEcho {
                    message: Some("callback after".into()),
                    value: "6".into(),
                    path: Some("src/library.gleam".into()),
                    line: Some(11),
                },
                RecordedEcho {
                    message: Some("outer after".into()),
                    value: "16".into(),
                    path: Some("src/library.gleam".into()),
                    line: Some(18),
                },
            ],
        );
        assert_eq!(state.events, ["callback returned"]);
        assert_eq!(state.total.get(), 1);
    }

    #[test]
    fn nested_callback_source_panic_keeps_its_source_identity() {
        let nested_polls = Arc::new(AtomicUsize::new(0));
        let host = AsyncHostProviderModule::<StatefulProfile>::new("application", "library")
            .expect("async host module")
            .with_async_function("suspend", {
                let nested_polls = Arc::clone(&nested_polls);
                move |value: BigInt| PendingOnce {
                    polls: Arc::clone(&nested_polls),
                    pending_returned: false,
                    value: ready(value),
                }
            })
            .expect("nested async function")
            .with_fallible_scoped_async_function::<
                StatefulProvider,
                (IntCallback, BigInt),
                BigInt,
                _,
            >("invoke", invoke_int_callback)
            .expect("callback host function");
        let providers = AsyncHostProviderSet::with_providers(
            Vec::<AsyncHostModule<StatefulProfile>>::new(),
            [host],
        )
        .expect("async host set");
        let source = r#"
@external(erlang, "native", "suspend")
fn suspend(value: Int) -> Int

@external(erlang, "native", "invoke")
fn invoke(callback: fn(Int) -> Int, value: Int) -> Int

fn callback(value: Int) -> Int {
  let _ = suspend(value)
  panic as "callback stopped"
}

pub fn run(value: Int) -> Int {
  invoke(callback, value)
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
            providers,
        )
        .expect("nested panic source");
        let (bindings, function) = AsyncHostedModuleBuilder::new(program)
            .expect("nested panic plan")
            .function(FunctionDeclaration::<(BigInt,), BigInt>::new("run"))
            .expect("nested panic binding");
        let mut module = bindings.seal();
        let mut state = StatefulRunState {
            capability: BigInt::from(10),
            gate: Arc::new(ManualGate::default()),
            total: Cell::new(0),
            events: Vec::new(),
        };
        let mut echo = SendEcho::default();
        let mut call =
            Box::pin(module.call_async(&function, (BigInt::from(5),), &mut state, &mut echo));
        let wake = Arc::new(WakeFlag(AtomicBool::new(false)));
        let waker = Waker::from(Arc::clone(&wake));
        let mut context = Context::from_waker(&waker);

        assert_eq!(call.as_mut().poll(&mut context), Poll::Pending);
        assert!(wake.0.swap(false, Ordering::SeqCst));
        let expression = "panic as \"callback stopped\"";
        let start = source.find(expression).expect("nested panic expression");
        assert_eq!(
            call.as_mut().poll(&mut context),
            Poll::Ready(Err(CallError::Execution(ExecutionError::source_panic(
                Some(&SourceContext::new("src/library.gleam", source)),
                PanicKind::Panic,
                Some("callback stopped".into()),
                PanicSite::new(
                    "library".into(),
                    "callback".into(),
                    SourceSpan::new(start, start + expression.len()),
                ),
            )))),
        );
        drop(call);
        assert_eq!(nested_polls.load(Ordering::SeqCst), 2);
        assert!(state.events.is_empty());
        assert_eq!(state.total.get(), 0);
        assert_eq!(echo.outputs, 0);
    }

    #[test]
    fn nested_callback_host_failure_keeps_the_nested_provider_and_call_site() {
        let nested_polls = Arc::new(AtomicUsize::new(0));
        let host = AsyncHostProviderModule::<StatefulProfile>::new("application", "library")
            .expect("async host module")
            .with_fallible_async_function("fail", {
                let nested_polls = Arc::clone(&nested_polls);
                move |_value: BigInt| PendingOnce {
                    polls: Arc::clone(&nested_polls),
                    pending_returned: false,
                    value: ready(Err::<BigInt, HostFailure>(HostFailure::new("nested stopped"))),
                }
            })
            .expect("nested fallible function")
            .with_fallible_scoped_async_function::<
                StatefulProvider,
                (IntCallback, BigInt),
                BigInt,
                _,
            >("invoke", invoke_int_callback)
            .expect("callback host function");
        let providers = AsyncHostProviderSet::with_providers(
            Vec::<AsyncHostModule<StatefulProfile>>::new(),
            [host],
        )
        .expect("async host set");
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
@external(erlang, "native", "fail")
fn fail(value: Int) -> Int

@external(erlang, "native", "invoke")
fn invoke(callback: fn(Int) -> Int, value: Int) -> Int

fn callback(value: Int) -> Int {
  fail(value)
}

pub fn run(value: Int) -> Int {
  invoke(callback, value)
}
"#,
                )],
            )],
            providers,
        )
        .expect("nested failure source");
        let (bindings, function) = AsyncHostedModuleBuilder::new(program)
            .expect("nested failure plan")
            .function(FunctionDeclaration::<(BigInt,), BigInt>::new("run"))
            .expect("nested failure binding");
        let mut module = bindings.seal();
        let mut state = StatefulRunState {
            capability: BigInt::from(10),
            gate: Arc::new(ManualGate::default()),
            total: Cell::new(0),
            events: Vec::new(),
        };
        let mut echo = SendEcho::default();
        let error = poll_woken_to_ready(module.call_async(
            &function,
            (BigInt::from(5),),
            &mut state,
            &mut echo,
        ))
        .expect_err("nested host failure");

        assert!(matches!(
            &error,
            CallError::Execution(ExecutionError::Host(error))
                if (
                    error.package().as_str(),
                    error.module().as_str(),
                    error.function().as_str(),
                    error.failure(),
                    error.location().site().map(|site| (
                        site.module().as_str(),
                        site.function().as_str(),
                    )),
                    error.location().path().map(|path| path.as_str()),
                    error.location().line(),
                ) == (
                    "application",
                    "library",
                    "fail",
                    &HostFailure::new("nested stopped"),
                    Some(("library", "callback")),
                    Some("src/library.gleam"),
                    Some(9),
                )
        ));
        assert_eq!(nested_polls.load(Ordering::SeqCst), 2);
        assert!(state.events.is_empty());
        assert_eq!(state.total.get(), 0);
        assert_eq!(echo.outputs, 0);
    }

    #[test]
    fn host_callback_failure_keeps_the_invoking_host_as_its_origin() {
        let nested_polls = Arc::new(AtomicUsize::new(0));
        let host = AsyncHostProviderModule::<StatefulProfile>::new("application", "library")
            .expect("async host module")
            .with_fallible_async_function("fail", {
                let nested_polls = Arc::clone(&nested_polls);
                move |_value: BigInt| PendingOnce {
                    polls: Arc::clone(&nested_polls),
                    pending_returned: false,
                    value: ready(Err::<BigInt, HostFailure>(HostFailure::new("nested stopped"))),
                }
            })
            .expect("nested fallible function")
            .with_fallible_scoped_async_function::<
                StatefulProvider,
                (IntCallback, BigInt),
                BigInt,
                _,
            >("invoke", invoke_int_callback)
            .expect("callback host function");
        let providers = AsyncHostProviderSet::with_providers(
            Vec::<AsyncHostModule<StatefulProfile>>::new(),
            [host],
        )
        .expect("async host set");
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
@external(erlang, "native", "fail")
fn fail(value: Int) -> Int

@external(erlang, "native", "invoke")
fn invoke(callback: fn(Int) -> Int, value: Int) -> Int

pub fn run(value: Int) -> Int {
  invoke(fail, value)
}
"#,
                )],
            )],
            providers,
        )
        .expect("host callback failure source");
        let (bindings, function) = AsyncHostedModuleBuilder::new(program)
            .expect("host callback failure plan")
            .function(FunctionDeclaration::<(BigInt,), BigInt>::new("run"))
            .expect("host callback failure binding");
        let mut module = bindings.seal();
        let mut state = StatefulRunState {
            capability: BigInt::from(10),
            gate: Arc::new(ManualGate::default()),
            total: Cell::new(0),
            events: Vec::new(),
        };
        let mut echo = SendEcho::default();
        let error = poll_woken_to_ready(module.call_async(
            &function,
            (BigInt::from(5),),
            &mut state,
            &mut echo,
        ))
        .expect_err("nested host function should fail");

        assert!(matches!(
            &error,
            CallError::Execution(ExecutionError::Host(error))
                if (
                    error.package().as_str(),
                    error.module().as_str(),
                    error.function().as_str(),
                    error.failure(),
                    error.location().site(),
                    error.location().caller().map(|caller| (
                        caller.package().as_str(),
                        caller.module().as_str(),
                        caller.function().as_str(),
                    )),
                ) == (
                    "application",
                    "library",
                    "fail",
                    &HostFailure::new("nested stopped"),
                    None,
                    Some(("application", "library", "invoke")),
                )
        ));
        assert_eq!(nested_polls.load(Ordering::SeqCst), 2);
        assert!(state.events.is_empty());
        assert_eq!(state.total.get(), 0);
        assert_eq!(echo.outputs, 0);
    }

    #[test]
    fn dropping_a_nested_callback_call_cancels_the_leaf_without_post_processing() {
        let gate = Arc::new(ManualGate::default());
        let host = AsyncHostProviderModule::<StatefulProfile>::new("application", "library")
            .expect("async host module")
            .with_async_function("wait", {
                let gate = Arc::clone(&gate);
                move |value: BigInt| GateFuture {
                    gate: Arc::clone(&gate),
                    value: ready(value),
                }
            })
            .expect("nested waiting function")
            .with_fallible_scoped_async_function::<
                StatefulProvider,
                (IntCallback, BigInt),
                BigInt,
                _,
            >("invoke", invoke_int_callback)
            .expect("callback host function");
        let providers = AsyncHostProviderSet::with_providers(
            Vec::<AsyncHostModule<StatefulProfile>>::new(),
            [host],
        )
        .expect("async host set");
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
@external(erlang, "native", "wait")
fn wait(value: Int) -> Int

@external(erlang, "native", "invoke")
fn invoke(callback: fn(Int) -> Int, value: Int) -> Int

fn callback(value: Int) -> Int {
  wait(value)
}

pub fn run(value: Int) -> Int {
  invoke(callback, value)
}
"#,
                )],
            )],
            providers,
        )
        .expect("nested cancellation source");
        let (bindings, function) = AsyncHostedModuleBuilder::new(program)
            .expect("nested cancellation plan")
            .function(FunctionDeclaration::<(BigInt,), BigInt>::new("run"))
            .expect("nested cancellation binding");
        let mut module = bindings.seal();
        let mut state = StatefulRunState {
            capability: BigInt::from(10),
            gate: Arc::new(ManualGate::default()),
            total: Cell::new(0),
            events: Vec::new(),
        };
        let mut echo = SendEcho::default();
        let mut call =
            Box::pin(module.call_async(&function, (BigInt::from(5),), &mut state, &mut echo));
        let wake = Arc::new(WakeFlag(AtomicBool::new(false)));
        let waker = Waker::from(Arc::clone(&wake));
        let mut context = Context::from_waker(&waker);

        assert_eq!(call.as_mut().poll(&mut context), Poll::Pending);
        assert_eq!(gate.polls.load(Ordering::SeqCst), 1);
        assert_eq!(call.as_mut().poll(&mut context), Poll::Pending);
        assert_eq!(gate.polls.load(Ordering::SeqCst), 2);
        drop(call);

        assert_eq!(gate.drops.load(Ordering::SeqCst), 1);
        assert!(state.events.is_empty());
        assert_eq!(state.total.get(), 0);
        gate.release();
        assert!(state.events.is_empty());
        assert_eq!(state.total.get(), 0);

        let value = poll_woken_to_ready(module.call_async(
            &function,
            (BigInt::from(5),),
            &mut state,
            &mut echo,
        ))
        .expect("the module should remain usable after cancellation");
        assert_eq!(value, BigInt::from(15));
        assert_eq!(gate.polls.load(Ordering::SeqCst), 3);
        assert_eq!(gate.drops.load(Ordering::SeqCst), 2);
        assert_eq!(state.events, ["callback returned"]);
        assert_eq!(state.total.get(), 1);
        assert_eq!(echo.outputs, 0);
    }

    #[test]
    fn cancellation_after_callback_completion_discards_only_remaining_host_work() {
        fn invoke_then_wait<'call>(
            mut call: AsyncHostCall<'call, StatefulProfile, StatefulProvider, BigInt>,
            callback: AsyncHostCallable<'call, StatefulProfile, IntCallbackArguments, BigInt>,
            value: BigInt,
        ) -> AsyncHostFuture<'call, Result<BigInt, AsyncHostCallError>> {
            AsyncHostFuture::new(async move {
                let value = call
                    .invoke(callback, (value, ()))
                    .await
                    .expect("the callback completes before cancellation");
                let gate = call
                    .with_state(|state| {
                        state.events.push("callback returned");
                        Arc::clone(&state.gate)
                    })
                    .await;
                let value = GateFuture {
                    gate,
                    value: ready(value),
                }
                .await;
                call.with_state(|state| {
                    state.events.push("postprocessed");
                    state.total.set(state.total.get() + 1);
                })
                .await;
                Ok(value)
            })
        }

        let host = AsyncHostProviderModule::<StatefulProfile>::new("application", "library")
            .expect("async host module")
            .with_fallible_scoped_async_function::<StatefulProvider, (IntCallback, BigInt), BigInt, _>(
                "invoke", invoke_then_wait,
            )
            .expect("callback and postprocessing host");
        let hosts = AsyncHostProviderSet::with_providers(
            Vec::<AsyncHostModule<StatefulProfile>>::new(),
            [host],
        )
        .expect("async host set");
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
@external(erlang, "native", "invoke")
fn invoke(callback: fn(Int) -> Int, value: Int) -> Int

fn callback(value: Int) -> Int {
  echo value as "callback"
  value + 1
}

pub fn run(value: Int) -> Int {
  let result = invoke(callback, value)
  echo result as "outer"
  result
}
"#,
                )],
            )],
            hosts,
        )
        .expect("post-callback cancellation source");
        let (bindings, function) = AsyncHostedModuleBuilder::new(program)
            .expect("post-callback cancellation plan")
            .function(FunctionDeclaration::<(BigInt,), BigInt>::new("run"))
            .expect("run binding");
        let mut module = bindings.seal();
        let gate = Arc::new(ManualGate::default());
        let mut state = StatefulRunState {
            capability: 0.into(),
            gate: Arc::clone(&gate),
            total: Cell::new(0),
            events: Vec::new(),
        };
        let mut echo = OrderedEcho::default();
        let observed = Arc::clone(&echo.outputs);
        let callback_echo = RecordedEcho {
            message: Some("callback".into()),
            value: "5".into(),
            path: Some("src/library.gleam".into()),
            line: Some(6),
        };
        let mut call = Box::pin(module.call_async(&function, (5.into(),), &mut state, &mut echo));
        let mut context = Context::from_waker(Waker::noop());
        assert_eq!(call.as_mut().poll(&mut context), Poll::Pending);
        assert_eq!(call.as_mut().poll(&mut context), Poll::Pending);
        assert_eq!(
            observed.lock().expect("echo observation lock").as_slice(),
            std::slice::from_ref(&callback_echo)
        );
        assert_eq!(gate.polls.load(Ordering::SeqCst), 2);
        drop(call);
        assert_eq!(gate.drops.load(Ordering::SeqCst), 1);
        assert_eq!(state.events, ["callback returned"]);
        assert_eq!(state.total.get(), 0);

        gate.release();
        assert_eq!(state.events, ["callback returned"]);
        assert_eq!(
            observed.lock().expect("echo observation lock").as_slice(),
            std::slice::from_ref(&callback_echo)
        );
        assert_eq!(
            poll_woken_to_ready(module.call_async(&function, (5.into(),), &mut state, &mut echo,)),
            Ok(6.into())
        );
        assert_eq!(gate.drops.load(Ordering::SeqCst), 2);
        assert_eq!(
            state.events,
            ["callback returned", "callback returned", "postprocessed"]
        );
        assert_eq!(state.total.get(), 1);
        assert_eq!(
            *observed.lock().expect("echo observation lock"),
            [
                callback_echo.clone(),
                callback_echo,
                RecordedEcho {
                    message: Some("outer".into()),
                    value: "6".into(),
                    path: Some("src/library.gleam".into()),
                    line: Some(12),
                },
            ]
        );
    }

    #[test]
    fn dropping_a_queued_callback_future_cancels_it_before_source_execution() {
        let host = AsyncHostProviderModule::<StatefulProfile>::new("application", "library")
            .expect("async host module")
            .with_scoped_async_function::<StatefulProvider, (IntCallback, BigInt), BigInt, _>(
                "invoke",
                cancel_queued_int_callback,
            )
            .expect("callback cancellation function");
        let providers = AsyncHostProviderSet::with_providers(
            Vec::<AsyncHostModule<StatefulProfile>>::new(),
            [host],
        )
        .expect("async host set");
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
@external(erlang, "native", "invoke")
fn invoke(callback: fn(Int) -> Int, value: Int) -> Int

fn callback(_value: Int) -> Int {
  panic as "cancelled callback executed"
}

pub fn run(value: Int) -> Int {
  invoke(callback, value) + 2
}
"#,
                )],
            )],
            providers,
        )
        .expect("queued callback cancellation source");
        let (bindings, function) = AsyncHostedModuleBuilder::new(program)
            .expect("queued callback cancellation plan")
            .function(FunctionDeclaration::<(BigInt,), BigInt>::new("run"))
            .expect("queued callback cancellation binding");
        let mut module = bindings.seal();
        let mut state = StatefulRunState {
            capability: BigInt::from(10),
            gate: Arc::new(ManualGate::default()),
            total: Cell::new(0),
            events: Vec::new(),
        };
        let mut echo = SendEcho::default();

        assert_eq!(
            poll_woken_to_ready(module.call_async(
                &function,
                (BigInt::from(5),),
                &mut state,
                &mut echo,
            )),
            Ok(BigInt::from(7)),
        );
        assert!(state.events.is_empty());
        assert_eq!(state.total.get(), 0);
        assert_eq!(echo.outputs, 0);
    }

    #[test]
    fn bounds_state_access_around_external_async_work_without_requiring_sync_state() {
        let host = AsyncHostProviderModule::<StatefulProfile>::new("application", "library")
            .expect("async host module")
            .with_scoped_async_function::<StatefulProvider, (BigInt,), BigInt, _>(
                "process",
                stateful_process,
            )
            .expect("scoped async function");
        let providers = AsyncHostProviderSet::with_providers(
            Vec::<AsyncHostModule<StatefulProfile>>::new(),
            [host],
        )
        .expect("async host set");
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
@external(erlang, "native", "process")
fn process(value: Int) -> Int

pub fn run(value: Int) -> Int {
  process(value) + 1
}
"#,
                )],
            )],
            providers,
        )
        .expect("async hosted source");
        let builder = AsyncHostedModuleBuilder::new(program).expect("async hosted plan");
        let (bindings, function) = builder
            .function(FunctionDeclaration::<(BigInt,), BigInt>::new("run"))
            .expect("run binding");
        let mut module = bindings.seal();
        let mut state = StatefulRunState {
            capability: BigInt::from(5),
            gate: Arc::new(ManualGate::default()),
            total: Cell::new(0),
            events: Vec::new(),
        };
        let mut echo = SendEcho::default();
        let mut call = Box::pin(require_send(module.call_async(
            &function,
            (BigInt::from(7),),
            &mut state,
            &mut echo,
        )));
        let wake = Arc::new(WakeFlag(AtomicBool::new(false)));
        let waker = Waker::from(Arc::clone(&wake));
        let mut context = Context::from_waker(&waker);

        assert_eq!(call.as_mut().poll(&mut context), Poll::Pending);
        assert!(wake.0.swap(false, Ordering::SeqCst));
        assert_eq!(call.as_mut().poll(&mut context), Poll::Ready(Ok(13.into())));
        drop(call);

        assert_eq!(state.events, ["read", "write"]);
        assert_eq!(state.total.get(), 1);
        assert_eq!(echo.outputs, 0);
    }

    #[test]
    fn cancellation_never_extends_or_replays_bounded_state_operations() {
        let host = AsyncHostProviderModule::<StatefulProfile>::new("application", "library")
            .expect("async host module")
            .with_scoped_async_function::<StatefulProvider, (BigInt,), BigInt, _>(
                "process",
                cancellable_stateful_process,
            )
            .expect("scoped async function");
        let providers = AsyncHostProviderSet::with_providers(
            Vec::<AsyncHostModule<StatefulProfile>>::new(),
            [host],
        )
        .expect("async host set");
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
@external(erlang, "native", "process")
fn process(value: Int) -> Int

pub fn run(value: Int) -> Int {
  process(value)
}
"#,
                )],
            )],
            providers,
        )
        .expect("async hosted source");
        let builder = AsyncHostedModuleBuilder::new(program).expect("async hosted plan");
        let (bindings, function) = builder
            .function(FunctionDeclaration::<(BigInt,), BigInt>::new("run"))
            .expect("run binding");
        let mut module = bindings.seal();
        let gate = Arc::new(ManualGate::default());
        let mut state = StatefulRunState {
            capability: BigInt::from(0),
            gate: Arc::clone(&gate),
            total: Cell::new(0),
            events: Vec::new(),
        };
        let mut echo = SendEcho::default();

        let unpolled = module.call_async(&function, (BigInt::from(7),), &mut state, &mut echo);
        drop(unpolled);
        assert!(state.events.is_empty());
        assert_eq!(gate.polls.load(Ordering::SeqCst), 0);
        assert_eq!(gate.drops.load(Ordering::SeqCst), 0);

        let mut call =
            Box::pin(module.call_async(&function, (BigInt::from(7),), &mut state, &mut echo));
        let wake = Arc::new(WakeFlag(AtomicBool::new(false)));
        let waker = Waker::from(Arc::clone(&wake));
        let mut context = Context::from_waker(&waker);
        assert_eq!(call.as_mut().poll(&mut context), Poll::Pending);
        assert_eq!(gate.polls.load(Ordering::SeqCst), 1);
        drop(call);

        assert_eq!(state.events, ["read"]);
        assert_eq!(state.total.get(), 0);
        assert_eq!(gate.drops.load(Ordering::SeqCst), 1);
        gate.release();
        assert_eq!(state.events, ["read"]);
        assert_eq!(state.total.get(), 0);

        let value = poll_woken_to_ready(module.call_async(
            &function,
            (BigInt::from(9),),
            &mut state,
            &mut echo,
        ))
        .expect("the module should remain usable after bounded-state cancellation");
        assert_eq!(value, BigInt::from(9));
        assert_eq!(state.events, ["read", "read", "write"]);
        assert_eq!(state.total.get(), 1);
        assert_eq!(gate.polls.load(Ordering::SeqCst), 2);
        assert_eq!(gate.drops.load(Ordering::SeqCst), 2);
    }

    #[test]
    fn preserves_scoped_async_host_failure_and_source_call_site_after_state_access() {
        let host = AsyncHostProviderModule::<StatefulProfile>::new("application", "library")
            .expect("async host module")
            .with_fallible_scoped_async_function::<StatefulProvider, (BigInt,), BigInt, _>(
                "process",
                fallible_stateful_process,
            )
            .expect("fallible scoped async function");
        let providers = AsyncHostProviderSet::with_providers(
            Vec::<AsyncHostModule<StatefulProfile>>::new(),
            [host],
        )
        .expect("async host set");
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
@external(erlang, "native", "process")
fn process(value: Int) -> Int

pub fn run(value: Int) -> Int {
  process(value)
}
"#,
                )],
            )],
            providers,
        )
        .expect("async hosted source");
        let builder = AsyncHostedModuleBuilder::new(program).expect("async hosted plan");
        let (bindings, function) = builder
            .function(FunctionDeclaration::<(BigInt,), BigInt>::new("run"))
            .expect("run binding");
        let mut module = bindings.seal();
        let mut state = StatefulRunState {
            capability: BigInt::from(5),
            gate: Arc::new(ManualGate::default()),
            total: Cell::new(0),
            events: Vec::new(),
        };
        let mut echo = SendEcho::default();
        let error = poll_woken_to_ready(module.call_async(
            &function,
            (BigInt::from(7),),
            &mut state,
            &mut echo,
        ))
        .expect_err("scoped host call should fail after one suspension");
        assert!(matches!(
            &error,
            CallError::Execution(ExecutionError::Host(error))
                if (
                    error.package().as_str(),
                    error.module().as_str(),
                    error.function().as_str(),
                    error.failure(),
                    error.location().site().map(|site| (
                        site.module().as_str(),
                        site.function().as_str(),
                    )),
                    error.location().path().map(|path| path.as_str()),
                    error.location().line(),
                ) == (
                    "application",
                    "library",
                    "process",
                    &HostFailure::new("stopped at 12"),
                    Some(("library", "run")),
                    Some("src/library.gleam"),
                    Some(6),
                )
        ));
        assert_eq!(state.events, ["read"]);
        assert_eq!(state.total.get(), 0);
        assert_eq!(echo.outputs, 0);
    }

    #[test]
    fn transfers_the_complete_public_data_surface_across_pending_execution() {
        type Scalars = (BigInt, f64, EcoString, BitArrayValue, char, bool, ());
        type Row = Result<(EcoString, BigInt), EcoString>;
        type Nested = AsyncList<AsyncList<Row>>;
        type OptionalInts = Option<AsyncList<BigInt>>;
        type PairList = AsyncList<(BigInt, EcoString)>;
        type OptionList = AsyncList<Option<BigInt>>;
        type NestedInts = AsyncList<AsyncList<BigInt>>;
        type DirectResult = Result<AsyncList<BigInt>, AsyncList<EcoString>>;
        type EdgeOutput = (
            AsyncList<()>,
            PairList,
            OptionList,
            NestedInts,
            DirectResult,
        );

        fn assert_send<Value: Send>() {}
        assert_send::<AsyncList<BigInt>>();

        let polls = Arc::new(AtomicUsize::new(0));
        let host = AsyncHostProviderModule::new("application", "library")
            .expect("async host module")
            .with_async_function("suspend", {
                let polls = Arc::clone(&polls);
                move |value: BigInt| PendingOnce {
                    polls: Arc::clone(&polls),
                    pending_returned: false,
                    value: ready(value),
                }
            })
            .expect("async host function");
        let providers = AsyncHostProviderSet::with_providers(Vec::<AsyncHostModule>::new(), [host])
            .expect("async host set");
        let program = compile_typed_async_host_program(
            "application",
            "library",
            [
                PackageSource::new(
                    "gleam_stdlib",
                    Vec::<EcoString>::new(),
                    [ModuleSource::new(
                        "gleam/option",
                        "gleam_stdlib/src/gleam/option.gleam",
                        "pub type Option(value) { Some(value) None }",
                    )],
                ),
                PackageSource::new(
                    "application",
                    ["gleam_stdlib"],
                    [ModuleSource::new(
                        "library",
                        "src/library.gleam",
                        r#"
import gleam/option.{type Option, None, Some}

@external(erlang, "native", "suspend")
fn suspend(value: Int) -> Int

pub fn round_trip(
  scalars: #(Int, Float, String, BitArray, UtfCodepoint, Bool, Nil),
  nested: List(List(Result(#(String, Int), String))),
  optional: Option(List(Int)),
) {
  let resumed = suspend(7)
  #(scalars, nested, optional, resumed)
}

pub fn result_value(success: Bool) -> Result(Int, String) {
  let _ = suspend(0)
  case success {
    True -> Ok(11)
    False -> Error("stopped")
  }
}

pub fn option_value(present: Bool) -> Option(List(Int)) {
  let _ = suspend(0)
  case present {
    True -> Some([12, 13])
    False -> None
  }
}

pub fn no_arguments() -> Nil {
  let _ = suspend(0)
  Nil
}

pub fn input_families(
  nils: List(Nil),
  pairs: List(#(Int, String)),
  options: List(Option(Int)),
  nested: List(List(Int)),
  result: Result(List(Int), List(String)),
) {
  let _ = suspend(0)
  #(nils, pairs, options, nested, result)
}
"#,
                    )],
                ),
            ],
            providers,
        )
        .expect("complete async data source");
        let builder = AsyncHostedModuleBuilder::new(program).expect("complete async data plan");
        let (mut bindings, function) = builder
            .function(FunctionDeclaration::<
                (Scalars, Nested, OptionalInts),
                (Scalars, Nested, OptionalInts, BigInt),
            >::new("round_trip"))
            .expect("complete async data binding");
        let result_value = bindings
            .function(
                FunctionDeclaration::<(bool,), Result<BigInt, EcoString>>::new("result_value"),
            )
            .expect("Result binding");
        let option_value = bindings
            .function(
                FunctionDeclaration::<(bool,), Option<AsyncList<BigInt>>>::new("option_value"),
            )
            .expect("Option binding");
        let no_arguments = bindings
            .function(FunctionDeclaration::<(), ()>::new("no_arguments"))
            .expect("zero-argument binding");
        let input_families = bindings
            .function(FunctionDeclaration::<
                (
                    AsyncList<()>,
                    PairList,
                    OptionList,
                    NestedInts,
                    DirectResult,
                ),
                EdgeOutput,
            >::new("input_families"))
            .expect("edge input families binding");
        let mut module = bindings.seal();
        let bits = BitArrayValue::try_from_parts(vec![0b1010_0000], 3)
            .expect("three bits should fit in one byte");
        let scalars = (
            BigInt::from(1),
            2.5,
            EcoString::from("three"),
            bits,
            '四',
            true,
            (),
        );
        let nested: Vec<Vec<Row>> = vec![
            vec![Ok(("first".into(), 1.into())), Err("error".into())],
            vec![Ok(("second".into(), 2.into()))],
        ];
        let optional = Some(vec![BigInt::from(8), 9.into()]);
        let mut state = ();
        let mut echo = SendEcho::default();
        let output = poll_woken_to_ready(require_send(module.call_async(
            &function,
            (scalars.clone(), nested.clone(), optional.clone()),
            &mut state,
            &mut echo,
        )))
        .expect("complete data call should finish after one suspension");

        assert_eq!(output.0, scalars);
        assert_eq!(output.1.len(), 2);
        assert_eq!(
            output.1.get(0).expect("first nested List").to_vec(),
            nested[0]
        );
        assert_eq!(
            output.1.get(1).expect("second nested List").to_vec(),
            nested[1]
        );
        assert_eq!(output.2.map(|values| values.to_vec()), optional,);
        assert_eq!(output.3, BigInt::from(7));

        let alternate = poll_woken_to_ready(module.call_async(
            &function,
            (
                scalars,
                vec![vec![Err(EcoString::from("alternate"))]],
                None::<Vec<BigInt>>,
            ),
            &mut state,
            &mut echo,
        ))
        .expect("Error and None inputs should cross the async boundary");
        assert_eq!(
            alternate.1.get(0).expect("nested List").to_vec(),
            [Err(EcoString::from("alternate"))],
        );
        assert!(alternate.2.is_none());

        assert_eq!(
            poll_woken_to_ready(module.call_async(&result_value, (true,), &mut state, &mut echo,)),
            Ok(Ok(BigInt::from(11))),
        );
        assert_eq!(
            poll_woken_to_ready(module.call_async(&result_value, (false,), &mut state, &mut echo,)),
            Ok(Err(EcoString::from("stopped"))),
        );

        let present =
            poll_woken_to_ready(module.call_async(&option_value, (true,), &mut state, &mut echo))
                .expect("Some output should cross the async boundary")
                .expect("Some output");
        assert_eq!(present.to_vec(), [BigInt::from(12), BigInt::from(13)]);
        let absent =
            poll_woken_to_ready(module.call_async(&option_value, (false,), &mut state, &mut echo))
                .expect("None output should cross the async boundary");
        assert!(absent.is_none());
        assert_eq!(
            poll_woken_to_ready(module.call_async(&no_arguments, (), &mut state, &mut echo,)),
            Ok(()),
        );

        let edge = poll_woken_to_ready(module.call_async(
            &input_families,
            (
                vec![(), ()],
                vec![(BigInt::from(2), EcoString::from("two"))],
                vec![Some(BigInt::from(3)), None],
                vec![vec![BigInt::from(4), 5.into()]],
                Err::<Vec<BigInt>, _>(vec![EcoString::from("failure")]),
            ),
            &mut state,
            &mut echo,
        ))
        .expect("fresh edge input families");
        assert_eq!(edge.0.to_vec(), [(), ()]);
        assert_eq!(edge.1.to_vec(), [(BigInt::from(2), "two".into())]);
        assert_eq!(edge.2.to_vec(), [Some(BigInt::from(3)), None]);
        assert_eq!(
            edge.3.get(0).expect("nested Int List").to_vec(),
            [BigInt::from(4), BigInt::from(5)],
        );
        assert_eq!(
            edge.4.as_ref().err().expect("Error List").to_vec(),
            [EcoString::from("failure")],
        );

        let retained_result = edge.4.as_ref();
        let retained = poll_woken_to_ready(module.call_async(
            &input_families,
            (&edge.0, &edge.1, &edge.2, &edge.3, retained_result),
            &mut state,
            &mut echo,
        ))
        .expect("same-owner retained edge inputs");
        assert_eq!(retained.1.to_vec(), [(BigInt::from(2), "two".into())]);
        assert_eq!(
            retained.4.err().expect("retained Error List").to_vec(),
            [EcoString::from("failure")],
        );

        let successful = poll_woken_to_ready(module.call_async(
            &input_families,
            (
                Vec::<()>::new(),
                Vec::<(BigInt, EcoString)>::new(),
                Vec::<Option<BigInt>>::new(),
                Vec::<Vec<BigInt>>::new(),
                Ok::<_, Vec<EcoString>>(vec![BigInt::from(6)]),
            ),
            &mut state,
            &mut echo,
        ))
        .expect("successful Result input");
        assert_eq!(
            successful.4.ok().expect("Ok List").to_vec(),
            [BigInt::from(6)],
        );

        assert_eq!(polls.load(Ordering::SeqCst), 20);
        assert_eq!(echo.outputs, 0);
    }

    #[test]
    fn returns_each_public_root_list_family_and_consumes_function_lists() {
        type Ints = AsyncList<BigInt>;
        type Floats = AsyncList<f64>;
        type Strings = AsyncList<EcoString>;
        type BitArrays = AsyncList<BitArrayValue>;
        type Codepoints = AsyncList<char>;
        type Bools = AsyncList<bool>;
        type Nils = AsyncList<()>;
        type Customs = AsyncList<Result<BigInt, EcoString>>;
        type Tuples = AsyncList<(BigInt, EcoString)>;
        type Nested = AsyncList<AsyncList<BigInt>>;

        let providers =
            AsyncHostProviderSet::new(Vec::<AsyncHostModule>::new()).expect("empty async host set");
        let program = compile_typed_async_host_program(
            "application",
            "library",
            [PackageSource::new(
                "application",
                Vec::<EcoString>::new(),
                [ModuleSource::new(
                    "library",
                    "src/library.gleam",
                    r#"
pub fn ints(values: List(Int)) { values }
pub fn floats(values: List(Float)) { values }
pub fn strings(values: List(String)) { values }
pub fn bit_arrays(values: List(BitArray)) { values }
pub fn codepoints(values: List(UtfCodepoint)) { values }
pub fn bools(values: List(Bool)) { values }
pub fn nils(values: List(Nil)) { values }
pub fn customs(values: List(Result(Int, String))) { values }
pub fn tuples(values: List(#(Int, String))) { values }
pub fn nested(values: List(List(Int))) { values }

fn functions(offset: Int) -> List(fn(Int) -> Int) {
  [fn(value) { value + offset }]
}

pub fn through_function_list(value: Int) -> Int {
  let assert [function] = functions(2)
  function(value)
}
"#,
                )],
            )],
            providers,
        )
        .expect("root List family source");
        let (mut bindings, ints) = AsyncHostedModuleBuilder::new(program)
            .expect("root List family plan")
            .function(FunctionDeclaration::<(Ints,), Ints>::new("ints"))
            .expect("Int List binding");
        let floats = bindings
            .function(FunctionDeclaration::<(Floats,), Floats>::new("floats"))
            .expect("Float List binding");
        let strings = bindings
            .function(FunctionDeclaration::<(Strings,), Strings>::new("strings"))
            .expect("String List binding");
        let bit_arrays = bindings
            .function(FunctionDeclaration::<(BitArrays,), BitArrays>::new(
                "bit_arrays",
            ))
            .expect("BitArray List binding");
        let codepoints = bindings
            .function(FunctionDeclaration::<(Codepoints,), Codepoints>::new(
                "codepoints",
            ))
            .expect("UtfCodepoint List binding");
        let bools = bindings
            .function(FunctionDeclaration::<(Bools,), Bools>::new("bools"))
            .expect("Bool List binding");
        let nils = bindings
            .function(FunctionDeclaration::<(Nils,), Nils>::new("nils"))
            .expect("Nil List binding");
        let customs = bindings
            .function(FunctionDeclaration::<(Customs,), Customs>::new("customs"))
            .expect("custom List binding");
        let tuples = bindings
            .function(FunctionDeclaration::<(Tuples,), Tuples>::new("tuples"))
            .expect("tuple List binding");
        let nested = bindings
            .function(FunctionDeclaration::<(Nested,), Nested>::new("nested"))
            .expect("nested List binding");
        let through_function_list = bindings
            .function(FunctionDeclaration::<(BigInt,), BigInt>::new(
                "through_function_list",
            ))
            .expect("function List consumer binding");
        let mut module = bindings.seal();
        let mut state = ();
        let mut echo = SendEcho::default();

        let int_values = vec![BigInt::from(1), 2.into()];
        assert_eq!(
            poll_woken_to_ready(module.call_async(
                &ints,
                (int_values.clone(),),
                &mut state,
                &mut echo,
            ))
            .expect("Int List call")
            .to_vec(),
            int_values,
        );
        assert_eq!(
            poll_woken_to_ready(module.call_async(
                &floats,
                (vec![1.5, 2.5],),
                &mut state,
                &mut echo,
            ))
            .expect("Float List call")
            .to_vec(),
            [1.5, 2.5],
        );
        assert_eq!(
            poll_woken_to_ready(module.call_async(
                &strings,
                (vec![EcoString::from("one"), "two".into()],),
                &mut state,
                &mut echo,
            ))
            .expect("String List call")
            .to_vec(),
            [EcoString::from("one"), EcoString::from("two")],
        );
        let bits = BitArrayValue::try_from_parts(vec![0b1010_0000], 3)
            .expect("three bits should fit in one byte");
        assert_eq!(
            poll_woken_to_ready(module.call_async(
                &bit_arrays,
                (vec![bits.clone()],),
                &mut state,
                &mut echo,
            ))
            .expect("BitArray List call")
            .to_vec(),
            [bits],
        );
        assert_eq!(
            poll_woken_to_ready(module.call_async(
                &codepoints,
                (vec!['a', '四'],),
                &mut state,
                &mut echo,
            ))
            .expect("UtfCodepoint List call")
            .to_vec(),
            ['a', '四'],
        );
        assert_eq!(
            poll_woken_to_ready(module.call_async(
                &bools,
                (vec![true, false],),
                &mut state,
                &mut echo,
            ))
            .expect("Bool List call")
            .to_vec(),
            [true, false],
        );
        assert_eq!(
            poll_woken_to_ready(module.call_async(&nils, (vec![(), ()],), &mut state, &mut echo,))
                .expect("Nil List call")
                .to_vec(),
            [(), ()],
        );
        let custom_values = vec![Ok(BigInt::from(3)), Err(EcoString::from("four"))];
        assert_eq!(
            poll_woken_to_ready(module.call_async(
                &customs,
                (custom_values.clone(),),
                &mut state,
                &mut echo,
            ))
            .expect("custom List call")
            .to_vec(),
            custom_values,
        );
        let tuple_values = vec![(BigInt::from(5), EcoString::from("five"))];
        assert_eq!(
            poll_woken_to_ready(module.call_async(
                &tuples,
                (tuple_values.clone(),),
                &mut state,
                &mut echo,
            ))
            .expect("tuple List call")
            .to_vec(),
            tuple_values,
        );
        let nested_values = vec![vec![BigInt::from(6), 7.into()]];
        let returned = poll_woken_to_ready(module.call_async(
            &nested,
            (nested_values.clone(),),
            &mut state,
            &mut echo,
        ))
        .expect("nested List call");
        assert_eq!(
            returned.get(0).expect("first nested List").to_vec(),
            nested_values[0],
        );
        assert_eq!(
            poll_woken_to_ready(module.call_async(
                &through_function_list,
                (BigInt::from(8),),
                &mut state,
                &mut echo,
            )),
            Ok(BigInt::from(10)),
        );
        assert_eq!(echo.outputs, 0);
    }

    #[test]
    fn resumes_every_source_return_family_at_its_exact_non_tail_destination() {
        type Scalars = (BigInt, f64, EcoString, BitArrayValue, char, bool, ());
        type Compound = (
            Result<BigInt, EcoString>,
            (BigInt, EcoString),
            AsyncList<BigInt>,
        );
        type Output = (Scalars, Compound, BigInt);

        let polls = Arc::new(AtomicUsize::new(0));
        let host = AsyncHostProviderModule::new("application", "library")
            .expect("async host module")
            .with_async_function("suspend", {
                let polls = Arc::clone(&polls);
                move |value: BigInt| PendingOnce {
                    polls: Arc::clone(&polls),
                    pending_returned: false,
                    value: ready(value),
                }
            })
            .expect("async host function");
        let providers = AsyncHostProviderSet::with_providers(Vec::<AsyncHostModule>::new(), [host])
            .expect("async host set");
        let program = compile_typed_async_host_program(
            "application",
            "library",
            [PackageSource::new(
                "application",
                Vec::<EcoString>::new(),
                [ModuleSource::new(
                    "library",
                    "src/library.gleam",
                    r#"
@external(erlang, "native", "suspend")
fn suspend(value: Int) -> Int

fn identity(value: value) -> value { value }

fn return_int(value: Int) -> Int { let _ = suspend(0) value }
fn return_float(value: Float) -> Float { let _ = suspend(0) value }
fn return_string(value: String) -> String { let _ = suspend(0) value }
fn return_bits(value: BitArray) -> BitArray { let _ = suspend(0) value }
fn return_codepoint(value: UtfCodepoint) -> UtfCodepoint { let _ = suspend(0) value }
fn return_bool(value: Bool) -> Bool { let _ = suspend(0) value }
fn return_nil(value: Nil) -> Nil { let _ = suspend(0) value }
fn return_custom(value: Result(Int, String)) -> Result(Int, String) {
  let _ = suspend(0)
  identity(value)
}
fn return_tuple(value: #(Int, String)) -> #(Int, String) {
  let _ = suspend(0)
  identity(value)
}
fn return_list(value: List(Int)) -> List(Int) {
  let _ = suspend(0)
  identity(value)
}
fn return_function(offset: Int) -> fn(Int) -> Int {
  let _ = suspend(0)
  identity(fn(value) { value + offset })
}

pub fn exercise(
  scalars: #(Int, Float, String, BitArray, UtfCodepoint, Bool, Nil),
  custom: Result(Int, String),
  tuple: #(Int, String),
  list: List(Int),
) {
  let #(int, float, string, bits, codepoint, bool, nil) = scalars
  let captured = fn(value) { value + 9 }
  let int = return_int(int)
  let float = return_float(float)
  let string = return_string(string)
  let bits = return_bits(bits)
  let codepoint = return_codepoint(codepoint)
  let bool = return_bool(bool)
  let nil = return_nil(nil)
  let custom = return_custom(custom)
  let tuple = return_tuple(tuple)
  let get_list = return_list
  let list = get_list(list)
  let function_value = return_function(3)
  #(
    #(int, float, string, bits, codepoint, bool, nil),
    #(custom, tuple, list),
    captured(function_value(4)),
  )
}
"#,
                )],
            )],
            providers,
        )
        .expect("all return families source");
        let (bindings, function) = AsyncHostedModuleBuilder::new(program)
            .expect("all return families plan")
            .function(FunctionDeclaration::<
                (
                    Scalars,
                    Result<BigInt, EcoString>,
                    (BigInt, EcoString),
                    AsyncList<BigInt>,
                ),
                Output,
            >::new("exercise"))
            .expect("all return families binding");
        let mut module = bindings.seal();
        let bits = BitArrayValue::try_from_parts(vec![0b1010_0000], 3)
            .expect("three bits should fit in one byte");
        let scalars = (
            BigInt::from(1),
            2.5,
            EcoString::from("three"),
            bits,
            '四',
            true,
            (),
        );
        let custom = Ok(BigInt::from(4));
        let tuple = (BigInt::from(5), EcoString::from("five"));
        let list = vec![BigInt::from(6), 7.into()];
        let mut state = ();
        let mut echo = SendEcho::default();

        let output = poll_woken_to_ready(require_send(module.call_async(
            &function,
            (scalars.clone(), custom.clone(), tuple.clone(), list.clone()),
            &mut state,
            &mut echo,
        )))
        .expect("all return families call");

        assert_eq!(output.0, scalars);
        assert_eq!(output.1.0, custom);
        assert_eq!(output.1.1, tuple);
        assert_eq!(output.1.2.to_vec(), list);
        assert_eq!(output.2, BigInt::from(16));
        assert_eq!(polls.load(Ordering::SeqCst), 22);
        assert_eq!(echo.outputs, 0);
    }

    #[test]
    fn retains_async_lists_lazily_and_rejects_a_foreign_owner_before_execution() {
        let build = || {
            let providers = AsyncHostProviderSet::new(Vec::<AsyncHostModule>::new())
                .expect("empty async host set");
            let program = compile_typed_async_host_program(
                "application",
                "library",
                [PackageSource::new(
                    "application",
                    Vec::<EcoString>::new(),
                    [ModuleSource::new(
                        "library",
                        "src/library.gleam",
                        "pub fn keep(values: List(String)) { values }",
                    )],
                )],
                providers,
            )
            .expect("async List source");
            AsyncHostedModuleBuilder::new(program).expect("async List plan")
        };

        let (bindings, keep) = build()
            .function(FunctionDeclaration::<
                (AsyncList<EcoString>,),
                AsyncList<EcoString>,
            >::new("keep"))
            .expect("first List binding");
        let mut module = bindings.seal();
        let mut state = ();
        let mut echo = SendEcho::default();
        let values = poll_woken_to_ready(module.call_async(
            &keep,
            (vec![EcoString::from("first"), "second".into()],),
            &mut state,
            &mut echo,
        ))
        .expect("fresh List call");
        assert_eq!(values.len(), 2);
        assert!(!values.is_empty());
        assert_eq!(values.item_reads(), 0);

        let retained =
            poll_woken_to_ready(module.call_async(&keep, (&values,), &mut state, &mut echo))
                .expect("same-owner retained List call");
        assert_eq!(values.item_reads(), 0);
        assert_eq!(retained.item_reads(), 0);
        assert_eq!(retained.get(1), Some(EcoString::from("second")));
        assert_eq!(retained.item_reads(), 1);

        let (foreign_bindings, foreign_keep) = build()
            .function(FunctionDeclaration::<
                (AsyncList<EcoString>,),
                AsyncList<EcoString>,
            >::new("keep"))
            .expect("foreign List binding");
        let mut foreign_module = foreign_bindings.seal();
        let error = poll_woken_to_ready(module.call_async(
            &foreign_keep,
            (&values,),
            &mut state,
            &mut echo,
        ));
        assert_eq!(error.err(), Some(CallError::ForeignFunction));

        let error = poll_woken_to_ready(foreign_module.call_async(
            &foreign_keep,
            (&values,),
            &mut state,
            &mut echo,
        ));
        assert_eq!(error.err(), Some(CallError::ForeignValue));
        assert_eq!(values.item_reads(), 0);

        let call_after_input_drop = module.call_async(&keep, (&values,), &mut state, &mut echo);
        drop(values);
        let retained_after_input_drop = poll_woken_to_ready(call_after_input_drop)
            .expect("owned retained input after call construction");
        assert_eq!(retained_after_input_drop.to_vec(), ["first", "second"]);
        assert_eq!(echo.outputs, 0);
    }

    #[test]
    fn preserves_echo_order_and_source_panic_identity_after_pending() {
        let polls = Arc::new(AtomicUsize::new(0));
        let host = AsyncHostProviderModule::new("application", "library")
            .expect("async host module")
            .with_async_function("suspend", {
                let polls = Arc::clone(&polls);
                move |value: BigInt| PendingOnce {
                    polls: Arc::clone(&polls),
                    pending_returned: false,
                    value: ready(value),
                }
            })
            .expect("async host function");
        let providers = AsyncHostProviderSet::with_providers(Vec::<AsyncHostModule>::new(), [host])
            .expect("async host set");
        let source = r#"
@external(erlang, "native", "suspend")
fn suspend(value: Int) -> Int

pub fn explode(value: Int) -> Int {
  echo value as "before"
  let resumed = suspend(value)
  echo resumed as "after"
  panic as "stopped"
}
"#;
        let program = compile_typed_async_host_program(
            "application",
            "library",
            [PackageSource::new(
                "application",
                Vec::<EcoString>::new(),
                [ModuleSource::new("library", "src/library.gleam", source)],
            )],
            providers,
        )
        .expect("panic source");
        let (bindings, function) = AsyncHostedModuleBuilder::new(program)
            .expect("panic plan")
            .function(FunctionDeclaration::<(BigInt,), BigInt>::new("explode"))
            .expect("panic binding");
        let mut module = bindings.seal();
        let mut state = ();
        let mut echo = OrderedEcho::default();
        let observed = Arc::clone(&echo.outputs);
        let mut call =
            Box::pin(module.call_async(&function, (BigInt::from(7),), &mut state, &mut echo));
        let wake = Arc::new(WakeFlag(AtomicBool::new(false)));
        let waker = Waker::from(Arc::clone(&wake));
        let mut context = Context::from_waker(&waker);

        assert_eq!(call.as_mut().poll(&mut context), Poll::Pending);
        assert!(wake.0.load(Ordering::SeqCst));
        assert_eq!(
            *observed.lock().expect("echo observation lock"),
            vec![RecordedEcho {
                message: Some("before".into()),
                value: "7".into(),
                path: Some("src/library.gleam".into()),
                line: Some(6),
            }],
        );

        let expression = "panic as \"stopped\"";
        let start = source
            .find(expression)
            .expect("fixture should contain panic expression");
        assert_eq!(
            call.as_mut().poll(&mut context),
            Poll::Ready(Err(CallError::Execution(ExecutionError::source_panic(
                Some(&SourceContext::new("src/library.gleam", source)),
                PanicKind::Panic,
                Some("stopped".into()),
                PanicSite::new(
                    "library".into(),
                    "explode".into(),
                    SourceSpan::new(start, start + expression.len()),
                ),
            )))),
        );
        assert_eq!(polls.load(Ordering::SeqCst), 2);
        assert_eq!(
            *observed.lock().expect("echo observation lock"),
            vec![
                RecordedEcho {
                    message: Some("before".into()),
                    value: "7".into(),
                    path: Some("src/library.gleam".into()),
                    line: Some(6),
                },
                RecordedEcho {
                    message: Some("after".into()),
                    value: "7".into(),
                    path: Some("src/library.gleam".into()),
                    line: Some(8),
                },
            ],
        );
    }

    #[test]
    fn preserves_async_host_failure_identity_and_source_call_site() {
        let polls = Arc::new(AtomicUsize::new(0));
        let host = AsyncHostProviderModule::new("application", "library")
            .expect("async host module")
            .with_fallible_async_function("fail", {
                let polls = Arc::clone(&polls);
                move |_value: BigInt| PendingOnce {
                    polls: Arc::clone(&polls),
                    pending_returned: false,
                    value: ready(Err::<BigInt, HostFailure>(HostFailure::new("stopped"))),
                }
            })
            .expect("fallible async host function");
        let providers = AsyncHostProviderSet::with_providers(Vec::<AsyncHostModule>::new(), [host])
            .expect("async host set");
        let program = compile_typed_async_host_program(
            "application",
            "library",
            [PackageSource::new(
                "application",
                Vec::<EcoString>::new(),
                [ModuleSource::new(
                    "library",
                    "src/library.gleam",
                    r#"
@external(erlang, "native", "fail")
fn fail(value: Int) -> Int

pub fn run(value: Int) { fail(value) }
"#,
                )],
            )],
            providers,
        )
        .expect("host failure source");
        let (bindings, function) = AsyncHostedModuleBuilder::new(program)
            .expect("host failure plan")
            .function(FunctionDeclaration::<(BigInt,), BigInt>::new("run"))
            .expect("host failure binding");
        let mut module = bindings.seal();
        let mut state = ();
        let mut echo = SendEcho::default();
        let error = poll_woken_to_ready(module.call_async(
            &function,
            (BigInt::from(7),),
            &mut state,
            &mut echo,
        ))
        .expect_err("host call should fail after one suspension");
        assert!(matches!(
            &error,
            CallError::Execution(ExecutionError::Host(error))
                if (
                    error.package().as_str(),
                    error.module().as_str(),
                    error.function().as_str(),
                    error.failure(),
                    error.location().site().map(|site| (
                        site.module().as_str(),
                        site.function().as_str(),
                    )),
                    error.location().path().map(|path| path.as_str()),
                    error.location().line(),
                ) == (
                    "application",
                    "library",
                    "fail",
                    &HostFailure::new("stopped"),
                    Some(("library", "run")),
                    Some("src/library.gleam"),
                    Some(5),
                )
        ));
        assert_eq!(polls.load(Ordering::SeqCst), 2);
        assert_eq!(echo.outputs, 0);
    }

    #[test]
    fn cancellation_before_poll_and_at_leaf_pending_has_exact_effect_and_drop_ownership() {
        let creations = Arc::new(AtomicUsize::new(0));
        let polls = Arc::new(AtomicUsize::new(0));
        let drops = Arc::new(AtomicUsize::new(0));
        let after_calls = Arc::new(AtomicUsize::new(0));
        let host = AsyncHostProviderModule::new("application", "library")
            .expect("async host module")
            .with_async_function("hold", {
                let creations = Arc::clone(&creations);
                let polls = Arc::clone(&polls);
                let drops = Arc::clone(&drops);
                move |_value: BigInt| {
                    creations.fetch_add(1, Ordering::SeqCst);
                    PendingForever {
                        polls: Arc::clone(&polls),
                        drops: Arc::clone(&drops),
                    }
                }
            })
            .expect("pending host function")
            .with_function("after", {
                let after_calls = Arc::clone(&after_calls);
                move |value: BigInt| {
                    after_calls.fetch_add(1, Ordering::SeqCst);
                    value
                }
            })
            .expect("post-pending host function");
        let providers = AsyncHostProviderSet::with_providers(Vec::<AsyncHostModule>::new(), [host])
            .expect("async host set");
        let program = compile_typed_async_host_program(
            "application",
            "library",
            [PackageSource::new(
                "application",
                Vec::<EcoString>::new(),
                [ModuleSource::new(
                    "library",
                    "src/library.gleam",
                    r#"
@external(erlang, "native", "hold")
fn hold(value: Int) -> Int

@external(erlang, "native", "after")
fn after(value: Int) -> Int

pub fn calculate(value: Int) {
  after(hold(value))
}

pub fn after_only(value: Int) {
  after(value)
}
"#,
                )],
            )],
            providers,
        )
        .expect("cancellation source");
        let (mut bindings, function) = AsyncHostedModuleBuilder::new(program)
            .expect("cancellation plan")
            .function(FunctionDeclaration::<(BigInt,), BigInt>::new("calculate"))
            .expect("cancellation binding");
        let after_only = bindings
            .function(FunctionDeclaration::<(BigInt,), BigInt>::new("after_only"))
            .expect("post-cancellation binding");
        let mut module = bindings.seal();
        let mut state = ();
        let mut echo = SendEcho::default();

        let call = module.call_async(&function, (BigInt::from(1),), &mut state, &mut echo);
        drop(call);
        assert_eq!(creations.load(Ordering::SeqCst), 0);
        assert_eq!(polls.load(Ordering::SeqCst), 0);
        assert_eq!(drops.load(Ordering::SeqCst), 0);
        assert_eq!(after_calls.load(Ordering::SeqCst), 0);

        let mut call =
            Box::pin(module.call_async(&function, (BigInt::from(2),), &mut state, &mut echo));
        let wake = Arc::new(WakeFlag(AtomicBool::new(false)));
        let waker = Waker::from(wake);
        let mut context = Context::from_waker(&waker);
        assert_eq!(call.as_mut().poll(&mut context), Poll::Pending);
        assert_eq!(creations.load(Ordering::SeqCst), 1);
        assert_eq!(polls.load(Ordering::SeqCst), 1);
        assert_eq!(drops.load(Ordering::SeqCst), 0);
        assert_eq!(after_calls.load(Ordering::SeqCst), 0);

        drop(call);
        assert_eq!(drops.load(Ordering::SeqCst), 1);
        assert_eq!(after_calls.load(Ordering::SeqCst), 0);

        assert_eq!(
            poll_woken_to_ready(module.call_async(
                &after_only,
                (BigInt::from(3),),
                &mut state,
                &mut echo,
            )),
            Ok(BigInt::from(3)),
        );
        assert_eq!(after_calls.load(Ordering::SeqCst), 1);
        assert_eq!(echo.outputs, 0);
    }
}
