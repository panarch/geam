use super::HostExecutionError;
use crate::host::{
    HostCall, HostCallError, HostCallable, HostCodecScope, HostConstructions, HostProfile,
    HostProvider, HostType, HostTypeSequence,
};
use crate::runtime::execution::ExecutionContext;
use crate::runtime::{HostCallOrigin, RetainedCallable};
use std::future::Future;
use std::marker::PhantomData;

pub(crate) struct NativeScope;

/// Short typed access to the original host during an owned native operation.
pub struct HostExecutionContext<'run, Profile, Provider, Constructions>
where
    Profile: HostProfile,
    Provider: HostProvider<Profile>,
    Constructions: HostTypeSequence,
{
    execution: ExecutionContext<Profile>,
    codec: HostCodecScope,
    origin: HostCallOrigin,
    callable_base: usize,
    scope: &'run mut NativeScope,
    marker: PhantomData<fn(Provider, Constructions)>,
}

impl<'run, Profile, Provider, Constructions>
    HostExecutionContext<'run, Profile, Provider, Constructions>
where
    Profile: HostProfile,
    Provider: HostProvider<Profile>,
    Constructions: HostTypeSequence,
{
    pub(crate) fn new(
        scope: &'run mut NativeScope,
        execution: ExecutionContext<Profile>,
        codec: HostCodecScope,
        origin: HostCallOrigin,
        callable_base: usize,
    ) -> Self {
        Self {
            execution,
            codec,
            origin,
            callable_base,
            scope,
            marker: PhantomData,
        }
    }

    pub(crate) fn without_constructions(
        self,
    ) -> HostExecutionContext<'run, Profile, Provider, crate::host::HostTypeListEnd> {
        HostExecutionContext {
            execution: self.execution,
            codec: self.codec,
            origin: self.origin,
            callable_base: self.callable_base,
            scope: self.scope,
            marker: PhantomData,
        }
    }

    pub(crate) fn execution(&self) -> &ExecutionContext<Profile> {
        &self.execution
    }

    /// Runs one bounded typed value operation. Call-scoped views cannot escape.
    pub fn with_call<'request, Output: Send + 'static, Operation>(
        &'request self,
        operation: Operation,
    ) -> impl Future<Output = Result<Output, HostExecutionError>> + Send + 'request
    where
        Operation:
            for<'call> FnOnce(HostCall<'call, Profile, Provider, ()>) -> Output + Send + 'static,
    {
        self.with_constructions(move |call, _| operation(call))
    }

    /// Runs a bounded operation with this invocation's exact construction permissions.
    /// The operation and its constructed result cannot borrow the call or its proof.
    pub fn with_constructions<'request, Output: Send + 'static, Operation>(
        &'request self,
        operation: Operation,
    ) -> impl Future<Output = Result<Output, HostExecutionError>> + Send + 'request
    where
        Operation: for<'call> FnOnce(
                HostCall<'call, Profile, Provider, ()>,
                HostConstructions<'call, Constructions>,
            ) -> Output
            + Send
            + 'static,
    {
        let callable_base = self.callable_base;
        let request =
            self.execution
                .with_codec(self.codec.clone(), self.origin.clone(), move |runtime| {
                    operation(
                        HostCall::new(runtime),
                        HostConstructions::with_base(callable_base),
                    )
                });
        async move {
            let _scope = &self.scope;
            request.await.map_err(Into::into)
        }
    }

    /// Runs a non-async operation against this provider's original mutable state.
    ///
    /// The operation and its result are owned. A reference into state cannot be
    /// returned or retained across the native Future's next await point.
    ///
    /// ```compile_fail
    /// use geam_core::host::{HostExecutionContext, HostProfile, HostProvider, HostTypeListEnd};
    /// struct Profile;
    /// impl HostProfile for Profile {
    ///     type RunState = String;
    ///     type ExternalStores = ();
    ///     type ExecutionState = ();
    /// }
    /// struct Provider;
    /// impl HostProvider<Profile> for Provider {
    ///     type State = String;
    ///     fn project(state: &mut String) -> &mut String { state }
    /// }
    /// async fn borrow_state(context: HostExecutionContext<'_, Profile, Provider, HostTypeListEnd>) {
    ///     let borrowed = context.with_state(|state| state.as_str()).await;
    ///     drop(borrowed);
    /// }
    /// ```
    pub fn with_state<'request, Output: Send + 'static, Operation>(
        &'request self,
        operation: Operation,
    ) -> impl Future<Output = Result<Output, HostExecutionError>> + Send + 'request
    where
        Operation: FnOnce(&mut Provider::State) -> Output + Send + 'static,
    {
        let request = self
            .execution
            .with_state(move |state| operation(Provider::project(state)));
        async move {
            let _scope = &self.scope;
            request.await.map_err(Into::into)
        }
    }
}

