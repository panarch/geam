mod native;

pub use native::NativeCallable;
#[doc(hidden)]
pub use native::{NativeArguments, NativeCaptures, NativeType};

use super::input::{
    InputConstructions, ListFamily, ScopedArgumentsInput, ScopedFreshInput, ScopedInputValue,
};
use super::value::{Arguments, EmbeddingValue};
use super::work::return_::{ScopedReturn, ScopedTake};
use super::work::{ReadValue, ScopeBrand, ScopedOutput, SharedValue, SourceType};
use super::{CallError, ExecutionScope};
use crate::host::HostProfile;
use crate::plan::execution::{
    LibraryCallable, LibraryFunctionEntries, LibraryInputConstructions, LibraryListConstructions,
};
use crate::plan::{
    FunctionType, LibraryCallableSignature, LibraryValueType, LibraryVariant, StandardVariant,
};
use crate::runtime::{
    BorrowedValue, EmbeddingCallable, EmbeddingInputStorage, EmbeddingOutput, RetainedInputs,
};
use std::marker::PhantomData;
use std::sync::Arc;

/// A typed Gleam function value in an embedding signature.
pub struct CallableType<Args, Return>(PhantomData<fn(Args) -> Return>);

/// A function value retained within its original execution scope.
///
/// Use [`ExecutionScope::invoke`] to call it. Cloning retains the same identity
/// and captures; each invocation has independent arguments and cancellation.
/// A callable cannot enter another execution lifetime:
///
/// ```compile_fail
/// use geam_core::embedding::{BigInt, Callable, ExecutionScope};
/// use geam_core::HostProfile;
/// fn foreign<'a, 'b, P: HostProfile>(
///     scope: &ExecutionScope<'a, '_, P>,
///     callback: Callable<'b, (BigInt,), BigInt>,
/// ) {
///     let _ = scope.invoke(&callback, (BigInt::from(1),));
/// }
/// ```
///
/// Containers and completed work preserve the same lifetime:
///
/// ```compile_fail
/// use geam_core::embedding::{BigInt, Callable, Completed, SharedList};
/// fn escape<'scope>(value: Completed<SharedList<Callable<'scope, (), BigInt>>>)
///     -> Completed<SharedList<Callable<'static, (), BigInt>>>
/// {
///     value
/// }
/// ```
#[allow(private_bounds)]
pub struct Callable<'scope, Args, Return> {
    value: EmbeddingCallable,
    context: CallableContext<'scope>,
    marker: PhantomData<fn(Args) -> Return>,
}

pub(crate) struct OutputCallables<'scope> {
    values: &'scope [LibraryCallable],
    position: usize,
}

pub(crate) struct CallableContext<'scope> {
    brand: ScopeBrand<'scope>,
    owner: Arc<()>,
    declaration: &'scope LibraryCallable,
}

impl<'scope> OutputCallables<'scope> {
    pub(super) fn new(values: &'scope [LibraryCallable]) -> Self {
        Self {
            values,
            position: 0,
        }
    }

    fn take(&mut self) -> &'scope LibraryCallable {
        let value = &self.values[self.position];
        self.position += 1;
        value
    }
}

impl Clone for CallableContext<'_> {
    fn clone(&self) -> Self {
        Self {
            brand: self.brand,
            owner: Arc::clone(&self.owner),
            declaration: self.declaration,
        }
    }
}

impl<Args, Return> Clone for Callable<'_, Args, Return> {
    fn clone(&self) -> Self {
        Self {
            value: self.value.clone(),
            context: self.context.clone(),
            marker: PhantomData,
        }
    }
}

impl<Args: Arguments, Return: EmbeddingValue> EmbeddingValue for CallableType<Args, Return> {
    const VARIANT_COUNT: usize = 0;
    const LIST_COUNTS: [usize; 12] = [0; 12];
    const LIST_FAMILY: ListFamily = ListFamily::Function;

    fn library_type() -> LibraryValueType {
        LibraryValueType::Function(FunctionType::new(Args::value_types(), Return::value_type()))
    }

    fn collect_variants(variants: &mut Vec<StandardVariant>) {
        variants.extend(Args::standard_variants());
        Return::collect_variants(variants);
    }

    fn collect_input_variants(_: &mut Vec<LibraryVariant>) {}
    fn collect_lists(_: &mut Vec<LibraryValueType>) {}

