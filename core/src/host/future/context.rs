use crate::host::{
    HostCall, HostCallError, HostCallable, HostCodecScope, HostConstructions, HostProfile,
    HostProvider, HostType, HostTypeSequence,
};
pub use crate::runtime::SharedExecutionError;
use crate::runtime::work::Dependencies;
use crate::runtime::work::execution::{Completion, WorkContext};
use crate::runtime::{HostCallOrigin, RetainedCallable};
use std::fmt;
use std::future::Future;
use std::marker::PhantomData;

/// A native work failure, separate from the Gleam function's return value.
#[derive(Debug)]
pub enum HostFutureError {
    /// The operation was cancelled before completing.
    Cancelled,
    /// A native failure or an unchanged error from a Gleam callback.
    Host(HostCallError),
    /// The unchanged shared failure of an explicitly observed source Future.
    Execution(SharedExecutionError),
}

pub(crate) struct NativeScope;

/// Access to short operations while one owned native Future is running.
///
/// The context borrows the native work's own lifetime, not its creating Gleam
/// invocation or the host state. It cannot survive the native Future:
///
/// ```compile_fail
/// use geam_core::host::{HostFutureContext, HostProfile, HostProvider, HostTypeSequence};
/// fn escape<'work, P, H, C>(context: HostFutureContext<'work, P, H, C>)
///     -> HostFutureContext<'static, P, H, C>
/// where P: HostProfile, H: HostProvider<P>, C: HostTypeSequence {
///     context
/// }
/// ```
pub struct HostFutureContext<'work, Profile, Provider, Constructions>
where
    Profile: HostProfile,
    Provider: HostProvider<Profile>,
    Constructions: HostTypeSequence,
{
    pub(in crate::host::future) work: WorkContext<Profile>,
    pub(in crate::host::future) dependencies: Dependencies<Completion>,
    scope: &'work mut NativeScope,
    marker: PhantomData<fn(Provider, Constructions)>,
}

/// A retained executable Gleam callback with its original execution endpoint.
///
/// Each invocation is distinct; the exact callable and captures are shared.
pub struct HostFutureCallable<Profile, Provider, Arguments, Return, Constructions>
where
    Profile: HostProfile,
    Provider: HostProvider<Profile>,
    Arguments: HostTypeSequence,
    Return: HostType,
    Constructions: HostTypeSequence,
{
    work: WorkContext<Profile>,
    codec: HostCodecScope,
    origin: HostCallOrigin,
    callable: RetainedCallable,
    signature: PhantomData<fn(Provider, Arguments, Constructions) -> Return>,
}

impl<Profile, Provider, Arguments, Return, Constructions> Clone
    for HostFutureCallable<Profile, Provider, Arguments, Return, Constructions>
