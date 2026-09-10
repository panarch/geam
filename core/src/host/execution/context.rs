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
    ) -> Self {
        Self {
            execution,
            codec,
            origin,
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
            scope: self.scope,
            marker: PhantomData,
        }
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
        let request =
            self.execution
                .with_codec(self.codec.clone(), self.origin.clone(), move |runtime| {
                    operation(HostCall::new(runtime))
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
    execution: ExecutionContext<Profile>,
    codec: HostCodecScope,
    origin: HostCallOrigin,
    callable: RetainedCallable,
    signature: PhantomData<fn(Provider, Arguments, Constructions) -> Return>,
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
            execution: self.execution.clone(),
            codec: self.codec.clone(),
            origin: self.origin.clone(),
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
    /// Retains an invocable callback, not merely an opaque function value.
    pub fn owned_callable<
        Arguments: HostTypeSequence,
        CallbackReturn: HostType,
        Constructions: HostTypeSequence,
    >(
        &self,
        callback: HostCallable<'call, Arguments, CallbackReturn>,
        _constructions: &HostConstructions<'call, Constructions>,
    ) -> HostOwnedCallable<Profile, Provider, Arguments, CallbackReturn, Constructions> {
        let codec = self.runtime.codec_scope();
        let origin = HostCallOrigin::host(codec.function());
        HostOwnedCallable {
            execution: self.runtime.execution(),
            codec,
            origin,
            callable: self.runtime.callable(callback.token),
            signature: PhantomData,
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
    /// Invokes the retained callback with one owned input and output conversion.
    ///
    /// Both codecs run synchronously on its original execution. No call-scoped
    /// view crosses an await, and a Future-valued result is not implicitly driven.
    pub fn invoke<'request, Output: Send + 'static, Inputs, Decode, ContextConstructions>(
        &'request self,
        context: &'request HostExecutionContext<'_, Profile, Provider, ContextConstructions>,
        inputs: Inputs,
        decode: Decode,
    ) -> impl Future<Output = Result<Output, HostExecutionError>> + Send + 'request
    where
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
        let invocation = self.execution.invoke_owned(
            self.callable.clone(),
            self.codec.clone(),
            self.origin.clone(),
            move |runtime| {
                let values = inputs(HostCall::new(runtime), HostConstructions::new());
                let mut scoped = Vec::new();
                crate::host::type_::into_scoped_values::<Arguments>(values, &mut scoped);
                runtime.callback_inputs(scoped.into_boxed_slice())
            },
            move |runtime, token| {
                let value = crate::host::type_::from_runtime_token::<Return, _>(runtime, token);
                decode(HostCall::new(runtime), HostConstructions::new(), value)
            },
        );
        async move {
            let _scope = &context.scope;
            invocation.await
        }
    }
}