    fn collect_callables(callables: &mut Vec<LibraryCallableSignature>) {
        callables.push(LibraryCallableSignature {
            type_: FunctionType::new(Args::value_types(), Return::value_type()),
            input_variants: Args::input_variants(),
            input_lists: Args::input_lists(),
            callables: Return::callables(),
        });
    }
}

impl<Args: Arguments, Return: EmbeddingValue> SourceType for CallableType<Args, Return> {
    type Value<'scope> = Callable<'scope, Args, Return>;
}

impl<'scope, Args, Return> ReadValue for Callable<'scope, Args, Return> {
    type View<'value> = Self;
}

impl<'scope, Args, Return> SharedValue for Callable<'scope, Args, Return> {
    type Context = CallableContext<'scope>;

    fn view<'value>(value: BorrowedValue<'value>, context: &Self::Context) -> Self {
        Self {
            value: value.function(),
            context: context.clone(),
            marker: PhantomData,
        }
    }
}

impl<Profile: HostProfile, Args: Arguments, Return: EmbeddingValue> ScopedOutput<Profile>
    for CallableType<Args, Return>
{
    type Retention = ();

    fn retain(_: &Profile::ExternalStores) {}

    fn context<'scope>(
        brand: ScopeBrand<'scope>,
        _: (),
        owner: &Arc<()>,
        callables: &mut OutputCallables<'scope>,
    ) -> <Self::Value<'scope> as SharedValue>::Context {
        let declaration = callables.take();
        CallableContext {
            brand,
            owner: Arc::clone(owner),
            declaration,
        }
    }
}

impl<Profile: HostProfile, Args: Arguments, Return: EmbeddingValue> ScopedTake<Profile>
    for CallableType<Args, Return>
{
    fn take<'scope>(
        output: &mut EmbeddingOutput,
        context: &<Self::Value<'scope> as SharedValue>::Context,
    ) -> Self::Value<'scope> {
        Callable {
            value: output.take_function(),
            context: context.clone(),
            marker: PhantomData,
        }
    }
}

impl<Profile: HostProfile, Args: Arguments, Return: EmbeddingValue> ScopedReturn<Profile>
    for CallableType<Args, Return>
{
    fn input_constructions(
        entries: &LibraryFunctionEntries,
        slot: usize,
    ) -> &LibraryInputConstructions {
        &entries.functions[slot].inputs
    }

    fn output_callables(entries: &LibraryFunctionEntries, slot: usize) -> &[LibraryCallable] {
        &entries.functions[slot].callables
    }

    async fn call<'scope>(
        execution: &crate::runtime::execution::EntryContext<Profile>,
        entries: &LibraryFunctionEntries,
        slot: usize,
        inputs: RetainedInputs,
        context: <Self::Value<'scope> as SharedValue>::Context,
    ) -> Result<Self::Value<'scope>, CallError> {
        let mut output = entries.functions[slot]
            .function
            .call(execution, inputs)
            .await?;
        Ok(<Self as ScopedTake<Profile>>::take(&mut output, &context))
    }
}

impl<'scope, Args: Arguments, Return: EmbeddingValue>
    ScopedInputValue<Callable<'scope, Args, Return>, ScopeBrand<'scope>>
    for CallableType<Args, Return>
{
    type ScopedRuntime = EmbeddingCallable;

    fn owners_match(_: &Callable<'scope, Args, Return>, _: &Arc<()>) -> bool {
        // The invariant scope brand is created with this loaded owner.
        true
    }

    fn into_runtime(
        input: Callable<'scope, Args, Return>,
        _: &mut InputConstructions<'_>,
        _: &EmbeddingInputStorage,
    ) -> Self::ScopedRuntime {
        input.value
    }
}

impl<'scope, Args: Arguments, Return: EmbeddingValue>
    ScopedInputValue<&Callable<'scope, Args, Return>, ScopeBrand<'scope>>
    for CallableType<Args, Return>
{
    type ScopedRuntime = EmbeddingCallable;

    fn owners_match(_: &&Callable<'scope, Args, Return>, _: &Arc<()>) -> bool {
        true
    }

    fn into_runtime(
        input: &Callable<'scope, Args, Return>,
        _: &mut InputConstructions<'_>,
        _: &EmbeddingInputStorage,
    ) -> Self::ScopedRuntime {
        input.value.clone()
    }
}