where
    Profile: HostProfile,
    Provider: HostProvider<Profile>,
    Arguments: HostTypeSequence,
    Return: HostType,
    Constructions: HostTypeSequence,
{
    fn clone(&self) -> Self {
        Self {
            work: self.work.clone(),
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
    pub fn future_callable<
        Arguments: HostTypeSequence,
        CallbackReturn: HostType,
        Constructions: HostTypeSequence,
    >(
        &self,
        callback: HostCallable<'call, Arguments, CallbackReturn>,
        _constructions: &HostConstructions<'call, Constructions>,
    ) -> HostFutureCallable<Profile, Provider, Arguments, CallbackReturn, Constructions> {
        let codec = self.runtime.codec_scope();
        let origin = HostCallOrigin::host(codec.function());
        HostFutureCallable {
            work: self.runtime.work(),
            codec,
            origin,
            callable: self.runtime.callable(callback.token),
            signature: PhantomData,
        }
    }
}

impl<'work, Profile, Provider, Constructions>
    HostFutureContext<'work, Profile, Provider, Constructions>
where
    Profile: HostProfile,
    Provider: HostProvider<Profile>,
    Constructions: HostTypeSequence,
{
    pub(crate) fn new(
        scope: &'work mut NativeScope,
        work: WorkContext<Profile>,
        dependencies: Dependencies<Completion>,
    ) -> Self {
        Self {
            work,
            dependencies,
            scope,
            marker: PhantomData,
        }
    }

    pub(crate) fn without_constructions(
        self,
    ) -> HostFutureContext<'work, Profile, Provider, crate::host::HostTypeListEnd> {
        HostFutureContext {
            work: self.work,
            dependencies: self.dependencies,
            scope: self.scope,
            marker: PhantomData,
        }
    }

    /// Runs a non-async operation against this provider's original mutable state.
    ///
    /// The operation and its result are owned. A reference into state cannot be
    /// returned or retained across the native Future's next await point.
    ///
    /// ```compile_fail
    /// use geam_core::host::{HostFutureContext, HostProfile, HostProvider, HostTypeListEnd};
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
    /// async fn borrow_state(context: HostFutureContext<'_, Profile, Provider, HostTypeListEnd>) {
    ///     let borrowed = context.with_state(|state| state.as_str()).await;
    ///     drop(borrowed);
    /// }
    /// ```
    pub fn with_state<'request, Output: Send + 'static, Operation>(
        &'request self,
        operation: Operation,
    ) -> impl Future<Output = Result<Output, HostFutureError>> + Send + 'request
    where
        Operation: FnOnce(&mut Provider::State) -> Output + Send + 'static,
    {
        let request = self
            .work
            .with_state(move |state| operation(Provider::project(state)));
        async move {
            let _scope = &self.scope;
            request.await.map_err(Into::into)
        }
    }
}

impl<Profile, Provider, Arguments, Return, Constructions>
    HostFutureCallable<Profile, Provider, Arguments, Return, Constructions>
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
        context: &'request HostFutureContext<'_, Profile, Provider, ContextConstructions>,
        inputs: Inputs,
        decode: Decode,
    ) -> impl Future<Output = Result<Output, HostFutureError>> + Send + 'request
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
                Return::Value<'call>,
            ) -> Result<Output, HostCallError>
            + Send
            + 'static,
    {
        let invocation = self.work.invoke_owned(
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
                decode(HostCall::new(runtime), value)
            },
        );
        async move {
            let _scope = &context.scope;
            invocation.await
        }
    }
}

impl From<crate::runtime::work::Cancelled> for HostFutureError {
    fn from(_: crate::runtime::work::Cancelled) -> Self {
        Self::Cancelled
    }
}

impl From<HostCallError> for HostFutureError {
    fn from(error: HostCallError) -> Self {
        Self::Host(error)
    }
}

impl From<crate::HostFailure> for HostFutureError {
    fn from(error: crate::HostFailure) -> Self {
        Self::Host(error.into())
    }
}

impl fmt::Display for HostFutureError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Cancelled => formatter.write_str("the Future operation was cancelled"),
            Self::Host(error) => fmt::Display::fmt(error, formatter),
            Self::Execution(error) => fmt::Display::fmt(error, formatter),
        }
    }
}

impl std::error::Error for HostFutureError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Cancelled => None,
            Self::Host(error) => Some(error),
            Self::Execution(error) => Some(error),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{HostFutureError, SharedExecutionError};
    use crate::runtime::shared::Shared;
    use crate::{ExecutionError, HostFailure, InvariantError, ValueType};
    use std::error::Error;

    #[test]
    fn termination_and_native_and_shared_failures_keep_distinct_diagnostics() {
        let cancelled = HostFutureError::from(crate::runtime::work::Cancelled);
        assert_eq!(cancelled.to_string(), "the Future operation was cancelled");
        assert_eq!(format!("{cancelled:?}"), "Cancelled");
        assert!(cancelled.source().is_none());

        let host = HostFutureError::from(HostFailure::new("disconnected"));
        assert_eq!(host.to_string(), "disconnected");
        assert_eq!(
            format!("{host:?}"),
            "Host(HostCallError { kind: Failure(HostFailure { message: \"disconnected\" }) })"
        );
        assert_eq!(
            host.source().expect("host failure").to_string(),
            "disconnected"
        );

        let shared = SharedExecutionError(Shared::new(ExecutionError::Invariant(
            InvariantError::ListIndexOutOfBounds {
                item_type: ValueType::Int,
                index: 1,
                length: 0,
            },
        )));
        let error = HostFutureError::Execution(shared);
        assert_eq!(
            error.to_string(),
            "list index out of bounds for Int list (index 1, length 0)"
        );
        assert_eq!(
            format!("{error:?}"),
            "Execution(Invariant(ListIndexOutOfBounds { item_type: Int, index: 1, length: 0 }))"
        );
        assert_eq!(
            error
                .source()
                .expect("shared execution failure")
                .to_string(),
            "list index out of bounds for Int list (index 1, length 0)"
        );
    }
}