// The original endpoint and static codec proof used when a demanded container
// field becomes an owned callback. No source item is decoded by this owner.
pub(crate) struct CallableRetention<Profile: HostProfile, Provider: HostProvider<Profile>> {
    execution: ExecutionContext<Profile>,
    codec: HostCodecScope,
    origin: HostCallOrigin,
    callable_base: usize,
    provider: PhantomData<fn(Provider)>,
}

impl<Profile: HostProfile, Provider: HostProvider<Profile>> Clone
    for CallableRetention<Profile, Provider>
{
    fn clone(&self) -> Self {
        Self {
            execution: self.execution.clone(),
            codec: self.codec.clone(),
            origin: self.origin.clone(),
            callable_base: self.callable_base,
            provider: PhantomData,
        }
    }
}

impl<Profile: HostProfile, Provider: HostProvider<Profile>> CallableRetention<Profile, Provider> {
    pub(crate) fn bind<
        Arguments: HostTypeSequence,
        Return: HostType,
        Constructions: HostTypeSequence,
    >(
        self,
        callable: RetainedCallable,
    ) -> HostOwnedCallable<Profile, Provider, Arguments, Return, Constructions> {
        HostOwnedCallable {
            retention: self,
            callable,
            signature: PhantomData,
        }
    }
}

/// A retained executable Gleam callback with its original execution endpoint.
///
/// Each invocation is distinct; the exact callable and captures are shared.
pub struct HostOwnedCallable<Profile, Provider, Arguments, Return, Constructions>
where
    Profile: HostProfile,
    Provider: HostProvider<Profile>,
    Arguments: HostTypeSequence,
    Return: HostType,
    Constructions: HostTypeSequence,
{
    retention: CallableRetention<Profile, Provider>,
    callable: RetainedCallable,
    signature: PhantomData<fn(Arguments, Constructions) -> Return>,
}

impl<Profile, Provider, Arguments, Return, Constructions> Clone
    for HostOwnedCallable<Profile, Provider, Arguments, Return, Constructions>
where
    Profile: HostProfile,
    Provider: HostProvider<Profile>,
    Arguments: HostTypeSequence,
    Return: HostType,
    Constructions: HostTypeSequence,
{
    fn clone(&self) -> Self {
        Self {
            retention: self.retention.clone(),
            callable: self.callable.clone(),
            signature: PhantomData,
        }
    }
}