impl<'scope, Args: Arguments, Return: EmbeddingValue, Input>
    ScopedFreshInput<Input, ScopeBrand<'scope>> for CallableType<Args, Return>
where
    Self: ScopedInputValue<Input, ScopeBrand<'scope>, ScopedRuntime = EmbeddingCallable>,
{
    fn list_id(
        lists: &LibraryListConstructions,
        index: usize,
    ) -> crate::plan::execution::type_::FunctionListTypeId {
        lists.functions[index]
    }
}

impl<'scope, 'module: 'scope, Profile: HostProfile> ExecutionScope<'scope, 'module, Profile> {
    /// Invokes a retained source or Rust function in this live execution scope.
    #[allow(private_bounds)]
    pub fn invoke<'call, Args, Return, Input>(
        &'call self,
        function: &Callable<'scope, Args, Return>,
        arguments: Input,
    ) -> impl std::future::Future<Output = Result<Return::Value<'scope>, CallError>> + Send + 'call
    where
        Args: ScopedArgumentsInput<Input, ScopeBrand<'scope>>,
        Return: ScopedTake<Profile> + 'call,
    {
        let inputs = if !Args::owners_match(&arguments, self.owner) {
            Err(CallError::ForeignValue)
        } else {
            Ok(Args::into_inputs::<crate::runtime::CallbackInputs>(
                arguments,
                &function.context.declaration.inputs,
            ))
        };
        let callable_context = function.context.clone();
        let function = function.value.clone();
        async move {
            let inputs = inputs?;
            let retention = self
                .context
                .retain_outputs(|stores| Return::retain(stores))
                .await
                .map_err(|_| CallError::Cancelled)?;
            let context = Return::context(
                callable_context.brand,
                retention,
                &callable_context.owner,
                &mut OutputCallables::new(&callable_context.declaration.callables),
            );
            let mut output = function.invoke(&self.context, inputs).await?;
            Ok(Return::take(&mut output, &context))
        }
    }
}
#[cfg(test)]
mod tests {
    use crate::embedding::{CallableType, FunctionDeclaration, HostedModuleBuilder};
    use crate::{
        HostCall, HostCallCompletion, HostCallContinuation, HostCallError, HostCallableSchema,
        HostCaptures, HostConstructions, HostFunctionType, HostOwnedCompletion, HostProfile,
        HostProvider, HostProviderModule, HostProviderSet, HostReturns, HostTypeList,
        HostTypeListEnd, ModuleSource, PackageSource,
    };
    use futures_channel::oneshot;
    use futures_util::future::select;
    use num_bigint::BigInt;
    use std::cell::Cell;
    use std::collections::BTreeMap;
    use std::sync::{Arc, Mutex};
    use std::task::Poll;

    type End = HostTypeListEnd;
    type One<T> = HostTypeList<T, End>;
    type Callback = HostFunctionType<One<BigInt>, BigInt>;
    type Captures = HostTypeList<Callback, One<BigInt>>;
    type EmbeddedCallback = CallableType<(BigInt,), BigInt>;

    struct Profile;
    struct State {
        gates: BTreeMap<BigInt, oneshot::Receiver<BigInt>>,
        effects: Arc<Mutex<Vec<String>>>,
        calls: Cell<usize>,
    }
    impl HostProfile for Profile {
        type RunState = State;
        type ExternalStores = ();
        type ExecutionState = ();
    }
    impl HostProvider<Profile> for Profile {
        type State = State;
        fn project(state: &mut State) -> &mut State {
            state
        }
    }

    struct Delayed;
    impl HostCallableSchema for Delayed {
        const PACKAGE: &'static str = "application";
        const MODULE: &'static str = "callbacks";
        const NAME: &'static str = "delayed";
        type Arguments = One<BigInt>;
        type Return = BigInt;
        type Captures = Captures;
        type Constructions = End;
        type Completion = HostReturns;
    }

    // Each invocation owns a distinct non-Clone guard. Releasing an invocation
    // must not retain or release its sibling's continuation state.
    struct InvocationGuard {
        argument: BigInt,
        effects: Arc<Mutex<Vec<String>>>,
    }
    impl Drop for InvocationGuard {
        fn drop(&mut self) {
            self.effects
                .lock()
                .unwrap()
                .push(format!("drop:{}", self.argument));
        }
    }

