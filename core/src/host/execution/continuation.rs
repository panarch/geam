use super::{HostExecutionContext, HostExecutionError, HostOwnedCompletion, NativeScope};
use crate::host::{
    HostCall, HostConstructions, HostProfile, HostProvider, HostType, HostTypeSequence,
};
use crate::runtime::execution::Continuation;
use std::convert::Infallible;
use std::future::Future;
use std::marker::PhantomData;
use std::pin::Pin;

/// An owned continuation of the current native call, not a Gleam Future value.
pub struct HostCallContinuation<'call, Return> {
    pub(crate) continuation: Continuation,
    call: PhantomData<&'call mut ()>,
    return_: PhantomData<fn() -> Return>,
}

/// An owned continuation that can suspend but cannot return a source value.
///
/// Its error or cancellation stays in the original execution. A pending
/// continuation is driven and cancelled through the same host as a value call.
pub struct HostNeverContinuation<'call> {
    pub(crate) continuation: Continuation<Infallible>,
    call: PhantomData<&'call mut ()>,
}

impl<'call, Profile, Provider, Output> HostCall<'call, Profile, Provider, Output>
where
    Profile: HostProfile,
    Provider: HostProvider<Profile>,
    Output: HostType,
{
    /// Continues this call with owned inputs and short access to its original host.
    ///
    /// The caller's execution driver polls the returned operation. Neither this
    /// method nor registration creates an executor or starts a source Future.
    /// An uninhabited result specialization executes this operation through the
    /// failure-only path. Its owned completion codec still runs; codec errors
    /// and cancellation retain their original domains, and no successful value
    /// can escape that specialization. Pending cancellation does not guarantee
    /// execution of provider-owned cleanup.
    pub fn resume<Constructions: HostTypeSequence>(
        self,
        constructions: HostConstructions<'call, Constructions>,
        start: impl for<'run> FnOnce(
            HostExecutionContext<'run, Profile, Provider, Constructions>,
        ) -> Pin<
            Box<
                dyn Future<
                        Output = Result<
                            HostOwnedCompletion<Profile, Provider, Output, Constructions>,
                            HostExecutionError,
                        >,
                    > + Send
                    + 'run,
            >,
        > + Send
        + 'static,
    ) -> HostCallContinuation<'call, Output> {
        let callable_base = constructions.callable_base();
        let execution = self.runtime.execution();
        let codec = self.runtime.codec_scope();
        let origin = self.runtime.origin();
        let continuation = Continuation::new(async move {
            let mut scope = NativeScope;
            let completion = start(HostExecutionContext::new(
                &mut scope,
                execution.clone(),
                codec.clone(),
                origin.clone(),
                callable_base,
            ))
            .await;
            execution
                .complete_native(completion, codec, origin, callable_base)
                .await
        });
        HostCallContinuation {
            continuation,
            call: PhantomData,
            return_: PhantomData,
        }
    }

    /// Continues a diverging native body without fabricating a return value.
    pub fn resume_never<Constructions: HostTypeSequence>(
        self,
        constructions: HostConstructions<'call, Constructions>,
        start: impl for<'run> FnOnce(
            HostExecutionContext<'run, Profile, Provider, Constructions>,
        ) -> Pin<
            Box<dyn Future<Output = Result<Infallible, HostExecutionError>> + Send + 'run>,
        > + Send
        + 'static,
    ) -> HostNeverContinuation<'call> {
        let callable_base = constructions.callable_base();
        let execution = self.runtime.execution();
        let codec = self.runtime.codec_scope();
        let origin = self.runtime.origin();
        let continuation = Continuation::new(async move {
            let mut scope = NativeScope;
            let error = match start(HostExecutionContext::new(
                &mut scope,
                execution.clone(),
                codec.clone(),
                origin.clone(),
                callable_base,
            ))
            .await
            {
                Ok(never) => match never {},
                Err(error) => error,
            };
            execution.fail_native(error, codec, origin).await
        });
        HostNeverContinuation {
            continuation,
            call: PhantomData,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::HostNeverContinuation;
    use crate::embedding::{CallableType, FunctionDeclaration, HostedModuleBuilder, NativeType};
    use crate::{
        HostCall, HostCallError, HostCallableSchema, HostCaptures, HostConstructions,
        HostCustomConstructorListEnd, HostCustomSchema, HostCustomType, HostDiverges,
        HostExecutionError, HostFailure, HostFunctionType, HostProfile, HostProvider,
        HostProviderSet, HostType, HostTypeList, HostTypeListEnd, HostTypeParameter, ModuleSource,
        PackageSource,
    };
    use num_bigint::BigInt;
    use std::cell::Cell;
    use std::marker::PhantomData;
    use std::task::Poll;

    type End = HostTypeListEnd;
    type One<T> = HostTypeList<T, End>;
    type Thunk = HostFunctionType<End, BigInt>;
    struct Abort<Return>(PhantomData<Return>);
    impl<Return: HostType> HostCallableSchema for Abort<Return> {
        const PACKAGE: &'static str = "application";
        const MODULE: &'static str = "callbacks";
        const NAME: &'static str = "abort";
        type Arguments = End;
        type Return = Return;
        type Captures = One<Thunk>;
        type Constructions = End;
        type Completion = HostDiverges;
    }
    struct NeverSchema;
    impl HostCustomSchema for NeverSchema {
        const PACKAGE: &'static str = "application";
        const MODULE: &'static str = "library";
        const NAME: &'static str = "Never";
        const PARAMETER_COUNT: usize = 0;
        type Constructors = HostCustomConstructorListEnd;
    }
    type Never = HostCustomType<NeverSchema>;
    type NeverView = <Never as NativeType>::Shape;

    #[derive(Default)]
    struct State {
        entries: Cell<usize>,
        events: Vec<&'static str>,
        cancel: bool,
    }
    struct Profile;
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

    fn abort<'call>(
        mut call: HostCall<'call, Profile, Profile, HostTypeParameter<0>>,
        captures: HostCaptures<'call, One<Thunk>>,
        constructions: HostConstructions<'call, End>,
    ) -> Result<HostNeverContinuation<'call>, HostCallError> {
        let (callback, ()) = call.captures(captures);
        let callback = call.owned_callable(callback, &constructions);
        let state = call.state();
        state.entries.set(state.entries.get() + 1);
        state.events.push("entered");
        let cancelled = state.cancel;
        Ok(call.resume_never(constructions, move |context| {
            Box::pin(async move {
                let mut pending = true;
                std::future::poll_fn(|cx| {
                    if pending {
                        pending = false;
                        cx.waker().wake_by_ref();
                        Poll::Pending
                    } else {
                        Poll::Ready(())
                    }
                })
                .await;
                context
                    .with_state(|state| state.events.push("resumed"))
                    .await
                    .unwrap();
                if cancelled {
                    return Err(HostExecutionError::Cancelled);
                }
                let value = callback
                    .invoke(&context, |_, _| (), |_, _, value| Ok(value))
                    .await?;
                context
                    .with_state(|state| state.events.push("returned callback"))
                    .await
                    .unwrap();
                Err(HostFailure::new(format!("stopped after {value}")).into())
            })
        }))
    }

    #[test]
    fn diverging_native_continuations_resume_once_and_preserve_failure_domains() {
        let providers = HostProviderSet::new([])
            .unwrap()
            .with_resumable_callable::<Profile, Abort<HostTypeParameter<0>>, (), _>(abort)
            .unwrap();
        let typed = crate::compile_typed_host_program(
            "application",
            "library",
            [PackageSource::new(
                "application",
                Vec::<&str>::new(),
                [ModuleSource::new(
                    "library",
                    "library.gleam",
                    r#"
pub type Never
pub fn callback(panics: Bool) -> fn() -> Int {
  fn() {
    echo "callback"
    case panics { True -> panic as "original callback failure" False -> 42 }
  }
}
pub fn value(callback: fn() -> Int) { callback() }
pub fn never(callback: fn() -> Never) -> Int { let _ = callback() 0 }
"#,
                )],
            )],
            providers,
        )
        .unwrap();
        let (mut bindings, callback) = HostedModuleBuilder::new(typed)
            .unwrap()
            .function(FunctionDeclaration::<(bool,), CallableType<(), BigInt>>::new("callback"))
            .unwrap();
        let value = bindings
            .function(FunctionDeclaration::<(CallableType<(), BigInt>,), BigInt>::new("value"))
            .unwrap();
        let never = bindings
            .function(FunctionDeclaration::<(CallableType<(), NeverView>,), BigInt>::new("never"))
            .unwrap();
        let value_factory = bindings.callable::<Abort<BigInt>>().unwrap();
        let never_factory = bindings.callable::<Abort<Never>>().unwrap();
        let mut module = bindings.seal().unwrap();
        let host = crate::execution_fixture::TestHost::default();
        for uninhabited in [false, true] {
            for (cancel, panics) in [(false, false), (false, true), (true, false)] {
                let mut state = State {
                    cancel,
                    ..State::default()
                };
                let mut echoes = Vec::new();
                let result = host
                    .block_on(module.with_execution(
                        &host,
                        &mut state,
                        &mut echoes,
                        async |scope| {
                            let callback = scope.call(&callback, (panics,)).await.unwrap();
                            if uninhabited {
                                let function =
                                    scope.construct(&never_factory, (&callback, ())).unwrap();
                                scope.call(&never, (&function,)).await
                            } else {
                                let function =
                                    scope.construct(&value_factory, (&callback, ())).unwrap();
                                scope.call(&value, (&function,)).await
                            }
                        },
                    ))
                    .unwrap();
                let error = result.unwrap_err();
                assert_eq!(state.entries.get(), 1);
                if cancel {
                    assert_eq!(error, crate::embedding::CallError::Cancelled);
                    assert!(echoes.is_empty());
                    assert_eq!(state.events, ["entered", "resumed"]);
                } else {
                    assert_eq!(
                        echoes
                            .iter()
                            .map(|echo| echo.value().inspect().to_string())
                            .collect::<Vec<_>>(),
                        ["\"callback\""]
                    );
                    if panics {
                        assert_eq!(error.to_string(), "panic: original callback failure");
                        assert_eq!(state.events, ["entered", "resumed"]);
                    } else {
                        assert_eq!(
                            error.to_string(),
                            "host function application::callbacks.abort failed: stopped after 42"
                        );
                        assert_eq!(state.events, ["entered", "resumed", "returned callback"]);
                    }
                }
            }
        }
    }
}