impl<'call, Profile, Provider, Return> HostCall<'call, Profile, Provider, Return>
where
    Profile: HostProfile,
    Provider: HostProvider<Profile>,
    Return: HostType,
{
    /// Starts an independent logical invocation owned by the execution domain.
    /// Its source return value is discarded; source failure is reported to the
    /// domain's execution services. It does not inherit the caller's lifetime.
    pub fn spawn<CallbackReturn: HostType>(
        &mut self,
        callback: HostCallable<'call, crate::host::HostTypeListEnd, CallbackReturn>,
    ) -> crate::execution::ExecutionUnit {
        let callable = self.runtime.callable(callback.token);
        let origin = HostCallOrigin::host(self.runtime.codec_scope().function());
        self.runtime.spawn(callable, origin)
    }

    /// Retains an invocable callback, not merely an opaque function value.
    pub fn owned_callable<
        Arguments: HostTypeSequence,
        CallbackReturn: HostType,
        Constructions: HostTypeSequence,
    >(
        &self,
        callback: HostCallable<'call, Arguments, CallbackReturn>,
        constructions: &HostConstructions<'call, Constructions>,
    ) -> HostOwnedCallable<Profile, Provider, Arguments, CallbackReturn, Constructions> {
        self.owned_callable_with::<Provider, _, _, _>(callback, constructions)
    }

    #[doc(hidden)]
    pub fn owned_callable_with<
        CodecProvider: HostProvider<Profile>,
        Arguments: HostTypeSequence,
        CallbackReturn: HostType,
        Constructions: HostTypeSequence,
    >(
        &self,
        callback: HostCallable<'call, Arguments, CallbackReturn>,
        constructions: &HostConstructions<'call, Constructions>,
    ) -> HostOwnedCallable<Profile, CodecProvider, Arguments, CallbackReturn, Constructions> {
        self.callable_retention_with::<CodecProvider, _>(constructions)
            .bind(self.runtime.callable(callback.token))
    }

    pub(crate) fn callable_retention_with<
        CodecProvider: HostProvider<Profile>,
        Constructions: HostTypeSequence,
    >(
        &self,
        constructions: &HostConstructions<'call, Constructions>,
    ) -> CallableRetention<Profile, CodecProvider> {
        let codec = self.runtime.codec_scope();
        let origin = HostCallOrigin::host(codec.function());
        CallableRetention {
            execution: self.runtime.execution().with_unit(None),
            codec,
            origin,
            callable_base: constructions.callable_base(),
            provider: PhantomData,
        }
    }
}

impl<Profile, Provider, Arguments, Return, Constructions>
    HostOwnedCallable<Profile, Provider, Arguments, Return, Constructions>