    fn delayed<'call>(
        mut call: HostCall<'call, Profile, Profile, BigInt>,
        captures: HostCaptures<'call, Captures>,
        constructions: HostConstructions<'call, End>,
        argument: BigInt,
    ) -> Result<HostCallContinuation<'call, BigInt>, HostCallError> {
        let (source, (offset, ())) = call.captures(captures);
        let source = call.owned_callable(source, &constructions);
        let state = call.state();
        state.calls.set(state.calls.get() + 1);
        state
            .effects
            .lock()
            .unwrap()
            .push(format!("enter:{argument}"));
        let gate = state.gates.remove(&argument).unwrap();
        let guard = InvocationGuard {
            argument: argument.clone(),
            effects: Arc::clone(&state.effects),
        };
        Ok(call.resume(constructions, move |context| {
            Box::pin(async move {
                let guard = guard;
                let delivered = gate.await.unwrap();
                guard
                    .effects
                    .lock()
                    .unwrap()
                    .push(format!("resume:{argument}:{delivered}"));
                let input = argument + delivered;
                let returned = source
                    .invoke(&context, move |_, _| (input, ()), |_, _, value| Ok(value))
                    .await
                    .unwrap();
                let result = returned + offset;
                drop(guard);
                Ok(HostOwnedCompletion::new(move |call, _| {
                    Ok(call.return_value(result))
                }))
            })
        }))
    }

    fn note<'call>(
        mut call: HostCall<'call, Profile, Profile, ()>,
        value: BigInt,
    ) -> Result<HostCallCompletion<'call, ()>, HostCallError> {
        call.state()
            .effects
            .lock()
            .unwrap()
            .push(format!("source:{value}"));
        Ok(call.return_value(()))
    }

    #[test]
    fn native_aliases_resume_independently_reenter_source_and_cancel_only_the_selected_call() {
        for (cancel_first, close_scope) in [(false, false), (true, false), (false, true)] {
            let providers = HostProviderSet::from_providers([HostProviderModule::new(
                "application",
                "library",
            )
            .unwrap()
            .with_scoped_function::<Profile, (BigInt,), (), _>("note", note)
            .unwrap()])
            .unwrap()
            .with_resumable_callable::<Profile, Delayed, (BigInt,), _>(delayed)
            .unwrap();
            let typed = crate::compile_typed_host_program(
                "application",
                "library",
                [PackageSource::new(
                    "application",
                    Vec::<String>::new(),
                    [ModuleSource::new(
                        "library",
                        "library.gleam",
                        r#"
@external(erlang, "native", "note") fn note(value: Int) -> Nil
pub fn source(offset: Int) -> fn(Int) -> Int {
  fn(value) { note(value) value + offset }
}
"#,
                    )],
                )],
                providers,
            )
            .unwrap();
            let (mut bindings, source) = HostedModuleBuilder::new(typed)
                .unwrap()
                .function(FunctionDeclaration::<(BigInt,), EmbeddedCallback>::new(
                    "source",
                ))
                .unwrap();
            let factory = bindings.callable::<Delayed>().unwrap();
            let mut module = bindings.seal().unwrap();
            let (first_gate, first_receive) = oneshot::channel();
            let (second_gate, second_receive) = oneshot::channel();
            let effects = Arc::new(Mutex::new(Vec::new()));
            let mut state = State {
                gates: [
                    (BigInt::from(1), first_receive),
                    (BigInt::from(2), second_receive),
                ]
                .into(),
                effects: Arc::clone(&effects),
                calls: Cell::new(0),
            };
            let host = crate::execution_fixture::TestHost::default();
            let mut echo = Vec::new();
            let mut execution =
                Box::pin(
                    module.with_execution(&host, &mut state, &mut echo, async |scope| {
                        let source = scope.call(&source, (BigInt::from(100),)).await.unwrap();
                        let native = scope
                            .construct(&factory, (&source, (BigInt::from(7), ())))
                            .unwrap();
                        let alias = native.clone();
                        let first = Box::pin(scope.invoke(&native, (BigInt::from(1),)));
                        let second = Box::pin(scope.invoke(&alias, (BigInt::from(2),)));
                        drop(source);
                        drop(native);
                        drop(alias);
                        let (second, first) = select(first, second).await.factor_first();
                        assert_eq!(second.unwrap(), BigInt::from(129));
                        if cancel_first {
                            drop(first);
                            None
                        } else {
                            Some(first.await.unwrap())
                        }
                    }),
                );
            assert!(host.poll(execution.as_mut()).is_pending());
            assert_eq!(*effects.lock().unwrap(), ["enter:1", "enter:2"]);
            if close_scope {
                drop(execution);
                host.step();
                assert!(first_gate.is_canceled());
                assert!(second_gate.is_canceled());
                let mut released = effects.lock().unwrap()[2..].to_vec();
                released.sort();
                assert_eq!(released, ["drop:1", "drop:2"]);
                assert_eq!(state.calls.get(), 2);
                assert!(echo.is_empty());
                continue;
            }
            second_gate.send(BigInt::from(20)).unwrap();
            let after_second = host.poll(execution.as_mut());
            if cancel_first {
                assert_eq!(after_second.map(Result::unwrap), Poll::Ready(None));
                assert!(first_gate.is_canceled());
                assert_eq!(
                    *effects.lock().unwrap(),
                    [
                        "enter:1",
                        "enter:2",
                        "resume:2:20",
                        "source:22",
                        "drop:2",
                        "drop:1"
                    ]
                );
            } else {
                assert!(after_second.is_pending());
                assert!(!first_gate.is_canceled());
                assert_eq!(
                    *effects.lock().unwrap(),
                    ["enter:1", "enter:2", "resume:2:20", "source:22", "drop:2"]
                );
                first_gate.send(BigInt::from(10)).unwrap();
                assert_eq!(
                    host.poll(execution.as_mut()).map(Result::unwrap),
                    Poll::Ready(Some(BigInt::from(118)))
                );
                assert_eq!(
                    *effects.lock().unwrap(),
                    [
                        "enter:1",
                        "enter:2",
                        "resume:2:20",
                        "source:22",
                        "drop:2",
                        "resume:1:10",
                        "source:11",
                        "drop:1"
                    ]
                );
            }
            drop(execution);
            assert_eq!(state.calls.get(), 2);
            assert!(state.gates.is_empty());
            assert!(echo.is_empty());
        }
    }

    #[test]
    fn callable_lists_preserve_captures_through_retained_and_fresh_inputs() {
        use crate::embedding::List;
        type Functions = List<CallableType<(), BigInt>>;
        let source = r#"
pub fn functions(base: Int) { [fn() { base }, fn() { base + 1 }] }
pub fn keep(values: List(fn() -> Int)) { values }
"#;
        let typed = crate::compile_typed_host_program(
            "application",
            "library",
            [PackageSource::new(
                "application",
                Vec::<&str>::new(),
                [ModuleSource::new("library", "library.gleam", source)],
            )],
            HostProviderSet::<crate::StatelessHostProfile>::new([]).unwrap(),
        )
        .unwrap();
        let (mut bindings, functions) = HostedModuleBuilder::new(typed)
            .unwrap()
            .function(FunctionDeclaration::<(BigInt,), Functions>::new(
                "functions",
            ))
            .unwrap();
        let keep = bindings
            .function(FunctionDeclaration::<(Functions,), Functions>::new("keep"))
            .unwrap();
        let mut module = bindings.seal().unwrap();
        let host = crate::execution_fixture::TestHost::default();
        let mut echo = Vec::new();
        host.block_on(
            module.with_execution(&host, &mut (), &mut echo, async |scope| {
                let original = scope.call(&functions, (BigInt::from(41),)).await.unwrap();
                let alias = scope.call(&keep, (&original,)).await.unwrap();
                let first = alias.read_item(0, std::convert::identity).unwrap();
                let second = alias.read_item(1, std::convert::identity).unwrap();
                let fresh = scope.call(&keep, (vec![second, first],)).await.unwrap();
                drop(original);
                drop(alias);
                assert_eq!(fresh.len(), 2);
                assert!(fresh.read_item(2, std::convert::identity).is_none());
                assert_eq!(
                    scope
                        .invoke(&fresh.read_item(0, std::convert::identity).unwrap(), ())
                        .await
                        .unwrap(),
                    BigInt::from(42)
                );
                assert_eq!(
                    scope
                        .invoke(&fresh.read_item(1, std::convert::identity).unwrap(), ())
                        .await
                        .unwrap(),
                    BigInt::from(41)
                );
            }),
        )
        .unwrap();
        assert!(echo.is_empty());
    }
}