where
    Profile: HostProfile,
    Provider: HostProvider<Profile>,
    Arguments: HostTypeSequence,
    Return: HostType,
    Constructions: HostTypeSequence,
{
    /// Restores this exact function value into an active call in its original execution.
    ///
    /// The restored value preserves identity and shares its retained captures.
    /// A different execution is rejected before a callable token is produced.
    pub fn restore<'call, CallerProvider, Output>(
        &self,
        call: &mut HostCall<'call, Profile, CallerProvider, Output>,
    ) -> Result<HostCallable<'call, Arguments, Return>, HostCallError>
    where
        CallerProvider: HostProvider<Profile>,
        Output: HostType,
    {
        self.retention
            .execution
            .for_invocation(&call.runtime.execution())
            .map_err(|_| crate::HostFailure::new("callback belongs to another execution"))?;
        Ok(HostCallable::new(
            call.runtime.restore_callable(self.callable.clone()),
        ))
    }

    /// Invokes the retained callback with one owned input and output conversion.
    ///
    /// Both codecs run synchronously on its original execution. No call-scoped
    /// view crosses an await, and a Future-valued result is not implicitly driven.
    pub fn invoke<
        'request,
        Output: Send + 'static,
        Inputs,
        Decode,
        CallerProvider,
        ContextConstructions,
    >(
        &'request self,
        context: &'request HostExecutionContext<'_, Profile, CallerProvider, ContextConstructions>,
        inputs: Inputs,
        decode: Decode,
    ) -> impl Future<Output = Result<Output, HostExecutionError>> + Send + 'request
    where
        CallerProvider: HostProvider<Profile>,
        ContextConstructions: HostTypeSequence,
        Inputs: for<'call> FnOnce(
                HostCall<'call, Profile, Provider, ()>,
                HostConstructions<'call, Constructions>,
            ) -> Arguments::Values<'call>
            + Send
            + 'static,
        Decode: for<'call> FnOnce(
                HostCall<'call, Profile, Provider, ()>,
                HostConstructions<'call, Constructions>,
                Return::Value<'call>,
            ) -> Result<Output, HostCallError>
            + Send
            + 'static,
    {
        self.try_invoke(context, move |call, proof| Ok(inputs(call, proof)), decode)
    }

    /// Invokes this function after a fallible typed argument conversion.
    ///
    /// Conversion failure prevents callback entry. This permits higher-order
    /// arguments to reject foreign execution ownership before source effects.
    pub fn try_invoke<
        'request,
        Output: Send + 'static,
        Inputs,
        Decode,
        CallerProvider,
        ContextConstructions,
    >(
        &'request self,
        context: &'request HostExecutionContext<'_, Profile, CallerProvider, ContextConstructions>,
        inputs: Inputs,
        decode: Decode,
    ) -> impl Future<Output = Result<Output, HostExecutionError>> + Send + 'request
    where
        CallerProvider: HostProvider<Profile>,
        ContextConstructions: HostTypeSequence,
        Inputs: for<'call> FnOnce(
                HostCall<'call, Profile, Provider, ()>,
                HostConstructions<'call, Constructions>,
            ) -> Result<Arguments::Values<'call>, HostCallError>
            + Send
            + 'static,
        Decode: for<'call> FnOnce(
                HostCall<'call, Profile, Provider, ()>,
                HostConstructions<'call, Constructions>,
                Return::Value<'call>,
            ) -> Result<Output, HostCallError>
            + Send
            + 'static,
    {
        let execution = self.retention.execution.for_invocation(&context.execution);
        let callable_base = self.retention.callable_base;
        async move {
            let execution = execution?;
            let invocation = execution.invoke_owned(
                self.callable.clone(),
                self.retention.codec.clone(),
                self.retention.origin.clone(),
                move |runtime| {
                    let values = inputs(
                        HostCall::new(runtime),
                        HostConstructions::with_base(callable_base),
                    )?;
                    let mut scoped = Vec::new();
                    crate::host::type_::into_scoped_values::<Arguments>(values, &mut scoped);
                    Ok(runtime.callback_inputs(scoped.into_boxed_slice()))
                },
                move |runtime, token| {
                    let value = crate::host::type_::from_runtime_token::<Return, _>(runtime, token);
                    decode(
                        HostCall::new(runtime),
                        HostConstructions::with_base(callable_base),
                        value,
                    )
                },
            );
            let _scope = &context.scope;
            invocation.await
        }
    }
}

#[cfg(test)]
mod tests {
    use super::HostOwnedCallable;
    use crate::embedding::{CallableType, FunctionDeclaration, HostedModuleBuilder};
    use crate::{
        HostCall, HostCallCompletion, HostCallError, HostCallable, HostCallableSchema,
        HostCaptures, HostConstructions, HostFunctionType, HostProfile, HostProvider,
        HostProviderModule, HostProviderSet, HostTypeList, HostTypeListEnd, ModuleSource,
        PackageSource,
    };
    use num_bigint::BigInt;
    use std::cell::Cell;
    use std::sync::{Arc, Mutex};

    type End = HostTypeListEnd;
    type One<T> = HostTypeList<T, End>;
    type Function = HostFunctionType<One<BigInt>, BigInt>;
    type Retained = HostOwnedCallable<Profile, Producer, One<BigInt>, BigInt, End>;
    type EmbeddedFunction = CallableType<(BigInt,), BigInt>;
    type OpaqueFunction = crate::provider_support::HostOpaqueFunctionType<One<BigInt>, BigInt>;
    type SavedOpaque = crate::provider::Value<
        fn(BigInt) -> BigInt,
        crate::provider::ProviderValueContext<OpaqueFunction>,
    >;

    struct Profile;
    #[derive(Default)]
    struct State {
        saved: Arc<Mutex<Option<Retained>>>,
        opaque: Arc<Mutex<Option<SavedOpaque>>>,
        native_calls: Cell<usize>,
    }
    impl HostProfile for Profile {
        type RunState = State;
        type ExternalStores = ();
        type ExecutionState = ();
    }
    struct Producer;
    struct Receiver;
    impl HostProvider<Profile> for Producer {
        type State = State;
        fn project(state: &mut State) -> &mut State {
            state
        }
    }
    impl HostProvider<Profile> for Receiver {
        type State = State;
        fn project(state: &mut State) -> &mut State {
            state
        }
    }
    struct Add;
    impl HostCallableSchema for Add {
        const PACKAGE: &'static str = "application";
        const MODULE: &'static str = "library";
        const NAME: &'static str = "add";
        type Arguments = One<BigInt>;
        type Return = BigInt;
        type Captures = One<BigInt>;
        type Constructions = End;
        type Completion = crate::HostReturns;
    }

    fn add<'call>(
        mut call: HostCall<'call, Profile, Producer, BigInt>,
        captures: HostCaptures<'call, One<BigInt>>,
        _: HostConstructions<'call, End>,
        value: BigInt,
    ) -> Result<HostCallCompletion<'call, BigInt>, HostCallError> {
        let (offset, ()) = call.captures(captures);
        let calls = &call.state().native_calls;
        calls.set(calls.get() + 1);
        Ok(call.return_value(offset + value))
    }

    struct AddForty;
    impl HostCallableSchema for AddForty {
        const PACKAGE: &'static str = "application";
        const MODULE: &'static str = "library";
        const NAME: &'static str = "add_forty";
        type Arguments = One<BigInt>;
        type Return = BigInt;
        type Captures = End;
        type Constructions = End;
        type Completion = crate::HostReturns;
    }

    fn add_forty<'call>(
        mut call: HostCall<'call, Profile, Producer, BigInt>,
        captures: HostCaptures<'call, End>,
        _: HostConstructions<'call, End>,
        value: BigInt,
    ) -> Result<HostCallCompletion<'call, BigInt>, HostCallError> {
        let () = call.captures(captures);
        let calls = &call.state().native_calls;
        calls.set(calls.get() + 1);
        Ok(call.return_value(BigInt::from(40) + value))
    }

    fn save<'call>(
        mut call: HostCall<'call, Profile, Producer, ()>,
        constructions: HostConstructions<'call, End>,
        callback: HostCallable<'call, One<BigInt>, BigInt>,
    ) -> Result<HostCallCompletion<'call, ()>, HostCallError> {
        let callback = call.owned_callable(callback, &constructions);
        *call.state().saved.lock().unwrap() = Some(callback);
        Ok(call.return_value(()))
    }

    fn restore<'call>(
        mut call: HostCall<'call, Profile, Receiver, Function>,
    ) -> Result<HostCallCompletion<'call, Function>, HostCallError> {
        let callback = call.state().saved.lock().unwrap().as_ref().unwrap().clone();
        let restored = callback.restore(&mut call)?;
        Ok(call.return_value(restored))
    }

    fn save_opaque<'call>(
        mut call: HostCall<'call, Profile, Producer, ()>,
        callback: <OpaqueFunction as crate::HostType>::Value<'call>,
    ) -> Result<HostCallCompletion<'call, ()>, HostCallError> {
        let callback = SavedOpaque::from_host(&call, callback);
        *call.state().opaque.lock().unwrap() = Some(callback);
        Ok(call.return_value(()))
    }

    fn restore_opaque<'call>(
        mut call: HostCall<'call, Profile, Producer, OpaqueFunction>,
    ) -> Result<HostCallCompletion<'call, OpaqueFunction>, HostCallError> {
        let callback = call.state().opaque.lock().unwrap().take().unwrap();
        let callback = callback.into_host(&mut call);
        let retained = SavedOpaque::from_host(&call, callback);
        *call.state().opaque.lock().unwrap() = Some(retained);
        Ok(call.return_value(callback))
    }

    fn program() -> HostedModuleBuilder<Profile> {
        let provider = HostProviderModule::new("application", "library")
            .unwrap()
            .with_scoped_function_and_constructions::<Producer, (Function,), (), End, _>(
                "save", save,
            )
            .unwrap()
            .with_scoped_function::<Receiver, (), Function, _>("restore", restore)
            .unwrap()
            .with_scoped_function::<Producer, (OpaqueFunction,), (), _>("save_opaque", save_opaque)
            .unwrap()
            .with_scoped_function::<Producer, (), OpaqueFunction, _>(
                "restore_opaque",
                restore_opaque,
            )
            .unwrap()
            .with_callable::<Producer, Add, (BigInt,), _>(add)
            .unwrap()
            .with_callable::<Producer, AddForty, (BigInt,), _>(add_forty)
            .unwrap();
        HostedModuleBuilder::new(
            crate::compile_typed_host_program(
                "application",
                "library",
                [PackageSource::new(
                    "application",
                    Vec::<String>::new(),
                    [ModuleSource::new(
                        "library",
                        "library.gleam",
                        r#"
@external(erlang, "native", "save") fn save(callback: fn(Int) -> Int) -> Nil
@external(erlang, "native", "restore") fn restore() -> fn(Int) -> Int
@external(erlang, "native", "save_opaque") fn save_opaque(callback: fn(Int) -> Int) -> Nil
@external(erlang, "native", "restore_opaque") fn restore_opaque() -> fn(Int) -> Int
pub fn source(offset: Int) -> fn(Int) -> Int {
  fn(value) { echo "source" value + offset }
}
pub fn roundtrip(callback: fn(Int) -> Int) {
  save(callback)
  let restored = restore()
  #(callback == restored, restored(2))
}
pub fn later() { restore()(2) }
pub fn stash(callback: fn(Int) -> Int) { save_opaque(callback) }
pub fn later_opaque() { restore_opaque()(2) }
pub fn wrap(callback: fn(Int) -> Int) { fn(value) { callback(value) } }
"#,
                    )],
                )],
                HostProviderSet::from_providers([provider]).unwrap(),
            )
            .unwrap(),
        )
        .unwrap()
    }

    #[test]
    fn restoration_preserves_identity_and_rejects_live_foreign_and_closed_executions() {
        const REJECTED: &str = "host function application::library.restore failed: callback belongs to another execution";
        for native in [false, true] {
            let (mut bindings, source) = program()
                .function(FunctionDeclaration::<(BigInt,), EmbeddedFunction>::new(
                    "source",
                ))
                .unwrap();
            let roundtrip = bindings
                .function(
                    FunctionDeclaration::<(EmbeddedFunction,), (bool, BigInt)>::new("roundtrip"),
                )
                .unwrap();
            let later = bindings
                .function(FunctionDeclaration::<(), BigInt>::new("later"))
                .unwrap();
            let factory = bindings.callable::<Add>().unwrap();
            let mut module = bindings.seal().unwrap();
            let (foreign, foreign_later) = program()
                .function(FunctionDeclaration::<(), BigInt>::new("later"))
                .unwrap();
            let mut foreign = foreign.seal().unwrap();
            let mut state = State::default();
            let mut foreign_state = State {
                saved: Arc::clone(&state.saved),
                ..State::default()
            };
            let mut echoes = Vec::new();
            let mut foreign_echoes = Vec::new();
            let host = crate::execution_fixture::TestHost::default();
            host.block_on(
                module.with_execution(&host, &mut state, &mut echoes, async |scope| {
                    let callback = if native {
                        scope.construct(&factory, (BigInt::from(40), ())).unwrap()
                    } else {
                        scope.call(&source, (BigInt::from(40),)).await.unwrap()
                    };
                    assert_eq!(
                        scope.call(&roundtrip, (&callback,)).await.unwrap(),
                        (true, BigInt::from(42)),
                    );
                    foreign
                        .with_execution(
                            &host,
                            &mut foreign_state,
                            &mut foreign_echoes,
                            async |other| {
                                assert_eq!(
                                    other
                                        .call(&foreign_later, ())
                                        .await
                                        .unwrap_err()
                                        .to_string(),
                                    REJECTED
                                );
                            },
                        )
                        .await
                        .unwrap();
                }),
            )
            .unwrap();
            assert_eq!(state.native_calls.get(), usize::from(native));
            assert_eq!(echoes.len(), usize::from(!native));
            assert_eq!(foreign_state.native_calls.get(), 0);
            assert!(foreign_echoes.is_empty());
            host.block_on(
                module.with_execution(&host, &mut state, &mut echoes, async |scope| {
                    assert_eq!(
                        scope.call(&later, ()).await.unwrap_err().to_string(),
                        REJECTED
                    );
                }),
            )
            .unwrap();
            assert_eq!(state.native_calls.get(), usize::from(native));
            assert_eq!(echoes.len(), usize::from(!native));
        }
    }

    #[test]
    fn native_callbacks_reject_opaque_retention_across_executions() {
        for (capturing, wrappers) in [
            (false, 0),
            (true, 0),
            (false, 1),
            (true, 1),
            (false, 3),
            (true, 3),
        ] {
            let (mut bindings, stash) = program()
                .function(FunctionDeclaration::<(EmbeddedFunction,), ()>::new("stash"))
                .unwrap();
            let later = bindings
                .function(FunctionDeclaration::<(), BigInt>::new("later_opaque"))
                .unwrap();
            let wrap = bindings
                .function(FunctionDeclaration::<(EmbeddedFunction,), EmbeddedFunction>::new("wrap"))
                .unwrap();
            let factory = bindings.callable::<Add>().unwrap();
            let empty_factory = bindings.callable::<AddForty>().unwrap();
            let mut module = bindings.seal().unwrap();
            // This separately sealed plan has no native factory root or matching
            // native function table entry. Reject before looking up that target.
            let (foreign, foreign_later) = program()
                .function(FunctionDeclaration::<(), BigInt>::new("later_opaque"))
                .unwrap();
            let mut foreign = foreign.seal().unwrap();
            let host = crate::execution_fixture::TestHost::default();
            let mut state = State::default();
            let mut foreign_state = State {
                opaque: Arc::clone(&state.opaque),
                ..State::default()
            };
            host.block_on(
                module.with_execution(&host, &mut state, &mut drop, async |scope| {
                    let mut callback = if capturing {
                        scope.construct(&factory, (BigInt::from(40), ())).unwrap()
                    } else {
                        scope.construct(&empty_factory, ()).unwrap()
                    };
                    for _ in 0..wrappers {
                        callback = scope.call(&wrap, (&callback,)).await.unwrap();
                    }
                    scope.call(&stash, (&callback,)).await.unwrap();
                    assert_eq!(scope.call(&later, ()).await.unwrap(), BigInt::from(42));
                    foreign
                        .with_execution(&host, &mut foreign_state, &mut drop, async |other| {
                            assert_eq!(
                                other.call(&foreign_later, ()).await,
                                Err(crate::embedding::CallError::Cancelled)
                            );
                        })
                        .await
                        .unwrap();
                    // Rejection in the other live domain does not cancel this function.
                    assert_eq!(scope.call(&later, ()).await.unwrap(), BigInt::from(42));
                }),
            )
            .unwrap();
            assert_eq!(state.native_calls.get(), 2);
            assert_eq!(foreign_state.native_calls.get(), 0);
            let result = host
                .block_on(
                    module.with_execution(&host, &mut state, &mut drop, async |scope| {
                        scope.call(&later, ()).await
                    }),
                )
                .unwrap();
            assert_eq!(result, Err(crate::embedding::CallError::Cancelled));
            assert_eq!(state.native_calls.get(), 2);
        }
    }
}
